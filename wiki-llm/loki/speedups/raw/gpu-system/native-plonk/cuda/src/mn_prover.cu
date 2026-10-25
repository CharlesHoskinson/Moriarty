#include "mn_prover.h"

#include "packed_polys.h"
#include "params.h"
#include "plonk_prover.h"

#include <chrono>
#include <cstdint>
#include <cstring>
#include <memory>
#include <mutex>
#include <string>

using midnight_plonk::PackedPolys;
using midnight_plonk::ParamsKZG;
using midnight_plonk::ParseStatus;
using midnight_plonk::PlonkProof;
using midnight_plonk::ProveStatus;
using midnight_plonk::ProverKey;

namespace {
thread_local std::string last_error;

struct ParseCache {
  const uint8_t *params_ptr = nullptr;
  size_t params_len = 0;
  std::shared_ptr<ParamsKZG> params;

  const uint8_t *pk_ptr = nullptr;
  size_t pk_len = 0;
  std::shared_ptr<ProverKey> pk;

  const uint8_t *polys_ptr = nullptr;
  size_t polys_len = 0;
  uint64_t polys_hash = 0;
  std::shared_ptr<PackedPolys> polys;
};

thread_local ParseCache parse_cache;
static std::mutex shared_cache_mutex;
static ParseCache shared_cache;

void set_last_error(const char *message) {
  last_error = message ? message : "unknown error";
}

bool timing_enabled() {
  static int enabled = -1;
  if (enabled < 0) {
    const char *env = std::getenv("MIDNIGHT_CUDA_TIMING");
    enabled = (env && std::strcmp(env, "1") == 0) ? 1 : 0;
  }
  return enabled == 1;
}

void log_timing(const char *label, std::chrono::steady_clock::duration duration) {
  if (!timing_enabled()) {
    return;
  }
  auto us = std::chrono::duration_cast<std::chrono::microseconds>(duration).count();
  std::fprintf(stderr, "timing: %s=%lldus\n", label, static_cast<long long>(us));
}

uint64_t hash_bytes(const uint8_t *data, size_t len) {
  uint64_t hash = 1469598103934665603ULL;
  for (size_t i = 0; i < len; ++i) {
    hash ^= static_cast<uint64_t>(data[i]);
    hash *= 1099511628211ULL;
  }
  return hash;
}
}  // namespace

extern "C" int mn_prove(const mn_prove_input *in, mn_prove_output *out) {
  if (!in || !out || !out->out || !out->out_len) {
    set_last_error("invalid mn_prove arguments");
    return 1;
  }

  if (!in->params || !in->prover_key || !in->preproc || !in->rng_seed) {
    set_last_error("null input buffer");
    return 1;
  }

  auto total_start = std::chrono::steady_clock::now();

  std::shared_ptr<ParamsKZG> params;
  auto params_start = std::chrono::steady_clock::now();
  if (parse_cache.params && parse_cache.params_ptr == in->params &&
      parse_cache.params_len == in->params_len) {
    params = parse_cache.params;
  } else {
    {
      std::lock_guard<std::mutex> lock(shared_cache_mutex);
      if (shared_cache.params && shared_cache.params_ptr == in->params &&
          shared_cache.params_len == in->params_len) {
        params = shared_cache.params;
      }
    }
    if (!params) {
      auto parsed_params = std::make_shared<ParamsKZG>();
      ParseStatus params_status = parse_params_kzg(in->params, in->params_len, *parsed_params);
      if (!params_status.ok) {
        set_last_error(params_status.error.c_str());
        return 1;
      }
      {
        std::lock_guard<std::mutex> lock(shared_cache_mutex);
        shared_cache.params = parsed_params;
        shared_cache.params_ptr = in->params;
        shared_cache.params_len = in->params_len;
      }
      params = std::move(parsed_params);
    }
    parse_cache.params = params;
    parse_cache.params_ptr = in->params;
    parse_cache.params_len = in->params_len;
  }
  log_timing("parse_params", std::chrono::steady_clock::now() - params_start);

  std::shared_ptr<ProverKey> pk;
  auto pk_start = std::chrono::steady_clock::now();
  if (parse_cache.pk && parse_cache.pk_ptr == in->prover_key &&
      parse_cache.pk_len == in->prover_key_len) {
    pk = parse_cache.pk;
  } else {
    {
      std::lock_guard<std::mutex> lock(shared_cache_mutex);
      if (shared_cache.pk && shared_cache.pk_ptr == in->prover_key &&
          shared_cache.pk_len == in->prover_key_len) {
        pk = shared_cache.pk;
      }
    }
    if (!pk) {
      auto parsed_pk = std::make_shared<ProverKey>();
      ParseStatus pk_status = parse_prover_key(in->prover_key, in->prover_key_len, *parsed_pk);
      if (!pk_status.ok) {
        set_last_error(pk_status.error.c_str());
        return 1;
      }
      {
        std::lock_guard<std::mutex> lock(shared_cache_mutex);
        shared_cache.pk = parsed_pk;
        shared_cache.pk_ptr = in->prover_key;
        shared_cache.pk_len = in->prover_key_len;
      }
      pk = std::move(parsed_pk);
    }
    parse_cache.pk = pk;
    parse_cache.pk_ptr = in->prover_key;
    parse_cache.pk_len = in->prover_key_len;
  }
  log_timing("parse_prover_key", std::chrono::steady_clock::now() - pk_start);

  std::shared_ptr<PackedPolys> polys;
  auto polys_start = std::chrono::steady_clock::now();
  uint64_t polys_hash = 0;
  bool have_polys_hash = false;
  auto maybe_match_polys = [&](const ParseCache &cache) -> bool {
    if (!cache.polys || cache.polys_len != in->preproc_len) {
      return false;
    }
    if (cache.polys_ptr == in->preproc) {
      polys = cache.polys;
      polys_hash = cache.polys_hash;
      have_polys_hash = true;
      return true;
    }
    if (!have_polys_hash) {
      polys_hash = hash_bytes(in->preproc, in->preproc_len);
      have_polys_hash = true;
    }
    if (cache.polys_hash == polys_hash) {
      polys = cache.polys;
      return true;
    }
    return false;
  };

  if (!maybe_match_polys(parse_cache)) {
    {
      std::lock_guard<std::mutex> lock(shared_cache_mutex);
      maybe_match_polys(shared_cache);
    }
  }
  if (!polys) {
    auto parsed_polys = std::make_shared<PackedPolys>();
    ParseStatus polys_status = parse_packed_polys(in->preproc, in->preproc_len, *parsed_polys);
    if (!polys_status.ok) {
      set_last_error(polys_status.error.c_str());
      return 1;
    }
    if (!have_polys_hash) {
      polys_hash = hash_bytes(in->preproc, in->preproc_len);
      have_polys_hash = true;
    }
    {
      std::lock_guard<std::mutex> lock(shared_cache_mutex);
      shared_cache.polys = parsed_polys;
      shared_cache.polys_ptr = in->preproc;
      shared_cache.polys_len = in->preproc_len;
      shared_cache.polys_hash = polys_hash;
    }
    polys = std::move(parsed_polys);
  }
  parse_cache.polys = polys;
  parse_cache.polys_ptr = in->preproc;
  parse_cache.polys_len = in->preproc_len;
  if (have_polys_hash) {
    parse_cache.polys_hash = polys_hash;
  }
  log_timing("parse_packed_polys", std::chrono::steady_clock::now() - polys_start);

  PlonkProof proof;
  auto prove_start = std::chrono::steady_clock::now();
  ProveStatus status = plonk_prove(*params, *pk, *polys, in->rng_seed, 32, proof);
  if (!status.ok) {
    set_last_error(status.error.c_str());
    return 1;
  }
  log_timing("plonk_prove", std::chrono::steady_clock::now() - prove_start);
  log_timing("mn_prove_total", std::chrono::steady_clock::now() - total_start);

  size_t capacity = *out->out_len;
  if (proof.bytes.size() > capacity) {
    set_last_error("output buffer too small");
    return 1;
  }
  std::memcpy(out->out, proof.bytes.data(), proof.bytes.size());
  *out->out_len = proof.bytes.size();
  return 0;
}

extern "C" const char *mn_last_error(void) {
  return last_error.empty() ? "no error" : last_error.c_str();
}

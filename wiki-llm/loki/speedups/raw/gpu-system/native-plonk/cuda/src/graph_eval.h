#pragma once

#include "field.cuh"
#include "params.h"

#include <algorithm>
#include <cstring>
#include <cstdlib>
#include <cstdint>
#include <thread>
#include <vector>

namespace midnight_plonk {

inline bool parallel_enabled() {
  const char *env = std::getenv("MIDNIGHT_CUDA_PARALLEL");
  return !(env && std::strcmp(env, "0") == 0);
}

inline size_t parallel_threads(size_t n, size_t min_chunk) {
  if (!parallel_enabled() || n == 0 || min_chunk == 0) {
    return 1;
  }
  unsigned int hw = std::thread::hardware_concurrency();
  if (hw == 0) {
    hw = 1;
  }
  size_t max_threads = (n + min_chunk - 1) / min_chunk;
  return std::min(static_cast<size_t>(hw), std::max<size_t>(1, max_threads));
}

template <typename F>
inline void parallel_for_chunks(size_t n, size_t min_chunk, F func) {
  size_t threads = parallel_threads(n, min_chunk);
  if (threads <= 1) {
    func(0, n);
    return;
  }
  size_t chunk = (n + threads - 1) / threads;
  std::vector<std::thread> workers;
  workers.reserve(threads - 1);
  for (size_t t = 1; t < threads; ++t) {
    size_t start = t * chunk;
    if (start >= n) {
      break;
    }
    size_t end = std::min(n, start + chunk);
    workers.emplace_back([=, &func]() { func(start, end); });
  }
  func(0, std::min(n, chunk));
  for (auto &worker : workers) {
    worker.join();
  }
}

inline size_t rotation_index(size_t idx, int32_t rot, int32_t rot_scale,
                             int32_t isize) {
  // isize is always a power of two (1 << extended_k), so we can mask instead of modulo.
  uint32_t mask = static_cast<uint32_t>(isize - 1);
  int64_t delta = static_cast<int64_t>(rot) * static_cast<int64_t>(rot_scale);
  uint32_t offset = static_cast<uint32_t>(idx) + static_cast<uint32_t>(delta);
  return static_cast<size_t>(offset & mask);
}

inline Fr value_from_source(
    const GraphValueSource &source, const std::vector<size_t> &rotations,
    const std::vector<Fr> &constants, const std::vector<Fr> &intermediates,
    const std::vector<PolynomialFr> &fixed_values,
    const std::vector<PolynomialFr> &advice_values,
    const std::vector<PolynomialFr> &instance_values,
    const std::vector<Fr> &challenges, const Fr &beta, const Fr &gamma,
    const Fr &theta, const Fr &y, const Fr &previous_value) {
  switch (source.tag) {
    case 0:
      return constants[source.a];
    case 1:
      return intermediates[source.a];
    case 2: {
      size_t idx = rotations[source.b];
      return fixed_values[source.a].values[idx];
    }
    case 3: {
      size_t idx = rotations[source.b];
      return advice_values[source.a].values[idx];
    }
    case 4: {
      size_t idx = rotations[source.b];
      return instance_values[source.a].values[idx];
    }
    case 5:
      return challenges[source.a];
    case 6:
      return beta;
    case 7:
      return gamma;
    case 8:
      return theta;
    case 9:
      return y;
    case 10:
      return previous_value;
    default: {
      Fr zero{};
      return zero;
    }
  }
}

inline Fr evaluate_graph_cached(
    const GraphEvaluator &graph, const std::vector<PolynomialFr> &fixed_values,
    const std::vector<PolynomialFr> &advice_values,
    const std::vector<PolynomialFr> &instance_values,
    const std::vector<Fr> &challenges, const Fr &beta, const Fr &gamma,
    const Fr &theta, const Fr &y, const Fr &previous_value, size_t idx,
    int32_t rot_scale, int32_t isize, std::vector<size_t> &rotation_indices,
    std::vector<Fr> &intermediates);

inline Fr evaluate_graph(
    const GraphEvaluator &graph, const std::vector<PolynomialFr> &fixed_values,
    const std::vector<PolynomialFr> &advice_values,
    const std::vector<PolynomialFr> &instance_values,
    const std::vector<Fr> &challenges, const Fr &beta, const Fr &gamma,
    const Fr &theta, const Fr &y, const Fr &previous_value, size_t idx,
    int32_t rot_scale, int32_t isize) {
  std::vector<size_t> rotation_indices;
  std::vector<Fr> intermediates;
  return evaluate_graph_cached(graph, fixed_values, advice_values, instance_values, challenges,
                               beta, gamma, theta, y, previous_value, idx, rot_scale, isize,
                               rotation_indices, intermediates);
}

inline Fr evaluate_graph_cached(
    const GraphEvaluator &graph, const std::vector<PolynomialFr> &fixed_values,
    const std::vector<PolynomialFr> &advice_values,
    const std::vector<PolynomialFr> &instance_values,
    const std::vector<Fr> &challenges, const Fr &beta, const Fr &gamma,
    const Fr &theta, const Fr &y, const Fr &previous_value, size_t idx,
    int32_t rot_scale, int32_t isize, std::vector<size_t> &rotation_indices,
    std::vector<Fr> &intermediates) {
  if (rotation_indices.size() != graph.rotations.size()) {
    rotation_indices.resize(graph.rotations.size());
  }
  for (size_t i = 0; i < graph.rotations.size(); ++i) {
    rotation_indices[i] = rotation_index(idx, graph.rotations[i], rot_scale, isize);
  }

  if (intermediates.size() != graph.num_intermediates) {
    intermediates.assign(graph.num_intermediates, Fr{});
  } else {
    std::fill(intermediates.begin(), intermediates.end(), Fr{});
  }
  for (const auto &calc : graph.calculations) {
    Fr result{};
    switch (calc.op) {
      case 0: {
        Fr a = value_from_source(calc.lhs, rotation_indices, graph.constants,
                                 intermediates, fixed_values, advice_values,
                                 instance_values, challenges, beta, gamma,
                                 theta, y, previous_value);
        Fr b = value_from_source(calc.rhs, rotation_indices, graph.constants,
                                 intermediates, fixed_values, advice_values,
                                 instance_values, challenges, beta, gamma,
                                 theta, y, previous_value);
        fr_add(result, a, b);
        break;
      }
      case 1: {
        Fr a = value_from_source(calc.lhs, rotation_indices, graph.constants,
                                 intermediates, fixed_values, advice_values,
                                 instance_values, challenges, beta, gamma,
                                 theta, y, previous_value);
        Fr b = value_from_source(calc.rhs, rotation_indices, graph.constants,
                                 intermediates, fixed_values, advice_values,
                                 instance_values, challenges, beta, gamma,
                                 theta, y, previous_value);
        fr_sub(result, a, b);
        break;
      }
      case 2: {
        Fr a = value_from_source(calc.lhs, rotation_indices, graph.constants,
                                 intermediates, fixed_values, advice_values,
                                 instance_values, challenges, beta, gamma,
                                 theta, y, previous_value);
        Fr b = value_from_source(calc.rhs, rotation_indices, graph.constants,
                                 intermediates, fixed_values, advice_values,
                                 instance_values, challenges, beta, gamma,
                                 theta, y, previous_value);
        fr_mul(result, a, b);
        break;
      }
      case 3: {
        Fr a = value_from_source(calc.lhs, rotation_indices, graph.constants,
                                 intermediates, fixed_values, advice_values,
                                 instance_values, challenges, beta, gamma,
                                 theta, y, previous_value);
        fr_mul(result, a, a);
        break;
      }
      case 4: {
        Fr a = value_from_source(calc.lhs, rotation_indices, graph.constants,
                                 intermediates, fixed_values, advice_values,
                                 instance_values, challenges, beta, gamma,
                                 theta, y, previous_value);
        fr_add(result, a, a);
        break;
      }
      case 5: {
        Fr a = value_from_source(calc.lhs, rotation_indices, graph.constants,
                                 intermediates, fixed_values, advice_values,
                                 instance_values, challenges, beta, gamma,
                                 theta, y, previous_value);
        Fr zero{};
        fr_sub(result, zero, a);
        break;
      }
      case 6: {
        Fr value = value_from_source(calc.lhs, rotation_indices, graph.constants,
                                     intermediates, fixed_values, advice_values,
                                     instance_values, challenges, beta, gamma,
                                     theta, y, previous_value);
        Fr factor =
            value_from_source(calc.factor, rotation_indices, graph.constants,
                               intermediates, fixed_values, advice_values,
                               instance_values, challenges, beta, gamma, theta,
                               y, previous_value);
        for (const auto &part : calc.parts) {
          Fr part_value =
              value_from_source(part, rotation_indices, graph.constants,
                                intermediates, fixed_values, advice_values,
                                instance_values, challenges, beta, gamma, theta,
                                y, previous_value);
          Fr tmp;
          fr_mul(tmp, value, factor);
          fr_add(value, tmp, part_value);
        }
        result = value;
        break;
      }
      case 7: {
        result = value_from_source(calc.lhs, rotation_indices, graph.constants,
                                   intermediates, fixed_values, advice_values,
                                   instance_values, challenges, beta, gamma,
                                   theta, y, previous_value);
        break;
      }
      default: {
        Fr zero{};
        result = zero;
        break;
      }
    }
    if (calc.target < intermediates.size()) {
      intermediates[calc.target] = result;
    }
  }

  if (!graph.calculations.empty()) {
    const auto &last = graph.calculations.back();
    if (last.target < intermediates.size()) {
      return intermediates[last.target];
    }
  }
  Fr zero{};
  return zero;
}

inline std::vector<Fr> evaluate_graph_values(
    const GraphEvaluator &graph, const std::vector<PolynomialFr> &fixed_values,
    const std::vector<PolynomialFr> &advice_values,
    const std::vector<PolynomialFr> &instance_values,
    const std::vector<Fr> &challenges, const Fr &beta, const Fr &gamma,
    const Fr &theta, const Fr &y, const Fr &previous_value, size_t size,
    int32_t rot_scale, int32_t isize) {
  std::vector<Fr> values(size);
  constexpr size_t kMinChunk = 1024;
  auto worker = [&](size_t start, size_t end) {
    std::vector<size_t> rotation_indices;
    std::vector<Fr> intermediates;
    for (size_t idx = start; idx < end; ++idx) {
      values[idx] = evaluate_graph_cached(graph, fixed_values, advice_values, instance_values,
                                          challenges, beta, gamma, theta, y, previous_value, idx,
                                          rot_scale, isize, rotation_indices, intermediates);
    }
  };
  parallel_for_chunks(size, kMinChunk, worker);
  return values;
}

}  // namespace midnight_plonk

#pragma once

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  const uint8_t *params;
  size_t params_len;
  const uint8_t *prover_key;
  size_t prover_key_len;
  const uint8_t *preproc;
  size_t preproc_len;
  const uint8_t *rng_seed;
} mn_prove_input;

typedef struct {
  uint8_t *out;
  size_t *out_len;
} mn_prove_output;

int mn_prove(const mn_prove_input *in, mn_prove_output *out);
const char *mn_last_error(void);
int mn_kzg_commit_lagrange(const uint8_t *params, size_t params_len,
                           const uint8_t *values, size_t values_len,
                           uint8_t *out, size_t out_len);

#ifdef __cplusplus
}
#endif

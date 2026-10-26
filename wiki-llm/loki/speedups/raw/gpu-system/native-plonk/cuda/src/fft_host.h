#pragma once

#include "fft.cuh"
#include "field.cuh"
#include "poly.h"

#include <cuda_runtime.h>
#include <string>
#include <vector>

namespace midnight_plonk {

struct FftTwiddleCache {
  size_t n = 0;
  Fr omega;
  std::vector<size_t> m_values;
  std::vector<Fr *> d_twiddles;
};

static thread_local FftTwiddleCache fft_cache;
inline bool fr_equal_host(const Fr &a, const Fr &b) {
  return a.limbs[0] == b.limbs[0] && a.limbs[1] == b.limbs[1] && a.limbs[2] == b.limbs[2] &&
         a.limbs[3] == b.limbs[3];
}

inline void clear_fft_cache() {
  for (auto *ptr : fft_cache.d_twiddles) {
    cudaFree(ptr);
  }
  fft_cache.d_twiddles.clear();
  fft_cache.m_values.clear();
  fft_cache.n = 0;
}

inline bool ensure_fft_cache(size_t n, const Fr &omega, std::string &error) {
  if (fft_cache.n == n && fr_equal_host(fft_cache.omega, omega)) {
    return true;
  }
  clear_fft_cache();
  fft_cache.n = n;
  fft_cache.omega = omega;

  for (size_t m = 1; m < n; m <<= 1) {
    std::vector<Fr> twiddles(m);
    size_t stride = n / (2 * m);
    for (size_t j = 0; j < m; ++j) {
      twiddles[j] = fr_pow_u64(omega, static_cast<uint64_t>(stride * j));
    }

    Fr *d_twiddles = nullptr;
    if (cudaMalloc(&d_twiddles, m * sizeof(Fr)) != cudaSuccess) {
      error = "cudaMalloc failed in fft_in_place twiddles";
      clear_fft_cache();
      return false;
    }
    if (cudaMemcpy(d_twiddles, twiddles.data(), m * sizeof(Fr), cudaMemcpyHostToDevice) !=
        cudaSuccess) {
      error = "cudaMemcpy failed in fft_in_place twiddles";
      cudaFree(d_twiddles);
      clear_fft_cache();
      return false;
    }
    fft_cache.m_values.push_back(m);
    fft_cache.d_twiddles.push_back(d_twiddles);
  }
  return true;
}

inline bool fft_in_place(std::vector<Fr> &values, const Fr &omega, std::string &error) {
  if (values.empty()) {
    return true;
  }
  size_t n = values.size();
  if ((n & (n - 1)) != 0) {
    error = "fft_in_place requires power-of-two length";
    return false;
  }

  size_t log_n = 0;
  while ((1ULL << log_n) < n) {
    ++log_n;
  }
  auto bit_reverse = [log_n](size_t value) -> size_t {
    size_t out = 0;
    for (size_t i = 0; i < log_n; ++i) {
      out = (out << 1) | (value & 1);
      value >>= 1;
    }
    return out;
  };
  std::vector<Fr> reordered(n);
  for (size_t i = 0; i < n; ++i) {
    reordered[bit_reverse(i)] = values[i];
  }
  values.swap(reordered);

  Fr *d_values = nullptr;
  if (cudaMalloc(&d_values, n * sizeof(Fr)) != cudaSuccess) {
    error = "cudaMalloc failed in fft_in_place";
    return false;
  }
  if (cudaMemcpy(d_values, values.data(), n * sizeof(Fr), cudaMemcpyHostToDevice) != cudaSuccess) {
    error = "cudaMemcpy failed in fft_in_place";
    cudaFree(d_values);
    return false;
  }

  if (!ensure_fft_cache(n, omega, error)) {
    cudaFree(d_values);
    return false;
  }

  for (size_t stage = 0; stage < fft_cache.m_values.size(); ++stage) {
    size_t m = fft_cache.m_values[stage];
    Fr *d_twiddles = fft_cache.d_twiddles[stage];
    int threads = 256;
    int total = static_cast<int>(n / 2);
    int blocks = (total + threads - 1) / threads;
    fft_stage_kernel<<<blocks, threads>>>(d_values, d_twiddles, static_cast<int>(n),
                                          static_cast<int>(m));
    if (cudaGetLastError() != cudaSuccess) {
      error = "fft_stage_kernel launch failed";
      cudaFree(d_values);
      return false;
    }
  }
  cudaDeviceSynchronize();

  if (cudaMemcpy(values.data(), d_values, n * sizeof(Fr), cudaMemcpyDeviceToHost) !=
      cudaSuccess) {
    error = "cudaMemcpy failed in fft_in_place";
    cudaFree(d_values);
    return false;
  }
  cudaFree(d_values);

  return true;
}

inline bool fft_in_place_device(Fr *d_values, size_t n, const Fr &omega, std::string &error) {
  if (!d_values || n == 0) {
    return true;
  }
  if ((n & (n - 1)) != 0) {
    error = "fft_in_place_device requires power-of-two length";
    return false;
  }
  if (!ensure_fft_cache(n, omega, error)) {
    return false;
  }
  for (size_t stage = 0; stage < fft_cache.m_values.size(); ++stage) {
    size_t m = fft_cache.m_values[stage];
    Fr *d_twiddles = fft_cache.d_twiddles[stage];
    int threads = 256;
    int total = static_cast<int>(n / 2);
    int blocks = (total + threads - 1) / threads;
    fft_stage_kernel<<<blocks, threads>>>(d_values, d_twiddles, static_cast<int>(n),
                                          static_cast<int>(m));
    if (cudaGetLastError() != cudaSuccess) {
      error = "fft_stage_kernel launch failed";
      return false;
    }
  }
  if (cudaDeviceSynchronize() != cudaSuccess) {
    error = "fft_stage_kernel sync failed";
    return false;
  }
  return true;
}

inline bool ifft_in_place(std::vector<Fr> &values, const Fr &omega_inv, const Fr &divisor,
                          std::string &error) {
  if (!fft_in_place(values, omega_inv, error)) {
    return false;
  }
  for (auto &value : values) {
    fr_mul(value, value, divisor);
  }
  return true;
}

}  // namespace midnight_plonk

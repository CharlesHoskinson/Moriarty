# Pipelonk NTT segmentation and benchmark
> Source: https://raw.githubusercontent.com/zlyber/pipelonk/main/ntt.cu
> Collected: 2026-10-26
> Published: Unknown

```text
#include <iostream>
#include <cstdint>
#include "caffe/interface.hpp"
#include "PLONK/utils/function.cuh"
#include "PLONK/utils/zkp/cuda/zksnark_ntt/ntt_kernel/ntt.cuh"
#include <thread>
#include <atomic>
#include <cmath>
#include <cstdlib>
#include <mutex>
#include <cstring>
#include <iomanip>
#include <string>
#include <utility>
#include <algorithm>
#include <cassert>
#include <condition_variable>
#include <vector>
#include <sstream>
#include <memory>

static_assert(sizeof(cpu::BLS12_381_Fr_G1) == 4 * sizeof(uint64_t),
              "field element layout assumes four 64-bit limbs");

void read_file(const char* filename, void* data){
   // 打开文件
   std::ifstream file(filename, std::ios::binary);
   if (!file) {
    std::cerr << "Error opening file: " << filename << std::endl;
    return;
   }
   file.seekg(0, std::ios::end);
   size_t fileSize = file.tellg();
   file.seekg(0, std::ios::beg);

   file.read(reinterpret_cast<char*>(data), fileSize);
   if (!file) {
    std::cerr << "Error reading file: " << filename << std::endl;
   }
   file.close();
}

class CpuThreadPool {
public:
    explicit CpuThreadPool(int num_threads) : worker_count_(std::max(1, num_threads)) {
        if (worker_count_ <= 1) {
            return;
        }
        workers_.reserve(worker_count_);
        for (int id = 0; id < worker_count_; ++id) {
            workers_.emplace_back([this, id]() { worker_loop(id); });
        }
    }

    ~CpuThreadPool() {
        {
            std::lock_guard<std::mutex> lock(mutex_);
            stop_ = true;
            ++generation_;
        }
        task_cv_.notify_all();
        for (auto& worker : workers_) {
            worker.join();
        }
    }

    int worker_count() const {
        return worker_count_;
    }

    template <typename Fn>
    void parallel_for(uint64_t total, Fn fn) {
        if (worker_count_ <= 1 || total < 4096) {
            fn(0, total);
            return;
        }

        std::vector<std::pair<uint64_t, uint64_t>> ranges(worker_count_);
        const uint64_t work_per_worker = total / worker_count_;
        for (int id = 0; id < worker_count_; ++id) {
            const uint64_t begin = id * work_per_worker;
            const uint64_t end = (id == worker_count_ - 1) ? total : begin + work_per_worker;
            ranges[id] = {begin, end};
        }

        {
            std::lock_guard<std::mutex> lock(mutex_);
            pending_workers_ = worker_count_;
            task_ = [&](int worker_id) {
                const auto [begin, end] = ranges[worker_id];
                if (begin < end) {
                    fn(begin, end);
                }
            };
            ++generation_;
        }
        task_cv_.notify_all();

        std::unique_lock<std::mutex> lock(mutex_);
        done_cv_.wait(lock, [&]() { return pending_workers_ == 0; });
    }

private:
    void worker_loop(int worker_id) {
        uint64_t seen_generation = 0;
        while (true) {
            std::function<void(int)> task;
            {
                std::unique_lock<std::mutex> lock(mutex_);
                task_cv_.wait(lock, [&]() { return stop_ || generation_ != seen_generation; });
                if (stop_) {
                    return;
                }
                seen_generation = generation_;
                task = task_;
            }

            task(worker_id);

            {
                std::lock_guard<std::mutex> lock(mutex_);
                --pending_workers_;
                if (pending_workers_ == 0) {
                    done_cv_.notify_one();
                }
            }
        }
    }

    int worker_count_;
    std::vector<std::thread> workers_;
    std::mutex mutex_;
    std::condition_variable task_cv_;
    std::condition_variable done_cv_;
    std::function<void(int)> task_;
    uint64_t generation_ = 0;
    int pending_workers_ = 0;
    bool stop_ = false;
};

enum class StageMapPolicy {
    automatic,
    disabled,
    enabled
};

struct StageIndexMap {
    uint64_t chunk_elems = 0;
    uint64_t chunk_count = 0;
    int stage = 0;
    int iterations = 0;
    std::vector<uint32_t> input;
    std::vector<uint32_t> output;

    bool matches(uint64_t elems,
                 uint64_t chunks,
                 int stage_value,
                 int iteration_count) const {
        return chunk_elems == elems &&
               chunk_count == chunks &&
               stage == stage_value &&
               iterations == iteration_count &&
               input.size() == elems * chunks &&
               output.size() == elems * chunks;
    }
};

void ct_stage_indices(uint64_t tid, int stage, int iterations, uint64_t& idx0, uint64_t& idx1) {
    const uint64_t inp_ntt_size = uint64_t{1} << stage;
    const uint64_t out_ntt_size = uint64_t{1} << (stage + iterations - 1);
    const uint64_t thread_ntt_pos = (tid & (out_ntt_size - 1)) >> (iterations - 1);

    idx0 = tid & ~(out_ntt_size - 1);
    idx0 += (tid << stage) & (out_ntt_size - 1);
    idx0 = idx0 * 2 + thread_ntt_pos;
    idx1 = idx0 + inp_ntt_size;
}

void local_stage_indices(uint64_t tid, int iterations, uint64_t& idx0, uint64_t& idx1) {
    const uint64_t out_ntt_size = uint64_t{1} << (iterations - 1);
    const uint64_t thread_ntt_pos = (tid & (out_ntt_size - 1)) >> (iterations - 1);

    idx0 = tid & ~(out_ntt_size - 1);
    idx0 += tid & (out_ntt_size - 1);
    idx0 = idx0 * 2 + thread_ntt_pos;
    idx1 = idx0 + 1;
}

uint64_t rotate_stage_output_index(uint64_t global_idx, int stage, int iterations) {
    const uint64_t mask = ((uint64_t{1} << (stage + iterations)) - (uint64_t{1} << stage));
    uint64_t rotated = global_idx & mask;
    rotated = (rotated >> 1) | (rotated << (iterations - 1));
    return (global_idx & ~mask) | (rotated & mask);
}

StageIndexMap build_stage_index_map(uint64_t chunk_elems,
                                    uint64_t chunk_count,
                                    int stage,
                                    int iterations,
                                    int num_threads) {
    StageIndexMap map;
    map.chunk_elems = chunk_elems;
    map.chunk_count = chunk_count;
    map.stage = stage;
    map.iterations = iterations;
    map.input.resize(chunk_elems * chunk_count);
    map.output.resize(chunk_elems * chunk_count);

    const uint64_t threads_per_chunk = chunk_elems / 2;
    CpuThreadPool pool(num_threads);
    pool.parallel_for(chunk_count * threads_per_chunk, [&](uint64_t begin, uint64_t end) {
        for (uint64_t flat_tid = begin; flat_tid < end; ++flat_tid) {
            const uint64_t chunk = flat_tid / threads_per_chunk;
            const uint64_t local_tid = flat_tid - chunk * threads_per_chunk;
            const uint64_t map_base = chunk * chunk_elems;

            uint64_t global0, global1;
            uint64_t local0, local1;
            ct_stage_indices(flat_tid, stage, iterations, global0, global1);
            local_stage_indices(local_tid, iterations, local0, local1);

            map.input[map_base + local0] = static_cast<uint32_t>(global0);
            map.input[map_base + local1] = static_cast<uint32_t>(global1);
            map.output[map_base + local0] =
                static_cast<uint32_t>(rotate_stage_output_index(global0, stage, iterations));
            map.output[map_base + local1] =
                static_cast<uint32_t>(rotate_stage_output_index(global1, stage, iterations));
        }
    });

    return map;
}

template <typename fr>
void gather_stage_input_map_range(fr* dst,
                                  const fr* src,
                                  const uint32_t* map,
                                  uint64_t begin,
                                  uint64_t end) {
    for (uint64_t i = begin; i < end; ++i) {
        dst[i] = src[map[i]];
    }
}

template <typename fr>
void gather_stage_input_map_parallel(fr* dst,
                                     const fr* src,
                                     const StageIndexMap& map,
                                     uint64_t chunk,
                                     int num_threads,
                                     CpuThreadPool* pool = nullptr) {
    const uint32_t* chunk_map = map.input.data() + chunk * map.chunk_elems;
    if (num_threads <= 1 || map.chunk_elems < 4096) {
        gather_stage_input_map_range(dst, src, chunk_map, 0, map.chunk_elems);
        return;
    }

    if (pool != nullptr && pool->worker_count() == num_threads) {
        pool->parallel_for(map.chunk_elems, [&](uint64_t begin, uint64_t end) {
            gather_stage_input_map_range(dst, src, chunk_map, begin, end);
        });
        return;
    }

    CpuThreadPool local_pool(num_threads);
    local_pool.parallel_for(map.chunk_elems, [&](uint64_t begin, uint64_t end) {
        gather_stage_input_map_range(dst, src, chunk_map, begin, end);
    });
}

template <typename fr>
void scatter_stage_output_map_range(fr* dst,
                                    const fr* src,
                                    const uint32_t* map,
                                    uint64_t begin,
                                    uint64_t end) {
    for (uint64_t i = begin; i < end; ++i) {
        dst[map[i]] = src[i];
    }
}

template <typename fr>
void scatter_stage_output_map_parallel(fr* dst,
                                       const fr* src,
                                       const StageIndexMap& map,
                                       uint64_t chunk,
                                       int num_threads,
                                       CpuThreadPool* pool = nullptr) {
    const uint32_t* chunk_map = map.output.data() + chunk * map.chunk_elems;
    if (num_threads <= 1 || map.chunk_elems < 4096) {
        scatter_stage_output_map_range(dst, src, chunk_map, 0, map.chunk_elems);
        return;
    }

    if (pool != nullptr && pool->worker_count() == num_threads) {
        pool->parallel_for(map.chunk_elems, [&](uint64_t begin, uint64_t end) {
            scatter_stage_output_map_range(dst, src, chunk_map, begin, end);
        });
        return;
    }

    CpuThreadPool local_pool(num_threads);
    local_pool.parallel_for(map.chunk_elems, [&](uint64_t begin, uint64_t end) {
        scatter_stage_output_map_range(dst, src, chunk_map, begin, end);
    });
}

template <typename fr>
void transpose_matrix_range(const fr* input,
                            fr* output,
                            uint64_t rows,
                            uint64_t cols,
                            uint64_t begin,
                            uint64_t end) {
    for (uint64_t idx = begin; idx < end; ++idx) {
        const uint64_t row = idx / cols;
        const uint64_t col = idx - row * cols;
        output[col * rows + row] = input[idx];
    }
}

template <typename fr>
void transpose_matrix_parallel(const fr* input,
                               fr* output,
                               uint64_t rows,
                               uint64_t cols,
                               int num_threads) {
    const uint64_t total = rows * cols;
    CpuThreadPool pool(num_threads);
    pool.parallel_for(total, [&](uint64_t begin, uint64_t end) {
        transpose_matrix_range(input, output, rows, cols, begin, end);
    });
}

template <typename fr>
void permute_stage_input_chunk_range(fr* dst,
                                     const fr* src,
                                     uint64_t chunk,
                                     uint64_t chunk_elems,
                                     int stage,
                                     int iterations,
                                     uint64_t begin,
                                     uint64_t end) {
    const uint64_t tid_base = chunk * (chunk_elems / 2);
    for (uint64_t local_tid = begin; local_tid < end; ++local_tid) {
        uint64_t global0, global1;
        uint64_t local0, local1;
        ct_stage_indices(tid_base + local_tid, stage, iterations, global0, global1);
        local_stage_indices(local_tid, iterations, local0, local1);

        dst[local0] = src[global0];
        dst[local1] = src[global1];
    }
}

template <typename fr>
void permute_stage_input_chunk_parallel(fr* dst,
                                        const fr* src,
                                        uint64_t chunk,
                                        uint64_t chunk_elems,
                                        int stage,
                                        int iterations,
                                        int num_threads,
                                        CpuThreadPool* pool = nullptr) {
    const uint64_t threads_per_chunk = chunk_elems / 2;
    if (num_threads <= 1 || threads_per_chunk < 4096) {
        permute_stage_input_chunk_range(dst, src, chunk, chunk_elems,
                                        stage, iterations, 0, threads_per_chunk);
        return;
    }

    if (pool != nullptr && pool->worker_count() == num_threads) {
        pool->parallel_for(threads_per_chunk, [&](uint64_t begin, uint64_t end) {
            permute_stage_input_chunk_range(dst, src, chunk, chunk_elems,
                                            stage, iterations, begin, end);
        });
        return;
    }

    CpuThreadPool local_pool(num_threads);
    local_pool.parallel_for(threads_per_chunk, [&](uint64_t begin, uint64_t end) {
        permute_stage_input_chunk_range(dst, src, chunk, chunk_elems,
                                        stage, iterations, begin, end);
    });
}

__device__ uint64_t reverse_bits_device(uint64_t value, int bits) {
    uint64_t reversed = 0;
    for (int bit = 0; bit < bits; ++bit) {
        reversed = (reversed << 1) | (value & 1);
        value >>= 1;
    }
    return reversed;
}

__global__ void block_bit_reverse_fields_kernel(uint64_t* data,
                                                uint64_t block_elems,
                                                uint64_t batch,
                                                int lg_block_elems) {
    const uint64_t idx = blockIdx.x * uint64_t(blockDim.x) + threadIdx.x;
    const uint64_t total = block_elems * batch;
    if (idx >= total) {
        return;
    }

    const uint64_t block = idx / block_elems;
    const uint64_t local = idx - block * block_elems;
    const uint64_t rev = reverse_bits_device(local, lg_block_elems);
    if (local >= rev) {
        return;
    }

    const uint64_t a = (block * block_elems + local) * 4;
    const uint64_t b = (block * block_elems + rev) * 4;
    uint64_t tmp0 = data[a];
    uint64_t tmp1 = data[a + 1];
    uint64_t tmp2 = data[a + 2];
    uint64_t tmp3 = data[a + 3];

    data[a] = data[b];
    data[a + 1] = data[b + 1];
    data[a + 2] = data[b + 2];
    data[a + 3] = data[b + 3];

    data[b] = tmp0;
    data[b + 1] = tmp1;
    data[b + 2] = tmp2;
    data[b + 3] = tmp3;
}

void launch_block_bit_reverse(void* data,
                              uint64_t block_elems,
                              uint64_t batch,
                              int lg_block_elems,
                              cudaStream_t stream) {
    const uint64_t total = block_elems * batch;
    const int block_size = 256;
    const uint64_t grid_size = (total + block_size - 1) / block_size;
    block_bit_reverse_fields_kernel<<<static_cast<unsigned int>(grid_size),
                                      block_size, 0, stream>>>(
        reinterpret_cast<uint64_t*>(data),
        block_elems,
        batch,
        lg_block_elems);
}

__global__ void transpose_fields_kernel(uint64_t* dst,
                                        const uint64_t* src,
                                        uint64_t rows,
                                        uint64_t cols) {
    const uint64_t idx = blockIdx.x * uint64_t(blockDim.x) + threadIdx.x;
    const uint64_t total = rows * cols;
    if (idx >= total) {
        return;
    }

    const uint64_t row = idx / cols;
    const uint64_t col = idx - row * cols;
    const uint64_t out = col * rows + row;
    const uint64_t src_word = idx * 4;
    const uint64_t dst_word = out * 4;

    dst[dst_word] = src[src_word];
    dst[dst_word + 1] = src[src_word + 1];
    dst[dst_word + 2] = src[src_word + 2];
    dst[dst_word + 3] = src[src_word + 3];
}

void launch_transpose_fields(void* dst,
                             const void* src,
                             uint64_t rows,
                             uint64_t cols,
                             cudaStream_t stream) {
    const uint64_t total = rows * cols;
    const int block_size = 256;
    const uint64_t grid_size = (total + block_size - 1) / block_size;
    transpose_fields_kernel<<<static_cast<unsigned int>(grid_size),
                              block_size, 0, stream>>>(
        reinterpret_cast<uint64_t*>(dst),
        reinterpret_cast<const uint64_t*>(src),
        rows,
        cols);
}

__global__ void standard4_twiddle_kernel(cuda::fr* data,
                                         uint64_t rows,
                                         uint64_t cols,
                                         uint64_t col_base,
                                         int lg_N,
                                         const cuda::fr* partial_twiddles) {
    const uint64_t idx = blockIdx.x * uint64_t(blockDim.x) + threadIdx.x;
    const uint64_t total = rows * cols;
    if (idx >= total) {
        return;
    }

    const uint64_t local_col = idx / rows;
    const uint64_t row = idx - local_col * rows;
    const uint64_t col = col_base + local_col;
    const uint64_t shift = MAX_LG_DOMAIN_SIZE - lg_N;
    const uint64_t exponent = (row * col) << shift;
    data[idx] *= cuda::get_intermediate_root(static_cast<cuda::index_t>(exponent),
                                             partial_twiddles);
}

void launch_standard4_twiddle(void* data,
                              uint64_t rows,
                              uint64_t cols,
                              uint64_t col_base,
                              int lg_N,
                              Ntt& ntt,
                              cudaStream_t stream) {
    auto* params = reinterpret_cast<cuda::fr*>(ntt.Params.mutable_gpu_data());
    cuda::fr* partial_twiddles = params;
    const uint64_t total = rows * cols;
    const int block_size = 256;
    const uint64_t grid_size = (total + block_size - 1) / block_size;
    standard4_twiddle_kernel<<<static_cast<unsigned int>(grid_size),
                               block_size, 0, stream>>>(
        reinterpret_cast<cuda::fr*>(data),
        rows,
        cols,
        col_base,
        lg_N,
                               partial_twiddles);
}

__global__ void batched_ct_stage_kernel(cuda::fr* data,
                                        uint64_t len,
                                        uint64_t batch,
                                        int lg_len,
                                        int stage,
                                        const cuda::fr* partial_twiddles) {
    const uint64_t idx = blockIdx.x * uint64_t(blockDim.x) + threadIdx.x;
    const uint64_t pairs_per_batch = len / 2;
    const uint64_t total = pairs_per_batch * batch;
    if (idx >= total) {
        return;
    }

    const uint64_t item = idx / pairs_per_batch;
    const uint64_t pair = idx - item * pairs_per_batch;
    const uint64_t half = uint64_t{1} << stage;
    const uint64_t m = half << 1;
    const uint64_t group = pair / half;
    const uint64_t j = pair - group * half;
    const uint64_t base = item * len + group * m + j;
    const uint64_t shift = MAX_LG_DOMAIN_SIZE - lg_len;
    const uint64_t exponent = (j << (lg_len - stage - 1)) << shift;

    cuda::fr root = cuda::get_intermediate_root(static_cast<cuda::index_t>(exponent),
                                                partial_twiddles);
    cuda::fr u = data[base];
    cuda::fr v = data[base + half] * root;
    data[base] = u + v;
    data[base + half] = u - v;
}

void launch_batched_ct_stage(void* data,
                             uint64_t batch,
                             uint64_t len,
                             int lg_len,
                             int stage,
                             Ntt& ntt,
                             cudaStream_t stream) {
    auto* params = reinterpret_cast<cuda::fr*>(ntt.Params.mutable_gpu_data());
    cuda::fr* partial_twiddles = params;
    const uint64_t total = batch * (len / 2);
    const int block_size = 256;
    const uint64_t grid_size = (total + block_size - 1) / block_size;
    batched_ct_stage_kernel<<<static_cast<unsigned int>(grid_size),
                              block_size, 0, stream>>>(
        reinterpret_cast<cuda::fr*>(data),
        len,
        batch,
        lg_len,
        stage,
        partial_twiddles);
}

void run_batched_simple_ntt(void* data,
                            uint64_t batch,
                            uint64_t len,
                            int lg_len,
                            Ntt& ntt,
                            cudaStream_t stream) {
    launch_block_bit_reverse(data, len, batch, lg_len, stream);
    for (int stage = 0; stage < lg_len; ++stage) {
        launch_batched_ct_stage(data, batch, len, lg_len, stage, ntt, stream);
    }
}

template <typename fr>
void scatter_stage_output_chunk_range(fr* dst,
                                      const fr* src,
                                      uint64_t chunk,
                                      uint64_t chunk_elems,
                                      int stage,
                                      int iterations,
                                      uint64_t begin,
                                      uint64_t end) {
    for (uint64_t local_tid = begin; local_tid < end; ++local_tid) {
        uint64_t global0, global1;
        uint64_t local0, local1;
        ct_stage_indices(chunk * (chunk_elems / 2) + local_tid, stage, iterations, global0, global1);
        local_stage_indices(local_tid, iterations, local0, local1);

        const uint64_t out0 = rotate_stage_output_index(global0, stage, iterations);
        const uint64_t out1 = rotate_stage_output_index(global1, stage, iterations);

        dst[out0] = src[local0];
        dst[out1] = src[local1];
    }
}

template <typename fr>
void scatter_stage_output_chunk_parallel(fr* dst,
                                         const fr* src,
                                         uint64_t chunk,
                                         uint64_t chunk_elems,
                                         int stage,
                                         int iterations,
                                         int num_threads,
                                         CpuThreadPool* pool = nullptr) {
    const uint64_t threads_per_chunk = chunk_elems / 2;
    if (num_threads <= 1 || threads_per_chunk < 4096) {
        scatter_stage_output_chunk_range(dst, src, chunk, chunk_elems,
                                         stage, iterations, 0, threads_per_chunk);
        return;
    }

    if (pool != nullptr && pool->worker_count() == num_threads) {
        pool->parallel_for(threads_per_chunk, [&](uint64_t begin, uint64_t end) {
            scatter_stage_output_chunk_range(dst, src, chunk, chunk_elems,
                                             stage, iterations, begin, end);
        });
        return;
    }

    CpuThreadPool local_pool(num_threads);
    local_pool.parallel_for(threads_per_chunk, [&](uint64_t begin, uint64_t end) {
        scatter_stage_output_chunk_range(dst, src, chunk, chunk_elems,
                                         stage, iterations, begin, end);
    });
}

__global__ void scatter_stage_kernel(uint64_t* dst,
                                     const uint64_t* src,
                                     uint64_t chunk,
                                     uint64_t chunk_elems,
                                     int stage,
                                     int iterations) {
    uint64_t local_tid = blockIdx.x * uint64_t(blockDim.x) + threadIdx.x;
    const uint64_t threads_per_chunk = chunk_elems / 2;
    if (local_tid >= threads_per_chunk) {
        return;
    }

    const uint64_t global_tid = chunk * threads_per_chunk + local_tid;
    const uint64_t out_ntt_size = uint64_t{1} << (stage + iterations - 1);
    const uint64_t inp_ntt_size = uint64_t{1} << stage;
    const uint64_t thread_ntt_pos = (global_tid & (out_ntt_size - 1)) >> (iterations - 1);

    uint64_t global0 = global_tid & ~(out_ntt_size - 1);
    global0 += (global_tid << stage) & (out_ntt_size - 1);
    global0 = global0 * 2 + thread_ntt_pos;
    uint64_t global1 = global0 + inp_ntt_size;

    const uint64_t local_out_ntt_size = uint64_t{1} << (iterations - 1);
    const uint64_t local_thread_ntt_pos = (local_tid & (local_out_ntt_size - 1)) >> (iterations - 1);

    uint64_t local0 = local_tid & ~(local_out_ntt_size - 1);
    local0 += local_tid & (local_out_ntt_size - 1);
    local0 = local0 * 2 + local_thread_ntt_pos;
    uint64_t local1 = local0 + 1;

    uint64_t mask = ((uint64_t{1} << (stage + iterations)) - (uint64_t{1} << stage));
    uint64_t rotw0 = global0 & mask;
    rotw0 = (rotw0 >> 1) | (rotw0 << (iterations - 1));
    uint64_t out0 = ((global0 & ~mask) | (rotw0 & mask)) - chunk * chunk_elems;

    uint64_t rotw1 = global1 & mask;
    rotw1 = (rotw1 >> 1) | (rotw1 << (iterations - 1));
    uint64_t out1 = ((global1 & ~mask) | (rotw1 & mask)) - chunk * chunk_elems;

    const uint64_t src0 = local0 * 4;
    const uint64_t src1 = local1 * 4;
    const uint64_t dst0 = out0 * 4;
    const uint64_t dst1 = out1 * 4;

    dst[dst0] = src[src0];
    dst[dst0 + 1] = src[src0 + 1];
    dst[dst0 + 2] = src[src0 + 2];
    dst[dst0 + 3] = src[src0 + 3];

    dst[dst1] = src[src1];
    dst[dst1 + 1] = src[src1 + 1];
    dst[dst1 + 2] = src[src1 + 2];
    dst[dst1 + 3] = src[src1 + 3];
}

template <int tile_dim>
__global__ void pack_final_stage_rows_kernel(ulonglong4* dst,
                                             const ulonglong4* src,
                                             uint64_t rows_half,
                                             uint64_t cols_per_chunk) {
    __shared__ ulonglong4 tile[tile_dim][tile_dim + 1];

    const uint64_t col_tile = blockIdx.x * uint64_t{tile_dim};
    const uint64_t row_tile = blockIdx.y * uint64_t{tile_dim};
    const uint64_t half = blockIdx.z;

    const uint64_t read_col = col_tile + threadIdx.y;
    const uint64_t read_row = row_tile + threadIdx.x;
    if (read_col < cols_per_chunk && read_row < rows_half) {
        const uint64_t src_idx = (read_col * rows_half + read_row) * 2 + half;
        tile[threadIdx.y][threadIdx.x] = src[src_idx];
    }
    __syncthreads();

    const uint64_t write_col = col_tile + threadIdx.x;
    const uint64_t write_row = row_tile + threadIdx.y;
    if (write_col < cols_per_chunk && write_row < rows_half) {
        const uint64_t dst_idx = (half * rows_half + write_row) * cols_per_chunk + write_col;
        dst[dst_idx] = tile[threadIdx.x][threadIdx.y];
    }
}

bool should_use_gpu_final_pack(uint64_t chunk_elems,
                               int stage,
                               int iterations,
                               int lg_N) {
    if (stage + iterations != lg_N || iterations <= 0 || iterations >= 32) {
        return false;
    }
    constexpr size_t threshold_bytes = size_t{2} << 20;
    return chunk_elems * sizeof(cpu::BLS12_381_Fr_G1) >= threshold_bytes;
}

void launch_pack_final_stage_rows(void* packed,
                                  const void* stage_output,
                                  uint64_t rows_half,
                                  uint64_t cols_per_chunk,
                                  cudaStream_t stream) {
    constexpr int tile_dim = 16;
    const dim3 block(tile_dim, tile_dim);
    const dim3 grid(static_cast<unsigned int>((cols_per_chunk + tile_dim - 1) / tile_dim),
                    static_cast<unsigned int>((rows_half + tile_dim - 1) / tile_dim),
                    2);
    pack_final_stage_rows_kernel<tile_dim>
        <<<grid, block, 0, stream>>>(
            reinterpret_cast<ulonglong4*>(packed),
            reinterpret_cast<const ulonglong4*>(stage_output),
            rows_half,
            cols_per_chunk);
    CUDA_CHECK(cudaGetLastError());
}

__global__ void transition_stage_kernel(uint64_t* dst,
                                        const uint64_t* src,
                                        uint64_t chunk,
                                        uint64_t chunk_elems,
                                        int prev_stage,
                                        int prev_iterations,
                                        int next_stage,
                                        int next_iterations) {
    uint64_t local_tid = blockIdx.x * uint64_t(blockDim.x) + threadIdx.x;
    const uint64_t threads_per_chunk = chunk_elems / 2;
    if (local_tid >= threads_per_chunk) {
        return;
    }

    const uint64_t global_tid = chunk * threads_per_chunk + local_tid;
    const uint64_t out_ntt_size = uint64_t{1} << (next_stage + next_iterations - 1);
    const uint64_t inp_ntt_size = uint64_t{1} << next_stage;
    const uint64_t thread_ntt_pos = (global_tid & (out_ntt_size - 1)) >> (next_iterations - 1);

    uint64_t global0 = global_tid & ~(out_ntt_size - 1);
    global0 += (global_tid << next_stage) & (out_ntt_size - 1);
    global0 = global0 * 2 + thread_ntt_pos;
    uint64_t global1 = global0 + inp_ntt_size;

    const uint64_t local_out_ntt_size = uint64_t{1} << (next_iterations - 1);
    const uint64_t local_thread_ntt_pos = (local_tid & (local_out_ntt_size - 1)) >> (next_iterations - 1);

    uint64_t local0 = local_tid & ~(local_out_ntt_size - 1);
    local0 += local_tid & (local_out_ntt_size - 1);
    local0 = local0 * 2 + local_thread_ntt_pos;
    uint64_t local1 = local0 + 1;

    uint64_t mask = ((uint64_t{1} << (prev_stage + prev_iterations)) - (uint64_t{1} << prev_stage));
    uint64_t rotw0 = global0 & mask;
    rotw0 = ((rotw0 << 1) | (rotw0 >> (prev_iterations - 1))) & mask;
    uint64_t src0 = ((global0 & ~mask) | rotw0) - chunk * chunk_elems;

    uint64_t rotw1 = global1 & mask;
    rotw1 = ((rotw1 << 1) | (rotw1 >> (prev_iterations - 1))) & mask;
    uint64_t src1 = ((global1 & ~mask) | rotw1) - chunk * chunk_elems;

    const uint64_t src_word0 = src0 * 4;
    const uint64_t src_word1 = src1 * 4;
    const uint64_t dst_word0 = local0 * 4;
    const uint64_t dst_word1 = local1 * 4;

    dst[dst_word0] = src[src_word0];
    dst[dst_word0 + 1] = src[src_word0 + 1];
    dst[dst_word0 + 2] = src[src_word0 + 2];
    dst[dst_word0 + 3] = src[src_word0 + 3];

    dst[dst_word1] = src[src_word1];
    dst[dst_word1 + 1] = src[src_word1 + 1];
    dst[dst_word1 + 2] = src[src_word1 + 2];
    dst[dst_word1 + 3] = src[src_word1 + 3];
}

void launch_scatter_stage(void* dst,
                          const void* src,
                          uint64_t chunk,
                          uint64_t chunk_elems,
                          int stage,
                          int iterations,
                          cudaStream_t stream) {
    const int block = 256;
    const uint64_t threads = chunk_elems / 2;
    const int grid = static_cast<int>((threads + block - 1) / block);
    scatter_stage_kernel<<<grid, block, 0, stream>>>(
        reinterpret_cast<uint64_t*>(dst),
        reinterpret_cast<const uint64_t*>(src),
        chunk,
        chunk_elems,
        stage,
        iterations);
}

void launch_transition_stage(void* dst,
                             const void* src,
                             uint64_t chunk,
                             uint64_t chunk_elems,
                             int prev_stage,
                             int prev_iterations,
                             int next_stage,
                             int next_iterations,
                             cudaStream_t stream) {
    const int block = 256;
    const uint64_t threads = chunk_elems / 2;
    const int grid = static_cast<int>((threads + block - 1) / block);
    transition_stage_kernel<<<grid, block, 0, stream>>>(
        reinterpret_cast<uint64_t*>(dst),
        reinterpret_cast<const uint64_t*>(src),
        chunk,
        chunk_elems,
        prev_stage,
        prev_iterations,
        next_stage,
        next_iterations);
}

struct FirstPassWorkspace {
    static constexpr int pipeline_slots = 2;

    explicit FirstPassWorkspace(size_t chunk_bytes) {
        for (int slot = 0; slot < pipeline_slots; ++slot) {
            d_buffers[slot] = SyncedMemory(chunk_bytes);
            d_scratch[slot] = SyncedMemory(chunk_bytes);
            d_ptrs[slot] = d_buffers[slot].mutable_gpu_data();
            scratch_ptrs[slot] = d_scratch[slot].mutable_gpu_data();
            cudaStreamCreate(&streams[slot]);
            cudaEventCreateWithFlags(&done[slot], cudaEventDisableTiming);
        }
    }

    ~FirstPassWorkspace() {
        for (int slot = 0; slot < pipeline_slots; ++slot) {
            cudaEventDestroy(done[slot]);
            cudaStreamDestroy(streams[slot]);
        }
    }

    FirstPassWorkspace(const FirstPassWorkspace&) = delete;
    FirstPassWorkspace& operator=(const FirstPassWorkspace&) = delete;

    SyncedMemory d_buffers[pipeline_slots];
    SyncedMemory d_scratch[pipeline_slots];
    void* d_ptrs[pipeline_slots] = {};
    void* scratch_ptrs[pipeline_slots] = {};
    cudaStream_t streams[pipeline_slots] = {};
    cudaEvent_t done[pipeline_slots] = {};
};

constexpr int kMaxSecondPipelineSlots = 2;

int configured_second_pipeline_slots() {
    return kMaxSecondPipelineSlots;
}

int configured_scatter_threads(int default_threads) {
    return default_threads;
}

struct SecondPassWorkspace {
    SecondPassWorkspace(size_t chunk_bytes,
                        int pipeline_slots_arg,
                        int gather_threads,
                        int scatter_threads,
                        bool need_scatter_pools,
                        FirstPassWorkspace& device_workspace)
        : pipeline_slots(pipeline_slots_arg),
          gather_pool(gather_threads) {
        assert(pipeline_slots <= FirstPassWorkspace::pipeline_slots);
        for (int slot = 0; slot < pipeline_slots; ++slot) {
            h_in[slot] = SyncedMemory(chunk_bytes);
            h_out[slot] = SyncedMemory(chunk_bytes);
            d_buffers[slot] = device_workspace.d_buffers[slot];

            h_in_ptrs[slot] = reinterpret_cast<cpu::BLS12_381_Fr_G1*>(
                h_in[slot].mutable_cpu_data());
            h_out_ptrs[slot] = reinterpret_cast<cpu::BLS12_381_Fr_G1*>(
                h_out[slot].mutable_cpu_data());
            d_ptrs[slot] = d_buffers[slot].mutable_gpu_data();
            cudaStreamCreate(&streams[slot]);
            cudaEventCreateWithFlags(&done[slot], cudaEventDisableTiming);
            if (need_scatter_pools) {
                scatter_pools[slot] = std::make_unique<CpuThreadPool>(scatter_threads);
            }
        }
    }

    ~SecondPassWorkspace() {
        for (int slot = 0; slot < pipeline_slots; ++slot) {
            cudaEventDestroy(done[slot]);
            cudaStreamDestroy(streams[slot]);
        }
    }

    SecondPassWorkspace(const SecondPassWorkspace&) = delete;
    SecondPassWorkspace& operator=(const SecondPassWorkspace&) = delete;

    int pipeline_slots;
    SyncedMemory h_in[kMaxSecondPipelineSlots];
    SyncedMemory h_out[kMaxSecondPipelineSlots];
    SyncedMemory d_buffers[kMaxSecondPipelineSlots];
    cpu::BLS12_381_Fr_G1* h_in_ptrs[kMaxSecondPipelineSlots] = {};
    cpu::BLS12_381_Fr_G1* h_out_ptrs[kMaxSecondPipelineSlots] = {};
    void* d_ptrs[kMaxSecondPipelineSlots] = {};
    cudaStream_t streams[kMaxSecondPipelineSlots] = {};
    cudaEvent_t done[kMaxSecondPipelineSlots] = {};
    CpuThreadPool gather_pool;
    std::unique_ptr<CpuThreadPool> scatter_pools[kMaxSecondPipelineSlots];
};

void run_first_ntt_pass(cpu::BLS12_381_Fr_G1* stage_in,
                        cpu::BLS12_381_Fr_G1* stage_out,
                        uint64_t chunk_elems,
                        uint64_t chunk_count,
                        int first_iterations,
                        int lg_N,
                        Ntt ntt,
                        FirstPassWorkspace& workspace) {
    constexpr int pipeline_slots = FirstPassWorkspace::pipeline_slots;
    const size_t chunk_bytes = chunk_elems * sizeof(cpu::BLS12_381_Fr_G1);

    auto* in_bytes = reinterpret_cast<char*>(stage_in);
    auto* out_bytes = reinterpret_cast<char*>(stage_out);
    for (uint64_t chunk = 0; chunk < chunk_count; ++chunk) {
        const int slot = chunk % pipeline_slots;
        if (chunk >= pipeline_slots) {
            cudaEventSynchronize(workspace.done[slot]);
        }

        cudaMemcpyAsync(workspace.d_ptrs[slot], in_bytes + chunk * chunk_bytes,
                        chunk_bytes, cudaMemcpyHostToDevice, workspace.streams[slot]);
        const int first_step = first_iterations / 2;
        const int second_step = first_iterations - first_step;
        ntt.stage_no_rotate(workspace.d_buffers[slot], lg_N, first_step, 0, chunk,
                            workspace.streams[slot]);
        launch_transition_stage(workspace.scratch_ptrs[slot], workspace.d_ptrs[slot],
                                chunk, chunk_elems, 0, first_step,
                                first_step, second_step, workspace.streams[slot]);
        ntt.stage_no_rotate(workspace.d_scratch[slot], lg_N, second_step, first_step,
                            chunk, workspace.streams[slot]);
        launch_scatter_stage(workspace.d_ptrs[slot], workspace.scratch_ptrs[slot],
                             chunk, chunk_elems, first_step, second_step,
                             workspace.streams[slot]);
        cudaMemcpyAsync(out_bytes + chunk * chunk_bytes, workspace.d_ptrs[slot],
                        chunk_bytes, cudaMemcpyDeviceToHost, workspace.streams[slot]);
        cudaEventRecord(workspace.done[slot], workspace.streams[slot]);
    }

    for (int slot = 0; slot < pipeline_slots; ++slot) {
        cudaEventSynchronize(workspace.done[slot]);
    }
}

void run_second_ntt_pass_async_scatter(cpu::BLS12_381_Fr_G1* stage_in,
                                       cpu::BLS12_381_Fr_G1* output,
                                       uint64_t chunk_elems,
                                       uint64_t chunk_count,
                                       int stage,
                                       int iterations,
                                       int lg_N,
                                       int num_threads,
                                       Ntt ntt,
                                       SecondPassWorkspace& workspace,
                                       const StageIndexMap* stage_map = nullptr) {
    const int pipeline_slots = workspace.pipeline_slots;
    const size_t chunk_bytes = chunk_elems * sizeof(cpu::BLS12_381_Fr_G1);
    const bool use_gpu_final_pack =
        should_use_gpu_final_pack(chunk_elems, stage, iterations, lg_N);
    const uint64_t final_rows_half = uint64_t{1} << (iterations - 1);
    const uint64_t final_rows = uint64_t{1} << iterations;
    const uint64_t final_cols = uint64_t{1} << stage;
    const uint64_t final_cols_per_chunk =
        use_gpu_final_pack ? (chunk_elems / 2) / final_rows_half : 0;
    const size_t field_bytes = sizeof(cpu::BLS12_381_Fr_G1);
    const int scatter_threads = configured_scatter_threads(num_threads);

    uint64_t completed_chunk[kMaxSecondPipelineSlots] = {};
    bool slot_valid[kMaxSecondPipelineSlots] = {};
    std::mutex scatter_mutex[kMaxSecondPipelineSlots];
    std::condition_variable scatter_task_cv[kMaxSecondPipelineSlots];
    std::condition_variable scatter_done_cv[kMaxSecondPipelineSlots];
    std::thread scatter_workers[kMaxSecondPipelineSlots];
    uint64_t scatter_task_chunk[kMaxSecondPipelineSlots] = {};
    bool scatter_has_task[kMaxSecondPipelineSlots] = {};
    bool scatter_done[kMaxSecondPipelineSlots] = {};
    bool scatter_stop[kMaxSecondPipelineSlots] = {};

    const bool use_map = stage_map != nullptr &&
                         stage_map->matches(chunk_elems, chunk_count, stage, iterations);

    if (use_gpu_final_pack) {
        cudaStream_t stream = workspace.streams[0];
        cudaEvent_t done_event = workspace.done[0];
        auto gather_chunk = [&](uint64_t chunk, int h_slot) {
            if (use_map) {
                gather_stage_input_map_parallel(workspace.h_in_ptrs[h_slot], stage_in, *stage_map,
                                                chunk, num_threads, &workspace.gather_pool);
            } else {
                permute_stage_input_chunk_parallel(workspace.h_in_ptrs[h_slot], stage_in,
                                                   chunk, chunk_elems, stage, iterations,
                                                   num_threads, &workspace.gather_pool);
            }
        };

        if (chunk_count > 0) {
            gather_chunk(0, 0);
        }
        for (uint64_t chunk = 0; chunk < chunk_count; ++chunk) {
            const int h_slot = chunk % 2;
            const int next_h_slot = 1 - h_slot;
            cudaMemcpyAsync(workspace.d_ptrs[0], workspace.h_in_ptrs[h_slot], chunk_bytes,
                            cudaMemcpyHostToDevice, stream);
            ntt.stage_no_rotate(workspace.d_buffers[0], lg_N, iterations, stage, chunk, stream);
            launch_pack_final_stage_rows(workspace.d_ptrs[1],
                                         workspace.d_ptrs[0],
                                         final_rows_half,
                                         final_cols_per_chunk,
                                         stream);
            const uint64_t col_begin = chunk * final_cols_per_chunk;
            cudaMemcpy2DAsync(output + col_begin,
                              final_cols * field_bytes,
                              workspace.d_ptrs[1],
                              final_cols_per_chunk * field_bytes,
                              final_cols_per_chunk * field_bytes,
                              final_rows,
                              cudaMemcpyDeviceToHost,
                              stream);
            cudaEventRecord(done_event, stream);

            if (chunk + 1 < chunk_count) {
                gather_chunk(chunk + 1, next_h_slot);
            }
            cudaEventSynchronize(done_event);
        }
        return;
    }

    auto run_scatter_task = [&](int slot, uint64_t chunk) {
        if (use_map) {
            scatter_stage_output_map_parallel(output, workspace.h_out_ptrs[slot],
                                              *stage_map, chunk,
                                              scatter_threads,
                                              workspace.scatter_pools[slot].get());
        } else {
            scatter_stage_output_chunk_parallel(output, workspace.h_out_ptrs[slot],
                                                chunk, chunk_elems,
                                                stage, iterations,
                                                scatter_threads,
                                                workspace.scatter_pools[slot].get());
        }
    };

    for (int slot = 0; slot < pipeline_slots; ++slot) {
        scatter_done[slot] = true;
        scatter_workers[slot] = std::thread([&, slot]() {
            while (true) {
                uint64_t chunk = 0;
                {
                    std::unique_lock<std::mutex> lock(scatter_mutex[slot]);
                    scatter_task_cv[slot].wait(lock, [&]() {
                        return scatter_has_task[slot] || scatter_stop[slot];
                    });
                    if (scatter_stop[slot] && !scatter_has_task[slot]) {
                        return;
                    }
                    chunk = scatter_task_chunk[slot];
                    scatter_has_task[slot] = false;
                }

                run_scatter_task(slot, chunk);

                {
                    std::lock_guard<std::mutex> lock(scatter_mutex[slot]);
                    scatter_done[slot] = true;
                }
                scatter_done_cv[slot].notify_one();
            }
        });
    }

    auto wait_scatter = [&](int slot) {
        std::unique_lock<std::mutex> lock(scatter_mutex[slot]);
        scatter_done_cv[slot].wait(lock, [&]() {
            return scatter_done[slot] && !scatter_has_task[slot];
        });
    };

    auto launch_scatter = [&](int slot, uint64_t chunk) {
        wait_scatter(slot);
        {
            std::lock_guard<std::mutex> lock(scatter_mutex[slot]);
            scatter_task_chunk[slot] = chunk;
            scatter_has_task[slot] = true;
            scatter_done[slot] = false;
        }
        scatter_task_cv[slot].notify_one();
    };

    for (uint64_t chunk = 0; chunk < chunk_count; ++chunk) {
        const int slot = chunk % pipeline_slots;
        if (slot_valid[slot]) {
            cudaEventSynchronize(workspace.done[slot]);
            launch_scatter(slot, completed_chunk[slot]);
            slot_valid[slot] = false;
        }

        if (use_map) {
            gather_stage_input_map_parallel(workspace.h_in_ptrs[slot], stage_in, *stage_map,
                                            chunk, num_threads, &workspace.gather_pool);
        } else {
            permute_stage_input_chunk_parallel(workspace.h_in_ptrs[slot], stage_in,
                                               chunk, chunk_elems, stage, iterations,
                                               num_threads, &workspace.gather_pool);
        }
        cudaMemcpyAsync(workspace.d_ptrs[slot], workspace.h_in_ptrs[slot], chunk_bytes,
                        cudaMemcpyHostToDevice, workspace.streams[slot]);
        ntt.stage_no_rotate(workspace.d_buffers[slot], lg_N, iterations, stage, chunk,
                            workspace.streams[slot]);
        wait_scatter(slot);
        cudaMemcpyAsync(workspace.h_out_ptrs[slot], workspace.d_ptrs[slot], chunk_bytes,
                        cudaMemcpyDeviceToHost, workspace.streams[slot]);
        cudaEventRecord(workspace.done[slot], workspace.streams[slot]);
        completed_chunk[slot] = chunk;
        slot_valid[slot] = true;
    }

    for (int slot = 0; slot < pipeline_slots; ++slot) {
        if (slot_valid[slot]) {
            cudaEventSynchronize(workspace.done[slot]);
            launch_scatter(slot, completed_chunk[slot]);
            slot_valid[slot] = false;
        }
    }
    for (int slot = 0; slot < pipeline_slots; ++slot) {
        wait_scatter(slot);
    }

    for (int slot = 0; slot < pipeline_slots; ++slot) {
        {
            std::lock_guard<std::mutex> lock(scatter_mutex[slot]);
            scatter_stop[slot] = true;
        }
        scatter_task_cv[slot].notify_one();
    }
    for (int slot = 0; slot < pipeline_slots; ++slot) {
        scatter_workers[slot].join();
    }
}

void our_ntt(SyncedMemory input,
             int lg_N,
             int lg_i,
             int lg_j,
             int lg_k,
             int step1_thread,
             int step2_thread,
             Ntt ntt,
             const StageIndexMap* second_stage_map = nullptr)
{
    (void)lg_j;
    uint64_t N = uint64_t{1} << lg_N;
    uint64_t I = uint64_t{1} << lg_i;
    uint64_t k = uint64_t{1} << lg_k;
    uint64_t chunk_elems = I * k;
    uint64_t chunk_count = N / chunk_elems;

    SyncedMemory stage1_in(input.size());
    SyncedMemory stage1_out(input.size());

    auto* input_cpu = reinterpret_cast<cpu::BLS12_381_Fr_G1*>(input.mutable_cpu_data());
    auto* stage1_in_cpu = reinterpret_cast<cpu::BLS12_381_Fr_G1*>(stage1_in.mutable_cpu_data());
    auto* stage1_out_cpu = reinterpret_cast<cpu::BLS12_381_Fr_G1*>(stage1_out.mutable_cpu_data());

    const int second_iterations = lg_N - lg_i;
    if (second_iterations > 10) {
        std::cerr << "unsupported NTT split: second stage is "
                  << second_iterations << " bits; expected <= 10" << std::endl;
        std::exit(1);
    }
    uint32_t* step1_map = new uint32_t[N];
    calculate_map1(step1_map, lg_N);

    const size_t chunk_bytes = chunk_elems * sizeof(cpu::BLS12_381_Fr_G1);
    FirstPassWorkspace first_workspace(chunk_bytes);
    const bool use_gpu_final_pack =
        should_use_gpu_final_pack(chunk_elems, lg_i, second_iterations, lg_N);
    const int second_pipeline_slots = configured_second_pipeline_slots();
    const int scatter_threads = configured_scatter_threads(step2_thread);
    SecondPassWorkspace second_workspace(chunk_bytes,
                                         second_pipeline_slots,
                                         step2_thread,
                                         scatter_threads,
                                         !use_gpu_final_pack,
                                         first_workspace);

    cudaEvent_t start, stop;
    cudaEventCreate(&start);
    cudaEventCreate(&stop);
    cudaDeviceSynchronize();
    cudaEventRecord(start, 0);

    bit_rev_step1_parallel(stage1_in_cpu, input_cpu, step1_map, lg_N, step1_thread, 0);
    run_first_ntt_pass(stage1_in_cpu, stage1_out_cpu,
                       chunk_elems, chunk_count, lg_i, lg_N, ntt, first_workspace);
    run_second_ntt_pass_async_scatter(stage1_out_cpu, input_cpu, chunk_elems, chunk_count,
                                      lg_i, second_iterations, lg_N, step2_thread, ntt,
                                      second_workspace, second_stage_map);

    cudaEventRecord(stop, 0);
    cudaEventSynchronize(stop);
    float milliseconds = 0;

    cudaEventElapsedTime(&milliseconds, start, stop);

    std::cout << "our-ntt execution time: " << milliseconds<< " ms" << std::endl;
    cudaDeviceSynchronize();

    cudaEventDestroy(start);
    cudaEventDestroy(stop);
    delete[] step1_map;
}

void standard4_ntt(SyncedMemory input,
                   int lg_N,
                   int lg_i,
                   int lg_j,
                   int lg_k,
                   int transpose_threads,
                   Ntt ntt) {
    const uint64_t N = uint64_t{1} << lg_N;
    const uint64_t I = uint64_t{1} << lg_i;
    const uint64_t J = uint64_t{1} << lg_j;
    const uint64_t k = uint64_t{1} << lg_k;
    const size_t field_bytes = sizeof(cpu::BLS12_381_Fr_G1);
    const uint64_t first_chunk_elems = I * k;
    const uint64_t row_batch = std::min<uint64_t>(
        I, std::max<uint64_t>(1, first_chunk_elems / J));
    const uint64_t second_chunk_elems = J * row_batch;

    SyncedMemory d_col_tile(first_chunk_elems * field_bytes);
    SyncedMemory d_col_batch(first_chunk_elems * field_bytes);
    SyncedMemory d_row_tile(second_chunk_elems * field_bytes);
    SyncedMemory transposed(input.size());

    auto* input_cpu = reinterpret_cast<cpu::BLS12_381_Fr_G1*>(input.mutable_cpu_data());
    auto* transpose_cpu = reinterpret_cast<cpu::BLS12_381_Fr_G1*>(transposed.mutable_cpu_data());
    void* d_col_tile_ptr = d_col_tile.mutable_gpu_data();
    void* d_col_batch_ptr = d_col_batch.mutable_gpu_data();
    void* d_row_tile_ptr = d_row_tile.mutable_gpu_data();

    cudaStream_t stream;
    cudaStreamCreate(&stream);

    cudaEvent_t start, stop;
    cudaEventCreate(&start);
    cudaEventCreate(&stop);
    cudaDeviceSynchronize();
    cudaEventRecord(start, 0);

    for (uint64_t col_base = 0; col_base < J; col_base += k) {
        cudaMemcpy2DAsync(d_col_tile_ptr,
                          k * field_bytes,
                          input_cpu + col_base,
                          J * field_bytes,
                          k * field_bytes,
                          I,
                          cudaMemcpyHostToDevice,
                          stream);
        launch_transpose_fields(d_col_batch_ptr, d_col_tile_ptr, I, k, stream);
        run_batched_simple_ntt(d_col_batch_ptr, k, I, lg_i, ntt, stream);
        launch_standard4_twiddle(d_col_batch_ptr, I, k, col_base, lg_N, ntt, stream);
        launch_transpose_fields(d_col_tile_ptr, d_col_batch_ptr, k, I, stream);
        cudaMemcpy2DAsync(input_cpu + col_base,
                          J * field_bytes,
                          d_col_tile_ptr,
                          k * field_bytes,
                          k * field_bytes,
                          I,
                          cudaMemcpyDeviceToHost,
                          stream);
        cudaStreamSynchronize(stream);
    }

    for (uint64_t row_base = 0; row_base < I; row_base += row_batch) {
        const uint64_t current_batch = std::min<uint64_t>(row_batch, I - row_base);
        auto* row_ptr = input_cpu + row_base * J;
        const size_t row_chunk_bytes = current_batch * J * field_bytes;
        cudaMemcpyAsync(d_row_tile_ptr, row_ptr, row_chunk_bytes,
                        cudaMemcpyHostToDevice, stream);
        run_batched_simple_ntt(d_row_tile_ptr, current_batch, J, lg_j, ntt, stream);
        cudaMemcpyAsync(row_ptr, d_row_tile_ptr, row_chunk_bytes,
                        cudaMemcpyDeviceToHost, stream);
        cudaStreamSynchronize(stream);
    }

    transpose_matrix_parallel(input_cpu, transpose_cpu, I, J, transpose_threads);
    std::memcpy(input_cpu, transpose_cpu, N * field_bytes);

    cudaEventRecord(stop, 0);
    cudaEventSynchronize(stop);
    float milliseconds = 0;
    cudaEventElapsedTime(&milliseconds, start, stop);

    std::cout << "standard4-ntt execution time: " << milliseconds << " ms" << std::endl;
    cudaDeviceSynchronize();

    cudaEventDestroy(start);
    cudaEventDestroy(stop);
    cudaStreamDestroy(stream);
}

int step2_threads_for_lg_k(int lg_k, int lg_N = 24) {
    if (lg_N == 22 && lg_k >= 2 && lg_k <= 9) {
        static const int tuned_22[8] = {6, 8, 8, 10, 6, 6, 20, 20};
        return tuned_22[lg_k - 2];
    }
    if (lg_N == 24 && lg_k >= 1 && lg_k <= 8) {
        static const int tuned_24[8] = {8, 8, 10, 12, 20, 12, 12, 20};
        return tuned_24[lg_k - 1];
    }
    if (lg_N == 26 && lg_k >= 0 && lg_k <= 7) {
        static const int tuned_26[8] = {8, 12, 12, 12, 12, 10, 20, 20};
        return tuned_26[lg_k];
    }
    if (lg_N >= 24) {
        return 12;
    }
    if (lg_k == 1 || lg_k == 2) {
        return 2;
    }
    if (lg_k == 3) {
        return 4;
    }
    return 8;
}

int step1_threads_default() {
    return 16;
}

struct NttScaleConfig {
    int lg_N;
    int lg_i;
    int ours_lg_k;
    int baseline_lg_k_begin;
    int baseline_lg_k_end;
    int reps;
    StageMapPolicy map_policy;
    const char* input_file;
};

NttScaleConfig ntt_scale_config(int lg_N) {
    switch (lg_N) {
    case 22:
        return {22, 12, 6, 2, 9, 3, StageMapPolicy::automatic, "../../input.bin"};
    case 24:
        return {24, 14, 4, 1, 8, 3, StageMapPolicy::automatic, "../../input-24.bin"};
    case 26:
        return {26, 16, 2, 0, 7, 3, StageMapPolicy::disabled, "../../input-26.bin"};
    default:
        std::cerr << "Unsupported lg_N = " << lg_N
                  << ". Supported NTT test scales are 22, 24, and 26." << std::endl;
        std::exit(1);
    }
}

int lg_i_for_domain(int lg_N) {
    return ntt_scale_config(lg_N).lg_i;
}

const char* input_file_for_domain(int lg_N) {
    return ntt_scale_config(lg_N).input_file;
}

size_t chunk_bytes_for(int lg_i, int lg_k) {
    return (size_t{1} << (lg_i + lg_k)) * sizeof(cpu::BLS12_381_Fr_G1);
}

std::string format_bytes(size_t bytes) {
    const double kb = static_cast<double>(bytes) / 1024.0;
    const double mb = kb / 1024.0;
    std::ostringstream out;
    const auto old_flags = out.flags();
    const auto old_precision = out.precision();
    if (mb >= 1.0) {
        out << std::fixed << std::setprecision(2) << mb << " MB";
    } else {
        out << std::fixed << std::setprecision(2) << kb << " KB";
    }
    out.flags(old_flags);
    out.precision(old_precision);
    return out.str();
}

bool should_build_stage_map(StageMapPolicy policy,
                            int iterations,
                            uint64_t chunk_elems = 0) {
    if (iterations > 10 && policy != StageMapPolicy::enabled) {
        return false;
    }
    if (policy == StageMapPolicy::enabled) {
        return true;
    }
    if (policy != StageMapPolicy::automatic) {
        return false;
    }

    if (chunk_elems == 0) {
        return true;
    }
    constexpr uint64_t max_map_chunk_bytes = uint64_t{2} << 20;
    return chunk_elems * sizeof(cpu::BLS12_381_Fr_G1) <= max_map_chunk_bytes;
}

bool compare_outputs(SyncedMemory expected, SyncedMemory actual) {
    const auto* expected_data = reinterpret_cast<const uint64_t*>(expected.mutable_cpu_data());
    const auto* actual_data = reinterpret_cast<const uint64_t*>(actual.mutable_cpu_data());
    const size_t field_words = 4;
    const size_t field_count = expected.size() / (field_words * sizeof(uint64_t));

    for (size_t i = 0; i < field_count; ++i) {
        const size_t word = i * field_words;
        if (std::memcmp(expected_data + word, actual_data + word, field_words * sizeof(uint64_t)) != 0) {
            std::cout << "mismatch at element " << i << "\nexpected:";
            for (size_t j = 0; j < field_words; ++j) {
                std::cout << " 0x" << std::hex << expected_data[word + j];
            }
            std::cout << "\nactual:  ";
            for (size_t j = 0; j < field_words; ++j) {
                std::cout << " 0x" << std::hex << actual_data[word + j];
            }
            std::cout << std::dec << std::endl;
            return false;
        }
    }

    std::cout << "outputs match for " << field_count << " elements" << std::endl;
    return true;
}

int run_check(int lg_k,
              int lg_N = 22,
              const char* file_path = nullptr,
              int lg_i_override = 0,
              StageMapPolicy map_policy = StageMapPolicy::automatic) {
    int lg_i = lg_i_override > 0 ? lg_i_override : lg_i_for_domain(lg_N);
    int lg_j = lg_N - lg_i;
    uint64_t N = uint64_t{1} << lg_N;
    int step1_thread = step1_threads_default();
    int step2_thread = step2_threads_for_lg_k(lg_k, lg_N);
    if (file_path == nullptr) {
        file_path = input_file_for_domain(lg_N);
    }

    Ntt ntt(32);
    SyncedMemory reference_input(N * 32);
    SyncedMemory ours_input(N * 32);
    read_file(file_path, reference_input.mutable_cpu_data());
    read_file(file_path, ours_input.mutable_cpu_data());

    SyncedMemory expected = ntt.forward(reference_input);
    cudaDeviceSynchronize();
    expected.mutable_cpu_data();

    StageIndexMap second_stage_map;
    StageIndexMap* second_stage_map_ptr = nullptr;
    const int second_iterations = lg_N - lg_i;
    const uint64_t chunk_elems = (uint64_t{1} << lg_i) * (uint64_t{1} << lg_k);
    if (should_build_stage_map(map_policy, second_iterations, chunk_elems)) {
        const uint64_t chunk_count = N / chunk_elems;
        second_stage_map = build_stage_index_map(chunk_elems, chunk_count,
                                                 lg_i, second_iterations, step2_thread);
        second_stage_map_ptr = &second_stage_map;
    }

    our_ntt(ours_input, lg_N, lg_i, lg_j, lg_k,
            step1_thread, step2_thread, ntt, second_stage_map_ptr);
    cudaDeviceSynchronize();
    ours_input.mutable_cpu_data();

    return compare_outputs(expected, ours_input) ? 0 : 1;
}

int run_standard4_check(int lg_k,
                        int lg_N = 22,
                        const char* file_path = nullptr,
                        int lg_i_override = 0) {
    const int lg_i = lg_i_override > 0 ? lg_i_override : lg_i_for_domain(lg_N);
    const int lg_j = lg_N - lg_i;
    const uint64_t N = uint64_t{1} << lg_N;
    const int transpose_threads = step1_threads_default();
    if (file_path == nullptr) {
        file_path = input_file_for_domain(lg_N);
    }

    Ntt ntt(32);
    SyncedMemory reference_input(N * 32);
    SyncedMemory standard_input(N * 32);
    read_file(file_path, reference_input.mutable_cpu_data());
    read_file(file_path, standard_input.mutable_cpu_data());

    SyncedMemory expected = ntt.forward(reference_input);
    cudaDeviceSynchronize();
    expected.mutable_cpu_data();

    standard4_ntt(standard_input, lg_N, lg_i, lg_j, lg_k,
                  transpose_threads, ntt);
    cudaDeviceSynchronize();
    standard_input.mutable_cpu_data();

    return compare_outputs(expected, standard_input) ? 0 : 1;
}

int run_our_benchmark(int lg_N,
                      int lg_i,
                      int lg_k_begin,
                      int lg_k_end,
                      int reps,
                      const char* file_path,
                      StageMapPolicy map_policy = StageMapPolicy::automatic) {
    int lg_j = lg_N - lg_i;
    uint64_t N = uint64_t{1} << lg_N;
    int step1_thread = step1_threads_default();

    Ntt ntt(32);
    SyncedMemory input(N * 32);
    void* input_ptr = input.mutable_cpu_data();
    for (int lg_k = lg_k_begin; lg_k <= lg_k_end; ++lg_k) {
        int step2_thread = step2_threads_for_lg_k(lg_k, lg_N);
        const size_t chunk_bytes = chunk_bytes_for(lg_i, lg_k);
        StageIndexMap second_stage_map;
        StageIndexMap* second_stage_map_ptr = nullptr;
        const int second_iterations = lg_N - lg_i;
        const uint64_t chunk_elems = (uint64_t{1} << lg_i) * (uint64_t{1} << lg_k);
        if (should_build_stage_map(map_policy, second_iterations, chunk_elems)) {
            const uint64_t chunk_count = N / chunk_elems;
            second_stage_map = build_stage_index_map(chunk_elems, chunk_count,
                                                     lg_i, second_iterations, step2_thread);
            second_stage_map_ptr = &second_stage_map;
        }
        std::cout << "=== lg_N = " << lg_N
                  << ", split = " << lg_i << "+" << lg_j
                  << ", mode = lowmem"
                  << ", chunk_size = " << format_bytes(chunk_bytes)
                  << ", step1_thread = " << step1_thread
                  << ", step2_thread = " << step2_thread
                  << " ===" << std::endl;

        for (int rep = 0; rep < reps; ++rep) {
            read_file(file_path, input_ptr);
            std::cout << "[our_ntt] run " << (rep + 1) << "/" << reps << std::endl;
            our_ntt(input, lg_N, lg_i, lg_j, lg_k,
                    step1_thread, step2_thread, ntt, second_stage_map_ptr);
        }
    }

    return 0;
}

int run_standard4_benchmark(int lg_N,
                            int lg_i,
                            int lg_k_begin,
                            int lg_k_end,
                            int reps,
                            const char* file_path) {
    const int lg_j = lg_N - lg_i;
    const uint64_t N = uint64_t{1} << lg_N;
    const int transpose_threads = step1_threads_default();

    Ntt ntt(32);
    SyncedMemory input(N * 32);
    void* input_ptr = input.mutable_cpu_data();

    for (int lg_k = lg_k_begin; lg_k <= lg_k_end; ++lg_k) {
        const size_t chunk_bytes = chunk_bytes_for(lg_i, lg_k);
        std::cout << "=== lg_N = " << lg_N
                  << ", split = " << lg_i << "+" << lg_j
                  << ", mode = baseline-standard4"
                  << ", chunk_size = " << format_bytes(chunk_bytes)
                  << ", transpose_threads = " << transpose_threads
                  << " ===" << std::endl;

        for (int rep = 0; rep < reps; ++rep) {
            read_file(file_path, input_ptr);
            std::cout << "[standard4_ntt] run " << (rep + 1) << "/" << reps << std::endl;
            standard4_ntt(input, lg_N, lg_i, lg_j, lg_k,
                          transpose_threads, ntt);
        }
    }

    return 0;
}

int run_configured_our_check(int lg_N) {
    const NttScaleConfig cfg = ntt_scale_config(lg_N);
    return run_check(cfg.ours_lg_k, cfg.lg_N, cfg.input_file, cfg.lg_i,
                     cfg.map_policy);
}

int run_configured_baseline_check(int lg_N) {
    const NttScaleConfig cfg = ntt_scale_config(lg_N);
    return run_standard4_check(cfg.ours_lg_k, cfg.lg_N, cfg.input_file, cfg.lg_i);
}

int run_configured_our_benchmark(int lg_N) {
    const NttScaleConfig cfg = ntt_scale_config(lg_N);
    return run_our_benchmark(cfg.lg_N, cfg.lg_i, cfg.ours_lg_k, cfg.ours_lg_k,
                             cfg.reps, cfg.input_file, cfg.map_policy);
}

int run_configured_baseline_benchmark(int lg_N) {
    const NttScaleConfig cfg = ntt_scale_config(lg_N);
    return run_standard4_benchmark(cfg.lg_N, cfg.lg_i,
                                   cfg.baseline_lg_k_begin,
                                   cfg.baseline_lg_k_end,
                                   cfg.reps, cfg.input_file);
}

void print_ntt_usage(const char* exe) {
    std::cerr << "Usage:\n"
              << "  " << exe << " --bench-split <lg_N>\n"
              << "  " << exe << " --bench-baseline <lg_N>\n"
              << "  " << exe << " --bench-baseline-custom <lg_N> <lg_i> <lg_k_begin> <lg_k_end> [reps]\n"
              << "  " << exe << " --check-split <lg_N>\n"
              << "  " << exe << " --check-baseline <lg_N>\n"
              << "  " << exe << " --bench-custom <lg_N> <lg_i> <lg_k_begin> <lg_k_end> [reps]\n"
              << "  " << exe << " --bench-sweep <lg_N> [reps]\n";
}

int main(int argc, char** argv) {
    if (argc < 2) {
        print_ntt_usage(argv[0]);
        return 1;
    }

    const std::string command(argv[1]);
    if (command == "--bench-baseline") {
        int lg_N = argc >= 3 ? std::atoi(argv[2]) : 22;
        return run_configured_baseline_benchmark(lg_N);
    }
    if (command == "--bench-baseline-custom") {
        int lg_N = argc >= 3 ? std::atoi(argv[2]) : 24;
        int lg_i = argc >= 4 ? std::atoi(argv[3]) : lg_i_for_domain(lg_N);
        int lg_k_begin = argc >= 5 ? std::atoi(argv[4]) : 1;
        int lg_k_end = argc >= 6 ? std::atoi(argv[5]) : lg_k_begin;
        int reps = argc >= 7 ? std::atoi(argv[6]) : 1;
        const NttScaleConfig cfg = ntt_scale_config(lg_N);
        return run_standard4_benchmark(lg_N, lg_i, lg_k_begin, lg_k_end,
                                       reps, cfg.input_file);
    }
    if (command == "--check-baseline") {
        int lg_N = argc >= 3 ? std::atoi(argv[2]) : 22;
        return run_configured_baseline_check(lg_N);
    }
    if (command == "--check-split") {
        int lg_N = argc >= 3 ? std::atoi(argv[2]) : 22;
        return run_configured_our_check(lg_N);
    }
    if (command == "--bench-split") {
        int lg_N = argc >= 3 ? std::atoi(argv[2]) : 22;
        return run_configured_our_benchmark(lg_N);
    }
    if (command == "--bench-sweep") {
        int lg_N = argc >= 3 ? std::atoi(argv[2]) : 22;
        int reps = argc >= 4 ? std::atoi(argv[3]) : 1;
        const NttScaleConfig cfg = ntt_scale_config(lg_N);
        return run_our_benchmark(cfg.lg_N, cfg.lg_i,
                                 cfg.baseline_lg_k_begin,
                                 cfg.baseline_lg_k_end,
                                 reps,
                                 cfg.input_file, cfg.map_policy);
    }
    if (command == "--bench-custom") {
        int lg_N = argc >= 3 ? std::atoi(argv[2]) : 22;
        int lg_i = argc >= 4 ? std::atoi(argv[3]) : lg_i_for_domain(lg_N);
        int lg_k_begin = argc >= 5 ? std::atoi(argv[4]) : 1;
        int lg_k_end = argc >= 6 ? std::atoi(argv[5]) : lg_k_begin;
        int reps = argc >= 7 ? std::atoi(argv[6]) : 1;
        const NttScaleConfig cfg = ntt_scale_config(lg_N);
        return run_our_benchmark(lg_N, lg_i, lg_k_begin, lg_k_end,
                                 reps, cfg.input_file, StageMapPolicy::disabled);
    }

    std::cerr << "Unknown command: " << command << "\n";
    print_ntt_usage(argv[0]);
    return 1;
}

```

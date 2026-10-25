#include <assert.h>
#include <chrono>
#include <climits>
#include <map>
#include <string>
#include <vector>

#include "common/exception.h"
#include "common/halo2_ffi.h"
#include "common/scratch_span.h"

#include "kernel/quotient.h"

using Scalar = utils::FFITraitObject;

struct ValueInfo_t {
    uint32_t source;
    uint32_t src_idx;
    int32_t rotation;
};

struct RotationInfo_t {
    int32_t min_rot { INT_MAX };
    int32_t max_rot { INT_MIN };
};

struct RangeInfo_t {
    RangeInfo_t() = default;
    void set_params(
        uint64_t row_start,
        uint64_t row_end,
        int64_t extend_before_start,
        int64_t extend_before_end)
    {
        this->ori_row_start = row_start;
        this->ori_row_end = row_end;
        this->row_start = row_start; // might be extended
        this->row_end = row_end; // might be extended
        this->extend_before_start = extend_before_start;
        this->extend_before_end = extend_before_end;
    }
    uint64_t total_length { 0 };
    uint64_t ori_row_start { 0 };
    uint64_t ori_row_end { 0 };
    uint64_t row_start { 0 };
    uint64_t row_end { 0 };
    int64_t extend_before_start { 0 };
    int64_t extend_before_end { 0 };
};

static inline bool has_extend_before_end(const RangeInfo_t& range_info, uint64_t num_quotient)
{
    return range_info.extend_before_end != static_cast<int64_t>(num_quotient);
}

static inline uint64_t extend_before_end_length(const RangeInfo_t& range_info, uint64_t num_quotient)
{
    return num_quotient - static_cast<uint64_t>(range_info.extend_before_end);
}

__host__ __device__ inline void decode_value(ValueInfo_t& info, uint64_t rule)
{
    rule = rule & 0xffffffffff; // 40bit
    info.source = rule & 0xf;
    info.src_idx = (rule >> 4) & 0xfffff;
    uint32_t rot_abs = (rule >> 24) & 0x7fff;
    uint32_t rot_sign = (rule >> 39) & 0x1;
    info.rotation = (rot_sign == 0) ? rot_abs : -rot_abs;
}

void decode_rules(RotationInfo_t* rot_info, const uint32_t info_num, const uint64_t* h_rule, uint64_t rule_num)
{
    for (uint64_t i = 0; i < rule_num; i++) {
        ValueInfo_t v_info;
        decode_value(v_info, h_rule[i]);
        uint32_t src = v_info.source;
        if (src < info_num) { // 3
            rot_info[src].min_rot = std::min(rot_info[src].min_rot, v_info.rotation);
            rot_info[src].max_rot = std::max(rot_info[src].max_rot, v_info.rotation);
        }
    }
    for (uint32_t i = 0; i < info_num; i++) {
        // reset unused rotation
        if (rot_info[i].min_rot == INT_MAX)
            rot_info[i].min_rot = 0;
        if (rot_info[i].max_rot == INT_MIN)
            rot_info[i].max_rot = 0;
    }
}

void set_range_info(
    RangeInfo_t& range_info,
    int64_t idx,
    int64_t rot,
    int64_t rot_scale,
    int64_t value_len)
{
    int64_t rotation = idx + (rot * rot_scale);
    if (rotation < 0) {
        rotation = value_len + rotation;
        range_info.extend_before_end = std::min(range_info.extend_before_end, rotation);
    } else if (rotation > value_len) { // row_end start with 1, so do not use >=
        rotation = rotation - value_len;
        range_info.extend_before_start = std::max(range_info.extend_before_start, rotation);
    } else {
        range_info.row_start = std::min(range_info.row_start, (uint64_t)(rotation));
        range_info.row_end = std::max(range_info.row_end, (uint64_t)(rotation));
    };
}

uint64_t compute_range(
    RangeInfo_t* range_info,
    RotationInfo_t* rot_info,
    const uint32_t info_num,
    uint64_t row_start,
    uint64_t row_end,
    uint64_t rotation_scale,
    uint64_t num_quotient)
{
    uint64_t max_length = 0;
    for (uint32_t i = 0; i < info_num; i++) {
        range_info[i].set_params(row_start, row_end, -1, num_quotient);
        set_range_info(range_info[i], row_start, rot_info[i].min_rot, rotation_scale, num_quotient);
        set_range_info(range_info[i], row_start, rot_info[i].max_rot, rotation_scale, num_quotient);
        set_range_info(range_info[i], row_end, rot_info[i].min_rot, rotation_scale, num_quotient);
        set_range_info(range_info[i], row_end, rot_info[i].max_rot, rotation_scale, num_quotient);

        range_info[i].total_length = range_info[i].row_end - range_info[i].row_start; // load_length_normal
        if (range_info[i].extend_before_start != (-1)) {
            range_info[i].total_length += range_info[i].extend_before_start;
        }
        if (has_extend_before_end(range_info[i], num_quotient)) {
            range_info[i].total_length += extend_before_end_length(range_info[i], num_quotient);
        }

        max_length = std::max(max_length, range_info[i].total_length);
    }
    return max_length;
}

class CudaQuotientInfo {
public:
    void set_params(
        uint64_t row_end, uint64_t row_start, uint64_t num_quotient,
        uint64_t num_advice, uint64_t num_instance, uint64_t num_fixed, uint64_t num_challenges,
        uint64_t num_rules, uint64_t num_vp_rules,
        uint64_t num_constants, uint64_t rotation_scale)
    {
        // input
        this->num_quotient = num_quotient;
        this->row_end = row_end;
        this->row_start = row_start;
        this->num_advice = num_advice;
        this->num_instance = num_instance;
        this->num_fixed = num_fixed;
        this->num_challenges = num_challenges;
        this->num_rules = num_rules;
        this->num_vp_rules = num_vp_rules;
        this->num_constants = num_constants;
        this->rotation_scale = rotation_scale;
        // basic
        this->trans_rows = compute_range(this->range_info, this->rot_info, 3, row_start, row_end, rotation_scale, num_quotient);
        this->trans_cols = num_advice + num_instance + num_fixed;
        uint64_t num_rows = row_end - row_start;
        // threads
        num_tiles_per_block = 64;
        num_rows_per_tile = 32;
        num_rows_per_block = num_rows_per_tile * num_tiles_per_block;
        num_blocks = (num_rows + num_rows_per_block - 1) / num_rows_per_block;
        num_tiles = num_blocks * num_tiles_per_block;
        // mem
#define align_32bytes(x) (((x) + 31) & ~31)
        mem_size_matrix = trans_cols * trans_rows * Scalar::ELT_BYTES; // 2 of this size
        mem_size_intermediates = num_tiles * num_rules * Scalar::ELT_BYTES;
        mem_size_challenges = num_challenges * Scalar::ELT_BYTES;
        mem_size_constants = num_constants * Scalar::ELT_BYTES;
        mem_size_expr_constants = zkpcuda::quotient::NUM_EXPR_CONSTs * Scalar::ELT_BYTES;
        mem_size_rules = align_32bytes(num_rules * sizeof(uint64_t) * 2); // 128bit rule
        mem_size_value_part_rules = align_32bytes(num_vp_rules * sizeof(uint64_t)); // 64bit rule
        mem_size_quotient_poly = num_rows * Scalar::ELT_BYTES;
#undef align_32bytes
    }

    uint64_t get_required_memory_size()
    {
        uint64_t required_memory_size = 0;
        required_memory_size += mem_size_matrix;
        required_memory_size += mem_size_intermediates;
        required_memory_size += mem_size_challenges;
        required_memory_size += mem_size_constants;
        required_memory_size += mem_size_expr_constants;
        required_memory_size += mem_size_rules;
        required_memory_size += mem_size_value_part_rules;
        required_memory_size += mem_size_quotient_poly;
        return required_memory_size;
    }

    uint64_t get_max_supported_length_on_device(uint64_t free_bytes)
    {
        uint64_t default_rows = row_end - row_start;
        for (uint64_t rows = default_rows; rows > 0; rows = rows >> 1) {
            set_params(
                row_start + rows, row_start /*don't change*/, num_quotient,
                num_advice, num_instance, num_fixed, num_challenges,
                num_rules, num_vp_rules,
                num_constants, rotation_scale);
            if (get_required_memory_size() < free_bytes) {
                return rows;
            }
        }
        return 0;
    }

    // decode rules
    RotationInfo_t rot_info[3]; // fixed / instance / advice
    RangeInfo_t range_info[3]; // fixed / instance / advice
    // input
    uint64_t num_quotient = 0;
    uint64_t row_end = 0;
    uint64_t row_start = 0;
    uint64_t num_advice = 0;
    uint64_t num_instance = 0;
    uint64_t num_fixed = 0;
    uint64_t num_challenges = 0;
    uint64_t num_rules = 0;
    uint64_t num_vp_rules = 0;
    uint64_t num_constants = 0;
    uint64_t rotation_scale = 0;
    // threads
    uint64_t num_tiles_per_block = 0;
    uint64_t num_rows_per_tile = 0;
    uint64_t num_rows_per_block = 0;
    uint64_t num_blocks = 0;
    uint64_t num_tiles = 0;
    // mem
    uint64_t trans_rows = 0;
    uint64_t trans_cols = 0;
    uint64_t mem_size_matrix = 0;
    uint64_t mem_size_intermediates = 0;
    uint64_t mem_size_challenges = 0;
    uint64_t mem_size_constants = 0;
    uint64_t mem_size_expr_constants = 0;
    uint64_t mem_size_rules = 0;
    uint64_t mem_size_value_part_rules = 0;
    uint64_t mem_size_quotient_poly = 0;
};

void copy_input_data(
    cudaStream_t& stream,
    CudaQuotientInfo& quotient_info,
    uint64_t* h_src, // eg: advices[i].ptr
    uint64_t src_idx, // source index: 0 1 2
    uint64_t* d_ptr) // d_row_matrix[offset]
{
    // rotation in [0, total_lenght)
    uint64_t* h_ptr = h_src + quotient_info.range_info[src_idx].row_start * Scalar::ELT_LIMBS;
    uint64_t normal_length = quotient_info.range_info[src_idx].row_end - quotient_info.range_info[src_idx].row_start;
    uint64_t copy_data_length = normal_length;
    CUDA_OK(cudaMemcpyAsync(d_ptr, h_ptr, copy_data_length * Scalar::ELT_BYTES, cudaMemcpyHostToDevice, stream));
    d_ptr += copy_data_length * Scalar::ELT_LIMBS;
    // rotation < 0
    if (has_extend_before_end(quotient_info.range_info[src_idx], quotient_info.num_quotient)) {
        h_ptr = h_src + static_cast<uint64_t>(quotient_info.range_info[src_idx].extend_before_end) * Scalar::ELT_LIMBS;
        copy_data_length = extend_before_end_length(quotient_info.range_info[src_idx], quotient_info.num_quotient);
        CUDA_OK(cudaMemcpyAsync(d_ptr, h_ptr, copy_data_length * Scalar::ELT_BYTES, cudaMemcpyHostToDevice, stream));
        d_ptr += copy_data_length * Scalar::ELT_LIMBS;
    }
    // rotation >= total_lenght
    if (quotient_info.range_info[src_idx].extend_before_start != -1) {
        h_ptr = h_src; // from the beginning
        copy_data_length = quotient_info.range_info[src_idx].extend_before_start;
        CUDA_OK(cudaMemcpyAsync(d_ptr, h_ptr, copy_data_length * Scalar::ELT_BYTES, cudaMemcpyHostToDevice, stream));
    }
}

RustError transpose_colunms(
    cudaStream_t& stream,
    CudaQuotientInfo& quotient_info,
    Scalar* fixed, int num_fixed,
    Scalar* instance, int num_instance,
    Scalar* advices, int num_advice,
    uint64_t* d_row_matrix,
    int row_start, int row_end)
{
    (void)row_start;
    (void)row_end;
    uint64_t offset = 0;

    try {
        for (int i = 0; i < num_fixed; i++) {
            copy_input_data(stream, quotient_info, fixed[i].ptr, 0, &d_row_matrix[offset]);
            offset += quotient_info.trans_rows * Scalar::ELT_LIMBS;
        }
        for (int i = 0; i < num_instance; i++) {
            copy_input_data(stream, quotient_info, instance[i].ptr, 1, &d_row_matrix[offset]);
            offset += quotient_info.trans_rows * Scalar::ELT_LIMBS;
        }
        for (int i = 0; i < num_advice; i++) {
            copy_input_data(stream, quotient_info, advices[i].ptr, 2, &d_row_matrix[offset]);
            offset += quotient_info.trans_rows * Scalar::ELT_LIMBS;
        }
    } catch (const cuda_error& error) {
        return RustError(error.code(), error.what());
    };

    return cudaSuccess;
}

extern "C" uint64_t _halo2_evaluate_h_max_rows(
    uint64_t* rules,
    uint64_t num_rules,
    uint64_t num_vp_rules,
    uint64_t num_quotient,
    uint64_t num_fixed,
    uint64_t num_instance,
    uint64_t num_advice,
    uint64_t num_challenges,
    uint64_t num_constants,
    uint64_t rotation_scale,
    uint64_t row_start,
    uint64_t row_end,
    uint64_t free_bytes)
{
    CudaQuotientInfo quotient_info;
    decode_rules(quotient_info.rot_info, 3, rules, num_rules * 2);
    quotient_info.set_params(
        row_end, row_start, num_quotient,
        num_advice, num_instance, num_fixed, num_challenges,
        num_rules, num_vp_rules,
        num_constants, rotation_scale);
    return quotient_info.get_max_supported_length_on_device(free_bytes);
}

extern "C" void _halo2_quotient_decode(
    uint64_t* rules, uint64_t num_rules, /* 128bit rule */
    uint64_t num_quotient,
    uint64_t rotation_scale,
    uint64_t row_start,
    uint64_t row_end)
{
    RotationInfo_t rot_info[3]; // fixed / instance / advice
    decode_rules(rot_info, 3, rules, num_rules * 2);
    RangeInfo_t range_info[3]; // fixed / instance / advice
    compute_range(range_info, rot_info, 3, row_start, row_end, rotation_scale, num_quotient);
}

// Pure host preflight: returns the byte size Rust must hand into
// `_halo2_quotient` as the `scratch` buffer. Reproduces the same
// `CudaQuotientInfo::get_required_memory_size()` formula the kernel uses.
extern "C" uint64_t _halo2_quotient_workspace_size(
    uint64_t* rules, uint64_t num_rules,
    uint64_t num_vp_rules,
    uint64_t num_quotient,
    uint64_t num_fixed,
    uint64_t num_instance,
    uint64_t num_advice,
    uint64_t num_challenges,
    uint64_t num_constants,
    uint64_t rotation_scale,
    uint64_t row_start,
    uint64_t row_end)
{
    CudaQuotientInfo quotient_info;
    decode_rules(quotient_info.rot_info, 3, rules, num_rules * 2);
    quotient_info.set_params(
        row_end, row_start, num_quotient,
        num_advice, num_instance, num_fixed, num_challenges,
        num_rules, num_vp_rules,
        num_constants, rotation_scale);
    return align_up(quotient_info.get_required_memory_size(), 32);
}

// D2D-copy sibling of `copy_input_data`. The source pointer is a
// device-resident full-length column; layout into `d_ptr` is identical
// to the H2D variant so the kernel body is unchanged.
void copy_input_data_device(
    cudaStream_t& stream,
    CudaQuotientInfo& quotient_info,
    uint64_t* d_src,
    uint64_t src_idx,
    uint64_t* d_ptr)
{
    uint64_t* h_ptr = d_src + quotient_info.range_info[src_idx].row_start * Scalar::ELT_LIMBS;
    uint64_t normal_length = quotient_info.range_info[src_idx].row_end - quotient_info.range_info[src_idx].row_start;
    uint64_t copy_data_length = normal_length;
    CUDA_OK(cudaMemcpyAsync(d_ptr, h_ptr, copy_data_length * Scalar::ELT_BYTES, cudaMemcpyDeviceToDevice, stream));
    d_ptr += copy_data_length * Scalar::ELT_LIMBS;
    if (has_extend_before_end(quotient_info.range_info[src_idx], quotient_info.num_quotient)) {
        h_ptr = d_src + static_cast<uint64_t>(quotient_info.range_info[src_idx].extend_before_end) * Scalar::ELT_LIMBS;
        copy_data_length = extend_before_end_length(quotient_info.range_info[src_idx], quotient_info.num_quotient);
        CUDA_OK(cudaMemcpyAsync(d_ptr, h_ptr, copy_data_length * Scalar::ELT_BYTES, cudaMemcpyDeviceToDevice, stream));
        d_ptr += copy_data_length * Scalar::ELT_LIMBS;
    }
    if (quotient_info.range_info[src_idx].extend_before_start != -1) {
        h_ptr = d_src;
        copy_data_length = quotient_info.range_info[src_idx].extend_before_start;
        CUDA_OK(cudaMemcpyAsync(d_ptr, h_ptr, copy_data_length * Scalar::ELT_BYTES, cudaMemcpyDeviceToDevice, stream));
    }
}

RustError transpose_colunms_device(
    cudaStream_t& stream,
    CudaQuotientInfo& quotient_info,
    const void* const* fixed_d_ptrs, int num_fixed,
    const void* const* instance_d_ptrs, int num_instance,
    const void* const* advice_d_ptrs, int num_advice,
    uint64_t* d_row_matrix,
    int row_start, int row_end)
{
    (void)row_start;
    (void)row_end;
    uint64_t offset = 0;
    try {
        for (int i = 0; i < num_fixed; i++) {
            copy_input_data_device(stream, quotient_info, (uint64_t*)fixed_d_ptrs[i], 0, &d_row_matrix[offset]);
            offset += quotient_info.trans_rows * Scalar::ELT_LIMBS;
        }
        for (int i = 0; i < num_instance; i++) {
            copy_input_data_device(stream, quotient_info, (uint64_t*)instance_d_ptrs[i], 1, &d_row_matrix[offset]);
            offset += quotient_info.trans_rows * Scalar::ELT_LIMBS;
        }
        for (int i = 0; i < num_advice; i++) {
            copy_input_data_device(stream, quotient_info, (uint64_t*)advice_d_ptrs[i], 2, &d_row_matrix[offset]);
            offset += quotient_info.trans_rows * Scalar::ELT_LIMBS;
        }
    } catch (const cuda_error& error) {
        return RustError(error.code(), error.what());
    };
    return cudaSuccess;
}

extern "C" RustError _halo2_quotient(
    Scalar* fixed,
    uint64_t num_fixed,
    Scalar* instance,
    uint64_t num_instance,
    Scalar* advices,
    uint64_t num_advice,
    Scalar* challenges,
    uint64_t num_challenges,
    Scalar constants,
    uint64_t num_constants,
    Scalar expr_constants,
    uint64_t* rules, uint64_t num_rules, /* 128bit rule */
    uint64_t* value_part_rules, uint64_t num_vp_rules, /* 64bit rule */
    Scalar quotient_poly,
    uint64_t num_quotient, /* total */
    uint64_t rotation_scale,
    uint64_t row_start,
    uint64_t row_end,
    void* scratch,
    uint64_t scratch_bytes,
    cudaStream_t stream)
{
    // init
    ScratchSpan span { (uint8_t*)scratch, (size_t)scratch_bytes };
    CudaQuotientInfo quotient_info;
    decode_rules(quotient_info.rot_info, 3, rules, num_rules * 2); // TODO: Use it as a constructor for CudaQuotientInfo
    quotient_info.set_params(
        row_end, row_start, num_quotient,
        num_advice, num_instance, num_fixed, num_challenges,
        num_rules, num_vp_rules,
        num_constants, rotation_scale);
    uint64_t required_memory_size = quotient_info.get_required_memory_size();
    uint64_t* quotient_gpu_mem = (uint64_t*)span.take(required_memory_size);

    // set ptrs
    uint64_t* d_row_matrix = (uint64_t*)((char*)quotient_gpu_mem);
    uint64_t* d_intermediates = (uint64_t*)((char*)d_row_matrix + quotient_info.mem_size_matrix);
    uint64_t* d_challenges = (uint64_t*)((char*)d_intermediates + quotient_info.mem_size_intermediates);
    uint64_t* d_constants = (uint64_t*)((char*)d_challenges + quotient_info.mem_size_challenges);
    uint64_t* d_expr_constants = (uint64_t*)((char*)d_constants + quotient_info.mem_size_constants);
    uint64_t* d_rules = (uint64_t*)((char*)d_expr_constants + quotient_info.mem_size_expr_constants);
    uint64_t* d_value_part_rules = (uint64_t*)((char*)d_rules + quotient_info.mem_size_rules);
    uint64_t* d_quotient_poly = (uint64_t*)((char*)d_value_part_rules + quotient_info.mem_size_value_part_rules);

    RustError state_transpose = transpose_colunms(
        stream, quotient_info,
        fixed, num_fixed,
        instance, num_instance,
        advices, num_advice,
        d_row_matrix,
        row_start, row_end);
    if (state_transpose.code != cudaSuccess) {
        return state_transpose;
    }

    try {
        // memcpy
        CUDA_OK(cudaMemcpyAsync(d_challenges, challenges, quotient_info.mem_size_challenges, cudaMemcpyHostToDevice, stream));
        CUDA_OK(cudaMemcpyAsync(d_constants, constants.ptr, quotient_info.mem_size_constants, cudaMemcpyHostToDevice, stream));
        CUDA_OK(cudaMemcpyAsync(d_expr_constants, expr_constants.ptr, quotient_info.mem_size_expr_constants, cudaMemcpyHostToDevice, stream));
        CUDA_OK(cudaMemcpyAsync(d_rules, rules, num_rules * sizeof(uint64_t) * 2, cudaMemcpyHostToDevice, stream));
        CUDA_OK(cudaMemcpyAsync(d_value_part_rules, value_part_rules, num_vp_rules * sizeof(uint64_t), cudaMemcpyHostToDevice, stream));
        // memset
        CUDA_OK(cudaMemsetAsync(d_intermediates, 0, quotient_info.mem_size_intermediates, stream));
        CUDA_OK(cudaMemsetAsync(d_quotient_poly, 0, quotient_info.mem_size_quotient_poly, stream));
    } catch (const cuda_error& error) {
        return RustError(error.code(), error.what());
    };

    const uint64_t normal_length_fixed = quotient_info.range_info[0].row_end - quotient_info.range_info[0].row_start;
    const uint64_t normal_length_instance = quotient_info.range_info[1].row_end - quotient_info.range_info[1].row_start;
    const uint64_t normal_length_advice = quotient_info.range_info[2].row_end - quotient_info.range_info[2].row_start;
    uint64_t before_end_length_fixed = 0;
    uint64_t before_end_length_instance = 0;
    uint64_t before_end_length_advice = 0;
    if (has_extend_before_end(quotient_info.range_info[0], quotient_info.num_quotient)) {
        before_end_length_fixed = extend_before_end_length(quotient_info.range_info[0], quotient_info.num_quotient);
    }
    if (has_extend_before_end(quotient_info.range_info[1], quotient_info.num_quotient)) {
        before_end_length_instance = extend_before_end_length(quotient_info.range_info[1], quotient_info.num_quotient);
    }
    if (has_extend_before_end(quotient_info.range_info[2], quotient_info.num_quotient)) {
        before_end_length_advice = extend_before_end_length(quotient_info.range_info[2], quotient_info.num_quotient);
    }
    uint64_t raw_offset_fixed = quotient_info.range_info[0].ori_row_start - quotient_info.range_info[0].row_start;
    uint64_t raw_offset_instance = quotient_info.range_info[1].ori_row_start - quotient_info.range_info[1].row_start;
    uint64_t raw_offset_advice = quotient_info.range_info[2].ori_row_start - quotient_info.range_info[2].row_start;

    // printf("quotient_info.num_blocks = %d, quotient_info.num_tiles_per_block = %d\r\n", quotient_info.num_blocks, quotient_info.num_tiles_per_block);

    zkpcuda::quotient::evaluate<<<quotient_info.num_blocks, quotient_info.num_tiles_per_block, 0, stream>>>(
        (const scalar_t*)d_row_matrix,
        (scalar_t*)d_intermediates,
        (const scalar_t*)d_challenges,
        (const scalar_t*)d_constants,
        (const scalar_t*)d_expr_constants,
        (const zkpcuda::quotient::Rule*)d_rules,
        num_rules,
        d_value_part_rules,
        num_vp_rules,
        (scalar_t*)d_quotient_poly,
        num_quotient,
        num_fixed,
        num_instance,
        num_advice,
        num_constants,
        raw_offset_fixed,
        raw_offset_instance,
        raw_offset_advice,
        normal_length_fixed,
        normal_length_instance,
        normal_length_advice,
        before_end_length_fixed,
        before_end_length_instance,
        before_end_length_advice,
        row_start,
        row_end,
        quotient_info.trans_rows,
        quotient_info.num_rows_per_tile,
        rotation_scale,
        nullptr);

    try {
        CUDA_OK(cudaMemcpyAsync(quotient_poly.ptr, d_quotient_poly, quotient_info.mem_size_quotient_poly, cudaMemcpyDeviceToHost, stream));
        CUDA_OK(cudaStreamSynchronize(stream));
    } catch (const cuda_error& error) {
        return RustError(error.code(), error.what());
    };

    return cudaSuccess;
}

// Device-pointer column input AND device-pointer output variant of
// `_halo2_quotient`. Mirrors `_halo2_quotient_device_columns` but writes
// the per-chunk quotient_poly result via D2D into a caller-provided
// device buffer; no D2H of the quotient poly happens inside this FFI.
// The result remains device-resident on the caller's stream.
extern "C" RustError _halo2_quotient_device_columns_device_out(
    const void* const* fixed_d_ptrs,
    uint64_t num_fixed,
    const void* const* instance_d_ptrs,
    uint64_t num_instance,
    const void* const* advice_d_ptrs,
    uint64_t num_advice,
    Scalar* challenges,
    uint64_t num_challenges,
    Scalar constants,
    uint64_t num_constants,
    Scalar expr_constants,
    uint64_t* rules, uint64_t num_rules,
    uint64_t* value_part_rules, uint64_t num_vp_rules,
    void* quotient_poly_device_ptr,
    uint64_t num_quotient,
    const void* prev_values_device_ptr, // per-row value-part seed; null => seed zero
    uint64_t rotation_scale,
    uint64_t row_start,
    uint64_t row_end,
    void* scratch,
    uint64_t scratch_bytes,
    cudaStream_t stream)
{
    ScratchSpan span { (uint8_t*)scratch, (size_t)scratch_bytes };
    CudaQuotientInfo quotient_info;
    decode_rules(quotient_info.rot_info, 3, rules, num_rules * 2);
    quotient_info.set_params(
        row_end, row_start, num_quotient,
        num_advice, num_instance, num_fixed, num_challenges,
        num_rules, num_vp_rules,
        num_constants, rotation_scale);
    uint64_t required_memory_size = quotient_info.get_required_memory_size();
    uint64_t* quotient_gpu_mem = (uint64_t*)span.take(required_memory_size);
    // No `cudaSetDevice` here — the Rust caller invokes
    // `ensure_current_device_matches_ctx`. No `cudaStreamSynchronize`
    // either: every op below is queued on the same `stream`, and the
    // function returns a device-pointer result with no host read.

    uint64_t* d_row_matrix = (uint64_t*)((char*)quotient_gpu_mem);
    uint64_t* d_intermediates = (uint64_t*)((char*)d_row_matrix + quotient_info.mem_size_matrix);
    uint64_t* d_challenges = (uint64_t*)((char*)d_intermediates + quotient_info.mem_size_intermediates);
    uint64_t* d_constants = (uint64_t*)((char*)d_challenges + quotient_info.mem_size_challenges);
    uint64_t* d_expr_constants = (uint64_t*)((char*)d_constants + quotient_info.mem_size_constants);
    uint64_t* d_rules = (uint64_t*)((char*)d_expr_constants + quotient_info.mem_size_expr_constants);
    uint64_t* d_value_part_rules = (uint64_t*)((char*)d_rules + quotient_info.mem_size_rules);
    uint64_t* d_quotient_poly = (uint64_t*)((char*)d_value_part_rules + quotient_info.mem_size_value_part_rules);

    RustError state_transpose = transpose_colunms_device(
        stream, quotient_info,
        fixed_d_ptrs, (int)num_fixed,
        instance_d_ptrs, (int)num_instance,
        advice_d_ptrs, (int)num_advice,
        d_row_matrix,
        (int)row_start, (int)row_end);
    if (state_transpose.code != cudaSuccess) {
        return state_transpose;
    }

    try {
        CUDA_OK(cudaMemcpyAsync(d_challenges, challenges, quotient_info.mem_size_challenges, cudaMemcpyHostToDevice, stream));
        CUDA_OK(cudaMemcpyAsync(d_constants, constants.ptr, quotient_info.mem_size_constants, cudaMemcpyHostToDevice, stream));
        CUDA_OK(cudaMemcpyAsync(d_expr_constants, expr_constants.ptr, quotient_info.mem_size_expr_constants, cudaMemcpyHostToDevice, stream));
        CUDA_OK(cudaMemcpyAsync(d_rules, rules, num_rules * sizeof(uint64_t) * 2, cudaMemcpyHostToDevice, stream));
        CUDA_OK(cudaMemcpyAsync(d_value_part_rules, value_part_rules, num_vp_rules * sizeof(uint64_t), cudaMemcpyHostToDevice, stream));
        CUDA_OK(cudaMemsetAsync(d_intermediates, 0, quotient_info.mem_size_intermediates, stream));
        CUDA_OK(cudaMemsetAsync(d_quotient_poly, 0, quotient_info.mem_size_quotient_poly, stream));
    } catch (const cuda_error& error) {
        return RustError(error.code(), error.what());
    };

    const uint64_t normal_length_fixed = quotient_info.range_info[0].row_end - quotient_info.range_info[0].row_start;
    const uint64_t normal_length_instance = quotient_info.range_info[1].row_end - quotient_info.range_info[1].row_start;
    const uint64_t normal_length_advice = quotient_info.range_info[2].row_end - quotient_info.range_info[2].row_start;
    uint64_t before_end_length_fixed = 0;
    uint64_t before_end_length_instance = 0;
    uint64_t before_end_length_advice = 0;
    if (has_extend_before_end(quotient_info.range_info[0], quotient_info.num_quotient)) {
        before_end_length_fixed = extend_before_end_length(quotient_info.range_info[0], quotient_info.num_quotient);
    }
    if (has_extend_before_end(quotient_info.range_info[1], quotient_info.num_quotient)) {
        before_end_length_instance = extend_before_end_length(quotient_info.range_info[1], quotient_info.num_quotient);
    }
    if (has_extend_before_end(quotient_info.range_info[2], quotient_info.num_quotient)) {
        before_end_length_advice = extend_before_end_length(quotient_info.range_info[2], quotient_info.num_quotient);
    }
    uint64_t raw_offset_fixed = quotient_info.range_info[0].ori_row_start - quotient_info.range_info[0].row_start;
    uint64_t raw_offset_instance = quotient_info.range_info[1].ori_row_start - quotient_info.range_info[1].row_start;
    uint64_t raw_offset_advice = quotient_info.range_info[2].ori_row_start - quotient_info.range_info[2].row_start;

    zkpcuda::quotient::evaluate<<<quotient_info.num_blocks, quotient_info.num_tiles_per_block, 0, stream>>>(
        (const scalar_t*)d_row_matrix,
        (scalar_t*)d_intermediates,
        (const scalar_t*)d_challenges,
        (const scalar_t*)d_constants,
        (const scalar_t*)d_expr_constants,
        (const zkpcuda::quotient::Rule*)d_rules,
        num_rules,
        d_value_part_rules,
        num_vp_rules,
        (scalar_t*)d_quotient_poly,
        num_quotient,
        num_fixed,
        num_instance,
        num_advice,
        num_constants,
        raw_offset_fixed,
        raw_offset_instance,
        raw_offset_advice,
        normal_length_fixed,
        normal_length_instance,
        normal_length_advice,
        before_end_length_fixed,
        before_end_length_instance,
        before_end_length_advice,
        row_start,
        row_end,
        quotient_info.trans_rows,
        quotient_info.num_rows_per_tile,
        rotation_scale,
        (const scalar_t*)prev_values_device_ptr);

    try {
        // Surface kernel-launch failures synchronously without blocking
        // the host on stream completion. No sync is performed inside
        // this method; the result is D2D into a caller-owned device
        // buffer, so the caller's eventual host read provides the
        // fence.
        CUDA_OK(cudaGetLastError());
        CUDA_OK(cudaMemcpyAsync(quotient_poly_device_ptr, d_quotient_poly, quotient_info.mem_size_quotient_poly, cudaMemcpyDeviceToDevice, stream));
    } catch (const cuda_error& error) {
        return RustError(error.code(), error.what());
    };

    return cudaSuccess;
}

// Device-pointer column input variant of `_halo2_quotient`. The internal
// `transpose_colunms_device` path does D2D copies from caller-provided
// device-resident columns instead of H2D copies from host columns. The
// kernel body is unchanged from `_halo2_quotient`; only the column
// ingest path differs.
extern "C" RustError _halo2_quotient_device_columns(
    const void* const* fixed_d_ptrs,
    uint64_t num_fixed,
    const void* const* instance_d_ptrs,
    uint64_t num_instance,
    const void* const* advice_d_ptrs,
    uint64_t num_advice,
    Scalar* challenges,
    uint64_t num_challenges,
    Scalar constants,
    uint64_t num_constants,
    Scalar expr_constants,
    uint64_t* rules, uint64_t num_rules,
    uint64_t* value_part_rules, uint64_t num_vp_rules,
    Scalar quotient_poly,
    uint64_t num_quotient,
    uint64_t rotation_scale,
    uint64_t row_start,
    uint64_t row_end,
    void* scratch,
    uint64_t scratch_bytes,
    cudaStream_t stream)
{
    ScratchSpan span { (uint8_t*)scratch, (size_t)scratch_bytes };
    CudaQuotientInfo quotient_info;
    decode_rules(quotient_info.rot_info, 3, rules, num_rules * 2);
    quotient_info.set_params(
        row_end, row_start, num_quotient,
        num_advice, num_instance, num_fixed, num_challenges,
        num_rules, num_vp_rules,
        num_constants, rotation_scale);
    uint64_t required_memory_size = quotient_info.get_required_memory_size();
    uint64_t* quotient_gpu_mem = (uint64_t*)span.take(required_memory_size);
    // No `cudaSetDevice` here — the Rust caller invokes
    // `ensure_current_device_matches_ctx`. No `cudaStreamSynchronize`
    // between same-stream ops below — stream ordering is the contract.
    // The trailing D2H sync (further below) stays because the caller
    // reads `quotient_poly.ptr` host memory after this FFI returns.

    uint64_t* d_row_matrix = (uint64_t*)((char*)quotient_gpu_mem);
    uint64_t* d_intermediates = (uint64_t*)((char*)d_row_matrix + quotient_info.mem_size_matrix);
    uint64_t* d_challenges = (uint64_t*)((char*)d_intermediates + quotient_info.mem_size_intermediates);
    uint64_t* d_constants = (uint64_t*)((char*)d_challenges + quotient_info.mem_size_challenges);
    uint64_t* d_expr_constants = (uint64_t*)((char*)d_constants + quotient_info.mem_size_constants);
    uint64_t* d_rules = (uint64_t*)((char*)d_expr_constants + quotient_info.mem_size_expr_constants);
    uint64_t* d_value_part_rules = (uint64_t*)((char*)d_rules + quotient_info.mem_size_rules);
    uint64_t* d_quotient_poly = (uint64_t*)((char*)d_value_part_rules + quotient_info.mem_size_value_part_rules);

    RustError state_transpose = transpose_colunms_device(
        stream, quotient_info,
        fixed_d_ptrs, (int)num_fixed,
        instance_d_ptrs, (int)num_instance,
        advice_d_ptrs, (int)num_advice,
        d_row_matrix,
        (int)row_start, (int)row_end);
    if (state_transpose.code != cudaSuccess) {
        return state_transpose;
    }

    try {
        CUDA_OK(cudaMemcpyAsync(d_challenges, challenges, quotient_info.mem_size_challenges, cudaMemcpyHostToDevice, stream));
        CUDA_OK(cudaMemcpyAsync(d_constants, constants.ptr, quotient_info.mem_size_constants, cudaMemcpyHostToDevice, stream));
        CUDA_OK(cudaMemcpyAsync(d_expr_constants, expr_constants.ptr, quotient_info.mem_size_expr_constants, cudaMemcpyHostToDevice, stream));
        CUDA_OK(cudaMemcpyAsync(d_rules, rules, num_rules * sizeof(uint64_t) * 2, cudaMemcpyHostToDevice, stream));
        CUDA_OK(cudaMemcpyAsync(d_value_part_rules, value_part_rules, num_vp_rules * sizeof(uint64_t), cudaMemcpyHostToDevice, stream));
        CUDA_OK(cudaMemsetAsync(d_intermediates, 0, quotient_info.mem_size_intermediates, stream));
        CUDA_OK(cudaMemsetAsync(d_quotient_poly, 0, quotient_info.mem_size_quotient_poly, stream));
    } catch (const cuda_error& error) {
        return RustError(error.code(), error.what());
    };

    const uint64_t normal_length_fixed = quotient_info.range_info[0].row_end - quotient_info.range_info[0].row_start;
    const uint64_t normal_length_instance = quotient_info.range_info[1].row_end - quotient_info.range_info[1].row_start;
    const uint64_t normal_length_advice = quotient_info.range_info[2].row_end - quotient_info.range_info[2].row_start;
    uint64_t before_end_length_fixed = 0;
    uint64_t before_end_length_instance = 0;
    uint64_t before_end_length_advice = 0;
    if (has_extend_before_end(quotient_info.range_info[0], quotient_info.num_quotient)) {
        before_end_length_fixed = extend_before_end_length(quotient_info.range_info[0], quotient_info.num_quotient);
    }
    if (has_extend_before_end(quotient_info.range_info[1], quotient_info.num_quotient)) {
        before_end_length_instance = extend_before_end_length(quotient_info.range_info[1], quotient_info.num_quotient);
    }
    if (has_extend_before_end(quotient_info.range_info[2], quotient_info.num_quotient)) {
        before_end_length_advice = extend_before_end_length(quotient_info.range_info[2], quotient_info.num_quotient);
    }
    uint64_t raw_offset_fixed = quotient_info.range_info[0].ori_row_start - quotient_info.range_info[0].row_start;
    uint64_t raw_offset_instance = quotient_info.range_info[1].ori_row_start - quotient_info.range_info[1].row_start;
    uint64_t raw_offset_advice = quotient_info.range_info[2].ori_row_start - quotient_info.range_info[2].row_start;

    zkpcuda::quotient::evaluate<<<quotient_info.num_blocks, quotient_info.num_tiles_per_block, 0, stream>>>(
        (const scalar_t*)d_row_matrix,
        (scalar_t*)d_intermediates,
        (const scalar_t*)d_challenges,
        (const scalar_t*)d_constants,
        (const scalar_t*)d_expr_constants,
        (const zkpcuda::quotient::Rule*)d_rules,
        num_rules,
        d_value_part_rules,
        num_vp_rules,
        (scalar_t*)d_quotient_poly,
        num_quotient,
        num_fixed,
        num_instance,
        num_advice,
        num_constants,
        raw_offset_fixed,
        raw_offset_instance,
        raw_offset_advice,
        normal_length_fixed,
        normal_length_instance,
        normal_length_advice,
        before_end_length_fixed,
        before_end_length_instance,
        before_end_length_advice,
        row_start,
        row_end,
        quotient_info.trans_rows,
        quotient_info.num_rows_per_tile,
        rotation_scale,
        nullptr);

    try {
        // Surface kernel-launch failures synchronously; the trailing
        // sync below is required because the caller reads
        // `quotient_poly.ptr` (host memory) after this FFI returns.
        CUDA_OK(cudaGetLastError());
        CUDA_OK(cudaMemcpyAsync(quotient_poly.ptr, d_quotient_poly, quotient_info.mem_size_quotient_poly, cudaMemcpyDeviceToHost, stream));
        CUDA_OK(cudaStreamSynchronize(stream));
    } catch (const cuda_error& error) {
        return RustError(error.code(), error.what());
    };

    return cudaSuccess;
}
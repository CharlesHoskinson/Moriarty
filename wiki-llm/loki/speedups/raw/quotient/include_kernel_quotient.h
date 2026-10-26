#pragma once

#include "field/alt_bn128.hpp"
#include <cstdint>
#include <cuda.h>
#include <cuda_runtime.h>

typedef fr_t scalar_t;

namespace zkpcuda {
namespace quotient {

    //   data source:
    //   variable: source(4 bit) ||  idx(20 bit)  || rotation(16 bit)
    //    fixed:       0         ||  fixed[idx]   || fixed[idx][row+rotation*rot_scale]
    //    instance:    1         || instance[idx] || instance[idx][row+rotation*rot_scale]
    //    advice:      2         ||  advice[idx]  || advice[idx][row+rotation*rot_scale]
    //  intermediates: 3         || interm[idx]   || interm[idx]
    //    constant:    4         || constant[idx] || constant[idx]
    //    challenge:   5         || challenge[idx]|| challenge[idx]
    //    dummy:       6         || unused

    // the rule denote an expression of the form
    //        (C1*a + C2*b)*(D ? 1 : b)
    // where Ci = arr[ci], D is bool.
    // and arr = [0, 1, -1, 2]

    //   expr   |  c1  |  c2  |  D  |
    //   a+b    |  1   |  1   |  0  |
    //   a-b    |  1   |  -1  |  0  |
    //   a*b    |  1   |  0   |  1  |
    //   -a     |  -1  |  0   |  0  |
    //   2*a    |  2   |  0   |  0  |
    //   a*a    |  1   |  0   |  1  |
    typedef uint4 Rule; // 128bit encoded

    static const int SOURCE_MASK = 0x0f;
    static const int SOURCE_SHIFT = 4;
    static const int SOURCE_CONSTANT = 4;
    static const int IDX_MASK = 0xfffff;
    static const int IDX_SHIFT = 20;
    static const int ROTATION_MASK = 0xffff;
    static const int ROTATION_ABS_MASK = 0x7fff;
    static const int ROTATION_SHIFT = 16;
    static const int C_MASK = 0x0f;
    static const int C_SHIFT = 4;
    static const int NUM_EXPR_CONSTs = 5;
    static const int MAX_SHARED_CONSTs = 500;

    __host__ __device__ inline int get_rotation(int rot)
    {
        int sign_a = (rot >> (ROTATION_SHIFT - 1));
        int abs_a = rot & ROTATION_ABS_MASK;
        return sign_a == 0 ? abs_a : -abs_a;
    }

    __global__ static void evaluate(
        const scalar_t* d_row_matrix,
        scalar_t* d_intermediates,
        const scalar_t* d_challenges,
        const scalar_t* d_constants,
        const scalar_t* d_expr_constants, // [0, 1, -1, 2, y]
        const Rule* d_rules,
        const uint64_t num_rules,
        uint64_t* d_value_part_rules,
        const uint64_t num_vp_rules,
        scalar_t* d_quotient_poly, // result
        const uint64_t num_quotient, // total length
        const uint64_t num_fixed,
        const uint64_t num_instance,
        const uint64_t num_advice,
        const uint64_t num_constants,
        const uint64_t raw_offset_fixed,
        const uint64_t raw_offset_instance,
        const uint64_t raw_offset_advice,
        const uint64_t normal_length_fixed,
        const uint64_t normal_length_instance,
        const uint64_t normal_length_advice,
        const uint64_t before_end_length_fixed,
        const uint64_t before_end_length_instance,
        const uint64_t before_end_length_advice,
        const int64_t row_start,
        const int64_t row_end,
        const uint64_t trans_rows,
        const uint64_t num_rows_per_tile,
        const uint64_t rotation_scale,
        const scalar_t* d_prev_values) // per-row seed for the value-part Horner; null seeds zero
    {
        const uint32_t block_idx = blockIdx.x;
        const uint32_t tile_idx = threadIdx.x;
        const uint32_t tile_size = blockDim.x;

        scalar_t y;
        __shared__ alignas(scalar_t) uint8_t shared_constants_storage[MAX_SHARED_CONSTs * sizeof(scalar_t)];
        __shared__ alignas(scalar_t) uint8_t shared_expr_constants_storage[NUM_EXPR_CONSTs * sizeof(scalar_t)];
        scalar_t* shared_constants = reinterpret_cast<scalar_t*>(shared_constants_storage);
        scalar_t* shared_expr_constants = reinterpret_cast<scalar_t*>(shared_expr_constants_storage);
        for (int i = tile_idx; i < NUM_EXPR_CONSTs; i += tile_size) {
            if (i < NUM_EXPR_CONSTs)
                shared_expr_constants[i] = d_expr_constants[i];
        }
        scalar_t* constants_ptr = (scalar_t*)d_constants;
        if (num_constants <= MAX_SHARED_CONSTs) {
            for (int i = tile_idx; i < num_constants; i += tile_size) {
                if (i < num_constants)
                    shared_constants[i] = d_constants[i];
            }
            __syncthreads();
            constants_ptr = shared_constants;
        }
        __syncthreads();
        y = shared_expr_constants[NUM_EXPR_CONSTs - 1];

        int row_offset = block_idx * tile_size * num_rows_per_tile + tile_idx;
        scalar_t* intermediates_ptr = d_intermediates + (block_idx * tile_size * num_rules + tile_idx);
        uint64_t compute_row_num = row_end - row_start;

        // compute intermediates
        for (int j = 0; j < num_rows_per_tile; j++) {
            int row = row_offset + j * tile_size;
            if (row >= compute_row_num)
                break;
            for (int i = 0; i < num_rules; i++) {
                Rule rule = d_rules[i];
                uint64_t* rule_ptr = (uint64_t*)(&rule);
                uint64_t rule_ac1d = rule_ptr[0];
                uint64_t rule_bc2 = rule_ptr[1];
                // get var a
                int source_a = rule_ac1d & SOURCE_MASK;
                rule_ac1d = rule_ac1d >> SOURCE_SHIFT;
                int idx_a = rule_ac1d & IDX_MASK;
                rule_ac1d = rule_ac1d >> IDX_SHIFT;
                int rot_a = rule_ac1d & ROTATION_MASK;
                rule_ac1d = rule_ac1d >> ROTATION_SHIFT;
                // rot_a might be negative
                rot_a = get_rotation(rot_a);

                // get var b
                int source_b = rule_bc2 & SOURCE_MASK;
                rule_bc2 = rule_bc2 >> SOURCE_SHIFT;
                int idx_b = rule_bc2 & IDX_MASK;
                rule_bc2 = rule_bc2 >> IDX_SHIFT;
                int rot_b = rule_bc2 & ROTATION_MASK;
                rule_bc2 = rule_bc2 >> ROTATION_SHIFT;
                // rot_b might be negative
                rot_b = get_rotation(rot_b);

                int c1 = rule_ac1d & C_MASK;
                rule_ac1d = rule_ac1d >> C_SHIFT;
                int c2 = rule_bc2 & C_MASK;
                rule_bc2 = rule_bc2 >> C_SHIFT;
                int d = rule_ac1d & C_MASK;

                scalar_t* vars_a[6];
                scalar_t* vars_b[6];
                int32_t row_a = row + rot_a * rotation_scale; // rotation

                int32_t row_a_normal = row_a;
                row_a_normal += (source_a == 0) ? raw_offset_fixed : 0;
                row_a_normal += (source_a == 1) ? raw_offset_instance : 0;
                row_a_normal += (source_a == 2) ? raw_offset_advice : 0;
                vars_a[0] = (scalar_t*)d_row_matrix + row_a_normal; // normal case
                if (row_start + row_a < (int64_t)0) { // absolute index in [0, total_rows)
                    int32_t normal_length = (source_a == 0) ? (normal_length_fixed + before_end_length_fixed) : 0;
                    normal_length = (source_a == 1) ? (normal_length_instance + before_end_length_instance) : normal_length;
                    normal_length = (source_a == 2) ? (normal_length_advice + before_end_length_advice) : normal_length;
                    int32_t rem_row_a = normal_length + (row_start + row_a); // data: [row_start, row_end, ..., rem_row_a, end)
                    vars_a[0] = (scalar_t*)d_row_matrix + rem_row_a;
                }
                if (row_start + row_a >= (int64_t)num_quotient) {
                    int32_t normal_length = (source_a == 0) ? (normal_length_fixed + before_end_length_fixed) : 0;
                    normal_length = (source_a == 1) ? (normal_length_instance + before_end_length_instance) : normal_length;
                    normal_length = (source_a == 2) ? (normal_length_advice + before_end_length_advice) : normal_length;
                    int32_t rem_row_a = row_start + row_a - (int64_t)num_quotient; // data: [0, rem_row_a, row_start, ...)
                    vars_a[0] = (scalar_t*)d_row_matrix + (normal_length + rem_row_a);
                }
                vars_a[1] = vars_a[0] + num_fixed * trans_rows;
                vars_a[2] = vars_a[1] + num_instance * trans_rows;
                vars_a[3] = intermediates_ptr;
                vars_a[4] = constants_ptr;
                vars_a[5] = (scalar_t*)d_challenges;

                int32_t row_b = row + rot_b * rotation_scale;
                int32_t row_b_normal = row_b;
                row_b_normal += (source_b == 0) ? raw_offset_fixed : 0;
                row_b_normal += (source_b == 1) ? raw_offset_instance : 0;
                row_b_normal += (source_b == 2) ? raw_offset_advice : 0;
                vars_b[0] = (scalar_t*)d_row_matrix + row_b_normal; // normal case
                if (row_start + row_b < (int64_t)0) { // absolute index in [0, total_rows)
                    int32_t normal_length = (source_b == 0) ? normal_length_fixed + before_end_length_fixed : 0;
                    normal_length = (source_b == 1) ? normal_length_instance + before_end_length_instance : normal_length;
                    normal_length = (source_b == 2) ? normal_length_advice + before_end_length_advice : normal_length;
                    int32_t rem_row_b = normal_length + (row_start + row_b); // data: [row_start, row_end, ..., rem_row_a, end)
                    vars_b[0] = (scalar_t*)d_row_matrix + rem_row_b;
                }
                if (row_start + row_b >= (int64_t)num_quotient) {
                    int32_t normal_length = (source_b == 0) ? normal_length_fixed + before_end_length_fixed : 0;
                    normal_length = (source_b == 1) ? normal_length_instance + before_end_length_instance : normal_length;
                    normal_length = (source_b == 2) ? normal_length_advice + before_end_length_advice : normal_length;
                    int32_t rem_row_b = row_start + row_b - (int64_t)num_quotient; // data: [0, rem_row_a, row_start, ...)
                    vars_b[0] = (scalar_t*)d_row_matrix + (normal_length + rem_row_b);
                }
                vars_b[1] = vars_b[0] + num_fixed * trans_rows;
                vars_b[2] = vars_b[1] + num_instance * trans_rows;
                vars_b[3] = intermediates_ptr;
                vars_b[4] = constants_ptr;
                vars_b[5] = (scalar_t*)d_challenges;

                scalar_t a, b;
                if (source_a == 0)
                    a = vars_a[0][idx_a * trans_rows];
                if (source_a == 1)
                    a = vars_a[1][idx_a * trans_rows];
                if (source_a == 2)
                    a = vars_a[2][idx_a * trans_rows];
                if (source_a == 3)
                    a = vars_a[3][idx_a * tile_size];
                if (source_a == 4)
                    a = vars_a[4][idx_a];
                if (source_a == 5)
                    a = vars_a[5][idx_a];

                if (source_b == 0)
                    b = vars_b[0][idx_b * trans_rows];
                if (source_b == 1)
                    b = vars_b[1][idx_b * trans_rows];
                if (source_b == 2)
                    b = vars_b[2][idx_b * trans_rows];
                if (source_b == 3)
                    b = vars_b[3][idx_b * tile_size];
                if (source_b == 4)
                    b = vars_b[4][idx_b];
                if (source_b == 5)
                    b = vars_b[5][idx_b];

                scalar_t D = scalar_t::one();
                if (d == 1) {
                    D = b;
                }

                // use if-else to compute intermediate
                scalar_t interm;
                interm.zero();
                if (c1 == 1) { // 1*a
                    if (c2 == 0) { // 1*a + 0*b
                        if (d == 1) {
                            interm = a * b;
                        } else {
                            interm = a;
                        }
                    } else if (c2 == 1) {
                        interm = a + b;
                    } else if (c2 == 2) {
                        interm = a - b;
                    }
                } else if (c1 == 2) { // -a
                    interm = a.cneg(true);
                } else if (c1 == 3) { // 2*a
                    interm = a + a;
                } else { // 0*a
                }

                intermediates_ptr[i * tile_size] = interm;
            }

            /// do value_part combination
            /// val = val * y + val_part, seeded by the prior circuit's row value
            /// (d_prev_values[row]) so a batch of circuits folds into one poly;
            /// a null seed starts from zero (single-circuit / first circuit).
            scalar_t val;
            if (d_prev_values != nullptr) {
                val = d_prev_values[row];
            } else {
                val.zero();
            }
            for (int i = 0; i < num_vp_rules; i++) {
                uint64_t rule = d_value_part_rules[i];
                int source = rule & SOURCE_MASK;
                rule = rule >> SOURCE_SHIFT;
                int idx = rule & IDX_MASK;
                rule = rule >> IDX_SHIFT;
                int rot = rule & ROTATION_MASK;
                rule = rule >> ROTATION_SHIFT;
                rot = get_rotation(rot);

                // Value-part rules only carry SOURCE_CONSTANT or
                // SOURCE_INTERMEDIATE: the Rust encoder (add_expression) wraps
                // every column and challenge in a Store, so the Horner parts
                // reduce to constants and intermediates. The two buffers use
                // different strides, so select explicitly by source.
                scalar_t val_part;
                if (source == SOURCE_CONSTANT) {
                    val_part = constants_ptr[idx];
                } else { // SOURCE_INTERMEDIATE
                    val_part = intermediates_ptr[idx * tile_size];
                }

                val = val * y + val_part;
            }

            d_quotient_poly[row] = val;
        }
    }

    __host__ __device__ inline int32_t lookups_rotation(int32_t idx, int32_t rot, int32_t rot_scale, int32_t isize)
    {
        int32_t row = idx + rot * rot_scale;
        if (row < 0) {
            row = isize + row;
        }
        if (row >= isize) {
            row = row - isize;
        }
        return row;
    }

    // Computes the numerator part of lookup's quotient polynomial.
    //  N(X) = L1(X) + L2(X)*Y + L3(X)*Y^2 + L4(X)*Y^3 + L5(X)*Y^4
    //       = ((((L5(X) * Y + L4(X)) * Y + L3(X)) * Y + L2(X)) * Y + L1(X)
    // Lookup identities includes
    // 1. l_active(X)*(A'(X) - S'(X))*(A'(X) - A'(w^(-1)*X)) = 0
    // 2. l0(X)*(A'(X) - S'(X)) = 0
    // 3. l_active(X)*(Z(wX)*(A'(X)+\beta)*(S'(X)+\gamma) - Z(X)*(A(X)+\beta)*(S(X)+\gamma)) = 0
    // 4. l_last(X)*(Z(X)^2 - Z(X)) = 0
    // 5. l0(X)*(1 - Z(X)) = 0
    template <uint32_t TILE_SIZE>
    __global__ static void cuda_kernel_quotient_lookups(
        scalar_t* d_values, // N(X)
        scalar_t* d_table_values, // (A(X)+beta)*(S(X)+gamma)
        scalar_t* d_product_coset, // Z(X)
        scalar_t* d_permuted_input_coset, // A'(X)
        scalar_t* d_permuted_table_coset, // S'(X)
        scalar_t* d_l0, // l0(X)
        scalar_t* d_l_last, // l_last(X)
        scalar_t* d_l_active_row, // l_active(X)
        scalar_t* d_beta,
        scalar_t* d_gamma,
        scalar_t* d_y,
        uint64_t length)
    {
        uint64_t offset = TILE_SIZE * blockIdx.x + threadIdx.x;
        uint64_t stride = TILE_SIZE * gridDim.x;

        scalar_t one = scalar_t::one();
        scalar_t beta = *d_beta;
        scalar_t gamma = *d_gamma;
        scalar_t y = *d_y;

        for (uint64_t idx = offset; idx < length; idx += stride) {
            if (idx >= length)
                break;
            uint64_t r_next = lookups_rotation(idx, 1, 1, length);
            uint64_t r_prev = lookups_rotation(idx, -1, 1, length);

            scalar_t value = d_values[idx];
            scalar_t table_value = d_table_values[idx];
            scalar_t product_coset = d_product_coset[idx];
            scalar_t permuted_input = d_permuted_input_coset[idx];
            scalar_t permuted_table = d_permuted_table_coset[idx];
            scalar_t product_coset_r_next = d_product_coset[r_next];
            scalar_t permuted_input_coset_r_prev = d_permuted_input_coset[r_prev];
            scalar_t l0 = d_l0[idx];
            scalar_t l_last = d_l_last[idx];
            scalar_t l_active_row = d_l_active_row[idx];

            scalar_t a_minus_s = permuted_input - permuted_table;
            // 5. l0(X)*(1 - Z(X))
            value = value * y + ((one - product_coset) * l0);
            // 4. l_last(X) * (Z(X)^2 - Z(X))
            value = (value * y + ((product_coset * product_coset - product_coset) * l_last));
            // 3. l_active(X) * (Z(wX) * (A'(X)+\beta)*(S'(X)+\gamma) - Z(X)*(A(X)+\beta)*(S(X)+\gamma))
            value = (value * y + ((product_coset_r_next * (permuted_input + beta) * (permuted_table + gamma) - product_coset * table_value) * l_active_row));
            // 2. l0(X)*(A'(X) - S'(X))
            value = (value * y + (a_minus_s * l0));
            // 1. l_active(X) * (A'(X) - S'(X)) * (A'(X) - A'(w^(-1)*X))
            value = (value * y + (a_minus_s * (permuted_input - permuted_input_coset_r_prev) * l_active_row));

            d_values[idx] = value;
        }
    }

} // namespace quotient
} // namespace zkpcuda

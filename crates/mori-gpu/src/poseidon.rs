// SPDX-License-Identifier: Apache-2.0
//! Midnight Poseidon over BLS12-381 Fr on the GPU (E4): batch Merkle leaf hashes and digest
//! layers of the `L_Fr` prover (stark-sdk `FrMerkleHasher`). Kernels in `cuda/poseidon.cuh`.
//!
//! The caller passes the 68 x 3 round constants and the 3 x 3 MDS matrix of
//! midnight-circuits (`PoseidonField`), so this crate holds no constants of its own. Every
//! function returns `None` when the GPU path is not taken; the caller then hashes on the CPU.
#![allow(unused_variables)]

use midnight_curves::Fq;

/// Rounds of the permutation (8 full and 60 partial).
pub const ROUNDS: usize = 68;
/// Partial rounds.
pub const PARTIAL: usize = 60;
/// Length of the constant table: `ROUNDS` x 3 round constants, the MDS matrix row-major,
/// then the sparse partial-round table (`PARTIAL` x 6: a, v0, v1, w0, w1, k), the final
/// 2 x 2 block (4) and the carried constants (2), all in the kernel's coordinates
/// `t = (s2, s0, s1)` (see `cuda/poseidon.cuh`).
pub const NCONST: usize = ROUNDS * 3 + 9 + PARTIAL * 6 + 6;

/// The permutation constants (`NCONST` values).
pub struct Constants(pub Vec<Fq>);

/// Kernel mode: `MORI_POS_MODE=0` the plain rounds, otherwise (default) the sparse
/// partial rounds.
pub fn mode() -> i32 {
    match std::env::var("MORI_POS_MODE").as_deref() {
        Ok("0") => 0,
        _ => 1,
    }
}

#[cfg(feature = "cuda")]
fn ensure(c: &Constants) -> bool {
    use std::sync::Mutex;
    static SET: Mutex<Option<(i32, Vec<[u64; 4]>)>> = Mutex::new(None);
    let m = mode();
    let words: Vec<[u64; 4]> =
        c.0.iter()
            .map(|x| unsafe { std::mem::transmute::<Fq, [u64; 4]>(*x) })
            .collect();
    let key = (m, words);
    let mut s = SET.lock().unwrap();
    if s.as_ref() == Some(&key) {
        return true;
    }
    let rc = unsafe { crate::ffi::mori_pos_set_constants(c.0.as_ptr().cast(), c.0.len(), m) };
    if rc != 0 {
        eprintln!("mori-gpu: poseidon constants: {}", crate::ffi::last_error());
        return false;
    }
    *s = Some(key);
    true
}

/// Leaf hashes of `vals.len() / width` rows (row-major) with capacity `cap`.
pub fn hash_rows(c: &Constants, vals: &[Fq], width: usize, cap: Fq) -> Option<Vec<Fq>> {
    #[cfg(feature = "cuda")]
    {
        if width == 0 || vals.len() % width != 0 || c.0.len() != NCONST {
            return None;
        }
        crate::info()?;
        let n = vals.len() / width;
        let _g = crate::ffi::GPU.lock().unwrap();
        if !ensure(c) {
            return None;
        }
        let mut out = vec![Fq::from(0u64); n];
        let rc = unsafe {
            crate::ffi::mori_pos_hash_rows(
                out.as_mut_ptr().cast(),
                vals.as_ptr().cast(),
                n,
                width as u32,
                (&cap as *const Fq).cast(),
            )
        };
        if rc != 0 {
            eprintln!("mori-gpu: poseidon rows: {}", crate::ffi::last_error());
            return None;
        }
        Some(out)
    }
    #[cfg(not(feature = "cuda"))]
    None
}

/// Digest layers as stark-backend `build_digest_layers_scalar`: `log2(rows_per_query)`
/// rounds at query stride `n / rows_per_query`, then halving to the root. Returns the
/// layers from the first one after the strided rounds down to the root.
pub fn digest_layers(
    c: &Constants,
    leaves: &[Fq],
    rows_per_query: usize,
    cap: Fq,
) -> Option<Vec<Vec<Fq>>> {
    #[cfg(feature = "cuda")]
    {
        let n = leaves.len();
        if !n.is_power_of_two()
            || !rows_per_query.is_power_of_two()
            || rows_per_query > n
            || c.0.len() != NCONST
        {
            return None;
        }
        crate::info()?;
        let l0 = n / rows_per_query;
        let mut flat = vec![Fq::from(0u64); 2 * l0 - 1];
        let _g = crate::ffi::GPU.lock().unwrap();
        if !ensure(c) {
            return None;
        }
        let rc = unsafe {
            crate::ffi::mori_pos_digest_layers(
                flat.as_mut_ptr().cast(),
                leaves.as_ptr().cast(),
                n,
                rows_per_query,
                (&cap as *const Fq).cast(),
            )
        };
        if rc != 0 {
            eprintln!("mori-gpu: poseidon layers: {}", crate::ffi::last_error());
            return None;
        }
        let mut layers = Vec::new();
        let mut at = 0;
        let mut len = l0;
        loop {
            layers.push(flat[at..at + len].to_vec());
            at += len;
            if len == 1 {
                break;
            }
            len /= 2;
        }
        Some(layers)
    }
    #[cfg(not(feature = "cuda"))]
    None
}

/// Fused RS encode and leaf hash: `coeffs` holds `width` columns of `height` coefficients
/// (column-major). Each column is zero-padded to `2^log_n` and evaluated with the forward
/// NTT at the standard root (natural order). Returns the row-major codeword
/// (`2^log_n x width`) and the leaf hashes with capacity `cap`.
pub fn encode_hash_rows(
    c: &Constants,
    coeffs: &[Fq],
    height: usize,
    width: usize,
    log_n: u32,
    cap: Fq,
) -> Option<(Vec<Fq>, Vec<Fq>)> {
    #[cfg(feature = "cuda")]
    {
        let n = 1usize << log_n;
        if width == 0
            || coeffs.len() != height * width
            || height > n
            || log_n > 28
            || c.0.len() != NCONST
        {
            return None;
        }
        crate::info()?;
        let mut rm = vec![Fq::from(0u64); n * width];
        let mut dg = vec![Fq::from(0u64); n];
        let _g = crate::ffi::GPU.lock().unwrap();
        if !ensure(c) {
            return None;
        }
        let rc = unsafe {
            crate::ffi::mori_pos_encode_hash(
                rm.as_mut_ptr().cast(),
                dg.as_mut_ptr().cast(),
                coeffs.as_ptr().cast(),
                height,
                width as u32,
                log_n,
                (&cap as *const Fq).cast(),
            )
        };
        if rc != 0 {
            eprintln!("mori-gpu: poseidon encode: {}", crate::ffi::last_error());
            return None;
        }
        Some((rm, dg))
    }
    #[cfg(not(feature = "cuda"))]
    None
}

// SPDX-License-Identifier: Apache-2.0
//! Fractional sumcheck (LogUp GKR) over BLS12-381 Fr on the GPU (E4): the segment tree and
//! the per-layer sumcheck of stark-backend `fractional_sumcheck` with `EF = F = Fr`. Kernels
//! in `cuda/frac.cuh`. A fraction is `(p, q)`, interleaved, as stark-backend `Frac` (repr(C)).
//! Field arithmetic is exact, so the values equal the CPU values bit for bit.
#![allow(unused_variables)]

use midnight_curves::Fq;

/// GPU path on unless `E4_GPU_FRAC=0`.
pub fn enabled() -> bool {
    !matches!(std::env::var("E4_GPU_FRAC").as_deref(), Ok("0"))
}

/// Fills nodes `1 .. 2^lg` of the segment tree `tree` (`2^(lg+1)` fractions, so
/// `2^(lg+2)` values; leaves at fractions `2^lg ..`). Node `k` = node `2k` + node `2k+1`.
/// Returns `false` when the GPU path is not taken (the tree is then unchanged).
pub fn build_tree(tree: &mut [Fq], lg: usize) -> bool {
    #[cfg(feature = "cuda")]
    {
        if tree.len() != 4usize << lg || !enabled() {
            return false;
        }
        if crate::info().is_none() {
            return false;
        }
        let _g = crate::ffi::GPU.lock().unwrap();
        let rc = unsafe { crate::ffi::mori_frac_tree(tree.as_mut_ptr().cast(), lg as u32) };
        if rc != 0 {
            eprintln!("mori-gpu: frac tree: {}", crate::ffi::last_error());
            return false;
        }
        return true;
    }
    #[allow(unreachable_code)]
    false
}

/// The `n = log2(eq.len())` sumcheck rounds of one GKR layer: `seg` holds the `2 es`
/// fractions of the layer (`4 es` values), `eq` the `es` values of `eq(xi, .)`. For each
/// round, `round` gets the round polynomial at 1, 2, 3 and returns the challenge `r`.
/// Returns the claims `(p0, q0, p1, q1)` at the folded point. `None` only before the first
/// call of `round`; an error after it panics (the transcript has moved on).
pub fn layer_sumcheck(
    seg: &[Fq],
    eq: &[Fq],
    lambda: Fq,
    round: &mut dyn FnMut([Fq; 3]) -> Fq,
) -> Option<[Fq; 4]> {
    #[cfg(feature = "cuda")]
    {
        let es = eq.len();
        if es < 2 || !es.is_power_of_two() || seg.len() != 4 * es || !enabled() {
            return None;
        }
        crate::info()?;
        let _g = crate::ffi::GPU.lock().unwrap();
        let ctx = unsafe {
            crate::ffi::mori_frac_layer_begin(seg.as_ptr().cast(), eq.as_ptr().cast(), es)
        };
        if ctx.is_null() {
            eprintln!("mori-gpu: frac layer: {}", crate::ffi::last_error());
            return None;
        }
        let zero = Fq::from(0u64);
        let mut m = es;
        while m > 1 {
            let mut s = [zero; 3];
            let rc = unsafe {
                crate::ffi::mori_frac_layer_round(
                    ctx,
                    (&lambda as *const Fq).cast(),
                    s.as_mut_ptr().cast(),
                )
            };
            assert_eq!(rc, 0, "mori-gpu: frac round: {}", crate::ffi::last_error());
            let r = round(s);
            let rc = unsafe { crate::ffi::mori_frac_layer_fold(ctx, (&r as *const Fq).cast()) };
            assert_eq!(rc, 0, "mori-gpu: frac fold: {}", crate::ffi::last_error());
            m /= 2;
        }
        let mut c = [zero; 4];
        let rc = unsafe { crate::ffi::mori_frac_layer_end(ctx, c.as_mut_ptr().cast()) };
        assert_eq!(rc, 0, "mori-gpu: frac end: {}", crate::ffi::last_error());
        return Some(c);
    }
    #[allow(unreachable_code)]
    None
}

// SPDX-License-Identifier: Apache-2.0
//! Device code of the Rust MSM (BLS12-381 G1). Built only for nvptx64-nvidia-cuda.
//!
//! Pipeline (host order in `mori-msm/src/msm.rs`):
//! 1. `msm_digits`: canonical scalars, signed c-bit digits, key = (window, |digit| - 1),
//!    bucket histogram with atomics. Zero digits get the key `NONE`.
//! 2. `scan_*`: exclusive scan of the histogram (bucket offsets), distinct bucket count.
//! 3. `msm_scatter`: counting sort of (key, point index | sign) by key.
//! 4. `seg_count` + scan + `seg_reduce_*`: load-balanced segmented reduction. Each
//!    thread takes `k` consecutive sorted entries whatever bucket they belong to and
//!    emits one partial sum per key run; rounds repeat until each key has one entry.
//!    No bucket is ever accumulated by one thread alone (skew robust).
//! 5. `bucket_scatter`, `bucket_reduce`, `sum_groups`, `horner`: window sums and the
//!    final combination.
#![no_std]
#![feature(abi_ptx, stdarch_nvptx)]
#![allow(clippy::missing_safety_doc)]

use core::arch::nvptx;
use core::sync::atomic::{AtomicU32, Ordering::Relaxed};

pub mod ec;
pub mod fp;
pub mod fr;

use ec::{Affine, INF, Xyzz};
use fp::Fp;

pub const NONE: u32 = u32::MAX;
pub const SIGN: u32 = 1 << 31;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe { nvptx::trap() }
}

#[inline(always)]
fn tid() -> usize {
    unsafe { nvptx::_thread_idx_x() as usize }
}
#[inline(always)]
fn bid() -> usize {
    unsafe { nvptx::_block_idx_x() as usize }
}
#[inline(always)]
fn bdim() -> usize {
    unsafe { nvptx::_block_dim_x() as usize }
}
#[inline(always)]
fn gid() -> usize {
    bid() * bdim() + tid()
}
#[inline(always)]
fn gsize() -> usize {
    unsafe { (nvptx::_grid_dim_x() as usize) * bdim() }
}
#[inline(always)]
fn sync() {
    unsafe { nvptx::_syncthreads() }
}
#[inline(always)]
unsafe fn atomic_add(p: *mut u32, v: u32) -> u32 {
    unsafe { AtomicU32::from_ptr(p) }.fetch_add(v, Relaxed)
}

// ---------------------------------------------------------------- tests

/// out[i] = a[i] * b[i], iterated `reps` times.
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn fp_mul_test(
    a: *const Fp,
    b: *const Fp,
    out: *mut Fp,
    n: u32,
    reps: u32,
) {
    let i = gid();
    if i >= n as usize {
        return;
    }
    unsafe {
        let mut x = *a.add(i);
        let y = *b.add(i);
        for _ in 0..reps {
            x = x.mul(&y);
        }
        *out.add(i) = x;
    }
}

/// out[3i..3i+3] = [a + b, a - b, a^-1].
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn fp_ops_test(a: *const Fp, b: *const Fp, out: *mut Fp, n: u32) {
    let i = gid();
    if i >= n as usize {
        return;
    }
    unsafe {
        let x = *a.add(i);
        let y = *b.add(i);
        *out.add(3 * i) = x.add(&y);
        *out.add(3 * i + 1) = x.sub(&y);
        *out.add(3 * i + 2) = x.inv();
    }
}

/// Point ops: out[4i..4i+4] = [p + q (mixed), p + q (xyzz), 2p, 5p] for affine p, q.
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn ec_test(
    p: *const Affine,
    q: *const Affine,
    out: *mut Xyzz,
    n: u32,
) {
    let i = gid();
    if i >= n as usize {
        return;
    }
    unsafe {
        let a = *p.add(i);
        let b = *q.add(i);
        let xa = Xyzz::from_affine(&a);
        let xb = Xyzz::from_affine(&b);
        *out.add(4 * i) = xa.add_affine(&b);
        *out.add(4 * i + 1) = xa.add(&xb);
        *out.add(4 * i + 2) = xa.dbl();
        *out.add(4 * i + 3) = xa.mul_small(5);
    }
}

// ---------------------------------------------------------------- MSM

/// Signed c-bit digits of every scalar. Entry `w * n + i` is window w of scalar i.
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn msm_digits(
    scalars: *const [u64; 4],
    n: u32,
    c: u32,
    nw: u32,
    keys: *mut u32,
    vals: *mut u32,
    counts: *mut u32,
) {
    let n = n as usize;
    let half = 1u32 << (c - 1);
    let full = 1u64 << c;
    let mask = full - 1;
    let mut i = gid();
    while i < n {
        let s = fr::from_mont(unsafe { *scalars.add(i) });
        let mut carry = 0u64;
        for w in 0..nw as usize {
            let bit = w * c as usize;
            let limb = bit / 64;
            let off = bit % 64;
            let mut raw = if limb < 4 { s[limb] >> off } else { 0 };
            if off + c as usize > 64 && limb + 1 < 4 {
                raw |= s[limb + 1] << (64 - off);
            }
            let raw = (raw & mask) + carry;
            let (mag, neg) = if raw > half as u64 {
                carry = 1;
                ((full - raw) as u32, true)
            } else {
                carry = 0;
                (raw as u32, false)
            };
            let e = w * n + i;
            unsafe {
                if mag == 0 {
                    *keys.add(e) = NONE;
                } else {
                    let key = (w as u32) * half + mag - 1;
                    *keys.add(e) = key;
                    *vals.add(e) = i as u32 | if neg { SIGN } else { 0 };
                    atomic_add(counts.add(key as usize), 1);
                }
            }
        }
        i += gsize();
    }
}

/// Block exclusive scan, 4 elements per thread, 256 threads, 1024 elements per block.
/// `tmp` holds one u32 per thread of the grid. Block totals go to `sums[block]`, the
/// count of nonzero inputs is added to `*nonzero`.
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn scan_blocks(
    inp: *const u32,
    out: *mut u32,
    n: u32,
    tmp: *mut u32,
    sums: *mut u32,
    nonzero: *mut u32,
    maxv: *mut u32,
) {
    let n = n as usize;
    let base = bid() * 1024 + tid() * 4;
    let mut v = [0u32; 4];
    let mut nz = 0u32;
    let mut tot = 0u32;
    let mut mx = 0u32;
    for j in 0..4 {
        if base + j < n {
            v[j] = unsafe { *inp.add(base + j) };
        }
        nz += (v[j] != 0) as u32;
        tot += v[j];
        mx = core::cmp::max(mx, v[j]);
    }
    unsafe {
        *tmp.add(gid()) = tot;
        if nz != 0 {
            atomic_add(nonzero, nz);
            AtomicU32::from_ptr(maxv).fetch_max(mx, Relaxed);
        }
    }
    sync();
    if tid() == 0 {
        let mut acc = 0u32;
        for t in 0..bdim() {
            let p = unsafe { tmp.add(bid() * bdim() + t) };
            let x = unsafe { *p };
            unsafe { *p = acc };
            acc += x;
        }
        unsafe { *sums.add(bid()) = acc };
    }
    sync();
    let mut acc = unsafe { *tmp.add(gid()) };
    for j in 0..4 {
        if base + j < n {
            unsafe { *out.add(base + j) = acc };
        }
        acc += v[j];
    }
}

/// Exclusive scan of the block totals in one thread; the grand total goes to sums[nb].
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn scan_sums(sums: *mut u32, nb: u32) {
    if gid() != 0 {
        return;
    }
    let mut acc = 0u32;
    for b in 0..nb as usize {
        unsafe {
            let x = *sums.add(b);
            *sums.add(b) = acc;
            acc += x;
        }
    }
    unsafe { *sums.add(nb as usize) = acc };
}

#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn scan_add(out: *mut u32, n: u32, sums: *const u32) {
    let mut i = gid();
    while i < n as usize {
        unsafe { *out.add(i) += *sums.add(i / 1024) };
        i += gsize();
    }
}

/// Counting-sort scatter by key; `cursor` starts as the bucket offsets.
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn msm_scatter(
    keys: *const u32,
    vals: *const u32,
    m: u32,
    cursor: *mut u32,
    skeys: *mut u32,
    svals: *mut u32,
) {
    let mut e = gid();
    while e < m as usize {
        unsafe {
            let key = *keys.add(e);
            if key != NONE {
                let pos = atomic_add(cursor.add(key as usize), 1) as usize;
                *skeys.add(pos) = key;
                *svals.add(pos) = *vals.add(e);
            }
        }
        e += gsize();
    }
}

/// Chunk t covers sorted entries [t k - shift, (t + 1) k - shift) clipped to [0, m).
/// Rounds alternate the shift so a run cut by a chunk border is whole in the next round.
#[inline(always)]
fn chunk(t: usize, k: u32, shift: u32, m: u32) -> (usize, usize) {
    let lo = (t * k as usize).saturating_sub(shift as usize);
    let hi = core::cmp::min((t + 1) * k as usize - shift as usize, m as usize);
    (lo, hi)
}

/// Number of key runs in each chunk of `k` consecutive entries.
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn seg_count(
    keys: *const u32,
    m: u32,
    k: u32,
    shift: u32,
    cnt: *mut u32,
    nt: u32,
) {
    let t = gid();
    if t >= nt as usize {
        return;
    }
    let (lo, hi) = chunk(t, k, shift, m);
    let mut c = 0u32;
    let mut prev = NONE;
    for j in lo..hi {
        let key = unsafe { *keys.add(j) };
        c += (j == lo || key != prev) as u32;
        prev = key;
    }
    unsafe { *cnt.add(t) = c };
}

/// First round: sorted (key, point index | sign) to partial sums, one per key run per chunk.
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn seg_reduce_affine(
    skeys: *const u32,
    svals: *const u32,
    m: u32,
    k: u32,
    shift: u32,
    nt: u32,
    bases: *const Affine,
    off: *const u32,
    okeys: *mut u32,
    opts: *mut Xyzz,
) {
    let t = gid();
    if t >= nt as usize {
        return;
    }
    let (lo, hi) = chunk(t, k, shift, m);
    if lo >= hi {
        return;
    }
    let mut o = unsafe { *off.add(t) } as usize;
    let mut cur = unsafe { *skeys.add(lo) };
    let mut acc = INF;
    for j in lo..hi {
        let key = unsafe { *skeys.add(j) };
        if key != cur {
            unsafe {
                *okeys.add(o) = cur;
                *opts.add(o) = acc;
            }
            o += 1;
            cur = key;
            acc = INF;
        }
        let v = unsafe { *svals.add(j) };
        let mut p = unsafe { *bases.add((v & !SIGN) as usize) };
        if v & SIGN != 0 {
            p = p.neg();
        }
        acc = acc.add_affine(&p);
    }
    unsafe {
        *okeys.add(o) = cur;
        *opts.add(o) = acc;
    }
}

/// Later rounds: sorted (key, partial) to fewer partials.
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn seg_reduce_xyzz(
    keys: *const u32,
    pts: *const Xyzz,
    m: u32,
    k: u32,
    shift: u32,
    nt: u32,
    off: *const u32,
    okeys: *mut u32,
    opts: *mut Xyzz,
) {
    let t = gid();
    if t >= nt as usize {
        return;
    }
    let (lo, hi) = chunk(t, k, shift, m);
    if lo >= hi {
        return;
    }
    let mut o = unsafe { *off.add(t) } as usize;
    let mut cur = unsafe { *keys.add(lo) };
    let mut acc = INF;
    for j in lo..hi {
        let key = unsafe { *keys.add(j) };
        if key != cur {
            unsafe {
                *okeys.add(o) = cur;
                *opts.add(o) = acc;
            }
            o += 1;
            cur = key;
            acc = INF;
        }
        acc = acc.add(unsafe { &*pts.add(j) });
    }
    unsafe {
        *okeys.add(o) = cur;
        *opts.add(o) = acc;
    }
}

/// Final pass of the bucket accumulation: the thread at the start of each key run sums
/// the run (the host guarantees runs are short) and writes the dense bucket array.
/// Buckets start zeroed (infinity).
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn run_scatter(
    keys: *const u32,
    pts: *const Xyzz,
    l: u32,
    buckets: *mut Xyzz,
) {
    let l = l as usize;
    let mut j = gid();
    while j < l {
        let key = unsafe { *keys.add(j) };
        if j == 0 || unsafe { *keys.add(j - 1) } != key {
            let mut acc = unsafe { *pts.add(j) };
            let mut t = j + 1;
            while t < l && unsafe { *keys.add(t) } == key {
                acc = acc.add(unsafe { &*pts.add(t) });
                t += 1;
            }
            unsafe { *buckets.add(key as usize) = acc };
        }
        j += gsize();
    }
}

/// One level of the weighted bucket sum. Window w holds `cnt` points at
/// `inp[w stride + off ..]`; thread (w, s) takes elements [s len, s len + len) and returns
/// U = sum (i + 1) B_(s len + i) and R = sum B_(s len + i) (running sums, no scalar
/// multiplication). The host combines sum (e + 1) B_e = sum U_s + len sum s R_s and
/// recurses on R.
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn wsum_level(
    inp: *const Xyzz,
    nw: u32,
    cnt: u32,
    stride: u32,
    off: u32,
    len: u32,
    segs: u32,
    out_u: *mut Xyzz,
    out_r: *mut Xyzz,
) {
    let segs = segs as usize;
    let j = gid();
    if j >= segs * nw as usize {
        return;
    }
    let w = j / segs;
    let s = j % segs;
    let lo = s * len as usize;
    let hi = core::cmp::min(lo + len as usize, cnt as usize);
    let base = unsafe { inp.add(w * stride as usize + off as usize) };
    let mut run = INF;
    let mut sum = INF;
    let mut b = hi;
    while b > lo {
        b -= 1;
        run = run.add(unsafe { &*base.add(b) });
        sum = sum.add(&run);
    }
    unsafe {
        *out_u.add(j) = sum;
        *out_r.add(j) = run;
    }
}

/// Plain sums in groups of g: window w has `cnt` points at inp[w stride ..];
/// out[w ceil(cnt / g) + t] = sum of the t-th group.
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sum_rows(
    inp: *const Xyzz,
    nw: u32,
    cnt: u32,
    stride: u32,
    g: u32,
    out: *mut Xyzz,
) {
    let per = (cnt as usize).div_ceil(g as usize);
    let j = gid();
    if j >= per * nw as usize {
        return;
    }
    let w = j / per;
    let t = j % per;
    let lo = t * g as usize;
    let hi = core::cmp::min(lo + g as usize, cnt as usize);
    let base = unsafe { inp.add(w * stride as usize) };
    let mut acc = INF;
    for e in lo..hi {
        acc = acc.add(unsafe { &*base.add(e) });
    }
    unsafe { *out.add(j) = acc };
}

/// Calibration: 8 independent u32 multiply-add chains, `reps` rounds.
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn imad_test(out: *mut u32, reps: u32) {
    let i = gid() as u32;
    let mut x = [i, i ^ 1, i ^ 2, i ^ 3, i ^ 4, i ^ 5, i ^ 6, i ^ 7];
    for _ in 0..reps {
        for j in 0..8 {
            x[j] = x[j].wrapping_mul(0x9e37_79b9).wrapping_add(reps);
        }
    }
    let mut s = 0u32;
    for j in 0..8 {
        s ^= x[j];
    }
    unsafe { *out.add(gid()) = s };
}

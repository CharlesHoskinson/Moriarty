// SPDX-License-Identifier: Apache-2.0
//! FFI to the CUDA core (cuda/mori_gpu.cu). Every GPU call holds `GPU`, one at a time.
use std::{
    collections::HashMap,
    ffi::{CStr, c_char, c_void},
    sync::{Mutex, OnceLock},
};

use group::{Curve, prime::PrimeCurveAffine};
use midnight_curves::{Fq, G1Affine, G1Projective};

use crate::GpuInfo;

#[allow(dead_code)]
unsafe extern "C" {
    fn mori_gpu_device_count(
        name: *mut c_char,
        len: usize,
        sm: *mut i32,
        major: *mut i32,
        minor: *mut i32,
    ) -> i32;
    pub(crate) fn mori_gpu_last_error() -> *const c_char;
    pub(crate) fn mori_gpu_mem_info(free: *mut usize, total: *mut usize) -> i32;
    fn mori_msm_ctx_new(points: *const c_void, n: usize) -> *mut c_void;
    fn mori_msm_ctx_run(
        ctx: *mut c_void,
        out: *mut c_void,
        scalars: *const c_void,
        n: usize,
    ) -> i32;
    fn mori_ntt(inout: *mut c_void, lg: u32, inverse: i32) -> i32;
    pub(crate) fn mori_dev_alloc(bytes: usize) -> *mut c_void;
    pub(crate) fn mori_dev_free(p: *mut c_void);
    pub(crate) fn mori_dtoh(dst: *mut c_void, src: *const c_void, bytes: usize) -> i32;
    pub(crate) fn mori_htod(dst: *mut c_void, src: *const c_void, bytes: usize) -> i32;
    pub(crate) fn mori_dev_sync() -> i32;
    pub(crate) fn mori_ntt_dev(d: *mut c_void, lg: u32, inverse: i32) -> i32;
    pub(crate) fn mori_coeff_to_extended_dev(
        dst: *mut c_void,
        src: *const c_void,
        k: u32,
        ext_k: u32,
        zeta2: *const c_void,
    ) -> i32;
    pub(crate) fn mori_distribute_zeta_dev(d: *mut c_void, n: usize, zeta2: *const c_void) -> i32;
    pub(crate) fn mori_scale_dev(d: *mut c_void, n: usize, c: *const c_void) -> i32;
    pub(crate) fn mori_mul_periodic_dev(
        d: *mut c_void,
        n: usize,
        t: *const c_void,
        tn: usize,
    ) -> i32;
    pub(crate) fn mori_quotient_run(job: *const c_void) -> i32;
}

/// Serialises every GPU call (sppark's streams are per device, not per thread).
pub(crate) static GPU: Mutex<()> = Mutex::new(());

pub(crate) fn last_error() -> String {
    unsafe { CStr::from_ptr(mori_gpu_last_error()) }
        .to_string_lossy()
        .into_owned()
}

pub(crate) fn info() -> Option<&'static GpuInfo> {
    static INFO: OnceLock<Option<GpuInfo>> = OnceLock::new();
    INFO.get_or_init(|| {
        if std::env::var("MORI_GPU").as_deref() == Ok("off") {
            return None;
        }
        let mut name = [0 as c_char; 128];
        let (mut sm, mut ma, mut mi) = (0, 0, 0);
        let _g = GPU.lock().unwrap();
        let n = unsafe {
            mori_gpu_device_count(name.as_mut_ptr(), name.len(), &mut sm, &mut ma, &mut mi)
        };
        (n > 0).then(|| GpuInfo {
            name: unsafe { CStr::from_ptr(name.as_ptr()) }
                .to_string_lossy()
                .into_owned(),
            sm_count: sm,
            cc: (ma, mi),
            arch: env!("MORI_GPU_ARCH"),
            nvcc: env!("MORI_GPU_NVCC"),
        })
    })
    .as_ref()
}

struct Ctx(*mut c_void, [u8; 96], [u8; 96]);
unsafe impl Send for Ctx {}

/// Base tables keyed by (address, length) of the caller's projective bases, checked
/// against the first and last point so a reused allocation is never served stale.
static TABLES: Mutex<Option<HashMap<(usize, usize), Ctx>>> = Mutex::new(None);

fn fingerprint(p: &G1Projective) -> [u8; 96] {
    let a = p.to_affine();
    let mut out = [0u8; 96];
    let raw = unsafe { std::slice::from_raw_parts(&a as *const G1Affine as *const u8, 96) };
    out.copy_from_slice(raw);
    out
}

pub(crate) fn msm_g1(scalars: &[Fq], bases: &[G1Projective]) -> Option<G1Projective> {
    const _: () = assert!(std::mem::size_of::<G1Affine>() == 96);
    const _: () = assert!(std::mem::size_of::<G1Projective>() == 144);
    let t0 = std::time::Instant::now();
    let key = (bases.as_ptr() as usize, bases.len());
    let (f0, f1) = (fingerprint(&bases[0]), fingerprint(&bases[bases.len() - 1]));
    let mut tables = TABLES.lock().unwrap();
    let tables = tables.get_or_insert_with(HashMap::new);
    let fresh = match tables.get(&key) {
        Some(c) => c.1 != f0 || c.2 != f1,
        None => true,
    };
    let _g = GPU.lock().unwrap();
    if fresh {
        crate::note_table_build();
        if std::env::var_os("MORI_GPU_DEBUG").is_some() {
            eprintln!(
                "mori-gpu: msm table build key={key:?} cached={}",
                tables.len()
            );
        }
        if let Some(old) = tables.remove(&key) {
            unsafe { crate::ffi::mori_msm_ctx_free(old.0) };
            crate::dev_sub(key.1 as u64 * 96);
        }
        let mut aff = vec![G1Affine::identity(); bases.len()];
        G1Projective::batch_normalize(bases, &mut aff);
        let ctx = unsafe { mori_msm_ctx_new(aff.as_ptr() as *const c_void, aff.len()) };
        if ctx.is_null() {
            eprintln!("mori-gpu: msm table: {}", last_error());
            return None;
        }
        crate::dev_add(bases.len() as u64 * 96);
        tables.insert(key, Ctx(ctx, f0, f1));
    }
    let ctx = tables.get(&key).unwrap().0;
    let t1 = std::time::Instant::now();
    let run = |s: &[Fq]| -> Option<G1Projective> {
        let mut out = G1Projective::default();
        let rc = unsafe {
            mori_msm_ctx_run(
                ctx,
                &mut out as *mut G1Projective as *mut c_void,
                s.as_ptr() as *const c_void,
                s.len(),
            )
        };
        if rc != 0 {
            eprintln!("mori-gpu: msm: {}", last_error());
            return None;
        }
        Some(out)
    };
    let out = match split_heavy(scalars) {
        None => run(scalars)?,
        Some((s1, s2)) => {
            crate::note_heavy_split();
            run(&s1)? + run(&s2)?
        }
    };
    if std::env::var_os("MORI_GPU_DEBUG").is_some() {
        use ff::Field;
        let zeros = scalars.iter().filter(|s| bool::from(s.is_zero())).count();
        let mut v: Vec<[u8; 32]> = scalars.iter().map(|s| ff::PrimeField::to_repr(s)).collect();
        v.sort_unstable();
        let (mut best, mut run_len, mut distinct) = (0usize, 0usize, 0usize);
        for i in 0..v.len() {
            if i == 0 || v[i] != v[i - 1] {
                run_len = 1;
                distinct += 1;
            } else {
                run_len += 1;
            }
            best = best.max(run_len);
        }
        let small = scalars
            .iter()
            .filter(|s| ff::PrimeField::to_repr(*s)[8..].iter().all(|b| *b == 0))
            .count();
        eprintln!(
            "mori-gpu: msm n={} total={:?} run={:?} zeros={zeros} distinct={distinct} top_mult={best} small64={small}",
            scalars.len(),
            t0.elapsed(),
            t1.elapsed()
        );
    }
    Some(out)
}

#[allow(dead_code)]
unsafe extern "C" {
    fn mori_msm_ctx_free(ctx: *mut c_void);
}

/// Bucket skew guard. sppark accumulates each bucket in one thread, so a non-zero
/// scalar repeated many times (a constant column, a lookup fill value) sends most
/// points of every window to one bucket: measured 7 to 9 s for one 2^16 MSM against
/// 20 ms normally. For each heavy value v at positions S, write v = (v - r_i) + r_i
/// with r_i = c * (i + 1) for a fixed c: MSM(s') + MSM(t) with s'_i = v - r_i,
/// t_i = r_i on S (zero elsewhere) equals the original sum, and neither vector
/// repeats a value. Zero scalars are cheap (skipped digits) and are left alone.
fn split_heavy(s: &[Fq]) -> Option<(Vec<Fq>, Vec<Fq>)> {
    use ff::Field;
    let n = s.len();
    if n < 4096 {
        return None;
    }
    // Candidates from 128 evenly spaced samples; heavy = more than n / 64 copies.
    let step = n / 128;
    let mut cands: Vec<Fq> = Vec::new();
    for j in 0..128 {
        let v = s[j * step];
        if bool::from(v.is_zero()) || cands.contains(&v) {
            continue;
        }
        let hits = (0..128).filter(|&t| s[t * step] == v).count();
        if hits >= 4 && cands.len() < 8 {
            cands.push(v);
        }
    }
    if cands.is_empty() {
        return None;
    }
    let heavy: Vec<Fq> = cands
        .into_iter()
        .filter(|v| s.iter().filter(|x| *x == v).count() > n / 64)
        .collect();
    if heavy.is_empty() {
        return None;
    }
    let c = Fq::from_raw([
        0x9e37_79b9_7f4a_7c15,
        0xbf58_476d_1ce4_e5b9,
        0x94d0_49bb_1331_11eb,
        0x2545_f491_4f6c_dd1d,
    ]);
    let mut s1 = s.to_vec();
    let mut s2 = vec![Fq::ZERO; n];
    let mut r = Fq::ZERO;
    for i in 0..n {
        r += c;
        if heavy.contains(&s[i]) {
            s1[i] = s[i] - r;
            s2[i] = r;
        }
    }
    Some((s1, s2))
}

/// Grow-only device scratch for host-to-host transforms (held under `GPU`).
struct Scratch(*mut c_void, usize);
unsafe impl Send for Scratch {}
static SCRATCH: Mutex<Scratch> = Mutex::new(Scratch(std::ptr::null_mut(), 0));

fn scratch(bytes: usize) -> Option<*mut c_void> {
    let mut s = SCRATCH.lock().unwrap();
    if s.1 < bytes {
        unsafe { mori_dev_free(s.0) };
        crate::dev_sub(s.1 as u64);
        let p = unsafe { mori_dev_alloc(bytes) };
        if p.is_null() {
            *s = Scratch(std::ptr::null_mut(), 0);
            return None;
        }
        *s = Scratch(p, bytes);
        crate::dev_add(bytes as u64);
    }
    Some(s.0)
}

/// Frees the transform scratch (between proofs, to return memory).
pub fn release_scratch() {
    let _g = GPU.lock().unwrap();
    let mut s = SCRATCH.lock().unwrap();
    unsafe { mori_dev_free(s.0) };
    crate::dev_sub(s.1 as u64);
    *s = Scratch(std::ptr::null_mut(), 0);
}

pub(crate) fn ntt(a: &mut [Fq], lg: u32, inverse: bool, post: Option<Fq>) -> bool {
    let _g = GPU.lock().unwrap();
    let bytes = a.len() * 32;
    let Some(d) = scratch(bytes) else {
        return false;
    };
    let p = a.as_mut_ptr() as *mut c_void;
    let mut rc = unsafe { mori_htod(d, p, bytes) };
    if rc == 0 {
        rc = unsafe { mori_ntt_dev(d, lg, inverse as i32) };
    }
    if rc == 0 {
        if let Some(c) = post {
            rc = unsafe { mori_scale_dev(d, a.len(), &c as *const Fq as *const c_void) };
        }
    }
    if rc == 0 {
        rc = unsafe { mori_dtoh(p, d, bytes) };
    }
    if rc != 0 {
        eprintln!("mori-gpu: ntt: {}", last_error());
    }
    rc == 0
}

pub(crate) fn coeff_to_extended(
    coeffs: &[Fq],
    out: &mut [Fq],
    k: u32,
    ext_k: u32,
    zeta: [Fq; 2],
) -> bool {
    let _g = GPU.lock().unwrap();
    let bytes = out.len() * 32;
    let Some(d) = scratch(bytes) else {
        return false;
    };
    let rc = unsafe {
        mori_coeff_to_extended_dev(
            d,
            coeffs.as_ptr() as *const c_void,
            k,
            ext_k,
            zeta.as_ptr() as *const c_void,
        )
    };
    let rc = if rc == 0 {
        unsafe { mori_dtoh(out.as_mut_ptr() as *mut c_void, d, bytes) }
    } else {
        rc
    };
    if rc != 0 {
        eprintln!("mori-gpu: coeff_to_extended: {}", last_error());
    }
    rc == 0
}

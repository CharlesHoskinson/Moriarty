use std::any::TypeId;
use std::os::raw::{c_char, c_int};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};

use halo2curves_real as real;
use libloading::Library;
use real::ff::PrimeField;
use real::group::Group;

const FR_BYTES: usize = 32;
const FP_BYTES: usize = 48;
const POINT_BYTES: usize = 1 + 2 * FP_BYTES;

pub use real::bls12381;
pub use real::bn256;
pub use real::ff_ext;
pub use real::grumpkin;
pub use real::hash_to_curve;
pub use real::pasta;
pub use real::pluto_eris;
pub use real::secp256k1;
pub use real::secp256r1;
pub use real::secq256k1;
pub use real::serde;

pub use real::ff;
pub use real::group;
pub use real::pairing;
pub use real::{Coordinates, CurveAffine, CurveExt};

#[cfg(test)]
pub mod tests {
    pub use halo2curves_real::tests::*;
}

struct KernelFns {
    _lib: Library,
    mn_msm: mn_msm_fn,
    mn_fft: mn_fft_fn,
    mn_last_error: mn_last_error_fn,
}

fn kernels() -> Result<&'static KernelFns, String> {
    static KERNELS: OnceLock<Result<KernelFns, String>> = OnceLock::new();
    KERNELS
        .get_or_init(|| {
            #[cfg(unix)]
            let lib: Library = unsafe {
                libloading::os::unix::Library::open(
                    Some("libmidnight_prover_cuda_kernels.so"),
                    libloading::os::unix::RTLD_NOW | libloading::os::unix::RTLD_GLOBAL,
                )
            }
            .map_err(|err| format!("failed to load libmidnight_prover_cuda_kernels.so: {err}"))?
            .into();
            #[cfg(not(unix))]
            let lib: Library = unsafe { Library::new("libmidnight_prover_cuda_kernels.so") }
                .map_err(|err| format!("failed to load libmidnight_prover_cuda_kernels.so: {err}"))?;

            let mn_msm = unsafe { lib.get::<mn_msm_fn>(b"mn_msm\0") }
                .map_err(|err| format!("failed to load mn_msm: {err}"))?;
            let mn_msm = *mn_msm;
            let mn_fft = unsafe { lib.get::<mn_fft_fn>(b"mn_fft\0") }
                .map_err(|err| format!("failed to load mn_fft: {err}"))?;
            let mn_fft = *mn_fft;
            let mn_last_error = unsafe { lib.get::<mn_last_error_fn>(b"mn_last_error\0") }
                .map_err(|err| format!("failed to load mn_last_error: {err}"))?;
            let mn_last_error = *mn_last_error;
            Ok(KernelFns {
                _lib: lib,
                mn_msm,
                mn_fft,
                mn_last_error,
            })
        })
        .as_ref()
        .map_err(|err| err.clone())
}

fn cuda_enabled() -> bool {
    std::env::var("MIDNIGHT_PROOFS_CUDA")
        .ok()
        .as_deref()
        .map(|val| val == "1")
        .unwrap_or(false)
}

fn kernels_enabled() -> bool {
    std::env::var("MIDNIGHT_CUDA_USE_KERNELS")
        .ok()
        .as_deref()
        .map(|val| val != "0")
        .unwrap_or(true)
}

fn force_kernels() -> bool {
    std::env::var("MIDNIGHT_CUDA_FORCE_KERNELS")
        .ok()
        .as_deref()
        .map(|val| val == "1")
        .unwrap_or(false)
}

fn trace_enabled() -> bool {
    std::env::var("MIDNIGHT_PROOFS_CUDA_TRACE")
        .ok()
        .as_deref()
        .map(|val| val == "1")
        .unwrap_or(false)
}

fn stats_enabled() -> bool {
    std::env::var("MIDNIGHT_PROOFS_CUDA_STATS")
        .ok()
        .as_deref()
        .map(|val| val == "1")
        .unwrap_or(false)
}

fn stats_every() -> u64 {
    std::env::var("MIDNIGHT_PROOFS_CUDA_STATS_EVERY")
        .ok()
        .and_then(|val| val.parse::<u64>().ok())
        .filter(|val| *val > 0)
        .unwrap_or(100)
}

fn min_msm_len() -> usize {
    std::env::var("MIDNIGHT_PROOFS_CUDA_MIN_MSM")
        .ok()
        .and_then(|val| val.parse::<usize>().ok())
        .unwrap_or(0)
}

fn min_fft_len() -> usize {
    std::env::var("MIDNIGHT_PROOFS_CUDA_MIN_FFT")
        .ok()
        .and_then(|val| val.parse::<usize>().ok())
        .unwrap_or(0)
}

fn repr_len<F: real::ff::PrimeField>() -> usize {
    F::Repr::default().as_ref().len()
}

fn last_error(fns: &KernelFns) -> String {
    let ptr = unsafe { (fns.mn_last_error)() };
    if ptr.is_null() {
        return "CUDA kernel error".to_string();
    }
    unsafe { std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}

#[repr(C)]
struct mn_msm_input {
    scalars: *const u8,
    scalars_len: usize,
    points: *const u8,
    points_len: usize,
    out: *mut u8,
    out_len: usize,
}

#[repr(C)]
struct mn_fft_input {
    values: *mut u8,
    values_len: usize,
    omega: *const u8,
    log_n: u32,
}

#[allow(non_camel_case_types)]
type mn_msm_fn = unsafe extern "C" fn(input: *const mn_msm_input) -> c_int;
#[allow(non_camel_case_types)]
type mn_fft_fn = unsafe extern "C" fn(input: *mut mn_fft_input) -> c_int;
#[allow(non_camel_case_types)]
type mn_last_error_fn = unsafe extern "C" fn() -> *const c_char;

static MSM_CALLS: AtomicU64 = AtomicU64::new(0);
static MSM_CUDA_CALLS: AtomicU64 = AtomicU64::new(0);
static MSM_CPU_TIME_NS: AtomicU64 = AtomicU64::new(0);
static MSM_CUDA_TIME_NS: AtomicU64 = AtomicU64::new(0);

static FFT_CALLS: AtomicU64 = AtomicU64::new(0);
static FFT_CUDA_CALLS: AtomicU64 = AtomicU64::new(0);
static FFT_CPU_TIME_NS: AtomicU64 = AtomicU64::new(0);
static FFT_CUDA_TIME_NS: AtomicU64 = AtomicU64::new(0);

fn record_msm(cuda: bool, len: usize, elapsed_ns: u64) {
    let calls = MSM_CALLS.fetch_add(1, Ordering::Relaxed) + 1;
    if cuda {
        MSM_CUDA_CALLS.fetch_add(1, Ordering::Relaxed);
        MSM_CUDA_TIME_NS.fetch_add(elapsed_ns, Ordering::Relaxed);
    } else {
        MSM_CPU_TIME_NS.fetch_add(elapsed_ns, Ordering::Relaxed);
    }
    if trace_enabled() {
        eprintln!("cuda-msm: len={len} cuda={cuda} time_ns={elapsed_ns}");
    }
    if stats_enabled() && calls % stats_every() == 0 {
        let cuda_calls = MSM_CUDA_CALLS.load(Ordering::Relaxed);
        let cpu_time = MSM_CPU_TIME_NS.load(Ordering::Relaxed);
        let cuda_time = MSM_CUDA_TIME_NS.load(Ordering::Relaxed);
        let avg_cpu = if calls > cuda_calls {
            cpu_time / (calls - cuda_calls)
        } else {
            0
        };
        let avg_cuda = if cuda_calls > 0 {
            cuda_time / cuda_calls
        } else {
            0
        };
        eprintln!(
            "cuda-msm-stats: calls={} cuda_calls={} avg_cpu_ns={} avg_cuda_ns={}",
            calls, cuda_calls, avg_cpu, avg_cuda
        );
    }
}

fn record_fft(cuda: bool, len: usize, elapsed_ns: u64) {
    let calls = FFT_CALLS.fetch_add(1, Ordering::Relaxed) + 1;
    if cuda {
        FFT_CUDA_CALLS.fetch_add(1, Ordering::Relaxed);
        FFT_CUDA_TIME_NS.fetch_add(elapsed_ns, Ordering::Relaxed);
    } else {
        FFT_CPU_TIME_NS.fetch_add(elapsed_ns, Ordering::Relaxed);
    }
    if trace_enabled() {
        eprintln!("cuda-fft: len={len} cuda={cuda} time_ns={elapsed_ns}");
    }
    if stats_enabled() && calls % stats_every() == 0 {
        let cuda_calls = FFT_CUDA_CALLS.load(Ordering::Relaxed);
        let cpu_time = FFT_CPU_TIME_NS.load(Ordering::Relaxed);
        let cuda_time = FFT_CUDA_TIME_NS.load(Ordering::Relaxed);
        let avg_cpu = if calls > cuda_calls {
            cpu_time / (calls - cuda_calls)
        } else {
            0
        };
        let avg_cuda = if cuda_calls > 0 {
            cuda_time / cuda_calls
        } else {
            0
        };
        eprintln!(
            "cuda-fft-stats: calls={} cuda_calls={} avg_cpu_ns={} avg_cuda_ns={}",
            calls, cuda_calls, avg_cpu, avg_cuda
        );
    }
}

fn try_msm_cuda<C>(scalars: &[C::Scalar], bases: &[C]) -> Result<Option<C::Curve>, String>
where
    C: CurveAffine,
    C::Scalar: real::ff::PrimeField,
    C::Base: real::ff::PrimeField,
    C::Curve: real::group::Group,
{
    if !cuda_enabled() || !kernels_enabled() {
        return Ok(None);
    }
    if scalars.len() < min_msm_len() {
        return Ok(None);
    }
    if repr_len::<C::Scalar>() != FR_BYTES || repr_len::<C::Base>() != FP_BYTES {
        return Ok(None);
    }
    let fns = match kernels() {
        Ok(fns) => fns,
        Err(err) => {
            if force_kernels() {
                return Err(err);
            }
            return Ok(None);
        }
    };
    if scalars.len() != bases.len() {
        return Err("msm scalar/base length mismatch".to_string());
    }
    if scalars.is_empty() {
        return Ok(Some(C::Curve::identity()));
    }

    let mut scalars_bytes = Vec::with_capacity(scalars.len() * FR_BYTES);
    for scalar in scalars {
        scalars_bytes.extend_from_slice(scalar.to_repr().as_ref());
    }
    let mut points_bytes = Vec::with_capacity(bases.len() * POINT_BYTES);
    for base in bases {
        let mut out = [0u8; POINT_BYTES];
        if base.is_identity().into() {
            out[0] = 1;
        } else {
            let coords = base
                .coordinates()
                .into_option()
                .ok_or_else(|| "failed to read curve coordinates".to_string())?;
            let x = coords.x().to_repr();
            let y = coords.y().to_repr();
            if x.as_ref().len() != FP_BYTES || y.as_ref().len() != FP_BYTES {
                return Ok(None);
            }
            out[1..1 + FP_BYTES].copy_from_slice(x.as_ref());
            out[1 + FP_BYTES..].copy_from_slice(y.as_ref());
        }
        points_bytes.extend_from_slice(&out);
    }

    let mut out = [0u8; POINT_BYTES];
    let input = mn_msm_input {
        scalars: scalars_bytes.as_ptr(),
        scalars_len: scalars.len(),
        points: points_bytes.as_ptr(),
        points_len: bases.len(),
        out: out.as_mut_ptr(),
        out_len: out.len(),
    };
    let rc = unsafe { (fns.mn_msm)(&input) };
    if rc != 0 {
        let err = last_error(fns);
        if force_kernels() {
            return Err(err);
        }
        if trace_enabled() {
            eprintln!("cuda-msm: kernel failed, falling back: {err}");
        }
        return Ok(None);
    }

    if out[0] != 0 {
        return Ok(Some(C::Curve::identity()));
    }
    let mut x_repr = <C::Base as real::ff::PrimeField>::Repr::default();
    let mut y_repr = <C::Base as real::ff::PrimeField>::Repr::default();
    x_repr.as_mut().copy_from_slice(&out[1..1 + FP_BYTES]);
    y_repr
        .as_mut()
        .copy_from_slice(&out[1 + FP_BYTES..]);
    let x = C::Base::from_repr(x_repr)
        .into_option()
        .ok_or_else(|| "invalid MSM output x".to_string())?;
    let y = C::Base::from_repr(y_repr)
        .into_option()
        .ok_or_else(|| "invalid MSM output y".to_string())?;
    let affine = C::from_xy(x, y)
        .into_option()
        .ok_or_else(|| "invalid MSM output point".to_string())?;
    Ok(Some(affine.to_curve()))
}

fn try_fft_cuda<Scalar, G>(a: &mut [G], omega: Scalar, log_n: u32) -> Result<bool, String>
where
    Scalar: real::ff::PrimeField,
    G: real::fft::FftGroup<Scalar>,
{
    if !cuda_enabled() || !kernels_enabled() {
        return Ok(false);
    }
    if a.len() < min_fft_len() {
        return Ok(false);
    }
    if repr_len::<Scalar>() != FR_BYTES {
        return Ok(false);
    }
    if TypeId::of::<G>() != TypeId::of::<Scalar>() {
        return Ok(false);
    }
    let fns = match kernels() {
        Ok(fns) => fns,
        Err(err) => {
            if force_kernels() {
                return Err(err);
            }
            return Ok(false);
        }
    };

    let scalars = unsafe { &mut *(a as *mut [G] as *mut [Scalar]) };
    let mut bytes = Vec::with_capacity(scalars.len() * FR_BYTES);
    for value in scalars.iter() {
        bytes.extend_from_slice(value.to_repr().as_ref());
    }
    let omega_bytes = omega.to_repr();
    let mut input = mn_fft_input {
        values: bytes.as_mut_ptr(),
        values_len: scalars.len(),
        omega: omega_bytes.as_ref().as_ptr(),
        log_n,
    };
    let rc = unsafe { (fns.mn_fft)(&mut input) };
    if rc != 0 {
        let err = last_error(fns);
        if force_kernels() {
            return Err(err);
        }
        if trace_enabled() {
            eprintln!("cuda-fft: kernel failed, falling back: {err}");
        }
        return Ok(false);
    }

    for (idx, value) in scalars.iter_mut().enumerate() {
        let start = idx * FR_BYTES;
        let mut repr = Scalar::Repr::default();
        repr.as_mut().copy_from_slice(&bytes[start..start + FR_BYTES]);
        *value = Scalar::from_repr(repr)
            .into_option()
            .ok_or_else(|| "invalid FFT output".to_string())?;
    }

    Ok(true)
}

pub mod msm {
    pub use crate::real::msm::{msm_parallel, msm_serial};
    use crate::real;
    use crate::CurveAffine;

    pub fn msm_best<C: CurveAffine>(coeffs: &[C::Scalar], bases: &[C]) -> C::Curve
    where
        C::Scalar: real::ff::PrimeField,
        C::Base: real::ff::PrimeField,
        C::Curve: real::group::Group,
    {
        let start = std::time::Instant::now();
        match crate::try_msm_cuda::<C>(coeffs, bases) {
            Ok(Some(result)) => {
                crate::record_msm(true, coeffs.len(), start.elapsed().as_nanos() as u64);
                result
            }
            Ok(None) => {
                let cpu_start = std::time::Instant::now();
                let res = real::msm::msm_best(coeffs, bases);
                crate::record_msm(false, coeffs.len(), cpu_start.elapsed().as_nanos() as u64);
                res
            }
            Err(err) => panic!("CUDA MSM failed: {err}"),
        }
    }
}

pub mod fft {
    pub use crate::real::fft::recursive_butterfly_arithmetic;
    pub use crate::real::fft::FftGroup;
    use crate::real;
    use crate::real::ff::PrimeField;

    pub fn best_fft<Scalar: PrimeField, G: FftGroup<Scalar>>(
        a: &mut [G],
        omega: Scalar,
        log_n: u32,
    ) {
        let start = std::time::Instant::now();
        match crate::try_fft_cuda(a, omega, log_n) {
            Ok(true) => {
                crate::record_fft(true, a.len(), start.elapsed().as_nanos() as u64);
            }
            Ok(false) => {
                let cpu_start = std::time::Instant::now();
                real::fft::best_fft(a, omega, log_n);
                crate::record_fft(false, a.len(), cpu_start.elapsed().as_nanos() as u64);
            }
            Err(err) => panic!("CUDA FFT failed: {err}"),
        }
    }
}

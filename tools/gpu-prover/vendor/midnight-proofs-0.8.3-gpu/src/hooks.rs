//! GPU hooks (Moriarty gpu-wrap patch, design 3.7 / gpu.md F6).
#![allow(unsafe_code)]
//!
//! A registry of optional accelerator functions, consulted by `msm_specific`, the
//! `EvaluationDomain` transforms and `compute_nu_poly`. With nothing registered the
//! crate behaves exactly as registry midnight-proofs 0.8.3. Every hook may decline
//! (returns `None` / `false`), and the stock CPU code then runs. Only the BLS12-381
//! scalar field `Fq` and `G1` are hooked; other types never reach a hook.
use std::{
    any::{Any, TypeId},
    sync::{
        OnceLock,
        atomic::{AtomicBool, Ordering},
    },
};

use midnight_curves::{Fq, G1Projective};

/// A buffer of `len` field elements owned by the accelerator (freed when dropped).
pub struct DevBuf {
    /// Device-readable pointer to the elements.
    pub ptr: *const Fq,
    /// Number of elements.
    pub len: usize,
    /// The accelerator's owner object; dropping it frees the buffer.
    pub owner: Box<dyn Any + Send + Sync>,
}

impl std::fmt::Debug for DevBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DevBuf({:p}, {})", self.ptr, self.len)
    }
}

unsafe impl Send for DevBuf {}
unsafe impl Sync for DevBuf {}

/// The accelerator functions. Each is optional.
#[derive(Default, Clone, Copy, Debug)]
pub struct GpuHooks {
    /// `sum scalars[i] * bases[i]`; `bases` may be longer than `scalars`.
    pub msm_g1: Option<fn(&[Fq], &[G1Projective]) -> Option<G1Projective>>,
    /// `best_fft(a, omega, log_n)`.
    pub fft: Option<fn(&mut [Fq], Fq, u32) -> bool>,
    /// `best_fft(a, omega_inv, log_n)` then `a[i] *= divisor`.
    pub ifft: Option<fn(&mut [Fq], Fq, u32, Fq) -> bool>,
    /// `coeff_to_extended(coeffs)` into `out` (k, extended k, [zeta, zeta^2]).
    pub coeff_to_extended: Option<fn(&[Fq], &mut [Fq], u32, u32, [Fq; 2]) -> bool>,
    /// `coeff_to_extended(coeffs)` into an accelerator-owned buffer.
    pub dev_coeff_to_extended: Option<fn(&[Fq], u32, u32, [Fq; 2]) -> Option<DevBuf>>,
    /// Quotient numerator: runs `job` (graphs need `nslots` scratch slots) into `out`.
    pub quotient: Option<unsafe fn(*mut QuotientJob, u32, &mut [Fq]) -> bool>,
}

static HOOKS: OnceLock<GpuHooks> = OnceLock::new();

/// Registers the accelerator once per process. Returns `false` if already set.
pub fn install(h: GpuHooks) -> bool {
    HOOKS.set(h).is_ok()
}

/// The registered hooks, if any.
pub fn get() -> Option<&'static GpuHooks> {
    HOOKS.get()
}

/// Test-only determinism switch: iterate the lookup leftover table in sorted order
/// instead of `HashMap` order (which depends on a per-process random hash seed).
/// Any leftover order gives a valid proof; a fixed order makes proof bytes a function
/// of (key, witness, RNG seed, rayon pool size). Off by default (stock behaviour).
pub static DETERMINISTIC_LOOKUP: AtomicBool = AtomicBool::new(false);

pub(crate) fn deterministic_lookup() -> bool {
    DETERMINISTIC_LOOKUP.load(Ordering::Relaxed)
}

/// Test-only: when set, `compute_nu_poly` also runs the CPU numerator and compares.
pub static QUOTIENT_CHECK: AtomicBool = AtomicBool::new(false);
static QUOTIENT_CHECK_DIFFS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub(crate) fn quotient_check() -> bool {
    QUOTIENT_CHECK.load(Ordering::Relaxed)
}

pub(crate) fn record_quotient_check(ndiff: usize) {
    QUOTIENT_CHECK_DIFFS.fetch_add(ndiff as u64, Ordering::Relaxed);
}

/// Differing numerator elements seen by the check mode since process start.
pub fn quotient_check_diffs() -> u64 {
    QUOTIENT_CHECK_DIFFS.load(Ordering::Relaxed)
}

/// Shape of the last quotient job: (gate calculations, scratch slots, lookups,
/// permutation sets, device-resident cosets).
pub static LAST_QUOTIENT_SHAPE: std::sync::Mutex<(u32, u32, u32, u32, u32)> = std::sync::Mutex::new((0, 0, 0, 0, 0));

/// Run-time switch for the quotient hook (to measure phases separately).
pub static QUOTIENT_ON: AtomicBool = AtomicBool::new(true);

#[inline]
pub(crate) fn is_fq<F: 'static>() -> bool {
    TypeId::of::<F>() == TypeId::of::<Fq>()
}

#[inline]
pub(crate) fn as_fq_mut<F: 'static>(a: &mut [F]) -> &mut [Fq] {
    debug_assert!(is_fq::<F>());
    unsafe { &mut *(a as *mut [F] as *mut [Fq]) }
}

#[inline]
pub(crate) fn as_fq<F: 'static>(a: &[F]) -> &[Fq] {
    debug_assert!(is_fq::<F>());
    unsafe { &*(a as *const [F] as *const [Fq]) }
}

#[inline]
pub(crate) fn to_fq<F: 'static + Copy>(a: F) -> Fq {
    debug_assert!(is_fq::<F>());
    unsafe { std::mem::transmute_copy(&a) }
}

// ------------------------------------------------------------------ quotient ABI
// C layout of mori-gpu `cuda/quotient.cuh` (QUOTIENT_ABI 1).

/// Job layout version.
pub const QUOTIENT_ABI: u32 = 1;

/// A value source of a graph calculation.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QSrc {
    /// 0 const, 1 intermediate, 2 fixed, 3 advice, 4 instance, 5 challenge, 6 beta,
    /// 7 gamma, 8 theta, 9 trash challenge, 10 y, 11 previous value
    pub kind: u32,
    /// Index (constant, scratch slot, column, challenge).
    pub a: u32,
    /// Rotation (columns).
    pub rot: i32,
}

/// A calculation (`op`: 0 add, 1 sub, 2 mul, 3 square, 4 double, 5 negate, 6 horner,
/// 7 store; horner: `x` start, `y` factor, parts in `parts[off..off+len]`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct QCalc {
    /// Operation.
    pub op: u32,
    /// Scratch slot written.
    pub target: u32,
    /// First operand.
    pub x: QSrc,
    /// Second operand.
    pub y: QSrc,
    /// Horner parts offset.
    pub parts_off: u32,
    /// Horner parts count.
    pub parts_len: u32,
}

/// A flattened `GraphEvaluator`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct QGraph {
    /// Calculations in stock order.
    pub calcs: *const QCalc,
    /// Count.
    pub ncalcs: u32,
    /// Slot of the last calculation.
    pub result_slot: u32,
    /// Horner parts.
    pub parts: *const QSrc,
    /// Constants.
    pub constants: *const Fq,
}

/// One lookup argument.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct QLookup {
    /// Its graph.
    pub graph: QGraph,
    /// Product coset.
    pub product: *const Fq,
    /// Permuted input coset.
    pub permuted_input: *const Fq,
    /// Permuted table coset.
    pub permuted_table: *const Fq,
}

/// One trashcan argument.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct QTrash {
    /// Its graph.
    pub graph: QGraph,
    /// Trash coset.
    pub trash: *const Fq,
    /// Fixed column of the selector.
    pub selector_col: u32,
}

/// The quotient job (pointers must stay live for the call).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(missing_docs)]
pub struct QuotientJob {
    pub out: *mut Fq,
    pub n: u64,
    pub rot_scale: i32,
    pub fixed: *const *const Fq,
    pub advice: *const *const Fq,
    pub instance: *const *const Fq,
    pub challenges: *const Fq,
    pub sc: *const Fq,
    pub gates: QGraph,
    pub nsets: u32,
    pub chunk_len: u32,
    pub ncols: u32,
    pub perm_products: *const *const Fq,
    pub col_kind: *const u32,
    pub col_index: *const u32,
    pub perm_sigma: *const *const Fq,
    pub last_rot: i32,
    pub l0: *const Fq,
    pub l_last: *const Fq,
    pub l_active: *const Fq,
    pub nlookups: u32,
    pub ntrash: u32,
    pub lookups: *const QLookup,
    pub trash: *const QTrash,
    pub scratch: *mut Fq,
    pub stride: u64,
}

// SPDX-License-Identifier: Apache-2.0
//! Quotient numerator on the GPU and device-resident polynomials.
//!
//! The job layout is the C ABI of `cuda/quotient.cuh` (`QUOTIENT_ABI`). The patched
//! midnight-proofs builds the job from its own `Evaluator` (it has a copy of these
//! `repr(C)` types) and passes it as a raw pointer, so this crate does not depend on
//! midnight-proofs. Every pointer in a job must stay valid for the call; on this host
//! (unified memory, `pageableMemoryAccess = 1`) host pointers are device-readable.
#![allow(missing_docs)]

use std::ffi::c_void;

use midnight_curves::Fq;

/// Version of the job layout shared with the patched midnight-proofs.
pub const QUOTIENT_ABI: u32 = 1;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QSrc {
    pub kind: u32,
    pub a: u32,
    pub rot: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct QCalc {
    pub op: u32,
    pub target: u32,
    pub x: QSrc,
    pub y: QSrc,
    pub parts_off: u32,
    pub parts_len: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct QGraph {
    pub calcs: *const QCalc,
    pub ncalcs: u32,
    pub result_slot: u32,
    pub parts: *const QSrc,
    pub constants: *const Fq,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct QLookup {
    pub graph: QGraph,
    pub product: *const Fq,
    pub permuted_input: *const Fq,
    pub permuted_table: *const Fq,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct QTrash {
    pub graph: QGraph,
    pub trash: *const Fq,
    pub selector_col: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
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

/// A polynomial in a device allocation (freed on drop).
pub struct DevPoly {
    ptr: *mut c_void,
    len: usize,
}

unsafe impl Send for DevPoly {}
unsafe impl Sync for DevPoly {}

impl DevPoly {
    pub fn as_ptr(&self) -> *const Fq {
        self.ptr as *const Fq
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Allocates `len` elements on the device.
    pub fn alloc(len: usize) -> Option<DevPoly> {
        #[cfg(feature = "cuda")]
        {
            let _g = crate::ffi::GPU.lock().unwrap();
            let p = unsafe { crate::ffi::mori_dev_alloc(len * 32) };
            if p.is_null() {
                return None;
            }
            crate::dev_add(len as u64 * 32);
            Some(DevPoly { ptr: p, len })
        }
        #[cfg(not(feature = "cuda"))]
        {
            let _ = len;
            None
        }
    }

    /// `EvaluationDomain::coeff_to_extended` into a new device buffer.
    pub fn coeff_to_extended(coeffs: &[Fq], k: u32, ext_k: u32, zeta: [Fq; 2]) -> Option<DevPoly> {
        #[cfg(feature = "cuda")]
        {
            if coeffs.len() != 1usize << k || crate::info().is_none() {
                return None;
            }
            let t = std::time::Instant::now();
            let d = DevPoly::alloc(1usize << ext_k)?;
            let _g = crate::ffi::GPU.lock().unwrap();
            let rc = unsafe {
                crate::ffi::mori_coeff_to_extended_dev(
                    d.ptr,
                    coeffs.as_ptr() as *const c_void,
                    k,
                    ext_k,
                    zeta.as_ptr() as *const c_void,
                )
            };
            let rc = if rc == 0 {
                unsafe { crate::ffi::mori_dev_sync() }
            } else {
                rc
            };
            if rc != 0 {
                eprintln!(
                    "mori-gpu: dev coeff_to_extended: {}",
                    crate::ffi::last_error()
                );
                crate::note_fallback();
                return None;
            }
            crate::note_coset(t.elapsed().as_nanos() as u64);
            Some(d)
        }
        #[cfg(not(feature = "cuda"))]
        {
            let _ = (coeffs, k, ext_k, zeta);
            None
        }
    }

    /// Copies the buffer to a host vector.
    pub fn to_host(&self) -> Option<Vec<Fq>> {
        #[cfg(feature = "cuda")]
        {
            let mut v = vec![Fq::default(); self.len];
            let _g = crate::ffi::GPU.lock().unwrap();
            let rc = unsafe {
                crate::ffi::mori_dtoh(v.as_mut_ptr() as *mut c_void, self.ptr, self.len * 32)
            };
            (rc == 0).then_some(v)
        }
        #[cfg(not(feature = "cuda"))]
        None
    }
}

impl Drop for DevPoly {
    fn drop(&mut self) {
        #[cfg(feature = "cuda")]
        {
            let _g = crate::ffi::GPU.lock().unwrap();
            unsafe { crate::ffi::mori_dev_free(self.ptr) };
            crate::dev_sub(self.len as u64 * 32);
        }
    }
}

/// Threads in the quotient grid (`MORI_GPU_Q_STRIDE`, default sm_count x 1024).
pub fn grid_threads() -> u64 {
    if let Some(s) = std::env::var("MORI_GPU_Q_STRIDE")
        .ok()
        .and_then(|s| s.parse().ok())
    {
        return s;
    }
    crate::info().map_or(0, |i| i.sm_count as u64 * 1024)
}

/// Runs a quotient job whose graphs use at most `nslots` scratch slots, writing `n`
/// values into `out` (host). Returns `false` (nothing written) if not taken.
///
/// # Safety
/// `job` must point to a valid `QuotientJob` whose pointers are live and
/// device-readable; `out` must hold `job.n` elements.
pub unsafe fn run(job: *mut QuotientJob, nslots: u32, out: &mut [Fq]) -> bool {
    #[cfg(feature = "cuda")]
    {
        let j = unsafe { &mut *job };
        if crate::info().is_none() || out.len() as u64 != j.n {
            return false;
        }
        let t = std::time::Instant::now();
        let stride = grid_threads();
        let Some(dout) = DevPoly::alloc(out.len()) else {
            return false;
        };
        let Some(scr) = DevPoly::alloc((nslots.max(1) as usize) * stride as usize) else {
            return false;
        };
        j.out = dout.ptr as *mut Fq;
        j.scratch = scr.ptr as *mut Fq;
        j.stride = stride;
        let _g = crate::ffi::GPU.lock().unwrap();
        let mut rc = unsafe { crate::ffi::mori_quotient_run(job as *const c_void) };
        if rc == 0 {
            rc = unsafe { crate::ffi::mori_dev_sync() };
        }
        if rc == 0 {
            rc = unsafe {
                crate::ffi::mori_dtoh(out.as_mut_ptr() as *mut c_void, dout.ptr, out.len() * 32)
            };
        }
        drop(_g);
        if rc != 0 {
            eprintln!("mori-gpu: quotient: {}", crate::ffi::last_error());
            crate::note_fallback();
            return false;
        }
        crate::add_quotient_time(t.elapsed().as_nanos() as u64);
        true
    }
    #[cfg(not(feature = "cuda"))]
    {
        let _ = (job, nslots, out);
        false
    }
}

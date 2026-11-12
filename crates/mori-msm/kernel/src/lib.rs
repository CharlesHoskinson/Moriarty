// SPDX-License-Identifier: Apache-2.0
//! Device code of the Rust MSM. Built only for nvptx64-nvidia-cuda.
#![no_std]
#![feature(abi_ptx, stdarch_nvptx)]
#![allow(clippy::missing_safety_doc)]

use core::arch::nvptx;

mod fp;
use fp::Fp;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe { nvptx::trap() }
}

#[inline(always)]
fn gid() -> usize {
    unsafe {
        (nvptx::_block_idx_x() as usize) * (nvptx::_block_dim_x() as usize)
            + nvptx::_thread_idx_x() as usize
    }
}

/// Spike kernel: out[i] = a[i] * b[i] (Montgomery, R = 2^384), iterated `reps` times.
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

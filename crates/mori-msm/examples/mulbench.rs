// SPDX-License-Identifier: Apache-2.0
//! Fp multiplication throughput of the device code (`fp_mul_test`, chained products).
use cudarc::driver::{LaunchConfig, PushKernelArg};
fn main() {
    let d = mori_msm::dev::dev().expect("device");
    let s = &d.stream;
    use cudarc::driver::sys::CUdevice_attribute as A;
    for (n, a) in [
        ("sms", A::CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT),
        (
            "regs_per_sm",
            A::CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_MULTIPROCESSOR,
        ),
        (
            "threads_per_sm",
            A::CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_MULTIPROCESSOR,
        ),
        ("l2", A::CU_DEVICE_ATTRIBUTE_L2_CACHE_SIZE),
    ] {
        println!("{n}={}", d.ctx.attribute(a).unwrap());
    }
    let n: usize = 1 << 20;
    let reps: u32 = 1000;
    let a: Vec<u64> = (0..6 * n)
        .map(|i| (i as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15) >> 4)
        .collect();
    let da = s.clone_htod(&a).unwrap();
    let mut out = s.alloc_zeros::<u64>(6 * n).unwrap();
    {
        let f = d.func("imad_test");
        let mut o = s.alloc_zeros::<u32>(n).unwrap();
        for _ in 0..2 {
            let t = std::time::Instant::now();
            let reps = 10000u32;
            let mut b = s.launch_builder(&f);
            b.arg(&mut o).arg(&reps);
            unsafe {
                b.launch(LaunchConfig {
                    grid_dim: (n as u32 / 256, 1, 1),
                    block_dim: (256, 1, 1),
                    shared_mem_bytes: 0,
                })
            }
            .unwrap();
            s.synchronize().unwrap();
            let e = t.elapsed().as_secs_f64();
            println!(
                "imad: {:.1} G IMAD/s",
                (n as f64) * 8.0 * reps as f64 / e / 1e9
            );
        }
    }
    let f = d.func("fp_mul_test");
    for threads in [64u32, 128, 256] {
        for _ in 0..2 {
            let t = std::time::Instant::now();
            let nn = n as u32;
            let mut b = s.launch_builder(&f);
            b.arg(&da).arg(&da).arg(&mut out).arg(&nn).arg(&reps);
            let cfg = LaunchConfig {
                grid_dim: (nn / threads, 1, 1),
                block_dim: (threads, 1, 1),
                shared_mem_bytes: 0,
            };
            unsafe { b.launch(cfg) }.unwrap();
            s.synchronize().unwrap();
            let e = t.elapsed().as_secs_f64();
            println!(
                "threads={threads} {:.2} G Fp mul/s ({e:.3} s)",
                (n as f64) * reps as f64 / e / 1e9
            );
        }
    }
}

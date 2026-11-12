// SPDX-License-Identifier: Apache-2.0
//! Feasibility spike: the Rust PTX Fp Montgomery multiplication equals blst_fp_mul.
#![cfg(feature = "cuda")]
use cudarc::driver::{LaunchConfig, PushKernelArg};
use rand::{RngCore, SeedableRng};

fn rand_fp(r: &mut rand_chacha::ChaCha8Rng) -> [u64; 6] {
    let mut l = [0u64; 6];
    for x in &mut l {
        *x = r.next_u64();
    }
    l[5] &= 0x0fff_ffff_ffff_ffff; // < 2^380 < p
    l
}

#[test]
fn fp_mul_matches_blst() {
    let d = mori_msm::dev::dev().expect("device");
    let n = 1usize << 16;
    let reps = 3u32;
    let mut r = rand_chacha::ChaCha8Rng::seed_from_u64(1);
    let a: Vec<[u64; 6]> = (0..n).map(|_| rand_fp(&mut r)).collect();
    let b: Vec<[u64; 6]> = (0..n).map(|_| rand_fp(&mut r)).collect();
    let fa: Vec<u64> = a.iter().flatten().copied().collect();
    let fb: Vec<u64> = b.iter().flatten().copied().collect();
    let s = &d.stream;
    let da = s.clone_htod(&fa).unwrap();
    let db = s.clone_htod(&fb).unwrap();
    let mut dout = s.alloc_zeros::<u64>(6 * n).unwrap();
    let f = d.func("fp_mul_test");
    let nn = n as u32;
    let mut lb = s.launch_builder(&f);
    lb.arg(&da).arg(&db).arg(&mut dout).arg(&nn).arg(&reps);
    unsafe { lb.launch(LaunchConfig::for_num_elems(nn)) }.unwrap();
    let out = s.clone_dtoh(&dout).unwrap();
    for i in 0..n {
        let mut x = blst::blst_fp { l: a[i] };
        let y = blst::blst_fp { l: b[i] };
        for _ in 0..reps {
            let t = x;
            unsafe { blst::blst_fp_mul(&mut x, &t, &y) };
        }
        assert_eq!(&out[6 * i..6 * i + 6], &x.l[..], "index {i}");
    }
}

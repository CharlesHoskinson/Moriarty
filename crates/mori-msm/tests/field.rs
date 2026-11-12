// SPDX-License-Identifier: Apache-2.0
//! Device field and curve operations against BLST.
#![cfg(feature = "cuda")]
use cudarc::driver::{LaunchConfig, PushKernelArg};
use ff::Field;
use group::{Curve, Group, prime::PrimeCurveAffine};
use midnight_curves::{G1Affine, G1Projective};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn rand_fp(r: &mut ChaCha8Rng) -> [u64; 6] {
    let mut l = [0u64; 6];
    for x in &mut l {
        *x = r.next_u64();
    }
    l[5] &= 0x0fff_ffff_ffff_ffff; // < 2^380 < p
    l
}

#[test]
fn fp_ops_match_blst() {
    let d = mori_msm::dev::dev().expect("device");
    let n = 1usize << 12;
    let mut r = ChaCha8Rng::seed_from_u64(2);
    let mut a: Vec<[u64; 6]> = (0..n).map(|_| rand_fp(&mut r)).collect();
    let b: Vec<[u64; 6]> = (0..n).map(|_| rand_fp(&mut r)).collect();
    a[0] = [0; 6];
    a[1] = b[1];
    let fa: Vec<u64> = a.iter().flatten().copied().collect();
    let fb: Vec<u64> = b.iter().flatten().copied().collect();
    let s = &d.stream;
    let da = s.clone_htod(&fa).unwrap();
    let db = s.clone_htod(&fb).unwrap();
    let mut dout = s.alloc_zeros::<u64>(18 * n).unwrap();
    let f = d.func("fp_ops_test");
    let nn = n as u32;
    let mut lb = s.launch_builder(&f);
    lb.arg(&da).arg(&db).arg(&mut dout).arg(&nn);
    let cfg = LaunchConfig {
        grid_dim: (nn / 128, 1, 1),
        block_dim: (128, 1, 1),
        shared_mem_bytes: 0,
    };
    unsafe { lb.launch(cfg) }.unwrap();
    let out = s.clone_dtoh(&dout).unwrap();
    for i in 0..n {
        let x = blst::blst_fp { l: a[i] };
        let y = blst::blst_fp { l: b[i] };
        let mut e = [blst::blst_fp::default(); 3];
        unsafe {
            blst::blst_fp_add(&mut e[0], &x, &y);
            blst::blst_fp_sub(&mut e[1], &x, &y);
            blst::blst_fp_inverse(&mut e[2], &x);
        }
        for j in 0..3 {
            assert_eq!(
                &out[18 * i + 6 * j..18 * i + 6 * j + 6],
                &e[j].l[..],
                "index {i} op {j}"
            );
        }
    }
}

fn xyzz_to_affine(h: &[u64]) -> G1Affine {
    use blst::{blst_fp, blst_fp_inverse, blst_fp_mul, blst_p1_affine};
    let f = |i: usize| blst_fp {
        l: h[i * 6..i * 6 + 6].try_into().unwrap(),
    };
    if f(2).l == [0; 6] {
        return G1Affine::identity();
    }
    let mut a = blst_p1_affine::default();
    let mut t = blst_fp::default();
    unsafe {
        blst_fp_inverse(&mut t, &f(2));
        blst_fp_mul(&mut a.x, &f(0), &t);
        blst_fp_inverse(&mut t, &f(3));
        blst_fp_mul(&mut a.y, &f(1), &t);
        std::mem::transmute::<blst_p1_affine, G1Affine>(a)
    }
}

#[test]
fn ec_ops_match_blst() {
    let d = mori_msm::dev::dev().expect("device");
    let mut r = ChaCha8Rng::seed_from_u64(3);
    let mut p: Vec<G1Projective> = (0..64).map(|_| G1Projective::random(&mut r)).collect();
    let mut q: Vec<G1Projective> = (0..64).map(|_| G1Projective::random(&mut r)).collect();
    // Special cases: q = p, q = -p, p = infinity, q = infinity.
    q[0] = p[0];
    q[1] = -p[1];
    p[2] = G1Projective::identity();
    q[3] = G1Projective::identity();
    let n = p.len();
    let pa: Vec<G1Affine> = p.iter().map(|x| x.to_affine()).collect();
    let qa: Vec<G1Affine> = q.iter().map(|x| x.to_affine()).collect();
    let words = |v: &[G1Affine]| {
        unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u64, v.len() * 12) }.to_vec()
    };
    let s = &d.stream;
    let dp = s.clone_htod(&words(&pa)).unwrap();
    let dq = s.clone_htod(&words(&qa)).unwrap();
    let mut dout = s.alloc_zeros::<u64>(4 * 24 * n).unwrap();
    let f = d.func("ec_test");
    let nn = n as u32;
    let mut lb = s.launch_builder(&f);
    lb.arg(&dp).arg(&dq).arg(&mut dout).arg(&nn);
    unsafe {
        lb.launch(LaunchConfig {
            grid_dim: (1, 1, 1),
            block_dim: (64, 1, 1),
            shared_mem_bytes: 0,
        })
    }
    .unwrap();
    let out = s.clone_dtoh(&dout).unwrap();
    let five = midnight_curves::Fq::from(5u64);
    for i in 0..n {
        let want = [
            (p[i] + q[i]).to_affine(),
            (p[i] + q[i]).to_affine(),
            p[i].double().to_affine(),
            (p[i] * five).to_affine(),
        ];
        for j in 0..4 {
            let got = xyzz_to_affine(&out[(4 * i + j) * 24..(4 * i + j + 1) * 24]);
            assert_eq!(got, want[j], "point {i} op {j}");
        }
    }
    let _ = midnight_curves::Fq::ONE;
}

//! Differential tests of the GPU kernels against BLST `multi_exp` and `best_fft`
//! (gpu.md section 5 edge classes). Run on this host with `--features cuda` and
//! MORI_GPU_MIN_K=0; on a host without a GPU every test fails loudly instead of
//! passing vacuously.
use ff::{Field, PrimeField};
use group::{Curve, Group};
use midnight_curves::{Fq, G1Projective, fft::best_fft};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

fn gpu() {
    assert!(
        mori_gpu::info().is_some(),
        "no GPU: the differential tests need one"
    );
    assert_eq!(mori_gpu::min_k(), 0, "run with MORI_GPU_MIN_K=0");
}

fn rnd_scalars(n: usize, seed: u64) -> Vec<Fq> {
    let mut r = ChaCha20Rng::seed_from_u64(seed);
    (0..n).map(|_| Fq::random(&mut r)).collect()
}

fn rnd_points(n: usize, seed: u64) -> Vec<G1Projective> {
    // Fast: multiples of one random point, plus projective Z != 1 from the additions.
    let mut r = ChaCha20Rng::seed_from_u64(seed);
    let g = G1Projective::random(&mut r);
    let step = G1Projective::random(&mut r);
    let mut v = Vec::with_capacity(n);
    let mut acc = g;
    for _ in 0..n {
        v.push(acc);
        acc += step;
    }
    v
}

fn check_msm(s: &[Fq], b: &[G1Projective], what: &str) {
    let want = G1Projective::multi_exp(&b[..s.len()], s);
    let got = mori_gpu::msm_g1(s, b).unwrap_or_else(|| panic!("{what}: GPU path not taken"));
    assert_eq!(got.to_affine(), want.to_affine(), "{what}: group element");
    assert_eq!(
        group::GroupEncoding::to_bytes(&got.to_affine())
            .as_ref()
            .to_vec(),
        group::GroupEncoding::to_bytes(&want.to_affine())
            .as_ref()
            .to_vec(),
        "{what}: compressed bytes"
    );
}

#[test]
fn msm_sizes() {
    gpu();
    let base = rnd_points((1 << 16) + 1, 1);
    for n in [
        1usize,
        2,
        3,
        1023,
        1024,
        1025,
        4095,
        4096,
        4097,
        1 << 14,
        (1 << 14) + 1,
        (1 << 16) + 1,
    ] {
        let s = rnd_scalars(n, n as u64);
        // fresh slice each size: exercise the table cache keyed by (ptr, len)
        check_msm(&s, &base[..n.max(1)], &format!("n={n}"));
    }
}

#[test]
fn msm_edge_classes() {
    gpu();
    let n = 4096;
    let mut b = rnd_points(n, 7);
    let r_minus_1 = -Fq::ONE;
    let mut s = rnd_scalars(n, 8);
    // zero scalars, scalar 1, r - 1, 2^(c-1) and multiples of 2^c at several windows
    for i in 0..64 {
        s[i] = Fq::ZERO;
    }
    s[64] = Fq::ONE;
    s[65] = r_minus_1;
    for (j, w) in (0..250).step_by(9).enumerate() {
        s[100 + j] = Fq::from(2u64).pow_vartime([w as u64]);
        s[200 + j] = Fq::from(2u64).pow_vartime([w as u64 + 1]) - Fq::ONE;
    }
    // identity bases at first, last and inner positions; duplicate bases
    b[0] = G1Projective::identity();
    b[n - 1] = G1Projective::identity();
    b[1000] = G1Projective::identity();
    for i in 2000..2100 {
        b[i] = b[1999];
    }
    check_msm(&s, &b, "edge classes");
    // all-zero vector: identity result
    let z = vec![Fq::ZERO; n];
    check_msm(&z, &b, "all zero");
    // one non-zero
    let mut one = vec![Fq::ZERO; n];
    one[123] = Fq::from(5u64);
    check_msm(&one, &b, "one non-zero");
    // one base repeated
    let rep = vec![b[5]; n];
    check_msm(&s, &rep, "repeated base");
    // negated pairs P and -P
    let mut np = b.clone();
    for i in (3000..3100).step_by(2) {
        np[i + 1] = -np[i];
        s[i + 1] = s[i];
    }
    check_msm(&s, &np, "P and -P with equal scalars");
}

#[test]
fn msm_heavy_repeated_scalars() {
    gpu();
    // The bucket skew class: one non-zero value repeated (constant or fill columns).
    let n = 1 << 16;
    let b = rnd_points(n, 21);
    let mut s = rnd_scalars(n, 22);
    let v = s[7];
    for i in 0..n - 1000 {
        s[i] = v;
    }
    let t = std::time::Instant::now();
    check_msm(&s, &b, "one value repeated 64,536 times");
    assert!(
        t.elapsed().as_secs_f64() < 5.0,
        "skew guard: {:?}",
        t.elapsed()
    );
    // two heavy values and ones
    let mut s2 = rnd_scalars(n, 23);
    for i in 0..n {
        match i % 4 {
            0 | 1 => s2[i] = Fq::ONE,
            2 => s2[i] = v,
            _ => {}
        }
    }
    check_msm(&s2, &b, "two heavy values");
}

#[test]
fn msm_large_sizes() {
    gpu();
    let base = rnd_points((1 << 21) + 1, 31);
    for n in [1usize << 19, (1 << 21) - 1, 1 << 21, (1 << 21) + 1] {
        let s = rnd_scalars(n, n as u64);
        check_msm(&s, &base[..n], &format!("n={n}"));
    }
}

#[test]
fn msm_2p20() {
    gpu();
    let n = 1 << 20;
    let b = rnd_points(n, 11);
    let s = rnd_scalars(n, 12);
    check_msm(&s, &b, "2^20");
}

fn structured(log_n: u32) -> Vec<Vec<Fq>> {
    let n = 1usize << log_n;
    let mut delta = vec![Fq::ZERO; n];
    delta[1 % n] = Fq::ONE;
    let alt: Vec<Fq> = (0..n)
        .map(|i| if i % 2 == 0 { Fq::ONE } else { -Fq::ONE })
        .collect();
    vec![
        rnd_scalars(n, log_n as u64),
        vec![Fq::ZERO; n],
        delta,
        vec![Fq::from(3u64); n],
        alt,
    ]
}

#[test]
fn ntt_forward_inverse() {
    gpu();
    for log_n in 1..=23u32 {
        let (w, wi) = mori_gpu::std_root(log_n);
        let div = Fq::from(1u64 << log_n).invert().unwrap();
        for (vi, v) in structured(log_n).into_iter().enumerate() {
            let mut cpu = v.clone();
            best_fft(&mut cpu, w, log_n);
            let mut g = v.clone();
            assert!(mori_gpu::fft(&mut g, w, log_n), "fft taken");
            assert!(g == cpu, "forward log_n={log_n} vec={vi}");
            // inverse with the iFFT divisor
            let mut cpu_i = cpu.clone();
            best_fft(&mut cpu_i, wi, log_n);
            cpu_i.iter_mut().for_each(|x| *x *= div);
            assert!(cpu_i == v, "cpu round trip");
            let mut gi = cpu.clone();
            assert!(mori_gpu::ifft(&mut gi, wi, log_n, div), "ifft taken");
            assert!(gi == cpu_i, "inverse log_n={log_n} vec={vi}");
            // unscaled inverse (best_fft with omega^-1)
            let mut cu = cpu.clone();
            best_fft(&mut cu, wi, log_n);
            let mut gu = cpu.clone();
            assert!(mori_gpu::fft(&mut gu, wi, log_n), "unscaled inverse taken");
            assert!(gu == cu, "unscaled inverse log_n={log_n} vec={vi}");
        }
    }
}

#[test]
fn ntt_rejects_non_standard_root() {
    gpu();
    let mut v = rnd_scalars(1 << 10, 3);
    let (w, _) = mori_gpu::std_root(10);
    assert!(!mori_gpu::fft(&mut v, w.square(), 10));
}

#[test]
fn coset_extended() {
    gpu();
    use ff::WithSmallOrderMulGroup;
    for (k, ext_k) in [(10u32, 12u32), (14, 17), (16, 18), (20, 22)] {
        let a = rnd_scalars(1 << k, k as u64 + 100);
        let zeta = [Fq::ZETA, Fq::ZETA.square()];
        let mut cpu = a.clone();
        for (i, x) in cpu.iter_mut().enumerate() {
            match i % 3 {
                1 => *x *= zeta[0],
                2 => *x *= zeta[1],
                _ => {}
            }
        }
        cpu.resize(1 << ext_k, Fq::ZERO);
        best_fft(&mut cpu, mori_gpu::std_root(ext_k).0, ext_k);
        let mut out = vec![Fq::ZERO; 1 << ext_k];
        assert!(mori_gpu::coeff_to_extended(&a, &mut out, k, ext_k, zeta));
        assert!(out == cpu, "coset k={k} ext_k={ext_k}");
        let d = mori_gpu::quotient::DevPoly::coeff_to_extended(&a, k, ext_k, zeta).unwrap();
        assert!(d.to_host().unwrap() == cpu, "device-resident coset k={k}");
    }
}

#[test]
fn field_repr_is_montgomery_blst() {
    // The kernels read Fq bytes as 8 x u32 Montgomery limbs with R = 2^256.
    let one = Fq::ONE;
    let raw: [u64; 4] = unsafe { std::mem::transmute(one) };
    assert_eq!(
        raw,
        [
            0x00000001fffffffe,
            0x5884b7fa00034802,
            0x998c4fefecbc4ff5,
            0x1824b159acc5056f
        ]
    );
    assert_eq!(Fq::S, 32);
    let _ = Fq::from_repr(one.to_repr()).unwrap();
}

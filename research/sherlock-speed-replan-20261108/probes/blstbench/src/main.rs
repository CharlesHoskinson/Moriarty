use ff::{Field, PrimeField};
use group::Group;
use midnight_curves::{Fq, G1Projective};
use rand_core::SeedableRng;
use std::time::Instant;
fn main() {
    let k: u32 = std::env::args().nth(1).map(|s| s.parse().unwrap()).unwrap_or(20);
    let n = 1usize << k;
    let mut rng = rand_xorshift::XorShiftRng::seed_from_u64(42);
    let g = G1Projective::generator();
    let bases: Vec<G1Projective> = (0..n).map(|i| g * Fq::from((i as u64) * 7 + 1)).collect();
    let scalars: Vec<Fq> = (0..n).map(|_| Fq::random(&mut rng)).collect();
    let raw = unsafe { std::slice::from_raw_parts(bases.as_ptr() as *const blst::blst_p1, n) };
    let t = Instant::now();
    let aff = blst::p1_affines::from(raw);
    println!("k={} p1_affines::from (proj->affine batch) {:?}", k, t.elapsed());
    let mut sb: Vec<u8> = Vec::with_capacity(n * 32);
    for s in &scalars { sb.extend_from_slice(&s.to_bytes_le()); }
    for rep in 0..3 {
        let t = Instant::now();
        let r = aff.mult(&sb, 255);
        println!("k={} rep={} blst mult(255-bit) on cached affine {:?} inf={}", k, rep, t.elapsed(), unsafe { blst::blst_p1_is_inf(&r) });
    }
    let t = Instant::now();
    let r2 = aff.mult(&sb, 128);
    println!("k={} blst mult(128-bit scalars) {:?} inf={}", k, t.elapsed(), unsafe { blst::blst_p1_is_inf(&r2) });
    // FFT: midnight-curves best_fft over Fq
    for kk in [20u32, 23] {
        let m = 1usize << kk;
        let mut a: Vec<Fq> = (0..m).map(|i| Fq::from(i as u64)).collect();
        let mut omega = Fq::ROOT_OF_UNITY;
        for _ in kk..Fq::S { omega = omega.square(); }
        let t = Instant::now();
        midnight_curves::fft::best_fft(&mut a, omega, kk);
        println!("fft k={} {:?}", kk, t.elapsed());
    }
}

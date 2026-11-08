extern "C" { fn mori_smoke_mul256(a: *const u32, b: *const u32, out: *mut u32, n: i32, maj: *mut i32, min: *mut i32) -> i32; }
fn main() {
    let n = 1usize << 16;
    let a: Vec<u32> = (0..n * 8).map(|i| (i as u32).wrapping_mul(2654435761) ^ 0xdead_beef).collect();
    let b: Vec<u32> = (0..n * 8).map(|i| (i as u32).wrapping_mul(40503) ^ 0x1234_5678).collect();
    let mut out = vec![0u32; n * 8];
    let (mut maj, mut min) = (0, 0);
    let t = std::time::Instant::now();
    let rc = unsafe { mori_smoke_mul256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), n as i32, &mut maj, &mut min) };
    let dt = t.elapsed();
    // CPU reference with u128 schoolbook, low 256 bits
    let mut bad = 0;
    for i in 0..n {
        let x = &a[8 * i..8 * i + 8]; let y = &b[8 * i..8 * i + 8]; let mut r = [0u32; 8];
        for j in 0..8 { let mut carry = 0u64; for k in 0..(8 - j) { let t = x[j] as u64 * y[k] as u64 + r[j + k] as u64 + carry; r[j + k] = t as u32; carry = t >> 32; } }
        if r != out[8 * i..8 * i + 8] { bad += 1; }
    }
    println!("rust->nvcc smoke: sm_{maj}{min} cuda_rc={rc} n={n} mismatches={bad} (zero-copy on plain Vec memory) {dt:?}");
}

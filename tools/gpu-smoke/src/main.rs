// SPDX-License-Identifier: Apache-2.0
//! GPU phase G0 toolchain smoke (task 2.10a, predicate F23, gpu.md §0 and §6 G0).
//!
//! 1. Records the nvcc release used by build.rs, the CUDA driver and runtime versions and
//!    device 0 (name, compute capability, memory-model flags).
//! 2. Runs an 8 x 32-bit-limb schoolbook product kernel directly on plain `Vec<u32>`
//!    memory (no copies) and compares every result with a CPU reference built on
//!    4 x 64-bit limbs and u128 products. Requires 0 mismatches.
//! 3. Allocates 1 GiB with `cudaMalloc` and 1 GiB with `cudaMallocManaged`, touches every
//!    page on the GPU, and requires the process RSS to rise by >= 0.9 GiB for each, so a
//!    cgroup `MemoryMax` is known to see device memory (Q10).
//!
//! Prints one JSON receipt line on stdout (and writes it to the path in argv[1] if given).
//! Exit 0 = pass, 1 = fail, 2 = no CUDA device visible (skip, never a pass).
use std::ffi::{CStr, c_char, c_int, c_void};
use std::process::ExitCode;
use std::time::Instant;

#[repr(C)]
struct GsDeviceInfo {
    name: [c_char; 256],
    device_count: c_int,
    major: c_int,
    minor: c_int,
    integrated: c_int,
    pageable_memory_access: c_int,
    uses_host_page_tables: c_int,
    concurrent_managed_access: c_int,
    driver_version: c_int,
    runtime_version: c_int,
}

unsafe extern "C" {
    fn gs_device_info(info: *mut GsDeviceInfo) -> c_int;
    fn gs_mul256(a: *const u32, b: *const u32, out: *mut u32, n: c_int) -> c_int;
    fn gs_alloc_touch(managed: c_int, bytes: usize, ptr: *mut *mut c_void) -> c_int;
    fn gs_free(ptr: *mut c_void) -> c_int;
    fn gs_error_string(e: c_int) -> *const c_char;
}

const GIB: u64 = 1 << 30;
const N: usize = 1 << 16;

fn cuda_err(e: c_int) -> String {
    // SAFETY: cudaGetErrorString returns a static NUL-terminated string for any value.
    unsafe { CStr::from_ptr(gs_error_string(e)) }
        .to_string_lossy()
        .into_owned()
}

fn rss_bytes() -> u64 {
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    status
        .lines()
        .find_map(|l| l.strip_prefix("VmRSS:"))
        .and_then(|v| v.trim().trim_end_matches("kB").trim().parse::<u64>().ok())
        .map_or(0, |kb| kb * 1024)
}

fn cuda_version(v: c_int) -> String {
    format!("{}.{}", v / 1000, (v % 1000) / 10)
}

/// Low 256 bits of x * y with 4 x 64-bit limbs and u128 partial products: an independent
/// decomposition from the kernel's 32-bit limbs.
fn mul256_lo_ref(x: &[u32], y: &[u32]) -> [u32; 8] {
    let to64 = |v: &[u32]| -> [u64; 4] {
        std::array::from_fn(|i| u64::from(v[2 * i]) | (u64::from(v[2 * i + 1]) << 32))
    };
    let (x, y) = (to64(x), to64(y));
    let mut r = [0u64; 4];
    for j in 0..4 {
        let mut carry: u128 = 0;
        for k in 0..(4 - j) {
            let t = u128::from(x[j]) * u128::from(y[k]) + u128::from(r[j + k]) + carry;
            r[j + k] = t as u64;
            carry = t >> 64;
        }
    }
    std::array::from_fn(|i| (r[i / 2] >> (32 * (i % 2))) as u32)
}

fn inputs() -> (Vec<u32>, Vec<u32>) {
    let mut a: Vec<u32> = (0..N * 8)
        .map(|i| (i as u32).wrapping_mul(2_654_435_761) ^ 0xdead_beef)
        .collect();
    let mut b: Vec<u32> = (0..N * 8)
        .map(|i| (i as u32).wrapping_mul(40_503) ^ 0x1234_5678)
        .collect();
    // Edge cases in the first slots: all-ones, zero, one, top-limb-only.
    a[0..8].fill(u32::MAX);
    b[0..8].fill(u32::MAX);
    a[8..16].fill(0);
    b[16..24].copy_from_slice(&[1, 0, 0, 0, 0, 0, 0, 0]);
    a[24..32].copy_from_slice(&[0, 0, 0, 0, 0, 0, 0, u32::MAX]);
    b[24..32].fill(u32::MAX);
    (a, b)
}

fn json_str(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

fn main() -> ExitCode {
    let out_path = std::env::args().nth(1);
    let mut fields: Vec<(String, String)> = vec![
        ("schema".into(), json_str("sherlock-f23-gpu-smoke/1")),
        ("nvcc".into(), json_str(env!("GPU_SMOKE_NVCC"))),
        ("arch".into(), json_str(env!("GPU_SMOKE_ARCH"))),
    ];
    let mut failures: Vec<String> = Vec::new();

    let mut info: GsDeviceInfo = unsafe { std::mem::zeroed() };
    // SAFETY: info is a valid, writable GsDeviceInfo with the C layout.
    let rc = unsafe { gs_device_info(&mut info) };
    if rc != 0 || info.device_count == 0 {
        let why = if rc != 0 {
            cuda_err(rc)
        } else {
            "no device".into()
        };
        eprintln!("F23 smoke: no CUDA device visible ({why}); skip");
        return ExitCode::from(2);
    }
    // SAFETY: gs_device_info NUL-terminates name.
    let name = unsafe { CStr::from_ptr(info.name.as_ptr()) }.to_string_lossy();
    let cc = format!("{}.{}", info.major, info.minor);
    for (k, v) in [
        ("device", json_str(&name)),
        ("computeCapability", json_str(&cc)),
        ("cudaDriver", json_str(&cuda_version(info.driver_version))),
        ("cudaRuntime", json_str(&cuda_version(info.runtime_version))),
        ("integrated", info.integrated.to_string()),
        (
            "pageableMemoryAccess",
            info.pageable_memory_access.to_string(),
        ),
        (
            "pageableMemoryAccessUsesHostPageTables",
            info.uses_host_page_tables.to_string(),
        ),
        (
            "concurrentManagedAccess",
            info.concurrent_managed_access.to_string(),
        ),
    ] {
        fields.push((k.into(), v));
    }
    if format!("sm_{}{}", info.major, info.minor) != env!("GPU_SMOKE_ARCH") {
        failures.push(format!(
            "device cc {cc} does not match build arch {}",
            env!("GPU_SMOKE_ARCH")
        ));
    }

    // Products on plain Vec<u32> memory.
    let (a, b) = inputs();
    let mut out = vec![0u32; N * 8];
    let t = Instant::now();
    // SAFETY: a, b, out each hold N * 8 u32 and outlive the synchronous call.
    let rc = unsafe { gs_mul256(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), N as c_int) };
    let ms = t.elapsed().as_secs_f64() * 1e3;
    let mismatches = if rc == 0 {
        (0..N)
            .filter(|&i| {
                let s = 8 * i..8 * i + 8;
                mul256_lo_ref(&a[s.clone()], &b[s.clone()]) != out[s]
            })
            .count()
    } else {
        failures.push(format!("mul256 kernel: {}", cuda_err(rc)));
        N
    };
    if mismatches != 0 {
        failures.push(format!("{mismatches} of {N} products mismatch"));
    }
    fields.push(("products".into(), N.to_string()));
    fields.push(("mismatches".into(), mismatches.to_string()));
    fields.push(("memory".into(), json_str("plain Vec<u32>, zero-copy")));
    fields.push(("kernelMs".into(), format!("{ms:.1}")));

    // RSS accounting of 1 GiB device and managed allocations.
    let min_delta = GIB * 9 / 10;
    for (managed, key) in [(0, "cudaMallocRssDelta"), (1, "cudaMallocManagedRssDelta")] {
        let before = rss_bytes();
        let mut p: *mut c_void = std::ptr::null_mut();
        // SAFETY: p is a valid out-pointer; the allocation is freed below.
        let rc = unsafe { gs_alloc_touch(managed, GIB as usize, &mut p) };
        let delta = rss_bytes().saturating_sub(before);
        if !p.is_null() {
            // SAFETY: p came from cudaMalloc/cudaMallocManaged and is freed once.
            unsafe { gs_free(p) };
        }
        if rc != 0 {
            failures.push(format!("{key}: allocation failed: {}", cuda_err(rc)));
        } else if delta < min_delta {
            failures.push(format!("{key}: RSS rose by {delta} B < {min_delta} B"));
        }
        fields.push((key.into(), delta.to_string()));
    }
    fields.push(("rssDeltaMin".into(), min_delta.to_string()));

    let pass = failures.is_empty();
    fields.push(("pass".into(), pass.to_string()));
    let failures_json: Vec<String> = failures.iter().map(|f| json_str(f)).collect();
    fields.push(("failures".into(), format!("[{}]", failures_json.join(","))));
    let line = format!(
        "{{{}}}",
        fields
            .iter()
            .map(|(k, v)| format!("{}:{v}", json_str(k)))
            .collect::<Vec<_>>()
            .join(",")
    );
    println!("{line}");
    if let Some(path) = out_path
        && let Err(e) = std::fs::write(&path, format!("{line}\n"))
    {
        eprintln!("F23 smoke: cannot write {path}: {e}");
        return ExitCode::from(1);
    }
    if pass {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

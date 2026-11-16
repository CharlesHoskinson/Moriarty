//! Formal-Quint counterexamples for Q-scheduling-2, -4, -5 on the real scheduler
//! (`sherlock_leaf::profile::{schedule, check_free, pipeline}`).
use sherlock_leaf::profile::*;
use sherlock_leaf::sched::{Bucket, Device, LeafCost, Schedule, GIB};

const MB: u64 = 1 << 20;

fn cal(shape: usize, device: Device, leaf_s: f64, host_bytes: u64, device_bytes: u64) -> Calib {
    Calib { shape, device, reps: 3, leaf_s, app_s: leaf_s / 2.0, host_bytes, device_bytes }
}
fn prof(gpus: Vec<GpuInfo>, mem: u64, calib: Vec<Calib>) -> Profile {
    Profile { host: "t".into(), created: "now".into(), key_root: "kr".into(), keys_d: "kd".into(), gpus, cpu_model: "cpu".into(),
        cores: 20, ram_total: mem, mem_budget: mem, disk_path: "/".into(), disk_free: 1 << 40, calib, setup_s: 1.0 }
}
fn discrete(b: u64) -> GpuInfo {
    GpuInfo { index: 0, name: "RTX".into(), compute_cap: "12.0".into(), vram_total: Some(b), vram_free: Some(b), unified: false, sm_clock_max_mhz: Some(3000) }
}
fn unified() -> GpuInfo {
    GpuInfo { index: 0, name: "GB10".into(), compute_cap: "12.1".into(), vram_total: None, vram_free: None, unified: true, sm_clock_max_mhz: None }
}
fn buckets() -> Vec<Bucket> {
    let cost = LeafCost::from_area(1 << 20, 1);
    vec![Bucket { shape: 0, cap: 4, cost }, Bucket { shape: 1, cap: 8, cost }]
}

/// Q-scheduling-4: a calibration entry with host_bytes = device_bytes = 0 passes from_json (only
/// leaf_s is checked) and the scheduler admits it under a 1-byte budget (q divides by max(1, d)).
#[test]
fn qsched4_zero_bytes_admitted_under_one_byte_budget() {
    let mut v = prof(vec![], 1, vec![cal(0, Device::Cpu, 1.0, 0, 0)]).to_json();
    v["mem_budget"] = serde_json::json!(1);
    let p = Profile::from_json(&v).expect("zero host/device bytes accepted by from_json");
    let r = schedule(&p, &buckets(), 5, &Limits::default(), 2);
    println!("QS4 zero bytes, 1-byte budget: {:?}", r.as_ref().map(|s| (s.shape, s.concurrency, s.cut.clone())));
    assert!(r.is_ok(), "the 1-byte budget admits a 0-byte leaf: STM-012 does not refuse");
    // a GPU entry with 0 device bytes on a 1-byte discrete GPU is admitted too
    let p = prof(vec![discrete(1)], 1, vec![cal(0, Device::Gpu, 1.0, 0, 0)]);
    let r = schedule(&p, &buckets(), 5, &Limits::default(), 2);
    println!("QS4 zero bytes on a 1-byte GPU: {:?}", r.as_ref().map(|s| (s.device, s.concurrency)));
    assert!(r.is_ok());
}

/// Q-scheduling-5: an overshooting cut [3, 3] for n = 5 and a shape with no bucket are refused.
#[test]
fn qsched5_overshoot_cut_and_unknown_shape_refused() {
    let s = Schedule { shape: 0, cut: vec![3, 3], device: Device::Cpu, concurrency: 1, per_gpu: vec![], node_arity: 2, why: String::new() };
    let r = check_free(&buckets(), 5, &s);
    println!("QS5 cut [3,3] n=5: {:?}", r);
    assert!(r.is_err());
    let s2 = Schedule { shape: 2, cut: vec![1, 1, 1, 1, 1], ..s.clone() };
    let r2 = check_free(&buckets(), 5, &s2);
    println!("QS5 shape 2 (no bucket): {:?}", r2);
    assert!(r2.is_err());
}

/// Q-scheduling-2: the policy uses the lowered memory (SHERLOCK_GPU_MEM / SHERLOCK_MEM_BUDGET) and
/// the host budget on a unified GPU.
#[test]
fn qsched2_policy_from_lowered_and_unified_memory() {
    let l = Limits::default();
    let odin = prof(vec![discrete(32 * GIB)], 64 * GIB, vec![]);
    let full = pipeline(&odin, &l, &PEAKS);
    let capped = pipeline(&odin, &Limits { gpu_mem: Some(16 * GIB), ..l.clone() }, &PEAKS);
    println!("QS2 odin 32 GiB: {:?} mem {}; capped 16 GiB: {:?} mem {}", full.overlap, full.mem, capped.overlap, capped.mem);
    assert_eq!(capped.mem, 16 * GIB);
    assert_ne!(full.overlap, capped.overlap);
    let mun = prof(vec![unified()], 100 * GIB, vec![]);
    let m1 = pipeline(&mun, &l, &PEAKS);
    let m2 = pipeline(&mun, &Limits { mem_budget: Some(16 * GIB), ..l.clone() }, &PEAKS);
    println!("QS2 unified 100 GiB budget: {:?} mem {}; budget limit 16 GiB: {:?} mem {}", m1.overlap, m1.mem, m2.overlap, m2.mem);
    assert_eq!((m1.mem, m2.mem), (100 * GIB, 16 * GIB));
    let _ = MB;
}

/// D-sched-1: the run concurrency never exceeds the leaf count (n = 5, S = 4: 2 leaves; S = 8: 1 leaf)
/// although the budget holds 8 leaves at once.
#[test]
fn dsched1_concurrency_capped_at_leaf_count() {
    let p = prof(vec![], 64 * GIB, vec![cal(0, Device::Cpu, 1.0, GIB, 0), cal(1, Device::Cpu, 1.5, GIB, 0)]);
    let bs = buckets();
    let s = schedule(&p, &bs, 5, &Limits::default(), 2).unwrap();
    println!("DS1 n=5: shape {} cut {:?} concurrency {} (capacity min(64, 20/4) = 5)", s.shape, s.cut, s.concurrency);
    assert!(s.concurrency as usize <= s.cut.len());
    let s1 = schedule(&p, &bs[1..], 5, &Limits::default(), 2).unwrap();
    println!("DS1 n=5 S=8: cut {:?} concurrency {}", s1.cut, s1.concurrency);
    assert_eq!(s1.concurrency, 1);
}

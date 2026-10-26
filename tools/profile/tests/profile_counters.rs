#![cfg(feature = "recursion-phase-profile")]

use mori_recursion_profile::counters::{
    Counters, Observation, RssSample, parse_cpu_ticks, parse_io, parse_status,
};

fn counters(rss: u64, ticks: u64, read: u64, write: u64) -> Counters {
    Counters::parse(
        Ok(format!("VmRSS:\t{rss} kB\nVmHWM:\t9999 kB\n")),
        Ok(format!(
            "1 (a tricky ) name) S 0 0 0 0 0 0 0 0 0 0 {ticks} 2 888 999"
        )),
        Ok(format!(
            "rchar: 999999\nwchar: 88888\nread_bytes: {read}\nwrite_bytes: {write}\n"
        )),
    )
}

#[test]
fn proc_parsing_uses_process_ticks_and_storage_bytes() {
    assert_eq!(
        parse_cpu_ticks("1 (a tricky ) name) S 0 0 0 0 0 0 0 0 0 0 11 7 888 999").unwrap(),
        18
    );
    assert_eq!(
        parse_status("VmRSS: 12 kB\nVmHWM: 90 kB\n", "VmRSS").unwrap(),
        12288
    );
    assert_eq!(
        parse_io("rchar: 999\nread_bytes: 23\n", "read_bytes").unwrap(),
        23
    );
    assert!(parse_cpu_ticks("1 (x) S").is_err());
    assert!(parse_status("VmRSS: 1 MB", "VmRSS").is_err());
    assert!(parse_io("rchar: 99", "read_bytes").is_err());
    assert!(parse_status("VmRSS: 18446744073709551615 kB", "VmRSS").is_err());
}

#[test]
fn rss_samples_are_strictly_interior_and_hwm_is_not_phase_peak() {
    let mut observation = Observation::new(100, 16);
    observation.boundary(1, 100, true, counters(10, 10, 100, 200));
    observation.boundary(1, 200, false, counters(8, 30, 105, 209));
    for (start, end, rss) in [
        (90, 100, 999),
        (100, 101, 999),
        (110, 111, 20),
        (150, 151, 12),
        (199, 200, 999),
        (200, 201, 999),
    ] {
        observation.sample(RssSample::new(start, end, Ok(rss * 1024), Ok(9999 * 1024)));
    }
    let m = observation.metrics(100, 200, true);
    assert_eq!(m["rss"]["samples"], 2);
    assert_eq!(m["rss"]["sampled_peak_bytes"], 20 * 1024);
    assert_eq!(m["rss"]["peak_delta_bytes"], 10 * 1024);
    assert_eq!(m["rss"]["net_delta_bytes"], -2 * 1024);
    assert_eq!(m["rss"]["quality"], "sampled_lower_bound");
    assert_eq!(m["cpu_core_seconds"], 0.2);
    assert_eq!(m["bytes"]["storage_read_bytes"], 5);
    assert_eq!(m["bytes"]["storage_write_bytes"], 9);
    assert!(m["bytes"]["dram_read_bytes"].is_null());
    assert!(!m["bytes"]["null_reasons"].as_array().unwrap().is_empty());
}

#[test]
fn short_phase_is_boundary_only_and_peak_delta_is_nonnegative() {
    let mut o = Observation::new(100, 4);
    o.boundary(1, 1, true, counters(12, 1, 0, 0));
    o.boundary(1, 2, false, counters(8, 1, 0, 0));
    let m = o.metrics(1, 2, true);
    assert_eq!(m["rss"]["quality"], "boundary_only");
    assert_eq!(m["rss"]["peak_delta_bytes"], 0);
    assert_eq!(m["rss"]["certifies_instantaneous"], false);
    assert_eq!(m["bytes"]["storage_read_bytes"], 0); // Measured zero is legitimate.
}

#[test]
fn absent_regressing_and_overlapping_counters_are_null_with_reasons() {
    let mut o = Observation::new(100, 8);
    o.boundary(1, 1, true, counters(12, 20, 20, 20));
    o.boundary(1, 2, false, counters(8, 10, 10, 10));
    let m = o.metrics(1, 2, true);
    assert!(m["cpu_core_seconds"].is_null());
    assert!(m["cpu_null_reason"].as_str().unwrap().contains("regressed"));
    assert!(m["bytes"]["storage_read_bytes"].is_null());
    let m = o.metrics(1, 2, false);
    assert!(m["cpu_core_seconds"].is_null());
    assert!(m["cpu_null_reason"].as_str().unwrap().contains("overlap"));
    let missing = Counters::parse(
        Err("status unavailable".into()),
        Err("stat unavailable".into()),
        Err("io unavailable".into()),
    );
    o.boundary(2, 3, true, missing.clone());
    o.boundary(2, 4, false, missing);
    let m = o.metrics(3, 4, true);
    assert!(m["rss"]["entry_bytes"].is_null());
    assert!(m["rss"]["reason"].is_string());
    assert!(m["cpu_core_seconds"].is_null());
    assert!(m["bytes"]["storage_write_bytes"].is_null());
    assert!(!o.valid());
}

#[test]
fn bounded_sampler_failure_invalidates_without_panicking() {
    let mut o = Observation::new(100, 1);
    o.sample(RssSample::new(
        1,
        2,
        Err("status unavailable".into()),
        Err("status unavailable".into()),
    ));
    o.sample(RssSample::new(3, 4, Ok(1), Ok(1)));
    assert!(!o.valid());
    assert_eq!(o.dropped_samples(), 1);
    assert!(Observation::new(0, 4).metrics(1, 2, true)["cpu_core_seconds"].is_null());
}

#[test]
fn real_sampler_joins_and_boundary_hook_records_nested_spans_and_errors() {
    use midnight_proofs::profile::{Accounting, Phase, Recorder};
    use mori_recursion_profile::counters::{Collector, Sampler};
    use std::{
        thread,
        time::{Duration, Instant},
    };
    let collector = Collector::new(128);
    let recorder = Recorder::new(16);
    assert!(recorder.set_observer(collector.clone()));
    let sampler = Sampler::start(collector.clone(), Duration::from_millis(2)).unwrap();
    let mut command = recorder.span(Phase::Command, Accounting::Diagnostic);
    let mut outer = recorder.span(Phase::Prove, Accounting::Diagnostic);
    let mut nested = recorder.span(Phase::Prove, Accounting::Diagnostic);
    let deadline = Instant::now() + Duration::from_secs(2);
    while collector.report(&[])["sample_count"].as_u64().unwrap() < 3 {
        assert!(
            Instant::now() < deadline,
            "sampler failed to produce samples"
        );
        thread::sleep(Duration::from_millis(2));
    }
    nested.finish();
    drop(nested);
    outer.finish();
    drop(outer);
    drop(recorder.span(Phase::Verify, Accounting::Exclusive)); // returned-error path
    command.finish();
    drop(command);
    let overhead = sampler.stop();
    assert!(overhead["sampler_core_seconds"].is_number());
    let snapshot = recorder.snapshot();
    assert_eq!(snapshot.events.last().unwrap().status, 2);
    let report = collector.report(&snapshot.events);
    assert_eq!(report["sampler_valid"], true);
    for item in report["measurements"].as_array().unwrap() {
        assert!(
            item["metrics"]["cpu_core_seconds"].is_number(),
            "nested proofs are not unrelated parallel jobs"
        );
        let start = item["start_ns"].as_u64().unwrap();
        let end = item["end_ns"].as_u64().unwrap();
        for point in item["metrics"]["rss"]["sample_points"].as_array().unwrap() {
            let at = point["at_monotonic_ns"].as_u64().unwrap();
            assert!(start < at && at < end);
        }
    }
    assert!(
        report["measurements"][0]["metrics"]["rss"]["samples"]
            .as_u64()
            .unwrap()
            >= 2
    );
}

#[test]
fn unrelated_parallel_proofs_never_each_claim_process_cpu() {
    use midnight_proofs::profile::{Accounting, Phase, Recorder};
    use mori_recursion_profile::counters::Collector;
    use std::sync::{Arc, Barrier};
    let collector = Collector::new(32);
    let recorder = Arc::new(Recorder::new(16));
    recorder.set_observer(collector.clone());
    let barrier = Arc::new(Barrier::new(2));
    std::thread::scope(|scope| {
        for _ in 0..2 {
            let recorder = recorder.clone();
            let barrier = barrier.clone();
            scope.spawn(move || {
                let mut proof = recorder.span(Phase::Prove, Accounting::Diagnostic);
                barrier.wait();
                let mut phase = recorder.span(Phase::Quotient, Accounting::Exclusive);
                barrier.wait();
                phase.finish();
                drop(phase);
                barrier.wait();
                proof.finish();
            });
        }
    });
    let report = collector.report(&recorder.snapshot().events);
    for item in report["measurements"].as_array().unwrap() {
        assert!(item["metrics"]["cpu_core_seconds"].is_null());
        assert!(
            item["metrics"]["cpu_null_reason"]
                .as_str()
                .unwrap()
                .contains("overlap")
        );
    }
}

#[test]
fn expected_control_errors_are_retained_and_scoped() {
    use midnight_proofs::profile::{Accounting, Phase, Recorder};
    use mori_recursion_profile::{counters::Collector, expected_rejection};
    let collector = Collector::new(32);
    let recorder = Recorder::new(16);
    recorder.set_observer(collector.clone());
    let mut verify = recorder.span(Phase::Verify, Accounting::Exclusive);
    assert!(expected_rejection(|| {
        drop(recorder.span(Phase::Transcript, Accounting::Exclusive));
        true
    }));
    verify.finish();
    drop(verify);
    let snapshot = recorder.snapshot();
    assert_eq!(snapshot.events[1].status, 2); // raw returned error is never erased
    assert!(!snapshot.profile_valid);
    assert!(collector.accepts(&snapshot));
    drop(recorder.span(Phase::Transcript, Accounting::Exclusive));
    assert!(!collector.accepts(&recorder.snapshot())); // unrelated errors still fail
}

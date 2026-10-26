#![cfg(feature = "recursion-phase-profile")]

use std::sync::OnceLock;

use midnight_proofs::profile::{Accounting, Phase, Recorder};

#[test]
fn bounded_buffer_keeps_integer_intervals() {
    let recorder = Recorder::new(8);
    let mut outer = recorder.span(Phase::Prove, Accounting::Diagnostic);
    let mut inner = recorder.span(Phase::Synthesis, Accounting::Exclusive);
    inner.finish();
    drop(inner);
    outer.finish();
    drop(outer);
    let snapshot = recorder.snapshot();
    assert!(snapshot.profile_valid);
    assert_eq!(snapshot.events.len(), 2);
    assert_eq!(snapshot.events[1].parent_id, snapshot.events[0].id);
    assert_eq!(snapshot.events[0].thread_id, snapshot.events[1].thread_id);
    for event in snapshot.events {
        assert!(event.end_ns >= event.start_ns);
        assert_eq!(event.status, 1);
    }
}

#[test]
fn overflow_drops_without_growing_and_invalidates_even_when_full() {
    let recorder = Recorder::new(2);
    for _ in 0..2 {
        recorder
            .span(Phase::Synthesis, Accounting::Exclusive)
            .finish();
    }
    assert!(!recorder.snapshot().profile_valid);
    for _ in 0..3 {
        recorder
            .span(Phase::Synthesis, Accounting::Exclusive)
            .finish();
    }
    let snapshot = recorder.snapshot();
    assert_eq!(snapshot.events.len(), 2);
    assert_eq!(snapshot.event_capacity, 2);
    assert_eq!(snapshot.dropped_events, 3);
    assert!(!snapshot.profile_valid);
}

#[test]
fn failing_timer_never_changes_the_operation_result() {
    let recorder = Recorder::with_clock(8, || None);
    let answer = {
        let mut span = recorder.span(Phase::Synthesis, Accounting::Exclusive);
        span.finish();
        42
    };
    assert_eq!(answer, 42);
    let snapshot = recorder.snapshot();
    assert!(snapshot.timer_failures > 0);
    assert!(!snapshot.profile_valid);
}

#[test]
fn returned_error_closes_the_span() {
    fn fail(recorder: &Recorder) -> Result<(), &'static str> {
        let _span = recorder.span(Phase::Synthesis, Accounting::Exclusive);
        Err("fixture failure")?;
        Ok(())
    }
    let recorder = Recorder::new(8);
    assert_eq!(fail(&recorder), Err("fixture failure"));
    let snapshot = recorder.snapshot();
    assert_eq!(snapshot.events.len(), 1);
    assert_eq!(snapshot.events[0].status, 2);
    assert!(snapshot.events[0].end_ns >= snapshot.events[0].start_ns);
    assert!(!snapshot.profile_valid);
}

#[test]
fn reentrant_clock_does_not_deadlock_or_recurse() {
    static RECORDER: OnceLock<Recorder> = OnceLock::new();
    fn clock() -> Option<u64> {
        RECORDER
            .get()
            .unwrap()
            .span(Phase::Transcript, Accounting::Diagnostic)
            .finish();
        Some(1)
    }
    let recorder = RECORDER.get_or_init(|| Recorder::with_clock(8, clock));
    recorder
        .span(Phase::Synthesis, Accounting::Exclusive)
        .finish();
    let snapshot = recorder.snapshot();
    assert_eq!(snapshot.events.len(), 1);
    assert!(snapshot.reentrant_events > 0);
    assert!(!snapshot.profile_valid);
}

#[test]
fn json_contains_only_public_event_metadata() {
    let recorder = Recorder::new(8);
    recorder
        .span(Phase::KeyLoad, Accounting::Exclusive)
        .finish();
    let mut bytes = Vec::new();
    recorder.write_json(&mut bytes).unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["schema"], "recursion.phase-events/1");
    assert_eq!(json["events"][0]["phase_id"], Phase::KeyLoad as u16);
    assert!(json["events"][0]["start_ns"].is_u64());
    assert!(json["events"][0]["end_ns"].is_u64());
    assert!(json["events"][0]["thread_id"].is_u64());
    assert_eq!(json["events"][0].as_object().unwrap().len(), 10);
}

#[test]
fn regressing_end_clock_invalidates_the_trace() {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TICK: AtomicU64 = AtomicU64::new(10);
    let recorder = Recorder::with_clock(8, || Some(TICK.fetch_sub(1, Ordering::Relaxed)));
    recorder
        .span(Phase::Synthesis, Accounting::Exclusive)
        .finish();
    let snapshot = recorder.snapshot();
    assert_eq!(snapshot.timer_failures, 1);
    assert!(!snapshot.profile_valid);
}

#[test]
fn threads_have_distinct_ids_and_no_false_parents() {
    let recorder = Recorder::new(8);
    std::thread::scope(|scope| {
        for _ in 0..2 {
            scope.spawn(|| recorder.span(Phase::Ntt, Accounting::Diagnostic).finish());
        }
    });
    let snapshot = recorder.snapshot();
    assert!(snapshot.profile_valid);
    assert_eq!(snapshot.events.len(), 2);
    assert_ne!(snapshot.events[0].thread_id, snapshot.events[1].thread_id);
    assert!(snapshot.events.iter().all(|event| event.parent_id == 0));
}

#[test]
fn flush_child() {
    if std::env::var_os("PROFILE_FLUSH_CHILD").is_some() {
        midnight_proofs::profile::global()
            .span(Phase::Command, Accounting::Diagnostic)
            .finish();
        midnight_proofs::profile::flush().unwrap();
    }
}

#[test]
fn flush_uses_only_the_opt_in_path() {
    // Separate processes avoid racing the environment of parallel tests.
    let dir = std::env::temp_dir().join(format!("profile-events-{}", std::process::id()));
    std::fs::create_dir(&dir).unwrap();
    let output = dir.join("events.json");
    let run = |enabled: bool| {
        let mut command = std::process::Command::new(std::env::current_exe().unwrap());
        command
            .args(["--exact", "flush_child"])
            .current_dir(&dir)
            .env("PROFILE_FLUSH_CHILD", "1")
            .env_remove("MORIARTY_PROFILE_EVENTS");
        if enabled {
            command.env("MORIARTY_PROFILE_EVENTS", &output);
        }
        assert!(command.status().unwrap().success());
    };
    run(false);
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 0);
    run(true);
    let json: serde_json::Value = serde_json::from_slice(&std::fs::read(&output).unwrap()).unwrap();
    assert_eq!(json["events"].as_array().unwrap().len(), 1);
    assert!(json["flush_wall_ns"].is_u64());
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn advice_ordinals_are_public_and_scoped() {
    let recorder = Recorder::new(8);
    {
        let _context = midnight_proofs::profile::AdviceContext::new(2, 3);
        recorder
            .span(Phase::Synthesis, Accounting::Exclusive)
            .finish();
    }
    recorder
        .span(Phase::Multiopen, Accounting::Exclusive)
        .finish();
    let snapshot = recorder.snapshot();
    assert_eq!(snapshot.events[0].advice_phase, 2);
    assert_eq!(snapshot.events[0].circuit_index, 3);
    assert_eq!(snapshot.events[1].advice_phase, u64::MAX);
}

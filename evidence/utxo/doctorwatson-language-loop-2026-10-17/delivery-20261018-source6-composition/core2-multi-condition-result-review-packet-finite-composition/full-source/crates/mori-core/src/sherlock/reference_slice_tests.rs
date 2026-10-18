//! Source-first normal grouping expectations. Rules, counts and saved frames
//! are literal N07/N08 expectations from EXTRA-NORMAL-DERIVATION.md, not values
//! computed by replaying another Machine. Cross-group state equality is only
//! an internal consistency check; the frozen independent oracles stay authoritative.
//! Compilation and execution are separate integrator work.

use super::*;
use std::sync::Arc;

use super::super::extra_oracle_programs;

// N07's 38 charged rules followed by its free publication, transcribed from
// the frozen independent derivation. Changing group size never changes this list.
const N07_RULES: &[usize] = &[
    131, 135, 102, 177, 102, 177, 102, 181, 200, 204, 208, 135, 117, 135, 117, 135, 108, 149, 117,
    135, 102, 149, 108, 153, 223, 153, 223, 149, 117, 135, 102, 149, 108, 153, 223, 153, 223, 185,
    80,
];

fn machine(case: &str, fuel: u128) -> Machine {
    Machine::new(
        Arc::new(extra_oracle_programs::extra_program(case).expect("normal source")),
        fuel,
        8,
    )
}

fn environment(n: u128) -> Environment {
    Environment::from([("0".into(), Value::UInt(n))])
}

fn rules(slice: &ReferenceSlice) -> Vec<usize> {
    slice
        .records
        .iter()
        .map(|record| record.rule_start_line)
        .collect()
}

fn assert_contiguous(slice: &ReferenceSlice) {
    assert_eq!(slice.profile_id, PROFILE);
    assert_eq!(slice.prefix_work.len(), slice.records.len() + 1);
    assert_eq!(slice.prefix_work[0], 0);
    let mut previous = &slice.start;
    let mut work = 0;
    for (index, record) in slice.records.iter().enumerate() {
        assert_eq!(&record.before, previous);
        work += u128::from(record.charge);
        assert_eq!(slice.prefix_work[index + 1], work);
        previous = &record.after;
    }
    assert_eq!(previous, &slice.end);
}

fn cut(machine: &mut Machine, events: usize) -> ReferenceSlice {
    let slice = advance_m(machine, events);
    assert_eq!(slice.records.len(), events);
    assert_eq!(slice.stop, SliceStop::TechnicalCut);
    assert_eq!(slice.end.outcome, Outcome::Pending);
    assert_eq!(
        slice.boundaries,
        vec![SliceBoundary {
            position: events,
            kind: SliceBoundaryKind::TechnicalCut,
        }]
    );
    assert_contiguous(&slice);
    assert_eq!(slice.end, machine.snapshot());
    slice
}

#[test]
fn all_four_group_sizes_retain_n07_literal_rules_and_exact_work() {
    for m in [1, 2, 4, 8] {
        let mut machine = machine("N07", 38);
        let mut retained_rules = Vec::new();
        let mut retained_charges = Vec::new();
        let mut total_work = 0;
        let mut previous = machine.snapshot();
        loop {
            let slice = advance_m(&mut machine, m);
            assert_eq!(slice.start, previous);
            assert!(slice.records.len() <= m);
            assert_contiguous(&slice);
            total_work += slice.prefix_work.last().copied().expect("initial work");
            retained_rules.extend(rules(&slice));
            retained_charges.extend(slice.records.iter().map(|record| record.charge));
            previous = slice.end.clone();
            if slice.stop == SliceStop::Outcome(Outcome::Done(Value::UInt(321))) {
                assert_eq!(
                    slice.boundaries,
                    vec![SliceBoundary {
                        position: slice.records.len(),
                        kind: SliceBoundaryKind::Terminal,
                    }]
                );
                break;
            }
            assert_eq!(slice.stop, SliceStop::TechnicalCut);
            assert!(!slice.records.is_empty());
            assert!(retained_rules.len() < N07_RULES.len());
        }
        assert_eq!(retained_rules, N07_RULES);
        assert_eq!(retained_charges[..38], [1; 38]);
        assert_eq!(retained_charges[38], 0);
        assert_eq!(total_work, 38);
        assert_eq!(previous.remaining_fuel, 0);
        assert!(previous.control.is_none());
        assert!(previous.environment.is_empty());
        assert!(previous.frames.is_empty());

        let terminal = advance_m(&mut machine, m);
        assert_eq!(terminal.start, previous);
        assert_eq!(terminal.end, previous);
        assert!(terminal.records.is_empty());
        assert_eq!(terminal.prefix_work, vec![0]);
        assert_eq!(
            terminal.stop,
            SliceStop::Outcome(Outcome::Done(Value::UInt(321)))
        );
        assert_eq!(
            terminal.boundaries,
            vec![SliceBoundary {
                position: 0,
                kind: SliceBoundaryKind::Terminal,
            }]
        );
    }
}

#[test]
fn n07_cut_keeps_pending_return_then_free_publication_at_zero_fuel() {
    let mut machine = machine("N07", 38);
    for _ in 0..4 {
        cut(&mut machine, 8);
    }
    cut(&mut machine, 4);
    let boundary37 = cut(&mut machine, 1).end;
    assert_eq!(boundary37.control, Some(Control::Return(Value::UInt(321))));
    assert_eq!(boundary37.remaining_fuel, 1);
    assert_eq!(
        boundary37.frames,
        vec![Frame::Return(Type::UInt, Environment::new())]
    );
    assert_eq!(
        boundary37.environment,
        Environment::from([
            ("0".into(), Value::UInt(1)),
            ("1".into(), Value::UInt(2)),
            ("2".into(), Value::UInt(3)),
        ])
    );
    let tail = advance_m(&mut machine, 8);
    assert_eq!(tail.start, boundary37);
    assert_eq!(rules(&tail), vec![185, 80]);
    assert_eq!(tail.prefix_work, vec![0, 1, 1]);
    assert_eq!(tail.records[0].after.remaining_fuel, 0);
    assert_eq!(tail.records[1].charge, 0);
    assert_eq!(tail.end.outcome, Outcome::Done(Value::UInt(321)));
    assert_eq!(tail.end.remaining_fuel, 0);
    assert_eq!(
        tail.stop,
        SliceStop::Outcome(Outcome::Done(Value::UInt(321)))
    );
    assert_contiguous(&tail);
}

#[test]
fn n08_group_cuts_preserve_five_pending_frames_and_each_caller_restoration() {
    let mut machine = machine("N08", 82);
    for _ in 0..9 {
        cut(&mut machine, 8);
    }
    cut(&mut machine, 2);
    let boundary75 = cut(&mut machine, 1).end;
    let saved_frames = vec![
        Frame::Return(Type::UInt, environment(1)),
        Frame::Right(BinaryOp::Mul, Value::UInt(1)),
        Frame::Return(Type::UInt, environment(2)),
        Frame::Right(BinaryOp::Mul, Value::UInt(2)),
        Frame::Return(Type::UInt, Environment::new()),
    ];
    assert_eq!(boundary75.control, Some(Control::Return(Value::UInt(1))));
    assert_eq!(boundary75.environment, environment(0));
    assert_eq!(boundary75.remaining_fuel, 7);
    assert_eq!(boundary75.frames, saved_frames);

    let base_return = cut(&mut machine, 1);
    assert_eq!(rules(&base_return), vec![185]);
    assert_eq!(base_return.end.environment, environment(1));
    assert_eq!(base_return.end.frames, saved_frames[1..].to_vec());
    let middle = cut(&mut machine, 4);
    assert_eq!(rules(&middle), vec![153, 223, 185, 153]);
    assert_eq!(middle.prefix_work, vec![0, 1, 2, 3, 4]);
    assert_eq!(middle.records[2].after.environment, environment(2));
    assert_eq!(middle.records[2].after.frames, saved_frames[3..].to_vec());

    let final_return = cut(&mut machine, 2);
    assert_eq!(rules(&final_return), vec![223, 185]);
    assert_eq!(final_return.prefix_work, vec![0, 1, 2]);
    assert_eq!(
        final_return.end.control,
        Some(Control::Return(Value::UInt(2)))
    );
    assert_eq!(final_return.end.remaining_fuel, 0);
    assert!(final_return.end.environment.is_empty());
    assert!(final_return.end.frames.is_empty());

    // The previous event budget ended before publication. Publication consumes
    // an event position but zero work in the following group.
    let publication = advance_m(&mut machine, 8);
    assert_eq!(publication.start, final_return.end);
    assert_eq!(rules(&publication), vec![80]);
    assert_eq!(publication.prefix_work, vec![0, 0]);
    assert_eq!(publication.end.outcome, Outcome::Done(Value::UInt(2)));
    assert_eq!(publication.end.remaining_fuel, 0);
    assert_eq!(
        publication.stop,
        SliceStop::Outcome(Outcome::Done(Value::UInt(2)))
    );
    assert_contiguous(&publication);
}

#[test]
fn n08_exhaustion_stops_group_with_last_return_frame_intact() {
    let mut machine = machine("N08", 81);
    for _ in 0..10 {
        cut(&mut machine, 8);
    }
    let tail = advance_m(&mut machine, 8);
    assert_eq!(rules(&tail), vec![223, 84]);
    assert_eq!(tail.prefix_work, vec![0, 1, 1]);
    assert_eq!(tail.end.outcome, Outcome::Exhausted);
    assert_eq!(tail.end.remaining_fuel, 0);
    assert_eq!(tail.end.environment, environment(2));
    assert_eq!(
        tail.end.frames,
        vec![Frame::Return(Type::UInt, Environment::new())]
    );
    assert_eq!(tail.stop, SliceStop::Outcome(Outcome::Exhausted));
    assert_eq!(
        tail.boundaries,
        vec![SliceBoundary {
            position: 2,
            kind: SliceBoundaryKind::Terminal,
        }]
    );
    assert_contiguous(&tail);
}

#[test]
fn zero_event_budget_is_an_observation_only_technical_cut() {
    let mut machine = machine("N08", 82);
    let before = machine.snapshot();
    let slice = cut(&mut machine, 0);
    assert_eq!(slice.start, before);
    assert_eq!(slice.end, before);
    assert!(slice.records.is_empty());
    assert_eq!(slice.prefix_work, vec![0]);
}

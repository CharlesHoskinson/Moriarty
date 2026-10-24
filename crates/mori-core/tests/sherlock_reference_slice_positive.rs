//! External ordinary public-API expectations for existing K-A host grouping.
//! Expected states/records are hand-transcribed from the frozen N01 literals
//! and K-A publication rules; no Machine/grouping output computes expectations.
//! Uses only valid fixed literal input, Machine::new and public observations.
//! This source is frozen before the production visibility afterimage.

use mori_core::sherlock::reference_slice::{
    ReferenceSlice, SliceBoundary, SliceBoundaryKind, SliceStop, advance_m,
};
use mori_core::sherlock::{
    Control, Environment, Expr, Machine, Outcome, Program, Snapshot, StepRecord, Value,
};
use std::sync::Arc;

fn literal_program() -> Arc<Program> {
    Arc::new(Program {
        functions: vec![],
        entry: Arc::new(Expr::Literal(Value::UInt(7))),
    })
}

// Complete expected snapshots. All eight fields are explicit, including the
// independent initial fuel and complete ordered empty environments/frames.
fn initial(fuel: u128) -> Snapshot {
    Snapshot {
        program: literal_program(),
        control: Some(Control::Eval(Arc::new(Expr::Literal(Value::UInt(7))))),
        environment: Environment::new(),
        frames: vec![],
        stack_limit: 8,
        remaining_fuel: fuel,
        initial_fuel: fuel,
        outcome: Outcome::Pending,
    }
}
fn pending_return() -> Snapshot {
    Snapshot {
        program: literal_program(),
        control: Some(Control::Return(Value::UInt(7))),
        environment: Environment::new(),
        frames: vec![],
        stack_limit: 8,
        remaining_fuel: 0,
        initial_fuel: 1,
        outcome: Outcome::Pending,
    }
}
fn done() -> Snapshot {
    Snapshot {
        program: literal_program(),
        control: None,
        environment: Environment::new(),
        frames: vec![],
        stack_limit: 8,
        remaining_fuel: 0,
        initial_fuel: 1,
        outcome: Outcome::Done(Value::UInt(7)),
    }
}
fn exhausted() -> Snapshot {
    Snapshot {
        program: literal_program(),
        control: None,
        environment: Environment::new(),
        frames: vec![],
        stack_limit: 8,
        remaining_fuel: 0,
        initial_fuel: 0,
        outcome: Outcome::Exhausted,
    }
}
fn literal_record() -> StepRecord {
    StepRecord {
        rule_start_line: 102,
        charge: 1,
        before: initial(1),
        after: pending_return(),
    }
}
fn publication_record() -> StepRecord {
    StepRecord {
        rule_start_line: 80,
        charge: 0,
        before: pending_return(),
        after: done(),
    }
}
fn expected(
    start: Snapshot,
    records: Vec<StepRecord>,
    prefix_work: Vec<u128>,
    position: usize,
    kind: SliceBoundaryKind,
    end: Snapshot,
    stop: SliceStop,
) -> ReferenceSlice {
    ReferenceSlice {
        profile_id: "sherlock-k-a/1",
        start,
        records,
        prefix_work,
        boundaries: vec![SliceBoundary { position, kind }],
        end,
        stop,
    }
}

#[test]
fn public_widths_keep_literal_charged_and_free_boundaries() {
    for width in [0, 1, 2, 4, 8] {
        let mut machine = Machine::new(literal_program(), 1, 8);
        let wanted = match width {
            0 => expected(
                initial(1),
                vec![],
                vec![0],
                0,
                SliceBoundaryKind::TechnicalCut,
                initial(1),
                SliceStop::TechnicalCut,
            ),
            1 => expected(
                initial(1),
                vec![literal_record()],
                vec![0, 1],
                1,
                SliceBoundaryKind::TechnicalCut,
                pending_return(),
                SliceStop::TechnicalCut,
            ),
            _ => expected(
                initial(1),
                vec![literal_record(), publication_record()],
                vec![0, 1, 1],
                2,
                SliceBoundaryKind::Terminal,
                done(),
                SliceStop::Outcome(Outcome::Done(Value::UInt(7))),
            ),
        };
        let actual = advance_m(&mut machine, width);
        assert_eq!(actual, wanted, "complete literal slice at width {width}");
        assert_eq!(
            machine.snapshot(),
            wanted.end,
            "machine returned state at width {width}"
        );
    }
}

#[test]
fn public_technical_cut_resumes_the_same_pending_publication() {
    let mut machine = Machine::new(literal_program(), 1, 8);
    assert_eq!(
        advance_m(&mut machine, 1),
        expected(
            initial(1),
            vec![literal_record()],
            vec![0, 1],
            1,
            SliceBoundaryKind::TechnicalCut,
            pending_return(),
            SliceStop::TechnicalCut,
        )
    );
    // A zero-event cut does not publish, even with zero remaining fuel.
    assert_eq!(
        advance_m(&mut machine, 0),
        expected(
            pending_return(),
            vec![],
            vec![0],
            0,
            SliceBoundaryKind::TechnicalCut,
            pending_return(),
            SliceStop::TechnicalCut,
        )
    );
    // Publication gets the next event position and contributes no work.
    assert_eq!(
        advance_m(&mut machine, 1),
        expected(
            pending_return(),
            vec![publication_record()],
            vec![0, 0],
            1,
            SliceBoundaryKind::Terminal,
            done(),
            SliceStop::Outcome(Outcome::Done(Value::UInt(7))),
        )
    );
    for width in [0, 1, 2, 4, 8] {
        assert_eq!(
            advance_m(&mut machine, width),
            expected(
                done(),
                vec![],
                vec![0],
                0,
                SliceBoundaryKind::Terminal,
                done(),
                SliceStop::Outcome(Outcome::Done(Value::UInt(7))),
            ),
            "published outcome is unchanged at width {width}"
        );
        assert_eq!(machine.snapshot(), done());
    }
}

#[test]
fn public_zero_fuel_literal_stops_on_the_first_free_publication() {
    // Frozen N01's ordinary fuel0 prefix: rule84 publishes Exhausted directly,
    // with no charged literal transition. It remains a valid fixed A input.
    for width in [0, 1, 2, 4, 8] {
        let mut machine = Machine::new(literal_program(), 0, 8);
        let wanted = if width == 0 {
            expected(
                initial(0),
                vec![],
                vec![0],
                0,
                SliceBoundaryKind::TechnicalCut,
                initial(0),
                SliceStop::TechnicalCut,
            )
        } else {
            expected(
                initial(0),
                vec![StepRecord {
                    rule_start_line: 84,
                    charge: 0,
                    before: initial(0),
                    after: exhausted(),
                }],
                vec![0, 0],
                1,
                SliceBoundaryKind::Terminal,
                exhausted(),
                SliceStop::Outcome(Outcome::Exhausted),
            )
        };
        assert_eq!(
            advance_m(&mut machine, width),
            wanted,
            "complete zero-fuel slice at width {width}"
        );
        assert_eq!(machine.snapshot(), wanted.end);
    }
}

//! Normal-program comparisons against frozen, independently authored literals.
//! `check-reference.py` verifies literal JSON transcription before compiling/running;
//! none of these expectations use
//! the machine, a second evaluator, or a compiler to calculate an answer.

#[path = "helper_call_adapter_tests.rs"]
mod helper_call_adapter_tests;

#[path = "checkpoint_normal_tests.rs"]
mod checkpoint_normal_tests;

use super::reference_display::{KDisplay, environment_to_k};
use super::*;
use std::sync::Arc;

struct ExpectedState {
    charged_steps: u128,
    remaining_fuel: u128,
    depth: usize,
    control: &'static str,
    environment: &'static str,
    frames: &'static [&'static str],
    via_rule_start_line: Option<usize>,
}

struct ExpectedPrefix {
    initial_fuel: u128,
    boundary_state_index: usize,
    remaining_fuel: u128,
    output: &'static str,
    publication_rule_start_line: usize,
}

struct ExpectedCase {
    case_id: &'static str,
    input_sha256: &'static str,
    code: &'static str,
    entry: &'static str,
    expected_value: &'static str,
    initial_fuel: u128,
    stack_limit: usize,
    pending_output: &'static str,
    states: &'static [ExpectedState],
    publication_pre_state_index: usize,
    publication_rule_start_line: usize,
    publication_charge: u8,
    publication_control: &'static str,
    publication_output: &'static str,
    prefixes: &'static [ExpectedPrefix],
}

include!("normal_oracles.rs");
include!("extra_normal_oracles.rs");

use super::normal_oracle_inputs::{expr, no_functions, normal_program, uint};

#[allow(clippy::too_many_arguments)]
fn compare_snapshot(
    actual: &Snapshot,
    case: &ExpectedCase,
    expected: &ExpectedState,
    initial_fuel: u128,
    remaining_fuel: u128,
    control: &str,
    output: &str,
    context: &str,
) {
    assert_eq!(actual.program.to_k(), case.code, "{context}: code");
    assert_eq!(actual.program.entry.to_k(), case.entry, "{context}: entry");
    assert_eq!(
        actual
            .control
            .as_ref()
            .map(KDisplay::to_k)
            .unwrap_or_else(|| ".K".into()),
        control,
        "{context}: control"
    );
    assert_eq!(
        environment_to_k(&actual.environment),
        expected.environment,
        "{context}: environment"
    );
    let frames: Vec<String> = actual.frames.iter().map(KDisplay::to_k).collect();
    assert_eq!(
        frames, expected.frames,
        "{context}: complete top-first frames"
    );
    assert_eq!(actual.depth(), expected.depth, "{context}: depth");
    assert_eq!(
        actual.stack_limit, case.stack_limit,
        "{context}: stack limit"
    );
    assert_eq!(actual.initial_fuel, initial_fuel, "{context}: initial fuel");
    assert_eq!(
        actual.remaining_fuel, remaining_fuel,
        "{context}: remaining fuel"
    );
    assert_eq!(
        actual.charged_steps(),
        expected.charged_steps,
        "{context}: work"
    );
    assert_eq!(actual.outcome.to_k(), output, "{context}: complete outcome");
}

trait NormalExecution {
    fn snapshot(&self) -> Snapshot;
    fn step(&mut self) -> Option<StepRecord>;
    fn run(&mut self) -> Snapshot;
}

impl NormalExecution for Machine {
    fn snapshot(&self) -> Snapshot {
        Machine::snapshot(self)
    }
    fn step(&mut self) -> Option<StepRecord> {
        Machine::step(self)
    }
    fn run(&mut self) -> Snapshot {
        Machine::run(self)
    }
}

impl NormalExecution for super::image_reference::ImageMachine {
    fn snapshot(&self) -> Snapshot {
        self.snapshot()
    }
    fn step(&mut self) -> Option<StepRecord> {
        self.step()
    }
    fn run(&mut self) -> Snapshot {
        self.run()
    }
}

// Exercise the new physical host representation, not a new semantic relation.
// The complete frozen literals below remain the independent authority. The
// encoder/decoder inverse comparison alone would only be internal consistency.
struct ImageMaterializedMachine(Machine);

fn materialize_snapshot(snapshot: &Snapshot) -> Snapshot {
    use super::machine_image::{HostImageLimits, decode_snapshot, encode_snapshot};
    // Explicit host test limits only; these select no native/product capacities.
    let limits = HostImageLimits {
        rows: 20_000,
        cells: 40_000,
        name_bytes: 1_000_000,
    };
    let image = encode_snapshot(snapshot, limits).expect("normal snapshot fits test host limits");
    let materialized = decode_snapshot(&image);
    assert_eq!(
        &materialized, snapshot,
        "complete typed host materialization"
    );
    materialized
}

impl NormalExecution for ImageMaterializedMachine {
    fn snapshot(&self) -> Snapshot {
        materialize_snapshot(&self.0.snapshot())
    }
    fn step(&mut self) -> Option<StepRecord> {
        self.0.step().map(|record| StepRecord {
            rule_start_line: record.rule_start_line,
            charge: record.charge,
            before: materialize_snapshot(&record.before),
            after: materialize_snapshot(&record.after),
        })
    }
    fn run(&mut self) -> Snapshot {
        materialize_snapshot(&self.0.run())
    }
}

// One common assertion consumer; neither implementation computes expectations.
fn compare_case_with<M: NormalExecution>(
    case_id: &str,
    new_machine: impl Fn(Arc<Program>, u128, usize) -> M,
) {
    let case = CASES
        .iter()
        .chain(EXTRA_CASES.iter())
        .find(|case| case.case_id == case_id)
        .expect("frozen case");
    let program = normal_program(case_id);
    assert_eq!(PROFILE, ORACLE_PROFILE);
    assert_eq!(K_SOURCE_SHA256, ORACLE_SOURCE_SHA256);
    assert_eq!(PROFILE, EXTRA_ORACLE_PROFILE);
    assert_eq!(K_SOURCE_SHA256, EXTRA_ORACLE_SOURCE_SHA256);
    assert_eq!(program.to_k(), case.code, "{case_id}: source code term");
    assert_eq!(
        program.entry.to_k(),
        case.entry,
        "{case_id}: source entry term"
    );
    assert_eq!(case.input_sha256.len(), 64);

    for prefix in case.prefixes {
        let context = format!("{case_id}, fuel {}", prefix.initial_fuel);
        let mut machine = new_machine(program.clone(), prefix.initial_fuel, case.stack_limit);
        let mut prior = machine.snapshot();
        let initial = &case.states[0];
        compare_snapshot(
            &prior,
            case,
            initial,
            prefix.initial_fuel,
            prefix.initial_fuel,
            initial.control,
            case.pending_output,
            &format!("{context}, startup post-state"),
        );

        for index in 1..=prefix.boundary_state_index {
            let expected = &case.states[index];
            let step = machine.step().expect("expected charged transition");
            assert_eq!(
                step.before, prior,
                "{context}, state {index}: complete pre-state"
            );
            assert_eq!(
                step.charge, 1,
                "{context}, state {index}: charged transition"
            );
            assert_eq!(
                Some(step.rule_start_line),
                expected.via_rule_start_line,
                "{context}, state {index}: actual via-rule source line"
            );
            // This is the frozen JSON's stated prefix reconstruction, using its
            // literal charged_steps. It computes no program work or result.
            let remaining = prefix.initial_fuel - expected.charged_steps;
            compare_snapshot(
                &step.after,
                case,
                expected,
                prefix.initial_fuel,
                remaining,
                expected.control,
                case.pending_output,
                &format!("{context}, state {index}"),
            );
            assert_eq!(
                machine.snapshot(),
                step.after,
                "{context}: machine post-state"
            );
            prior = step.after;
        }

        let boundary = &case.states[prefix.boundary_state_index];
        compare_snapshot(
            &prior,
            case,
            boundary,
            prefix.initial_fuel,
            prefix.remaining_fuel,
            boundary.control,
            case.pending_output,
            &format!("{context}, prefix boundary"),
        );
        let publication = machine.step().expect("expected free terminal publication");
        assert_eq!(
            publication.before, prior,
            "{context}: publication pre-state"
        );
        assert_eq!(publication.charge, 0, "{context}: free publication");
        assert_eq!(
            publication.rule_start_line, prefix.publication_rule_start_line,
            "{context}: actual publication source line"
        );
        compare_snapshot(
            &publication.after,
            case,
            boundary,
            prefix.initial_fuel,
            prefix.remaining_fuel,
            ".K",
            prefix.output,
            &format!("{context}, publication"),
        );
        assert_eq!(
            machine.snapshot(),
            publication.after,
            "{context}: published machine state"
        );
        assert!(
            machine.step().is_none(),
            "{context}: terminal state has no next transition"
        );
        assert_eq!(
            machine.snapshot(),
            publication.after,
            "{context}: terminal state unchanged"
        );

        // Exercise run() through the same production implementation as step().
        let mut run_machine = new_machine(program.clone(), prefix.initial_fuel, case.stack_limit);
        assert_eq!(
            run_machine.run(),
            publication.after,
            "{context}: run/step complete state"
        );

        if prefix.initial_fuel == case.initial_fuel {
            assert_eq!(
                prefix.boundary_state_index,
                case.publication_pre_state_index
            );
            assert_eq!(
                prefix.publication_rule_start_line,
                case.publication_rule_start_line
            );
            assert_eq!(case.publication_charge, 0);
            assert_eq!(case.publication_control, ".K");
            assert_eq!(prefix.output, case.publication_output);
            assert_eq!(prefix.output, format!("uiDone({})", case.expected_value));
            // Retain each original full-trace fuel literal as a separate check.
            for state in case.states {
                assert_eq!(
                    state.remaining_fuel,
                    case.initial_fuel - state.charged_steps
                );
            }
        }
    }
}

fn compare_case(case_id: &str) {
    compare_case_with(case_id, Machine::new);
}

fn compare_image_case(case_id: &str) {
    compare_case_with(case_id, |program, fuel, stack_limit| {
        let image = super::program_image::prepare_program_image(&program);
        super::image_reference::ImageMachine::new(Arc::new(image), fuel, stack_limit)
    });
}

#[test]
fn normal_n01_literal() {
    compare_case("N01");
}
#[test]
fn normal_n02_addition() {
    compare_case("N02");
}
#[test]
fn normal_n03_conditional() {
    compare_case("N03");
}
#[test]
fn normal_n04_lexical_restoration() {
    compare_case("N04");
}
#[test]
fn normal_n05_named_recursion() {
    compare_case("N05");
}
#[test]
fn normal_n06_ordered_list_tree_match() {
    compare_case("N06");
}

#[test]
fn normal_n07_three_ordered_arguments() {
    compare_case("N07");
}

#[test]
fn normal_n08_recursive_factorial() {
    compare_case("N08");
}

#[test]
fn frozen_normal_oracle_membership() {
    assert_eq!(
        CASES.iter().map(|case| case.case_id).collect::<Vec<_>>(),
        ["N01", "N02", "N03", "N04", "N05", "N06"]
    );
    assert_eq!(
        CASES.iter().map(|case| case.prefixes.len()).sum::<usize>(),
        92
    );
}

#[test]
fn image_n01_literal() {
    compare_image_case("N01");
}

#[test]
fn image_n02_addition() {
    compare_image_case("N02");
}

#[test]
fn image_n03_conditional() {
    compare_image_case("N03");
}

#[test]
fn image_n04_lexical_restoration() {
    compare_image_case("N04");
}

#[test]
fn image_n05_named_recursion() {
    compare_image_case("N05");
}

#[test]
fn image_n06_ordered_list_tree_match() {
    compare_image_case("N06");
}

#[test]
fn image_n07_three_ordered_arguments() {
    compare_image_case("N07");
}

#[test]
fn image_n08_recursive_factorial() {
    compare_image_case("N08");
}

// Compare retained grouped records directly with the frozen literals. This does
// not use an ungrouped execution or either evaluator to derive expectations.
fn compare_grouped_case(case_id: &str) {
    use super::reference_slice::{SliceBoundary, SliceBoundaryKind, SliceStop, advance_m};
    let case = CASES
        .iter()
        .chain(EXTRA_CASES.iter())
        .find(|case| case.case_id == case_id)
        .expect("frozen normal case");
    for m in [1, 2, 4, 8] {
        for prefix in case.prefixes {
            let context = format!("{case_id}, group {m}, fuel {}", prefix.initial_fuel);
            let mut machine = Machine::new(
                normal_program(case_id),
                prefix.initial_fuel,
                case.stack_limit,
            );
            let mut prior = machine.snapshot();
            let initial = &case.states[0];
            compare_snapshot(
                &prior,
                case,
                initial,
                prefix.initial_fuel,
                prefix.initial_fuel,
                initial.control,
                case.pending_output,
                &context,
            );
            let mut charged = 0;
            let mut published = false;
            while !published {
                let slice = advance_m(&mut machine, m);
                assert_eq!(slice.profile_id, PROFILE, "{context}: profile");
                assert_eq!(slice.start, prior, "{context}: complete group start");
                assert!(
                    !slice.records.is_empty(),
                    "{context}: pending execution must retain its next event"
                );
                assert!(slice.records.len() <= m, "{context}: event budget");
                assert_eq!(slice.prefix_work.len(), slice.records.len() + 1);
                assert_eq!(slice.prefix_work[0], 0);
                let mut group_work = 0;
                for (position, record) in slice.records.iter().enumerate() {
                    assert!(!published, "{context}: no event after publication");
                    assert_eq!(
                        record.before, prior,
                        "{context}: complete internal pre-state"
                    );
                    if charged < prefix.boundary_state_index {
                        charged += 1;
                        let expected = &case.states[charged];
                        assert_eq!(record.charge, 1, "{context}: internal charged event");
                        assert_eq!(Some(record.rule_start_line), expected.via_rule_start_line);
                        compare_snapshot(
                            &record.after,
                            case,
                            expected,
                            prefix.initial_fuel,
                            prefix.initial_fuel - expected.charged_steps,
                            expected.control,
                            case.pending_output,
                            &context,
                        );
                    } else {
                        assert_eq!(record.charge, 0, "{context}: free publication");
                        assert_eq!(record.rule_start_line, prefix.publication_rule_start_line);
                        let boundary = &case.states[prefix.boundary_state_index];
                        compare_snapshot(
                            &record.after,
                            case,
                            boundary,
                            prefix.initial_fuel,
                            prefix.remaining_fuel,
                            ".K",
                            prefix.output,
                            &context,
                        );
                        published = true;
                    }
                    group_work += u128::from(record.charge);
                    assert_eq!(slice.prefix_work[position + 1], group_work);
                    prior = record.after.clone();
                }
                assert_eq!(slice.end, prior, "{context}: complete group end");
                assert_eq!(machine.snapshot(), prior, "{context}: live machine end");
                let kind = if published {
                    SliceBoundaryKind::Terminal
                } else {
                    SliceBoundaryKind::TechnicalCut
                };
                assert_eq!(
                    slice.boundaries,
                    vec![SliceBoundary {
                        position: slice.records.len(),
                        kind
                    }]
                );
                assert_eq!(
                    slice.stop,
                    if published {
                        SliceStop::Outcome(prior.outcome.clone())
                    } else {
                        SliceStop::TechnicalCut
                    }
                );
            }
            assert_eq!(charged, prefix.boundary_state_index);
            let terminal = advance_m(&mut machine, m);
            assert_eq!(terminal.start, prior);
            assert_eq!(terminal.end, prior);
            assert!(terminal.records.is_empty());
            assert_eq!(terminal.prefix_work, vec![0]);
            assert_eq!(terminal.stop, SliceStop::Outcome(prior.outcome.clone()));
            assert_eq!(
                terminal.boundaries,
                vec![SliceBoundary {
                    position: 0,
                    kind: SliceBoundaryKind::Terminal
                }]
            );
        }
    }
}

#[test]
fn grouped_n01_literal() {
    compare_grouped_case("N01");
}

#[test]
fn grouped_n02_addition() {
    compare_grouped_case("N02");
}

#[test]
fn grouped_n03_conditional() {
    compare_grouped_case("N03");
}

#[test]
fn grouped_n04_lexical_restoration() {
    compare_grouped_case("N04");
}

#[test]
fn grouped_n05_named_recursion() {
    compare_grouped_case("N05");
}

#[test]
fn grouped_n06_ordered_list_tree_match() {
    compare_grouped_case("N06");
}

#[test]
fn grouped_n07_three_ordered_arguments() {
    compare_grouped_case("N07");
}

#[test]
fn grouped_n08_recursive_factorial() {
    compare_grouped_case("N08");
}

#[test]
fn materialized_n01_literal() {
    compare_case_with("N01", |program, fuel, limit| {
        ImageMaterializedMachine(Machine::new(program, fuel, limit))
    });
}

#[test]
fn materialized_n02_addition() {
    compare_case_with("N02", |program, fuel, limit| {
        ImageMaterializedMachine(Machine::new(program, fuel, limit))
    });
}

#[test]
fn materialized_n03_conditional() {
    compare_case_with("N03", |program, fuel, limit| {
        ImageMaterializedMachine(Machine::new(program, fuel, limit))
    });
}

#[test]
fn materialized_n04_lexical_restoration() {
    compare_case_with("N04", |program, fuel, limit| {
        ImageMaterializedMachine(Machine::new(program, fuel, limit))
    });
}

#[test]
fn materialized_n05_named_recursion() {
    compare_case_with("N05", |program, fuel, limit| {
        ImageMaterializedMachine(Machine::new(program, fuel, limit))
    });
}

#[test]
fn materialized_n06_ordered_list_tree_match() {
    compare_case_with("N06", |program, fuel, limit| {
        ImageMaterializedMachine(Machine::new(program, fuel, limit))
    });
}

#[test]
fn materialized_n07_three_ordered_arguments() {
    compare_case_with("N07", |program, fuel, limit| {
        ImageMaterializedMachine(Machine::new(program, fuel, limit))
    });
}

#[test]
fn materialized_n08_recursive_factorial() {
    compare_case_with("N08", |program, fuel, limit| {
        ImageMaterializedMachine(Machine::new(program, fuel, limit))
    });
}

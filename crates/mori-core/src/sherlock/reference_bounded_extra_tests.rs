//! Production `Machine::run_steps` continuation against the existing A literals.
//!
//! Expected observations come only from the independently frozen normal corpus,
//! through its existing Rust literal transcriptions and HOST-OBSERVATIONS.md's
//! fuel-overlay contract. No one-step baseline or other evaluator supplies them.
//! N03-N08 have 204 fuel rows and 5,510 row/event-cut pairs (including zero and
//! terminal cuts); widths 1/2/4/8 exercise 22,040 fresh continuation schedules.
//! The comparison is the complete diagnostic K projection on the trusted normal
//! typed inputs, plus typed record continuity/current-machine equality. It is
//! neither a structural codec proof nor Candidate B or native evidence.
//!
//! Frozen normal-prefixes.json SHA256:
//! dca67b553d7483fdf7d9148e7f4bf1a6022b42b8843635f33f23c58a2415d56d
//! Frozen extra-normal-prefixes.json SHA256:
//! acf460094b258910bad0499fb8feda0cd85b680023939800ad714d2cb71bee16

use super::normal_oracle_inputs::normal_program;
use super::reference_display::{KDisplay, environment_to_k};
use super::*;

// Reader schema for the existing mechanical JSON transcriptions. Keeping the
// literal files shared avoids another hand-maintained set of expected traces.
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

fn case(case_id: &str) -> &'static ExpectedCase {
    CASES
        .iter()
        .chain(EXTRA_CASES.iter())
        .find(|case| case.case_id == case_id)
        .expect("frozen normal case")
}

// Event position 0 is the initial boundary. Each charged event selects its
// literal next state; the final event retains that state except control/output.
// This is solely the committed reconstruction, not a transition calculation.
fn compare_boundary(
    actual: &Snapshot,
    case: &ExpectedCase,
    prefix: &ExpectedPrefix,
    position: usize,
    context: &str,
) {
    assert!(position <= prefix.boundary_state_index + 1);
    let published = position == prefix.boundary_state_index + 1;
    let expected = &case.states[position.min(prefix.boundary_state_index)];
    let control = if published { ".K" } else { expected.control };
    let output = if published {
        prefix.output
    } else {
        case.pending_output
    };
    let remaining = prefix.initial_fuel - expected.charged_steps;
    assert_eq!(actual.program.to_k(), case.code, "{context}: code");
    assert_eq!(actual.program.entry.to_k(), case.entry, "{context}: entry");
    assert_eq!(
        actual
            .control
            .as_ref()
            .map(KDisplay::to_k)
            .unwrap_or_else(|| ".K".into()),
        control,
        "{context}: control at event boundary {position}"
    );
    assert_eq!(
        environment_to_k(&actual.environment),
        expected.environment,
        "{context}: complete environment at event boundary {position}"
    );
    assert_eq!(
        actual.frames.iter().map(KDisplay::to_k).collect::<Vec<_>>(),
        expected.frames,
        "{context}: complete top-first frames at event boundary {position}"
    );
    assert_eq!(actual.depth(), expected.depth, "{context}: depth");
    assert_eq!(
        actual.stack_limit, case.stack_limit,
        "{context}: stack limit"
    );
    assert_eq!(
        actual.initial_fuel, prefix.initial_fuel,
        "{context}: initial fuel"
    );
    assert_eq!(
        actual.remaining_fuel, remaining,
        "{context}: remaining fuel"
    );
    assert_eq!(
        actual.charged_steps(),
        expected.charged_steps,
        "{context}: work"
    );
    assert_eq!(actual.outcome.to_k(), output, "{context}: complete outcome");
    if position >= prefix.boundary_state_index {
        assert_eq!(
            remaining, prefix.remaining_fuel,
            "{context}: frozen terminal fuel"
        );
    }
}

// The first visit to each (fuel prefix, event position) must pass the full
// independent literal comparison above. Later visits still compare the complete
// typed Snapshot against that validated immutable observation. This avoids
// rendering the same program, control, environment and frames for every replay;
// it does not use another evaluator or skip any Machine::run_steps call.
struct ValidatedBoundaries<'a> {
    case: &'a ExpectedCase,
    prefix: &'a ExpectedPrefix,
    snapshots: Vec<Option<Snapshot>>,
}

impl<'a> ValidatedBoundaries<'a> {
    fn new(case: &'a ExpectedCase, prefix: &'a ExpectedPrefix) -> Self {
        Self {
            case,
            prefix,
            // Positions include the initial state and free terminal publication.
            snapshots: vec![None; prefix.boundary_state_index + 2],
        }
    }

    fn check(&mut self, actual: &Snapshot, position: usize, context: &str) {
        if let Some(validated) = self.snapshots[position].as_ref() {
            assert_eq!(
                actual, validated,
                "{context}: complete validated boundary at {position}"
            );
            return;
        }
        compare_boundary(actual, self.case, self.prefix, position, context);
        // Store only after every independent literal assertion has succeeded.
        self.snapshots[position] = Some(actual.clone());
    }

    fn assert_complete(&self, context: &str) {
        assert!(
            self.snapshots.iter().all(Option::is_some),
            "{context}: every frozen boundary must receive its literal comparison"
        );
    }
}

// Consume exactly the next literal event interval. Both endpoints of every
// record are checked against independent observations, even across call cuts.
fn compare_group(
    machine: &mut Machine,
    boundaries: &mut ValidatedBoundaries<'_>,
    offset: usize,
    budget: usize,
    context: &str,
) -> usize {
    let case = boundaries.case;
    let prefix = boundaries.prefix;
    let event_count = prefix.boundary_state_index + 1;
    let expected_len = budget.min(event_count - offset);
    let before = machine.snapshot();
    boundaries.check(&before, offset, context);
    let records = machine.run_steps(budget);
    assert_eq!(
        records.len(),
        expected_len,
        "{context}: retained event count at {offset}"
    );
    let mut previous = before;
    let mut group_work = 0;
    for (index, record) in records.iter().enumerate() {
        let position = offset + index;
        assert_eq!(
            record.before, previous,
            "{context}: typed record continuity at {position}"
        );
        boundaries.check(&record.before, position, context);
        boundaries.check(&record.after, position + 1, context);
        let (rule, charge) = if position < prefix.boundary_state_index {
            (
                case.states[position + 1]
                    .via_rule_start_line
                    .expect("literal charged edge"),
                1,
            )
        } else {
            (prefix.publication_rule_start_line, 0)
        };
        assert_eq!(
            record.rule_start_line, rule,
            "{context}: source rule at event {position}"
        );
        assert_eq!(
            record.charge, charge,
            "{context}: raw charge at event {position}"
        );
        group_work += u128::from(record.charge);
        previous = record.after.clone();
    }
    let end = offset + expected_len;
    let after = machine.snapshot();
    assert_eq!(
        after, previous,
        "{context}: complete live machine at group end"
    );
    boundaries.check(&after, end, context);
    let expected_work = case.states[end.min(prefix.boundary_state_index)].charged_steps
        - case.states[offset.min(prefix.boundary_state_index)].charged_steps;
    assert_eq!(group_work, expected_work, "{context}: retained prefix work");
    end
}

fn compare_all_frozen_prefixes_and_cuts(case_id: &str) {
    let case = case(case_id);
    let program = normal_program(case_id);
    assert_eq!(PROFILE, ORACLE_PROFILE);
    assert_eq!(PROFILE, EXTRA_ORACLE_PROFILE);
    assert_eq!(K_SOURCE_SHA256, ORACLE_SOURCE_SHA256);
    assert_eq!(K_SOURCE_SHA256, EXTRA_ORACLE_SOURCE_SHA256);
    assert_eq!(program.to_k(), case.code, "{case_id}: input code identity");
    assert_eq!(
        program.entry.to_k(),
        case.entry,
        "{case_id}: input entry identity"
    );

    for prefix in case.prefixes {
        let event_count = prefix.boundary_state_index + 1;
        let mut boundaries = ValidatedBoundaries::new(case, prefix);
        for cut in 0..=event_count {
            for width in [1, 2, 4, 8] {
                let context = format!(
                    "{case_id}, fuel {}, cut {cut}, width {width}",
                    prefix.initial_fuel
                );
                let mut machine =
                    Machine::new(program.clone(), prefix.initial_fuel, case.stack_limit);
                // Cut zero and cut terminal are real run_steps calls, so empty
                // calls and an exact cut immediately before free publication
                // are covered alongside every saved control/frame boundary.
                let mut offset = compare_group(&mut machine, &mut boundaries, 0, cut, &context);
                assert_eq!(offset, cut, "{context}: exact requested cut");
                let saved = machine.snapshot();
                assert!(
                    machine.run_steps(0).is_empty(),
                    "{context}: zero-event observation"
                );
                assert_eq!(
                    machine.snapshot(),
                    saved,
                    "{context}: zero-event cut preserves full state"
                );
                while offset < event_count {
                    offset = compare_group(&mut machine, &mut boundaries, offset, width, &context);
                }
                let terminal = machine.snapshot();
                boundaries.check(&terminal, event_count, &context);
                for budget in [0, 1, 2, 4, 8] {
                    assert!(
                        machine.run_steps(budget).is_empty(),
                        "{context}: terminal event budget {budget}"
                    );
                    assert_eq!(
                        machine.snapshot(),
                        terminal,
                        "{context}: published state remains stable"
                    );
                }
            }
        }
        boundaries.assert_complete(&format!("{case_id}, fuel {}", prefix.initial_fuel));
    }
}

#[test]
fn run_steps_n03_conditional_all_frozen_prefixes_and_cuts() {
    compare_all_frozen_prefixes_and_cuts("N03");
}

#[test]
fn run_steps_n04_lexical_restoration_all_frozen_prefixes_and_cuts() {
    compare_all_frozen_prefixes_and_cuts("N04");
}

#[test]
fn run_steps_n05_named_recursion_all_frozen_prefixes_and_cuts() {
    compare_all_frozen_prefixes_and_cuts("N05");
}

#[test]
fn run_steps_n06_ordered_list_tree_match_all_frozen_prefixes_and_cuts() {
    compare_all_frozen_prefixes_and_cuts("N06");
}

#[test]
fn run_steps_n07_three_ordered_arguments_all_frozen_prefixes_and_cuts() {
    compare_all_frozen_prefixes_and_cuts("N07");
}

#[test]
fn run_steps_n08_recursive_factorial_all_frozen_prefixes_and_cuts() {
    compare_all_frozen_prefixes_and_cuts("N08");
}

#[test]
fn run_steps_extra_normal_frozen_membership() {
    let membership = [
        ("N03", 11, 9, 76),
        ("N04", 19, 17, 208),
        ("N05", 29, 27, 463),
        ("N06", 21, 19, 251),
        ("N07", 40, 38, 859),
        ("N08", 84, 82, 3653),
    ];
    let mut prefix_count = 0;
    let mut cut_count = 0;
    for (case_id, expected_prefixes, charged, expected_cuts) in membership {
        let case = case(case_id);
        assert_eq!(
            case.prefixes.len(),
            expected_prefixes,
            "{case_id}: frozen fuel rows"
        );
        assert_eq!(
            case.states.len(),
            charged + 1,
            "{case_id}: complete charged boundaries"
        );
        assert_eq!(case.publication_pre_state_index, charged);
        assert_eq!(case.initial_fuel, charged as u128);
        assert_eq!(case.input_sha256.len(), 64);
        assert_eq!(case.publication_charge, 0);
        assert_eq!(case.publication_control, ".K");
        assert_eq!(case.publication_rule_start_line, 80);
        assert_eq!(
            case.publication_output,
            format!("uiDone({})", case.expected_value)
        );
        assert_eq!(case.prefixes[charged].output, case.publication_output);
        assert_eq!(
            case.prefixes[charged].publication_rule_start_line,
            case.publication_rule_start_line
        );
        for (index, state) in case.states.iter().enumerate() {
            assert_eq!(
                state.charged_steps, index as u128,
                "{case_id}: literal work"
            );
            assert_eq!(
                state.remaining_fuel,
                case.initial_fuel - state.charged_steps
            );
        }
        let cuts = case
            .prefixes
            .iter()
            .map(|prefix| prefix.boundary_state_index + 2)
            .sum::<usize>();
        assert_eq!(
            cuts, expected_cuts,
            "{case_id}: exhaustive event-cut membership"
        );
        prefix_count += case.prefixes.len();
        cut_count += cuts;
    }
    assert_eq!(prefix_count, 204);
    assert_eq!(cut_count, 5510);
    assert_eq!(4 * cut_count, 22040);
}

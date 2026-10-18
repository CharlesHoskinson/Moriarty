//! SOURCE_ONLY / UNRUN: two ordinary Core-only host integration assertions.
//! Actual execution calls below are proposed test source, never executed here.
//! ASTs, provenance and result literals are independent handwritten expectations.
use crate::sherlock::{Environment, Machine, Outcome, Program, Snapshot, Type, Value};
use crate::sherlock::image_reference::ImageMachine;
use crate::sherlock::program_image::{ExprId, SourceLocator, SourceOwner};
use crate::sherlock_draft::SourceProvenance;
use crate::sherlock_draft_image::{prepare_draft_image, ImageSourceSite};
use crate::sherlock_draft_types::DraftTypeSummary;
use crate::sherlock_draft_tests::{FACTORIAL, TREEFOLD, factorial_expected,
    treefold_expected, factorial_provenance, treefold_provenance};

// Caller choices for these two positive normal examples only. These are not new
// host admission limits, a profile decision, recursion bounds or B capacities.
const CALLER_FUEL: u128 = 2_048;
const CALLER_STACK: usize = 64;

fn check_done(actual: Snapshot, expected_program: &Program, value: u128) {
    assert_eq!(actual.program.as_ref(), expected_program);
    assert_eq!(actual.outcome, Outcome::Done(Value::UInt(value)));
    assert_eq!(actual.control, None);
    assert_eq!(actual.environment, Environment::new());
    assert!(actual.frames.is_empty());
    assert_eq!(actual.initial_fuel, CALLER_FUEL);
    assert_eq!(actual.stack_limit, CALLER_STACK);
    // Exact charge/prefix clocks are deliberately not an oracle here. Existing
    // independent 8-program/216-prefix comparisons remain separate and intact.
}

fn check_pipeline(source: &str, expected: Program, provenance: SourceProvenance,
                  expected_value: u128) {
    let prepared = prepare_draft_image(source).expect("normal parse/typecheck/image preparation");
    assert_eq!(prepared.draft.program.as_ref(), &expected);
    assert_eq!(prepared.draft.provenance, provenance);
    assert_eq!(prepared.types, DraftTypeSummary {
        function_returns: vec![Type::UInt], entry_type: Type::UInt,
    });
    assert_eq!(prepared.image_sources.len(), provenance.expressions.len());
    assert_eq!(prepared.image.source_map.len(), provenance.expressions.len());
    for (index, expected_site) in provenance.expressions.iter().enumerate() {
        assert_eq!(prepared.image_sources[index], ImageSourceSite {
            expression: ExprId(index), source: expected_site.clone(),
        });
        assert_eq!(prepared.image.source_map[index], SourceLocator {
            owner: match expected_site.source.function_index {
                None => SourceOwner::Entry,
                Some(index) => SourceOwner::Function(index),
            },
            ast_path: expected_site.source.child_path.clone(),
        });
    }
    assert_eq!(prepared.image.functions.len(), 1);
    for (index, expected_function) in expected.functions.iter().enumerate() {
        let row = &prepared.image.functions[index];
        assert_eq!(row.source_table_index, index);
        assert_eq!(row.name, expected_function.name);
        assert_eq!(row.return_type, expected_function.return_type);
        assert_eq!(&prepared.image.ordered_parameters[
            row.parameters.start..row.parameters.start + row.parameters.len],
            expected_function.parameters.as_slice());
        assert_eq!(prepared.image.source_map[row.body.0], SourceLocator {
            owner: SourceOwner::Function(index), ast_path: vec![],
        });
    }

    // Distinct genuine consumers of the parsed, checked actual Program/image.
    // Each is compared to literal expectations, never to the other evaluator.
    let mut image_machine = ImageMachine::new(prepared.image, CALLER_FUEL, CALLER_STACK);
    check_done(image_machine.run(), &expected, expected_value);
    let mut ast_machine = Machine::new(prepared.draft.program, CALLER_FUEL, CALLER_STACK);
    check_done(ast_machine.run(), &expected, expected_value);
}

#[test]
fn factorial4_parser_checker_actual_image_and_host_consumers_expect24() {
    // Whole-integer oracle: 4*3*2*1*1 = 24; never obtained from a candidate run.
    check_pipeline(FACTORIAL, factorial_expected(), factorial_provenance(), 24);
}

#[test]
fn treefold_parser_checker_actual_image_and_host_consumers_expect7() {
    // Whole-integer oracle: leaf(3)+leaf(4) folds to 3+4 = 7.
    check_pipeline(TREEFOLD, treefold_expected(), treefold_provenance(), 7);
}

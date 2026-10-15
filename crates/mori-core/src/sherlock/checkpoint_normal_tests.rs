//! Wire as a child of reference_oracles_tests to use its unchanged frozen
//! CASES/EXTRA_CASES and complete K-state assertion consumer. Selected normal checks.
use super::*;
use super::super::helper_call_adapter::{self as adapter, CallContext};
use super::super::image_reference::ImageMachine;
use super::super::program_image::{prepare_program_image, SourceLocator, SourceOwner};

fn case(id: &str) -> &'static ExpectedCase {
    CASES.iter().chain(EXTRA_CASES.iter()).find(|case| case.case_id == id).unwrap()
}
fn entry(path: &[usize]) -> SourceLocator {
    SourceLocator { owner: SourceOwner::Entry, ast_path: path.to_vec() }
}
fn function(path: &[usize]) -> SourceLocator {
    SourceLocator { owner: SourceOwner::Function(0), ast_path: path.to_vec() }
}
fn check_pending(snapshot: &Snapshot, case: &ExpectedCase, index: usize) {
    let expected = &case.states[index];
    compare_snapshot(snapshot, case, expected, case.initial_fuel, expected.remaining_fuel,
        expected.control, case.pending_output, "checkpoint normal frozen boundary");
}
fn check_origin(machine: &ImageMachine, origin: &SourceLocator, owners: &[SourceLocator]) {
    let image = machine.program_image();
    assert_eq!(&image.source_map[machine.control_origin().0], origin);
    assert_eq!(machine.frame_origins().iter()
        .map(|id| image.source_map[id.0].clone()).collect::<Vec<_>>(), owners);
    assert_eq!(machine.frame_origins().len(), machine.snapshot().frames.len());
}
fn advance(machine: &mut ImageMachine, case: &ExpectedCase, start: usize, end: usize) {
    for index in start + 1..=end {
        let record = machine.step().expect("frozen charged transition");
        assert_eq!(record.charge, 1);
        assert_eq!(Some(record.rule_start_line), case.states[index].via_rule_start_line);
        check_pending(&record.before, case, index - 1);
        check_pending(&record.after, case, index);
    }
}

#[test]
fn image_cuts_resume_lexical_arguments_recursion_and_pending_returns_against_k_literals() {
    // Source-path expectations are transcribed from these fixed source ASTs.
    // No source locator or expected state is derived from machine execution.
    let cuts = [
        ("N04", 1, entry(&[]), vec![]), // pending Push owns the let expression
        ("N04", 4, entry(&[1]), vec![entry(&[])]), // lexical scope installed
        ("N05", 16, function(&[1]), vec![entry(&[])]), // recursive Bool argument
        ("N06", 7, entry(&[]), vec![]), // tentative list-pattern bindings
        ("N06", 14, entry(&[2]), vec![entry(&[])]), // tree binding and saved scope
        ("N07", 8, entry(&[]), vec![]), // all three argument values retained
        ("N07", 37, function(&[]), vec![entry(&[])]), // pending typed return
        ("N08", 32, function(&[2, 1]), vec![function(&[2]), entry(&[])]),
        ("N08", 60, function(&[2, 1]), vec![function(&[2]), function(&[2, 1]),
            function(&[2]), entry(&[])]),
        ("N08", 75, function(&[1]), vec![function(&[2, 1]), function(&[2]),
            function(&[2, 1]), function(&[2]), entry(&[])]),
        ("N08", 82, entry(&[]), vec![]), // zero-fuel free done publication next
    ];
    for (id, cut, origin, owners) in cuts {
        let case = case(id);
        let image = Arc::new(prepare_program_image(&normal_program(id)));
        let mut original = ImageMachine::new(image.clone(), case.initial_fuel, case.stack_limit);
        advance(&mut original, case, 0, cut);
        check_pending(&original.snapshot(), case, cut);
        check_origin(&original, &origin, &owners);
        let saved_program = original.snapshot().program;
        let checkpoint = original.checkpoint();
        // The producer can advance independently; the owned cut retains its old
        // controls/frames/values. This equality is supplemental to frozen checks.
        original.run();
        let mut resumed = ImageMachine::resume(checkpoint);
        assert!(Arc::ptr_eq(resumed.program_image(), &image));
        assert!(Arc::ptr_eq(&resumed.snapshot().program, &saved_program));
        check_pending(&resumed.snapshot(), case, cut);
        check_origin(&resumed, &origin, &owners);
        advance(&mut resumed, case, cut, case.publication_pre_state_index);
        let record = resumed.step().expect("frozen free publication");
        assert_eq!(record.rule_start_line, case.publication_rule_start_line);
        assert_eq!(record.charge, case.publication_charge);
        let expected = &case.states[case.publication_pre_state_index];
        compare_snapshot(&record.after, case, expected, case.initial_fuel, expected.remaining_fuel,
            case.publication_control, case.publication_output, "resumed terminal K literal");
        check_origin(&resumed, &entry(&[]), &[]);
        let mut terminal = ImageMachine::resume(resumed.checkpoint());
        compare_snapshot(&terminal.snapshot(), case, expected, case.initial_fuel,
            expected.remaining_fuel, case.publication_control, case.publication_output,
            "checkpoint of terminal machine");
        check_origin(&terminal, &entry(&[]), &[]);
        assert!(terminal.step().is_none());
    }
}

fn check_helper(context: &CallContext, machine: &ImageMachine, case: &ExpectedCase,
    start: usize, completions: usize, origin: &SourceLocator, owners: &[SourceLocator])
{
    check_pending(&context.snapshot().unwrap(), case, start + completions);
    check_pending(&context.resume_snapshot().unwrap(), case,
        if completions == 0 { start } else { start + 1 });
    let (id, locator) = context.resolve_origin(0).unwrap();
    assert_eq!(*id, machine.control_origin());
    assert_eq!(locator, origin);
    let image = machine.program_image();
    let saved = context.saved_frame_origins().expect("actual machine frame owners");
    assert_eq!(saved.iter().map(|id| image.source_map[id.0].clone()).collect::<Vec<_>>(), owners);
}
fn completion(record: &StepRecord, case: &ExpectedCase, start: usize, count: usize) {
    assert_eq!((record.rule_start_line, record.charge), (if count == 1 { 200 } else { 204 }, 1));
    check_pending(&record.before, case, start + count - 1);
    check_pending(&record.after, case, start + count);
}

#[test]
fn helper_cuts_at_every_cursor_resume_to_frozen_find_and_binding_completions() {
    // N07's frozen three-parameter helper work is23/H21/visits13. This includes
    // live pair scratch and inserted environment cells at mid-binding cuts.
    // N08 at32 has the frozen pending multiplication and saved caller Return.
    for (id, start, total, helper_work, visits, origin, owners, values) in [
        ("N07", 8, 23, 21, 13, entry(&[]), vec![],
            vec![Value::UInt(1), Value::UInt(2), Value::UInt(3)]),
        ("N08", 32, 12, 10, 4, function(&[2, 1]), vec![function(&[2]), entry(&[])],
            vec![Value::UInt(1)]),
    ] {
        let case = case(id);
        let image = Arc::new(prepare_program_image(&normal_program(id)));
        let mut machine = ImageMachine::new(image, case.initial_fuel, case.stack_limit);
        advance(&mut machine, case, 0, start);
        check_origin(&machine, &origin, &owners);
        for cut in 0..=total {
            let mut original = CallContext::from_image_machine(&machine).unwrap();
            let mut completed = 0;
            for _ in 0..cut {
                let event = original.step().unwrap().expect("fixed helper prefix");
                if let Some(record) = event.reference_completion {
                    completed += 1;
                    completion(&record, case, start, completed);
                }
            }
            check_helper(&original, &machine, case, start, completed, &origin, &owners);
            let work = original.work();
            let slot = *original.resolve_result_slot(0).unwrap();
            let checkpoint = original.checkpoint();
            let mut resumed = CallContext::resume(checkpoint);
            assert_eq!(resumed.work(), work);
            assert_eq!(*resumed.resolve_result_slot(0).unwrap(), slot);
            assert!(Arc::ptr_eq(resumed.resolve_program(0).unwrap(), original.resolve_program(0).unwrap()));
            assert!(std::ptr::eq(resumed.resolve_source(0).unwrap(), original.resolve_source(0).unwrap()));
            for (index, expected) in values.iter().enumerate() {
                assert_eq!(resumed.resolve_value(adapter::helper::ValueRef(index)).unwrap(), expected);
            }
            check_helper(&resumed, &machine, case, start, completed, &origin, &owners);
            // Event equality supplements independent K boundary/work literals;
            // it checks exact cursor continuation rather than a Find/Bind restart.
            for _ in cut..total {
                let expected_event = original.step().unwrap().expect("original remaining event");
                let actual = resumed.step().unwrap().expect("resumed remaining event");
                assert_eq!(actual.helper_event, expected_event.helper_event);
                assert_eq!(actual.reference_completion, expected_event.reference_completion);
                if let Some(record) = actual.reference_completion {
                    completed += 1;
                    completion(&record, case, start, completed);
                }
                check_helper(&resumed, &machine, case, start, completed, &origin, &owners);
            }
            assert_eq!(completed, 2);
            assert_eq!(resumed.work(), adapter::helper::Work { consumed: total,
                helper: helper_work, helper_visits: visits,
                reference_cursor: case.states[start + 2].charged_steps });
            assert!(resumed.resolve_result_slot(0).unwrap().is_some());
            let mut finished = CallContext::resume(resumed.checkpoint());
            check_helper(&finished, &machine, case, start, 2, &origin, &owners);
            assert!(finished.step().unwrap().is_none());
        }
    }
}

#[test]
fn existing_n08_fuel_prefix_keeps_pending_returns_and_exhausted_outcome_through_cut() {
    // Reuse an already frozen normal prefix; this adds no resource-limit policy.
    let case = case("N08");
    let prefix = case.prefixes.iter().find(|prefix| prefix.initial_fuel == 75).unwrap();
    assert_eq!(prefix.boundary_state_index, 75);
    let mut machine = ImageMachine::new(Arc::new(prepare_program_image(&normal_program("N08"))),
        prefix.initial_fuel, case.stack_limit);
    for index in 1..=75 {
        let record = machine.step().unwrap();
        assert_eq!(record.charge, 1);
        assert_eq!(Some(record.rule_start_line), case.states[index].via_rule_start_line);
        compare_snapshot(&record.after, case, &case.states[index], prefix.initial_fuel,
            prefix.initial_fuel - case.states[index].charged_steps, case.states[index].control,
            case.pending_output, "frozen normal fuel-prefix before cut");
    }
    let mut resumed = ImageMachine::resume(machine.checkpoint());
    let expected = &case.states[75];
    compare_snapshot(&resumed.snapshot(), case, expected, prefix.initial_fuel, 0,
        expected.control, case.pending_output, "resumed frozen pending-return fuel prefix");
    let owners = [function(&[2, 1]), function(&[2]), function(&[2, 1]), function(&[2]), entry(&[])];
    check_origin(&resumed, &function(&[1]), &owners);
    let record = resumed.step().unwrap();
    assert_eq!((record.rule_start_line, record.charge), (prefix.publication_rule_start_line, 0));
    compare_snapshot(&record.after, case, expected, prefix.initial_fuel, prefix.remaining_fuel,
        ".K", prefix.output, "resumed frozen prefix publication");
    let mut exhausted = ImageMachine::resume(resumed.checkpoint());
    compare_snapshot(&exhausted.snapshot(), case, expected, prefix.initial_fuel,
        prefix.remaining_fuel, ".K", prefix.output, "cut retains frozen exhausted outcome");
    check_origin(&exhausted, &function(&[1]), &owners);
    assert!(exhausted.step().is_none());
}

#[test]
fn prior_literal_normal_underflow_keeps_error_publication_and_saved_let_through_cut() {
    // The same ordinary underflow source/rules/paths were frozen before this
    // checkpoint implementation in source_origin_normal_tests; no evaluator
    // calculates these complete expected pending/error snapshots.
    let program = no_functions(expr(Expr::Let("x".into(),
        expr(Expr::Binary(BinaryOp::Sub, uint(1), uint(2))), uint(0))));
    let mut machine = ImageMachine::new(Arc::new(prepare_program_image(&program)), 9, 16);
    for line in [114, 135, 117, 135, 102, 149, 102, 153, 225] {
        let record = machine.step().unwrap();
        assert_eq!((record.rule_start_line, record.charge), (line, 1));
    }
    let pending = Snapshot { program, control: Some(Control::Error(10)),
        environment: Environment::new(),
        frames: vec![Frame::Let("x".into(), uint(0), Environment::new())],
        stack_limit: 16, remaining_fuel: 0, initial_fuel: 9, outcome: Outcome::Pending };
    let rejected = Snapshot { control: None, outcome: Outcome::Rejected(10), ..pending.clone() };
    let mut resumed = ImageMachine::resume(machine.checkpoint());
    assert_eq!(resumed.snapshot(), pending);
    check_origin(&resumed, &entry(&[0]), &[entry(&[])]);
    let record = resumed.step().unwrap();
    assert_eq!((record.rule_start_line, record.charge), (82, 0));
    assert_eq!(record.before, pending);
    assert_eq!(record.after, rejected);
    let mut terminal = ImageMachine::resume(resumed.checkpoint());
    assert_eq!(terminal.snapshot(), rejected);
    check_origin(&terminal, &entry(&[0]), &[entry(&[])]);
    assert!(terminal.step().is_none());
}

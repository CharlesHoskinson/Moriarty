//! Hand-derived normal inputs and literal state/work expectations. UNRUN.
//! No reference evaluator or previous execution supplies expected fields.
use super::*;

const ARGS: Slice = Slice { start: 1, len: 3 };
const TARGET: FunctionRow = FunctionRow { source_table_index: 1, name: NameId(17),
    parameters: Slice { start: 1, len: 3 }, return_sort: Sort::UInt, body: ExprId(40) };
const P0: Parameter = Parameter { name: NameId(0), sort: Sort::UInt };
const P1: Parameter = Parameter { name: NameId(1), sort: Sort::UInt };
const P2: Parameter = Parameter { name: NameId(2), sort: Sort::UInt };
// Source specimen: already evaluated UInt(1), UInt(2), UInt(3) at these handles.
// The value-table adapter is outside this slice; helpers retain exact handles.
const A0: Argument = Argument { value: ValueRef(101), sort: Sort::UInt };
const A1: Argument = Argument { value: ValueRef(102), sort: Sort::UInt };
const A2: Argument = Argument { value: ValueRef(103), sort: Sort::UInt };
const T0: PairTask = PairTask { parameter: P0, argument: A0 };
const T1: PairTask = PairTask { parameter: P1, argument: A1 };
const T2: PairTask = PairTask { parameter: P2, argument: A2 };
const C0: EnvCell = EnvCell { name: NameId(2), value: ValueRef(103), previous: None };
const C1: EnvCell = EnvCell { name: NameId(1), value: ValueRef(102), previous: Some(CellId(0)) };
const C2: EnvCell = EnvCell { name: NameId(0), value: ValueRef(101), previous: Some(CellId(1)) };

// These are explicit retained handles, not reconstructed caller environments.
const INVOKE: Boundary = Boundary { reference_profile: "sherlock-k-a/1",
    program_handle: 7, source_binding_handle: 8,
    control_origin_handle: 9, control: LegacyControl::Invoke { name: NameId(17), arguments: ARGS },
    caller_environment_handle: 77, frames_top_first: Slice { start: 4, len: 2 }, stack_limit: 8,
    reference_initial_fuel: 10, reference_remaining_fuel: 10, outcome: BoundaryOutcome::Pending };
const ENTER: Boundary = Boundary { control: LegacyControl::Enter {
    function: Some(TARGET), arguments: ARGS }, reference_remaining_fuel: 9, ..INVOKE };
const BOUND: Boundary = Boundary { control: LegacyControl::Bound {
    binding: Binding::Fresh(Some(CellId(2))), return_sort: Sort::UInt, body: ExprId(40) },
    reference_remaining_fuel: 8, ..INVOKE };
const FIND_RESUME: Resume = Resume { origin_rule: Rule::InvokeFind,
    origin_reference_cursor: 41, result_slot: 6, saved_pre_state: INVOKE };
const BIND_RESUME: Resume = Resume { origin_rule: Rule::EnterBind,
    origin_reference_cursor: 42, result_slot: 6, saved_pre_state: ENTER };

fn event(kind: EventKind, helper: u8, visits: u8, reference: u8) -> Event {
    Event { kind, work_delta: 1, helper_work_delta: helper,
        helper_visits: visits, reference_cursor_delta: reference }
}

#[test]
fn source_order_three_arguments_and_complete_resume() {
    let functions = [
        FunctionRow { source_table_index: 0, name: NameId(8), parameters: Slice { start: 0, len: 0 },
            return_sort: Sort::UInt, body: ExprId(30) },
        TARGET,
        // Ordinary repeated function name: first match must win and stop the scan.
        FunctionRow { source_table_index: 2, name: NameId(17), parameters: Slice { start: 0, len: 0 },
            return_sort: Sort::Bool, body: ExprId(99) },
    ];
    let parameters = [Parameter { name: NameId(55), sort: Sort::Bool }, P0, P1, P2];
    let arguments = [Argument { value: ValueRef(999), sort: Sort::Bool }, A0, A1, A2];
    let tables = Tables { functions: &functions, parameters: &parameters, arguments: &arguments };
    let mut pairs = [None; 3];
    let mut cells = [None; 3];
    let mut scratch = Scratch { pairs: &mut pairs, cells: &mut cells };
    let mut state = CallHelper::begin(INVOKE, 41, 6).unwrap();
    assert_eq!(state, CallHelper {
        resume: FIND_RESUME, boundary: INVOKE, phase: Phase::FindEntry,
        function_name: NameId(17), arguments: ARGS, row_cursor: 0, selected: None,
        parameter_cursor: 0, argument_cursor: 0, task_len: 0, pending: None,
        duplicate_cursor: None, fresh_root: None, cells_used: 0, binding: Binding::Fresh(None),
        work: Work { consumed: 0, helper: 0, helper_visits: 0, reference_cursor: 41 },
    });
    // Exactly 24 events: 22 helper events plus the two separate A-rule completions.
    // Visits = 2 function rows + 3 pairs + 3 type reads + 3 inserts + 3 duplicate cells = 14.
    let expected = [
        event(EventKind::FindEntry, 1, 0, 0),
        event(EventKind::FindRow { row: 0, matched: false }, 1, 1, 0),
        event(EventKind::FindRow { row: 1, matched: true }, 1, 1, 0),
        event(EventKind::FindExit, 1, 0, 0),
        event(EventKind::CompleteFind, 0, 0, 1),
        event(EventKind::BindEntry, 1, 0, 0),
        event(EventKind::Pair { ordinal: 0 }, 1, 1, 0),
        event(EventKind::Pair { ordinal: 1 }, 1, 1, 0),
        event(EventKind::Pair { ordinal: 2 }, 1, 1, 0),
        event(EventKind::EmptySeed, 1, 0, 0),
        event(EventKind::Type { task: 2, accepted: true }, 1, 1, 0),
        event(EventKind::Insert { cell: CellId(0) }, 1, 1, 0),
        event(EventKind::Release { task: 2 }, 1, 0, 0),
        event(EventKind::Type { task: 1, accepted: true }, 1, 1, 0),
        event(EventKind::Duplicate { cell: CellId(0), matched: false }, 1, 1, 0),
        event(EventKind::Insert { cell: CellId(1) }, 1, 1, 0),
        event(EventKind::Release { task: 1 }, 1, 0, 0),
        event(EventKind::Type { task: 0, accepted: true }, 1, 1, 0),
        event(EventKind::Duplicate { cell: CellId(1), matched: false }, 1, 1, 0),
        event(EventKind::Duplicate { cell: CellId(0), matched: false }, 1, 1, 0),
        event(EventKind::Insert { cell: CellId(2) }, 1, 1, 0),
        event(EventKind::Release { task: 0 }, 1, 0, 0),
        event(EventKind::BindExit, 1, 0, 0),
        event(EventKind::CompleteBind, 0, 0, 1),
    ];
    for (index, expected_event) in expected.into_iter().enumerate() {
        assert_eq!(state.step(&tables, &mut scratch).unwrap(), Some(expected_event));
        // The complete original caller context stays available at every event.
        assert_eq!(state.boundary.program_handle, 7);
        assert_eq!(state.boundary.source_binding_handle, 8);
        assert_eq!(state.boundary.control_origin_handle, 9);
        assert_eq!(state.boundary.caller_environment_handle, 77);
        assert_eq!(state.boundary.frames_top_first, Slice { start: 4, len: 2 });
        assert_eq!(state.boundary.stack_limit, 8);
        assert_eq!(state.boundary.reference_initial_fuel, 10);
        assert_eq!(state.boundary.reference_profile, "sherlock-k-a/1");
        assert_eq!(state.boundary.outcome, BoundaryOutcome::Pending);
        assert_eq!(state.resume, if index < 4 { FIND_RESUME } else { BIND_RESUME });
        if index == 8 {
            assert_eq!(state, CallHelper {
                resume: BIND_RESUME, boundary: ENTER, phase: Phase::PairCursor,
                function_name: NameId(17), arguments: ARGS, row_cursor: 1, selected: Some(TARGET),
                parameter_cursor: 3, argument_cursor: 3, task_len: 3, pending: None,
                duplicate_cursor: None, fresh_root: None, cells_used: 0, binding: Binding::Fresh(None),
                work: Work { consumed: 9, helper: 8, helper_visits: 5, reference_cursor: 42 },
            });
            assert_eq!(&*scratch.pairs, &[Some(T0), Some(T1), Some(T2)]);
            assert_eq!(&*scratch.cells, &[None, None, None]);
        }
        if index == 11 {
            assert_eq!(state, CallHelper {
                resume: BIND_RESUME, boundary: ENTER, phase: Phase::Release,
                function_name: NameId(17), arguments: ARGS, row_cursor: 1, selected: Some(TARGET),
                parameter_cursor: 3, argument_cursor: 3, task_len: 3, pending: Some(T2),
                duplicate_cursor: None, fresh_root: Some(CellId(0)), cells_used: 1,
                binding: Binding::Fresh(Some(CellId(0))),
                work: Work { consumed: 12, helper: 11, helper_visits: 7, reference_cursor: 42 },
            });
            assert_eq!(&*scratch.pairs, &[Some(T0), Some(T1), Some(T2)]);
            assert_eq!(&*scratch.cells, &[Some(C0), None, None]);
        }
    }
    assert_eq!(state, CallHelper {
        resume: BIND_RESUME, boundary: BOUND, phase: Phase::Done,
        function_name: NameId(17), arguments: ARGS, row_cursor: 1, selected: Some(TARGET),
        parameter_cursor: 3, argument_cursor: 3, task_len: 0, pending: None,
        duplicate_cursor: None, fresh_root: Some(CellId(2)), cells_used: 3,
        binding: Binding::Fresh(Some(CellId(2))),
        work: Work { consumed: 24, helper: 22, helper_visits: 14, reference_cursor: 43 },
    });
    assert_eq!(&*scratch.pairs, &[None, None, None]);
    assert_eq!(&*scratch.cells, &[Some(C0), Some(C1), Some(C2)]);
    assert_eq!(state.step(&tables, &mut scratch).unwrap(), None);
}

#[test]
fn zero_arguments_produce_a_fresh_empty_binding() {
    let no_args = Slice { start: 0, len: 0 };
    let function = FunctionRow { source_table_index: 0, name: NameId(3),
        parameters: no_args, return_sort: Sort::UInt, body: ExprId(4) };
    let invoke = Boundary { control: LegacyControl::Invoke { name: NameId(3), arguments: no_args },
        ..INVOKE };
    let enter = Boundary { control: LegacyControl::Enter { function: Some(function), arguments: no_args },
        reference_remaining_fuel: 9, ..invoke };
    let functions = [function];
    let tables = Tables { functions: &functions, parameters: &[], arguments: &[] };
    let mut scratch = Scratch { pairs: &mut [], cells: &mut [] };
    let mut state = CallHelper::begin(invoke, 41, 6).unwrap();
    let expected = [
        event(EventKind::FindEntry, 1, 0, 0),
        event(EventKind::FindRow { row: 0, matched: true }, 1, 1, 0),
        event(EventKind::FindExit, 1, 0, 0),
        event(EventKind::CompleteFind, 0, 0, 1),
        event(EventKind::BindEntry, 1, 0, 0),
        event(EventKind::EmptySeed, 1, 0, 0),
        event(EventKind::BindExit, 1, 0, 0),
        event(EventKind::CompleteBind, 0, 0, 1),
    ];
    for e in expected { assert_eq!(state.step(&tables, &mut scratch).unwrap(), Some(e)); }
    assert_eq!(state, CallHelper {
        resume: Resume { origin_rule: Rule::EnterBind, origin_reference_cursor: 42,
            result_slot: 6, saved_pre_state: enter },
        boundary: Boundary { control: LegacyControl::Bound { binding: Binding::Fresh(None),
            return_sort: Sort::UInt, body: ExprId(4) }, reference_remaining_fuel: 8, ..invoke },
        phase: Phase::Done, function_name: NameId(3), arguments: no_args,
        row_cursor: 0, selected: Some(function), parameter_cursor: 0, argument_cursor: 0,
        task_len: 0, pending: None, duplicate_cursor: None, fresh_root: None, cells_used: 0,
        binding: Binding::Fresh(None),
        work: Work { consumed: 8, helper: 6, helper_visits: 1, reference_cursor: 43 },
    });
}

#[test]
fn ordinary_duplicate_parameters_preserve_bad_binding_semantics() {
    let two = Slice { start: 0, len: 2 };
    let function = FunctionRow { source_table_index: 0, name: NameId(5),
        parameters: two, return_sort: Sort::UInt, body: ExprId(6) };
    let invoke = Boundary { control: LegacyControl::Invoke { name: NameId(5), arguments: two }, ..INVOKE };
    let enter = Boundary { control: LegacyControl::Enter { function: Some(function), arguments: two },
        reference_remaining_fuel: 9, ..invoke };
    let functions = [function];
    let parameters = [P0, P0];
    let arguments = [A0, A1];
    let tables = Tables { functions: &functions, parameters: &parameters, arguments: &arguments };
    let mut pairs = [None; 2];
    let mut cells = [None; 2];
    let mut scratch = Scratch { pairs: &mut pairs, cells: &mut cells };
    let mut state = CallHelper::begin(invoke, 41, 6).unwrap();
    let expected = [
        event(EventKind::FindEntry, 1, 0, 0),
        event(EventKind::FindRow { row: 0, matched: true }, 1, 1, 0),
        event(EventKind::FindExit, 1, 0, 0),
        event(EventKind::CompleteFind, 0, 0, 1),
        event(EventKind::BindEntry, 1, 0, 0),
        event(EventKind::Pair { ordinal: 0 }, 1, 1, 0),
        event(EventKind::Pair { ordinal: 1 }, 1, 1, 0),
        event(EventKind::EmptySeed, 1, 0, 0),
        event(EventKind::Type { task: 1, accepted: true }, 1, 1, 0),
        event(EventKind::Insert { cell: CellId(0) }, 1, 1, 0),
        event(EventKind::Release { task: 1 }, 1, 0, 0),
        event(EventKind::Type { task: 0, accepted: true }, 1, 1, 0),
        event(EventKind::Duplicate { cell: CellId(0), matched: true }, 1, 1, 0),
        event(EventKind::Release { task: 0 }, 1, 0, 0),
        event(EventKind::BindExit, 1, 0, 0),
        event(EventKind::CompleteBind, 0, 0, 1),
    ];
    for e in expected { assert_eq!(state.step(&tables, &mut scratch).unwrap(), Some(e)); }
    assert_eq!(state, CallHelper {
        resume: Resume { origin_rule: Rule::EnterBind, origin_reference_cursor: 42,
            result_slot: 6, saved_pre_state: enter },
        boundary: Boundary { control: LegacyControl::Bound { binding: Binding::Bad,
            return_sort: Sort::UInt, body: ExprId(6) }, reference_remaining_fuel: 8, ..invoke },
        phase: Phase::Done, function_name: NameId(5), arguments: two,
        row_cursor: 0, selected: Some(function), parameter_cursor: 2, argument_cursor: 2,
        task_len: 0, pending: None, duplicate_cursor: None, fresh_root: Some(CellId(0)), cells_used: 1,
        binding: Binding::Bad,
        work: Work { consumed: 16, helper: 14, helper_visits: 7, reference_cursor: 43 },
    });
    assert_eq!(&*scratch.pairs, &[None, None]);
    // The tail was bound first. It is retained as an unreachable partial result,
    // not exposed as a successful environment or replaced by the head argument.
    assert_eq!(&*scratch.cells, &[Some(EnvCell { name: NameId(0), value: ValueRef(102), previous: None }), None]);
}

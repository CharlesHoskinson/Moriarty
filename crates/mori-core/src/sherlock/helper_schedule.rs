//! Source-only proposal for one call-helper slice, not an adopted B executor.
//! No A evaluator, preparation, native code, dynamic allocation or recursive
//! traversal is invoked by `step`. Table and handle adapters are still absent.
pub const PROPOSED_PROFILE: &str = "sherlock-helper-microstep/1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NameId(pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExprId(pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValueRef(pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellId(pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slice {
    pub start: usize,
    pub len: usize,
}

/// Only the existing four K-A sorts. No generic ADT declaration validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sort {
    UInt,
    Bool,
    List,
    Tree,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Parameter {
    pub name: NameId,
    pub sort: Sort,
}
/// An already evaluated immutable value handle and its existing K-A outer sort.
/// A real value-table adapter must establish this association separately.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Argument {
    pub value: ValueRef,
    pub sort: Sort,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FunctionRow {
    pub source_table_index: usize,
    pub name: NameId,
    pub parameters: Slice,
    pub return_sort: Sort,
    pub body: ExprId,
}
pub struct Tables<'a> {
    /// Original source order, including repeated names; never sorted here.
    pub functions: &'a [FunctionRow],
    pub parameters: &'a [Parameter],
    pub arguments: &'a [Argument],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PairTask {
    pub parameter: Parameter,
    pub argument: Argument,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnvCell {
    pub name: NameId,
    pub value: ValueRef,
    pub previous: Option<CellId>,
}
pub struct Scratch<'a> {
    /// Supplied zeroed storage. One saved pair per pair event; one release per event.
    pub pairs: &'a mut [Option<PairTask>],
    /// Fresh parameter environment only; caller cells are a distinct namespace.
    pub cells: &'a mut [Option<EnvCell>],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Binding {
    Fresh(Option<CellId>),
    Bad,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegacyControl {
    Invoke {
        name: NameId,
        arguments: Slice,
    },
    Enter {
        function: Option<FunctionRow>,
        arguments: Slice,
    },
    Bound {
        binding: Binding,
        return_sort: Sort,
        body: ExprId,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundaryOutcome {
    Pending,
}
/// Complete saved reference boundary by immutable handles. The handle resolver
/// is outside this slice: program, caller env and every frame remain available.
/// Top-first matches the actual A reference; the B bottom-first adapter is open.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Boundary {
    pub reference_profile: &'static str,
    pub program_handle: usize,
    pub source_binding_handle: usize,
    pub control_origin_handle: usize,
    pub control: LegacyControl,
    pub caller_environment_handle: usize,
    pub frames_top_first: Slice,
    pub stack_limit: usize,
    pub reference_initial_fuel: u128,
    pub reference_remaining_fuel: u128,
    pub outcome: BoundaryOutcome,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    InvokeFind,
    EnterBind,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Resume {
    pub origin_rule: Rule,
    pub origin_reference_cursor: u128,
    pub result_slot: usize,
    pub saved_pre_state: Boundary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    FindEntry,
    FindCursor,
    FindExit,
    CompleteFind,
    BindEntry,
    PairCursor,
    Type,
    Duplicate,
    Insert,
    Release,
    Propagate,
    BindExit,
    CompleteBind,
    Done,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventKind {
    FindEntry,
    FindRow { row: usize, matched: bool },
    FindEmpty,
    FindExit,
    CompleteFind,
    BindEntry,
    Pair { ordinal: usize },
    EmptySeed,
    Arity,
    Type { task: usize, accepted: bool },
    Duplicate { cell: CellId, matched: bool },
    Insert { cell: CellId },
    Release { task: usize },
    Propagate { task: usize },
    BindExit,
    CompleteBind,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Event {
    pub kind: EventKind,
    pub work_delta: u8,
    pub helper_work_delta: u8,
    pub helper_visits: u8,
    pub reference_cursor_delta: u8,
}
/// Local host counters; no native widths or maximum-work policy is selected.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Work {
    pub consumed: u128,
    pub helper: u128,
    pub helper_visits: u128,
    pub reference_cursor: u128,
}
/// Missing adapters/policy are engineering holes, outside application Outcome.
/// This is not B exhaustion, a new rejection reason, admission or cap policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unsupported {
    SourceHandleAdapter,
    ScratchStateAdapter,
    TaskCapacityPolicy,
    EnvironmentCapacityPolicy,
    CounterAdmission,
    ReferenceFuelBoundary,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallHelper {
    pub resume: Resume,
    pub boundary: Boundary,
    pub phase: Phase,
    pub function_name: NameId,
    pub arguments: Slice,
    pub row_cursor: usize,
    pub selected: Option<FunctionRow>,
    pub parameter_cursor: usize,
    pub argument_cursor: usize,
    pub task_len: usize,
    pub pending: Option<PairTask>,
    pub duplicate_cursor: Option<CellId>,
    pub fresh_root: Option<CellId>,
    pub cells_used: usize,
    pub binding: Binding,
    pub work: Work,
}

impl CallHelper {
    /// Entry preparation copies fixed host fields; no source record is visited.
    /// Caller provides an actual Invoke boundary and externally retained handles.
    pub fn begin(
        boundary: Boundary,
        reference_cursor: u128,
        result_slot: usize,
    ) -> Result<Self, Unsupported> {
        let LegacyControl::Invoke { name, arguments } = boundary.control else {
            return Err(Unsupported::SourceHandleAdapter);
        };
        Ok(Self {
            resume: Resume {
                origin_rule: Rule::InvokeFind,
                origin_reference_cursor: reference_cursor,
                result_slot,
                saved_pre_state: boundary,
            },
            boundary,
            phase: Phase::FindEntry,
            function_name: name,
            arguments,
            row_cursor: 0,
            selected: None,
            parameter_cursor: 0,
            argument_cursor: 0,
            task_len: 0,
            pending: None,
            duplicate_cursor: None,
            fresh_root: None,
            cells_used: 0,
            binding: Binding::Fresh(None),
            work: Work {
                consumed: 0,
                helper: 0,
                helper_visits: 0,
                reference_cursor,
            },
        })
    }

    /// One proposed event. No loops, iterator searches, recursive calls, map
    /// operations, whole-value cloning, reallocation or global task cleanup.
    /// All writes are deferred until fixed host arithmetic/handle checks finish.
    /// Success events cost one; missing policy returns Unsupported with no change.
    pub fn step(
        &mut self,
        tables: &Tables<'_>,
        scratch: &mut Scratch<'_>,
    ) -> Result<Option<Event>, Unsupported> {
        if self.phase == Phase::Done {
            return Ok(None);
        }
        let mut next = *self;
        let mut pair_write: Option<(usize, Option<PairTask>)> = None;
        let mut cell_write: Option<(usize, EnvCell)> = None;
        let (kind, visits) = match self.phase {
            Phase::FindEntry => {
                next.phase = Phase::FindCursor;
                (EventKind::FindEntry, 0)
            }
            Phase::FindCursor => match tables.functions.get(self.row_cursor).copied() {
                Some(row) => {
                    let matched = row.name == self.function_name;
                    if matched {
                        next.selected = Some(row);
                        next.phase = Phase::FindExit;
                    } else {
                        next.row_cursor = add(self.row_cursor, 1)?;
                    }
                    (
                        EventKind::FindRow {
                            row: self.row_cursor,
                            matched,
                        },
                        1,
                    )
                }
                None => {
                    next.phase = Phase::FindExit;
                    (EventKind::FindEmpty, 0)
                }
            },
            Phase::FindExit => {
                next.phase = Phase::CompleteFind;
                (EventKind::FindExit, 0)
            }
            Phase::CompleteFind => {
                next.boundary.reference_remaining_fuel = self
                    .boundary
                    .reference_remaining_fuel
                    .checked_sub(1)
                    .ok_or(Unsupported::ReferenceFuelBoundary)?;
                next.boundary.control = LegacyControl::Enter {
                    function: self.selected,
                    arguments: self.arguments,
                };
                next.phase = if self.selected.is_some() {
                    Phase::BindEntry
                } else {
                    Phase::Done
                };
                next.resume.origin_rule = Rule::EnterBind;
                next.resume.origin_reference_cursor = self
                    .work
                    .reference_cursor
                    .checked_add(1)
                    .ok_or(Unsupported::CounterAdmission)?;
                next.resume.saved_pre_state = next.boundary;
                (EventKind::CompleteFind, 0)
            }
            Phase::BindEntry => {
                next.phase = Phase::PairCursor;
                (EventKind::BindEntry, 0)
            }
            Phase::PairCursor => {
                let function = self.selected.ok_or(Unsupported::SourceHandleAdapter)?;
                let p_end = self.parameter_cursor == function.parameters.len;
                let a_end = self.argument_cursor == self.arguments.len;
                if p_end && a_end {
                    next.fresh_root = None;
                    next.binding = Binding::Fresh(None);
                    next.phase = if self.task_len == 0 {
                        Phase::BindExit
                    } else {
                        Phase::Type
                    };
                    (EventKind::EmptySeed, 0)
                } else if p_end || a_end {
                    next.binding = Binding::Bad;
                    next.phase = if self.task_len == 0 {
                        Phase::BindExit
                    } else {
                        Phase::Propagate
                    };
                    (EventKind::Arity, 0)
                } else {
                    let slot = scratch
                        .pairs
                        .get(self.task_len)
                        .ok_or(Unsupported::TaskCapacityPolicy)?;
                    if slot.is_some() {
                        return Err(Unsupported::ScratchStateAdapter);
                    }
                    let parameter = *tables
                        .parameters
                        .get(add(function.parameters.start, self.parameter_cursor)?)
                        .ok_or(Unsupported::SourceHandleAdapter)?;
                    let argument = *tables
                        .arguments
                        .get(add(self.arguments.start, self.argument_cursor)?)
                        .ok_or(Unsupported::SourceHandleAdapter)?;
                    pair_write = Some((
                        self.task_len,
                        Some(PairTask {
                            parameter,
                            argument,
                        }),
                    ));
                    next.task_len = add(self.task_len, 1)?;
                    next.parameter_cursor = add(self.parameter_cursor, 1)?;
                    next.argument_cursor = add(self.argument_cursor, 1)?;
                    (
                        EventKind::Pair {
                            ordinal: self.parameter_cursor,
                        },
                        1,
                    )
                }
            }
            Phase::Type => {
                let task = self
                    .task_len
                    .checked_sub(1)
                    .ok_or(Unsupported::ScratchStateAdapter)?;
                let pair = scratch
                    .pairs
                    .get(task)
                    .copied()
                    .flatten()
                    .ok_or(Unsupported::ScratchStateAdapter)?;
                let accepted = pair.parameter.sort == pair.argument.sort;
                next.pending = Some(pair);
                if accepted {
                    next.duplicate_cursor = self.fresh_root;
                    next.phase = if self.fresh_root.is_some() {
                        Phase::Duplicate
                    } else {
                        Phase::Insert
                    };
                } else {
                    next.binding = Binding::Bad;
                    next.phase = Phase::Release;
                }
                (EventKind::Type { task, accepted }, 1)
            }
            Phase::Duplicate => {
                let pair = self.pending.ok_or(Unsupported::ScratchStateAdapter)?;
                let id = self
                    .duplicate_cursor
                    .ok_or(Unsupported::ScratchStateAdapter)?;
                let cell = scratch
                    .cells
                    .get(id.0)
                    .copied()
                    .flatten()
                    .ok_or(Unsupported::ScratchStateAdapter)?;
                let matched = cell.name == pair.parameter.name;
                if matched {
                    next.binding = Binding::Bad;
                    next.phase = Phase::Release;
                } else {
                    next.duplicate_cursor = cell.previous;
                    next.phase = if cell.previous.is_some() {
                        Phase::Duplicate
                    } else {
                        Phase::Insert
                    };
                }
                (EventKind::Duplicate { cell: id, matched }, 1)
            }
            Phase::Insert => {
                let pair = self.pending.ok_or(Unsupported::ScratchStateAdapter)?;
                let destination = scratch
                    .cells
                    .get(self.cells_used)
                    .ok_or(Unsupported::EnvironmentCapacityPolicy)?;
                if destination.is_some() {
                    return Err(Unsupported::ScratchStateAdapter);
                }
                let cell = CellId(self.cells_used);
                cell_write = Some((
                    cell.0,
                    EnvCell {
                        name: pair.parameter.name,
                        value: pair.argument.value,
                        previous: self.fresh_root,
                    },
                ));
                next.cells_used = add(self.cells_used, 1)?;
                next.fresh_root = Some(cell);
                next.binding = Binding::Fresh(Some(cell));
                next.phase = Phase::Release;
                (EventKind::Insert { cell }, 1)
            }
            Phase::Release | Phase::Propagate => {
                let task = self
                    .task_len
                    .checked_sub(1)
                    .ok_or(Unsupported::ScratchStateAdapter)?;
                if scratch.pairs.get(task).copied().flatten().is_none() {
                    return Err(Unsupported::ScratchStateAdapter);
                }
                pair_write = Some((task, None));
                next.task_len = task;
                next.pending = None;
                next.duplicate_cursor = None;
                next.phase = if task == 0 {
                    Phase::BindExit
                } else if self.binding == Binding::Bad {
                    Phase::Propagate
                } else {
                    Phase::Type
                };
                if self.phase == Phase::Propagate {
                    (EventKind::Propagate { task }, 1)
                } else {
                    (EventKind::Release { task }, 0)
                }
            }
            Phase::BindExit => {
                next.phase = Phase::CompleteBind;
                (EventKind::BindExit, 0)
            }
            Phase::CompleteBind => {
                let function = self.selected.ok_or(Unsupported::SourceHandleAdapter)?;
                next.boundary.reference_remaining_fuel = self
                    .boundary
                    .reference_remaining_fuel
                    .checked_sub(1)
                    .ok_or(Unsupported::ReferenceFuelBoundary)?;
                next.boundary.control = LegacyControl::Bound {
                    binding: self.binding,
                    return_sort: function.return_sort,
                    body: function.body,
                };
                next.phase = Phase::Done;
                (EventKind::CompleteBind, 0)
            }
            Phase::Done => return Ok(None),
        };
        let reference = u8::from(matches!(
            kind,
            EventKind::CompleteFind | EventKind::CompleteBind
        ));
        let helper = 1 - reference;
        next.work = Work {
            consumed: self
                .work
                .consumed
                .checked_add(1)
                .ok_or(Unsupported::CounterAdmission)?,
            helper: self
                .work
                .helper
                .checked_add(helper as u128)
                .ok_or(Unsupported::CounterAdmission)?,
            helper_visits: self
                .work
                .helper_visits
                .checked_add(visits as u128)
                .ok_or(Unsupported::CounterAdmission)?,
            reference_cursor: self
                .work
                .reference_cursor
                .checked_add(reference as u128)
                .ok_or(Unsupported::CounterAdmission)?,
        };
        if let Some((slot, value)) = pair_write {
            scratch.pairs[slot] = value;
        }
        if let Some((slot, value)) = cell_write {
            scratch.cells[slot] = Some(value);
        }
        *self = next;
        Ok(Some(Event {
            kind,
            work_delta: 1,
            helper_work_delta: helper,
            helper_visits: visits,
            reference_cursor_delta: reference,
        }))
    }
}

fn add(a: usize, b: usize) -> Result<usize, Unsupported> {
    a.checked_add(b).ok_or(Unsupported::CounterAdmission)
}

#[cfg(test)]
#[path = "helper_schedule_tests.rs"]
mod tests;

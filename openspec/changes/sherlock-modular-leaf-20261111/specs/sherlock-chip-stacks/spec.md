## ADDED Requirements

### Requirement: STK-001 One stack step is an optional rewrite followed by none, push or pop
FRAMES and TASKS SHALL be hash-chain stacks with a cached top. One `StackStep` per stack per slot SHALL be an optional rewrite of the top, then one of none, push or pop, as one owner event. A rewrite followed by a push SHALL spill the rewritten top and SHALL install the child as the new top.

#### Scenario: Rewrite then push
- **WHEN** a rewrite and a push happen in one step at `count = 1` and at `count >= 2`
- **THEN** the rewritten top is spilled and the child becomes the top (positive `stack_rewrite_then_push_one_step`), and the positive `stack_rewrite_then_pop_one_step` passes

#### Scenario: Compound step forged
- **WHEN** the spill hashes the old top, the child is not installed, or the count grows by two
- **THEN** the correct circuit refuses the witness (mutants `stack_compound_spills_unrewritten_top`, `stack_compound_child_not_installed`, `stack_compound_count_plus_two`), and the Quint invariant `compoundStepIsOneEvent` of `hashStacks.qnt` holds

#### Scenario: Lean evidence for the compound step
- **WHEN** the kit is offered for completion
- **THEN** O11 (`HashStacks.lean`, cached-top protocol with the compound step refines the list function) and O46 (partition independence for the compound effect) are proved, and P7 (proved in C3) is cited only for one `StackOp` per stack per step, and today O11 and O46 are open

### Requirement: STK-002 The count is the rule and an empty stack has the fixed non-zero constant
`count` SHALL decide the stack shape. With `count = 0` the top SHALL be zero and `below` SHALL be the fixed non-zero constant `EMPTY_F` or `EMPTY_T`. With `count = 1`, `below` SHALL be that constant. `count` SHALL NOT wrap. A rewrite or a pop at `count = 0` SHALL leave the stack empty, as the reference list function does.

#### Scenario: Empty-stack shape forged
- **WHEN** `count = 0` with `below = 0` or a non-zero top, or `count = 1` with `below != EMPTY`, or the count wraps
- **THEN** the correct circuit refuses the witness (mutants `stack_empty_below_not_empty_const`, `stack_empty_nonzero_top`, `stack_count_one_below_nonempty`, `stack_count_wrap`), and the Quint invariant `belowEmptyAtCountLe1` holds

#### Scenario: Count 0 to 1 to 0
- **WHEN** a stack goes from `count` 0 to 1 to 0, and a stack at `count = 1` has `below = EMPTY`
- **THEN** the constraint checker accepts both leaves, the first with no hash (the count 0-1-0 positive of the kit, and positive `stack_count_one_below_empty`)

### Requirement: STK-003 A restored record is the record spilled at that depth
A pop SHALL restore exactly the record that was spilled at that depth, bound through a depth-indexed chain hash. A push SHALL spill the old top, and a rewrite SHALL NOT spill.

#### Scenario: Restored record forged
- **WHEN** a restored record, its predecessor or its depth is forged, a push has no spill, or a rewrite spills
- **THEN** the correct circuit refuses the witness (mutants `stack_restored_record_forged`, `stack_predecessor_forged`, `stack_count_unchecked`, `stack_push_without_spill`, `stack_rewrite_spills`), and the Quint invariant `cachedTopRefinesList` holds

#### Scenario: Across a leaf boundary
- **WHEN** nested push, rewrite and pop run on both stacks, also across a leaf boundary
- **THEN** the constraint checker accepts each leaf, and P6 (`chainHash_inj`, proved in C3) is cited only as a lemma on whole lists, not as the cached-top protocol

### Requirement: STK-004 Frame and task chains are domain separated
The FRAMES chain and the TASKS chain SHALL use distinct hash domains from the frozen registry. STACKS SHALL export `frame_count`, `task_count`, `frames_below`, `tasks_below` and the cached tops to CORE, BOUNDARY COMMITMENT and INIT.

#### Scenario: Domain swap
- **WHEN** a task chain value is used as a frame chain value
- **THEN** the correct circuit refuses the witness (mutant `stack_frame_task_domain_swapped`)

### Requirement: STK-005 The STACKS specification freezes only after the hash-stack Quint lane
The STACKS kit SHALL NOT be reported complete before the hash-stack part of experiment X3 has a verdict file for `hashStacks.qnt` with the compound step, re-run by the lead (RL AUDIT finding 8 adoption condition).

#### Scenario: X3 verdict missing
- **WHEN** the STACKS task is offered for a tick and no X3 hash-stack verdict file exists
- **THEN** the task stays open until `hashStacks.qnt` has a verdict file, because only the dense-memory part of X3 is done

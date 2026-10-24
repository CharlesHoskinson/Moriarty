# UIREQ-017: bind the financial observation to its actual producer

The complete original clause/scenario is frozen in ORIGINAL-UIREQ017.md. Current
UIREQ-017 remains partial/OPEN. R176 already observed its exact first-paid-prefix
numbers on the Core/2 projection component; R189 compiled its canonical locator
promotion. Reuse those scoped receipts without rerunning their unchanged test.
Neither receipt establishes the missing Sherlock financial-extension producer.

The actual gap is a call relationship, not another arithmetic interpreter.
`financial_projection::project_invocation(program, input, result)` accepts
independent caller-supplied references and explicitly does not establish that its
input produced its result. Actual `commands/simulate.rs::{run_stage_start,
run_resume}` call stage::start/resume and `print_stage_result`; they never call
that adapter. The projection is not the mandatory output of a real selected
financial producer.

## Small connected ordinary source slice

Extend the existing public `mori_core::sherlock_financial_projection` module,
using its current Core/2 profile and adapter. Do not relabel this as B financial
runtime or invent universal financial opcodes. Proposed declarations (not current
APIs):

```rust
pub struct ProducedInvocation<'p> { /* private captured input and raw result */ }
pub fn start_observed<'p>(
    program: &'p stage::StagedProgram, action: String,
    scenario: Scenario, cap: Option<UInt>,
) -> ProducedInvocation<'p>;
pub fn resume_observed<'p>(
    program: &'p stage::StagedProgram, continuation: stage::Continuation,
    event: stage::StageEvent, cli_action: Option<String>, cli_cap: Option<UInt>,
) -> ProducedInvocation<'p>;
impl ProducedInvocation<'_> {
    pub fn raw_result(&self) -> &Result<stage::StageOutcome, stage::StageRejection>;
    pub fn observation(&self) -> InvocationObservation<'_>;
}
```

Implement the two functions by retaining the exact program reference, owned
input and flags, invoking the existing stage function exactly once on an equal
clone of its consumed input, and retaining its raw returned Result unchanged.
`observation()` must build the existing borrowed InvocationInput from that stored
input and call the existing project_invocation over this same raw Result. It
must not accept a replacement result, evaluate again or infer admission from
hashes. Keep StageRejection inside the raw Result rather than adding an outer
error that loses attempted/committed counters. Program lifetime remains external;
no self-referential struct is needed. This creates a same-call source relationship,
not authentication or proof.

At the real Core/2 simulate start/resume sites, keep the existing source/strict
JSON decoding, action/cap and error policy, call these owning functions and render
the complete observation before the staged program leaves scope. Preserve every
old raw status/segment/continuation/work/residual key and STAGE_LABELS. Add the
full adapter observation/accounting/publication value to that actual financial
output, including program+plan, exact captured input, raw result and complete
ordered states/steps/effects/frames/duties. The borrowed observation types currently
lack Serialize; add an explicit exhaustive serializer/projection matching their
actual declared fields, not a summary table. Existing arbitrary-input projection
APIs remain diagnostic and are not promoted into producer authority.

The exact public choice of a Sherlock financial-extension selector/consumer is
still an explicit Root/designated-owner interface decision: current explicit
Sherlock CLI selection routes only pure Core/draft HostOnly and rejects financial
source. Reusing the existing Core/2 stage branch is an ordinary integration slice,
but that branch alone cannot be renamed universal Sherlock finance to close017.
If the extension exposes a separate selected entry, it must call these same actual
producer functions and publish their full observation. Independent review must
judge this declared producer/adapter/profile relationship against the WHOLE017
clause before adoption. No requirement for018 opcodes, B MachineImage-only
realization or native execution is introduced here.

## Exact new positive consumer

Add one external SDK-free test of the new produced-invocation path; do not rerun
`actual_core2_positive_financial_observations_match_complete_independent_r152`.
Load the frozen existing retained-payment source once. Call new start_observed,
then use its actual returned continuation for normal false release and the
false-returned continuation for true release. This executes only the NEW owning
producer/mandatory publication path. Compare its entire Program, StagePlan,
three raw results, all frames/state/ordered steps/effects/residuals/work,
three accounting reports and publication classes against this frozen artifact.
Compare full JSON values or typed values, not just leaf counts. Test same-call
program/input/raw-result references as an additional relationship check, not as
an independent semantic oracle. A source-only pass/adoption is not predicted.

Whole expected data is in expected/. First paid prefix is reserve68,
liability10, gross debit32, beneficiary receipt30 and residual pay duty10;
audit duty5 and every unaffected balance/state entry remain. Final positive
resume retains a present-zero pay liability and positive audit duty5. First
segment work7, false guard attempted2/committed7, final attempted8/committed15
remain logical work, separate from fees2/1 and no measured circuit/native cost.

Security/negative cases, authentication/consumption, hostile inputs, later local
failure/UIREQ-019, universal await lowering/UIREQ-018 and all formal/native/target
acceptance stay with their designated owners and existing requirements. This
plan changes no guard, financial policy, parser or evaluator.

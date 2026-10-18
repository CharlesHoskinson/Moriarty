# R151 ordinary financial observation source handoff

Actual source proposal: one public read-only module over existing pure Core/2
stage data, plus a separate minimal Root-owned lib.rs registration hunk. Candidate
source is unregistered/uncompiled/unrun. No W/Git/runtime/guard changes or subject,
model, SDK/native/provider execution occurred in this lane.

The module reuses the prior complete borrowed adapter and adds complete actual
StagedProgram/input/raw-result context, explicit unavailable failed tentative
publication, and read-only per-asset/account transfer/fee/gross/net accounting with
checked representability. Ordered original effects, full snapshots/steps/frames/
continuations/duties/work remain caller-owned and accessible. No new financial
execution, fee/cost policy, error code, refund semantics, cap or admission exists.

Falsifiable source predicate: every current StageOutcome/StageRejection/Execution/
StageSegment field is retained, fees contribute to each gross bucket once, and
missing publication stays unavailable. Source inspection is not tested correctness.

INTERFACE-FREEZE-01/02 seal body-free types and positive source/scenario/event
inputs before the accounting implementation. Independent oracle author R152 owns
expected outputs. No expected-output file or actual subject output was imported
or read by this author. Existing F01/F02 oracle data was not used to manufacture
new expected cells or accounting bodies.

Exact first missing carrier: StageRejection has no failed tentative Snapshot,
ordered tentative effects/Steps or replacement continuation. runtime::run_segment
returns Err(Error), discarding those locals. The adapter reports this absence and
retains captured input separately; it cannot fully close original UIREQ-019.
No local-failure trigger, rollback continuation or global ledger rollback is
invented. A designated-owner scenario/diagnostic is still required for that
actual boundary. UIREQ-017/018 source projection is implemented here; validation
and complete Sherlock financial lowering/target correspondence remain unobserved.

Root can apply the source/registration proposal after applicable source review,
then perform separately classified ordinary typecheck/positive comparisons using
independently frozen expectations. Existing execution/guard paths are untouched.
All retained U/M/MC/SP/G, original UIREQ clauses and SCON exits remain governed by
their actual acceptance predicates. No source-only full-EARS closure is claimed.

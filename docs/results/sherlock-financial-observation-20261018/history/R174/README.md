# R174 corrected financial source input — source only

The original R151 source/scenario pair was not a valid Core/2 awaiting input:
its boundary included auditDuty and pay-beneficiary, but source-wide stage
registry had no Statement::Liability declarations for either name. Root's R159
actual test returned the existing liability diagnostic before projection. The
failed source, original expectations and actual R159 record remain unchanged.
This author missed the stricter Core/2 boundary registry while relying on the
general runtime incoming-duty behavior. This is a fixture correction, not a
change to engine, admission, guard, error, financial, fee or work policy.

The corrected source adds only a separate `declareDuties` action AFTER `pay`.
Its two zero-amount Liability statements register both exact original identities:
auditDuty uses AUX / observer / fee_recipient; pay-beneficiary uses UNIT / reserve /
beneficiary. The declarations use existing source syntax demonstrated by the
existing liability_reads.rs declaration action. The fixed sequence still selects
only pay. It never invokes declareDuties; the declarations do not alter the
scenario's opening balances/liabilities or add executed pay effects.

The complete existing source prefix, including pay body and its action position,
is byte-identical. scenario.json and invocations.json remain the original frozen
inputs, referenced by hash in ORACLE-INPUT-HANDOFF.json. No production adapter,
test harness, runtime, source registry or existing sealed packet is edited.

Source-wide precondition, read-only source facts: stage/decode.rs registry() at52
scans Liability statements over EVERY action. canonical_boundary at94 (liability-member check181–211)
requires every supplied boundary liability name to have exactly the registered
AssetId/debtor/creditor. stage/admit.rs start() invokes canonical_boundary before
run_segment for an awaiting action. The source-wide registry inspects the new
unselected declarations without executing them. These locators describe existing
behavior only; this handoff supplies no security judgment or guard approval.

The new unused action changes the complete Program/StagePlan and identities.
Independent R175 must derive that complete new context and verify any unchanged
financial/work/capture/step cells before a new run. Text equality of pay does not
certify unchanged generated IDs, plan/cost/hash or outcome fields. No golden was
read/modified, and no expected output cells were produced here. The old failed
fixture is retained as negative evidence, never relabeled as a pass.

This package has only a corrected input, exact preimage, literal four-line diff,
source fact/readset pins and an input-only independent-oracle handoff. No compiler,
evaluator, test, SDK/native, model, provider or W/Git action was run. All writes
are R174-only. Static text/hash checks are not parser or execution validation.
Full original UIREQ017/018/019, Sherlock financial lowering/target and retained
U/M/MC/SP/G exits keep their actual open predicates. Root owns review, fresh
expectations, source composition and any later classified execution.

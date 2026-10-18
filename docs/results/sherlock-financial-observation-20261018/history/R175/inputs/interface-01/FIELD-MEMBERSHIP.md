# R151 declaration/input freeze before accounting implementation

Public module proposal: mori_core::sherlock_financial_projection, reusing the
prior unregistered borrowed financial_projection source, not a second executor.

Complete raw StagedProgram remains available on InvocationObservation.program;
complete original Result<StageOutcome,StageRejection> on .result. Input is
Start{action,scenario,cap} or Resume{continuation,event,cli_action,cli_cap}.
InvocationObservation also exposes .outcome (unchanged prior FinancialObservation),
.accounting and .tentative_publication. This object is borrowed diagnostic data,
not a claim the supplied result was produced from those supplied arguments.

outcome preserves exactly Waiting{advanced,segment,continuation,attempted,residual},
Complete{segment,committed,attempted,residual}, Rejected{error,attempted,committed}.
SegmentObservation carries action/frame_pre/frame_post and ExecutionObservation:
semantics/action/pre/post/steps/effects. Every Step retains index/statement/value/
snapshot/effects. Snapshot keeps state/balances/all liabilities including zero;
positive residual is separate. Continuation keeps format/program_hash/plan_hash/
action/site/captures/logical{profile,cap,committed}/boundary. No fields are inferred.

AccountingReport.effects is the exact original ordered full effect slice, including
nonmoney effects. Accounts contains groups by exact AssetId equality and account
string, in first-effect encounter order: from then to for each Transfer/Fee row.
A self-transfer updates the same group twice, preserving directional gross rows.
Transfer adds amount once to from.transfer_debit/gross_debit and once to
 to.transfer_receipts/gross_receipts. Fee adds amount once to from.fee_debit/
gross_debit and once to to.fee_receipts/gross_receipts. Fees are a subset of gross,
never an extra addition to gross. Reversed transfers/refunds add gross receipts
without cancelling earlier gross debit. State/Liability/Emit rows affect no money
bucket but remain in the full ordered effects. Asset/party names are not guessed.

Each nonnegative bucket is checked independently into UInt: Exact(value) or
AboveU128; no saturation/wrapping/refund policy/runtime error. net_flow compares
exact gross receipts vs gross debit: Inflow/Outflow absolute UInt difference,
Balanced if equal; Unavailable if either gross bucket is AboveU128, even if an
uncomputed larger-number cancellation could be representable. This is report
metadata, not runtime Outcome or admission. No fee rule/cost plane is chosen.

Accounting scope is ONLY the current successful published segment. A false await
or rejection has no published effect slice: explicit unavailable, not zero or
fabricated history. Prior cumulative effects cannot be recovered from continuation
alone. program/result/input references preserve their actual full values. Rejected
failed tentative snapshots/effects do not exist on StageRejection and remain
UnavailableFromStageRejection. Captured input continuation is not a returned
rollback/resume continuation and no admission/authenticity is inferred.

Inputs/retained-payment.mori is a new ordinary positive source chosen to instantiate
the original017 arithmetic, not a fault scenario. scenario.json freezes exact
asset/account/liability identities and unrelated state/balances/duty as INPUT.
invocations.json selects start/false/true inputs only. No outputs/work expectations
are supplied. R152 derives source work independently from source + current static
cost semantics; source work is separate from A/B fuel and target fees/DUST. Local
failure requires the existing designated-owner scenario or explicitly supplied
actual diagnostic input; do not invent a trigger or missing tentative cells.

Normative: original universal-interpreter spec119–138; revision03 design101–103;
SCON010/R02 financial membership and illustrative-vs-F01/F02 distinction; actual
stage/mod.rs declarations; runtime run_segment934–1021 drops failed local payload.

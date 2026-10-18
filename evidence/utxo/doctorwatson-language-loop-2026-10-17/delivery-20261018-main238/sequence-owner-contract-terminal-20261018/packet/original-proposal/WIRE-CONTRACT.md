# Proposed complete ordinary success-report DTO

UNADOPTED authored codec/observation contract. It is derived from inspected current stage/IR DTOs plus the proposed ordinary sequence design; it is not an independent wire oracle. All original embedded objects retain their complete existing fields. Owner-dependent error/identity/admission/resource semantics are outside this success DTO and remain explicit dependencies. These exact field sets are sufficient for a separate custodian to derive normal-path expectations after adoption; no implementation-generated expected values are permitted.

Notation: Index and Count are JSON unsigned integer array positions/counts, not hashes. UInt is the current canonical unsigned decimal-string/u128 type. Snapshot, Effect, PendingFrame, Value, Program, StagePlan and StageEvent are the existing complete DTOs at the pinned main. OutcomeReport is the entire JSON value produced by current `stage::outcome_report(Ok(outcome))`, without field projection or label changes. Optional fields specified below are mandatory keys with null when absent.

## Root and catalog

`SequenceRunReport` has exactly:

| Key | Type and meaning |
| --- | --- |
| `format` | `"moriarty-local-sequential-proposed/1"` |
| `qualification` | Existing `"local-reference-stage"` qualification retained for this reference observation |
| `composition_qualification` | `"proposed-local-sequential-composition"`; no native/operator acceptance claimed |
| `authentication`, `consumption` | Existing local labels `"none"`, `"untracked"`; no new assurance level |
| `status` | Final observation.progress.status: `ready`, `waiting`, `between-programs` or `complete` |
| `catalog` | CatalogEntry[] in program manifest order |
| `plan` | InvocationSpec[] in planned execution order |
| `opening` | The complete initial global Snapshot |
| `calls` | CallObservation[] in actual invocation order; each original real call occurs once |
| `effects` | EffectLink[] in actual call, Step and original effect order |
| `observations` | PrefixObservation[]: opening followed by one per successful requested operation |
| `final_observation` | Index of the final element in observations; never a separate summary replacing it |

CatalogEntry has exactly `{key,program,stage_plan,program_hash,plan_hash}`. `program` and `stage_plan` are complete original objects from actual `stage::load`, including unused actions/expressions and filled costs; hashes are the original existing fields copied as data, with no new commitment/identity assertion. There is no sequence hash in this proposal.

InvocationSpec has exactly `{program,action,inputs,state_bindings,duty_bindings,logical_cap}`. It contains actual typed input values rather than input file paths. program is the catalog routing key. Bindings are the declared local-to-global maps. logical_cap is the original Option<UInt>, including mandatory null when absent. No successor scenario or caller financial prefix is present.

## Each real call

CallObservation has exactly:

| Key | Type |
| --- | --- |
| `index` | Its position in root.calls |
| `invocation` | Index of the action instance in root.plan |
| `kind` | `start` or `resume`; both P start and Q start use start |
| `event` | Complete StageEvent for resume; null for start |
| `whole_pre`, `whole_post` | Complete global Snapshots immediately before/after this call |
| `projected_pre`, `projected_post` | Exact local Snapshots passed/returned by the real stage operation |
| `outside` | Complete unselected global Snapshot retained for this call; the same frame is used for lifting each original Step and post |
| `outcome` | Entire original OutcomeReport, with all original fields/frames/captures/Steps/effects/work/continuation/residuals and labels |
| `framed_steps` | FramedStep[] in the original segment.execution.steps order; [] for a successful nonadvancing Waiting observation |

FramedStep has exactly `{step_offset,source_index,whole_snapshot,effect_prefix_len}`. step_offset is the position in the original Execution.steps array; source_index is the untouched original Step.index. whole_snapshot is the complete lifted global image of that exact Step.snapshot. effect_prefix_len is the length of root.effects after that Step, including all prior calls. It is a length, not a last index or limit. Empty Step deltas leave the length unchanged. The original full Step is still in outcome; this record does not synthesize its statement, ExprId, value or local snapshot.

For original Waiting(advanced:false), projected_pre and projected_post equal the retained continuation.boundary; whole_pre equals whole_post, outside is unchanged, framed_steps is empty and no effect is appended. The original outcome has segment:null and its original attempted/committed continuation data. This is an ordinary successful observation, not a new error branch.

EffectLink has exactly `{index,call_index,step_offset,step_effect_offset,execution_effect_offset,whole_effect}`. Each index names a real original record. The original per-Step effect at step_effect_offset must equal the original Execution.effects element at execution_effect_offset; whole_effect is its explicit global name image. State.name and Liability.name map through the invocation bindings. AssetId/from/to/amount and Emit name/value retain their original values. The complete original local effects stay embedded in outcome. Effects are never sorted, deduplicated, netted or canceled.

## Every published prefix

PrefixObservation has exactly `{index,cause,call_prefix_len,effect_prefix_len,head,progress,frames,latest,financial,logical,residual}`.

| Member | Exact meaning |
| --- | --- |
| `cause` | Opening is `{kind:"opening"}`; later observations are `{kind:"call",call_index:Index}` |
| `call_prefix_len` | Complete prefix of root.calls covered by this observation; opening is0 |
| `effect_prefix_len` | Complete prefix of root.effects covered by this observation; opening is0 |
| `head` | Entire global Snapshot at this observation |
| `progress` | Exactly `{status,completed_invocations,active_invocation,next_invocation}`; optional indices are null |
| `frames` | Exactly `{invocation,frame}` rows for every started invocation, plan order; complete actions retain original idle frame, pending action its actual frame; no invented frame for an unstarted action |
| `latest` | Exactly `{invocation,call_index}` rows for every started invocation in plan order; the index names its last original call, not a cumulative synthesized segment |
| `financial` | Complete FinancialSummary below, derived from opening and this exact effect prefix with attached original statements |
| `logical` | Complete LogicalSummary below |
| `residual` | Every positive global head liability in complete existing Liability representation, global name order; positive outside duties included |

Tables are run-local. Expanding a prefix means taking all calls/effects at positions below the corresponding length. No suffix is needed to invent fields in a call; all call bodies are already complete. Original member/local work, latest envelopes, original continuations, global state and complete history are distinct observations. Prefix equality/extension must preserve every earlier complete object. No table is shared between test cases. A public regression compares the entire root, complete expanded prefixes and bytes, not merely these lengths.

Progress rules for a successful trace: ready has completed0, active null, next0. Waiting has exactly its actual pending invocation active and only completed earlier invocations. Between-programs has active null and next equal to the next not-yet-started invocation. Complete has all invocations completed and active/next null. These are local program-control states, not external settlement/finality statuses.

## Complete financial fields

FinancialSummary has exactly `{balances,liabilities,asset_totals}`.

BalanceAccounting rows, sorted by exact `(AssetId,account)`, have exactly `{asset,account,opening,closing,transfer_debits,transfer_credits,fees_paid,fees_received,gross_debits,gross_credits,net_change}`. The set includes every opening/closing global balance row, including untouched rows with zero volumes. All amount fields are UInt. net_change is exactly `{direction,amount}`, where direction is `increase`, `decrease` or `unchanged`, and amount is the nonnegative magnitude of closing−opening; unchanged uses0. Gross debits = transfer_debits + fees_paid; gross credits = transfer_credits + fees_received. Each original Transfer/Fee contributes its full amount even for a zero or same-account effect. A later refund contributes a credit, never a negative earlier debit. These descriptive counters do not grant authority or certify an optimization.

LiabilityAccounting rows, sorted by global name, have exactly `{name,asset,debtor,creditor,opening,additions,discharges,closing,effect_indices}`. opening/closing are each exactly `{present,amount}`, with amount:null for absence and amount:UInt for presence, including present zero. additions sums the actual source Liability amounts; discharges sums actual source Discharge amounts. effect_indices names every original liability effect for this global duty in root.effects prefix order. The EffectLink's attached original Statement distinguishes addition from discharge; before/after amounts alone are insufficient. Rows include untouched outside duties and newly created or zero-valued duties; absent in both opening and closing with no effect yields no invented duty row. This adds no interest/day-count convention, mint/burn, refund right or duty consent rule.

AssetTotals rows, sorted by complete AssetId, have exactly `{asset,transfer_volume,fee_volume,gross_debit_volume,gross_credit_volume}`. They sum only the corresponding observed amounts in that exact asset; whole asset identities remain present even where an untouched opening asset has zero volumes. There is no cross-asset grand money total, implicit conversion, custody claim or native-fee field.

## Complete logical fields

LogicalSummary has exactly `{profile,resource_basis,allocation,per_invocation,attempted_total,committed_total,available_ordinary,remaining_closure_reserve}`.

profile is the existing `/2` logical-cost profile string; resource_basis is `"reference-logical-accounting-only"`. allocation is exactly `{ordinary,closure_reserve}` with the declared UInt values unchanged. per_invocation contains exactly `{invocation,cap,attempted,committed}` for every started action instance, in plan order. cap is its unchanged Option<UInt>. attempted sums actual attempted fields for that instance's calls; committed is its latest original committed prefix (from its continuation while pending, or original Complete.committed). attempted_total sums all original call attempted values; committed_total sums these latest per-instance committed values once each. No unstarted instance is assigned fictitious work.

On the sufficiently funded ordinary successful trace, available_ordinary = allocation.ordinary − attempted_total; remaining_closure_reserve equals allocation.closure_reserve. These success equations preserve a visible reserve instead of treating it as ordinary spendable work. They do not define owner-dependent cap preflight, rejected-call accounting, reserve consumption/authority, adapter cost, numeric overflow, native fees or target adequacy. Those rules must be fixed by the owner before production. Existing local caps/committed/attempted fields never change to make a sequence fit.

## Rendering and independent expected bytes

Arrays above retain their specified order. Object key order does not determine execution order. The proposed public rendering uses the existing pretty JSON string/escaping convention and one final LF. The production serializer must retain the existing document/individual-export limits; the new report's finite admissible size and diagnostic path require the owner contract. This document provides no size waiver, streaming-protocol invention, truncation, partial success output or output-memory adequacy claim.

Complete semantics and byte compatibility require separate independent expectations after adoption. The original disjoint goldens are not sequential expectations and cannot be relabeled. Embedded OutcomeReport/Program/Plan bytes inherit their exact existing contracts, with their original independent-wire qualifications retained. New outer-field ordering/prefix representation is an authored proposed codec and cannot certify itself by producer round-trip.

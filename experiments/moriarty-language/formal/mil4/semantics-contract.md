# MIL/4 semantic contract candidate

**Status:** revised Sprint 0 working candidate, 2026-09-29. The initial nine-seat review rejected a freeze. No language adoption, K proof, Quint simulation, native proof or ledger acceptance follows from this document. The [completion plan](../../../../docs/superpowers/plans/2026-09-29-mil4-k-quint-completion.md) defines the intended exit.

## Version boundary

Propose **Source/6 → Core/5** for the MIL/4 first-profile language. Keep `moriarty-financial-agreement-source/5`, `moriarty-financial-expression-contract/4`, `moriarty-financial-lifecycle/1` and `moriarty-financial-lifecycle-state/1` byte and API behavior unchanged. Source/5 binds the closed Core/4 expression constructor set to lifecycle/1, and its compiler invokes the Core/4 evaluator. Generic `Record` or `Emit` values cannot stand for a new authenticated financial transition. A legacy record enters MIL/4 only through a field-by-field injection plus a named authentication premise. Otherwise return a named version rejection. Do not infer a `/4` signed encoding from the Source/6 or Core/5 labels.

S0 still has proposed `/3` intent bytes. W-D1 and W-D2 must select the signature scheme, verifier locus, digest and canonical encoding before any `/4` migration is stated. Unknown tags must reject. Migration must bind the source record, selected Core program, signed digest and all effect obligations to one new record. Require Source/5 to reject Source/6 headers and Core/4 to reject Core/5 tags with named codes before fallback parsing. Preserve Source/5 check, format, elaborate and evaluate behavior on their existing contract. These are compatibility obligations, not results. No compatibility proof is inherited from Source/5 or Core/4.

## State and constructor classes

Core/5 must represent these classes as typed constructors or typed records with closed tags and total validation:

| Class | Required content |
| --- | --- |
| Header and selection | Version/profile, executing domain, selected program and policy digest, current authenticated head, predecessor, validity, stage and episode IDs. |
| Typed values | Nominal asset/domain IDs, checked atomic quantities, deltas, oriented prices, shares, positions, instants and durations. |
| Intent and completion | One signed template, fixed fields, typed holes, permitted source and finite cell range, narrowing fill, selected branch, gross/fee/net bounds, recipients, surplus and residue. |
| Formula | Closed Φ₀ guard and outcome terms with distinct pre/post access; general Φ₁ and uncertified Ω reject. A closed certified Ω needs a versioned numeric and cost contract. |
| Authenticated state | Balance, supply, obligation, encumbrance, lock, pool, managed assets, shares, observation, grant, policy, receipt, claim, replay, episode head and tombstone cells, plus profile-specific aggregates. |
| Effects and duties | Complete ordered typed debit/credit, mint/burn, lock/unlock, debt, claim/receipt, budget and authority consumption, work and retained duty lines. Product remainders are metadata until a whole-unit ledger effect is derived. |
| Evidence and authority | Evidence policy, selected observation, provenance premise, signer/key/digest/verifier locus, grant scope/epoch/budget and replay identity. |
| Transition and result | Prepared stage, pending/unknown/recovery/terminal episode phase, accepted success or signed accepted failure, complete observation, or atomic rejection with judgment and code. |

The executing program derives the full read/write footprint after holes resolve. It reads each current cell from one authenticated head. It computes every post-cell and effect line. A supplied effect hash alone does not authenticate omitted endpoints, aggregates, recipients, reserves or duties.

## Stage relation and observation

Use one selected program and one signed intent per initial stage. Establish the six judgments in this candidate order, pending W-D3's final rejection precedence: `stage`, `intent`, `effect`, `authority`, `history`, `failure`. `stage` checks selection and typed authenticated pre-state. `intent` checks exact signed scope, validity and filling. `effect` checks the complete canonical prepared vector, conservation and post-state. `authority` checks the signer-to-debited-owner link, grant, allowance and budget. `history` checks current head, predecessor, replay and successor. `failure` checks the signed terminal branch and retained duties. S0 admits only terminal success or atomic local rejection with an empty retained-effect and duty set. Define stable codes and within-judgment precedence before implementation.

An accepted result contains the version/profile, signed scope digest, selected program, authenticated pre-head, prepared ordered effect and consumption vectors, all derived reads/writes, post-head and post-state, phase, remaining duty and work, consumed authority and replay, and judgment result. A local rejection contains the first failing judgment, stable code and diagnostic work, with no published post-state or effects. A ledger phase failure can retain only effects and duties named in the signed failure branch. Snapshot-to-head authentication, native qualification, signature verification and ledger atomicity remain explicit external premises until their interfaces are pinned.

## S0 candidate domain

W-D0 proposes one signer, one executing domain, one settlement asset, a literal-fee transfer, and funded AccrualFirst repayment of an existing obligation. Transfer requires `v>0`, `f≥0`, `f≤feeCap`, `v+f≤grossDebitCap` and the signed recipient's net floor. It debits the signed owner by gross `v+f`, credits `v` to the fixed recipient and `f` to the fixed fee recipient. It consumes that owner's allowance remaining and spent by gross `v+f`. The zero-fee line, absent receiver-cell and endpoint alias policies remain explicit W-D3 leaves.

Repay reads an authenticated obligation with debtor, creditor, settlement asset, principal `p`, accrued `a`, outstanding `p+a` and status. It requires `0<n≤p+a` and identity conversion `(mantissa=1, scale=0, rounding=none)`. The same stage debits the signed payer `n`, credits the obligation's bound creditor `n`, reduces accrued by `min(n,a)`, reduces principal by the remainder, writes `outstanding'=p'+a'` and settles status only at zero. It consumes gross allowance and a signed intent replay ID. A different legitimate partial repayment uses a different signed ID. This is a new Core/5 stage; Core/4 Repay consumes a separate Transfer in one prepare step and is not an equivalent implementation.

Both actions advance an authenticated head and preserve every cell outside the derived footprint. S0 has no division, rounding, reserve posting, mint, foreign evidence or accepted fee-bearing failure.

Use checked UInt128 atomic balances and counters. Existing lifecycle state fields are UInt128; current source-to-kernel nominal admission bounds selected nominal amounts and caps to `2^127−1`. Fix the first S0 comparison domain to that signed nominal bound while keeping the distinct state-field width. The exact source/Core/K comparison domain and near-bound cases remain W-D4 obligations. S0 cannot be called admitted until W-D0–W-D4 have selected bytes, signature binding, ordered rejection, exact effect lines and widths.

## First-profile and deferred boundary

The eight first profiles are `amm-cp/1`, `loan-fixed/1`, `cdp/1`, `opt-capped/1`, `obs/1`, `gov/1`, `bridge-pair/1` and `vault/1`. Each needs its own typed cells, authenticated transition, complete effects, positive trace and otherwise-well-formed hostile trace. The current [K surge](../../../../deliverables/mil4-k-surge-2026-09-29/RESULT.md) supplies arithmetic and effect sketches only. Its `m4AdmitFamily` rejects every first family with `FAMILY_STAGE_ADAPTER_ABSENT`. Preserve that fail-closed boundary until the corresponding full stage adapter exists.

Defer LP mint/burn, variable credit, unbacked/rebasing issuance, uncapped options and general derivatives, median/TWAP, general voting and multisigner admission, bonded fast fill and foreign verifier assurance, complex vault withdrawals/slashing, general Φ₁ and uncertified Ω. Give each an explicit versioned rejection. A broad family label never admits its later profile.

## Historical K disposition

Archive old bounded `moriarty.k` behavior, original fixtures, toolchain locks and SP03 receipts as historical domain evidence. Adapt the expression and lifecycle modules, codecs, runner and surge static/formula/escrow rules only after a documented Source/6/Core/5 mapping. Rewrite the surge's current judgment order and supplied `m4cNativeQualified` authority shortcut before any semantic reuse. Replace the old composition roots and version-specific state signatures. Treat family arithmetic/effect projections as sketches. The old runner's proof entry returns `PROOF_UNIMPLEMENTED`; no old theorem is inherited. Record each file-level disposition before Sprint 0 exit. Historical expression/lifecycle fixtures remain evidence on their original contract, not MIL/4 coverage.

## Open decision gate

The [decision register](decisions.md) owns W-D0–W-D6 and M4-C1–M4-C5. W-D0–W-D4 and the S0 part of M4-C1 gate S0; later rows gate their named profiles. The [constructor inventory](coverage.tsv) is provisional, and the [projection](projection.md) is a design map. Sprint 0 cannot exit until a Source/6 grammar exists, each first-profile production and Core constructor has a version/profile and a rule or named rejection, S0 leaves and historical files have exact dispositions, and two substantive independent votes agree on frozen bytes. Sprint 0 freezes independent expected discriminators; executable comparisons are Sprint 1 evidence. A changed signature or transition opens new proof obligations.

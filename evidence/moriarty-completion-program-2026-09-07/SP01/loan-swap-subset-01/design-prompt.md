You are the user-selected Grok4.6 high implementation author. Write the existing atomic loan/swap semantic design subset for Moriarty SP01.6 now. Parent has read the requirements and sources, created the isolated worktree and recorded bounded admission. The task is documentation/evidence for existing specified behavior, not a successor semantic decision. No permission cycle. Fresh GPT6 independently checks your artifacts.

WORKTREE: /home/charl/Moriarty/.worktrees/sp01-loan-swap-grok
Own ONLY these three files, plus FOREMAN_REPORT.md/.json at worktree root:
- evidence/moriarty-completion-program-2026-09-07/report-reconciliation/semantic-challenges.json (create; whole gate pending-review with only subsets.RP01-MC02 pending-review, RP01-MC03 and full RP01 specified-only)
- evidence/moriarty-completion-program-2026-09-07/SP01/loan-swap-subset-01/design.md
- evidence/moriarty-completion-program-2026-09-07/SP01/loan-swap-subset-01/traces.json
No code/spec/test/register/wiki edits, shell, Git writes, agents/providers, installs, web, wallets, native proving or public actions. Use Read/Edit/Write tools. Parent does verification; do not claim you ran commands. Finish within1190sec.

SCOPE: loan first-period accrue→settle and swap exact-input minimum-receive→close only. These are synthetic bookkeeping and initial restricted Compact examples, NOT live custody, ACTUS-wide conformity, general AMM behavior, exact-output semantics, complete PCD or ledger acceptance. Existing source embeds sample-specific expected amounts/reserves; name that limitation. Do not change source semantics to solve any gap.

INPUTS (read narrowly; hashes supplied below):
1. spec/examples/loan.mori and swap.mori under experiments/moriarty-language: complete authoritative source.
2. spec/semantics.md and spec/typed-schemas.md: only relevant state/effect/authority/bounds/settlement sections.
3. tests/semantics.test.mjs first~160lines and B2 controls near214–260, plus tests/lowering.test.mjs. Tests are observation evidence, not definitions.
4. retained MC01/profile-04/evaluator/legacy-reference.json: independent older implementation capture. Preserve recorded source digests and renaming provenance; do not claim its absent old source paths are runnable.
5. openspec/REPORT-RECONCILIATION-2026-09-07.md RP01 subset/gate contracts and SP01.6 task.

REQUIRED RECORD CONTRACT:
semantic-challenges.json must have schemaVersion, gateId=RP01, scope, status=pending-review, ownerPackages, inputs as [{path,sha256}], outputs, findings, closureTasks, reviews=[], subsets. Each subset names scope,rowIds,inputs,outputs,candidateHash (null until parent freezes),reviews=[],status. RP01-MC02 has four row IDs loan-accrue, loan-settle, swap-exact-input, swap-close; also negative trace IDs. RP01-MC03 specified-only and full RP01 incomplete. Put output digest null with explicit parent-freeze-pending status, NEVER invent hashes. Main record must not pretend all required challenge/operator/theorem rows exist. Full RP01, signing successor, native statement and corpus gaps retain closure tasks SP01.2/.3/.7 and owning sprints. Use exact source hashes supplied below. No success/passed/executed claims from this writing phase.

traces.json must store explicit independent expected states, ordered writes, ordered complete effects (all operand identities, ordinal, nominal unit/amount, and settlement asset/quantum/ledgerAmount where applicable), obligation creation/settlement and retained tombstones, revision/remaining/status/remaining-notional, and authority consequences for all four rows. Include initial state for each example. Do NOT derive expected numeric answers from running current evaluator. Distinguish envelope hash/identity fields (bound symbolically to exact inputs) from concrete financial values; include an observation map for every field rather than dropping digest/provenance fields. Static trace derivations are specified expectations pending parent verification, not executed results.

Independent arithmetic/expected anchors:
Loan 5,000,000,000 USD_micro ×8×31/(100×365) = floor33,972,602, remainder27,000/36,500=54/73 micro-units. Principal installment500,000,000. After accrue: notional4,500,000,000 principal_due500,000,000 interest_due33,972,602 principal_paid0 interest_paid0 borrower_cash20,000,000,000 lender_cash0 cursor1 episode_closed0; revision1 remaining1; episodeOpen agreementOutstanding remainingNotional4.5b; two Outstanding dues PR500m then IP33,972,602. After settle533,972,602: borrower_cash19,466,027,398 lender_cash533,972,602 principal_paid500m interest_paid33,972,602 dues0 cursor2 closed1; notional4.5b; revision2 remaining0; Closed episode STILL Outstanding agreement; two Settled tombstones retained. Ordered effects: Transfer borrower→lender USD_TEST_ASSET533,972,602, then DueSettled PR500m, IP33,972,602. Closing episode must not erase remaining notional.
Swap: initial reserve_a1m reserve_b2m trader_a100k trader_b0 provider_a0 provider_b0 epoch_closed0. Input10k AssetA_quantum. effective_input9,970,000; numerator19,940,000,000,000; denominator1,009,970,000; flooroutput19,743; remainder numerator−19,743×denominator. Input transfer trader→pool10k ASSET_A, output pool→trader19,743 ASSET_B. After swap: reserves1,010,000/1,980,257 trader90k/19,743 providers0/0 closed0; revision1 remaining7. Minimum receive predicate min_out <=19,743 accepts0/1/19,743, rejects19,744; overdelivery relative to min_out allowed; NOT exact-output equality. Fee factor997/1000 affects pricing; no explicit Fee effect and full10k input stays in pool. Do not label fee difference a separately transferred fee. Close: provider receives1,010,000 ASSET_A then1,980,257 ASSET_B; reserves0/0, trader unchanged, providers receive exact reserves, closed1 revision2 remaining6; no remaining notional, empty obligations. Last allowance reserved for close. Both quantum1 nominal units/ledgerunit.

REQUIRED NEGATIVES (specified expected diagnostic and unchanged input/zero accepted effects; cite existing tests or source; unexecuted variants labeled): wrong actor/recipient/assets; swap min_out19,744; settle-beforeaccrue; wrong due amount; no reserve for swap; stale/replayed state/nonce; exact-plan extra/reordered effects; gross debit cap below actual protected debit incl Fee and refunds not netted; minimum net credit subtracts every debit; missing proof backend/client booleans cannot create accepted Complete. Include direct missing/replayed/erased due mutation and why obligations cannot vanish. Keep runtime RESOURCE_BOUNDS defensive/unreachable in fixed admitted straight-line profile distinct from PROGRAM_BOUNDS source rejection.

AUTHORITY/CUSTODY: explicitly bind program/profile/genesis/domain/instance/beforestate/predecessors/observations, action/args, nonce/validity/principal and required claim root. ExactPlan binds every ordered write/effect. IntentRefinement checks principal-scoped gross debit and net credits/recipients, but does NOT introduce nominal-debt caps for DueCreated; fixed source/action authorization is the existing scope and general debt authority remains successor work. Pool outgoing effects are part of all-effects evidence yet principal debit caps alone do not prove pool/provider custody authorization. Spell that external custody/consent relation as an unresolved MC02/MC04/MC05 obligation before public spending. Trusted simulation checks do not prove signatures/currentness/oracle truth. Four mandatory judgments remain named, not discharged.

Each row identifies bounded read/write footprint, full observation map, roles, denominations/rounding, environment assumptions, independent positive trace, invalid mutation, source scope/gap, implemented/specified/unsupported disposition, closure owner and task. Covers lifetime/horizon and continuation ownership: loan residual notional persists beyond the closed two-step episode, future servicing not implemented; swap close is provider-only, cancellation/races/private workflow/general fees and liquidity operations unsupported. Do not claim whole RP01 closes.

REPORT: concise scope, owned files, absent tests/commands by worker, unresolved gaps, pending independent review.

PINNED INPUT HASHES:
{
  "experiments/moriarty-language/spec/examples/loan.mori": "1e1e61158ef80d44aa326399731440971fe50de7147ae5fb04e3fb36c48fef49",
  "experiments/moriarty-language/spec/examples/swap.mori": "0c2217365f2e518ec70835cf05a09150253d2df334e7dcf0504fd8c8d887051e",
  "experiments/moriarty-language/spec/bounds.json": "b548641a1a9d74bab68ba699ffb1e2350fa0889d61b8704e98216f9d4a6c3664",
  "experiments/moriarty-language/spec/semantics.md": "d2c62ea1e34a345753b319532d85652d29b1946b2d77d1f60f9518b01c519002",
  "experiments/moriarty-language/spec/typed-schemas.md": "6ef3383ad42b7ea2a22822f8116c7e182cbe8b476ef01e9cd3ef899679da4f04",
  "experiments/moriarty-language/tests/semantics.test.mjs": "3eac34c2bdb2fd818991d39dfe815ba5ab8884e84bb9210f2ae5d7b5eacadcef",
  "experiments/moriarty-language/tests/lowering.test.mjs": "0057938310ca2630cbf7475d9d03479cd8b96816f7dbedb013ea9ff66789ecdd",
  "evidence/moriarty-completion-program-2026-09-07/MC01/profile-04/evaluator/legacy-reference.json": "7d4ed0761603978c331a7b70910dc53721cea2e0b0a1764cf9da4f93a13cb032",
  "openspec/REPORT-RECONCILIATION-2026-09-07.md": "a1a1661fc0d554d8e469651fdbd42c41e7c9e2115c8550f369742dcee94bd5da",
  "openspec/sprints/sp01-financial-contract-and-execution-admission.md": "fdb586d9bf05f2fbf7e9c2294be49854528db20e575faf9aaa1144b033908172"
}

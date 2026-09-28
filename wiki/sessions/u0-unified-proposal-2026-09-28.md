---
id: moriarty.session.u0-unified-proposal-2026-09-28
type: session
title: U0 unified proposal — nine-reviewer study
status: active
created: 2026-09-28
updated: 2026-09-28
updated_at: 2026-09-28T17:40:00Z
tags:
  - moriarty
  - language-design
  - u0
---

# U0 unified proposal — nine-reviewer study

**Specified-only.** Everything below is proposal knowledge. Under the repository
rule, work is accepted by evidence, not by review, and nine-reviewer agreement
does not close a U0 predicate or establish a capability. No task here has been
executed, and the study directory was still uncommitted when this page was
written (`?? deliverables/u0-study-2026-09-28/`, Moriarty at
`8f73784042bd692733c296d0d49f5173be96725e`).

**Inputs.** Nine independent read-only reviews, three lenses × three model
families, plus the merged proposal:
[UNIFIED-PROPOSAL.md](../../deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md).
L1 semantics: `opus55-L1.md`, `gpt6sol-L1.md`, `grok47-L1.md`. L2 target and
enforcement: `opus55-L2.md`, `gpt6sol-L2.md`, `grok47-L2.md`. L3 numeric profile
and exit path: `opus55-L3.md`, `gpt6sol-L3.md`, `grok47-L3.md`. Briefs and run
logs are in `briefs/` and `logs/` beside them. The reviews study the
2026-09-23 packet at
[`deliverables/u0-semantic-contract-2026-09-23/`](../../deliverables/u0-semantic-contract-2026-09-23/).

No new `SRC-####` identifier was minted: the study consumed repository artifacts
already inside the vault and captured no new external source bytes. The one
external fact used — `midnightntwrk/midnight-zkir` PR #17 — was read from its
GitHub page on 2026-09-28 and is recorded below as an unreproduced claim.

## Key findings

Counts are how many of the nine reviewers reached the point independently.

| Id | Finding | Reviewers |
| --- | --- | --- |
| F1 | The U0 gate should split: "contract frozen" is a different claim from "capability demonstrated". Requiring 84/84 natively enforced leaves or 24 demonstrated backend rows pulls U2–U4 work into U0. | 9/9 |
| F2 | The Aeon `repay` counterexample is a **contract gap, not an evaluator defect**. The written source guards do not carry the safety argument; the Core evaluator and the K semantics do. | 9/9 (L2 on the enforcement side only) |
| F3 | "66 absent embeddings" measures the wrong thing. It counts 12 evidence-envelope leaves and about 12 toolchain/ledger leaves as missing language, and source/Core "embedding" is currently name matching, not computation. | L1 3/3, L3 2/3 |
| F4 | The stage-relation schema admits financially invalid instances: negative amounts, one-account gross lines, liabilities without components, opaque authority strings, and `unchecked` judgments inside an accepted stage. The aeon −1 state is schema-valid. | L1 3/3 |
| F5 | The six judgments are prose. None states conservation, a cap inequality, non-negativity, liability roll-forward or replay freshness. | L1 3/3, L3 3/3 |
| F6 | K "0 of 23 covered" is structural: 17 rows are roadmap-wide UNI requirements owned by U3–U7, and `covered` has no objective definition. The 2026-09-17 K-versus-TypeScript lifecycle run is the strongest semantic evidence available. | L1 3/3 |
| F7 | The target pins are not one tuple. They merge the historical released path with native-recursion development objects, chosen by tie-break rather than joint execution, with no IR major/minor, no per-circuit key manifest, and an unresolved Blake2b-versus-Poseidon transcript split. | L2 3/3 |
| F8 | The enforcement map is honest but unprioritised: all 84 leaves read `NOT_ENFORCED`, with no locus, tier or owner, and only an absence finding where the design requires a signed-intent authentication decision. | L2 3/3 |
| F9 | Premises and backend rows carry no blocking data: no premise says what it blocks, MNR01–MNR08 have `owner: null`, and ZR01 is labelled U0/U1 while no U0 task exercises it. | L2 3/3 |
| F10 | Numeric rule D2 ("dust accrues to the protocol reserve") is under-specified. `prorata-principal-share` conserves value (`dA = n − dP`) and is misclassified as a reserve gap; ceil accrual adds to a liability; sub-quantum remainders cannot be posted to an integer reserve; unit dimensions are implemented but recorded only as widths. | L3 3/3 |
| F11 | The first slice is unnamed. `ROADMAP.md:38` says "connect the smallest useful signed financial stage" without naming it, so U1's "every primitive needed by the initial slice" has no denominator. | L3 3/3, L1 3/3 |

## Design decisions and why

These are the merged positions of the unified proposal. Each is a recommendation
awaiting owner sign-off unless marked as an owner correction already applied.

- **Split the gate into three independently reported states.** `U0-F` contract
  frozen (relation, judgments, numeric profile, dispositions, premises versioned
  and hash-bound) unblocks U1/U2 design; `U0-S` slice demonstrated at reference
  level unblocks U1 host certification; `U0-T` target qualified unblocks U1
  native certificates and U2 native work. "U0 done" means U0-F and U0-S both
  hold and U0-T either holds or is recorded as blocked on a named upstream
  dependency. *Reason:* F1 — the single gate conflated a contract milestone with
  native enforcement, making U0 unfinishable before U2–U4.
- **Sequencing rule that resolves the one disagreement.** U1 **host**
  certificates may start after U0-F plus U0-S; U1 **native** certificates may not
  start before U0-T. *Reason:* feasibility measured against an unpinned target is
  not evidence, but host work does not depend on the pin.
- **Name the first slice S0.** One signer, one domain, one settlement asset, one
  admitted obligation. Program A is a funded **repay** using AccrualFirst and
  exact conversion (mantissa 1, scale 0, `rounding: none`). Program B is a plain
  **transfer** with a **literal** fee line — the structurally contrasting program
  U2 needs. Accrual, ProRata, price, division, reserve posting, partial phases
  and non-ledger-induced history are excluded. *Reason:* F11, and S0 is the
  intersection that every reviewer's constraints admit; the literal fee exercises
  `feeCap` without opening any rounding gap, which settles the fees disagreement
  (Grok L1 wanted empty fees, Opus/Sol L1 wanted a fee).
- **S0 uses no open numeric primitive.** Checked add/subtract on u128, checked
  amount construction, comparison, and the AccrualFirst `min`/subtract. U1's
  first certified primitive is `checked-add-u128`. Limits are integer
  comparisons; when U3 needs the 11 A / 1 A / 20 B ratios they are written as
  cross-multiplied UInt256 inequalities, never divisions, so S0 is a strict
  prefix of the U3 discriminator. *Reason:* F10 — a slice containing division
  would drag every open rounding and reserve gap into U1.
- **Lift the lifecycle laws into the contract, not into a refinement-type
  package.** L1–L7 (`outstanding = principal + accrued`; component
  non-negativity; `0 < nominal ≤ outstanding`; settlement ≤ balance and ≤
  allowance; width fits; exact conversion; the AccrualFirst split) plus E1–E2
  conservation, I1–I5 intent limits, A1–A2 authority and replay, S1/H1 hashes and
  predecessor commitment, and F1 rejection behaviour become executable judgment
  clauses. *Reason:* F2/F5 — the safety argument exists only as kernel rejection
  codes, so a signed intention does not bind it and the enforcement map cannot
  attribute it to a layer.
- **Replace name-matched "embedding" with an executed projection.** A total
  `stage-relation.ts` calls `prepareFinancialLifecycle` rather than
  reimplementing it, and "present" is redefined as "emitted by the projection
  under a cited law". *Reason:* F3 — citation counting can be raised without
  demonstrating anything.
- **Numeric remainder classes replace a single reserve rule.** `none`;
  `conserved-split` (ProRata — no posting allowed); `charged-increment` (ceil on
  a liability — the extra unit is the debt); `sub-unit-residual` (not value, not
  posted, not rounded again); `protocol-reserve` only for a whole atomic unit
  that would otherwise leave the conservation equation. *Reason:* F10 — D2 as
  written has no carrier for a fraction of a quantum, and posting ProRata's
  truncated unit would break `dP + dA = n`.
- **Three separate target profiles.** Profile **V3** is the target
  (`midnight-zkir` `zkir-v3` at `0303e6d` or later, the PR #17 Agda mechanization
  as its formal reference). Profile **H** (compactc 0.31.1, proof-server 8.1.0,
  `ledger-v8@8.1.0`, ZKIR v2) is historical evidence only. Profile **IVC**
  (`midnight-zk` `695351f1`, k=17 SRS) is `blocked`. *Reason:* F7, plus the owner
  correction below.
- **Owner correction, already applied.** ZKIRv3 is the target; Opus L2's option
  of amending U2 to "the released ZKIR major" (which is v2) is withdrawn. There
  is no downgrade; U0-T is `blocked` until upstream v3 integration lands.
- **Make a verified tuple representable at all.** The current checkers make
  success impossible (`const: false` on the tuple flag, a banned-word list, a
  hard-coded "NOT established" line, and G6 requiring the flag to stay false).
  Per-profile states `unverified` / `blocked` / `reverified`, with `reverified`
  legal only against a matching receipt. *Reason:* a real tuple could not turn
  the gate green without changing the checkers.
- **Signed-intent authentication is in-circuit (proposed).** The canonical intent
  digest becomes a public input of the stage circuit, authenticated in-circuit;
  ZKIR v3's curve surface makes this expressible. Grok L2's fallback — a circuit
  commitment to the intent bytes with a ledger-checked signature over that
  commitment — is admissible only if a negative control shows the binding holds.
  Unshielded spend signatures do not bind intent fields. *Reason:* F8 and the
  design's requirement that U0 *decide* the locus rather than record an absence.
- **Enforcement gets a locus and a tier.** Loci: circuit, ledger-intrinsic,
  contract-state, host-advisory, trust-premise. The identity cohort
  (`circuitIdentity.*`, `profiles.numericProfile`) is demonstrable in U0-T; a
  U2-slice cohort of about 22 leaves follows; everything else is owned by U3, U4
  or U6; the 12 `judgments.*` leaves are `not-a-constraint`. A new status
  `LEDGER_INTRINSIC_DEMONSTRATED` is legal only with a receipt, and only when the
  value the ledger checked is shown to be the Moriarty field.
- **`host-only` is tightly bounded.** Allowed only for lifecycle-law leaves where
  the projection's hostile fixtures show the cited kernel code checks that exact
  projected field; never for `signedIntent.*`, because the host checks a
  different object.
- **Aeon package: re-scope, do not adopt in U0.** Take laws L1–L7, the classified
  z3 transcripts and the evaluator replay; keep the advisory obligation analyser
  for U0–U2; defer the `/6` grammar, SMT discharge, linear/affine obligations and
  refinement-directed lowering until after U2; reject the `ROADMAP-DELTA.md`
  §4–§5 sprint-file edits (those IDs are provenance aliases now); correct the
  README wording from "drives principal and borrower balance to −1" to "the
  written source guards omit load-bearing preconditions that the kernel
  enforces".

### Things the study says not to do

Do not grow the name-matching embedding table or add fields to source/5 to raise
counts. Do not treat a checker exit of 0, a cited identifier or a matching
fixture as coverage or enforcement. Do not set a tuple to `reverified` from
hashes found in documents. Do not pair zkir-v3 commits with the ledger-8 proof
server, or call the v2 path ZKIRv3. Do not declare a reserve type before the
remainder-class decision — a declaration alone would falsely close the
`prorata-principal-share` gap, though not "every rounding gap" as the study says
(corrected in CLM-0969). Do not edit numeric decision v1, `spec/examples/loan.mori` (its
bytes are bound to Preview evidence), `financial-lifecycle.ts`, or the
2026-09-23 receipts. Do not submit to Preview, raise k above 17, or rerun R3
synthesis.

## Supporting evidence

Anchors as cited by the reviewers. Line numbers are as recorded on 2026-09-28 and
were not independently re-verified when this page was written.

| Claim | Anchor |
| --- | --- |
| Kernel rejects repay above outstanding | `experiments/moriarty-language/src/successor/financial-lifecycle.ts:1568` (`EXCEEDS_OUTSTANDING`) |
| Kernel rejects insufficient balance | `financial-lifecycle.ts:1459` (`INSUFFICIENT_BALANCE`) |
| Kernel rejects component over-allocation | `financial-lifecycle.ts:1426`, `:1433` (`ALLOCATION_COMPONENT`) |
| K semantics carries the same repay guards | `formal/k/lifecycle-kernel.k:52` |
| Written source guards that omit the law | `experiments/moriarty-language/spec/successor/examples/loan-lifecycle.mori:119-121` |
| Component sum invariant in the evaluator | `financial-lifecycle.ts:854-859` (`parseObligation`) |
| Two-sided transfers already in Core | `financial-lifecycle.ts:85-92` (`TransferAction.from`/`.to`) |
| Exact conversion branch with no profile row | `financial-lifecycle.ts:1378-1382` |
| AccrualFirst split | `financial-lifecycle.ts:1405-1407` |
| One `Conversion.rounding` field required to be both floor and ceil | `financial-lifecycle.ts:1376` (`convertNominal`), used by `applyOriginate` `:1669` and `applyRepay` `:1585` |
| ProRata conserves value | `financial-lifecycle.ts:1423-1424` (`dP = product / total`, `dA = n − dP`) |
| Ceil accrual adds a unit to the liability | `financial-lifecycle.ts:1779-1787` |
| Schema permits negative amounts | `openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json` amount pattern `^-?[0-9]+$` (e.g. `:158-160`, `:297-299`) |
| Authority is opaque strings | same schema `:461-468` |
| Judgment status may be `unchecked` in an accepted stage | same schema `:573-578` |
| Admitted price scale is 0..18, checker's 77 is a UInt256 bound | `experiments/moriarty-language/src/successor/financial-expression-types-v1.ts:94-95`; `scripts/check_u0_numeric_profile.py:101-104` |
| Numeric close rule can be satisfied by declaring a reserve | `scripts/check_u0_exit_gate.py:228` |
| Gate requires the tuple flag to stay false (G6) | `scripts/check_u0_exit_gate.py:193-194` |
| Tuple statuses limited to historical/absent; banned prose words; hard-coded "NOT established" | `scripts/check_u0_target_pins.py:40`, `:62-64`, `:147-149`, `:243-244` |
| Only host-side intent predicate checks | `experiments/moriarty-language/src/successor/evaluate.ts:290-295` |
| Strongest executed semantic evidence | `deliverables/k-lifecycle-execution-2026-09-17/RESULT.md` (104 lifecycle cases, K matches TypeScript Core) |
| K `lcRepay` uses unbounded `Int` | `formal/k/lifecycle-kernel.k:50-54` |
| Unnamed first slice | `ROADMAP.md:38` |
| Signed-intent locus must be decided in U0 | `docs/MORIARTY-CONSOLIDATED-DESIGN.md:44` |

## Claim records

Shared fields for CLM-0961 … CLM-0972: source is the internal repository
artifacts at commit `8f73784042bd692733c296d0d49f5173be96725e`, observed
2026-09-28; authority descriptive; scope "U0 unified proposal (specified-only)";
lifecycle S2 unless stated.

| Claim | Statement | Evidence kind | Reproduction | Confidence |
| --- | --- | --- | --- | --- |
| CLM-0961 | Nine independent reviewers agree the U0 gate should report contract freeze separately from demonstrated capability. | recommendation | not applicable | high |
| CLM-0962 | The Aeon repay counterexample is a source-contract omission; the TypeScript evaluator and the K semantics reject the case at the anchors above. | repository observation | not reproduced (no reviewer executed the evaluator against the witness) | medium |
| CLM-0963 | The 2026-09-23 stage-relation schema validates financially invalid instances, including the aeon −1 state. | repository observation | partially reproduced (read from the schema; no negative corpus was run) | high |
| CLM-0964 | The six judgments are prose definitions with no stated conservation, cap, non-negativity, roll-forward or replay condition. | repository observation | reproduced (read from `judgments.json`) | high |
| CLM-0965 | K reconciliation's 0-of-23 score is structural: 17 rows are roadmap-wide UNI requirements owned by U3–U7 and `covered` has no objective definition. | repository observation | reproduced | high |
| CLM-0966 | The 11 target pins are not one tuple; they merge a historical released path with native-recursion development objects, and record no IR major/minor. | repository observation | reproduced | high |
| CLM-0967 | The U0 checkers make a verified tuple unrepresentable, so a real receipt could not turn the gate green without changing them. | repository observation | reproduced during synthesis at the cited lines | medium |
| CLM-0968 | `prorata-principal-share` conserves value and is misclassified as a reserve gap; on that reading there are five real numeric gaps, not six. | inference from repository observation | reproduced (the `dA = n − dP` line) | medium |
| CLM-0969 | **Refuted 2026-09-28.** The study's §10 claim that declaring a protocol-reserve type would close all six numeric gaps is wrong. `conformance_reasons` (`scripts/check_u0_numeric_profile.py:1529-1547`) accumulates reasons, and a reserve declaration removes only "protocol-reserve posting is absent". Five of the six open rows carry `authorSelectable: true`, so "author can select the rounding" survives; only `prorata-principal-share` has `authorSelectable: false` with its fixed direction (floor) equal to its required direction, so the reserve is its sole reason. A declaration would close **one** gap, not six. | repository observation | reproduced (checker source plus the six `open-gap` rows of `numeric-profile.json`) | high |
| CLM-0970 | Slice S0 as specified uses no primitive with an open numeric conformance gap. | inference | not applicable | medium |
| CLM-0971 | ZKIRv3 is the target. Its formal specification merged into `midnightntwrk/midnight-zkir` `zkir-v3` through PR #17 on 2026-09-23 as commit `0303e6d`, head `ebb662c`, adding an Agda mechanization of 13 types and 34 instructions with P5 faithfulness and statement-soundness theorems. | source fact (owner correction) | not reproduced (GitHub page read 2026-09-28; no local capture, no end-to-end v3 toolchain check) | medium |
| CLM-0972 | Whether any compiler, ledger and proof server emit and accept ZKIR v3 end to end was not checked by this study, so U0-T is blocked on a named upstream dependency. | open question | not applicable | high |

## Owner decision after the study

**2026-09-28: the intent language is the place for conditional settlement with
programmable escrow.** Conditions governing funding, release, refund and partial
progress are signed as part of the canonical intention. The decision, its reasons
and the obligations it puts on S6, the predicate language, the intent judgment,
the observation surface and in-circuit authentication are recorded in
[[wiki/moriarty-architecture|the architecture page]] as CLM-0978. It bears
directly on this study because S6 sits in the U0 freeze phase: the signed-intent
abstract syntax must admit a condition grammar even though S0 uses none.

## Corrections found after the study

Established on 2026-09-28 by the nine-reviewer DeFi-coverage study that followed
this one ([report](../../deliverables/u0-study-2026-09-28/DEFI-COVERAGE-REPORT.md)),
and re-verified directly against the cited files.

| Claim | Statement | Evidence kind | Reproduction | Confidence |
| --- | --- | --- | --- | --- |
| CLM-0973 | Law E1 as written in the unified proposal — "per-asset conservation: Σ gross = Σ supply changes = 0" (`UNIFIED-PROPOSAL.md:90`) — is wrong outside a slice that mints nothing. Its source states the qualified form, "Σ gross = Σ supplyChanges (0 in MSS)" (`opus55-L1.md:155`), and the DeFi kernel's law is Σ effect = supply. Hash-binding the unqualified form as an S3 clause would make every later issuance program violate an accepted law. | repository observation | reproduced (both lines read directly) | high |
| CLM-0974 | The TypeScript evaluator is runnable on this host, so the study's ENOSPC blocker for task N4 was environmental and no longer applies. `/tmp` has 32 GB free; Node v22.22.1 refuses the `.ts` imports (`ERR_UNKNOWN_FILE_EXTENSION`, and `ERR_NO_TYPESCRIPT` under `--experimental-strip-types`), but `bun` 1.4.2 runs the suite: `tests/successor-source-repayment.test.mjs` passes 57 of 57, and the whole suite is 903 pass / 8 fail / 1 error across 46 files. The 8 failures sit in `tests/lowering.test.mjs` (needs `@midnight-ntwrk/compact-runtime` resolved through a `.worktrees/r3-native` path that does not exist), `tests/legacy-funded-result-types.test.mjs`, and a README grammar-drift test. No lifecycle or repayment test failed. | experiment observation | reproduced (commands run 2026-09-28) | high |
| CLM-0975 | S0's binding of `observations[]` to an empty set freezes a regression rather than making a neutral scoping choice: the older atomic profile carries observation declarations, genesis provider bindings, evidence digests and an authenticity gate (`src/evaluate.ts:84,128,202-203`), while source/5 hard-codes `observations: {}` (`src/successor/financial-agreement-source-compiler.ts:606`) and lists `observation` under `unsupportedDeclarations` (`spec/successor/syntax-profile.json`). | repository observation | reproduced (both files read directly) | high |
| CLM-0976 | No brief given to the nine U0 reviewers mentions oracles or observations; a grep for "oracle" and "observation" over `deliverables/u0-study-2026-09-28/briefs/` returns no match. The U0 study's silence on the observation surface is a scoping artefact of its briefs, not a finding. | repository observation | reproduced (grep run 2026-09-28) | high |
| CLM-0977 | Moriarty's own pre-existing coverage register, `experiments/moriarty-language/spec/target-crosswalk.json`, holds 104 rows (32 ACTUS, 72 DeFi) of which 102 are `unsupported-row-conformance` and 2 are `partial-pilot` — a LAM loan pilot and a constant-product swap pilot. Every row's `evidence_status` is specified-only, with no frontend, conformance run, proof or ledger acceptance. | repository observation | reproduced (parsed 2026-09-28) | high |

## Open questions

Owner decisions that block downstream work (Phase A of the plan):

- **Remainder classes and the liability domain.** Adopt the five remainder
  classes and the rule that opening + accrual − discharge = closing with
  non-negative components?
- **Fee-cap treatment.** Does a charged-minus-credited rounding spread count
  against `feeCap` and `grossDebitCap`? Opus L3 recommends yes. It must be
  decided before U3, where "at most 1 A fee" could become tight.
- **Signed-intent boundary (TP07).** In-circuit authentication as primary, with
  the commitment-plus-ledger-signature fallback only on a passing negative
  control?
- **Approve S0** as the named first slice, with programs A and B as described.
- **Run the T3 harness on profile H now?** It would validate the negative-control
  machinery and close the "proved, not verified" gap (CLM-0958), labelled harness
  validation with no target claim.
- **Aeon package.** Commit it under re-scoped text, or leave it uncommitted?

Single-reviewer findings that need verification before anyone acts on them:

- The only Moriarty prove used k=14 and was never verified; the k=17 SRS comes
  from an unapproved IVC preparation, and the pinned `accrue` key never produced
  a transaction proof (Grok L2).
- `Conversion.rounding` is required to be both floor (origination) and ceil
  (repayment) at the same call site (Grok L3).
- K `lcRepay` uses unbounded `Int`, so correspondence needs a stated UInt128
  bound and a near-bound case (Grok L1).
- Sub-quantum remainders cannot be posted at all (Opus L3).
- The enforcement search roots exclude the ledger itself, so ledger-intrinsic
  checks were never considered — a candidate locus only (Opus L2).

Evidence limits of the study itself:

- **No reviewer ran the callable evaluator against the repay counterexample.**
  One attempt failed with ENOSPC on a full host `/tmp`. The rejection is read
  from code and test sources; task N4 exists to demonstrate it.
- Two reviewers independently re-ran the Aeon z3 probes and reproduced the
  recorded results for the written source guards.
- Every reviewer re-ran the U0 checkers, which exited 0 with the EXIT-GATE
  counts. One reviewer saw 7 `test_u0_exit_gate.py` failures, all ENOSPC while
  copying `.git` into `/tmp`, so that suite's status is **unknown** for the run.
- The guarded `develop` CLI `status` was not run during the study. It was run
  when this page was written: capability `SP01.6 loan-swap-subset`, next action
  `sp01-loan-report`, with `operational-history` and three stale/missing inputs
  listed as missing evidence.

## Next steps

The plan writes to a new versioned directory, `deliverables/u0-semantic-contract-v2/`;
the 2026-09-23 files stay unchanged because their hashes are bound.

1. **Phase A — owner decisions (can start now):** the six decisions listed above.
2. **Phase B — contract freeze, mostly parallel:** leaf dispositions (S1),
   stage-relation schema v2 (S2), executable judgments v2 (S3), signed-intent
   abstract syntax (S6), numeric profile v2 (N2), target record v2 (T1),
   representable tuple states (T2), enforcement locus and tiers (T5), premise
   blocking graph and backend ownership (T7), gate rewrite (G1) → **U0-F**.
3. **Phase C — reference demonstration:** Core-to-relation projection (S4), then
   the TypeScript/K differential on S0 (S5); executed numeric vectors (N3) and
   the repay precondition lift and evaluator replay (N4) in parallel → **U0-S**.
   U1 host certification of the S0 primitives starts here.
4. **Phase D — target:** tuple receipt on V3 with ZR01 negative controls (T3) and
   the proof/ledger seam and IVC record (T4), when upstream v3 integration can be
   installed → **U0-T**, then U1 native certificates and U2 native work.

Critical path: S2 → S3 → S4 → S5. The decisions that most often block downstream
work are the numeric one (N1) and the intent boundary (T6). Keep one delivery
item in progress at a time, as the develop workflow requires.

## Links

[[wiki/index|Research index]] · [[wiki/open-questions|Open questions]] ·
[[wiki/contradictions|Contradictions]] ·
[[wiki/sessions/aeon-integration-2026-09-19|Aeon integration session]] ·
[consolidated design](../research/consolidated-design/index.md) ·
[[wiki/moriarty-architecture|Architecture]] ·
[[wiki/formal-assurance|Formal assurance]] ·
[roadmap](../../ROADMAP.md) ·
[product contract](../../docs/MORIARTY-PRODUCT-CONTRACT.md) ·
[backend requirements](../../docs/MORIARTY-BACKEND-REQUIREMENTS.md)

## Architecture axis — 2026-09-28

A second pass re-scored the same nine categories on whether the **roadmap and
design say how each will be built**, rather than whether it is built. The
[architecture report](../../deliverables/u0-study-2026-09-28/ARCHITECTURE-COVERAGE-REPORT.md)
holds the detail; the knowledge worth keeping:

| Claim | Statement | Evidence kind | Reproduction | Confidence |
| --- | --- | --- | --- | --- |
| CLM-0979 | Moriarty has a real architecture, and it covers more than its specification does. Scoring 29 kernel abstractions by whether the design names a responsible component: 16 designated, 4 partial, 8 none, 1 excluded by design — against 3 expressed and 12 absent on the build axis. The three-column responsibility boundary (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:23-36`) is the strongest artifact in either repository. | repository observation | reproduced (both documents read in full) | high |
| CLM-0980 | The architecture has no notion of a **region**. `:72` requires disjoint parallel composition to have "checked independence" and the core's eight capacities (`:15`) contain no footprint, frame or region. A per-constructor `frame` field exists in `experiments/moriarty-language/spec/successor/expression-signatures.json` with four values, 37 of 40 reading `unchanged`; it classifies which machine component a constructor mutates, not which ledger cells it touches, so it can never decide independence. This costs independence, interference reasoning and framing at once, and it cannot be retrofitted. | repository observation | reproduced (file parsed 2026-09-28) | high |
| CLM-0981 | The core's constructor inventory **exists** — 40 base constructors in `expression-signatures.json` plus 8 financial at `spec/successor/financial-expression-source.md:67-90`. What does not exist is the artifact joining the nine library families to those constructors. The "scope matrix" promised at `:93` has two repo-wide occurrences, both the aspirational sentence. The one worked precedent is `deliverables/sp02-financial-pure-expression-2026-09-10/inputs/financial-pure-extensions.md:44-50`, never generalised. The filed obligation at `docs/ROADMAP-RECONCILIATION-2026-09-19.md:145` is owned by "P0/P2", which `ROADMAP.md:5` says is not a work queue. | repository observation | reproduced (parsed and grepped 2026-09-28) | high |
| CLM-0982 | The authority model is closed and too narrow, found independently from four categories. `:54` admits linear consumed receipts and affine spending permission; `:68` enumerates six rights over one's own workflow. There is no issuance right, though `docs/MORIARTY-PRODUCT-CONTRACT.md:43` requires "authorized mint/burn supply changes"; no enforcement or seizure right, so liquidation is inconsistent with `:54`'s consent rule; and no carrier for right-kind at all. A right-kind tag cannot be retrofitted once U0 freezes the relation and U2 enforces it. | repository observation | reproduced | high |
| CLM-0983 | The canonical stage statement structurally excludes three things, which no added field fixes: everything below stage granularity (intra-stage trace, per-step receipt, located refusal), everything above stage arity (it ends "continuations **or** terminal outcome", so a join's two branch outcomes and an interleaving schedule have no home), and residue — an effect the relation cannot record makes the instance invalid rather than residual. | inference from repository observation | reproduced (`:42` read; schema `additionalProperties: false` confirmed) | medium |
| CLM-0984 | `ROADMAP.md` assigns demonstrations and evidence, never designs. Every U row's closing column is artifacts and receipts, and no row says "produce the architecture for X". Consequently the region notion, the family→constructor map, authority right-kinds, the intent condition grammar, the observation value, domain-qualified effect lines, what a full AMM requires, issuance placement and staking are all unassigned. | repository observation | reproduced | high |
| CLM-0985 | Placement is mostly correct where it exists: derivatives, oracles and bridges are placed coherently. Issuance and governance are placed nowhere; order books, auctions and RFQ are placed nowhere; staking is placed nowhere, and `stak*`, `slash*`, `reward` and `emission` occur zero times across both controlling documents. The failure is rarely placement — it is the mechanisms placement presupposes. | repository observation | reproduced (greps run 2026-09-28) | high |

Two corrections to earlier synthesis in this session: the constructor inventory
does exist (CLM-0981 supersedes the claim that no document names the core's
constructors), and the four composition modes are architecturally designated —
`:15` names "defined composition operators" and `:32` assigns mode-distinction to
the Moriarty column, so the missing piece is the independence premise, not the
operator.

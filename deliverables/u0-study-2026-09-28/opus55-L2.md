# U0 study — Lens L2 (Target and enforcement) — Claude Opus 5.5 reviewer

## 1. Goal restatement

For U1 and U2 to start, U0 has to fix two things. The first is **which exact, installable Midnight toolchain the work targets**. The second is **where each accepted-statement field is supposed to be enforced**, with a named owner and milestone for every field that is not enforced yet. U1 cannot measure "native feasibility" (ROADMAP.md:22) against a moving or imagined target. U2 cannot claim "actual pinned ZKIRv3, native verification" (ROADMAP.md:23) unless a pinned tuple exists that actually emits and verifies ZKIRv3. U0 does not need to *enforce* the 84 leaves. It does need to:
- produce one **reverified tuple receipt** on released interfaces, with fail-closed negative controls (the ZR01 acceptance, backend-requirement-matrix.json `rows[ZR01].acceptance`);
- make a **per-field locus decision** (circuit, ledger primitive, contract state, host-advisory or trust premise) for every leaf, including the signed-intent boundary;
- **classify each premise by the milestone it blocks**.

## 2. Findings

### 2.1 The only demonstrated Midnight path emits ZKIR v2, not v3. The U0 artifacts do not record this.
- `experiments/moriarty-midnight-network/hello-world/contracts/managed/hello-world/zkir/storeMessage.zkir` begins `"version": { "major": 2, "minor": 0 }`, and the `.bzkir` header is `midnight:ir-source[v2]`. Its `compiler/contract-info.json` says `"compiler-version": "0.31.1"`.
- The 2026-09-11 measurements record the loan build keys (compactc 0.31.1, the same build family pinned in target-pins.json `pins[proving-keys]`) as `midnight:prover-key[v7](ir-source[v2])`, `midnight:verifier-key[v6]`. They also record the runtime as `ledger-v8 8.1.0, zkir-v2 2.1.1` (`wiki/attachments/historical-evidence/evidence/pcd-midnight-native-2026-09-11/MEASUREMENTS.md.txt:535-543`).
- `wiki/contradictions.md:90` already names "mixed ZKIR 2/3 artifacts" and asks for an explicit IR major/minor.
- target-pins.json nevertheless pins `zkir` to a **zkir-v3 git checkout** (`pins[zkir].pin = 7dff84a…`, lines 120-208). It sits next to a compactc 0.31.1 compiler, a ledger-8 proof server (`pins[proof-server]`, 8.1.0) and v2-format keys. The recorded absence search omits `wiki/` (`absenceSearch.roots`, lines 6-11), and no field records IR major/minor.
- docs/MORIARTY-BACKEND-REQUIREMENTS.md:3 says "Current execution targets ZKIRv3". On the evidence in the repo, that holds only for unreleased or source-checkout paths. **I have not verified** whether any *released* compactc (the evidence packet cites compactc-v0.34.0 release notes; target-pins.json:112-114) emits v3 with a matching released ledger and proof server. That is the first thing U0 must settle.

### 2.2 target-pins.json merges two different targets into one "tuple"
- **Released deployable path (ZKIR v2 / ledger-8):** compactc 0.31.1 plus the release zip digest (lines 61-118), proof-server 8.1.0 image digest (631-658), `@midnight-ntwrk/ledger-v8` 8.1.0 (listed only as a *conflict* at lines 604-608), and the loan and swap Compact keys. This path has *demonstrated historical* evidence: the Preview loan finalized four stages on 2026-09-17 (deliverables/preview-loan-2026-09-17/recovery-run01/RESULT.md).
- **Native recursion development path:** midnight-zk `695351f…` (native proof library and "verifier" rows, 209-272), zkir-v3 branch commits, ledger git commits `3fa0d1d`/`a8ab82b`/ledger-9 alternatives, and the R3 k=17 SRS. Nothing in the repo shows these composing into a proof that a released ledger accepts. `unresolved[0]` (line 708) records the Blake2b vs Poseidon transcript mismatch.

Rows are chosen by tie-break heuristics ("first in receipt order", lines 284 and 386; "chose the clean tracked tree", line 131) and not by compatibility. Pinning *one* accrue key digest as "the" proving key is a category error. A tuple pins **key format versions** (`prover-key[v7]`, `verifier-key[v6]`) plus a per-circuit digest manifest. The "verifier" row names the native IVC verifier, but the verifier that actually accepted Preview transactions is the ledger-8 node's, and no pin names it. The SRS degree comes from an R3 experiment rather than from the degree the deployed circuits use (line 713 admits "The degree used by the ledger and the proof server is unknown"). The Moriarty compiler pin (`f702692…`, line 23) is a 2026-09-03 advisory head. It is not the generator commit that produced the current `experiments/moriarty-language/compact` kernels.

**What is sound:** the claim/conflict/unresolved evidence discipline, `compatibleTupleEstablished: false`, and the checker's honesty line.
**What is weak:** there is no profile separation, no IR-version field, and no bytes-level observation.
**What is mis-scoped:** it answers "what hashes appear in our documents" when the question is "what installs and verifies together".

### 2.3 Enforcement map: honest but undifferentiated
- 84/84 `NOT_ENFORCED`, `mechanisms: []` for every row (enforcement-map.json `rows[*]`). `check_u0_enforcement_map.py` confirms: `OK: 84 leaf fields, 0 enforced, 0 host-only, 84 NOT_ENFORCED`.
- The `nativeRoots` (4 experiment directories) are Moriarty code only. The map never considers **ledger-intrinsic enforcement**, meaning properties that Midnight's ledger enforces for any contract call:
  - verification against the deployed contract's per-entry-point VK (candidate for `circuitIdentity.verifierKeyId`, `circuitIdentity.circuitId`, `programIdentity.entryPoint`);
  - network-id binding (`domain.chainId`);
  - unshielded/Zswap balance and nullifier checks (`effects.*`, `authority.replayState`).

  These are *candidate* native boundaries and are not demonstrated. But the map's note "No circuit constraint, ledger primitive … binds" (rows 3-6, 9-11) reflects a search limited to Moriarty roots, not a finding about the ledger.
- The map has no priority. All 84 leaves look equally urgent, so it cannot drive U1/U2 scope.
- The signed-intent locus (TP07; docs/MORIARTY-CONSOLIDATED-DESIGN.md:44 "must also state whether signed-intent authentication occurs in the circuit, a bound ledger primitive or another explicitly justified native boundary") is answered only negatively (rows 72-83). The design requires a *decision*, not just an absence finding. Row 82 correctly notes that unshielded-input signatures do not bind intent fields.
- The only intent-predicate checks are host-side (`evaluate.ts:290-295`, rows 76, 78, 79). The aeon SMT finding (repay driving principal to −1) shows host and source guards are insufficient. It should become a native negative control: Compact `Uint` subtraction underflow may reject it natively. That is testable and not assumed.

### 2.4 Trust premises: nine recorded, none tagged with what they block
trust-premises.json has 6 open (TP01, TP02, TP06, TP07, TP08, TP09; `kind` unresolved-interface or planning-assumption). No premise has `blocks`, `owner`, `closeCondition` or `reviewBy`. By my reading:
- **TP01 (native target) + TP09 (tuple): block U1 and U2.**
- **TP07 (intent auth boundary): blocks U2**, and U1's "signature feasibility" measurement, which needs the chosen mechanism.
- **TP06, TP08: block U4 only.**
- **TP02 (March 2027 recursion): a planning assumption; it should block nothing before U4.**

### 2.5 Backend requirement matrix: 24 rows, 8 without an owner
All 24 rows are `specified-only`, which is fine. But MNR01–MNR08 have `owner: None, milestone: None` (backend-requirement-matrix.json rows 17-24). ZR01's milestone reads "U0/U1" (`rows[ZR01].milestone`), yet U0 has no task that exercises ZR01's acceptance test. The matrix is generated from docs by `build_u0_backend_matrix.py`, so the owner gap is in the source doc.

### 2.6 Checker and test state (run 2026-09-28)
All seven `check_u0_*.py` exit 0 with the counts EXIT-GATE.md reports. The tests (`test_u0_target_pins`, `enforcement_map`, `trust_and_backend_matrix`, `exit_gate`): 146 passed, 7 failed. All 7 failures in `test_u0_exit_gate.py` are `ENOSPC` while copying `.git` into a full `/tmp`. That is an environment failure, not a code verdict, so exit-gate test status is **unknown** for this run. `/tmp` was still full when I finished and needs freeing before these tests are re-run.

## 3. Proposed work

**T1 — Split the target into two named profiles and add IR/format fields.** (S)
- Output: `deliverables/u0-target-2026-10/target-profiles.json` (schema `moriarty-u0-target-pins/2`).
  - Profile **R** ("released-deployable"): compact CLI, compactc and its zip digest, emitted ZKIR major/minor, key format headers (`prover-key[vN]`, `verifier-key[vN]`, `ir-source[vN]`), proof-server image digest, ledger package and node version, network id, SRS degree and digest actually used, Moriarty generator commit.
  - Profile **N** ("native-recursion-dev"): midnight-zk, zkir-v3, PR738 or successor, ledger-9 candidate.
  - Each profile has its own `compatibleTupleEstablished`.
- Checker: extend `scripts/check_u0_target_pins.py`. It rejects a profile without IR major/minor observed from artifact bytes, rejects a single-digest key pin in place of a per-circuit manifest, and adds `wiki` to the absence roots.
- Exit: the checker fails on the current file and passes on the split file.
- Depends on: nothing.

**T2 — Decide the ZKIR target.** (S to write, user decision)
- Output: `docs/decisions/u0-target-ir-decision.md`. From released upstream artifacts, record whether a released compactc/ledger/proof-server combination emits and verifies ZKIRv3.
  - (a) If one does, profile R moves to it.
  - (b) If none does, the user chooses between two options. Option one: keep U2's "actual pinned ZKIRv3" and accept that U2 waits for a v3 release. Option two: amend U2 to "the released ZKIR major named in profile R", with v3 moved to U4 or profile N. No silent substitution in either case.
- Checker: `check_u0_target_pins.py` requires `targetIr.decisionRef` to resolve.
- Exit: the decision is signed off by the user.
- Depends on: T1. **This blocks U2's exit wording.**

**T3 — Reverified tuple receipt for profile R (local only).** (M)
- Output: `deliverables/u0-target-2026-10/tuple-receipt.json`, `scripts/check_u0_tuple_receipt.py`, `tests/test_u0_tuple_receipt.py`.
- Procedure:
  - Install from release artifacts with digests into a clean prefix.
  - Compile hello-world plus the current generated loan kernel.
  - Read headers from the output bytes.
  - Prove with the pinned proof-server digest.
  - Verify the proof in a separately installed ledger-package verifier process.
  - Optionally run the contract call on a pinned `midnight-local-dev` devnet (origins.json commit) for ledger acceptance.
- **Negative controls (ZR01):**
  - rewrite the IR major header;
  - substitute another circuit's VK;
  - substitute a different-k SRS;
  - tamper one public input.

  Each must produce an explicit rejection. Every positive control must first succeed, so that no rejection is vacuous.
- No Preview submission. Existing resource and admission rules apply.
- Exit: `compatibleTupleEstablished: true` for profile R only, backed by a receipt hash. Profile N stays false.
- Depends on: T1 and the T2 fact-finding.

**T4 — Locus assignment and tiering for all 84 leaves.** (M)
- Output: an `enforcement-map.json` v2 with added fields per row:
  - `requiredLocus` ∈ {circuit, ledger-intrinsic, contract-state, host-advisory, trust-premise};
  - `tier` ∈ {U2-slice, U3, U4, U5};
  - `ownerMilestone`;
  - `nativeCandidate` (with the upstream code citation, for ledger-intrinsic).
- Status stays `NOT_ENFORCED` until a negative control passes. Add status `LEDGER_INTRINSIC_DEMONSTRATED`, legal only when it cites a tuple-receipt negative control.
- **Tier U2-slice (about 22 leaves):**
  - `signedIntent.{signer,intentId,validity,replayPolicy,grossDebitCap,feeCap,minNetOutcome,recipients[],assetIdentities[]}`;
  - `effects.{gross,net,fees}[].{account,amount,asset}`;
  - `domain.chainId`, `programIdentity.{programId,entryPoint}`, `circuitIdentity.verifierKeyId`, `authority.replayState`, `profiles.{semanticProfile,numericProfile}`, `schemaVersion`.
- All other rows (liabilities, observations, predecessors, disclosures, failurePolicy, resources, continuations) go to U3/U4 with owners.
- Checker: `check_u0_enforcement_map.py` requires every row to have locus, tier and owner, and rejects any enforced status without a receipt reference.
- Exit: 84/84 rows have a locus and a tier.
- Depends on: T1.

**T5 — Signed-intent boundary decision (TP07).** (S decision, M evidence)
- Output: `docs/decisions/u0-intent-auth-boundary.md`. My proposed answer is **circuit**, with two realizations:
  - (i) proof of knowledge of the owner secret against a committed public key, over a statement containing the canonical intent digest. This covers the self-proving signer.
  - (ii) in-circuit verification of the signer's signature over the intent digest, needed when a solver proves on the signer's behalf.
- Ledger unshielded-input signatures are rejected as the locus because they do not bind intent fields (enforcement-map row 82).
- Whether the profile-R standard library exposes the curve and hash operations (ii) needs is **unverified**. T3's install is where that gets checked. U1 owns the cost measurement ("measured signature … feasibility", ROADMAP.md:22).
- Exit: the decision is recorded, TP07 moves from "unresolved" to "decided, feasibility owned by U1", and U0 includes a positive and a wrong-key negative control for (i) in the T3 receipt.
- Depends on: T3.

**T6 — Premise blocking graph.** (S)
- Output: trust-premises.json v2 adding `blocks[]`, `owner`, `closeCondition` and `reviewBy`.
- Checker: `check_u0_trust_backend_matrix.py` rejects an open premise without them.
- Exit: every open premise names blocked milestones. A premise that blocks U1 or U2 must be closed or decided before those milestones start.
- Depends on: T2, T5.

**T7 — Backend matrix ownership.** (S)
- Output: an edit to docs/MORIARTY-BACKEND-REQUIREMENTS.md, then regenerate the matrix. MNR01–08 get owners and milestones (proposed: MNR01/03/05/06 → U1 or profile-N tracking; MNR02/04/07/08 → U4). Add a `u0Demonstrates` flag, true only for ZR01 scoped to profile R.
- Checker: the builder `--check`, plus reject null owners.
- Depends on: none.

**T8 — Recursion tripwire.** (S)
- Output: TP02 gets `reviewBy: 2026-12-15` and a tripwire. If no public-testnet release carrying profile-N recursion (ledger, ZKIR, verifier) exists by then, re-plan U4 and keep U2 on the ledger-induced history profile ROADMAP.md:34 already permits.
- Exit: the gate records TP02 as non-blocking for U0–U3.

## 4. Exit redefinition

Yes, split the gate:
- **U0-C "contract frozen":** the current recorded artifacts with schema hashes, plus T4 locus/tier, T6 blocking graph and T7 owners. It is checked mechanically and makes no capability claims.
- **U0-T "released target demonstrated":** the T3 tuple receipt for profile R with every negative control rejecting, the T2 decision recorded, and the T5 (i) wrong-key control rejecting. This is the minimum *demonstrated* in U0.

**Deferred with owners:**
- in-circuit signature cost and ZR06 adversarial soundness → U1;
- native enforcement of the ~22 U2-slice leaves → U2;
- liabilities, failure policy and partial progress → U3;
- profile N tuple, ZR02–ZR05/08/09/14 and TP06/TP08 → U4.

U1 may start after U0-C plus U0-T. It must not start on U0-C alone, because feasibility measured against an unpinned target is not evidence.

## 5. Risks and things NOT to do

- Do **not** set `compatibleTupleEstablished: true` by picking consistent-looking hashes from documents. Only a receipt with negative controls counts (FOOTGUNS evidence discipline).
- Do **not** pair zkir-v3 source commits with a ledger-8 proof server, or call the Compact v2 path "ZKIRv3".
- Do **not** mark ledger-intrinsic checks as enforcement for a Moriarty field unless a negative control shows rejection *and* the field's value is shown to be the one the ledger checked. For example, VK-by-address binds the circuit only if the deployed contract address is itself bound to `programIdentity`.
- Do **not** let the March 2027 assumption gate U0–U2, and do not build profile-N integration on unmerged PR branches as if it were a release.
- Do **not** submit to Preview during U0. A local devnet is enough for ZR01, and prior resource debits stand.
- Risk: T2(b) may force a visible scope change to U2. That is correct; hiding it is the failure mode.
- Risk: the released standard library may lack efficient signature primitives. Then T5(ii) becomes a U1 blocker, and delegated solving waits.

## 6. Disagreements

1. **ROADMAP.md:23 and BACKEND-REQUIREMENTS.md:3 ("current execution targets ZKIRv3")** conflict with the repo's own artifacts (ZKIR v2 bytes, v2-sourced keys). The claim must be reverified or amended (T2).
2. **target-pins.json treats a tuple as one row per component.** It needs one tuple per profile, with format versions and a per-circuit manifest. The "first in receipt order" key selection has no meaning.
3. **The enforcement map's native roots exclude the ledger itself.** That understates candidate native enforcement and gives U2 no priority order.
4. **EXIT-GATE.md lists 6 open premises without saying which gate what.** As written, U1 could "start" while TP01/TP09 remain undecided.
5. **ZR01 is labelled a U0/U1 milestone, but no U0 task exercises its acceptance test.** T3 closes that gap for profile R.
6. **The "trust premises" list mixes decisions with trust assumptions.** TP07 is a design decision the consolidated design requires U0 to make, not a premise to carry.

## 7. Top three recommendations

1. **Establish a reverified profile-R tuple receipt with ZR01 negative controls (T1+T3)**, and state in bytes which ZKIR major it uses. Without this, U1 and U2 have no target.
2. **Force the ZKIR v2/v3 decision now (T2).** The released evidence shows v2, the contract says v3, and U2's exit depends on which is true.
3. **Assign a locus, tier and owner to all 84 leaves, and decide the signed-intent boundary as "circuit" (T4+T5).** That turns 0/84 into a bounded U2-slice of about 22 fields, with U1 owning signature cost.

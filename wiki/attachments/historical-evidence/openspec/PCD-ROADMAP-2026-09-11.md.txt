# Midnight-native PCD roadmap

**Status.** Adopted into the OpenSpec roadmap by the [PCD integration amendment](PCD-INTEGRATION-2026-09-11.md) and the [PCD change package](changes/pcd-ledger-anchored-acceptance/README.md). Specified-only.

- Date: 2026-09-11 UTC.
- **Scope of effect.**
  - The amendment re-roots atomic F3 on the Stage 0 seam, repurposes SP04 and SP06 for certificates and removes MC03 from the MC04 and MC05 dependencies.
  - It changes no resource authority or model routing. Certificate campaigns need a reviewed resource amendment.
  - It does not alter the active eight-hour AFK work selection.
  - Six design defaults await the user's confirmation, and the Charter's second reviewer has not reviewed the adoption.
- **Hard Preview gate.** Unchanged. Verification-enabled mandatory acceptance on Midnight Preview stays required.

**Sources.**

| Source | Location |
|---|---|
| Decision report | [Moriarty PCD on Midnight](../deliverables/pcd-midnight-native-2026-09-11/REPORT.md) |
| Rendered page | [moriarty-pcd-on-midnight.html](../deliverables/pcd-midnight-native-2026-09-11/moriarty-pcd-on-midnight.html) |
| Reproduced measurements | [evidence](../evidence/pcd-midnight-native-2026-09-11/README.md) |
| Primary captures | [receipts](../raw/sources/pcd-midnight-native-2026-09-11/receipts.jsonl) |
| Vault decision | [wiki decision page](../wiki/decisions/pcd-midnight-native-architecture.md) |

## 1. Decision this roadmap implements

Build a **ledger-anchored certified state machine with bounded native certificates**. Do not build general DAG proof-carrying data, and do not make each transaction recursively verify its predecessor's proof.

1. **One fused step relation per entry point.** Each Moriarty action compiles to one Midnight contract-call circuit. It proves authorization, transition validity, effect correspondence, intent refinement and per-step invariants together. The ledger verifies it against the operation key stored in contract state.
2. **History by ledger induction.**
   - Every entry point reads the head it replaces, or reads a head's absence when creating one.
   - Operation keys are immutable, and genesis is constrained.
   - Every live head therefore descends from genesis through accepted steps.
3. **Recursion only for certificates.** Midnight's `verify_proof` verifies bounded certificates inside a contract call: off-ledger execution segments, third-party attestations and cross-domain imports.
4. **The claim manifest is compiled into the key.** Mandatory claims are constraints of one proof. Certificate keys are circuit constants. The signed intent digest binds contract, instance, head, revision, program digest, effect bounds, observation policy, certificate requirements, nonce and validity window.

**Why the current proposal changes.**

- **Recursion cannot re-verify contract proofs.** Ledger-accepted contract proofs use a Blake2b transcript. `verify_proof` accepts only Poseidon-transcript zk-stdlib proofs, so a contract call cannot verify its predecessor contract call's proof.
- **The ledger already enforces the rest.** It selects the verifier from contract state, rejects stale reads at application, prevents coin, UTXO, DUST and intent reuse, and binds proofs to contract, entry point and effects.
- **Recursion is too expensive per transaction.** On one six-core machine, a Compact loan circuit proved in about 1.5 s. A trivial one-level `verify_proof` outer circuit took about 35 s and 4.1 GiB; a two-level one took about 82 s and 7.8 GiB.

## 2. Planning assumption and platform facts

**Planning assumption (user direction).** Midnight-native recursion reaches production within a few months. The concrete interface is midnight-ledger pull request 738 on the `ledger-10` branch, head `416da99309cff5f953f029ff6c89211c00cf423c`. On 2026-09-08 it was marked resolved for merge, on condition that the remaining concerns are fixed before `ledger-10` reaches release candidate.

**Observed 2026-09-11** (moving facts; re-check before use):

- **Networks.** Preview, Preprod and Mainnet run node `1.0.2-eb71e64e`, runtime spec 1000000, transaction version 3: the ledger 8 generation.
- **Ledger 9.** The next hard fork, in node 2.1.0-beta.1.
- **Compact.** Toolchain 0.34.0 targets ledger 9, and Mainnet developers are told to stay on 0.31.x. Cross-contract calls arrived in 0.33.0.
- **Recursion today.** No released ledger (8, 9 or 10) soundly supports application recursion. Compact and ZKIR have no proof-verification primitive.
- **Pull request 738 hazards at its head.**
  - Accumulator public inputs and pairings are not charged by the fee model.
  - A zero guard disables verification.
  - The `Collapsed` decider does not check an IVC proof's own key representation or state decider.
  - An invalid inner proof still yields an outer proof, rejected only at the ledger's accumulator pairing.
- **Setup files.** The ceremony SRS files for 2^18 and 2^19 are published and match the trusted-setup catalog hashes.

## 3. Relationship to existing packages and sprints

| Existing owner | Current text | Proposed amendment | Preserved |
|---|---|---|---|
| MC03 / SP06 real recursive history | Genuine recursive proofs for the fixed two-step loan episode, verified in a fresh process | Satisfy it with an **off-ledger segment certificate** over the Moriarty step (PCD Stage 7), verified by native IVC verification and by a `ledger-10` contract call. Retire the fixed 54-limb table relation, which failed key generation at k = 17. | Real recursion, independent-process verification and mutation controls. No host verdict or nonrecursive re-proof substitutes. |
| MC04 / SP09 ledger correspondence and consumption, with SP04 certificate feasibility | Complete native-to-Preview verifier with the actual final accumulator decision; compiler correspondence; durable one-time consumption | Declare the seam: the operation key in `ContractState.operations`, over the ledger-built statement. Map compiler correspondence to the fused step relation and head read-then-write discipline, and durable consumption to head discipline and E1. Certificates use pull request 738's ledger-side deferred pairing in SP04, with the P1/P2/P3 control discipline. | Strict encoding; deployed-version provenance; unique consumption under restart and concurrency. |
| MC05 / SP09 mandatory PCD and ledger correspondence | Four mandatory claims in every acceptance path; non-circular commitments; verifier and spec activation and revocation | Fuse transition, refinement, per-step invariant, authorization and applicability into one step relation per entry point. Discharge on-ledger history compliance by ledger induction. Keep the universal contract property as a deploy-time certificate bound by program digest. Replace the transaction-level manifest with the compiled claim set and intent digest v2. Replace in-place verifier revocation with forward-declared migration and a principal-threshold `Pause`. | Stripped claims, arbitrary verifiers, missing dependencies, stale observations and intent-invalid actions must all reject. Verification-enabled Preview acceptance stays required. |
| MC06 / SP10 private handoff and composition | Private continuation, five operators, split and join without predecessor secrets | Implement split and join as ledger-atomic head operations. Within one contract on ledger 8; across contracts with the Compact 0.33 toolchain, using `Releasing`/`Reclaim` recovery. Hand over recipient-encrypted openings and join summaries, not predecessor proofs. Off-ledger branches join only through certificates. | Real isolated participants; conserved residual duties, authority and work; recovery through the accepted lineage. |
| RP02 complete native history route | Specify successor artifacts, private secrets, predecessor composition and the final native decision | Settled in design for on-ledger history by sections 12–13 of the report, pending RP02 review, E1 and E2 for the core, and E4 for migration and reclaim. The certificate route stays conditional on the pull request 738 release, k ≥ 18 parameter serving, fee accounting, E3 and E5. | Pinned source and deployment versions. |

## 4. Change classes

| Class | Meaning |
|---|---|
| **M0** | Moriarty-only |
| **M1** | Supported use of Compact, Midnight.js, the proof server or an existing verifier |
| **M2** | Exposing or stabilizing an existing Midnight capability |
| **M3** | Midnight ledger or protocol change |
| **M4** | New cryptography |

This roadmap needs no M3 or M4 work from Moriarty. It depends on Midnight's own in-progress M3 work (pull request 738) only for Stage 7.

## 5. Stages

Each stage lists deliverables, a positive test, negative controls, retained evidence and an exit criterion. A local Rust proof is never ledger acceptance; those gates stay separate. Stages 0–6 run on the live ledger 8 generation or the ledger 9 toolchain and do not depend on recursion.

### Stage 0: Pin the verifier boundary (M0)

- **Deliverables.**
  - Toolchain manifest per network generation.
  - Seam specification.
  - Generated-ZKIR head-discipline checker.
  - Deploy audit tool that recomputes the contract address and checks four things:
    - the exact operation set;
    - every key against a reproducible build;
    - the uninitialized initial state;
    - an authority of committee `[]`, threshold ≥ 1, counter 0.
- **Positive test.** Recompute a deployed Preview contract's address and keys from its build.
- **Negative controls.** The audit flags a mismatched key, an extra operation, a non-uninitialized initial state, or an authority with threshold 0. The checker rejects a head write without a read, and a head creation without an absence read.
- **Evidence.** Toolchain digests, RPC receipts, checker and audit output.
- **Exit.** Seam written, reviewed and reproducible.

### Stage 1: One native transition proof (M1)

- **Deliverables.** The funded repayment step as one Compact circuit with the fused relation; intent digest v2; in-circuit JubJub Schnorr authorization; a Rust harness that verifies with ledger-8 `VerifierKey::verify`.
- **Positive test.** A proof from the proof server verifies against the compiled key.
- **Negative controls.** Altered amount, recipient or asset; missing signature; exceeded cap; wrong revision. Each fails proving or verification.
- **Evidence.** Rows, k, proving time, peak memory, proof bytes, public-input count.
- **Exit.** Every control fails as expected, and the circuit fits k ≤ 17.

### Stage 2: One authenticated predecessor extension (M1)

- **Deliverables.** `Initialize` and `Step` entry points with head discipline on a local ledger-8 node.
- **Positive test.** `Initialize`, then `Step`, both accepted, with the head at revision 1.
- **Negative controls.** A second `Initialize`, a stale `Step`, a forged head and a stale observation all reject at application. Record which transcript section each call landed in.
- **Evidence.** Transaction results and ledger error codes.
- **Exit.** Linearity observed at application.

### Stage 3: Ledger-enforced acceptance on Preview (M1)

- **Deliverables.** The Stage 2 contract deployed through a custom deploy path with committee `[]` and threshold 1, plus a passing deploy audit.
- **Positive test.** A Preview transaction accepted with verification enabled; head updated after finality.
- **Negative controls.** A maintenance update, a conflicting call from the same head and a mismatched effect all reject.
- **Evidence.** Block hashes, finalized head, public receipts.
- **Exit.** Requirements R2–R5 are ledger-enforced for one step on a public network. The R1 property certificate, off-ledger R6 certificates and R7 observations come in later stages.

### Stage 4: Sequential history (M1)

- **Deliverables.** 10 and 100 steps on one head, plus a `Terminate` transition.
- **Positive test.** Constant per-call cost; the terminal state rejects further steps.
- **Negative controls.** Replay of an earlier signed intent, a skipped revision and a step after termination all reject.
- **Evidence.** Per-call timing and size series.
- **Exit.** No cost growth with history length.

### Stage 5: Cross-party successor (M0 and M1)

- **Deliverables.** Recipient-keyed opening package; per-party sub-state commitments.
- **Positive test.** A second party proves the next step from the first party's head using only its opening.
- **Negative controls.** A tampered opening fails proving. A process audit shows the successor never held the predecessor's secret fields.
- **Evidence.** Package sizes and the audit log.
- **Exit.** Successor proving needs no predecessor proof or witness.

### Stage 6: Branch and join (M1)

- **Deliverables.** `Split` and `Join` within one contract on ledger 8. `Release`, `JoinFrom`, `Migrate`, `ImportFrom` and `Reclaim` across contracts on a ledger-9 network.
- **Positive test.** A → B, C → D with residual obligations conserved.
- **Negative controls.**
  - Duplicate predecessor.
  - Mixed-policy join.
  - Budget restoration.
  - Dropped obligation.
  - Downgrade migration.
  - `Release` without `JoinFrom`: it must apply, leave the head in `Releasing`, and be recovered only by `Reclaim`.
- **Evidence.** Transaction receipts for each case.
- **Exit.** All controls reject as specified.

### Stage 7: Native certificates (M1 once `ledger-10` ships; blocked on Midnight M2 and M3 items)

- **Deliverables.**
  - An attestation certificate relation.
  - An off-ledger segment certificate over the Moriarty step, in `Collapsed` shape. The outer circuit constrains the inner key representation and the state decider.
  - A certificate entry point with constant `VerifyProof` and `InnerProof` guards, on a `ledger-10` devnet.
- **Positive test.** A certificate-bearing call is accepted, and its accumulator pairing checks.
- **Negative controls.** A tampered inner proof, an inner proof for another key, a free or mismatched guard, a substituted `vk_repr` and an unbound inner instance all reject, or are caught by the Moriarty lint.
- **Evidence.** Outer k, proving time, memory, and charged fee against measured validation time.
- **Exit.** Certificates are sound, fit the k, time and memory bound set by a reviewed resource amendment, and accumulator fee accounting has landed upstream. This stage satisfies the amended MC03/SP06 recursion requirement.

### Stage 8: Performance and profile freeze (M0)

- **Deliverables.** All report section 14.4 microbenchmarks on one pinned machine; browser WASM measurements; final bounds.
- **Exit.** The profile is frozen into program digest v1.

## 6. Experiments, in order

| Order | Experiment | Unlocks | Runs on |
|---|---|---|---|
| E1 | Read-then-write linearity: two conflicting calls from one head, plus a write-without-read control | Stages 2–4; confirms ledger induction | Preview, ledger 8 |
| E2 | Fit of the fused step relation, in three variants | Stage 1 bounds; Compact versus hand-written zk-stdlib relation | Proof server 8.1.0; browser WASM |
| E3 | Native certificate with negative controls and fee-versus-work measurement | Stage 7 go or no-go | `ledger-10` devnet, pull request 738 or its merged successor |
| E4 | Immutable keys, one-direction cross-contract migration and `Reclaim` | Upgrade rule; Stage 6 cross-contract rules | Ledger-9 network or local node |
| E5 | Off-ledger IVC segment certificate at 1, 10 and 100 steps | Whether segment mode is offered at all | midnight-zk 7.2.4 / 2.3.5 plus pull request 738 |

E1 and E2 decide the core on today's network. A failed E1 replaces head discipline with an explicit consumption set in contract state. A failed E3 or E5 removes certificates or segment mode from the profile without affecting Stages 0–6.

## 7. Midnight dependency tracker

Observed 2026-09-11. Re-check before each dependent stage.

| Item | Needed by | Status | Class |
|---|---|---|---|
| Pull request 738 merged and `ledger-10` released and deployed | Stage 7 | Open; merge conditionally agreed | M3 (Midnight) |
| Fee accounting for accumulator public inputs and pairings | Stage 7 exit | Absent at the pull request head | M3 (Midnight) |
| Guard lint for `VerifyProof` and `InnerProof` | Stage 7 | Absent; Moriarty lint covers it meanwhile | M2 |
| Collapsed decider checking `vk_repr` and state | Stage 7 | Absent; the Moriarty outer relation covers it meanwhile | M2 or M3 |
| k ≥ 18 parameters served through the proof server and data provider | Stage 7 | Ceremony files published; serving path unverified | M1 or M2 |
| Compact `verifyProof` frontend | Stage 7 ergonomics | Draft MIP only; ZKIR hand emission meanwhile | M2 |
| midnight-js and proof-server inner-proof plumbing; Poseidon proving for zk-stdlib relations | Stage 7 | Absent; Moriarty Rust prover meanwhile | M2 (M0 workaround) |
| Published IVC module with instance and accumulator serialization | E5, off-ledger handoff | Unpublished | M2 |
| Recipient-keyed private-state handoff | Stage 5 | Absent; Moriarty encryption meanwhile | M2 (M0 workaround) |
| Network id in the contract statement | Residual replay risk | Absent; `netTag` separates networks but not byte-identical replicated deployments | M3 (optional) |
| Maintenance-authority threshold validation | Defense in depth | Absent; deploy audit covers it | M3 (optional) |
| Ledger 9 on Preview | SP09.3 migration and SP10.3 cross-contract Preview evidence | Hard fork in node 2.1.0-beta.1; Preview runs ledger 8 | M3 (Midnight) |
| Reviewed resource amendment for certificate k | Stage 7, E3 and E5 | Not requested; campaigns capped at k ≤ 17 | Moriarty decision |

## 8. Bounds to freeze

Initial values are proposals. The named benchmark sets each final value.

| Bound | Initial | Set by |
|---|---|---|
| On-ledger join fan-in | 2 | Join rows, prover time and public inputs for fan-in 2, 3, 4 |
| Branch fan-out | 2 | Split for k = 2, 3, 4 |
| Certificates per call | 1 | Outer k and prover time for m = 1, 2 |
| Recursion depth, on-ledger history | 0 | Structural |
| Off-ledger segment length | 16 steps | Segment benchmark at 1, 10, 100 steps |
| Effects / assets / obligations touched per step | 16 / 4 / 8 | Step benchmark variants |
| Public inputs per call | ≤ 1,024 | Verification time against public-input count |
| Core step k / certificate entry point k | ≤ 17 / set by a reviewed resource amendment (measured 18–19) | Step and certificate benchmarks |
| Proving time / memory on the proof server | ≤ 600 s / ≤ 8 GiB | Step and certificate benchmarks |
| Verifier work per transaction | ≤ 100 ms at the cost model | Public-input and pairing measurements |

## 9. Open decisions requiring design review

The [PCD integration amendment](PCD-INTEGRATION-2026-09-11.md#decision-register) records these with owners, defaults and status.

1. **Contract layout.** One contract per agreement instance, or one contract holding several heads.
2. **Upgrade rule.** Immutable keys with forward-declared migration, or an all-principal committee with delay for long-lived agreements.
3. **Privacy and circuit size.** Whether a single `step` entry point is required to hide action type.
4. **Join summaries.** Their content per financial profile, and the rules they cannot express.
5. **Retirement of the fixed MC03 table relation.** Replacement by Stage 7 segment certificates.
6. **Relationship to the atomic profile's claim schema.** Migration of `requiredClaimRoot` to the compiled claim set and intent digest v2.

## 10. Stop conditions

- **E1 fails.** Stop Stages 2–6 until a consumption-set design is reviewed.
- **Stage 1 cannot fit within bounds after the E2 variants.** Stop and revisit relation decomposition, not the acceptance requirements.
- **`ledger-10` changes the pull request 738 formats.** Pin the released formats and rerun E3 before Stage 7 evidence is accepted.
- **Midnight reverses or indefinitely delays recursion.** Stages 0–6 and the ledger-anchored acceptance model stand. Stage 7 and the amended MC03/SP06 requirement return to design review. Mandatory evidence is not weakened.
- **The certificate resource amendment is refused.** Stage 7 stays blocked. Stages 0–6 and the Preview gate stand.

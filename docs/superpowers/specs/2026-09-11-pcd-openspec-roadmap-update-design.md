# PCD-driven OpenSpec roadmap update: design

- Date: 2026-09-11 UTC.
- Status: design for a specified-only planning revision. It grants no admission, dispatch or acceptance.
- Inputs:
  - Decision report `deliverables/pcd-midnight-native-2026-09-11/REPORT.md`.
  - Proposed roadmap `openspec/PCD-ROADMAP-2026-09-11.md` (PCDR).
  - Four read-only audits of the OpenSpec roadmap, repository, design sources and knowledge graph, all taken at `main` `89b0c7b`.

## 1. Goal

Adopt the Midnight-native PCD decision into the OpenSpec roadmap. The decision is a ledger-anchored certified state machine with bounded native certificates. Adoption must leave the planning validators, the development plugin, the hash pins and every protected gate intact.

The update succeeds when four things hold:

1. **Recursion no longer gates core acceptance.** Verification-enabled mandatory acceptance on Midnight Preview no longer waits on `ledger-10` recursion.
2. **Certificates stay in the release.** Off-ledger segments, attestations and imports remain required release scope, under explicit stop conditions.
3. **Every new PCD obligation has an owner.** Each obligation is an OpenSpec requirement with a negative scenario, an owning MC package and an owning sprint.
4. **All three validators pass.** These are `openspec validate --all --strict`, the plugin tests, and `verify.py`. The planning validator is run in a clean copy that neutralizes only its two pre-existing, unrelated failures.

## 2. Audit synthesis

### 2.1 Findings that shape the design

- **The stage graph makes the core wait on recursion.**
  - `f3` requires `f2` and `f1`.
  - `mandatory`, `composition`, `finance` and `release` all transitively require `f2`.
  - Package dependencies `MC04←MC03` and `MC05←MC03` hard-code the recursive proof.
  - SP09 lists SP06 as a completion dependency. SP09.1 says "Use the SP06 retained proof as the MC04 extension input".
  - Without re-plumbing, the hard Preview gate waits for `ledger-10`.
- **Core assumptions are false on Midnight.**
  - A contract proof cannot be an inner proof: it uses a Blake2b transcript, and `verify_proof` needs Poseidon.
  - Pairings are never computed in-circuit.
  - Compact writes emit no read, so blind writes do not conflict.
  - Deploy runs no proof.
  - midnight-js deploys a one-key maintenance committee by default.
  - Claimed cross-contract calls are atomic in one direction only.
  - The transaction limit is 1 MiB. The current bounds profile allows 1 MiB per proof and 16 MiB total.
- **PCD obligations with no owner.**
  - Deploy audit and immutable authority.
  - Head-discipline checker.
  - Constrained genesis.
  - Intent digest v2, program digest Π_P and genesis body v2 encodings.
  - Block-time observation freshness.
  - Forward-declared migration.
  - `Releasing`/`Reclaim` recovery.
  - Certificate guard lint and `Collapsed` decider constraints.
  - Experiments E1–E5.
  - Midnight dependency tracker.
  - Bounds freeze.
- **Errors in the PCD roadmap and report.**
  1. The migration allowlist is circular: two contract addresses would commit to each other.
  2. The commitment hash is SHA-256 in one section and Poseidon in another.
  3. Peak-memory figures are misquoted.
  4. The MC04 row is mislabelled "native verifier feasibility".
  5. `Reclaim` and `Terminate` are missing from the entry-point list.
  6. A Stage 1 offline control depends on block time, which only exists at application.
  7. Stage 3 omits the custom immutable-authority deploy path.
  8. Stage 3's exit overstates "mandatory acceptance".
  9. The roadmap says RP02 is "resolved in design", but it still depends on unmerged `ledger-10` work.
  10. The roadmap lists `netTag` as a mitigation, but it gives no separation against byte-identical replicas.
- **Graph and records.**
  - The graphify graph predates today's files.
  - The wiki canvas is stale.
  - The stage DAG itself is acyclic, with 21 stages.

### 2.2 Constraints the design must respect

- **Validator couplings in `openspec/sprints/verify.py`:**
  - Stage `requires` and owners must equal each sprint `entryGates` entry.
  - Every stage must have a sprint owner.
  - `native-path-freeze` must stay in `f0a.requires` and `f2.requires`.
  - Every `### Requirement:` heading under `openspec/changes/*/specs/*/spec.md` needs exactly one `coverage.json` row. The row's package must own every listed sprint.
  - `Register.packages[].sprints` must equal the ordered list of sprints that package owns.
  - Original `mc*/tasks.md` lines are locked verbatim. A new `mc*` directory's `tasks.md` would be parsed and locked too.
  - Lesson ids must be exactly LR01–LR18. Every lesson source must be digest-pinned. A sprint's `reportLessons` must equal the lessons that name its tasks.
- **Closed shapes.** The plugin's `records.py` rejects unknown top-level Register keys, stage keys, stage-admission keys and gate keys. The schema limits stage owners to MC01–MC08. Plugin tests hard-code stage `f0` and task `SP04.1`.
- **Hash pins.**
  - `REPORT-RECONCILIATION-2026-09-07.md` is pinned in `report-lessons.json` and in `.moriarty-dev/loan-design-verification-binding.json`.
  - The SP01 sprint document is pinned in that binding.
  - The in-flight AFK work depends on both.
- **Pre-existing validator failures, not caused by this update.**
  - The untracked `openspec/changes/afk-live-financial-execution/` has 37 requirement headings without coverage rows. Another session edited it today.
  - `docs/FOOTGUNS.md` gained a four-line Grok-timeout section after its digest was pinned at commit `214141c`.
  - Both make `verify.py` fail and seven of eight `test_verify.py` tests fail.
- **Gates that must not weaken.**
  - The hard Preview gate with verification enabled.
  - Mandatory evidence, with no host verdict, mock or deterministic-only substitute.
  - Independent review rules.
  - `dispatchEnabled: false` and the resource ceilings.
  - Verbatim retention of original tasks and G01–G24.
  - Failed k17 evidence stays failed.

## 3. Approaches considered

### Approach 1: re-root the existing stage graph and layer a PCD change package (recommended)

- Keep all 21 stage identifiers and repurpose the native track:
  - `f0`, `native-path-freeze`, `f0a`, `f1-fixtures`, `f1` and `f2` become the Stage 0 and certificate track.
  - `f3` becomes the ledger-anchored core (Stages 1–3).
- Cut `f3` loose from `f1`/`f2`, and MC04/MC05 loose from MC03. Keep `f2` in `release`.
- Add one non-`mc` change package, `pcd-ledger-anchored-acceptance`, whose requirements are owned by MC03–MC06.
- Record the mapping and supersessions in an integration amendment and a machine crosswalk. Update sprint contracts, narrative documents and the PCD roadmap.
- **Trade-offs.**
  - Stage purposes change under stable identifiers, which a reader must follow through the amendment.
  - In return, no closed schema, plugin test, hash pin or locked task changes, and the AFK work is undisturbed.

### Approach 2: add a parallel PCD stage track with new identifiers

- Add new stages such as `pcd-seam`, `pcd-step`, `pcd-e1`, `pcd-preview` and `pcd-cert`, and retire `f0a`–`f2`.
- **Trade-offs.**
  - Names are clearer.
  - But every new stage needs a sprint owner, gate rows, sprint tasks and plugin fixtures.
  - Retiring `f0a`/`f2` requires editing `verify.py`'s path-freeze check.
  - SP01's pinned document names `native-path-freeze`, and plugin tests name `f0`/`SP04.1`.
  - The blast radius is roughly three times Approach 1, with no gain in enforced semantics.

### Approach 3: proposal-only

- Mark PCDR adopted in prose and change no machine record.
- **Trade-offs.**
  - Cheapest.
  - But the machine graph still makes `mandatory` wait on `f2`, so the plugin would keep refusing core work behind recursion.
  - That fails the goal.

**Recommendation:** Approach 1.

## 4. Design

### 4.1 Stage graph (Register `reportReconciliation.stageAdmission.stages`, mirrored in `sprints.json`)

Identifiers, owners and statuses do not change. Only `requires` and `purpose` change where the table shows it.

| Stage | Owners | `requires` (new) | New purpose |
|---|---|---|---|
| `f0` | MC04, MC03 | [] (unchanged) | PCD Stage 0: pin the verification seam (operation key in `ContractState.operations`, ledger `well_formed`); toolchain manifest per network generation; head-discipline checker and deploy-audit designs; certificate route go/no-go conditional on `ledger-10` |
| `native-path-freeze` | MC03, MC04 | [f0] (unchanged) | Reviewed path and command ownership for the step-relation compiler output, head-discipline checker, deploy audit and certificate-relation roots before source authorship |
| `f0a` | MC04, MC03 | [f0, native-path-freeze] (unchanged) | Certificate-relation authorship against the pinned pull request 738 / `ledger-10` sources: guard-constant lint, `Collapsed` decider constraints on `vk_repr` and state, frozen hashes and current source reviews |
| `f1-fixtures` | MC03, MC04 | [f0a] (unchanged) | Separately admitted independent non-loan certificate fixtures (Poseidon zk-stdlib inner proofs, verifier-test accumulator) |
| `f1` | MC04, MC03 | [f0a, f1-fixtures] (unchanged) | E3: a native certificate accepted on a `ledger-10` devnet, including its accumulator pairing. All negative controls reject: free or mismatched guard, substituted `vk_repr`, unbound inner instance, inner proof for another key, tampered inner proof. Fee against measured validation work is recorded |
| `f2` | MC03 | [atomic-accept, rp01-mc03, f1, native-path-freeze] (unchanged) | E5: off-ledger segment certificate over the Moriarty step at 1, 10 and 100 steps, retained and verified in a fresh process; `ledger-10` import |
| `f3` | MC04 | **[atomic-accept, i2, f0, native-path-freeze]** (was [atomic-accept, i2, f2, f1]) | PCD Stages 1–3 for the atomic loan/swap profile. E2: fused step relation fits within bounds. E1: head read-then-write linearity on Preview. Deploy audit of immutable authority. Verification-enabled Preview acceptance of R2–R5 for one step each |
| `mandatory` | MC05 | unchanged | Versioned mandatory-claim extension under the fused step relation: all four claim families discharged at their declared loci, constrained genesis, deploy-audited property certificate, observation freshness and forward-declared migration (E4) |
| `composition` | MC06 | unchanged | Versioned composition: ledger-atomic split/join, cross-contract `Release`/`JoinFrom`/`Reclaim` on ledger 9, recipient-keyed handoff and operator mapping |
| `finance`, `release`, others | unchanged | unchanged | `release` keeps `f2`. Certificates stay release scope under the planning assumption that recursion ships within months |

`f3` gains `native-path-freeze` because the step-relation compiler and checker paths are frozen there. `f0` is already an ancestor of `native-path-freeze`, but listing it keeps the Stage 0 dependency explicit.

The resulting graph is acyclic. `mandatory`'s transitive prerequisites exclude `f0a`, `f1-fixtures`, `f1` and `f2`.

**Ordering notes.**
- `f2` keeps `f1`. E5's ledger import uses the certificate entry point whose soundness E3 establishes, so E5 follows E3 deliberately.
- `f2` keeps `rp01-mc03`. The amendment reinterprets SP01.7: the fixed native-statement subset becomes the reviewed statement subset of the Moriarty step that segment certificates fold.

### 4.1a Register fields outside the stage graph

The plugin closes none of these fields.

| Field | Current value | New value |
|---|---|---|
| `packages[MC03].commands` | Three `native-ivc-r3/successor/run-reviewed.py` commands; that root does not exist | `[]` |
| `packages[MC03].commandStatus` | Planned sole successor root | The `native-ivc-r3/successor` root is retired. Certificate-relation commands are fixed at `native-path-freeze` and frozen after implemented-source review |
| `packages[MC04].status` | `interface-blocked` | `specified-only` |
| `packages[MC04].feasibilityStatus` | Complete native-to-Preview verifier unresolved | Core seam declared from pinned ledger-8 source; RP02 review, E1 and E2 pending. The certificate route belongs to MC03 and waits on `ledger-10` |
| `reportReconciliation.feasibilityStages` | Recursive-route text for F0–F4, F0a and I2 | F0: Stage 0 seam. F0a: certificate authorship. F1: E3. F2: E5. F3: ledger-anchored core with E1, E2 and Preview acceptance. F4: composition and full finance. I2 unchanged |
| `reportReconciliation.gates[RP02]` | `blocked`; no complete native-to-Preview interface | Status stays `blocked`. Reason: the on-ledger history route is settled in design by the PCD integration, pending RP02 review; the certificate route is blocked on `ledger-10` release, a resource amendment for k ≥ 18 and accumulator fee accounting |
| `packages[MC04]` and `packages[MC05]` `dependencies` | See §4.3 | See §4.3 |

### 4.2 Sprint records (`openspec/sprints/sprints.json`)

- **Entry gates.** SP09's `f3` gate `requires` mirrors §4.1.
- **SP09 `completionRequires`.** Becomes [SP03, SP05, SP07, SP08]; SP06 is removed.
- **SP12 `completionRequires`.** Becomes [SP06, SP11]. Release still needs the certificate sprint, and the sprint-level edges must keep saying so.
- **Owners, stages and tasks.** No sprint changes owners or stages. No task ids are added or removed, so `Register.packages[].sprints` and the plugin tests are unaffected.
- **Report lessons.** Unchanged, because lesson task lists do not change (§4.7).

### 4.3 Package dependencies (Register `packages[].dependencies`, Charter table)

| Package | Old | New |
|---|---|---|
| MC04 | MC01, MC02, MC03 | MC01, MC02 |
| MC05 | MC03, MC04 | MC04 |
| others | unchanged | unchanged |

- **Charter table.** Update the MC03 outcome to "Certificate relations and off-ledger segment certificate, independently verified".
- **Dependencies.** Update the MC04 and MC05 columns to match the Register.
- **Package design files.** The "Dependencies:" lines in `mc04…/design.md` and `mc05…/design.md` change to match. Those files are not validator-locked.

### 4.4 New change package `openspec/changes/pcd-ledger-anchored-acceptance/`

**Files:**
- `README.md`: status and scope.
- `proposal.md`: why; what changes; the supersession table in §4.5.
- `design.md`: interfaces, meaning the step relation, head discipline, deploy audit, digests and certificates.
- `tasks.md`: unchecked tasks. The directory name does not start with `mc`, so `verify.py` does not lock them.
- Four capability specs. Each starts with `## ADDED Requirements`. Each requirement has at least one positive and one negative scenario.

Each requirement gets one `coverage.json` row. The row's package owns every listed sprint, and the primary sprint closes the requirement.

| # | Spec file | Requirement heading | Package | Sprints | Primary |
|---|---|---|---|---|---|
| 1 | `specs/pcd-ledger-correspondence/spec.md` | Declared ledger verification seam | MC04 | SP01, SP04, SP09 | SP09 |
| 2 | same | Fused step relation per entry point | MC04 | SP09 | SP09 |
| 3 | same | Head read-then-write discipline | MC04 | SP09 | SP09 |
| 4 | same | Immutable authority deployment audit | MC04 | SP05, SP09 | SP09 |
| 5 | `specs/pcd-mandatory-acceptance/spec.md` | Claim discharge map | MC05 | SP01, SP09 | SP09 |
| 6 | same | Constrained genesis and termination | MC05 | SP09 | SP09 |
| 7 | same | Intent digest v2 and program digest | MC05 | SP01, SP08, SP09 | SP09 |
| 8 | same | Observation freshness at application | MC05 | SP09 | SP09 |
| 9 | same | Forward-declared migration replaces in-place revocation | MC05 | SP09 | SP09 |
| 10 | same | Governed deploy-time property certificate | MC05 | SP09 | SP09 |
| 11 | `specs/pcd-composition/spec.md` | Ledger-atomic split and join | MC06 | SP10 | SP10 |
| 12 | same | Cross-contract release with reclaim | MC06 | SP10 | SP10 |
| 13 | same | Recipient-keyed successor handoff | MC06 | SP10 | SP10 |
| 14 | same | Composition operators under head discipline | MC06 | SP10, SP11 | SP10 |
| 15 | `specs/pcd-certificates/spec.md` | Bounded native certificates | MC03 | SP04, SP06 | SP04 |
| 16 | same | Off-ledger segment certificate | MC03 | SP06 | SP06 |
| 17 | same | Recursion dependency tracking and stop | MC03 | SP01, SP04, SP06 | SP04 |
| 18 | `specs/pcd-ledger-correspondence/spec.md` | Measured bounds frozen into the program digest | MC04 | SP09, SP11 | SP11 |

Before writing coverage rows, ownership was checked against `sprints.json` owners:

| Package | Sprints it owns |
|---|---|
| MC03 | SP01, SP04, SP06 |
| MC04 | SP01, SP03, SP04, SP05, SP09, SP11 |
| MC05 | SP01, SP03, SP08, SP09, SP11 |
| MC06 | SP01, SP10, SP11 |

**Normative content per requirement (scenarios in the spec files):**

1. **Seam.** A contract-call proof is accepted only by ledger `well_formed` against the operation key stored in `ContractState.operations`. A toolchain manifest pins Compact, ZKIR, ledger and proof-server versions per network generation (ledger 8, ledger 9, `ledger-10`).
   - **Negative.** A host verdict, a mock verifier, a claimed in-circuit pairing, or a proof checked against a key not read from contract state is not acceptance evidence.
2. **Fused relation.** Each entry point compiles to one circuit. It proves authorization, transition validity, effect correspondence, intent refinement and the per-step invariant together. The native effects equal the projection of the Moriarty effects.
   - **Negative.** An unbound effect, recipient, asset or cap mutation fails proving or verification. Separate per-claim proofs for one action are rejected as a design.
   - **Stop.** If E2 shows the funded repayment step cannot fit k ≤ 17 in any of its three variants, relation decomposition is revisited. The acceptance requirements are not relaxed.
3. **Head discipline.**
   - Every head write is preceded by a read of that head in the same transcript section, with no checkpoint between. Head creation is preceded by an absence read.
   - A checker over generated ZKIR enforces this.
   - E1 on Preview shows that of two individually valid conflicting calls from one head, at most one applies.
   - **Negative.** A blind-write mutant is rejected by the checker.
   - **Stop.** If E1 fails, Stages 2–6 stop until a reviewed explicit consumption-set design replaces head discipline.
4. **Deploy audit.**
   - The audit recomputes the contract address and checks the exact operation set and each key against a reproducible build. It also checks that the initial state is `Uninit(Π_P, netTag)` and that authority is committee `[]`, threshold ≥ 1, counter 0.
   - A custom deploy path is used because midnight-js defaults to a one-key committee.
   - Existing continuation tooling that asserts a one-key committee must not gate these deployments.
   - **Negative.** An extra operation, a key mismatch, a non-`Uninit` state, threshold 0 or a non-empty committee fails the audit. No acceptance is claimed for that deployment.
5. **Claim discharge map.**
   - Four families map to their discharge loci in the report's R1–R7:
     - ContractInvariant: deploy-time property certificate plus the per-step invariant.
     - IntentRefinement and TransitionValidity: the fused relation.
     - HistoryCompliance: ledger applicability plus provenance by induction on ledger, or by certificate off ledger.
   - One canonical claim vocabulary is used, with recorded historical aliases.
   - **Negative.** If immutable keys, constrained genesis or head discipline is not established for a deployment, HistoryCompliance is not discharged by induction, and acceptance for that deployment is not claimed.
6. **Genesis and termination.**
   - Deploy writes `Uninit(Π_P, netTag)`.
   - `Initialize` proves the genesis predicate with all-principal signatures and succeeds once.
   - It derives `instanceId = H(A ‖ H(G))`.
   - `Terminate` moves a `Live` head to `Terminated`. It needs either the terminal condition of the contract rules or the principal threshold declared in Π_P, and it records a disposition for every residual obligation. The report names the `Terminated` lifecycle value but no entry point, so this rule is an addition.
   - **Negative.** A second `Initialize`, a missing principal signature, or a deploy not in `Uninit` rejects. Any step, split, join or migration on a `Terminated` head rejects. A `Terminate` that leaves an obligation without a recorded disposition rejects.
7. **Digests.**
   - Canonical encodings and test vectors exist for Π_P, intent digest v2 and genesis body v2.
   - Two signing modes exist.
     - **Exact-head.** It binds `netTag`, A, e, instanceId, headId, r, S_r, Π_P, action and arguments, caps, obsPolicy, CertReq, validity window, and a nonce that is informational only.
     - **Outcome.** It binds the constraint set, a program-digest allowlist and the validity window, plus a nonce recorded in a per-instance consumed-nonce set.
   - **Negative.** Any altered field rejects. Outcome replay with a consumed nonce rejects. Exact-head replay after the revision increments rejects.
8. **Freshness.**
   - Observations are signed by `σ.oracleKeys` and verified in the circuit.
   - The feed allowlist and `maxObservationAge` sit in obsPolicy.
   - Freshness uses block-time reads in the transcript. The step relation proves them, and the ledger re-executes them at application.
   - **Negative.** A stale observation rejects at application, on a local ledger node or on Preview. An offline proof verification is not evidence of freshness.
9. **Migration.**
   - Operation keys are immutable. Replacement happens only through `Migrate`, and the successor's version must be strictly greater.
   - Following the report, `Migrate` accepts a successor under one of two branches:
     - **Declared.** The successor program digest Π_new is in the successor allowlist compiled into Π_old. That allowlist holds program digests only, never addresses. The principal threshold that Π_old declares for migration signs a digest binding netTag, A_old, A_new, Π_new, r and S.
     - **Unanimous.** All principals sign `MORIARTY-MIGRATE-v2` over netTag, A_old, A_new, Π_new, r and S. This admits a successor written after the predecessor was deployed.
   - The successor records its predecessor in its deploy-time initial state, `Uninit(Π_new, netTag, A_old)`, not in Π_new. `ImportFrom` accepts only a claimed `Migrate` call from that address whose commitment is absent from the imported set.
   - Hash edges therefore run one way. A_old commits to Π_old, which may contain Π_new. A_new commits to Π_new and A_old. No cycle remains.
   - Principals run the deploy audit on A_new before signing. Relying parties run it on the counterparty.
   - An optional principal-threshold `Pause` entry point halts new steps without changing keys.
   - Cross-contract migration evidence requires ledger 9. Its Preview qualification is a tracked dependency.
   - **Negative.** Each of these rejects: a maintenance update; key substitution; a downgrade; a successor neither declared nor unanimously signed; an import whose claimed caller differs from the recorded predecessor; a second import of the same commitment; revival of consumed authority.
10. **Property certificate.**
    - The universal ContractProperty certificate is checked by the deploy audit, which acts as the governed registration mechanism.
    - Its hash is bound in Π_P.
    - A ledger-checked `Initialize` certificate may replace the audit once `ledger-10` certificates are accepted.
    - **Negative.** A hash present with a failing or absent certificate check fails the audit.
11. **Split/join.**
    - Same-contract `Split` and `Join` are ledger-atomic.
    - Child `headId`s are computed in the circuit.
    - Budget is conserved, obligations are partitioned and authority is attenuated.
    - The lifetime invariant is per head: `revision + remaining = allocatedLifetime(head)`, fixed when the head is created.
    - Join requires distinct heads and `Compatible_P`. Fan-in and fan-out are at most 2 until benchmarks set them.
    - **Negative.** A duplicate head, a mixed-policy join, budget restoration, a dropped obligation, or exceeded fan-in or fan-out rejects.
12. **Release/reclaim.**
    - Cross-contract `Release`, `JoinFrom` and `Reclaim` use claimed calls (Compact 0.33+, ledger 9).
    - Atomicity is one-directional. An unclaimed `Release` applies alone and leaves the head `Releasing`.
    - `Reclaim` returns a `Releasing` head to `Live` only when it claims, in the same intent, a target-side call that reads the release commitment as absent from the target's imported set and marks it dead there. A claim can occur only in the same intent as its `Release`, so an unclaimed `Release` can never be imported later. E4 must confirm this.
    - **Negative.** Each of these rejects: a second import of the same commitment; a `Reclaim` without the claimed target-side absence call; a `Reclaim` after the commitment was imported; a `JoinFrom` whose claimed `Release` is missing.
13. **Handoff.**
    - A successor proves from the head commitment and a recipient-encrypted opening. It needs no predecessor proof or witness.
    - Per-party sub-state commitments are used where parties differ.
    - **Negative.** A tampered opening fails proving. Isolation evidence shows the successor never held predecessor secrets.
14. **Operators.**
    - Operators map to rules as follows:
      - sequential maps to `Step`;
      - disjoint parallel maps to `Split`;
      - atomic synchronization maps to same-contract `Join` or a claimed cross-contract call.
    - Shared-state interleaving, asynchronous messaging and Pending receive a reviewed rule under head discipline before any SP10 campaign that advertises them.
    - **Negative.** An operator without an accepted rule is unavailable and is not advertised.
15. **Certificates.**
    - Certificate relations are Poseidon zk-stdlib relations. They are verified through `VerifyProof`/`InnerProof` with constant guard 1, and their per-accumulator pairing is checked in ledger `well_formed` (`ledger-10`).
    - Contract proofs are never inner proofs.
    - The outer relation constrains `vk_repr` and the state decider.
    - Certificate campaigns need a reviewed resource amendment first. The measured outer circuits need k = 18 for one level (about 4.1 GiB) and k = 19 for two levels (about 7.8 GiB), above the Charter's k ≤ 17 ceiling.
    - **Negative.** A free or mismatched guard, a substituted `vk_repr`, an unbound inner instance, an inner proof for another key, or a tampered inner proof rejects or is caught by lint. An outer proof produced over an invalid inner proof is not evidence until the ledger pairing accepts.
16. **Segments.**
    - E5 produces segment certificates at 1, 10 and 100 steps, with a segment length bound of 16 until benchmarked.
    - They are verified in a fresh process from retained bytes and imported through a certificate entry point.
    - **Negative.** A host hash chain, a MockProver run, a nonrecursive re-proof or an altered segment is not evidence.
17. **Dependency tracking and stop.**
    - Certificate campaigns cannot dispatch until their tracker items are satisfied and pinned:
      - pull request 738 merged, `ledger-10` released;
      - accumulator fee accounting;
      - k ≥ 18 parameters served;
      - released formats pinned;
      - a reviewed resource amendment for the certificate k bound.
    - The tracker also carries ledger 9 on Preview, needed by `mandatory` for SP09.3 migration and by `composition` for SP10.3. Stages that need it record a blocked status until it holds.
    - **Stop.** If recursion slips, certificate stages record a blocked status. The core requirements and the hard Preview gate are unaffected, and no scope is dropped without new user direction.
    - **Negative.** A dispatch attempted with an unsatisfied tracker item is refused.
18. **Bounds freeze.**
    - The report's §14.4 microbenchmarks run on one pinned machine, together with browser WASM measurements.
    - They set the final bounds: join fan-in, branch fan-out, certificates per call, segment length, effects, assets and obligations per step, public inputs per call, core step k, proving time and memory, and verifier work.
    - The final bounds enter `bounds.json`, owned by MC01, and Π_P as a new program-digest version.
    - **Negative.** A bound not set by a retained benchmark, or a profile whose Π_P omits a bound, is not frozen, and no campaign may rely on it.

### 4.5 Supersession of existing requirements

MC requirement headings and locked `mc*/tasks.md` lines do not change. Scenario text in MC `spec.md` files is not validator-locked. Each affected scenario therefore gains one bullet in place: "**Amended by** `pcd-ledger-anchored-acceptance` requirement N: <one sentence>". Each affected MC `README.md` also gains one line naming the amending change.

Two dispositions are used:
- **Mechanism replaced.** The rejection the MC text requires still holds; the PCD requirement supplies how it is enforced.
- **Removed.** The MC text described the retired recursive route. Each removal is listed in §4.9 as a decision.

| MC requirement or scenario | Disposition | Governing PCD requirement |
|---|---|---|
| MC03 "Reviewed restart", scenario "Checked smaller encoding" | Removed: the fixed 54-limb relation is retired (decision 7) | 16 |
| MC03 "Real recursion and independent verification", including scenario "Valid episode" | Mechanism replaced: the fixed two-step episode becomes the E5 segment certificate at 1, 10 and 100 steps | 15, 16 |
| MC03 "Complete backend decision before native dispatch" | Mechanism replaced | 17 |
| MC04 "Actual verifier compatibility", scenario "Missing interface" | Mechanism replaced: the core is not interface-blocked; host assertions and mocked verification still cannot substitute | 1 |
| MC04 "Durable one-time consumption" | Mechanism replaced | 3 |
| MC04 "Early complete verifier feasibility" | Mechanism replaced | 1, 15 |
| MC05 "Mandatory acceptance", scenario "Downgrade attack" | Mechanism replaced: compiled claim set and immutable keys | 4, 5, 9 |
| MC05 "Intent refinement and complete effects" | Mechanism replaced: digest v2 binding fields | 7 |
| MC05 scenario "Revoked verifier or inactive specification" | Mechanism replaced: a defective or compromised program is stopped by `Pause` and migration, not by key revocation (decision 2) | 9 |
| MC05 scenario "Oversized verification input" | Mechanism replaced: Π_P bounds checked before proving | 7, 18 |
| MC06 "Bounded history composition" | Mechanism replaced | 11, 12 |
| MC06 "Composition operators and witness ownership" | Mechanism replaced: on-ledger successors need no predecessor proof; off-ledger branches still need certificates | 13, 14 |

The implementation adds two rows that carry in-place notes, MC03 "Invalid proof context" and MC05 "Bound outcome claims", and a third disposition, **Extended**, for a rejection list that gains cases.

### 4.5a Locked task dispositions

Task lines stay verbatim. The amendment and the crosswalk's `lockedTasks` section record what each closing sprint task now does.

| Package task | Disposition | Governing PCD requirement |
|---|---|---|
| MC03 1.1 (compare limbs with commitment encoding) | Removed with the fixed relation (decision 7); SP06.1 reviews the segment relation encoding instead | 16 |
| MC03 2.1 (preimage, limb-boundary, arithmetic, context, malformed-genesis tests) | Mechanism replaced: the tests apply to the segment relation; limb-boundary tests are removed | 6, 16 |
| MC03 2.2 (compare every episode field with R2) | Retained for the Moriarty step folded by the segment | 16 |
| MC03 3.1, 3.2 (separate verifier process; fresh deserialization, key and SRS identity, accumulator obligations) | Retained for certificates | 15, 16 |
| MC03 4.2 (exactly two positive steps) | Mechanism replaced: E5 runs 1, 10 and 100 steps under a reviewed resource amendment; stop on the first failed predicate is retained | 16, 17 |
| MC03 D.1 (source-only verifier-interface intake) | Mechanism replaced: pins the released pull request 738 / `ledger-10` formats | 17 |
| MC04 1.1, 1.2 (pin formats and entry point; positive probe with mutated controls) | Retained as the Stage 0 toolchain manifest, seam and Stage 1 controls | 1, 2 |
| MC04 4.1, 4.2 (crash, duplicate, competing-branch tests; at most one finalized success) | Mechanism replaced: head discipline and E1 | 3 |
| MC04 P.1–P.7 (adapter-profile-01 proof campaign) | Mechanism replaced: Compact step circuits proved by the proof server and verified by a Rust harness; no MC03 proof input | 2 |
| MC04 D.1 (interface-intake outcome and wrapper alternatives) | Mechanism replaced: the intake outcome is the declared seam; the core needs no wrapper alternative | 1 |
| MC04 D.2 (acceptance lineage and migration/replay policy) | Retained; adds immutable authority, deploy audit and `netTag` | 4, 9 |
| MC05 1.1, 1.2 (hash-order vectors and allowed verifiers; reject altered fields) | Mechanism replaced: test vectors and mutations for Π_P, intent digest v2 and genesis body v2; certificate keys are circuit constants | 7 |
| MC05 3.1 (absent-proof and intent-invalid tests; connect MC03 artifacts through the MC04 adapter) | Mechanism replaced: the tests run against the fused step relation; no MC03 artifact is an input | 2, 5 |
| MC05 4.1 (real evidence only for supported profiles) | Retained | 5 |
| MC05 D.1 (signed-intent verification location) | Mechanism replaced: in-circuit JubJub Schnorr, with fit measured by E2 | 2, 7 |
| MC06 3.1 (duplicate-predecessor, policy, fan-in, amplification controls) | Retained, applied to heads | 11 |
| MC06 3.2 (verify real branch proofs and joined history) | Mechanism replaced: ledger-atomic split and join; branch proofs only for off-ledger branches | 11, 12 |
| MC06 4.1 (competing-branch and unavailable-handoff recovery) | Retained; adds `Reclaim` | 12 |
| MC06 D.1 (retained-artifact successor and join probe) | Mechanism replaced: same-contract realization on ledger 8, cross-contract on ledger 9 | 11, 12, 13 |

### 4.6 Integration amendment and crosswalk

- **`openspec/PCD-INTEGRATION-2026-09-11.md`.** This is the human amendment, with status "S2, specified-only planning revision". It contains:
  - The adopted decision and planning assumption.
  - The §4.1 stage mapping and the §4.3 dependency cut.
  - PCD Stages 0–8 and E1–E5 mapped to stage identifiers and existing task identifiers.
  - A reinterpretation of SP01.4, SP01.5 and SP01.7. It is recorded here because the SP01 document is hash-pinned and is not edited.
  - The §4.5a locked-task dispositions.
  - Reconciliation rows (Recon) superseded or amended: F0/F0a/F1/F2/F3 rows, P1–P3 recipes, route table rows and the RP02 pre-MC03 gate.
  - The §4.5 supersession table.
  - The decision register (§4.9).
  - The stop conditions.
  - The review record.
- **`openspec/sprints/pcd-integration.json`.** This is the machine twin. Its keys:
  - `schemaVersion` and `status`.
  - `stageMapping`: PCD stage to stage id and task ids.
  - `experiments`: E1–E5 to stage id and task id.
  - `supersessions`.
  - `dependencyTracker`: item, needed-by stage, status, class and observed date.
  - `decisions`: id, question, default, owner, status.
  - `lockedTasks`: package, task id, disposition and governing requirements.
  - `sourceDigests`: REPORT.md and PCDR, pinned after corrections.
- **Validator additions in `verify.py`, which loads `pcd-integration.json`:**
  1. Every referenced stage id and task id exists.
  2. `mandatory`'s transitive stage prerequisites exclude `f0a`, `f1-fixtures`, `f1` and `f2`. The core stays independent of recursion.
  3. `release` transitively requires `f2`. Certificates stay in release.
  4. Crosswalk `sourceDigests` match.
  5. Every `lockedTasks` entry names an original task in `package-task-map.json`.
- **Test additions in `test_verify.py`.**
  - `setUp` also copies the paths in the crosswalk's `sourceDigests`. Otherwise every test fails on a missing file.
  - Three mutation tests with their expected diagnostics:

    | Mutation | Expected diagnostic |
    |---|---|
    | `f3` requires `f2`, changed in both the Register stage and the SP09 gate | `PCD core depends on certificate stage` |
    | `release` loses `f2`, changed in both the Register stage and the SP12 gate | `PCD release lost certificate stage` |
    | A crosswalk stage mapping names task `SP99.9` | `Unknown PCD crosswalk reference` |

  - The first two mutations must change the gate too; otherwise the existing check "Task admission differs from RP prerequisite graph" fires first.

### 4.7 Sprint contracts

- **SP01 document.** Not edited, because it is hash-pinned by the loan design binding and consumed by the AFK work. The amendment carries its reinterpretation.
- **SP04.**
  - Goal: "Decide whether bounded native certificates work on the ledger".
  - Stage purposes follow §4.1.
  - Task titles and bodies:
    - SP04.1 becomes certificate-relation authorship after Stage 0.
    - SP04.2 becomes independent non-loan certificate fixtures.
    - SP04.3 becomes E3 on a `ledger-10` devnet with all negative controls and the fee measurement.
  - The report-informed refinement bullets are rewritten to match. No in-circuit pairing language remains.
  - The exit gate becomes "E3 passes for the pinned `ledger-10` build, or certificate status is recorded blocked; the core is unaffected".
- **SP06.**
  - Goal: "Produce and independently verify an off-ledger segment certificate".
  - SP06.1 becomes review of the segment relation.
  - SP06.2 becomes E5 at 1, 10 and 100 steps.
  - SP06.3 becomes fresh-process verification and ledger import.
  - Commands move off the retired `native-ivc-r3/successor` root to a path frozen at `native-path-freeze`.
  - The exit gate keeps "no general language completion claim".
- **SP09.**
  - Full-completion dependencies become SP03, SP05, SP07, SP08.
  - SP09.1 becomes "Close the ledger-anchored atomic core (F3)": Stage 0 artifacts in use, E2 fit, E1 on Preview, deploy audit, verification-enabled Preview acceptance of loan and swap steps. The SP06 input sentence is removed.
  - SP09.2 adds the claim discharge map, genesis and the property certificate.
  - SP09.3 replaces revocation with forward-declared migration and `Pause`, adds block-time freshness, and adds digest v2 modes.
  - SP09.4 adds head-discipline and section invariants to correspondence.
  - SP09.5 keeps Preview requalification. It notes that migration Preview evidence depends on ledger 9 at Preview.
  - The file map and packet boundaries follow.
- **SP10.**
  - SP10.1 gets the operator mapping and the reviewed rules for interleaving, messaging and Pending.
  - SP10.2 gets recipient-keyed handoff.
  - SP10.3 gets same-contract split/join and cross-contract release/reclaim on ledger 9.
- **`openspec/sprints/README.md`.** Update the delivery table and entry prose: SP06 needs F1 and path freeze; SP09 no longer needs SP06; SP12 needs SP06.
- **Report lessons.**
  - LR13's `positive` and `negative` text is rewritten to the new seam: Stage 0 → path freeze → certificate fixtures → E3 → E5, and the ledger-anchored core F3 with E1/E2.
  - Its task list and sources stay unchanged, so sprint `reportLessons` stay valid.
  - LR14 gains head discipline and the claim discharge map in its positive text.
- **Asset study (`asset-study.json`, AS09).** Its conditional SP04.2/.3 note is updated to certificate fixtures.

### 4.8 Narrative documents

- **`ROADMAP.md`.**
  - Sprint table rows for SP04, SP06, SP09 and SP10.
  - Track sentence: the core track is SP01 F0 → SP09.1, and the certificate track is SP04 → SP06.
  - RP02 line: design settled, certificate route pending `ledger-10`.
  - MC03–MC06 checklist wording.
  - Execution mermaid: Stage 0 → core F3 with E1/E2; certificate branch F0A → F1 → F2 → release.
  - The F1/F3 paragraph (line 200) drops in-circuit pairing and the sentence that MC03 supplies the terminal proof for F3.
  - The existing PCD paragraph points to the integration amendment.
- **Charter (`MORIARTY-COMPLETION-PROGRAM.md`).**
  - Package table (§4.3) and MC03 description.
  - The lines saying MC04 and MC05 extend MC03 evidence (Charter:85-86).
  - The line saying MC04 first probes the retained MC03 proof interface (Charter:296).
  - The verifier-interface inspection paragraph (Charter:343-346).
  - The campaign ceiling stays k ≤ 17, 8 GiB and two CPU jobs for every campaign. A note records that certificate campaigns need k 18–19 on the measured evidence, so they need a reviewed resource amendment (decision 6).
  - Historical k17 rows remain as history. Charter:87 and :279 are retained.
- **`openspec/ROADMAP-REFINEMENT-2026-09-09.md`.** One note: the 21-stage graph is amended by the PCD integration.
- **`REPORT-RECONCILIATION-2026-09-07.md` (Recon).** Not edited, because it is hash-pinned. Superseded rows are listed in the amendment.
- **PCDR.** Status becomes "Adopted into the OpenSpec roadmap by the PCD integration amendment; specified-only". Apply the corrections in §4.10.
- **MC03–MC06 `README.md`.** One amending-change line each. Affected scenarios gain the in-place notes from §4.5.
- **MC04 and MC05 `design.md` and unlocked `tasks.md` prose.** The Dependencies lines, and MC04's prose note requiring the retained MC03 verifier interface, change to match §4.3.

### 4.9 Decision register defaults

Decisions 1–5 and 7 carry defaults flagged for the user's confirmation, recorded with status `default-pending-confirmation`. None of them weakens a gate. Decision 6 has no default and is recorded as `user-decision-required`.

1. **Intent modes.** Exact-head and outcome modes as in requirement 7. Cross-instance solver route choice is owned by SP08 with MC05.
2. **Revocation and upgrade.** Immutable keys, migration under the declared or unanimous branch, and an optional principal-threshold `Pause` (requirement 9). This follows the report's default. The report's alternative, an all-principal committee with Midnight's update delay, stays available only as a Π_P-declared option for long-lived agreement profiles, after review.
3. **Property certificate.** Checked by the deploy audit as governed registration (requirement 10).
4. **Cross-contract calls.** Allowed for `Release`, `JoinFrom`, `Migrate`, `ImportFrom` and `Reclaim` from Compact 0.33 on ledger 9. For these entry points only, this supersedes the architecture page's statement that cross-contract calls stay outside the initial Core (`wiki/moriarty-architecture.md:143`). The same page's PCD section already lists them, so the contradiction is recorded within that page. External non-Moriarty calls stay excluded.
5. **Operators.** Shared-state interleaving, asynchronous messaging and Pending are redesigned under head discipline as an SP10 design decision before any campaign advertises them.
6. **Certificate k bound.** No default. This changes a resource ceiling, which the user must approve through a reviewed `resourceAmendments` entry. The evidence supports k = 18 at about 4.1 GiB and k = 19 at about 7.8 GiB against an 8 GiB ceiling. Until approval, every campaign keeps k ≤ 17 and certificate stages cannot be admitted.
7. **Fixed MC03 relation.** Default: retire the fixed 54-limb table relation and its smaller-encoding retry. Its k17 failure stays failed evidence. The segment certificate replaces it.

The remaining audit decisions are recorded as open, with the proposed owners from the design audit:

- contract layout;
- single `step` entry point;
- join-summary content;
- `requiredClaimRoot` migration;
- per-head lifetime details;
- `now` as a block-time-bounded value;
- shielded settlement;
- commitment hash;
- DUST fee caps;
- `netTag` residual;
- toolchain per network;
- Compact versus hand-written relation;
- claim vocabulary;
- handoff encryption;
- fallible-section placement;
- `ledger-10` certificate soundness items;
- exported history proofs;
- signature griefing.

### 4.10 Corrections to the report, PCD roadmap and wiki

1. **Migration cycle.** Π_old's successor allowlist holds program digests only. The successor records its predecessor address in its deploy-time initial state, not in Π_new. The unanimous-signature branch stays. The deploy audit runs on the counterparty.
2. **Commitment hash.** State commitments use SHA-256 `persistentHash`, following Midnight's upgrade guidance. Poseidon applies only inside certificate relations. The E2 failure path must not switch commitments silently; it records a reviewed decision.
3. **Memory figures.** Retained logs give a one-level peak of 4,168 MiB (VmHWM; `/usr/bin/time` 4,268,316 kB) and a two-level peak of 8,004 MiB (8,206,868 kB). Quote them as about 4.1 GiB and 7.8 GiB. Fix REPORT.md §14.2 and §14.5, the rendered page, PCDR §1, the wiki benchmark table and the decision page.
4. **MC04 row.** Retitle it "MC04 / SP09 ledger correspondence and consumption, with SP04 certificate feasibility". Map compiler correspondence to head discipline and the fused relation.
5. **Entry points.** Add the missing `Reclaim`, which the report defines. Add `Terminate` and optional `Pause`; these two are additions defined by requirements 6 and 9, not report corrections.
6. **Stale-observation control.** Move it from Stage 1 (offline) to Stage 2 (local ledger application).
7. **Stage 3.** Add the custom immutable-authority deploy path and the deploy audit. Its exit becomes "R2–R5 ledger-enforced for one step on Preview".
8. **RP02.** "Design settled for on-ledger history. The certificate route stays conditional on pull request 738 release, k ≥ 18 parameter serving and fee accounting."
9. **`netTag`.** Separates networks but gives no protection against byte-identical replicated deploys. That residual is recorded.
10. **Stage 7 exit and bounds.** Replace "fit k ≤ 20 and 600 s" with the bound a reviewed resource amendment sets (decision 6). Mark the certificate k row in PCDR §8 as requiring that amendment.
11. **Claim-model count.** The report says the claim model exists in four inconsistent forms but lists three. The repository holds at least five vocabularies.
12. **Citations.** The 128-records-per-state bound is at `bounds.json:91`, not in the typed-schemas range cited. The atomic statements do carry `validity`; only ClaimRequirement lacks it.

Wiki updates go through one claude-obsidian ingest transaction:
- The decision page status becomes "adopted into planning".
- Memory figures are corrected.
- The five defaults are recorded as pending confirmation.
- The contradiction between CLM-0131 and cross-contract composition is recorded.

The project memory note is updated in the same step.

### 4.11 Out of scope, flagged

- The AFK change package and its coverage rows.
- The `FOOTGUNS.md` digest refresh. The drift is unrelated to this update and the policy requires a reviewed step.
- The public site copy (`site/src/data/assurance.ts`, `roadmap.ts`, `CONTENT-SPEC.md`), owned by MC08/SP12.
- Design-source schemas `typed-schemas.md` and `bounds.json`. Their changes are a new semantic profile owned by MC01 through extension paths. The amendment lists them as required follow-ups.
- A graphify rebuild and the wiki canvas.
- Design-document follow-ups from the design audit: new threat rows on the security page, the site's fixed "thirteen trust boundaries" count, per-transaction recursive checking in the site content spec, and the README pipeline diagram.
- The AFK F0 record's citation of the seam, deferred with the AFK package.
- `AGENTS.md` routing and the stale SP01 binding digest.
- No commits.

## 5. Validation

1. **OpenSpec.** `openspec validate --all --strict` passes for 11 changes.
2. **Planning validator.** In a temporary copy of the tree:
   - exclude the untracked AFK change;
   - restore `docs/FOOTGUNS.md` from commit `214141c`;
   - run `python3 openspec/sprints/verify.py` and `python3 -m pytest openspec/sprints/test_verify.py -q`.
   - Both must pass, including the new mutation tests.
   - In the real tree, `verify.py` stops at its first failure, so check it in two steps. The real tree must stop at the AFK requirement crosswalk. A copy without the AFK package must stop at the stale FOOTGUNS digest.
3. **Plugin.** `python3 -m pytest plugins/moriarty-dev/tests -q` passes, using fixtures. The baseline had 174 tests; another session later raised the suite to 199. The real Register and `sprints.json` also go through the plugin's `_validate_program` and `_validate_sprints`, and must report exactly the findings they reported before the edits. The CLI status command already fails because `.moriarty-dev/actions.json` is absent, so it is not used.
4. **Site.** The site test still passes, since the sprint count and site data are unchanged.
5. **Vault.** Lint reports 0 issues after the transaction.
6. **Review.** An independent Fable 5.1 audit reviews the changed files against this design and the report. Blocking findings are fixed before the work is reported.

## 6. Risks

- **E1 fails.** Head discipline is replaced by a consumption set. Requirement 3 already specifies the stop.
- **Ledger 9 on Preview slips.** SP09.3 migration and SP10 cross-contract Preview evidence wait. Single-step core acceptance does not.
- **Readers confuse stage purposes with the old identifiers.** Mitigation: purposes are rewritten in the Register itself, and the amendment carries the mapping table.
- **Another session edits the Register or sprint files concurrently.** Mitigation: re-read each file immediately before editing, and apply edits by script against exact anchors.

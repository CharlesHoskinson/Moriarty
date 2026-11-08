# Assurance strategy

Status: CONSENSUS v2.1 (council vote 6/6 ACCEPT at 2026-11-07 21:29Z; text unchanged from DRAFT v2.1; amended 2026-11-08 (3) by the owner decisions in `dissent.md` "Resolved 2026-11-08", "Resolved 2026-11-08 (2)" and "Resolved 2026-11-08 (3)" (§3 node/decider row: Quint aggregation model F20 and Lean composition theorems F21; §4 M2/M5 gates)). Requirements SAS-001..014 (`requirements.md`). Sources: UISEC-012 evidence-class table (`openspec/changes/sherlock-universal-interpreter-design/security-requirements.md:14-24`), astra-1 `assurance-proposed.md`, astra-2 `security-clauses.md`, astra-3 `proposed-text.md`, fable-2 §"Assurance strategy", fable-3 §5.

## 1. The argument

For the qualified profile `sherlock-helper-microstep/1` with committed bounds, let `R(P, s, s')` be the owner-step relation, `Enc` the relation from semantic state to `sherlock-c6-phys/1` carriers satisfying `StepInv`, and `C` the keyed leaf relation with statement `sherlock-leaf-stmt/1`. The obligations, each with its evidence class:

| # | Obligation (universal unless stated) | Finite release evidence required by this change | What discharges the universal claim | What cannot |
|---|---|---|---|---|
| O1 | Given a committed profile, layout and bounds, canonical admission and genesis establish `StepInv` and `Enc` for the initial carriers bound to `prog_d` and `init_d` | initialization cases (task 4.9): canonical genesis admitted and bound; non-canonical, inconsistent `init_d`/`prog_d`, and forged initial carriers refused at the leaf/node boundary | a theorem over the codec and genesis constraints (SAS-009) | host `checkStepInv` |
| O2 | Soundness per rule, given `StepInv(pre)` and a valid image/configuration: every satisfying witness of `C` with `sel[r]=1` decodes to a valid `R` step by rule `r`, preserves `StepInv`, and exposes exactly the statement | hostile raw witnesses by dispatch class with a valid baseline and weakened-constraint controls (SAS-004, SAS-013), refusal observed at the constraint boundary | a theorem, or an exhaustive checked argument over the stated domain (SAS-009) | finite tests alone; `ProofPreimage::check` |
| O3 | Completeness per rule: every valid bounded step has a satisfying witness | differential run on every step of P1, P2 and (after freeze) P3 through the witness stream (SVM-007) plus real proofs | a theorem | model agreement |
| O4 | Composition: the one-hot leaf checks every fragment premise on the shared carriers (no disconnected copies), with `StepInv(pre)` initialization and preservation | SAS-010 directed cases per admitted rule; shared-carrier premise checks (task 4.6) | a theorem | per-rule circuit passes; "one key" by itself |
| O5 | Recursion: node links `post_d = next pre_d`, `h` arithmetic bounded, authenticated genesis, authorized keys | SRC-004/005 raw witnesses at the node boundary and through the final pairing; flipped-proof refusal; nonmember-key refusal under F | a proof or recorded security argument for the IVC relation and the genesis construction | `ToyLeaf` tests (`sherlock_ivc.rs:68-72`) |
| O6 | Terminal closure: decider pins keys, pairs, derives outcome/result | SRC-006 mutations with fresh witnesses; 70-lane record labelled existing-proof binding | a proof or recorded security argument for the decider relation and the pairing | outer-only success |
| O7 | Target (scope-limited to the observed chain instance and operation): result bound to chain instance, call, key, effects; unique consumption | A6-shape local acceptance, A8p/A9p/A10p manifest-bound predicates on Preview (SAS-007, SMN-004) | a scope-limited target observation; no universal claim exists for O7 | any `ContractCall` at the address; stagenet evidence for a mainnet claim |

A formal proof names its definitions and premises (SAS-009). Finite tests give finite correspondence on their declared domain. O2 and O3 are universally quantified; no finite hostile or differential suite closes them. The release gates for this change are therefore *finite* gates with the premise `StepInv(pre)` stated: hostile classes covered, directed cases run, differential agreement on P1-P3. The universal obligations stay open and labelled until a theorem artifact exists (astra-2). No class substitutes for another (UISEC-012).

## 2. Evidence classes and labels

| Class | Required record (SAS-001) | Limit |
|---|---|---|
| Source inspection | path, hash, definitions, assumptions | authored behaviour, not execution |
| Model execution (K, Quint) | model/tool version, bounds, seed or exhaustive claim, reached actions, trace | that instance only; digests, binding, decider soundness are premises |
| Differential execution | oracle provenance, complete ordered observations, projection, first divergence (SAS-005) | finite correspondence |
| IR preprocessing / witness execution (`ProofPreimage::check`, implemented by `preprocess`, `repos/midnightntwrk/midnight-ledger/zkir/src/ir.rs:111-116`; `c6_dispatch_consumer.rs:244-285`) | baseline, mutation site, failure class, scope label "nonproving" | no constraint-rejection, cryptographic or chain claim; this is the class of every current fragment and dispatch receipt |
| Constraint check (keyed relation, synthesized and verified) | baseline, mutation site, reached stage, failure class (SAS-003) | no cryptographic-binding or chain claim beyond the key |
| Native cryptography | proof/VK/SRS/statement/backend identities, independent verification (SRC-010), invalid controls | no ledger entitlement |
| Target observation | chain instance (genesis, specVersion, ledger rev), manifest, call, finalized block, pre/post state, effects (SMN-004) | that target and operation only |
| Formal theorem | statement, premises, domain, checkable artifact | encoded definitions only |

Every record also carries the dependency fingerprint set: HEAD, `Cargo.lock` hash, `target-pins.json` row, key manifest hashes, chain identity (SAS-002).

## 3. Formal versus test coverage

| Layer | Formal today | Tests today | Added by this change |
|---|---|---|---|
| Reference semantics | K `models/sherlock/k/main.k`; Quint `universalMachine.qnt`, `kProfileMachine.qnt`; `model_package.py projection` (UIREQ-021) | 216 prefixes, 5,355 records (`model-coverage.md`) | none new; relabel as class "model execution" |
| Carrier codec | Quint C6 codec models (4,697 checks, `c6-positive-whole-host`) | `sherlock_digest_parity` | parity on every step of P1-P3 at new pins (SVM-007) |
| Dispatch | none | per-rule `ProofPreimage::check` (quadratic test) | Quint model of the one-hot selector (exactly one bit per enabled reachable class; zero bits and identity under `enable=0`) checked against the owner event stream; partition receipt with a completeness/congruence argument or the heuristic label (SAS-013) |
| Fragments / leaf | none | per-fragment `ProofPreimage::check` receipts | hostile raw witnesses per class (A4/B002 cases: inactive-slot garbage, unreachable node, foreign codeId, tag/layout mismatch, write below activeLength, nondeterministic post-image, publication padding); real keygen/prove/verify |
| Node / decider (aggregation: linear chain and tree, SRC-017) | none | `sherlock_ivc`, `sherlock_gp` (stand-in leaf), 70-lane mutation | (Amended 2026-11-08 (3).) **F20, class "model execution":** Quint `models/sherlock/quint/aggregation.qnt` + `aggregation_test.qnt` (constants `TREE`, `S`; pattern of RPP task 1.1), invariants I1 linkage, I2 height monotone, additive and below 2^64, I3 exactly-once coverage, I4 identity and padding neutrality, I5 schedule independence, I6 unique genesis, I7 replay set, I8 program fixity, I9 shape binding, I10 liveness witness, fixed keys, Q-C leaf-worker scheduler under the 48/100 GB caps; one seeded mutant per invariant, every mutant caught, `TREE` false and true, S in {1, 2, 4, 8}; premises: digests, binding, in-circuit verifier and decider soundness. **F21, class "formal theorem":** Lean `clean/fork/Clean/Sherlock/Aggregation.lean` + `AggregationAxioms.lean` in `SherlockSpec`: L1 segment soundness for every program (root statement implies a valid execution of the general machine), L2 associativity of linkage with the identity as unit, L3 padding neutrality, L4 exactly-once coverage, L5 height arithmetic without wrap (and the insufficiency of the weaker check pair), L6 decider pinning, L7 independence from shape, depth and S, L-D genesis scale-by-bit, L-E `assert_lower_than_fixed` for a non-power-of-two bound; zero `sorry`; `#print axioms` within the declared premises (in-circuit verifier, accumulator, pairing, digest injectivity on the carrier domain, the reference step relation as a parameter). Both gate 2.3, 4.2 and 5.7 (design §3.6); the council review of design plus results (task 2.14) is an input, not evidence. SRC-004/005 raw-witness tests through the pairing stay as the constraint-check class. SAS-009 is triggered: the release report names F21's theorems, premises and source closure |
| Prover backend (GPU, SRC-018) | none | none | Differential tests of the Rust GPU field kernels, MSM and NTT against the CPU path on random and edge vectors (A24-A26, class "constraint check" of a pure function: bit-identity, no protocol claim); proof bytes identical under a fixed seed and stock verification (A27, class "native cryptography"); SAS-008 is triggered when the `gpu` feature is enabled for a qualified candidate: declared traces equal baseline, refusal set re-run |
| Ledger | none | A6 local acceptance, A9 refusals | A6'/A9' on local 9.1.0.1 and Preview with manifests |

## 4. Gates per milestone (see `tasks.md`)

| Milestone | Gate (predicates) | Evidence class required |
|---|---|---|
| M0 Re-arm | F4, F11 re-earned; STOP recorded as bookkeeping (SWF-006) | factory |
| M1 Land change | F14 traceability, F15 pins (static); all new rows `unimplemented.sh` | factory |
| M2 Re-pin, measure, fix, re-key (amended 2026-11-08 (3)) | A1 build; F18 memory guard before any further heavy run; F19/A15 C0 profile of the node chain, A17 thread sweep, A23 concurrency probe, F22 tree-node rows; F20 Quint and F21 Lean before 2.3; council review record (2.14); gate G-A JSON; A16 node step at k=19 (<= 60 s, 16 GB) after the cut; SRC-009 regression record (A11); A6p local acceptance + five refusals; SMN-003 template diff | factory; measured resources; model execution + formal theorem; native cryptography + local ledger |
| M3 Fragments and receipt | SVM-008 zero gaps for the frozen inventory `R`; SAS-013 partition receipt; U023 scenario 1 | IR preprocessing / witness execution (nonproving; labelled as such) |
| M4 Universal leaf | lane/row measurement; SRC-013 decision; real keygen/prove/verify of every leaf of `entry 7;`; SRC-003, SVM-002 and initialization hostile cases at the keyed boundary | constraint check (keyed) + native cryptography |
| M5 Local chain and the throughput gates (amended 2026-11-08 (3)) | leaves -> node (or tree) -> decider for `entry 7;`, fact(3), list123; SRC-004/005/006 negatives; `verify_chain` (SRC-010); `validate_call` (A6'); A18 `entry 7;` under 20 min (G-B); A19/A19b fact(3) and list123 under 2 h, 48 GB per process, 100 GB aggregate (G-C, blocks OP-PV-DEPLOY); A22 tree fold if G-A built it; A21/A27 GPU bit-identity if the track is open | native cryptography + local ledger; measured resources under seeded caps |
| M6 Preview | OP-PV-* operations; A4'/A8'/A9'/A10' preview rows manifest-bound; U023 scenario 2 records P1-P3; C4 flips; synthetic-rollup B rows on Preview (FSYN data; mandatory for the synthetic-rollup Preview gate, outside C4; amended 2026-11-08 (2)) | target observation |
| M7 Mainnet readiness | T4 tier exists; rollback rehearsal receipt; `:269` amended (done 2026-11-08); owner authorization per operation; synthetic-rollup B*m rows after their Preview rows | target observation (mainnet) |

## 5. What does not count

Stand-in leaf bundles (`crates/mori-recursion/src/standin.rs`); receipts from the lost host or only under `target/`; `checkStepInv` or `ProofPreimage::check` for a constraint or proof claim; a Kore, K or Quint result for circuit correspondence; `ToyLeaf` IVC tests for C6 semantics; public-input flips of an existing proof as malicious-witness soundness; a `ContractCall` at the address not bound to the manifest; a `status.json` pass whose fingerprints are stale; a systemd `Result=success` when the `heavy` line says exit 4; reviewer verdicts; aggregate pass counts as completion; stagenet (specVersion 2000000) evidence for a Preview or mainnet claim; a council vote.

## 6. Known defects to fix in the assurance tooling

1. `qa_mutation::run_once` maps synthesis errors to `Rejected`; `run_suite` counts panics as rejected; `assert_sound` lacks a zero-panic requirement (`crates/mori-backend/src/qa_mutation.rs:458-486,640-650`; `tests/soundness_mutation.rs:227-245`). Fix under SAS-003 before any new campaign is cited.
2. `check-a8.sh` accepts any `ContractCall`; `check-a10.sh` selects by mtime (SAS-007).
3. `status.py` merges old entries under a new HEAD without fingerprints (SAS-002).
4. Dispatch tests convert every checker error to a boolean refusal (`c6_dispatch_tests.rs:85-87`); the receipt must keep the failure class (SAS-003).

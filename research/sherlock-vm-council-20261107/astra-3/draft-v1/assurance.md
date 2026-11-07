# Assurance strategy

Status: DRAFT v1. Requirements SAS-001..014 (`requirements.md`). Sources: UISEC-012 evidence-class table (`openspec/changes/sherlock-universal-interpreter-design/security-requirements.md:14-24`), astra-1 `assurance-proposed.md`, astra-2 `security-clauses.md`, astra-3 `proposed-text.md`, fable-2 §"Assurance strategy", fable-3 §5.

## 1. The argument

For the qualified profile `sherlock-helper-microstep/1` with committed bounds, let `R(P, s, s')` be the owner-step relation, `Enc` the relation from semantic state to `sherlock-c6-phys/1` carriers satisfying `StepInv`, and `C` the keyed leaf relation with statement `sherlock-leaf-stmt/1`. The obligations, each with its evidence class:

| # | Obligation | Evidence class that can close it | What cannot |
|---|---|---|---|
| O1 | Admission and genesis establish `StepInv` and `Enc` for the initial carriers | constraint check + hostile raw witnesses; later a theorem | host `checkStepInv` |
| O2 | Soundness per rule: every satisfying witness of `C` with `sel[r]=1` decodes to a valid `R` step by rule `r`, preserves `StepInv`, and exposes exactly the statement | hostile raw witnesses by dispatch class (SAS-013), weakened-constraint controls (SAS-004); later a theorem (SAS-009) | finite differential tests alone |
| O3 | Completeness per rule: every valid bounded step has a satisfying witness | differential run on every step of P1-P3 through the witness stream (SVM-007) | model agreement |
| O4 | Composition: the one-hot leaf checks fragment premises on the same carriers (no disconnected copies) | SAS-010 directed cases; code review of the gating | per-rule circuit passes |
| O5 | Recursion: node links `post_d = next pre_d`, `h` arithmetic bounded, canonical genesis, authorized keys | SRC-004/005 raw witnesses through the final pairing; flipped-proof refusal | `ToyLeaf` tests (`sherlock_ivc.rs:68-72`) |
| O6 | Terminal closure: decider pins keys, pairs, derives outcome/result | SRC-006 mutations with fresh witnesses; 70-lane record labelled existing-proof binding | outer-only success |
| O7 | Target: result bound to chain instance, call, key, effects; unique consumption | A6-shape local acceptance, A8/A9/A10-shape manifest-bound predicates on Preview (SAS-007, SMN-004) | any `ContractCall` at the address; stagenet evidence for a mainnet claim |

A formal proof names its definitions and premises (SAS-009). Finite tests give finite correspondence on their declared domain. O2 and O3 are universally quantified; no finite hostile or differential suite closes them. The release gates for this change are therefore *finite* gates with the premise `StepInv(pre)` stated: hostile classes covered, directed cases run, differential agreement on P1-P3. The universal obligations stay open and labelled until a theorem artifact exists (astra-2). No class substitutes for another (UISEC-012).

## 2. Evidence classes and labels

| Class | Required record (SAS-001) | Limit |
|---|---|---|
| Source inspection | path, hash, definitions, assumptions | authored behaviour, not execution |
| Model execution (K, Quint) | model/tool version, bounds, seed or exhaustive claim, reached actions, trace | that instance only; digests, binding, decider soundness are premises |
| Differential execution | oracle provenance, complete ordered observations, projection, first divergence (SAS-005) | finite correspondence |
| Constraint check (nonproving) | baseline, mutation site, reached stage, failure class (SAS-003) | no cryptographic or chain claim |
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
| Node / decider | none | `sherlock_ivc`, `sherlock_gp` (stand-in leaf), 70-lane mutation | Quint model of chain invariants (linkage, height monotone and bounded, unique genesis, replay set) in `models/sherlock/`, pattern of RPP task 1.1; SRC-004/005 raw-witness tests through the pairing |
| Ledger | none | A6 local acceptance, A9 refusals | A6'/A9' on local 9.1.0.1 and Preview with manifests |

## 4. Gates per milestone (see `tasks.md`)

| Milestone | Gate (predicates) | Evidence class required |
|---|---|---|
| M0 Re-arm | F4, F11 re-earned; STOP recorded as bookkeeping (SWF-006) | factory |
| M1 Land change | F14 traceability, F15 pins (static); all new rows `unimplemented.sh` | factory |
| M2 Re-pin and re-key | A1 build; SRC-009 regression record; A2'-A4' (IVC node, decider) at new pins; A6' local acceptance + five refusals; SMN-003 template diff | native cryptography + local ledger |
| M3 Fragments and receipt | SVM-008 zero gaps for P1-P3; SAS-013 partition receipt; U023 scenario 1 | constraint check |
| M4 Universal leaf | M1' lane/row measurement; SRC-013 decision; real keygen/prove/verify of every leaf of `entry 7;`; SRC-003 hostile cases; SVM-002 hostile cases | native cryptography |
| M5 Local chain | leaves -> node -> decider for `entry 7;`, fact(3), list123; SRC-004/005/006 negatives; `verify_chain` (SRC-010); `validate_call` (A6') | native cryptography + local ledger |
| M6 Preview | OP-PV-* operations; A4'/A8'/A9'/A10' preview rows manifest-bound; U023 scenario 2 records P1-P3; C4 flips; B rows re-earned (non-gating) | target observation |
| M7 Mainnet readiness | T4 tier exists; rollback rehearsal receipt; `:269` amended; owner authorization | target observation (mainnet) |

## 5. What does not count

Stand-in leaf bundles (`crates/mori-recursion/src/standin.rs`); receipts from the lost host or only under `target/`; `checkStepInv` or `ProofPreimage::check` for a constraint or proof claim; a Kore, K or Quint result for circuit correspondence; `ToyLeaf` IVC tests for C6 semantics; public-input flips of an existing proof as malicious-witness soundness; a `ContractCall` at the address not bound to the manifest; a `status.json` pass whose fingerprints are stale; a systemd `Result=success` when the `heavy` line says exit 4; reviewer verdicts; aggregate pass counts as completion; stagenet (specVersion 2000000) evidence for a Preview or mainnet claim; a council vote.

## 6. Known defects to fix in the assurance tooling

1. `qa_mutation::run_once` maps synthesis errors to `Rejected`; `run_suite` counts panics as rejected; `assert_sound` lacks a zero-panic requirement (`crates/mori-backend/src/qa_mutation.rs:458-486,640-650`; `tests/soundness_mutation.rs:227-245`). Fix under SAS-003 before any new campaign is cited.
2. `check-a8.sh` accepts any `ContractCall`; `check-a10.sh` selects by mtime (SAS-007).
3. `status.py` merges old entries under a new HEAD without fingerprints (SAS-002).
4. Dispatch tests convert every checker error to a boolean refusal (`c6_dispatch_tests.rs:85-87`); the receipt must keep the failure class (SAS-003).

# Slice C: wrap, settlement and operations

Independent EARS draft, 2026-11-15. This draft states required behavior. It does not certify implementation completion.

## Source names and evidence limits

Paths below use these aliases. A suffix after an alias is relative to that directory. Line numbers refer to the inspected files.

| Alias | Path |
|---|---|
| ROOT | `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec` |
| MAIN | `/home/hoskinson/Projects/Moriarty` |
| WT | `/home/hoskinson/Projects/Moriarty-wt` |
| CACHE | `/home/hoskinson/.cache` |
| FINAL | `ROOT/openspec/changes/sherlock-processor-final-20261111` |
| COMPLETE | `WT/spec-audit/openspec/changes/sherlock-completion-20261115` |
| GOV | `WT/t22-governance/openspec/changes/sherlock-governance` |
| T22 | `WT/t22-governance/research/sherlock-t22-governance-20261114` |
| T20 | `WT/t20-batched/research/sherlock-t20-batched-20261114` |
| T17 | `WT/t17-preview/research/sherlock-t17-preview-20261114` |
| MAP | `WT/t23-args/research/sherlock-t23-args-20261115/MAP.md` |
| LOOP | `MAIN/research/sherlock-tested-design-20261113/LOOP.md` |
| DEC | `ROOT/research/sherlock-final-design-20261111/DECISIONS.md` |
| WIKI | `MAIN/llm-wiki/wiki` |
| OPENVM | `CACHE/m7-wrap/src/openvm-rekey-chips` |

The brief supplies OPENVM revision `dcf1b4c`. The revision was not checked with git. Code citations identify inspected paths and lines.
No tests, builds, provers, network requests or transactions ran for this draft. No wallet or key file was opened.
Verification lines name existing checks or records. `MISSING` means no matching complete verification was confirmed in the inspected sources.
Historical run results retain their original scope. A decider, synthetic shape or T10m key does not qualify the final R_SL key set.

## 1. Capabilities

Existing IDs retain their identity. Reworded entries below revise the same requirement, rather than create a second requirement.
Existing clauses remain required unless this draft explicitly identifies their replacement. Unlisted requirements of the referenced capabilities remain in force.
Slice B owns the full proving and security obligations referenced here.

| Capability | ID prefixes | Purpose | Components | LOOP rows |
|---|---|---|---|---|
| sherlock-l-and-wrap | LW | Produce one ledger-readable BLS12-381 wrap with the complete statement binding. | `L_Fr` interface, wrap verifier, decoder, log replay, call vector | T3, T4, T9, T10a, T10, T10m, T10q, T12, T25 |
| sherlock-derived-gates | DG, RKY references | Enforce circuit, row, VK and security limits on actual keys. | keygen, D3, serialized VK measurement | T4, T12, T25 |
| sherlock-midnight-side | MID | Settle through the unchanged ledger 9.1.0.1 verifier. | proof framing, installed entry keys, transaction limits | T9, T10a, T10, T17, T23 |
| sherlock-suite-settlement | STL | Map proved calls to acceptRun and compare Preview outcomes with native outcomes. | argument builder, per-program ledger, Part A runner, settlement report | T17, T18, T23, T24, T25 |
| sherlock-ledger-contract | LDG | Commit operator jobs by checkpoint and prove claim membership off chain. | SherlockLedger, claims tree, consumer, foreign registry | T8b, T19, T20, T22 |
| sherlock-anchor-chunks | ANC | Pin slot 15 and keep oversized calls atomic within the fixed chunk cap. | wrap templates, anchor, planner, native prologue | T9, T10q, T12, T20, T23 |
| sherlock-era-table | ERT | Keep sealed era records with one accepting era. | era registry, deployment readback, accepting intervals | T22 |
| sherlock-era-ledger | ELD | Bind each sealed program ledger to its era. | sealed ledger, native reads, claimed active call | T12, T17, T20, T22 |
| sherlock-governance-verbs | GOV | Govern eras through rotated Fenrir operator keys. | quorum, announce, activate, retire, rotate, attest | T22, T15 |
| sherlock-consumer-read | CRR | Preserve claim verification and once-only use across eras. | consumer profiles, lineage, nullifier, safe-era read | T20, T22 |
| sherlock-gpu-wrap | GPW, PRD references | Prove on owned hardware with a CPU fallback and measured resource use. | wrap kernels, daemon, one-use slots, profile | T11, T14, T14b, T16 |
| sherlock-zero-knowledge | ZK references | Enforce case A and disclose only the blinded settlement proof. | trust boundary, test seeds, deployment salt, helper refusal | T11, T16, T17, T23 |
| sherlock-client-operations | CSDK, COPS | Provide SDK/CLI settlement, wallet privacy, receipts and safe recovery. | wallet, runner, prover adapter, diagnostics, resume | T3, T14b, T16, T17, T19, T21, T23 |

`CSDK` and `COPS` are proposed prefixes for requirements without an identified existing ID. Their requirements are draft additions, not new owner rulings.
The 62 completion requirements are referenced by their existing IDs. This slice carries STL-001..006 and the C-facing RKY and ZK entries.

## 2. EARS draft

### sherlock-l-and-wrap

#### LW-002 — Verify the terminal STARK
The wrap shall verify `L_Fr` natively in Fr under the frozen key and transcript schedule.

Scenario: GIVEN an `L_Fr` proof with another key, WHEN the wrap checks it, THEN verification fails.
Verification: `ROOT/tools/sherlock-wrap/tests/i5_whir.rs:55`, `i5_whole_verifier_native_equals_stock`, and `:99`, `i5_whole_verifier_mockprover`. Final-key run MISSING.

#### LW-003 — Public call vector
The wrap shall expose exactly 1,026 call-vector words, with `binding_input` at lane 0, `comm` at lane 1 and unused lanes constrained zero.

Scenario: GIVEN an unused lane, WHEN its zero value changes, THEN the proof fails verification.
Verification: `ROOT/tools/sherlock-wrap/tests/i7_decode.rs:79`, `i7_decode_native_equals_stock`, and `:104`, `i7_decode_gadget_mock`.

#### LW-004 — Complete replay
The wrap shall enforce canonical values, bounded reductions, reservation entries, offsets, padding and native replay of every field-log entry, including PATH.

Scenario: GIVEN a valid log, WHEN a mapped digest or replayed PATH value changes, THEN the wrap refuses it.
Verification: `ROOT/tools/sherlock-wrap/tests/i7_decode.rs:79,104`; `WT/e5-wrap/research/sherlock-e5-wrap-20261112/RESULTS.md:337` records replay refusals. Full final-key PATH coverage MISSING.

#### LW-005 — Anchor and lane binding
The wrap shall bind the anchor read, anchor outputs, entry layout and public lanes to the proved log and mode.

Scenario: GIVEN another program's valid proof, WHEN it targets this program's anchor, THEN the call is refused.
Verification: T17 R4, `T17/RESULTS.md:25`. Slot-15 final-key repetition MISSING.

#### LW-006 — DAG key commitment
The wrap shall require canonical `X` limbs and equality between `X` and the cached-trace commitment of `L`.

Scenario: GIVEN a valid root proof, WHEN `X` changes independently, THEN wrap verification fails.
Verification: MISSING for the final integration. Existing obligation: `FINAL/specs/sherlock-l-and-wrap/spec.md:54`.

#### LW-007 — Hidden run statement
The settlement path shall publish the hiding run statement without plain program identity, private execution length, memory roots or witness values.

Scenario: GIVEN a private run, WHEN its call vector, transcript and state are inspected, THEN only permitted public values appear.
Verification: T17 privacy scans and positive scanner controls, `T17/RESULTS.md:22`. General zero-knowledge proof remains a slice B dependency.

#### LW-009 — Batched collection statement
The collection settlement path shall publish `[reg_pub, claims_root]` without `shape_d`, `set_root`, `set_pub` or claim count.

Scenario: GIVEN a collection, WHEN acceptCollection executes, THEN the ledger stores one claims root and no member count.
Verification: `WT/t20-batched/tools/sherlock-sdk/t20-ledger.test.mjs:122`. Collection proof binding MISSING, `T20/RESULTS.md:70`.

This supersedes LW-009's older set-root publication, using T20's existing amendment. It does not allocate a new collection requirement.

#### LW-010 — Stable replay identity
The settlement contract shall reject a previously accepted `claim_pub`, independent of proof randomness, partition or schedule.

Scenario: GIVEN an accepted run, WHEN a fresh proof and transaction binding repeat its claim, THEN the contract refuses it.
Verification: T17 R3, `T17/RESULTS.md:26`; schedule identity evidence `WT/t16-elastic/research/sherlock-t16-elastic-20261114/RESULTS.md:66` reaches `L`, not the wrap.

#### LW-012 — Fresh proof blinders
The wrap prover shall use fresh CSPRNG blinders for each production proof.

Scenario: GIVEN one statement, WHEN it is proved twice in production, THEN both proofs verify and their bytes differ.
Verification: `ROOT/tools/sherlock-wrap/src/bin/wrap.rs:79` uses OsRng. EQ-ZK completion remains MISSING here, tracked by ZK-015.

#### LW-013 — Exactly one BLS12-381 wrap
The proving path shall finish with one BLS12-381 `ZkStdLib` wrap of `L_Fr`, while leaves and nodes remain small-field STARKs.

Scenario: GIVEN a proposed second BLS proving layer or custom G4 key, WHEN the path is qualified, THEN qualification fails.
Verification: E5 stock-ledger run, `WT/e5-wrap/research/sherlock-e5-wrap-20261112/RESULTS.md:344,345`; G4 refusal `WT/m7-wrap/research/sherlock-m7-wrap-20261112/RESULTS.md:31`.

#### LW-014 — Keygen input guards
If a reservation exceeds log capacity or a supplied key violates frozen shape parameters, then keygen shall refuse the configuration.

Scenario: GIVEN an oversized reservation, WHEN keygen starts, THEN it produces no usable wrap key.
Verification: MISSING for these keygen refusals. Requirement exists at `FINAL/specs/sherlock-l-and-wrap/spec.md:123`.

#### LW-016 — Qualification on real interfaces
The qualification runner shall measure the complete wrap on real non-NOP logs for the full tree and path C at admitted shapes.

Scenario: GIVEN only a synthetic shape result, WHEN completion is reported, THEN the final qualification remains open.
Verification: historical integrated T10m run `LOOP:31`; final-key full-tree and path-C qualification MISSING.

#### LW-017 — Unchanged verifier
The wrap shall remain readable and verifiable by the unchanged Midnight ledger 9.1.0.1 verifier.

Scenario: GIVEN the serialized wrap key and proof, WHEN the stock verifier parses and verifies them, THEN it accepts without patches.
Verification: E5 ledger run `WT/e5-wrap/research/sherlock-e5-wrap-20261112/RESULTS.md:345`; T17 `RESULTS.md:9`. Final-key run MISSING.

#### LW-022 — WHIR shape refusal
If a verifying-key WHIR shape has `log_rs < k`, then the wrap host shall refuse the shape before synthesizing a cell.

Scenario: GIVEN a shape whose subtraction would underflow, WHEN the host loads it, THEN shape validation refuses it.
Verification: `wl_c31_c52_shape_guards`, Moriarty `95990a6d3`, recorded in `COMPLETE/specs/sherlock-l-and-wrap/spec.md:34`. Integration rerun MISSING.

LW-001, LW-008, LW-015 and LW-018..021 remain dependencies from slice B. The wrap must preserve their root, key and soundness bindings.

### sherlock-derived-gates

#### DG-001 — Circuit limit
The key generator shall refuse every BLS12-381 circuit with `k > 21`.

Scenario: GIVEN `k = 22`, WHEN keygen is requested, THEN keygen refuses before proving.
Verification: guard `ROOT/tools/sherlock-wrap/src/bin/wrap.rs:32`; dedicated refusal test MISSING.

#### DG-002 — Row headroom
The wrap shall fit 85 percent of usable rows at the smallest fitting `k` in `{19, 20, 21}`.

Scenario: GIVEN a k=21 wrap above 1,782,573 rows, WHEN the row gate runs, THEN the gate fails without raising k.
Verification: E5 measured rows `WT/e5-wrap/research/sherlock-e5-wrap-20261112/RESULTS.md:342`; T10m `LOOP:31`. New-key measurement MISSING.

#### DG-003 — Serialized VK size
The wrap verifier key shall contain at most 50,000 bytes in the ledger-readable serialization.

Scenario: GIVEN a larger key, WHEN deployment qualification runs, THEN deployment qualification fails.
Verification: T17 raw VK 1,325 bytes and framed VK 1,353 bytes, `T17/RESULTS.md:17`. Final-key measurement MISSING.

#### DG-010 — Preserve GP targets
If a derived gate fails, then Sherlock shall retain GP-1 through GP-4 without narrowing their required behavior.

Scenario: GIVEN an oversized foreign-proof or PCD case, WHEN qualification fails, THEN that GP target remains open.
Verification: MISSING for an automated policy check. Authority: `LOOP:102`, brief hard rules.

#### RKY-003 — Measure after the single rekey
When T25 completes, the qualification runner shall measure the actual wrap at k=21 against the 1,782,573-row limit.

Scenario: GIVEN changed circuit bindings, WHEN the new keys are generated, THEN old row measurements cannot close the gate.
Verification: MISSING. Reuses `COMPLETE/specs/sherlock-rekey/spec.md`, RKY-003.

#### RKY-004 — Measure the new VK
When T25 completes, the qualification runner shall measure the new serialized VK and confirm the 50,000-byte limit.

Scenario: GIVEN the new wrap key, WHEN the measurement completes, THEN its exact byte count and serialization form are recorded.
Verification: MISSING. Reuses `COMPLETE/specs/sherlock-rekey/spec.md`, RKY-004.

#### RKY-005 — Security on actual keys
When T25 completes, the security calculator shall establish 100 bits for every term and 101 bits for every grindable term.

Scenario: GIVEN a grindable term at 100.9 bits, WHEN D3 evaluates it, THEN qualification fails.
Verification: final-key D3 MISSING. Historical T10m floor 101.0110: `LOOP:31`. G3/G4 and FP-8 remain separate gates, not consequences of D3.

RKY-001 and RKY-002 retain the single-list rekey and committed-input requirements. SQG-014..016 retain the T12 transcript and final-key gates.

### sherlock-midnight-side

#### MID-001 — Stock call verification
The settlement runner shall submit a KZG BLS12-381 PLONK proof with the Blake2b transcript to the entry's stock ledger verifier.

Scenario: GIVEN changed public inputs or a flipped proof byte, WHEN the ledger verifies the call, THEN it refuses the proof.
Verification: `WT/k21-measure/tools/k21-ledger-verify/src/main.rs:27` and negative controls at `:32`; actual wrap evidence E5 `RESULTS.md:345`.

#### MID-002 — Universal wrap entry keys
The deploy tool shall install the same universal wrap VK on every wrap-keyed entry of the applicable era.

Scenario: GIVEN acceptRun and acceptCollection, WHEN their installed VKs are read back, THEN both equal the qualified wrap VK.
Verification: T17 readback `T17/RESULTS.md:17`. Native read keys follow ELD-005 and ERT-008.

This supersedes the older phrase “every exported entry.” T20 and T22 require native keys for read entries.

#### MID-003 — Ledger envelope
The settlement runner shall refuse transactions above 1,048,576 bytes or 65,536 public inputs, while using the 1,026-word wrap statement.

Scenario: GIVEN an oversized serialized transaction, WHEN preflight runs, THEN submission stops.
Verification: MISSING for a combined boundary test. Existing limits: `FINAL/specs/sherlock-midnight-side/spec.md:18`.

#### MID-004 — Network qualification receipts
The qualification runner shall record VK size, proof size, verification time, fees and included deploy/call receipts for the actual wrap.

Scenario: GIVEN an honest Preview call, WHEN it is included, THEN its contract address, transaction hash, block and readback are recorded.
Verification: `T17/RESULTS.md:15` through `:26`. Final-key receipts MISSING.

#### MID-006 — Network identity
The Preview runner shall record the observed genesis, node build and runtime version and verify them against the selected network pins.

Scenario: GIVEN a node on another network, WHEN preflight checks its identity, THEN submission stops.
Verification: `WT/preview-deploy/tools/sherlock-sdk/preview-session.test.mjs:12`; `WT/t23-suite/tools/t23-suite/preview_probe.py:44`. Exact final-run pin MISSING.

This supersedes the stale literal node-build requirement. No new build value is invented. SWF-009 still requires explicit Mainnet authorization.

### sherlock-suite-settlement

#### STL-001 — Arguments from the proved statement
When building acceptRun, the settlement runner shall derive three arguments and the computed `claim_pub` from the proved statement and cross-check public lanes.

Scenario: GIVEN the T17 accepted statement, WHEN the builder runs, THEN its serialized arguments equal the accepted arguments byte for byte.
Verification: `WT/t23-args/tools/t23-suite/test_accept_args.py:134,147,152,156`, `T17Receipt` tests. R_SL proof binding remains MISSING.

The three arguments are `c_p`, `input_pub`, `outcome_pub`. `claim_pub = transientHash([c_p, input_pub, outcome_pub])` is computed by the contract.
The T10/T17 layout has anchor lane 10, claim lanes 20 and 34, and outcome lane 39. Source: `MAP:23` through `:43`.
These lane numbers require renewed checks after slot 15 and era-tail integration. The wrap vector alone does not reveal `c_p` and `input_pub`.

#### STL-002 — Full chain on final keys
When proving a suite call, Sherlock shall prove app, leaf, N1..N3, L, L_Fr and the wrap on the rekeyed universal key set.

Scenario: GIVEN a box-produced proof, WHEN muninn checks it, THEN the stock verifier and ledger 9.1.0.1 verifier accept before KEEP.
Verification: MISSING for final R_SL calls. Historical integrated chain: `LOOP:31`. This reuses completion STL-002.

#### STL-003 — Preview redeployment
When T25 and T12 pass, the deploy runner shall deploy slot-15 SherlockLedger on Preview with the qualified wrap VK.

Scenario: GIVEN a readback VK mismatch, WHEN deployment validation runs, THEN live settlement stops.
Verification: MISSING for final keys and slot 15. T17 readback is a historical baseline, `T17/RESULTS.md:17`.

#### STL-004 — Accepted run state
When acceptRun succeeds, SherlockLedger shall store `runs[claim_pub] = outcome_pub`.

Scenario: GIVEN an included call, WHEN the runner reads contract state, THEN the stored outcome equals the proved outcome.
Verification: `T17/RESULTS.md:19,20`; code `WT/t20-batched/tools/sherlock-sdk/sherlock-ledger.compact:54` through `:58`.

#### STL-005 — Per-call comparison
If Sherlock and native outcomes differ, then the suite report shall record a mismatch for that call.

Scenario: GIVEN all 208 recorded cases, WHEN the report closes, THEN each case has a result and every mismatch is visible.
Verification: `WT/t23-suite/research/sherlock-t23-suite-20261115/T23-DRYRUN.md:11` through `:36`; live proof-to-native comparison MISSING.

Results distinguish pass, refused-as-native, mismatch, blocked with reason, prove-failed and pure/non-settling cases.
A zero mismatch count with zero proved comparisons is not suite success. Source: that dry-run report, line 21.

#### STL-006 — Native Part A baseline
When Part A runs, the native runner shall compare each call of the 33 compilable programs with the prior Preview baseline.

Scenario: GIVEN a changed runtime version, WHEN comparison runs, THEN the report identifies the version boundary and preserves both baselines.
Verification: `WT/t23-suite/tools/t23-suite/parta_diff.py:63` and `parta_report.py:40`; complete rerun evidence MISSING in the inspected report.

#### LPC-008 — Final owned-box Preview demo
When T11 and T12 pass, the prototype runner shall demonstrate an honest call, tamper refusals and a second program on Preview with huginn proving.

Scenario: GIVEN a final-key wrap from huginn, WHEN the prototype submits it, THEN the honest call lands and tampered controls receive node refusals.
Verification: MISSING. Historical T17(a) run and remaining hook: `T17/RESULTS.md:9,49`. Reuses completion LPC-008.

#### LPC-009 — Batched validium settlement
When T17(b) passes, the validium runner shall settle at least two Preview batches and refuse an invalid batch and a stale root.

Scenario: GIVEN a batch with an old input root, WHEN the L2 contract checks it, THEN it preserves the current root.
Verification: MISSING. Existing required run: `COMPLETE/specs/sherlock-loop-completion/spec.md`, LPC-009, and `LOOP:41`.

The demonstration must also compare proof size across batch counts. No completed validium run is claimed.

#### CSDK-001 — Per-program deployment binding
The settlement SDK shall target the ledger whose constructor anchor equals the proved program's `c_p`.

Scenario: GIVEN program B and program A's ledger, WHEN settlement is attempted, THEN anchor validation refuses it.
Verification: T17 R4, `T17/RESULTS.md:25`; argument mapping `MAP:31,74`.

This refines N-11 for the client. The T23 mapping proposes one deployment per `(contract, circuit)` R_SL program.

#### CSDK-002 — Refused calls
When the native call refuses before proving, the suite runner shall record refused-as-native without submitting acceptRun.

Scenario: GIVEN a call with no native proof preimage, WHEN the suite processes it, THEN no settlement receipt is claimed.
Verification: dry-run refusal table `WT/t23-suite/research/sherlock-t23-suite-20261115/T23-DRYRUN.md:32`; live parity verification MISSING. Authority: `MAP:75`.

#### CSDK-003 — Private builder inputs
The settlement SDK shall keep statement preimages inside the user's proving trust domain and return only public settlement arguments to the transaction runner.

Scenario: GIVEN a remote owned prover, WHEN argument construction finishes, THEN the coordinator receives public words without the private bundle.
Verification: MISSING for the complete box protocol. Required interface gap: `MAP:85`.

#### CSDK-004 — Reuse only matching state
When reusing a native Preview instance, the runner shall require its current state to equal its deployment-block state.

Scenario: GIVEN a previously mutated instance, WHEN Part A plans the run, THEN the plan selects a fresh deployment.
Verification: implementation `WT/t23-suite/tools/t23-suite/parta.py:56`; automated refusal test MISSING. Confirmation: `WIKI/log.md:4107`.

### sherlock-ledger-contract

#### LDG-001 — Bounded collection state
SherlockLedger shall keep runs, accepted claims roots and registry state without per-member collection state.

Scenario: GIVEN a collection with many claims, WHEN it is accepted, THEN state grows by one root rather than one entry per member.
Verification: `WT/t20-batched/tools/sherlock-sdk/t20-ledger.test.mjs:117,122`.

#### LDG-002 — User-submitted run
When a user submits their own run, SherlockLedger shall compute its claim hash and reject an existing run-map entry.

Scenario: GIVEN one accepted claim, WHEN the same user resubmits it, THEN the contract refuses the replay.
Verification: T17 R3, `T17/RESULTS.md:26`. Operator jobs use LDG-006.

#### LDG-003 — One root per accepted collection
When acceptCollection succeeds, SherlockLedger shall insert one new claims root after checking the current registry root and rejecting a duplicate collection.

Scenario: GIVEN a stale registry root or accepted claims root, WHEN the call runs, THEN it refuses without changing state.
Verification: `WT/t20-batched/tools/sherlock-sdk/t20-ledger.test.mjs:122`.

#### LDG-004 — Membership without Sherlock writes
When consuming a collection claim, the consumer shall verify membership without an openClaim, descend or per-claim SherlockLedger write.

Scenario: GIVEN a forged path or unaccepted root, WHEN a Compact consumer checks the claim, THEN it refuses the claim.
Verification: `WT/t20-batched/tools/sherlock-sdk/t20-ledger.test.mjs:134,148,176,191` and `models/sherlock/quint/t20-batched/claimsRoot.qnt` in that worktree.

Sherlock consumers discharge GP-2 dependencies in circuit. Compact consumers use a native-keyed root read and their own nullifier.

#### LDG-005 — Claims tree and availability
The collection builder shall produce a depth-20 Fr claims tree and publish its leaf list and membership paths off chain at epoch close.

Scenario: GIVEN withheld member data, WHEN a consumer lacks a valid path, THEN it cannot use the claim.
Verification: membership tests at `WT/t20-batched/tools/sherlock-sdk/t20-ledger.test.mjs:148,236`; publication and real collection-proof checks MISSING.

Run leaves are `claim_pub`, empty leaves are zero and nodes are `transientHash([left, right])`. ADM and VIEW are not claim leaves.
Source: `WT/t20-batched/openspec/changes/sherlock-modular-leaf-20261111/specs/sherlock-ledger-contract/spec.md:38`.

#### LDG-006 — Batched writes
When settling operator jobs, Sherlock shall submit at most one collection root per checkpoint epoch and no write for an empty epoch.

Scenario: GIVEN N operator jobs in one epoch, WHEN the epoch closes, THEN settlement uses one transaction rather than N transactions.
Verification: `WT/t20-batched/models/sherlock/quint/t20-batched/epochSettle.qnt`, `OperatorWritesPerEpochAtMostOne`, recorded in `T20/RESULTS.md:58`.

#### LDG-007 — Foreign claims by membership
The consumer shall verify domain-separated foreign leaves through collection membership without a foreign map or FOREIGN opening transaction.

Scenario: GIVEN a run leaf offered as a foreign claim, WHEN membership is checked, THEN the consumer refuses it.
Verification: `WT/t20-batched/tools/sherlock-sdk/t20-ledger.test.mjs:202,214`; real foreign-lift collection proof MISSING.

#### LDG-008 — Batched registry changes
When registry updates exist, Sherlock shall commit at most one registry root per registry epoch and no update for an unchanged epoch.

Scenario: GIVEN several registrations and revocations, WHEN an epoch closes, THEN one root represents all changes.
Verification: `WT/t20-batched/models/sherlock/quint/t20-batched/registryEpoch.qnt`, `RegWritesPerEpochAtMostOne`; contract update entry MISSING, `T20/RESULTS.md:143`.

#### COPS-001 — Scale accounting
The settlement report shall distinguish deployment, checkpoint settlement, registry update and consumer transactions, with measured or derived counts at the stated workload.

Scenario: GIVEN one million jobs within one capacity-limited epoch, WHEN counts are derived, THEN the report gives one Sherlock collection transaction.
Verification: arithmetic model and tables `T20/RESULTS.md:45` through `:58`; million-job end-to-end run MISSING.

At depth 20, capacity is 1,048,576 claims. Epochs close after 3,600 seconds from the first job or at capacity.
Ten million jobs in one time window need ten capacity-bounded collections, assuming no additional early closes. These counts exclude deployments and consumer calls.
The steady settlement rate is approximately `max(1/3600, lambda/2^20)` during sustained arrivals, not an unconditional one transaction per hour.
These are DERIVED counts from `T20/RESULTS.md:33,54,66`, not measured throughput. Consumer nullifier transactions remain outside the Sherlock settlement count.

### sherlock-anchor-chunks

#### ANC-013 — Fixed slot 15
The wrap shall pin the anchor read key at lane 6 and chunk write key at lane 15 to storage slot 15.

Scenario: GIVEN a proof that reads another slot, WHEN wrap verification runs, THEN verification fails.
Verification: `t12_anchor_slot_15` in `CACHE/m7-wrap/src/openvm-t12-slot15/crates/sherlock-lfr/src/iface.rs`, named by `COMPLETE/specs/sherlock-anchor-chunks/spec.md:12`; final-key integrated test MISSING.

This reuses completion ANC-013 and its correction to ANC-001. Slot 15 is a storage slot, not public lane 15.

#### ANC-014 — Contract slot alignment
The deployed SherlockLedger shall place its anchor in the same pinned slot 15 used by the qualified proof chain.

Scenario: GIVEN a slot-0 contract build, WHEN final Preview deployment is prepared, THEN qualification refuses that build.
Verification: MISSING for final deployment. Reuses completion ANC-014 and STL-003.

#### ANC-002 — Canonical lock
When processing an intermediate chunk, the anchor shall equal `PoseidonFr_4(SHK_LOCK, x_j)` with `x_j = sponge(U_LOCK; c, pack(run_tag), k, d_in, d_out, j)`.

Scenario: GIVEN two runs with equal run tags but different bound inputs, WHEN chunks are spliced, THEN the anchor check refuses them.
Verification: `t13_t10q_chunk` run, 9/9 at openvm `e3b407d`, `COMPLETE/specs/sherlock-anchor-chunks/spec.md:45`; NT-1..NT-3 records at `WT/spec-audit/research/sherlock-spec-audit-20261115/AUDIT.md:42`. Final-key repetition MISSING.

This supersedes the stale P2(DOM_LOCK) form. Completion ANC-016..019 retain the corresponding guards and FP-3 binding requirements.

#### ANC-012 — Fixed chunk cap
The planner shall refuse any call that needs more than `K_MAX = 2` chunks.

Scenario: GIVEN an oversized entry, WHEN planning requires three chunks, THEN the planner refuses without truncating the call.
Verification: `WT/t20-batched/models/sherlock/quint/t20-batched/chunkCap.qnt`, `ChunksAtMostCap`, `NoSilentTruncation`; production planner MISSING.

The hard cap does not increase with a longer TTL. This supersedes the variable-cap text in `T20/RESULTS.md:126`.
The existing minimum-TTL and whole-transaction fee rules remain required. A direct general run uses its fixed hiding statement and does not chunk.

#### ANC-004 — One intent
The chunked call shall place every chunk in one intent of one transaction with the required forward claims.

Scenario: GIVEN a dropped suffix or interleaved chunk, WHEN the ledger checks the intent, THEN the transaction fails.
Verification: ANC-010 identifies ledger T1/T2 tests. Final universal-key T3 run MISSING, `T20/RESULTS.md:126`.

#### ANC-005 — Fallible atomicity
When a fallible chunk fails, the ledger shall preserve contract state while charging the included transaction's fee.

Scenario: GIVEN two fallible chunks, WHEN the second fails, THEN neither chunk's state effects remain.
Verification: MISSING for the final slot-15 wrap path. Existing mode-1F requirement: `FINAL/specs/sherlock-anchor-chunks/spec.md:32`.

#### ANC-006 — Guaranteed atomicity
If a guaranteed chunk fails or chunk placements mix, then the ledger shall reject the transaction.

Scenario: GIVEN mixed guaranteed and fallible chunks, WHEN the transaction is validated, THEN it is invalid.
Verification: MISSING for the final slot-15 wrap path. Existing mode-1G requirement: `FINAL/specs/sherlock-anchor-chunks/spec.md:39`.

ANC-007..011 retain cut, context, native prologue, replay and proof obligations. Slice A supplies the planner and native-entry integration.

### sherlock-era-table

These entries reuse ERT-001..009. Governance profile A/B below is unrelated to helper-host case A/B.

#### ERT-001 — Sealed registry
The era table shall deploy with maintenance authority `{committee: [], threshold: 1, counter: 0}`.

Scenario: GIVEN a maintenance update, WHEN it targets the era table, THEN the ledger refuses it.
Verification: `T22/quint/eraTable.qnt`, maintenance mutant. Sealed deployment and real ledger refusal MISSING, `T22/REPORT.md:59`.

#### ERT-002 — Readback gate
When deployment finishes, the deploy tool shall verify the empty committee, positive threshold, zero counter and exact installed verifier keys through indexer readback.

Scenario: GIVEN threshold zero or the SDK's default one-key committee, WHEN readback runs, THEN deployment validation fails.
Verification: MISSING. Required controls: `GOV/tasks.md:8`; ledger findings `T22/REPORT.md:8,9`.

#### ERT-003 — Unique era identity
If an era announcement reuses an earlier key-set digest or ledger identity, then the era table shall refuse it.

Scenario: GIVEN a previous era's key-set digest, WHEN another era announces it, THEN announcement fails.
Verification: `T22/quint/eraTable.qnt`, `inv_eraKeyUnique`, `inv_ledgerEraUnique`, `T22/REPORT.md:43,44`.

#### ERT-004 — Effective acceptance interval
The era table shall accept an era only within `[start, end_eff)` and never after cancellation or soundness_end.

Scenario: GIVEN a pending successor whose start has passed, WHEN the old era checks active, THEN it fails without waiting for activate.
Verification: `T22/quint/eraTable.qnt`, `inv_acceptInInterval`, `inv_oneAccepting`, `T22/REPORT.md:38,45`.

Here `end_eff = min(end, pending_start)`. The pending-start term applies only when a successor exists.

#### ERT-005 — One accepting era
The era table shall permit at most one accepting era at each block time.

Scenario: GIVEN two adjacent eras, WHEN the successor starts, THEN the predecessor no longer accepts.
Verification: `T22/quint/eraTable.qnt`, `inv_oneAccepting`, noLazyEnd mutant, `T22/REPORT.md:45`.

#### ERT-006 — Append-only history
The era table shall preserve every era record and its announced identity fields.

Scenario: GIVEN a normally ended era, WHEN a successor activates, THEN the old ledger and key references remain available.
Verification: `T22/quint/eraTable.qnt`, `inv_acceptedCheckable`, `T22/REPORT.md:39`.

#### ERT-007 — Irreversible end
The era table shall set an era's end at most once and never later than its effective end.

Scenario: GIVEN an ended era, WHEN governance tries to reopen it, THEN the request fails.
Verification: `T22/quint/eraTable.qnt`, `inv_endSetOnce`, `T22/REPORT.md:40`; dedicated mutant MISSING.

#### ERT-008 — Read-only native entries
The era table shall use native keys for active, era, ledgerOf, eraSafe and ledgerRootUsable without state writes.

Scenario: GIVEN a consumer read, WHEN it resolves an era, THEN the table state remains unchanged.
Verification: compile-only sketches `T22/sketch/era_table.compact`; deployed read test MISSING.

#### ERT-009 — Activation grid
If an announced start is not a multiple of GRID, then the era table shall refuse it.

Scenario: GIVEN a start between grid points, WHEN announce runs, THEN it refuses that start.
Verification: announce guard in `T22/quint/eraTable.qnt`; broader grid model MISSING, `T22/REPORT.md:53`.

### sherlock-era-ledger

#### ELD-001 — Sealed program ledger
The era ledger shall seal its era identity, table address and maintenance authority at deployment.

Scenario: GIVEN an installed era ledger, WHEN governance requests a VK replacement, THEN maintenance rejects the change.
Verification: `T22/quint/eraTable.qnt`, maintenance mutant; deployed sealed ledger MISSING.

#### ELD-002 — Ledger readback
When deploying an era ledger, the deploy tool shall pass ERT-002 before any announcement or attestation names it.

Scenario: GIVEN an unsealed ledger, WHEN it is prepared for attestation, THEN the readback gate stops the operation.
Verification: MISSING, `GOV/tasks.md:8`.

#### ELD-003 — Inclusion-time era check
Where premises A1 through A3 hold, every wrap-keyed entry shall append a claimed `table.active(era_id)` call checked at inclusion time.

Scenario: GIVEN a proof made before era end, WHEN its transaction arrives after era end, THEN no ledger write succeeds.
Verification: model `T22/quint/eraTable.qnt`, `inv_noRetiredAccept`; runtime and wrap-tail checks MISSING, `T22/REPORT.md:58`.

#### ELD-004 — Atomic era refusal
If the table's active check fails, then the era ledger shall refuse the whole call.

Scenario: GIVEN an inactive era, WHEN acceptCollection executes, THEN its root is not inserted.
Verification: `T22/quint/eraTable.qnt`, noLedgerCheck mutant; real three-contract run MISSING.

#### ELD-005 — Native read keys
The era ledger shall keep rootAccepted and runAccepted read-only under their compiled native keys.

Scenario: GIVEN a callee with a different installed read key, WHEN a consumer resolves it, THEN resolution fails.
Verification: `WT/t20-batched/tools/sherlock-sdk/t20-ledger.test.mjs:191`.

#### ELD-006 — Entry budget
The era ledger's wrap-keyed entries shall fit within `L_TX = 1,024`.

Scenario: GIVEN an appended era check, WHEN the contract is compiled, THEN the full entry fits the call-vector budget.
Verification: compile-only result `T22/REPORT.md:32`, 85 acceptRun lanes and 91 acceptCollection lanes. Actual wrapped tail binding MISSING.

#### ELD-007 — Named fallback
Where premise A2 or A3 fails, the era ledger shall keep the T20 entries unchanged and require consumer enforcement under CRR-006.

Scenario: GIVEN a failed claimed-read premise, WHEN the fallback is selected, THEN first use outside the active era is refused.
Verification: MISSING. `T22/REPORT.md:53` states the fallback is not modeled.

A1 is inclusion-time block-time enforcement. A2 is the claimed cross-contract read. A3 is binding that read in the wrapped statement.
The fallback is an explicit change in acceptance semantics. It does not establish that every old-era ledger acceptance occurred inside its interval.

### sherlock-governance-verbs

#### GOV-001 — Current-roster quorum
The era table shall authorize signed verbs only with at least Q distinct valid signatures from the roster in force.

Scenario: GIVEN duplicate signatures from one key, WHEN quorum is checked, THEN duplicates do not increase the signer count.
Verification: `T22/quint/eraTable.qnt`, singleSigner mutant; production signature implementation MISSING.

#### GOV-002 — Delayed announcement
When quorum announces an unused era without another pending era, the table shall require `start >= now + TL`.

Scenario: GIVEN an early start or an existing pending era, WHEN announce executes, THEN it refuses.
Verification: `T22/quint/eraTable.qnt`, noTimelock mutant and `inv_activeEraGoverned`, `T22/REPORT.md:41`.

#### GOV-003 — Permissionless activation
When any party activates a pending era after its start, the table shall close its predecessor, select the successor and clear pending state.

Scenario: GIVEN a matured announcement, WHEN a non-operator calls activate, THEN the announced transition executes.
Verification: `T22/quint/eraTable.qnt`, second-era witness, `T22/REPORT.md:51`.

#### GOV-004 — No early era change
The table shall change acceptance only at a sufficiently delayed announced start or through soundness_end.

Scenario: GIVEN an announced successor before its start, WHEN it receives a claim, THEN acceptance fails.
Verification: `T22/quint/eraTable.qnt`, `inv_activeEraGoverned`.

#### GOV-005 — Rotation preserves proof keys
When the current quorum authorizes rotation, the table shall replace the roster after RTL without changing eras, ledgers, claims or nullifiers.

Scenario: GIVEN accepted claims, WHEN the roster rotates, THEN the same claims remain checkable under the same proof keys.
Verification: `T22/quint/eraTable.qnt`, `inv_rotationKeepsClaimKeys`, rotateRekeys mutant, `T22/REPORT.md:42`.

#### GOV-006 — Immediate irreversible soundness end
When quorum invokes soundness_end, the table shall end the currently accepting era irreversibly after resolving any matured activation.

Scenario: GIVEN a successor already active by time, WHEN soundness_end executes, THEN it ends that successor rather than cancelling an era with claims.
Verification: `T22/quint/eraTable.qnt`, endStaleEra mutant, `T22/REPORT.md:49,51`.

An unmatured pending era is cancelled. The verb provides no resume operation.

#### GOV-007 — Cancel pending era
When quorum cancels an era before its start, the table shall mark it cancelled without reusing its identity.

Scenario: GIVEN a cancelled era, WHEN governance tries to announce the same identity again, THEN it refuses.
Verification: `T22/quint/eraTable.qnt`, cancelled-era witness and `inv_noClaimInCancelled`.

#### GOV-008 — Batched lineage attestations
When quorum attests a root of `(era_id, C_P, lineage, ledger)` leaves, the table shall make that root usable only after TL.

Scenario: GIVEN an unattested or immature program-ledger leaf, WHEN a consumer uses it, THEN the consumer refuses it.
Verification: MISSING. `T22/REPORT.md:53` excludes attestation from the model.

#### GOV-009 — No maintenance powers
The governance contract shall expose no maintenance, freeze, seizure or claim-pause operation.

Scenario: GIVEN an operator quorum, WHEN it attempts to rewrite an accepted claim, THEN no authorized verb permits that action.
Verification: `T22/quint/eraTable.qnt`, maintenance mutant covers sealing only. Full deployed verb-surface test MISSING.

#### GOV-010 — Signed-message replay protection
The table shall bind each signed message to its address, verb, roster generation and a nonce incremented by every signed verb.

Scenario: GIVEN a valid message for an earlier nonce or another table, WHEN it is replayed, THEN it fails authorization.
Verification: MISSING. `T22/REPORT.md:53` excludes this behavior from the model.

#### GOV-011 — Cancel rotation
When current-roster quorum cancels a pending rotation before activation, the table shall remove that pending rotation.

Scenario: GIVEN a hostile pending rotation, WHEN the current quorum cancels it in time, THEN the roster remains unchanged.
Verification: MISSING. `T22/REPORT.md:53` excludes cancel_rotation from the model.

### sherlock-consumer-read

#### CRR-001 — Registry-following profile
Where a consumer uses governance profile A, it shall seal the table address and take the claim's era as an argument.

Scenario: GIVEN a claim naming another table, WHEN the consumer checks it, THEN it uses only its sealed table.
Verification: compile-only `T22/sketch/era_consumer.compact`; runtime check MISSING.

#### CRR-002 — Fixed-era profile
Where a consumer uses governance profile B, it shall read only ledgers sealed at its deployment.

Scenario: GIVEN a later announced era, WHEN an old profile-B consumer receives its claim, THEN that ledger is outside the accepted set.
Verification: MISSING for a deployed profile-B consumer.

#### CRR-003 — Authenticated ledger address
The registry-following consumer shall obtain a claim's ledger from ledgerOf or a usable attested program-ledger leaf.

Scenario: GIVEN an attacker-supplied ledger that returns success, WHEN its address lacks table authentication, THEN the consumer refuses it.
Verification: compile-only `T22/sketch/era_consumer.compact`; three-contract ledger test MISSING, `T22/OPEN-QUESTIONS.md:22`.

#### CRR-004 — Unsafe-era claims
If an era is soundness_ended, then the consumer shall refuse its claims.

Scenario: GIVEN an older accepted claim from a broken era, WHEN it is used, THEN the consumer refuses it.
Verification: `T22/quint/eraTable.qnt`, `inv_noUnsafeUse`, keepBrokenClaims mutant, `T22/REPORT.md:48`.

#### CRR-005 — Era-stable nullifier
The consumer shall key once-only use by `H(SHK_CLAIM_NUL, lineage, input_pub, outcome_pub)` with authenticated lineage.

Scenario: GIVEN the same run under two era-specific program commitments, WHEN the second claim is used, THEN the consumer refuses duplicate use.
Verification: `T22/quint/eraTable.qnt`, `inv_runUsedOnce`, nullifierByCp mutant, `T22/REPORT.md:47`. Attestation binding MISSING.

This supersedes claim_pub-only nullifiers for cross-era use. T20's same-era duplicate tests remain required.

#### CRR-006 — Use-time fallback
Where premise A2 or A3 fails, the consumer shall refuse first use when the claim's era is not active at use time.

Scenario: GIVEN an old era's unused claim after transition, WHEN the fallback consumer receives it, THEN it refuses that first use.
Verification: MISSING, `T22/REPORT.md:53`.

#### CRR-007 — Consumer-owned write
When a consumer uses a claim, it shall write only its own nullifier.

Scenario: GIVEN an accepted collection, WHEN many consumers use its members, THEN Sherlock contract state receives no per-use writes.
Verification: `WT/t20-batched/tools/sherlock-sdk/t20-ledger.test.mjs:134,162,227`.

### sherlock-gpu-wrap and privacy boundary

#### GPW-001 — Owned GPU and CPU fallback
The wrap prover shall use the user's GPU with the stock CPU implementation as its fallback.

Scenario: GIVEN no usable GPU, WHEN the user proves a wrap, THEN the CPU proof passes the unchanged ledger verifier.
Verification: historical CPU/GPU runs `WT/gpu-wrap/research/sherlock-gpu-wrap-20261111/RESULTS.md:7` through `:27`. Final-wrap repeat MISSING.

#### GPW-003 — Seeded implementation parity
Under test-only deterministic randomness, the CPU and GPU wrap configurations shall produce identical proof bytes accepted by both verifiers.

Scenario: GIVEN identical test inputs and randomness, WHEN either configuration differs, THEN that configuration fails qualification.
Verification: CPU/GPU parity runs, `WT/gpu-wrap/research/sherlock-gpu-wrap-20261111/RESULTS.md:7` through `:27`. They cover a decider and proxy, not final T25 qualification.

#### GPW-007 — Measured timing records
The performance report shall label timing as MEASURED only with its run, machine, load, memory, key revision and proof-verification result.

Scenario: GIVEN a synthetic shape or loaded-host projection, WHEN performance is reported, THEN it retains its scope and derived or estimated label.
Verification: `WT/gpu-wrap/research/sherlock-gpu-wrap-20261111/RESULTS.md:7,33`; final-key repeated timings MISSING.

GPW-002, GPW-004..006 retain the kernel, MSM and fallback obligations. They do not permit a second proving engine or a changed verifier.

#### PRD-001 — Local daemon boundary
The resident prover shall accept socket requests only from the same user on the same machine.

Scenario: GIVEN a different-user client, WHEN it connects, THEN the daemon refuses the request.
Verification: MISSING in the inspected evidence. Existing requirement: `FINAL/specs/sherlock-prover-daemon/spec.md:4`.

#### PRD-003 — Private artifacts stay inside case A
The prover shall keep private STARK proofs, checkpoints and shard plans within the user's owned-device trust domain.

Scenario: GIVEN a helper outside that domain, WHEN transfer is requested, THEN the prover refuses it.
Verification: MISSING for host-ownership enforcement. Authority: OD-9, `DEC:23`, `WIKI/log.md:4112,4113`.

#### ZK-001 — End-to-end zero knowledge
Sherlock shall preserve end-to-end zero knowledge by exposing only the blinded settlement proof and permitted public statement outside the user's trust domain.

Scenario: GIVEN private inputs and execution details, WHEN a run settles, THEN external observers receive no plain witness or intermediate private proof.
Verification: T17 privacy scan `T17/RESULTS.md:22` is partial evidence. Final EQ-ZK and formal composition evidence MISSING here.

#### ZK-014 — Test seeds never reach production
If production receives a deterministic test-seed option, then the wrap prover shall exit with code 4 and produce no proof.

Scenario: GIVEN SHERLOCK_TEST_SEED, GW_DET, GW_CHIP_SEED or GW_SEED without test-seed support, WHEN proving starts, THEN it refuses.
Verification: MISSING in this slice's inspected tree. Reuses completion ZK-014 without asserting its completion.

#### ZK-015 — EQ-ZK
When proving one statement twice, the production wrap prover shall produce different proof bytes that both pass ledger 9.1.0.1 verification.

Scenario: GIVEN fresh production randomness, WHEN both proofs complete, THEN both verify and their bytes differ.
Verification: MISSING for final keys. Reuses completion ZK-015.

#### ZK-016 — One-use slots
If a request reuses a blinding slot, then wrapd or the STARK worker shall refuse it.

Scenario: GIVEN a consumed slot after an interrupted request, WHEN recovery tries to reuse it, THEN the daemon refuses.
Verification: MISSING in the inspected evidence. Reuses completion ZK-016 and PRD-002.

#### ZK-017 — GPU randomness
While a production wrap runs on odin or huginn, its prover shall obtain fresh blinders from OsRng.

Scenario: GIVEN the GPU backend, WHEN a proof starts, THEN deterministic test randomness cannot supply production blinders.
Verification: MISSING for the final GPU path. Completion ZK-017 tracks this gate.

#### ZK-018 — Case A only
If a configuration assigns proving to a host the user does not own, then Sherlock shall refuse that configuration.

Scenario: GIVEN an outsourced helper, WHEN a job is scheduled, THEN no plain witness, private proof or checkpoint is sent.
Verification: MISSING for an executable ownership guard. Owner ruling `WIKI/log.md:4111` through `:4113` defers case B to Veritee.

ZK-012 and ZK-013 retain one CSPRNG `r_P` per deployment and fresh salt per run. Wrap blinders remain fresh per proof.
The case-A decision accepts no PCS masking only while intermediate STARK layers stay on owned devices. It does not authorize outsourced plaintext proving.

### sherlock-client-operations

#### CSDK-005 — SDK lifecycle
The SDK and CLI shall expose planning, proving, deployment, submission and receipt inspection with explicit network and qualified artifact identities.

Scenario: GIVEN a proof for another key revision, WHEN submission is prepared, THEN artifact validation stops the call.
Verification: partial runner `WT/t17-preview/tools/sherlock-sdk/t10a-ledger-run.mjs`; complete public SDK/CLI contract and conformance suite MISSING.

#### CSDK-006 — Readiness before live settlement
If proof bindings, final-key qualification or deployment readback are incomplete, then the client shall refuse live settlement.

Scenario: GIVEN a dry-run stub with public words 1, 2 and 3, WHEN a real call is prepared, THEN argument validation refuses it.
Verification: `WT/t23-args/tools/t23-suite/test_accept_args.py:188`, `test_prove_result_crosscheck`; integrated final-readiness gate MISSING.

#### COPS-002 — Wallet secrets never logged
The wallet adapter shall exclude wallet seeds, mnemonics, private keys and witness preimages from logs, reports, exceptions and network diagnostics.

Scenario: GIVEN an error containing secret data, WHEN diagnostics are emitted, THEN the secret data is redacted.
Verification: `WT/preview-deploy/tools/sherlock-sdk/preview-session.test.mjs:34,108,119`; runner scrub `WT/t17-preview/tools/sherlock-sdk/t10a-ledger-run.mjs:116,497`. Full error-path audit MISSING.

#### COPS-003 — Preserve the existing wallet
The Preview runner shall reuse the existing authorized wallet without overwriting its secrets or copying them to proving boxes.

Scenario: GIVEN an existing wallet, WHEN the runner restarts, THEN the same local wallet remains intact.
Verification: `WT/preview-deploy/tools/sherlock-sdk/preview-session.test.mjs:23,39,65` covers exclusive creation and permissions. No-secret-transfer test MISSING.

Wallet handling is described from code only. Secret files use mode 0600 within a 0700 directory in that adapter, `preview-session.mjs:35` through `:50`.

#### COPS-004 — Serialize wallet operations
When Preview transactions share a wallet, the runner shall hold the common wallet lock through synchronization, balance, submit, verdict and recovery.

Scenario: GIVEN concurrent callers, WHEN one holds the lock, THEN the other waits before modifying wallet state.
Verification: T17 run `T17/RESULTS.md:40`; implementation `WT/t17-preview/tools/sherlock-sdk/t10a-ledger-run.mjs:537` through `:545`. Concurrency test MISSING.

#### COPS-005 — Fees from the selected chain
The Preview wallet shall calculate fees with parameters read from the selected chain.

Scenario: GIVEN Preview prices differing from initial defaults, WHEN balancing runs, THEN it uses the Preview parameters.
Verification: T17 result `T17/RESULTS.md:35`; runner `t10a-ledger-run.mjs:233,505` in that worktree.

#### COPS-006 — Idempotent deployment recovery
When a deployment lacks confirmation, the runner shall inspect the saved address and chain state before sending another deployment.

Scenario: GIVEN a submitted deployment followed by a crash, WHEN the runner resumes, THEN it discovers inclusion before retrying.
Verification: recorded T17 fix `T17/RESULTS.md:38`; crash-injection test MISSING.

#### COPS-007 — Distinguish refusal from transport failure
The runner shall report a cryptographic refusal only when it has an explicit node verdict.

Scenario: GIVEN an indexer timeout, WHEN a negative test runs, THEN the result is no-node-verdict or timeout, never a passed refusal.
Verification: T17 refusal rerun `T17/RESULTS.md:42,43`; verdict branches `WT/t17-preview/tools/sherlock-sdk/t10a-ledger-run.mjs:564` through `:572`.

#### COPS-008 — Public receipt evidence
The runner shall retain included transaction hashes and blocks, or rejected-byte hashes with node errors, without private inputs.

Scenario: GIVEN a rejected transaction, WHEN evidence is saved, THEN it contains no fabricated inclusion hash or block.
Verification: T17 evidence rule `T17/RESULTS.md:29` and receipts at `:19` through `:26`.

#### COPS-009 — Checkpoint validation
When resuming a proof, the prover shall verify the checkpoint's key manifest, proof and interval before continuing.

Scenario: GIVEN a changed key manifest or wrong height, WHEN resume runs, THEN it refuses the checkpoint.
Verification: `resume_mismatched_key_manifest_refused`, `resume_wrong_h_refused`, `WT/t16-elastic/research/sherlock-t16-elastic-20261114/RESULTS.md:68`.

#### COPS-010 — Resource policy from setup
The owned-box runner shall derive scheduling and wrap-overlap memory admission from the machine's setup profile.

Scenario: GIVEN insufficient free device memory, WHEN overlap is requested, THEN it uses a safe schedule without changing the universal keys.
Verification: MISSING for the combined scheduler-to-wrap guard. Existing PF-008 rule: `WT/t16-elastic/research/sherlock-t16-elastic-20261114/SETUP-SPEC.md:27`; `WT/t23-suite/tools/t23-suite/t23.py:73` rejects missing wrap-tail data.

This references setup ST/PF/SC requirements. It does not restore a schedule menu in key_root.

#### COPS-011 — Box artifact validation
When a proving box returns artifacts, the coordinator shall check their paths, hashes and ledger verification before accepting the result.

Scenario: GIVEN a corrupt returned proof, WHEN muninn verifies it, THEN the result receives no KEEP verdict.
Verification: operational rule `LOOP:96`; automated transport-integrity and final-key receipt test MISSING.

#### COPS-012 — Recovery without secret transfer
When an owned-box link fails, the coordinator shall preserve resumable state and move ready work to another owned box without transferring wallet secrets.

Scenario: GIVEN an unavailable box, WHEN recovery begins, THEN valid checkpoints remain local to the trust domain and relinking waits for required owner authentication.
Verification: owner recovery procedure `LOOP:96,104`; end-to-end crash and link-loss test MISSING.

#### COPS-013 — Useful progress without witness leakage
The status interface shall report stage, time, resources and verdict without sending private witnesses or source outside the owned-device trust domain.

Scenario: GIVEN a private run displayed in the GUI, WHEN progress refreshes, THEN public diagnostics reveal no private source or witness.
Verification: MISSING. T21 GUI/live-daemon integration remains a completion dependency, `LOOP:43`, LPC-011.

#### COPS-014 — Era-aware admission
When an era approaches its effective end, the operator shall stop starting jobs whose proof and inclusion budget exceeds the remaining interval.

Scenario: GIVEN an open batch near era end, WHEN the admission bound fails, THEN it closes the batch in time or routes work to the successor.
Verification: MISSING. Existing T22 rule `T22/OPEN-QUESTIONS.md:70,86`; no fixed historical proving time becomes a production guarantee.

#### COPS-015 — Proof-version migration
If Midnight stops accepting a sealed contract's proof version, then governance shall deploy successor contracts with a predecessor link and an explicitly downgraded snapshot attestation.

Scenario: GIVEN an incompatible protocol upgrade, WHEN migration is prepared, THEN old contracts remain sealed and snapshot trust is explicit.
Verification: MISSING. Existing T22 O11, `T22/OPEN-QUESTIONS.md:21`.

#### COPS-016 — No tooling prerequisite for developers
The public SDK and CLI shall compile, prove and deploy without requiring the maintainer's factory or coordination tooling.

Scenario: GIVEN a clean developer environment, WHEN a documented public workflow runs, THEN no DoctorWatson, box bus or agent service is required.
Verification: MISSING. Authority: supplied project charter, Product boundary.

#### COPS-017 — Maintainer resource exclusion
When maintainer jobs use an owned proving box, the runner shall enforce measured resource caps and box-local GPU exclusion.

Scenario: GIVEN another job holding gpu.lock, WHEN a GPU stage starts, THEN it waits without oversubscribing the device.
Verification: MISSING for a concurrency fault test. Existing stage locks: `WT/t16-elastic/research/sherlock-t16-elastic-20261114/run-odin.sh:20,30,35` and `run-huginn.sh:11,24,29`.

The maintainer workflow uses DoctorWatson preflight and heavy-job controls under the supplied charter. It is not a public SDK dependency.
LOOP's measured-cap rule is at `LOOP:90`; its older 90G wording at `LOOP:99` conflicts with that rule and must not silently raise the cap.

## 3. Decisions

IDs below reuse the source's namespace. For example, `T22/O3` and `LOOP/O3` are different decisions.
“No ID” means the source gives no stable decision ID. This draft does not invent an existing MADR identifier.

| Existing ID or source label | Outcome | Authority | Source file:line | Requirement binding |
|---|---|---|---|---|
| Hard rules, no ID | k <= 21, VK <= 50,000 bytes, 100-bit security, unchanged ledger 9.1.0.1, one BLS12-381 wrap | OWNER | `LOOP:102`; `ROOT/research/sherlock-e2e-spec-20261115/BRIEF.md:54` | DG-001..003, LW-013, LW-017, RKY-003..005 |
| Hard rules, no ID | One universal key set, small-field STARK leaves/nodes, GP-1..GP-4 retained, no plaintext witness to helpers | OWNER | brief hard-rules section; `LOOP:102`; `WIKI/log.md:4113` | LW-013, DG-010, STL-002, ZK-001, ZK-018 |
| OD-24 | Option E: L_Fr before one stdlib wrap; reject ledger changes, G4 and a second BLS layer | OWNER | `DEC:35` | LW-002, LW-013, LW-017 |
| OD-11 | No second Fr proving engine as a call-time fallback | OWNER | `DEC:25` | LW-014, GPW-006 reference |
| OD-19, OD-4 | Unique decoding and security margin on actual layer parameters | OWNER | `DEC:18,33`; `LOOP:72` | RKY-005, LW-016 |
| OD-7, OD-15 | L_TX=1,024 and one 1,026-word call-vector layout | OWNER | `DEC:21,29` | LW-003, MID-003, ELD-006 |
| O4 / D-10, D-12 | Keep reservation order and OnionAir in L | OWNER | `LOOP:73`; `DEC:170` | LW-004, LW-005 and slice B |
| O4 / D-11 | Replay every entry, including PATH, after T4 established fit | OWNER | `DEC:180` | LW-004 |
| O5 / D-9 | Every decoder check is in circuit; L key pre-hash is a key constant | OWNER | `LOOP:74`; `DEC:171` | LW-002, LW-004, LW-006 |
| O7 | Five openClaim-era lane-map pairs for the historical interface | OWNER | `LOOP:76` | LW-016 historical regression only |
| O9 | No shape_d in acceptCollection | OWNER | `LOOP:78` | LW-009, LDG-003 |
| O6 | E6 four-hour TTL and removal of the old callvec map | OWNER | `LOOP:75` | Historical E6 decision, not the current T20 default TTL |
| LOOP/O3 | Option A for prototype key freeze; revisit B only after real keys and row qualification | LEAD PICK confirmed | `LOOP:83,84` | LW-016, RKY-003 |
| N-11 | One SherlockLedger per program | LEAD PICK confirmed | `LOOP:87`; `MAP:31` | CSDK-001, STL-003 |
| Revocation after acceptCollection | Acceptance is final under registry revocation; no recheck on every opening | LEAD PICK confirmed | `LOOP:83,86` | LDG-004, CRR-004 distinguishes soundness_end |
| T20/W-1..W-4 | Batch operator settlement, membership reads, foreign claims and registry updates | OWNER | `LOOP:42,106`; `T20/RESULTS.md:3` | LDG-001..008 |
| T20 collection form | `[reg_pub, claims_root]`, depth 20, native-keyed reads, consumer nullifier, SHK_FCLAIM | LEAD PICK confirmed | `LOOP:106`; `T20/RESULTS.md:65` through `:68`, `:95` | LW-009, LDG-003..007, ELD-005 |
| T20 epoch / TTL | 3,600-second epochs; TTL below 1,755 seconds refused | LEAD PICK confirmed | `LOOP:106`; `T20/RESULTS.md:33,163` | LDG-006, ANC-012 |
| T20 K_MAX | K_MAX=2, not an automatically expanding TTL-dependent cap | OWNER | `LOOP:106`; brief hard rules | ANC-012 |
| OD-16 / x_j canon | A_0=A_k=C_P; intermediate locks use the x_j form with k and input/output digests | OWNER | `DEC:30`; `LOOP:106`; `WIKI/log.md:4107` | ANC-002, LW-005 |
| T12-SLOT15 | Pin anchor slot 15 in the proof chain and contract | LEAD PICK confirmed | `WIKI/log.md:4098,4100,4107` | ANC-013, ANC-014, STL-003 |
| OD-8 | Native entries use the anchor prologue and anchor_home | OWNER | `DEC:22,30` | ANC-009 reference |
| OD-9 | Proving trust domain is the user's own devices and GPUs | OWNER | `DEC:23` | PRD-003, ZK-018 |
| Case A / case B | Case A now; outsourced case B deferred to a Veritee design | OWNER | `WIKI/log.md:4111` through `:4113` | ZK-018, CSDK-003 |
| OD-13 | Resident local daemon with one-use slots | OWNER | `DEC:27` | PRD-001, ZK-016 |
| T11-1 | r_P per deployment, input salt per run, wrap blinders per proof | LEAD PICK confirmed | `WIKI/log.md:4062,4107` | ZK-012/013 references, LW-012 |
| T11-2 | Merge the T14b slot implementation | LEAD PICK confirmed | `WIKI/log.md:4062,4107` | ZK-016 |
| T11-3 | No PCS masking while intermediate STARK proofs stay on owned devices | LEAD PICK confirmed | `WIKI/log.md:4062,4107,4113` | ZK-001, PRD-003 |
| T11-4 | Every seeded path is test-only | LEAD PICK confirmed | `WIKI/log.md:4062,4107` | ZK-014, ZK-017 |
| T25 single rekey | One complete rekey list, followed by new measurements and gates | LEAD PICK confirmed | `LOOP:47`; `WIKI/log.md:4107` | RKY-001..005 references |
| F-3 replacement | Setup profiles determine schedules; no schedule menu in key_root | OWNER | `LOOP:105` | COPS-010, SC-011/012 references |
| MERGE-WORK | Preserve 5,400 columns and the full GP obligations | OWNER | `LOOP:106`; brief hard rules | DG-010, slice B obligation |
| Preview final redeploy | Deploy final keys after T12; T17(a) is an earlier rehearsal | LEAD PICK confirmed | `LOOP:39,88`; `LOOP:95` | STL-003, MID-004 |
| T23 instance reuse / profile path | Reuse only deployment-equal state; use the box setup profile | LEAD PICK confirmed | `LOOP:45`; `WIKI/log.md:4107` | CSDK-004, COPS-010 |
| MAP pick 1 | Outcome is Halted([comm] ++ eff) | LEAD PICK for T15 | `MAP:70,72` | STL-001, STL-005 |
| MAP pick 2 | Input binds vd(tape), fresh salt, zero fuel and stack limit for straight-line R_SL | LEAD PICK for T15 | `MAP:73` | STL-001, slice A/B statement binding |
| MAP pick 3 | One ledger per (contract, circuit), with private deployment salt | LEAD PICK for T15 | `MAP:74` | CSDK-001, STL-003 |
| MAP pick 4 | Native-refused calls do not settle | LEAD PICK for T15 | `MAP:75` | CSDK-002 |
| T22 authority | Fenrir operator keys govern the on-chain representation and rotate | OWNER | `WIKI/log.md:3663,3664` | GOV-001, GOV-005 |
| T22 sealing / HFC eras | Sealed contracts, era composition and registry verbs instead of in-place key swaps | LEAD PICK confirmed | `WIKI/log.md:3670,4107`; `T22/USE-CASES.md:21` | ERT-001, ELD-001, GOV-009 |
| T22/O1 | Immediate irreversible soundness_end under the same quorum | LEAD PICK confirmed | `T22/OPEN-QUESTIONS.md:11`; confirmation `WIKI/log.md:4107` | GOV-006 |
| T22/O2 | Rotatable roster with delayed, key-preserving rotation | LEAD PICK confirmed | `T22/OPEN-QUESTIONS.md:12`; `WIKI/log.md:4107` | GOV-005 |
| T22/O3 | Tail active check if A1..A3 hold; named consumer fallback otherwise | LEAD PICK confirmed | `T22/OPEN-QUESTIONS.md:13`; `WIKI/log.md:4107` | ELD-003..007, CRR-006 |
| T22/O4 | Claims from a soundness-ended era are unsafe by default | LEAD PICK confirmed | `T22/OPEN-QUESTIONS.md:14`; `WIKI/log.md:4107` | CRR-004 |
| T22/O5 | Cancel an announcement before start | LEAD PICK confirmed | `T22/OPEN-QUESTIONS.md:15`; `WIKI/log.md:4107` | GOV-007 |
| T22/O6 | Use authenticated lineage for an era-stable nullifier | LEAD PICK confirmed | `T22/OPEN-QUESTIONS.md:16`; `WIKI/log.md:4107` | CRR-005 |
| T22/O7 | One accepting universal set per era; a successor can be prepared before activation | LEAD PICK confirmed | `T22/OPEN-QUESTIONS.md:17`; `WIKI/log.md:4107` | ERT-003..005 |
| T22/O8 | Keep per-program ledgers and attest program-ledger roots in batches | LEAD PICK confirmed | `T22/OPEN-QUESTIONS.md:18`; `WIKI/log.md:4107` | GOV-008, CSDK-001 |
| T22/O9 | Distinct operator signatures counted on chain | LEAD PICK confirmed | `T22/OPEN-QUESTIONS.md:19`; `WIKI/log.md:4107` | GOV-001 |
| T22/O10 | Support governance profiles A and B, with A the default | LEAD PICK confirmed | `T22/OPEN-QUESTIONS.md:20`; `WIKI/log.md:4107` | CRR-001, CRR-002 |
| T22/O11 | Proof-version migration uses new deployments and a marked snapshot trust downgrade | LEAD PICK confirmed | `T22/OPEN-QUESTIONS.md:21`; `WIKI/log.md:4107` | COPS-015 |
| T22/O12 | Runtime ledger addresses require table or attestation authentication | LEAD PICK confirmed | `T22/OPEN-QUESTIONS.md:22`; `WIKI/log.md:4107` | CRR-003 |
| T22/O13 | Current quorum can cancel a pending rotation | LEAD PICK confirmed | `T22/OPEN-QUESTIONS.md:23`; `WIKI/log.md:4107` | GOV-011 |
| T22/P1..P6 | Parameter proposals: TL=7 days, RTL=TL, no overlap, Q=12 of 16, grid=3,600 s, ledger-derived skew | LEAD PICK for T15 for exact parameter values | `T22/OPEN-QUESTIONS.md:53,66,70,82,86,90` | GOV-001..005, ERT-009, COPS-014 |
| Design openness | Parameters and keys can change until the end-to-end prototype; hard-rule changes remain owner-gated | OWNER | `LOOP:101,102` | RKY gates, all qualification entries |
| Compute and recovery | Owned boxes return hashes; muninn verifies; recover resumable jobs after reboot | OWNER | `LOOP:96,104` | COPS-009..012 |

The 09:04 wiki entry confirms “T22 governance picks” as a group. It does not enumerate P1..P6 numeric values.
Exact parameter-value confirmation is UNCONFIRMED. The draft therefore binds the symbolic parameters and does not declare a seven-day delay canon.
T22's 14-day historical value is not canon, `WIKI/log.md:3694`. Distinct T22 decision IDs must keep their namespace in MADR records.

## 4. Evidence table

Every code or measurement assertion above is tied to a check or source. This table consolidates the consequential facts and their limits.

| Claim | Evidence | Status and limit |
|---|---|---|
| The CLI rejects k>21 and calls prove with OsRng | `ROOT/tools/sherlock-wrap/src/bin/wrap.rs:32,79` | CODE. No execution in this draft. |
| M7's custom-gate key fails stock key parsing | `WT/m7-wrap/research/sherlock-m7-wrap-20261112/RESULTS.md:31` through `:43` | MEASURED toy refusal. Not a valid final architecture. |
| The standalone ledger tool uses tagged_deserialize and VerifierKey::verify | `WT/k21-measure/tools/k21-ledger-verify/src/main.rs:25,27` | CODE. Tool reads artifacts only when run. It was not run here. |
| The standalone tool mutates every public lane and flips a proof byte | same file `:32` through `:48` | CODE, decider harness. Final wrap uses its own evidence. |
| E5 round 3 has 1,668,323 rows, 2,912-byte proof and 1,325-byte key | `WT/e5-wrap/research/sherlock-e5-wrap-20261112/RESULTS.md:342,344` | MEASURED historical E5 circuit. |
| E5 stock ledger accepts and refuses three PI changes and three proof-bit mutations | same report `:345` | MEASURED historical E5 run. |
| T10m records 1,685,845 rows, k=21, 1,325-byte VK, D3 floor 101.0110 | `LOOP:31`, Moriarty `ca96c1466`, openvm `5b7ada8` | MEASURED integration record, not T25. |
| Wrap-diet measurements use synthetic shapes without new keys | `WT/wrap-diet/research/sherlock-wrap-diet-20261114/RESULTS.md:26` | MEASURED cost model only. No final-key performance conclusion. |
| Wrap-diet k=20 fitting candidates use list decoding contrary to OD-19/OD-24 | same report `:8,9`; `DEC:33,35` | Search result under different assumptions, not an adopted path. |
| Historical CPU/GPU proof equality uses deterministic test seeds | `WT/gpu-wrap/research/sherlock-gpu-wrap-20261111/RESULTS.md:7,21,27` | MEASURED decider/proxy. Does not establish EQ-ZK. |
| Current inspected decoder has slot-0 template and six digest tags | `ROOT/tools/sherlock-wrap/src/decode.rs:66,87` | CODE. Slot-15 and foreign-tag changes need integration. |
| SherlockLedger takes three acceptRun arguments and computes the claim hash | `WT/t9-contract/tools/sherlock-sdk/sherlock-ledger.compact:41` through `:45`; `MAP:23` | CODE. claim_pub is not a fourth argument. |
| T20 contract writes one claims root and keeps native read entries | `WT/t20-batched/tools/sherlock-sdk/sherlock-ledger.compact:61` through `:75` | CODE. Contract execution alone does not bind a collection proof. |
| T20 tests cover root, path, replay, read-key and kind confusion | `WT/t20-batched/tools/sherlock-sdk/t20-ledger.test.mjs:117` through `:236` | Existing tests inspected by name. Not rerun. |
| T20 measured acceptRun/acceptCollection lanes are 42/48 | `T20/RESULTS.md:39,40` | MEASURED compiled-contract counts. |
| T20 6,797-byte collection transaction is an estimate | `T20/RESULTS.md:43` | ESTIMATE from 6,783-byte devnet acceptRun, not a collection receipt. |
| One prover's 492 calls/day is derived from 175.5 seconds per call | `T20/RESULTS.md:13,14,56` | DERIVED historical stage sum. Not a service guarantee. |
| L over SET and foreign-lift changes are absent from T20's implementation | `T20/RESULTS.md:70,97` | Explicit source limit. Integration completion UNCONFIRMED. |
| T20's registry root is sealed with no update entry | `WT/t20-batched/tools/sherlock-sdk/sherlock-ledger.compact:38`; `T20/RESULTS.md:143` | CODE plus report. Epoch-update behavior exists as a model requirement. |
| T17 deployed and included an honest call at blocks 113,966 and 115,187 | `T17/RESULTS.md:16,19` | MEASURED published receipts, T10m keys. |
| T17 raw/framed VK sizes are 1,325/1,353 bytes | `T17/RESULTS.md:17` | MEASURED. Report both forms in final qualification. |
| T17 honest transaction is 6,780 bytes | `T17/RESULTS.md:19` | MEASURED Preview receipt. Distinct from 6,783-byte devnet figure. |
| T17 R1..R4 have node refusals and the privacy scanner has controls | `T17/RESULTS.md:22` through `:26` | MEASURED public-path checks. Not a ZK proof. |
| T17 runner scrubs exceptions, locks wallet operations and distinguishes node verdicts | `WT/t17-preview/tools/sherlock-sdk/t10a-ledger-run.mjs:116,497,537,564` | CODE. Full no-leak and crash testing MISSING. |
| Wallet helper enforces private permissions and exclusive creation | `WT/preview-deploy/tools/sherlock-sdk/preview-session.mjs:35` through `:50` | CODE only. No wallet data read. |
| Builder maps statement preimages and compares public lanes | `WT/t23-args/tools/t23-suite/accept_args.py:305,327,347`; `MAP:49` through `:52` | CODE. Does not prove that R_SL binds those preimages. |
| Builder tests reproduce accepted T17 argument bytes | `WT/t23-args/tools/t23-suite/test_accept_args.py:147`; `MAP:61` | Recorded result and test. input/outcome derivation starts at packed operands for this fixture. |
| R_SL FP1 probe is not a complete run statement binding | `OPENVM/crates/sherlock-leaf/src/rsl/air.rs:217` through `:242` | CODE: probe comment, 54 values, equality to provided public values. No inference of complete R_SL settlement. |
| Backend config targets 100 bits and uses midnight-circuits 7.3.0 for Fr support | `CACHE/m7-wrap/src/stark-backend-tested/crates/stark-sdk/src/config/mod.rs:34,40`; its `Cargo.toml:23` | CODE. Configuration is not final security qualification. |
| FP-3 reference tests exist for forged effects and general-run refusals | `CACHE/m7-wrap/src/stark-backend-enc-fp3/crates/stark-sdk/tests/fp3_closure.rs:287,454,603` | Existing reference tests. Not evidence that every R_SL call is bound. |
| T23 dry run had 208 cases, zero proved comparisons and 145 blocked cases | `WT/t23-suite/research/sherlock-t23-suite-20261115/T23-DRYRUN.md:11` through `:36` | MEASURED dry-run report. Refused-as-native rows are DERIVED, not new native executions. |
| Part A reuses state-equal instances and has a version-aware report | `WT/t23-suite/tools/t23-suite/parta.py:56`; `parta_report.py:63` through `:67` | CODE. Complete Part A success UNCONFIRMED. |
| T22 has 34 EARS entries and compile-only contract sketches | `T22/REPORT.md:23,25,30,32` | Source report. No production governance deployment. |
| T22 model records 10,000 sampled traces and eleven killed mutants | `T22/REPORT.md:35` through `:51`; `WIKI/log.md:3863` | MEASURED model evidence. Sampling is not exhaustive proof. |
| Attestation, replay, cancel_rotation and fallback are not modeled | `T22/REPORT.md:53` | Explicit coverage gaps. |
| T22 era-tail sketches preserve existing transcript prefixes and add 43 words | `T22/OPEN-QUESTIONS.md:13`; `T22/REPORT.md:32` | Compile-only. Wrap binding and inclusion ordering MISSING. |
| T17's SDK deployment is not sealed | `T22/REPORT.md:8,9,59` | Reported SDK source finding. Fresh authority readback UNCONFIRMED. |
| T16 resume checks manifest, prefix proof and interval | `WT/t16-elastic/research/sherlock-t16-elastic-20261114/RESULTS.md:68` | MEASURED run and refusal names. |
| T16 schedule equality was measured at L, without a wrap per schedule | same report `:66,98` | MEASURED with explicit boundary. |
| Final universal-key Preview suite and collection settlement pass | No matching final receipt inspected | UNCONFIRMED. Must not appear as complete. |

## 5. Gaps and contradictions

### Missing implementation or verification

1. **R_SL to settlement binding.** `MAP:81` through `:86` lists outcome, program identity, INIT/input binding, private bundle and deployment gaps. The inspected FP1 probe still describes a measurement-only statement, `OPENVM/crates/sherlock-leaf/src/rsl/air.rs:217` through `:242`. A correct argument builder cannot close these constraints. Slice A/B must supply the proved statement before C enables live settlement.
2. **Final-key qualification.** T25, T12 and T23 form the critical chain, `LOOP:47,52`. No new final-key row count, serialized VK, full-chain security gate and complete Preview suite receipt was confirmed. RKY-003..005, SQG-014..016 and STL-002..006 remain open.
3. **Slot-15 integration.** The brief and wiki require slot 15. The inspected wrap template still contains slot 0, `ROOT/tools/sherlock-wrap/src/decode.rs:66`. The T20 contract also has the old anchor layout, `WT/t20-batched/tools/sherlock-sdk/sherlock-ledger.compact:18,35`. A final compiled layout and negative proof test are required.
4. **Collection proving.** T20 builds contract/runtime tests and models. It explicitly lacks L over SET and the foreign-lift circuit delta, `T20/RESULTS.md:70,97`. The seventh digest tag and collection statement must enter the correct rekey and security gates. No one-root-at-scale proof run was confirmed.
5. **Claims data availability.** LDG-005 requires publishing leaf lists and paths. No storage protocol, retention rule, availability test or operator recovery implementation was confirmed. Withholding prevents use but cannot authorize a false member.
6. **Registry updates.** The contract seals reg_root, `T20/RESULTS.md:143`. The batching model does not implement registry mutation, history or grace behavior. No additional ruling is invented.
7. **Chunk planner and final atomicity.** T20 says the planner and universal-key T3 run are absent, `T20/RESULTS.md:126`. Preserve all anchor, prefix/suffix, context and atomicity rules while enforcing K_MAX=2.
8. **Governance deployment.** `T22/REPORT.md:58,59` leaves A1/A2 runtime behavior, A3 wrap-tail binding and the sealed deploy path open. The SDK default committee is not a sealed deployment. Readback must verify the actual installed authority.
9. **Governance verification coverage.** Attestation, signed replay protection, cancel_rotation and use-time fallback lack models, `T22/REPORT.md:53`. End-once and non-overlap lack dedicated mutants. Lean obligations and the Carbon lead verdict remain unconfirmed, `T22/REPORT.md:60` and completion LPC-012.
10. **Era identity with per-program ledgers.** ERT-003 still speaks of one ledger identity per era, while T22/O8 chooses attested per-program ledger roots. Specify how uniqueness applies to the attested ledger set. Sources: `GOV/specs/sherlock-era-table/spec.md:22` and `T22/OPEN-QUESTIONS.md:18`. No production structure settles this yet.
11. **Cross-era duplicate identity.** CRR-005 depends on stable input_pub/outcome_pub across equivalent reproving. Fresh per-run salt can change both. Define whether a retry reuses the logical run's salt or starts a distinct run, and test the distinction. Sources: `GOV/specs/sherlock-consumer-read/spec.md:32`, `COMPLETE/specs/sherlock-zero-knowledge/spec.md:15`, `MAP:73`. This is an interface gap, not a new ruling.
12. **Privacy enforcement.** Case A is confirmed, but no executable ownership/transport guard was confirmed. No-PCS-masking does not protect disclosure of intermediate proofs to untrusted helpers. Completion ZK-014..018 need production CPU/GPU and recovery evidence.
13. **Wallet and recovery coverage.** Existing scrubbing, permissions and locking code supplies partial evidence. Exception redaction, concurrent submissions, ambiguous inclusion and interrupted deployment need fault tests. The exact retry/idempotency contract for ordinary calls remains unspecified.
14. **Public SDK/CLI.** Research scripts exist. A stable developer API, error schema, artifact version protocol and clean-environment workflow are unconfirmed. Factory infrastructure must remain optional for public users.
15. **Observability.** T21 requires a live GUI and daemon bridge, `LOOP:43`. No complete private-data filter, progress-event schema or live status test was confirmed. Logs must not expose witnesses or wallet material.
16. **Final owned-box path.** T17(c) lacks the huginn prover hook and second-program demo in its recorded result, `T17/RESULTS.md:49`. T16 equality evidence ends at L, `RESULTS.md:98`. Final proof, wrap and settlement across the owned-box boundary remain required.
17. **Validium boundary.** T19 requires chained batches, stale-root and invalid-batch refusals, and fixed proof size, `LOOP:41`. It waits for T17(b). No completed application settlement run was confirmed. Slice A owns its program, B owns folding, and C owns root acceptance and receipts.
18. **Numeric governance parameters.** The group confirmation names T22 picks but gives no numeric list, `WIKI/log.md:4107`. Exact P1..P6 confirmation remains UNCONFIRMED. Keep Q, TL, RTL, GRID and SKEW explicit and do not substitute the historical 14 days.

### Contradictory or stale sources

| Topic | Source A | Source B | Required treatment |
|---|---|---|---|
| Anchor slot | `FINAL/specs/sherlock-anchor-chunks/spec.md:4` says cell 0 | `WIKI/log.md:4100,4107` confirms slot 15 | Supersede ANC-001 through ANC-013/014. Recheck contract and wrap together. |
| Lock formula | `FINAL/specs/sherlock-anchor-chunks/spec.md:11` uses P2(DOM_LOCK) | `LOOP:106` and completion audit fix require x_j | Use canonical x_j with k, d_in, d_out and j. |
| Entry VKs | `FINAL/specs/sherlock-midnight-side/spec.md:11` assigns wrap VK to every exported entry | `GOV/specs/sherlock-era-ledger/spec.md:36` requires native read keys | Limit MID-002 to wrap-keyed entries. |
| Collection publication | `FINAL/specs/sherlock-l-and-wrap/spec.md:80` publishes reg_root/set_root | `T20/RESULTS.md:65` uses reg_pub/claims_root | Carry the existing T20 LW-009 amendment. |
| Opening transactions | `WIKI/entities/AcceptRunContract.md:21` retains descend/openClaim and foreign state | `T20/RESULTS.md:62,95` removes them | Treat the wiki entity as historical. |
| Preview evidence | `WIKI/entities/AcceptRunContract.md:16` says no ledger-9.1.0.1 accepted call | `T17/RESULTS.md:19` records inclusion | Use T17's scoped receipt. Do not infer final T25 success. |
| Anchor identity | `WT/t9-contract/tools/sherlock-sdk/sherlock-ledger.compact:14` says not the C_P of any run | `MAP:27,31` derives equality for ordinary calls | Document one ledger per program and correct the misleading comment during implementation. |
| Chunk cap | `T20/RESULTS.md:126` permits raising cap with TTL | Brief hard rules fix K_MAX=2; `LOOP:106` confirms it | Do not implement automatic cap growth. |
| Seal status | T17 successful deploy record `T17/RESULTS.md:16,17` | `T22/REPORT.md:9,59` says default SDK authority is not sealed | A successful key readback is not proof of sealing. Add authority readback. |
| Upgrade model | `WIKI/log.md:3664` describes key replacements | `WIKI/log.md:3670` corrects it to sealed eras | No in-place key replacement. |
| Governance confirmation | `T22/OPEN-QUESTIONS.md:3` and `LOOP:44` say picks await owner | `WIKI/log.md:4107` confirms T22 picks | Use confirmed architecture outcomes and retain the numeric-value evidence limit. |
| Cross-era nullifier | T20 uses claim_pub alone, `T20/RESULTS.md:68` | `GOV/specs/sherlock-consumer-read/spec.md:32` uses lineage/input/outcome | T22 refines cross-era use. Keep same-era T20 controls. |
| T23 builder status | Completion STL-001 says no builder commit/status open | `MAP:3,54,61` records builder and T17 reproduction | Builder evidence exists. Live R_SL binding still remains open. |
| Kit scope | T23 dry run says secp256k1/keccak are deferred, `T23-DRYRUN.md:29,30` in its worktree | `LOOP:46` adds both kits to T24 | Preserve dry-run history. Do not preserve its deferred scope in the prototype spec. |
| Rekey inputs | `LOOP:47` enumerates one rekey list | `T22/REPORT.md:10` requires wrap-tail constants before T12, and `T20/RESULTS.md:97` adds a digest tag | Reconcile all key-changing inputs before T25. Do not assume governance or collections are already included. |
| Scheduling | `FINAL/specs/sherlock-derived-gates/spec.md:29` refers to a plan menu | `LOOP:105` replaces F-3 with setup profiles | Retain shape bounds without a schedule menu in key_root. |
| Blinder lifetime | `LOOP:33` asks for r_P fresh per proof | `WIKI/log.md:4062,4107` confirms per-deployment r_P | Use per-deployment r_P, per-run input salt and per-proof wrap blinders. |

## Integration boundary

Slice C consumes a proved run statement, qualified key manifest, public call vector and owned-box proof result.
It produces a public settlement receipt, contract readback, batch-root receipt or explicit failure verdict.
The ProgramLedger aggregate binds its anchor and era. The EraTable aggregate owns acceptance intervals and the operator roster.
The Consumer aggregate owns the era-stable nullifier. The Wallet adapter owns local signing and must never export its secrets.
The claim-tree membership format is a published interface between proving, settlement and consumers. The public SDK translates this format into contract calls.
These are proposed DDD boundaries for integration. They do not claim that all adapters or aggregates are implemented.

# Position: fable-2 (recursion and proving pipeline, Midnight ledger 9 and ZKIR)

Date: 2026-11-07 (written 20:50Z-21:30Z). Checkout `/home/hoskinson/Projects/Moriarty` at `8fd34ed79`. Written before reading any other member's position.

Every claim names a repository path, a commit or a URL. "Measured" means a recorded run exists in the repository. "Estimated" means arithmetic in the research wiki with no run behind it.

## Summary of the lens

The recursion design is already decided and partly measured. The pipeline is: host run -> one universal digest leaf per S machine steps -> a linear, self-recursive IVC node -> a program-independent run decider -> `SherlockRun.acceptRun` on a ledger-9 contract (`docs/sherlock/stagenet-gp-recursion.md` section 3; `crates/mori-recursion/src/ivc.rs`; `crates/mori-recursion/src/run_decider.rs`; `tools/sherlock-sdk/sherlock-run.compact`). The node and decider have real keys and proofs (M3, M4). The leaf has one measured C6-shaped prototype (M2). No general program has yet been proved end to end. The pipeline is correct in shape and too slow by one to two orders of magnitude for the committed programs. The Midnight pins are three months and 388 commits behind the mainnet ledger. Both facts drive my plan.

## Keep

K1. **The three-relation shape: universal leaf L, linear self-recursive node N, program-independent decider D.** Reasons: one key set serves every program (E2 in `docs/sherlock/stagenet-gp-recursion.md:34-38`); the node is measured at k=20, 531,989 rows, about 95 s per step, 6,432 B proofs, 26.2 GB peak RSS, three folded steps pass and a flipped proof is refused (`docs/sherlock/stagenet-gp-recursion.md:190-193`, commit `588bc5f25`); the decider is measured at k=20, 833,235 rows, about 100 s, 70 public lanes, every lane mutation refused, accepted by the ledger-9 `VerifierKey` reader (`docs/sherlock/stagenet-gp-recursion.md:194-196`). Per-rule leaf keys cannot feed one node, so a universal leaf or a committed finite family is forced (`docs/sherlock/stagenet-gp-recursion.md:138-155`).

K2. **Stock Midnight proof system, no ledger change.** PLONK/KZG over BLS12-381 from `midnight-proofs` 0.8.3, `midnight-circuits`, `midnight-zk-stdlib` (`Cargo.toml:36-39`); Blake2b outer transcript for the ledger (`openspec/changes/sherlock-stagenet-recursive-rollup/fenrir-source-cut.json` pins `transient-crypto/src/proofs.rs:65`); in-circuit KZG accumulation with one deferred pairing in the decider (`crates/mori-recursion/src/run_decider.rs:1-16`, `decider.rs`). No `VerifyProof` primitive and no ledger 10 are required (`openspec/changes/sherlock-universal-interpreter-design/security-requirements.md:12`). The design of record already says this (`docs/sherlock/design.md:13`, `:309`). Ledger 9 keeps the same family: proofs now target zk-stdlib v2 under `ProofVersioned::V3` (https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-9.1.0.0), so the route survives the hard fork.

K3. **Digest-only leaf statements.** Leaf public lanes are `[binding, prog_d, pre_d, post_d, rule, h_pre, steps]` with state digest `Poseidon(SHK_SD, tag, value, lanes_d)` and carriers private (`crates/mori-recursion/src/ivc.rs:57-70`). The reason stands: one carrier side at the draft config is about 35k lanes, above `MAX_CHILD_INSTANCE = 2^14-1` (`docs/sherlock/stagenet-gp-recursion.md:157-163`).

K4. **Ledger-side binding of program and initial state.** `acceptRun` reads `program_d` and `init_d` into the transcript, records `stmt` once in `runs`, and writes the result as a fixed 16-byte big-endian window (`tools/sherlock-sdk/sherlock-run.compact:26-47`). The decider binds those read values, so a proof of another program, input or result is refused by the ledger (`docs/sherlock/stagenet-gp-recursion.md:138-142`). This is the UISEC-010 decider requirement made concrete (`security-requirements.md:79-85`).

K5. **The refusal matrix and independent verification.** Five rejection cases (tampered inner proof, mismatched VK, wrong predecessor root or height, reordered batch, replayed statement) were refused locally and on stagenet for the A chain (`docs/sherlock/design.md:271-276`). A verifier that is not the prover re-verifies the exact submitted bytes (E6, `stagenet-gp-recursion.md:54-59`; `crates/mori-recursion/examples/verify_chain.rs`). Keep both as acceptance conditions for every program.

K6. **Evidence discipline.** No stand-in, mock or estimate counts. `src/standin.rs` is "for rehearsal only" (`stagenet-gp-recursion.md:143-144`). MC03's contract: "Do not reset exhausted contracts or replace a missing proof with a mock" (`openspec/changes/mc03-native-recursive-proof/design.md`). The research consensus agreed only C0, the measured phase profile; every speedup row is estimated and none is agreed (`wiki-llm/recursion-perf/CONSENSUS.md`).

K7. **Resource caps as part of the design.** Heavy work runs under `scripts/doctorwatson heavy` with memory and wall caps; failures are preserved, never reset (`docs/sherlock/HANDOFF-2026-10-26.md`; SRPOC-008 in `openspec/changes/sherlock-stagenet-recursive-rollup/requirements.md`; `qa/sherlock/budgets.json`).

## Improve

I1. **Throughput budget, then measured speedups.** With one step per leaf, fact(3) (293 steps) is about 7.7 h serial (`stagenet-gp-recursion.md:199-201`). The owner's end state needs three programs, one of them fresh (E1-E9). Proposed budget: a committed program of at most 320 machine steps proves, folds, decides and verifies in under 2 h on the 20-core host and under 48 GB peak; P3 under 4 h. The levers in order: S machine steps per universal leaf (`steps` lane already exists, `ivc.rs:62-66`), node rows under 2^19 to reach k=19 (the node is only 1.5% over 2^19, `stagenet-gp-recursion.md:190-191`), two concurrent heavy proofs when the sum fits the cap (estimated 30.7 GB, `wiki-llm/recursion-perf/stages.json` s9), then GPU MSM on Loki. Adopt none of these on the estimate; adopt each after the C0 profile shows its share (`openspec/changes/recursion-phase-profile/design.md`).

I2. **Run the C0 phase profile on the general-purpose workloads.** The profile's workloads are A2, A3, A4 and B3 (`scripts/qa/profile-recursion.py:20-24`), which predate the IVC node and run decider. Tasks 5.1-7.1 (the actual on/off runs) are unchecked (`openspec/changes/recursion-phase-profile/tasks.md`). Add the leaf, the IVC node and the run decider as workloads and run the profile before any accelerator work.

I3. **Make the leaf U/F decision a measured gate.** The rule is: one universal leaf if k is at most 17, else a finite family with a forced uniform `ZkStdLibArch` and k (`stagenet-gp-recursion.md:148-155`). The WIP is on branch `u023-leaf` at `85a27ebc6`, unreviewed (`HANDOFF-2026-10-26.md`). The union leaf (dispatch plus every fragment) has never been keyed; M2 measured only one fragment-shaped leaf at k=13 (`stagenet-gp-recursion.md:186-189`). Write the gate as a requirement with both outcomes and their key tables.

I4. **Bind the qualification host configuration.** Leaves run under a tight qualification host config (frames, data, env, value slots) and the same config feeds E1 (`stagenet-gp-recursion.md:161-163`). I could not find where that config enters `prog_d` or any public lane. If it is not bound, a prover could prove a run under a different config than the one the reference used. Either show that `prog_d` commits the config or add `cfg_d` to the leaf and decider statements.

I5. **Refresh the Midnight pins and re-verify keys.** See "Midnight ledger 9 impact" below. The workspace pins `midnight-ledger` at rev `04c9c5d9` (committed 2026-07-31, 388 commits behind tag `ledger-9.1.0.1` per `gh api .../compare`), `midnight-circuits =7.2.4`, `midnight-zk-stdlib =2.3.5`, `midnight-zkir-v3 3.0.0-rc.2` (`Cargo.toml:28-42`, `Cargo.lock`). Ledger 9.1.0.0 requires `midnight-circuits 7.3.0` and `midnight-zk-stdlib 2.4.0` and ships zkir 3.2.0 (https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-9.1.0.0). Keys for some circuit classes change. Regenerate the node and decider keys at the new pins and re-run the key regression (`crates/mori-recursion/tests/fixtures/stagenet-verifier.json` holds `relation_sha256`, `vk_sha256`, `parent_vk_sha256`).

I6. **Bind the A8 check to the bundle.** `scripts/qa/check-a8.sh` passes on any `ContractCall` at the pinned address. It should also read the post-call state and check that the recorded `stmt` (or the A-chain `validity_ok` member) equals the statement of the submitted bundle. Today A8 fails only because the redeployed anchor at block 1,032,519 (`HANDOFF-2026-10-26.md`) has no call yet (`qa/sherlock/status.json`, A8 exit 1 at 2026-10-26T13:52Z).

I7. **A key manifest for every installed verifier.** Generalize `stagenet-verifier.json` to the run decider: relation SHA-256, VK SHA-256, `midnight:verifier-key[vN]:` header version, `midnight-circuits`/`zk-stdlib`/`proofs` versions, ledger rev, Compact toolchain and runtime versions, template SHA-256. The deploy script must refuse when the manifest and the local keygen differ (the A4 pattern, `docs/sherlock/stagenet-deployment.md`).

I8. **Decider transcript against the new ledger rules.** Ledger 9.1.0.x rejects non-canonical `field` atoms in transcripts, rejects `noop 0`, rejects calls over gas and public-input limits, and prices each segment's writes separately (https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-9.1.0.0 and the 9.1.0.1 notes at https://github.com/midnightntwrk/midnight-ledger/releases). The acceptRun template (70 slots, `crates/mori-recursion/tests/fixtures/run-template.json`) was captured with Compact 0.34.0 (`tools/sherlock-sdk/deploy-run.mjs:3`). Recapture it with Compact 0.36.0 and runtime 0.21.0 and diff the slot layout against `template.rs`.

I9. **The dispatch receipt.** `dispatch_selects_exactly_the_fired_rule_on_factorial` ran past a 16,000 s wall cap (`HANDOFF-2026-10-26.md`). Root-cause the time before raising the cap; a test that needs more than four hours hides a design cost in the dispatch owner that the universal leaf will inherit.

## Cut

C1. **The binary tree of nodes over per-rule leaves.** The original proposal, superseded by the linear IVC node; keep only as a record (`stagenet-gp-recursion.md:146`).

C2. **The A102/A80 two-child parent as a production path.** `crates/mori-recursion/src/parent.rs` (k=20, two literal children, about 16.5 GB) is the A-chain demonstrator. Keep its tests as regression, keep A8 cheap to re-earn on the current anchor, but take it off the general-purpose critical path. Speedup row C1 (fused fixed-pair terminal at about k15) applies only to this literal pair and is cut with it (`wiki-llm/recursion-perf/CONSENSUS.md`).

C3. **The MC03 financial IVC successor and the `midnight-aggregation` route.** `openspec/changes/mc03-native-recursive-proof/` builds a two-step financial IVC in `experiments/moriarty-native-ivc-r3/successor/` on the `midnight-aggregation` traits (`docs/research/2026-09-06-midnight-native-recursion.md`). The Sherlock node implements its own IVC on `midnight-circuits::verifier` gadgets (`ivc.rs:27-35`). Mark MC03 superseded by the Sherlock IVC node; do not depend on `midnight-aggregation` (unreleased on the ledger side; version gap noted in the research note).

C4. **Speedup rows C8 and C10.** C8 backports a BLST fix to 0.8.2; the pipeline is on 0.8.3 (`Cargo.toml:37`). C10 is wallet synchronization and key caching with an estimated 1.07-1.10x on proving (`CONSENSUS.md`); out of the recursion scope.

C5. **The Fenrir rollup B chain as a gate for the Sherlock VM.** B4, B5, B8, B9 measure a 16-transaction synthetic batch leaf (`docs/sherlock/design.md:277-284`). It is a separate workload with its own anchor. Keep it as a parallel experiment; do not let it gate U023, C4 or the consensus change.

C6. **The stand-in leaf from any acceptance path.** `src/standin.rs` and `gp_fold_and_key` stay as pipeline tests. No receipt from them counts (`stagenet-gp-recursion.md:197-198`).

C7. **The segmented-anchor fallback unless the owner chooses it.** Already stated as an explicit trade-off (`stagenet-gp-recursion.md:170-174`). It fails E4 and costs one call per segment.

## Implementation plan (ordered milestones)

Each milestone names its gate. A gate is a recorded run under `scripts/doctorwatson heavy` or a read-only check, never a source review.

**M0. Pin refresh and compatibility gate (first, light and heavy).**
- Bump `Cargo.toml` to the `ledger-9.1.0.1` commit (`a2b3012`), `midnight-circuits 7.3.0`, `midnight-zk-stdlib 2.4.0`, `midnight-zkir-v3 3.2.0`; adapt to `midnight-storage 3.0.0` (`Map` keys need `EncodingShape`; 9.1.0.1 notes).
- SDK root: `@midnightntwrk/midnight-js` 5.0.0, Compact 0.36.0, runtime 0.21.0, proof server 9.0.1 (https://docs.midnight.network/relnotes/support-matrix).
- Keygen the node and the run decider; compare VK and relation hashes with the pre-bump values; record which changed.
- Recapture the acceptRun template; run `validate_call` (A6 pattern) on a local proof-verifying 9.1.0.1 ledger with mainnet `LedgerParameters`; run the 70-lane mutation set again.
- Gate G0: positive local acceptance and the full refusal set at the new pins, with a key manifest (I7).

**M1. U023 scenario 1 and the missing fragments (heavy.lock).**
- Dispatch receipt after root-causing the factorial test time (I9).
- Receipts for KAdminPublishValue, BHelperValidate x4, BHelperEqual x4 (`stagenet-gp-recursion.md:224-243`).
- Gate G1: `doctorwatson check U023` scenario 1 passes; inventory rows flipped with receipts.

**M2. Universal leaf (G4 of the gap list).**
- `sherlock_c6_child.rs`: any C6 `IrSource` to a Poseidon child with the digest statement and `steps >= 1`.
- Measure M2 for the dispatch-plus-fragment union: k, RSS, time. Decide U or F by the k <= 17 rule (I3). Bind `cfg_d` or show it is in `prog_d` (I4).
- Gate G2: one real leaf proof per rule of `entry 7;` verified by a separate process; the union k recorded.

**M3. Node and decider at the new pins.**
- Re-measure M3 and M4. Attempt the k=19 node (rows under 2^19) as an improvement, not a gate.
- Choose S from the measured leaf cost and the node cost.
- Gate G3: `sherlock_ivc` and `sherlock_gp` pass with real leaves for `entry 7;`; the decider proof is accepted by `validate_call`.

**M4. End to end for P1, P2, P3 (E1-E9).**
- `entry 7;` (6 steps), then fact(3), then list123, then P3 written after the keys are frozen.
- Leaves -> node -> decider -> `verify_chain` -> `validate_call` -> deploy SherlockRun -> call -> indexer readback -> `scripts/qa/assemble-u023-record.py`. Negative control: altered result and altered program digest refused locally and on the network.
- Gate G4: U023 scenario 2 record per program; C4 flips.

**M5. Phase profile and throughput (after G4, heavy.lock and gpu.lock).**
- Run the C0 profile on leaf, node and decider (I2). Adopt S, concurrency and GPU MSM only where the measured share justifies them (I1). Record the budget in `qa/sherlock/budgets.json`.
- Gate G5: the 320-step budget met or the shortfall recorded with its measured cause.

**M6. Mainnet readiness.**
- Stagenet runs node `2.0.0-d9729c13`, `specVersion 2000000` (read-only RPC `system_version` and `state_getRuntimeVersion` on 2026-11-07; indexer block 1,211,324 `protocolVersion 2000000`). Mainnet runs node 2.1.101 with ledger 9.1.0.1, `specVersion 2001100` (support matrix; https://github.com/midnightntwrk/midnight-expert/pull/291). Stagenet is therefore a pre-release ledger 9 and is not in Midnight's support matrix; its acceptance does not transfer to mainnet.
- Rehearse on Preview (ledger 9, reset on 2026-11-06, same component versions as mainnet per the support matrix), then deploy SherlockRun on mainnet from a fresh wallet with a new encrypted key backup (`HANDOFF-2026-10-26.md`).
- Rollback: contracts are immutable; the rollback is a new deployment with a new address and manifest. Maintenance updates can replace operation IR (9.1.0.0 notes) but the decider VK is hand-written, so re-deploy rather than patch.
- Gate G6: one accepted call and one refused call on mainnet with indexer readback bound to `stmt`, fees recorded against the price floor.

## Assurance strategy (recursion lens)

Evidence classes, from weakest to strongest; a claim names its class (UISEC-012):
1. Source and model: Quint or K checks, source review. Never cited as proof or acceptance.
2. Local native proof plus independent verification by a separate process over the exact bytes.
3. Local ledger-9 proof-verifying acceptance and the refusal matrix (`validate_call`).
4. Network acceptance with indexer readback bound to the statement, plus one refused call.
5. Measured resources under `doctorwatson heavy` with preserved receipts.

Soundness checks per layer, each with a positive and a negative run:
- Leaf: every public lane mutated alone is refused; rule coverage is 100% of fired rules; `pre_d` of step i+1 equals `post_d` of step i; `steps >= 1`.
- Node: a step relabelled as genesis is refused (the genesis bit); a flipped child proof is refused; `self_vk_repr` cannot be substituted; heights add exactly.
- Decider: the pairing closes; `self_vk_repr`, `init_d`, `prog_d` are pinned; each of the public values mutated alone is refused; the halted-state opening refuses a non-halted digest.
- Contract: replay of `stmt` is refused; a different program or initial digest is refused by the ledger reads; the result window is 16 bytes with a nonzero low byte.
- Keys: the manifest matches the fresh keygen byte for byte before any deploy or call.

Formal coverage: write a small Quint model of the chain invariants (statement linkage, height monotonicity, unique genesis, replay set) in `models/sherlock/` using the pattern of recursion-phase-profile task 1.1 (commit `658edf096`). The leaf relation is checked against the reference machine by differential testing on the C6 witness stream, with `crates/mori-recursion/tests/sherlock_digest_parity.rs` as the oracle bridge (commit `3c2657acb`). K and Quint abstract digests, binding and decider soundness; those remain premises, as `security-requirements.md:24` says.

What does not count as evidence: stand-in bundles; a stagenet `ContractCall` not bound to the bundle; holdout verifier independence by name only (`stagenet-gp-recursion.md:57-59`); estimated speedups; a passing run whose receipt was lost; acceptance on stagenet's pre-release node as mainnet readiness.

## Midnight ledger 9 impact

Primary sources read on 2026-11-07: https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-9.1.0.0, https://github.com/midnightntwrk/midnight-ledger/releases, https://docs.midnight.network/relnotes/support-matrix, https://github.com/midnightntwrk/midnight-expert/pull/291 (a docs PR; fork height from it is not an official announcement), https://github.com/midnightntwrk/servicedesk/pull/288. The owner's statement that ledger 9 is live on mainnet is confirmed: "Ledger 9.1.0.1 ... is already live on mainnet" (releases page, 07 Nov, commit `a2b3012`). Mainnet forked from ledger 8 to ledger 9 at block 3,346,139 on 2026-11-06 according to the docs PR.

| Area | What changed | Effect on Sherlock | Evidence |
|---|---|---|---|
| Proof system | Proofs target zk-stdlib v2 under `ProofVersioned::V3`; `proof_verify` dispatches on version; v1 proofs and keys stay verifiable. Requires `midnight-circuits 7.3.0`, `midnight-zk-stdlib 2.4.0`. | Same PLONK/KZG family; the custom-decider route stands. Workspace pins (`7.2.4`, `2.3.5`, rev `04c9c5d9` of 2026-07-31) must move. Node and decider keys must be regenerated and compared. | 9.1.0.0 notes; `Cargo.toml:28-42` |
| ZKIR | Proof server supports zkir v3 via `midnight-zkir 3.2.0`; default `IrMinorVersion` is V2; `VersionedInnerPK` removed. VKs change for zkir V2 circuits using `hash_to_curve` and zkir v3 circuits that `encode` a Jubjub scalar; V0/V1 keys unchanged; "Affected contracts need their keys regenerated." ZKIR v3 artifacts with a proof server older than 9.0.0 mismatch on every network. | Our leaf is ZKIR v3 to `IrSource` (`stagenet-gp-recursion.md:112-115`; `scripts/qa/prepare-rollup-local.sh:31-45`). The leaf key must be regenerated under zkir 3.2.0 and checked for the Jubjub-encode case. The SherlockRun compiled circuit key is irrelevant (the decider VK replaces it), but the deploy uses the compiled contract's other keys. | 9.1.0.0 notes; servicedesk PR 288 |
| Verifier keys | As above, plus the `midnight:verifier-key[vN]:` header the deploy checks (E7). | Add the header version and the circuits/zk-stdlib versions to the key manifest (I7). | `stagenet-gp-recursion.md:60-64` |
| Contract call semantics | Transcripts must embed canonical field values; non-canonical `field` atoms rejected; `noop 0` rejected; `AlignedValueChecked` in onchain-vm; calls over gas, public-input or Dust-transfer limits rejected; repeated entries rejected; segment writes priced separately; contracts with more than 256 operations rejected. | Recapture the acceptRun template with Compact 0.36.0; confirm the 70 public lanes are under the public-input limit (value not found in the notes; must be read from `LedgerParameters` at the new rev); re-run the A6 local acceptance. | 9.1.0.0 and 9.1.0.1 notes |
| Fees | `parallelism_factor` replaced by `validation_factor`, `guaranteed_factor`, `fallible_factor` (compute cost only); explicit price floor in full blocks; tooling must supply the new `LedgerParameters` fields; Dust registration uses block time. Known issues #838 (one DUST spend can fail `OutsideTimeToDismiss`) and #839. | `tools/sherlock-sdk/producer-policy.mjs` compares `TransactionCostModel`; its fee bounds and the deploy fee checks (`docs/sherlock/stagenet-deployment.md`) need the new parameters. Pay calls with more than one DUST spend until #838 is fixed. | 9.1.0.0 notes |
| Serialization and storage | Non-canonical encodings rejected; `midnight-storage 3.0.0` (`Map` keys implement `EncodingShape`), public Rust API signature changes. | `crates/mori-ledger` and `mori-recursion` `sherlock-ledger` features need source changes at the bump. | 9.1.0.1 notes |
| Networks | Support matrix lists only Mainnet (ledger 9, node 2.1.101, Compact 0.36.0, runtime 0.21.0, Midnight.js 5.0.0, proof server 9.0.0, wallet SDK 2.0.0, indexer 4.4.101), Preview (same, reset as a new ledger-9 chain on 2026-11-06, earlier contracts gone) and Preprod (ledger 8). Stagenet is not listed. | Stagenet (`indexer.stagenet.shielded.tools`, `rpc.stagenet.shielded.tools`, `tools/sherlock-sdk/stagenet-session.mjs:23-27`) answers `system_version 2.0.0-d9729c13`, `specVersion 2000000`. It is a pre-release ledger 9. Use it for cheap iteration; use Preview as the mainnet rehearsal; mainnet last. | support matrix; read-only RPC on 2026-11-07 |
| Recursion on chain | No statement that ledger 9 verifies recursive or aggregated proofs natively. `midnight-zk-stdlib` says it "facilitates recursion" and is unaudited. | The custom decider remains the only route. No change to K2. | https://docs.rs/midnight-zk-stdlib/2.1.0/midnight_zk_stdlib/ |

Stale notes to fix: `wiki/midnight-repositories.md:41` pins `ledger-8` at `a8ab82b`; the code is already on `midnight-ledger-v9` (`Cargo.toml:30`). The brief's statement that "ZKIR v3 is coupled to that ledger revision" is right in form: `midnight-zkir-v3` is a crate inside the ledger repo at the pinned rev (`Cargo.toml:37`), so the ZKIR version moves with the ledger bump.

## Open questions

Q1. Do the `midnight-circuits 7.2.4 -> 7.3.0` and `zk-stdlib 2.3.5 -> 2.4.0` bumps change the constraint systems of our hand-written node and decider? The notes name only two affected circuit classes. Only a keygen regression answers it.

Q2. What is the public-input limit in ledger 9.1.0.1 `LedgerParameters`, and does a 70-lane decider transcript plus the template fit under it and under the gas limit at the price floor?

Q3. Does `prog_d` commit the qualification host configuration (I4)?

Q4. Will shielded.tools upgrade stagenet to 9.1.0.x? If not, stagenet evidence stops at class 4 for a network no user runs.

Q5. What does one mainnet `acceptRun` call cost in DUST at the new price floor with a k=20 proof and 70 public inputs? Unmeasured.

Q6. Why does the factorial dispatch test exceed 16,000 s (I9)?

Q7. Does the owner still want the Fenrir B chain inside the Sherlock VM scope (C5)?

## Confidence

- Keep K1-K7: high. The node and decider are measured with real keys, proofs, refusals and a ledger reader acceptance (`stagenet-gp-recursion.md:184-201`).
- Improve I1 (throughput budget): low on the numbers. The only measured general-purpose cost is 95 s per node step and 100 s per decider; everything faster is estimated (`CONSENSUS.md`).
- Improve I5/I8 and the ledger-9 table: medium. The release notes are primary and were read directly, but no build or proof has been run at the new pins. Compatibility of our hand-written relations is untested.
- Cut C2-C5: medium. These are scope judgments; the owner's priority on the B chain is the main unknown.
- Mainnet path (M6): medium. Stagenet's node version was observed directly; the fork height and date come from a docs PR, not an announcement.

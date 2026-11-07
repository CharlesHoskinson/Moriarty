# Position: fable-1 (chair; lens: VM architecture)

Run `sherlock-vm-council-20261107`. Written 2026-11-07 ~21:10Z before reading any other position. Checkout `/home/hoskinson/Projects/Moriarty` at `8fd34ed79`. Paths below are relative to the checkout unless absolute.

## Summary

Sherlock's VM is sound in shape and the general-purpose recursion chain is designed and partly measured. What blocks it is not a design flaw but (1) an unfinished and unmeasured universal leaf, (2) a quadratic host test that stands in for a receipt, (3) nine fragment gaps that make every real program unsupported, and (4) a dependency set pinned to a ledger-9 release candidate while mainnet is on ledger 9.1.0.1. Keep the machine, the bounded images, the in-circuit dispatch and the digest-leaf / linear-IVC / decider / contract chain. Change the leaf to one universal, one-hot-gated, S-step circuit; replace the exhaustive dispatch test with a bounded receipt; re-pin and re-key for ledger 9.1.0.x; and cut everything that is not on the P1-P3 stagenet path.

## Keep

K1. **The fixed reference machine.** One transition relation, explicit frames, pending returns kept in state, immutable program table, finite fuel; no program-specific unrolling. 39 A events (36 charged, 3 free) in profile `sherlock-k-a/1`; the B owner engine with `RuleIdC` rules in profile `sherlock-helper-microstep/1`; `/2` adds `Await`/`Waiting`/`LogicalCap` (tags 11, 4, 7). Sources: `openspec/changes/sherlock-universal-interpreter-design/design.md:9-11`, `docs/sherlock/design.md:246-250`, `crates/mori-core/src/sherlock/reference.rs`, `crates/mori-core/src/sherlock/helper_machine/shared.rs:400` (RuleIdC).

K2. **Bounded images and the codec laws.** `ProgramImage` / `MachineImage`, the `sherlock-c6-phys/1` lane layout (48-lane image header, 17-lane control, 13-lane frames bottom-to-top; sections META/PROG/CTX/IMAGE/DATA/HELPER), `StepInv`, and the `decodeCanonical` / `decodeStep` / `project` laws. Sources: `openspec/changes/sherlock-universal-interpreter-design/interfaces.md:132-193`, `crates/mori-core/src/sherlock/helper_machine/c6_phys.rs:1-30`, `c6_phys_host.rs:1-30`.

K3. **In-circuit dispatch derived from the pre-state.** The rule is not a prover choice: P0 (Running), P1 (publication), P2 (exhaustion), then `owner_select` with mutually exclusive paths, so at most one rule block is satisfiable on any pre-state. This is the property UISEC-001 needs. Source: `crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:1-37`; `openspec/changes/sherlock-universal-interpreter-design/security-requirements.md:38-41`.

K4. **The GP recursion chain as designed 2026-10-26.** Leaf public statement `[prog_d, pre_d, post_d, rule, h_pre]` with carriers private; state digest `Poseidon(SHK_SD, tag, value, lanes_d)` as an arity-4 tree; linear self-recursive IVC node `[vk_N_repr, prog_d, init_d, cur_d, h, acc]`; run decider with `values = [prog_d, init_d, fin_d, n_steps, result_be]`; `SherlockRun.acceptRun` writing a big-endian `Bytes<16>` result. The ledger-9 custom-decider route needs no `VerifyProof`. Sources: `docs/sherlock/stagenet-gp-recursion.md:106-146`, `docs/sherlock/design/c6-recursion-adapter.md:59-80`, `crates/mori-recursion/src/{ivc.rs,run_decider.rs,decider.rs}`, `security-requirements.md:12`.

K5. **Measured geometry.** Digest leaf stand-in k=13 (2.55 s, 3,824 B); IVC node k=20 (531,989 rows, ~50 s keygen, ~95 s/step, 26.2 GB); run decider k=20 (833,235 rows, ~100 s, 70 lanes all mutation-refused). Source: `docs/sherlock/stagenet-gp-recursion.md:184-201`.

K6. **The host trust boundary.** `HostSnapshot` (`crates/mori-core/src/sherlock/helper_machine/complete_observation.rs:427`) is a host-side diagnostic record produced by `capture(&admitted,&image)` after a real `host_owner_engine().step`; it becomes a *private* C6 witness via `witness_from_host_snapshot` (`c6_dispatch_tests.rs:42-63,102`). Nothing host-side decides acceptance (`openspec/changes/sherlock-machine-circuit-layout/interfaces.md`, "packStepWitnessA must not decide transition acceptance"). The G5 witness stream `moriarty-sherlock-c6-witness-stream/1` and the `sherlock_digest_parity` check tie host digests to the circuit's Poseidon (`docs/sherlock/HANDOFF-2026-10-26.md:27`, `crates/mori-backend/src/sherlock_c6_witness_stream.rs`).

K7. **The requirement base and the evidence-class rule.** UIREQ-001..035 and UISEC-001..012 keep their IDs. UISEC-012: a K trace, a Quint sample or a host comparison is never cited as circuit, proof or ledger evidence. Sources: `requirements.md:5-41`, `security-requirements.md:14-24,92-93`.

K8. **The factory loop.** `scripts/sherlock-next` -> first failing predicate -> `doctorwatson heavy` / `check` -> `stop-rule`. Predicates in `qa/sherlock/predicates.yaml` (sets A, B, C, U, F). Sources: `AGENTS.md:12-19`, `models/doctorwatson/factory/CONTRACT.md`.

## Improve

I1. **One universal leaf, one-hot gated, S steps per leaf.** Today dispatch is "one constant `RuleIdC` per circuit" and each fragment is its own circuit (`c6_dispatch_step.rs:4-6`; `c6-recursion-adapter.md:20-24`). The leaf the node consumes must compute dispatch once, produce a one-hot selector over supported rules, gate every fragment on its bit, and chain S consecutive steps (post_i = pre_{i+1} inside the leaf) so the public statement is `[prog_d, pre_d(0), post_d(S), rules_d, h_pre]` where `rules_d` is a digest of the S selected rules. Target S=8: fact(3) drops from ~293 node proofs (~7.7 h serial) to ~37 (~1 h). Decision rule stays: option U if leaf k <= 17, otherwise family F with forced uniform `ZkStdLibArch` and a committed key table (`stagenet-gp-recursion.md:150-155,164-169,199-201`). Risk: the 16-lane-per-frame mux over the full PROG parser walk may push the universal leaf past k17; M1 (lane and row count per fragment) must run first.

I2. **Replace the exhaustive dispatch test with a bounded receipt.** `dispatch_selects_exactly_the_fired_rule_on_factorial` runs `ProofPreimage::check` for the fired rule on all 293 steps, all supported rules every 16th step, and 2-5 neighbours otherwise, each over a separately built circuit (`c6_dispatch_tests.rs:89-139`). It ran >16,000 s and ~50 GB and produced no receipt (`qa/sherlock/runs.jsonl` run `20261026T105549Z-U023-dispatch-receipt`, exit 4; `HANDOFF-2026-10-26.md:15-16`). No timing per circuit is recorded. Change: (a) measure build and check time per rule circuit once (M1'), (b) split into a T1 test (fired-rule acceptance on every step, mutation refusal on 40 steps) and a T2 test (full exclusivity on a committed seeded sample of 24 states), (c) give the receipt a wall budget under 2 h, (d) record the IR instruction count per circuit in the receipt. An exhaustive host check is not a soundness proof (UISEC-012), so exhaustion buys nothing here.

I3. **Close the fragment gaps before any leaf keygen.** `KAdminPublishValue`, `BHelperValidate` x4, `BHelperEqual` x4, `BHelperMatch` x10, `BHelperScalarConstruct` have no inventoried fragment; every program publishes its value, so no program can qualify (`stagenet-gp-recursion.md:224-243`; `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs` `gap(...)` rows). "A rule with no fragment means the program is unsupported" (E3) stays.

I4. **Freeze the qualification host config and layout as one versioned artifact.** Leaf key identity = (relation id, `sherlock-c6-phys/1`, `LayoutSchema` SHA-256, `Bounds`, ZKIR version, zk-stdlib and circuits versions). `prog_d` must include the layout digest so a program image under a different layout cannot reuse a proof. Today the caps are "being frozen with gpr" (`c6-recursion-adapter.md:78-80`).

I5. **Make the witness stream the only producer.** The holdout engine (`mori simulate`), the leaf prover and the digest-parity test must all consume `moriarty-sherlock-c6-witness-stream/1`. `HostSnapshot` stays host-only and never appears in a public statement.

I6. **Re-pin to the ledger 9.1.0.x line and regenerate every key.** See "Midnight ledger 9 impact".

I7. **Workflow on the factory, not on the retired plan engine.** `workflow.md` must compile to `qa/sherlock/predicates.yaml` rows (id, set, tier, check, depends, source) and the existing `check` / `heavy` / `next` / `stop-rule` commands. Needed additions: a `workflow validate` subcommand or a committed script that checks every `source:` anchor exists, and a per-change set prefix (the `SETS` tuple is closed to A|B|C|U|F, `models/doctorwatson/factory/predicates.py:11`).

I8. **A-chain and B-chain re-earned on the new contracts** before any new native work is accepted. `status.json` (2026-10-26T13:52Z) shows A 7/10, B 4/9, C 5/6, U 34/35, F 11/13; A8 fails because the redeployed `SherlockAnchor` (`2391d62f…`, block 1,032,519) has only a `ContractDeploy` (`scripts/qa/check-a8.sh`; `docs/sherlock/stagenet-deployment.md:282-300`).

## Cut

C1. Per-rule leaf keys (option F) as the primary path; keep only as the measured fallback of I1.
C2. The original binary-tree-of-nodes proposal and the sequential-anchored-segments fallback as E4 paths (`stagenet-gp-recursion.md:146,170-174`); document as fallbacks with an explicit owner trade-off, do not build.
C3. The fixed A102/A80 literal chain (`sherlock_child.rs`, `sherlock_link.rs`, `parent.rs` two-child relation) as a deliverable; it stays as regression evidence for A1-A10 only. No new work on it.
C4. Candidate B "realization comparisons / measured winner" (universal vs per-program vs family residualization) from the stagenet milestone (`design.md:88-89`). UIREQ-033 is closed on host; no circuit-cost winner is needed for v1.
C5. Speedup rows C1-C10 of `wiki-llm/recursion-perf/CONSENSUS.md` until the C0 profile exists (only C0 is agreed there).
C6. The MC03 financial IVC package (`openspec/changes/mc03-native-recursive-proof/`, every task unchecked, R3 blocked at k17) is superseded by the Sherlock GP chain.
C7. `/2` native constraints (Await, Waiting, LogicalCap) and `sherlock-run-anchor` are out of the P1-P3 milestone; `/1` is the qualification profile.
C8. The legacy `scripts/doctorwatson.sh` plan engine; not revived.

## Implementation plan (ordered milestones)

| M | Deliverable | Exit gate (predicate) |
|---|---|---|
| M0 Re-pin | Cargo pins to `midnight-ledger-v9` 9.1.0.1 line, zk-stdlib 2.4.0, circuits 7.3.0, zkir-v3 3.2.0; SDK to ledger-v9 1.0.0, onchain-runtime-v4 4.0.0, Midnight.js 5, Compact 0.36.0; all VKs regenerated; local ledger-9.1.0.1 acceptance and refusal (A6 pattern) | A2-A4 re-pass under the new pins; `validate_call` refuses 5 mutations |
| M1 Fragments + receipt | I3 gap closure with per-fragment receipts; I2 bounded dispatch receipt; `native_inventory` has zero gaps for P1, P2 | U023 scenario 1 passes; receipt run < 2 h T2 |
| M2 Universal leaf | M1' lane/row measurement; I1 universal one-hot leaf, S=1 then S=8; real keygen/prove/verify for every step of `entry 7;` and fact(3) | leaf k recorded; U/F decision committed; `sherlock_digest_parity` T0 |
| M3 Local chain | Node N over real leaves, run decider, `SherlockRun` local deploy/call; independent `verify_chain` on exact bytes; 5 negative controls | E1-E6 for P1 and P2 locally |
| M4 Stagenet | Deploy `SherlockRun` (Compact 0.36.0), call, indexer readback for P1, P2, P3 (P3 written after keys freeze); A8/A9/A10 and B3-B9 re-earned | U023 scenario 2 record; C4 flips; A 10/10, B 9/9 |
| M5 Mainnet readiness | Fee model (min_block_price 10; validation/guaranteed/fallible factors), 256-op cap, `TooManyPublicInputs` check, maintenance authority for VK rotation, DUST registration; Preview (new chain) acceptance before mainnet | owner decision to deploy on mainnet; rollback = maintenance-authority key removal + redeploy |
| M6 /2 profile | Await/Waiting native constraints; `sherlock-run-anchor` | later change |

## Assurance strategy (VM lens)

The equivalence chain and what each link may claim:

1. Source -> `ProgramImage`: compiler mapping report (UIREQ-022, closed on host) plus two contrasting committed programs; claims admission only.
2. `ProgramImage` -> reference steps: K (`models/sherlock/k/main.k`) and Quint (`models/sherlock/quint/universalMachine.qnt`, `kProfileMachine.qnt`) against frozen oracles (`models/sherlock/oracles/`, `crates/mori-core/src/sherlock/normal_oracles.rs`); `model_package.py projection` (UIREQ-021). Claims: finite, sampled semantic agreement. Never circuit evidence.
3. Reference steps -> C6 witness: `sherlock_digest_parity` and the whole-host Quint codec checks (4,697 checks, `models/sherlock/quint/c6-positive-whole-host`). Claims: the host encodes what the machine did.
4. C6 witness -> leaf: differential test on every step of P1-P3: the fired rule's block accepts, every other rule's block rejects on a seeded sample, mutated lanes reject; then real keygen/prove/verify. `ProofPreimage::check` alone is a nonproving host check and is not a proof (`stagenet-gp-recursion.md:96`).
5. Leaf -> node -> decider: link checks (post = next pre, h+1, program digest), flipped-proof refusal, 70-lane decider mutation refusal (`stagenet-gp-recursion.md:190-196`).
6. Decider -> ledger: local ledger-9.1.0.1 refusal (A6), stagenet refusal (A9), indexer readback (E9).

Does not count as evidence: stand-in leaf bundles (`crates/mori-recursion/src/standin.rs`), receipts from the lost host, `checkStepInv` (host-only), any claim about a profile other than the one proved, silence or a `Result=success` from a systemd unit whose wrapper exits 0 (`HANDOFF-2026-10-26.md:16`).

Formal coverage I would add: a Quint model of the one-hot dispatch (exactly one selector bit set for every reachable pre-state; zero bits when `enable = 0`) checked against the owner engine's event stream for P1-P3; and a K/Quint statement that the S-step chained leaf's internal links equal S single steps.

## Midnight ledger 9 impact

Facts (primary sources via WebFetch 2026-11-07; the midnight-scout refresh had not posted when this was written):

- The repo already targets ledger 9, but a release candidate: `Cargo.toml:28-42` pins `midnight-ledger` rev `04c9c5d9` (dated 2026-07-31, "p256-in-zkir"; `repos/midnightntwrk/midnight-ledger` git log) as package `midnight-ledger-v9`, `midnight-proofs =0.8.3`, `midnight-circuits =7.2.4`, `midnight-zk-stdlib =2.3.5`, `midnight-curves =0.3.1`; `crates/mori-ledger/src/profile.rs:35` pins `("zkir-v3","3.0.0-rc.2")`; the SDK seam used `@midnightntwrk/ledger-v9` 1.0.0-rc.3, proof server `9.0.0-rc.5`, `onchain-runtime-v4` 4.0.0-rc.3 (`docs/sherlock/midnight-sdk-seam.md:4`, `HANDOFF-2026-10-26.md:10,45`). `wiki/midnight-repositories.md` (ledger-8 `a8ab82b`) is stale relative to the code.
- Ledger 9.1.0.0 (tag, commit `31c90b8`, 2026-11-04) and 9.1.0.1 (`a2b3012`, security patch, published 2026-11-07): https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-9.1.0.0 and `.../ledger-9.1.0.1`. Mainnet moved to 9.1.0.1 with node 2.1.101 on 2026-11-06; Preview was relaunched as a new chain (old state gone); Preprod stays on ledger 8.1.3: https://docs.midnight.network/relnotes/ledger-9.md, https://docs.midnight.network/relnotes/support-matrix.
- Proof system: proving and verification move to `midnight-zk-stdlib` v2 2.4.0 and `midnight-circuits` 7.3.0; proofs carried as `ProofVersioned::V3`; `ContractOperation` gains a v3 field holding zk-stdlib v2 verifier keys. ZKIR stays v3 (midnight-zkir 3.2.0). **Verifier keys change for zkir v3 circuits that encode a Jubjub scalar**; affected contracts must regenerate keys. (9.1.0.0 release page.) Sherlock uses `JubjubPoint` IR types, so re-keying is likely; not verified per circuit.
- Contract calls: `ContractCall::public_inputs` returns `Result` (`TooManyPublicInputs`); contracts capped at 256 operations; transcripts must embed canonical field values; `noop 0` rejected; gas over block limits rejected; cross-contract-call fix needs Compact 0.36.0 recompile (9.1.0.1 page; ledger-9.md).
- Fees: `min_block_price` floor 10; `parallelism_factor` replaced by `validation_factor` (1/4), `guaranteed_factor`, `fallible_factor`; per-segment write pricing; deploys charged for full state; DUST reset once on mainnet (ledger-9.md; 9.1.0.1 page).
- Toolchain: Compact 0.36.0 (language 0.28.0), Midnight.js 5.0.0 (ESM only, `providers.config{networkId,ttlSeconds}` required, `setNetworkId()` removed), Wallet SDK 2.0.0, proof server 9.0.0/9.0.1, indexer 4.4.101 (support-matrix).

Consequences for Sherlock:

1. `stagenet` is `indexer.stagenet.shielded.tools` / `rpc.stagenet.shielded.tools` with network id `stagenet` (`tools/sherlock-sdk/stagenet-session.mjs:24-31`). It is not an official Midnight network; its ledger version after the hard fork is unknown and must be queried before any M4 call. The A8 indexer still answers for block 1,032,519 (run 2026-11-07), so its state was not reset.
2. The mainnet path needs M0: every VK (leaf, node, decider, contract) regenerated under zk-stdlib 2.4.0 / circuits 7.3.0; the decider's hand-written VK re-emitted with the new `midnight:verifier-key[vN]:` header; `SherlockRun` compiled with 0.36.0; `setNetworkId` removed from `rollup-prepare.mjs:59`, `settle-rehearsal.mjs:35` and the test harnesses; the 70-lane decider statement checked against `TooManyPublicInputs`.
3. The ZKIR v3 leaf design survives (no v4). The IVC `inner.rs` mirrors "ledger-10 verify_proof" code on released crates (`crates/mori-recursion/src/inner.rs:4-7`); it must be re-validated against circuits 7.3.0.
4. The decider's Blake2b transcript vs the Poseidon child transcript (`docs/sherlock/stagenet-recursive-rollup-experiment.md:17`) is unchanged by ledger 9.1.0.x as far as the release notes say; verify on re-pin.

## Open questions

Q1. Universal leaf k: is the one-hot gated union of ~46 fragments plus the PROG parser walk under k17? Only M1'/M2 answer this.
Q2. Which ledger version does `stagenet.shielded.tools` run today, and will it track 9.1.0.1? If not, stagenet becomes a dead end and Preview (new chain) is the pre-mainnet target.
Q3. Does the Jubjub-scalar VK change hit the C6 leaf, the IVC node or the decider? Needs a per-circuit check after re-pin.
Q4. Why does one `ProofPreimage::check` of a dispatch circuit cost minutes? No per-circuit timing exists; is it IR size, SDK check cost or the per-rule circuit cache (46 circuits resident, ~50 GB)?
Q5. S-step chained leaf: does the per-step carrier duplication (S copies of IMAGE/DATA/HELPER lanes as private witness) stay within prover memory at the qualification config?
Q6. Rollback on mainnet: a contract without a maintenance authority cannot rotate a VK (ledger-9.md). Does `SherlockRun` deploy with one?

## Confidence

- Keep K1-K8: high. They are implemented, measured or already the design of record, and the hostile cases that matter (UISEC-001/004/009/010) depend on them.
- Improve I1 (universal one-hot S-step leaf): medium. The shape follows from E2 and the node's single fixed leaf key, but k is unmeasured; F remains the fallback.
- Improve I2 (bounded receipt): high. The current test is quadratic in steps x rules and no soundness claim rests on exhaustiveness.
- Midnight impact: medium-high on the facts (primary release pages fetched), low on per-circuit VK impact and on stagenet's version.
- Milestone ordering: medium. M0 before M1 is the contentious choice; it costs time now but avoids re-keying twice.

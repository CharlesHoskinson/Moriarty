# Proposed SRC (recursion) clauses for the chair

fable-2, 2026-11-07 21:10Z. Proposed text for `openspec/changes/sherlock-vm-consensus-20261107/requirements.md` and `specs/recursion/spec.md`. IDs follow the chair's ISSUE 8 scheme. Each clause names the EARS form, the source finding and an acceptance check that is a recorded run, not a review. Paths relative to the checkout at `8fd34ed79`. None of these clauses claims the code satisfies it today.

## SRC-001 Leaf statement schema (Ubiquitous)

The leaf, the node and the run decider SHALL consume one versioned leaf statement `sherlock-leaf-stmt/1 = [binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]` (seven field lanes), and a producer or consumer SHALL refuse any other lane count or order before proving.

- Source: `crates/mori-recursion/src/ivc.rs:57-66,352` (seven lanes consumed); `crates/mori-backend/src/sherlock_c6_child.rs:817` (six produced); `tests/sherlock_c6_child.rs:107`; `tests/sherlock_gp.rs:184`. Refines UIREQ-023, UISEC-010; adopts SND-001.
- Acceptance: a real C6 export fed to the node harness is refused with a shape error until the bridge exists; after the bridge, one leaf per rule of `entry 7;` verifies under the node with all seven lanes constrained; `rules_d` equals `rule` when `steps = 1`.

## SRC-002 Program identity commits layout and bounds (Ubiquitous)

`prog_d` SHALL be the Poseidon digest of the admitted program image together with the `sherlock-c6-phys/1` layout schema SHA-256, the qualification bounds (frames, data, env, value slots) and the semantic profile id, so that a proof under one layout or bound set cannot be accepted for another.

- Source: `docs/sherlock/stagenet-gp-recursion.md:161-163` (qualification config feeds E1 and the leaves); fable-1 I4; my I4. Refines UISEC-001, UISEC-009.
- Acceptance: the same program image under two bound sets yields two `prog_d` values; a deployment pinned to one refuses a proof built under the other (`validate_call` refusal).

## SRC-003 In-leaf constraint of rule and step lanes (Ubiquitous)

The leaf relation SHALL constrain `rules_d` to the digest of the dispatch selector outputs of its `steps` internal transitions, SHALL constrain `steps` to the number of owner_step events it proves (free terminal publication included, padding excluded), and SHALL chain `post_i = pre_{i+1}` internally for `1 <= i < steps`.

- Source: `crates/mori-backend/src/sherlock_c6_child.rs:598-608` (`rule`, `h` published with `Impact`, no `ConstrainEq`); ISSUE 5 chair proposal. Refines UISEC-001, UISEC-006; adopts SND-001/003.
- Acceptance: a leaf proved with a `rules_d` or `steps` that differs from its internal transitions is refused by the keyed relation (fresh malicious witness, not a public-input flip); a leaf whose internal chain is broken is refused.

## SRC-004 Bounded height arithmetic (Unwanted behaviour)

If `prev_h`, `steps` or `h = prev_h + steps` is not below `2^64`, or `steps = 0`, the node relation SHALL be unsatisfiable.

- Source: `crates/mori-recursion/src/ivc.rs:308-310` (only `steps != 0`); pattern `crates/mori-recursion/src/parent.rs:665-670` (`assert_lower_than_fixed(2^64)`). Adopts SND-003; astra-2 finding 4.
- Acceptance: raw witnesses with `steps = 0`, `steps = p - 1`, `prev_h = 2^64 - 1`, `h >= 2^64` each fail synthesis or verification at the node key, recorded with ASR-003 outcome classes.

## SRC-005 Genesis is canonical (Unwanted behaviour)

If `prev_h = 0`, the node SHALL require `prev_d = init_d` and SHALL fold the carried accumulator scaled by zero (the trivial accumulator); a nontrivial carried accumulator at genesis SHALL be unsatisfiable.

- Source: `crates/mori-recursion/src/ivc.rs:311-314` (genesis = `is_zero(prev_h)`, `prev_d = init_d` enforced), `:318-327` (`prev_acc` assigned from witness), `:343` (`proof_acc` scaled by `not_genesis`), `:364` (`prev_acc` folded unscaled). Refines UISEC-010 "genesis SHALL be an authenticated base case"; astra-2 finding 4.
- Acceptance: a genesis node proof with a malformed carried accumulator is refused through the final pairing in the run decider; a valid genesis with the trivial accumulator passes.

## SRC-006 Decider pins and discharges (Ubiquitous)

The run decider SHALL verify the last node proof under the node key fixed as constants, SHALL pin the node's `self_vk_repr` lane to that key, SHALL fold and pair the carried accumulator in circuit, SHALL open `fin_d` as a halted state with a range-checked `u128` result, and SHALL expose exactly `[prog_d, init_d, fin_d, n_steps, result_be]` in the `acceptRun` transcript.

- Source: `crates/mori-recursion/src/run_decider.rs:1-16,50-60,201,229,278`; `tools/sherlock-sdk/sherlock-run.compact:1-15,42-47`. Refines UISEC-010 (decider discharges every deferred obligation); adopts SND-004.
- Acceptance: each of the public values mutated alone is refused; an outer-only proof (decider skipped) is not reported final-valid; a non-halted `fin_d` is refused. Refusals counted under ASR-003 classes.

## SRC-007 Ledger binding and replay (Ubiquitous)

`acceptRun` SHALL read `program_d` and `init_d` from sealed ledger state into the transcript, SHALL refuse a `stmt` already in `runs`, and SHALL write the result as a 16-byte big-endian window whose low byte is nonzero.

- Source: `tools/sherlock-sdk/sherlock-run.compact:19-47`; `docs/sherlock/stagenet-gp-recursion.md:138-142`. Refines UISEC-011 (unique consumption).
- Acceptance: the A9 pattern on the target network: a replayed `stmt`, a proof for another program digest and a proof for another initial digest are each refused; the accepted call's post-state `stmt`, `steps` and `result` match the submitted bundle (manifest-bound, ASR-005).

## SRC-008 Verifier manifest (Event-driven)

When a leaf, node, decider or contract key is generated or installed, the toolchain SHALL write a manifest with relation SHA-256, VK SHA-256, `midnight:verifier-key[vN]:` header version, `midnight-proofs`/`midnight-circuits`/`midnight-zk-stdlib`/`midnight-zkir` versions, ledger crate revision, Compact toolchain and runtime versions, SRS identity and template SHA-256; a deploy or call SHALL refuse when the fresh keygen differs from the manifest.

- Source: `crates/mori-recursion/tests/fixtures/stagenet-verifier.json` (partial: address, vk, parent_vk, relation); `docs/sherlock/stagenet-deployment.md` A4 pattern; fable-3 I8 `target-pins.json`. Adopts SND-005, ASR-002.
- Acceptance: changing any one manifest field makes `deploy-run.mjs` and `check-a4`-style preflight refuse; the manifest is committed with the key file.

## SRC-009 Re-key closure after a pin change (Event-driven)

When the ledger revision, `midnight-circuits`, `midnight-zk-stdlib` or `midnight-zkir` pin changes, the toolchain SHALL regenerate and hash-compare every relation (leaf, node, decider, parent, contract) and SHALL regenerate the dependent closure of any changed VK in the order leaf -> node -> decider -> contract template, because the leaf key is a node circuit constant and the node key is a decider circuit constant.

- Source: `crates/mori-recursion/src/ivc.rs:10-15` (leaf key constant in node); `run_decider.rs:5-6` (node key constant in decider); https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-9.1.0.0 ("Affected contracts need their keys regenerated"). ISSUE 2 chair proposal.
- Acceptance: the keygen regression at the 9.1.0.1 pins records, per relation, `unchanged` or `changed` with both hashes; `validate_call` on a local proof-verifying 9.1.0.1 ledger accepts the regenerated bundle and refuses the five mutations.

## SRC-010 Independent verification of the exact bundle (Ubiquitous)

Before any network call, a verifier process that is not the prover SHALL re-verify the exact leaf, node and decider proof bytes to be submitted, SHALL reconstruct the accumulator and SHALL check every link (`post_d = next pre_d`, heights, `prog_d`); the record SHALL state that the verifier shares the `midnight-proofs` implementation and is therefore an artifact-handling check, not a second cryptographic implementation.

- Source: `docs/sherlock/stagenet-gp-recursion.md:54-59` (E6); `crates/mori-recursion/examples/verify_chain.rs`; astra-1 "second process, same library" precision. Refines UISEC-012.
- Acceptance: the E6 record names the verifier binary, its library versions and the bundle hashes; a one-byte mutation of any proof in the bundle is refused by that verifier.

## SRC-011 Throughput target (Optional, measured)

Where the owner enables the throughput target, a committed program of at most 320 owner steps SHALL prove, fold, decide and independently verify in under 2 h wall and under 48 GB peak RSS on the 20-core host, or the shortfall SHALL be recorded with its measured phase cause from the C0 profile; a recorded shortfall is not a pass.

- Source: `docs/sherlock/stagenet-gp-recursion.md:199-201` (7.7 h at S=1); `wiki-llm/recursion-perf/CONSENSUS.md` (only C0 agreed); `openspec/changes/recursion-phase-profile/design.md`. ISSUE 9 chair proposal.
- Acceptance: `qa/sherlock/budgets.json` rows for leaf, node and decider with measured values; the C0 profile run on the `entry 7;` chain before any S, concurrency or GPU choice.

## SRC-012 Resource caps are hard limits (Unwanted behaviour)

If a leaf, node or decider run exceeds its `doctorwatson heavy` memory or wall cap, the run SHALL stop, the terminal result and consumed resources SHALL be preserved, and no counter SHALL be reset, no k increased and no passing profile substituted.

- Source: SRPOC-008 in `openspec/changes/sherlock-stagenet-recursive-rollup/requirements.md`; `docs/sherlock/HANDOFF-2026-10-26.md` (16,000 s wall cap, exit 4). Refines UISEC-006.
- Acceptance: a run that trips the cap appears in `runs.jsonl` with exit 4 and a preserved trace; the next run carries a `--hypothesis`.

# Design: Sherlock VM and recursion (consensus 2026-11-07)

Status: DRAFT v2.1 (revision round 1 plus cleanup, 2026-11-07 21:30Z). Specifies; executes nothing. Section numbers are referenced from `requirements.md`, `tasks.md` and `traceability.md`. Base `8fd34ed79`.

## 1. Decisions at a glance

| # | Decision | Keep / Improve / Cut | Rationale (evidence) |
|---|---|---|---|
| D1 | One fixed reference machine: 39 A events in `sherlock-k-a/1`; owner engine `RuleIdC` rules in `sherlock-helper-microstep/1`; explicit frames, pending returns in state, immutable program table, finite fuel | Keep | design of record `openspec/changes/sherlock-universal-interpreter-design/design.md:9-11`; implemented `crates/mori-core/src/sherlock/reference.rs`, `helper_machine/shared.rs:400`; `docs/sherlock/design.md:246-250` |
| D2 | Bounded images `ProgramImage`/`MachineImage`, lane layout `sherlock-c6-phys/1`, `StepInv`, codec laws (`decodeCanonical`/`decodeStep`/`project`) | Keep | `interfaces.md:132-193`; `c6_phys.rs:1-30`; `security-candidate-b.md:A4` |
| D3 | Dispatch derived in-circuit from the pre-state (P0/P1/P2 then `owner_select`, mutually exclusive paths) | Keep; Improve to one-hot | `c6_dispatch_step.rs:1-37`; UISEC-001 |
| D4 | Leaf = one universal relation: dispatch once -> one-hot selector -> every fragment gated; S owner steps per leaf; option F (finite family, uniform arch, authenticated in-circuit key membership) as the measured fallback | Improve | per-rule leaf keys cannot feed one node (`stagenet-gp-recursion.md:138`); U if k <= 17 else F (`:150-155`); S=1 is the qualified baseline, S=8 an experiment (ISSUE 6) |
| D5 | Leaf statement `sherlock-leaf-stmt/1 = [binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]`, seven lanes, one producer and one consumer | Improve (interface fix) | producer six lanes `sherlock_c6_child.rs:817-834` vs consumer seven `ivc.rs:191-201,352`; `rule`/`h` lanes unconstrained `sherlock_c6_child.rs:598-608` |
| D6 | Linear self-recursive IVC node, run decider, `SherlockRun.acceptRun` | Keep; Improve (range checks, genesis accumulator) | measured k=20 node and decider `stagenet-gp-recursion.md:184-201`; `ivc.rs:308-310` no range check; `ivc.rs:318-364` `prev_acc` folded unscaled at genesis |
| D7 | Host boundary: `HostSnapshot` host-only; witness stream `moriarty-sherlock-c6-witness-stream/1` the single producer; digests only public | Keep | `complete_observation.rs:427`; `sherlock_c6_witness_stream.rs`; `c6_dispatch_tests.rs:42-63,102` |
| D8 | Profile `/1` is the qualification profile; `/2` (Await/Waiting/LogicalCap) native constraints are a later change | Cut from this milestone | `interfaces.md:315-342` |
| D9 | Fragment gaps closed before leaf keygen; a rule without a fragment makes the program unsupported | Improve | `stagenet-gp-recursion.md:224-243`; `native_inventory.rs` `gap(...)` rows |
| D10 | Dispatch receipt by complete dispatch-predicate partition plus directed cases, under budgets | Improve | `c6_dispatch_tests.rs:89-139`; run `20261026T105549Z-U023-dispatch-receipt` exit 4 (ISSUE 4) |
| D11 | Re-pin to the ledger 9.1.0.1 line; keygen regression; regenerate the affected closure | Improve | `Cargo.toml:28-42`; https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-9.1.0.0 |
| D12 | Preview (genesis-pinned) is the qualification network; stagenet rows frozen as history; Mainnet an owner-authorized T4 operation | Improve | RPC reads: stagenet specVersion 2000000, Preview 2001100 (`fable-3/position.md` §6, `astra-1/target-observations.md`) |
| D13 | A102/A80 two-child parent, tree-of-nodes, anchored segments, MC03 route, recursion-perf C1-C10 as plan items, legacy `doctorwatson.sh` | Cut (recorded as history/fallback) | `proposal.md` supersession table |
| D14 | B (Fenrir rollup) predicates kept, re-targeted, outside the U023/C4 dependency chain | Keep (non-gating) | ISSUE 7 |

## 2. VM design

### 2.1 Machine model (unchanged)

The source fragment is literals, variables, let, checked UInt128/Bool primitives, conditional, named call, constructor and ordered match (`design.md:11`). State is control, environment, ordered frames, values, outcome, fuel and counters (`interfaces.md:55-88`). A step is one owner-engine event; helpers in `/1` are explicit bounded microsteps with cursors (`interfaces.md:92-96`). Outcomes: `Running | Halted(v) | Rejected(code) | Exhausted(kind)`. Error codes 4 (frame cap), 9 (no match), 10 (checked arithmetic) are fixed (`interfaces.md:7`).

Physical layout `sherlock-c6-phys/1`: image header 48 lanes, control 17 lanes, frames 13 lanes each bottom-to-top, sections META/PROG/CTX/IMAGE/DATA/HELPER (`c6_phys.rs:1-30`, `c6_phys_host.rs:1-30`). `StepInv` holds between canonical boundaries: data arena append-only and child-before-parent, frames and helper slots zeroed on release, inactive fields zero (`interfaces.md:177-193`). `decodeCanonical` only at genesis, `ChunkStore`, checkpoint resume and external claims; `decodeStep` mid-run. Internal microsteps preserve `StepInv` without canonical projection (astra-2 SND-002; `security-candidate-b.md:A4`).

### 2.2 Dispatch (one-hot)

Today the dispatch owner emits a block for one constant `RuleIdC` and each fragment is its own circuit. In the universal leaf the dispatch is computed once per step:

1. Inputs: the pre carriers (IMAGE, HELPER, DATA where a value read is gated on, PROG via the parser walk, CTX).
2. Derivation, in the order Core uses (`c6_dispatch_step.rs:9-16`): P0 `outcome == Running`; P1 publication (`Error` control -> `KAdminPublishError`; `Return` at depth 0 -> `KAdminPublishValue`); P2 `remaining_work == 0` -> `KAdminPublishExhausted`; else `owner_select` over helper group, result-reading rule, helper plan entry, legacy group.
3. Output: a one-hot vector `sel[r]` over the supported rule set `R` with `sum(sel) == enable` and each `sel[r]` boolean; `sel[r] == 1` iff rule `r`'s guard path holds. Guard paths include every value read the current per-rule blocks make (for example `equal_differ` reads UInt/Bool/type/constructor values, `c6_dispatch_step.rs:750-806`).
4. Every fragment `F_r` is emitted once and gated: its constraints are multiplied by `sel[r]`, and it produces a candidate post image `cand_r` in which lanes outside its declared write set equal the pre lanes. The step's post image is the selection `post = sum_r sel[r] * cand_r + (1 - sum_r sel[r]) * pre`. Inactive fragments therefore never constrain the shared post image (astra-2 early correction: a `post == pre` constraint under `1 - sel[r]` would block the selected fragment's writes). Test: two rules with differing write sets on the same class, valid differing writes accepted.
5. Two cases are distinct. A *disabled slot* (`enable == 0`, used for padding a short chunk) accepts the constrained identity `post == pre` and contributes zero to `steps`. An *enabled* step whose Core selection is outside the supported set (extension, financial, `XAdtHelperPlan*`, `XAdtHelperBuild*`; `dispatch_supported`, `c6_dispatch_step.rs:29-32`) has no `sel` bit, so `sum(sel) == 0` with `enable == 1` and the leaf refuses (SVM-004).
6. `rules_d` (D5) is the Poseidon digest of the S selected rule ids in order; for S=1 it is the rule id lane the node calls `rule`. The leaf derives `rules_d` and `steps`; `h_pre` is bound by the node to the bounded previous height, not by the leaf (astra-2 lane ownership).

Soundness premises carried from the per-rule design: canonicality of the pre IMAGE/HELPER/DATA sections is enforced by the gated fragment on the same lanes; PROG canonicality by the parser walk. Putting dispatch and fragments inside one key makes the "caller obligation" at `c6_dispatch_step.rs:34-38` checkable, not discharged: SAS-010 requires, for every admitted rule, evidence that `StepInv` is initialized at canonical genesis, preserved by the gated fragment under `StepInv(pre)`, and that every fragment premise refers to the shared carriers (no disconnected copies). Task 4.6 covers initialization as well as hostile transitions (astra-2).

### 2.3 S-step leaf

A leaf proves `S >= 1` consecutive owner steps on one key. Inside the leaf: `pre_0` is the digested `pre_d`; for `i < S-1`, `post_i == pre_{i+1}` lane by lane; `post_{S-1}` is digested as `post_d`. `steps` is the number of *enabled* owner-step events in the leaf; a short final chunk pads with disabled steps (`enable == 0`, identity on lanes, zero contribution to `steps`). Terminal: a step that publishes (`Halted`, `Rejected`, `Exhausted`) is the last enabled step; later slots are disabled. Charged work and fuel stay inside the state digest; `steps` and `h` are physical counts (ISSUE 5).

S=1 is the baseline that qualifies P1-P3. S=8 is a measured candidate: with one step per leaf fact(3) is about 293 node proofs at about 95 s each (`stagenet-gp-recursion.md:199-201`); S=8 gives about 37. The leaf's private witness holds S copies of the carriers; prover memory at the qualification config is measured before S>1 is adopted (open question Q5).

### 2.4 Identity and configuration

`prog_d = Poseidon(dom, programImageDigest, layoutSchemaDigest, boundsDigest, profileId)` where the layout schema digest covers `sherlock-c6-phys/1`, `LayoutSchema.recordWidths`, the qualification host config (frames, data, env, value slots) and `Bounds`. A program image under a different layout or caps therefore has a different `prog_d`, and `SherlockRun` seals `program_d` (`sherlock-run.compact:26-47`). This replaces a separate `cfg_d` lane (fable-2 I4; fable-2 critique concession).

Leaf key identity (manifest): relation id `sherlock-leaf/1`, `sherlock-leaf-stmt/1`, layout schema digest, `Bounds`, ZKIR version (minor read from the compiled `.zkir`), `midnight-zk-stdlib`, `midnight-circuits`, `midnight-proofs` versions, SRS identity, k. The node and decider manifests add the leaf VK digest and the node VK digest respectively (SRC-008).

### 2.5 Host interface

`mori simulate` runs the owner engine; `capture(&admitted, &image)` yields a `HostSnapshot` per step; `witness_from_host_snapshot` builds the C6 witness; the witness stream `moriarty-sherlock-c6-witness-stream/1` carries witness, `RuleIdC`, `h_pre`, and digests `prog, init, pre, post` with their `StateOutcome`s (`sherlock_c6_witness_stream.rs`). The stream is the only *application* producer for the leaf prover and the digest-parity test (`sherlock_digest_parity`, commit `3c2657acb`). It is not the oracle: the holdout keeps its independent expected traces (`models/sherlock/oracles/`, `qa/holdout/`) and the hostile-witness suites keep raw witness entry points that bypass the stream (astra-3 early correction). `HostSnapshot` never appears in a public statement; the host supplies private witness only. The public digests `prog_d`, `pre_d`, `post_d` are permitted statement components because the circuit constrains them to the actual witness. Host checks (`checkStepInv`, `ProofPreimage::check`) are diagnostics with no acceptance authority (UISEC-012).

### 2.6 Fragment coverage

Gaps at `8fd34ed79`: `KAdminPublishValue`, `BHelperValidate` x4, `BHelperEqual` x4, `BHelperMatch` x10, `BHelperScalarConstruct` (`stagenet-gp-recursion.md:224-243`). WIP source exists (`36456d510`, `5d7663275`) with `native_inventory.rs` rows still `gap(...)`.

Order (astra-1 early correction, avoids the P3 circularity): first freeze the *admitted rule inventory* `R` for profile `/1` (every supported pure rule, not only the rules P1 and P2 fire), the layout, the caps and the manifests; every rule in `R` has a fragment with a compiled-and-run receipt before the universal leaf is keyed (D9); then P3 is authored from the admitted domain and must need no new key (E2). P1/P2/P3 evidence is finite cross-program evidence, not a proof that every admitted program is sound; the per-rule obligations in `assurance.md` stand beside it.

## 3. Recursion design

### 3.1 Chain

```
owner run -> witness stream -> LEAF (sherlock-leaf/1, S steps, stmt/1 seven lanes)
          -> NODE N (ivc.rs, self-recursive; [vk_N_repr, prog_d, init_d, cur_d, h, acc])
          -> RUN DECIDER D (run_decider.rs; values [prog_d, init_d, fin_d, n_steps, result_be]; Blake2b)
          -> SherlockRun.acceptRun(stmt, p, i, n, fin, r)  (Compact 0.36.0; decider VK installed)
```

Measured at `8fd34ed79` pins: one C6-shaped digest leaf with 96 private lanes (M2, not `standin.rs`) k=13, 2.55 s, 3,824 B; node k=20, 531,989 rows, ~50 s keygen, ~95 s/step, 26.2 GB; decider k=20, 833,235 rows, ~100 s, 70 lanes mutation-refused (`stagenet-gp-recursion.md:184-201`). These numbers are re-measured at the new pins (M2).

### 3.2 Node fixes before new keygen (SRC-003, SRC-004, SRC-005)

1. Range checks: `prev_h`, `steps`, `h < 2^64` with `assert_lower_than_fixed` (pattern `parent.rs:665-670`); today only `steps != 0` and field addition (`ivc.rs:308-310`).
2. Genesis: `is_genesis = is_zero(prev_h)`; require `prev_d == init_d` (exists, `ivc.rs:312-313`). Today `proof_acc` is scaled by `not_genesis` (`ivc.rs:343`) while the witness-supplied `prev_acc` is folded unscaled (`:318-327,364`). The normative requirement is UISEC-010: no unauthenticated deferred obligation at genesis. The implementation must either (a) constrain the supplied carry to the designated neutral encoding at genesis, or (b) constrain a zero/ignore construction (scale `prev_acc` by `not_genesis`) and justify that it cannot authorize an invalid leaf or predecessor. Tests match the chosen semantics: non-neutral refusal for (a); semantic irrelevance of a well-formed ignored carry for (b), without requiring identical folded bytes (ignored inputs may still alter transcript encodings); and invalid-leaf refusal through the final pairing for both. Retaining an unscaled supplied carry requires a separately recorded security argument. The target implementation hashes accumulator encodings before deriving folding powers (`circuits/src/verifier/accumulator.rs:177-192,315-343` at `24a1eb5a`, astra-2); this is a transcript observation, not a proof that cancellation is impossible. Malformed point-encoding refusal is classified at its actual boundary. No practical attack was demonstrated; this is hardening (astra-2, fable-2).
3. Leaf lanes: `rules_d` and `steps` are constrained inside the leaf (equated to the dispatch selector digest and the enabled-step count), not merely published (`sherlock_c6_child.rs:598-608`); `h_pre` is bound by the node to its bounded previous height.
4. `h_post = h_pre + steps` as bounded integer addition; the decider's `n_steps` equals the final `h`.

### 3.3 Decider and contract

The decider verifies the last node proof, pins `vk_N_repr`, folds and pairs the accumulator, opens `cur_d = Poseidon(SHK_SD, HALTED, result, lanes_d)` and binds `values`. The outcome tag and result are derived from the proved state, not supplied as constants (astra-2 SND-004; `sherlock_c6_child.rs:560`). `acceptRun` reads sealed `program_d` and `init_d`, refuses replay of `stmt`, writes `result` as big-endian `Bytes<16>` (`sherlock-run.compact:26-47`). The acceptRun transcript template (70 slots, `tests/fixtures/run-template.json`) is recaptured with Compact 0.36.0 / runtime 0.21.0 and diffed against `template.rs` (SMN-003).

### 3.4 Leaf shape decision (SRC-013)

Milestone M4 (task 4.4; the historical measurement label "M2" in `docs/sherlock/stagenet-gp-recursion.md` is a different numbering) measures the universal gated leaf (dispatch + every fragment of the frozen inventory `R`) at S=1: k, rows, RSS, keygen and prove time. If k <= 17: option U. Otherwise option F: each rule its own `IrSource` under a forced uniform `ZkStdLibArch` and k, the node checks the child's `transcript_repr` against a committed family table in-circuit (authenticated membership), and E2 holds because the table is fixed before P3 is written. Option F is defined for S=1; a mixed-rule chunk under F needs a separate constrained construction before S>1 is adopted there (astra-3). Both branches carry identical semantic obligations and the same seven-lane statement.

### 3.5 Throughput (SRC-011, optional)

Target: a committed program of at most 320 owner steps proves, folds, decides and verifies in under 2 h and under 48 GB peak on the 20-core host. Levers in order, each adopted only after the C0 phase profile (`openspec/changes/recursion-phase-profile/`) shows its share: S>1, node at k=19 (1.5% over 2^19 today), heavy concurrency (needs a second lock class with aggregate resource enforcement; optional separate work named in `workflow.md` E11), GPU MSM. A recorded shortfall closes the measurement task only.

## 4. Assurance summary

See `assurance.md`. The chain source -> image -> reference steps -> witness -> leaf -> node/decider -> ledger has one evidence class per link; no class substitutes for another (UISEC-012). Completeness and soundness per rule are stated obligations. Dispatch coverage is by complete dispatch-predicate partition plus directed cases, not by sample (ISSUE 4 as revised).

## 5. Midnight ledger 9

### 5.1 Facts

- Mainnet on ledger 9.1.0.1 since block 3,346,139 (2026-11-06T20:51Z, spec 1000300 -> 2001100); Preview reset as a new chain (ledger 9 from block 1572, 2026-11-07T03:27Z); Preprod on 8.1.3 (`wiki/midnight-ledger-9-hardfork.md`; https://docs.midnight.network/relnotes/ledger-9; `.../support-matrix`).
- Stack: Compact 0.36.0 (language 0.28.0, runtime 0.21.0), Midnight.js 5.0.0 (ESM, `providers.config{networkId,ttlSeconds}`, `setNetworkId` removed), Wallet SDK 2.0.0, onchain-runtime-v4 4.0.0, proof server 9.0.0/9.0.1, indexer 4.4.101. Pins: `ledger-9.1.0.1` `a2b30128`, `zkir-3.2.0` `446c7fec`, `compactc-v0.36.0` `b5d9e4df`.
- Proof system: zk-stdlib v2 2.4.0, circuits 7.3.0, `ProofVersioned::V3`, `ContractOperation` v3 VK field; ZKIR stays v3 (3.2.0). VKs change for zkir-v3 circuits that encode a Jubjub scalar (release 9.1.0.0). `ContractCall::public_inputs -> Result` (`TooManyPublicInputs`), 256 ops per contract, canonical field atoms, `noop 0` rejected, segment writes priced separately (9.1.0.1). Fees: `min_block_price` 10; `validation_factor`/`guaranteed_factor`/`fallible_factor`; DUST reset. Known issues #838, #839, node #2375.
- Workspace: rev `04c9c5d9` (2026-07-31), proofs 0.8.3, circuits 7.2.4, zk-stdlib 2.3.5, zkir-v3 3.0.0-rc.2; SDK ledger-v9 1.0.0-rc.3, proof server 9.0.0-rc.5, Compact 0.34.0 (`Cargo.toml:28-42`; `profile.rs:35`; `midnight-sdk-seam.md:4`; `stagenet-deployment.md:112-114`).
- Stagenet (`rpc.stagenet.shielded.tools`): ledger `=1.0.0`, specVersion 2000000, distinct genesis; Preview: `crate-ledger-9.1.0.1`, ledger rev `3e69c5e3`, specVersion 2001100 (`astra-1/target-observations.md`, `fable-3/position.md` §6).

### 5.2 Consequences (SMN-*)

1. Re-pin and build; `storage 3.0.0` (`Map` keys need `EncodingShape`), `AlignedValueChecked`, fallible cost API, unified signing envelope need source changes in `mori-ledger` and `mori-recursion`.
2. Keygen regression: compute relation and VK hashes for leaf, node, decider, parent and the contract at the new pins; compare with the manifests; regenerate the affected closure in the order leaf -> node -> decider -> acceptRun template -> deploy, because the leaf VK is a node circuit constant (`ivc.rs:10-15`), the node VK a decider constant (`run_decider.rs:5-6`) and the decider VK is installed in the contract; a changed leaf VK changes every key above it. The release note's VK-change classes (zkir V2 `hash_to_curve`; zkir v3 Jubjub-scalar `encode`) are informative only: the node and decider are hand-written `midnight_zk_stdlib::Relation`s on `midnight_circuits` (`ivc.rs:27-45`), not ZKIR, and the leaf is ZKIR v3; the hash regression is the sole discriminator for all four relations (fable-2). Historical keys and receipts stay as historical evidence; they are not declared false (ISSUE 2).
3. Recapture the acceptRun template with 0.36.0; check the 70-lane statement against the public-input limit read from `LedgerParameters`; check the 256-op cap; pay with at least two DUST spends until #838 is fixed.
4. Chain identity = network name + genesis hash + specVersion + ledger rev (SMN-004). Local source/binary provenance and live runtime identity are recorded separately; the deployed node source for spec 2001100 is not public (scout), which is an explicit limit on "exact production reproduction".
5. Deploy `SherlockRun` with a maintenance authority or declare it disposable. Inserting a key does not revoke an old one, and redeploying or re-pinning a client does not revoke anything on chain; "retired" is a client-side label until the on-chain key state is observed. A contract holding active state (funds, duties, consumed statements) is never discarded: recovery must migrate or explicitly record the residual exposure; only empty instances may be abandoned (astra-2). Immutable contracts remain permitted (SMN-007; `workflow.md` §7).

## 6. Open questions carried into tasks

Q1 universal leaf k (M2). Q2 Jubjub-scalar VK impact per circuit (M1 regression). Q3 why one `ProofPreimage::check` of a dispatch circuit costs minutes (M3 instrumentation). Q4 prover memory for S>1 (M4). Q5 whether shielded.tools upgrades stagenet (owner; no dependency). Q6 the public-input limit value in 9.1.0.1 `LedgerParameters` (M1). Q7 whether `verifier.accumulate` binds its inputs with a challenge or sums them; partial answer from astra-2: at `24a1eb5a` the accumulator hashes all public encodings before deriving folding powers (`accumulator.rs:177-192,315-343`); M2 confirms at the re-pinned version and selects realization (a) or (b) of §3.2(2).

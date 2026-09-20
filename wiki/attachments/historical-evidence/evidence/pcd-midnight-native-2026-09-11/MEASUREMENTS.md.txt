# Reproduced measurements: Midnight-native proving components

Every number in the "reproduced locally" tables was measured on the machine
below on 2026-09-11. Numbers from other hardware appear only
in the separated "documented elsewhere" section.

## 1. Hardware and software

```
Architecture:        x86_64
CPU(s):              6 (1 thread per core, 6 cores, 1 socket)
Vendor ID:           GenuineIntel
Model name:          Intel(R) Core(TM) Ultra 7 365 (family 6, model 204, stepping 3)
Hypervisor vendor:   Microsoft (WSL2, full virtualization)
L1d cache:           288 KiB (6 instances)
Flags include:       avx2 adx bmi2 sha_ni avx_vnni vaes vpclmulqdq (no avx512)
RAM:                 31 GiB, swap 8 GiB (4 GiB in use before the runs)
Kernel:              6.18.33.2-microsoft-standard-WSL2
/tmp:                tmpfs 16 GiB (bench tree lives there)
Docker 29.7.2, Node v24.18.1
```

Toolchains actually used:

| Build | rustc | Why |
|---|---|---|
| midnight-zk (IVC) | 1.90.0 (1159e78c4 2025-09-14) | repo `rust-toolchain.toml` pins 1.90.0 |
| midnight-ledger PR #738 | 1.96.0 (ac68faa20 2026-05-25) | no toolchain file; default stable |

## 2. Sources and commits

| Component | Commit / version | Location |
|---|---|---|
| midnight-zk main | 695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7 (2026-08-21) | copy at `bench/zk` (clone at `src/zk` untouched) |
| midnight-ledger PR #738 head | 416da99309cff5f953f029ff6c89211c00cf423c ("fix(zkir-v3): fix CI?") | copy at `bench/ledger` (clone at `src/ledger-pr738` untouched) |
| ledger PR #738 resolved zk crates | midnight-proofs 0.8.2, midnight-circuits 7.2.4, midnight-zk-stdlib 2.3.5, midnight-curves 0.3.1 (crates.io, from Cargo.lock) | |
| proof server image | midnightntwrk/proof-server:8.1.0, image id sha256:801bbc0340e9e96f16735f77b523f23c7459e3359842f7c79c2c53f4e994d531 | local image |

Retained logs, patches and proof-server harness files are under `bench/` in this evidence directory. Build trees, proving keys and SRS files were not retained; the SRS hashes below identify the downloaded parameter files.

## 3. SRS / parameter provenance

midnight-zk IVC uses `load_srs(SrsSource::Midnight, K, cs_degree)`. It does not
download anything itself: it reads `$SRS_DIR/midnight-srs-2p<K>` and panics with a
curl hint if the file is missing. Without the `single-h-commitment` feature the
monomial blow-up is 1, so only the 2^K file is read. Files were downloaded
(HTTP GET only) from `https://srs.midnight.network/midnight-srs-2p<K>` and checked
against `MIDNIGHT_SRS_CATALOG.md` in github.com/midnightntwrk/midnight-trusted-setup
(main):

| K | Bytes | SHA-256 (local = catalog) |
|---|---|---|
| 17 | 25,166,212 | 4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74 |
| 18 | 50,332,036 | e8436dc5d8b598f169c127c745135d889744007e6d384ff126df8d1332522f86 |
| 19 | 100,663,684 | 8e8dc15c4362f05c912f1e770559a3945db3e58a374def416ed5d3e65ad5b10e |

The 2^17 file also existed at
`/home/charl/Moriarty/.worktrees/r3-native/.native/srs/midnight-srs-2p17` with the
same hash (read only, not used).

ledger PR #738 `verify_proof_e2e` does not use the ceremony SRS. It calls
`ParamsKZG::unsafe_setup(19, rng)` in-process with a seeded ChaCha20 RNG (known
toxic waste; the test comment says the published params "stop at degree 17" for
that path). Its timings therefore include SRS generation that production would
replace with a file read.

## 4. Build commands and build cost (reproduced locally)

```
cd bench/zk
/usr/bin/time -v cargo build --release -p midnight-aggregation --example ivc --features truncated-challenges
# wall 1:01.25, max RSS 705,844 kB (registry already cached)

cd bench/ledger
/usr/bin/time -v cargo test --release -p midnight-zkir-v3 --test verify_proof_e2e --test verify_proof --test inner_witnesses --no-run
# wall 2:33.74, max RSS 1,273,504 kB, exit 0, no system-dependency blocker (plain cargo, no nix;
# crates fetched from crates.io under the committed Cargo.lock)
# after patching: --lib added, rebuild of the test crates 30.3 s wall
```

## 5. Patches applied (benchmark copies only)

### 5.1 midnight-zk (`bench/logs/midnight-zk-ivc-bench.patch`, 5 files, +270/-34)

- `aggregation/src/ivc/setup.rs`: print keygen_vk and keygen_pk time; print
  `cost_model` when `IVC_COST` is set.
- `aggregation/src/ivc/prover.rs`: print per-step sub-phase times (off-circuit
  verification of the previous proof, native accumulation, PLONK prove); a
  `BENCH_SKIP_PREV_CHECK` switch that lets `prove_step` continue past an invalid
  predecessor (negative control only); `bench_pk_bytes()`.
- `aggregation/src/ivc/verifier.rs`: `bench_vk_repr()`, `bench_vk_bytes()`.
- `aggregation/src/ivc/circuit.rs`: `IvcInstance::bench_parts()` /
  `bench_from_parts()` for building tampered instances.
- `aggregation/examples/ivc.rs`: transition relation unchanged; `main` replaced by
  a driver parameterised by `IVC_K`, `IVC_N` (Poseidon rounds per step: 1 or 1000),
  `IVC_STEPS`, `IVC_NEG`, `IVC_WRONGVK`; prints per-step prove/verify time, proof
  bytes, instance size, VmHWM; 20 repeated verifications; negative controls.

### 5.2 midnight-ledger PR #738 copy

`zkir-v3/tests/verify_proof_e2e.rs` only (instrumentation and negative controls);
described in 7.2. Original at `bench/logs/verify_proof_e2e.orig.rs`.

### 5.3 Proof-server harness

No image or source changes. New files only under `bench/ps` (staged copy of the
SP05 loan build, `package.json`, `make-payload.mjs`); described in 8.5.

## 6. midnight-zk IVC example (reproduced locally)

### 6.1 Relation and parameters

- Step relation (`IvcCircuit<T>`, aggregation/src/ivc/circuit.rs): public instance
  `(vk_repr, state, acc)`; witness `(prev_state, prev_acc, prev_proof,
  transition_witness)`. In one circuit it (1) assigns its own VK as a public
  input, (2) applies the transition, (3) runs the in-circuit PLONK verifier
  `prepare` on `prev_proof` against `(vk_repr, prev_state, prev_acc)`, (4) scales
  that accumulator by `is_not_genesis`, (5) folds it with `prev_acc` and
  collapses, (6) exposes the folded accumulator as public input. The pairing is
  never computed in-circuit; it is deferred to the native verifier.
- Example transition (`PoseidonChain<N>`): state `(cnt, val)`; each step applies
  N = 1000 Poseidon rounds to `val` and adds N to `cnt`. No transition witness;
  the decider always returns true.
- Native verifier (`IvcVerifier::verify`): VK-repr equality, decider, PLONK
  `prepare` of the proof, transcript-empty check, fold with `instance.acc`, one
  pairing check on the result.
- Prover (`prove_step`): off-circuit `prepare` + pairing check of the previous
  proof (a guard: it refuses an invalid predecessor), native accumulation, then a
  full PLONK prove of the step circuit.

### 6.2 Upstream example as shipped does not run at this commit

`aggregation/examples/ivc.rs` sets `K = 17`. Unmodified parameters (K=17, N=1000)
panic in key generation after 1.41 s:

```
keygen_vk should not fail: Synthesis("Relation::circuit error")
```

(`bench/logs/ivc_n1000_s1.log`). A diagnostic run at K=18 and K=19 showed both
pass key generation. That run's output was not saved to a file, and its timings
were discarded because it overlapped the ledger build. All figures below use
K=18, the smallest K that worked. The cost model at K=18:

```
CircuitModel { k: 18, rows: 163172, table_rows: 245779, nb_unusable_rows: 11, max_deg: 5,
  advice_columns: 15, fixed_columns: 49, lookups: 2, trashcans: 1, permutations: 18,
  column_queries: 52, point_sets: 5, size: 5264 }
```

### 6.3 Commands

```
cd bench/zk/aggregation
SRS_DIR=bench/srs IVC_K=18 IVC_N=1000 IVC_STEPS=1 IVC_COST=1 \
  /usr/bin/time -v ../target/release/examples/ivc          # logs/ivc_k18_n1000_s1.log
SRS_DIR=bench/srs IVC_K=18 IVC_N=1000 IVC_STEPS=10 IVC_NEG=1 IVC_WRONGVK=1 \
  /usr/bin/time -v ../target/release/examples/ivc          # logs/ivc_k18_n1000_s10_neg.log
```

Both runs had the machine to themselves, apart from light file reads. During
steps 1-3 of the 10-step run a Python script wrote a source file, which may
explain their slightly higher times (19.9-23.3 s against 18.6-19.2 s later).

### 6.4 Phase timings, K=18, N=1000 (reproduced locally)

| Phase | 1-step run | 10-step run |
|---|---|---|
| SRS load (2^18 file read) | 465 ms | 468 ms |
| keygen_vk | 9,487 ms | 9,552 ms |
| keygen_pk | 5,393 ms | 5,624 ms |
| setup total (configure + vk + pk) | 16,514 ms | 15,259 ms |
| VK size (RawBytes) | 4,734 B | 4,734 B |
| PK size (RawBytes) | 411,046,711 B | 411,046,711 B |

Per step (10-step run; sub-phases from the patched prover):

| Step | prove_step total ms | off-circuit prev verify ms | native accumulate ms | PLONK prove ms | native verify ms | proof B |
|---|---|---|---|---|---|---|
| 1 (genesis) | 19,982 | 0.0 | 1.4 | 19,930 | 12.2 | 5,264 |
| 2 | 20,568 | 9.0 | 3.6 | 20,505 | 13.3 | 5,264 |
| 3 | 23,300 | 8.7 | 3.3 | 23,236 | 13.7 | 5,264 |
| 4 | 19,946 | 9.4 | 3.4 | 19,880 | 13.4 | 5,264 |
| 5 | 19,227 | 9.1 | 3.1 | 19,166 | 12.9 | 5,264 |
| 6 | 18,715 | 8.6 | 3.2 | 18,654 | 12.4 | 5,264 |
| 7 | 18,649 | 8.6 | 3.4 | 18,588 | 11.9 | 5,264 |
| 8 | 18,937 | 8.5 | 3.1 | 18,877 | 12.9 | 5,264 |
| 9 | 18,786 | 9.1 | 3.2 | 18,723 | 12.5 | 5,264 |
| 10 | 18,767 | 8.8 | 3.5 | 18,706 | 13.7 | 5,264 |

1-step run, step 1: prove 17,512 ms (PLONK 17,464 ms), verify 12.1 ms.

Summary (10-step run):
- steady-state steps 4-10: mean 19,004 ms, min 18,649, max 19,946.
- all 10 steps: 196,877 ms total proving, mean 19,688 ms (recomputed from the log by script).
- native verification of the final proof, 20 repeats: min 11.55, median 12.31,
  max 12.88 ms (1-step run: 10.64 / 11.52 / 12.25 ms).
- proof size is constant at 5,264 B for every step.
- instance: 65 field elements = 1 vk_repr + 2 state + 62 accumulator elements.
  At 32 B per BLS12-381 scalar this is 2,080 B (1,984 B for the accumulator).
  This is the in-circuit public-input encoding. The crate has no separate
  compressed wire format for `Accumulator`.
- whole 10-step process: wall 4:34.41, user 1,289 s, sys 29 s, 480% CPU.

Peak RSS (VmHWM sampled in process, and `/usr/bin/time` max RSS):

| Point | MiB |
|---|---|
| after SRS load | 162 |
| after setup (keys in memory) | 3,189 |
| during steps 1-10 | 4,375-4,418 |
| end of 10-step run (after negative controls, which clone the prover twice, and a second full setup for the wrong-VK control) | 9,737 |
| `/usr/bin/time` max RSS, 1-step run | 4,273 (4,375,560 kB) |
| `/usr/bin/time` max RSS, 10-step run incl. controls | 9,737 (9,970,952 kB) |

The 4.4 GiB figure is the one for a single prover. The 9.7 GiB figure is an
artefact of the control harness holding three provers and two key sets.

100 steps (reproduced locally; `SRS_DIR=bench/srs IVC_K=18 IVC_N=1000 IVC_STEPS=100 timeout 2700 /usr/bin/time -v ../target/release/examples/ivc`, log `bench/logs/ivc_k18_n1000_s100.log`, no negative controls):

| Quantity | Value |
|---|---|
| keygen_vk / keygen_pk / setup total | 9,722 / 5,435 / 15,241 ms |
| steps completed | 100 (exit 0) |
| total proving time, 100 steps | 1,950,324 ms |
| per-step prove: mean / median / min / max | 19,503 / 19,452 / 18,696 / 21,214 ms |
| per-step prove, steps 4-100: mean / stdev | 19,513 / 276 ms |
| per-step native verify range | 12.4-21.8 ms |
| final proof native verify, 20 repeats: min / median / max | 12.37 / 12.63 / 13.62 ms |
| proof size | 5264 B at every step |
| max VmHWM sampled over the steps | 4,405 MiB |
| process wall / max RSS | 32:48.00 / 4,511,068 kB |

Mean prove time per block of 10 steps (ms):

| Steps | 1-10 | 11-20 | 21-30 | 31-40 | 41-50 | 51-60 | 61-70 | 71-80 | 81-90 | 91-100 |
|---|---|---|---|---|---|---|---|---|---|---|
| mean | 19,536 | 19,553 | 19,474 | 19,381 | 19,591 | 19,493 | 19,689 | 19,386 | 19,549 | 19,382 |

### 6.5 Negative controls (K=18, after step 10)

| # | Tamper | Outcome |
|---|---|---|
| N0 | none (positive control) | ACCEPTED |
| N1 | instance state `cnt + 1` | REJECTED(InvalidProof), 11.9 ms |
| N2 | instance state `val + 1` | REJECTED(InvalidProof), 12.8 ms |
| N3 | current state and proof, accumulator from step 9 (stale accumulator) | REJECTED(InvalidProof), 12.0 ms |
| N4 | step-9 instance with step-10 proof | REJECTED(InvalidProof), 12.6 ms |
| N5 | step-10 instance with step-9 proof | REJECTED(InvalidProof), 12.5 ms |
| N6 | one proof byte flipped (middle) | REJECTED(InvalidProof), 11.9 ms |
| N7 | one byte appended to proof | REJECTED(TranscriptNotEmpty), 7.6 ms |
| N8 | last proof byte removed | REJECTED(InvalidProof), 7.0 ms |
| N9 | instance `vk_repr + 1` | REJECTED(VkMismatch), <0.1 ms |
| N10 | prover resumed from invalid predecessor (state `cnt+1` with real proof), guard on | prove_step returns Err(InvalidProof) after 55.7 ms; no proof produced |
| N11a | same, prover guard disabled by patch | successor proof PRODUCED (20,143 ms, 5,264 B) |
| N11b | verify that successor | REJECTED(InvalidProof), 12.3 ms |
| N11c | one more step on the poisoned chain (guard still off) | proof PRODUCED (18,385 ms) |
| N11d | verify it | REJECTED(InvalidProof), 12.4 ms |
| W1 | verifier of a different IVC circuit (N=7, own keys), instance carrying original vk_repr | REJECTED(VkMismatch), <0.1 ms |
| W2 | same verifier, instance claiming that verifier's vk_repr | REJECTED(InvalidProof), 12.7 ms |
| W3 | original verifier, instance claiming the other circuit's vk_repr | REJECTED(VkMismatch), <0.1 ms |

Every rejection is an `Err(IvcError::…)`, never a silent `false` or a panic.
N11 shows that the circuit accepts witnesses for an invalid predecessor, and a
proof is produced for them. Unsoundness is not hidden, though: the bad
accumulator propagates, and the next native verification rejects the chain,
even one step later. Detection is deferred to the final pairing check, not
enforced at proving time.

## 7. Ledger PR #738, ZKIR-v3 `verify_proof` (reproduced locally)

### 7.1 What the PR's tests do

- `zkir-v3/tests/verify_proof_e2e.rs`: two tests, both `#[ignore]`d ("proves a
  k=18 / k=19 circuit in-circuit; too slow for CI, run with --ignored").
  - `verify_proof_without_a_decider`: an inner `Echo` circuit exposes one field
    element. Its proof (Poseidon transcript) is verified inside an outer ZKIR-v3
    circuit: guarded `private_input` for the inner instance, `inner_proof`,
    `verify_proof` (vk resolved by SHA-256 hash from the IR side-table,
    `DeciderKind::None`). The outer proof exposes one deferred accumulator.
    `VerifierKey::verify` prepends the accumulator PIs to the statement, runs
    PLONK verification, then runs the pairing check on each exposed accumulator.
    The same proving key is then proven with the guard off, which exposes the
    trivial accumulator.
  - `verify_proof_with_a_collapsed_decider`: inner `Echo` proof, then a
    `Recursive` midnight-zk-stdlib circuit that verifies it in-circuit and
    carries its accumulator in its instance tail, then an outer ZKIR circuit
    verifying the `Recursive` proof with `DeciderKind::Collapsed`, so the
    instruction folds the carried accumulator into the exposed one. This is a
    two-level recursion.
  - SRS: `ParamsKZG::unsafe_setup(19)` in process (see section 3).
- `zkir-v3/tests/inner_witnesses.rs` (4 tests, not ignored): the
  `inner_proof`/`verify_proof` pairing rules, checked by `IrSource::check`.
- `zkir-v3/tests/verify_proof.rs` (2 tests, not ignored): text-format round trip,
  and that a side-table requires IR minor version 1.
- `zkir-v3/src/decider.rs` unit tests: stable tag bytes, unknown tag rejected,
  padded VK blob rejected, non-collapsed accumulator tail rejected.

### 7.2 Commands

```
cd bench/ledger
cargo test --release -p midnight-zkir-v3 --test verify_proof --test inner_witnesses
cargo test --release -p midnight-zkir-v3 --lib decider
/usr/bin/time -v cargo test --release -p midnight-zkir-v3 --test verify_proof_e2e -- \
  --ignored --nocapture --exact <test_name> --test-threads=1
```

Instrumentation patch (copy only): `bench/ledger/zkir-v3/tests/verify_proof_e2e.rs`,
original saved as `bench/logs/verify_proof_e2e.orig.rs`. It adds timing, size and
VmHWM prints around test-SRS generation, inner keygen/prove, the inner off-circuit
`verify_proof` prepare, outer cost model, outer keygen, outer prove (guard on and
off), and outer verify (10 repeats). It also adds negative controls: V1-V3 and V5
on verification, P1-P6 on proving/checking. Upstream assertions are unchanged.
First compile attempt failed on my patch (E0509: `ProofPreimage` implements
`Drop`, so struct-update syntax is invalid); fixed by mutating a local. Rebuild
of the patched test crate: 30.3 s wall.

### 7.3 Suite negative tests: observed outcomes

| Suite test | Checks | Observed |
|---|---|---|
| inner_witnesses::one_proof_witness_per_instruction_whatever_the_guard | too few / too many `inner_proofs` witnesses rejected | ok (pass) |
| inner_witnesses::a_verify_proof_must_name_a_proof_bound_before_it | unpaired `verify_proof`, and binding after use, rejected | ok (pass) |
| inner_witnesses::the_two_guards_must_agree | `inner_proof`/`verify_proof` guard mismatch rejected | ok (pass) |
| inner_witnesses::a_bound_proof_is_verified_exactly_once | double verify, and unused `inner_proof`, rejected | ok (pass) |
| verify_proof::verify_proof_text_format_roundtrips | format round trip | ok (pass) |
| verify_proof::side_table_requires_minor_1 | side-table in a minor-0 IR rejected | ok (pass) |
| decider::tests::decider_tags_are_stable (lib) | tag bytes None=0, Collapsed=1 | ok (pass) |
| decider::tests::an_unknown_tag_is_rejected (lib) | tags 2/3/255 and empty blob rejected | ok (pass) |
| decider::tests::a_padded_blob_is_rejected (lib) | VK blob with trailing bytes rejected | ok (pass) |
| decider::tests::a_non_collapsed_tail_is_rejected (lib) | carried accumulator whose RHS scalar is not 1, or missing tail, rejected | ok (pass) |

Command for the lib tests: `cargo test --release -p midnight-zkir-v3 --lib decider`
(4 passed, 24 filtered out).

### 7.4 e2e `verify_proof_without_a_decider` (inner Echo, DeciderKind::None), reproduced locally

Log: `bench/logs/e2e_verify_proof_without_a_decider.log`.

| Quantity | Value |
|---|---|
| test SRS generation `unsafe_setup(19)` (not part of production) | 15,330 ms |
| inner circuit (Echo) k / rows | k=4, rows 1 (table rows 1) |
| inner keygen vk / pk | 7.8 ms / 5.0 ms |
| inner prove (Poseidon transcript) | 33.4 ms |
| inner proof bytes | 2,528 B |
| inner decider-tagged VK blob (`serialize_vk`, Processed format) | 1,230 B |
| inner off-circuit `verify_proof_offcircuit` (prepare, no pairing) | 5.28 ms |
| outer ZKIR-v3 circuit k / rows | k=18, rows 150,966, table rows 253,969, 15 advice, 45 fixed columns, 5 lookups, max degree 5 |
| outer keygen (`IrSource::keygen`, vk + pk) | 79,918 ms |
| outer prove, guard on (one inner verify_proof) | 34,773 ms |
| outer prove, guard off (same pk, trivial accumulator) | 34,668 ms |
| outer PLONK proof bytes | 6,432 B |
| outer tagged `Proof` bytes (proof + 1 deferred accumulator, `tagged_serialize`) | 6,550 B |
| deferred accumulators exposed / PI elements each | 1 / 12 (= 384 B as 32-byte scalars) |
| outer statement length (caller-facing) | 1 element |
| outer native verify incl. deferred pairing check, 10 repeats | min 4.99, median 5.88, max 6.07 ms |
| outer verify, guard-off proof | 6.11 ms |
| VmHWM after outer keygen / after outer prove | 3,267 MiB / 4,168 MiB |

Guard on and guard off cost the same to prove (34.8 s vs 34.7 s). The circuit
shape is fixed at keygen, so an unused `verify_proof` slot costs a full
in-circuit verifier's worth of rows.

Verification-side negative controls (patched in):

| # | Tamper | Outcome |
|---|---|---|
| V1 | exposed accumulator replaced by the trivial accumulator | REJECTED("Invalid outer proof"), 4.93 ms |
| V2 | one outer proof byte flipped | REJECTED("Invalid outer proof"), 5.26 ms |
| V3 | statement with one extra element | REJECTED("Invalid outer proof"), 0.41 ms |

Proving-side negative controls, first run (patched in; same proving key):

| # | Tamper | Outcome |
|---|---|---|
| P1 | inner proof, one byte flipped at the midpoint; outer `prove` | ACCEPTED: an outer proof was produced (33,538 ms) |
| P1b | same tampered inner proof through `IrSource::check` | ACCEPTED (6.3 ms) |
| P2 | valid inner proof, inner instance claimed as ECHO+1; outer `prove` | ACCEPTED: an outer proof was produced (33,931 ms) |
| P3 | side-table holds a different VK blob than the instruction's `vk_hash`; `check` | REJECTED("no verifying key in `verify_proof_vks` for vk_hash 0x73d6…75c0"), <0.1 ms |
| P3b | same; `prove` | REJECTED (same message), <0.1 ms |
| P4 | VK blob padded with one trailing byte (hash recomputed over the padded blob); `check` | REJECTED("`verify_proof_vks` entry has 1 trailing bytes after its verifying key"), 1.3 ms |
| P5 | Echo VK registered as `DeciderKind::Collapsed`; `check` | REJECTED("a `Collapsed` inner proof's instance must end with the 12 fields of a collapsed accumulator, but it has 1"), 6.0 ms |
| P6 | `inner_proofs` witness vector empty; `check` | REJECTED("Not enough proof witnesses: ran out at index 0"), <0.1 ms |

P1 and P2 contradict the test file's comment ("Proving already runs both
`verify_proof` passes, so it fails if the inner proof does not check out"). On
this build, an inner proof that decodes but is invalid, or a valid proof bound to
the wrong instance, does not stop the prover. The first run did not verify the
resulting outer proofs. A rerun that does is recorded below (P1c, P1e, P2c, P3c).

Whole process for this test (`/usr/bin/time -v` around cargo test): wall 4:10.66,
max RSS 4,268,316 kB. The test harness reports 250.43 s, which includes SRS
generation, both outer proves and the negative controls.

### 7.5 e2e `verify_proof_with_a_collapsed_decider` (two-level), reproduced locally

Log: `bench/logs/e2e_verify_proof_with_a_collapsed_decider.log`.

| Quantity | Value |
|---|---|
| test SRS generation `unsafe_setup(19)` | 17,897 ms |
| level 0: Echo k / keygen vk / pk / prove / proof | k=4 / 9.0 ms / 4.6 ms / 68.3 ms / 2,528 B |
| level 1: `Recursive` (midnight-zk-stdlib `Relation` calling `verify_proof_incircuit` on the Echo proof) k / rows | k=18, rows 150,885, table rows 253,969 |
| level 1 keygen vk / pk | 11,910 ms / 5,826 ms |
| level 1 prove | 36,304 ms |
| level 1 proof bytes | 6,432 B |
| level 1 VmHWM after prove | 4,078 MiB |
| level 2: outer ZKIR-v3 circuit (`verify_proof`, DeciderKind::Collapsed) k / rows | k=19, rows 514,873, table rows 507,921, 15 advice, 45 fixed columns, 5 lookups |
| level 2 outer keygen (`IrSource::keygen`, vk + pk) | 95,164 ms (VmHWM 6,381 MiB) |
| level 2 outer prove, guard on (verifies Recursive proof, folds carried accumulator) | 81,947 ms (VmHWM 8,004 MiB) |
| level 2 outer prove, guard off (same pk) | 84,847 ms (VmHWM 8,015 MiB) |
| level 2 PLONK proof / tagged `Proof` bytes | 6,432 B / 6,550 B (1 accumulator, 12 PI fields) |
| level 2 statement length | 1 element |
| level 2 native verify incl. one deferred pairing (discharges both levels), 10 repeats | min 5.61, median 6.15, max 6.39 ms |
| level 2 verify, guard-off proof | 6.42 ms |
| whole process (`/usr/bin/time -v`) | wall 7:47.81, max RSS 8,206,868 kB; harness 467.52 s |

Verification-side negative controls, two-level test:

| # | Tamper | Outcome |
|---|---|---|
| V5 | outer proof carrying the Recursive proof's own accumulator instead of the fold, so the carried Echo accumulator is dropped | REJECTED("Invalid outer proof"), 5.48 ms |
| V1 | accumulator replaced by trivial | REJECTED("Invalid outer proof"), 5.58 ms |
| V2 | one outer proof byte flipped | REJECTED("Invalid outer proof"), 6.13 ms |
| V3 | extra statement element | REJECTED("Invalid outer proof"), 0.65 ms |

Observations:
- The level 1 relation has almost the same row count as the test-1 outer ZKIR
  circuit (150,885 vs 150,966 at k=18). Direct stdlib keygen took 17.7 s;
  `IrSource::keygen` took 79.9 s (84.7 s in the rerun). From the code
  (`zkir-v3/src/ir.rs`), `IrSource::keygen` calls `self.k()`, which runs
  `midnight_zk_stdlib::optimal_k` (a cost-model pass per candidate k) before
  `setup_vk`/`setup_pk`. How much of the difference that search accounts for was
  not measured separately.
- The collapsed-decider outer circuit has 514,873 rows at k=19, against 150,966 at
  k=18 for DeciderKind::None. Folding a carried accumulator cost 3.4x the rows and
  one step of k.
- Proof size and verification time stay flat across recursion depth (6,550 B and
  about 6 ms at one and two levels). Outer proving time does not: 34.8 s at k=18
  for one level, 81.9 s at k=19 for two.

### 7.6 Rerun of `verify_proof_without_a_decider` with verify-after-prove controls (reproduced locally)

Log: `bench/logs/e2e_verify_proof_without_a_decider_rerun.log`. The patch now also
verifies any outer proof that P1, P1d, P2 or P3b produced. Positive-path repeat:
outer keygen 84,718 ms, prove guard on 36,011 ms, guard off 35,497 ms, verify
median 5.88 ms (min 5.09, max 6.81), tagged proof 6,550 B; V1-V3 REJECTED as before.
Whole process: wall 4:22.80, max RSS 4,473,364 kB; harness 262.59 s.

| # | Tamper | Prove / check outcome | Outer verify outcome |
|---|---|---|---|
| P1 | inner proof, midpoint byte flipped | outer proof PRODUCED (35,335 ms); `check` ACCEPTED (5.4 ms) | P1c: REJECTED("inner-proof accumulator failed pairing check"), 5.76 ms |
| P1d | inner proof, last byte flipped | REJECTED at prove in 4.3 ms ("Multi-opening proof was invalid") | not applicable (no proof) |
| P2 | valid inner proof, inner instance ECHO+1 | outer proof PRODUCED (35,272 ms) | P2c: REJECTED("inner-proof accumulator failed pairing check"), 6.54 ms |
| P3/P3b | vk_hash not in side-table | REJECTED at check and at prove (<0.1 ms) | not applicable |
| P4 | padded VK blob | REJECTED at check (1.2 ms) | not applicable |
| P5 | wrong decider tag | REJECTED at check (5.5 ms) | not applicable |
| P6 | missing inner-proof witness | REJECTED at check (<0.1 ms) | not applicable |

Reading: the ZKIR-v3 prover's off-circuit pass rejects inner proofs that fail to
parse, or whose multi-opening transcript is internally inconsistent (P1d). An
inner proof that parses but does not satisfy the pairing (P1), or a valid proof
bound to the wrong instance (P2), is not rejected at proving time. A full outer
proof is produced (about 35 s of work), and PLONK verification of the outer
proof alone passes. Only the deferred accumulator pairing inside
`VerifierKey::verify` rejects it. Any system that stores or forwards outer
proofs without running that pairing (for example batching accumulators for later
discharge) carries unverified inner claims until discharge.

## 8. Proof server 8.1.0

### 8.1 Image inspection (reproduced locally)

```
docker image inspect midnightntwrk/proof-server:8.1.0
  Id          sha256:801bbc0340e9e96f16735f77b523f23c7459e3359842f7c79c2c53f4e994d531
  Created     1970-01-01T00:00:01Z (nix reproducible build)
  Size        26,730,919 B (as reported by docker image inspect; `docker history` shows one layer of 87.7 MB)
  Entrypoint  /nix/store/d24gb0hj1rasyyp3jh0x52i5fd4mkxsi-bash-interactive-x86_64-unknown-linux-musl-5.3p9/bin/bash -c
  Cmd         /nix/store/6naj0x3l5n0b4cx722xwasyp597p6z3h-ledger-8.1.0/bin/midnight-proof-server --port $PORT
  Env         PATH=/nix/store/...-ledger-8.1.0/bin, PORT=6300; ExposedPorts 6300/tcp
  No coreutils in the image (ls/head: command not found).

docker run --rm --network none --entrypoint .../midnight-proof-server midnightntwrk/proof-server:8.1.0 --help
  -p, --port <PORT>                  [env: MIDNIGHT_PROOF_SERVER_PORT=] [default: 6300]
  -v, --verbose                      [env: MIDNIGHT_PROOF_SERVER_VERBOSE=]
      --job-capacity <JOB_CAPACITY>  [default: 0]
      --num-workers <NUM_WORKERS>    [default: 2]
      --job-timeout <JOB_TIMEOUT>    [default: 600]
      --no-fetch-params              [env: MIDNIGHT_PROOF_SERVER_NO_FETCH_PARAMS=]
```

There is no CLI mode that proves a request file. The binary is an HTTP server only.

### 8.2 API and offline behaviour (from pinned source)

Source reading, not execution: `/home/charl/Moriarty/repos/midnightntwrk/midnight-ledger`
at a8ab82ba2124c36f92795c683e70bd888bc1d1fb, `proof-server/src/{main,lib,endpoints}.rs`.
I have not confirmed that the pinned source matches the 8.1.0 binary byte for
byte. The `--help` output matches the source's `Args`.

- Routes: `POST /prove`, `POST /prove-tx`, `POST /check`, `POST /k`, `GET /version`,
  `GET /proof-versions`, `GET /ready`, `GET /` and `/health`, plus a `fetch_k` handler
  (k as a path parameter) registered only when fetching is allowed. I did not
  read that handler's route string.
- `/prove` body: `tagged_serialize((ProofPreimageVersioned, Option<ProvingKeyMaterial>, Option<Fr>))`.
  JS builds it with `ledger-v8` `createProvingPayload(serializedPreimage,
  overwriteBindingInput, keyMaterial)`. `keyMaterial` = `{proverKey, verifierKey, ir}` bytes.
- The serialized preimage comes from `proofDataIntoSerializedPreimage(input, output,
  public_transcript, private_transcript_outputs, key_location)`. Those four values
  are the proof data returned by running the Compact circuit through
  `compact-runtime` against a `CircuitContext` (contract state, zswap local state).
  No wallet and no transaction are needed, but the circuit must execute with a
  valid ledger state.
- Parameters: at startup, unless `--no-fetch-params`, the server fetches SRS k=10..15
  plus the zswap spend/output/sign and dust spend keys. During proving,
  `MidnightDataProvider` uses `FetchMode::OnDemand` from
  `$MIDNIGHT_PARAM_SOURCE` (default `https://srs.midnight.network/`), caching under
  `$MIDNIGHT_PP` or `$XDG_CACHE_HOME/midnight/zk-params` or `~/.cache/midnight/zk-params`,
  and verifies SHA-256 against compiled-in hashes. Offline proving therefore needs
  `--no-fetch-params`, plus a mounted cache containing `bls_midnight_2p<k>` for each
  circuit k. The local cache `/home/charl/.cache/midnight/zk-params` holds 2p6,
  2p14 and 2p15 only.

### 8.3 Existing artifacts

- `find /home/charl/Moriarty/evidence /home/charl/Moriarty/deliverables -path '*compiled*' ...`
  finds only `.zkir` files (SP05 token-api tests; MC01 loan/swap harness
  `record0/1.zkir`). No `.prover` or `.verifier` keys exist under those compiled
  trees.
- A proven loan build with keys exists outside the repo:
  `/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/loan-output/build/loan`
  (compact wrapper 0.5.2, compiler 0.31.1, language 0.23.0, runtime 0.16.0;
  receipt says "compiler key artifacts only; no transaction proof was generated").
  Circuits: `initialize` (prover 5,207,639 B, verifier 2,119 B, zkir 20,896 B),
  `accrue` (5,208,875 / 2,119 / 26,637 B), `settle` (5,204,721 / 2,119 / 35,778 B).
  Headers: `midnight:prover-key[v7](ir-source[v2])`, `midnight:verifier-key[v6]`,
  `midnight:ir-source[v2]`, all compatible in format with a ledger-8 proof server.
- The pinned JS runtime at
  `/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk`
  has compact-runtime 0.16.0, ledger-v8 8.1.0, zkir-v2 2.1.1 (which also has an in-process
  wasm `prove(serializedPreimage, KeyMaterialProvider)`), midnight-js-contracts 4.1.1.
- `/home/charl/Moriarty/experiments/moriarty-midnight-financial` has no standalone
  prove script. Its proving path is `ledger/launch-local.mjs` through
  `midnight-js-http-client-proof-provider`, which needs an existing wallet (seed
  file, wallet state snapshots, DUST), a local node and an indexer. By the task's
  rules it is not usable.

### 8.4 Offline start probe (reproduced locally)

```
docker run --rm -d --name ps-bench -p 127.0.0.1:6300:6300 -e MIDNIGHT_PP=/params \
  -v bench/proof-server/params:/params midnightntwrk/proof-server:8.1.0 \
  "/nix/store/6naj0x3l5n0b4cx722xwasyp597p6z3h-ledger-8.1.0/bin/midnight-proof-server --port 6300 --no-fetch-params --verbose"
```

- Starts in about 1 s with `--no-fetch-params`: 6 actix workers, no parameter download at start.
- `GET /health` -> `{"status":"ok",...}`; `GET /version` -> `8.1.0`; `GET /proof-versions` -> `["V2"]`.
- `bench/proof-server/params` held copies of the local cache (`bls_midnight_2p6`, `2p14`, `2p15`).
- First payload-builder attempt failed: the staged copy of `contract/index.js` is ES
  module source with no `package.json` marking the directory as a module, so Node
  reported "SyntaxError: Cannot use import statement outside a module". Fixed by
  adding `bench/proof-server/package.json` with `"type": "module"`, in the staged copy only.
- Container stopped and removed after the probe (`--rm`).

### 8.5 Offline proof of the loan `initialize` circuit (reproduced locally)

Harness: `bench/proof-server/make-payload.mjs` (Node v24.18.1). It imports the staged copy of
the SP05 proven loan contract, compact-runtime 0.16.0 and ledger-v8 8.1.0, and
runs `initialState` with the fixed synthetic roles from
`ledger/compiled-comparison.test.mjs`. It then calls
`impureCircuits.initialize(ctx, firstSecret, program, net, 2n, TIME)`,
serialises `proofData` with `proofDataIntoSerializedPreimage(..., 'initialize')`,
and builds `createProvingPayload(preimage, undefined, {proverKey, verifierKey, ir: initialize.bzkir})`
and `createCheckPayload(preimage, ir)`. No wallet, node, indexer or network.
Driver: `bench/proof-server/run.log`; requests with curl to 127.0.0.1:6300; container
started with `--no-fetch-params` and `MIDNIGHT_PP=/params` (local cache copies
2p6/2p14/2p15), then stopped and removed.

Input hashes (sha256): initialize.prover 7df149a3…0b52, initialize.verifier
5de6c86d…5985, initialize.bzkir 5c3ff234…b270; prove payload 7ea1b0e8…03db9;
preimage da8c262b…713e.

| Quantity | Value |
|---|---|
| circuit execution in compact-runtime (JS, offline) | 27.02 ms |
| public transcript ops / private transcript outputs | 84 / 0 |
| serialized preimage | 1,191 B |
| `/prove` request body (preimage + prover key + verifier key + IR) | 5,212,303 B |
| `/check` request body | 2,521 B |
| circuit k (`POST /k` on initialize.bzkir) | 14 (server-side 198 ms) |
| `POST /check` | HTTP 200, 1.3 ms, 112 B response |
| `POST /prove` #1 (cold: key and params load) | HTTP 200, 1.736 s (server log 1.735 s) |
| `POST /prove` #2 | HTTP 200, 1.295 s |
| `POST /prove` #3 | HTTP 200, 1.349 s |
| proof response (`midnight:proof-versioned` tagged) | 4,508 B each (three distinct proofs, randomised) |
| parameter download during proving | none (server log shows no fetch; 2p14 served from mounted cache) |
| container memory, one `docker stats` sample during prove #1 | 229.6 MiB, 326.76% CPU (sampling too coarse for #2/#3; see 8.6) |

Proof verification of these three proofs: see 8.6.

### 8.6 Container memory, and what was not measured

Second run (`bench/proof-server/mem_run.log`, same payload, container without `--verbose`),
reading the container's cgroup v2 files
`/sys/fs/cgroup/system.slice/docker-<id>.scope/memory.{current,peak}`:

| Point | memory.current | memory.peak |
|---|---|---|
| after start, before any request | 2,764,800 B | 4,624,384 B |
| after prove #1 (1.557 s) | 219,516,928 B | 242,667,520 B |
| after prove #2 (1.194 s) | 235,552,768 B | 243,953,664 B |
| after prove #3 (1.158 s) | 251,420,672 B | 261,017,600 B (248.9 MiB) |

Across both runs, six `/prove` calls on the loan `initialize` circuit (k=14) took
1.158-1.736 s wall (curl-measured); the first call in each container was the
slowest. Peak container memory is 249 MiB.

NOT MEASURED:
- Verification of the proof-server proofs. ledger-v8 8.1.0 and zkir-v2 2.1.1 expose
  no standalone proof-verify function (only `verifySignature` and transaction-level
  well-formedness flags). Needed: a Rust harness on ledger-8 `transient-crypto`
  `VerifierKey::verify(params, proof, statement)` with the statement rebuilt from the
  preimage (binding input, communications commitment, public transcript), or a
  full unproven-transaction wrap plus `wellFormed` with `verifyContractProofs`.
- Negative controls through the proof server (altered preimage or wrong keys).
- `accrue` and `settle` circuits (they need the post-`initialize` state threaded in).

Environment note: three unrelated long-running containers were up on the host
throughout the session (josie, qbittorrent, ollama; each at 0.00-0.02% CPU and
28-46 MiB per `docker stats`, sampled once at 01:00). No CPU activity from them was observed; they were not sampled during every timed run.

## 9. Required microbenchmarks: status

Legend: MEASURED = run here with the number shown (reproduced locally).
PARTIALLY MEASURED = a structural analogue was run, but not the Moriarty-shaped
case. NOT MEASURED = not run; the exact harness needed is given.

| # | Microbenchmark | Status | Reproduced-locally numbers / harness needed |
|---|---|---|---|
| 1 | One ordinary transition | PARTIALLY MEASURED | midnight-zk IVC genesis step (1000-Poseidon transition plus trivial predecessor, K=18, 163,172 rows): prove 17,512 ms (1-step run) / 19,982 ms (10-step run), verify 11.5-12.3 ms median, proof 5,264 B. No Moriarty transition was proven. Needed: the loan `initialize`/`accrue`/`settle` relation as an `IvcTransition` (section 8 shows the proof-server path for a plain, non-recursive `initialize`). Separately, proof server 8.1.0 proved the compiled Moriarty loan `initialize` circuit (k=14) offline from the SP05 keys: 1.158-1.736 s per `/prove` (6 calls), proof 4,508 B, peak container memory 249 MiB (section 8). This is a single non-recursive Compact transition, and it was not verified. |
| 2 | One authenticated predecessor | MEASURED (toy predecessor) | (a) IVC steady-state step, in-circuit verification of previous step plus transition, K=18: mean 19,004 ms (steps 4-10), verify 12.31 ms median, proof 5,264 B, peak RSS 4.4 GiB. (b) ZKIR-v3 `verify_proof` of one inner Echo proof (PR #738), outer k=18, 150,966 rows: keygen 79,918 ms, prove 34,773 ms, verify incl. deferred pairing 5.88 ms median, tagged proof 6,550 B (1 accumulator, 12 PI fields), VmHWM 4,168 MiB. (c) two-level (collapsed decider): Echo -> midnight-zk-stdlib `Recursive` (k=18, 150,885 rows, keygen 17.7 s, prove 36.3 s, 6,432 B) -> ZKIR-v3 outer with DeciderKind::Collapsed (k=19, 514,873 rows): outer keygen 95,164 ms, outer prove 81,947 ms (VmHWM 8,004 MiB), verify incl. one deferred pairing discharging both levels 6.15 ms median, tagged proof 6,550 B. |
| 3 | 10 sequential | MEASURED (midnight-zk IVC only) | 10 steps K=18: 196,877 ms total proving, per-step 18,649-23,300 ms, proof constant 5,264 B, final verify 12.31 ms median, instance 65 fields. ZKIR-v3 chaining NOT MEASURED. PR #738 has no harness in which a ZKIR circuit verifies a proof of itself. That needs a fixed self-VK design, like `IvcCircuit`, expressed in ZKIR. |
| 4 | 100 sequential | MEASURED (midnight-zk IVC only) | 100 steps K=18, N=1000 (`bench/logs/ivc_k18_n1000_s100.log`): setup 15,241 ms; total proving 1,950,324 ms; per-step mean 19,503 ms, median 19,452, min 18,696, max 21,214; steps 4-100 mean 19,513 ms; per-step native verify 12.4-21.8 ms; final proof verified 20x: median 12.63 ms; proof size 5264 B at every step; max VmHWM over steps 4,405 MiB; process wall 32:48.00, max RSS 4,511,068 kB. ZKIR-v3 chaining NOT MEASURED (see row 3). |
| 5 | 2-way branch | NOT MEASURED | Needed: a transition with a witness (the example's `Witness = ()`), `IvcProver::clone()` at step n, `prove_step(w_a)` and `prove_step(w_b)`, then verify both. The IVC verifier accepts both branches by design, so a ledger-side uniqueness check (nullifier or state-root CAS) must also be measured. |
| 6 | 2-predecessor join | NOT MEASURED | midnight-zk IVC is strictly linear (one `prev_proof`). Needed: a ZKIR-v3 outer IR with two `inner_proof`/`verify_proof` pairs (`accumulator_count() == 2`; the PR's `outer_ir` asserts 1), keygen/prove/verify at the resulting k, or a custom `Relation` calling `verifier.prepare` twice then `accumulate` over three accumulators. |
| 7 | Invalid predecessor | MEASURED | IVC: guard on, `prove_step` returns Err(InvalidProof) in 55.7 ms (N10). Guard patched off: successor proof produced (20,143 ms), native verify REJECTED(InvalidProof) 12.3 ms; a further step also REJECTED (N11). ZKIR-v3: tampered inner proof and wrong inner instance both produce outer proofs (P1, P2); outer verify: REJECTED("inner-proof accumulator failed pairing check") in 5.76 ms (P1c) and 6.54 ms (P2c). PLONK verification of the outer proof passes; only the deferred pairing rejects. An inner proof with its last byte flipped is refused at prove time in 4.3 ms ("Multi-opening proof was invalid", P1d). |
| 8 | Wrong vk | MEASURED | IVC: instance `vk_repr+1` REJECTED(VkMismatch) (N9); verifier of another circuit REJECTED(VkMismatch) or, when the instance claims that circuit's vk, REJECTED(InvalidProof) 12.7 ms (W1-W3). ZKIR-v3: vk_hash not in side-table REJECTED at check and at prove (P3, P3b); padded VK blob REJECTED (P4, and suite `a_padded_blob_is_rejected` pass); wrong decider tag REJECTED (P5, suite `an_unknown_tag_is_rejected` pass). Outer proof verified under a different outer VK: NOT MEASURED (needs a second outer keygen; the outer keygen measured here took 79.9 s and 84.7 s). |
| 9 | Altered public state | MEASURED | IVC: `cnt+1`, `val+1`, stale instance with current proof, current instance with stale proof, all REJECTED(InvalidProof) in 11.9-12.8 ms (N1, N2, N4, N5). ZKIR-v3: extra statement element REJECTED("Invalid outer proof") 0.41 ms (V3); exposed accumulator replaced by trivial REJECTED 4.93 ms (V1); two-level proof carrying the Recursive proof's own accumulator instead of the fold (carried accumulator dropped) REJECTED("Invalid outer proof") 5.48 ms (V5). |
| 10 | Stale ledger state | PARTIALLY MEASURED (structural analogue only) | IVC stale accumulator (N3) and stale instance (N4) REJECTED(InvalidProof). No ledger state root is in either statement, so ledger staleness is not tested. Needed: a transition whose public state includes a ledger state commitment, and a verifier/ledger check that compares it to current state. Moriarty has `ledger/stale-loan-rejection.mjs` for the Compact path, but it needs a local node and wallet and was not run. |
| 11 | Altered recipient / asset / fee | NOT MEASURED | The example state has no recipient, asset or fee fields. N1/N2 show only that generic state fields are bound. Needed: loan `settle` effects (recipient address, `usdColor`, amount, DUST fee) as IVC state or ZKIR statement fields, a positive proof, then one tamper per field against the same proof. |
| 12 | Multi-party successor | NOT MEASURED | Needed: a successor whose relation requires authorisations from two parties: either two in-circuit signature checks, or a ZKIR-v3 IR with two `verify_proof` instructions over proofs made under two parties' VKs. Measure keygen/prove/verify, and rejection when one party's input is missing or substituted. |

## 10. Documented elsewhere

No numbers from other hardware or third-party reports are used in this note.
Every figure above was measured on the machine in section 1.

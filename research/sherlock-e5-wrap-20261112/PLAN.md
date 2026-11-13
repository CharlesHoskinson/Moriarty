# E5 plan: the one BLS12-381 wrap that verifies an `L_Fr` proof natively in Fr

Owner ruling 2026-11-12: OD-24 is option E. Item E5 of `~/Projects/Moriarty/research/sherlock-e-path-20261112/LOOP.md`. Build plan row E5: `research/sherlock-m7-wrap-20261112/RESULTS.md` section 2c (worktree `~/Projects/Moriarty-wt/m7-wrap`). Branch `e5-wrap` (worktree `~/Projects/Moriarty-wt/e5-wrap`), made from `e2-lfr` 6fc8e3c21. Nothing is pushed. `main` is not touched.

## 1. Gate (E5 row)

| Gate item | Check |
|---|---|
| Rows | `MockProver` rows of the full circuit on a real `L_Fr` proof are within the k = 21 gate (1,782,573 rows). Never k = 22. |
| Key | Verifier key bytes in the `MidnightVK` framing (arch header, k, public-input count, vk) at most 50,000. |
| Ledger | Ledger 9.1.0.1 (`midnight-ledger` tag `ledger-9.1.0.1` = rev `a2b30128`) `VerifierKey::verify`, unchanged, accepts one blinded proof (`OsRng`). |
| Negative controls | The ledger refuses a changed public input and a changed proof byte; the circuit refuses a changed `L_Fr` proof value. |
| Public inputs | 1,026 (`2 + L_TX`, `L_TX` = 1,024, PROPOSAL section 7.1, OD-15). |

Hard rules: one BLS12-381 wrap; zero knowledge end to end (the wrap proof is blinded); proven 100-bit security (the `L_Fr` terms are the E1 D3 terms; the wrap adds no soundness premise beyond PLONK-KZG of the existing decider); one universal key set; ledger verifier unchanged; never k = 22.

## 2. Where the code lives (decision)

M7 put its wrap code in the Moriarty worktree, in the standalone workspace `tools/gpu-prover` (own `Cargo.lock`, vendored and patched midnight crates, `m7_ledger_toy.rs`). E5 does the same:

| Part | Place | Why |
|---|---|---|
| Wrap circuit, native verifier, tests | `tools/sherlock-wrap` in the Moriarty worktree `e5-wrap`: a standalone workspace like `tools/gpu-prover`, lock seeded from `tools/gpu-prover/Cargo.lock` | the midnight graph (stdlib 2.4.0, circuits 7.3.0, proofs 0.8.3) stays apart from the p3 and openvm graph |
| Ledger check | `tools/sherlock-wrap/ledger-check`: a second standalone workspace with the stock (not vendored) midnight crates and `midnight-transient-crypto` at rev `a2b30128` | the verdict comes from a dependency graph that holds no patched midnight crate |
| Exporter of `L_Fr` proofs | branch `e5-wrap` of the openvm clone, in a separate git worktree `~/.cache/m7-wrap/src/openvm-e5` (so the `../openvm-org_stark-backend` path patch resolves). Never a branch switch in the two shared clones: E2 I10 works in them. | it needs the stock verifier, the vk types and the E2 machine |

The two sides meet in one exported file (`wrapin`, section 5). The wrap crate has no p3 or openvm dependency.

## 3. Poseidon access (vendored, visibility only)

The `L_Fr` hashes use the Midnight Poseidon permutation with the capacities `cap_fixed(5, l)` (leaf), `cap_fixed(6, 2)` (node) and `cap_var(7)` (transcript duplex). The stdlib exposes only `ZkStdLib::poseidon` (the u = 0 fixed-length sponge: capacity `len`, one output). `PoseidonChip::permutation` is `pub(super)`, and `ZkStdLib::poseidon_gadget` is private (midnight-circuits 7.3.0 `hash/poseidon/poseidon_chip.rs:422`; midnight-zk-stdlib 2.4.0 `src/lib.rs:342`).

Fix (the route of M7 PLAN, M7a): vendored copies `vendor/midnight-circuits-7.3.0-perm` and `vendor/midnight-zk-stdlib-2.4.0-perm` with a visibility-only patch `patches/perm-access.diff`:
- `PoseidonChip::permutation` and the type `AssignedRegister` become `pub`;
- `ZkStdLib::poseidon_chip(&self) -> &PoseidonChip<F>` is added.

`configure` of both crates is not changed, so `ZkStdLib::configure(arch, k - 1)` builds the same constraint system for the patched prover and the stock ledger reader. The ledger check (section 2) makes this MEASURED.

Architecture: `ZkStdLibArch { poseidon: true, nr_pow2range_cols: N, ..default }` (N set at keygen, 1 to 4).

## 4. Public inputs (LEAD decision, parameter)

The wrap statement is the call vector of `2 + L_TX` = 1,026 words (PROPOSAL section 7.1). How the call vector is computed from the public values of `L_Fr` (the `L` vk pre-hash, then the `L` public values packed 8 per Fr, E2 `PLAN.md` D-2) is E3 work and is not written in any source read for E5. Until E3 fixes it, the wrap takes a parameter `PublicMap`: the first `n` words are the `L_Fr` public values in order, the remaining `1,026 - n` words are constrained zero. LEAD, to be replaced by the E3 map.

## 5. The exported input (`wrapin`, JSON, version 1)

Produced from the stock objects by the exporter; read by the wrap crate. Fr values are 64-digit hex, big-endian canonical.

- `params`: `l_skip`, `n_stack`, `log_blowup`, `k_whir`, rounds with `num_queries`, `log_final_poly_len`, PoW bits (LogUp, `mu`, folding, query), domain codes (5, 6, 7).
- `vk`: pre-hash; per AIR: `is_required`, preprocessed commit and `hypercube_dim`, `need_rot`, widths, public-value count, the symbolic constraint DAG (nodes in order), interactions (bus, message and count expressions as DAG node ids), and the trace-height constraints.
- `shape`: per AIR presence and log height (the shape pin, LEAD D-1 of E2), the layouts of `verify_proof_shape`.
- `proof`: every field of `Proof<SC>`.
- `log`: the stock transcript record (observe, observe-commit and sample events in order), from a recording wrapper around the stock transcript while `verifier::verify` accepts.

The circuit is generated from `params`, `vk` and `shape` (key constants) and fed with `proof` (witness). `log` is a test oracle only.

## 6. Circuit design

- One generic verifier, written against a trait `Ops` with two back ends: `Native` (Fr values; witness generator and plain verdict) and `Gadget` (stdlib cells and the layouter). The `MockProver` checks that the two agree.
- Every check of the stock verifier is an `assert_equal` (or `assert_zero`) in the circuit. Map of the stock `Err` returns:

| Stock error (file) | In the wrap |
|---|---|
| `SystemParamsMismatch`, `TraceHeightsTooLarge`, `PreprocessedTraceHeightMismatch`, `EmptyTraces`, every `ProofShapeError` (`mod.rs`, `proof_shape.rs`) | key constants: params, presence, heights and layouts are fixed at keygen; the generator refuses a proof of another shape |
| `InvalidLogupPowWitness`, `MuPoWInvalid`, `FoldingPoWInvalid`, `QueryPhasePoWInvalid` | `check_witness`: observe, sample, canonical low-bits decomposition, low bits = 0 |
| `GkrVerificationError` (zero round, zero check, root and layer consistency), `GkrNumeratorMismatch`, `GkrDenominatorMismatch`, `SumClaimMismatch`, `InconsistentClaims` (`fractional_sumcheck_gkr.rs`, `batch_constraints.rs`) | `assert_equal` of the two sides |
| `S0Mismatch`, `FinalSumMismatch` (`stacked_reduction.rs`) | `assert_equal` |
| `MerkleVerify` (`whir.rs`) | root recomputed from the opened rows; `assert_equal` with the commitment cell (key constant or observed proof value) |
| `FinalPolyDegree` | key constant (length of the final polynomial) |
| `FinalPolyConstraint` | `assert_equal(acc, claim)` |

- Soundness points that a passing test does not show (DERIVED in RESULTS.md as each lands):
  - `sample_bits(b)`: `x = lo + 2^b hi` with `lo < 2^b`, and canonicity: `r = c 2^b + 1`, so `x <= r - 1` means `hi <= c` and `hi = c` forces `lo = 0`. Without canonicity the prover could use `x + r`. A negative test uses the `x + r` split.
  - The query index bits that select the Merkle path direction are the same cells as the sampled low bits.
  - The vk pre-hash, the heights, the presence bits, the commitments of preprocessed traces and every DAG constant are fixed cells (key constants).
  - The PoW witnesses and every proof value are private witness; they enter the transcript as observed cells.

## 7. Increments (each with its own test; commit after its tests pass)

| # | Increment | Test gate |
|---|---|---|
| I0 | Crate `tools/sherlock-wrap`, vendored patch, lock check (no new version); native Midnight Poseidon and caps; a ledger smoke circuit | the native permutation equals midnight's `permutation_cpu`; the smoke circuit (poseidon arch, the patched permutation, 1,026 public inputs, small k) gives a `MidnightVK` key that stock ledger `a2b30128` `VerifierKey::verify` accepts with a blinded proof, and refuses a changed public input and a flipped proof byte. If the ledger refuses, stop and report (M7 stopped here). |
| I1 | Fr transcript replay (duplex at u = 7: observe, observe-commit, sample, `sample_bits`, `check_witness`) | on the recorded stock transcript of a real `L_Fr`-config proof, every in-circuit sample equals the stock sample; `MockProver` passes; refused: a changed observed value, a non-canonical `x + r` split. Rows per permutation and per absorbed word MEASURED; DERIVED full-circuit rows at `w_stack` 48 before I2 (stop if over the gate). |
| I2 | Merkle paths (leaf u = 5 over `w` values, node u = 6, `tree_compress` of the `2^k_whir` leaf digests, path direction from the index bits) | every opening of the WHIR proof checks against its root; refused: changed sibling, changed opened value, changed index bit |
| I3 | Sumchecks: GKR fractional sumcheck, zerocheck rounds, `eq` and Lagrange helpers | the GKR and batch-constraint claims of the real proof equal the stock values; refused: changed layer claim, changed round polynomial |
| I4 | Constraint DAG of the `L_Fr` AIRs, interaction numerator and denominator, the batched zerocheck final claim, stacked reduction | the stock `evaluated_claim` and the stacked final sum are reproduced; refused: changed column opening, changed stacking opening |
| I5 | WHIR verifier (`mu` batching, folding sumchecks, OOD, queries, `binary_k_fold`, final polynomial check) | the whole WHIR check of the real proof passes; refused: changed final-polynomial coefficient, changed OOD value |
| I6 | Full circuit on a real `L_Fr` proof: public inputs (1,026), `MockProver` rows, keygen at the smallest k <= 21, key bytes, blinded proof, library verify, ledger verify and negative controls | the gate of section 1 |

Fixtures: first the E1 Fr engine at the `L_Fr` domains (u = 5, 6, 7) and the `L_Fr` parameter set (UD, log blowup 3, queries 98/83/82, query PoW 20, `l_skip` 2, `n_stack` 18, `k_whir` 4) on a small AIR set with interactions and preprocessed data; then the E2 machine proof (I9 tape, or the I10 proof at the `L_Fr` set when it exists). The `L_Fr` shape is a parameter (the `wrapin` file), never hard-coded.

## 8. Rules for every run

- Builds and tests: `flock ~/.cache/cost-opt/measure/heavy.lock /home/hoskinson/.local/bin/heavy-run --mem <size> --name e5-<stage> -- ...`, runner `~/.cache/e5-wrap/run.sh`; over 30 minutes as a `systemd-run --user --collect` unit; logs in `~/.cache/e5-wrap/logs/`; `CARGO_TARGET_DIR=~/.cache/e5-wrap/target`; `RUSTUP_AUTO_INSTALL=0`; no `+nightly`.
- Install nothing; no crate version outside the openvm, backend or midnight lock files (checked by script on each lock change).
- Every number is MEASURED, DERIVED or ESTIMATE.

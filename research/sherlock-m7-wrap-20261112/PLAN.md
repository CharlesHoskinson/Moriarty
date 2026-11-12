# M7: the real G4 wrap of a real `L` proof

Agenda item S10, milestone M7. Branch `m7-wrap` (worktree `~/Projects/Moriarty-wt/m7-wrap`), made from `main` 4ea6fa29d with `l-structure` (3957ddf7c) and `gpu-wrap` (52ad95069) merged in. Nothing is pushed. `main` is not touched.

Inputs (read by absolute path; they are untracked in `main` and are not added to this branch):

- `~/Projects/Moriarty/research/sherlock-processor-final-20261111/PROPOSAL.md` sections 3.4, 3.5, 8, 9 and 13.
- `~/Projects/Moriarty/research/sherlock-design-paper-review-20261111/UD-SIZING.md` (OD-19 set: log blowup 3, queries 98/83/82, fold PoW 0, `mu` PoW 6, query PoW 20, LogUp PoW 18, 1 OOD sample).
- `~/Projects/Moriarty/research/sherlock-call-time-opt-20261111/proposals/wrap-rows.md`.
- `research/sherlock-stark-framework-20261111/L-STRUCTURE.md` and `wrap_model.py` (merged from `l-structure`).
- `research/sherlock-gpu-wrap-20261111/RESULTS.md` and `tools/gpu-prover` (merged from `gpu-wrap`).

Rules for every run: builds and proves go through `flock ~/.cache/cost-opt/measure/heavy.lock /home/hoskinson/.local/bin/heavy-run`; a run over 30 minutes is a `systemd-run --user --unit=m7-... --collect` unit with logs in `~/.cache/m7-wrap/`. Every number is MEASURED or ESTIMATE. Production proofs are blinded with a fresh CSPRNG (`OsRng`); `GW_DET` and `GW_CHIP_SEED` stay unset outside tests. If the measured rows exceed the k = 21 gate (1,782,573), the work stops and the breakdown is reported.

Baseline to beat (ESTIMATE, `UD-SIZING.md` section 3, `wrap_ud.out`): G4 at UD blowup 3 is 562,558 / 639,486 / 672,117 rows (low / mid / high) at the scenario `w_stack`, 697,659 mid at `w_stack` 36 and 728,763 mid at `w_stack` 48. k = 20 in every case. The old 434,692 figure is at the list-decoding queries 57/28/19 and does not apply.

## M7-0 Blocking checks (before any gadget)

1. **Ledger acceptance of a non-stdlib constraint system.** Ledger 9 reads a verifier key with `MidnightVK::read` (`repos/midnightntwrk/midnight-ledger/transient-crypto/src/proofs.rs:533`). That function rebuilds the constraint system from `ZkStdLib::configure(&mut cs, (architecture, k - 1))` and then reads only the commitments with `VerifyingKey::read_from_cs` (`midnight-zk-stdlib-2.4.0/src/lib.rs:1573-1587`). The key bytes carry the architecture flags, not the gates. A G4 key (custom lane gates) therefore cannot be read as its own circuit. Toy `tools/gpu-prover/src/bin/m7_ledger_toy.rs` makes this MEASURED: one lane of 8 advice columns, one degree-5 gate, one lookup, 1,026 public inputs, ceremony SRS, blinded proof, library verify, then the ledger path under three architecture headers.
2. **Single-lane stdlib wrap at the OD-19 set.** The one shape the unchanged ledger reads is a `ZkStdLib` architecture (5-column `NativeChip`, stdlib Poseidon, pow2range lookups). The wrap model gives it 2,370,944 / 2,620,323 / 2,724,845 rows at the scenario `w_stack` and 2,947,241 mid at 48 (ESTIMATE, `wrap_model.py` shape 1 with the 19-row Poseidon, UD blowup 3, script in section "Commands" of RESULTS.md). That is over the k = 21 gate in every case.
3. If both checks hold, M7 cannot meet "G4 and ledger 9 unchanged" together. The work stops at M7-0 and the lead chooses among the options in RESULTS.md section 2.

## M7a Gadget skeleton and row counter on the real `L` shape

- One gadget code base with two back ends: a counting back end (rows per verifier line, no field work) and the midnight-proofs assign back end. Rows are MEASURED only from the layouter or keygen; the counter is checked against them on a small instance.
- Shape asserted at `configure`: degree at most 5, at most 4 lookups into one 2^16-row table, rotations in {-1, 0, +1}, 4 lanes x 8 advice columns, shared fixed columns.
- Poseidon per lane: the stdlib 19-row layout (midnight-circuits 7.3.0, `NB_SKIPS_CIRCUIT = 5`), with the bare permutation exposed by a vendored patch beside `tools/gpu-prover/patches/chip-seed.diff`.
- The verifier lines follow stark-backend v2.1.0 `verifier/*.rs` and the halo2 template `openvm-static-verifier` (its stages `proof_shape`, `batch_constraints`, `stacked_reduction`, `whir`, `full_pipeline`).
- Output: rows per line on `L_cached` (39 AIRs, 1,537 columns, 487 interactions) at the OD-19 set.

## M7b Full circuit on one real `L` proof

- An Fr configuration for stark-backend: a p3 field for BLS12-381 Fr (fork of p3-bn254 0.4.3 with the BLS modulus), the Midnight Poseidon permutation (width 3, 8 full and 60 partial rounds, `x^5`), the section 3.6 capacities `cap(u, l)` and the transcript start `s_2 = 2^64 + 3 * 2^66`. Template: `stark-sdk/src/config/baby_bear_bn254_poseidon2.rs`. Sources copied to `~/.cache/m7-wrap/`, not the shared stark-bench checkout.
- One real `L` proof: W1 leaf, bench node, `L` at the OD-19 set (CPU prove), checked by the stock verifier, which also gives the transcript reference.
- `MockProver` of the wrap on that proof; measured rows.

## M7c Keygen and key bytes

- k = the smallest of 19, 20 and 21 at which the measured rows are within 85 percent of `2^k - 7`. Never 22.
- Verifier key bytes in the ledger framing; gate: at most 50,000.

## M7d Prove and ledger verify

- One blinded proof (`OsRng`), proof bytes, prove time, library verify, verify time at 1,026 public inputs, negative controls (one public input changed, one proof byte flipped), and ledger 9 `VerifierKey::verify`.

## M7e Devnet call

- Deploy the wrap key under an entry label on the local devnet (node 2.1.101, ledger 9.1.0.1 line, as in XC-9 and `DEPLOY-REHEARSAL.md`), one call, its fee and inclusion.

## Lead decisions (owner to confirm)

- The real `L` is the bench `L_cached` (39 AIRs, no log AIRs, 0 public values of its own); the log section of the 1,026 public inputs is filled at capacity 256/256/0 with all-`NOP` entries.
- Until D10 gives heights, `w_stack` is set from the proof that M7b makes, within the key bound 48.

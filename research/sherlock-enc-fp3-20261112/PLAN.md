# ENC-1 to ENC-5 and FP-3 facts 1 to 3: plan

Loop items ENC and FP3 of `research/sherlock-e-path-20261112/LOOP.md` (main checkout). Branch `enc-fp3` from `final-design`. `FD/` is `research/sherlock-final-design-20261111/`, `PR` is `research/sherlock-processor-final-20261111/PROPOSAL.md`, `SBW` is the stark-backend worktree `~/.cache/m7-wrap/src/stark-backend-enc-fp3` (branch `enc-fp3` from `e2-lfr` 5f698b5 of the clone `~/.cache/m7-wrap/src/openvm-org_stark-backend`).

## Inputs

- `FD/ARCHITECTURE.md` 1.8 (log, call vector, hash domains), 1.11 FP-1 (word types), FP-3 (facts 1 to 3), FP-4 (digest table, tag codes), FP-7 (V+), 1.12 (ENC-1 to ENC-5; rows I-19, I-21 to I-25, I-29, I-31, I-33, I-34, I-36).
- `PR` 3.2, 3.6, 4.2, 7.2, 7.3.
- Owner rulings 2026-11-12 (`DECISIONS-ASK.md`): OD-17 and OD-18 confirmed; `L_Fr` use codes u = 5, 6, 7, transcript u = 7; AD-1 TOP = 63; AD-11; AD-18.
- Code: `SBW/crates/stark-sdk/src/config/{baby_bear_bls12381_poseidon.rs, bls12381_fr_poseidon.rs, baby_bear_poseidon2.rs}`; midnight-circuits 7.3.0 `PoseidonChip` (CPU); the landed tag rule `dom()` at `crates/mori-recursion/src/parent.rs:115`. No Moriarty crate computes `P2`, `SHK_PROGC`, `SHK_LOCK`, `SHK_INPUT`, `SHK_OUTCOME` or `DOM_LAYOUT` today (rg over all worktrees).

## Work

1. ENC-1: a stated rule for the four Fr tag values and the `DIGEST` tag index; vectors of `PoseidonFr_4` for each tag.
2. ENC-2: `enc(e)` word by word for every operation, reconciled with OD-17 (`DIGEST` has 2 values); the Lean amendment listed, not made.
3. ENC-3: the typed V+ tape parameters and the `layout_d` preimage; options for the byte packing; a vector at the measured layout (26 domain bytes, offset 58).
4. ENC-4, ENC-5: the typed layouts as far as the rulings fix them; gaps listed with options.
5. `P2` (FP-4 form) written as a reference in the test on the stock BabyBear Poseidon2; every choice that the text leaves open listed for the owner.
6. Vectors: one new test file `SBW/crates/stark-sdk/tests/enc_vectors.rs` (feature `bls12381-fr-poseidon`), each digest computed two ways (config path and a hand-rolled sponge on the bare permutation; `u = 0` against midnight-circuits `PoseidonChip::hash`), plus the G3 transcript vectors of `L` (u = 3, multi-field) and `L_Fr` (u = 7, native Fr duplex) on the cases `PR` 3.6 names. The printed vectors are copied into `VECTORS.txt` here.
7. FP-3: a timeboxed search of later sources for facts 1 and 3; if no source fixes them, stop with options and a recommendation. Fact 2: compile the LEDGER CONTRACT sketch of `acceptRun` with `compactc` 0.36.0 and read its public-input lanes as evidence (N-11 is unruled; `acceptCollection` waits on fact 3).
8. ARCHITECTURE 1.12 rows change only where a gate closes. 1.12 closes an ENC item with vectors on the Rust reference, the AIR and, for ENC-1 to ENC-3, the wrap; this item gives the Rust reference only, so rows get a pointer, not FROZEN.

## Rules

Builds through `flock ~/.cache/cost-opt/measure/heavy.lock heavy-run --mem <size> --name enc-<stage>`, logs `~/.cache/enc-fp3/`, `CARGO_TARGET_DIR=~/.cache/enc-fp3/target`, `RUSTUP_AUTO_INSTALL=0`, `--locked --offline`, no nightly. No installs, no new crates.

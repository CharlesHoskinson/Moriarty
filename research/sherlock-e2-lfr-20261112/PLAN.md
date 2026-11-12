# E2 plan: the Fr recursion AIR set `L_Fr` that verifies one `L` proof

Brief: `BRIEF.md` beside this file. Gate: `L_Fr` proves over the real `L` proof and the stock verifier accepts it; negative controls refuse a changed `L` proof byte and a changed public word; heights MEASURED, then D3 at the measured shape.

## 1. What `L_Fr` must verify (CODE and MEASURED inputs)

| Item | Value | Source |
|---|---|---|
| Config of `L` | BabyBear base, `EF` degree 4, Midnight Poseidon over BLS12-381 Fr (width 3), Merkle leaf u = 1 (8 BabyBear words per Fr), node u = 2, transcript u = 3 (multi-field overwrite duplex, 5 base-`p` digits per squeeze) | `stark-sdk/src/config/baby_bear_bls12381_poseidon.rs` |
| `L` parameters | UD, log blowup 3, queries 98/83/82, `mu` PoW 6, query PoW 20, LogUp PoW 18, `l_skip` 2, `n_stack` 18, `k_whir` 4 | `lfr/l_fr.rs` `l_ud(48)` |
| `L` structure | 39 AIRs, 1,537 columns (1,525 common, 12 cached), 2 commitments, no preprocessed trace, 487 interactions, 2,210 constraints, 15,331 DAG nodes, max degree 4, 0 public values (MEASURED, keygen) | `research/sherlock-stark-framework-20261111/L-STRUCTURE.md` |
| `L` proof | 430,073 bytes, 14 distinct heights, maximum 2^16, `T` 21, stacked width 18 (MEASURED) | M7 `RESULTS.md` 2b.3 |
| Verifier stages | proof shape, preamble, LogUp GKR (fractional sumcheck), zerocheck and constraint batching with univariate skip, stacked reduction, WHIR | `stark-backend/src/verifier/*.rs`, 2,408 lines without the test extractor |

**Shape dependence (CODE, `verifier/mod.rs`, `proof_shape.rs`).** The verifier reads from the proof: each `log_height`, the presence of each optional AIR, the sort order of AIRs by height (`trace_id_to_air_id`), and through them the stacked layouts, `n_max`, the GKR depth and the number of sumcheck rounds. Every other loop bound comes from the vk and the parameters.

## 2. Route

**Chosen: a dedicated `L` verifier, driven by the `L` vk with the `L` shape pinned in the `L_Fr` key.** The verifier is a straight-line program (a "tape") over typed values, generated from the vk and the pinned shape. A small set of generic Fr AIRs executes the tape.

| Criterion | Port of OpenVM `crates/recursion` (35,162 lines) | Dedicated `L` verifier (chosen) |
|---|---|---|
| Field of the constraints | written natively over BabyBear (every constraint is a BabyBear identity); at an Fr base every product needs a quotient and range checks, so almost every constraint is rewritten | written once over Fr with emulated BabyBear and `EF4`; each product is one Fr identity plus range checks, because `34 p^2 < 2^68` is far below `r` |
| Hash | Poseidon2 over BabyBear sub-AIRs; `L` uses Midnight Poseidon over Fr, so the transcript, Merkle and Poseidon AIRs are replaced, not ported | Midnight Poseidon is native in Fr: one sub-AIR row per permutation |
| What the port buys | dynamic proof shape (any heights, any AIR presence) | not needed: the `L` vk is one universal key, and its shape is pinned (LEAD below) |
| Code size (ESTIMATE) | most of 35,162 lines touched | tape generator about 3,000 lines (mirror of the 2,408-line verifier) plus 5 to 7 AIRs |
| Testability | needs the whole port before the first end-to-end check | each stage of the tape is checked against the stock verifier's transcript record on small fixtures and on the real `L` |
| Wrap rows | `L_Fr` has about 39 AIRs (OpenVM layout), each a wrap cost | few AIRs, so fewer commitments, openings and DAG nodes for the wrap (E5) |
| Universality | one key verifies any `L` shape | one key per pinned `L` shape; `L` is one universal key, so one `L_Fr` key |

**Decisions.** D-1 to D-3 are LEAD (owner to confirm); D-4 is decided.

- D-1 Shape pin. The `L_Fr` key fixes the `L` vk and the `L` trace heights (per AIR, presence included). `L_Fr` constrains every `log_height` and presence bit observed in the transcript to equal the pinned values. A proof of another shape is refused. A different `L` shape needs a new `L_Fr` key. If the production `L` shows proof-dependent heights, a bounded set of pinned shapes (one key each) or a shape-generic tape (dynamic loop bounds) is the fallback.
- D-2 Public values of `L_Fr`: the `L` vk pre-hash (one Fr word), then the public values of `L` (BabyBear words packed 8 per Fr, in vk order). The bench `L` has 0 public values, so the public-word control on the real `L` changes the pre-hash word, and the control on a changed `L` public value runs on a small fixture with public values.
- D-3 Hash tags of `L_Fr`'s own proof: leaf u = 5, node u = 6, transcript u = 7 (config fields `FrDomains`), distinct from `L`'s u = 1, 2, 3 and from u = 0 and u = 4 (PROPOSAL 3.6, OD-18). Inside the tape, `L`'s hashes keep u = 1, 2, 3.
- D-4 **DECIDED (owner ruling 2026-11-12).** One shared program table: every preprocessed program of every `L_Fr` machine AIR is in ONE `ProgramTable` AIR, so an `L_Fr` proof has 2 commitments (common main and the table), the count that the E5 wrap estimate (879,472 mid rows) assumes. Done in I5c; MEASURED 2 on the I5b tape (`RESULTS.md` section 8b). Every AIR added in I6 to I10 takes its program from this table.

## 3. Architecture of `L_Fr`

Values in the tape: Fr words (native), BabyBear words (canonical, in `[0, p)`), `EF4` elements (4 canonical BabyBear coefficients, `x^4 = 11`).

As built (I2 to I5c; `RESULTS.md` sections 5 to 8b). Each execution AIR carries its slice of the tape (op kind, operand and result addresses, result multiplicities, constants) as main columns after its other columns, and receives each row on the program bus from the one preprocessed `ProgramTable` AIR (D-4), so the key fixes the tape and the proof has 2 commitments.

| AIR | Rows | Job |
|---|---|---|
| `Input` | one per BabyBear or `EF4` input or constant | the value is witness (a value of the `L` proof) or a constant of the key; each coefficient canonical |
| `FrInput` | one per Fr input or constant | commitments of `L`, the transcript's initial state |
| `BbAlu` | one per BabyBear or `EF4` op | add, sub, mul; `a op b = q p + r` per coefficient as an Fr identity; `r` canonical (16-bit limbs, `hi <= 0x7800`, `hi = 0x7800 -> lo = 0`); `q` bounded by limbs |
| `PoseidonFr` | one per permutation | Midnight Poseidon, width 3, 8 full and 60 partial rounds, `x^5` with an `x^2` column (degree 3) |
| `Pack` | one per packed Fr word | `sum b_i 2^(31 i)` of up to 8 BabyBear words (transcript observes; later the u = 1 leaves) |
| `Split` | one per squeeze | 5 base-`p` digits and a high part, unique by the bound of the integer below `r` |
| `Reshape` | one per `EF4` compose or decompose | `sample_ext` and `observe_ext` |
| `ProgramTable` | one per op, plus one nop row per AIR kind | the only preprocessed trace: `[tag, cells[18], count]`; sends each program row with count 1 and the nop row with the padding count of its AIR kind; every execution row receives `[tag, its program cells]` with count 1 |
| `Range16` | 2^16 | the 16-bit lookup table (LogUp) |
| memory bus | (no AIR) | write-once: each tape value has one address and one writer, which sends it with the fixed read count; each read sends it with count `-1` |

Added in I6: `AssertEq` (every verifier check) and `LowBits` (`sample_bits`). Added in I9: `Halve` (index bits), `FrAdd` (leaf sponge), `CSwap` (Merkle path order); no `EF4` inversion is needed (I6 to I9 use only key-constant inverses; WHIR's `x^-1` comes from the index bits). The transcript is not an AIR: `TapeTranscript` emits `Perm`, `Pack` and `Split` ops.

The row and cell counts of the full tape are MEASURED only after the tape exists (increment I6 onward).

**Budget (ESTIMATE until I6).** Each AIR at most 2^20 rows (`l_skip + n_stack` = 20); all cells at most `w_stack` x 2^20 (48 is the key bound; D3 used 18 and 48); PROPOSAL 11.2 bounds: at most 50 AIRs, 2,000 columns, 1,000 constraints per AIR, 100 interactions per AIR. Op counts from the wrap model (`M7 RESULTS.md` 2b, BabyBear-hash `L`, ESTIMATE): about 32,000 `EF4` products and 22,161 permutations. At one row per permutation and about 170 columns, `PoseidonFr` is about 2^15 rows and 5.6 M cells; `BbAlu` at about 40 columns is about 2^16 rows and 2.6 M cells.

## 4. Increments and test gates

Code: the `L_Fr` crate `crates/sherlock-lfr` in the openvm checkout, branch `e2-lfr` (it needs `openvm-recursion-circuit` for the fixtures); the domain-tag change in the backend clone, branch `e2-lfr`. Test parameters: E1's small Fr set (UD, log blowup 1, `l_skip` 2, `n_stack` 8, `k_whir` 4); the `L_Fr` set (175 s per proof regardless of trace size, E1) only at the gate.

| # | Increment | Test gate |
|---|---|---|
| I0 | Fixture: the real `L` proof, `L` vk and node vk written to `~/.cache/e2-lfr/fixture/` by a new example; re-read and accepted by the stock verifier on backend `e2-lfr` | decode, verify accepts; one byte flip refused |
| I1 | Domain tags D-3: `FrDomains::l_fr()` and engines at those domains | an `L_Fr`-domain proof verifies; a default-domain verifier refuses it and the reverse; E1 tests unchanged |
| I2 | `BbAlu` and `Range16`: emulated BabyBear and `EF4` add, sub, mul in Fr | a random op batch proves and verifies at the small Fr set; results equal p3 BabyBear and `EF4`; refused: a non-canonical result `r + p`, a wrong quotient, a wrong result, a limb out of range |
| I3 | `PoseidonFr` AIR | outputs equal `MidnightPoseidonFr` on random states; refused: a changed output, a changed intermediate S-box value |
| I4 | The tape and the write-once memory bus (`Input`, `BbAlu` with preprocessed programs) | a toy tape (Horner over `EF4`, a hash) proves; refused: a changed read value, a read of an unwritten address |
| I5 | `Transcript` AIR | its absorb and squeeze trace equals the stock `MultiFieldTranscript` on a recorded `L` transcript; refused: the squeeze witness `N + r` (non-unique split), a changed digit |
| I5c | D-4: one shared `ProgramTable` AIR holds every program; execution AIRs receive their rows on a program bus | the I5b test on the real `L` proof: 1,500 of 1,500 samples equal, proves; the I4 and I5 negative controls still refused; new: a changed program cell in an execution row and a forged write from padding rows refused; commitment count MEASURED at 2 |
| I6 | Tape generator, part 1: shape pin, preamble, LogUp GKR | on the small fixtures and the real `L`, every observed and sampled value equals the stock verifier's transcript record; op counts MEASURED |
| I7 | Part 2: zerocheck and constraint batching, including the DAG of every `L` AIR | as I6 |
| I8 | Part 3: stacked reduction | as I6 |
| I9 | Part 4: WHIR, with the `Merkle` AIR (u = 1, 2 inside), query bits, folds, final polynomial | as I6; a changed opening or Merkle sibling refused by the plain-Rust tape run |
| I10 | E2 gate: the tape of the real `L` proved on the AIRs at the `L_Fr` set | stock verifier accepts the `L_Fr` proof; refused: a changed `L` proof byte, a changed public word; heights MEASURED; D3 rerun at the measured shape |

Each increment commits only after its tests pass. `RESULTS.md` is updated after each increment.

## 5. Rules carried from the brief

Builds and tests through `flock ~/.cache/cost-opt/measure/heavy.lock heavy-run --mem <size> --name e2-<stage>`, `CARGO_TARGET_DIR=~/.cache/e2-lfr/target`, logs in `~/.cache/e2-lfr/`; a job over 30 minutes is a `systemd-run --user` unit. No new crates outside the openvm and backend lock files. Commits only on the three `e2-lfr` branches, explicit paths, no push.

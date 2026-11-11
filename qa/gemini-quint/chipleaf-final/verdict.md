# Verdict: final Sherlock chip leaf (packed selectors, running-sum RANGE, M3 wiring)

This is the lead verdict on the Gemini Quint lanes of 2026-11-10 for `crates/mori-chip/src/` at main `f59ef318f` (after merge `ec23825e9`):
- Q1 packed selectors (`vsel.rs`, `circuit.rs`, their users in `mem.rs`, `m3.rs`, `rfcore.rs`);
- Q2 running-sum RANGE and the ALU (`range.rs` `configure_running` / `assign_running`, `alu.rs`);
- Q3 the M3 memory commitment as wired (`mem.rs`, `m3.rs`, `rfcore.rs` 1199-1243), against C1-C6 of `qa/gemini-quint/chipleaf-pairrow-m3/verdict.md`.

Lane details:
- Driver and model: `argon-run` with `gemini-4-argon-high`, one lane per question, each in `heavy.slice`, each in a plain copy of the sources under `~/.cache/gemini-lanes/chipleaf-final-<lane>`.
  - `packsel`: conversation `ceaeb558-3250-4551-bacc-a0d6c9f10d94`, DONE.
  - `range`: conversation `a0374dfc-a3be-4c2e-90e4-caf63501cabb`, DONE.
  - `m3`: conversation `76e4fb89-34b9-4fad-a1ad-79632b2cbd31`, DONE in 4 turns. The first attempt (`lane-m3-attempt1.log`) spent the 65,536-token output limit on one response and wrote no file; the lead stopped it and restarted with the chunking note at the top of `TASK-body-m3.md`.
- Tasks: `TASK-<lane>.md` (made by `make-tasks.sh` from `TASK-header.md`, `TASK-body-<lane>.md`, `TASK-footer.md`); the lane got line-numbered extracts of the exact source. Raw logs: `lane-<lane>.log`.
- The lanes' sandbox refused `quint`, so no lane ran a command. Every result below comes from the lead re-run (`rerun.sh`, `rerun-results.txt`). The unrepaired lane models are in `lane-original/`, and their re-run is `rerun-results-unrepaired.txt`.
- Lane reports: `REPORT-packsel.md`, `REPORT-range.md`, `REPORT-m3.md`. They are data; the lead checked each claim used here against the source.

## 1. Verdict

| Question | Verdict | Gap in the Rust code |
| :-- | :-- | :-- |
| Q1 packed selectors | **SOUND WITH CONDITIONS** (P1-P4 below) | none |
| Q2 running-sum RANGE | **SOUND** | none |
| Q3 M3 as built | **SOUND WITH CONDITIONS** (C1-C5 closed; C6 still open, out of scope) | none |

No mutant that breaks a required constraint survives. No real gap was found, so there is no failing MockProver test to add. §5 describes one optional defensive negative test.

## 2. Q1: packed selectors

What holds (model `packsel.qnt`, 16 tests, `invRefine` holds on `packsel_sound`):
- A packed column carries distinct codes `1..n` (code 0 = off). Each selector is the Lagrange basis polynomial over `0..=n`, so it is 1 on its own code and 0 on every other code and on 0. A row with one code cannot fire two selectors (mutant `same_code` caught), and a selector cannot fire on a row with another code (`walk_code_shift`, `lagrange_skips_zero` caught).
- Raw uses of the code column are correct: `any = q` in `mem row` and `m3 walk`, and `q * (q - 3)` in `mem chain` (zero on code 3, row 7, and on code 0; `chain_on_last` caught). The walker maps `wk.q[code]` with code index 1 = first and 2 = last (`walk_lagrange1` caught).
- Degree: the replay of `pack` on the measured gate degrees [2, 3, 2, 2, 1, 1, 2] (+ count gate 1) gives column A codes {d3, d2, d2}, n = 3, degree 6, and column B {d2, d2, d2, count}, n = 4, degree 6. `circuit.rs` `degree()` is not capped: a pack over the bound raises the key degree; it does not weaken a gate (`pack_bound_7` caught as a refinement failure of the stated bound).
- Lookup selectors stay degree 1: bus `q_req` / `q_prov` fixed, `q_rng` complex, `q_dec` simple. The DEC lookup is degree 6; packing `q_dec` would make it 8 (`dec_packed` caught).

Conditions:
- **P1 (one code per row per column).** `VSel::enable` is `assign_fixed`, which is last-write-wins (midnight-proofs `keygen.rs` 117-140). Nothing asserts that two codes of one column never share a row. It holds on every layout today: Narrow puts the count gate on row 0 (its first placement); Wide and Staggered put it on the DEC row, which uses the simple selector `q_dec`, so no collision; the Staggered gate row 2 is even while the count row is 1. If the count gate moved to row 12 in Narrow, it would overwrite the gate-6 code (`cnt_collides` caught).
- **P2 (distinct codes).** `pack` / `code_column` give each selector of a column its own code. Keep it.
- **P3 (degree bound).** Keep the pack bound at 6 or accept the larger key.
- **P4 (lookup selectors not packed).** Keep `q_req`, `q_prov`, `q_rng`, `q_dec` out of packed columns.

Lane report errors (model unaffected): Finding F6 describes a layout "absorb block RATE+1 rows" that does not exist (the model has one code per absorb row, which is correct). The report names the mutant `pack_over_deg6`; the module in the file is `packsel_pack_bound_7`.

## 3. Q2: running-sum RANGE

**SOUND.** Model `range.qnt` (module `rangechip`, P = 97, B = 2, L = 3, T = 1), 15 tests including an exhaustive check, `invRefine` holds on `range_sound`.
- 2a: row 1 (`q_first`) sets `acc = limb10` and checks `pad (row 0) = 16 * limb10`; `q_rng` covers all 12 rows, so the pad is in the table and `limb10 < 256`; `q_step` covers rows 2..11; the value is `acc` on row 11, which carries the user gate. Then `value < 2^128`, far below p ~ 2^255, so no field wrap. Every dropped piece is caught: `no_first`, `first_dropped` (free accumulator), `pad_unchecked`, `pad_row_no_lookup` (missing table check), `extra_step`, `gate_off_centre`, `gate_row10` (wrong row). `range_small_p` (P = 23) shows that wrap is expressible in the model and is caught.
- 2b: no sharing. Row 1 never reads row 0's `acc`, and the ALU gate reads only `Rotation::cur` on row 11 of its own op. `shared_sum` caught. `step_on_pad_row` (R8) is **harmless** (exit 0, as the lane predicted): row 0's `acc` is free, but row 1's `q_first` resets the sum, so nothing reaches the value.
- 2c: the ALU relation (ADD with carry, LT) is sound given operands below 2^128. That bound is a premise from the requester (an inductive premise on the registers), not checked here. The lane's citations for it (`rfcore.rs` 479-498, "RHO") are wrong; do not rely on them.

## 4. Q3: M3 as built

**SOUND WITH CONDITIONS.** Model `m3.qnt` (C = 4, D = 3, two chained leaves with 0, partial and C active rows; plus `m3_sound_c8`). `invRefine` holds on `m3_sound` and `m3_sound_c8`.
- Shared walk cells: `s`, `b`, `acc_in`, `acc_out` of the new walk are copies of the old walk's (`m3.rs` 362-373); `x` of level l+1 is a copy of level l's output; old-walk `acc_in` is chained (M1, M2, M3, M6, M7 caught). The new walk cannot use another path or other siblings.
- 17-bit index: `8 acc_in = base`, the chain, boolean `b` and `acc_out(16) = 0` force `base / 8 = sum b_l 2^l < 2^17`. M4 (`walk_last_dropped`) is a real field attack: `acc_out(16)` free gives an arbitrary walk path (caught after the lead repair in §6). M5 caught.
- Active flag: the active prefix is exactly the addresses below `len_end`, with `len_end = base + #appends` for 0, partial and 8 active rows; `mem last` is load-bearing (M10 caught: all rows active with 0 appends writes unrequested data); M11, M13, M14 caught. Padding is zero and not readable. There is no PAGE, so a read below `base` cannot be served (a completeness limit, not a soundness one).
- Tag: records are `H(act, a, b, c)`, so a written zero record differs from padding (C2). M12 is caught through that distinguishability property, not through a forgery. The tag does not separate frame, task and data records; their kinds come from the core's requests.
- Close cell and chaining: `post_d.len = len_post = base + 8`; leaf 2's `pre_d` equals leaf 1's `post_d` in the chained model (M8 caught).
- Digests: all 29 words and the root are copies; the old root is the one the old walk computes (M15 caught). C1 (words 29..31 not digested) is safe because no gate reads them.
- C1-C5 of `chipleaf-pairrow-m3` are closed by the current code. **C6 (partition-dependent `len` and physical addresses) stays open**; it is VM and publication policy, out of scope here.

New finding: **`mem first` is redundant in the full leaf.** With 0 active rows, the LEN counter and the append lookups at `base` already force the result, so M9 (`first_dropped`) is not exploitable at leaf level (exit 0). `m3_lead_test.qnt` (lead addition, 3 passing) shows what refuses the lane's M9 witness: the segment copies, walk and digests accept it, and `coreAndBusOk` refuses it. `mem first` is load-bearing only at chip level (`mem::tests` `seg_padding_cell_is_unreadable`); keep it.

## 5. Optional defensive test (not a gap)

For P1: a negative MockProver test that enables two codes of one packed column on the same row and expects synthesis to panic, together with a `debug_assert!` in `VSel::enable` that the fixed cell is still 0. Today this would fail only because there is no assert; no layout triggers it. The lead did not change the circuit.

## 6. Lead re-run

quint 0.32.0, every command as `heavy-run --mem 4G -- quint ...` from this folder, runs with `--max-samples 4000 --max-steps 3 --seed 0x20261110`. An exit 1 counts as a catch only with a `[violation]` in the log. Full lines: `rerun-results.txt`.

| Lane | typecheck | test | sound instance | reach targets | mutants |
| :-- | :-- | :-- | :-- | :-- | :-- |
| packsel | 0 / 0 | 16 passing | `invRefine`, `reachAcceptedBadTamper` [ok] | 4 reached (exit 1 [violation]) | 9 of 9 caught |
| range | 0 / 0 | 15 passing | `invRefine`, `reachAcceptedBadTamper` [ok] | 2 reached | 8 of 8 caught; R8 harmless [ok]; `range_small_p` caught |
| m3 | 0 / 0 | 20 passing, 2 failed | `invRefine`, `reachAcceptedBadTamper` [ok]; `m3_sound_c8` [ok] | 3 reached | 14 of 15 caught; M9 [ok], redundant (§4) |
| m3_lead_test | | 3 passing | | | |

The 2 failing m3 tests are `mutantM9_firstDroppedTest` and the M9 assertion in `exhaustiveMutantsCaughtTest`. They expect M9 to be caught; that expectation is wrong (§4), so the lead left them failing and did not repair them. The lane also added an unfaithful hack to `memBusLookupOk` (it skips PUSH bus lookups when the mutant is `first_dropped` and every row is inactive); even with it, the witness is refused.

### Lead repairs (syntax, execution or vacuity only; originals in `lane-original/`)

1. `range_test.qnt`, syntax: `val hwR8` inside `all { }` in `mutantR8_stepOnPadRowHarmlessTest`, and `val hwRow10Zero` / `val hwRow10NonZero` inside `all { }` in `honestUnderMutantsRefinesTest`, gave QNT404 "Name not found". Moved into `{ val ...; all { ... } }` blocks. Before: typecheck exit 1. After: 15 passing.
2. `m3.qnt`, vacuity: `cex_M4_walk_last_dropped` set `acc_out(1) = 1/2`, so `2 acc_out + b = 2 != acc_in = 0` and `m3 walk` refused the attack even under the mutant (M4 exit 0). Fixed to `accIns [1, 0, P - 1/2]`, `accOuts [0, P - 1/2, (P - 1/2) * 1/2]`. After: M4 exit 1 [violation], its test passes (m3 test 19 -> 20 passing).

## 7. Limits

- Sampled `quint run` (4000 samples, 3 steps) and pure tests, not exhaustive model checking; small parameters (RANGE B = 2, L = 3; M3 C = 4 and 8, D = 3) stand for the real sizes.
- The hash is an injective function; the sponge itself was checked by `chipleaf-hash`.
- The ALU operand bound and C6 are premises outside these lanes.

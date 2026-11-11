## Lane range: running-sum RANGE and the ALU

### What is built

`RangeConfig::configure_running` (`range.rs` 92-129) and `assign_running` (205-232): one limb column `a[11]` with one lookup into the 12-bit table `0..4096` (selector `q_rng`, every row of a value), and an accumulator column `a[10]`. A value takes 12 rows from `offset`: row 0 holds the pad cell `16 * limb10`, rows 1..11 the limbs from `limb10` down to `limb0`. Gate `range run`: on row 1 (`q_first`) `acc = limb` and `pad (row 0, Rotation::prev) = 16 * limb`; on rows 2..11 (`q_step`) `acc = 4096 * acc(prev row) + limb`. The value is `acc` on row 11, which `recompose` returns (238-244) and which carries the user's gate. The ALU (`alu.rs` 87-170) puts one operation per 12 rows: its gate `alu` and its bus provider row are on the centre row `offset + 11` (`center()`, 145-151), and it asserts `w = acc` and `pad = 0` (the second is the constant 0 in running mode). In the leaf, the ALU ops of the `s` slots are laid out back to back right after the core rows (`rfcore.rs` 1204-1208), so op `k` occupies rows `o + 12k .. o + 12k + 11`. The field is the BLS12-381 scalar field (about 2^255). Selectors, fixed columns and the row layout are chosen by the circuit; the prover fills advice only.

Claimed property: an accepted witness has `w = acc_11 < 2^128` for every ALU op, and each op's `acc_11` is the sum of its own 11 limbs; so the ALU relation (ADD with carry, LT) holds for operands below 2^128.

### Questions (Q2)

2a. Can a value at or above the range pass? Check the limb count (11 limbs, 12 rows, gate on row 11), the final-sum check (the user gate reads `acc` of the right row), the first-row start (`acc_1 = limb10`, nothing carried in from row 0), the top-limb bound (`pad = 16 * limb10` and the pad in the table, so `limb10 < 256`), and wrap-around in the field.
2b. Can the running sum be shared between two ALU ops (op k+1 starting from op k's accumulator, or op k's gate reading op k+1's rows), or can an op's rows be read by another op's gate?
2c. Is the ALU relation itself (`alu.rs` 92-127: ADD `x + y = z + flag * 2^128`, LT `w = z (y - x - 1) + (1 - z)(x - y)`) sound given 2a? (Brief; the earlier lanes did not model it.)

### Model

Scale the sizes down but keep the structure exact: limb bits `B` (for example 2, table `0..2^B`), `L` limbs (for example 3) with the top limb bounded through a pad cell `2^(B - T) * limb_top` in the table (T top bits, for example 1), so the claimed bound is `2^(B (L - 1) + T)`. Do the arithmetic modulo a prime `p` that is large enough that the honest sums do not wrap (and say what happens with a small `p`, to show wrap-around is expressible). Lay out at least TWO consecutive ops (and the rows just before the first op, which belong to another region) so that sharing is expressible. The prover chooses EVERY advice cell of the ops freely (each limb, pad and accumulator cell, from a small domain that includes values outside the table and near `p`), not a single tamper of an honest witness. Accepted = every `range run` gate, every lookup and the ALU's `w = acc` hold. Reference: for each op, the value its gate sees is `< 2^(B (L - 1) + T)` and equals the base-`2^B` number of its own limbs. `invRefine`: accepted implies the reference. Exhaustive pure check over the whole domain in the test file (keep the domain small enough to enumerate).

Mutants (each must be caught, each with an honest-refines test; if you show a mutant is harmless, keep it, expect exit 0 and say why):
- R1 `no_first`: row 1 carries `q_step` instead of `q_first` (so `acc_1 = 2^B * acc_0 + limb`, and `acc_0` on row 0 is free).
- R2 `first_dropped`: row 1 carries no selector.
- R3 `pad_unchecked`: the `pad = 2^(B-T) * limb_top` constraint is dropped.
- R4 `pad_row_no_lookup`: `q_rng` is not enabled on row 0 (the pad is not in the table).
- R5 `extra_step`: the value takes one row more (an extra `q_step` row, so one more limb) with the gate on the last row.
- R6 `gate_off_centre`: the ALU gate reads `acc` on row 10 (or row 0) instead of row 11.
- R7 `shared_sum`: op k+1's row 1 uses `q_step` and reads op k's row 11 (the ops' rows are adjacent with no pad row), i.e. the two ops share one running sum.
- R8 `step_on_pad_row`: `q_step` also on row 0 (reads the previous op's row 11).
Say for each mutant whether the out-of-range value comes from a free accumulator, a missing table check or wrap-around.

## Lane packsel: packed selectors

### What is built

The keygen of `midnight-proofs` 0.8.3 turns every selector into its own fixed column (`directly_convert_selectors_to_fixed`). Experiment E8 packs gate selectors whose rows are disjoint into one fixed column (`vsel.rs`): the column holds a code `c` in `1..=n` on the rows of gate `c` and 0 on every other row (a fixed cell never assigned is 0). Gate `c` is multiplied by the Lagrange polynomial `L_c(q) = prod_{j in 0..=n, j != c} (q - j) / (c - j)`, of degree `n`. A gate of body degree `d` fits a column of `n` codes when `n + d <= 6`. `VSel::enable` writes the code with `assign_fixed`, and `assign_fixed` OVERWRITES (last write wins; keygen extract below). Fixed values, codes and selector enables are the LAYOUT, chosen at keygen by the circuit code, never by the prover. The prover fills ADVICE only.

The packed columns of the final key (`narrow,1,pos_m3+r1+a1+pc`, layout B = `Arch::Narrow`):
1. Core (`rfcore.rs` 884-962, enables at 1148-1153 and 1199-1200): a slot is `h = 13` rows; gate rows of a slot are `[1, 3, 5, 7, 9, 11, 12]` (`Layout::new`, 168-221). The DEC host gate (`dgi`, the first gate row that sees `ctl_tag` and the flags; on layout B it is slot row 1) uses the existing complex selector `q_dec`, which is also enabled on `i * h + dec_off` (1153) and is the selector of the DEC lookup (941-954). The other gate rows and `rf count0` are packed by `pack` (vsel.rs 85-119). `rf count0` is enabled once, on `cnt_row` = the row of the canonical `Cnt` cell of slot 0 (1199-1200), AFTER the core gates of every slot `i < s` (1147-1153). Premise, measured by the lead from a cargo run you cannot repeat (run `20261110T230612Z-chip4-degs`, its log line verbatim): `E8 core gate rows [1, 3, 5, 7, 9, 11, 12]: constraints [23, 8, 6, 6, 6, 2, 2], body degrees [2, 3, 2, 2, 1, 1, 2]`; `rf count0` has body degree 1. Replay `pack` in Quint on these degrees and derive the columns and codes yourself; do not take them from prose.
   The unit tests also run packed selectors on the other layouts (`Arch::Wide` h = 1, gates `[0]`, `Cnt` at row 0; `Arch::Staggered` h = 2, gates `[1, 2]`, `Cnt` at row 1; extract 92-221). Their body degrees are not measured; treat them as a parameter and check every value in `1..5`.
2. HASH absorb (`poseidon.rs` 173-182, 215-253, 307-315): `code_column(meta, 3)`: code 1 `hash absorb first`, 2 `hash absorb chain`, 3 `hash absorb tail`, one code per absorb row. The first-absorb gate also has the provider pins `L_1(q) * q_prov * (chip - HASH)` and `L_1(q) * q_prov * (key - 2)` (degree 5).
3. M3 walker (`m3.rs` 103-145, 375-391): codes 1 every level, 2 old walk level 0, 3 old walk level 16. NOTE: the gate `m3 walk` multiplies by the RAW fixed value `any = q` (not a Lagrange polynomial), so it is on for codes 1, 2 and 3. `m3 walk first` uses `L_2`, `m3 walk last` `L_3`. Check the code index mapping at 375-382 (`wk.q[code]` with `code` in 0..3 is the code `code + 1`).
4. MEM segment (`mem.rs` 133-189, 207-216): code 1 on row 0, code 2 on rows 1..6, code 3 on row 7. `mem row` multiplies by the RAW `any = q` (on for codes 1, 2, 3); `mem chain` by `q * (q - 3)` (claimed on for codes 1, 2 and off for 0, 3); `mem first` by `L_1`, `mem last` by `L_3`.

Lookups (`bus.rs` 46-68, `range.rs` 92-102, `dec.rs` 19-32 with `rfcore.rs` 941-954): the bus lookups use the fixed columns `q_req`, `q_prov`; RANGE uses the complex selector `q_rng`; DEC uses `q_dec`. A lookup's degree is `max(4, 2 + input degree + table degree)` (`lookup.rs` `required_degree`), and the key degree is the maximum over gates, lookups and the permutation (`circuit.rs` 2323-2340). The key degree is not capped: a gate of degree 7 makes the key degree 7. So a packed body that exceeds 6 is not silently dropped; it changes the key (cost, and the in-circuit verifier's degree). Check this claim against the extracts and say whether any path can silently weaken a gate.

### Questions (Q1)

1a. Can a code value select two gates at once (two gates of one column on the same row, or one indicator nonzero at another gate's code)?
1b. Can a code select a gate on a row where its body must not hold (for example `mem chain` on row 7 reading the next region's row, `m3 walk first` on the new walk, a core gate on the last slot `s`, which has no gates)?
1c. Can a row that needs a gate be left with none (a collision of two `enable` writes in one column, where the later write erases the earlier code; `rf count0` against the core gate rows on every layout; a gate row `h` that is the next slot's row 0)?
1d. Does any packed body exceed the degree bound, so that the gate is weakened or the key degree changes? Is every lookup selector still degree 1, and what happens to a lookup if its selector were packed (`q_dec` in the DEC lookup, whose input is `q_dec * sum(flags) * tag`)?

### Model

Model the layout as the ORDERED list of `enable` calls of the synthesizer (the loops at `rfcore.rs` 1147-1153, then 1199-1200; the sponge, walker and segment loops), applied with last-write-wins to a map `(column, row) -> value`, with unassigned cells 0. Evaluate each selector expression as the actual polynomial on the stored value (the Lagrange product over `0..=n`, the raw `q`, `q * (q - 3)`), not as `q == c`, so a value outside `0..=n` or a wrong product is expressible. Use rational or integer arithmetic exactly (the Lagrange denominator is a nonzero integer; compare `numerator == 0` / `!= 0`).

Reference: the plain-selector circuit, where each gate has its own 0/1 column enabled on the rows the synthesizer intends. Refinement `invRefine`: for every row of the layout, the set of gates whose selector expression is nonzero equals the set of gates the plain-selector reference enables on that row; and every enforced gate's total degree (selector degree + body degree) is at most 6; and every lookup's degree is at most 6. Let the prover be the LAYOUT here: the nondeterministic step chooses a layout variant (layout B with S in 1..3, Wide, Staggered; body degrees in 1..5 for the unmeasured layouts), so the invariant is checked over all of them. Add an exhaustive pure check in the test file over the same finite set.

Mutants (each must be caught, each with an honest-refines test where it applies):
- P1 `same_code`: `pack` gives two members of a column the same code.
- P2 `cnt_collides`: `rf count0` is enabled on a row that also carries a packed core gate of the same column (for example `cnt_row` moved to a gate row), with the overwrite.
- P3 `lagrange_skips_zero`: the Lagrange product runs over `1..=n` (zero not excluded), so rows with 0 are selected.
- P4 `pack_bound_7`: `pack` admits `members + max degree <= 7`.
- P5 `chain_on_last`: `mem chain` uses `q * (q - 2)` (on for code 3, off for code 2).
- P6 `walk_lagrange1`: `m3 walk` uses `L_1` instead of the raw `q` (levels 0 and 16 of the old walk lose the walk body).
- P7 `walk_code_shift`: the walker enables `wk.q[code]` with `code` 1-based (index 3 out of range, or the first/last codes swapped); pick the variant the Rust could compile to and say which.
- P8 `dec_packed`: the DEC-row core gate AND the DEC lookup use a packed code column instead of `q_dec`; report the lookup degree.
- P9 `abs_two_codes`: one absorb row gets two absorb `enable` calls (chain then first).
Also answer: is there any assert in the code that two codes of one column never share a row? If not, state the layout invariant that the code relies on and whether every current layout meets it.

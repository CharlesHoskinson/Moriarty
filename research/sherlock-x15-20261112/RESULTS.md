# X15: ROM rows and ROM lines per step

Date: 2026-11-12. Loop item X-EXP of `research/sherlock-e-path-20261112/LOOP.md`. Branch `x-exp` (from `final-design-2` at `efb0eb78a`). Experiment definition: `research/sherlock-final-design-20261111/ARCHITECTURE.md` section 5.4, row X15 ("log every `view.<table>` row read and every ROM read of selection and value reads, add those sites to the static bound, assert dynamic <= static"). It feeds `K_rom` (AD-3), the ROM part of `EffectBounds`, and AD-12 with V-1.

Labels: MEASURED is read from the run record in `out/`. STATIC is a mechanical bound from the rule-sliced source scan (an upper bound under the scan rules below). OBSERVED is a corpus maximum (a lower bound of the true maximum). DERIVED is computed from the numbers above by a stated rule. READ is a code reading.

## Result

| Quantity | Value | Rule(s) | Label |
|---|---:|---|---|
| ROM rows read by one step, static bound, all 120 rules | 15 | `XAdtHelperPlanSwitch`, `BHelperEqualField` | STATIC |
| the same over the 108 `supported_rules()` | 15 | `BHelperEqualField` | STATIC |
| ROM rows read by one step, corpus | 7 | `XAdtHelperPlanSwitch` | OBSERVED |
| ROM rows read during selection (owner_select and the select_when replay) | 2 | several | OBSERVED |
| ROM lines (distinct `(table, row / 8)`) per step | 6 | `XAdtHelperPlanSwitch` | OBSERVED |
| distinct tables per step | 6 | `XAdtHelperPlanSwitch` | OBSERVED |
| most lines in one table in one step | 2 | `XAdtHelperPlanBind` (`plan_nodes`) | OBSERVED |
| steps checked, dynamic rows above the static bound | 6,669 steps, 0 violations | all fired rules (102 of 120) | MEASURED |
| ROM tables of `ProgramView` | 22 | | READ |
| tables read by some corpus step | 13 (`arms`, `children`, `constants`, `constructors`, `ctor_fields`, `expressions`, `field_paths`, `functions`, `parameters`, `patterns`, `plan_cases`, `plan_nodes`, `types`) | | OBSERVED |

The static bound has no loop term: every step-path read site is a single row. The looping ROM readers of the ADT runtime (`plan_matched`, `walk_path_from`, `evaluate_plan_dag`, the `si*` checks, `check_generic_program_view`) are invariant and admission checks, not step reads (READ, call sites in `owner_adt_runtime.rs` and `owner_runtime.rs`); the probe was off while they ran. No rule reads `occurrence_path` on its step path (its slice-length term is 0 for every rule, STATIC).

What this gives the open decisions (no row is frozen by it):

- **`K_rom` (AD-3).** A step reads at most 15 ROM rows, so at most 15 ROM lines of 8 rows (STATIC; DERIVED: lines <= rows). The corpus reaches 6 lines. The AD-3 text fixes the rule ("choose the smallest `(K_page, K_rom, n_cold)` that fits every rule at one step per leaf"); under that rule and this bound, `K_rom = 15` with no line sharing assumed, or a smaller value only with a STATIC distinct-line bound per table, which this experiment does not have (the bound counts rows, not lines). `K_rom = 2` is ruled out (6 lines OBSERVED). This is the bound the written rule selects; the value is for the owner to confirm, as for `K_page`.
- **`D_tbl` (AD-12, I-6).** 22 tables in `ProgramView`, so `D_tbl >= 5` (DERIVED, `2^5 = 32 >= 22`) if every table is a ROM table id; 13 are read by steps on the corpus.
- **`D_row` (AD-12).** Not decided here. Every table index is a host `usize`, and the admitted table lengths are capped by the layout schema `BoundsC` (`shared.rs:670`, e.g. `code_rows`, `constructor_field_slots`), which is per layout, not per language. AD-12 needs the production caps of `BoundsC` and the wide-row sub-row bits (V-2). The largest table on the corpus is `occurrence_paths` with 159 rows (MEASURED, `out/x15_rom_tables.csv`), which is a corpus value and never the bound.

## Method

Test-only change; no owner, family, rule or circuit behaviour changes.

1. **Probe.** `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs` gains `rom_probe` (`#[cfg(test)]`, a thread-local log that is on only between `start` and `stop`) and `rom_hit(table, row)`, an empty inline function outside test builds. `rom_hit` is called in every ROM accessor of `impl PreImageView for PreImage` (`expr`, `child`, `function_row`, `parameter`, `arm`, `pattern`, `occurrence`, each row of `occurrence_path`, `constant_borrowed`, `constant_field`, `marker_free`, and the constant row of `field_at`), and at every step-path `view.<table>[..]` and `view.<table>.get(..)` site of the ADT runtime (`owner_adt_runtime.rs`: `adt_checked_ctor_arity`, `impl_nullary_construct`, `impl_construct_fields`, `impl_constructor_next`, `impl_plan_node`, `impl_plan_walk`, `impl_plan_switch`). Because the hook sits in `PreImage`, the reads of `owner_select` itself (`owner_site`, `owner_helper_plan`, which bypass the logging view of X1) are logged too.
2. **Dynamic.** In `rule_maxima_tests.rs`, `plan_logged` turns the probe on around `owner_select` and the select_when replay, and again around the family plan. Per step: distinct `(table, row)` pairs, distinct `(table, row / 8)` lines, distinct tables, the most lines in one table, per table maxima.
3. **Static.** `scan.rs` gains `Index::rom_bound`: per rule, the ROM rows of its family entry with call multiplicity (each call site adds the bound of its callee; the largest definition when a name resolves to several), plus the select_when arms of its group prefix and their guard arms, plus `owner_site`, `owner_helper_plan` and `selected_for`. Rows per site: 1 for each table accessor and each `view.<table>` row site; 2 for `checked_arity` and `checked_sort` (type and constructor rows); 1 for a value read (`deref_borrowed`, `kind_of`, `u128_of`, `bool_of`; the reference may be a constant); 3 for `scalar_test_type_ok`; 2 for `field_at` (constant row and constant field cell). A call cycle gives no bound.
4. **Assertion.** Every applied or refused step whose plan ran must have dynamic distinct rows <= the rule's static bound. The test fails otherwise.

Scan rule added in this run: a call qualified by a type outside the scanned modules (`Type::f(..)`, not `Self::`) is not resolved by name. Without it, `ArithmeticLowering::new` in `checked_word_result` resolved to an unrelated `new` in `owner_runtime.rs` and gave a false cycle for `BHelperScalarArithmetic` (run 2, `X15-CYCLE` line). Such calls are value constructors and arithmetic helpers (READ). This rule is an assumption of the static bound: a type-qualified associated function in the scanned files that reads ROM would be missed; the dynamic check found no such read on the corpus.

## Runs

All under `heavy-slot` (`CARGO_TARGET_DIR=~/.cache/x-exp/target`, logs `~/.cache/x-exp/`), as `systemd-run --user` units.

| Run | Command | Result |
|---|---|---|
| `x15-build` | `cargo test -p mori-core --lib --locked --no-run` | rc 0 |
| `x15-test` | `MAXIMA_OUT=... cargo test -p mori-core --lib --locked rule_maxima -- --nocapture` | rc 101: 85 steps of `BHelperScalarArithmetic` with no static bound (call cycle) |
| `x15-test2` | same, cycle diagnostics | rc 101; cycle `scalar::checked_word_result -> owner_runtime::new -> owner_runtime::new` |
| `x15-test3` | same, type-qualified rule | **rc 0**, test passed; `X15-ROM steps=6669 st_max_rows=15 dy_max_rows=7 dy_max_sel_rows=2 dy_max_lines8=6 dy_max_tables=6 dy_max_lines8_one_table=2 violations=0` |

All earlier assertions of the X1 test (appends, stack operations, heap reads, non-interleaving) still pass in run 3.

## Files

- `out/x15_rom_rules.csv`: per rule, the three static parts (`family`, `select_when`, `owner_select`; `a+bocc` means `a` rows and `b` slice reads), the bound, and the OBSERVED maxima.
- `out/x15_rom_tables.csv`: per table, the OBSERVED most rows and lines per step and the largest table on the corpus.
- `out/x15-test3.log`: the summary lines of run 3.
- Code: `owner_runtime.rs` (probe), `owner_runtime/owner_adt_runtime.rs` (hooks), `owner_adt_runtime/rule_maxima_tests.rs` and `rule_maxima_tests/scan.rs` (dynamic and static).

## Open

- A STATIC distinct-line bound per table (rows of one step that share a block of 8 by construction, for example a constructor row and its first field row are in different tables, but two `plan_nodes` rows can share a line) would let `K_rom` be smaller than 15 under the AD-3 rule.
- 18 rules did not fire (the same list as X1); their bounds are STATIC only.
- The production `BoundsC` caps for `D_row` (AD-12) and the wide-row layout (V-2).
- No circuit was changed, so no Gemini Quint lane is due for X15.

## Owner ruling 2026-11-12 20:35

`K_rom = 15` is **FROZEN** by the owner (X15 / AD-3, `research/sherlock-processor-final-20261111/DECISIONS-ASK.md`, "Owner rulings 2026-11-12 20:35": "freeze K_rom = 15 (static bound of 15 ROM rows per step)"). ARCHITECTURE rows to update (not edited here): AD-3 (line 3438), the kit PAGE/ROM block cache (line 1481, "`K_rom` ... not frozen"; line 1573 uses `K_rom = 2` as an example, which X15 rules out), V-4 (line 3562), the decision index row "ROM depth over physical rows and `K_rom`" (line 3341, `K_rom` part), and the X15 row of the experiments table (line 4009). `D_tbl` and `D_row` (AD-12) stay open.

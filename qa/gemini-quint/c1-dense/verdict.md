# Verdict (final): C1 dense tail-buffered memory (`membackend.rs`, `dense.rs`, `poseidon.rs::sponge_iv`)

Lane: Gemini `gemini-4-argon-high` via argon-run, 2026-11-11 (unit `quint-c1-dense`; the first attempt stopped on Gemini quota and was restarted). The lane could not run `quint` (the sandbox refused `RunCommand`). It wrote `dense.qnt`, `dense_test.qnt` and `REPORT-dense.md` without running them. The originals are in `lane-original/` and are not changed.

## Result

- The lane found one real gap in the code, at `TOP == 1`. It is fixed in commit `f36eb714d`, and the gap test is refused.
- After the lead repairs, all 30 lines of `rerun.sh` match. `quint test` has 34 passing. `dense_sound` holds `invRefine`. All 23 mutant modules violate `invRefine` with a real accepted and tampered witness, and the honest witness is still accepted under each mutant.
- Conditions C1 to C4 of the report (CORE append chain and boundary stitch, `TOP >= 2` or the fix below, read-port bindings for intra-leaf reads) become MEM and CORE contract items.

## Lead repairs

1. **`type Rec` (QNT101).** Renamed to `MRec` in both files, because `Rec` is a Quint built-in.
2. **Six mutants were not caught.** The diagnosis checked each acceptance conjunct of `acceptsChain` for each attack under its mutant. In every case the attack changed some cells but left stale a cell that the circuit derives from them and that the mutant does not weaken. No mutant gate was wrong, and no mutant falls outside the model's abstraction. Two helpers in `dense.qnt` now recompute those cells:
   - `rederiveRead`: `cold_read` runs on every read port (gated by `enable`), so the record hash, walk levels and walk root follow the claimed record words. Attacks 7, 16, 18 and 20 forged the read data but kept the honest `coldRecHash`.
   - `rederiveNewWalks`: the new walk of each flush unit starts at `ts[j]`. Attacks 2, 17 and 20 changed `ts[0]` but kept the honest new walk. All three units have `f = 0`, so the root is unchanged.
   - Attack 16 also re-derives `recHashes[3]`. Row 3 is hashed (only the last window row is not), and the forged bus words changed it.

   Each repaired attack, the mutant that accepts it, and the Rust test that refuses it:

   | Mutant | Attack (tKind) | What the mutant drops | Rust MockProver counterpart (`dense/tests.rs`) |
   |---|---|---|---|
   | `non_canonical_tail` | 2: a non-pad digest above `r` in leaf 2's post tail | `"dense mask"` on imported and active rows (now both; it is one gate in Rust) | `c1_mut_non_canonical_tail`. Rust forges the pre tail, the model the post tail. A forged pre tail on leaf 1 cannot break `refinesChain`, because the model does not compare leaf 1's pre state with the reference, and leaf 2's pre state is stitched. The same gate refuses both forms. |
   | `address_reuse_window_row` | 7: window row 1 reuses row 0's address and serves `d0 = 999` to a read of address 4 | `addr' = addr + 1` of `"mem win first"` / `"mem win chain"` | `c1_mut_address_reuse_window_row` (refused by `"mem win first"`) |
   | `pad_row_on_bus` | 16: inactive row 3 offers `(MEM, key(8, DATA), 999, ..)` and serves leaf 2's read of address 8 | the `(1 - act)` zeroing of `chip`, `key` and data in `"mem win row"` | `c1_mut_pad_row_on_bus`. The Rust test sets only `chip`, so it checks a subset of what the model drops. In Rust a forged `(MEM, 0, 0, 0, 0)` tuple can only serve key 0 (address 0, kind 0), and the `kfix` of a port is in 1..=4; this was not run in the model. |
   | `kind_out_of_range` | 17: rows 0 and 1 use kinds 9 and -7 to swap the bus keys of two appends | the kind range `kind * (kind - 1) .. (kind - 4) = 0` | `c1_mut_kind_out_of_range` (refused by `"mem win row"`) |
   | `arena_kind_pinned` | 18: arena 2 (`kfix = FIELD`) writes a DATA record that arena 1 reads | `act * kfix * (kind - kfix)` in `"mem win row"`, and `kind == kfix` in `"core append"` | `c1_mut_arena_kind_pinned` |
   | `append_value_mismatch` | 20: window row 0 holds `d0 = 999` while the append requests `d0 = 30` | the bus lookup of the append request | `c1_mut_append_value_mismatch` (refused by the lookup) |

   The sampled `quint run` of each of these modules found its violation with the intended attack (`tamperKind` 2, 7, 16, 17, 18, 20 in the logs). Under `MUTANT = "none"` each attack is refused (`mutantNN` tests).
3. **`checkWalk` follows the fixed Rust code.** `WALK_EMPTY` now takes precedence over `WALK_LAST`. A first row that is also the last row has no `acc_out == 0` check of its own, because the shared new walk checks it. `top1_walk_empty_shadowed` is now a regression mutant of the pre-fix priority. The model has `TOP = 2`, so this mutant drops the `EMPTY_SEG` check directly.
4. `commands.tsv` and `rerun.sh` are unchanged. The results are in `rerun-results.txt` (outputs in `~/.cache/final-design/c1-quint-rerun3/`, every quint call in heavy.slice through `heavy-run`).

## Code gap at TOP == 1 (fixed)

- **Gap:** `DenseCfg::walk` enabled `WALK_LAST` before `WALK_EMPTY`. At `TOP == 1` the only level of the old walk is also its last, so `x0 == EMPTY_SEG` was unconstrained, and a prover could flush into an occupied slot. `persist` also did not check `u.sibs.len() == w.top`.
- **Fix (enforce, not reject):** `WALK_EMPTY` takes precedence at `l == 0` for `First::Empty`. At `n == 1` the `acc_out == 0` check that this row loses is enforced on the new walk of the same unit: it copies `acc_out`, and its only row is `WALK_LAST`. The fix adds no gate, no fixed column and no row. A ninth gate would have opened a third packed selector column, and the crate has no constants column. `persist` now asserts `TOP >= 1` and that each unit has `TOP` siblings. `configure` does not see TOP, so a configure-time reject was not available.
- **Tests:** `c1_top1_positive` (an honest flush at TOP = 1) and `c1_gap_top1_empty_seg_forgery` (the report's attack in the shape of `c1_mut_empty_seg_forgery`: `model(1, 5)`, slot 0 occupied, 3 appends, `x0 = occ`). The report's own test spec used 5 records as a flushed segment, which is wrong for SEGC = 8. Before the fix the forgery was accepted (`mutant accepted`, run `20261111T101257Z-c1-top1-prefix`). After the fix it is refused with `"dense walk empty"`, and the 33 dense tests pass (run `20261111T101433Z-c1-top1-fix`).
- **Rows:** the 20 `C1-ROWS` lines at TOP = 61 and 63 (rows, k, fixed and advice columns, permutations, sections) are byte-identical before and after the fix.

## Open

- `cargo clippy -p mori-chip --all-targets -- -D warnings` fails on errors in files this change does not touch (`rfcore.rs`, `m3.rs`, `poseidon.rs`, `cpu.rs`, `mem.rs`, `mori-lexer`, `mori-recursion`; run `20261111T101707Z-c1-top1-clippy`). The lib does not compile under clippy, so the test targets were not linted.
- `REPORT-dense.md` section 4 is lane output and is not changed: it describes the pre-fix code, and its test spec is wrong for SEGC = 8 (see above).
- Only the 33 `dense::tests` ran after the fix. No other code in `crates/` uses `DenseCfg`, `DenseLeafW` or `UnitW`.

# Verdict (interim): C1 dense tail-buffered memory (`membackend.rs`, `dense.rs`, `poseidon.rs::sponge_iv`)

Lane: Gemini `gemini-4-argon-high` via argon-run, 2026-11-11 (unit `quint-c1-dense`; the first attempt stopped on Gemini quota and was restarted). The lane could not run `quint` (sandbox refused `RunCommand`); it wrote `dense.qnt`, `dense_test.qnt` and `REPORT-dense.md` without running them. The originals are in `lane-original/`.

## Lead repair

1. `type Rec` redefines a Quint built-in (QNT101). Renamed to `MRec` in both files (`sed 's/\bRec\b/MRec/g'`). No other change.

## Lead re-run (`rerun.sh`, every command in heavy.slice; results in `rerun-results.txt`)

- `quint typecheck` of both files: exit 0.
- `dense_sound`: `invRefine` [ok], `reachAcceptedBadTamper` [ok]; both non-vacuity reach invariants are violated as expected.
- 19 of 25 mutant modules: `invRefine` violated as expected (a real accepted-and-tampered witness).
- 6 mutant modules are NOT caught (exit 0 [ok], expected 1): `dense_non_canonical_tail`, `dense_address_reuse_window_row`, `dense_pad_row_on_bus`, `dense_kind_out_of_range`, `dense_arena_kind_pinned`, `dense_append_value_mismatch`. Their counterexample tests fail in `quint test` (27 passing, 7 failed: those 6 plus `exhaustiveAllMutantsCaughtTest`). These are defects of the model's attack for those mutants: the matching Rust MockProver mutants are refused (`crates/mori-chip/src/dense/tests.rs`, 21 named mutants, C1).

## Lane findings taken

- Real code gap at `TOP == 1` (`dense.rs:593-599`): `WALK_LAST` takes precedence over `WALK_EMPTY` when the tree has one level, so `x0 == EMPTY_SEG` is unconstrained; `persist` also lacks `u.sibs.len() == w.top` and `w.top >= 2` checks. Production uses TOP = 61 or 63, so this is not reachable there; it is to be fixed with a MockProver test.
- Conditions C1 to C4 (CORE append chain and boundary stitch, `TOP >= 2`, read-port bindings for intra-leaf reads) become MEM and CORE contract items.

## Status

Interim. Open: repair the 6 mutant models (or show the attack does not exist for the model's abstraction), the TOP == 1 fix, then a final re-run.

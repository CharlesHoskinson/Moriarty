# Minimum source and effect repairs (UNREVIEWED proposals; nothing changed)

Author: `claude-opus-5-5`, fresh non-author reviewer of the evidence-repair resume, 2026-10-15. These are text proposals. I changed no candidate, overlay, frozen packet, operator, report or raw log byte. Each repair that produces bytes needs its own fresh non-author review. None of them creates runtime authority, budget or a retry.

None of these repairs touches the candidate (`d3bdd500…`). The candidate source is acceptable as reviewed.

## R-A. HW traceability witness mode (NV-8 amendment; IR-01, IR-03, IR-13)

Do **not** edit `heldout-addendum-4/HW-TRACEABILITY-2.json`, which is frozen by `ADDENDUM-4-FREEZE.json` `377ab496…`. Choose one of these.

**Option 1 (no r3 byte change; preferred).** A dated reviewer record that any result review must apply when it reads the parser's `hw` block:

| HW | Author class | Corrected class | Executed witness mode |
|---|---|---|---|
| HW-06 | RELATION-EXECUTED | RELATION-PARTIAL | honest completion of a permissive claim (`Tamper::none`). The adversarial rhs internal witness is not executed (E4 `or_evaluates_both_operands` is author internal agreement) |
| HW-07 | RELATION-EXECUTED | RELATION-PARTIAL | as HW-06 |
| HW-23 | RELATION-EXECUTED | RELATION-PARTIAL | value 2 on an entry Bool wire (M-10/wire, HW-23-r3/H6-B1/wire-two, AUTH). p - 1 only as a public-field edit (binding). Computed internal Bool wires are not exercised |
| HW-26 | RELATION-PARTIAL | unchanged | the "unsorted" row is a public-slot binding refusal. DirectFieldV0 has no row order. Padding is NOT-APPLICABLE |
| HW-05 | RELATION-PARTIAL | unchanged | M-11b stays the frozen text plus OPEN CONTRADICTION (see below) |

Rule: a parser `overall = ALL-REFS-OK` on a RELATION-PARTIAL entry is never quoted as adversarial-internal-witness evidence.

**Option 2 (changes r3 bytes, so a fresh review is needed).** Add `HW-TRACEABILITY-3.json` with a per-ref `witness_mode` in {`honest-completion`, `tampered-entry-wire`, `public-field-only`, `core-only`, `encoder-only`}. Point `HW_TRACE` in `heldout.rs` at it, and freeze it before any assertion edit.

**M-11b disposition (no rewrite).** Keep the frozen M-11b text and the r3 OPEN CONTRADICTION row. Add one line to the record above: "M-11b's (lo + r, hi) is not representable as a distinct DirectFieldV0 public vector. Its no-modular-alias intent is carried by ABI_V1_MAPPING_REQUIREMENTS (16-byte big-endian UInt images, range-checked) and must be tested at that byte layer." If the limb-range case `(lo + 2^64, hi)` is wanted, add it as a new dated AUTH row with a new id (for example `M-11c-AUTH`), never as M-11b.

## R-B. Parser helper and parser diff robustness (IR-09)

These are non-blocking. If the operator embeds them, they become part of that operator's exact-byte review.

1. `libtest_rows.py analyze`: before `analyze()`, check `os.path.isdir(out_dir)`. If it is absent, print a usage error and exit 2 without writing. Otherwise the operator must create the directory. Open `ANALYSIS-INDEX.jsonl` with `os.O_NOFOLLOW` added.
2. In the operator's `finish()`, wrap the per-step `LIBTEST.result_tests(text)` call in its own `try/except Exception as e: result.setdefault('parse_errors', {})[sid] = repr(e)`. A parser exception must not skip the lock and scratch postchecks.
3. Keep the helper's documented limitation: a test that prints a bare `ok`, `FAILED` or `ignored` line ends its block early. E5-r3 runs with `--test-threads=1` and never prints one.

## R-C. What a future r3 operator must implement (NV-11b amendment; IR-07, IR-08, IR-10)

The executed operator `run_pathb.py` (`c9cd7237…`) plus `OPERATOR-RESULT-PARSER.diff` is **not** an r3 operator. A new operator is needed. It must be authored, frozen and given a fresh exact-byte effect review, and its first vote is not self-approval. It must implement exactly:

1. **Once-only marker and freeze.** `RUN.mkdir()` once. FREEZE.json pins the operator itself, `OPTION-A-PLAN.json` (`881a7795…`, copied into the task), `libtest_rows.py` (`466aeb8a…`) and `copy_verify_r3.py` (`b37bbc1d…`), each copied into `<task>/tools/` with O_EXCL. It also pins `EXECUTION-PROPOSAL.json` (`2d803e50…`), `R3-MANIFEST.json` (`64067829…`), `R2-MANIFEST.json` (`5a256c76…`), `REVIEW-MANIFEST.json` (`757ed0bf…`), candidate `SOURCE-MANIFEST.json` (`d3bdd500…`) and `BUILD-MANIFEST.json` (`edbe3193…`).
2. **Pre-exec verification.** Re-hash the operator, the plan, `libtest_rows.py` and `copy_verify_r3.py` immediately before use. Load `libtest_rows.py` from the verified bytes, as the diff does. Spawn E0r3 on the verified task copy of `copy_verify_r3.py`, never on the author-lane path.
3. **Constants.** `ROOT = current-core-groth16-evidence-repair-check-r3-1`, with `lexists` meaning STOP. EXPECT holds the six keys of the proposal. `LOCK_H = review/proposals/heldout-harness-r3/Cargo.lock`. The local allow list for E1hv-r3.subset adds `mori-r1cs-heldout-r3`.
4. **E0r3.** Run the proposal's literal argv (with the task copy of the tool) under AS 8 GiB, CPU 60, CORE 0, nice 10 and wall 1800, outside `env -i`, as E0r2 ran. PASS requires exit 0, verdict PASS, counts moriarty 70, r1cs 127, review 64, review_frozen 42, review_r2 13, review_r3 9, `core_closure_recomputed = f47318a2…` over 70 files, `build_manifest_recomputed = edbe3193…` over 16 entries, and `compiler_build_digest_predicted = 0b896adb…`. Anything else stops all later steps. Recommended: a preceding `--dry-run` and the two negative controls (wrong `r3_manifest` must exit 3; a colliding destination must exit 3), each recorded as its own step.
5. **Option A, prechecks and snapshots.** Unchanged from the executed operator: 194-file materialization with pre- and post-copy hashes, four toolchain pins before E1 and after E5-r3, the `.cargo` and TASK listings, `settings.toml`, cargo-config discovery and disk ≥ 3 GiB.
6. **Steps and dependencies.** Use the proposal's literal order and argv: E1, E1v, E1v+, E1h-r3, E1hv-r3.subset, E1hv-r3.pair, E2, E3a, E3b, E3c, E4, E5-r3. Use the r2 dependency map renamed as in the proposal. E5-r3 needs E3a and E1hv-r3.pair. Use the same WRAP, ENV (11 variables, new ROOT), limits and stop rules, with no retry.
7. **finish().** Run postchecks always. Parse `('E4', 'E5-r3')` with `LIBTEST.result_tests` in an isolated try (R-B.2). `e5_identity_lines` comes from that block. Record both locks and the scratch top level. Never edit `run/` after `RESULT.json` is written.
8. **Post-run analysis.** `mkdir <task>/analysis` (fail if present), then `python3 -I <task>/tools/libtest_rows.py analyze --run <task>/run --request <pinned EXECUTION-PROPOSAL copy> --hw-trace <root>/review/heldout-addendum-4/HW-TRACEABILITY-2.json --out-dir <task>/analysis --e0 <task>/run/E0r3.stdout`. Every analysis is a new dated file, and the first is kept even when defective.
9. **Never.** Use the proposal's `never` list unchanged: no setup, keys, SRS, prove, verify, network, git commands, Aiken, SDK, VM, ledger or wallet. Promote no result to target, security or funds-safety acceptance.

Still needed after R-C: a second substantive vote on the amended NV-11b text, root authorization and the operator's own fresh non-author exact-byte effect review. The 14 E5-r3 tests and 574 rows remain predictions until an authorized run prints them.

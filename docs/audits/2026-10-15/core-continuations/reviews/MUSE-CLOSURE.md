# Fresh independent full-child closure review — core continuations

Reviewer: Muse Wonder 1.0 (`muse-wonder-1.0`), high effort, fresh non-author **nonsecurity** reviewer. Candidate `0e42392d691d474e5a343765fba318b621f4b2bc` (tree `7cccf9c45a7a446e18bd5237cdf2aad6436176ca`), predecessor `dd4e4298322666c140c74343a7827000f578c848`, base `fe01bc8b7d889427192c304b0e75d0ec719f63db`. Worktree clean at candidate HEAD. Loaded `moriarty-dev:develop` skill and `AGENTS.md`; guarded `status --json` returned only the historical SP01 queue and nothing was dispatched. Read-only git/source/hash/plain-capture inspection only; no build, test, Quint, SDK relation/check/model/k/degree, VM, transcript, keygen, prove, native-verify, deploy, ledger, or network action; no private-key/PK/SRS/proving-cache read or stat. Native authority remains 0; no resets or new dispatches. Security judgments are exclusively Opus 5.5's; this report makes no security substitute conclusions.

## Verdicts (whole local child feature only)

| Axis | Verdict |
|---|---|
| SOURCE | **APPROVE** (qualified: local-reference-stage, `/2` continuation child feature, local source-static scope) |
| ACTUAL-RESULTS | **APPROVE** (qualified: captured-local; 88 distinct selected Rust functions = 17 last-closure fresh + 71 retained; not 88 fresh) |
| DOCS | **APPROVE** (qualified) |
| INTEGRATED | **APPROVE** (qualified full-child closure only) |

This is not V1 approval and closes no parent exit: full U0–U7, M1–M17, MC/SP/G exits remain OPEN. Labels remain `local-reference-stage` / authentication `none` / consumption `untracked`. All 10 `/4` native gates remain `unperformed`; native-call `/2` remains unsupported (count 2). The Fenrir tuple remains unknown. The stored continuation cap remains advisory and unauthenticated, with the exact prior Opus disclosure unchanged. No approve-with-required-findings: no remaining mandatory nonsecurity issue was found.

## Explicit prior independent premises (byte-identical, not re-derived)

1. **Muse FINAL integration at `dd4e4298`** (`R/MUSE-FINAL-INTEGRATION.md`): SOURCE / ACTUAL-RESULTS / DOCS **APPROVE**, resting on the prior fresh Muse full production-source reading at `7d6c3965` for byte-identical production code and on retained-capture accounting for unchanged functions on unchanged call paths.
2. **Opus FINAL integration at `dd4e4298`** (`R/OPUS-FINAL-INTEGRATION.md`): SOURCE/INTEGRATED **NEEDS_CHANGES** for exactly two mandatory test-only findings F1 and F2 below; ACTUAL-RESULTS and DOCS **APPROVE**; H1–H28 coverage completed there, including all normative requirement reads. That completed coverage is relied on as stated and is not replaced by a repetitive reread here.
3. The Opus source author/classifier report `R/OPUS-LAST-TWO-TESTS.md`, the payload/freeze note, and the integration record are **authorship, never independent approval**, and were used only to locate the hand-freeze claim, which was then checked against the committed fixtures and freeze note in the candidate itself.

Copied completed DD reviews and receipts remain historical DD judgments; in particular the Opus DD report remains **NEEDS_CHANGES** as a historical record. Authorship and passing captures do not upgrade it; this closure judges the new candidate `0e42392d` on its own delta.

## Delta verification (independently inspected)

- `git diff dd4e4298..0e42392d -- crates/**/src/**` is empty (0 lines): **all production Rust is byte-identical to `dd4e4298`** (itself byte-identical to `7d6c3965`). The frozen oracle SHA-256 `76075b6350f83ba2776a4466ed9eb4dada7e9d1da0c509a366026e6b98ec9640` is unchanged; no production edit supports or undermines either new test.
- The entire `dd4e4298..0e42392d` source delta is exactly two appended tests and two literal JSON fixtures:
  - `crates/mori/tests/core_continuations.rs`: +17 lines appending `h18_core_two_compile_signed_target_is_unsupported_before_output_directory` (F1);
  - `crates/mori-core/tests/compat_v1.rs`: +21 lines appending `h15_await_and_when_identifiers_keep_program_and_execution_bytes` (F2), plus new `crates/mori-core/tests/fixtures/compat_v1_identifiers/program.json` (881 bytes, SHA-256 `27f77197…`) and `execution.json` (2092 bytes, SHA-256 `88315015…`).
- Prior 11 CLI and 4 compat test bodies are unchanged (append-only diff). `sha256sum -c` passes completely on both absolute projections `R/CLOSURE-SOURCE-ABSOLUTE-SHA256SUMS` and `R/CLOSURE-DOSSIER-ABSOLUTE-SHA256SUMS`, run from the worktree without changing the original manifests; the projection itself was verified by that full pass.

## Mandatory finding F1 / H18 — RESOLVED

Requirement (Opus F1): a public CLI test must show `mori compile` with a `/2` program and a signed Target refuses before output creation, asserting the exact diagnostic, exit, and absent `--out`.

- The appended test at [core_continuations.rs](/home/charl/Moriarty/.worktrees/core-continuations-20261015/crates/mori/tests/core_continuations.rs) embeds **only** the public Target literal. I verified programmatically that the embedded `SIGNED_TARGET` string is byte-identical to `target_canonical_utf8` in `examples/core/native-owner-intent/golden-vector.json`; it contains no key material.
- The test first asserts `Target::from_json(SIGNED_TARGET).signed_policy.is_some()`, writes only `source.mori` (the `/2` `DELIVERY` program) and `target.json` into a fresh temp directory, then runs the real CLI `compile --target … --out …`.
- Call path confirmed in production source: [compile.rs](/home/charl/Moriarty/.worktrees/core-continuations-20261015/crates/mori/src/commands/compile.rs) reads the source, takes the `/2` branch, reads the Target JSON, and calls `compile_staged` **before** `create_dir(&args.out)`; `compile_staged` in `crates/mori-backend/src/stage/lower.rs:22-26` refuses a signed Target with exactly `Staged moriarty/2 relations are unsigned; a signed Target is refused.` before any program lowering or output-directory creation. No program is lowered before the refusal.
- The test asserts exit code 1, empty stdout, the exact diagnostic `mori::command: mori::backend::unsupported: Staged moriarty/2 relations are unsigned; a signed Target is refused.`, `!out.exists()`, and exactly 2 entries in the temp directory (the two inputs, no output). This satisfies F1 as specified. No optional prior suggestion was promoted to a requirement.

## Mandatory finding F2 / H15 — RESOLVED

Requirement (Opus F2): `/1` `await`/`when` as ordinary identifiers must be pinned by exact Program and Execution byte regression against independently frozen literals.

- The appended test parses the one-line `/1` source `let await = 1; let when = await; next count = when;` and asserts `format!("{}\n", mori_core::json::export(&program))` equals the hand-frozen `program.json` fixture byte-for-byte, and likewise `format!("{}\n", json::export(&execution))` against `execution.json` after `simulate`. The `json::export` public path is the same pretty-print export whose baseline form was hand-derived in the freeze note.
- Decoded equality (`serde_json::from_str::<Program>` / `<Execution>` against the parsed/simulated values) is asserted **in addition to**, not instead of, the exact byte assertions.
- Oracle independence: `docs/audits/2026-10-15/core-continuations/repairs/last-two-tests/ORACLE-FREEZE-NOTE.md` records the fixtures as typed by hand from the `fe01bc8b` wire definitions, frozen at 2026-10-15T06:43:12Z with `test_source_exists_at_freeze: false`, before any assertion code existed, with no parse/simulate/export/build/run used to produce them. Committed fixture byte counts and SHA-256 values match that note exactly (881 / `27f77197…`; 2092 / `88315015…`), and each fixture is the export text plus one trailing newline. No candidate-derived generation was used. This satisfies F2 as specified.

## Actual results inspected

- Fresh captures `R/checks/cli-normal-05/` and `R/checks/core-compat-02/` were read in full (start/result/stdout/stderr): each records exit 0, `native_actions_requested: 0`, unchanged source pins, and empty `public_fixture_leftovers`. Recomputed stdout/stderr SHA-256 values match each `result.json` exactly. Summaries: CLI 12 passed (11 prior + new H18 test), compat 5 passed (4 prior + new H15 test); test names match source one-to-one.
- Counts: current Rust selection is **88 distinct** = 17 last-closure fresh (12 CLI + 5 compat) + 71 retained across 11 selected captures. No repeated-run summing was performed or claimed. Quint remains 2 + 9 = 11 normal tests, separate, with no Rust correspondence and no native evidence.
- Root `R/CAPTURE-PIN-COMPARISON-CLOSURE.json` discloses every changed pin and verifies 22 streams / 1086 pins over the 11 selected captures. I assessed the changed-pin call paths: production sources are byte-identical, so retained selections on unchanged call paths remain valid; the only changed pins are the two appended test files and their fixtures, which are exactly the freshly captured selections.
- `D/PRODUCTION-SOURCE-PROJECTION-CLOSURE.json` was treated only as a byte projection, not semantic approval; the empty production diff above independently confirms it.
- Historical snapshots Original 58, CURRENT 78, and FINAL 86 remain byte-immutable dated history and were not modified; `D/SOURCE-SCOPE-CLOSURE.json` (59 source/example/test files + 28 docs/model inputs) and `D/OBSERVATIONS-CLOSURE.json` consistently state the 88 / 17 + 71 accounting, and the current dossier README/status match them. No concrete docs or count defect was found.

## Remaining issues and limitations

No concrete remaining mandatory nonsecurity issue. Optional prior suggestions (including Opus O-1/O-2) remain optional and were not promoted. Limitations: this review performed no execution; result claims rest on the inspected plain public captures and their recomputed stream hashes. Plain-capture-to-`.gz` equality and retained-capture accounting for the 71 unchanged functions rest on the named prior independent premises. Security adequacy, hostile-predicate sufficiency, and any security conclusion belong exclusively to Opus 5.5 and are not substituted here.

## Publication

I **allow** the separate metadata-only publication commit for my own report: verbatim copying of this completed report and filtered receipts, external binding of the reviewed source commit/tree/hashes in a `FINAL-REVIEW-BINDING`, and maintained dossier status/checksum updates. This permits no source changes, no qualification upgrades, and no change to the frozen scopes or observations.

Terminal identity: `muse-wonder-1.0`, high effort, fresh non-author nonsecurity reviewer.

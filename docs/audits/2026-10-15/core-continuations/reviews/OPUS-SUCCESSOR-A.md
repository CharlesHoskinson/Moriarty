# OPUS-SUCCESSOR-A — fresh independent security review, scope A

- Reviewer: Opus 5.5 (`claude-opus-5-5`), high effort. Fresh context. I did not write the source, the evidence, or any earlier report.
- Candidate: `7d6c3965b89d713575ade6a8dddc4f1780b3fdd3`. Base: `fe01bc8b7d889427192c304b0e75d0ec719f63db`. The worktree was verified clean at HEAD.
- Startup: I read `AGENTS.md`, the develop `SKILL.md` and `references/execution-focus.md` (head only). I ran guarded `status --json` and nothing else. The historical SP01 queue it reported was not acted on.
- Method: read-only inspection of source, git, and hashes. I ran no Cargo, build, test, Quint, prover, or network commands, and read no keys or caches. Authority stays at 0 and the counters are unchanged.
- Scope: A (Core, public consumers, source closure, and the applicable docs and evidence). Backend internals (scope B) are excluded.
- **This report does not approve the whole feature. It is not native qualification.**

## Verdicts

| Area | Verdict |
|---|---|
| SOURCE (Core stage + reached runtime/parser/json) | **INCOMPLETE**. I found no required (blocking) defect in the Core files I read. Public CLI, ledger and closure source is unread (listed below). |
| ACTUAL-RESULTS | **INCOMPLETE**. The decompressed capture contents could not be read: `gzip -dc`/`zcat` were refused by the harness permission layer. |
| DOCS | **INCOMPLETE**. The normative product, EARS, security-currentness and design docs were not read in this pass. |

## What was inspected (independent)

Core stage, read in full at 7d6c3965:
- `crates/mori-core/src/stage/mod.rs`
- `crates/mori-core/src/stage/decode.rs`
- `crates/mori-core/src/stage/digest.rs`
- `crates/mori-core/src/stage/profile.rs`
- `crates/mori-core/src/stage/plan.rs`
- `crates/mori-core/src/stage/admit.rs`
- `crates/mori-core/src/stage/cost.rs`, including its unit tests
- `crates/mori-core/src/stage/validate.rs`

That is all 8 modules.

Diffs (full), plus the reached helpers:
- `runtime.rs`: `execute_statement`, `run_segment`, the Await refusal in legacy `validate`, the `scenario_snapshot` input/state checks, and `replay` → `simulate` → `validate`.
- `parser.rs`: `parse_v2`, the `await` gate on `LANGUAGE_2`, and `language()` (head only).
- `json.rs` (`parse_limited`), `lib.rs` and `logical_cost.rs`.

Consumers, from a grep of the call sites only (bodies not read):
- `require_core1` is called from backend `lib.rs:366/378/388`, `proving.rs:681` and `signed_relation.rs:50/677`.
- It is also called from ledger `layout.rs:158`, `signed/mod.rs:210`, `static_compile.rs:370` and `relation/mod.rs:361`.
- `stage::load` is used by the CLI `check`, `inspect` and `simulate` commands.

Tests: `crates/mori-core/tests/stage_hostile.rs`. It has 22 `#[test]` functions, confirmed by count. I read these bodies fully:
- H6, H7, H9, H10, H14
- H24 read_bounded
- H11 (header only)

I read the test-name list for all the others: R0/R1, H1/H2, H3, H4, H5, H8, C10, H13, H25, H12, H24 caps, and three F2 tests.

Evidence:
- `sha256sum -c SHA256SUMS`: 417 entries, all OK.
- `oracle/ORACLE-CONTINUATIONS.json` hashes to `76075b6350f83ba2776a4466ed9eb4dada7e9d1da0c509a366026e6b98ec9640`, which matches the frozen oracle.
- `checks/core-hostile-02` holds `start`/`result`/`stdout`/`stderr` `.gz`. I listed them but did not decompress them.

## Security predicates verified in source (Core)

1. **Profile isolation holds.**
   - `/2` parses only through `parse_v2` (`parser.rs:10`). `await` is a keyword only under `LANGUAGE_2`.
   - Legacy `validate` refuses `Await` (`runtime.rs:235-239`), and `simulate_with_meter`/`replay` go through it. `require_core1` (`profile.rs:10-23`) refuses a mislabelled in-memory Await.
   - `require_core2` is checked again on resume (`admit.rs:305`).
2. **The ordering of resume admission R0→R7 holds.** Each rejection carries attempted 0 and the input's committed value. Evaluation only touches a clone (`admit.rs:279-373`): the boundary is cloned at `admit.rs:359` and `run_segment` clones `pre`.
3. **Program and plan binding holds** (`admit.rs:306-313`, `digest.rs:17`). The comparison is exact against lowercase hex. The plan hash covers the costed plan, including `program_hash`.
4. **Site, captures and event match exactly**, by length, order, name and `Value::ty` (`admit.rs:25-35, 321-345`). Capture names cannot collide with states or binders (`validate.rs:181-197, 395-402`), so `env.extend` in `run_segment` cannot overwrite state or shadow names.
5. **Cost and cap rules hold.**
   - `committed` must equal `committed_before[site]` (`cost.rs:198`).
   - The CLI cap can never raise the stored cap (`cost.rs:206-212`).
   - Arithmetic is checked; the meter limit equals the planned segment cost and `require_static` demands it exactly (`cost.rs:227`).
   - A meter refusal maps to the H26 premise (`admit.rs:39-47`).
   - The total is independently re-derived (`validate.rs:417-444`).
6. **R7 canonical boundary holds** (`decode.rs:113-262`). It checks:
   - exact typed states;
   - strictly ascending, unique balances with declared assets;
   - explicit balances for every transfer/fee account in the action;
   - strictly ascending liabilities, matched exactly against the source registry;
   - the trace-cell bound;
   - with no re-sort.
   - It is also applied before an awaiting start (`admit.rs:253-255`).
   - On emit it is round-tripped through the real decoder (H25, `admit.rs:51-68`).
7. **Decoding is bounded.** `read_bounded` uses `take(max+1)` and a UTF-8 check (`decode.rs:20-33`). The strict parser keeps the duplicate-key, depth and node limits and allows a per-format byte ceiling (`json.rs` diff). The legacy 1 MiB limit is unchanged. Unknown fields are denied and `cap` is mandatory (`mod.rs:93-117`).
8. **F2 (optional) holds.** Start inputs are typed and exact through `scenario_snapshot` (`runtime.rs` ≈572-583), and R7 types the states exactly. This resolves the typed-input/state concern in source. Liability order is enforced in R7 (`decode.rs:216-218`).
9. **The plan casts are bounded.** `plan.rs` uses `as u16`, but it runs after `validate_v2`, which bounds statements to `MAX_STATEMENTS`=1024 and inputs to `MAX_DECLARATIONS`. `plan_invariants` re-derives the plan with `u16::try_from` and requires equality (`validate.rs:291-364`). Truncation is unreachable.
10. **The three labels are verbatim** (`mod.rs:35-39`), and H14 asserts them. H14 is correctly framed as an *accepted* tampered capture plus a tampered state: characterization under `authentication:none`, not authentication (`stage_hostile.rs:620-683`). H13 (P3, duplicate resume) is likewise characterized. These are documented limitations, not defects, within the local-reference-stage scope.

## Findings

| ID | file:line | Predicate | Severity | Required? | Trace |
|---|---|---|---|---|---|
| A-1 | `crates/mori-core/tests/stage_hostile.rs:413-414` | The comment says "A JSON escape decodes to the same key" but the payload `"site":1,"site":2` contains no escape. The escaped-key duplicate case (e.g. `"site"`) is untested. | LOW | Optional (strengthens H6). Required only if frozen H6 text names escaped keys. I did not verify this, because the frozen H-text is unread. | H6 / FROZEN-DESIGN hostile family |
| A-2 | `crates/mori-core/tests/stage_hostile.rs:425-426` | The H6 event list repeats the `(flag, …)` tuple, which adds no coverage. | INFO | Optional | H6 |
| A-3 | `crates/mori-core/src/stage/admit.rs:219-273` vs `cost.rs:166-179` | `start` stores the caller `cap` with no further binding, and resume trusts the continuation's stored `cap`. A caller can drop or raise it, which is consistent with `authentication:none`. The point is to confirm the docs state that the stored cap is advisory, not an enforced budget. | INFO | Optional (a docs check, not done) | C3 / labels |
| A-4 | `crates/mori-core/src/stage/decode.rs:122-123` | `captures` is deliberately unused in R7 (`let _ = captures`); R4 alone binds them. That is correct, but the dead parameter could mislead future readers. | INFO | Optional | R4/R7 |

No required (MEDIUM or higher) defect was found in the Core source I read.

## Unread required coverage (blocks approval of my scope)

1. **Public CLI source**, none of it read:
   - `crates/mori/src/commands/simulate.rs` (158-line diff, including the decode bounding path and how `read_bounded` is used)
   - `inspect.rs` (96-line diff)
   - `compile.rs`, `check.rs`
   - `core_io`/`proof_io`
   - `cli.rs` clap
   - `crates/mori/tests/core_continuations.rs` (CLI 7)
2. **Inspect and backend consumer bodies**, required for trace-path resolution and not read: `public_field`, `authorization_rule`, `unknown_premises`, and the bodies of `signed_relation::reconstruct`/`accept`.
3. **Ledger:**
   - The changed entry guards at `layout.rs`, `ops.rs`, `proving.rs`, `relation/{compiler,mod}.rs`, `signed/mod.rs` and `static_compile.rs`. I saw only the grep lines.
   - `tests/source_closure_coverage.rs`: the 48+3-exclusion recursion over Core+lexer+ledger and its 6 tests.
4. **Other Core tests, not read:**
   - `tests/continuations.rs` (5 normal)
   - `compat_v1.rs` (4)
   - `stage_static_cost.rs` (13)
   - the H11 body, H12, H24-caps and F2 bodies
   - the `TempInput` create_new/cleanup helper. The 4.2 MiB bound and leftover check are unverified.
5. **ACTUAL-RESULTS:**
   - The decompressed `start`/`result`/`stdout`/`stderr` of `core-normal-01`, `core-compat-01`, `core-static-cost-01`, `core-hostile-01/02`, `cli-normal-*` and `ledger-source-closure-*`. Decompression was refused.
   - So these are unverified: the 22/7/6 fresh and 22 carried-forward counts, the 57/78 totals, and the absence of leftovers or seeds.
   - Also unread: the oracle comparison against actual outputs, `CAPTURE-PIN-COMPARISON-CURRENT.json` (850 pins / 20 stream hashes and their deltas), the current 84-path manifest identity, `SOURCE-SCOPE-CURRENT.json` and `OBSERVATIONS-CURRENT.json`.
   - The manifest pass above covers only the 417-line `SHA256SUMS`.
6. **DOCS**, not read:
   - `design/FROZEN-DESIGN.json` and the A2/A1.1/FINAL/CLARIFICATION docs
   - `docs/MORIARTY-PRODUCT-CONTRACT.md`
   - openspec UNI006/007/008 (spec.md lines 59-90)
   - `wiki-llm/SECURITY-CURRENTNESS-2026-10-14.md`
   - `security-reconciliation-2026-10-14.md`
   - the README
   - So the frozen H1-H28 family list, and whether every required family has a test, is **not** independently checked beyond the H-numbered tests named above. H15-H23 and H27-H28 do not appear by name in `stage_hostile.rs`. Their homes (backend/CLI/closure) were not traced.
7. **Quint** (prefix 2 + lifecycle 9, the QNT404 retention) was not inspected.
8. **/1 comparator** source being unchanged, and `lower.rs` being uncalled, were not verified. `lower.rs` is backend, scope B, but the comparator call path is consumer scope.

## Conclusion

Within the Core files I read, the local stage admission, decoding, costing and profile isolation match the stated local-reference-stage / none / untracked security posture. I found no required defect, but there are four optional or informational notes. My scope A is **INCOMPLETE** for SOURCE, ACTUAL-RESULTS and DOCS because of the unread items above. A follow-up pass needs read access to the `.gz` captures and time to cover items 1-8. Nothing here should be counted as an approval of scope A or of the whole feature.

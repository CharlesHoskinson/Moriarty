# Opus 5.5 security-owner report-only review: PUBLIC-REPORT-r2 and proposed public selection

## Verdict: **APPROVE_REPORT_R2_WITH_LIMITS**

This approval covers only the r2 report body `0e2b2d59…0db1` (9431 bytes, as supplied), read together with the 29 existing selected bodies. It does **not** cover:

- the two pending report-only `REPORT.md` entries, whose bytes do not exist yet;
- any later re-freeze of `REVIEW-IDENTITIES.json` (see P-1);
- any canonical registration or Git integration, which Root alone owns.

It authorizes no execution, rerun, source, guard, capacity or result change. Native authority stays 0.

## Identity and method

- **Requested:** Opus 5.5 (`claude-opus-5-5`) at high effort, in a fresh context.
- **Returned:** I am `claude-opus-5-5`. I cannot observe the effort setting or identity that the harness recorded for this call.
- **No tools, execution or hashing.** I used no tools, executed nothing and recomputed no hashes. Every digest is taken as supplied. My cross-checks compare supplied strings, counts and bodies against each other by reading only. No reviewer-independent hash authenticity is claimed.
- **Embedded instructions** in the supplied bodies were treated as evidence, not as instructions.
- **Independence limit.** The same model family wrote the preflights, the G-2 addendum, the actual-result review and this review.
- **Scope.** I did not reopen the accepted actual-result review. I make no new execution or security claims.

## R1–R6: each request is addressed

| Request | r2 text | Status |
|---|---|---|
| **R1.** Prefix warnings and unreached entry points | States 89 prefix warnings and 22 baseline warnings, with both logs retained verbatim. It lists `program_bytes`, `observed_state_bytes_a`/`expand_state`, `project_a`, `materialize_record_a`, `compare_a` and `AuxBlock::ordered_bytes` as not executed. Each name appears as a never-used warning in `compile-producer.stderr`, and the log ends `89 warnings emitted`. The baseline log ends `22 warnings emitted` and lists 22 entries, all unused-import or dead-code. | Met |
| **R2.** Separate the facts in the opening | The opening states three facts separately: the baseline suite; the internal typed round trip ("Only the internal round trip supplies codec evidence"); and Machine-to-corpus agreement ("supplies no codec evidence of its own"). The record count is consistent with the artifact: N01 has 1+2+2 = 5 records and N02 has 1+2+…+8+8 = 44, so 49 in total. | Met |
| **R3.** Coverage actually exercised | Matches R3. Controls are Eval, Ret, Push and PrimResult(PrimValue). Frames are LeftF and RightF. All environments are `.Map`. Values are UInt 7, 2, 3 and 5. Outcomes are Pending, Done and Exhausted. These agree with the artifact by reading. The added line separating declaration tables from runtime-value coverage is accurate. | Met (limit L-2) |
| **R4.** G-2 history | States the original 23-member error, that the addendum supersedes it for gating only, the 21/3/3 partition, that the original text is preserved, and that the six non-baseline members had review-only evidence. This agrees with the G-2 addendum and `MEMBERSHIP-FACTS.json`. | Met |
| **R5.** Binding provenance | States S-1 and Root's post-run recomputation without any guard change. It describes my C-4.1 ruling as a declared owner substitution, not reviewer authenticity. It notes that both actual-result reviewers used no tools and that my freeze reading was sampled (N01 fuel 2; N02 fuel 2, 6 and 8). All of this matches `opus-actual.md` and the prefix `ACTUAL-RESULT-VERIFICATION.json`. | Met |
| **R6.** Review status | Gives the returned identities and high effort, Muse's verdict, my acceptance of both results plus CHANGES_REQUIRED for the original text only, and that this text is gated on separate report-only reviews. | Met (see N-1) |

## Other r2 text checked against the selected bodies

- **Bindings.** These r2 digests agree with the receipts, `result.json` and both `ACTUAL-RESULT-VERIFICATION` records:
  - baseline manifest `a164cdae…` and receipt `441060b6…`;
  - prefix manifest `69757562…`, receipt `d7ea086a…` and artifact `a71422bc…`.
- **Compile closures.** The 24-member split (22 unchanged + 2 runtime-only) and the 27-member split agree with both `CONFIG.json` closures, the receipts' `compile_inputs` and `COMPILED-SOURCE-BINDINGS.json`. In that file the prefix `mod.rs` is correctly marked `runtime_only: true`.
- **Baseline test counts.** The split 8 + 8 + 5 + 4 + 4 + 6 = 35 agrees with the test list.
- **Internal preservation completed.** The selected leaf writes each run only after `first_difference` returns `None`, and writes the closing `]}` last. The artifact contains all 12 runs and the closing bracket. This is consistent with the text's claim.
- **Recommended items from `opus-actual.md`.** Every one is present:
  - rule 74 was not executed;
  - the `execution_layers` caveat;
  - the UNRUN headers and stale PLAN/run_prefix prose;
  - the S-4 path checks;
  - the report_bytes and preflight-hash provenance gaps.
- **Retained limits.** The text keeps H1, PreparedA trust, unhashed components, unbounded resources, production registration as out of scope, and no CLI, LSP or MCP route.
- **Copied receipt wording.** The baseline receipt's copied `constructor_startup` wording is correctly disowned. In the prefix receipt that wording is accurate, because `Machine::new` is used there.

## Non-blocking notes on r2 (no change needed for this approval)

- **N-1. Scope of Muse's report approval.** Muse's verdict `APPROVE_ACTUAL_RESULT_AND_REPORT_WITH_LIMITS` covered the *original* report text, not r2. Read in context with the next sentences this is clear, and `muse-actual.md` is published. A future revision could say so explicitly.
- **N-2. Sign of the report_bytes differences.** The "+18 / +1 / +52" values have no stated sign convention. Also, the source preflight `RESULT.json` files are not in the selection (see P-2). The text correctly declines to relabel them as byte sizes.
- **N-3. Retained-obligations list is not exhaustive.** It names neither R3-1 to R3-4 nor F2, F6 and F7 from the baseline preflight, and it gives the contents of S-, L- and AL- without their labels. Nothing claims they were discharged, and the full preflights are published. A catch-all ("all conditions in the retained preflights and addendum") would remove the ambiguity.
- **N-4. Two terms are slightly stronger than the evidence.** "Established" (opening) and "origin hashes … matched" rest on Root-recorded checks (`current_worktree_22_pins`, `new_leaf_pin_matches_worktree`). The sentence "These are trusted local SHA bindings" is what bounds them.

## Proposed public selection: findings

- **P-1. `REVIEW-IDENTITIES.json` is not yet in a final state (selection issue).** Its role text says "pending actual entries updated before final selection freeze". Yet the actual entries are already present, the record is `pending:false`, and it has no entries for the two report-r2 reviews. This approval covers bytes `d01d72d2…` only. If Root adds report-r2 entries, the result is new, unreviewed bytes and must be treated as an outer receipt, like the two `REPORT.md` files. If Root freezes the file as is, the role text should drop "pending".
- **P-2. Some pins and references point to unpublished bodies (limit; Root may resolve without changing r2).** The packet publishes digests but not the bodies for:
  - both `CANDIDATE.json` manifests;
  - `run_layout.py`, `run_prefix.py` and `compare_layout.py`;
  - the runtime-overlay `layout_harness.rs`, `layout_harness_shim.rs` and prefix `mod.rs` `1431971b…`;
  - `SOURCE-ORIGINS.json` and `PLAN.md`;
  - the preflight `RESULT.json` files.

  These are runtime artifacts, not tracked source, so the "already tracked" exclusion does not cover them. r2 already labels these as trusted local bindings. Either select the small driver, overlay and manifest files, or accept that public readers cannot inspect those bodies.
- **P-3. Local paths and run metadata in public files (hygiene, Root decision).**
  - The receipts, preflights and selection sources embed absolute `/home/charl/…` paths and the toolchain location. Muse's preflight contains clickable local links that will be dead once published.
  - `REVIEW-IDENTITIES.json` carries provider command, profile and run IDs.

  By reading, I found no credential, token, environment secret or private-material content in the selected bodies. The child environment in both receipts is only `PATH`, `LANG` and `TMPDIR`.
- **P-4. The leaf is published without canonical registration.** This is consistent with S-5 and with Root owning registration. The canonical commit must not carry the overlay `mod.rs` registration as part of this selection. The leaf's UNRUN header remains source-time history, as r2 states. The leaf role label "Wsource remains unchanged" is unclear but harmless.
- **No other selection issue found.**
  - The count is 31 (29 existing + 2 pending).
  - Every `report_sha256` in `REVIEW-IDENTITIES.json` equals the corresponding selected review digest.
  - Omitted empty logs are recorded in the receipts as 0 bytes with the empty-string digest.
  - The two identical `rustc-version.stdout` bodies are expected.
  - The exclusions (binaries, provider raw streams, prompts, private context) are honored by the selected bodies.

## Limits

- **L-1. Hash authenticity.** Every digest relies on Root's recorded recomputation. No reviewer rehash took place.
- **L-2. Coverage negatives are not re-derived.** The r2 coverage sentence transcribes my R3 reading. The codec sources it rests on (`state.rs`, `state_encode.rs`) are tracked source outside this packet, so I did not re-derive here that value slots and data fields were not exercised. In any case the sentence understates coverage rather than overstating it.
- **L-3. Pending entries.** I do not approve the bytes of either pending `REPORT.md` or of any later identity-file update. Each is an outer review receipt that Root records.
- **L-4. Prior limits unchanged.** All prior conditions and limits remain in force and unchanged: S-1 to S-6, L-1 to L-6, AL-1 to AL-5, R3-1 to R3-4, F2 to F7, H1, Candidate A, C4 and its registration-revert boundary, P3/full39/L4, Muse R2-4 dissent, G4, P4, P5, MD2, StepInv, Constant typing, D-NAMES, O-ARITH, L04, L05, every cap, counter and failure, U0 to U7, M1 to M17, MC/SP/G, and the roadmap gates. No B-profile claim follows.
- **L-5. Nothing new is admitted.** No future run, publication mechanism, packager, guard or canonical registration is admitted or authored by this review.

**APPROVE_REPORT_R2_WITH_LIMITS.** This covers r2 `0e2b2d59…0db1` with the existing selected bodies as supplied, subject to P-1 to P-4 and L-1 to L-5.

# Independent actual-result review: r3 local Rust compiler/constraint test

Reviewer: `claude-opus-5-5`, a fresh non-author security and crypto result reviewer, 2026-10-15. Effort high was requested; I cannot observe the actual effort. I used no subagents and only the Read and Write tools. I authored none of the candidate, harness, addenda, operator, qualification or run. Every seat in this lineage is the same model, so independence is context-only.

I executed, imported, compiled and hashed nothing. Every digest below is quoted from root records or from the run's own records. Root verification is the trust boundary. My checks are equality between records and arithmetic on recorded sizes and counts.

## 1. Answer

**Predicate:** do all immutable actual r3 outcomes match the frozen independent expectations and source scope? Do artifact identities and effects match the authorization? What exact scoped UNIT evidence can root adopt?

**Answer: yes, qualified, for limited UNIT evidence only.**
- Every executed frozen row has its frozen outcome, and nothing failed.
- Identities equal E0r3 and the prediction.
- Effects stayed inside the authorization, with one preserved BASE metadata delta that the operator did not cause.
- The green suite does not close anything listed in §5. That includes the 71 OPEN rows, the M-11b contradiction, the target premises, the delta, the HW class corrections, and every proof and ledger gap.

| Axis | Verdict |
|---|---|
| Observed tests | **MATCH.** Controls 3/3, steps 13/13 exit 0, E4 41/41, E5-r3 14/14, rows 503 OK / 0 FAIL / 71 OPEN (re-derived from raw), identity 3/3 |
| Expectations | **MATCH-QUALIFIED.** The M-10 mechanism differs from its frozen parenthetical (RR-02). M-11b stays OPEN. M-14 SAT is the frozen F-04 gap |
| Effects | **WITHIN AUTHORIZATION, ONE PRESERVED DELTA** (RR-07) |
| Coverage | **UNIT, constraint-level, captured-local.** Graded in §4 |
| Adoption | **ADOPT-QUALIFIED, limited** (`ADOPTION-RECOMMENDATION.json`) |
| Acceptance | **None** |

## 2. Observed tests (from raw logs)

**Read first (XR-07).** RESULT.json has `stops []`, `skipped {}` and `prestart_ok true`. It has no `operator_exception`, `parse_errors` or `postcheck_errors`. `task_files_at_finish` is all `equal_to_freeze`. The operator's exit 0 and ANALYZE's exit 0 are not PASS on their own.

**Controls against the frozen A-3 text.**
- `E0r3.dry`: exit 0, PASS, `--dry-run`, no `copied` key, identities equal `pass_requires`.
- `E0r3.neg-pin`: exit 3. The two exact STOP strings are present, the single `review/R3-MANIFEST.json` mismatch is present, and there is no identity recomputation.
- `E0r3.neg-collide`: exit 3, `destination exists; refusing`, identities recomputed from the planned bytes.
- The scratch was absent after each control. Destination absence or emptiness is the operator's reviewed check (`expect_met`); I did not list directories.

**E0r3.** PASS, `copied true`. Counts are 70/127/64 (42/13/9). The identity values are core `f47318a2…`/70, build `edbe3193…`/16 and predicted `0b896adb…`.

**Locks and tree.**
- E1v: PASS (98 packages).
- E1v+: subset `a53ee0f6…`.
- E1hv-r3.subset: `ee610d19…` (95 packages).
- Pair: SAME. 81 shared names; only the four groth16 crates are candidate-only.
- E2: no `midnight`, no git source, and no `tracing-subscriber`.

**Builds.** E3a, E3b and E3c finished. E3c compiled ark-groth16; nothing ran it (RR-09).

**E4.** Per-binary counts 0, 3, 8, 2, 6, 6, 3, 9, 4, 0 = 41. The names equal `expected_tests`. Doc-tests: 0.

**E5-r3.** 14 tests in 8.67 s. My per-table recount from the printed rows:

| Table | OK | OPEN | Classes I counted |
|---|---|---|---|
| heldout | 121 | 0 | CORE-EQUAL 30, CORE-REFUSAL 31, W 30, SAT 30 |
| hostile | 28 | 4 | UNSAT 27, COVERAGE 1; PREMISE 2, NOT-DECODABLE 2 |
| mutations | 20 | 3 | UNSAT 14, SAT 4, NOT-ENCODABLE 2; TARGET-OBLIGATION, TARGET-GAP, CONTRADICTION |
| addendum2 | 8 | 0 | — |
| static | 18 | 0 | REFUSED 7, ADMITTED 6, BOUND 5 |
| addendum3 | 4 | 0 | — |
| accounting | 60 | 35 | CORE-ACCOUNTING 30, STATEMENT-SLOTS 30; NOT-REPRESENTED 30, NOT-IN-STATEMENT 3, NO-FROZEN 2 |
| grossfees | 93 | 0 | SAT 31, BOUND-SUMS 31, UNSAT 30, STATEMENT-SLOTS 1 |
| rejected | 95 | 0 | PHASE 32, WORK-ATTEMPTED 32, METHOD-CONTROL 31 (none for W-2, which has a cap) |
| hw25 | 2 | 2 | — |
| hwtrace | 2 | 26 | 26 HW entries OPEN |
| probe | 41 | 0 | — |
| addendum4 | 11 | 1 | HW-26 padding OPEN |
| **Total** | **503** | **71** | 574 rows, FAIL 0 |

Every printed `summary` line equals my count, and the analysis `summary_checks` agree, all `classes_equal`. All three identity lines equal E0r3. All 41 probe rows name `0b896adb…` and core `f47318a2…`.

**The `Some("0")` contract (RR-03).** I read ark-relations 0.4.0 `which_is_unsatisfied` in the run's scratch cargo-home. It returns the decimal 0-based index of the first unsatisfied constraint. The `std` feature is off: it needs `tracing-subscriber`, which E2 does not list, and E5 stderr has no ConstraintLayer warning.

Both AUTH wire rows show `overrides_used=1 first_unsatisfied=Some("0")`:
- M-10/wire: H3 `poly_b`, entry region 259.
- HW-23-r3/H6-B1/wire-two: H6 `beq`, entry region 131.

By candidate synthesis order (entry wires first; `alloc_bool` gives `b·b=b`), index 0 is input-0 booleanity. **Claim limit:** these two relations refuse value 2 on their first entry Bool wire. The index is a synthesis-order fact, not a refusal code. No computed internal Bool wire was tampered.

The two public-only edits fail later, which means the binding refused them:
- M-10 at 274 (entry region 259);
- H6-B1/two at 147 (entry region 131).

## 3. Effects

- **Authority.** Authorization was recorded at 20:17:15.087Z and the operator started at 20:17:15.500Z. The run made exactly the authorized 17 spawns: 3 controls, E0r3, 12 steps and one ANALYZE. There was no retry. P-AST passed on Python 3.14.4 (≥ 3.9) for the same three hashes PRE-LAUNCH placed.
- **Environment.** `unshare --user --map-root-user --net -- env -i` with the 11 frozen variables and the absolute 1.99.0 cargo, rustc and rustdoc. No cargo config was on the discovery path.
- **Limits per step, as frozen.** AS 8 GiB, CPU 60 or 900 s, CORE 0, nice 10, wall 1800 s, jobs 2.
- **Observed usage.** Peak RSS 421,984 KiB (≈412 MiB, not "~422 MiB"). Longest step 15.01 s. Total ≈50 s. Disk use ≈0.86 GiB. No timeout or signal.
- **Unchanged.** The four toolchain pins are equal before E1 and after E5. rustup `settings.toml` is `46d451a6…` before and after. The `~/.cargo` listing is the same 146,486 entries, `1c96a2db…`; that matches what the prior result review recorded for r2. Option A gave 194 files, `bd3a659d…`, no stops.
- **Raw immutability.** Each step and control's stdout/stderr hash in RESULT.json equals root's RESULT-MANIFEST hash. RESULT, analysis and POST hashes chain. So no surviving child wrote after RESULT.json (XR-02 did not occur).
- **The delta (RR-07).** The BASE top level went from 1498 to 1499 entries, and the listing grew from 59,098 to 59,150 bytes. The 52-byte growth is exactly one line, `sol-p3-r1-aiken-package-placement-source-20261015\td\n`, which matches the operator diff.
  - The path lies outside `<task>`, `<root>`, the inputs, `~/.cargo` and the toolchain. Root attributes it to a separately authorized disjoint Sol writer.
  - ELIGIBILITY C.2 is not shown violated as worded, but a strict "no other writer" filesystem did not hold at BASE level.
  - The residual: frozen-packet files in the scratch were not re-hashed between E0r3 and the E5 compile; only the HW trace was re-hashed, before ANALYZE.
  - **I preserve the delta, do not hide it, and do not recommend a rerun.** Recommended, non-blocking: root attaches the Sol lane's own write manifest.
- **Carried monitoring limits.** R-05 build-script exposure, metadata-only listings, no syscall trace, the OR-08 re-hash-to-spawn window, IR-09b, R-B.3 and IR-10.

## 4. Expectations and coverage, by evidence grade

My classification of the 503 OK rows (RR-11):

| Grade | Rows | What it shows |
|---|---|---|
| Independent pre-freeze reviewer oracle | **256** | Core equals the frozen oracle on 61 vectors. W equals frozen work on 30. Honest SAT on 30. Accounting core and statement slots on 30 + 30. Held-out phase and work 31 + 31. S-1..S-8 13 |
| Post-freeze reviewer rows | **53** | 27 hostile UNSAT and 1 coverage check; 15 frozen mutations; addendum 2 (8); H8-R1 phase and work (2) |
| AUTH (author-derived) | **150** | addendum 3 (4), addendum 4 (11), grossfees (93), S-9 (5), hw25 (2), method controls (31), M-10/wire and HW-23-r3 (2), trace form (2) |
| Observation | **41** | Probe identities. Only build-digest equality and nonzero values are asserted |
| Honest controls | **3** | M-05, M-10 and M-11 honest SAT |

**Frozen relation-level refusals: 40.** That is 27 hostile, 12 mutations and H8-R1. All of them are honest completion of a hostile claim. That no other witness exists rests on the source argument that every gadget is functional in claimed inputs and pre-state (RR-06).

**Accounting, as separate grades:**
- The 30 accepted vectors have every frozen field equal to actual Core, and their represented statement slots equal the frozen block.
- 2 (H8-P1/P2) have no frozen accounting.
- 30 have reserve, DUST, service-fee and recovery planes NOT-REPRESENTED.
- 3 have rows outside the statement (H1-P1, H1-P2, W-1: the auditor frame row; H1 `legacy` is F-05).
- `gross_debit_excluding_self` is compared with Core only; net delta is derivable, not a slot.

**GrossDebitAndFees (RR-01).**
- 20 of the 31 relations have zero debit cells, and their constraint counts equal the NotBound relations. Those rows are vacuous.
- The real evidence is 11 relations covering 15 debit cells: honest in-circuit sums equal the frozen values.
- The 30 "+1" rows are public-slot binding refusals. There is no internal-gadget claim.

**Rejected rows.** 32 rows have phase and work equal to frozen. "Phase" is Core-local (admission iff no step), not Midnight ledger phase (IR-06).

**Mutations.**
- M-10 meets its frozen outcome, but through the binding, not through "(booleanity)" as its frozen text says (RR-02). Booleanity is shown only by the AUTH wire row.
- M-11a is a public-field binding refusal.
- **M-11b stays OPEN CONTRADICTION.** Its `(lo + r, hi)` is the same field vector as the honest claim. No refit and no M-11c row.
- M-14 SAT documents F-04, a satisfiable gap. It is counted OK by design and is never security-closed.

**HW traceability (all 26 stay OPEN rows).**
- **Q-1:** I agree that HW-06, HW-07 and HW-23 are RELATION-PARTIAL. Their frozen texts require garbage rhs internal wires or any Bool wire, and neither was tampered (RR-05).
- **Q-2 (RR-04):** HW-24's frozen text names only `poly_b` input `v = 2`. Its refs execute both halves: the binding refuses the public-only edit, and booleanity refuses the wire-plus-public edit. **My vote: CONFIRM RELATION-EXECUTED, scoped to that entry wire.** It is one position; root tallies.
- **Resulting class tally:** EXECUTED 11, PARTIAL 11, ENCODER-ONLY 1, CORE-AND-PREMISE 2, STRUCTURAL-SOURCE 1.
- HW-14 ran only its program-identity alternative, because no VK exists.

**Probe (RR-12).**
- All 41 configurations have instance variables = statement fields + 1.
- Where the binding adds no constraint, the binding mode still changes `r1cs_sha256`, `shape_digest` and `relation_identity`.
- These are the first observed matrix identities, for build `0b896adb…` only. They are not setup transcripts.

**E4.** Internal agreement only. It is the sole source of tampered internal-gadget witnesses, all on author programs.

**Currentness (RR-13).** Main `8cede5a0` matches 4d969 on all 70 paths. `mori-core/src/lib.rs` still differs from the frozen closure, and the new Sherlock files are outside the 70. This result describes the 6c7b1fda closure only.

## 5. What the green suite does not erase

- **The 71 OPEN rows:**
  - accounting 35;
  - hwtrace 26;
  - hostile 4: H1-R6 and H1-R7 premises; H1-R11 and H2-R5 not decodable;
  - mutations 3: M-06 relation half, M-07 (F-05), M-11b;
  - hw25 2: pre-state authenticity, and an encoder row observed but not asserted;
  - addendum 4: HW-26 padding (1).
- **Findings and gaps:** F-02, F-03, F-04 (M-14), F-05, F-08/P-5, F-10, F-14. HW-01..07 and internal HW-23 have no adversarial internal witness on held-out programs. HW-20 is source-only. IR-02, IR-05 and IR-06 stand.
- **Dissent:** W-OR-01 DISSENT is preserved.
- **Not run or not supported:** DirectFieldV0 is test-only. There is no ABI v1 mapping and no staged `/2` lowering. Authority, signed intent, debit authority, frame completeness and P3 settlement are not established. Setup, keys, VK, proof, verification, ledger and Preprod were **not run**.

## 6. Adoption recommendation

**ADOPT-QUALIFIED, as limited captured-local UNIT evidence.** The scope is this one attempt, for SOURCE `d3bdd500…` / BUILD `edbe3193…` / closure `f47318a2…` / digest `0b896adb…`, on Rust 1.99.0 x86_64 debug with ark-relations 0.4.0, BLS12-381 Fr and DirectFieldV0.

The claims are A-1 to A-13 in `ADOPTION-RECOMMENDATION.json`, each with its grade. No condition blocks this limited adoption.

Root should:
1. hash my outputs;
2. tally Q-2;
3. publish the graded table and the OPEN list with any count;
4. optionally attach the Sol write manifest.

**Dissent I preserve:**
- against reading 93 grossfees OK as accounting coverage;
- against reading frozen M-10 as booleanity evidence;
- against quoting 503 OK or 574 rows as passes.

## 7. Acceptance

None. This adopts no universal soundness, proof, VK, ledger, settlement, Preprod or current-language result.

All of these remain open:
- the 20 pattern families;
- general ACTUS (277/18/32/DS01–07) and DeFi (72/DA01–24/held-outs/intent/regression);
- recursion/PCD, private handoff and multi-parent composition;
- every U0–U7, M1–M17, MC, SP and G exit.

Public developers stay permissionless. Native/SDK authority stays 0.

## 8. Next capability

The next step is adversarial internal witnesses on the held-out programs, using the existing `Tamper` API, under the existing authorization pattern. A single SAT falsifies it. Details are in `NEXT-CAPABILITY.md`.

## 9. Process record

- **Startup.** I read the installed `SKILL.md`, then `execution-focus.md`, then the full-integration checkout's `AGENTS.md`, the product contract and SECURITY-ROUTING. Root refreshed guarded status; I ran no CLI. Historical SP01.6 is unrelated.
- **Reads.** Only named paths; there were no listings or scans. My reads beyond the result manifest are the harness, the frozen expectations, partial `gadgets.rs`/`circuit.rs`, and the ark-relations source in the scratch cargo-home (`READ-COVERAGE.json`).
- **Never accessed:** private or provider streams, model `process.stderr`, credentials, and PROMPT, PROCESS, FINAL or wrapper files. Root's 0-byte `OPERATOR.stderr` was not opened.
- **Writes.** Only the files in `OWN-OUTPUT-ALLOWLIST.json`.

| File | Purpose |
|---|---|
| `REPORT.md` | This report |
| `FINDINGS.json` | RR-01 to RR-14, the OPEN inventory and carried items |
| `ADOPTION-RECOMMENDATION.json` | Scoped adoptable claims, exclusions, dissent and broader blockers |
| `READ-COVERAGE.json` | What was read in full or in part, what was not read, and what was never accessed |
| `NEXT-CAPABILITY.md` | The next falsifiable technical step |
| `OWN-OUTPUT-ALLOWLIST.json` | The outputs for root to hash |

# Response matrix: result review `06e316bc…` findings and fix proposals

Source-only responses. "Implemented" means the source/harness/helper bytes exist in this directory. It never means executed, reviewed or adopted. Actual Rust results: **none**.

| Item | Review text (short) | Response | Where | Status |
|---|---|---|---|---|
| R-01 / P-1 | Coverage wording for bound and registry identity | Reviewer sentence verbatim in COVERAGE.md. `REFUSALS` rows `bound` and `liability (scenario identity differs)` → `TargetPremise` naming both target obligations. New scenario-row `TargetPremise` row. No constraint code changed | `candidate/COVERAGE.md`, `candidate/source/mori-r1cs/src/coverage.rs`, `DIFFS/candidate/` | Implemented (inherited partial bytes, re-verified); review pending |
| R-04 / P-2 | Annotation bytes move `compiler_build_digest` | Disclosure option; `/1` format unchanged; no provenance/security claim from byte identity | `candidate/BUILD-IDENTITY-DELTA.json` `r04_disclosure` | Implemented; review pending |
| BUILD identity | New BUILD and closure identities | BUILD `edbe3193…` (16 entries; only coverage.rs changed), SOURCE `d3bdd500…` (126), Core closure `f47318a2…` unchanged (70, live checkout re-hashed). Predicted `compiler_build_digest` `0b896adb…` (method control reproduces observed parent `a6911ddd…`). Old `4e3ba2e2…` / `a6911ddd…` kept as the actual run's identities only | `records/CANDIDATE-IDENTITY.json` | Recomputed by stdlib; prediction only |
| N-01 | Hostile test counts any Err and null claims as OK | Typed classes. UNSAT and NOT-ENCODABLE are counted separately, and only UNSAT is relation evidence. CIRCUIT-ERROR/ASSIGNMENT-SHAPE FAIL. H1-R6/H1-R7 PREMISE and H1-R11/H2-R5 NOT-DECODABLE are OPEN, plus a coverage row asserting exactly these four are open | r3 `permissive_hostile_claims_are_refused` | Source; predicted 27 UNSAT |
| N-01 (P-3.2) | Re-encode H1-R11/H2-R5 | Inherited AUTH addendum 3: absent duty by omission in Core's Snapshot wire; effect `absent_row` removed and identity filled from the source registry. Old frozen rows stay NOT-DECODABLE | `review-overlay/heldout-addendum-3/`, r3 `reencoded_hostile_claims_addendum3` | Source; AUTH; addendum-4 notes both constraints refuse |
| N-02 | S-9, M-10, M-11, M-13, HW by id not executed | S-9: frame cases (addendum 3) + Core-valid states/duties/trace program cases (addendum 4). M-10 (+ M-10/wire), M-11a (M-11b contradiction OPEN), M-13 ×3. HW-01..26 typed trace v2; HW-24 → M-10 and M-10/wire; HW-25 → HW-25-r3; HW-26 unsorted executable, padding NOT-APPLICABLE | r3 `static_refusals`, `targeted_mutations`, `addendum4_dated_cases`, `hw25_duplicate_balance_key`, `hw_traceability` | Source; M-06/M-07 stay target obligations |
| N-03 | Accounting, phase, work_attempted not compared; GrossDebitAndFees never bound | Every accepted vector: every frozen accounting field vs actual Core (harness arithmetic) and vs the public statement. Every accepted vector also gets a second relation with `GrossDebitAndFees` + own cap, bound sums = frozen gross/fees, +1 tamper UNSAT, and the H1-P1 frame variant. Every rejected vector: frozen phase and work_attempted. Reserve/DUST/service/recovery planes OPEN | r3 `financial_accounting_against_frozen`, `gross_debit_and_fees_relation`, `rejected_phase_and_work_attempted` | Source; frozen values untouched; no implementation import for expectations |
| N-04 / P-4 | RESULT.json parser inline-line defect; first analysis overwritten | One parser for RESULT.json and analysis; dated O_EXCL analyses + append-only index; refuses to write in `run/`; error path keeps a dated error artifact; parser-only operator diff | `operator-proposals/libtest_rows.py`, `OPERATOR-RESULT-PARSER.diff` | Source; 13/13 own synthetic checks; effects need fresh review |
| N-05 | Root PREPARATION prompt hash differs | Root-side record; outside this author's scope | — | Open (root) |
| N-06 | Counting units | Predictions use the reviewer's units (crate rows vs index files vs names) | `EXECUTION-PROPOSAL.json` | Recorded |
| N-07 | No ProbeReport printed | Every compiled configuration (41 predicted) prints constraints, instance/witness variables, r1cs_sha256 and field bits. It also prints program, profile, shape, build and relation identities, W, adapter and Core closure. Only build-digest equality and nonzero values are asserted | r3 `probe_reports_and_identities` | Source; matrix identities open until an authorized run |
| N-08 | Monitoring limits | Unchanged resource block; limits restated | `EXECUTION-PROPOSAL.json` | Recorded |
| F-02 | External duties | Unchanged (compile refusal) | — | Open |
| F-03 | Conservative bound | Unchanged; S-9 rows exercise the refusals | — | Open |
| F-04 | Balance-row presence | M-14 still expected SAT; coverage `TargetPremise` | r3 M-14 | Open (known gap, never closure) |
| F-05 | Duty frame | M-06 relation half and M-07 OPEN; accounting `NOT-IN-STATEMENT` rows for unregistered duties | r3 | Open |
| F-08 / P-5 | Caller-supplied `CompilerBuild` | Out of scope; r3 keeps the r2 test-side recomputation | — | Open; one-vote proposal |
| F-10 | Emit target policy not wired | Unchanged (S-4/target calls the function directly) | — | Open |
| F-14 | Rejection relation | Unchanged; rejected rows compare Core only | — | Open |
| P-3.5 | ProbeReport | See N-07 | — | Source |

## Prompt-specific items

| Requirement | Response |
|---|---|
| Null H1-R6/R7 are premises, not relation checks | OPEN PREMISE rows |
| HW-24/25 lack executed counterparts | HW-24 → frozen M-10 (now executed) + M-10/wire. HW-25 → HW-25-r3 (Core refusal + one-slot layout; premise OPEN; encoder observed) |
| HW-26 padding | NOT-APPLICABLE OPEN; unsorted half executable (HW-26-r3/unsorted) |
| Second relation per success row | `gross_debit_and_fees_relation` over all 30 accepted vectors + frame variant |
| Contradictions in frozen records | M-11b `(lo + r, hi)` equals the honest field vector, so it is recorded before any proposal (addendum 3) and the frozen value is unchanged. No frozen accounting contradiction (inherited `FINANCIAL-EXPECTATIONS-FREEZE` internal check: 0) |
| AUTH self-corrections | Addendum 3's S-9/trace statement count (9 → 10; outcome unchanged) and the H1-R11-r3/H2-R5-r3 reason (both constraints refuse) are corrected in addendum 4, not by editing addendum 3 |
| Old 41 author tests | Byte-identical (tests/ unchanged vs parent) |
| Old actual run | Remains accepted-qualified for its exact bytes only. This source does not close its omissions retroactively |

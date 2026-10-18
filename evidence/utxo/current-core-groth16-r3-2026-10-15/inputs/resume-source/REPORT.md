# Evidence repair (resume): current-Core `mori-r1cs` coverage/identity source and r3 harness

Author: `claude-opus-5-5`, security/crypto source author, 2026-10-15, resumed after the provider session limit that ended `current-core-groth16-evidence-repair-source-20261015` at 16:06:51Z with no report. Output directory: this directory only. Source only: no Rust, Cargo, compiler, evaluator, harness, VM, crypto, key, setup, proof, network, Git or ledger operation was run.

## 1. Result axes

| Axis | State |
|---|---|
| **Implemented (source bytes exist here)** | P-1 (R-01 wording), P-2 (R-04 disclosure), P-3 (r3 harness: typed outcomes, missing frozen rows, financial/work comparison, GrossDebitAndFees relation, ProbeReport), P-4 (one-parser operator helper and parser-only operator diff), E0r3 copy proposal, bounded execution proposal |
| **Source-inspected** | Candidate `src/` API used by r3, Core `validate`/`scenario_snapshot`/`simulate_metered`, ark-relations 0.4 `which_is_unsatisfied`, frozen packet rows, r2 harness, reviewed operator (read coverage in section 10) |
| **Own checks** | Stdlib pin/identity recomputation (all PASS); 13/13 synthetic parser cases (`records/PARSER-CHECK-RESULT.json`); static text checks of the r3 harness (`records/STATIC-HARNESS-CHECKS.json`, PASS). None is a Rust result |
| **Actual Rust results** | **NONE.** Nothing compiled or ran. All test/row counts below are predictions |
| **Adoption** | **NONE.** Fresh non-author source/effect review, second votes and root authorization are required before any use |
| **Acceptance** | None. The old actual run (`18afb040…`) stays accepted-qualified for its exact bytes only (BUILD `4e3ba2e2…`, `compiler_build_digest` `a6911ddd…`). This source does not retroactively close its omissions. Every U0–U7, M1–M17, MC/SP/G exit, all 20 pattern families, general obligations, ACTUS/DeFi memberships, recursion/PCD, private and multi-parent composition, P3 authority/physical settlement, ABI v2, staged `/2` lowering and Midnight ZKIRv3 remain open |

## 2. Identity, effort, independence

| Item | Value |
|---|---|
| Requested | `claude-opus-5-5`, effort high, one attempt after 18:00:05Z (wrapper) |
| Acting | `claude-opus-5-5` (runtime-stated); I cannot observe my effort. No subagents |
| Independence | Context-only. Every author, reviewer, voter and operator in this lineage is the same model |
| Authorship | I authored the terminated partial's work (same role, earlier context) and everything new here. I authored none of the reviewer packet, addenda 1–2, r2 harness/plumbing, result review or operator |
| Not read | No `provider-result.private.json`, `process.stderr`, private reasoning, session log, credential, environment, key, PK/SRS or proof cache was opened, hashed or stat'ed. Wrapper files in this directory (`PROMPT.md`, `PREPARATION.json`, `DELAYED-START.json`, `PROCESS.json`, `run-delayed.py`) were not read |

## 3. Recovery and failure chronology (UTC)

1. 18:00:15 Read the installed `develop/SKILL.md`, then `execution-focus.md`, `RECOVERY.json` and the full `PUBLIC-PARTIAL-TREE-MANIFEST.json`, whose recomputed SHA `fd38e491…9aee` matched the given value.
2. **D-1.** My first verification command (a `mkdir` + heredoc Python + `cd`) was denied by the harness before execution ("brace with quote character"). Nothing ran. I moved all scripts into `tools/` files run with `python3 -I`.
3. Pin verification (`tools/verify_partial_pins.py`): **156/156** pinned public partial files of the terminated task match. Unpinned names: the old wrapper's `PROMPT.md`, `PREPARATION.json` and `run-author.py`, listed by name only.
4. Parent re-verification (`tools/verify_parents.py`, `records/PARENT-PIN-REVERIFY.json`): all 7 manifests in the old `PARENT-PINS.json` (`11a348a5…`) and the 18 named files are byte-equal now. The live 70-file Core closure recomputes to `f47318a2…`. 14 run-log entries with `stderr` in the name were skipped by name. They are experiment logs, not provider files, but they were not needed.
5. `records/PRE-EDIT-FREEZE.json` (`bfcfbf63…`, PASS) pins the 42 packet files, the 12 r2 files, both reports, the operator tools and the inherited financial/addendum-3 records, before any assertion edit.
6. Seeded own `candidate/` (128 files), `review-overlay/` (5) and `inherited/old-partial/` (17 history files) byte-identically (`records/SEED-RECORD.json`).
7. Addendum 4 and HW-TRACEABILITY-2 written, then frozen (`records/ADDENDUM-4-FREEZE.json`, `377ab496…`). The r3 assertions were edited after that.
8. **D-2.** A compound `head`/`grep` read of the run logs was denied by the harness before execution. I used the Read tool instead.
9. **D-3 (my deviation).** The prompt's first-call list includes the active checkout's AGENTS.md, product contract, SECURITY-ROUTING and UNI-020. I read them after the harness edits (≈18:27Z), not first. They did not change any edit: UNI-020's dust field has no frozen value and stays an OPEN row.
10. Parser check scope written (`records/PARSER-CHECK-SCOPE.md`) before the single bounded run: 13/13 pass on the first run.

## 4. What was inherited and what is new

The terminated partial is UNACCEPTED work. I verified it and carried it forward rather than redoing it:

| Inherited (byte-identical, pins verified) | Status here |
|---|---|
| `candidate/` P-1/P-2 changes: `COVERAGE.md`, `coverage.rs`, `BUILD-MANIFEST.json`, `BUILD-IDENTITY-DELTA.json`, regenerated `SOURCE-MANIFEST.json` | Kept unchanged. My own recomputation confirms them (section 5) |
| `review-overlay/heldout-addendum-3/` (AUTH, frozen 16:00:42Z) | Kept unchanged. Two prose corrections are in addendum 4 |
| Unfinished `heldout-harness-r3` draft (`0dd8b1f5…`) | Edited. The diff vs the draft is in `DIFFS/review/heldout.rs.vs-inherited-r3-draft.diff` (−23/+326) |

New in this resume: addendum 4 + HW-TRACEABILITY-2, the r3 edits listed below, `r3-tools/copy_verify_r3.py`, `R3-MANIFEST.json`, `operator-proposals/`, `EXECUTION-PROPOSAL.json`, `COVERAGE-MATRIX.json`, `RESPONSE-MATRIX.md`, `VOTES.md`, the records under `records/`, and this report.

## 5. Candidate identities (stdlib recomputation; predictions where stated)

| Identity | Value |
|---|---|
| Delta vs parent `a6e17efd…` | 122 unchanged, 4 changed (`BUILD-IDENTITY-DELTA.json`, `BUILD-MANIFEST.json`, `COVERAGE.md`, `src/coverage.rs`), 0 added; all 9 test files and the other 14 `src/` files unchanged (`DIFFS/candidate/`) |
| BUILD-MANIFEST | `edbe3193c8eabc51a1d5595757c69c0bbe2caeb89e53cc3dc25564cead44b41a`; `/1` format and keys equal to the parent; 16 entries equal the bytes; only `coverage.rs` differs from `4e3ba2e2…` |
| SOURCE-MANIFEST | `d3bdd5009e42e13110559142a7d62ba2823f0723690ae1094033e7684b09005c` (126 entries; sidecar equal; names BUILD `edbe3193…`) |
| Core closure | `f47318a2…35ed`, 70 files, unchanged (`6c7b1fda`; main `30b65` Sherlock is outside and unqualified) |
| `compiler_build_digest` | **Predicted** `0b896adbe183b79d82455fde7b2c5e988b166a63a5fde972d3621eb7ad98bf19`; method control recomputes the observed parent `a6911ddd…`. Evidence only when an authorized r3 run prints it |
| Constraint code | Unchanged. `coverage.rs` is data, not referenced by `lower`/`circuit`/`adapter` (grep). The E4 test `coverage_rows_exist_for_every_construct` reads only construct rows. Every `r1cs_sha256` still changes through the header constant (R-04 disclosure) |

## 6. P-1 to P-4 as implemented

**P-1 (R-01).** The reviewer's sentence is in `COVERAGE.md` verbatim. `REFUSALS` rows `mori::core::bound` and `mori::core::liability (scenario identity differs)` are `TargetPremise` and name pre-state authenticity and frame completeness. A third `TargetPremise` row covers scenario asset/balance/liability row refusals. Missing balance (F-04, M-14 SAT) stays a premise, and unknown or outside-frame rows are stated as not excluded by the relation.

**P-2 (R-04).** `BUILD-IDENTITY-DELTA.json` `r04_disclosure`: `origin_build_manifest_sha256` and `vs_origin_r2` bytes move `compiler_build_digest`, the header constant and every `r1cs_sha256`. Format `/1` is unchanged, and byte identity is not provenance or security.

**P-3 (harness r3, 14 tests, `c2a1d165…`).**
- *Typed outcomes:* SAT / UNSAT / NOT-ENCODABLE / NOT-DECODABLE / ASSIGNMENT-SHAPE / CIRCUIT-ERROR. Circuit, synthesis and `is_satisfied` errors FAIL. Rows that carry no relation evidence are OPEN, each test prints per-class counts, and nothing OPEN counts as OK.
- *Hostile:* 27 UNSAT predicted. H1-R6/R7 PREMISE and H1-R11/H2-R5 NOT-DECODABLE are OPEN. The re-encoded `-r3` claims (addendum 3) are expected UNSAT. Addendum 4 notes that both step-0 `requires liability_present` and Discharge refuse.
- *Missing frozen rows:* S-9 runs as 5 rows, each requiring `CompileError::Bound` naming its cap with no relation produced:
  - trace and cells, via `FrameCells::Include`;
  - states: 200 states with 400 statements, 203 × 402 = 81,606 > 65,536;
  - duties: 1,026 registry duties compiled in a 1-statement action, so 1,028 × 3 ≤ 65,536 and the 1,024 duty cap refuses;
  - duties-trace.

  M-10, M-11a and M-13 (×3) run on their frozen base vectors. M-11b is a recorded CONTRADICTION: `lo + r` is the honest field vector. M-06's relation half and M-07 are OPEN target obligations.
- *Booleanity isolation (new):* the public-only M-10 edit is refused by the binding equality, not by booleanity. M-10/wire and HW-23-r3/H6-B1/wire-two set the entry wire (`Tamper` `entry/input0#0`) and its public field to 2. They require UNSAT at constraint index 0, inside the entry region, with exactly 1 override.
- *HW-01..26:* v2 trace with machine refs, 14 RELATION-EXECUTED, 8 RELATION-PARTIAL, 1 ENCODER-ONLY, 2 CORE-AND-PREMISE, 1 STRUCTURAL-SOURCE. All 26 refs resolve statically. The harness prints every entry as OPEN, and the parser resolves the refs after a run. New dated cases:
  - HW-14-r3: the H1' (`usdy`) public vector under H1 is UNSAT.
  - HW-15-r3: ASSIGNMENT-SHAPE, explicitly not relation evidence.
  - HW-18-r3: W = 35 is UNSAT.
  - HW-23-r3/H6-B1.
  - HW-25-r3: Core refusal plus a one-slot layout; the premise stays OPEN.
  - HW-26-r3/unsorted. HW-26 padding is NOT-APPLICABLE (OPEN).
- *Financial (N-03):* for all 30 accepted vectors, every frozen accounting field is compared with actual Core, via harness arithmetic over Core's records and never the candidate's accounting code. The fields are gross, self-excluded gross, fees, net, pre/post, duty opening/accrued/discharged/closing, presence, present-zero and residual. The public statement slots are checked too. A second relation per accepted vector uses `GrossDebitAndFees` with the vector's own cap. Its bound public sums must equal the frozen gross/fees, and +1 tamper rows must be UNSAT. The H1-P1 frame variant is included. Multi-asset, self-transfer, self-fee and PublicCap cases are covered by the frozen vectors themselves. All 32 rejected vectors compare frozen phase and work_attempted (`simulate_metered`, AUTH method, unmetered control). The addendum-2 `work_used` is compared. Reserve, DUST, service and recovery planes are OPEN NOT-REPRESENTED.
- *ProbeReport (N-07):* 41 configurations print constraints, instance/witness variables, `r1cs_sha256`, field bits, program/profile/shape/build/relation identities, W, adapter and Core closure. These are future observations; only build-digest equality and nonzero values are asserted. N-07 stays open until an authorized run.

**P-4.** `operator-proposals/libtest_rows.py` (`466aeb8a…`) is one parser for RESULT.json (`result_tests`) and analysis (`analyze`). It splits `test NAME ... <first line>` before classification, and its identity lines include the inline line. Analyses are dated and created O_EXCL, with an append-only index and a dated error artifact on failure. It refuses to write inside `run/` and never touches raw logs or RESULT.json. It compares a legacy RESULT.json and reports the defect without rewriting anything. `OPERATOR-RESULT-PARSER.diff` (`7c70270e…`) changes only the reviewed operator's parsing: it executes the FREEZE-pinned helper bytes and adds no step change. Own synthetic checks pass 13/13, including reproduction of the N-04 legacy defect on synthetic text (scope written first; CPU 60 s, AS 1 GiB, wall 120 s; writes only `records/parser-check/` and its result file). The helper's effects for operator use are **not reviewed**.

## 7. Predicted future observations (AUTH, not evidence)

`records/PREDICTED-ROWS.json`: E4 41 tests (names unchanged); E5-r3 14 tests; 3 identity lines (BUILD `edbe3193…`, closure `f47318a2…`, digest `0b896adb…` predicted). The r3 tables are predicted at 574 rows: 503 OK, 0 FAIL and 71 OPEN by design.

| Table | Rows | Table | Rows |
|---|---|---|---|
| heldout | 121 | accounting | 95 |
| hostile | 32 | grossfees | 93 |
| mutations | 23 | rejected | 95 |
| addendum2 | 8 | hw25 | 4 |
| static | 18 | hwtrace | 28 |
| addendum3 | 4 | probe | 41 |
| addendum4 | 12 | | |

## 8. Bounded future execution proposal (no authority)

`EXECUTION-PROPOSAL.json` (`2d803e50…`) reuses the proven block unchanged:
- toolchain: Rust 1.99.0 with four absolute pins and absolute `RUSTC`/`RUSTDOC`;
- environment: `unshare --net` + `env -i` with the same 11 variables;
- cargo home: Option A over the 102-crate union `328d157e…`, which suffices because the candidate `Cargo.toml` is unchanged and r3 declares exactly r2's dev-dependencies (the lock gates enforce this);
- limits: AS 8 GiB, CPU 60 for E0r3/E1-family/gates/E2 and 900 for E3a–E5-r3, wall 1800, jobs 2, disk stop at 3 GiB;
- one attempt, no retry.

Fresh scratch: `current-core-groth16-evidence-repair-check-r3-1` (absent by a name-only check; the consumed scratch is never reused). E0r3 must reproduce the layout `moriarty/` 70, `r1cs/` 127 and `review/` 64 (42 frozen + 13 r2 + 9 r3) with the old relative paths. The harness recomputes BUILD and closure from the copied bytes, as r2 did. The proposal requests nothing and consumes, resets or creates no counter. It needs a fresh non-author source/effect review of the exact candidate, the overlay with its diffs, the helper and the proposal, plus a second vote on the new step list and scratch, root authorization, and a separately reviewed operator.

## 9. Findings carried and opened

Open: R-01 and R-04 implemented in source only, pending review. F-02, F-03, F-04 (M-14 SAT), F-05 (M-06/M-07), F-08 (P-5, one vote), F-10 and F-14 are unchanged. N-05 is root-side. Further open items:
- M-11b v2 needs reviewer adoption.
- HW-01..04 adversarial internal witnesses on held-out programs (honest completion only).
- HW-20 is supported by source only.
- HW-26 padding.
- Matrix identities (N-07).
- `which_is_unsatisfied` index format (UNVERIFIED-LIB until run).
- All §1 acceptance items.

No new constraint-level defect was found in the inspected source. This is a source reading, not a soundness claim.

## 10. Read coverage

- **Full:** installed `SKILL.md` and `execution-focus.md`; `RECOVERY.json`; `PUBLIC-PARTIAL-TREE-MANIFEST.json`; result review `REPORT.md` and `FIX-PROPOSALS.md`; old `DELTA.json`, `PARENT-PINS.json`, `ADDENDUM-3-FREEZE.json`, `FINANCIAL-EXPECTATIONS-FREEZE.json` and `PROCESS.json`; the 4 old diffs; old addendum 3 and HW-TRACEABILITY; old r3 draft; r2 `heldout.rs`; candidate `lib.rs`, `coverage.rs`, `adapter.rs`, `circuit.rs`, `statement.rs`, `plan.rs`, `error.rs`, `identity.rs`, `witness.rs`, `r1cs.rs` and `frontend.rs`; `copy_verify_r2.py`, `run_pathb.py` and `analyze.py`; held-out programs H1, H2, H4, H5, H6 and H8; checkout `AGENTS.md`, product contract and SECURITY-ROUTING; superpowers `verification-before-completion`.
- **Partial:** candidate `COVERAGE.md` (55–106 plus grep), `refusals.rs` (140–153), `arithmetic.rs` (44–155), `tamper.rs` (grep), `gadgets.rs` (grep: `alloc_bool`, `alloc_uint`); Core `runtime.rs` (8–64 grep, 60–334, 525–664, 865–912), `logical_cost.rs` (33–72), `ir.rs` and `lib.rs` (grep); ark-relations 0.4.0 `constraint_system.rs` (`which_is_unsatisfied`); UNI-020 in spec.md; `E5-r2.stdout` (1–40) and `E4.stdout` (1–30); `receiving-code-review` (1–60).
- **Parsed by my scripts only:** HELDOUT-VECTORS, HOSTILE-CLAIMS-ADDENDUM, HELDOUT-ADDENDUM-2, H3 JSON, the R2 request, CORE-CLOSURE-PINS, SOURCE/BUILD manifests.
- **Not read:** candidate `lower.rs`, `staged.rs` and `groth16.rs` (hash only); other test files beyond the lines named; the independent source-review REPORT; ROADMAP; the wiki; P3 lane drafts (deliberately not read); wrapper and private files (section 2).

## 11. Files

| Path | Purpose |
|---|---|
| `candidate/` | Resume candidate (SOURCE `d3bdd500…`, BUILD `edbe3193…`). Its `REPORT.md`, `RESPONSE-MATRIX.md` and request files are the parent's historical copies, not this report |
| `review-overlay/` + `R3-MANIFEST.json` (`64067829…`) | Addenda 3–4, r3 harness, E0r3 copy proposal |
| `operator-proposals/` | P-4 helper and parser-only operator diff |
| `DIFFS/candidate/`, `DIFFS/review/` | Diffs vs parent, r2 and the inherited draft |
| `EXECUTION-PROPOSAL.json`, `COVERAGE-MATRIX.json` (`5e6ee8db…`, 144 typed rows), `RESPONSE-MATRIX.md`, `VOTES.md` | Proposal, typed coverage, responses, literal votes |
| `records/` | Pin verification, freezes, seed, identities, predictions, parser and static checks |
| `inherited/old-partial/` | Byte copies of the terminated job's top-level records and tools (history) |
| `tools/` | My stdlib scripts (run with `python3 -I`) |
| `OUTPUT-MANIFEST.json` | SHA-256 of every output file except the wrapper files |

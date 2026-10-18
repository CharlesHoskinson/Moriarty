# REPORT: generic current-Core → arkworks 0.4 R1CS compiler, source candidate (r2 recovery)

**Author:** Claude Opus 5.5 (`claude-opus-5-5`), cryptographic compiler source author. "High" effort was requested; I cannot observe it. No auxiliary model or subagent was used. No child process remains.

**Terminal status:** `SOURCE_COMPLETE_FIRST_SLICE_UNEXECUTED`. The candidate is UNADOPTED, UNREVIEWED and UNCOMPILED. Nothing in this directory was built, type-checked, tested, proven, set up, keyed or run against a ledger.

## 1. Separate axes

| Axis | State |
|---|---|
| Implementation | Complete source for the first slice: a generic `moriarty/1` typed-Program → R1CS compiler, one relation per (program, action), admission through actual Core, a typed statement layout, an UNADOPTED adapter boundary, witness proposal separated from acceptance, `/2` admitted then refused with a real staged plan, coverage/refusal tables as code, and 39 test functions |
| Evidence | Source-static only: 64 lexical static checks (`STATIC-CHECKS.json`; delimiters, module wiring, forbidden constructs, exhaustive-match spelling, manifest pins, fixture presence, test-name cross-check). Not a compile, type check or test |
| Acceptance | None. Fresh non-author review of the exact frozen bytes, fresh non-author held-out vectors and hostile mutations, and an authorized isolated run (`EXECUTION-REQUEST.json`) are all still required |

## 2. Startup and scope

I read the installed `moriarty-dev` develop skill, `references/execution-focus.md`, checkout `AGENTS.md`, `docs/MORIARTY-PRODUCT-CONTRACT.md`, `ROADMAP.md` and the consolidated-language-kernel `spec.md` (UNI-001–024). The root-refreshed guarded status (`guarded-status-20261015-continuation-1000.json`) reports historical SP01.6 loan-swap blockers. That status is operational history: this task dispatches nothing, and I did not run the CLI. Native SDK authority stays zero. Writes stayed inside this directory, and the live worktrees and the previous lane are unchanged.

The product contract's terminal target is Midnight ZKIRv3. This arkworks/Groth16 relation belongs to the separately scoped Cardano eUTxO research lane. It is not a ZKIRv3 artefact or a Midnight result.

## 3. Inputs (read in full for every claimed support)

- **Root copy.** `ROOT-PARTIAL-SOURCE-COPY.json`: all 26 copied files hash-verified. `PRIOR-SOURCE-PINS.json`: all 36 checkout files re-verified against the live worktree (`SOURCE-PINS.json`, `all_match: true`). HEAD `6c7b1fdaffec98d4fa912c9b16a3c4879f572a47` was read from the worktree's git metadata files, without running git.
- **Core, read fully.** lib, ir, parser, runtime, logical_cost, policy, intent, source_sponsor, and stage/{mod, plan, profile, digest, validate, admit, decode, cost}.
- **mori-cardano, read fully.** lib, types, layout, profile, export, projection, staged, staged_projection, plus the fixture-set listing and the metadata-consumer manifest and lock.
- **Reviews and design.** `x7-independent-result-review-20261015/REPORT.md` and `claim-utxo-aiken-r1-independent-result-review-20261015/REPORT.md` (complete). The prototype `implementation-attempt-2/source/x7` Cargo.toml, Cargo.lock (arkworks 0.4 versions) and relations.rs (API shape only; no code copied). Frozen ABI v1: `ABI-V1.md`, `COMPILER-PLAN-B-C.md`, `VOTES.md`, with manifest `96e85df9…` and report `ae697f4a…` verified.
- **Oracles.** The `/1` independent oracle packet and the staged positive oracle packet.
- **Upstream reports.** `sol-currentness-analysis/REPORT.md` and the FREEZE-4 r2 `REPORT.md`.
- **Not opened.** No private provider body or reasoning, no key, PK, SRS, cache or old-campaign material.

## 4. The falsifiable source predicate and how the source meets it

The predicate: one compiler accepts two contrasting fresh valid programs through the same source → typed Program → constraint-lowering path, and every accepted witness satisfies the compiled current semantics.

- **Same path.** `compile_source` → `frontend::admit_source` (actual `mori_core::language` + `parse`, which validates, + `require_core1`) → `plan::plan_action` → `StatementLayout::build` → `circuit::synthesize` → `lower::lower_action`. No branch looks at a program name, hash or fixture; `static_checks.py` scans `src/` for name recognition and fixture paths.
- **Contrasting programs.** The installment invoice (3 actions) and the conditional reserve (2 actions, two assets) come from the non-author oracle with complete outcomes. The fresh payroll ledger (Emit, self-transfer, repeated Next, positional ensures) and the arith-boundary and self-move programs come from the author packet. There is also a manual DAG IR program, and an inline canonicality program in the tests.
- **Witness acceptance.** Internal wires are computed only from the claimed inputs and pre-state. The claimed post-state, effects, cap and accounting enter only as public values equated with computed wires. `witness::propose` may call `simulate`/`simulate_metered`, but a refusal yields no proposal and a success is still checked (`propose.rs::a_reference_success_is_not_trusted`).

## 5. What the relation constrains (admitted successful `/1` transitions)

- **UInt128.** Bit-decomposed 64-bit limbs. Checked Add, Sub and Mul; Mul keeps the 256-bit intermediate exact (`a1·b1 = 0` plus exact recomposition), so the field-wrapped product of A11 is refused. Comparisons use a unique flag-and-remainder gadget; equality uses the inverse-flag pair.
- **Bool.** Canonical {0,1}.
- **Evaluation.** Strict: every node reachable from each statement root is lowered, `lhs` before `rhs`, memoized per statement and re-lowered in later statements. There is no short-circuit and no inactive path to gate (A15/T8, D1).
- **Statements.** Positional Requires/Ensures. Next rebinds both the state and the name. Emit is a value slot. Transfer/Fee debit before credit, and a self-transfer keeps its debit check. Liability accrues and sets present (present-zero allowed). Discharge requires present and keeps present-zero. Absent duties are canonical `(false, 0)` in-circuit.
- **Effects.** Ordered effects with constant kinds and fields bound through the shape digest.
- **Work.** `W` is a header constant; `W ≤ cap` is enforced when `CapBinding::PublicCap`.
- **Accounting (optional, UNADOPTED).** Exact 144-bit gross debit and fees per debit cell, never stricter than `simulate`.
- **Identities, in-circuit.** Relation version, canonical Program hash, profile digest (language, semantics, logical-cost profile, relation format), action index, shape digest and compiler build digest. The build digest covers BUILD-MANIFEST `2c943b511bb6cb912889728ed2c5de90d4257939ded53b1d626b998edd3beb21` plus Core closure `f47318a26a4df0477e89962a9de0e9b48f12d01d0e9b4a8f5e74603beaa235ed`. The raw source hash is recorded separately and never bound. The R1CS matrix digest is computed by the compile probe and binds no VK without a setup transcript.
- **Public encoding.** Exact widths, limb ranges forced in-circuit, no modular alias.

Arguments are in `MATH-AND-SPEC-ATTRIBUTION.md`; per-construct rows are in `COVERAGE.md` and `src/coverage.rs`.

## 6. First-slice refusals and declared semantics

`COVERAGE.md` lists every refusal with its reason:

- `moriarty/2` → `StagedNotLowered(plan)`.
- External duties → `Unsupported`.
- Actions that can never succeed → `StaticFailure`.
- Over-bound represented snapshot → `Bound`.
- Small field → `Field`.
- Zero build digests → `Identity`.
- Pending claimant-bound ABI → `Adapter(Unadopted)`.
- Initial ledger target with any Emit → `TargetRefusal` (two adopted votes).

Every `/1` Core refusal has no satisfying success assignment (`coverage::REFUSALS`). A rejection relation is not produced. Target obligations stay explicit (`adapter::TARGET_OBLIGATIONS`): pre-state authenticity and KeyHeldExternal premises, frame completeness, physical asset delta, debit authority and intent limits, replay and consumption, phases, and setup provenance.

## 7. Staged `/2`

`staged.rs` derives segment shapes from the real `StagePlan`: entries, exits, capture layouts, binders, statement ranges, segment costs, committed prefixes, NotAdvanced attempted work and Emit presence. The stored cap is an explicit public slot, and the D-13 value stays split and unselected. Tests compare this against the independent staged oracle: costs `[5, 6]` and `[11, 15, 10]`, NotAdvanced 2 / 4 / 2, and oracle-derived program and plan hashes. `STAGED-LOWERING-PLAN.md` gives the relation design and implementation order. All staged obligations remain required.

## 8. Expectations and tests

- **Packet.** The prior packet (`EXPECTED-PACKET/FREEZE.json`, unchanged) plus `ADDENDUM-1.json`, frozen at 12:17:30Z before any r2 assertion was written. The addendum records errata E-T1 and E-T4, oracle-derived program hashes, accounting rows G1–G7 and staged rows. The author rows are my own extra derivation, not an independent oracle.
- **Tests.** 39 functions in 7 files. They cover both oracle lifecycles end to end; the typed Program and hash equality; the payroll P1–P3 and P5 vectors; A1–A18; the hostile A11 wrap and Lt-flag tampering; present-zero versus absent (T9) and isolated canonicality (T6); P6/P7; S1–S3; frame cells; D1; T1 over every body slot with the E-T1 exception; T2, T3, T4 and T1-cap; R-PARTIAL-EQ honest and forced; G1–G7 and G-T1; the staged plan; refusals; coverage exhaustiveness; and the reference-proposal differential. Every test is labelled ORACLE, AUTHOR or HOSTILE. None ran.

## 9. Design votes (detail in `DESIGN-VOTES.md`)

- **Four audit questions.** I agree on identity (amended: in-circuit constants and Core-closure build identity), constrained semantics and complete ordered effects/state/duties/work/ranges. On the differential oracles I agree with the direction and dissent on the clause matching refusal codes to the first violated region.
- **P4.** Literal AGREE on items 2, 5 and 6, which gives them two votes with V5. Items 1, 3 and 4 are AMENDed.
- **P3.** Literal AGREE on P3-1, P3-2 and P3-4 through P3-12. P3-3 is AMENDed: net per cell is `post − pre`; gross is implemented.
- **New choices.** C1–C15 are my first votes. I abstain on the D-13 value.

## 10. Failures and deviations (`FAILURES.json`)

- Inherited: the previous lane ended on quota; the prior manifest named arkworks 0.5 (corrected to the exact prototype 0.4 pins); the prior `lib.rs` declared seven missing modules.
- Found in r2: errata E-T1 and E-T4.
- One tool denial: an inline `python3 -c` scan was refused for a quoted newline followed by `#`. It was not retried in that form; the same checks were run as a script file.
- The main open risk: all Rust is uncompiled, and the arkworks 0.4 API facts are recalled, not inspected.

## 11. Proposed future execution (`EXECUTION-REQUEST.json`; not authorized)

- **Layout.** Isolated byte copies of the 70 Core closure files and this task tree; the relative `mori-core` path resolves only inside that layout.
- **Lock.** No lock is invented. `cargo generate-lockfile --offline` over the expected union of the prototype arkworks 0.4 closure and the metadata-consumer Core closure is BLOCKED if anything is missing from the cache, and network access needs its own authorization.
- **Toolchain and sandbox.** Rust 1.99.0 under `unshare --net`, 8 GiB address space, CPU and wall caps, a stop rule, and the 39 expected test names.
- **Never.** Setup, keys, proving, network, Aiken, SDK or ledger.

## 12. Remaining required work (none waived; no whole-language percentage)

- Compile and run this candidate under review.
- The ABI v1 mapping, after that ABI's own review.
- The full staged `/2` relation set and the codec correspondence.
- A rejection relation, external duties and P3 physical integration.
- All 20 patterns and the general obligations; liability execution and asset integration; private witnesses and handoff; multi-parent composition; recursion and PCD.
- Every U0–U7, M1–M17 and MC/SP/G exit.

The incoming Sherlock typed host, K/Quint models, Candidate B and helper microstep get no compiler or target support claim. Its 35 UIREQ mandatory clauses and 6 unregistered closure members stay open. No VK, PK, proof or claim from X7 v0 or attempt-2 is reused; orphan Class 0 VKs cannot be a real genesis.

## 13. Files

- **Source.** `source/mori-r1cs/` (Cargo.toml, 16 src files under BUILD-MANIFEST.json, tests/).
- **Pins.** `CORE-CLOSURE-PINS.json`, `SOURCE-PINS.json`.
- **Expectations.** `EXPECTED-PACKET/` (prior + ADDENDUM-1), `fixtures/reused`, `fixtures/staged` (+ COPY-RECORD.json).
- **Documents.** COVERAGE.md, MATH-AND-SPEC-ATTRIBUTION.md, STAGED-LOWERING-PLAN.md, DESIGN-VOTES.md, EXECUTION-REQUEST.json, FAILURES.json.
- **Checks.** STATIC-CHECKS.json and `static-passes/STATIC-CHECKS-PASS-1.json`; `tools/` (static_checks.py, pin_core_closure.py, manifest.py).
- **Manifest.** `SOURCE-MANIFEST.json` covers every task file except the harness-owned and private ones; its digest is reported in the final response.

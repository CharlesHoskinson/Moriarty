# Independent source and effect review: evidence-repair resume (P-1 to P-4, r3 harness, parser helper, execution proposal)

Reviewer: `claude-opus-5-5`, fresh non-author security and cryptographic source/effect reviewer, 2026-10-15. Requested effort high; I cannot observe my actual effort. No subagents. I authored nothing under review. Every author, reviewer, voter and operator in this lineage is the same model, so independence is context-only.

Reviewed candidate: `current-core-groth16-evidence-repair-resume-source-20261015`, with REPORT `e72f1aa3…1fda` and OUTPUT-MANIFEST `99589a04…c9`. Candidate pins: SOURCE `d3bdd500…005c`, BUILD `edbe3193…b41a`, R3-MANIFEST `64067829…76aa`, EXECUTION-PROPOSAL `2d803e50…a51d6`, Core closure `f47318a2…35ed` (70 files at `6c7b1fda`).

This is a source review. I ran no Rust, Cargo, rustdoc, compiler, model, harness, helper, operator, crypto, setup, proof, network, Git or ledger operation. I imported and executed no candidate or helper code. My checks are my own stdlib Python in `work/`, run with `python3 -I -B`.

## 1. Verdicts (separate axes)

| Axis | Verdict |
|---|---|
| **SOURCE** (candidate `d3bdd500…`) | **ACCEPT-DELTA, unit scope.** P-1 (R-01 wording) and P-2 (R-04 disclosure) are implemented as specified. The delta against the parent `a6e17efd…` is 4 changed files, 122 unchanged and 0 added. 14 of 15 `src/` files and all 9 test files are byte-equal to the parent bytes the old run executed. The only changed `src` file, `coverage.rs`, is data: it is referenced by no constraint-generating code, and its single test consumer reads only construct rows. **Constraint semantics are unchanged.** I found no new source defect. This is not adoption and not a soundness proof. |
| **EXPECTATIONS** | **Frozen independent expectations preserved.** All 42 frozen packet files and the 12 r2 files equal their manifests. The 61 vectors, addenda 1 and 2, the HW proposals and the static rows are unchanged. r3's assertions match the frozen rows wherever they execute them. M-11b is kept and reported OPEN CONTRADICTION, not rewritten. Addenda 3 and 4 are correctly labelled AUTH, post-freeze and not held-out. **One CHANGE:** HW-06, HW-07 and HW-23 overstate their witness mode (IR-01, NV-8). |
| **PARSER-ONLY ACTUAL RESULTS** | **ACCEPT-QUALIFIED as synthetic parser evidence only.** The author's 13/13 synthetic cases (`records/PARSER-CHECK-RESULT.json`, helper `466aeb8a…`) ran under a scope written beforehand. I did not rerun them. They show parser behaviour on synthetic text, including reproduction of the N-04 legacy defect. They are no evidence about any Rust run and no review of operator effects. |
| **EXECUTION PROPOSAL** | **NOT YET ELIGIBLE; eligible to proceed to operator authoring.** The resource block reused from the executed run has my AGREE (NV-11a). The new scratch and r3 step list have my CHANGE (NV-11b): pin and verify the E0 tool and the parser before exec, and create the analysis directory. No r3 operator exists yet (IR-07). A future run also needs a second vote on the amended step list, root authorization and an exact-byte effect review of the r3 operator. The proposal adds no setup, proof or ledger authority. |
| **ADOPTION** | **NOT ADOPTED.** No r3 Rust result exists. The 14 E5-r3 tests, 574 rows (503 OK / 71 OPEN), the 41 probe configurations and `compiler_build_digest` `0b896adb…` are predictions. The old run (`18afb040…`, BUILD `4e3ba2e2…`, observed digest `a6911ddd…`) remains accepted-qualified captured-local constraint evidence for its exact bytes only, not adopted as complete financial or security evidence. |
| **Acceptance** | None. Every U0–U7, M1–M17, MC01–MC08, SP01–SP12 and G01–G24 exit stays open. So do the 20 pattern families, general obligations, ACTUS 277/18/32/DS01–07 and DeFi 72/DA01–24/held-outs/intent/regression memberships, recursion/PCD, private handoff, multi-parent composition, ABI2, P3 physical settlement, signed intent and debit authority. No current-language proof, ABI2 execution, physical settlement or Preprod transaction is established. |

## 2. Callable review predicate

1. **Does the candidate implement P-1 to P-4 without altering constraint semantics or frozen independent expectations?** **Yes, at source level.**
   - P-1 and P-2 are in the candidate. P-3 is the r3 overlay and P-4 the helper plus diff; both sit outside the candidate.
   - Constraint code is byte-identical to the executed parent. No frozen file changed.
   - Qualifications: M-11b is not executable as frozen (it is reported OPEN), and three HW classes need correction (IR-01).
2. **Does r3 distinguish relation refusal, encoding/decoding refusal, target premises, assignment shape and circuit errors?** **Yes.**
   - Six typed classes plus OPEN.
   - Circuit, synthesis and `is_satisfied` errors are FAIL. Candidate-internal synthesis failures surface as `SynthesisError::Unsatisfiable` from `synthesize_into` and are therefore CIRCUIT-ERROR, never UNSAT.
   - Null claims are PREMISE OPEN, not-decodable frozen claims are OPEN, and an exact-set check covers the four expected OPEN rows.
   - NOT-ENCODABLE counts as OK only where the frozen row says "or not encodable". Only UNSAT counts as relation evidence.
3. **Does it compare complete frozen accounting and rejected phase/work outcomes through actual APIs?** **Yes, for every represented frozen field.**
   - Fields compared against actual Core `simulate`/`simulate_metered` with harness arithmetic: balance pre, post, net delta, gross, gross excluding self and fees; duty opening, accrued, discharged, closing, present before/after and present-zero after; residual positive.
   - The candidate encoder's public statement is compared too. GrossDebitAndFees sums are compared through an honest SAT relation plus slot equality.
   - Phase and work attempted are compared for all 32 rejected rows.
   - Reserve, DUST, service and recovery planes are explicit OPEN rows, never zero.
   - `gross_debit_excluding_self` is Core-only, and net delta is derivable but not a statement slot.
   - My own recomputation of every frozen accounting field of all 30 accepted vectors from the frozen records shows 0 contradictions.
4. **Do the proposed effects and limits support a bounded future local check without setup, proof or ledger authority?** **Yes in proposal form, conditional on the r3 operator (R-C).**
   - No reachable setup, prove or verify path exists. `groth16.rs` is verify glue compiled only by E3c and called by nothing, and the harness does not enable the feature.
   - Network is isolated, the environment is `env -i`, Option A's 102-crate union is gated by the lock checks, and the limits and no-retry rule are unchanged.
   - Filesystem exposure of build scripts and proc-macros remains, as before (R-05).

## 3. Mechanical verification (my own; `work/VERIFY-RESULT.json`, `work/diffs/SUMMARY.json`, `work/FROZEN-DATA-CHECK.json`)

- **Author outputs.** 205 of 207 author output files hash-equal their manifest. The aggregate `b93d6a5b…` recomputes. I skipped two copies of wrapper-owned names (`inherited/old-partial/FINAL-RESPONSE.md`, `PROCESS.json`) by choice.
- **Pins.** All 16 pins I checked match, including root VERIFICATION `77064c8b…`, the result review `06e316bc…`, R2-MANIFEST `5a256c76…`, upstream 4d969 MANIFEST `186c4907…` and the parent manifests `a6e17efd…`/`4e3ba2e2…`.
- **Candidate manifests.** SOURCE is 126/126 with the sidecar equal and naming BUILD `edbe3193…`. BUILD is 16/16 and differs from the parent only in `coverage.rs`. Its format, keys and entry keys equal the parent's.
- **Lineage manifests.** The 42-file frozen packet, 12 r2 files, 8 R3 files and the old run outputs (excluding `*.stderr` and listings) all match their manifests.
- **Digest arithmetic.** `SHA-256(compact sorted-key JSON {compiler_source_manifest_sha256, core_closure_sha256})` reproduces the observed parent `a6911ddd…` and gives `0b896adb…` for this candidate. That is a prediction.
- **Diffs.** My difflib diffs equal the author's changed lines for `coverage.rs`, `COVERAGE.md`, `BUILD-IDENTITY-DELTA.json`, r2→r3 `heldout.rs` (+1386/−193), `Cargo.toml`, `lib.rs` and `copy_verify` r2→r3 (+38/−23).
- **Predicted row counts.** Derived from the harness loops and frozen data, I get heldout 121, hostile 32, addendum2 8, rejected 95, accounting 92 plus 3 "outside" rows, and probe configurations 41. All equal the author's predictions.
- **HW references.** The 26 HW entries are in frozen order, and every `{table,row}` ref resolves to a row id the harness can print.

## 4. Candidate against its pinned parent

- **R-01.** The reviewer's sentence is in `COVERAGE.md` verbatim. REFUSALS `bound` and `liability (scenario identity differs)` are now `TargetPremise` and name pre-state authenticity and frame completeness.
  - The new scenario-row `TargetPremise` row covers undeclared assets and invalid or duplicate balance or liability rows. That matches Core `scenario_snapshot` (`runtime.rs:586-608, 632-651`), which I read.
  - `plan.rs`'s bound is stated correctly. It refuses only when the represented snapshot alone exceeds Core's caps, counting every registry duty as present, while Core counts every scenario row.
- **R-04.** The disclosure is accurate and the `/1` format is kept.
- **Unchanged production source.** I read all 15 `src` files in full. Gadgets are functional in their inputs:
  - bit decompositions are unique;
  - the `lt` flag is unique;
  - `mul` is exact through `a1·b1 = 0` with a range-proven `c`;
  - `eq_uint`'s `inv` is free only when `d = 0`.
  
  Internal wires come only from claimed inputs and pre-state, and post-state and effects enter only through the binding. This agrees with the source and result reviews. It is a source argument, not a proof for arbitrary witnesses.

## 5. r3 harness review (`heldout.rs` `c2a1d165…`, 1,715 lines, read in full)

- **S-9.** Each construction reaches its designated cap first, within Core's declaration and statement limits (`ir.rs` 256 and 1,024):
  - trace via a 6,000-cell frame on H1 `settle` (10 statements): 6,010 × 12 = 72,120;
  - cells via a 4,097-cell frame on H8 `pay`: trace 4,099 × 3 = 12,297, then the cell cap;
  - states via H3 plus 200 states and 400 `requires true`: 203 × 402 = 81,606;
  - duties via H3 `d` plus 1,025 source liabilities over two actions and a one-statement action: 1,026 > 1,024 with trace 3,084;
  - duties-trace: 1,541 × 515 = 793,615.
  
  The cells and duties caps share one refusal message, so the substring check is weaker than its label (IR-02).
- **M-10 and HW-23 booleanity isolation.**
  - The frozen M-10 public-only edit is refused by the binding equality, not by booleanity. The author says so correctly.
  - M-10/wire and HW-23-r3/H6-B1/wire-two set the entry wire `entry/input0#0` and its public field to 2. That label is the first Bool input in both programs (H3 `poly_b` `v`; H6 `beq` `p`). `alloc_bool` emits `b·b = b` as constraint 0 of the entry region.
  - The row requires exactly 1 override and first unsatisfied index `"0"`. The index format is UNVERIFIED-LIB here (IR-04) and fails closed.
  - Only entry Bool wires are tampered. Computed internal Bool wires remain E4-only or source-argued.
- **M-11.** M-11a executes the frozen limb alias `(lo + 2^64, hi − 1)`, which the per-limb bit-sum binding refuses. The frozen M-11b `(lo + r, hi)` is the same field vector as the honest claim, so it is untestable at this layer. r3 keeps it OPEN CONTRADICTION without substituting anything. My disposition is in VOTES.md: keep the frozen text, re-home its intent to the byte-image ABI mapping, and add v2 only under a new id.
- **HW-01..26.** The classes are mostly right. HW-06, HW-07 and HW-23 should be RELATION-PARTIAL (IR-01, R-A). Every public-field mutation row is a binding-level refusal (IR-03). All HW entries print OPEN, and the parser never upgrades a class.
- **Financial comparison and GrossDebitAndFees.** Verified in §2(3). Multi-asset (H1-P1, H1-P2, W-1), self-transfer (same) and self-fee (H2-P3, H2-P4) are covered by frozen vectors. Unregistered duties (H1 `legacy`) and frame rows outside the statement become `NOT-IN-STATEMENT` OPEN rows, which keeps F-05 visible. The H1-P1 frame variant represents the auditor cell.
- **Rejected rows.** The `simulate_metered` API matches Core `logical_cost.rs:35-55`, and all 32 frozen rows follow the admission/execution rule. "Phase" is Core-local, not Midnight ledger phase (IR-06).
- **ProbeReport.** 41 configurations print counts, `r1cs_sha256`, identities and W. Only build-digest equality and nonzero values are asserted. All of these are future observations; nothing is predicted.
- **Evidence classes.** E4 (41 author/oracle tests, unchanged) is internal agreement and is the only source of tampered internal-gadget witnesses. E5-r3 relation rows use honest completion (`Tamper::none`) except the two AUTH entry-wire rows. Neither establishes arbitrary-witness soundness, target admission, signed intent, debit authority or frame completeness.
- **Static API check (prediction, not compilation).** Core IR struct fields used by the in-memory programs match `ir.rs` (`StateDecl`, `Action`, `Balance`, `Statement::Liability/Await/Requires`). Every candidate API the harness calls exists with a matching signature. The r3 dev-dependencies equal r2's.

## 6. Parser and effect review

- **`libtest_rows.py` (`466aeb8a…`).**
  - It handles the libtest `--nocapture` inline first line correctly.
  - RESULT.json and analysis share `parse_stream`. Analyses are `O_EXCL` dated files, the index is append-only, failures leave a dated error artifact, and writing into `run/` is refused.
  - It uses no subprocess, network or candidate import. It self-limits to CPU 60 s and AS 1 GiB.
  - Robustness gaps (IR-09): the output directory must already exist, the index is opened without `O_NOFOLLOW`, and a parser exception inside the operator's broad postcheck `try` would skip later postchecks.
- **`OPERATOR-RESULT-PARSER.diff` (`7c70270e…`).**
  - It is parser-only and fail-closed (FREEZE hash mismatch means STOP). It `exec`s the verified bytes once.
  - It keeps every r2 constant and step name, so it is not an r3 operator (IR-07).
  - The old `RESULT.json` defect it addresses is real: the frozen record lacks the inline build-manifest identity line and lists first data rows as outcomes. That record must stay unedited.
- **`copy_verify_r3.py` (`b37bbc1d…`).** It is the minimal successor of the reviewed and executed r2 tool, with collision and escape STOPs, pin checks and unchanged identity rules.
  - It has never run, not even `--dry-run`.
  - It reads git metadata beyond "the named directories" (IR-10).
  - It is executed from the author lane path, the same TOCTOU pattern as the executed E0r2 (IR-08).
- **`lock_gate.py` (`5e728a19…`), unchanged and previously executed.** The subset gate checks exact crates.io source, checksum and union pin, and refuses git, midnight and missing checksums. The pair gate checks identical `(version, checksum)` sets. The r3 local allow name `mori-r1cs-heldout-r3` equals the r3 `Cargo.toml` package.
- **What a future operator must implement:** listed exactly in `proposals/MINIMUM-REPAIRS.md` R-C. It still needs its own exact-byte effect review. No budget, retry or counter reset is created here, and the consumed old scratch is never reused.

## 7. Currentness limit

- The frozen compiler is tied to the 70-file closure `f47318a2…` at `6c7b1fda`.
- Public main `4d969554…` (PR68) matches 69 of those paths. `mori-core/src/lib.rs` differs: it adds `pub mod sherlock;`.
- `sherlock/mod.rs` at both a19b0 and 4d969 exports non-test modules (`compare`, `reference`, `reference_display`, `reference_types`). Only the helper and image proposals are `#[cfg(test)]` private.
- So a path-dependency build of mori-core at main compiles files outside the closure and gets a different `compiler_build_digest`. E0r3 STOPs on any closure byte change, so this fails closed.
- No r3 observation would say anything about main, Sherlock helpers or the staged `/2` source. The old compiler is not claimed to support them.

## 8. Findings, votes and repairs

- Findings IR-01..IR-13 are in `FINDINGS.json`. There are two Medium (IR-01 evidence class, IR-07 operator absence) and no blocking source defect.
- Votes are in `VOTES.md`:
  - P-1 AGREE, P-2 AGREE;
  - NV-1 to NV-7 AGREE (NV-4 as AUTH), NV-8 CHANGE, NV-9 AGREE (parser mechanism), NV-10 AGREE;
  - NV-11a (unchanged resources) AGREE, NV-11b (new scratch and step list) CHANGE;
  - M-11b: keep frozen and OPEN, re-home its intent, no rename of v2;
  - F-08/P-5 and D-13: ABSTAIN.
- Minimum repairs are in `proposals/MINIMUM-REPAIRS.md` (unreviewed): R-A traceability witness mode, R-B parser robustness, R-C r3 operator requirements. None touches the candidate.

## 9. Still open (preserved; none waived)

- R-01 and R-04: implemented, and my AGREE gives each a second vote; adoption is still pending.
- F-02 external duties, F-03 conservative bound, F-04 balance-row presence (M-14 SAT), F-05 duty frame (M-06 relation half, M-07), F-08 caller-supplied `CompilerBuild`, F-10 Emit policy unwired, F-14 rejection relation.
- N-05 (root side) and N-07 matrix identities (future observation).
- HW-01..04 adversarial internal witnesses on held-out programs; HW-20 source-only; HW-26 padding; the `which_is_unsatisfied` format.
- The staged `/2` lowering, ABI v1 mapping and DirectFieldV0 test-only status.
- Authority, signed intent and debit authority (V6 P3-7 dispute preserved); physical P3 effects.
- Every acceptance item in §1.

## 10. Process record

- **Startup.** I first read the installed `SKILL.md`, then `execution-focus.md`, then checkout `AGENTS.md`, the product contract, SECURITY-ROUTING and UNI-020 (spec, traceability, tasks, crosswalk). The root refreshed guarded status; I did not run the CLI. SP01.6 history selects nothing here. Native and SDK authority is 0, and the public developer path stays permissionless.
- **Denied commands.** Two, neither executed (`REVIEW-FAILURES.json`):
  - D-1: a `cd /tmp` prefix;
  - D-2: a `mkdir` plus heredoc Python.
- **Scope.** Before the phase-1 allowlist I read only governance files and the author's named OUTPUT-MANIFEST, and I listed entry names (no stat) of nine named directories. Allowlists phase 1–4 are in `work/`; each was written before its files were read.
- **Never opened.** I never opened, stat'ed, hashed or grepped any PROMPT, PROCESS, FINAL-RESPONSE, PREPARATION, run script, provider-result.private or stderr file, nor the old run's `*.stderr` or `operator.stderr`. I did no recursive hashing and no whole-task search.
- **Partial reads and uncertainties.** These are in `READ-COVERAGE.json` and `REVIEW-FAILURES.json`. The main ones: ark-relations not read; candidate test files hash-verified rather than re-read; compile-ability is a prediction.
- **Untouched.** I did not read or alter the P3 settlement or catalogue lane drafts.

## 11. Files

| Path | Purpose |
|---|---|
| `REPORT.md` | This report |
| `FINDINGS.json` | Findings IR-01..13, evidence axes, carried open items |
| `VOTES.md` | Literal votes with reasons |
| `proposals/MINIMUM-REPAIRS.md` | Unreviewed minimum repairs R-A, R-B, R-C |
| `READ-COVERAGE.json` | Full, partial, parsed-only, hash-only, not-read and never-opened lists |
| `REVIEW-FAILURES.json` | Denied commands, scope notes, own-tool defects, uncertainties |
| `work/ALLOWLIST-PHASE{1..4}.json`, `work/allowlist_phase{1,2}.py` | Read allowlists, written before the reads they permit |
| `work/verify.py`, `work/VERIFY-RESULT.json` | Hash, manifest, parent-delta and digest-arithmetic checks |
| `work/diffs.py`, `work/diffs/` | My difflib diffs and the comparison with the author's diffs |
| `work/frozen_data.py`, `work/FROZEN-DATA-CHECK.json` | Frozen accounting recomputation, inventories, rejected-phase rule, row counts, HW refs |
| `work/review_manifest.py`, `REVIEW-MANIFEST.json` | Manifest of these public artifacts, written last |

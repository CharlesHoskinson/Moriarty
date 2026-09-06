I could not save the report: this workspace is read-only. The audit verdict is **REVISE**. Report content follows.

# Audit of experiments/zkir-k/docs/01-overview.md (formal methods expert)

Verdict: REVISE

Checks performed:

- Compared module names, constructors, configuration cells, entry points and gate emission against the K sources.
- Checked the base Rust addition dispatch at commit `92e8bdd3a97b61b229e38916e1b180de6f448dd5` using `git show`; the ordinary checkout is at a different commit.
- Compared every reported corpus and test count with directory contents and the six `2026-09-05b` receipts; all counts match.
- Inspected preprocessor mappings, runner result extraction and differential comparison logic.
- Confirmed all four compiled definitions identify the LLVM backend.
- Attempted the documented static-check command; `uv` failed before execution because the read-only filesystem prevents creating its cache lock. Runtime output was not reproduced.
- Evaluated the runner’s argument-parser declarations with the documented arguments; literal bracketed options produce exit code 2.
- Confirmed one top-level title, no em-dashes and no prohibited drafting vocabulary.
- Compared the reading guide with all fifteen required chapter names.

## Findings

### F1 blocking “What a green run establishes”

Claim: Line 130 says an `ok()` run with all verdicts holding establishes Rust witness agreement and satisfaction of every circuit instruction relation.

Evidence: Repository observation: `experiments/zkir-k/tools/zkir_run.py:298` executes K without invoking Rust. Actual agreement requires the separate comparison in `experiments/zkir-k/tools/diff_test.py:137`. Furthermore, `wiki/zkir/zkir-k-definition.md:59` explicitly limits meaningful final-memory gate evaluation to programs passing the static check. `experiments/zkir-k/semantics/zkir-vm.k:218` permits overwrites, and line 517 evaluates all gates against final memory.

Fix: State that a finished, non-generation run with `wf(P) = wfOk()`, status `ok()` and all verdicts holding satisfies the definition’s modeled instruction relations on that concrete witness. State separately that Rust agreement is observed only for program/preimage pairs actually compared by the differential harness.

### F2 blocking “The three entry points share the same rules”

Claim: Lines 77–83 describe generation mode as differing during execution only when a transcript runs out.

Evidence: Repository observation: `experiments/zkir-k/semantics/zkir-vm.k:491` records expected public transcript inputs instead of checking them. Lines 525–527 bypass transcript-consumption checks and record the computed commitment instead of checking it; ordinary execution performs these checks at lines 531–539.

Fix: Explain all three generation relaxations: default values for exhausted transcript reads, recording expected impact inputs, and recording the commitment while bypassing final transcript-consumption checks. Explicitly exclude generation runs from the trust statement.

### F3 major “Every production … JSON key of the type”

Claim: Line 16 says every syntax production has a symbol matching an instruction’s JSON operation or a type’s JSON key.

Evidence: Repository observation: `experiments/zkir-k/semantics/zkir-syntax.k:31` uses `symbol(Native)` for JSON type `Scalar<BLS12-381>`. The explicit translation appears in `experiments/zkir-k/tools/zkir_kast.py:35`. Function productions such as `encodedLen` at syntax line 47 have no explicit `symbol(...)` attribute.

Fix: Replace with: “Instruction constructors use symbols matching their JSON `op` names. The preprocessor maps JSON type names to K symbols through `TYPE_SYMBOLS` and constructs the surrounding program structure explicitly.”

### F4 blocking “emitting one gate per instruction”

Claim: The module table at line 49 describes uniform one-gate-per-instruction emission.

Evidence: Repository observation: `experiments/zkir-k/semantics/zkir-vm.k:184` emits one `guardGate` plus one `piGate` per impact operand; line 192 constructs those additional gates. Initialization also emits `bindGate` and optionally `commGate` at lines 143–148.

Fix: Replace with: “The configuration, run entry points and witness computation, with instruction-dependent gate emission and initialization gates.”

### F5 blocking “each returning vOk … vErr … vPanic”

Claim: Line 48 gives these as the return constructors for every off-circuit operation.

Evidence: Repository observation: `experiments/zkir-k/semantics/zkir-ops.k:14` defines `PairResult`. `intoCoordinatesV` at line 153 returns `pOk(Value, Value)` or `pErr(String)`; `bytes32IntoLowHighV` at line 198 also returns a pair.

Fix: Remove the universal return claim, or distinguish single-value operations returning `ValueResult` from pair-valued operations returning `PairResult`.

### F6 major malformed verdict representation

Claim: Diagram line 74 shows `verdict(gate(...), holds())` alongside bare `violated(msg)`, `synthErr(msg)`, `unknown(msg)` and `unsupported(msg)` as alternative entries of `<verdicts>`.

Evidence: Repository observation: `experiments/zkir-k/semantics/zkir-constraints.k:43` defines `verdict(Constraint, Outcome)`. Line 91 wraps every evaluated outcome in that constructor.

Fix: Show `verdict(C, O)` as the list entry and list the five alternatives for `O` separately.

### F7 blocking the full-run command is not executable as shown

Claim: Lines 85–92 present two commands that run from the repository root and describe their outputs.

Evidence: Repository observation: `PROGRAM.zkir` and `PREIMAGE.json` do not exist at the root. The parser declared at `experiments/zkir-k/tools/zkir_run.py:359` accepts `--ext` and `--checked`, not their bracketed forms. Executing those parser declarations with the shown arguments returns:

```text
error: unrecognized arguments: [--ext] [--checked]
```

Fix: Supply an executable example using an existing program and concrete preimage. Alternatively, label the second line explicitly as CLI usage syntax, explain substitutions and remove the claim that the literal line runs and prints a result.

### F8 blocking reading guide omits a required chapter

Claim: The reading guide covers the documentation set.

Evidence: Repository observation: lines 113–126 contain fourteen entries, covering chapters 02–15. `experiments/zkir-k/docs-surge-2026-09-05/briefs/01.md` requires all fifteen.

Fix: Add an entry for `01-overview.md`, covering scope, repository layout, validation overview and trust boundaries.

### F9 major depth exhaustion is described as a confirmed cause

Claim: Line 83 says the runner reports `depth-exhausted` when a depth bound was hit.

Evidence: Repository observation: `experiments/zkir-k/tools/zkir_run.py:311` checks whether computation remains. Line 315 selects `depth-exhausted` whenever a depth argument was supplied; it does not verify that the bound was reached.

Fix: State that residual computation is labeled `depth-exhausted` when a depth bound was supplied, otherwise `stuck`; this classification does not independently establish the stopping cause.

### F10 major “no successful run has a non-holding gate”

Claim: Lines 102–103 describe the non-holding-gate result without limiting its coverage within the comparison set.

Evidence: Repository observation: `experiments/zkir-k/tools/diff_test.py:227` records non-holding gates for primary attempts. Perturbed runs at lines 258–263 call `compare` but do not inspect `violations`; `compare` at lines 137–162 does not compare verdicts. The receipts reproduce this narrower counter.

Fix: State: “No successful primary attempt inspected by the gate counter had a non-holding gate; the counter does not include perturbed runs.”

### F11 major differential comparison and convergence are overstated

Claim: Line 107 says generation continues until transcript needs stabilize and describes comparison of status, error class, registers, public inputs and skips without distinguishing success from failure.

Evidence: Repository observation: `experiments/zkir-k/tools/diff_test.py:118` caps generation at eight passes. Lines 130–134 can return without establishing convergence at that cap. Error and panic comparisons return after error-class checking at lines 141–145; memory and public-vector comparisons occur only for successful runs.

Fix: Describe generation as up to eight passes, stopping early when no additional private/public-output transcript values are requested. State that failures compare status and error class; successful runs additionally compare register types and encodings, public inputs and skips.

## Coverage

- Definition, pinned surfaces, modeled behavior and exclusions: covered; trust qualifications need correction.
- Repository map and K module table: covered; gate-emission and operation-return descriptions need correction.
- Execution picture and checking pipeline with receipt counts: covered; verdict representation and harness coverage need correction.
- Reading guide for all fifteen chapters: partly covered; chapter 01 is missing.
- Trust statement: partly covered; missing static-check and execution-mode assumptions, and conflates K success with observed Rust agreement.
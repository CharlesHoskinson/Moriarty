Could not save `experiments/zkir-k/docs-surge-2026-09-05/audits/10-fm-codex.md`: this session permits no filesystem writes.

# Audit of 10-well-formedness-and-static-checks.md (formal methods expert)

Verdict: REVISE

Checks performed:

- Compared predicates, constructors, guards, error messages and quoted rules with the K sources.
- Checked the described load, preprocessing and witness-memory behavior against the pinned Rust sources.
- Evaluated all 63 corpus programs using the existing compiled interpreter through stdin/stdout: 63 matched expectations.
- Verified all seven negative examples, including the 37-byte immediate.
- Evaluated an extension program and counterexamples for diagnostic ordering and single assignment.
- Recomputed the reassignment and undefined-variable examples under `job` and `checkedJob`.
- Confirmed one top-level title, no em-dashes and no prohibited placeholder or drafting terms.
- Attempted both documented commands; `uv` stopped at lock-file creation because the filesystem is read-only. Direct interpreter checks above bypassed temporary files. Rust circuit synthesis was inspected, not executed.

## Findings

### F1 blocking — Extension checking is incorrectly documented as unsupported

Claim: Chapter lines 61, 68 and 132 describe a fourteen-line checker importing only ZKIR-WF, without extension symbols, and report that `check --ext` fails.

Evidence: **Repository observation:** `experiments/zkir-k/semantics/zkir-check.k:6` requires `zkir-ext.k`; line 10 imports ZKIR-EXT-SYNTAX. The file has 18 lines. `experiments/zkir-k/tools/zkir_kast.py:396` defines `--ext`, and line 399 passes it to `load_program`.

**Experiment observation:** Loading `experiments/zkir-k/corpus/midnight-zkir-2ffe2d1-tests/test_bytes_slice.zkir` with `ext=True`, converting it against the current compiled checker and evaluating it returned `wfOk`. This exercises extension types and instructions.

Fix: **Recommendation:** Replace the limitation with: “The checker supports the base surface and, with `--ext`, the extension surface.” Remove the obsolete first maintainer note and the fourteen-line count. In the second note, explain that `check_corpus.py` currently omits extension selection; the compiled checker itself supports it.

### F2 blocking — Single assignment is presented as a necessary-and-sufficient soundness condition

Claim: Chapter line 126 says the final-memory reading is sound “exactly when every register is written once.” Line 128 says verdicts are meaningful only for programs passing `wf`.

Evidence: **Repository observation:** `experiments/zkir-k/semantics/zkir-syntax.k:277` rejects repeated identifiers regardless of their values. `experiments/zkir-k/semantics/zkir-vm.k:218` overwrites memory. `experiments/zkir-k/semantics/zkir-constraints.k:293` checks copy-value equality, and line 189 returns `holds()` when values match.

**Experiment observation:** Starting from `reassignment.zkir`, replace its second instruction in memory with another `copy %a -> %b`. With `%a = 1`, raw `job` finishes `ok`, both copy gates return `holds`, and `checkedJob` rejects reassignment. Both instruction-time values equal the final value despite the repeated write.

**Source fact:** `repos/_build/ledger-92e8bdd3/zkir-v3/src/ir_vm.rs:769` checks inserted values against the final witness; line 772 detects value inequality, not repeated assignment itself.

Fix: **Recommendation:** State the narrower guarantee: “Definition before use and single assignment ensure that successfully computed register values remain available unchanged for final-memory gate evaluation. Reassignment can invalidate this correspondence, although repeated writes of the same value need not.” Remove the necessary-and-sufficient wording and avoid implying that `wf` establishes general circuit soundness.

### F3 major — The ordering table incorrectly separates variable and immediate checks

Claim: Chapter lines 7–16 specify a fixed order with all variable-definition checks at step 3 and immediate-range checks at step 4.

Evidence: **Repository observation:** `experiments/zkir-k/semantics/zkir-syntax.k:282` defines one recursive traversal of the operand list. Lines 284–290 inspect each operand in sequence, selecting the applicable check.

**Experiment observation:** A directly constructed program containing `add(imm(r), var("%missing"), "%out")`, with no inputs, reports `immediate out of field range: …` before reaching the undefined variable.

Fix: **Recommendation:** Combine steps 3 and 4 into one operand-validation phase: “Traverse `reads(I)` in order; check definition membership for variables and field range for immediates. Report the first failing operand.” Keep arity checks and writes as subsequent phases.

## Coverage

- **Partly:** `wf`, constituent predicates, `reads`/`writes`, arities and bit bounds are covered; diagnostic ordering needs F3.
- **Partly:** ZKIR-CHECK, CLI and corpus expectations are covered; extension support needs F1. The distinction between script expectations and provenance manifests is correct.
- **Covered:** Static rejection versus crate behavior, raw versus checked execution, and remaining runtime checks.
- **Covered:** All seven negative programs, failing predicates and diagnostics.
- **Partly:** Final-memory motivation and the original reassignment example are correct; the guarantee needs F2.
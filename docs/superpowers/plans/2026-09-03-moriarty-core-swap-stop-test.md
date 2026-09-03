# Moriarty Core Atomic-Swap Stop-Test Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use
> superpowers:subagent-driven-development (recommended) or
> superpowers:executing-plans to implement this plan task-by-task. Steps use
> checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement E00 as a finite Core interpreter, generated Compact swap,
independent backend model, and 1,000-trace translation certificate.

**Architecture:** Immutable Python data types define the normative experimental
Core. A shape-restricted compiler emits fixed-state Compact and a transition
manifest. An independent manifest machine and deterministic trace generator
compare observable behavior without importing the Core interpreter.

**Tech Stack:** Python 3.13+, pytest, standard-library JSON and SHA-256, Compact
compiler 0.34.100, Compact language 0.26.0, ZKIR 3 mock compiler.

## Global Constraints

- Keep every contract path finite and statically bounded.
- Use no dynamic collection or loop in generated Compact.
- Constrain every private witness before the first state or value effect.
- Preserve per-token conservation and non-negative internal accounts.
- Label differential agreement as S3 evidence, not formal proof.
- Use the pinned Compact and ZKIR toolchain from the existing escrow experiment.
- Write each behavioral test before its production implementation.

---

### Task 1: Core syntax and transaction semantics

**Files:**

- Create: `moriarty/__init__.py`
- Create: `moriarty/core.py`
- Create: `tests/test_core_semantics.py`

**Interfaces:**

- Consumes: no project code.
- Produces: immutable Core constructors, `State`, `TransactionResult`,
  `reduce_to_quiescence()`, and `compute_transaction()`.

- [ ] **Step 1: Write failing semantics tests**

  Test exact deposit matching, deadline precedence, bounded choices, atomic
  multi-payment reduction, deterministic close refunds, partial-payment
  warnings, and input rejection.

- [ ] **Step 2: Verify the tests fail for the missing module**

  Run: `uv run pytest tests/test_core_semantics.py -q`

  Expected: collection error for missing `moriarty.core`.

- [ ] **Step 3: Implement the minimum immutable Core**

  Define frozen dataclasses and tagged unions for `Close`, `Pay`, `If`, `When`,
  `Deposit`, `Choice`, `Constant`, `ChoiceEquals`, `DepositInput`, and
  `ChoiceInput`. Keep account mutation inside copied dictionaries.

- [ ] **Step 4: Verify the semantics tests pass**

  Run: `uv run pytest tests/test_core_semantics.py -q`

  Expected: all tests pass with no warning output.

### Task 2: Canonical swap and static bounds

**Files:**

- Create: `moriarty/swap.py`
- Create: `moriarty/bounds.py`
- Create: `tests/test_swap_bounds.py`

**Interfaces:**

- Consumes: Core constructors and transaction functions from Task 1.
- Produces: `SwapParameters`, `canonical_swap()`, `Bounds`, and
  `analyze_bounds()`.

- [ ] **Step 1: Write failing canonical-swap tests**

  Test success, cancellation, timeout in each non-terminal phase, conservation,
  non-negative balances, three-input maximum, two-payment maximum, and finite
  syntax bounds.

- [ ] **Step 2: Verify the tests fail for missing swap and bounds modules**

  Run: `uv run pytest tests/test_swap_bounds.py -q`

  Expected: collection error for missing modules.

- [ ] **Step 3: Build the exact finite Core term**

  Build two nested deposits followed by one bounded choice. Lower choice `1` to
  two sequential `Pay` terms and choice `0` to `Close`. Use the same absolute
  deadline for each `When`.

- [ ] **Step 4: Implement structural bound analysis**

  Traverse every branch. Return maxima for external inputs, reductions,
  payments, accounts, deadline, and syntax nodes. Reject no node silently.

- [ ] **Step 5: Verify the swap and bound tests pass**

  Run: `uv run pytest tests/test_swap_bounds.py -q`

  Expected: all tests pass.

### Task 3: Restricted Compact lowering and backend model

**Files:**

- Create: `moriarty/compact.py`
- Create: `moriarty/backend.py`
- Create: `tests/test_compact_lowering.py`

**Interfaces:**

- Consumes: `SwapParameters`, canonical Core, and `Bounds`.
- Produces: `lower_swap() -> Lowering`, generated Compact text, an artifact
  manifest, and `BackendMachine.apply()`.

- [ ] **Step 1: Write failing lowering tests**

  Test the exact accepted Core shape. Test rejection after any AST mutation.
  Scan generated source for collections, loops, recursion, cross-contract calls,
  and unconstrained witnesses. Test manifest effects and disclosure records.

- [ ] **Step 2: Verify the tests fail for missing lowering modules**

  Run: `uv run pytest tests/test_compact_lowering.py -q`

  Expected: collection error for missing modules.

- [ ] **Step 3: Emit fixed-state Compact**

  Emit sealed parties, authority hashes, tokens, quantities, deadline, and a
  finite phase enum. Emit deposit, decision, and expiry circuits. Hash each
  authority witness and compare it before an effect.

- [ ] **Step 4: Emit a canonical artifact manifest**

  Record Core hash, Compact hash, parameter types, witnesses, disclosures,
  entry-point effects, bounds, toolchain tuple, and expected ZKIR files.

- [ ] **Step 5: Implement the independent backend machine**

  Interpret only the manifest transition table. Do not import
  `moriarty.core`. Return the same observable result schema as the certificate
  comparison adapter.

- [ ] **Step 6: Verify lowering tests pass**

  Run: `uv run pytest tests/test_compact_lowering.py -q`

  Expected: all tests pass.

### Task 4: Differential certificate and preserved artifacts

**Files:**

- Create: `moriarty/certificate.py`
- Create: `tests/test_translation_certificate.py`
- Create: `experiments/moriarty-core-swap/generate.py`
- Generate: `experiments/moriarty-core-swap/swap.compact`
- Generate: `experiments/moriarty-core-swap/artifact-manifest.json`
- Generate: `experiments/moriarty-core-swap/translation-certificate.json`

**Interfaces:**

- Consumes: Core interpreter, restricted lowerer, and backend machine.
- Produces: `generate_traces()`, `validate_translation()`, and immutable research
  artifacts.

- [ ] **Step 1: Write failing trace and certificate tests**

  Require at least 1,000 unique ordered traces, every transition and rejection
  class, deadline boundaries, every terminal phase, zero divergence, and stable
  canonical hashes.

- [ ] **Step 2: Verify the tests fail**

  Run: `uv run pytest tests/test_translation_certificate.py -q`

  Expected: collection error for missing certificate module.

- [ ] **Step 3: Implement deterministic trace generation**

  Use a fixed seed. Build valid prefixes and mutate parties, tokens, quantities,
  choices, times, order, repetitions, and terminal calls. Deduplicate exact
  ordered payload sequences.

- [ ] **Step 4: Compare independent observations**

  Normalize phase, accounts, payments, choices, warnings, acceptance, and
  rejection code. Preserve the first divergence and fail the run when any
  divergence exists.

- [ ] **Step 5: Generate the artifacts**

  Run: `uv run python experiments/moriarty-core-swap/generate.py`

  Expected: three files with at least 1,000 traces and zero divergence.

- [ ] **Step 6: Verify certificate tests pass**

  Run: `uv run pytest tests/test_translation_certificate.py -q`

  Expected: all tests pass.

### Task 5: Compact and ZKIR toolchain gate

**Files:**

- Generate: `experiments/moriarty-core-swap/output/`
- Create: `experiments/moriarty-core-swap/toolchain-results.json`
- Create: `experiments/moriarty-core-swap/README.md`

**Interfaces:**

- Consumes: generated `swap.compact` and pinned local compilers.
- Produces: compiler artifacts, ZKIR metrics, mock-compiler results, and exact
  hashes.

- [ ] **Step 1: Compile generated Compact**

  Run:

  ```bash
  /nix/store/5h37yza1bii76f71wjpnhpdjs8m2a7pf-compactc/bin/compactc \
    --feature-zkir-v3 --skip-zk \
    experiments/moriarty-core-swap/swap.compact \
    experiments/moriarty-core-swap/output
  ```

  Expected: TypeScript, compiler metadata, and ZKIR 3 artifacts exist.

- [ ] **Step 2: Run the local ZKIR mock compiler**

  Use the same `mock-compile-many` binary and parameters recorded by the escrow
  experiment. Preserve each circuit's result, rows, `k`, byte size, instruction
  count, private-input count, and SHA-256 digest.

- [ ] **Step 3: Write the toolchain result and experiment README**

  State exact tested predicates. List proof generation, deployment, fee, audit,
  and compiler-correctness claims as not established.

### Task 6: Research integration and verification

**Files:**

- Modify: `wiki/benchmarks.md`
- Modify: `wiki/decision.md`
- Modify: `wiki/open-questions.md`
- Modify: `wiki/index.md`
- Modify: `evidence/source-inventory.csv`
- Create: `evidence/moriarty-goal-completion-matrix-2026-09-03.csv`

**Interfaces:**

- Consumes: all experiment outputs.
- Produces: updated decision evidence and an explicit remaining-work matrix.

- [ ] **Step 1: Update only claims supported by the experiment**

  Record E00 as passed, failed, or partially passed per predicate. Keep E19 real
  proof generation and live deployment open unless new evidence closes them.

- [ ] **Step 2: Update the completion matrix**

  Give each required workstream and experiment one status from `reproduced`,
  `implemented`, `specified-only`, `blocked`, or `not-started`. Link evidence.

- [ ] **Step 3: Run focused and full verification**

  Run:

  ```bash
  uv run pytest -q
  npx --yes markdownlint-cli2 '**/*.md' '#repos/**' '#corpus/**' '#raw/**'
  git diff --check
  ```

  Expected: all tests pass, markdown has zero issues, and Git reports no
  whitespace errors.

- [ ] **Step 4: Commit the completed increment**

  Stage only the E00 files and their research integration. Preserve unrelated
  worktree changes.

  Commit message: `experiment: validate Moriarty atomic-swap lowering`

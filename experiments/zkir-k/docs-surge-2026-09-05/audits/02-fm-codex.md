I could not write `experiments/zkir-k/docs-surge-2026-09-05/audits/02-fm-codex.md` because this session’s filesystem is read-only. The report follows.

# Audit of 02-getting-started.md (formal methods expert)

Verdict: REVISE

Checks performed:

- Compared prerequisites with `pyproject.toml` and `check_k_toolchain.sh`.
- Inspected all four main modules and confirmed the compiled directories exist; did not run kompile.
- Checked the example programs, manifest preimage, entry points, gate emission, commitment handling and assertion rules.
- Ran the installed Python runner’s help command; its output matches the chapter.
- Ran both Rust oracles with preimages supplied through stdin. The successful examples’ memory, public inputs and skips match the chapter.
- Confirmed the negative examples’ Rust error classes and the base preprocessor’s rejection of `Bool`.
- Executed `encode_value` and `type_string` on a one-byte `bytesV` value.
- Compared all four suite summaries with the saved `2026-09-05b` receipts.
- Inspected suite implementations and differential flags; did not run the full differential suite.
- Confirmed one top-level title, no em-dashes and no prohibited placeholder or drafting terminology.

Execution limitation: uv failed with `Read-only file system` while acquiring its cache lock. Direct runner execution failed because pyk could not create a temporary file. Fresh K runs and suite results were therefore not reproduced. `divergence_tests.py` also rewrites corpus files and was not executed.

## Findings

### F1 major “The two names agree except for `bytes32V`”

Claim: Chapter line 225 identifies `bytes32V` as the sole exception to agreement between memory `variant` and `type` labels.

Evidence: Repository observation: [zkir_run.py:207](/home/charl/Moriarty/.worktrees/zkir-k-semantics/experiments/zkir-k/tools/zkir_run.py:207) returns `Bytes(n)` for `bytesV`, while [zkir_run.py:230](/home/charl/Moriarty/.worktrees/zkir-k-semantics/experiments/zkir-k/tools/zkir_run.py:230) returns variant `Bytes`. Experiment observation: a one-byte value produced `('Bytes', [5])` and `Bytes(1)`.

Fix: Replace the exception sentence with: “On the extension surface, byte strings have variant `Bytes` and type `Bytes(n)`, including `bytes32V` at length 32. On the base surface, `bytes32V` has variant and type `Bytes32`.”

### F2 major `depth-exhausted` implies the rewrite limit was reached

Claim: Chapter line 225 says this status occurs when `--depth N` cuts the run after `N` rewrite steps.

Evidence: Repository observation: [zkir_run.py:311](/home/charl/Moriarty/.worktrees/zkir-k-semantics/experiments/zkir-k/tools/zkir_run.py:311) checks whether `<k>` is empty, then selects `depth-exhausted` whenever `depth is not None`. It does not inspect the number of executed steps or distinguish an earlier stuck configuration.

Fix: Replace that clause with: “A residual computation is reported as `stuck` without a depth limit and `depth-exhausted` with one. The latter label does not establish that the limit was reached.”

### F3 minor Missing K source locators for the assertion explanation

Claim: Chapter line 498 explains the off-circuit and in-circuit assertion predicates but names only `tools/divergence_tests.py`.

Evidence: Repository observation: common-brief hard rule 1 requires the K file and symbol when describing a rule. The relevant definitions are `#exec(assert(C))` in `semantics/zkir-vm.k:286`, `asBool` in `semantics/zkir-ops.k:350`, and `eval(gate(assert(C)), …)` in `semantics/zkir-constraints.k:218`.

Fix: Add these file and symbol references to the paragraph. The explanation is correct for the Native-valued example.

## Coverage

- Prerequisites: covered.
- Four compilation commands, duration and output directories: covered; compilation not performed.
- First program, preimage and output explanation: covered; memory-label correction required.
- `--checked` and extension execution: covered.
- Handmade-negative and divergence failures: covered; assertion locators required.
- Check suites and filtered differential execution: covered; summary totals match saved receipts, fresh suites not reproduced.
- Common problems and fixes: covered.
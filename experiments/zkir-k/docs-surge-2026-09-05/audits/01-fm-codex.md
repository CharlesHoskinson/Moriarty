The audit verdict is **REVISE**. I could not save `experiments/zkir-k/docs-surge-2026-09-05/audits/01-fm-codex.md` because this session’s filesystem is read-only. The report follows.

# Audit of experiments/zkir-k/docs/01-overview.md (formal methods expert)

Verdict: REVISE

Checks performed:

- Compared module names, constructors, entry points, constraint emission, generation rules, and verdict evaluation with the K sources.
- Inspected the pinned Rust preprocessing, circuit, addition, and extension declarations.
- Verified corpus counts and all six reported receipt totals.
- Checked differential comparison predicates and generation stopping conditions.
- Recomputed the example’s eight emitted constraints from its seven instructions and `bindGate(0)`.
- Ran the runner’s help through the installed interpreter; confirmed `--depth`.
- Attempted the static-check and example-run commands. Execution was blocked by read-only cache and temporary directories; their runtime outputs were not reproduced.
- Confirmed one top-level title, no em-dashes, and no prohibited drafting language or placeholders.

## Findings

### F1 blocking “the command line has no depth option”

Claim: Chapter line 85 says depth bounds are available through `Runner.run` but unavailable through the command line.

Evidence: Repository observation: `experiments/zkir-k/tools/zkir_run.py:366` declares `--depth`; line 377 passes it into `Runner.run`. Experiment observation: running `PYTHONDONTWRITEBYTECODE=1 .venv/bin/python experiments/zkir-k/tools/zkir_run.py --help` exits successfully and displays `--depth DEPTH`.

Fix: Replace the parenthetical with “the command line exposes this bound as `--depth N`.”

### F2 major Operation helpers have different result sorts and do not write registers themselves

Claim: Chapter line 48 says each operation function returns `ValueResult`, except two functions that “write two registers” and return `PairResult`.

Evidence: Repository observation: `experiments/zkir-k/semantics/zkir-ops.k:130` declares `constrainEqV(Value, Value)` with result sort `CheckResult`; lines 131–133 return `cOk()` or `cErr(...)`. Line 338 similarly declares `checkBits` with result sort `CheckResult`. The coordinate helper at line 153 is a pure function returning `PairResult`. Register updates occur in `experiments/zkir-k/semantics/zkir-vm.k:220`, in the rule for `#put2`.

Fix: Describe these as value-level operation and conversion helpers with operation-specific result sorts. State that `intoCoordinatesV` and `bytes32IntoLowHighV` return pairs which the VM writes through `#put2`.

### F3 major Incorrect hash function names

Claim: Chapter line 47 lists `sha256` and `keccak256` as functions in `zkir-hash.k`.

Evidence: Repository observation: `experiments/zkir-k/semantics/zkir-hash.k:161` declares `sha256Bytes(Bytes)`; line 231 declares `keccak256Bytes(Bytes)`. The similarly named `keccak256` constructor in `experiments/zkir-k/semantics/zkir-syntax.k:108` is an instruction with three arguments, not the byte-hash function.

Fix: Replace the row’s function list with `poseidonHash`, `hashToCurve`, `sha256Bytes`, and `keccak256Bytes`.

### F4 blocking Unqualified claim of exact Rust correspondence

Claim: Chapter line 22 says the VM runs programs “exactly as `preprocess` does,” including failure behavior and recorded cells.

Evidence: Repository observation: `experiments/zkir-k/tools/diff_test.py:137–162` compares status and error class on failures, and register types/encodings, public inputs, and skip vectors on success. It does not compare failure-state cells. `experiments/zkir-k/review-2026-09-05/CONSOLIDATED.md:223–224` explicitly records that reproducing exact partial memory at failure was not adopted because the crate does not expose it. Chapter line 139 itself limits established correspondence to compared program/preimage pairs.

Inference: The unconditional wording promises stronger correspondence than the recorded checks establish.

Fix: Replace “runs the program exactly as `preprocess` does” with “models the witness computation of `preprocess`, including the implemented runtime checks and error/panic cases.” State that correspondence is checked for the compared observables and tested pairs, and that exact failure-state correspondence is not established.

## Coverage

- Definition, pinned surfaces, modeled behavior, and exclusions: covered; qualify correspondence as in F4.
- Repository map and K module table: covered; correct F2 and F3.
- Execution diagram and checking pipeline with receipt-backed numbers: covered; correct F1. Runtime examples were not reproduced in this sandbox.
- Reading guide for all fifteen chapters: covered.
- Trust statement: partly; the final section gives appropriate limits, but the earlier exact-correspondence claim contradicts those limits.
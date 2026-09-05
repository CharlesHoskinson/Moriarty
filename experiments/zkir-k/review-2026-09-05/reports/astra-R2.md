VERDICT: BLOCKED — K accepts programs rejected by the pinned Rust VM, rejects valid Rust hash executions, and its runner can report a stuck computation as successful.

## 2. Findings

Locators are relative to the repository unless prefixed below:

- `R/` = `/home/charl/Moriarty/repos/_extracts/ledger9-92e8bdd3-zkir-v3-src/zkir-v3/src/`
- `L/` = `/home/charl/Moriarty/repos/_build/ledger-92e8bdd3/`

The build checkout reports commit `92e8bdd3a97b61b229e38916e1b180de6f448dd5`. Findings distinguish source facts, repository observations, executed observations, and recommendations.

### 1. Blocker — A stuck VM is reported as `status: ok`

**Locations:** `experiments/zkir-k/tools/zkir_run.py:274`, `experiments/zkir-k/tools/zkir_run.py:281`, `experiments/zkir-k/tools/zkir_run.py:295`; `experiments/zkir-k/semantics/zkir-vm.k:302`.

**Repository observation:** The runner derives success solely from `<status>`. It extracts the remaining `<k>` computation but never checks that it is empty. The VM has a `divModPowerOfTwo` execution rule only for exactly two output identifiers, with no runtime fallback for another arity.

**Source fact:** `R/ir_vm.rs:395` checks output arity before operand resolution and returns `DivModPowerOfTwo requires exactly 2 outputs`.

**Executed observation:** This instruction, in an otherwise empty program, produces the following:

```json
{"op":"div_mod_power_of_two","val":"0x01","bits":1,"outputs":["%q"]}
```

- Rust: `error`, `DivModPowerOfTwo requires exactly 2 outputs`.
- K runner: `ok`, empty memory, no verdicts.
- K computation remains at `#exec(divModPowerOfTwo(...)) ~> .Instrs ~> #verdicts`.

**Why it matters:** Callers of `Runner.run`, `run_file`, or the CLI can interpret failure to execute as successful preprocessing. A depth-limited invocation has the same classification problem.

**Recommendation:** Require an empty computation and completed finalization before returning `ok`. Represent stuck and depth-exhausted executions separately. Add a runtime arity-error rule preserving Rust’s check order.

### 2. Major — `test_eq` incorrectly accepts Jubjub scalars

**Locations:** `experiments/zkir-k/semantics/zkir-ops.k:104`; `experiments/zkir-k/semantics/zkir-vm.k:196`.

**Repository observation:** `testEqV` accepts every pair with the same `IrType`.

**Source fact:** `R/ir_instructions/eq.rs:42` enumerates supported pairs. It includes Native, Bytes32, all four point types, and the six foreign base/scalar types. It does **not** include `JubjubScalar`; that pair reaches the unsupported-operation error at line 61. `R/ir_vm.rs:342` propagates this error without inserting the destination.

**Executed observation:** Declare `%s : Scalar<Jubjub>`, supply raw input `["1"]`, and execute:

```json
{"op":"test_eq","a":"%s","b":"%s","output":"%eq"}
```

K returns `ok` and writes Native `1` to `%eq`. Rust returns:

```text
Unsupported test_eq: JubjubScalar == JubjubScalar
```

K additionally reports a synthesis error for the emitted gate, incorrectly presenting this example as an off-circuit/in-circuit divergence.

**Why it matters:** This changes acceptance on a well-formed program with a valid preimage. Subsequent instructions and commitment checks execute only in K.

**Recommendation:** Give witness equality its own explicit Rust dispatch predicate. Reject JubjubScalar pairs; retain their support in `constrain_eq` and `cond_select`, whose off-circuit implementations differ.

### 3. Major — The execution entry point bypasses WF, while runtime rules depend on it for Rust’s bit limits

**Locations:** `experiments/zkir-k/semantics/zkir-vm.k:87`, `experiments/zkir-k/semantics/zkir-vm.k:301`, `experiments/zkir-k/semantics/zkir-vm.k:310`; `experiments/zkir-k/tools/zkir_run.py:271`; `experiments/zkir-k/tools/zkir_kast.py:355`.

**Repository observation:** Importing `ZKIR-WF` does not invoke `wf`. Neither `job`, `Runner.run_file`, nor `load_program` calls it. The separate checker invokes it at `experiments/zkir-k/semantics/zkir-check.k:13`. Nevertheless, split execution explicitly assumes its bit limit was checked statically.

**Source fact:** Rust checks `bits <= 248` inside both preprocessing arms, before resolving operands: `R/ir_vm.rs:399` and `R/ir_vm.rs:418`.

**Executed observations:**

| Instruction | K | Rust |
|---|---|---|
| `div_mod_power_of_two(1, bits=249, outputs=[q,r])` | `ok`; `q=0`, `r=1` | `Excessive bit count` |
| `reconstitute_field(divisor=0, modulus=0, bits=249)` | `ok`; result `0` | `Excessive bit count` |

Both K runs terminate with no gate violations.

The bypass also contradicts the claim at `wiki/zkir/zkir-k-definition.md:56` that reassignment is rejected before a run. Executing `copy(1,%a); copy(2,%a)` through the runner succeeds and leaves `%a=2`, matching Rust preprocessing but violating the first K copy gate when evaluated against final memory.

**Why it matters:** The documented execution domain and actual execution domain differ. Static checking in `divergence_tests.py:164` masks this problem; the ordinary differential runner does not perform that check.

The consequences of enforcing WF must also be stated accurately:

- Rust overwrites duplicate input names, existing destinations, and repeated destinations within multi-output instructions.
- Rust can accept an inactive `impact` containing an undefined operand; `reads(impact(...))` checks that operand unconditionally at `zkir-syntax.k:142`.
- Checking every instruction before execution changes error precedence for invalid instructions following an earlier runtime failure.
- Final-memory gate evaluation needs a single-assignment assumption or instruction-specific bindings; merely accepting Rust’s overwrites does not preserve the intended gate interpretation.

**Recommendation:** Restore Rust’s runtime arity and bit checks regardless of WF. Define an explicit checked execution entry point if single assignment is required, and distinguish its restricted domain from unrestricted `IrSource::preprocess` compatibility. Test both entry points.

### 4. Major — Standard-hash alignment decoding rejects Rust-successful preimages

**Locations:** `experiments/zkir-k/semantics/zkir-ops.k:215`, `experiments/zkir-k/semantics/zkir-ops.k:221`; `experiments/zkir-k/semantics/zkir-vm.k:351`.

Two separate restrictions affect both `persistent_hash` and `keccak256`.

**Repository observation — trailing fields:** K requires the operand list to be exhausted when the alignment ends.

**Source fact:** `L/transient-crypto/src/fab.rs:337` calls `parse_field_repr_inner` and returns the parsed value without checking the remaining field slice. Rust still resolves and converts every instruction operand first, at `R/ir_vm.rs:488`, but unused Native suffix elements are not included in the hash.

**Executed observation:**

```json
{"op":"persistent_hash","alignment":[],"inputs":["0x01"],"output":"%h"}
```

Rust succeeds and hashes the empty byte string. K returns `Inputs did not match alignment`.

**Repository observation — options:** K rejects every option segment as unsupported.

**Source fact:** `L/transient-crypto/src/fab.rs:318` accepts option segments, consumes a `u16` selector, parses the selected alternative, and checks zero field padding to the maximum alternative width. Binary serialization handles the selected value and byte padding at `L/transient-crypto/src/fab.rs:273`. The in-circuit rejection at `R/ir_vm.rs:114` is a separate behavior.

**Executed observation:**

```json
{
  "op":"persistent_hash",
  "alignment":[{"tag":"option","value":[[]]}],
  "inputs":["0x00"],
  "output":"%h"
}
```

Rust succeeds and writes the digest. K returns `unsupported: alignment option segment`.

**Why it matters:** Both counterexamples use valid instruction syntax and Native operands. These are VM acceptance differences, not merely absent tests. Rejecting options off-circuit also hides a real off-circuit/in-circuit distinction.

**Recommendation:** Mirror Rust’s alignment parser: permit unused Native suffix elements and implement option selection and padding. Keep the in-circuit option rejection in the constraint layer. Add positive and negative option cases, including invalid selectors and nonzero padding.

### 5. Major — Differential “agreement” ignores every error message and several relevant observables

**Locations:** `experiments/zkir-k/tools/diff_test.py:98`, `experiments/zkir-k/tools/diff_test.py:172`; `experiments/zkir-k/tools/zkir_run.py:189`, `experiments/zkir-k/tools/zkir_run.py:287`.

**Repository observation:** `compare` immediately returns an empty difference list whenever both statuses are non-`ok`. Error text is not compared loosely; it is not compared at all. The prefix comparison at line 173 affects printed notes only.

**Executed observation:**

```python
compare(
    {"status": "error", "error": "Failed direct assertion"},
    {"status": "error", "error": "Expected communications commitment"},
)
```

returns `[]`.

**Source facts:** These are distinct failures at `R/ir_vm.rs:337` and `R/ir_vm.rs:213`. The precedence differences in finding 6 likewise identify different failed predicates.

Additional repository observations:

- Successful memory comparisons use Python’s `encode_value`, not K’s `encodeValue`.
- `Runner` extracts cursors, but `compare` ignores them.
- `Runner` does not extract `<outputs>`.
- `compare` ignores the remaining computation and verdicts.
- The Rust oracle does not expose cursors, the output accumulator, or partial state on errors: `L/zkir-oracle/src/main.rs:43` and `L/zkir-oracle/src/main.rs:120`.

**Why it matters:** The reported agreement count cannot establish error precedence or full VM-state correspondence. A wrong K encoding can remain invisible when a program neither executes `encode` for that value nor commits its output. Wrong output accumulation can remain invisible when communications commitment is disabled.

**Recommendation:** Compare structured error categories and relevant positions/indices, normalizing only documented formatting differences. Require terminal execution. Expose and compare outputs and K-computed encodings; instrument Rust separately for any claimed cursor or partial-state correspondence. Report status-only error agreement separately from successful-state agreement.

### 6. Minor — Batch resolution changes Rust’s error precedence

**Locations:** `experiments/zkir-k/semantics/zkir-vm.k:310`, `experiments/zkir-k/semantics/zkir-vm.k:324`, `experiments/zkir-k/semantics/zkir-vm.k:335`, `experiments/zkir-k/semantics/zkir-vm.k:413`, `experiments/zkir-k/semantics/zkir-vm.k:437`; `experiments/zkir-k/semantics/zkir-ops.k:253`.

**Repository observation:** Several rules resolve all operands before checking earlier values. Rust interleaves resolution and conversion/checking for these operations.

**Executed observations:**

| Program fragment | Rust’s first error | K’s first error |
|---|---|---|
| `transient_hash([Bytes32-value, %missing])` | Cannot convert Bytes32 to Native | Missing `%missing` |
| `less_than(8, %missing, bits=3)` | `8` violates the bit bound | Missing `%missing` |
| `reconstitute_field(divisor=%missing, modulus=2, bits=1)` | `2` violates the modulus bound | Missing `%missing` |
| Output signature `[]`, instruction `output([%missing])` | Output arity mismatch | Missing `%missing` |
| Output signature `[Native,Native]`, operands `[Bytes32-value,%missing]` | Type mismatch at position 0 | Missing `%missing` |

**Source facts:** The corresponding ordering is explicit at `R/ir_vm.rs:432`, `R/ir_vm.rs:454`, `R/ir_vm.rs:468`, and `R/ir_vm.rs:625`. The hash pattern also occurs in persistent hashing, Keccak and hash-to-curve; active impact converts operands individually at `R/ir_vm.rs:519`.

A further source-level difference: `checkBits` reports an excessive bound before checking the value’s type. Rust’s helper converts to Native first, then checks the bound (`R/ir_vm.rs:255`).

**Why it matters:** These inputs are reachable through the actual unchecked runner. The selected failure predicate differs even when both executions return an error. The current harness classifies all these examples as agreement.

**Recommendation:** Implement sequential checked resolution. For output, check arity first, then resolve, type-check and append each position. For reconstruction, fully check the modulus before touching the divisor. For hashes and impact, resolve and convert each operand before advancing.

### 7. Minor — Error snapshots discard completed work and introduce work Rust never reaches

**Locations:** `experiments/zkir-k/semantics/zkir-vm.k:87`, `experiments/zkir-k/semantics/zkir-vm.k:107`, `experiments/zkir-k/semantics/zkir-vm.k:113`, `experiments/zkir-k/semantics/zkir-vm.k:413`; `experiments/zkir-k/semantics/zkir-values.k:245`.

**Repository observation:** Input decoding builds all bindings before installing any. Active impact converts its entire operand list before updating the public-input vector or cursor. PI seeding runs even after input decoding has failed.

**Executed observations and source-derived Rust states:**

- Declare `%a : Native`, then `%b : Bytes32`, with raw inputs `["7"]`. K reports the correct shortage index but retains empty memory and seeds the binding PI. Rust inserts `%a=7` before discovering the shortage and returns before creating `pis` (`R/ir_vm.rs:191`, `R/ir_vm.rs:213`).
- Execute active `impact([7, Bytes32-value])`. K retains only the binding PI and `pubInIdx=0`. Rust pushes `7` and increments the cursor before the second operand fails; it has not yet appended a skip entry (`R/ir_vm.rs:518`).
- With communications commitment required but absent, K inserts a synthetic commitment PI of `0` before reporting the error (`zkir-vm.k:116`). Rust’s fallible lookup prevents that push (`R/ir_vm.rs:215`).
- Output batch resolution similarly loses an already valid output prefix when a later operand is undefined (`R/ir_vm.rs:633` versus `zkir-vm.k:437`).

**Evidence limit:** Rust’s partial states above follow directly from statement order; the existing oracle returns only the error, so those snapshots were not observed through its JSON output.

**Why it matters:** The K configuration is presented as the frozen witness at its first failure. These snapshots are not that state. They also change subsequent constraint diagnostics.

**Recommendation:** Preserve successful prefixes using sequential state transitions and prevent witness PI seeding after decode failure. If error configurations are intentionally abstract, state that partial memory, PIs, cursors and outputs have no Rust correspondence after failure.

### 8. Minor — Panic normalization loses an observable failure class

**Locations:** `experiments/zkir-k/semantics/zkir-vm.k:378`, `experiments/zkir-k/semantics/zkir-vm.k:388`, `experiments/zkir-k/semantics/zkir-vm.k:399`; `experiments/zkir-k/semantics/zkir-values.k:160`.

**Repository observation:** Short transcripts and invalid Bytes32 limbs become ordinary K errors. The source comments acknowledge this normalization.

**Source facts:** Rust slices transcripts before incrementing their cursors (`R/ir_vm.rs:359`, `R/ir_vm.rs:378`) and uses assertions for Bytes32 limb bounds (`R/ir_instructions/encode.rs:155`, `R/ir_instructions/encode.rs:159`).

**Executed observations:**

- An unguarded Native public input with an empty transcript causes Rust process exit 101; K returns an ordinary error and advances `pubOutIdx` to 1.
- A Bytes32 raw input `[0,256]` causes Rust process exit 101; K returns `Bytes32 decoding assertion failed`.

**Why it matters:** “Returns an error” and “panics” are different preprocessing outcomes. Short transcripts additionally produce a K cursor update that Rust never executes.

**Recommendation:** Introduce a distinct panic outcome, preserving the pre-panic cursor. Alternatively, explicitly define the correspondence predicate to normalize panic and error, while retaining separate regression assertions for Rust’s actual behavior. This is a documented abstraction, not a newly discovered Rust defect.

## 3. Coverage gaps

**Repository observations:**

- No shipped regression covers the JubjubScalar `test_eq` rejection, the 249–254 split/reconstruction interval through `Runner`, terminal-state classification, or the two accepted hash-alignment shapes above.
- Positive typed input generation does not systematically test unsupported operand pairs. `gen_handmade.py:25` exercises selected successful pairs; `zkir_values.py:106` generates valid encodings.
- The differential perturbation changes only `inputs[0]`, and only after both sides succeed (`diff_test.py:179`). It does not independently corrupt transcript elements, truncate or extend transcripts, mutate the commitment/opening, or target later raw-input limbs.
- Generation constructs impact inputs and commitments from K itself (`diff_test.py:79`). Rust acceptance still provides an independent check on successful comparisons, but this does not test the rejection boundaries of those generated values.
- Eight generation passes are a limit, not a checked convergence condition (`diff_test.py:79`, `diff_test.py:95`). Native generation uses only `0,1,2,7,255`, with repeated weight on `1` (`zkir_values.py:148`).
- Unit encoding checks cover selected examples rather than the full runtime domain (`unit_values.py:198`). Python re-encoding in the main comparison remains a distinct limitation.
- Error-prefix state, simultaneous faults, repeated output instructions, duplicate input names, multi-output destination aliases, and inactive impacts with unresolved operands need retained tests.
- Active and inactive transcript reads need a retained matrix over all 13 types, including insufficient slices at every offset, malformed encodings, and unused suffixes.

**Executed observations from this review:** Active and inactive public/private reads were checked for all 13 types. Successful memory encodings matched Rust; K cursors advanced by each declared encoded length for active reads and stayed at zero for inactive reads.

Mixed active/inactive and empty impacts produced:

```text
pis       = [19, 0, 0, 7, 8]
pi_skips  = [Some(2), None, Some(0), None]
pubInIdx  = 2
```

The exposed Rust vectors matched. A transcript mismatch occurred after the active group had been pushed, as Rust specifies.

A two-output-instruction program confirmed commitment accumulation over raw input `[7]` followed by outputs `[7,8]`, using opening `5`. Wrong commitment, missing commitment, excess raw inputs, and transcript exhaustion before commitment produced the expected failure categories.

Generation followed by a real run on the same `Runner` did not leak generation mode: generation succeeded with a default transcript value, while the subsequent real run rejected the missing transcript. The normal `job` entry initializes a fresh configuration; no cross-run mode contamination was found.

These focused checks were not saved as repository tests.

## 4. Questions for the authors

1. Is the promised VM correspondence over every program accepted by `IrSource::load`, or only programs satisfying `wf`? The current runner implements the former entry domain while several rules and documentation assume the latter.
2. Are error configurations intended to reproduce Rust’s internal state at failure? Rust does not expose that state through `Preprocessed`, and current K snapshots do not preserve it.
3. Is panic-to-error normalization part of the approved comparison predicate? If so, which observables remain meaningful after a normalized panic?
4. Is `preimage_term` a trusted constructor? It accepts unrestricted Python integers for raw inputs, binding input, transcripts, commitment and opening (`zkir_run.py:56`), whereas the oracle rejects noncanonical field elements (`L/zkir-oracle/src/main.rs:55`). The required canonical-field precondition should be explicit or enforced.

## 5. What I checked and how

**Source inspection:** Read `WIKI_SCHEMA.md`, queried `wiki/index.md`, and read the semantics plan and definition record. Reviewed all 34 preprocessing arms at `R/ir_vm.rs:287` through `R/ir_vm.rs:644`, plus initialization, operand helpers, exhaustion and commitment finalization.

The arm-by-arm inspection covered:

- `encode`, `add`, `mul`, `neg`, `inv`, `not`;
- `constrain_eq`, `cond_select`, `assert`, `test_eq`;
- `public_input`, `private_input`, `copy`, `constrain_to_boolean`, `constrain_bits`;
- `div_mod_power_of_two`, `reconstitute_field`, `less_than`, `jubjub_scalar_from_native`;
- `transient_hash`, `persistent_hash`, `keccak256`, `impact`, `hash_to_curve`;
- `ec_mul`, `ec_mul_generator`, `into_coordinates`, `from_coordinates`;
- `into_bytes32`, `from_bytes32`, `reverse_bytes`, `bytes32_into_low_high`, `bytes32_from_low_high`, `output`.

Read the corresponding K execution rules, `reads`/`writes`/WF rules, value dispatch, decoding/default helpers, runner and differential harness. Read every corresponding Rust off-circuit helper, plus `IrValue::default`, alignment parsing/serialization, and transient commitment construction. Consulted the supplied textual specification and existing differential, value-unit and divergence receipts. Existing receipt totals were not treated as fresh test results.

**Commands and execution method:**

- Used `rg`, `nl -ba`, `sed`, and small read-only Python extracts for the source comparison.
- Ran `git -C /home/charl/Moriarty/repos/_build/ledger-92e8bdd3 rev-parse HEAD`.
- Attempted `uv run --group zkir-k python -B ...`; it failed before execution because its cache lock required a temporary file on the read-only filesystem.
- Used the existing `.venv/bin/python -B` and compiled LLVM interpreter through stdin/stdout. Only the subprocess transport was replaced in memory; no semantics or runner-result extraction code was changed.
- Invoked the existing Rust oracle with program/preimage JSON supplied through inherited pipes exposed as `/dev/fd/N`.
- Ran 50 focused paired cases: the 16 counterexample/failure cases described above; 13 inactive and 13 active transcript-type cases; mixed impacts and mismatch; multiple-output commitment, wrong/missing commitment, transcript-exhaustion precedence, excess raw inputs; and a real run after generation.
- Evaluated the unrelated-error `compare` example directly.
- An exploratory `interpreter --help` invocation exited 139; subsequent interpreter calls used its documented positional interface. This invocation is not evidence of a semantics defect.
- Did not recompile, run corpus-writing generators, or modify files.

The read-only K transport used for the focused cases was:

```python
from pyk.ktool.krun import llvm_interpret_raw
from pyk.kore.prelude import top_cell_initializer, inj, SORT_K_ITEM
from pyk.kore.syntax import SortApp
from zkir_run import Runner

runner = Runner()
runner.krun.run_process = lambda kore, depth=None: llvm_interpret_raw(
    runner.krun.definition_dir,
    top_cell_initializer({
        "$PGM": inj(SortApp("SortJob"), SORT_K_ITEM, kore)
    }).text,
    depth=depth,
    check=False,
)
```

Counterexample instructions in the findings were wrapped in version `3.0` programs with empty input/output declarations unless specified, `do_communications_commitment: false`, binding input `"0"`, and empty transcripts. Programs were converted with `zkir_kast.program` and executed with `runner.run`. Rust error snapshots were inferred from the cited source order only; acceptance differences, terminal-state behavior, panic exits, and the reported successful comparisons were executed.
Top finding: no blocking nonsecurity defect was found. The consumer is genuinely decoder-bound, derives its arithmetic and fuel inputs from actual program/state rows, and the proposed standalone execution stays within the stated 15-file, 9-test scope.

### Approved scope

This approval covers only:

- Direct `rustc --test --edition=2024 --emit=dep-info,link` compilation of `arithmetic_harness.rs` and its exact 15-file closure.
- One execution of that test binary containing exactly the 9 named normal tests.
- At most 30 seconds per child, with the guard’s minimal environment, new process session/process-group timeout handling, dep-info closure check, artifact rehash, read-only snapshot files, and source/compiler/interpreter postflights.
- No Cargo, SDK, VM, model, circuit, native, proof, key, ledger, or cache execution.

All tests remain UNRUN. This is an execution-scope approval, not a claim that compilation or any test passes.

### Compilation interface

The harness resolves coherently through the private `sherlock` shim:

- `reference_types.rs`, `arithmetic_witness.rs`, and its 5 normal tests are included.
- The layout shim includes `rows`, `limits`, `constants`, `prepare`, `state`, `state_encode`, `state_decode`, `arithmetic_fuel`, and its 4 normal tests.
- No `Machine` or evaluator module is in the closure.
- The configured dep-info expectation is therefore the same 15 files, and the configured test names correspond to the actual module paths `sherlock::arithmetic_witness_normal_tests::*` and `sherlock::layout_a::arithmetic_fuel_normal_tests::*`.
- Visibility, the `&Arc<Program>` to `&Program` coercion, literal inference, and the `pub(super)` relationships are consistent on source inspection. Compilation itself remains to be established by the run; the first failure stops dependent steps.

### Consumer correctness

`materialize_numeric_aux_a` first calls `decode_observed_a(..., DecodeMode::Step)` and propagates failure. Only after that does it derive:

- `en_evt` from `outcome_tag == 0 && control_length != 0`;
- terminal status from head `Return` at depth zero or head `Error`;
- `charged = en_evt && !term && fuel != 0` through `materialize_fuel`;
- `en_bin` from charged status, head `Return`, nonzero depth, and head frame tag `2`;
- `Add/Sub/Mul/Lt` only from binary-operation tags `0/1/2/4`, excluding Eq, Cons, and Branch;
- header operands from the actual head Right-frame and Return-control `valueRef`s;
- operand nodes through the space-selected Constants/Data active-length fetch;
- UInt byte operands only when the corresponding decoded node has `type_id == 0`.

That matches the stated ARH-0 through ARH-3 host materialization. The Step decoder has already structurally bound the full program, supplied `ProgramBlock`, state lengths, padding, Push spine, node fields, and zero columns, so the subsequent fetch refusal is defensive rather than a separately claimed refusal path.

The arithmetic recurrences for 16-byte Add, Sub/Lt borrow, full-width Mul carries/high half, and the fuel probe/decrement/publication formulas match the literal owner specimens on paper. This includes P-1, A-3, S-3, L-4, M-6, T-1 through T-4, and the fuel cases through ordinary Eval/Return/Error controls.

### Runner and configuration

The runner/configuration adaptation is consistent with the narrowed scope:

- The final manifest is the 19 source rows plus `SOURCE-CANDIDATE.json`, `CONFIG.json`, and `run_arithmetic.py`.
- `CONFIG.json` pins the same source rows, the 15-file closure, the corresponding 19 snapshot members, and the 9 test names.
- The runner refuses before creating `attempt-01` unless both required execution-preflight reports approve the same final manifest hash.
- It verifies materialized bytes against the manifest, checks exact `candidate/` membership, loads the guard from verified bytes, and adapts only the stated preflight and realization selection.
- The underlying guard still performs the direct rustc invocation, exact dep-info comparison, exact libtest membership/result checks, artifact rehash before execution, and postflight rehashing.
- Claims correctly narrow T-1/T-2 to selectors, `uu`, and zero limb blocks rather than unasserted header lanes, and correctly avoid claiming rule lines for reused fuel values.

### Exclusions and residuals

This approval does not cover canonical-mode acceptance, reachability, transition validation, appendData, branch commit, aux/2 packing, native binding, constraints, hostile/decoder-refusal tests, product `mod.rs`/`lib.rs` wiring, or any change to C4, aux/1, schemas, profiles, public API, or native authority `0`.

The recorded residuals—unbounded memory/disk/process counts, possible surviving descendants after a normally exiting leader, same-UID limits of `0444` snapshot files, and unhashed sysroot/linker/libc/Python/kernel—remain outside the mathematical/interface approval and do not create a nonsecurity blocker for this bounded honest-host run. No security-guard replacement is proposed here; any required security-guard change remains Opus-authored work.

VERDICT: APPROVE_WITH_SCOPE
CANDIDATE_MANIFEST_SHA256: 5639b21a98f94170bf1d372441d1e4827e3f9e5324538b92acdce67d61a2382d

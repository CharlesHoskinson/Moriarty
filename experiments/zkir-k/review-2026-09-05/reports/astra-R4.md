VERDICT: BLOCKED — Reproduced execution and deserialization mismatches, false successful-run reporting, and incorrect extension gate verdicts prevent accepting the definition as a faithful executable model.

## Findings

Source locators below use these exact roots:

- `L`: `/home/charl/Moriarty/repos/_extracts/ledger9-92e8bdd3-zkir-v3-src/zkir-v3/src`
- `E`: `/home/charl/Moriarty/repos/_build/midnight-zkir-2ffe2d1/zkir/src`
- `SPEC`: `/home/charl/Moriarty/repos/input-output-hk/arc-zkir/docs/zkir-v3-spec.md`
- `MANUAL`: `/home/charl/Moriarty/repos/runtimeverification/k/docs/user_manual.md`

### R4-01 — major — Execution bypasses checks on which instruction rules depend

**Locations:** `experiments/zkir-k/semantics/zkir-vm.k:87`, `:301`, `:305`, `:310`; `experiments/zkir-k/tools/zkir_run.py:271`; `experiments/zkir-k/semantics/zkir-syntax.k:298`.

**Repository observation:** Neither `job` nor `Runner.run` invokes `wf`. Nevertheless, the `div_mod_power_of_two` rule assumes its bit count was checked statically. Its witness implementation accepts any integer bit count. `reconstitute_field` likewise omits Rust’s explicit upper-bound check.

**Source fact:** `L/ir_vm.rs:395` checks the div/mod output count and rejects `bits > 248` before resolving the operand. `L/ir_vm.rs:418` rejects the same excessive bound for reconstitution.

**Experiment observation:** This instruction, with no inputs and an otherwise empty version-3.0 program,

```json
{"op":"div_mod_power_of_two","val":"0x01","bits":249,"outputs":["%q","%r"]}
```

completed in K with `status=ok`, `%q=0`, `%r=1`, and two holding verdicts. Rust returned `error: Excessive bit count`. Running the same program through `ZKIR-CHECK` produced `wfError`.

Two successive copies assigning `%x` also executed successfully and overwrote the register. This contradicts the claim that reassignment is rejected before execution in `wiki/zkir/zkir-k-definition.md:56`.

**Recommendation:** Make the advertised checked entry point invoke `wf`. If a separate raw entry point is retained for studying malformed Rust programs, implement Rust’s runtime arity and bit-bound checks there. Do not depend on a separate checker that callers are free to omit.

### R4-02 — major — A stuck or depth-limited execution is reported as successful

**Locations:** `experiments/zkir-k/tools/zkir_run.py:274`, `:281`, `:295`; `experiments/zkir-k/semantics/zkir-vm.k:302`.

**Repository observation:** The runner derives success solely from `<status>`. It returns the residual `<k>` term as informational text but never requires execution to have finished. There is no witness rule for a div/mod instruction with an incorrect output count.

**Source fact:** `L/ir_vm.rs:396` returns an error for that shape.

**Experiment observation:** With

```json
{"op":"div_mod_power_of_two","val":"0x01","bits":8,"outputs":["%q"]}
```

K returned `status=ok`, empty memory, zero verdicts, and a residual computation beginning with `#exec(divModPowerOfTwo(...))`. Rust returned `DivModPowerOfTwo requires exactly 2 outputs`. An empty program run with `depth=0` was also reported as `ok`.

**Inference:** Any missing execution rule can masquerade as success. Empty verdict lists can then be mistaken for successful constraint checking.

**Recommendation:** Distinguish `finished`, `stuck`, and `depth-exhausted` results. Require an explicit terminal condition before reporting witness success, and require completed verdict generation before reporting constraint-check success. Add the missing malformed-div/mod error rule independently.

### R4-03 — major — Extension gate checking falsely accepts unsupported circuits

**Locations:** `experiments/zkir-k/semantics/zkir-constraints.k:179`, `:182`, `:185`, `:190`; `experiments/zkir-k/semantics/zkir-ext.k:261`.

There are two independently exposed omissions.

**Bytes selection**

- **Repository observation:** The inherited `#eqSupported` rejects `cond_select` specifically on `bytes32`, while its fallback accepts `bytesV` values of other lengths.
- **Source fact:** `E/ir_instructions/select.rs:85` has supported in-circuit arms for Native, Bool, Byte, and the listed curve/field types. No `Bytes` arm exists; the fallback at `:121` returns a synthesis error for every byte-string length.
- **Experiment observation:** Selecting between two `Bytes<1>` inputs with native guard `0x01` completed in K with no violations. Rust preprocessing also succeeded, as expected; the synthesis discrepancy follows from the inspected Rust dispatch, not an executed synthesis experiment.
- **Recommendation:** Reject all byte-string values in the extension’s in-circuit selection support predicate. Representation aliasing at length 32 must not determine instruction support.

**Constants requiring an uninitialized chip**

- **Repository observation:** `load_constant` checks only whether its decoded value matches the output register; it never calls `#chipFor`.
- **Source fact:** `E/ir_vm.rs:1395` calls `assign_constant_incircuit`. For JubjubScalar, `E/ir_instructions/assign_constant.rs:54` calls `std_lib.jubjub().assign_fixed`. However, `E/ir_vm.rs:1470` does not count `LoadConstant` when selecting chips. The accessor at `/home/charl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:752` uses `expect` when that chip is absent.
- **Experiment observation:** A program containing only `load_constant Scalar<Jubjub>` with encoding `["0x01"]` completed in K with no violations. The missing-chip synthesis failure is source-derived; Rust synthesis was not executed.
- **Recommendation:** Check the constant’s required chip in the gate relation. Preserve the pinned source’s chip-selection behavior rather than silently enabling the chip and repairing the source being modeled.

### R4-04 — major — `test_eq` uses type equality instead of Rust’s dispatch

**Locations:** `experiments/zkir-k/semantics/zkir-ops.k:103`; `experiments/zkir-k/semantics/zkir-ext.k:65`.

**Repository observation:** `testEqV` accepts every pair with equal `typeOf` values and rejects every pair with different types.

**Source facts:**

- `L/ir_instructions/eq.rs:44` and `E/ir_instructions/eq.rs:46` omit JubjubScalar and return an unsupported-operation error.
- `E/ir_instructions/eq.rs:50` compares two `Bytes` vectors without requiring equal lengths. Different lengths produce `false` off-circuit.
- The extension’s in-circuit implementation separately requires equal byte lengths at `E/ir_instructions/eq.rs:107`.

**Experiment observations:**

- For a JubjubScalar input encoded as `1`, `test_eq %s %s` succeeded in K with Native output `1`; the extension Rust oracle returned an unsupported-operation error.
- Comparing `Bytes<1>` encoded as `[1]` with `Bytes<2>` encoded as `[1]` errored in K. Rust preprocessing succeeded with Native output `0`.

**Recommendation:** Implement the actual off-circuit support matrix, including the JubjubScalar exclusion and unequal-length byte comparison. Keep the in-circuit length restriction separate so the existing Rust divergence remains observable.

### R4-05 — major — The JSON preprocessor does not mirror `IrSource::load`

**Locations:** `experiments/zkir-k/tools/zkir_kast.py:78`, `:96`, `:185`, `:313`, `:332`, `:338`, `:342`, `:345`.

**Experiment observations:** The following differences were reproduced against the existing Rust oracle binaries.

| Input shape | Python/K result | Rust result |
|---|---|---|
| `private_input` without a `guard` member, with sufficient transcript data | Format error | Successful unguarded read |
| `copy` with an additional `"note":"ignored"` member | Format error | Successful execution |
| Immediate `"0x01 00"` | Accepted as `1` | Hex-decoding error |
| Version `{"major":3.0,"minor":0}` | Accepted and executed | Rejected: expected `u8` |
| Empty objects for `inputs`, `outputs`, and `instructions` | Accepted as empty collections | Rejected: expected sequence |
| Extension type `Bytes<01>` | Accepted and executed | Rejected as noncanonical |
| Extension type `Bytes<١>` | Accepted and executed | Rejected as noncanonical |

**Source facts:** Optional guards are `Option<Operand>` at `L/ir.rs:880` and `:903`. The relevant deserialized structures do not deny unknown fields. Rust uses `const_hex::decode` at `L/ir.rs:227`, `u8` version components at `:923`, and vector fields at `:41`, `:45`, and `:49`. The extension explicitly requires ASCII digits without a leading zero at `E/ir_types.rs:176`.

**Inference:** The translator rejects valid artifacts and admits invalid artifacts. Python’s whitespace-tolerant hex decoder, numeric equality, iteration over dictionaries, and Unicode-aware `isdigit` are observable parts of the accepted language.

**Recommendation:** Match these boundaries explicitly: allow omitted optional guards, reproduce unknown-field handling, validate collection kinds and exact numeric types, enforce Rust-compatible hex syntax, and require canonical ASCII byte lengths. Add independent deserialization comparisons that do not skip a case merely because Python rejected it.

### R4-06 — major — Preimages admit integers that cannot inhabit Rust `Fr`

**Locations:** `experiments/zkir-k/tools/zkir_run.py:56`, `:60`; `experiments/zkir-k/semantics/zkir-values.k:152`, `:226`.

**Repository observation:** Preimage construction converts values with `int()` without checking field range. Native decoding stores the supplied integer directly. Extension strict decoding merely re-encodes it, so it does not repair this boundary.

**Source fact:** `ProofPreimage` contains field elements. The oracle’s `/home/charl/Moriarty/repos/_build/ledger-92e8bdd3/zkir-oracle/src/main.rs:52` explicitly rejects values that cannot be decoded as canonical field elements.

**Experiment observation:** An otherwise empty program with one Native input accepted raw input exactly equal to the BLS12-381 scalar modulus in both K definitions. The resulting register contained that out-of-range integer.

**Inference:** The runner can report successful executions outside the source model’s value domain.

**Recommendation:** Validate every raw field element, transcript element, binding input, commitment, and opening at the Python boundary. For direct K entry and symbolic claims, provide and require a corresponding preimage-validity predicate.

### R4-07 — major — Several `total` declarations assert unsupported domains

**Locations:** `experiments/zkir-k/semantics/zkir-field.k:34`; `experiments/zkir-k/semantics/zkir-hash.k:156`, `:211`, `:224`; `experiments/zkir-k/semantics/zkir-values.k:90`, `:132`; `experiments/zkir-k/semantics/zkir-ext.k:209`, `:219`.

**Repository observation:** Functions declared total accept unrestricted `Int`, `List`, or `Bytes` arguments but assume nonzero moduli, bounded indices, correctly shaped lists, or fixed byte lengths.

**Source fact:** `MANUAL:546` defines `total` as having a value for every possible argument tuple; it is an assertion to the symbolic engine, not an assertion restricted to intended callers.

**Experiment observations using the existing `ZKIR-TEST` interpreter:**

| Term | Result |
|---|---|
| `fmod(1,0)` | Aborted with modulus-by-zero exception |
| `shaK(64)` | Aborted with out-of-range exception |
| `rotl64(1,-1)` | Exit 113 and unresolved function term |

Additional source-visible examples include `encForeign` dividing by its unrestricted limb-width argument, `encodeValue(bytes32(B))` indexing byte 31 without a length predicate, and extension slicing/indexing with unrestricted negative integers.

**Inference:** Intended execution invariants may protect ordinary inputs, but they do not justify these declarations for arbitrary terms or symbolic configurations. No Haskell inconsistency or successful unsound proof was demonstrated in this review.

**Recommendation:** Remove unjustified `total` attributes, introduce suitable domain sorts, or return explicit error results outside the supported domain. State and prove the domain invariants needed by exported operations and claims. Avoid arbitrary success-valued fallbacks used solely to make a function appear exhaustive.

### R4-08 — major — Green differential results omit essential acceptance predicates

**Locations:** `experiments/zkir-k/tools/diff_test.py:98`, `:147`, `:169`, `:175`, `:194`; `experiments/zkir-k/tools/divergence_tests.py:179`; `experiments/zkir-k/tools/zkir_run.py:203`.

**Repository observations:**

- Once both statuses are non-`ok`, `compare` returns agreement without comparing errors or state.
- Non-holding verdicts are printed but do not increment the differential failure count.
- Python format errors are skipped with the unverified assertion that both sides reject them.
- The divergence harness infers `holds` whenever no matching violation is found; it never establishes that the requested gate was emitted and evaluated.
- Extension memory comparison records `Bytes` plus field encoding without byte length. That representation is not injective: `[0x01]` and `[0x01,0x00]` both encode as `[1]`.

**Experiment observations:** `compare` returned `[]` for unrelated error messages. It also returned `[]` for otherwise matching successful results where K contained a deliberately supplied violated gate. The unequal-length byte test in R4-04 produced identical variant/encoding records for its two differently typed input registers.

**Source-of-truth requirement:** The plan’s comparison predicates are stated at `wiki/zkir-k-semantics-plan.md:67` onward; the claimed gate-check result is described at `wiki/zkir/zkir-k-definition.md:48`. The current implementation enforces a weaker predicate.

**Recommendation:** Compare normalized error categories, verify rejection on both parsers, and make unexpected gate outcomes fail the test. Require explicit gate coverage, terminal execution, and completed verdict generation. Include byte length in both oracle and K result records. Known deliberate divergences should have explicit expected outcomes.

### R4-09 — major — Fixture extraction mistakes transcript inputs for raw inputs

**Locations:** `experiments/zkir-k/tools/extract_test_inputs.py:50`, `:51`, `:67`; `experiments/zkir-k/corpus/ledger9-92e8bdd3-tests/manifest.json:367`; `experiments/zkir-k/tools/diff_test.py:71`.

**Repository observation:** The regex for `inputs:` has no identifier boundary. It matches the suffix of `public_transcript_inputs:`. The resulting empty list is recorded as a literal raw-input preimage and replaces generated input data on the first attempt.

**Source fact:** At ledger commit `92e8bdd3`, `zkir-v3/tests/proofs.rs:1150` constructs nonempty secp256k1 inputs. Its `ProofPreimage` uses shorthand `inputs` at `:1177`, followed by `public_transcript_inputs: vec![]` at `:1179`. Those are different fields.

**Experiment observation:** Reapplying the extraction regex read-only found this suffix match in fifteen fixtures. The manifest records empty raw inputs for `test_secp256k1_proof.zkir`; `evidence/zkir-k-differential-92e8bdd3-2026-09-05.txt:148` consequently records a passing comparison that fails before decoding its first input.

**Inference:** Some purported source-test seeds are fabricated empty inputs. They inflate early-error comparisons and do not reproduce the source test.

**Recommendation:** Match complete field identifiers within the selected `ProofPreimage` literal. Leave computed or shorthand inputs absent as the tool’s contract specifies. Regenerate the affected manifests and supersede the receipts with corrected runs. Preserve whether each field was extracted, computed, or generated.

### R4-10 — minor — Extension concatenation omits the maximum result length

**Location:** `experiments/zkir-k/semantics/zkir-ext.k:198`.

**Repository observation:** Any nonempty concatenation succeeds, regardless of total length.

**Source fact:** `E/ir_vm.rs:733` rejects results exceeding `MAX_BYTES_LEN`; the in-circuit path also rejects excessive length at `:1382`.

**Inference / specified-only counterexample:** Start with one Byte and repeatedly concatenate the preceding result with itself. The result of length `2^25` exceeds the `2^24` limit. Rust rejects it; the K rules contain no corresponding rejection. This large allocation experiment was not run.

**Recommendation:** Check the accumulated result length in the witness operation and gate relation. Test the boundary and one byte beyond it without requiring an unnecessarily large ordinary regression fixture.

### R4-11 — minor — Overlapping function rules can produce different results

**Locations:** `experiments/zkir-k/semantics/zkir-syntax.k:296`, `:298`; `experiments/zkir-k/semantics/zkir-field.k:83`, `:85`; `experiments/zkir-k/semantics/zkir-ext.k:190`, `:191`, `:202`, `:203`.

**Repository observations:**

- A div/mod instruction with both excessive bits and wrong arity matches two ordinary `#checkArity` rules yielding different error strings.
- `#nonResidue(7,10)` matches the nonresidue rule and the `Z >= P` rule, yielding `10` and `0`.
- The typed and generic `owise` fallbacks of `#boolFold` and `#concat` overlap and yield different diagnostics.

**Source fact:** `MANUAL:1743` states that multiple `owise` rules share priority and can apply in any order.

**Experiment observation:** For div/mod with `bits=249` and one output, the LLVM checker selected the excessive-bit error, whereas Rust selected the output-count error.

**Inference:** These are actual differing-result overlaps, unlike the harmless duplicate identity cases in `ecAdd`. Their Haskell behavior was not tested.

**Recommendation:** Make conditions disjoint or define explicit error precedence. Exclude typed values from generic list fallbacks. Guard the nonresidue success rule with its intended search range. Do not rely on textual rule order.

### R4-12 — minor — Documentation exceeds the preserved evidence

**Locations:** `wiki/zkir/zkir-k-definition.md:15`, `:47`, `:48`, `:50`, `:56`, `:57`, `:63`, `:69`.

**Repository observations and source facts:**

- The claim that reassignment is rejected before execution is contradicted by R4-01.
- The parser-equivalence claim at `:34` is contradicted by R4-05.
- Comparing four Poseidon examples does not establish that paired rounds are algebraically equivalent for all states, as inferred at `:47`.
- “Every instruction on every type it supports” at `:50` is not established by `gen_handmade.py`; for example, its foreign-scalar arithmetic coverage is not an instruction/type cross-product.
- The preserved divergence receipt ends with thirteen cases at `evidence/zkir-k-divergence-tests-2026-09-05.txt:41` and contains no `k02` result, although `:63` claims its reproduction.
- `evidence/k-rust-compatibility-2026-09-05.txt:6` records parsing one syntax term; `:8` records a compile timeout without output. These do not establish the wiki’s diagnosis that full-definition failure is frontend performance rather than a feature boundary.
- Error-path agreement is weaker than the prose implies because error contents are not checked.

**Inference:** A reader can mistake sampled behavior, inferred causes, and a newly added test for preserved verification results.

**Recommendation:** Narrow claims to their tested predicates, label the timeout explanation as unresolved inference, and distinguish implemented tests from recorded runs. Preserve a new receipt before claiming `k02` reproduction. Do not overwrite the existing receipts.

## Coverage gaps

### Static checks versus specification §8

The following comparison records source facts from `SPEC:1156` and repository observations from `experiments/zkir-k/semantics/zkir-syntax.k:250`.

| Producer obligation | What `ZKIR-WF` checks |
|---|---|
| O1: single assignment, including distinct input names | Implemented through the defined-name set, but not enforced by the ordinary runner. |
| O2: value typing and the JubjubScalar equality exclusions | Not implemented. The checker tracks names, not typing contexts. |
| O3: bit bounds | Implements the specified upper bounds: `<255` for `constrain_bits`/`less_than`, `≤248` for div/mod and reconstitution. Direct K terms additionally need nonnegative/u32-domain checks. |
| O4: witness shape | No corresponding witness-shape checker. Static div/mod arity is covered, but transcript round trips, read-guard booleanity, exact assertion value, non-overflow, and exact less-than bounds are not established by `wf`. Honest execution checks some of these dynamically. |

Additional checks include minor version, immediate range, define-before-use, output arity, and div/mod arity. Input encoded width and encode-output width are checked dynamically. Therefore `wfOk` is neither the complete §8 producer predicate nor a witness-shape certificate. The comment at `zkir-syntax.k:237` also incorrectly groups `less_than` with the 248-bit bound; its implemented 255-exclusive bound agrees with the specification.

### Instructions, values, and preimages

**Repository observations / recommendations:**

- Add the concrete boundary cases in R4-01 through R4-06, including direct execution of malformed programs.
- Cover extension byte lengths across representation and encoding boundaries: 1, 31, 32, 33, unequal comparison lengths, and the maximum result length.
- Cover constants of every chip-dependent type, both with and without another instruction/input that enables the chip.
- Exercise `loadConstant` direct K terms containing variable operands: `zkir-ext.k:237` silently drops variables, while Rust constants contain field encodings. A narrower constant-encoding sort would prevent this unsupported term shape.
- Add systematic typed instruction coverage rather than inferring it from program-level success. Existing generation stops after the first successful attempt and uses small Native values.
- The application precompiles lack successful consistent-transaction preimages in the receipts. Their error-path comparisons do not exercise their complete execution.
- `zkir-ops.k:221` explicitly rejects alignment options. There is no execution coverage for these shapes. Compress atoms also take an error path; their rejection should be checked independently against the source.
- Fixed public-input and output gates are accepted unconditionally at `zkir-constraints.k:147` and `:384`. Green verdicts do not independently validate those parts of the claimed circuit relation.
- The divergence tests evaluate the constructed witness. They do not establish behavior for adversarially changed witnesses or prove equivalence to Rust circuit synthesis.

### Haskell and proof engineering

**Repository observation:** The existing Haskell check is the tutorial program, not this definition: `experiments/zkir-k/toolchain-check/check_k_toolchain.sh:32`. No full ZKIR Haskell compilation, execution, or claim was reproduced here.

**Source inspection:** The extension’s priority-30 initialization precedes ordinary priority-50 rules and avoids the LLVM-reserved 50–150 range described at `MANUAL:1758`. A targeted LLVM test confirmed that Jubjub input `[2,1]` is accepted by the base definition and rejected by the extension’s canonical re-encoding check.

**Inference / recommendation:** Backend equivalence remains unestablished. Before claims, resolve unjustified totality, differing-result overlaps, and unrestricted map/list/value domains. A map-key membership test does not establish that a lookup inhabits `Value`. Claims need canonical field values, correctly shaped collections, `genMode=false`, the appropriate strict-decoding setting, and the intended producer obligations.

Hashes currently have executable equations, including `poseidonHash` at `zkir-hash.k:66`; they are not an isolated uninterpreted proof interface. Symbolic hash expansion, modular exponentiation, and nonlinear field arithmetic will require an explicit proof strategy, abstraction boundaries, and justified lemmas. The plan’s proposed uninterpreted-hash treatment is not implemented.

**Repository observation:** A scan found no duplicate explicit `symbol(...)` names in the handwritten definitions. Same-spelling overloaded constructors such as `reverseBytes` are not evidence of a symbol collision. No differing-result LLVM/Haskell collection-pattern case was demonstrated.

**Recommendation:** Replace operation-name strings with small operation sorts where practical, particularly fallbacks that silently choose a hash algorithm. Preserve compiler stderr and classify warnings during a full Haskell build; the reviewed receipts do not establish warning-free compilation.

### Reproducibility and flakiness

**Repository observation:** `diff_test.py:150` seeds a dedicated generator from the seed and filename, so ordinary runs are deterministic for a fixed environment and corpus. The larger risks are changed fixtures, modified binaries at fixed paths, stale compiled definitions, and silently skipped corpus entries.

Both oracle worktrees had the advertised full commits and a one-line visibility change exposing `preprocess`, plus Cargo manifest/lock changes. This review inspected those changes but did not rebuild the binaries. The harness neither authenticates these binaries nor records every generated preimage. Temporary oracle files created with `delete=False` are also left behind at `diff_test.py:53`.

**Recommendation:** Preserve corpus/definition/binary digests, full pins, patch identities, generated preimages, and expected case counts with each run.

## Questions for the authors

1. Is `job` intended to model arbitrary Rust-deserializable programs, or only programs satisfying a separately checked producer predicate? The current implementation and documentation imply different contracts.
2. Is the gate evaluator intended to reproduce exact Rust synthesis availability, including missing-chip panics and unsupported dispatch, or only selected mathematical relations? Several extension verdicts currently contradict the former interpretation.
3. Where are the full ZKIR Haskell build/claim evidence and the claimed `k02` receipt? Neither was present in the reviewed evidence.
4. Will symbolic hashes use a separately justified abstract interface, and what validity predicate will connect its values and configurations to the concrete definition?

## What I checked and how

**Repository observations:** Read `WIKI_SCHEMA.md`, the controlling assignment, relevant `wiki/index.md` entries, both review briefs, both design pages, all handwritten semantics modules, generated-constant declarations/fallbacks, the four requested tools, and the runner, differential, divergence, and unit-test harnesses. Inspected the named receipts and relevant corpus manifests. Compared against the pinned Rust deserializers, instruction dispatch, VM checks, extension type parser, constant assignment, chip selection, specification §§5.3/8.2, and K manual sections on functions, totality, symbols, and priorities.

**Commands and execution method:**

- Used `rg`, `nl -ba`, `sed`, `git show`, `git rev-parse`, `git diff`, and `sha256sum` for source and provenance inspection.
- Attempted `uv run --no-sync --group zkir-k python -B`; it failed acquiring a cache lock because the filesystem is read-only.
- Ran inline checks with `.venv/bin/python -B -`.
- Constructed terms using `zkir_kast.program`, `preimage_term`, and pyk. Wrapped them with `top_cell_initializer`; invoked the existing LLVM binaries as `interpreter /dev/stdin -1 /dev/stdout`. This avoided temporary files. For runner tests, only the in-memory `run_process` method was replaced with that transport; result extraction remained the repository implementation.
- Passed JSON program/preimage documents to the existing Rust oracles through inherited pipe descriptors named `/dev/fd/N`.
- Reapplied the extractor’s regex read-only against `git show 92e8bdd3:<source>`.
- Tested `compare` directly with unrelated errors and a supplied gate violation.
- A preliminary `interpreter --help` invocation segfaulted; this was not treated as a semantics finding. Subsequent invocations used its positional interface.

The reproduced programs used version 3.0, no commitment, empty output signatures, and binding input `0` unless stated otherwise in a finding. All omitted transcript vectors were empty.

**Observed pins:**

- Ledger: `92e8bdd3a97b61b229e38916e1b180de6f448dd5`.
- Extension: `2ffe2d17bbb736aec36fb300aeaca679a10d2278`.

**Observed SHA-256 digests:**

| Executable | SHA-256 |
|---|---|
| Base LLVM interpreter | `bb00dcaf8c6a6d28ec5eadacd21bd1c8d277514eec5f50d90aa7a9374e1594ae` |
| Extension LLVM interpreter | `97acd06b794a08ca2d0f972348dcfc8161cd7f87d399615d0e295a33e0c78e78` |
| Ledger oracle | `7e1e4fbc7a9a6101b816b186d591da94754a45019f11c0fe849e14862d823c81` |
| Extension oracle | `6322a91f797213934955e50d7b2df8b4b0f34e78a338b7e4408874efe6c00333` |

**Limitations:** No files were modified. No subagents were used. No definition was recompiled, no full corpus suite was rerun, and no Rust synthesis or Haskell proof was executed. Runtime observations above are fresh targeted checks; synthesis conclusions are explicitly source-derived; the large concatenation counterexample is specified-only.
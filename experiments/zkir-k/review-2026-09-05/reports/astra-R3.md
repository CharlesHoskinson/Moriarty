VERDICT: BLOCKED — The checker omits enforced constraints and accepts some circuits that cannot be synthesized; all 14 current divergence expectations pass, but they do not establish the claimed circuit correspondence.

## 2. Findings

Paths below are relative to the repository unless prefixed as follows:

- `R/`: `/home/charl/Moriarty/repos/_extracts/ledger9-92e8bdd3-zkir-v3-src/zkir-v3/src/`
- `C/`: `/home/charl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/`
- `S/`: `/home/charl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/`
- `A/`: `/home/charl/Moriarty/repos/input-output-hk/arc-zkir/`

### R3-1 — blocker — The circuit commitment equation is absent, masking an actual completeness failure

**Location:** `experiments/zkir-k/semantics/zkir-constraints.k:147`; `experiments/zkir-k/semantics/zkir-vm.k:116`, `:473`.

**Repository observation:** Every `piFixed`, including the commitment, evaluates unconditionally to `holds`. The only commitment calculation is the witness-side check, using raw preimage inputs.

**Source fact:** These are different equations. Rust preprocessing commits to raw inputs (`R/ir_vm.rs:666`), whereas circuit synthesis re-encodes the assigned input values and outputs before hashing and asserting equality with public input 1 (`R/ir_vm.rs:1184`, `:1201`). Noncanonical raw encodings need not survive decoding and re-encoding unchanged.

**Experiment observation:** I executed this counterexample:

- One declared input `%s : Scalar<Jubjub>`.
- No instructions, no outputs, commitment enabled.
- Raw input `2^252`, binding input `42`, opening randomness `0`.
- Commitment:
  `47542911288058811688192079035282004831694446661825114438449692124287687414261`.

Rust preprocessing succeeds and reports `%s` encoded as `[0]`. K succeeds and reports both `piFixed` verdicts as `holds`.

Using Rust `transient_hash`, I checked:

```text
Poseidon([0, 2^252])
= 47542911288058811688192079035282004831694446661825114438449692124287687414261

Poseidon([0, 0])
= 6821569053928206604283034532994160159088093657754227095837456232926426922860
```

The decoding behavior follows `C/ecc/native/edwards_chip.rs:158`: it reads a 254-bit batch and takes only the first 252 bits. Assignment and subsequent circuit encoding of the decoded scalar zero produce zero.

**Inference:** The supplied honest witness fails the circuit commitment equation, although K reports success with every emitted constraint holding. This is a concrete failure of the advertised completeness check, without requiring a malicious internal witness. Circuit rejection itself was established by source inspection, not by running MockProver.

**Recommendation:** Emit an explicit commitment relation containing the opening, declared input identifiers, output operands, and absolute PI index. Calculate its preimage using **in-circuit encoding of assigned values**. Preserve the separate witness-side raw-input check. Also replace the unconditional binding-input verdict with an explicit check, or document that equality as an invariant of initialization rather than a checked constraint.

### R3-2 — major — Empty `impact` omits guard booleanity

**Location:** `experiments/zkir-k/semantics/zkir-vm.k:135`, `:151`, `:155`.

**Repository observation:** `impact` is excluded from ordinary gate emission and emits only one `piGate` per operand. An empty operand list emits no constraint.

**Source fact:** Rust converts the guard to `AssignedBit` before entering the operand loop (`R/ir_vm.rs:871`). The conversion constrains booleanity even when the list is empty. The divergence review explicitly records this behavior at `A/docs/zkir-v3-divergence-review.md:710`.

**Experiment observation:** With native input `%g = 2` and only:

```text
impact(%g, [])
```

K reports a witness error but emits only the holding binding-input constraint. There is no violated guard constraint.

**Why it matters:** This constructs a false apparent divergence: preprocessing rejects the program, while K’s entire constraint list holds. The real circuit also rejects the nonboolean guard.

**Recommendation:** Emit a separate guard-booleanity constraint for every impact, including empty impacts. Emit PI equalities independently.

### R3-3 — major — Native `from_coordinates` misses its Jubjub chip dependency; K1 does not isolate its claimed circuit behavior

**Location:** `experiments/zkir-k/semantics/zkir-constraints.k:302`, `:307`; `experiments/zkir-k/tools/divergence_tests.py:126`.

**Repository observation:** `from_coordinates` checks chips associated with its input values. Native inputs require no chip, so the Jubjub branch never checks that Jubjub is enabled.

**Source fact:** The native/native arm calls `std_lib.jubjub().point_from_coordinates` (`R/ir_instructions/from_coordinates.rs:100`). `used_chips` does not enable Jubjub for `FromCoordinates` (`R/ir_vm.rs:1235`). The accessor panics when disabled (`S/lib.rs:752`).

**Experiment observation:** A program containing only native `%x = 0`, `%y = 1`, and `from_coordinates(%x,%y)` finishes in K with every verdict holding and no enabled chips.

**Inference:** Actual circuit construction reaches the disabled Jubjub accessor, including during key generation with unknown witnesses.

K1’s existing program has the same missing dependency. Its selected `from_coordinates` verdict is `violated`, but its later `into_coordinates` verdict is `synthErr` for the missing Jubjub chip. The test checks only the selected gate.

**K1 disposition:** The underlying parity defect is real. I executed the existing Rust oracle with `(x,y)=(2,1)` and obtained point `(0,1)`. The defective implementation is the `CircuitCurve for JubjubExtended` implementation in **midnight-circuits**, `C/ecc/curves.rs:120`, which encodes only the parity of `x`. It contradicts the exact-coordinate law at `A/src/zkir-v3/Assumptions.agda:480`. With Jubjub enabled, the circuit independently pins both coordinates (`C/ecc/native/edwards_chip.rs:797`).

**Recommendation:** Check the output curve’s chip in every `from_coordinates` dispatch arm. Split K1 into two tests: missing-chip construction failure, and parity divergence with an additional declared Jubjub input enabling the chip. Require the parity case to have no unrelated synthesis failures.

### R3-4 — major — The checker omits chip width assertions

**Location:** `experiments/zkir-k/semantics/zkir-constraints.k:164`, `:166`, `:236`; `experiments/zkir-k/semantics/zkir-syntax.k:304`.

**Repository observation:** `less_than` pads the width but applies no upper synthesis bound. Static checking permits widths 253 and 254. `constrain_bits` treats every width at least 255 as a holding relation.

**Source fact:**

- Rust pads `less_than` to `max(bits + bits % 2, 4)` (`R/ir_vm.rs:966`).
- `S/lib.rs:896` calls `bounded_of_element`.
- That method asserts the padded width is at most `F::NUM_BITS - 2 = 253` (`C/field/native/native_gadget.rs:330`, `:338`).
- `assigned_to_le_bits` asserts its requested width is at most 255 (`C/field/native/native_gadget.rs:894`).

Consequently, comparison widths **253 and 254 both become 254 and panic during circuit construction**. For `constrain_bits`, 255 and greater than 255 are materially different.

**Experiment observation:** K accepts `less_than(1,2,bits=253)` and `bits=254`, returning output 1 and `holds`. Rust preprocessing also accepts the 253-bit case. Direct K VM runs report holding `constrain_bits` gates at both 255 and 256.

**Why it matters:** The comparison cases pass K’s static checks and witness computation but cannot build in the pinned circuit implementation.

**Recommendation:** Model chip API preconditions before evaluating mathematical relations. Reject or classify a padded comparison width above 253 as construction failure. Distinguish `constrain_bits` at 255 from widths above 255. Its ordinary static rejection does not make the direct checker’s blanket relation accurate.

### R3-5 — major — PI indices depend on how an impact fails, and missing PI entries are called violations

**Location:** `experiments/zkir-k/semantics/zkir-vm.k:151`, `:158`, `:407`; `experiments/zkir-k/semantics/zkir-constraints.k:150`.

**Repository observation:** PI indices are calculated as `size(Pi) + deadPis`. `deadPis` advances only when an impact begins execution with an already-failed witness. An impact that starts alive and then fails before appending its PI values is never counted.

**Source fact:** Circuit PI positions depend on program order and the number of preceding impact operands (`R/ir_vm.rs:783`, `:869`), not on preprocessing progress.

**Experiment observation:** For native `%g = 2`:

```text
impact(%g, [1]);
impact(1, [1]);
```

K emits **both** PI gates at index 1; the second belongs at index 2. Final `deadPis` is 1.

There is a separate diagnostic problem: an absent PI entry caused by stopping preprocessing is classified as `violated`. For example, after an off-circuit-only failure such as `assert(2)`, a subsequent constant impact has no generated PI entry. Its absence does not establish that the circuit equation is unsatisfiable.

**Recommendation:** Maintain an independent synthesis PI cursor, initialized to 1 or 2 and advanced at emission time for every impact. Distinguish an incomplete generated PI vector from a complete supplied vector of the wrong length; use `unknown` for the former.

### R3-6 — major — Standard-hash gates reuse the wrong aligned decoder

**Location:** `experiments/zkir-k/semantics/zkir-constraints.k:353`, `:377`; `experiments/zkir-k/semantics/zkir-ops.k:216`.

**Repository observation:** Both standard-hash gates reuse the off-circuit `alignedBytes`, which rejects leftover input fields.

**Source fact:** `fab_decode_to_bytes` calls its inner parser and returns the decoded bytes without checking whether input fields remain (`R/ir_vm.rs:79`, `:86`, `:97`). Insufficient input, options, and compressed atoms have explicit error paths; surplus native fields do not.

**Experiment observation:** A `persistent_hash` with a one-byte alignment and native inputs `[1,2]` produces K witness error and gate `synthErr("...Inputs did not match alignment")`.

**Inference:** The real circuit parser consumes the first field as one byte and ignores the remaining field. Construction can proceed. On this partial K witness, the output comparison should be unknown; with a supplied output, the relation should compare against the hash of byte `0x01`.

**Recommendation:** Implement a separate circuit aligned decoder. Match its consumption rules and range constraints explicitly. Share byte ordering and hash primitives where appropriate, but not the off-circuit parser’s acceptance predicate.

### R3-7 — major — `output` and the catch-all gate rule report unsupported behavior as holding

**Location:** `experiments/zkir-k/semantics/zkir-constraints.k:383`.

**Repository observation:** Every `output` gate holds unconditionally. The following catch-all rule also makes every otherwise-unhandled instruction gate hold.

**Source fact:** Rust checks output arity, resolves every operand, and checks each runtime type against the declared output signature during synthesis (`R/ir_vm.rs:1151`). K’s static pass checks output arity but not these runtime types.

**Experiment observation:** A native input returned against a `Bytes<32>` output signature produces a K witness type error and a **holding output gate**.

**Why it matters:** The constraint checker conceals an actual synthesis rejection. The catch-all also makes omission of a future gate implementation look like successful coverage.

**Recommendation:** Include the output signature in the output constraint and check arity, operand availability, and types. Missing values from an incomplete witness should be unknown; known type mismatches should be synthesis errors. Replace the catch-all success with an explicit unsupported-gate result.

### R3-8 — minor — Failed mathematical relations are confused with unsupported synthesis

**Location:** `experiments/zkir-k/semantics/zkir-constraints.k:136`, `:218`, `:345`.

**Repository observation:** `#matches(vErr(...),...)` always produces `synthErr`. This conflates an unsupported dispatch with a supported operation whose relation has no satisfying result.

**Source fact:**

- Native inversion assigns a zero fallback hint and enforces `x·inverse=1`; zero makes the relation unsatisfiable, not structurally unsupported (`C/field/native/native_chip.rs:362`, `:377`).
- Foreign inversion explicitly enforces the same equation (`C/field/foreign/field_chip.rs:997`).
- `bytes32_from_low_high` asserts the low operand’s final byte is zero and converts the native high operand to a byte (`R/ir_vm.rs:1144`).

**Experiment observation:** K reports `synthErr` for native inversion of zero and for `bytes32_from_low_high(0,256)`.

**Recommendation:** Separate dispatch failure from domain/range failure. Express inversion as the multiplicative relation, returning `violated` for known zero even if its output is absent. Likewise classify known low/high range failures as violations. Apply the same distinction to aligned-byte range failures.

### R3-9 — major — The divergence harness can pass without establishing the claimed divergence

**Location:** `experiments/zkir-k/tools/divergence_tests.py:148`, `:176`; `wiki/zkir/zkir-k-definition.md:54`.

**Repository observation:**

- Only non-holding verdicts are searched. If no matching violation exists, line 184 always returns `holds`, even if the target gate was never emitted.
- Rust is used only through `preprocess`; no case invokes `Relation::circuit`, key generation, or MockProver.
- Only statuses and one selected K outcome are compared. Claimed decoded values and other gate failures are not checked.
- Any nonzero oracle exit is classified as `oracle-failed`; K2 does not require the expected panic location or message.

**Source fact:** The distinction matters particularly for findings 10, 11, and 13, whose live defects concern circuit construction or witness-hint execution. Finding 11’s surviving defect is the panicking subgroup conversion at `C/ecc/foreign/edwards_chip.rs:1093`, not merely the subgroup relation tested by K.

**Experiment observation:** My file-free execution of all 14 current cases matched every expected status and selected outcome. This includes `unknown` for findings 1 and 4, `n/a` for finding 10, and K1’s unrelated synthesis failure.

**Why it matters:** A pass currently means agreement with hand-written K expectations and Rust preprocessing. It cannot substantiate the page’s statement that the review’s circuit findings reproduce.

**Recommendation:** Require exactly identified target verdicts, completed K execution, expected output values, and expected whole-program construction status. Add independent Rust circuit tests, including unknown-witness key generation and controlled known-witness synthesis. Preserve panic classification separately from returned errors.

## 3. Coverage gaps

**Repository observations and source-derived dispositions:**

| Review item | What the current case establishes; remaining gap |
|---|---|
| 1 | Off-circuit overflow rejection. The output is absent, so `unknown` does not demonstrate modular circuit acceptance. |
| 2 | `assert(2)` rejects off-circuit and satisfies the nonzero K relation. The current upstream review labels finding 2 retired; the test should preserve that historical status. |
| 3 | Guard-false produces the default value, which the K gate accepts. It does not test a nondefault inactive value or a nonboolean unused guard. |
| 4 | Off-circuit rejection of values outside the requested width. `unknown` does not test the padded relation’s accepted output. |
| 5 | A noncanonical foreign encoding is accepted, but the harness does not assert that it decoded to 5. It does not test commitment consequences or malformed limb/point encodings. |
| 6 | Bytes32 dispatch mismatch only; the JubjubScalar branch is absent. |
| 7 | Equal JubjubScalar operands exercise the missing constrain-equality arm. |
| 8 | One foreign-high type exercises the dispatch mismatch; low/high range boundaries are absent. |
| 9 | Appropriately documentation-only. |
| 10 | K static rejection and Rust preprocessing rejection. Key-generation underflow/panic behavior is untested. |
| 11 | Graceful off-circuit torsion rejection and K subgroup failure. Neither the fixed declared-input decoding path nor the surviving circuit hint panic is exercised. |
| 12 | Secp256r1 rejection only; Curve25519 is absent. This is an unsupported operation on both sides, not a divergence. |
| 13 | Expected K missing-chip classification. Actual key-generation panic is untested. |
| K1 | Real parity behavior, but the circuit test needs Jubjub enabled; see R3-3. |
| K2 | Real transcript panic. I independently reproduced exit 101 at `ir_vm.rs:379:33`. Public input, guard-true, and multielement truncation variants remain untested. |

The current review retires findings **1, 2, and 4**, and classifies 3 and 12 as non-defects (`A/docs/zkir-v3-divergence-review.md:64`). The K documentation should not describe these as thirteen current divergences.

The preserved receipt `evidence/zkir-k-divergence-tests-2026-09-05.txt:41` contains **13/13**, with no K2 entry. `wiki/zkir-k-semantics-plan.md:80` cites that missing entry as reproduced evidence. This review’s fresh 14-case execution confirms current expectations, but does not repair the saved receipt.

**Gate relations checked by source inspection:**

- `assert` nonzero and `cond_select` boolean conversion match the Rust call sites. The supported equality/select dispatch exclusions match the inspected instruction files.
- The canonical `div_mod_power_of_two` relation follows the canonical bit decomposition at `R/ir_vm.rs:1013`.
- `reconstitute_field` correctly permits field wrap-around in its relation while bounding divisor and modulus separately (`R/ir_vm.rs:1036`).
- Weierstrass identity rejection by `into_coordinates`, and Edwards identity coordinates `(0,1)`, match `R/ir_instructions/into_coordinates.rs:109` and `:125`.
- Exact-coordinate and subgroup requirements for `from_coordinates` match the safe chip paths, conditional on successful construction. Hint panics and missing chips are additional operational behavior.
- The enabled-chip predicates in `usedChips` match the inspected `used_chips` booleans. Missing chips mean construction failure, specifically a **panic** in the pinned accessors; they do not mean unconstrained arithmetic or a returned `Error::Synthesis`.
- Add, multiply, negate, copy, equality, selection, scalar multiplication, native-to-Jubjub reduction, and byte conversions were checked at the typed mathematical-operation level. Internal gate wiring and auxiliary assignments were not independently validated.

**Assignment and encoding abstraction:**

`public_input` and `private_input` correctly ignore the guard in their circuit relation. However, `#typed` checks only a constructor tag (`zkir-constraints.k:255`), and declared program inputs emit no assignment constraints. Actual assignment behavior includes:

| Type family | Actual chip behavior omitted from the explicit K relation |
|---|---|
| Native | Assignment to a native field cell. |
| Bytes32 | Thirty-two byte assignments with eight-bit range checks (`C/field/native/native_gadget.rs:572`). |
| Jubjub/Curve25519 points | Curve checks and subgroup enforcement by cofactor clearing (`C/ecc/native/edwards_chip.rs:842`; `C/ecc/foreign/edwards_chip.rs:562`). |
| Secp256k1/Secp256r1 points | On-curve constraints, with identity handling, through the Weierstrass assignment path (`C/ecc/foreign/weierstrass_chip.rs:343`). |
| Foreign fields | Shifted limb representation and limb range constraints (`C/field/foreign/field_chip.rs:424`). |
| JubjubScalar | Bit assignments; the assigned object explicitly starts with `enforced_canonical: false` (`C/ecc/native/edwards_chip.rs:877`). Assignment alone must not be described as proving integer canonicity below the scalar modulus. |

`encode_incircuit` additionally asserts that extra JubjubScalar encoding batches are zero (`R/ir_instructions/encode.rs:104`). Ordinary scalar assignment at this pin uses 252 bits, so that loop is empty on that path. Scalars built from byte representations can have a longer internal bit vector (`C/ecc/native/edwards_chip.rs:1362`). K’s single scalar integer does not represent those internal bits or the extra assertions.

**Inference:** The instruction-level abstraction loses auxiliary witness choices, representation aliases, range/canonicity enforcement, copy wiring, and hint execution. Equality with an off-circuit hash result cannot independently detect an underconstrained hash gadget.

The page’s limitation to concrete honest-witness checking at `wiki/zkir/zkir-k-definition.md:40` is appropriate, but incomplete: it does not disclose these representation assumptions, and its “whole circuit” claim at line 38 is false for the omissions above. Agda’s conditional theorems do not supply missing correspondence between K, the chip implementation, and assumptions contradicted by K1.

## 4. Questions for the authors

1. Does `violated` mean “this supplied assignment fails,” “no completion of this partial assignment exists,” or “the circuit is unsatisfiable”? These are different predicates; `zkir-constraints.k:15` currently conflates them.
2. Should `synthErr` deliberately abstract both returned synthesis errors and Rust panics? If so, where is that abstraction documented, and how will findings 10, 11, 13, and K2 preserve their operational distinctions?
3. Is there an independent `Relation::circuit`/MockProver harness outside the inspected tools? The named Rust oracle calls only preprocessing (`/home/charl/Moriarty/repos/_build/ledger-92e8bdd3/zkir-oracle/src/main.rs:87`).
4. What explicit invariant restricts checker inputs to valid canonical K values? Without one, constructor tags alone do not encode the stated assignment contracts.
5. Which receipt supports the claim that K2 was reproduced against **both** crates? The current divergence harness selects only the ledger oracle, and the cited receipt omits K2.

## 5. What I checked and how

**Repository observation:** I made no file changes and used no delegation.

I read the supplied repository instructions, `WIKI_SCHEMA.md`, the controlling assignment, `wiki/index.md`, the semantics plan, the definition page, the complete constraint checker, VM emission/evaluation paths, divergence harness and corpus, and supporting value/operation rules. I inspected Rust `Relation::circuit`, `used_chips`, preprocessing, and all relevant instruction dispatch files, including `eq.rs` for `test_eq`. I traced the relevant native/foreign field and curve methods through midnight-circuits 7.2.4, chip accessors through midnight-zk-stdlib 2.3.5, and scalar width through midnight-curves 0.3.1. I also inspected the divergence review, pertinent spec sections, Agda assumptions, and named receipts.

Source inspection used `rg`, `nl -ba`, and bounded `sed -n` ranges.

**Experiment observations:**

- The requested `uv run --offline --no-sync --group zkir-k python -B ...` route failed before execution because uv could not create its cache lock temporary file.
- I then used `.venv/bin/python -B` and pyk’s `llvm_interpret` against the existing compiled definitions. This invokes:

  ```text
  experiments/zkir-k/semantics/zkir-kompiled/interpreter /dev/stdin -1 /dev/stdout
  ```

  Programs were constructed with `zkir_kast.program`; jobs used `preimage_term`, `kast_to_kore(..., KSort("Job"))`, and `top_cell_initializer` with `$PGM` injected from `SortJob` to `SortKItem`. Static checks used the corresponding `zkir-check-kompiled` definition and `SortProgram`.

- I executed all 14 entries imported from `divergence_tests.CASES`, without calling its file-writing `main`. Each was checked against the compiled WF definition, compiled VM, and existing Rust oracle. All 14 matched their current expectations. Target verdicts were read from the complete verdict list.
- Rust oracle calls used existing corpus files with `/dev/stdin`, or pipe-backed `/dev/fd` program input. No temporary program or preimage files were written.
- Additional K runs exercised the precise counterexamples in R3-1 through R3-8: empty impact; failing-impact indices; inversion of zero; comparison widths 252/253/254; constrain-bits widths 255/256; high operand 256; output-type mismatch; surplus hash input; Jubjub coordinates with the chip absent/present; and incorrect/noncanonical-input commitments.
- Direct Rust executions confirmed parity-only decoding, K2’s transcript panic, preprocessing acceptance at comparison width 253, and the two distinct commitment hashes reported in R3-1.

No fresh `kompile`, key generation, MockProver, proof generation, or PLONK-row audit was performed. Circuit construction and constraint consequences beyond these executed K/Rust witness checks are **source-derived inferences**; the proposed independent circuit experiments remain **specified-only**.
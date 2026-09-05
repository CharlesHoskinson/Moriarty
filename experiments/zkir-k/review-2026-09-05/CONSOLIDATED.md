# Consolidated review findings, 2026-09-05

Eight independent reviews of the ZKIR-in-K semantics: four on Claude Fable 5.1
(`reports/fable-R1..R4.md`) and four on OpenAI GPT-6 Astra through Codex in a
read-only sandbox (`reports/astra-R1..R4.md`), two per scope (R1 mathematics and
encodings, R2 VM versus `preprocess`, R3 constraints and divergence tests, R4 K
engineering and documentation). The Codex CLI accepted `gpt-6-astra` (its
configured default, high reasoning effort); the served model cannot be
attested from the event stream, so the requested model is recorded and the
served model is unknown.

Verdicts: astra-R1 WARNING, astra-R2 BLOCKED, astra-R3 BLOCKED, astra-R4 BLOCKED,
fable-R2 WARNING, fable-R3 WARNING (fable-R1 and fable-R4 appended when they arrive).

## Findings accepted for the next iteration, by severity

Each item names the reports that raised it (cross-vendor agreement where two
vendors report the same defect), the defect, and the fix adopted.

### Blockers

1. **Stuck configurations are reported as `ok`** (astra-R2 #1, fable-R2 F1).
   `zkir_run.py` derives success from `<status>` only; a program that no rule
   matches (for example `div_mod_power_of_two` with one output) leaves `#exec`
   in `<k>` and is reported as a successful run with an empty memory. Fix:
   the runner requires `<k>` to be `.K`; anything else is `stuck`, a failure in
   every comparison. Add the missing runtime rule (below) so that the
   configuration cannot get stuck on this program.

### Major

2. **Missing runtime checks that `preprocess` performs** (astra-R2 #3, fable-R2 F1):
   `div_mod_power_of_two` arity and `bits > 248`, `reconstitute_field`
   `bits > 248`, all returned by Rust at run time regardless of any static
   check; the VM relied on `ZKIR-WF`, which `job` never runs. Fix: add the
   three runtime rules with the Rust messages, and evaluate `wf` at the start
   of `job` so that a well-formedness error is a run error (the K side stays
   strictly stricter, as the plan intends); state on the wiki page that the
   VM's own domain is every program `IrSource::load` accepts.
3. **`test_eq` accepts `JubjubScalar` pairs** (astra-R2 #2, fable-R3 #4);
   `test_eq_offcircuit` rejects them. Fix: reject in `testEqV`; add a handmade
   negative; the gate already reports a synthesis error, which was a spurious
   divergence.
4. **Alignment parser narrower than `parse_field_repr`** (astra-R2 #4, fable-R2 F3):
   trailing unused operands are accepted by Rust and rejected by K; `option`
   segments are parsed by Rust off-circuit (selector, chosen alternative, zero
   padding to the widest alternative) and rejected by K. Fix: implement both
   in `alignedBytes`, keep the in-circuit rejection of options in the
   constraint layer, add positive and negative cases.
5. **Differential agreement on error runs is status-only** (astra-R2 #5, fable-R2 F2):
   277 of 314 comparisons were "both failed" with no message comparison. Fix:
   classify both messages into error classes (variable not found, type
   conversion, bit bound, boolean, assertion, equality, decode, transcript
   short, transcript unconsumed, commitment, impact mismatch, unsupported
   operation, output arity, output type, excessive bits, panic) and fail on a
   class mismatch; report success-run agreement and error-run agreement
   separately; compare `<outputs>` and cursors where the oracle exposes them.
6. **Commitment gate is never checked** (fable-R3 #1): `piFixed(1, _)` is
   unconditionally `holds`; the circuit hashes the re-encoded inputs and
   outputs with the opening and pins public input 1 to it. Fix: a `commGate`
   evaluated as `pi[1] == poseidon(rand ++ encode(inputs) ++ encode(outputs))`;
   turn divergence case f05 into a commitment test with `comm=True`.
7. **Unsatisfiable relations reported as synthesis errors** (fable-R3 #2):
   `#matches(vErr(..))` maps every in-circuit evaluation error to `synthErr`,
   including `inv` of zero and the `bytes32_from_low_high` byte bounds, which
   are unsatisfiable constraints of a circuit that exists. Fix: distinguish
   `vUnsat` from `vErr` in the in-circuit evaluators and map it to `violated`.
8. **Post-failure impact bookkeeping** (fable-R3 #3): after a witness failure
   later impacts are reported `violated: index out of range` instead of
   `unknown`, and an impact that itself fails does not advance `<deadPis>`,
   so later gates reuse its indices. Fix: a dedicated `<piIdx>` counter
   advanced at emission time; `unknown` when the vector is incomplete.
9. **`less_than` with 253 or 254 bits** (fable-R3 #5): accepted by K, by WF
   and by `preprocess`, but `bounded_of_element` asserts `n <= 253` after the
   even rounding, so keygen panics. Fix: gate outcome `synthErr` (keygen
   cannot build the circuit) when the padded bound exceeds 253; new divergence
   case; report upstream as a candidate finding.
10. **Native values outside the field are accepted** (astra-R1 major, fable-R2 F11):
    `decodeValue([x], native())` and the preimage builder take any integer.
    Fix: range-check `[0, r)` in `decodeValue` for natives and reject
    non-canonical preimage integers in `preimage_term` with an explicit error.
11. **Rust panics modelled as ordinary errors** (astra-R1 major, astra-R2 #8,
    fable-R2 F5): short transcripts and the two Bytes32 decoder assertions are
    process aborts in Rust. Fix: a third status `panic(msg)`, reported by the
    runner and compared with the oracle's exit-101 outcome; the Bytes32 case
    becomes divergence case k03.

### Minor

12. **Operand-resolution order and error precedence** (astra-R2 #6, fable-R2 F4, F10):
    `checkBits` tests the bound before the type; `output` resolves before the
    arity check; hashes, impact, `less_than`, `reconstitute_field` resolve all
    operands before converting any. Fix: sequential resolve-and-convert per
    operand in the order `preprocess` uses; arity first for `output`.
13. **Error snapshots differ from Rust's partial state** (astra-R2 #7): inputs
    are bound only after all decode; impact pushes are all-or-nothing; the
    commitment PI is seeded before the commitment check fails. Fix the cheap
    ones (bind inputs as decoded, push impact values one at a time, do not
    seed a commitment that is absent) and state that post-failure cells have
    no Rust correspondence beyond that.
14. **`decodeValue` narrower than the underlying decoders on irregular lengths**
    (astra-R1 minor): documented as a fixed-width wrapper; VM paths always
    slice `encodedLen(T)` elements, so no behaviour change.
15. **Chip gating vocabulary** (fable-R3 #6, #7): `synthErr` covers a missing
    dispatch arm, a missing chip (a panic in Rust) and chip-internal asserts;
    `jubjub_scalar_from_native` mints a JubjubScalar that `used_chips` does
    not see (candidate K3, the review's finding 13 says `from_bytes32` is the
    only such entry). Fix: outcome vocabulary documented on the wiki page;
    divergence case k04 for the JubjubScalar chip.
16. **Documentation and comments** (fable-R2 F7, F8, F9; astra-R1): the VM
    overwrites registers like the crate while its header says write-once and
    CLM-0711 describes the static check; the byte-order comment in
    `bytes_from_field_repr` contradicts the (correct) code; dead rules;
    "paired partial rounds" should read "three partial rounds per batch";
    the divergence receipt says 13 cases while the tool has 14. Fix all.

### Added from astra-R3 and astra-R4

17. **Empty `impact` emits no guard-booleanity gate** (astra-R3 R3-2): the
    circuit converts the guard to a bit before the operand loop. Fix: emit a
    `guardGate(g)` for every impact, independently of the PI gates.
18. **Chip dependencies of `from_coordinates` and `load_constant`** (astra-R3
    R3-3, astra-R4 R4-03): the native/native `from_coordinates` arm needs the
    Jubjub chip, which `used_chips` does not enable for it; `load_constant` of
    a JubjubScalar needs it too. Fix: check the output curve's chip in every
    `from_coordinates` arm and the constant's chip in `load_constant`; split K1
    into a missing-chip case and a parity case with the chip enabled.
19. **Chip width assertions** (astra-R3 R3-4, fable-R3 #5): `less_than`
    padded widths above 253 and `constrain_bits` at 255 versus above 255 are
    construction failures the checker calls `holds`. Fix as in item 9, plus
    `constrain_bits` with bits > 255 as a synthesis error.
20. **In-circuit aligned decoder differs from the off-circuit one** (astra-R3
    R3-6): `fab_decode_to_bytes` ignores surplus fields and rejects options and
    compress atoms; the checker reuses the off-circuit parser. Fix: a separate
    `alignedBytesCircuit` in the constraint layer.
21. **`output` gate and the catch-all gate rule hold unconditionally**
    (astra-R3 R3-7): synthesis checks arity and per-position runtime types.
    Fix: an `outputGate` carrying the signature, evaluated for arity and types;
    replace the catch-all `holds` with `unsupported(gate)`.
22. **Extension `cond_select` on Bytes<n>** (astra-R4 R4-03): only `Bytes<32>`
    was rejected by the checker; every byte-string length lacks an in-circuit
    arm. Fix: reject all byte strings in the select support predicate.
23. **Extension `test_eq` on unequal-length Bytes** (astra-R4 R4-04): the
    crate compares vectors off-circuit and returns false; K errors on the type
    mismatch. Fix: compare byte strings of any two lengths off-circuit; keep
    the in-circuit equal-length requirement in the checker.
24. **Preprocessor mismatches with `IrSource::load`** (astra-R4 R4-05): an
    omitted `guard` member is a valid absent guard; unknown fields are
    ignored by serde; `const_hex::decode` rejects whitespace; the version
    components must be integers (`u8`); `inputs`, `outputs` and
    `instructions` must be JSON arrays; the extension's `Bytes<n>` requires
    canonical ASCII digits. Fix all six.
25. **Unjustified `total` attributes** (astra-R4 R4-07): `fmod(1, 0)`,
    `shaK(64)`, `rotl64(1, -1)` and others abort or get stuck. Fix: guard the
    domains (return a defined value outside them) or drop `total` where the
    function is genuinely partial; document the domain invariants.
26. **Fixture extraction regex** (astra-R4 R4-09): `inputs:` matched the
    suffix of `public_transcript_inputs:`, recording empty raw inputs for
    fifteen fixtures. Fix the regex with an identifier boundary, regenerate
    the manifest, rerun.
27. **Extension memory comparison is not injective** (astra-R4 R4-08):
    `Bytes` values of different lengths can share an encoding. Fix: include
    the type (with length) in the comparison record on both sides.
28. **`concat` maximum length** (astra-R4 R4-10): reject results above 2^24
    bytes off-circuit and in the gate.
29. **Overlapping function rules with different results** (astra-R4 R4-11):
    `#checkArity` for div/mod with both defects, `#nonResidue` beyond its
    range, the typed and generic fallbacks of `#boolFold` and `#concat`. Fix:
    disjoint conditions or explicit precedence.
30. **Divergence harness rigor** (astra-R3 R3-9, fable-R3 #8): the harness
    infers `holds` when no matching non-holding verdict exists, never
    checking that the gate was emitted; K2 does not check the panic site.
    Fix: return all verdicts from the runner, select the target gate exactly,
    require a finished run, and match the Rust panic location.
31. **Documentation claims to narrow** (astra-R4 R4-12, astra-R3 §3):
    "whole circuit" and "every instruction on every type", the parser
    equivalence claim, the Poseidon equivalence inference, the divergence
    receipt with 13 cases, the k-rust diagnosis; the review's findings 1, 2
    and 4 are retired and 3 and 12 are non-defects, so the page must not call
    them thirteen current divergences; the representation assumptions of the
    instruction-level abstraction (auxiliary witnesses, canonicity of
    JubjubScalar assignment, hash gadgets) must be listed.

## Coverage additions adopted

- Retained regression tests for every counterexample above (handmade negatives
  and divergence cases), including the transcript-type matrix, mixed impacts,
  duplicate destinations, inactive impacts with undefined operands.
- A second perturbation that re-encodes a valid random value of the declared
  type at a random input position; one run per program with a wrong public
  transcript input and one with a wrong commitment.
- Drop empty seed preimages; hash boundary cases (empty message, block
  boundaries) as unit tests; square-root matrix per modulus.

## Not adopted, with reasons

- Reproducing Rust's exact partial memory at failure beyond the cheap cases
  in item 13: the crate does not expose that state, so it cannot be checked.
- Modelling prover-side panics inside chips generally: out of scope of a
  constraint-level checker; the vocabulary section states the boundary.

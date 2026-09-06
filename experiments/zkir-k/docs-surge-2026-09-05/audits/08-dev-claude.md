# Audit of experiments/zkir-k/docs/08-constraints-and-verdicts.md (developer)

Verdict: REVISE
(one blocking and two major findings, all local rewrites; the structure, the
rule names, the messages, the chip table, the width limits, the commitment
section and every number check out against the sources)

Checks performed:
- Read in full: briefs/AUDIT-COMMON.md, briefs/AUDIT-dev.md, briefs/COMMON.md, briefs/08.md, the chapter, semantics/zkir-constraints.k (497 lines), semantics/zkir-vm.k (549 lines), tools/divergence_tests.py, tools/diff_test.py, and all 20 programs under corpus/divergence/.
- Compared the Constraint and Outcome constructors and every quoted message with zkir-constraints.k lines 34-42, 96-135, 206-221, 225-230, 242-244, 282-292, 312-318, 341, 359, 392-400, 428-429, 448, 474-475, 494.
- Compared the emission rules (bindGate, commGate, gate(I), guardGate, piGate, outputGate, isSpecialEmit, #piGates) with zkir-vm.k lines 105-109, 143-159, 172-192.
- Compared the chip table with zkir-constraints.k lines 46-85, zkir-ext.k lines 273-275, and `IrSource::used_chips` in repos/_build/ledger-92e8bdd3/zkir-v3/src/ir_vm.rs lines 1212-1262.
- Compared the width limits with ir_vm.rs lines 844-851, 966, 1036-1043 and midnight-circuits 7.2.4 src/field/native/native_gadget.rs lines 330-344, 885-896; #frBits, #frBytesStored and #checkArity in zkir-syntax.k lines 126-129, 293-306; checkBits in zkir-ops.k lines 340-343.
- Compared the commitment section with zkir-vm.k lines 527, 544-547, zkir-hash.k lines 74-75, zkir-constraints.k lines 101-121, and `encode_incircuit` in ir_vm.rs lines 1187-1195.
- Confirmed `AssignedScalarOfNativeCurve` exists in midnight-circuits 7.2.4 (src/ecc/native/edwards_chip.rs) and that `invV` has no JubjubScalar arm (zkir-ops.k lines 77-85).
- Ran `uv run --group zkir-k python experiments/zkir-k/tools/divergence_tests.py` from the repository root: 20/20 PASS; every case-to-outcome claim in the chapter (f01, f02, f04, f05, f10, f11, f13, k01, k01b, k04, k05, k07) matches the printed outcome.
- Ran the chapter's `zkir_run.py` command on corpus/handmade/transient_hash.zkir: status ok, constraints 8, verdicts 8, violations [], gates bindGate(0), four transientHash, three hashToCurve, exactly as described.
- Ran a scratch program with a one-output signature and a two-operand `output`: status error, `outputGate` verdict synthErr with the arity message, confirming the "error runs may carry non-holding verdicts" claim.
- Confirmed the receipt numbers: evidence/zkir-k-differential-92e8bdd3-2026-09-05b.txt (358 comparisons, 46 successful-run agreements, "oracle 2: 0 successful K runs with a non-holding gate"), evidence/zkir-k-differential-ext-2ffe2d1-2026-09-05b.txt (418, 50, 0), evidence/zkir-k-divergence-tests-2026-09-05b.txt (20/20).
- Confirmed the verdict fields of tools/zkir_run.py lines 333-350 (constraints, verdicts, all_verdicts, violations) and the oracle2_flags handling of tools/diff_test.py lines 184, 227-230, 272.
- Confirmed the K4 label: wiki/zkir/zkir-k-definition.md line 58 and the wiki/contradictions.md row citing cases k04 and k01b.
- Hygiene: one `#` title, no em-dash, no placeholder or drafting vocabulary; word count outside tables, code and the maintainers section is 2309.

## Findings

### F1 blocking "What `holds` establishes", last paragraph
Claim: "A `holds` verdict on `assert` of a non-boolean non-zero, on `public_input` whose off-circuit guard was false, or on `less_than` whose off-circuit bit bound failed, is therefore expected".
Evidence: When the off-circuit bit bound of `less_than` fails, the witness stops before writing the output register, and the gate's last conjunct `#matches(#ltOf(...), rdId(O, M), ...)` (zkir-constraints.k lines 314-315, 195-196) yields `unknown`, not `holds`. The divergence run prints `f04_less_than_odd_bits ... gate=less_than:unknown` with detail `unknown: register %lt is not in the witness`, and the chapter's own "Outcomes" section says the same about f04. The sentence contradicts the tool output and the chapter.
Fix: Remove `less_than` from the list, or replace the clause with: "on `less_than` whose off-circuit bit bound failed the verdict is `unknown`, because the output register is absent, even though the padded relation would have accepted the inputs (case `f04`)".

### F2 major "What `holds` establishes", second bullet
Claim: "`from_coordinates` and `into_coordinates` re-check on-curve and subgroup facts because those circuit arms do."
Evidence: The `into_coordinates` gate (zkir-constraints.k lines 368-376) is `#coordsMatch`, which rejects only the secp256k1 and secp256r1 identity (`violated("into_coordinates of the identity is unsatisfiable")`) and otherwise compares the two output registers with `intoCoordinatesV` (zkir-ops.k lines 154-162). No on-curve or subgroup test is applied. Only `#fromCoordsPt` (lines 397-400) performs those checks.
Fix: "`from_coordinates` re-checks that the exact `(x, y)` is on the curve and in the prime-order subgroup (`#fromCoordsPt`). `into_coordinates` only rejects the secp256k1 and secp256r1 identity (`#coordsMatch`) and compares the coordinates."

### F3 major "Outcomes", paragraph on `alignedBytesCircuit`
Claim: "`alignedBytesCircuit` keeps only `atom` segments, ignores surplus fields, and rejects `option` ...".
Evidence: zkir-constraints.k lines 473-475. The success rule fires only when `#onlyAtoms(Segs) ==K Segs` (every segment is an atom) and no compress atom is present; an alignment containing any `option` segment goes to the `bErr("synthesis: in-circuit decoding of alignment options is not yet implemented")` rule. `#onlyAtoms` is a test, not a filter: option segments are never dropped and the remaining atoms are never decoded. "Keeps only atom segments" reads as if options were skipped and decoding continued.
Fix: "`alignedBytesCircuit` decodes an alignment only when every segment is an `atom` and none is a `compressAtom`; it ignores surplus fields (`#abcOk` drops the remainder of the list). Any `option` segment is rejected with ... and any `compressAtom` with ...".

### F4 minor "Outcomes", paragraph on `#fromCoordsChip`
Claim: "`#fromCoordsPt` then demands that the exact `(x, y)` lie on the curve and, after cofactor clearing, in the prime-order subgroup."
Evidence: `inSubgroup` (zkir-curves.k lines 82-84) is `isIdentity(N * Q)` for the Edwards curves and unconditionally `true` for the Weierstrass curves; the point is never multiplied by the cofactor before the output comparison (`#wrapPoint(Q, T)` at line 400 wraps `Q` itself). "After cofactor clearing" suggests the cleared point is what the gate compares, which would make case f11 `holds` instead of `violated`.
Fix: "and that the point itself lies in the prime-order subgroup (`inSubgroup`, which for Jubjub and Curve25519 tests that the subgroup order times the point is the identity, and is always true for secp256k1 and secp256r1)".

### F5 minor "Width limits", `constrain_bits` paragraph
Claim: "`wf` rejects `bits >= 255`, so `checkedJob` never reaches that gate."
Evidence: True, but for a broader reason the reader should be told: a `wfError` sets `<status>` to `error` and `job` then rewrites to `.K` (zkir-vm.k lines 101-103), so `checkedJob` emits and evaluates no gate at all for such a program (the divergence tool reports `n/a`). As written, a reader may think the other gates are still evaluated.
Fix: "`wf` rejects `bits >= 255`, and a failed static check ends `checkedJob` before any gate is emitted or evaluated, so this gate is only reachable through `job`."

### F6 minor "Communications commitment", `#commOutcome` paragraph
Claim: "`synthErr("chip not initialised: poseidon")` if Poseidon is not in `<chips>`".
Evidence: `commGate` is emitted only when `<doComm>` is true (zkir-vm.k lines 146-159) and `usedChips` adds `"poseidon"` whenever the flag is true (`#commChip(true)`, zkir-constraints.k line 50). Under `job` the branch cannot fire. A developer looking for a triggering program will not find one.
Fix: Add "(unreachable from `job`, because `usedChips` enables Poseidon whenever the flag is set; the rule exists so `eval` is total)".

### F7 minor "How the harness uses verdicts", `divergence_tests.py` paragraph
Claim: "selects one gate per case exactly: the pretty-printed constraint must start with `gate(<op>(` or with the constructor name".
Evidence: tools/divergence_tests.py lines 224-231: the comparison is a prefix match after removing underscores and spaces and lower-casing both sides, and the first matching verdict (`targets[0]`) is taken when several match. "Exactly" overstates it.
Fix: "selects the first verdict whose pretty-printed constraint, with underscores and spaces removed and lower-cased, starts with `gate(<op>(` or with the constructor name (`commGate(`, `guardGate(`)".

### F8 minor Cross-references
Claim: "13-known-divergences.md" is cited five times.
Evidence: experiments/zkir-k/docs/ currently holds chapters 01 to 09 only. The reference follows the common brief's chapter list, so no change to this chapter is needed; the editor should make sure the K1 and K4 labels and the case names used here (`f05`, `k01`, `k01b`, `k04`, `k05`, `k07`) appear in that chapter when it exists.
Fix: None in this chapter; note for the editor.

### F9 minor Length
Claim: The brief asks for about 1500 words outside tables and code.
Evidence: 2309 words outside tables, code blocks and the maintainers section.
Fix: Optional trim; the "Outcomes" section repeats several helper descriptions (`#need`, `#matches`, `#and`) that appear twice.

## Coverage
- Constraint sort, six constructors, when each is emitted with pointers to zkir-vm.k rules: covered.
- Outcome sort with precise meanings and the rule shapes (`eval/4`, `eval/6`, `#piEq`, `#invNonZero`, `isZeroField`, `#fromCoordsChip`, `#lowHighBounds`, `alignedBytesCircuit`, owise unsupported): covered (F3 and F4 correct two of the descriptions).
- Chip gating (`<chips>`, `usedChips`, instruction to chip table, K4 keygen panics): covered.
- Width limits (less_than padded bound above 253, constrain_bits above 255, div_mod and reconstitute bit counts): covered.
- Commitment gate versus witness-side commitment and the non-canonical split: covered.
- What a verdict does and does not establish (auxiliary cells, copy wiring, assignment constraints, JubjubScalar canonicity, hash internals, prover-side panics): covered (F1 and F2 correct two statements).
- How the harness uses verdicts (no successful run with a non-holding gate; divergence cases select one gate): covered.

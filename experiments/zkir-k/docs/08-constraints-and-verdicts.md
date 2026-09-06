# Constraints and verdicts

A run of the definition builds two things from the same instruction list: the off-circuit witness of `IrSource::preprocess`, and a list of instruction-level in-circuit relations. After the last instruction, every relation is evaluated on the final memory, the public-input vector and the chip set, giving one `verdict(Constraint, Outcome)` per emitted constraint. A run whose status is `ok` and whose every verdict is `holds()` establishes that this honest witness satisfies those relations, and nothing more: not that no other witness does, not that the Halo2 rows the crate would synthesise are uniquely determined, not that a proof exists. The checker establishes completeness of an instruction-level abstraction of `Relation::circuit` in `ir_vm.rs` at midnight-ledger 92e8bdd3 (plan decision CLM-0721).

The module is `ZKIR-CONSTRAINTS` in `zkir-constraints.k`; emission lives in `zkir-vm.k`. Per-instruction relations of the base surface are in 07-instruction-reference.md, the extension `eval` rules of `zkir-ext.k` in 09-extension-surface.md, and the off-circuit versus in-circuit splits in 13-known-divergences.md.

## Instruction-level constraints

`Constraint` has six constructors. None of them is a PLONKish gate row. `gate(I)` is the relation that `Relation::circuit` and the matching `*_incircuit` function enforce for instruction `I`, read off the chip calls they make.

| Constructor | Relation |
|---|---|
| `gate(Instr)` | the in-circuit relation of that instruction over registers and immediates |
| `piGate(Int, Operand, Operand)` | public input `i` equals `g ? x : 0`, with `g` boolean and `x` Native |
| `guardGate(Operand)` | the impact guard is boolean, including an empty impact |
| `bindGate(Int)` | public input `i` equals the preimage binding input |
| `commGate(Int, TypedIds, Int)` | public input `i` equals Poseidon of the opening, the re-encoded input registers and the encoded outputs |
| `outputGate(Operands, IrTypes)` | output arity and per-position runtime type against the program signature |

The cells in `zkir-vm.k` are `<constraints>` (emitted list), `<chips>` (chip set), `<verdicts>` (evaluated list) and `<piIdx>` (constraint-side public-input counter). `job` sequences `#loadInputs`, `#seedPi`, the instruction list and `#verdicts`. Instructions still emit after a witness failure: `#exec` is a no-op when `<status>` is `error` or `panic`, and later verdicts that need a missing register or public-input slot become `unknown` rather than `violated`. A failed static check under `checkedJob` sets `<status>` to `error` first and `job` then rewrites to `.K`, so no gate is emitted or evaluated.

### When each constructor is emitted

`#seedPi` always appends `bindGate(0)`. When `<doComm>` is true it also appends `commGate(1, Ins, Rand)`, where `Ins` is the program input list and `Rand` is the opening, or `0` if the commitment is absent (the witness half then fails with `"Expected communications commitment"`). The witness half seeds `<pi>` with the binding input and, while still alive and when a commitment is present, with the commitment value. After an earlier decode failure the gates are still emitted and `<piIdx>` still advances to 1 or 2.

Every ordinary instruction emits `gate(I)` before `#exec(I)`:

```k
rule <k> I:Instr => #exec(I) ... </k> <constraints> Cs => Cs ListItem(gate(I)) </constraints>
  requires notBool isSpecialEmit(I)
```

`isSpecialEmit` is true only for `impact` and `output`. `impact(G, Xs)` appends `guardGate(G)` and then one `piGate` per operand from the current `<piIdx>`, which advances by `lenOperands(Xs)` at emission (`#piGates`); an empty impact still emits `guardGate(G)`. `output(Vs)` appends `outputGate(Vs, Ts)` with `Ts` from `<outTypes>`.

## Outcomes

`Outcome` has five constructors. `eval` is total. `verdicts` maps each constraint to `verdict(C, eval(C, mem, pi, chips, outputs, binding))`; `violations` is the sublist whose outcome is not `holds()`.

| Outcome | Meaning |
|---|---|
| `holds()` | the relation is satisfied on this witness |
| `violated(msg)` | a circuit for this instruction exists and rejects the witness |
| `synthErr(msg)` | the circuit cannot be built |
| `unknown(msg)` | a register or public-input slot the gate reads is absent |
| `unsupported(msg)` | no in-circuit relation is modelled for this gate |

`eval/6` (`eval(Constraint, Map, List, Set, List, Int)`) handles the three gates that need the outputs or the binding input. `bindGate(I)` is `holds` when `pi[I]` equals the binding, `unknown("public input vector incomplete")` when the vector is shorter than `I + 1`, and `violated("binding input differs")` otherwise. `commGate` is described below. `outputGate` is `synthErr("Output: signature declares N return values but instruction has M")` on an arity mismatch; otherwise `#outputTypes` is `holds` on matching runtime types, `synthErr("Output position I: signature declares T but operand has runtime type U")` on a mismatch, and `unknown` if a register is missing. Every other constraint falls through to `eval/4`, the per-instruction relation.

`eval/4` rules are conjunctions built with `#and`, which returns the first non-`holds` outcome. `rd` reads an operand; an absent name is `vErr("register X is not in the witness")`, which `#need` turns into `unknown`. `#matches(expected, register, what)` maps a computed `vErr` or `vPanic` to `synthErr` (the in-circuit operation is undefined on these types), a missing register to `unknown`, equal values to `holds` and unequal values to `violated`. `#boolean` accepts only `native(0)` and `native(1)`: another Native value is `violated`, a non-Native value is `synthErr`, as for `#native`. `#chipFor` and `#chipNamed` are `synthErr` when the required chip is absent from `<chips>`.

`#piEq` (from `piGate`) is `unknown` when index `I` is past the public-input vector, `holds` when `pi[I]` equals `#guarded(g, x)` (`1` selects `x`, anything else yields `0`), and `violated("public input I differs from the guarded value")` otherwise. A non-boolean guard is `violated("impact guard is not boolean: V")` on both `guardGate` and `piGate` (case `k06`).

`#invNonZero` uses `isZeroField`: `inv` of a zero field element is `violated("inv: a * inv = 1 has no solution at zero")`. `isZeroField` is true of `native(0)` and of the six foreign field zeros, and false of every other value, including `jubjubScalar(0)`, whose inversion has no in-circuit arm and becomes `synthErr` through `#matches`.

`#fromCoordsChip` requires the Jubjub chip for native/native `from_coordinates`; other coordinate types rely on `#chipsForValues` of the operands. `#fromCoordsPt` then demands that the exact `(x, y)` lie on the curve and that the point itself lie in the prime-order subgroup: `inSubgroup` in `zkir-curves.k` tests that the subgroup order times the point is the identity for Jubjub and Curve25519, and is always true for secp256k1 and secp256r1. No cofactor is cleared before that test or before the output comparison. Failures are `violated("from_coordinates: (x, y) is not on the curve")` (case `k01`; off-circuit Jubjub decompression uses only the parity of `x`, K1 in 13-known-divergences.md) and `violated("from_coordinates: point is not in the prime-order subgroup")` (case `f11`, the order-2 Curve25519 point).

`#lowHighBounds` makes `bytes32_from_low_high` unsatisfiable, not unbuildable, when the low operand's byte 31 is non-zero or the high operand is not a byte. High must be Native; a foreign high is `synthErr` (`#native`, case `f08`).

`alignedBytesCircuit` decodes an alignment only when every segment is an `atom` and none is a `compressAtom`; surplus fields are ignored (`#abcOk` drops the remainder of the list). Any `option` segment is rejected with `"synthesis: in-circuit decoding of alignment options is not yet implemented"` and any `compressAtom` with `"synthesis: Cannot decode compressed value from field elements"`; both become `synthErr` through `#matches`. Off-circuit `alignedBytes` in `zkir-ops.k` parses options (case `k07`).

The `[owise]` rule is `unsupported("no in-circuit relation modelled for this instruction: " +String #opName(I))`. `#opName` has only an `[owise]` rule returning `"instruction"`, so the message never names the constructor. `impact` and `output` never take that path, because they do not emit `gate(I)`.

`synthErr` also covers a missing in-circuit dispatch arm (`#eqSupported`: `JubjubScalar` equality, case `f07`, and `Bytes32` `cond_select`, case `f06`), a chip that `usedChips` did not initialise, a `less_than` padded bound above 253, a `constrain_bits` width above 255, and `div_mod_power_of_two` with a number of outputs other than 2.

`assert` is `cond != 0` in circuit (`#nonZero`); off-circuit `preprocess` demands a boolean, so `assert` of 2 fails the witness while the gate `holds` (case `f02`). `public_input` and `private_input` check that the register has the declared type (`violated` on a mismatch) and that the type's chip is initialised; the guard is not part of the gate (case `f03`).

## Chip gating

`<chips>` is set at the start of `job` to `usedChips(program(...))`, which follows `IrSource::used_chips` in `ir_vm.rs`. A foreign chip is enabled when its types occur among the program inputs or among `public_input` / `private_input` instructions (`#chipsOfTypes`). Named chips are enabled by selected instructions (`#chipsOfInstrs`) and by the communications-commitment flag (`#commChip`).

| Chip name in `<chips>` | Enabled by |
|---|---|
| `"jubjub"` | a JubjubPoint or JubjubScalar input or public/private input, or `hash_to_curve` |
| `"poseidon"` | `do_communications_commitment`, `transient_hash`, or `hash_to_curve` |
| `"sha2_256"` | `persistent_hash` |
| `"keccak_256"` | `keccak256` |
| `"secp256k1"` | a secp256k1 Point, Base or Scalar input or public/private input |
| `"p256"` | a secp256r1 Point, Base or Scalar input or public/private input |
| `"curve25519"` | a Curve25519 Point, Base or Scalar input or public/private input |
| `"sha2_512"` | `sha512` (extension surface only, `zkir-ext.k`) |

`chipOfType` maps those eleven curve types onto the four curve chips and every other type, including Native and Bytes32, to the empty set. Value gates call `#chipsForValues` or `#chipFor`; hash gates call `#chipNamed`. `from_bytes32` checks `#chipFor` of its target type, `jubjub_scalar_from_native` checks `#chipFor(jubjubScalar(), Chips)`, native `from_coordinates` checks `#fromCoordsChip`, which is `#chipFor(jubjubPoint(), Chips)`, and the extension `load_constant` gate in `zkir-ext.k` checks `#chipFor` of its type.

`usedChips` does not look at any of those four instructions. A program that first introduces a chipped type through one of them succeeds in `preprocess`, while the crate's chip accessor (`std.jubjub()`, `std.secp256k1()`, ...) panics at keygen. The gate is `synthErr("chip not initialised for T")` or `synthErr("chip not initialised: name")`. The cases are `f13` (`from_bytes32` onto Secp256k1Base), `k04` (`jubjub_scalar_from_native`) and `k01b` (native `from_coordinates` without a Jubjub input); the last two are K4 in 13-known-divergences.md.

## Width limits

The static check `wf` and the `job` run-time rules follow `preprocess`; the gates follow the chip assertions that keygen would hit.

`less_than` pads the requested width with `#ltBits(N) = maxInt(N +Int (N modInt 2), 4)`, matching `max(bits + bits % 2, 4)` of `std.lower_than` in `ir_vm.rs`. `bounded_of_element` in midnight-circuits 7.2.4 then asserts that the padded width is at most `MAX_BOUND_IN_BITS = F::NUM_BITS - 2 = 253`. The gate is `synthErr("less_than: padded bound P exceeds MAX_BOUND_IN_BITS = 253")` when that fails; `bits = 253` pads to 254 (case `k05`). `preprocess` (`checkBits` in `zkir-ops.k`) and `wf` (`#checkArity` in `zkir-syntax.k`) only reject `bits >= 255` (`#frBits`). An odd width such as 3 is padded to 4, so the padded relation accepts a value that fails the off-circuit 3-bit bound; the witness then stops before writing the output and the gate is `unknown` (case `f04`).

`constrain_bits` calls `assigned_to_le_bits` with `Some(bits)` and `enforce_canonical = bits >= FR_BITS`. That function asserts `nb_bits <= F::NUM_BITS` (255). The gate is `synthErr("constrain_bits: width N exceeds 255")` when `N > 255`. At `N <= 255` it requires a Native value that fits in `N` bits, except that `#bits` is `holds` whenever `N >= #frBits`, so a 255-bit request is a full-width decomposition. `wf` rejects `bits >= 255`, and a failed static check ends `checkedJob` before any gate exists, so this `synthErr` is reachable only through `job`.

`div_mod_power_of_two` and `reconstitute_field` reject `bits > #frBytesStored *Int 8` (248) in both `wf` (`#checkArity`) and `#exec` (`"Excessive bit count"`). The `div_mod_power_of_two` gate does not repeat that check; it requires Native input, two outputs (else `synthErr("Unexpected output length of DivModPowerOfTwo instruction")`), and outputs equal to `highBits` and `lowBits`. The `reconstitute_field` gate requires Native inputs, `#bits` of the divisor in `255 - N` bits, `#bits` of the modulus in `N` bits, and an output equal to `((D << N) + Mo) mod r`. Off-circuit `preprocess` also rejects a reconstituted value above `r - 1`; the in-circuit sum wraps, so overflow leaves the output absent and the gate `unknown` (case `f01`). At 256 bits and above the crate's keygen would underflow `FR_BITS - bits`; `wf` rejects such a program first (case `f10`, outcome `n/a`).

## Communications commitment

The witness-side check `#commCheck` in `zkir-vm.k` follows `preprocess`: it hashes the raw preimage input stream concatenated with the encoded outputs,

```k
transientCommit(#preInputs(Pre) #encodeAll(Os), Rand)
```

which is `poseidonHash(ListItem(Rand) Vs)` in `zkir-hash.k`. Failure is the run error `"Communications commitment mismatch"`. Generation mode records the same hash in `<needs>` instead of checking it.

The in-circuit gate `commGate(I, Ins, Rand)` re-encodes the assigned input registers with `encodeValue` and hashes `rand ++ encode(inputs) ++ encode(outputs)`, matching the `encode_incircuit` loop in `Relation::circuit`. Evaluation is `#commOutcome`: `unknown` if a register is missing or the public-input vector is short, `synthErr("chip not initialised: poseidon")` if Poseidon is not in `<chips>`, `holds` when `pi[I]` equals that hash, and otherwise `violated("communications commitment differs from poseidon(rand ++ encode(inputs) ++ encode(outputs))")`. The `synthErr` branch is unreachable from `job`, because `commGate` is emitted only when `<doComm>` is true and `usedChips` enables Poseidon whenever that flag is set; the rule exists so that `eval` is total.

The two hashes agree when every input encoding is canonical. They differ when a foreign-field input is a non-canonical limb encoding of a reduced value: `preprocess` hashes the raw stream and accepts, while the circuit hashes the canonical re-encoding of the assigned value and rejects. Case `f05` (`corpus/divergence/f05_noncanonical_foreign_limbs.zkir`) is that split: status `ok`, `commGate` `violated`. A `holds` on `commGate` therefore proves agreement with the circuit's commitment, not with the raw-stream commitment of `preprocess`.

## What `holds` establishes

A finished `ok` run whose `violations` list is empty proves that every emitted instruction-level relation is satisfied by the final `<mem>`, `<pi>` and `<outputs>`, that every chip those relations need is in `usedChips` of this program, that the widths they ask of the native gadget are inside the chip limits the checker models, and that the public-input vector matches the binding input, the guarded impacts, and, when the flag is set, the circuit-side communications commitment.

It does not prove the following, which the instruction-level abstraction omits.

- Auxiliary advice cells and copy wiring. Equality of two registers in a `gate` is value equality, not a permutation argument between named Halo2 cells.
- Assignment constraints of each type: byte range checks, on-curve and cofactor constraints at assignment, and foreign-limb ranges. A value that `preprocess` placed in a register is treated as already well-typed. Only `from_coordinates` re-checks that the exact `(x, y)` is on the curve and in the prime-order subgroup (`#fromCoordsPt`). `into_coordinates` (`#coordsMatch`) rejects the secp256k1 and secp256r1 identity and compares the two output registers with `intoCoordinatesV`; it does not re-check curve or subgroup membership.
- Canonicity of an assigned JubjubScalar. `AssignedScalarOfNativeCurve` is not proven canonical by assignment alone. `jubjub_scalar_from_native` reduces modulo `#rJ` and checks the chip, not a canonical bit decomposition.
- Hash gadget internals. Hash gates compare the output with the concrete functions of `zkir-hash.k`. They do not replay round constraints, MDS wiring or sponge padding as Halo2 rows.
- Prover-side panics inside chips. A hint that would abort the prover while the relation is unsatisfiable is `violated` (case `f11`). A missing chip or a width assert that aborts keygen is `synthErr`. Other panics inside midnight-circuits are not modelled.

A `holds` verdict on `assert` of a non-boolean non-zero, or on `public_input` whose off-circuit guard was false, is therefore expected: the circuit relation is weaker than, or different from, the witness check. When the off-circuit bound of `less_than` fails, the verdict is `unknown` rather than `holds`, because the output register is absent even though the padded relation would have accepted the inputs (case `f04`).

## How the harness uses verdicts

`tools/zkir_run.py` derives the run status from `<status>` and `<k>` alone. It pretty-prints each entry of `<verdicts>` into `all_verdicts` (triples `(outcome, message, gate)`) and `violations` (the triples whose outcome is not `holds`); `constraints` and `verdicts` are the lengths of the two cells. From the repository root, the handmade Poseidon program emits eight constraints and eight `holds` verdicts:

```
printf '%s\n' '{"inputs": ["5", "123456789", "987654321987654321"]}' > /tmp/transient_hash.json
uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py \
  experiments/zkir-k/corpus/handmade/transient_hash.zkir \
  /tmp/transient_hash.json
```

The object has `"status": "ok"`, `"constraints": 8`, `"verdicts": 8` and `"violations": []`: `bindGate(0)`, four `transientHash` and three `hashToCurve`.

`tools/diff_test.py` appends an `ok` K run whose `violations` is non-empty to `oracle2_flags` and does not fail the comparison on that list. It prints `oracle 2: N successful K runs with a non-holding gate`, where successful means status `ok`. The receipts `evidence/zkir-k-differential-92e8bdd3-2026-09-05c.txt` and `evidence/zkir-k-differential-ext-2ffe2d1-2026-09-05c.txt` both report `N = 0`, over 358 base-surface comparisons (46 successful-run agreements) and 418 extension-surface comparisons (50 successful-run agreements). Error runs may carry non-holding verdicts: an `output` arity mismatch is `synthErr` on `outputGate` while the status is `error`.

`tools/divergence_tests.py` selects one gate per case: the first verdict whose pretty-printed constraint, with underscores and spaces removed and lower-cased, starts with `gate(<op>(` or with the constructor name (`commGate(`, `guardGate(`). A missing emission is `no-such-gate`, not an inferred `holds`. The expected outcome is `holds`, `violated`, `synthErr`, `unknown`, or `n/a` when `checkedJob` stops on well-formedness before `#verdicts` (case `f10`). The receipt `evidence/zkir-k-divergence-tests-2026-09-05c.txt` records 20/20 matches.

Evidence that the honest witness satisfies the modelled circuit relations therefore requires both a finished `ok` run and an empty `violations` list, produced from gates that were actually emitted; status alone is not enough (case `f05` is `ok` with a `violated` gate). Even together they are not a proof of unique witnesses, of gadget soundness, or of keygen success on a program whose chips `used_chips` did not enable.

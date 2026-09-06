# Constraints and verdicts

A run of the definition builds two things from the same instruction list: the off-circuit witness of `IrSource::preprocess`, and a list of instruction-level in-circuit relations. After the last instruction, every relation is evaluated on the final memory, the public-input vector and the chip set, and each emitted constraint receives one `verdict(Constraint, Outcome)`. The run then decides whether the final memory is a point of the modelled witness space: every verdict is `holds()` or `unconstrained(msg)`, and every register holds a value satisfying the semantic invariant of its type. Two claims relate that space to the circuit (the section "What the witness space establishes"): the soundness direction, that every cell assignment the circuit accepts projects to a memory of the space, and the tightness direction, that every memory of the space is accepted; both are tested against the crate's circuit under the MockProver, never proved. A run establishes nothing about other witnesses: not that no other witness satisfies the relations, not that the Halo2 rows the crate would synthesise are uniquely determined, not that a proof exists. The checker models an instruction-level abstraction of `Relation::circuit` in `ir_vm.rs` at midnight-ledger 92e8bdd3 (plan decision CLM-0721).

The module is `ZKIR-CONSTRAINTS` in `zkir-constraints.k`, and the emission rules are in `zkir-vm.k`. Per-instruction relations of the base surface are in 07-instruction-reference.md, the extension `eval` rules of `zkir-ext.k` in 09-extension-surface.md, and the off-circuit versus in-circuit splits in 13-known-divergences.md.

## Instruction-level constraints

`Constraint` has seven constructors. None of them is a PLONKish gate row. `gate(I)` is the relation that `Relation::circuit` and the matching `*_incircuit` function enforce for instruction `I`, read off the chip calls they make.

| Constructor | Relation |
|---|---|
| `gate(Instr)` | the in-circuit relation of that instruction over registers and immediates |
| `inputGate(String, IrType)` | the declared input register holds a value of its declared type (`assign_incircuit` from the witness memory) |
| `piGate(Int, Operand, Operand)` | public input `i` equals `g ? x : 0`, with `g` boolean and `x` Native |
| `guardGate(Operand)` | the impact guard is boolean, including an empty impact |
| `bindGate(Int)` | public input `i` equals the preimage binding input |
| `commGate(Int, TypedIds, Int)` | public input `i` equals Poseidon of the opening, the re-encoded input registers and the encoded outputs |
| `outputGate(Operands, IrTypes)` | output arity and per-position runtime type against the program signature |

`job` sequences `#loadInputs`, `#seedPi`, the instruction list, `#verdicts`, `#witnessSpace` and `#observable`. The cells in `zkir-vm.k` are `<constraints>` (emitted list), `<chips>` (chip set), `<verdicts>` (evaluated list), `<witnessSpace>` (the membership decision), `<unconstrainedRegs>` (the registers whose assigning relation is `unconstrained`), `<observable>` (the observable result `obs(status, encoded outputs, public inputs, skips)`, 16-compilation-target-contract.md) and `<piIdx>` (constraint-side public-input counter).

Instructions still emit after a witness failure. `#exec` is a no-op when `<status>` is `error` or `panic`, and later verdicts that need a missing register or public-input slot become `unknown` rather than `violated`. A failed static check under `checkedJob` sets `<status>` to `error` first and `job` then rewrites to `.K`, so no gate is emitted or evaluated.

### When each constructor is emitted

The `job` rule appends one `inputGate(N, T)` per declared input to `<constraints>` before anything runs, in declaration order, whether or not the raw inputs decode: the circuit assigns every declared input from the witness memory before the binding input. `#seedPi` then always appends `bindGate(0)`. When `<doComm>` is true it also appends `commGate(1, Ins, Rand)`, where `Ins` is the program input list and `Rand` is the opening, or `0` if the commitment is absent (the witness half then fails with `"Expected communications commitment"`). The witness half seeds `<pi>` with the binding input and, while still alive and when a commitment is present, with the commitment value. After an earlier decode failure the gates are still emitted and `<piIdx>` still advances to 1 or 2.

Every ordinary instruction emits `gate(I)` before `#exec(I)`:

```k
rule <k> I:Instr => #exec(I) ... </k> <constraints> Cs => Cs ListItem(gate(I)) </constraints>
  requires notBool isSpecialEmit(I)
```

`isSpecialEmit` is true only for `impact` and `output`. `impact(G, Xs)` appends `guardGate(G)` and then one `piGate` per operand from the current `<piIdx>`, which advances by `lenOperands(Xs)` at emission (`#piGates`); an empty impact still emits `guardGate(G)`. `output(Vs)` appends `outputGate(Vs, Ts)` with `Ts` from `<outTypes>`.

## Outcomes

`Outcome` has six constructors. `eval` is total. `verdicts` maps each constraint to `verdict(C, eval(C, mem, pi, chips, outputs, binding))`, and `violations` is the sublist whose outcome is neither `holds()` nor `unconstrained(msg)`.

| Outcome | Meaning |
|---|---|
| `holds()` | the relation is satisfied on this witness |
| `violated(msg)` | a circuit for this instruction exists and rejects the witness |
| `synthErr(msg)` | the circuit cannot be built |
| `unknown(msg)` | a register or public-input slot the gate reads is absent |
| `unconstrained(msg)` | the relation is satisfied and this relation pins the register to its type only |
| `unsupported(msg)` | no in-circuit relation is modelled for this gate |

`unconstrained` is a gate-level marker: the relation of this gate pins the register to its type only. It is emitted in three places. A `public_input` or `private_input` whose guard register holds `native(0)` reports `unconstrained("<op> guard is 0: the gate pins register X to its type only, the witness holds the default and the transcript has no entry")` after the type and chip checks pass (`#guardedOff`). The gate is the same for every guard: the `PublicInput | PrivateInput` arm of `Relation::circuit` (`ir_vm.rs:977-1003`) matches `guard: _`, assigns the register from `preproc.memory` and stores it with `mem_insert` against the entry it was assigned from, with no `pi_push` and no equality against the transcript. The marker is reported at guard 0, where the witness holds `defaultValue(T)` and the transcript has no entry for the register; whether the cell is free in the circuit is decided by the register's consumers (a later relation that reads it pins it), which the comparison table's D5 walk decides and the `inject-transcript` column tests. An `inputGate`, `public_input` or `private_input` of type `JubjubScalar` reports `unconstrained("JubjubScalar canonicity: the assigned bits of X are not proven below the scalar field order")` (`#canonicity`): `EccChip::assign` in midnight-circuits 7.2.4 `ecc/native/edwards_chip.rs` assigns the 252 little-endian bits of the witness scalar with `enforced_canonical = false`, and nothing constrains the bit string below `#rJ`. The other two ways a JubjubScalar register is assigned are canonical and report `holds`, and neither is canonical by the flag: `jubjub_scalar_from_native` reduces through `BigUintGadget::div_rem`, whose `assert_lower_than` constrains the remainder below the order (`jubjub_scalar_from_biguint` in `ir_instructions/encode.rs`), and the `scalar_from_le_bytes` it then calls (`edwards_chip.rs:1362-1380`) still builds the scalar with `enforced_canonical = false`; the extension `load_constant` uses `assign_fixed`, a fixed cell whose value is the constant. The same three gates on one of the six foreign field types or the three foreign point types report `unconstrained("foreign limb canonicity: ...")`: `FieldChip::assign` (`field_chip.rs:424-455`) range-checks the limbs against `well_formed_log2_bounds` (`field_chip.rs:263-278`), bounds chosen so that the encoded integer lies below twice the modulus, not below the modulus, and `as_public_input`, `encode_incircuit` and the commitment hash the raw limbs; a foreign point inherits the residual through its coordinates. `#and` returns the first non-`holds` outcome, and the `unconstrained` checks come last in their conjunctions, so a `violated` or `synthErr` on the same gate wins.

`eval/6` (`eval(Constraint, Map, List, Set, List, Int)`) handles the three gates that need the outputs or the binding input. `bindGate(I)` is `holds` when `pi[I]` equals the binding, `unknown("public input vector incomplete")` when the vector is shorter than `I + 1`, and `violated("binding input differs")` otherwise. `commGate` has its own section below. `outputGate` is `synthErr("Output: signature declares N return values but instruction has M")` on an arity mismatch; otherwise `#outputTypes` is `holds` on matching runtime types, `synthErr("Output position I: signature declares T but operand has runtime type U")` on a mismatch, and `unknown` if a register is missing. Every other constraint falls through to `eval/4`, the per-instruction relation.

`eval/4` rules are conjunctions built with `#and`, which returns the first non-`holds` outcome. `rd` reads an operand, and an absent name is `vErr("register X is not in the witness")`, which `#need` turns into `unknown`. `#matches(expected, register, what)` maps a computed `vErr` or `vPanic` to `synthErr` (the in-circuit operation is undefined on these types), a missing register to `unknown`, equal values to `holds` and unequal values to `violated`. `#boolean` accepts only `native(0)` and `native(1)`: another Native value is `violated`, a non-Native value is `synthErr`, as for `#native`. `#chipFor` and `#chipNamed` are `synthErr` when the required chip is absent from `<chips>`.

`#piEq` (from `piGate`) is `unknown` when index `I` is past the public-input vector, `holds` when `pi[I]` equals `#guarded(g, x)` (`1` selects `x`, anything else yields `0`), and `violated("public input I differs from the guarded value")` otherwise. A non-boolean guard is `violated("impact guard is not boolean: V")` on both `guardGate` and `piGate` (case `k06`).

`#invNonZero` uses `isZeroField`: `inv` of a zero field element is `violated("inv: a * inv = 1 has no solution at zero")`. `isZeroField` is true of `native(0)` and of the six foreign field zeros, and false of every other value, including `jubjubScalar(0)`, whose inversion has no in-circuit arm and becomes `synthErr` through `#matches`.

`#fromCoordsChip` requires the Jubjub chip for native/native `from_coordinates`, and other coordinate types rely on `#chipsForValues` of the operands. `#fromCoordsPt` then requires that the exact `(x, y)` lie on the curve and that the point itself lie in the prime-order subgroup. `inSubgroup` in `zkir-curves.k` tests that the subgroup order times the point is the identity for Jubjub and Curve25519, and is always true for secp256k1 and secp256r1. No cofactor is cleared before that test or before the output comparison. A pair off the curve is `violated("from_coordinates: (x, y) is not on the curve")` (case `k01`). Off-circuit Jubjub decompression uses only the parity of `x`, recorded as K1 in 13-known-divergences.md. A point outside the subgroup is `violated("from_coordinates: point is not in the prime-order subgroup")` (case `f11`, the order-2 Curve25519 point).

`#lowHighBounds` makes `bytes32_from_low_high` unsatisfiable, not unbuildable, when the low operand's byte 31 is non-zero or the high operand is not a byte. High must be Native, and a foreign high is `synthErr` (`#native`, case `f08`).

`alignedBytesCircuit` decodes an alignment only when every segment is an `atom` and none is a `compressAtom`. Surplus fields are ignored (`#abcOk` drops the remainder of the list). Any `option` segment is rejected with `"synthesis: in-circuit decoding of alignment options is not yet implemented"`, and any `compressAtom` with `"synthesis: Cannot decode compressed value from field elements"`; too few fields for the atoms is the `"Inputs did not match alignment"` of `fab_decode_to_bytes_atom`. These three rejections become `synthErr` through `#matches`. A field element that exceeds the byte range of its atom (a `bytes n` atom whose stray or chunk element does not fit) is a different case, `cbRange` from the shared decoder, which the hash gates report as `violated("<op>: a field element exceeds the ... atom of its alignment")` through `#alignRange` before the output comparison: the circuit exists, `assigned_to_le_bytes` with a fixed width constrains the decomposition, the truncated hint recomputes another digest, and `mem_insert` of the output rejects the witness. Off-circuit `alignedBytes` in `zkir-ops.k` parses options (case `k07`) and reports the range failure with the crate's arity text.

The `[owise]` rule is `unsupported("no in-circuit relation modelled for this instruction: " +String #opName(I))`. `#opName` has only an `[owise]` rule returning `"instruction"`, so the message never names the constructor. `impact` and `output` never take that path, because they do not emit `gate(I)`.

`synthErr` also covers a missing in-circuit dispatch arm (`#eqSupported`: `JubjubScalar` equality, case `f07`, and `Bytes32` `cond_select`, case `f06`), a chip that `usedChips` did not initialise, a `less_than` padded bound above 253, a `constrain_bits` width above 255, and `div_mod_power_of_two` with a number of outputs other than 2.

`assert` is `cond != 0` in circuit (`#nonZero`). Off-circuit `preprocess` requires a boolean, so `assert` of 2 fails the witness while the gate `holds` (case `f02`). `public_input`, `private_input` and `inputGate` check that the register has the declared type and that the type's chip is initialised; a register of another runtime type is `synthErr("cannot convert T to \"U\"")`, the `Error::Synthesis` that `assign_incircuit` raises from `TryFrom<IrValue>` (`ir_instructions/assign.rs` `convert_values`), reachable only under `job` for a program that overwrites an input register. The guard is not read by the relation; a guard of 0 makes the verdict `unconstrained` (case `f03`).

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

`chipOfType` maps those eleven curve types onto the four curve chips and every other type, including Native and Bytes32, to the empty set. Value gates call `#chipsForValues` or `#chipFor`, and hash gates call `#chipNamed`. `from_bytes32` checks `#chipFor` of its target type, `jubjub_scalar_from_native` checks `#chipFor(jubjubScalar(), Chips)`, native `from_coordinates` checks `#fromCoordsChip`, which is `#chipFor(jubjubPoint(), Chips)`, and the extension `load_constant` gate in `zkir-ext.k` checks `#chipFor` of its type.

`usedChips` does not look at any of those four instructions. A program that first introduces a chipped type through one of them succeeds in `preprocess`, while the crate's chip accessor (`std.jubjub()`, `std.secp256k1()`, ...) panics at keygen. The gate is `synthErr("chip not initialised for T")` or `synthErr("chip not initialised: name")`. Of the cases `f13` (`from_bytes32` onto Secp256k1Base), `k04` (`jubjub_scalar_from_native`) and `k01b` (native `from_coordinates` without a Jubjub input), the last two are K4 in 13-known-divergences.md.

## Width limits

The static check `wf` and the `job` run-time rules follow `preprocess`; the gates follow the chip assertions that keygen would hit.

`less_than` pads the requested width with `#ltBits(N) = maxInt(N +Int (N modInt 2), 4)`, which matches `max(bits + bits % 2, 4)` of `std.lower_than` in `ir_vm.rs`. `bounded_of_element` in midnight-circuits 7.2.4 then asserts that the padded width is at most `MAX_BOUND_IN_BITS = F::NUM_BITS - 2 = 253`. When that assertion fails the gate is `synthErr("less_than: padded bound P exceeds MAX_BOUND_IN_BITS = 253")`, and `bits = 253` pads to 254 (case `k05`). `preprocess` (`checkBits` in `zkir-ops.k`) and `wf` (`#checkArity` in `zkir-syntax.k`) only reject `bits >= 255` (`#frBits`). An odd width such as 3 is padded to 4, so the padded relation accepts a value that fails the off-circuit 3-bit bound. The witness then stops before writing the output, and the gate is `unknown` (case `f04`).

`constrain_bits` calls `assigned_to_le_bits` with `Some(bits)` and `enforce_canonical = bits >= FR_BITS`. That function asserts `nb_bits <= F::NUM_BITS` (255). The gate is `synthErr("constrain_bits: width N exceeds 255")` when `N > 255`. At `N <= 255` it requires a Native value that fits in `N` bits, except that `#bits` is `holds` whenever `N >= #frBits`, so a 255-bit request is a full-width decomposition. `wf` rejects `bits >= 255`, and a failed static check ends `checkedJob` before any gate exists, so this `synthErr` is reachable only through `job`.

`div_mod_power_of_two` and `reconstitute_field` reject `bits > #frBytesStored *Int 8` (248) in both `wf` (`#checkArity`) and `#exec` (`"Excessive bit count"`). The `div_mod_power_of_two` gate does not repeat that check. It requires Native input, two outputs (else `synthErr("Unexpected output length of DivModPowerOfTwo instruction")`), and outputs equal to `highBits` and `lowBits`. The `reconstitute_field` gate requires Native inputs, `#bits` of the divisor in `255 - N` bits, `#bits` of the modulus in `N` bits, and an output equal to `((D << N) + Mo) mod r`. Off-circuit `preprocess` also rejects a reconstituted value above `r - 1`. The in-circuit sum wraps, so overflow leaves the output absent and the gate `unknown` (case `f01`). At 256 bits and above the crate's keygen would underflow `FR_BITS - bits`. `wf` rejects such a program first (case `f10`, outcome `n/a`).

## Communications commitment

The witness-side check `#commCheck` in `zkir-vm.k` follows `preprocess`: it hashes the raw preimage input stream concatenated with the encoded outputs,

```k
transientCommit(#preInputs(Pre) #encodeAll(Os), Rand)
```

which is `poseidonHash(ListItem(Rand) Vs)` in `zkir-hash.k`. Failure is the run error `"Communications commitment mismatch"`. Generation mode records the same hash in `<needs>` instead of checking it.

The in-circuit gate `commGate(I, Ins, Rand)` re-encodes the assigned input registers with `encodeValue` and hashes `rand ++ encode(inputs) ++ encode(outputs)`, which matches the `encode_incircuit` loop in `Relation::circuit`. Evaluation is `#commOutcome`: `unknown` if a register is missing or the public-input vector is short, `synthErr("chip not initialised: poseidon")` if Poseidon is not in `<chips>`, `holds` when `pi[I]` equals that hash, and otherwise `violated("communications commitment differs from poseidon(rand ++ encode(inputs) ++ encode(outputs))")`. The `synthErr` rule exists so that `eval` is total: that branch is unreachable from `job`, because `commGate` is emitted only when `<doComm>` is true and `usedChips` enables Poseidon whenever that flag is set.

The two hashes agree when every input encoding is canonical. They differ when a foreign-field input is a non-canonical limb encoding of a reduced value: `preprocess` hashes the raw stream and accepts, while the circuit hashes the canonical re-encoding of the assigned value and rejects. Case `f05` (`corpus/divergence/f05_noncanonical_foreign_limbs.zkir`) is that split: status `ok`, `commGate` `violated`. A `holds` on `commGate` therefore proves agreement with the circuit's commitment, not with the raw-stream commitment of `preprocess`.

## What the witness space establishes

`witnessSpace(verdicts, mem)` in `zkir-constraints.k` is true when every verdict is `holds()` or `unconstrained(msg)` (`#admissible`) and every value in `<mem>` satisfies `wellTypedValue` of `zkir-values.k` (`wellTyped(values(mem))`): `0 <= x < r` for `native(x)`, `0 <= s < rJ` for `jubjubScalar(s)`, a length of 32 for `bytes32`, `onCurve` and `inSubgroup` (`zkir-curves.k`) for the four point types with the Edwards identity `pt(0, 1)` and never `inf()`, a value below its modulus for the six foreign field types, and on the extension surface a byte below 256 and a byte string of a length the type system admits. The constructors themselves carry no invariant: `native(#r)`, `jubjubPoint(pt(0, 0))`, a three-byte `bytes32` and `secp256k1Base(p)` are terms of sort `Value` on which `witnessSpace` is false (`unit_values.py`); the decoders of `ZKIR-VALUES` build only values satisfying the predicate, and a claim over a symbolic memory discharges it from its `requires`. `#witnessSpace` writes it to `<witnessSpace>` after `#verdicts`, together with `unconstrainedRegs(verdicts)` in `<unconstrainedRegs>`: the declared input of an `inputGate` and the written registers of a `gate(I)` whose outcome is `unconstrained`. The predicate reads neither `<status>` nor the transcripts: a run that `preprocess` rejects can still have a memory in the space (case `f02`, `assert` of 2), and the oracle sees it only through a witness injection (the comparison table's D9).

Typing discharges the assignment constraints of the types under one assumption, stated per chip: the circuit constrains each assigned cell to its type. The three constraints read for it are the byte range of `AssignedByte` (`native_gadget.rs:565-604`), the cofactor clearing of an assigned Jubjub point (`edwards_chip.rs:842-856`) and the limb ranges of a foreign element (`field_chip.rs:424-455`), all in midnight-circuits 7.2.4. On that assumption the per-type range, on-curve and cofactor constraints hold on every register of a well-typed memory and are not evaluated a second time; the assumption itself is the third residual item of the soundness direction below.

The projection from circuit cells to registers, stated in the header of `zkir-constraints.k`:

1. Each named register is the value of the cell (or cell group) that the crate binds to that identifier: through `mem_insert` for every instruction output and for `public_input` / `private_input` registers, through `memory.insert` for the declared inputs (assigned from the witness by `assign_incircuit`) and for the outputs of `from_bytes32`, `reverse_bytes`, `bytes32_into_low_high` and `bytes32_from_low_high`. The K register holds the typed value of that cell.
2. Auxiliary cells (bit decompositions, limbs, hash and curve gadget rows, lookup witnesses) are existentially quantified: a memory is in the modelled space when some assignment of the auxiliary cells satisfies the crate's constraints together with the projected registers. The model never names them.
3. Copy wiring between named cells is value equality: where the crate copies a cell into a gadget through the permutation argument, the model states that the two registers hold the same value.

Let `Acc(P, pi)` be the cell assignments the circuit of `P` accepts with the instance `pi`, `proj` their projection to the named registers (a K memory when every projected cell is a typed value), and `Mod(P, pi) = { M | witnessSpace(verdicts(P, M, pi), M) }`. Two claims relate them, each with its residual list on its own side.

(a) Soundness direction: every cell assignment the circuit of P accepts with the instance pi projects to a memory of the modelled witness space, proj(Acc(P, pi)) ⊆ Mod(P, pi), up to three residual items: JubjubScalar canonicity, foreign limb canonicity and the per-chip typing assumption. This is the claim a compiler proof uses: the soundness direction of compiler correctness quantifies over `Mod`, and (a) is what makes that quantification cover the circuit. Its residual names the places where the circuit accepts a cell assignment whose projection is not in `Mod` or is not a K memory at all.

- JubjubScalar canonicity: a JubjubScalar assigned from the witness (declared input, `public_input`, `private_input`) is 252 witness bits that nothing constrains below the scalar field order; visible as `unconstrained` on `inputGate` and on the two input gates.
- Foreign limb canonicity: a foreign field element assigned from the witness is a limb tuple range-checked below twice the modulus, not below it, and the commitment and the public-input encoding hash the raw limbs; a foreign point inherits it through its coordinates; visible as `unconstrained` on the same three gates for the six foreign field types and the three foreign point types.
- Per-chip typing: the typing clause above rests on the three cited chip constraints, which are read, not modelled.

Evidence for (a): `tools/diff_test.py --circuit` reports no `violated` and no `synthErr` verdict on any circuit-accepted witness, on the honest rows and on the accepted injections of the comparison table. The two canonicity items have no injection column: `--inject` takes typed values, and the typed interface cannot carry a non-canonical representative (a malicious-prover harness that writes limb advice directly is deferred).

(b) Tightness: every memory of the modelled witness space extends, by some assignment of the auxiliary cells, to a cell assignment the circuit accepts with the instance pi, Mod(P, pi) ⊆ proj(Acc(P, pi)), up to four residual items: auxiliary cells, prover-side panics, hash gadget internals and copy wiring. This is the claim the injection columns test: `inject-unconstrained` expects `accepted` (D5), and a `violated` verdict expects the `C`, `W` or `X` cell the table assigns. Its residual names the places where the circuit may reject a memory the model admits.

- Auxiliary cells: their existence is assumed, never constructed.
- Prover-side panics: a chip hint that aborts the prover is reported as `violated` or `synthErr`; other panics inside midnight-circuits are not modelled.
- Hash gadget internals: hash gates compare against the concrete functions of `ZKIR-HASH`, not against the gadget's rows.
- Copy wiring: modelled as value equality, not as the permutation.

Transcript inputs are a residual of the observation rather than of either claim. The `public_input` / `private_input` gate pins the cell to its type only, for every guard (the section on outcomes above); `unconstrained` is reported at guard 0, where the witness holds the type's default and the transcript has no entry; whether the cell is free in the circuit is decided by its consumers (D5). The public transcript is bound by the ledger's re-execution of the transcript, the private transcript by nobody, so a specification must not read "holds the ledger's value" from a `holds` on these gates.

Both claims are tested, not proved. `tools/diff_test.py --circuit` runs the crate's circuit under the MockProver on every corpus witness, on injections into the preprocessed memory (a Native input, an `unconstrained` register, a transcript register, and for each a second value chosen by the register's first consumer) and on the divergence cases, and compares each outcome with `plan-iter3/circuit-comparison-table.md`. The receipts `evidence/zkir-k-circuit-differential-92e8bdd3-2026-09-06d.txt` and `evidence/zkir-k-circuit-differential-ext-2ffe2d1-2026-09-06d.txt` record the D5 cell on `%color.11` of swap `expire.zkir`, the `inject-transcript` cells (`A` on a register without a consumer, `W` on one an `impact` reads) and the range sub-cells of D3.

A `holds` verdict on `assert` of a non-boolean non-zero is therefore expected: the circuit relation is weaker than the witness check. When the off-circuit bound of `less_than` fails, the verdict is `unknown` rather than `holds`, because the output register is absent even though the padded relation would have accepted the inputs (case `f04`). A memory outside the space, whether by a `violated`, `synthErr`, `unknown` or `unsupported` verdict, carries no claim about the circuit beyond the cell the comparison table assigns to that verdict.

## How the harness uses verdicts

`tools/zkir_run.py` derives the run status from `<status>` and `<k>` alone. It pretty-prints each entry of `<verdicts>` into `all_verdicts` (triples `(outcome, message, gate)`) and `violations` (the triples whose outcome is neither `holds` nor `unconstrained`), reads `<witnessSpace>` into `witness_space` and `<unconstrainedRegs>` into `unconstrained`. `constraints` and `verdicts` are the lengths of the two cells. From the repository root, the handmade Poseidon program emits eleven constraints and eleven `holds` verdicts:

```
printf '%s\n' '{"inputs": ["5", "123456789", "987654321987654321"]}' > /tmp/transient_hash.json
uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py \
  experiments/zkir-k/corpus/handmade/transient_hash.zkir \
  /tmp/transient_hash.json
```

The object has `"status": "ok"`, `"constraints": 11`, `"verdicts": 11`, `"violations": []`, `"witness_space": true` and `"unconstrained": []`: three `inputGate`, `bindGate(0)`, four `transientHash` and three `hashToCurve`.

`tools/diff_test.py` appends an `ok` K run whose `violations` is non-empty to `oracle2_flags` and does not fail the comparison on that list. It prints `oracle 2: N successful K runs with a non-holding gate`, where successful means status `ok`, and `witness space: X of Y successful K runs in the modelled witness space, Z with unconstrained registers`, naming the registers. The receipts `evidence/zkir-k-circuit-differential-92e8bdd3-2026-09-06d.txt` and `evidence/zkir-k-circuit-differential-ext-2ffe2d1-2026-09-06d.txt` both report `N = 0` and every successful run in the space, over 365 base-surface comparisons (53 successful-run agreements) and 418 extension-surface comparisons (50 successful-run agreements); the registers named as `unconstrained` are the guard-0 transcript registers and the JubjubScalar and foreign-typed inputs. Error runs may carry non-holding verdicts: an `output` arity mismatch is `synthErr` on `outputGate` while the status is `error`.

`tools/divergence_tests.py` selects one gate per case: the first verdict whose pretty-printed constraint, with underscores and spaces removed and lower-cased, starts with `gate(<op>(` or with the constructor name (`commGate(`, `guardGate(`). A missing emission is `no-such-gate`, not an inferred `holds`. The expected outcome is `holds`, `violated`, `synthErr`, `unknown`, `unconstrained`, or `n/a` when `checkedJob` stops on well-formedness before `#verdicts` (case `f10`). Case `k08` runs on the extension definition and the midnight-zkir 2ffe2d1 oracles. The receipt `evidence/zkir-k-divergence-tests-2026-09-06d.txt` records one match per case, including the injected cases (a hash input above its alignment atom, `inv` of zero, the two conversions of `bytes32_from_low_high` and `less_than` above the padded bound).

Evidence that the honest witness is in the modelled witness space therefore requires a finished run with `witness_space` true, produced from gates that were emitted; the `unconstrained` list names the registers the claim leaves free. Status alone is not enough: case `f05` is `ok` with a `violated` gate. Even together they are not a proof of unique witnesses, of gadget soundness, or of keygen success on a program whose chips `used_chips` did not enable.

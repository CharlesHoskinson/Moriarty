VERDICT: WARNING The corpus and the comparison table show no K `violated`/`synthErr` on a circuit-accepted honest or Native-injected witness, but the soundness predicate as written is not the set the header describes: `wellTyped` is a K-sort check that does not discharge assignment constraints, and the residual list both overstates guarded-off freedom and omits circuit constraints the projection claims to cover.

## Findings

### 1. major — `wellTyped` does not discharge assignment constraints

**Where.** `experiments/zkir-k/semantics/zkir-constraints.k:32-44` (header), `:169-183` (`witnessSpace` / `wellTyped`); `experiments/zkir-k/docs/08-constraints-and-verdicts.md:119-120`; `experiments/zkir-k/semantics/zkir-values.k:27-39` (constructors).

**What K says.** `witnessSpace(Vs, M)` is `#admissible(Vs) andBool wellTyped(values(M))`. `wellTyped` accepts every `ListItem(_:Value)`. The header and chapter 08 then state that a term of sort `Value` is “by construction a canonical native field element, a point on its curve and in its prime-order subgroup, a byte string of its declared length, or a foreign element below its modulus”, and that this typing discharges the circuit’s per-type range, on-curve, cofactor and (implicitly) limb/byte checks, “because the crate’s typed `IrValue` cannot be malformed and the K decoders of ZKIR-VALUES build only such values”.

**What the source of truth says.** The K constructors do not carry those invariants:

- `native(Int)` is any integer (`zkir-values.k:27`). Range `[0, r)` is enforced only in `decodeValue` (`zkir-values.k:142-143`), not on the sort.
- `jubjubScalar(Int)` is any integer (`zkir-values.k:30`). Canonical `s < #rJ` is enforced only in `decodeValue` (`zkir-values.k:164-167`).
- `jubjubPoint(Point)` wraps `pt(X, Y) | inf()` (`zkir-curves.k:19-20`) with no on-curve or subgroup check. `inSubgroup` is a separate function (`zkir-curves.k:82-84`).
- `bytes32(Bytes)` does not require length 32; `encodeValue` on a wrong length returns `.List` (`zkir-values.k:122-123`).
- `secp256k1Base(Int)` and the other foreign constructors do not require `X < p`.

The crate side is the opposite of a K-sort invariant. `assign_incircuit` (`ir_instructions/assign.rs:36-142`) takes `Value<IrValue>` and then:

- Native: `std.assign_many` of `Fr` cells (already in \(\mathbb{F}_r\)).
- Bytes32: `NativeGadget::assign_many` for `AssignedByte`, which is `assign_less_than_pow2(..., 8)` (`native_gadget.rs:572-604`).
- JubjubPoint: `EccChip::assign`, which assigns `p * cofactor^{-1}` under the `q_mem` curve equation (`edwards_chip.rs:401-419`, `:842-856`) and multiplies by the cofactor.
- JubjubScalar: 252 `AssignedBit`s, `enforced_canonical: false` (`edwards_chip.rs:877-888`).
- Foreign field: `FieldChip::assign` range-checks every limb with `assign_lower_than_fixed` against `well_formed_log2_bounds` (`field_chip.rs:424-454`).
- Extension Bool/Byte/Bytes: `AssignedBit` / `AssignedByte` (`midnight-zkir-2ffe2d1/.../assign.rs:70-88`).

Those are real constraints on advice cells. `IrValue` cannot express a malformed inhabitant; a K `Value` can.

`#typed` (`zkir-constraints.k:407-410`) only compares `typeOf(V)` to the declared `IrType`. `typeOf(native(_)) => native()` for any integer; `typeOf(jubjubPoint(_)) => jubjubPoint()` for an off-curve point.

**Why it matters.** The soundness claim of M2 quantifies over `witnessSpace`. As implemented, that set includes memories the circuit cannot even name:

- `native(#r)` or `native(-1)` in a register. The circuit cell is in \(\mathbb{F}_r\). K `==K` does not identify `native(#r)` with `native(0)`, so an `add`/`copy`/`commGate` relation can `violated` on a witness the circuit would reduce and accept (under-approximation / completeness defect of the predicate). The same register left unused is `unconstrained`/`holds` in K and accepted by the circuit after reduction (vacuous over-approximation).
- `jubjubPoint(pt(0,0))` as a declared input. `inputGate` `#typed` holds; `wellTyped` holds. `EccChip::assign` cannot be given a non-`JubjubSubgroup` value through `IrValue`, and `q_mem` rejects an off-curve pair.
- `secp256k1Base(#k256P)`: K encoding is `encForeign(p, p, ...) = enc(0)` (`zkir-values.k:77`), so `commGate` can hold against a circuit that assigned `0` while the K register still prints as `p`.

A VM run of `job` from `decodeValue` plus `addV`/`fadd` does not construct these terms. The predicate used by the claim is not restricted to VM-produced memories.

**Fix.** Replace `wellTyped` with a semantic predicate that matches `decodeValue`’s inhabitants: `0 ≤ native < #r`, `0 ≤ jubjubScalar < #rJ`, `lengthBytes = 32` and bytes in `[0,255]`, `onCurve ∧ inSubgroup` for points, `0 ≤ x < p` for each foreign sort, and the extension `boolV`/`byteV`/`bytesV` bounds. Keep the comment that *decoder-produced* values satisfy this; do not attribute it to the K sort. If the claim is only over `job` memories, state that restriction in the header and drop the “by construction” sentence.

### 2. major — guarded-off residual claims a globally free cell; the circuit only leaves *this gate* unconstrained

**Where.** `zkir-constraints.k:57-66`, `:379-390` (`#guardedOff`); `docs/08-constraints-and-verdicts.md:51`, `:135`; `corpus/handmade/transcripts_guard_off.zkir:37-80`.

**What K says.** After type and chip checks, `#guardedOff(guard(G), O, M, What)` is `unconstrained` when `rd(G, M) ==K vOk(native(0))`, otherwise `holds`. The residual list: “the register of a `public_input` / `private_input` whose guard is 0 is assigned from the witness and pushed nowhere; the circuit leaves the cell free where the witness holds the type’s default value”. Chapter 08 repeats that wording.

**What the source of truth says.** Public and private inputs share one in-circuit arm (`ir_vm.rs:978-1004` at 92e8bdd3; same shape at 2ffe2d1 `:1143-1169`):

```
I::PublicInput { guard: _, val_t, output } | I::PrivateInput { ... } => {
    mem_insert(output, assign_incircuit(std, layouter, val_t, &[witness[output]])?, ...);
}
```

The guard is not read. Every type is assigned with the constraints of finding 1. The cell is a witness advice cell. Whether it is *globally* free is decided by later instructions, not by the guard.

`transcripts_guard_off.zkir` is the counterexample already in the corpus: `%g` is `test_eq(%a, 1)`; when `%a = 0`, `%g = 0`, so K marks `%pp` and `%ps` unconstrained (receipt `evidence/zkir-k-circuit-differential-92e8bdd3-2026-09-06c.txt`, handmade `transcripts_guard_off.zkir`, `unconstrained=%pp,%ps`). Instruction 8 then `encode`s `%pp` into `%x,%y`. Encode in-circuit is `encode_incircuit` plus `mem_insert` (`ir_vm.rs:818-828`). The point cell is consumed. Injecting a different well-typed point while leaving the encode outputs at the identity is a `witness-consistency-error`, not `accepted`. D5 was not run here because `%pp` is a JubjubPoint and `%ps` a Secp256k1Scalar (`--inject` is Native-only).

The Compact pattern in swap `expire.zkir` / `decide.zkir` happens to feed the off-guard register only to an `impact` under the same guard or a `cond_select` that discards it; those Native cells *are* free, and D5 records `accepted` on `%color.11` / `%color.27`. That is a property of those programs, not of the `public_input` relation.

`#guardedOff` also does not require `defaultValue(T)`. A non-default well-typed value with guard 0 is still `unconstrained`. Off-circuit `preprocess` (`ir_vm.rs:353-365`) writes the default; the circuit assigns whatever is in `Preprocessed.memory`.

**Why it matters.** A compiler-soundness argument that treats every `unconstrained` register as a free cell will accept extra preimages on programs that encode, hash, or `constrain_eq` that register. Conversely, a `public_input` with `noGuard()` or guard 1 that nobody later reads is the same circuit cell (guard ignored) but K reports `holds` and omits it from `unconstrainedRegs`. The marker is “off-circuit guard was 0”, not “circuit-free”.

**Fix.** Residual text: this gate does not constrain the cell beyond type and chip; global freedom is “no later gate reads the register”. Keep `#guardedOff` as a visibility marker for the Compact default-on-guard-0 pattern. Have D5 (or a taint walk) refuse to claim `A` when a consumer exists, which the harness already does for Native; extend that walk to the non-Native unconstrained registers the receipt already names (`%pp`, `%ps`, `%s`, `%s1`, `%c`).

### 3. major — residual list is missing circuit constraints the projection claims typing or existential quantification covers

**Where.** Residual list `zkir-constraints.k:50-66` and `docs/08-constraints-and-verdicts.md:128-135`. Circuit sources as cited below.

The header says a memory is in the modelled space when “some assignment of the auxiliary cells satisfies the crate’s constraints together with the projected registers”, and that typing discharges assignment constraints. The residual list then names six items. The following constraints exist in midnight-circuits 7.2.4 / midnight-zk-stdlib 2.3.5 and are not named as modelled relations, typing clauses (see finding 1), or residual items.

**Foreign-limb ranges at assignment.** `FieldChip::assign` (`field_chip.rs:424-454`) writes `(x - 1)` in base `2^LOG2_BASE` and `assign_lower_than_fixed`s each limb to `well_formed_log2_bounds` (`field_chip.rs:263-278`): all limbs in `[0, 2^{LOG2_BASE})` except the most-significant, sized so `m ≤ base^{n-1}·msl < 2m`. That admits a well-formed non-canonical limb tuple for some field elements (the `assign_fixed` comment at `field_chip.rs:474-478` is explicit about canonical vs well-formed forms). The named register’s *value* is still the field element; the limbs are auxiliary. They are not in the residual list. A non-canonical *raw preimage* encoding is modelled (`commGate`, case `f05`). A non-canonical *limb witness* for the same assigned value is not.

**Byte / Bool ranges at assignment.** Bytes32 assignment is 8-bit range per cell (`native_gadget.rs:572-604`). Extension Bool is `AssignedBit`; Byte/Bytes are `AssignedByte` (`assign.rs:70-88` at 2ffe2d1). No modelled relation re-checks this; `wellTyped` does not.

**Decomposition hints.** `constrain_bits` → `assigned_to_le_bits(..., Some(bits), bits >= FR_BITS)` (`ir_vm.rs:844-852`). `div_mod_power_of_two` → full canonical bit split (`ir_vm.rs:1011-1023`). `reconstitute_field` → `assert_lower_than_fixed` on both limbs (`ir_vm.rs:1036-1045`). `fab_decode_to_bytes` Field atom → `assigned_to_le_bytes(..., None)` (`ir_vm.rs:141`). K models the integer consequences (`#bits`, `#hi`/`#lo`, `#recOf`, `alignedBytesCircuit`) and treats the bit/byte cells as unnamed. That is the existential treatment, but “decomposition hints” is not on the residual list the plan asked to complete.

**Lookup-table membership.** Range-check lookups inside `assign_lower_than_fixed` / `assign_less_than_pow2`, SHA round-constant / message-schedule tables, Poseidon MDS and round constants, Keccak round constants. K compares hash *outputs* to `sha256Bytes` / `keccak256Bytes` / `poseidonHash` (`zkir-constraints.k:515-556`, `zkir-hash.k`). Table membership of internal rows is unmodelled and unnamed except as “hash gadget internals” for the hash chips.

**Hash-gadget padding.** SHA-256 pad in the chip (`sha256_chip.rs:584-599`: `0x80`, zeros, 64-bit big-endian length) matches K `#shaPad` (`zkir-hash.k:171-172`). Keccak pad in K is pre-NIST `0x01 .. 0x80` (`zkir-hash.k:13, 237-242`). The padding *cells* are gadget-internal; the residual “hash gadget internals” covers them only if the output relation is the claim. It does not name padding as a constraint family a reader searching the list would find.

**`memory.insert` outputs (D4), including extension `reverse`.** `from_bytes32`, `reverse_bytes`, `bytes32_into_low_high`, `bytes32_from_low_high` store with `memory.insert`, not `mem_insert` (`ir_vm.rs:1115-1149`). The extension `reverse` does the same (`midnight-zkir-2ffe2d1/.../ir_vm.rs:1290-1294`). An injected `Preprocessed.memory` that disagrees on those outputs is accepted by MockProver (cell is the recomputed value). K’s gate requires the register to equal the recomputed value (`#matches`), so that memory is not in `witnessSpace`. That is modelled-space ⊂ circuit-accepted-as-Preprocessed.memory. The comparison table records it as D4; the soundness residual list does not. The projection paragraph names the four base instructions and omits extension `reverse`.

**Why it matters.** Plan M2 / compiler soundness wants the modelled space to *over*-approximate the circuit (safe: a proof about every modelled memory is a proof about every circuit-accepted memory). An unnamed constraint that the circuit enforces and K does not is that over-approximation (extra modelled memories, or extra auxiliary witnesses). An unnamed constraint that K enforces and the circuit does not, or a `memory.insert` slot K treats as a cell, is under-approximation (finding 3’s D4). Leaving them off the residual list makes the claim look closed.

**Fix.** Extend the residual list with: (a) foreign well-formed limb tuples as auxiliary witnesses of a named field element; (b) byte/bit range checks at assignment, discharged only after finding 1’s semantic `wellTyped`; (c) decomposition and lookup witnesses, existential; (d) hash padding cells, under hash internals, with the FIPS/pre-NIST function named; (e) `memory.insert` outputs, including extension `reverse`, as a mismatch between `Preprocessed.memory` and circuit cells, not as extra cells in the modelled space. Add `reverse` to the projection paragraph.

### 4. minor — canonicity analysis of the three assignment paths is right; `scalar_from_le_bytes` still clears the flag

**Where.** `zkir-constraints.k:394-406`; `docs/08-constraints-and-verdicts.md:51`; `edwards_chip.rs:877-900`, `:1362-1380`; `encode.rs:227-239`; `biguint_gadget.rs:509-524`; `assign_constant.rs:54-57` at 2ffe2d1.

**What K says.** Witness-assigned JubjubScalar (declared input, `public_input`, `private_input`) is 252 bits with `enforced_canonical = false`, so representatives in `[#rJ, 2^252)` are admitted; visible as `unconstrained` on `inputGate` / the two input gates. `jubjub_scalar_from_native` is canonical via `BigUintGadget::div_rem` / `assert_lower_than`. Extension `load_constant` is canonical via `assign_fixed`.

**What the source of truth says.** Confirmed:

- `JubjubFr::NUM_BITS = MODULUS_BITS = 252` (`midnight-curves-0.3.1/.../jubjub/fr.rs:101, 723`). Modulus `0x0e7db4ea6533afa9...` is strictly less than `2^252`, so `[r_J, 2^252)` is non-empty.
- `EccChip::assign` for `AssignedScalarOfNativeCurve` (`edwards_chip.rs:877-888`): `to_bits_le(Some(NUM_BITS))`, `assign_many` of bits, `enforced_canonical: false`.
- `assign_fixed` (`edwards_chip.rs:891-899`): `assign_many_fixed` of `to_bits_le(None)`, `enforced_canonical: true`. `load_constant` calls this (`assign_constant.rs:54-57`).
- `jubjub_scalar_from_biguint` (`encode.rs:227-239`): `div_rem` against the assigned Jubjub order, then `scalar_from_le_bytes`. `div_rem` does `assert_lower_than(r, y)` (`biguint_gadget.rs:524`). The remainder is proven `< r_J`.

The only inaccuracy is the last hop: `scalar_from_le_bytes` (`edwards_chip.rs:1362-1380`) still constructs `AssignedScalarOfNativeCurve { bits, enforced_canonical: false }`. Canonicity is a preceding constraint on the integer, not the flag. The K comment is right about the constraint and slightly loose about the flag.

Off-circuit `encode` still claims “PublicInput or PrivateInput … yields canonical assigned scalars” (`encode.rs:54-56`). That comment is false of `EccChip::assign`. K is the more accurate of the two.

`--inject` cannot witness `[r_J, 2^252)` because `IrValue::JubjubScalar(JubjubFr)` is reduced. The residual is about the constraint system, not the typed interface. That limitation is already stated.

**Fix.** In the `#canonicity` comment, say the `div_rem` remainder is `< r_J` and `scalar_from_le_bytes` does not set the flag; the bits are canonical by the remainder constraint. Do not change the `unconstrained` outcome on witness-assigned scalars.

### 5. minor — projection is right for named `mem_insert` cells and wrong as a statement about every instruction output

**Where.** `zkir-constraints.k:14-30`; `ir_vm.rs:729-732` (declared inputs), `:761-780` (`mem_insert`), `:865-867` (`Copy`), `:1115-1149` (four byte ops); `edwards_chip.rs:690-691` (`copy_advice`).

**What K says.** (1) Each named register is the cell `mem_insert` binds, except declared inputs and the four byte instructions which use `memory.insert`. (2) Auxiliary cells are existential. (3) Copy wiring is value equality.

**What the source of truth says.**

- Declared inputs: `assign_incircuit` then `memory.insert` with no `mem_insert` check (`ir_vm.rs:729-732`). Injecting a declared Native input *does* change the cell (D3). Projection “K register = assigned cell = witness value” holds.
- Ordinary instruction outputs: recomputed, then `mem_insert` compares to `Preprocessed.memory` (`ir_vm.rs:761-780`). A disagreeing memory is `W`, not a circuit cell with that value. K `#matches` agrees.
- `Copy`: `mem_insert` of the cloned `CircuitValue` (`ir_vm.rs:865-867`) — the destination *is* the source cells. Value equality is actually cell identity.
- Chip-internal copies (`copy_advice` in `point_from_coordinates_unsafe`, permutation on the first seven Edwards columns, `edwards_chip.rs:466-468, 690-691`) are the permutation argument. Modelling them as value equality of the *named* registers is the right abstraction; the copied gadget rows stay auxiliary.
- The four byte ops, and extension `reverse`, do not bind the witness slot (finding 3).

Auxiliary-as-existential is the right treatment for bit decompositions, foreign limbs, hash/curve gadget rows, lookup witnesses, and the cofactor-root used in `EccChip::assign` (`edwards_chip.rs:847-855`: assign `p * inv(cofactor)` under `q_mem`, then `clear_cofactor`). Existence is assumed. That is already residual item 1.

**Fix.** Keep (2) and (3). In (1), name extension `reverse` next to `reverse_bytes`, and state that for `memory.insert` outputs the modelled register is the *recomputed cell*, not the `Preprocessed.memory` slot (D4).

### 6. nit — `#unsat` is a no-op; chapter 08 cites the `06b` receipts

**Where.** `zkir-constraints.k:207-210`; `docs/08-constraints-and-verdicts.md:137`.

`#unsat(O, _) => O` does not distinguish `vUnsat` from `vErr` (the distinction lives in `#invNonZero` / `#lowHighBounds` instead). Dead. Chapter 08 still points at `evidence/zkir-k-circuit-differential-*-2026-09-06b.txt`; the current receipts are `06c`.

**Fix.** Delete `#unsat` or use it. Point the chapter at `06c`.

## Coverage gaps

- **JubjubScalar non-canonical bits.** No oracle column. Typed `IrValue` cannot carry `[r_J, 2^252)`. The residual is untested against MockProver.
- **Guarded-off non-Native cells with a consumer.** `transcripts_guard_off.zkir` marks `%pp` (JubjubPoint, then encoded) and `%ps` (Secp256k1Scalar). D5 skipped. No test that encode/hash of a guard-0 point is `W`/`C` under injection.
- **Guarded-off every type, unused.** D5 Native-only on swap `expire`/`decide`. No unused guard-0 Bytes32, Bool, Byte, foreign point, or JubjubPoint without a later `encode`.
- **`public_input` with guard 1 or `noGuard()` and no consumer.** Same circuit cell as guard 0; K reports `holds`. `test_extension_attack.zkir` inject-input `A` is the nearest Native case (D3), not tagged unconstrained.
- **Foreign well-formed non-canonical limbs.** Cannot be injected through `IrValue`. No `prove_unchecked` path that writes limb advice independently of `FieldChip::assign`.
- **`memory.insert` outputs other than Native.** D4 injects Native `from_bytes32` / `bytes32_into_low_high` outputs (`native_bytes.zkir` `%a3`). Extension `reverse` of `Bytes<n>` is uninjected.
- **Semantic ill-typed K memories.** No unit test that `witnessSpace` is false for `native(#r)`, off-curve `jubjubPoint`, short `bytes32`, or `secp256k1Base(p)`.
- **Hash padding cells and lookup rows.** Output KATs exist (`unit_hash.py`); internal padding/lookup witnesses do not.
- **`ok/unsupported`.** Table D8, not hit. An extension instruction whose `eval` falls through to `[owise]` (`zkir-constraints.k:560`) would be incomparable.
- **Weierstrass `from_coordinates` of the identity / torsion Curve25519 in-circuit.** `f11` is preprocess-rejected (D9). The in-circuit `violated` vs panic split (D1c) is not observed on a built circuit for foreign torsion.
- **`job` vs `checkedJob` immediates.** `rd(imm(I))` (`zkir-constraints.k:196`) does not reduce `I` mod `#r`. `checkedJob` is supposed to reject out-of-range immediates; a `job`-only memory with `copy(imm(#r), o)` is untested against the circuit’s `assign_fixed`.

## Questions for the authors

1. Is `witnessSpace` a predicate over arbitrary K maps, or only over memories `job` can produce from `decodeValue`? The header reads as the former; the “decoders build only such values” sentence is the latter. Finding 1 is a defect only of the former.
2. For compiler soundness, which inclusion is the theorem? Plan M2: modelled space over-approximates the circuit (modelled ⊇ circuit) so a proof about modelled memories covers circuit-accepted ones. Header: any modelled memory is circuit-accepted (modelled ⊆ circuit) up to residuals that run in *both* directions. Those are different statements. JubjubScalar extra bit-strings and D4 `memory.insert` live in circuit \ modelled; unused Native cells and `assert` of 2 live in modelled \ (honest preprocess).
3. Should `unconstrained` mean “this assigning relation does not pin the cell beyond type” (current `#guardedOff` / `#canonicity`) or “the cell has no consumer in the circuit” (D5’s `A`)? The residual list uses the second wording for the first implementation.
4. Is a well-formed non-canonical foreign-limb tuple considered an auxiliary witness of the same named value (safe to leave existential) or an extra representative that should be `unconstrained` like JubjubScalar? `assign_fixed`’s own comment treats it as a completeness issue of `assert_equal`, not of assignment.
5. `scalar_from_le_bytes` after `div_rem` leaves `enforced_canonical: false`. Do you want the flag mentioned in the `#canonicity` comment, or only the `assert_lower_than` constraint?

## What was checked

**K and docs.** `experiments/zkir-k/semantics/zkir-constraints.k` (header, `witnessSpace`, `unconstrainedRegs`, `inputGate`, `#canonicity`, `#guardedOff`, `#typed`, `#admissible`); `zkir-vm.k` (`#inputGates`, `#verdicts`, `#witnessSpace`, `#observable`); `zkir-values.k` (constructors, `decodeValue`, `encodeValue`); `zkir-ops.k` (`fromBytes32V`, `addV`); `zkir-curves.k` (`Point`, `inSubgroup`, `#rJ`); `zkir-hash.k` (`#shaPad`, `#kPad`); `zkir-ext.k` (`loadConstant` gate, concat/slice/nth/reverse); `docs/08-constraints-and-verdicts.md`; `plan-iter3/PLAN.md` M2 and the compiler-soundness paragraph; `plan-iter3/circuit-comparison-table.md`; `wiki/zkir/zkir-k-definition.md` CLM-0765; `review-2026-09-05/CONSOLIDATED.md` (no re-report).

**Crate and chips.** `ledger-92e8bdd3/zkir-v3/src/ir_vm.rs` (`circuit`, `mem_insert`, public/private input, copy, hashes, byte ops); `ir_instructions/assign.rs`, `encode.rs` (`jubjub_scalar_from_biguint`, JubjubScalar encode comment), `from_bytes32.rs`, `from_coordinates.rs`; `midnight-zkir-2ffe2d1/zkir/src/ir_vm.rs` (same arms plus `reverse`/`concat`/`slice`/`nth`/`load_constant`); `assign.rs`, `assign_constant.rs`. midnight-circuits 7.2.4: `ecc/native/edwards_chip.rs` (`assign`/`assign_fixed`/`scalar_from_le_bytes`/`q_mem` membership gate/`point_from_coordinates`); `field/foreign/field_chip.rs` (`assign`, `well_formed_log2_bounds`); `field/native/native_gadget.rs` (`AssignedByte::assign`); `hash/sha256/sha256_chip.rs` (`pad`); `biguint/biguint_gadget.rs` (`div_rem`); `ecc/foreign/edwards_chip.rs` and `weierstrass_chip.rs` (`point_from_coordinates`). midnight-curves 0.3.1 `jubjub/fr.rs` (`MODULUS_BITS = 252`). midnight-zk-stdlib 2.3.5 `lib.rs` (`assign_many` delegates to the native gadget).

**Receipts.** `evidence/zkir-k-circuit-differential-92e8bdd3-2026-09-06c.txt`: 464 circuit comparisons, 463 agree, 0 outside the table, 1 N/C (D3 undecided `cond_select` bit); cells include `inject-unconstrained: A (D5) x3`, `inject-ignored: A (D4) x6`; `witness space` on the off-circuit summary is every successful K run in the space. `evidence/zkir-k-circuit-differential-ext-2ffe2d1-2026-09-06c.txt`: 498 comparisons, 496 agree, 0 outside, 2 N/C (same D3 class); 37/37 successful K runs in the space, unconstrained registers `test_ec_proof.zkir %s1`, `curve_jubjub.zkir %s`, `transcripts_guard_off.zkir %pp,%ps`. No `DISAGREE`. No K `violated`/`synthErr` on an oracle `accepted` honest run.

**Not run.** `kompile` / `krun` / `uv run` were not required: the suspicions are source mismatches, and the `06c` receipts already contain the D3/D4/D5 cells. Oracle binaries were not invoked.

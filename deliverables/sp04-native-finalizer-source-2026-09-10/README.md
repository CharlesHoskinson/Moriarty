# Native finalizer source diagnosis

**Proposal: no-go for F0a finalizer authorship or F1 execution on the evidence now retained.** The missing bound is the total constrained cost of the complete pairing predicate in an exact outer stack. This is an engineering evidence gap, not a proof that the route is impossible. No backend choice or resource allocation is approved by this document.

The useful candidate for any subsequently reviewed local investigation is the retained **ledger-9 V3** stack: its exact dependency archives are now available and checksum-verified. This removes one historical source-availability obstacle. It does not establish that Preview runs this stack, or authorize a target change. Do not repeat the k17 financial IVC run to answer this question.

## Exact source and field boundary

[Source evidence](source-evidence.json) records pinned Git bytes, line excerpts, exact archive checksums and the bounded search. Native source is `midnight-zk@695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7`; ledger-8 is `a8ab82ba2124c36f92795c683e70bd888bc1d1fb`; the retained ledger-9 alternative is `0d364eb9f8c388a0399d05f59f2468c96d765015`. All 21 inspected native/ledger-8 working files match their pinned Git objects. Ledger-9 excerpts come directly from its Git object.

The native `BlstrsEmulation` uses BLS12-381 G1 and the scalar field named **Fq** in Midnight (`circuits/src/verifier/types.rs:140–153`). Its coordinate field is **Fp**, not Fq:

| Quantity | Exact modulus or encoding |
| --- | --- |
| Coordinate field Fp, 381 bits | `0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab` |
| Circuit/scalar field Fq, 255 bits | `0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001` |
| Foreign-field representation | Seven limbs, base `2^56`; the assigned representation uses the shift `1 + sum(limb[i] * base^i)` modulo Fp. Normalization, bounds and canonicity remain necessary. |
| Native curve bytes | G1 compressed/uncompressed sizes 48/96 bytes; G2 96/192. Checked compressed decoding also calls the subgroup check. This diagnosis does not implement the BLST byte decoder or prove equivalence to its flags/exception rules. |

The exact candidate outer archives `midnight-curves-0.3.1` and `midnight-circuits-7.2.4` use the same moduli and seven-limb parameters. This proves equality of these source constants, not API or proof-format compatibility. The scalar representation is little-endian; foreign shifted limbs and curve compressed bytes are different encodings and cannot be interchanged.

The [lightweight check](check-public-constants.py) read these public constants and passed nine shifted-limb boundary roundtrips. It also exhibited a concrete lossy cast: distinct canonical base-field values `0` and `Fq.modulus` both reduce to scalar zero. Neither value was claimed to be a valid curve point. Therefore a one-scalar coordinate cast cannot replace the foreign-field representation. [Results](public-constant-results.json) establish only integer arithmetic, not circuit soundness or pairing correctness.

## What the finalizer must constrain

Native `proofs/src/poly/kzg/msm.rs:294–309` evaluates two G1 MSMs and accepts exactly when the two-term Miller-loop result, after final exponentiation, is the target-group identity. The terms use `s_g2_prepared` and `n_g2_prepared`. `params.rs:364–399` binds these to the inner SRS: `[tau]G2` and negative G2; the serialized-verifier reader uses the negative canonical generator, while conversion from full parameters uses the negative stored `g2`. Their equality must be established for the actual selected SRS; it cannot be inferred from a filename.

These are **inner** verifier constants. They must not be replaced by the outer ledger SRS. The selected inner SRS point bytes, exact constant line table and retained nontrivial fixture have not been frozen in this diagnosis.

`curves/src/bls12_381/g2.rs:758–781` allocates 68 prepared line entries for a nonidentity fixed G2 point. `mod.rs:49–71` calls BLST's host Miller-loop routine for each nonidentity term, multiplies the two Fp12 results and handles identity terms explicitly. `bls_pairing.rs:42–46` calls host final exponentiation. These calls are not constrained gadgets. Precomputing fixed-G2 lines could remove variable-G2 arithmetic, but both variable G1 inputs, line evaluations, extension-field arithmetic and the final identity predicate still need constraints.

The existing `prepare` gadget returns an assigned accumulator and expressly conditions proof validity on its final check (`verifier_gadget.rs:850–865`). Native IVC also combines the final proof accumulator with the carried accumulator. A finalizer must consume the same combined statement, not one arbitrary substituted point pair.

## Reusable arithmetic and concrete missing operations

| Boundary | Source-grounded disposition |
| --- | --- |
| Fp over Fq | Existing `FieldChip`: constrained add/subtract/multiply/divide/inverse, normalization, equality, zero and bit/byte conversion; seven 56-bit limbs, range-check limb size 15, auxiliary moduli `2^134` and `2^134-1`. Native `field/foreign/params.rs:277–288`, `field_chip.rs:894–1008`; matching parameters exist in exact outer circuits 7.2.4. |
| G1 | Existing foreign Weierstrass gadget provides assignment, on-curve/subgroup-capable assignment, addition and bounded-scalar MSM. Calls which explicitly skip subgroup checking remain distinct. Canonical point/identity handling and the exact accumulation inputs need review. |
| Fp2/Fp6/Fp12 | Host types exist. The host tower documents Fp6 using `v^3=u+1` and Fp12 using `w^2=v`. No constrained extension-field implementation was found in all 139 Rust source files of exact circuits 7.2.4 or all seven stdlib 2.3.5 source files under the recorded search. |
| Complete pairing | Missing constrained fixed-line evaluation, extension-field product/squaring schedule, final exponentiation or a proved equivalent membership relation, and final identity assertion. A replacement inversion witness needs a constrained nonzero denominator; a host error check is insufficient. |
| Complete verifier | Missing measured/defensible total port cost, strict transcript byte/point acceptance correspondence, canonical export, selected inner SRS constants and independent valid/invalid fixtures. The native preparation gadget alone does not close these boundaries. |
| Compact/ZKIR elliptic-curve instructions | `EcAdd`/`EcMul` operate on the embedded Jubjub curve (`ir_vm.rs:779–799`, `transient-crypto/curve.rs:63–72`), not BLS G1 pairing points. Native field `Add`/`Mul` are scalar-field arithmetic; they are not Fp or pairing primitives. |

A generic extension-field implementation may be possible using the base-field gadget. That is an inference about a construction route, not implemented support. There is no evidence here of an automatic native-0.8 to ledger-0.7 port.

## Outer stack, limits and fit

Ledger-8's locked proofs 0.7.1/circuits 6.2.0/stdlib 1.2.0/curves 0.2.0 remain absent in the bounded Cargo cache check. The ledger-9 lock instead has V3 proofs 0.8.2/circuits 7.2.4/stdlib 2.3.5/curves 0.3.1 and an older V2 dependency set. All eight corresponding archives were found and matched the exact lock checksums. No source was downloaded or dependency changed.

Both inspected ledger families use ordinary Blake2b proof verification; selecting V3 does not expose a native IVC/carried-accumulator acceptance argument. Preview binary/backend/features/SRS alignment remains unverified for this proposed route.

The ledger-8 `VERIFIER_MAX_DEGREE = 14` is documented as limiting public inputs, **not total circuit rows** (`transient-crypto/proofs.rs:103–127`). Its embedded verifier parameter source is `static/bls_midnight_2p14`. The pinned public-parameter catalog includes k0 through k25 with content hashes (`base-crypto/data_provider.rs:82–210`). A catalog entry does not grant a proving allocation or prove the running Preview binary's identity. No outer finalizer k, exact outer SRS artifact, advice-column configuration, degree/blinding overhead or CPU/memory envelope has been selected and reviewed.

**Fit result: unknown, therefore no-go under the current F0 rule.** Seven limbs and 68 fixed line entries are representation/schedule facts, not a row estimate. No defensible full-circuit bound follows without an implemented or fully specified extension-field/Miller/final-exponentiation schedule, its base-field normalization/range-check cost, row packing and selected outer parameters. Do not invent a time/row estimate or treat the catalog's largest k as permission to increase k.

The narrow next technical question is whether one fixed-line two-term finalizer schedule, expressed using the exact outer Fp gadget, fits a reviewed outer envelope. Resolving it requires an operation/normalization and row-bound accounting with explicit overhead, or a separately reviewed finite falsifying P3 probe once its prerequisites are met. A successful base multiplication or identity-only accumulator would not answer that question. P3 must accept an independently verified nontrivial fixture and reject direct assigned-point/residual mutations; all P1/P2/P3 gates still apply before financial F2.

## Scope and preservation

This source diagnosis creates no admission, gate completion, native implementation, build, proof or transaction. Backend selection and consequential resource decisions still require fresh GPT-6 Astra and Grok 4.6 reviews. The original R3 k17 exhaustion, zero proofs and 220 rounded charged seconds remain; its historical 1200-second cap and source-only successor 1800-second cap are not refreshed balances. Preserve all subsequent preparation/review/build/runtime charges. Ordinary Preview loan/swap evidence and host pairing do not discharge mandatory PCD or SP09/SP11/SP12.

Reproduce the public arithmetic check only:

```sh
python3 check-public-constants.py /home/charl/Moriarty /home/charl/.cargo/registry/cache/index.crates.io-1949cf8c6b5b557f
```

The initial script-generation syntax error was corrected before the recorded successful run. No Rust or operational module was imported.

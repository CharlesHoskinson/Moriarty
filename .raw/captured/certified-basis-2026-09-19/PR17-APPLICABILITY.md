# PR17 theorem-applicability audit

Assessment date: 2026-09-19. **Useful conditional mechanization; not an unconditional certificate for current ZKIRv3, Moriarty compilation, or Midnight deployment.** The essential additional integration issue is its substantive witness-side `WShape` premise, which must not be collapsed into a static producer check.

This audit reads actual captured Agda statements and definitions. It did not run Agda/Nix, rebuild Halo2, test a live prover, or execute a ledger transaction. The PR's clean typecheck is an author-reported result, not reproduced here. This is an applicability analysis, not a PR approval or a security exploit report.

## 1. Three identities must remain distinct

| Object | Pinned identity | Meaning |
|---|---|---|
| Proposed mechanization | PR17 head `ebb662c716fef2638ec1b0f42805a8bce23c75dd` | Captured Agda source; PR open and unmerged in the retrieved metadata |
| Implementation surface modeled by that mechanization | `5b593d1`, identified as midnight-zkir 3.0.0; README says equivalent to prior midnight-ledger `92e8bdd3` surface | Thirteen IR types and 34 instructions |
| PR base/current `zkir-v3` branch snapshot | `2ffe2d17bbb736aec36fb300aeaca679a10d2278` | Contains a later instruction/type batch, not covered merely by sharing major version 3 |

The PR explicitly excludes later `LoadConstant`, Boolean operations and `Bool`, `Byte`, generalized `Bytes(n)` with indexing/concatenation/slicing/reversal, `Sha512`, and stricter decoding canonicity. Existing operations can also have changed semantics; checking that emitted operation names belong to the old list is insufficient without a semantic mapping. This analysis does not independently establish Rust byte identity at the historical pin.

Primary metadata: [PR17](https://github.com/midnightntwrk/midnight-zkir/pull/17). Scoped source map: [pinned README](https://github.com/midnightntwrk/midnight-zkir/blob/ebb662c716fef2638ec1b0f42805a8bce23c75dd/zkir-spec/src/zkir-v3/README.md).

## 2. What the theorem statements actually say

### Canonical-witness faithfulness

`CircuitProof.agda:2130` declares, schematically:

```text
producer-WT S
→ init S P = just st0
→ BwdWalk P S st0 instructions s
→ Consumed P s
→ (preprocess S P = just s) ⇔ satisfies (synth S) (witness-of P s)
```

This relates a **particular canonical witness** to its run, with initialization, backward-shape and terminal-consumption data. It is not by itself a theorem quantifying over all arbitrary witnesses that an adversarial prover may produce. The source supplies a projection from successful runs to the shape spine; that establishes availability for honest runs, not removal of the hypotheses.

### Statement soundness

`StatementSoundness.agda:5728` declares:

```text
producer-WT S
→ satisfies (synth S) w
→ WShape S w
→ SubRealizer S w
```

The `SubRealizer` record at line 5444 contains a preimage and final state, successful `preprocess`, equal public-input lists, agreement of run memory with the witness **on the run's domain**, commitment-randomness agreement when enabled, and absence of a vestigial commitment when disabled.

It deliberately does **not** conclude full witness equality. Off-domain witness assignments can remain unconstrained. This is a meaningful, appropriately limited statement, but an application must identify which public inputs and run cells bind its financial effects.

### Uniqueness and extractor completeness

`StatementUniqueness.agda:836` proves equality of the preimages and states of two sub-realizers of the **same witness**, under source/input premises. The combined theorem at line 904 additionally assumes `producer-WT`, satisfaction and `WShape`. It does not establish a unique witness or unique transaction for a public-input statement, and it is not a theorem of collision resistance or unique ledger consumption.

`extractor-complete` at `StatementSoundness.agda:5750` says a sub-realizer's reconstructed canonical witness satisfies the modeled circuit. Its name must not be confused with universal compiler completeness or a cryptographic knowledge extractor for a deployed SNARK. These are logical model witnesses, not a security reduction proving that an arbitrary accepted serialized proof yields them.

Sources: [CircuitProof](https://github.com/midnightntwrk/midnight-zkir/blob/ebb662c716fef2638ec1b0f42805a8bce23c75dd/zkir-spec/src/zkir-v3/CircuitProof.agda), [StatementSoundness](https://github.com/midnightntwrk/midnight-zkir/blob/ebb662c716fef2638ec1b0f42805a8bce23c75dd/zkir-spec/src/zkir-v3/StatementSoundness.agda), [StatementUniqueness](https://github.com/midnightntwrk/midnight-zkir/blob/ebb662c716fef2638ec1b0f42805a8bce23c75dd/zkir-spec/src/zkir-v3/StatementUniqueness.agda).

## 3. WShape is a real security-relevant hypothesis

The definitions at `StatementSoundness.agda:2372–2446` require more than allocation shape:

| Instruction/context | Additional witness premise |
|---|---|
| Declared inputs | Actual cells have the declared types |
| Public/private transcript input | Guard resolves to a Boolean; active cell has declared type; inactive cell equals the profile default |
| Impact | Guard reads as a Boolean even for an empty impact list |
| Assert | Condition reads as Boolean true |
| Reconstitute-field | Integer reconstruction stays below field order |
| Div-mod-power-of-two | Exactly two output identifiers |
| Circuit-output | Output collection succeeds against the declared output types |
| From-bytes32 | The chosen constrained value has the instruction's declared type |
| Less-than | Operands satisfy the exact `2^bits` bounds, not only the chip's widened bounds |

Two concrete source comparisons show why these are substantive:

- `Circuit.agda:802` synthesizes `assert` as `non-zero`, whose semantics at line 537 only reject zero. `Semantics.agda:76–82,239–240` reads Boolean values as exactly 0/1 and accepts assert only at true. `WShape` supplies the stronger Boolean-true condition. In the intended native field, a nonzero non-Boolean value illustrates the gap between those local predicates. This is a source-level applicability example, not an executed whole-program exploit.
- `Circuit.agda:564–569` permits less-than operands below `2^even4(bits)`, where `even4` rounds up and has a minimum of four bits. `WShape` requires below `2^bits`. For small/odd bit widths these are different domains.

`WShape?` is a decidable checker, but **running it only on the honest producer's witness does not show that every adversarial witness accepted by the actual circuit satisfies it**. `preprocess→WShape` proves that successful runs yield shaped witnesses; this implication cannot be reversed into a universal fact about satisfying witnesses.

A Moriarty application must therefore derive the premise from its supported source/profile and all accepted witnesses, enforce the needed constraints in the actual acceptance relation, or prove a separately adequate relation with a weaker conclusion. Merely naming producer obligations as discharged leaves this issue unresolved. None of these options requires a human allowlist of programs.

Sources: [Circuit definitions](https://github.com/midnightntwrk/midnight-zkir/blob/ebb662c716fef2638ec1b0f42805a8bce23c75dd/zkir-spec/src/zkir-v3/Circuit.agda), [Semantics](https://github.com/midnightntwrk/midnight-zkir/blob/ebb662c716fef2638ec1b0f42805a8bce23c75dd/zkir-spec/src/zkir-v3/Semantics.agda), and StatementSoundness above.

## 4. Producer and model-correspondence obligations

`producer-SA` checks unique input names and fresh, nonduplicated outputs. `producer-WT` adds operand/output typing. Neither is a complete statement of financial correctness or all witness preconditions.

`producer-WF2` checks structural bit bounds: constrain-bits and less-than use `bits < FR-BITS`; div/mod and reconstitution use `bits ≤ 248`. The mechanized `step` omits these dynamic guards. The defined `stepʳ`/`preprocessʳ` reinstate them, and `preprocessʳ-agree` proves agreement **between the two Agda semantics** for WF2-conforming sources. Although called Rust-faithful, this is not a mechanically verified equivalence to the compiled Rust binary. That correspondence still requires evidence.

Decidability of these checks is useful for automatic producer enforcement. It supplies no target-platform cost bound until their concrete implementations, inputs and execution cost model are tied together. [Obligations.agda](https://github.com/midnightntwrk/midnight-zkir/blob/ebb662c716fef2638ec1b0f42805a8bce23c75dd/zkir-spec/src/zkir-v3/Obligations.agda).

## 5. What remains assumed

All principal modules are parameterized by `Assumptions`. The record supplies carrier types and operations for fields, curves, encoders, hashes and commitments, plus nontriviality and encoding round-trip laws. Many arithmetic and hash relationships are used through shared abstract operations; the development is not a derivation of concrete chip gates from field/group definitions.

The source specifically models foreign limb decoders as canonical partial inverses. The relevant deployment must satisfy those assumptions; the same record does not prove this. The range/decomposition, curve and hash constraints are functional contracts, not a checked derivation of their Halo2 implementations. A source comment about a particular Jubjub subgroup check is useful review evidence, not a general concrete-backend proof reproduced here. Cryptographic collision resistance, commitment hiding, SNARK knowledge soundness, verifier/key/SRS provenance and ledger uniqueness are not furnished by the module's use of opaque functions.

`--safe` and absence of postulates prevent particular unchecked proof escapes inside the development. They do not instantiate or discharge a record passed as a premise, nor prove that the real Rust/Halo2 backend models it. [Assumptions.agda](https://github.com/midnightntwrk/midnight-zkir/blob/ebb662c716fef2638ec1b0f42805a8bce23c75dd/zkir-spec/src/zkir-v3/Assumptions.agda).

## 6. Required composition before a Moriarty certificate claim

A primitive certificate should identify its exact operation surface and semantic pin; producer-SA/WT/WF2 evidence; derivation or enforcement of every relevant WShape condition for accepted witnesses; the concrete chip/encoding instantiation; and the theorem connecting the Moriarty reference operation to the emitted target constraints. It must separately preserve checked-width arithmetic, failure behavior, complete effects and declared work. Composition needs frame/state/authority invariants, not only equality of one returned scalar.

The deployed path additionally needs proof-system assumptions, actual circuit and verifying-key identity, serialization and binding, mandatory ledger verification, owner-authorized intent and predecessor consumption, phase-specific effects, and finalized outcomes. No source-model theorem by itself proves those deployment facts.

**Recommended public description:** PR17 provides a conditional Agda mechanization of an older pinned ZKIRv3 subset, including canonical faithfulness and shaped-witness statement soundness. Reuse requires discharging its source **and witness** premises and proving the concrete compiler/backend/ledger links. It is promising proof infrastructure, not existing end-to-end Moriarty certification.

## Evidence retention

The parent captured raw Assumptions, README and StatementSoundness sources with Scrapling receipts under `sources-receipts.json`. This audit additionally reconstructs seven new Agda files from the captured PR diff in `pr17-extracted/`, with hashes and extraction method in its manifest. Those reconstructions retain exact added source lines; they were used for statement inspection, not compiled. No large build or repository mutation was performed.

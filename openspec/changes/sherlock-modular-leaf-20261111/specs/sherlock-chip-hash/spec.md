## ADDED Requirements

### Requirement: HSH-001 The pinned sponge is unchanged for digests that leave the leaf
The HASH chip SHALL compute the pinned zk-stdlib Poseidon sponge with the length in the start state for every digest that leaves the leaf or that `sherlock-value/1` defines. Such a call site SHALL NOT add a capacity domain word.

#### Scenario: Differential against the stdlib
- **WHEN** the AIR sponge is compared with the pinned stdlib on even and odd lengths, zero, repeated and maximal inputs
- **THEN** the outputs are equal (the landed differential vectors, and `sponge_iv` against `sponge_native_iv`)

#### Scenario: Capacity tag on a value digest
- **WHEN** a `sherlock-value/1` call site uses `sponge_iv` with a capacity word
- **THEN** the differential vector fails (mutant `hash_value_digest_given_capacity_tag`)

### Requirement: HSH-002 Internal hashes carry a capacity domain word from one registry
Every leaf-internal hash SHALL carry a capacity domain word from one frozen domain registry. The capacity word SHALL be injective in `(dom, sub, len)` and non-zero. The registry SHALL be frozen in group F.

#### Scenario: Domain swapped
- **WHEN** a `DOM_NODE` call is given the `DOM_REC` capacity word
- **THEN** the correct AIR refuses the witness (mutant `hash_domain_swapped`), the Quint model `hashProtocol.qnt` checks domain separation, and O7 (`HashPremises.lean`, the hash premises as one structure, open premise statement) states the injectivity hypothesis

### Requirement: HSH-003 Inputs and outputs are copy-connected and no provider row has a free output
A whole-sponge slot SHALL take ordered input cells through its port and SHALL return one digest from the final state, with a fixed length and a fixed capacity word per call site. The two-input provider SHALL serve the FP-1 HASH message `(tag, x, y, h)` with `h = P2(tag, x, y)` and identity `(tag, x, y)`. The prototype tuple `(3, BUS_KEY, x, y, H([x, y]))` is the halo2 form only.

#### Scenario: Copy removed
- **WHEN** a caller's cell differs from the sponge input that it is said to feed, or a digest cell differs from the final state
- **THEN** the correct AIR refuses the witness (mutants `hash_input_copy_removed`, `hash_output_copy_removed`), and the controls without that copy accept it

#### Scenario: Round or framing tampered
- **WHEN** the state, a round constant, a round, an absorb, the length, the odd tail, the chip id or the key is tampered
- **THEN** the correct AIR refuses the witness (mutants `hash_state_tampered`, `hash_round_constant_changed`, `hash_round_skipped`, `hash_absorb_skipped`, `hash_length_changed`, `hash_odd_tail_absorbs_padding`, `hash_chip_id_forged`, `hash_key_forged`)

### Requirement: HSH-004 A constant-start mode for VALUE DIGEST is gated on X9
A HASH constant-start mode that folds a per-length and per-form start state into the absorb row SHALL be added only if experiment X9 shows that it adds no fixed column (V-5). Without it, VALUE DIGEST SHALL use the full sponge.

#### Scenario: X9 decides the mode
- **WHEN** the constant-start mode is proposed
- **THEN** X9 records a differential test against `sponge_native` and a key-shape check of the fixed columns, and O26 (`ValueDigest.lean`, constant-start equivalence, open) is its Lean evidence

### Requirement: HSH-005 The co-processor digests are of the `sherlock-value/2` form
Under `sherlock-processor-final-20261111` the co-processor digests SHALL be of the `sherlock-value/2` form: Poseidon2 over BabyBear inside the STARK layers and `PoseidonFr(tag, pack(d2))` at the Fr boundary, with salted `input_d` and `outcome_d` (HDM-001, HDM-002, ZK-006, SPL-003). The `sherlock-value/1` form of HSH-001 is the halo2 prototype form and SHALL NOT be a co-processor digest. HASH SHALL compute `P2(tag, words)` with the tag codes of `FD/ARCHITECTURE.md` section 1.11, FP-4 (FPR-011).

#### Scenario: Prototype digest offered as a co-processor digest
- **WHEN** a co-processor public digest is computed with the `sherlock-value/1` zk-stdlib sponge in place of the `sherlock-value/2` form
- **THEN** it differs from the `PoseidonFr(tag, pack(d2))` value that the wrap checks (HDM-002), so the co-processor uses the FP-4 table (FPR-011) and refuses the `sherlock-value/1` value (mutant `digest_tag_swapped`)

### Requirement: HSH-006 The HASH AIRs meet the STARK kit gate
The HASH AIRs SHALL meet the STARK kit gate SG-1 to SG-6 of `design.md` section 9 inside the app set (at most 5,000 constraints and 1,000 interactions per AIR, 100 AIRs, 30,000 columns), with every AIR required and a fixed height in its menu shape. Where the inline kernel of SHP-006 carries them, each AIR SHALL also fit the per-AIR bounds of the internal set (1,000 constraints, 200 interactions).

#### Scenario: AIR omitted from the proof
- **WHEN** a proof omits one of the HASH AIRs
- **THEN** N1, N3 and `L` refuse it (mutant `hash_air_omitted`), and the kit does not pass SG-1 before row T3 passes

#### Scenario: Positive and negative proofs on the stock verifier
- **WHEN** the FP-4 vectors `digest_table_vectors` (FPR-011) and the mutant `hash_round_skipped` are proved with the stark-backend prover
- **THEN** the stock verifier accepts the first, refuses the second and accepts the second under its weakened control AIR; a prover panic or tracegen error earns no rejection credit

#### Scenario: Shape, budget and D3
- **WHEN** the HASH AIRs are keyed in a menu shape
- **THEN** each AIR has the fixed height of the shape, keygen refuses a shape outside the bounds above, D3 at the measured shape prints every term at 100 bits or more and every grindable term at 101 or more (SG-5), and the receipt records seconds per proved step (SG-6)

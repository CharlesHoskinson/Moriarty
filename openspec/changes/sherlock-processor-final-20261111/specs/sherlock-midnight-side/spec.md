## ADDED Requirements

### Requirement: MID-001 The ledger verifies the wrap as it is
The ledger SHALL check each Sherlock `ContractCall` with `midnight_transient_crypto::proofs::VerifierKey::verify` on the key installed under the entry label. The wrap SHALL be a midnight-proofs PLONK proof with KZG over BLS12-381 and the Blake2b transcript, so ledger 9.1.0.1 needs no change (CPR-004).

#### Scenario: Wrap on the unchanged ledger
- **WHEN** a wrap proof is submitted on a local ledger 9.1.0.1 devnet
- **THEN** the ledger verifies it with no ledger change (frontend M2b and M3)

### Requirement: MID-002 Every exported entry holds the universal wrap key
Every exported entry of a Sherlock contract SHALL hold the same universal wrap key.

#### Scenario: Entry with its own key
- **WHEN** a contract has Sherlock entries and native-routed entries
- **THEN** every Sherlock entry holds the universal wrap key, and only the native-routed entries of MID-005 hold their own keys

### Requirement: MID-003 Ledger limits
The wrap verifier key SHALL be at most 50,000 bytes. A call SHALL have at most 65,536 public inputs. The wrap SHALL have `2 + L_TX` = 1,026 (OD-7, OD-15). A transaction SHALL be at most 1,048,576 bytes. `VERIFIER_MAX_DEGREE` 14 SHALL be read as a bound on the public inputs, not on the circuit k.

#### Scenario: Key over the limit
- **WHEN** the wrap verifier key exceeds 50,000 bytes
- **THEN** the key is refused (DG-003)

### Requirement: MID-004 Wrap evidence on chain
The wrap SHALL be qualified on chain by its own records: verifier-key bytes, proof bytes, verify time and fee at `2 + L_TX` public inputs, then a deploy and call of the wrap key on the local devnet and on Preview. The records of today's k = 21 decider SHALL be read as evidence of the transaction path only.

#### Scenario: Refusals on the devnet
- **WHEN** a wrap call has a changed result, a flipped proof bit, or a proof for another program digest
- **THEN** the ledger refuses each call, as it refuses them for today's k = 21 decider (MEASURED, `PR` section 9)

#### Scenario: Wrap figures pending
- **WHEN** the wrap figures are reported before M2b
- **THEN** verifier-key bytes, proof bytes (8 to 9 kB, ESTIMATE, `PR` sections 8 and 9), verify time and fee are PENDING

### Requirement: MID-005 Small entries may stay native
A small entry of a hybrid contract MAY route native under its own key at compile time, outside the Sherlock key set (OD-8). A native-routed entry SHALL run the anchor prologue of ANC-009.

#### Scenario: Native entry without the prologue
- **WHEN** a hybrid contract has a native-routed entry without the prologue
- **THEN** the frontend refuses the contract

### Requirement: MID-006 Network pins and Mainnet
The Preview qualification SHALL name the node build that the Preview node reports, `2.1.101-bbd616b1`. The Preview pin SHALL record the same build. A Mainnet operation SHALL follow SWF-009 of `sherlock-vm-consensus-20261107`.

#### Scenario: Pin differs from the node
- **WHEN** a Preview qualification record is written
- **THEN** it names the reported node build, and that build equals the Preview pin

### Requirement: MID-007 Lean obligations of the Midnight side
The design lists no Lean obligation for the Midnight side. The settlement rule SHALL rest on LDG-L1 of kit LEDGER CONTRACT in `sherlock-modular-leaf-20261111`.

#### Scenario: Obligation status
- **WHEN** the status is reported
- **THEN** it reads "no Lean obligation in the design", and LDG-L1 (O35) is open in kit LEDGER CONTRACT

## ADDED Requirements

### Requirement: CSL-001 A collection is proved by a collector run
CSL-001: When a checkpoint seals a collection, the Sherlock prover SHALL prove it with a collector run that consumes each member claim through DL-STEP and is sealed by SEAL-WITH-DEPS.

Source: FD-30 (C-01 (a), chair decision, upheld by the tie audit). Evidence: DL-STEP and SEAL-WITH-DEPS ran 48/48 on the universal keys, MEASURED (`lset-keys/VERDICT.md`). Gate: E-COLL. Dissent: astra-1, astra-2, astra-3 hold paired-root certificates.

#### Scenario: Honest collection on a devnet
- **WHEN** E-COLL proves a 4-member collection with one `FOREIGN` and one `DEP` member and one nested level
- **THEN** a local devnet accepts it through `acceptCollection`

### Requirement: CSL-002 Fallback to paired-root certificates
CSL-002: If E-COLL fails a pass bar of the collector route, then the Sherlock prover SHALL bind `claims_root` through paired-root certificates (ledger C-01 option b).

Source: FD-30.

#### Scenario: Collector cannot bind deps_d
- **WHEN** E-COLL shows the collector cannot bind a computed `deps_d`
- **THEN** the certificate route is built before the final keys

### Requirement: CSL-003 Collector checks its dependency chain
CSL-003: The collector run SHALL recompute the `SHK_DEPS` chain of its consumed claims and check it against its own `deps_d`.

Source: FD-30. Evidence: today R_SL fixes `deps_d` to `EMPTY_DEPS` (`sherlock-leaf/src/core.rs:250`).

#### Scenario: List differs from deps_d
- **WHEN** a forged collector run lists members that differ from its `deps_d`
- **THEN** the stock verifier refuses the proof

### Requirement: CSL-004 Collector computes the claims tree
CSL-004: The collector run SHALL derive each member's `claim_pub` and hash the depth-20 `H2` claims tree with the `z_d` empty-subtree constants and halt with its root.

Source: FD-30, A-10.

#### Scenario: Wrong root with a valid SET
- **WHEN** a collector run halts with a root that is not the tree of its members
- **THEN** the proof is refused

### Requirement: CSL-005 Collector chip set and statement AIR
CSL-005: The collector SHALL run on an R_SL-class chip set whose statement AIR computes `deps_d`.

Source: FD-31 (C-03 (b), 6 of 6). Gate: E-SHAPE prints the statement AIR within the app bounds.

#### Scenario: Statement AIR shape
- **WHEN** E-SHAPE prints the collector statement AIR
- **THEN** it is within the app bounds

### Requirement: CSL-006 One self-nesting collector image
CSL-006: The collector SHALL be one self-nesting program image of fan-in 32 that reads its own `prog_d` and `reg_root`.

Source: FD-31. Label: fan-in 32 is a design parameter; 2^20 slots per `claims_root`.

#### Scenario: Nested level
- **WHEN** a collector consumes the claim of another collector run of the same image
- **THEN** the outer `claims_root` covers the inner members

### Requirement: CSL-007 Collector pinned by its ledger anchor
CSL-007: The `acceptCollection` entry SHALL accept a collection only from the program whose `C_P` is the anchor of its ledger.

Source: FD-31 (N-11 anchor).

#### Scenario: Wrong collector prog_d
- **WHEN** a collection proved by another program is submitted
- **THEN** the ledger refuses it (`L_Fr` asserts `A_0 = C_P`)

### Requirement: CSL-008 Fr Poseidon by the R_SL micro-op permutation
CSL-008: The collector run SHALL compute the Fr Poseidon work of the claims tree with the existing R_SL micro-op permutation at 6 permutations per run member.

Source: FD-32 (C-05 (a), chair decision, upheld). Gate: E-PERM.

#### Scenario: Evidence of CSL-008
- **WHEN** the cost is reported
- **THEN** it reads 1,476 ALU rows and 864 multiplies per permutation, DERIVED (`rsl/compile.rs:347-382`), 8.7 ms per permutation on the app proof, DERIVED, and the dissent of astra-1, astra-2, astra-3 (a dedicated relation)

#### Scenario: E-PERM over the bar
- **WHEN** E-PERM measures more than 30 ms or 3,000 rows per permutation
- **THEN** a dedicated AIR replaces the micro-ops before the final keys

### Requirement: CSL-009 Collector halts with its own entry record
CSL-009: The collector run SHALL halt with a call record of its own `acceptCollection` entry whose effect words put `claims_root` at lanes 29 and 43.

Source: FD-33 (C-02 (b), chair decision, upheld). Label: lanes 29 and 43 MEASURED (T20). Gate: E-COLL on a devnet.

#### Scenario: Shape and fallback of CSL-009
- **WHEN** E-COLL cannot settle the non-fallback record
- **THEN** the fallback is built: a collection seal in `L`, a third `C_FB` in `L_Fr`, a third `R` form and a canonical 9-limb lane map in the decoder, inside the rows of WRB-003; otherwise `L` gains only a root tag column and the `DEP` head opening

#### Scenario: Root altered at one lane
- **WHEN** `claims_root` at lane 29 or lane 43 is altered
- **THEN** the stock verifier or the ledger refuses the transaction

#### Scenario: Wrong-entry reuse
- **WHEN** the collector record is offered to another entry
- **THEN** the ledger refuses it

### Requirement: CSL-010 Members of every kind
CSL-010: The collector run SHALL accept run, `DEP` and `FOREIGN` members.

Source: FD-34 (C-04 (a) with (c)'s normalization, chair decision, upheld). Hard rule: GP-1 to GP-4 never narrowed; a run-only collection does not ship (FD-34). DL-STEP gains a `FOREIGN` head form; T20's `SHK_CLAIM_FGN` leaf stays.

#### Scenario: Mixed collection
- **WHEN** E-COLL settles a collection with one `FOREIGN` and one `DEP` member
- **THEN** both claims are in `claims_root`

#### Scenario: Run offered as foreign
- **WHEN** a run claim is offered as a `FOREIGN` member
- **THEN** the proof is refused

### Requirement: CSL-011 Normalization relation
CSL-011: The collector run SHALL implement a written normalization relation covering `DEP` heads and covers, diamonds, repeated reachable claims, unbalanced sources and the 2^20-slot capacity.

Source: FD-34. Gate: the Quint model of normalization and E-COLL.

#### Scenario: Member at two slots
- **WHEN** one claim is placed at two slots
- **THEN** the proof is refused

#### Scenario: Dropped member
- **WHEN** a reachable claim is left out of the tree
- **THEN** the proof is refused

### Requirement: CSL-012 ADM and VIEW are never spendable
CSL-012: If an `ADM` or `VIEW` head reaches the collector, then the collector run SHALL keep it out of the spendable claims.

Source: FD-34.

#### Scenario: VIEW head
- **WHEN** a `VIEW` head is offered as a member
- **THEN** no slot of `claims_root` holds it

### Requirement: CSL-013 Contract surface
CSL-013: The SherlockLedger contract SHALL expose `acceptRun`, `acceptCollection`, `rootAccepted` and `runAccepted` with no `openClaim` and no per-member write.

Source: FD-35 (A-10). Hard rule: batched Midnight writes, one root per checkpoint.

#### Scenario: Collection write
- **WHEN** a checkpoint settles
- **THEN** one transaction writes one `claims_root`

### Requirement: CSL-014 Consumers prove a path
CSL-014: When a consumer spends a collection member, the consumer contract SHALL check a 20-hash path to an accepted `claims_root` in its own call and nullify the claim.

Source: FD-35.

#### Scenario: Double spend
- **WHEN** a consumer spends the same claim twice
- **THEN** the second call is refused by the nullifier

### Requirement: CSL-015 Checkpoint interval and K_MAX
CSL-015: The collection service SHALL seal a checkpoint at the accepted 3,600 s epoch with `K_MAX` 2.

Source: FD-36. Label: 3,600 s and 2 are accepted owner values. OQ-3 and OQ-4 are `OPEN (owner)` (design.md); this requirement holds until the owner rules.

#### Scenario: Epoch
- **WHEN** members wait for settlement
- **THEN** one `acceptCollection` is sent per 3,600 s epoch

### Requirement: CSL-016 Rates in runs or claims
CSL-016: The Sherlock performance report SHALL state every batched rate in runs or claims per hour with its transaction count beside it.

Source: FD-38 (A-12). Label: S3 at 4,500 to 7,200 runs per hour per 4x RTX PRO 6000 box is an ESTIMATE of an unbuilt path.

#### Scenario: Compact calls
- **WHEN** a batched rate is reported
- **THEN** it is never in state-writing Compact calls

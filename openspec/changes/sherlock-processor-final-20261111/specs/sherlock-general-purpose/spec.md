## ADDED Requirements

### Requirement: GP-ZK Zero knowledge end to end
Every proof path of GP-001 to GP-004 SHALL be zero knowledge end to end (owner, 2026-11-11: "we need to make this zero knowledge or else its useless"). GP-ZK SHALL have the same level as GP-001 to GP-004. The published view of a call SHALL be the call vector and one blinded wrap proof. The witness, a private program and the run shape SHALL stay on the user's machine. Kit ZERO KNOWLEDGE (ZK-001 to ZK-010) SHALL give the rules.

#### Scenario: Published view of a GP run
- **WHEN** a run of any GP target is submitted to the ledger
- **THEN** the ledger receives only the call vector and one blinded wrap proof (ZK-002), EQ-ZK passes on two wraps of the same statement at frontend M2b, and ZK-L1 (`wrap_view_only_claim`, built at `bdb92973a` under premise A7) states that the view is a function of the call vector alone

#### Scenario: GP evidence without zero knowledge
- **WHEN** a GP acceptance line passes with a seeded blinder, with a helper that sees the plain witness, or with a STARK proof that leaves the user's machine
- **THEN** the line does not count for its GP target, and the target stays open

### Requirement: GP-ZK Lean obligations
GP-ZK SHALL be reported proved only when ZK-L1 and ZK-L2 are proved theorems with axiom reports that hold only `propext`, `Quot.sound` and `Classical.choice`. Premise A7 (`wrapZK`) SHALL stay a named hypothesis.

#### Scenario: Obligation status
- **WHEN** the GP-ZK status is reported
- **THEN** ZK-L1 `wrap_view_only_claim` is built at `bdb92973a` (0 sorry, standard axioms only, premise A7), and ZK-L2 (the `pv` encoding is unique and does not depend on the used count beyond the call vector) is open

### Requirement: CPR-001 No BLS12-381 circuit at k = 22 or above
No BLS12-381 circuit of the co-processor SHALL have k = 22 or above. The wrap SHALL take k = 19, 20 or 21.

#### Scenario: Candidate at k = 22
- **WHEN** a wrap, decider or Fr-terminal layer needs k = 22 to fit its rows
- **THEN** the candidate is refused, and the fallback is a layer change of LW-014, never a larger k

### Requirement: CPR-002 Every proof is a fixed-size shard
Every proof of the co-processor SHALL be a fixed-size shard of a shape from the plan menu. Work SHALL grow by adding shards. No key constant SHALL bound the run length or the Fr work.

#### Scenario: Long run
- **WHEN** a run needs more steps or more Fr work than one shard holds
- **THEN** the planner adds shards and tier E chips, and no circuit grows (SPL-006, FLG-007)

### Requirement: CPR-003 One universal key set
One universal key set SHALL accept every program: the internal-recursive key, the `L` key and the wrap key. Per-program keys SHALL sit under `key_root` inside `prog_d`. A new program SHALL need no new universal key.

#### Scenario: Program with new chips
- **WHEN** a program uses a chip set that no earlier program used
- **THEN** only its per-program keys under `key_root` change, and the internal-recursive, `L` and wrap keys stay byte-equal (frontend M2a)

### Requirement: CPR-004 Ledger 9.1.0.1 verifier unchanged
The co-processor SHALL need no change of the ledger 9.1.0.1 verifier. The wrap SHALL be checked by `VerifierKey::verify` as the ledger runs it today. Ledger 9 refuses a key with the G4 lane gates (M7-0); OD-21 option A would change this rule, and the owner decides (LW-017).

#### Scenario: Ledger change requested
- **WHEN** a kit needs a ledger change to verify its proof
- **THEN** the kit design is refused and the kit is changed instead (MID-001)

### Requirement: CPR-005 GP-001 to GP-004 are never narrowed
No requirement, constant or gate of the co-processor SHALL narrow GP-001 to GP-004 or GP-ZK. A design constant that could narrow them SHALL be an owner decision with a path that has no bound.

#### Scenario: Gate failure
- **WHEN** a derived gate fails or a capacity is full
- **THEN** the GP target stays open and goes to the owner, the spill rule or chunked calls keep the unbounded path, and no narrowed statement is accepted (DG-010)

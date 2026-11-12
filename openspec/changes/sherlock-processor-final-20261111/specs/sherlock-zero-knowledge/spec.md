## ADDED Requirements

### Requirement: ZK-001 Every prover layer runs on the user's machine
Leaves, nodes, `L` and the wrap prover SHALL run on the user's machine. Local devices and GPUs on hosts that the user owns SHALL count as the user's machine (OD-9). STARK proofs, prover checkpoints and the shard plan of a private program SHALL be treated as secrets like the witness.

#### Scenario: Remote STARK prover
- **WHEN** a configuration sends a leaf, node or `L` job to a host that the user does not own
- **THEN** the prover refuses the configuration, and the Quint property "no STARK proof, checkpoint or plan leaves the user's machine" of the placement model holds

### Requirement: ZK-002 The wrap is the only published proof
The published view of a call SHALL be the call vector and one blinded wrap proof. The used log count, the deferred-or-emulated split, the shard plan, the AIR set and heights, and plain `prog_d` SHALL NOT be in that view.

#### Scenario: Two witnesses of one claim
- **WHEN** two wrap witnesses with different logs, used counts, splits and proofs of `L` prove the same call vector
- **THEN** the two ledger proofs are indistinguishable (ZK-L1, `wrap_view_only_claim`, built at `bdb92973a` under premise A7)

### Requirement: ZK-003 Fresh blinders and seeded proofs in test mode only
Every wrap proof SHALL take fresh CSPRNG blinders. A seeded blinder SHALL be accepted only by a build with the test feature. Byte-identity tests of the GPU wrap SHALL use the test-only seed patch, active only under `GW_DET=1`.

#### Scenario: Seeded blinder in a release build
- **WHEN** a build without the test feature receives a seeded blinder
- **THEN** the build refuses it (seeded-proof guard)

#### Scenario: EQ-ZK
- **WHEN** two wraps of one statement are made with fresh blinders at frontend M2b
- **THEN** their bytes differ and the ledger 9 verifier accepts both

### Requirement: ZK-004 Nothing from the transcript of `L` is public
No value derived from the transcript of `L` SHALL become a public input or a constant of the wrap. `X`, `mode`, `j`, `k` and `run_tag` SHALL be public values of `L` only. The wrap SHALL take them as private witness.

#### Scenario: Transcript value in the wrap key
- **WHEN** a wrap key fixes a challenge, a query position or `X` as a constant or a public input
- **THEN** keygen refuses the key (LW-003)

### Requirement: ZK-005 Padding hides the used counts
The log SHALL be padded with `NOP` to its fixed capacity. The call vector SHALL be padded with constrained zeros to `L_TX` (OD-7, OD-15). The count of used entries and the deferred-or-emulated split SHALL NOT be readable from the call vector.

#### Scenario: Short and long logs
- **WHEN** two runs with 3 and 200 used log entries and the same call vector are wrapped
- **THEN** both wraps have the same public inputs and the same proof length, and ZK-L2 (padding uniqueness) is open

### Requirement: ZK-006 Statement digests are salted
The salt of `input_d` SHALL come from the user's CSPRNG: 8 BabyBear elements in the STARK and a full Fr element in public. The hiding commitment in `outcome_d` and the salted `prog_d` of a private program SHALL be inputs of `d2` inside `PoseidonFr(tag, pack(d2))`. The shard plan SHALL bind to the unsalted program digest inside the STARK layers only.

#### Scenario: Unsalted input digest
- **WHEN** `input_d` is computed without a salt, or the salt comes from a seeded generator outside the test mode
- **THEN** the prover refuses the run

### Requirement: ZK-007 The anchor lane carries a salted digest
The anchor lane SHALL carry `C_P` or a lock value `A_j`. Plain `prog_d` SHALL NOT reach the ledger through the anchor (ANC-001).

#### Scenario: Plain program digest in the anchor
- **WHEN** a deploy writes plain `prog_d` into the anchor cell
- **THEN** the frontend refuses the deploy

### Requirement: ZK-008 The run shape stays private
Per-AIR heights and the set of AIRs present SHALL stay inside the user's trust boundary. `n_steps` SHALL NOT be public (FPR-013, LEAD F-12 under the owner's `n_steps` rule): the exact height `n_steps = h_hi` that SEAL forwards (NODE-003) SHALL stay private inside the SET forms. Leaf counts SHALL follow the public padding `N_pub` of the plan. Where a secret branch changes the transcript length of a chunked call, filler chunks SHALL pad the chunk count to the entry bound.

#### Scenario: Secret branch changes the length
- **WHEN** two inputs of one chunked entry take branches of different transcript length
- **THEN** both calls have the same chunk count

### Requirement: ZK-009 No helper sees the plain witness
The design SHALL NOT allow a helper that proves a STARK layer or `L`, a remote unmasked MSM, NTT, quotient or Merkle hash, the full sponge state on a helper, reuse of a blinding slot, or a helper-made slot. No aggregator SHALL receive STARK proofs. One wrap SHALL settle one `ContractCall`. A user batch SHALL be opt-in with one batch digest per `acceptRun` (OD-12).

#### Scenario: Remote MSM
- **WHEN** a configuration sends an unmasked MSM of the wrap to another host
- **THEN** the prover refuses the configuration

### Requirement: ZK-010 Lean obligations of ZERO KNOWLEDGE
The kit SHALL be reported complete only when ZK-L1 and ZK-L2 are proved with axiom reports that hold only `propext`, `Quot.sound` and `Classical.choice`. Premise A7 (`wrapZK`) SHALL stay a named hypothesis.

#### Scenario: Obligation status
- **WHEN** the kit status is reported
- **THEN** ZK-L1 (`wrap_view_only_claim`) is built at `bdb92973a` with premise A7, and ZK-L2 (the encoding of a log in `pv` is unique and does not depend on the used count beyond the call vector) is open

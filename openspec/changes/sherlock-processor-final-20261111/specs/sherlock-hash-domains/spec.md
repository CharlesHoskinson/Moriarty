## ADDED Requirements

### Requirement: HDM-001 Fr-side capacity words and use codes
Every Fr-side Sherlock hash SHALL use the Midnight Poseidon permutation, width 3, rate 2. The initial capacity SHALL be `cap(u, l) = l + u 2^66` for a fixed length `l` and `2^64 + u 2^66` for the transcript. The use codes SHALL be 0 for program operations, 1 for a Merkle leaf of `L`, 2 for a Merkle node of `L`, 3 for the Fr transcript of `L` and 4 for public digests (OD-18).

#### Scenario: Two uses with one capacity
- **WHEN** a Merkle leaf hash of `L` and a public digest of the same length are computed
- **THEN** their initial capacities differ by the use code, and the T-FS vector 9 checks each against Midnight's CPU sponge

### Requirement: HDM-002 Public digest tags
A public digest SHALL be `PoseidonFr(tag, pack(d2))` with `u` = 4. The tags SHALL be `SHK_PROGC` for `C_P`, `SHK_LOCK` for the lock values `A_j`, `SHK_INPUT` for `input_d` and `SHK_OUTCOME` for `outcome_d` (OD-18).

#### Scenario: Digest with another tag
- **WHEN** a `DIGEST` entry names a tag outside the frozen list
- **THEN** the wrap refuses the entry (FLG-005), and `public_digest_binds` (proved at `bdb92973a`) gives the binding of the digest form

### Requirement: HDM-003 Merkle hashes of `L` use the fixed-length sponge
The Merkle leaf and node hashes of `L` (`u` = 1 and 2) SHALL use Midnight's fixed-length sponge in place of the stock leaf hasher and the truncated-permutation compression. A Merkle leaf SHALL pack 8 BabyBear words per Fr element, with checked length and words below `2^31`.

#### Scenario: Packing without a check
- **WHEN** packing omits the length check or the range check
- **THEN** two different inputs pack to one element (`pack_length_check_needed`, `pack_range_check_needed`, proved at `bdb92973a`)

### Requirement: HDM-004 The transcript of `L`
The transcript of `L` SHALL be the stark-backend multi-field overwrite duplex over Fr with Midnight's permutation, stock in every transition, with one change: the initial capacity cell `s_2 = 2^64 + 3 * 2^66` (OD-18). The wrap SHALL implement the duplex on its 19-row permutation layout.

#### Scenario: Stock capacity
- **WHEN** a prover or verifier starts the transcript of `L` with the stock capacity
- **THEN** its T-FS outputs differ from the reference vectors, and the gate check G3 fails

### Requirement: HDM-005 The key schedule fixes the flushes
The key schedule `sched` SHALL fix the count of words between any two flushes. The wrap's verifier of `L` SHALL follow `sched`. It SHALL NOT follow the proof (LW-002).

#### Scenario: Proof-driven flush
- **WHEN** a verifier of `L` flushes where the proof places a commitment and not where `sched` does
- **THEN** the wrap refuses the witness (mutant `wrap_l_transcript_from_proof`)

### Requirement: HDM-006 One OOD sample in the stock WHIR order
Each non-final round of `L` SHALL take one OOD sample in the stock order of WHIR Construction 5.1: observe the round commitment, sample one OOD point, observe its answer, then the query PoW, the query indices and `gamma` as stock, with the answer batched with the query answers under one `gamma` (`q_i + 1` claims) (OD-19).

#### Scenario: Answer observed late
- **WHEN** a prover observes the OOD answer after the query PoW
- **THEN** the T-FS vector 7 differs and the verifier refuses the proof

### Requirement: HDM-007 BabyBear-side tags
Every Sherlock-defined Poseidon2 hash over BabyBear SHALL absorb its tag word first, with the total length in the first capacity word. The tags SHALL be `TAG_EMPTY`, `tagStep`, `tagEnt`, `TAG_NODE`, `DOM_PROG`, `DOM_PROGC`, `DOM_LOCK`, `DOM_KEYTRIPLE`, `DOM_KEYDAG`, `DOM_KEYNODE` and `DOM_LAYOUT`. Each new digest SHALL take a new tag. The leaf, leaf-layer and internal layers SHALL keep the stock OpenVM hashes and challenger (premise A4).

#### Scenario: Shared tag
- **WHEN** a new Sherlock digest is defined
- **THEN** it takes a tag that no other digest uses

### Requirement: HDM-008 Squeeze digits are canonical
Each squeeze SHALL give 5 base-`p` digits with bias at most `2^-100`. The recomposition of the digits SHALL be checked below `r`.

#### Scenario: Aliased squeeze
- **WHEN** the digits of `r` are offered as a squeeze of `0`
- **THEN** the wrap refuses the witness (mutant `wrap_squeeze_noncanon`), and `squeeze_alias` and `squeeze_digits_sound` (proved at `bdb92973a`) give the reason

### Requirement: HDM-009 Transcript vectors T-FS
The nine transcript vectors T-FS of `FD/EXPERIMENTS.md` section 2 SHALL give the same digits and the same final state on the CPU prover, the GPU prover, the stark-backend verifier and the wrap's verifier of `L`. Until they pass, the transcript SHALL be reported PENDING.

#### Scenario: One implementation differs
- **WHEN** one of the four implementations gives other digits on one vector
- **THEN** gate check G3 fails and no `L` key freezes (SQG-008)

### Requirement: HDM-010 Lean obligations of the hash domains and the transcript
The hash domains and the transcript SHALL be reported proved only when BR-L2 names the two Fr sponge algorithms on the one permutation and follows the one-sample order of HDM-006. Premises A2b (`frSpongeInj`) and A5 (`p2Inj`) SHALL stay named hypotheses.

#### Scenario: Proved theorems
- **WHEN** the status is reported
- **THEN** `packed_hash_inj`, `pack_injective`, `pack_length_check_needed`, `pack_range_check_needed`, `squeeze_digits_sound`, `squeeze_alias`, `pv_absorbed_before_challenge`, `late_pv_rejected`, `partial_pv_rejected` and `public_digest_binds` are built at `bdb92973a`

#### Scenario: Open obligations
- **WHEN** the open obligations are listed
- **THEN** the naming of the two Fr sponge algorithms and the one-sample order (both part of BR-L2) are open

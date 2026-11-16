## ADDED Requirements

### Requirement: RSH-001 One proof path with no layer removed
RSH-001: The Sherlock prover SHALL prove every run along the path app shard, leaf, N1, N2, N3 (repeatable fixed point), `L`, `L_Fr` and one BLS12-381 wrap at k = 21 on the `ZkStdLib` arch.

Source: FD-01 (A-01, 6 of 6). Label: k = 21 is the hard-rule bound.

#### Scenario: Layer stack of a multi-shard run
- **WHEN** a run of more than one shard is proved
- **THEN** the proof record names each layer in that order and one wrap proof at k = 21

### Requirement: RSH-002 Fields and hashes below the tail
RSH-002: The Sherlock prover SHALL use BabyBear with its degree-4 extension and Poseidon2 in the app, leaf and node layers.

Source: FD-02 (A-01, A-18).

#### Scenario: Node layer field
- **WHEN** the N1 key is read
- **THEN** its field is BabyBear, its challenge field the degree-4 extension and its Merkle hash Poseidon2

### Requirement: RSH-003 Fields and hashes in the tail
RSH-003: The Sherlock prover SHALL use BabyBear arithmetic with Midnight Poseidon over Fr for the commitments and transcript of `L`, and BLS12-381 Fr with Midnight Poseidon for `L_Fr`.

Source: FD-02 (A-18, C-19 (a) 5 of 6: no replacement tail hash).

#### Scenario: Tail hash unchanged
- **WHEN** a replacement tail hash is proposed without a named candidate under 19.6 wrap rows per permutation (DERIVED) at equal security
- **THEN** the proposal is not an experiment of this design

### Requirement: RSH-004 Binary node arity
RSH-004: The node layer SHALL have arity 2.

Source: FD-03 (A-05, C-11 (b) 6 of 6). Label: K-10, 2,046 against 1,364 child verifications at N = 1,024, DERIVED.

#### Scenario: Arity 4 not admitted
- **WHEN** E-ARITY has not shown `C4/3 < C2` on the complete tree with every bound holding
- **THEN** every node key has arity 2

### Requirement: RSH-005 Arity 4 after E-ARITY
RSH-005: Where E-ARITY has passed, the node layer SHALL have arity 4 with the N3 fixed point re-derived.

Source: FD-03, C-11.

#### Scenario: E-ARITY passes
- **WHEN** arity 4 wins on the complete tree and every bound holds
- **THEN** N1 to N3 and every key above them are regenerated in the final rekey

### Requirement: RSH-006 Path C for a one-leaf R_SL run
RSH-006: When a run has exactly one R_SL leaf, the Sherlock prover SHALL take path C (leaf, then `L`) with the R_SL leaf key admitted as a path C leaf of `key_root`.

Source: FD-04 (A-16). Gate: E-PATHC (25 of 25 route tests; saves 0.5 s or more, ESTIMATE bar; `L` heights do not rise).

#### Scenario: One-leaf call
- **WHEN** a one-leaf R_SL call is proved on the final keys
- **THEN** no N1, N2 or N3 proof is made and `L` verifies the leaf

#### Scenario: Route refusal
- **WHEN** a leaf whose key is not a path C leaf of `key_root` is offered to `L`
- **THEN** `L` refuses it (T7c route test)

### Requirement: RSH-007 Tail constant in the member count
RSH-007: The tail key set SHALL have one shape for every member count N of a collection.

Source: FD-05 (A-09). The tail key set is the `L`, `L_Fr` and wrap keys.

#### Scenario: Shape at N = 1 and N = 1,024
- **WHEN** E-COLL compares the `L` and `L_Fr` shapes at N = 1 and N = 1,024
- **THEN** the shapes are equal

### Requirement: RSH-008 No member loop in L
RSH-008: The `L` AIR SHALL contain no loop over collection members.

Source: FD-05 (A-09). The tail key set is the `L`, `L_Fr` and wrap keys.

#### Scenario: Claims-tree work below L
- **WHEN** a collection is proved
- **THEN** the claims-tree hashing is in the collector run below `L`

### Requirement: RSH-009 MERGE-WORK column bound
RSH-009: The MERGE-WORK kernel SHALL stay a specialized kernel of at most 5,400 columns with at most 1,000 constraints per AIR.

Source: FD-06 (A-14, K-6). Label: `Kind::Work` 4,988 columns MEASURED; 5,121 with STACKS and the value-set CORE, DERIVED on one ESTIMATE; split CORE at 999 of 1,000 constraints, MEASURED.

#### Scenario: Completed kernel measured
- **WHEN** STACKS, VALUE DIGEST, memory and the work rules are in the kernel
- **THEN** E-SHAPE prints its columns and constraints before the final keys

### Requirement: RSH-010 Per-chunk identity on the CORE split
RSH-010: The CORE-to-FRAME and rule-family transfers SHALL bind row or slot, rule family and chunk identity on every chunk.

Source: FD-07 (C-20 (a), 6 of 6). Evidence: `send_frame` sends `(chunk_index, chunk_words)` with the row identifier in the first chunk only (`sherlock-leaf/src/core.rs:1074-1080`).

#### Scenario: Swapped chunks
- **WHEN** a forged witness swaps non-initial CORE chunks across two rows (E-CHUNK)
- **THEN** the constraints and the stock verifier refuse it, and the verdict is recorded as CONFIRMED, NOT REPRODUCED or GUARD HOLDS with a regression test

### Requirement: RSH-011 Full-state transfer first
RSH-011: The CORE split SHALL start with full-state transfer, and admit a projection only on a measured whole-proof win.

Source: FD-07 (C-20).

#### Scenario: Projection proposed
- **WHEN** a projection of the CORE state is proposed without a paired whole-proof measurement
- **THEN** the split keeps full-state transfer

### Requirement: RSH-012 Program-independent bucket keys
RSH-012: The R_SL bucket keys SHALL be independent of the program image.

Source: FD-08 (A-25).

#### Scenario: Two programs in one bucket
- **WHEN** two programs of one R_SL bucket are proved
- **THEN** no key is generated on the call path and both use the same bucket keys

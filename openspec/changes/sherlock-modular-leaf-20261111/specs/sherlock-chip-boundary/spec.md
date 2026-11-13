## ADDED Requirements

### Requirement: BND-001 Complete keyed boundary schema
The boundary SHALL serialize every future-readable CORE register, the fold cells, the FRAMES and TASKS heads, `input_d`, `deps_d` and the arena `(len, root, tail_d)` in a fixed keyed order with a schema id. Scratch SHALL be excluded only because a step is independent of its incoming scratch (CORE-007).

#### Scenario: Word omitted or reordered
- **WHEN** a word, the tail, the length, a stack head, the memory token or the schema id is omitted, reordered or unbound between leaves
- **THEN** the correct AIR refuses the witness (mutants `boundary_word_omitted`, `boundary_word_reordered`, `boundary_tail_omitted`, `boundary_len_omitted`, `stack_head_omitted`, `memory_token_unbound`, `schema_id_unbound`)

#### Scenario: Scratch in the digest
- **WHEN** a scratch lane is hashed into `body_d`, or incoming scratch is set to any value
- **THEN** the correct AIR refuses the first witness (mutant `scratch_in_digest`), and the second changes neither `pre_d`, `post_d` nor acceptance (positive `core_scratch_incoming_free`)

#### Scenario: Lean serialization obligation
- **WHEN** the kit is offered for completion
- **THEN** O28 (`BoundaryCommitment.lean`, the serialization is injective and complete) is proved, and today it is open

#### Scenario: Field waits on T5
- **WHEN** a freeze or a kit tick needs the state schema body words
- **THEN** it waits on `research/sherlock-tested-design-20261113/T5-SCHEMAS.md` (DRAFT for owner sign-off, O12) until row I-15 of `FD/ARCHITECTURE.md` section 1.12 reads FROZEN (`design.md` section 13), and no draft value is read as settled

### Requirement: BND-002 Poseidon header and body commitment is the default
`pre_d` and `post_d` SHALL be the header digest `P2(SHK_SD2, outcome_tag, outcome_payload, input_d, deps_d, view_root, body_d)` with `body_d` a digest of the other schema words, in one pass with no challenge (AD-19, FPR-012). They SHALL be published in the seven leaf lanes. Leaf lane 1 SHALL equal the `prog_d` body word.

#### Scenario: Two-leaf stitch
- **WHEN** two leaves stitch
- **THEN** `post_d` equals the next `pre_d` if and only if all words match (positive `ok_boundary_poseidon_two_leaf_stitch`), and O29 (`BoundaryCommitment.lean`, `digest_inj` for the physical state) is open

#### Scenario: Header word moved or unlinked
- **WHEN** a header word is hashed only inside the body, `body_d` is not the body sponge output, the header uses the landed `SHK_SD` domain, lane 1 differs from the body word, or the post digest uses a pre cell
- **THEN** the correct AIR refuses the witness (mutants `header_word_moved_to_body`, `header_body_d_unlinked`, `header_domain_landed_sd`, `prog_d_lane_differs_from_body`, `post_uses_pre_cell`)

#### Scenario: Tag collision or substituted commitment
- **WHEN** the boundary domain tag equals the memory domain tag, or the boundary commitment is substituted by another value
- **THEN** the correct AIR refuses the witness (mutants `boundary_tag_equals_memory_tag`, `boundary_commitment_substituted`)

### Requirement: BND-003 Typed header opening for the seal
`open_state_header` SHALL return the five header words and `body_d` of the state that an authenticated `post_d` commits, with one header sponge (FPR-012). The seal modes of NODE INTEGRATION SHALL be its only users. Kit L AND WRAP SHALL open a SET head only. Its fit inside NODE INTEGRATION SHALL be measured by X10 before the NODE spec freezes. If X10 fails, the fallback leaf statement SHALL be a new versioned statement (AD-13, OWNER).

#### Scenario: Leaf against library sponge
- **WHEN** the leaf header digest and the library sponge are computed on the same words
- **THEN** they are equal (positive differential `boundary_header_leaf_vs_std`)

#### Scenario: Opened word forged
- **WHEN** an opened word differs from the committed word, or the terminal opening is unbound
- **THEN** the correct AIR refuses the witness (mutants `node_seal_header_word_forged`, `terminal_opening_unbound`), and O48 (`BoundaryCommitment.lean`, header opening) is open

### Requirement: BND-004 Partition independence of boundary values
At every cut shared by two partitions that the leaf AIRs accept, the Poseidon boundary values and the public statement SHALL be equal (SRC-023).

#### Scenario: Three cuts
- **WHEN** a trace is cut as full leaves, single steps and mixed leaves
- **THEN** the digests at common cuts and the public statement agree (positive `ok_boundary_partition_three_cuts`), and the Quint model `boundaryCommitment.qnt` keeps serialization apart from commitment

#### Scenario: Lean scope
- **WHEN** the partition evidence is cited
- **THEN** `partition_independent_flush`, `cuts_agree` (P9, P14) and `result_unique_tail` (P10) are proved in C3 only for leaves of at most 8 appends, and O14 is open for the capacity of the window

### Requirement: BND-005 Horner boundary only with Profile B
A Horner boundary SHALL be offered only with the ZmLogUp backend, not before M4. Its words SHALL be committed before `beta` is derived.

#### Scenario: Horner words or beta free
- **WHEN** boundary words are not committed, `beta` is free or mismatched, or two words change with the fingerprint kept
- **THEN** the test refuses the witness (mutants `horner_words_uncommitted`, `horner_beta_free`, `horner_beta_mismatch`, `horner_two_word_compensation`), and O30 (Horner soundness) is open and not scheduled

### Requirement: BND-006 The BOUNDARY COMMITMENT AIRs meet the STARK kit gate
The boundary AIRs SHALL meet the STARK kit gate SG-1 to SG-6 of `design.md` section 9 inside the app set (at most 5,000 constraints and 1,000 interactions per AIR, 100 AIRs, 30,000 columns), with every AIR required and a fixed height in its menu shape.

#### Scenario: AIR omitted from the proof
- **WHEN** a proof omits one of the boundary AIRs
- **THEN** N1, N3 and `L` refuse it (mutant `boundary_air_omitted`), and the kit does not pass SG-1 before row T3 passes

#### Scenario: Positive and negative proofs on the stock verifier
- **WHEN** the positive `ok_boundary_poseidon_two_leaf_stitch` and the mutant `boundary_word_omitted` are proved with the stark-backend prover
- **THEN** the stock verifier accepts the first, refuses the second and accepts the second under its weakened control AIR; a prover panic or tracegen error earns no rejection credit

#### Scenario: Shape, budget and D3
- **WHEN** the boundary AIRs are keyed in a menu shape
- **THEN** each AIR has the fixed height of the shape, keygen refuses a shape outside the bounds above, D3 at the measured shape prints every term at 100 bits or more and every grindable term at 101 or more (SG-5), and the receipt records seconds per proved step (SG-6)

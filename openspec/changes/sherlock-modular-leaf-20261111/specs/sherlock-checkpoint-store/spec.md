## ADDED Requirements

### Requirement: CKPT-001 A checkpoint holds every witness resource of the suffix in a versioned codec
`CheckpointV1` in the codec `sherlock-ckpt/1` SHALL hold the key-set manifest hash, the identity `(prog_d, input_d, h, post_d)` with `1 <= h < 2^64`, every header and body word, the memory append log, both stack lists, the program image and input preimage, the tier 1 carry stack and the SRC-019 prefix hash. The codec SHALL be canonical and length prefixed.

#### Scenario: Resume after a stop
- **WHEN** a run with a non-empty tail and nested stacks is stopped and resumed
- **THEN** later pops and reads succeed (host tests `ckpt_resume_nonempty_tail`, `ckpt_resume_nested_stacks`, `ckpt_roundtrip_allocating_run`, `ckpt_carry_stack_of_several_segs`)

#### Scenario: Canonical codec
- **WHEN** a record is encoded twice, or decoded with an unknown version or trailing bytes
- **THEN** each record has one byte string (host test `ckpt_codec_canonical`), and the decoder refuses the other two (host refusals `ckpt_unknown_version`, `ckpt_trailing_bytes`)

### Requirement: CKPT-002 Restore validates the whole record against the committed state
Restore SHALL check, in order: the codec version and manifest, every prefix SEG proof and the interval cover of `[0, h)`, the memory log against `(len, root, tail_d)`, each stack list against its head and count, `body_d` and `state_d` against `post_d`, and the program and input preimages. Any failure SHALL stop the restore before proving.

#### Scenario: Tampered record
- **WHEN** any record, stack preimage or register is changed, also one that no later leaf reads, or a section is missing, the manifest differs or a prefix proof is invalid
- **THEN** restore refuses with a host integrity failure (host refusals `ckpt_tampered_unused_cell`, `ckpt_tampered_tail_record`, `ckpt_stack_preimage_changed`, `ckpt_register_changed`, `ckpt_missing_stack_section`, `ckpt_wrong_manifest`, `ckpt_prefix_proof_invalid`)

### Requirement: CKPT-003 Host integrity failures are kept apart and soundness does not depend on the store
The store SHALL report a host integrity failure apart from a constraint failure. The record digest SHALL use the domain `SHK_CKPT`, disjoint from `SHK_RUN` and the SET domains, so a checkpoint is never a claim. Writing and restoring SHALL execute no machine event.

#### Scenario: Validation off
- **WHEN** restore validation is switched off and a read cell is changed
- **THEN** the suffix leaf is unsatisfiable (constraint check `cont_tampered_read_cell_unvalidated` of CONTINUATION), and the Quint invariant `CorruptStoreNeverYieldsAcceptedWrongClaim` of `continuation.qnt` holds

#### Scenario: No Lean obligation of its own
- **WHEN** the kit is offered for completion
- **THEN** its evidence is the host tests and refusals above, and restore correctness rests on O28 and O29 (open) applied by the host

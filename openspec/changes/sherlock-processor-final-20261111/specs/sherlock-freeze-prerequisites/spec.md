## ADDED Requirements

### Requirement: FPR-001 Every lane has one word type over BabyBear
Every lane of a bus message, a port message, a public value and a digest preimage SHALL have one type of `FD/ARCHITECTURE.md` section 1.11, FP-1: `bit`, `tag(w)` with `w <= 16` and `u16` in one word; `u32` in 2, `u64` in 4, `addr` in `ceil((TOP + 3) / 16)` and `u128` in 8 little-endian 16-bit limbs; `digest` in 8 words; `fr` in 9 30-bit limbs with `Canon`. The writer SHALL range-check every limb. No word SHALL hold two typed fields (LEAD F-1).

#### Scenario: Limb vectors
- **WHEN** vector `bus_enc_vectors` encodes `u64` `2^64 - 1`, `u64` `0x0001000200030004`, `addr` `2^65 + 1` at TOP = 63 and `u128` `2^127`
- **THEN** the words are `[65535, 65535, 65535, 65535]`, `[4, 3, 2, 1]`, `[1, 0, 0, 0, 2]` and `[0, 0, 0, 0, 0, 0, 0, 32768]`

#### Scenario: Limb out of range
- **WHEN** a limb equals `2^16`, or the top `addr` limb equals 4 at TOP = 63
- **THEN** the constraint checker refuses the trace (mutants `bus_limb_over_range`, `bus_addr_top_limb_over`)

#### Scenario: Packed key alias
- **WHEN** the control AIR packs `8 addr + kind` into one word
- **THEN** two `(addr, kind)` pairs give one key word (control `bus_key_packed_alias`), and the FP-1 encoding gives two messages

### Requirement: FPR-002 One LogUp bus index for each chip id
Chip id `c` SHALL be the stark-backend bus index `BUS_BASE + c`, with `BUS_BASE` above every bus index of a stock OpenVM chip of the chip set. Ids 1 to 6 SHALL be ALU, MEM, HASH, ROM, RANGE and DEC with the message layouts of `FD/ARCHITECTURE.md` section 1.11, FP-1. Ids 7 to 15 SHALL have no provider. A disabled request SHALL have count 0. The L-6 tag SHALL be the bus index. Every DEC slot SHALL query with count 1, a disabled slot on the idle row `(0, 0)` (LEAD F-2).

#### Scenario: Two layouts on one index
- **WHEN** a chip set puts two message layouts on one bus index
- **THEN** keygen refuses the chip set

#### Scenario: Reserved bus
- **WHEN** a request is sent on a bus index of ids 7 to 15
- **THEN** the proof is refused, because no provider row exists (BUS-001)

### Requirement: FPR-003 Ports are shared columns or permutation buses
A port whose producer and consumer are columns of one AIR SHALL be shared columns of one row. A port between two AIRs SHALL be a `PermutationCheckBus` of index `BUS_BASE + 16 + j`, with `j` the position of the port in the ports manifest. The producer SHALL send `(seq, cells)` with count `act`, and the consumer SHALL receive the same message with count `act`, with `seq: u32` the leaf-local event index (LEAD F-2).

#### Scenario: Second consumer
- **WHEN** a second `port_recv` consumes one producer instance
- **THEN** the LogUp sum is not zero and the proof is refused (mutant `bus_port_received_twice`)

### Requirement: FPR-004 LogUp bounds and AIR interfaces replace BUS-007 and the halo2 signatures
Each bus of ids 1 to 6 SHALL be a `LookupBus`: a request with a boolean count, a provider with a witness multiplicity. A message with its bus index SHALL have at most 128 words. For each bus, the sum of `count_weight` times height SHALL be below `p`, and the sum of interactions times height SHALL be below `p`. Each kit SHALL implement `KitAir` and `KitTrace` of FP-1 and SHALL use only `request`, `provide`, `port_send` and `port_recv` across AIRs.

#### Scenario: Message too long
- **WHEN** an interaction has 128 message words and its bus index
- **THEN** keygen refuses it (refusal `bus_message_too_long`)

#### Scenario: Height over the field
- **WHEN** a proof has trace heights that break the per-bus bound
- **THEN** the verifier refuses the proof (refusal `bus_height_constraint_over_p`)

### Requirement: FPR-005 The node statement has 74 public words and one log rule
Every node proof SHALL expose the 74 words of FP-2: `stmt_tag`, a 32-word payload, `key_root`, `reg_root`, `shape_d`, `log_d`, `log_n` and `keys_d`. A log with `log_n = 0` SHALL be `(D_EMPTY, 0)`. The output log SHALL be `merge(merge(c_0, c_1), w)`: an empty operand is neutral, two non-empty logs give `(P2(TAG_NODE, d_a, d_b), n_a + n_b)`, and the output `n` is below `2^16`. A disabled slot SHALL contribute `(D_EMPTY, 0)`, and administrative modes no entry (LEAD F-3).

#### Scenario: Unary pass-through
- **WHEN** one child has the log `(d, n)` and the other slot is disabled
- **THEN** the output log is `(d, n)` (positive `pos_node_log_unary_passthrough`)

#### Scenario: Forged merge
- **WHEN** a child log is dropped, the slots are swapped, the count reaches `2^16` or an empty child changes `d`
- **THEN** the proof is refused (mutants `node_log_child_dropped`, `node_log_slots_swapped`, `node_log_count_over`, `node_log_empty_not_neutral`)

### Requirement: FPR-006 Key routing and the DISCHARGE mode
For a LEAF child, the node SHALL open `prog_d` to `key_root` and SHALL prove the child's key leaf `P2(DOM_KEYTRIPLE, set_id, c_app, c_leaf, c_ifl)` in it at depth 16. Every enabled NODE child SHALL have the parent's `keys_d`; a SEG child, the parent's `key_root` (LEAD F-5). A DISCHARGE node SHALL require the LG statement of its slot-1 LEAF to equal the log of slot 0, and SHALL output the slot-0 statement with the log `(D_EMPTY, 0)` (LEAD F-4).

#### Scenario: Key from another program
- **WHEN** a LEAF child's key leaf is outside the `key_root` of its `prog_d`, a NODE child has another `keys_d`, or `key_root` comes from another program
- **THEN** the proof is refused (mutants `node_leaf_key_outside_root`, `node_keys_d_other`, `node_key_root_other_prog`)

#### Scenario: Discharge of another log
- **WHEN** the discharge leaf proves a log other than the log of slot 0
- **THEN** the proof is refused (mutant `node_discharge_other_log`), and `pos_node_discharge` passes on the equal log

#### Scenario: Discharge child
- **WHEN** the slot-1 LEAF of a DISCHARGE node is of the discharge shape
- **THEN** the node reads `(dis_d, dis_n)` from its 9 public words, finds its key leaf in the `key_root` of slot 0, permits `LEAF -> {SEG, LG}`, and keeps the `shape_d` of slot 0

### Requirement: FPR-007 `L` opens the sealed SET head
`L` SHALL open the sealed SET head of a general run with one Poseidon2 hash over BabyBear and SHALL recompute `claim_d` from its witnessed preimage (LW-008, LEAD F-6). The wrap SHALL compute no BabyBear Poseidon2 for the head.

#### Scenario: Head forged
- **WHEN** the head preimage differs from the run statement
- **THEN** the trace of `L` is refused (mutant `decider_set_head_forged`)

### Requirement: FPR-008 Public values of `L` and the discriminator
`L` SHALL expose its public values in the order of the FP-3 index table: the root statement, the anchor statement `(mode, j, k, e_in, e_out)`, `run_tag`, `layout_d`, `X`, a lane map of 4 `(lane, entry)` pairs, `pv` and the call vector of `2 + L_TX` `fr` lanes. The wrap SHALL require each mapped lane to equal the output of its `DIGEST` entry. The entry label bound by lane 0 SHALL discriminate the run statement from the collection statement (LEAD F-8).

#### Scenario: Statement under the other entry
- **WHEN** a run statement is offered to `acceptCollection`
- **THEN** lane 0 does not equal the bind of `acceptCollection`, and the call is refused (CF-008)

### Requirement: FPR-009 FP-3 open facts stay open
The freeze proposal SHALL NOT freeze the public statement codec while these facts are missing, and SHALL name them: the carrier of the effect region and `comm` from the terminal state of a run of more than one shard to the call vector of `L`; the call-vector lanes of the run statement and of the collection statement, which follow from the compiled LEDGER CONTRACT; and the contract-side form of a collection opening under `sherlock-value/2`, with the `openClaim` key and its public `n_steps`.

#### Scenario: Freeze proposal written
- **WHEN** the interface freeze proposal is written at loop step S11
- **THEN** FP-3 is marked open with these three facts, and FP-1, FP-2 and FP-4 to FP-7 are marked closed

### Requirement: FPR-010 Public digests of the run
The public digests SHALL be `C_P = PoseidonFr_4(SHK_PROGC, pack(P2(DOM_PROGC, r_P, prog_d)))`, `input_pub = PoseidonFr_4(SHK_INPUT, pack(input_d))`, `outcome_pub = PoseidonFr_4(SHK_OUTCOME, pack(P2(DOM_OUTHIDE, input_d, outcome_d)))` and `claim_pub = transientHash([C_P, input_pub, outcome_pub])`. The salt SHALL be 8 words from the user's CSPRNG in `input_d`, fixed for the run at INIT (LEAD F-7, F-10).

#### Scenario: Unsalted outcome
- **WHEN** the control computes `outcome_pub` over `outcome_d` alone
- **THEN** two runs with one outcome give one `outcome_pub` (control `outcome_pub_unsalted`), and the FP-4 form gives two

#### Scenario: Salt changed inside a run
- **WHEN** a leaf of the run uses another salt
- **THEN** the proof is refused (mutant `input_salt_changed_midrun`)

### Requirement: FPR-011 One internal digest table
Every internal digest SHALL be `P2(tag, words)` with the tag code of FP-4, the tag word first and the length `1 + l` in the capacity. `image_d`, `prog_d`, `vd`, `input_d`, `outcome_d`, `claim_d`, `fclaim_d`, `adm_d`, `vw_d`, the SET forms, `deps_d`, `state_d` and `body_d` SHALL have the preimage order of the FP-4 table. Each constant SHALL be the hash of its tag alone. VALUE DIGEST, HASH and INIT SHALL compute these forms in place of their `sherlock-value/1` forms (LEAD F-9).

#### Scenario: Differential vectors
- **WHEN** `digest_table_vectors` hashes the preimage word list of each table row
- **THEN** the Rust `P2`, the stark-sdk Poseidon2 and the AIR give equal outputs on the CPU and the GPU

#### Scenario: Tag or constant misuse
- **WHEN** `claim_d` is computed with `SHK_CLAIM`, or `NO_INPUT_LINK` is offered as a claim
- **THEN** the proof is refused (mutants `digest_tag_swapped`, `digest_constant_as_claim`)

### Requirement: FPR-012 `view_root` is a header word of the state commitment
`state_d` SHALL be `P2(SHK_SD2, outcome_tag, outcome_payload, input_d, deps_d, view_root, body_d)`. `view_root` SHALL be `VIEW_NONE` under forms (a) and (c) of AD-17 and for an absent input. INIT SHALL set it, and no rule SHALL write it. SEAL and SEAL-WITH-DEPS SHALL compare the opened `view_root` with the `view_root` of the `VIEW` claim (LEAD F-11; AD-17 stays OWNER).

#### Scenario: Forged view root
- **WHEN** the seal opens a `view_root` that is not the committed header word, or a rule writes `view_root`
- **THEN** the proof is refused (mutants `seal_view_root_forged`, `view_root_written_by_rule`)

### Requirement: FPR-013 `n_steps` is not public
`n_steps` SHALL NOT be in the run statement, the call vector, `acceptRun` or the `runs` map. It SHALL stay private as `h_hi` of the sealed segment inside the SET forms `LEAF` and `DEP`. For a collection this SHALL hold only when FPR-009 fixes the contract-side opening (LEAD F-12; the `n_steps` rule and the amendments of SRC-006, SRC-007 and LDG-001 are owner items).

#### Scenario: Public n_steps
- **WHEN** a wrap key puts `n_steps` in the call vector
- **THEN** keygen refuses the key (refusal `decider_nsteps_public`), and `pos_run_nsteps_private` reads one private `n_steps` under three partitions

### Requirement: FPR-014 Chunk claim address and `ep_hash` under placement V
`bind_label` SHALL require `offset = |domain| + 32`, SHALL take `addr_c = m[|domain| .. |domain| + 32)`, and SHALL compute `ep_hash = SHA256(EP_OPEN || lab)` on the `sha2` chip, with `EP_OPEN` the bytes `"midnight:entry-point"` and 12 zero bytes and `lab` the label bytes without the SCALE length prefix. A chunk `j < k` SHALL write `(addr_c, ep_hash, comm_(j+1))` into the effect words of its `claimContractCall` (LEAD F-13).

#### Scenario: Ledger equality
- **WHEN** `ep_hash_vs_ledger` computes `ep_hash` for `acceptRun`, `acceptRunB` and labels of 1, 55, 56 and 64 bytes
- **THEN** each equals `EntryPointBuf::ep_hash` of the ledger

#### Scenario: Free address or other label
- **WHEN** a chunk claims another address or another label
- **THEN** the proof is refused (mutants `chunk_claim_addr_free`, `chunk_ep_hash_other_label`)

### Requirement: FPR-015 E-1 window under a variable-length preimage
Under placement V, `m` SHALL be a tape byte array of the per-entry length `M_B(e)` from `abi.json`, with the true length `len_m: u32` on the tape and zero bytes after it. The window offsets SHALL be absolute. `bind_label` SHALL run `ceil((M_B(e) + 9) / 64)` compressions and SHALL select the chaining value after block `ceil((len_m + 9) / 64)` as the digest.

#### Scenario: Length selection
- **WHEN** `bind_len_select` runs `len_m` in {119, 131, 183, 247, 503, 1,015, 2,039} at `M_B` = 2,039
- **THEN** the digest is selected after block 2, 3, 3, 4, 8, 16 and 32 and equals `persistent_hash`, and a wrong block is refused (mutant `bind_len_select_wrong_block`)

### Requirement: FPR-016 Lean obligations of the freeze
The freeze interfaces SHALL be reported proved only when `bus_enc_inj`, `node_log_merge`, `node_key_route`, `digest_table_inj`, `header_open_sound`, `run_view_no_nsteps` and `bind_label_sound` are proved with an axiom report that holds only `propext`, `Quot.sound` and `Classical.choice`, with premises A4 and A5 as hypotheses.

#### Scenario: Obligation status
- **WHEN** the status is reported
- **THEN** each of the seven lemmas is open, and `horner_inj`, `digest_inj`, `onion_snoc`, `discharge_sound`, `packed_hash_inj`, `public_digest_binds` and `wrap_view_only_claim`, on which they build, are proved at `bdb92973a`

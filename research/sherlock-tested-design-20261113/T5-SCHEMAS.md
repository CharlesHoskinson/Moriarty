# T5 leaf schemas: drafts for owner sign-off

Lane T5 of `research/sherlock-tested-design-20261113/LOOP.md`, drafting part. Owner ruling O12: the lead drafts first, the owner signs. Status: SIGNED by the owner on 2026-11-13 08:49 ("Sign all recommendations", O12; O3-2 stays open until T6). The signed layouts are written into ARCHITECTURE section 1.11, FP-9, on branch `tested`. Each section is one draft that, once signed, unblocks the FREEZE-PENDING rows of ARCHITECTURE 1.12 that it names. No row becomes FROZEN by this file: a row reaches FROZEN only after the owner signs the draft and its closure test passes.

Conventions:

- `ARCH` is `~/Projects/Moriarty-wt/fp8/research/sherlock-final-design-20261111/ARCHITECTURE.md` (the fp8 worktree). `O/` is `~/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/`. `ML/` is `~/Projects/Moriarty/openspec/changes/sherlock-modular-leaf-20261111/`. `DA` is `~/Projects/Moriarty/research/sherlock-processor-final-20261111/DECISIONS-ASK.md`. `X15` is `~/Projects/Moriarty-wt/x-exp/research/sherlock-x15-20261112/RESULTS.md`. `ENC` is `~/Projects/Moriarty-wt/enc-fp3/research/sherlock-enc-fp3-20261112/RESULTS.md`.
- Code paths are relative to `~/Projects/Moriarty` at main `4ea6fa29d`. `hm/` is `crates/mori-core/src/sherlock/helper_machine/`. `chip/` is `crates/mori-chip/src/`.
- Word types are the FP-1 types of ARCH 1.11 (I-1, FROZEN): `bit` 1 word, `tag(w)` 1, `u16` 1, `u32` 2, `u64` 4, `addr` 5 (TOP = 63, I-2, FROZEN), `u128` 8, `digest` 8, `fr` 9. Every layout below is the BabyBear form: one lane group per typed field, no word holds two parts. The halo2 forms of the kit sections (`8 addr + kind`, `tbl 2^D_row + row`, five-cell tuples over `Fq`) are prototype text and are not the frozen form (ARCH 1.11, FP-1).
- Labels: MEASURED is read from a run record. DERIVED is computed by a stated rule from MEASURED numbers or from source. ESTIMATE is a derived number with named inputs that are not measured. The X15 words map as follows: STATIC is DERIVED (a mechanical bound from a source scan), OBSERVED is MEASURED (a corpus maximum in a run record; it is a lower bound of the true maximum), READ is a code citation and carries no label.
- New hash tag codes take the next free BabyBear codes in the order of this file, after the HASH registry codes 33 to 40 and `DOM_PLAN` = 41 (owner-confirmed pick P-5, `DA`, "ENC picks P-1 to P-6 confirmed"; ENC section 4). Each new code is a LEAD pick of this draft.
- "Closure test" is the named test whose pass, after the owner signs, lets the row read FROZEN. Positives must pass. Refused mutants follow the ARCH mutant protocol: a malicious witness that the correct AIR refuses at a constraint, lookup or permutation boundary, with a weakened control that accepts it.

## Owner sign-off table

| # | Item (rows unblocked) | Recommendation | Open choices for the owner |
|---|---|---|---|
| 1 | AD-11 record schema (I-5, I-9, I-15 records; kits MEM, MEMORY BACKEND, VALUE DIGEST, CORE) | Every reference record is one physical cell (`e = 1`): `kind: tag(3)` plus three typed slots. `W_rec = 28` (DATA), and the MEM message is 34 words. `vd` sits in the DATA record (V-7 (a)). Body schema: 461 words for the reference image, 476 recommended (ESTIMATE; section 1.6) | C1-1, the reading of "three-word" (the literal BabyBear reading needs TOP >= 65, against AD-1); O1-1, the input reference variant `Input(i)`; O-1 and O-2, the widths of the state counters |
| 2 | AD-12 with X15 (I-6, `AuthRom`; kit PAGE/ROM) | `D_tbl = 5` (22 tables). One physical row per logical row, so no sub-row bits. `W_rom = 33`, and the ROM message is 38 words (DERIVED). `D_row = 64` (`row: u64`), `D_rom = 66`. `K_rom = 15` is already frozen | O2-1, `D_row`: 32 (a new language bound of `2^32` rows per table, which is a GP-1 amendment) or 64 (recommended; `entry_a`, every `ridx` lane and every program slice length become `u64`); O2-2, the permutations per tree node |
| 3 | AD-9 guard list and control word (I-7; kit DEC) | The 120 rules in reference priority order, each with its guard and source line (section 3.4, script-checked). The class is the priority ordinal, `tag(7)`. Lanes unpacked; `rule_lane = class` | O3-1, `rule_lane` (class, or the existing 128-bit SHA-256 lane); O3-2, `microplan_id`, `write_set_id` and `flags` have no source table, so these lanes freeze with the family `RuleContract`s in T6; O3-3, the MatchArm guard place; O3-4, residue states (a Quint task) |
| 4 | `D_view` and the view form (I-10; AD-17) | A hash-consed view (one record per distinct `vd`). One tree with a kind bit, so `D_view = 67` with `addr` indices. `AuthIn` is 39 words (DERIVED). Lazy view allowed, with a host canonical rule | O4-1, `D_view` 67 (recommended), 68 (needs an I-1 edit) or 65 (narrows GP-2); the view form follows the I7 answer (whole-input type check forces the full view); X11 has not run |
| 5 | `ty_d` normal form (I-14; AD-14) | Carbon's `ty_d` with `decl_d` over the strongly connected group of declarations. Names enter the hash (the language is nominal). Two-input `P2` chains checked by ADMIT-WORK in bounded units. Tags 45 to 52 | O5-1, the order of `types` rows; O5-2, the ADMIT-WORK carry fields in `adm_state_d`; O5-3, polymorphic recursion; O5-4, an `Fr` builtin type |
| 6 | SRC-021 amendment (I-15 Init part; kit INIT; AD-18) | Replacement texts for `O/requirements.md:46`, the spec requirement and the design Init table (42 fields, one arena, input by digest, salted `input_d`). `openspec validate` passes on a scratch copy (MEASURED, 6.6) | O6-1, SRC-022 to the BabyBear `sherlock-value/2` (recommended in the same change); O6-2, the "dependency claims" sentence stays, with AD-20; O6-3, counters under one arena |

New LEAD tag codes in this file: 42 to 44 (view), 45 to 57 (types and names), all after `DOM_PLAN` = 41. No row of ARCH 1.12 changes status from this file. Each row reads FROZEN only after the owner signs its draft and the closure test named in its section passes. I-7 then stays FREEZE-PENDING on O3-2 for the plan lanes.

## 1. AD-11 record schema (I-5, I-9, I-15; kits MEM window, MEMORY BACKEND, PAGE/ROM, VALUE DIGEST, CORE)

Owner ruling: "AD-11: three-word records at consecutive addresses" (`DA` line 65). ARCH AD-11 (`ARCH:3448`). AD-1 ruling: "(b) TOP = 63 (AD-11 makes one logical record one physical cell, `e = 1`)" (`ARCH:3438`).

### 1.1 What "three-word" means over BabyBear (choice C1-1)

The landed cell is the tuple `[kind, a, b, c]` at one address (`chip/membackend.rs:69`), keyed by `8 addr + kind` (`chip/membackend.rs:56`) over `Fq`. Over BabyBear no word holds two parts (FP-1), so the three slots `a, b, c` cannot each be one BabyBear word for the reference records.

- **Choice:** a physical cell is one address with `kind` and three typed slots `a`, `b`, `c`. Each slot is a lane group of FP-1 types. `W_rec` is the word count of the three slots of the widest kind. Every reference record of every kind is one cell, so `e = 1`.
- **Alternative:** three BabyBear words per address, and a wider record takes `ceil(words / 3)` consecutive addresses.
- **Reason:** the alternative breaks the AD-1 ruling. With the lane counts of 1.3, a DATA record is 28 words, so 10 cells, and a FIELDS record is 2 cells (DERIVED). The largest X1 step, `BHelperScalarExit` (2 FIELDS + 1 DATA, `ARCH` FD `EXPERIMENTS-MAXIMA.md:19`), would then append 14 cells. The AD-1 condition `len_0 + a e 2^64 <= 2^(TOP+3) - 1` (`ARCH:3438`) with `a e = 14` needs TOP >= 65 (DERIVED: `14 x 2^64 <= 2^66` is false). The owner ruled TOP = 63 on the premise `e = 1`. The chosen reading keeps that premise, the X1 cell counts (`EXPERIMENTS-MAXIMA.md:19-22`) and the window capacity `8 W + 7 - r_pre` (`ARCH:1262`) unchanged, because each of those counts reference records and each reference record is one cell.
- The sentence "A logical record wider than three words (`a, b, c`) takes consecutive addresses" (`ARCH:1268`) then has no case in the reference language: no reference record needs two cells. The mechanism stays for a later record kind; its mutant `mem_wide_record_split_across_leaves` (`ARCH:1300`) stays as a control.

### 1.2 Record kinds

The four kinds are the landed constants `KIND_DATA = 1`, `KIND_FIELD = 2`, `KIND_ENV = 3`, `KIND_SLOT = 4` (`chip/membackend.rs:50-53`), carried as `kind: tag(3)` with `kind` in `1..=4` (FP-1 MEM row, `ARCH:332`). They are the four reference heap tables `data`, `data_fields`, `env`, `value_slots` (`hm/c6_phys_data.rs:328-334`, struct `C6DataTables`) in one arena (L-9, M-1).

### 1.3 Lane types per kind

Type rule for a tagged union (the ENC-4 rule, `ENC` section 4): each lane takes the widest type of the fields that any variant maps to it. Index rule: a reference into a heap table becomes an `addr` in the one arena; a reference into a program table is a ROM row index of type `ridx`. `ridx` is `u64` under the AD-12 recommendation of section 2 and `u32` under its alternative; the word counts below give both.

A reference `RefC` is `(rtag, idx)`. The reference has two variants, `Const(i)` (tag 0) and `Data(i)` (tag 1) (`hm/c6_phys_data.rs:115-123`, `put_ref`; `hm/shared.rs:701-705`). Under AD-17 (b) the input value is held by digest and is not a PROG constant, so a third variant `Input(i)`, a record index of the input view (section 4), is needed (OPEN O1-1 below). The lane is `rtag: tag(2)` either way, so the word count does not depend on O1-1. `idx` takes the widest of `ridx` (Const), `addr` (Data) and `addr` (Input, section 4): `addr`, 5 words.

An environment root `EnvRootC = Option<usize>` (`hm/shared.rs:725`) is `(present: bit, at: addr)` (`hm/c6_phys_data.rs:131-136`, `put_env`).

**DATA, kind 1** (reference `encode_data_row`, `hm/c6_phys_data.rs:194-224`, 5 lanes; decode and padding `:233-272`):

| Slot | Lane | Variants mapped (reference lane) | Type | Words |
|---|---|---|---:|---:|
| a | `form` | value tag 0 `U128` to 6 `Constructor` (lane 0) | `tag(3)` | 1 |
| a | `p1` | `U128.n` (u128), `Bool.b` (bit), `Constructor.type_id` (program table index) (lane 1) | `u128` (widest) | 8 |
| b | `ctor` | `Constructor.ctor_id` (lane 2) | `ridx` | 4 (2) |
| b | `fstart` | `fields.start` of `Cons`, `Leaf`, `Branch`, `Constructor` (lane 3): the address of the first FIELDS cell | `addr` | 5 |
| b | `flen` | `fields.len` (lane 4), at most `max_arity` (`hm/c6_phys_data.rs:179-189`) | `u32` (the `arity: u32` of the FP-4 `vd` row) | 2 |
| c | `vd` | the value digest of the node (V-7 (a), choice C1-2) | `digest` | 8 |
| | | | **total** | **28 (26)** |

**FIELD, kind 2** (reference `encode_ref_row`, `hm/c6_phys_data.rs:292-296`, 2 lanes):

| Slot | Lane | Type | Words |
|---|---|---|---:|
| a | `rtag` (0 Const, 1 Data, 2 Input) | `tag(2)` | 1 |
| a | `idx` | `addr` | 5 |
| b, c | zero | | 0 |
| | | **total** | **6** |

**ENV, kind 3** (reference `encode_env_row`, `hm/c6_phys_data.rs:301-307`, 5 lanes; caps `:316-320`):

| Slot | Lane | Reference | Type | Words |
|---|---|---|---|---:|
| a | `binder` | lane 0, a name-table index (`cap_index(.., max_names)`, `:314`) | `ridx` | 4 (2) |
| b | `vtag`, `vidx` | lanes 1-2, `RefC` | `tag(2)`, `addr` | 6 |
| c | `prev_present`, `prev` | lanes 3-4, `EnvRootC` | `bit`, `addr` | 6 |
| | | | **total** | **16 (14)** |

**SLOT, kind 4** (reference `encode_ref_row`, 2 lanes): the FIELD layout, 6 words.

**`W_rec` = 28** at `ridx = u64` (26 at `ridx = u32`), DERIVED: the maximum of 28, 6, 16 and 6. Every kind is padded to `W_rec` with zero words in its unused positions, in the fixed lane order DATA `form, p1, ctor, fstart, flen, vd`; the other kinds use the leading lanes of the table above. The MEM message is `kind: tag(3), addr: addr, rec: W_rec words`, so `6 + W_rec` = **34 words** (32), DERIVED; with the bus index, 35 <= 128 (LogUp bound 1, `ARCH` FP-1).

### 1.4 Range checks, padding and the record hash

| Lane type | Range owner at the append (readers inherit: memory is write-once and authenticated, `ARCH:113`) |
|---|---|
| `form`, `rtag` (`tag(w)`) | the appending rule, by a gate that pins the constant of its value form |
| `p1` as `u128` | ALU, when the value is an ALU output (`ARCH:108`); otherwise RANGE on each limb. As a `bit` or a `ridx` the unused limbs are zero (padding rule) |
| `ctor`, `binder` (`ridx`) | copied from an `AuthRom` row cell (ranged by `D_row`, section 2) or from a RANGE handle |
| `fstart`, `idx`, `prev` (`addr`) | copied from an append-chain address or an `AuthRec` address, so below `len_cur`; never a free payload (`core_append_address_free`, `ARCH:1092`) |
| `flen` (`u32`) | RANGE on each limb |
| `vd` (`digest`) | none (canonical by representation, FP-1); its value is the VALUE DIGEST output copied in (`ARCH:1868`) |

- **Padding rule.** A lane that the variant does not use is zero, as the reference decode requires (`zero(DATA_S, l, used, DATA_ROW_LANES)`, `hm/c6_phys_data.rs:271`; `get_env` `:137-142`). The appending rule constrains it. Mutant `rec_pad_lane_nonzero` (new): a `Nil` DATA record with `p1 = 1`; refused by the rule's padding gate; the control (gate removed) accepts it and gives two cells for one value.
- **Record hash** (MEMORY BACKEND over BabyBear): `H_REC = P2(DOM_MEMREC, kind, rec[0..W_rec])`, `DOM_MEMREC` = 33 (the HASH registry order `MemRec, MemPad, MemEmpty, MemNode, ...`, `ENC` section 4, ENC-5 paragraph). 30 words with the tag, so 4 permutations per record (DERIVED: `ceil(30 / 8)`). The halo2 `H_REC(kind, a, b, c)` of `chip/dense.rs:66-69` is the prototype.

**Choice C1-2: `vd` in the DATA record (V-7 (a)).** Alternative: `vd` with every reference (V-7 (b)), which adds 8 words to each FIELD, SLOT and ENV value lane. Reason: V-7 recommends (a) so that a child `vd` is read from a kind-constrained record (`ARCH:3585`), and ARCH places V-7 inside AD-11 (`ARCH:3538`).

### 1.5 Port cells (I-9)

| Port | Cells, in order | Words (`ridx = u64`), DERIVED |
|---|---|---:|
| `AuthRec` | `addr: addr`, `kind: tag(3)`, `rec: W_rec` | 34 |
| `ColdRead` | `enable: bit`, `addr: addr`, `kind: tag(3)`, `rec: W_rec` | 35 |
| `Window` row | `addr: addr`, `act: bit`, `imp: bit`, `kind: tag(3)`, `rec: W_rec` | 36 |
| `Window` frontier | `len_pre: addr`, `len_post: addr`, `q_pre` (`addr` type, top limb below `2^(TOP - 16 (n_a - 1))`), `base: addr` | 20 |
| `AuthRom` | section 2.5 | |

Each port message across AIRs adds `seq: u32` (FP-1 ports, I-8). The port certificates are those of ARCH 1.3 (`ARCH:137-138`), unchanged.

### 1.6 State schema lanes (I-15 body words)

The CORE persistent state uses the same type rule as the records (ENC-4 widest-lane rule, `ENC` section 4). A helper pass typed every lane of the reference encoders: the frame (`encode_frame`, `hm/c6_phys.rs:670`, 13 lanes), the task (`encode_task_record`, `hm/c6_phys_helper.rs:988`, 7 lanes), the helper (23 lanes), the result (4 lanes), the control (`encode_control`, `hm/c6_phys.rs:892`, 17 lanes) and the scalar registers of image lanes 19 to 47 (`hm/c6_phys.rs:56-74`). The lead checked the totals by script (session scratch scripts `lanes.py` and `lanes_ridx_len.py`, not kept in the repository; Appendix A holds their data and output). Appendix A gives the lane-by-lane table with sources. The rule that decides between a ROM index and a heap address is the cap that the reference checks for that field: a program-section cap gives `ridx`, a heap-table cap gives `addr` (Appendix A, "Rules applied").

| Part | Lanes | Words at `ridx = u64` (`u32`) | `ridx` lanes |
|---|---:|---:|---:|
| control | 17 | 69 (61) | 4 |
| scalar registers, lanes 19 to 46 | 28 | 147 (145) | 1 |
| frame stack `StackCells`: `count: u64`, `below: digest`, top frame | 2 + 13 | 64 (54) | 5 |
| task stack `StackCells`: `count: u64`, `below: digest`, top task | 2 + 7 | 31 (29) | 1 |
| helper record | 23 | 97 (89) | 4 |
| result record | 4 | 16 (16) | 0 |
| memory `len: addr`, `root: digest`, `tail_d: digest` | 3 | 21 | 0 |
| `prog_root: digest`, `prog_d: digest` | 2 | 16 | 0 |
| **body total, reference image** | | **461 (431)**, DERIVED | 15 |

Every word column of this table is DERIVED (script sums of the FP-1 word counts). The `ridx` lanes include the lengths of program slices and the cursors over them (choice C2-6, section 2.2). Lane 47 (`frame_count`) is the frame stack `count` and is counted once. The codec framing lanes (magic, version, section headers) are not state.

Recommended adjustments, each a choice for the owner:

- O-3: move the outcome (image lanes 23 to 25, 7 words) to the header words `outcome_tag`, `outcome_payload`, as FP-5 already does (`ARCH:493-497`). Body -7.
- O-4: `view_root` is a header word (FP-5) and is not in the body. Body +0.
- O-5: add the constructor fold cells that VALUE DIGEST carries across leaves (`ARCH:1870`): `cursor: u32`, `m: u32`, `acc: digest`, `ty_d: digest`, `c: u32`, which is 22 words, ESTIMATE (the cell list is from ARCH and the types are this draft's picks). The input reference of the kit INIT table depends on O1-1. Scratch `v0..v2` stays outside the body (CORE invariant 8).
- With these, the recommended body is **476 words** at `ridx = u64` (446 at `u32`), ESTIMATE. The body digest `P2(DOM_STATE_BODY, schema_id: u16, body)` then has 478 words, which is 60 permutations per boundary (DERIVED: `ceil(478 / 8)`).

OPEN items of the state schema that change the count (Appendix A, "OPEN lanes"):

- O-1, helper `parent_resume` (lane 4). The source defers its width ("Bounded width DEFERRED to X-05", `hm/shared.rs:745-746`). `u32` gives the totals above; `u64` adds 2 words, `addr` adds 3.
- O-2, the 14 counter lanes (image lanes 32 to 45: allocation, active and segment counters). Reference type `usize` (`hm/shared.rs:727-738`). Typed `addr` here, because each counts heap cells of the one arena. With `u64` the body is 26 words smaller. Whether the per-kind active lengths stay registers or follow from `len` is not settled by any text (`ARCH:2766-2768`) and is OPEN (O6-3). Recommendation: keep them as `addr` registers, because each has its own cap and exhaust kind in the reference.
- O-6 to O-9 settle type names or double counts with no word effect (Appendix A).

The `StackStep` and `StackCells` layouts of ENC-4 (`ENC` section 4) take the frame record (52 words at `ridx = u64`, DERIVED) and the task record (19 words, DERIVED) of Appendix A. So this section also gives ENC-4 its record types. ENC-4 is a separate FREEZE-PENDING item of kit STACKS, not a T5 row.

### 1.7 OPEN items of the record schema

- **O1-1, the input reference.** Under AD-17 (b) the reference construction `apply(prog, x)` puts `x` in the PROG constants (`O/design.md:218-220`), but the leaf's `prog_root` does not change with `x` (`O/design.md:218`). A step that binds or forwards the input needs a reference that names an input-view record. Options: (a) a third `RefC` variant `Input(i)` with `rtag = 2`; (b) `Const(i)` with `i` at or above the program's constant count, served by `read_input` instead of ROM, which needs a range comparison on every `Const` read. Recommendation (a): no comparison on the hot path, and the `rtag: tag(2)` lane costs no word. Both need the `ENTRY`/`ARG` rule change that `O/design.md:218` records as not made; X11 (`ARCH:4007`) has not run.
- **O1-2, `ridx`.** Set by AD-12 (section 2).

### 1.8 Closure test (I-5, I-9 and the AD-11 part of I-15)

- Test `rec_schema_vectors` (new): for each kind and variant, the typed lanes of a reference record computed by the Rust reference encoder, by the AIR trace generator and by a hand-written vector agree. Hand vectors: `U128 2^128 - 1` gives `p1 = [65535; 8]`; `Bool true` gives `form = 1, p1 = [1, 0, ...]`; `Constructor(type 3, ctor 2, fields at 2^65 + 1, len 3)` gives `fstart = [1, 0, 0, 0, 2]` (the `bus_enc_vectors` limb rule, `ARCH` FP-1 checks).
- Positives: `rec_schema_roundtrip_reference` (every appended record of the X1 corpus, 6,669 steps (MEASURED, `X15`), decodes to the reference row through REFERENCE BRIDGE); the landed C1 positives re-run at `W_rec` (`c1_pass_through_and_every_tail_occupancy`, `c1_cross_boundary_reads_and_chain`, `ARCH:1298`); `ok_max_rule_step_at_s1` (`ARCH:1592`) with `BHelperScalarExit` appending 3 cells.
- Refused mutants: `rec_pad_lane_nonzero` (new, above); `rec_kind_layout_swap` (new: FIELD lanes appended under `kind = 1`; refused by the rule's kind pin, A4 of kit CORE, `ARCH:1038`); `rec_rtag_out_of_range` (new: `rtag = 3`); existing `bus_limb_over_range`, `bus_addr_top_limb_over`, `append_value_mismatch`, `wrong_kind_warm`, `wrong_kind_cold`, `kind_out_of_range`. Keygen refusal `bus_message_too_long` stays unreachable at 35 words.

### 1.9 Lines that change

- ARCH 1.11 FP-1 bus table, MEM row (`ARCH:332`): write `W_rec = 28` and the layout table of 1.3; the paragraph at `ARCH:339` points to this layout.
- ARCH 1.12 I-5 (`ARCH:549`), I-9 (`ARCH:553`): status text after sign-off; kit rows MEM window (`ARCH:594`), MEMORY BACKEND (`ARCH:595`), VALUE DIGEST (`ARCH:598`), CORE (`ARCH:592`, AD-11 part).
- Kit MEM window: `ARCH:1268` (wide records: no case in the reference language; mechanism kept). Kit MEMORY BACKEND: the `H_REC` form over BabyBear. Kit VALUE DIGEST: `ARCH:1868` (`vd` in the DATA record, V-7 (a)). AD-11 row `ARCH:3448`; V-7 `ARCH:3585-3590`; the decision index row `ARCH:3342`.
- `ML/tasks.md:28` (P.2, there named OD-11): done when the owner signs. `ML/specs/sherlock-chip-mem/spec.md` and `ML/specs/sherlock-chip-membackend/spec.md`: the record width and the `H_REC` preimage. The modular-leaf tasks call these items OD-11, OD-12, OD-17, OD-18; ARCH calls them AD-11, AD-12, AD-17, AD-18.
- OpenSpec lines: `ML/design.md:49` (MEM tuple `(2, 8 addr + kind, a, b, c)`: replace by the FP-1 MEM message of 1.3); `ML/design.md:38` and `ML/specs/sherlock-chip-bus/spec.md:4` (the five-cell `(chip, key, a, b, c)` tuple: prototype form, pointer to FP-1); `ML/specs/sherlock-chip-bus/spec.md:52` (the packed key `8 addr + kind`: the BabyBear message has no packed key); `ML/specs/sherlock-chip-mem/spec.md:22` (MEM-003, the key `8 addr + kind`) and `:62` (MEM-007, "three-word records": write the C1-1 reading, one cell with a kind tag and three typed slots); `ML/specs/sherlock-chip-membackend/spec.md:37` (MBK-004, the cold read opens the `H_REC` leaf of 1.4); `ML/specs/sherlock-chip-core/spec.md:33` (CORE-004, `AuthRec` cells of 1.5); `ML/tasks.md:34` (F.1, the bus schema and the ports manifest).
- ARCH section 5.2 (the spec outline): `ARCH:3718` (MEM key requirement, as for MEM-003). ARCH 3.1 decision index row `ARCH:3342`: status "drafted, awaiting owner sign-off".

## 2. AD-12 with X15 (I-6; `AuthRom` part of I-9; kit PAGE/ROM)

Inputs: `K_rom = 15` FROZEN by the owner (`DA` line 80; `X15`, "Owner ruling 2026-11-12 20:35"); AD-12 (`ARCH:3449`) and V-1 (`ARCH:3540-3547`) reject a depth below the language's index width and a per-program depth.

### 2.1 Tables (`D_tbl`)

`ProgramView` has 22 tables (`hm/shared.rs:1269-1295`; `entry` is a scalar, not a table), READ and counted in `X15` ("ROM tables of `ProgramView` | 22"). 13 of them are read by some corpus step (MEASURED, `X15`).

- **`D_tbl = 5`** (DERIVED: `2^5 = 32 >= 22`). Table id `tbl: tag(5)`, assigned in the field order of `ProgramView`: 0 `expressions`, 1 `occurrences`, 2 `occurrence_paths`, 3 `children`, 4 `functions`, 5 `parameters`, 6 `arms`, 7 `patterns`, 8 `pattern_children`, 9 `types`, 10 `type_args`, 11 `constructors`, 12 `ctor_fields`, 13 `decl_param_counts`, 14 `decl_ctor_counts`, 15 `decl_templates`, 16 `plan_nodes`, 17 `plan_cases`, 18 `field_paths`, 19 `constants`, 20 `constant_fields`, 21 `names`. Ids 22 to 31 have no rows; a request with such an id is unsatisfiable.
- **Choice C2-1, every table is a ROM table.** Alternative: only the 13 tables that steps read. Reason: ADMIT-WORK opens every row of every table under `prog_root` (`ARCH:2234`), and `prog_d` must bind the whole program normal form (PAGE/ROM item 17, `ARCH:1559`); a table outside ROM would need a second commitment.

### 2.2 Row layouts (`W_rom`)

The reference program encoding is a serial lane stream, not fixed rows (`hm/c6_phys_program.rs:419-836`, `encode_program`); the rows below type the same fields per row, with the slice rule `(start: ridx, len: ridx)` (choice C2-6) and the option rule `(present: bit, value: ridx)` of `Writer::slice` and `Writer::opt_usize` (`hm/c6_phys_program.rs:144-155`).

| tbl | Table | Row fields (encoder lines) | Words at `ridx = u64` (`u32`) |
|---:|---|---|---:|
| 0 | `expressions` | `kind: tag(4)` (0 to 8), then six lanes, widest per position: 1 to 3 `ridx`; 4 `fields.len` or `plan.present`, so `ridx`; 5 `constant.present` or `plan.value`, so `ridx`; 6 `constant.value: ridx` (`Construct`: type, ctor, fields start, len, constant option; `Match`: scrutinee, arms start, len, plan option; `:430-498`) | 1 + 6x4 = 25 (13) |
| 1 | `occurrences` | `owner_is_function: bit`, `function: ridx`, `path` slice (`:505-509`) | 1 + 4 + 8 = 13 (7) |
| 2, 3, 8, 10, 13, 14, 18, 20 | single-index tables | one `ridx` (`:516-533`, `:602-610`, `:620-624`, `:648-665`, `:746-754`, `:823-831`) | 4 (2) |
| 4 | `functions` | `name: ridx`, `params` slice, `return_type: ridx`, `body: ridx` (`:534-545`) | 4 + 8 + 4 + 4 = 20 (10) |
| 5, 6, 12 | `parameters`, `arms`, `ctor_fields` | two `ridx` (`:546-561`, `:638-647`) | 8 (4) |
| 7 | `patterns` | `kind: tag(3)` (0 to 6), `type: ridx`, `ctor: ridx`, `children` slice (`:562-601`) | 1 + 4 + 4 + 8 = 17 (9) |
| 9 | `types` | `name: ridx`, `decl` option, `args` slice, `ctor_start: ridx`, `ctor_len: ridx`, `ty_d: digest` (`:611-619`; `ty_d` by section 5) | 4 + 5 + 8 + 4 + 4 + 8 = 33 (21) |
| 11 | `constructors` | `owning_type, name, source_order, field_start, field_len: ridx` (`:625-637`) | 5x4 = 20 (10) |
| 15 | `decl_templates` | `decl_d: digest` (section 5; the template text is variable length, `:666-691`) | 8 |
| 16 | `plan_nodes` | `kind: tag(2)`, then `Switch` lanes `source_match: ridx`, `path` slice, `type: ridx`, `cases` slice, `default: ridx` (`:692-736`) | 1 + 4 + 8 + 4 + 8 + 4 = 29 (15) |
| 17 | `plan_cases` | option (`:737-745`) | 5 (3) |
| 19 | `constants` | `kind: tag(3)` (0 to 7), `marker_free: bit`, `p1: u128` (n, b, start or type), `p2: ridx`, `p3: ridx`, `p4: ridx` (`fields.len` of `Constructor`), `vd: digest` (`:755-822`) | 1 + 1 + 8 + 4 + 4 + 4 + 8 = 30 (24) |
| 21 | `names` | `name_d: digest` (section 5; names are variable-length strings, `:832-835`) | 8 |

Every word column above is DERIVED (FP-1 word counts of the typed lanes). **`W_rom` = 33** at `ridx = u64` (24 at `ridx = u32`), DERIVED: the maximum over the rows above, `types` at `u64`, `constants` at `u32`. Rows are padded with zero words to `W_rom`. The ROM message is `tbl: tag(5), row: ridx, rec: W_rom` = **38 words** at `u64` (1 + 4 + 33; 1 + 2 + 24 = 27 at `u32`), DERIVED; with the bus index at most 39 <= 128.

- **Choice C2-2, one physical row per logical row.** Every row fits one message, so no row has sub-rows and `D_row` has no sub-row bits. Alternative: V-2 (a) sub-rows with the index in the low key bits (`ARCH:3551-3556`). Reason: V-2 recommends sub-rows to keep the five-cell halo2 bus (`bus.rs:34`); over BabyBear the bus message has up to 127 words (LogUp bound 1) and FP-1 already types the ROM message as `3 + W_rom` words (`ARCH:334`).
- **Choice C2-3, variable-length tables as digest rows.** `decl_templates` and `names` hold strings and type trees (`hm/c6_phys_program.rs:666-691`, `:832-835`). Each row is one digest: `decl_d` (section 5) and `name_d = P2(DOM_NAME, len: u32, bytes, one byte per word)` (the one-byte-per-word rule of pick P-4, `DA` line 74). Alternative: a byte table with slices. Reason: no step reads either table (they are not among the 13 tables read on the corpus, MEASURED, `X15`); admission needs only their digests (section 5).
- **Choice C2-6, a slice length has the type of its index.** Every program slice is `(start: ridx, len: ridx)`, and a cursor that runs over a program slice (`Find.row_cursor`, `Bind.p_cursor`, `Plan.path_cursor`) is `ridx`; a heap slice length and its cursor take `addr`, the type of the heap start. Alternative: `len: u32`, the base typing of the encoder lanes. Reason: under O2-1 (b) a `u32` length bounds every slice to `2^32 - 1` entries, a language bound of the same kind as option (a) puts on a table. At `ridx = u32` the rule changes no word. At `ridx = u64` it adds 4 words to `W_rom` and 4 body words (461 against 457, DERIVED by the rerun of the state script; every heap length already sits in an `addr` lane). The lanes that stay `u32` are listed in O2-1.
- **Choice C2-4, ROM rows carry `ty_d` and constant `vd`.** VALUE DIGEST reads `ty_d` from `tbl_types` and constant `vd` from `tbl_consts` (`ARCH:1863`). Alternative: recompute in each step. Reason: AD-14 puts the check of both in ADMIT-WORK at the seal (`ARCH:1560`, item 18).

### 2.3 Depth over physical rows (`D_row`, `D_rom`)

Every table index in the reference is a host `usize` (`hm/shared.rs:1269-1295`; `X15`, "Every table index is a host `usize`"). The per-layout caps `BoundsC` (`hm/shared.rs:670-692`) are per layout, not per language (`X15`), and `prog_d` does not commit `Bounds` (`O/design.md:92`, 2.7.7). The sources give no language-level index width (search: no production `index_widths` entry outside tests and wire code; OPEN fact, not a choice).

OPEN O2-1, `D_row`:

| Option | `row` type | `D_row` | `D_rom = D_tbl + D_row - 3` | Effect |
|---|---|---:|---:|---|
| (a) | `u32` (as written in FP-1, `ARCH:334`, and FP-4 `entry_a: u32`, `ARCH:462`) | 32 | 34 | bounds every program table to `2^32` rows. This is a language bound that no source states, so under AD-12 it is an amendment of GP-1 that only the owner can make (`ARCH:3449`, last column) |
| (b) | `u64` | 64 | 66 | covers every index of the 64-bit reference host (`usize` = `u64`). Changes the FP-1 ROM row to `row: u64`, the FP-4 `image_d` field `entry_a` to `u64` (+2 words, 24 with the tag), and every program index lane, program slice length and cursor over a program slice of the state schema to `u64` (C2-6) |

Lanes that stay `u32` under (b), each with its bound:

- DATA `flen` (1.3), the task `cursor` and `ConstructorFieldsF.index` (Appendix A): each counts or indexes the fields of one constructor value, at most `max_arity` (`hm/c6_phys_data.rs:188`), and FP-4 already types the arity as `arity: u32` (`ARCH:464`). They add no bound beyond that frozen one.
- The count and position words of the section 5 preimages (`p`, `nc`, `m`, `n`, `j`, `i` and the `name_d` length): they follow this option, `u32` at (a) and `u64` at (b). They are preimage words, not lanes, so the pick changes preimage lengths only.
- Helper `parent_resume` stays OPEN O-1 (1.6).

With these, (b) adds no language bound: every remaining `u32` lane is bounded by the arity.

Recommendation: **(b), `D_row = 64`, `D_rom = 66`**, by the same reasoning the owner accepted for TOP = 63 in AD-1: a circuit constant must not bound the run scope below what the reference admits. Cost of (b) over (a), DERIVED: +11 words per ROM message (38 against 27), +2 words on `W_rec`, +30 body words (461 against 431), one more permutation per leaf hash (35 words against 26), and 32 more node levels per ROM line. That is `15 x (8 + 32) = 600` more HASH requests per leaf at `K_rom = 15`. The cost per level is OPEN O2-2.

### 2.4 ROM tree and line cost

- Leaf: `H_ROMROW = P2(DOM_ROMLEAF, tbl, row_words[0..W_rom])`, `DOM_ROMLEAF` = 39 (`ENC` section 4). With the table id in the preimage the leaf of table 1 cannot stand for a row of table 14 (`rom_tbl_row_alias`, `ARCH:1557`). 35 words, 5 permutations at rate 8 (DERIVED; 26 words and 4 permutations at (a)).
- Node: `P2(DOM_ROMNODE, l, r)`, `DOM_ROMNODE` = 40, one HASH bus request of the frozen 25-word HASH message (I-4) per level. **Choice C2-5, no height word in the node.** Alternative: the height in the preimage, as `H_ROMNODE_h` (`ARCH:1537`). Reason: the HASH message `(tag, x, y, h)` has no height lane, the depth is fixed so every path has exactly `D_rom + 3` levels, and leaf preimages (35 words) and node preimages (17 words) have different lengths, so under premise A5 no splice exists. The mutant `rom_height_splice` (`ARCH:1558`) becomes a mutant at a wrong path length.
- Line cost, DERIVED: 8 leaves x 5 + 7 `Tree8` levels + `D_rom` path levels = 113 HASH requests or sponge slots per line at (b), 1,695 per leaf at `K_rom = 15`; at (a) 8 x 4 + 7 + 34 = 73 per line, 1,095 per leaf.
- OPEN O2-2, the permutations per node: `P2` over 17 words is 3 permutations (`P2` as fixed by pick P-3, `ENC` section 5); a capacity-tagged compress of one permutation is not defined in FP-4. This changes the permutation count, not the layout.

### 2.5 `AuthRom` port cells

`tbl: tag(5)`, `row: ridx`, `rec: W_rom`: 38 words at (b) (1 + 4 + 33) and 27 at (a) (1 + 2 + 24), DERIVED. The port certificate is unchanged (`ARCH:138`): `tbl` is a call-site constant, `row` is a RANGE handle of `D_row` bits.

### 2.6 Closure test (I-6 and the `AuthRom` part of I-9)

- Test `rom_schema_vectors` (new): per table, the typed row of the Rust reference against the AIR trace generator; `name_d` and `decl_d` rows against section 5.
- Positives: `ok_rom_line_each_table` (`ARCH:1590`, now 22 tables); `ok_rom_last_partial_line`; `ok_max_rule_step_at_s1` with `BHelperEqualField` and `XAdtHelperPlanSwitch` at 15 ROM rows (the X15 static maxima, DERIVED); `rom_depth_covers_table_index` (`ARCH:1561`, a host completeness check: a row index `2^64 - 1` in a sparse tree of depth `D_rom + 3 = 69` opens).
- Refused mutants: existing `rom_row_overflow_alias` (row `2^D_row`), `rom_tbl_row_alias`, `rom_key_not_from_path`, `rom_heap_domain_splice`, `rom_pad_noncanonical`, `vd_rom_row_overflow`; new `rom_tbl_unassigned` (`tbl = 22`, unsatisfiable), `rom_path_length_short` (a path of `D_rom + 2` levels, the C2-5 replacement of `rom_height_splice`).

### 2.7 Lines that change

- ARCH FP-1 bus table ROM row (`ARCH:334`): `tbl: tag(5)`, `row: u64`, `W_rom = 33`, 38 words; FP-4 `image_d` row (`ARCH:462`): `entry_a: u64` (option (b) only).
- ARCH 1.2 field table (`ARCH:111`, `:115`): prototype rows, add a pointer to FP-1. Kit PAGE/ROM: `ARCH:1486` (`K_rom = 15` frozen), `:1522-1529` (`D_tbl = 5`, `D_row = 64`, `D_rom = 66`), `:1535` (no sub-rows), `:1537` (BabyBear tags 39 and 40, no height word), `:1577-1578` (line cost; the `K_rom = 2` example is ruled out by X15). AD-3 `ARCH:3440`, AD-12 `ARCH:3449`, V-1 `ARCH:3540-3547`, V-2 `ARCH:3549-3556`, V-4 `ARCH:3564-3568`.
- ARCH 1.12 I-6 (`ARCH:550`), I-9 (`ARCH:553`), kit row PAGE/ROM (`ARCH:596`).
- `ML/tasks.md:29` (P.3, there OD-12): done when the owner signs. `ML/specs/sherlock-chip-page-cache/spec.md`: the ROM key, depth and row layout.
- OpenSpec lines: `ML/specs/sherlock-chip-page-cache/spec.md:37` (PAGE-004: replace the key `tbl 2^D_row + row` by the typed `tbl, row` lanes, and "carry heights" by C2-5 with tags 39 and 40) and `:40` (the overflow scenario reads `row >= 2^64` at (b)); `ML/specs/sherlock-chip-bus/spec.md:52` (the packed ROM key `tbl 2^D_row + row`); `ML/specs/sherlock-chip-core/spec.md:44` (a ROM row is a RANGE handle of width `D_row = 64`); `ML/specs/sherlock-value-digest/spec.md:25` (row overflow of `D_row`, no text change at (b)); `ML/design.md:51` (ROM tuple `(4, tbl 2^D_row + row, a, b, c)`: replace by the FP-1 ROM message of 2.2).
- ARCH section 5.2: `ARCH:3759` (PAGE/ROM key and "carry heights", as for PAGE-004) and `:3760`. ARCH 3.1 decision index row `ARCH:3343`: status "drafted, awaiting owner sign-off". ARCH 5.4 experiment row X15 (`ARCH:4011`): the decision column reads "decided for `K_rom`; `D_row` by AD-12 sign-off".

## 3. AD-9 guard list and the control word (I-7; kit DEC)

Owner ruling: "AD-9: (a) priority network" (`DA` line 65). Under (a), `class = sum_r selected[r] (ordinal(r) + 1)` and `sum_r selected[r] = enable` (`ARCH:867`), and "the control word layout is frozen with the guard list of the 120 rules" (`ARCH:3446`). A helper pass extracted the list by script (a session scratch script, not kept in the repository; section 3.4 is its full output). The lead reran the script with the same output and checked three guards against source by hand (`BHelperBindExit`, `OR:879-886`; `BHelperMatchArm`, `OR:1052-1058`; P1 and P2, `OR:3705-3742`). Prefixes: `OR` = `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs`, `SH` = `crates/mori-core/src/sherlock/helper_machine/shared.rs`, `BE` = `crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs`.

### 3.1 Inventory checks (DERIVED, by script, at main `4ea6fa29d`)

- `RuleIdC` has 133 variants (`SH:400-534`). Removing 7 disabled (`OR:70-81`), 4 extension (`OR:82-90`) and the 2 `KAdminStartup*` rules leaves **120**; the script asserts this.
- That set equals the rules of `X15` `out/x15_rom_rules.csv` and of FD `maxima/rule_maxima.csv`. The 108 of `supported_rules()` (`BE:363`) are the 120 minus the 12 `XAdtHelper{Build,Plan}*` rules.
- Partition: 3 publication rules, 60 helper-group rules, 46 site (legacy) rules and 11 plan-entry rules, 120 in all. Each of the 117 non-publication rules has exactly one `owner_select_when` arm (`OR:541-1203`), so no guard is OPEN. `BBookkeepingTaskRelease` is first in both the Validate and the Equal group; it has one ordinal.
- Core and backend agree on the order of every group: `helper_group` (`OR:1589-1673` against `BE:141-212`, 9 shared kinds), `legacy_group` (`OR:1674-1720` against `BE:215-261`, 27 sites), the result-reader set (`OR:507-536` against `BE:297`) and the site order (`SH:220` against `BE:263`).

### 3.2 The priority network

`owner_step` (OR:3695):

- AB: `PreImage::bind` (OR:3701). `fin` present -> refusal (OR:3702).
- **P0** (OR:3705): `outcome != Running` -> `StepResult::Terminal`, no event, no rule.
- **P1** (OR:3710): only when `helper = none AND result = none`:
  - `ctl = Error` -> `KAdminPublishError` (OR:3712).
  - `ctl = Return AND frames empty` -> `KAdminPublishValue` (OR:3724).
- **P2** (OR:3739): `remaining_work = 0` -> `KAdminPublishExhausted`. This test is not gated on helper or result, so an active helper with zero fuel publishes Exhausted.
- **PS** (OR:3756): `owner_select` (OR:1722):
  1. `fin` / non-Running -> error (unreachable after P0).
  2. **Helper stage** (OR:1734): if a helper is present, scan `helper_group(helper_kind_of(h))` in order and take the first rule whose `owner_select_when` holds. If no rule holds -> `SelectUndefined "helper group exhausted"`.
  3. `site = owner_site(pre)` (OR:1747; definition OR:1207-1267). Errors (no rule) for `ctl in {Await, Error}`, for `ctl = Return AND depth = 0`, and when the Eval expression row is missing.
  4. **Result stage** (OR:1748-1750): if a result is present, scan `legacy_group(site)`, restricted to `reads_result_root` rules, and take the first whose `select_when` holds. If no rule holds -> `SelectUndefined "stale result"`.
  5. **Plan-entry stage** (OR:1761): if `owner_helper_plan(site, pre) = Some(k)` (OR:1268-1312), select `entry_rule(k)`. The Entry rules' `select_when` is constant `false` (OR:735-738), so they are reached only here.
  6. **Legacy stage** (OR:1765): scan `legacy_group(site)`, restricted to non-result-reading rules. If no rule holds -> `SelectUndefined "legacy group exhausted"`.
- Every selected rule then passes `selected_for` (OR:1542). That function calls `owner_site(pre)?` again (OR:1545), so a rule chosen on the helper path also needs a defined site. `check_enabled` (OR:91) runs on the selected rule (OR:1543) and again in `call_family` (OR:3642).

In the extracted list, every non-publication rule depends first on the PRE atoms `OUT`, `FUEL0` and `SITE` (3.3). This includes the 60 helper rows: a helper step also needs a defined site, and at an Eval control that means a ROM read of the expression row. `PRE` in the `guard` column abbreviates the stage prefix that every non-publication rule carries: `outcome = Running AND NOT P1 AND remaining_work != 0 AND owner_site defined`. The negations of earlier rules in the same scan are implied by priority and are not repeated. Two stage-level facts are written out explicitly, because they are stage tests and not negations of rules: `helper = none` / `result present|none` / `site = S`, and for the legacy stage `owner_helper_plan(S) = none`.

`owner_helper_plan(site)` (OR:1268-1312), per kind:

| kind | condition | Entry rule |
|---|---|---|
| Validate | site = EvalLiteral | BHelperValidateEntry |
| EnvGet | site = EvalVar | BHelperEnvGetEntry |
| EnvPut | site = RetLet | BHelperEnvPutEntry |
| Equal | site = RetRight AND RightF.op = Eq | BHelperEqualEntry |
| Scalar | (RetRight AND op != Eq) OR RetUnary OR RetIf OR RetCall OR (RetCtorFields AND checked = false) | BHelperScalarEntry |
| Append | RetArgs OR (RetCtorFields AND checked = true) | BHelperAppendEntry |
| Build | site = Materialize | XAdtHelperBuildEntry |
| Find | site = Invoke | BHelperFindEntry |
| Bind | site = Enter AND Enter.function present | BHelperBindEntry |
| Plan | site = Cases AND origin row = Match{plan = Some} | XAdtHelperPlanEntry |
| Match | site = Cases AND origin row = Match{plan = None} AND arms.len > 0 | BHelperMatchEntry |

Consequence: at the sites EvalLiteral, EvalVar, RetLet, RetRight, RetUnary, RetIf, RetCall, RetArgs, RetCtorFields, Materialize and Invoke the plan always exists, so their non-result scan is never reached. Their legacy groups hold only result readers, which is consistent.

**Ordinal (choice C3-1).** `ordinal` is the order in which the network tries rules: P1 (`KAdminPublishError`, then `KAdminPublishValue`), P2, the helper stage (kinds in `HelperKind` order, `SH:179`; rules in `helper_group` order), the result stage (sites in `SiteTag` order, `SH:220`), the plan-entry stage (one Entry rule per kind, `OR:1313-1327`), then the legacy stage. The alternative is enum order, the `enc_RuleIdC` encoding with 0 to 132 (`crates/mori-core/src/sherlock/helper_machine/c6_stream_event.rs:112`). The reason for priority order: DEC must not take the inventory from enum discriminants (`ARCH:869`), and the enum includes the 13 rules that have no DEC row. In the table the ordinal runs 1 to 120 and the class is that number. The AD-9 form `ordinal + 1` is the same class with a 0-based ordinal. Class 0 is the idle row, and 121 values fit in `tag(7)`.

Groups: 11 helper kinds and 27 sites, 38 group keys. Split by the result bit, these are 40 scans (11 helper, 13 result, 16 non-result), 11 plan-entry branches and 3 publication tests. The longest scan is 9 rules (Bind, Match), DERIVED by the script count. All counts of this paragraph are DERIVED (source scan).

### 3.3 Guard facts (atoms)

The network needs 63 distinct facts (DERIVED, source scan). Each has a producer that `DispatchInputs` must link (`ARCH:863`).


63 atoms (DERIVED, source scan). Widths come from core enum variant counts (SH:77-250, SH:748-1240), with +1 where 'absent' is possible. Comparators and is-zero gates give booleans. Backend lane codes were used only as a cross-check; they cover 9 of the 11 helper kinds.

| atom | fact | producer | width |
|---|---|---|---|
| OUT | outcome tag (= Running) | image header outcome tag | tag(5) OutcomeC: Running/Halted/Rejected/Exhausted/Waiting -> 3 bits; used as boolean is-Running |
| FUEL0 | remaining_work = 0 | is-zero gate on remaining_work | boolean |
| DEPTH0 | frame count = 0 | is-zero gate on frame count | boolean |
| C_TAG | control tag | control tag cell (image control lane 0) | tag(12) ControlC -> 4 bits |
| H_TAG | helper kind tag (none or one of 11 HelperControlC variants) | helper kind tag (helper record lane 0) | tag(12) -> 4 bits |
| R_TAG | result tag (none or one of 19 HelperResultC variants) | result present bit + result record tag | tag(20) -> 5 bits |
| R_PAY | result payload: Validated.ok, ScalarSelector.sel, ScalarTypeTest.ok | result record lane 1 | tag(3) (BoolSelector) -> 2 bits; boolean for ok |
| SITE | owner_site(pre): site tag; defined unless ctl in {Await, Error} or (ctl = Return AND depth = 0) or Eval expr row missing | derived: C_TAG, E_TAG (ROM row at control.expr), DEPTH0, F_TAG | tag(27) SiteTag -> 5 bits (+1 'undefined' -> still 5 bits) |
| E_TAG | Eval expression row tag at control.expr | expr row via ROM (authenticated PROG read) | tag(9) ExpressionRowC -> 4 bits |
| E_CALL_ARGS0 | Eval Call row args.len = 0 | expr row via ROM + is-zero gate | boolean |
| E_CON_FIELDS0 | Eval Construct row fields.len = 0 | expr row via ROM + is-zero gate | boolean |
| E_CON_CONST | Eval Construct row constant present | expr row via ROM | boolean |
| O_MATCH | expression row at control_origin is Match | expr row via ROM at control_origin | boolean (from tag(9)) |
| O_PLAN | control_origin Match row plan present | expr row via ROM at control_origin | boolean |
| F_TAG | top frame tag | top frame (frame lanes muxed by frame count) | tag(10) FrameRowC -> 4 bits |
| F_RIGHT_EQ | top RightF.op = Eq | top frame field (BinaryOp) | boolean |
| F_ARGS_REST0 | top ArgsF.rest.len = 0 | top frame field + is-zero gate | boolean |
| F_CTOR_CHECKED | top ConstructorFieldsF.checked | top frame field | boolean |
| F_CTOR_REST0 | top ConstructorFieldsF.rest.len = 0 | top frame field + is-zero gate | boolean |
| C_ENTER_FN | Enter.function present | control payload lane | boolean |
| C_BOUND_OK | Bound.binding = Ok (else Bad) | control payload lane | boolean |
| C_CASES_ARMS0 | Cases.arms.len = 0 | control payload lane + is-zero gate | boolean |
| C_CASERES_HIT | CaseResult.matching = Matched (else NoMatch) | control payload lane | boolean |
| C_PRIM_ERR | PrimResult.result = Error (else Value) | control payload lane | boolean |
| PUSH_LT | depth < stack_limit | comparator via ALU (frame count vs stack_limit) | boolean |
| TASK_LIVE0 | task_live = 0 | is-zero gate on task_live (helper section) | boolean |
| T_TAG | top task tag | task top tag (task lanes muxed by task_live) | tag(4) none/ValidateTask/EqualTask/PairTask -> 2 bits |
| T_VISIT | top task visit = Fields (else Unvisited) | task top lane | boolean |
| V_MARKER | kind_of(top ValidateTask.r) = Marker | value read of a DATA record kind (checked value read; ref may point to PROG constant or DATA) | boolean |
| EQ_DIFFER | shallow_differ(top EqualTask.left, .right) | two value reads of DATA records (kind, uint, bool, type_id, ctor_id projections) + equality comparators | boolean |
| SC_PHASE | Scalar.phase | helper record field | tag(3) -> 2 bits |
| SC_MODE | Scalar.mode | helper record field | tag(10) ScalarMode -> 4 bits |
| SC_STAGED | Scalar.staged = PrimError | helper record field (ScalarStaged tag) | tag(7) -> 3 bits; boolean is-PrimError |
| EG_STATUS | EnvGet.status | helper record field | tag(3) TriStatus -> 2 bits |
| EG_CURSOR | EnvGet.cursor present | helper record field | boolean |
| EP_STATUS | EnvPut.status | helper record field | tag(3) -> 2 bits |
| FD_STATUS | Find.status | helper record field | tag(3) -> 2 bits |
| FD_ROW_LT | Find.row_cursor < admitted_function_count | comparator via ALU; function count from PROG (ROM header/parser walk) | boolean |
| FD_ROW_EQ | Find.row_cursor = admitted_function_count | equality via ALU; function count from PROG | boolean |
| BD_PHASE | Bind.phase | helper record field | tag(6) BindPhase -> 3 bits |
| BD_STATUS | Bind.status = Bad (else Ok) | helper record field | boolean |
| BD_P_LT | Bind.p_cursor < params.len | comparator via ALU over helper lanes | boolean |
| BD_A_LT | Bind.a_cursor < args.len | comparator via ALU over helper lanes | boolean |
| BD_P_EQ | Bind.p_cursor = params.len | equality over helper lanes | boolean |
| BD_A_EQ | Bind.a_cursor = args.len | equality over helper lanes | boolean |
| AP_PHASE | Append.phase | helper record field | tag(3) AppendPhase -> 2 bits |
| AP_STAGED | Append.staged present | helper record field | boolean |
| AP_LT | Append.src_cursor < src.len | comparator via ALU over helper lanes | boolean |
| AP_EQ | Append.src_cursor = src.len | equality over helper lanes | boolean |
| VA_STATUS | Validate.status | helper record field | tag(3) -> 2 bits |
| EQ_STATUS | Equal.status | helper record field | tag(3) -> 2 bits |
| MA_PHASE | Match.phase | helper record field | tag(8) MatchPhase -> 3 bits |
| MA_STATUS | Match.status | helper record field | tag(3) -> 2 bits |
| BU_PHASE | Build.phase | helper record field | tag(2) BuildPhase -> 1 bit |
| BU_STAGED | Build.staged present | helper record field | boolean |
| BU_LT | Build.src_cursor < src.len | comparator via ALU over helper lanes | boolean |
| BU_NODE | Build.node present | helper record field | boolean |
| PL_PHASE | Plan.phase | helper record field | tag(6) PlanPhase -> 3 bits |
| PL_AFTER | Plan.after_walk = Bind (else Test) | helper record field | boolean |
| PL_STATUS | Plan.status | helper record field | tag(3) -> 2 bits |
| PL_ARM | Plan.arm present | helper record field | boolean |
| PL_LT | Plan.path_cursor < path.len | comparator via ALU over helper lanes | boolean |
| PL_EQ | Plan.path_cursor = path.len | equality over helper lanes | boolean |

Tag-valued facts in the network: OUT, C_TAG, H_TAG, R_TAG, R_PAY, SITE (derived), E_TAG, F_TAG, T_TAG and the helper phase/status/mode tags. The rest are booleans.

Guards that read data. DEC takes these facts as authenticated inputs, never as supplied booleans:


These atoms cannot come from header or tag cells. DEC must receive them as authenticated facts with a linked producer.

| atom | rules | producer | core source |
|---|---|---|---|
| V_MARKER: `kind_of(top ValidateTask.r) = Marker` | BHelperValidateInvalid (37), BHelperValidateOccurrence (38) | checked value read of the referenced DATA record (or PROG constant) kind | OR:925-936 (Occurrence), OR:960-970 (Invalid); backend gate+`read_a` BE:725-756 |
| EQ_DIFFER: `pure_leaves::shallow_differ(left, right)` | BHelperEqualDifferent (42), BHelperEqualPair (43) | two value reads (kind, uint, bool, type_id, ctor_id projections) + 7 equality tests | OR:986-1004 (Pair), OR:1030-1048 (Different); backend gate+`read_b`/`read_c` BE:759-812 |
| FD_ROW_LT / FD_ROW_EQ: `row_cursor` vs `admitted_function_count` | BHelperFindRow (17), BHelperFindEmpty (18) | ALU comparator; the function count comes from the PROG walk | OR:809-824; backend BE:1094-1103 (`functions_count` from the PROG capture, BE:827) |
| E_TAG and the Eval row fields (args.len, fields.len, constant) | every non-publication rule whose control is Eval, including helper-stage rules (via `owner_site` in `selected_for`), plus KMachineEvalCallEmpty/Arguments, XAdtEvalConstructNullary/Fields | ROM read of the expression row at `control.expr` | OR:1209-1219 (owner_site Eval arm), OR:559-570 (Call), OR:683-694 (Construct) |
| O_MATCH / O_PLAN: the expression row at `control_origin` | KMachineCasesEmpty, KMachineCasesTry, XAdtCasesPlanHit, XAdtCasesPlanMiss, BHelperMatchEntry, XAdtHelperPlanEntry (and the legacy `NOT plan` test at site Cases) | ROM read at `control_origin` | OR:641-655 (CasesEmpty/Try), OR:719-734 (PlanHit/Miss), OR:1299-1309 (owner_helper_plan Cases) |
| PUSH_LT: `depth < stack_limit` | KMachinePushAccept, KMachinePushRefuse | ALU comparator over header cells | OR:571-572 |
| helper-lane comparators BD_*, AP_*, BU_LT, PL_* | BindPair/Empty/Arity, AppendCell/Empty, BuildCell/Field, PlanWalk/Switch/Bind | ALU comparators over helper record lanes (no external data) | the `defined at` column of 3.4 for each rule |

`GuardId` / `eval_guard` (OR:412-499: GScalarAllocNode, GScalarFields1/2, GValidateOccLeaf, GValidateFieldNontail, GEqualPairLeaf, GEqualFieldNontail, GBindTypeReject, GBindDupHit, GBuildLast) are **not** selection guards. `owner_select_when` ignores its `_g` argument (OR:544). These guards branch effects inside families (for example `families/structural.rs:416`, `fragments/scalar.proposed.rs:219`, `families/adt_build.rs:44`). They belong to the micro-op plan, not to DEC.

A `select_when` can also fail with an error (missing expression row, bad ref in `kind_of`, function count). In core that is a HostViolation, not a rule. For DEC it is an unsatisfiable enabled slot.

Core against backend:


The backend file has two emitters: `emit` (BE:1319, `facts` BE:610) and `emit_dedup` (BE:1637, `facts_dedup` BE:1357). Both call the same `Self::path` (BE:1335 and BE:1653), and `path` calls the same `Self::guard` (BE:1270, 1278, 1308). The guard and path deltas below therefore hold for both. Only the fact construction differs, and it was not compared line by line.

Order: **no difference.** The script compared `helper_group` for the 9 shared kinds (BE:141-212 vs OR:1589-1673), `legacy_group` for 27 sites (BE:215-261 vs OR:1674-1720), `reads_result` (BE:297) vs `reads_result_root` (OR:507-536), and `ALL_SITES` (BE:263) vs `SiteTag` (SH:220). Two differences outside order: `supported_rules` (BE:363) lists PublishValue before PublishError, which does not matter because the two are exclusive on the control tag. The backend has no Plan or Build kind; the 12 `XAdtHelper{Build,Plan}*` rules are core-only.

Guard deltas (backend `guard()` BE:902-1247 and `path()` BE:1248-1317 vs core `owner_select_when`):

1. **BHelperMatchArm.** The backend adds `origin row = Match AND origin.plan = None` (BE:1229-1237). Core `select_when` checks only `phase = Arm` (OR:1052-1058; 3.4 row 47). Core enforces the same condition later as a family refusal (`families/structural.rs:799-806` returns `wrong_variant`). Satisfiable steps are the same; the place where the condition is checked differs. Section 3.4 records the core guard.
2. **BHelperValidateField, BHelperValidateInvalid/Occurrence, BHelperEqualField, BHelperEqualPair/Different.** The backend adds `task_live > 0` explicitly. In core, `top_task() = Some(..)` already implies it. Equivalent on canonical state.
3. **Site defined.** Backend `path()` puts `site_defined` into every non-publication path (BE:1266; `site_defined` BE:698). Core gets the same effect from `owner_site` in `selected_for` (OR:1545). Equivalent.
4. **P1 Value.** Core tests `frames.is_empty()` (OR:3724). The backend tests frame count = 0 (BE:1253-1254). Equivalent on canonical state.
5. **any_plan.** The backend `any_plan` (BE:888-899) includes the Plan (Cases AND Match AND plan) and Build (Materialize) plans, so the legacy-stage `NOT plan` matches core even though those two Entry rules are not supported there.

### 3.4 The guard list (120 rules)

`PRE` abbreviates `outcome = Running AND NOT P1 AND remaining_work != 0 AND owner_site defined`. Every non-publication rule carries it, helper rules included, because `selected_for` calls `owner_site` again (`OR:1545`). At an Eval control that means a ROM read of the expression row. `defined at` is the selection test; `family entry` is the function of the rule family that builds the step (the publication rules have no family).

| ord | rule | stage, group, pos | guard (reference order; earlier rules of the group are negated by priority) | defined at | family entry |
|---:|---|---|---|---|---|
| 1 | `KAdminPublishError` | P1, -, 1 | outcome = Running AND helper = none AND result = none AND ctl = Error | OR:3705 (P0), OR:3710 (helper,result none), OR:3712 | OR:3661 publish_event (no family; owner_step builds the event) |
| 2 | `KAdminPublishValue` | P1, -, 2 | outcome = Running AND helper = none AND result = none AND ctl = Return AND frames empty (depth = 0) | OR:3705 (P0), OR:3710 (helper,result none), OR:3724 | OR:3661 publish_event (no family; owner_step builds the event) |
| 3 | `KAdminPublishExhausted` | P2, -, 1 | outcome = Running AND remaining_work = 0 (helper and result may be present) | OR:3739 | OR:3661 publish_event (no family; owner_step builds the event) |
| 4 | `BHelperScalarConstruct` | helper, Scalar, 1 | PRE AND helper = Scalar AND ((phase = First AND mode = Leaf) OR (phase = Second AND mode IN {Cons, Branch})) | OR:748 (select_when) | hm/fragments/scalar.proposed.rs:189 |
| 5 | `BHelperScalarArithmetic` | helper, Scalar, 2 | PRE AND helper = Scalar AND ((phase = First AND mode = Not) OR (phase = Second AND mode IN {Add, Sub, Mul, Lt})) | OR:743 (select_when) | hm/fragments/scalar.proposed.rs:142 |
| 6 | `BHelperScalarTag` | helper, Scalar, 3 | PRE AND helper = Scalar AND phase = First AND mode NOT IN {Leaf, Not} | OR:739 (select_when) | hm/fragments/scalar.proposed.rs:93 |
| 7 | `BHelperScalarFailure` | helper, Scalar, 4 | PRE AND helper = Scalar AND phase = Done AND staged = PrimError | OR:757 (select_when) | hm/fragments/scalar.proposed.rs:254 |
| 8 | `BHelperScalarExit` | helper, Scalar, 5 | PRE AND helper = Scalar AND phase = Done AND staged != PrimError | OR:753 (select_when) | hm/fragments/scalar.proposed.rs:217 |
| 9 | `BHelperEnvGetExit` | helper, EnvGet, 1 | PRE AND helper = EnvGet AND status = Positive | OR:781 (select_when) | hm/families/environment.rs:441 |
| 10 | `BHelperEnvGetFailure` | helper, EnvGet, 2 | PRE AND helper = EnvGet AND status = Negative | OR:788 (select_when) | hm/families/environment.rs:442 |
| 11 | `BHelperEnvGetCell` | helper, EnvGet, 3 | PRE AND helper = EnvGet AND status = Pending AND cursor present | OR:765 (select_when) | hm/families/environment.rs:439 |
| 12 | `BHelperEnvGetEmpty` | helper, EnvGet, 4 | PRE AND helper = EnvGet AND status = Pending AND cursor absent | OR:773 (select_when) | hm/families/environment.rs:440 |
| 13 | `BHelperEnvPutInsert` | helper, EnvPut, 1 | PRE AND helper = EnvPut AND status = Pending | OR:795 (select_when) | hm/families/environment.rs:454 |
| 14 | `BHelperEnvPutExit` | helper, EnvPut, 2 | PRE AND helper = EnvPut AND status = Positive | OR:802 (select_when) | hm/families/environment.rs:455 |
| 15 | `BHelperFindExit` | helper, Find, 1 | PRE AND helper = Find AND status = Positive | OR:825 (select_when) | hm/families/environment.rs:475 |
| 16 | `BHelperFindFailure` | helper, Find, 2 | PRE AND helper = Find AND status = Negative | OR:832 (select_when) | hm/families/environment.rs:476 |
| 17 | `BHelperFindRow` | helper, Find, 3 | PRE AND helper = Find AND status = Pending AND row_cursor < admitted_function_count | OR:809 (select_when) | hm/families/environment.rs:467 |
| 18 | `BHelperFindEmpty` | helper, Find, 4 | PRE AND helper = Find AND status = Pending AND row_cursor = admitted_function_count | OR:817 (select_when) | hm/families/environment.rs:474 |
| 19 | `BHelperBindExit` | helper, Bind, 1 | PRE AND helper = Bind AND phase = Finished AND status = Ok | OR:879 (select_when) | hm/families/structural.rs:751 |
| 20 | `BHelperBindFailure` | helper, Bind, 2 | PRE AND helper = Bind AND phase = Finished AND status = Bad | OR:887 (select_when) | hm/families/structural.rs:751 |
| 21 | `BHelperBindPair` | helper, Bind, 3 | PRE AND helper = Bind AND phase = Pairing AND p_cursor < params.len AND a_cursor < args.len | OR:839 (select_when) | hm/families/structural.rs:731 |
| 22 | `BHelperBindEmpty` | helper, Bind, 4 | PRE AND helper = Bind AND phase = Pairing AND p_cursor = params.len AND a_cursor = args.len | OR:843 (select_when) | hm/families/structural.rs:747 |
| 23 | `BHelperBindArity` | helper, Bind, 5 | PRE AND helper = Bind AND phase = Pairing AND (p_cursor = params.len) XOR (a_cursor = args.len) | OR:847 (select_when) | hm/families/structural.rs:747 |
| 24 | `BHelperBindType` | helper, Bind, 6 | PRE AND helper = Bind AND phase = UnwindType | OR:851 (select_when) | hm/families/structural.rs:747 |
| 25 | `BHelperBindDuplicate` | helper, Bind, 7 | PRE AND helper = Bind AND phase = UnwindDup | OR:858 (select_when) | hm/families/structural.rs:747 |
| 26 | `BHelperBindInsert` | helper, Bind, 8 | PRE AND helper = Bind AND phase = UnwindInsert | OR:865 (select_when) | hm/families/structural.rs:748 |
| 27 | `BHelperBindPropagate` | helper, Bind, 9 | PRE AND helper = Bind AND phase = Propagate | OR:872 (select_when) | hm/families/structural.rs:748 |
| 28 | `BHelperAppendCopy` | helper, Append, 1 | PRE AND helper = Append AND phase = Copy AND staged present | OR:895 (select_when) | hm/families/environment.rs:497 |
| 29 | `BHelperAppendCell` | helper, Append, 2 | PRE AND helper = Append AND phase = Copy AND staged absent AND src_cursor < src.len | OR:903 (select_when) | hm/families/environment.rs:494 |
| 30 | `BHelperAppendEmpty` | helper, Append, 3 | PRE AND helper = Append AND phase = Copy AND staged absent AND src_cursor = src.len | OR:907 (select_when) | hm/families/environment.rs:495 |
| 31 | `BHelperAppendTail` | helper, Append, 4 | PRE AND helper = Append AND phase = Tail | OR:911 (select_when) | hm/families/environment.rs:496 |
| 32 | `BHelperAppendExit` | helper, Append, 5 | PRE AND helper = Append AND phase = Done | OR:918 (select_when) | hm/families/environment.rs:498 |
| 33 | `BBookkeepingTaskRelease` | helper, Validate+Equal, 1+1 | PRE AND ((helper = Validate AND Validate.status = Negative) OR (helper = Equal AND Equal.status = Negative)) AND task_live > 0 | OR:1180 (select_when) | hm/families/environment.rs:293 |
| 34 | `BHelperValidateFailure` | helper, Validate, 2 | PRE AND helper = Validate AND status = Negative AND task_live = 0 | OR:977 (select_when) | hm/families/structural.rs:769 |
| 35 | `BHelperValidateExit` | helper, Validate, 3 | PRE AND helper = Validate AND status = Positive | OR:970 (select_when) | hm/families/structural.rs:768 |
| 36 | `BHelperValidateEmpty` | helper, Validate, 4 | PRE AND helper = Validate AND status = Pending AND task_live = 0 | OR:951 (select_when) | hm/families/structural.rs:766 |
| 37 | `BHelperValidateInvalid` | helper, Validate, 5 | PRE AND helper = Validate AND status = Pending AND top task = ValidateTask AND top task visit = Unvisited AND kind_of(top task r) = Marker | OR:960 (select_when) | hm/families/structural.rs:767 |
| 38 | `BHelperValidateOccurrence` | helper, Validate, 6 | PRE AND helper = Validate AND status = Pending AND task_live > 0 AND top task = ValidateTask AND top task visit = Unvisited AND kind_of(top task r) != Marker | OR:925 (select_when) | hm/families/structural.rs:764 |
| 39 | `BHelperValidateField` | helper, Validate, 7 | PRE AND helper = Validate AND status = Pending AND top task = ValidateTask AND top task visit = Fields | OR:936 (select_when) | hm/families/structural.rs:765 |
| 40 | `BHelperEqualExit` | helper, Equal, 2 | PRE AND helper = Equal AND (status = Positive OR (status = Negative AND task_live = 0)) | OR:1050 (select_when) | hm/families/structural.rs:786 |
| 41 | `BHelperEqualEmpty` | helper, Equal, 3 | PRE AND helper = Equal AND status = Pending AND task_live = 0 | OR:1021 (select_when) | hm/families/structural.rs:784 |
| 42 | `BHelperEqualDifferent` | helper, Equal, 4 | PRE AND helper = Equal AND status = Pending AND top task = EqualTask AND top task visit = Unvisited AND shallow_differ(left, right) | OR:1030 (select_when) | hm/families/structural.rs:785 |
| 43 | `BHelperEqualPair` | helper, Equal, 5 | PRE AND helper = Equal AND status = Pending AND top task = EqualTask AND top task visit = Unvisited AND NOT shallow_differ(left, right) | OR:986 (select_when) | hm/families/structural.rs:782 |
| 44 | `BHelperEqualField` | helper, Equal, 6 | PRE AND helper = Equal AND status = Pending AND top task = EqualTask AND top task visit = Fields | OR:1006 (select_when) | hm/families/structural.rs:783 |
| 45 | `BHelperMatchExit` | helper, Match, 1 | PRE AND helper = Match AND phase = Done AND status = Positive | OR:1101 (select_when) | hm/families/structural.rs:828 |
| 46 | `BHelperMatchFailure` | helper, Match, 2 | PRE AND helper = Match AND phase = Done AND status = Negative | OR:1109 (select_when) | hm/families/structural.rs:829 |
| 47 | `BHelperMatchArm` | helper, Match, 3 | PRE AND helper = Match AND phase = Arm | OR:1052 (select_when) | hm/families/structural.rs:799 |
| 48 | `BHelperMatchOccurrence` | helper, Match, 4 | PRE AND helper = Match AND phase = Tag | OR:1059 (select_when) | hm/families/structural.rs:822 |
| 49 | `BHelperMatchDistinct` | helper, Match, 5 | PRE AND helper = Match AND phase = Distinct | OR:1080 (select_when) | hm/families/structural.rs:825 |
| 50 | `BHelperMatchField` | helper, Match, 6 | PRE AND helper = Match AND phase = Field | OR:1066 (select_when) | hm/families/structural.rs:823 |
| 51 | `BHelperMatchBinding` | helper, Match, 7 | PRE AND helper = Match AND phase = Bind | OR:1073 (select_when) | hm/families/structural.rs:824 |
| 52 | `BHelperMatchSelected` | helper, Match, 8 | PRE AND helper = Match AND phase = Ready | OR:1094 (select_when) | hm/families/structural.rs:827 |
| 53 | `BHelperMatchNext` | helper, Match, 9 | PRE AND helper = Match AND phase = NoMatchReady | OR:1087 (select_when) | hm/families/structural.rs:826 |
| 54 | `XAdtHelperPlanExit` | helper, Plan, 1 | PRE AND helper = Plan AND phase = Done AND status = Positive AND arm present | OR:1163 (select_when) | hm/families/adt_plan.rs:73 |
| 55 | `XAdtHelperPlanFailure` | helper, Plan, 2 | PRE AND helper = Plan AND phase = Done AND status = Negative | OR:1172 (select_when) | hm/families/adt_plan.rs:78 |
| 56 | `XAdtHelperPlanWalk` | helper, Plan, 3 | PRE AND helper = Plan AND phase = Walk AND path_cursor < path.len | OR:1133 (select_when) | hm/families/adt_plan.rs:34 |
| 57 | `XAdtHelperPlanSwitch` | helper, Plan, 4 | PRE AND helper = Plan AND phase = Test AND after_walk = Test AND path_cursor = path.len | OR:1137 (select_when) | hm/families/adt_plan.rs:42 |
| 58 | `XAdtHelperPlanBind` | helper, Plan, 5 | PRE AND helper = Plan AND phase = Bind AND after_walk = Bind AND path_cursor = path.len | OR:1141 (select_when) | hm/families/adt_plan.rs:48 |
| 59 | `XAdtHelperPlanSelect` | helper, Plan, 6 | PRE AND helper = Plan AND phase = Ready AND status = Pending AND arm present | OR:1145 (select_when) | hm/families/adt_plan.rs:61 |
| 60 | `XAdtHelperPlanNoMatch` | helper, Plan, 7 | PRE AND helper = Plan AND phase = NoMatchReady AND status = Pending AND arm absent | OR:1154 (select_when) | hm/families/adt_plan.rs:67 |
| 61 | `XAdtHelperBuildField` | helper, Build, 1 | PRE AND helper = Build AND phase = Copy AND staged present AND src_cursor < src.len | OR:1121 (select_when) | hm/families/adt_build.rs:37 |
| 62 | `XAdtHelperBuildCell` | helper, Build, 2 | PRE AND helper = Build AND phase = Copy AND staged absent AND src_cursor < src.len | OR:1117 (select_when) | hm/families/adt_build.rs:31 |
| 63 | `XAdtHelperBuildExit` | helper, Build, 3 | PRE AND helper = Build AND phase = Done AND node present | OR:1125 (select_when) | hm/families/adt_build.rs:56 |
| 64 | `KMachineEvalLiteralValid` | result, EvalLiteral, 1 | PRE AND helper = none AND result present AND site = EvalLiteral AND result = Validated{ok = true} | OR:549 (select_when) | hm/fragments/legacy.proposed.rs:55 |
| 65 | `KMachineEvalLiteralInvalid` | result, EvalLiteral, 2 | PRE AND helper = none AND result present AND site = EvalLiteral AND result = Validated{ok = false} | OR:550 (select_when) | hm/fragments/legacy.proposed.rs:63 |
| 66 | `KMachineEvalVariablePresent` | result, EvalVar, 1 | PRE AND helper = none AND result present AND site = EvalVar AND result = EnvFound | OR:553 (select_when) | hm/fragments/legacy.proposed.rs:68 |
| 67 | `KMachineEvalVariableAbsent` | result, EvalVar, 2 | PRE AND helper = none AND result present AND site = EvalVar AND result = EnvMissing | OR:554 (select_when) | hm/fragments/legacy.proposed.rs:75 |
| 68 | `KMachineReturnLet` | result, RetLet, 1 | PRE AND helper = none AND result present AND site = RetLet AND result = EnvUpdated | OR:573 (select_when) | hm/fragments/legacy.proposed.rs:160 |
| 69 | `KMachineReturnRight` | result, RetRight, 1 | PRE AND helper = none AND result present AND site = RetRight AND result IN {ScalarValue, ScalarError, EqualDone} | OR:574 (select_when) | hm/fragments/legacy.proposed.rs:181 |
| 70 | `KMachineReturnUnary` | result, RetUnary, 1 | PRE AND helper = none AND result present AND site = RetUnary AND result IN {ScalarValue, ScalarError} | OR:582 (select_when) | hm/fragments/legacy.proposed.rs:191 |
| 71 | `KMachineReturnIfTrue` | result, RetIf, 1 | PRE AND helper = none AND result present AND site = RetIf AND result = ScalarSelector{sel = True} | OR:586 (select_when) | hm/fragments/legacy.proposed.rs:201 |
| 72 | `KMachineReturnIfFalse` | result, RetIf, 2 | PRE AND helper = none AND result present AND site = RetIf AND result = ScalarSelector{sel = False} | OR:592 (select_when) | hm/fragments/legacy.proposed.rs:212 |
| 73 | `KMachineReturnIfInvalid` | result, RetIf, 3 | PRE AND helper = none AND result present AND site = RetIf AND result = ScalarSelector{sel = NotBool} | OR:598 (select_when) | hm/fragments/legacy.proposed.rs:223 |
| 74 | `KMachineReturnArgumentsNext` | result, RetArgs, 1 | PRE AND helper = none AND result present AND site = RetArgs AND result = AppendDone AND top frame = ArgsF AND ArgsF.rest.len > 0 | OR:604 (select_when) | hm/fragments/legacy.proposed.rs:238 |
| 75 | `KMachineReturnArgumentsFinal` | result, RetArgs, 2 | PRE AND helper = none AND result present AND site = RetArgs AND result = AppendDone AND top frame = ArgsF AND ArgsF.rest.len = 0 | OR:608 (select_when) | hm/fragments/legacy.proposed.rs:253 |
| 76 | `KMachineReturnCallValid` | result, RetCall, 1 | PRE AND helper = none AND result present AND site = RetCall AND result = ScalarTypeTest{ok = true} | OR:612 (select_when) | hm/fragments/legacy.proposed.rs:266 |
| 77 | `KMachineReturnCallInvalid` | result, RetCall, 2 | PRE AND helper = none AND result present AND site = RetCall AND result = ScalarTypeTest{ok = false} | OR:615 (select_when) | hm/fragments/legacy.proposed.rs:276 |
| 78 | `KMachineInvokeFind` | result, Invoke, 1 | PRE AND helper = none AND result present AND site = Invoke AND result IN {FindFound, FindMissing} | OR:618 (select_when) | hm/fragments/legacy.proposed.rs:290 |
| 79 | `KMachineEnterBind` | result, Enter, 1 | PRE AND helper = none AND result present AND site = Enter AND result IN {BindOk, BindBad} | OR:623 (select_when) | hm/fragments/legacy.proposed.rs:302 |
| 80 | `KMachineCasesTry` | result, Cases, 1 | PRE AND helper = none AND result present AND site = Cases AND origin row = Match{plan = None} AND result IN {MatchMatched, MatchNoMatch} | OR:648 (select_when) | hm/fragments/legacy.proposed.rs:327 |
| 81 | `XAdtCasesPlanHit` | result, Cases, 2 | PRE AND helper = none AND result present AND site = Cases AND ctl = Cases AND origin row = Match{plan = Some} AND result = PlanMatched | OR:719 (select_when) | hm/families/adt_constructor.rs:85 |
| 82 | `XAdtCasesPlanMiss` | result, Cases, 3 | PRE AND helper = none AND result present AND site = Cases AND ctl = Cases AND origin row = Match{plan = Some} AND result = PlanNoMatch | OR:727 (select_when) | hm/families/adt_constructor.rs:97 |
| 83 | `XAdtReturnCtorFieldsTyped` | result, RetCtorFields, 1 | PRE AND helper = none AND result present AND site = RetCtorFields AND top frame = ConstructorFieldsF{checked = false} AND result = ScalarTypeTest{ok = true} | OR:695 (select_when) | hm/families/adt_constructor.rs:38 |
| 84 | `XAdtReturnCtorFieldsMistyped` | result, RetCtorFields, 2 | PRE AND helper = none AND result present AND site = RetCtorFields AND top frame = ConstructorFieldsF{checked = false} AND result = ScalarTypeTest{ok = false} | OR:701 (select_when) | hm/families/adt_constructor.rs:44 |
| 85 | `XAdtReturnCtorFieldsNext` | result, RetCtorFields, 3 | PRE AND helper = none AND result present AND site = RetCtorFields AND top frame = ConstructorFieldsF{checked = true} AND rest.len > 0 AND result = AppendDone | OR:707 (select_when) | hm/families/adt_constructor.rs:50 |
| 86 | `XAdtReturnCtorFieldsFinal` | result, RetCtorFields, 4 | PRE AND helper = none AND result present AND site = RetCtorFields AND top frame = ConstructorFieldsF{checked = true} AND rest.len = 0 AND result = AppendDone | OR:711 (select_when) | hm/families/adt_constructor.rs:66 |
| 87 | `XAdtMaterializeDone` | result, Materialize, 1 | PRE AND helper = none AND result present AND site = Materialize AND ctl = Materialize AND result = BuildDone | OR:715 (select_when) | hm/families/adt_constructor.rs:79 |
| 88 | `BHelperScalarEntry` | plan-entry, Scalar, 1 | PRE AND helper = none AND result = none AND (site = RetRight AND RightF.op != Eq) OR site IN {RetUnary, RetIf, RetCall} OR (site = RetCtorFields AND ConstructorFieldsF.checked = false) | OR:1761 (stage) | hm/fragments/scalar.proposed.rs:55; RetCtorFields site: hm/families/adt_constructor.rs:113 (via hm/families/scalar.rs:11) |
| 89 | `BHelperEnvGetEntry` | plan-entry, EnvGet, 1 | PRE AND helper = none AND result = none AND site = EvalVar | OR:1761 (stage) | hm/families/environment.rs:438 |
| 90 | `BHelperEnvPutEntry` | plan-entry, EnvPut, 1 | PRE AND helper = none AND result = none AND site = RetLet | OR:1761 (stage) | hm/families/environment.rs:453 |
| 91 | `BHelperFindEntry` | plan-entry, Find, 1 | PRE AND helper = none AND result = none AND site = Invoke | OR:1761 (stage) | hm/families/environment.rs:466 |
| 92 | `BHelperBindEntry` | plan-entry, Bind, 1 | PRE AND helper = none AND result = none AND site = Enter AND Enter.function present | OR:1761 (stage) | hm/families/structural.rs:720 |
| 93 | `BHelperAppendEntry` | plan-entry, Append, 1 | PRE AND helper = none AND result = none AND site = RetArgs OR (site = RetCtorFields AND ConstructorFieldsF.checked = true) | OR:1761 (stage) | hm/families/environment.rs:487; RetCtorFields site: hm/families/adt_constructor.rs:137 |
| 94 | `BHelperValidateEntry` | plan-entry, Validate, 1 | PRE AND helper = none AND result = none AND site = EvalLiteral | OR:1761 (stage) | hm/families/structural.rs:763 |
| 95 | `BHelperEqualEntry` | plan-entry, Equal, 1 | PRE AND helper = none AND result = none AND site = RetRight AND RightF.op = Eq | OR:1761 (stage) | hm/families/structural.rs:781 |
| 96 | `BHelperMatchEntry` | plan-entry, Match, 1 | PRE AND helper = none AND result = none AND site = Cases AND origin row = Match{plan = None} AND Cases.arms.len > 0 | OR:1761 (stage) | hm/families/structural.rs:798 |
| 97 | `XAdtHelperPlanEntry` | plan-entry, Plan, 1 | PRE AND helper = none AND result = none AND site = Cases AND origin row = Match{plan = Some} | OR:1761 (stage) | hm/families/adt_plan.rs:15 |
| 98 | `XAdtHelperBuildEntry` | plan-entry, Build, 1 | PRE AND helper = none AND result = none AND site = Materialize | OR:1761 (stage) | hm/families/adt_build.rs:16 |
| 99 | `KMachineEvalLet` | legacy, EvalLet, 1 | PRE AND helper = none AND result = none AND site = EvalLet AND owner_helper_plan(EvalLet) = none AND true (select_when = true) | OR:555 (select_when) | hm/fragments/legacy.proposed.rs:80 |
| 100 | `KMachineEvalBinary` | legacy, EvalBinary, 1 | PRE AND helper = none AND result = none AND site = EvalBinary AND owner_helper_plan(EvalBinary) = none AND true (select_when = true) | OR:555 (select_when) | hm/fragments/legacy.proposed.rs:90 |
| 101 | `KMachineEvalUnary` | legacy, EvalUnary, 1 | PRE AND helper = none AND result = none AND site = EvalUnary AND owner_helper_plan(EvalUnary) = none AND true (select_when = true) | OR:555 (select_when) | hm/fragments/legacy.proposed.rs:100 |
| 102 | `KMachineEvalIf` | legacy, EvalIf, 1 | PRE AND helper = none AND result = none AND site = EvalIf AND owner_helper_plan(EvalIf) = none AND true (select_when = true) | OR:555 (select_when) | hm/fragments/legacy.proposed.rs:109 |
| 103 | `KMachineEvalMatch` | legacy, EvalMatch, 1 | PRE AND helper = none AND result = none AND site = EvalMatch AND owner_helper_plan(EvalMatch) = none AND true (select_when = true) | OR:556 (select_when) | hm/fragments/legacy.proposed.rs:119 |
| 104 | `KMachineEvalCallEmpty` | legacy, EvalCall, 1 | PRE AND helper = none AND result = none AND site = EvalCall AND owner_helper_plan(EvalCall) = none AND ctl = Eval AND expr row = Call AND Call.args.len = 0 | OR:559 (select_when) | hm/fragments/legacy.proposed.rs:129 |
| 105 | `KMachineEvalCallArguments` | legacy, EvalCall, 2 | PRE AND helper = none AND result = none AND site = EvalCall AND owner_helper_plan(EvalCall) = none AND ctl = Eval AND expr row = Call AND Call.args.len > 0 | OR:565 (select_when) | hm/fragments/legacy.proposed.rs:136 |
| 106 | `KMachinePushAccept` | legacy, Push, 1 | PRE AND helper = none AND result = none AND site = Push AND owner_helper_plan(Push) = none AND depth < stack_limit | OR:571 (select_when) | hm/fragments/legacy.proposed.rs:148 |
| 107 | `KMachinePushRefuse` | legacy, Push, 2 | PRE AND helper = none AND result = none AND site = Push AND owner_helper_plan(Push) = none AND depth >= stack_limit | OR:572 (select_when) | hm/fragments/legacy.proposed.rs:158 |
| 108 | `KMachineReturnLeft` | legacy, RetLeft, 1 | PRE AND helper = none AND result = none AND site = RetLeft AND owner_helper_plan(RetLeft) = none AND true (select_when = true) | OR:556 (select_when) | hm/fragments/legacy.proposed.rs:172 |
| 109 | `KMachineReturnMatch` | legacy, RetMatch, 1 | PRE AND helper = none AND result = none AND site = RetMatch AND owner_helper_plan(RetMatch) = none AND true (select_when = true) | OR:556 (select_when) | hm/fragments/legacy.proposed.rs:228 |
| 110 | `KMachineReturnScope` | legacy, RetScope, 1 | PRE AND helper = none AND result = none AND site = RetScope AND owner_helper_plan(RetScope) = none AND true (select_when = true) | OR:556 (select_when) | hm/fragments/legacy.proposed.rs:281 |
| 111 | `KMachineEnterMissing` | legacy, Enter, 1 | PRE AND helper = none AND result = none AND site = Enter AND owner_helper_plan(Enter) = none AND ctl = Enter{function = None} | OR:622 (select_when) | hm/fragments/legacy.proposed.rs:300 |
| 112 | `KMachineBoundInvalid` | legacy, Bound, 1 | PRE AND helper = none AND result = none AND site = Bound AND owner_helper_plan(Bound) = none AND ctl = Bound{binding = Bad} | OR:627 (select_when) | hm/fragments/legacy.proposed.rs:314 |
| 113 | `KMachineBoundValid` | legacy, Bound, 2 | PRE AND helper = none AND result = none AND site = Bound AND owner_helper_plan(Bound) = none AND ctl = Bound{binding = Ok} | OR:634 (select_when) | hm/fragments/legacy.proposed.rs:316 |
| 114 | `KMachineCasesEmpty` | legacy, Cases, 1 | PRE AND helper = none AND result = none AND site = Cases AND owner_helper_plan(Cases) = none AND ctl = Cases AND Cases.arms.len = 0 AND origin row = Match{plan = None} | OR:641 (select_when) | hm/fragments/legacy.proposed.rs:325 |
| 115 | `KMachineCasesMiss` | legacy, CaseResult, 1 | PRE AND helper = none AND result = none AND site = CaseResult AND owner_helper_plan(CaseResult) = none AND ctl = CaseResult{matching = NoMatch} | OR:657 (select_when) | hm/fragments/legacy.proposed.rs:339 |
| 116 | `KMachineCasesHit` | legacy, CaseResult, 2 | PRE AND helper = none AND result = none AND site = CaseResult AND owner_helper_plan(CaseResult) = none AND ctl = CaseResult{matching = Matched} | OR:664 (select_when) | hm/fragments/legacy.proposed.rs:345 |
| 117 | `KMachinePrimitiveValue` | legacy, Prim, 1 | PRE AND helper = none AND result = none AND site = Prim AND owner_helper_plan(Prim) = none AND ctl = PrimResult{result = Value} | OR:671 (select_when) | hm/fragments/legacy.proposed.rs:353 |
| 118 | `KMachinePrimitiveError` | legacy, Prim, 2 | PRE AND helper = none AND result = none AND site = Prim AND owner_helper_plan(Prim) = none AND ctl = PrimResult{result = Error} | OR:677 (select_when) | hm/fragments/legacy.proposed.rs:359 |
| 119 | `XAdtEvalConstructNullary` | legacy, EvalConstruct, 1 | PRE AND helper = none AND result = none AND site = EvalConstruct AND owner_helper_plan(EvalConstruct) = none AND ctl = Eval AND expr row = Construct AND fields.len = 0 AND constant present | OR:683 (select_when) | hm/families/adt_constructor.rs:14 |
| 120 | `XAdtEvalConstructFields` | legacy, EvalConstruct, 2 | PRE AND helper = none AND result = none AND site = EvalConstruct AND owner_helper_plan(EvalConstruct) = none AND ctl = Eval AND expr row = Construct AND fields.len > 0 AND constant absent | OR:689 (select_when) | hm/families/adt_constructor.rs:19 |

### 3.5 The control word layout (`dec_key`, `dec_val`)

The DEC message is typed by FP-1 rules: each part is its own lane, so nothing is packed. That replaces the packed `dec_val` of the halo2 text (`ARCH:868`).

| Lane | Type | Value | Source |
|---|---|---|---|
| `dec_key` = class | `tag(7)` | 0 idle, `ordinal(r)` for rule `r` (3.4) | the network, C3-1; C3-2 for the type |
| `rule_lane` | `tag(7)` | equal to the class (recommended, O3-1) | |
| `microplan_id` | `u16` | per-rule plan id | OPEN O3-2 |
| `write_set_id` | `u16` | per-rule write set id | OPEN O3-2 |
| `flags` | `n_f` lanes of `bit` | per-rule booleans | OPEN O3-2 |

**Choice C3-2, one-word parts typed `tag(7)` and `bit`.** FP-1 types the DEC parts as `u16` (`ARCH:336`). Each lane here is one word, as a `u16` part is, so the message length does not change. The narrower types give RANGE `(x, 7)` and boolean checks at the lane. Alternative: every part `u16`, as FP-1 writes it, with the DEC table membership as the only bound. Reason: a class above 127 or a flag above 1 is then refused at the lane, before the table lookup. The ARCH DEC row then reads "one-word parts" in place of "`u16` parts".

The message has `4 + n_f` words, plus the bus index. The table has 121 rows: the idle row `(0, 0, ...)` and one row per rule (`ARCH:865`, `:873`).

- **O3-1, `rule_lane`.** The code already defines a lane: the low 128 bits of SHA-256 of a rule id string (`crates/mori-backend/src/sherlock_c6_child.rs:432-436`). The callers do not use one id form: the witness stream passes the `RuleIdC` debug name (`crates/mori-backend/src/sherlock_c6_witness_stream.rs:93-97`), and a test passes `"AppendHelper"` (`crates/mori-backend/tests/sherlock_c6_child.rs:118`). Over BabyBear a 128-bit lane is a `u128`, 8 words. Options: (a) `rule_lane = class`, a `tag(7)` that is non-zero and distinct by construction; (b) keep the SHA-256 lane as `u128`, with one fixed id form. Recommendation (a). `rules_d` hashes the lanes (`ARCH:1049`) and AD-23 puts `rules_d` into `shape_d`, so (a) changes the lane values but no digest form. Under (b), `validate_rom` must check that the 120 truncated hashes are distinct and non-zero.
- **O3-2, `microplan_id`, `write_set_id`, `flags`.** No source table exists. A search of `crates/` finds none of the names, and the only per-rule flags are those of the three-rule spike (`crates/mori-chip/src/leaf.rs:66-70`). The values must be generated from each rule family's `RuleContract` (`ARCH:1115-1122`). They are fixed when CORE's rule families are built, so this part of I-7 cannot freeze before that. Recommendation: freeze the key and the rule lane now, and freeze the three plan lanes with the `RuleContract` table of each family in T6.
- **O3-3, the `BHelperMatchArm` guard** (backend delta 1). Recommendation: the core guard, because it is the reference, with the family refusal (`families/structural.rs:799-806`) as the plan's check.
- **O3-4, residue states.** Where a scan finds no rule (helper group exhausted, stale result, legacy group exhausted, undefined site), the reference raises an error and selects nothing. An enabled slot in such a state must be unsatisfiable. The script does not enumerate these cases; that is the `qnt/decDispatch.qnt` task (`ARCH:883`).

The `GuardId`/`eval_guard` guards (`OR:412-499`) are not selection guards: `owner_select_when` ignores them (`OR:544`). They branch effects inside families and belong to the micro-operation plan.

### 3.6 Closure test (I-7)

- Test `dec_guard_list_vs_reference` (new): for every step of the X1/X15 corpus (6,669 steps, MEASURED, `X15`), and for constructed cases of the 18 rules that did not fire on the corpus (REFERENCE BRIDGE), the class computed from the step's pre-state equals the ordinal of the rule that `owner_select` selects. A residue state gives no satisfiable enabled slot.
- Positives (`ARCH:884`): `dec_every_enabled_rule_roundtrip`, `dec_publication_with_zero_fuel`, `dec_helper_vs_result_present`, `dec_comparator_boundaries`, `dec_disabled_slot_idle_row`, `dec_partial_leaf_same_key`.
- Refused mutants (`ARCH:885`): `dec_priority_swapped` (two rules of the Bind group swapped), `dec_guard_unlinked`, `dec_wrong_instruction_source`, `dec_two_rules_selected`, `dec_enabled_without_rule` (a residue state), `dec_metadata_forged`, `dec_rule_lane_zero`, `dec_unknown_class_accepted` (class 121), `dec_disabled_rule_query`, `dec_extension_rule_query`, `dec_disabled_slot_nonzero_lane`, `dec_duplicate_key_at_load`, `dec_rule_with_key_zero_at_load`. New: `dec_site_skipped_on_helper` (a helper rule selected at an Eval control without the expression-row read). `dec_wrap_forgery` is not reachable once nothing is packed, and it stays as a control.
- Quint `qnt/decDispatch.qnt` with the residue enumeration of O3-4.

### 3.7 Lines that change

- ARCH FP-1 bus table, DEC row (`ARCH:336`): the layout of 3.5. Kit DEC: `ARCH:865-869` (class, unpacked lanes, the rule inventory as in 3.4) and `ARCH:919` (dependencies: the guard list is done). AD-9 row (`ARCH:3446`).
- ARCH 1.12 I-7 (`ARCH:551`) and the DEC kit row (`ARCH:590`). After sign-off the status reads FROZEN for the key and the rule lane, and FREEZE-PENDING O3-2 for the plan lanes.
- `ML/specs/sherlock-chip-dec/spec.md`: the class and the lane table. Lines: `:4` (DEC-001, "its `rule_lane` and bounded control parts": name the 3.5 lanes), `:22` (DEC-003, "so that the packing is injective": nothing is packed, so the requirement reads "each part is ranged at its lane", C3-2), `:24` (scenario "Wrap forgery": kept as a control only, 3.6), `:33` (DEC-004, lane 0: under O3-1 (a) the lane is the class, which is non-zero by construction). `ML/specs/sherlock-reference-bridge/spec.md:22` (the 120 rules: unchanged, the inventory matches). `ML/specs/sherlock-chip-bus/spec.md:48` (the idle row: unchanged).
- ARCH section 5.2: the DEC lines `ARCH:3636-3638` (tagged table shared with RANGE: unchanged by this draft).

## 4. `D_view` and the view form (I-10; AD-17 (b); kits CORE read port, INIT, NODE INTEGRATION VIEW-WORK)

Owner ruling: "AD-17: by-digest admission with an empty genesis memory" (`DA` line 66). The open points that ARCH leaves: the depth `D_view`, whether a lazy view is allowed beside the full view, and the I7 question (`ARCH:2244`, `:3454`). The view tree is defined at `ARCH:2240-2245`; `read_input` at `ARCH:1027`; the `AuthIn` port at `ARCH:139`. X11 (`ARCH:4007`) has not run: no result file exists in the worktrees (search of `~/Projects/Moriarty-wt/*/research/` for X11 results, none).

### 4.1 The view tree

**Choice C4-1, one record per distinct `vd` (a hash-consed view).** A record describes one distinct sub-value; two child-vd leaves may point to the same record. Alternative: one record per tree position of the unfolded value. Reason: the unfolded tree of a value with sharing can be exponentially larger than its stored form (`Cons(y, y)` nested `n` times is `n + 1` stored nodes and `2^(n+1) - 1` tree positions), so no fixed depth covers it and the view would bound the input (I2, `ARCH:2787`). The VIEW-WORK relation allows sharing as written: it requires `child_pos` to be a running count and every index below `n_rec` (`ARCH:2242`), and does not require distinct targets.

Leaves (`P2` preimages, types of FP-1):

```text
record leaf  = P2(DOM_VIEW_REC, idx: addr, form: tag(4), ty_d: digest, c: c9, m: u32,
                  vd: digest, child_pos: addr)                                  39 words, 5 permutations
child leaf   = P2(DOM_VIEW_CHILD, pos: addr, vd_k: digest, entered: bit, idx_k: addr)
                                                                                20 words, 3 permutations
node         = P2(DOM_VIEW_NODE, l: digest, r: digest)                         17 words, one HASH request
empty leaf   = P2(DOM_VIEW_NODE)   (tag only; empty subtree E_(h+1) = node(E_h, E_h))
```

- `form` is the `vd` form of FP-4 (`tag(4)`, `ARCH:464`), including form 8 `Fr`, because a GP-2 input can be a Compact outcome (FP-3 fact 1).
- `c: c9` is 9 words: `fr` for form 8; `u128` in words 0 to 7 with word 8 zero for `UInt`; `bit` in word 0 for `Bool`; the constructor ordinal `c: u32` in words 0 to 1 (FP-4 `vd` completion row); zero otherwise. The `form` lane fixes the reading.
- `entered = 0` with `idx_k = 0` is `NOT_ENTERED` (`ARCH:2240`).
- Tags: `DOM_VIEW_REC` = 42, `DOM_VIEW_CHILD` = 43, `DOM_VIEW_NODE` = 44 (LEAD picks, next free codes). The node has no height word, as in C2-5: fixed depth, and leaf lengths (39, 20) differ from the node length (17).
- Position: the top path bit is the leaf kind (0 record, 1 child-vd leaf), and the low 66 bits are `idx` or `pos`. **Choice C4-2, one tree with a kind bit.** Alternative: two trees and two roots. Reason: `vw_d = P2(SHK_VIEW, 1, val_d, view_root)` has one root word (FP-4, `ARCH:470`), and the I-14 row is settled.

### 4.2 `D_view`

**`D_view = TOP + 4 = 67`**: 66 index bits, the `addr` width of I-2, plus the kind bit. DERIVED from the choice of `addr` as the index type.

- What it covers: every value with fewer than `2^66` distinct sub-values and fewer than `2^66` child slots. A value that a run produces (the GP-2 input link) is made of DATA and FIELDS cells of one arena of fewer than `2^66` cells (TOP = 63) and of PROG constants; so the bound can bind only for a value whose distinct sub-values, memory cells and program constants together, reach `2^66`. At `D_row = 64` the constants of one program are fewer than `2^64` rows, so that case needs a producer memory filled to more than three quarters with DATA cells of one value (DERIVED from `2^66 - 2^64`).
- OPEN O4-1: (a) `D_view = 67` with `addr` indices (recommended: no new word type, the same envelope as the memory of AD-1); (b) `D_view = 68` with a 67-bit index, which closes that corner but needs a new word type, an edit of I-1, which is FROZEN; (c) `u64` indices, `D_view = 65`, which is below the memory envelope and narrows GP-2.
- Cost, ESTIMATE (named inputs: `P2` of pick P-3, one HASH request per node): one `read_input` of an entered field is a child-leaf opening (3 + 67 x 3 permutations) and a record opening (5 + 67 x 3), about 410 permutations; about 142 if O2-2 gives a one-permutation node. X11 measures it.

### 4.3 `AuthIn` port cells (I-10)

`entered: bit, idx: addr, form: tag(4), ty_d: digest, c: c9, m: u32, vd: digest, child_pos: addr`: 39 words (DERIVED), plus `seq: u32` across AIRs. With `entered = 0` only `vd` is set and the other lanes are zero: this is the token of a field whose subtree is carried by digest. `read_input` refuses a token with `entered = 0` as `src` (no record to read under it). The certificate is that of `ARCH:139`, with "against the `view_root` header word" (FP-5 wins over the "body word" wording at `ARCH:139`, `:1027`, `:3454`).

### 4.4 The view form (full or lazy)

The circuit relation does not distinguish the two forms: a full view is a lazy view in which no child-vd leaf is `NOT_ENTERED`. The decision is whether VIEW-WORK accepts `NOT_ENTERED`.

- **Recommendation: lazy allowed** (VIEW-WORK accepts `NOT_ENTERED`), with a host canonical rule: the canonical view of a run is the smallest hash-consed view that contains record 0 and every record the reference run reads, in breadth-first order from record 0. It is a function of `(prog, input, fuel, stack_limit)`, because the reference run is deterministic.
- Alternative: full view only (VIEW-WORK refuses `NOT_ENTERED`).
- Reasons: under full-only, VIEW-WORK work grows with the whole input even when the run reads one field (a head read of a long list). Under lazy, it grows with the part that the run reads. Soundness is the same (`ARCH:2244`). A lazy `view_root` depends on the prover's choice, so boundary digests of two proofs of one run are equal at a common cut only if both use the same view; the public statement does not contain `view_root` (`claim_d = P2(SHK_RUN, 1, prog_d, input_d, outcome_d)`, I-18), so it is independent of the view. The host canonical rule makes the boundary digests equal for every partition that HOST CHECKPOINT STORE produces.
- Coupling with I7 (owner question, `ARCH:2792`): lazy works with I7 "inspected nodes only" (the kit INIT recommendation). If the owner chooses the whole-input check `ty(x) == in_ty`, VIEW-WORK must enter every node, and the full view is required. So the view form follows the I7 answer.

### 4.5 Closure test (I-10)

- Test `view_schema_vectors` (new): the leaf words, the empty-subtree constants and `view_root` for `Cons(UInt 1, Nil)` (full view, 3 records) and for its lazy view with the tail `NOT_ENTERED`, by the Rust reference and the AIR.
- Positives: `pos_view_full_and_lazy_same_claim` (new: both views of one input seal; `claim_d` and `claim_pub` are equal; `view_root` differs); `pos_view_shared_subvalue` (new: `Cons(x, x)` with one record for `x`); `pos_view_wide_ctor` (new: a constructor of 9 fields, a step reads field 8 with one `read_input`, the X11 case of `ARCH:4007`); `init_main_entry_with_input` (`ARCH:2808`).
- Refused mutants: existing `in_read_outside_view`, `view_record_forged`, `view_claim_before_done`, `seal_view_other_input`, `seal_view_root_forged`, `view_root_written_by_rule` (`ARCH:2258`, `:505`); new `view_read_not_entered` (a step enters a `NOT_ENTERED` child), `view_leaf_kind_splice` (a child-vd leaf opened at a record position), `view_path_length_short` (66 levels).

### 4.6 Lines that change

- ARCH 1.3 `AuthIn` row (`ARCH:139`): the 39-word layout, "header word". Kit CORE `read_input` (`ARCH:1027`): depth 67, "header word". VIEW-WORK (`ARCH:2240`, `:2244`): the leaf forms, C4-1, C4-2, the form rule. AD-17 (`ARCH:3454`): close `D_view` and the view form, keep I7 with the coupling. Kit INIT (`ARCH:2794`): the open points. FP-4 tag list (`ARCH:454`): codes 42 to 44.
- ARCH 1.12 I-10 (`ARCH:554`), kit rows CORE (`ARCH:592`, `D_view` part) and INIT (`ARCH:607`, first part).
- `ML/tasks.md:36` (F.3, there OD-17): the input-admission item; its gate also names X11, which has not run. `ML/specs/sherlock-init-admission/spec.md` and `ML/specs/sherlock-node-integration/spec.md`: VIEW-WORK leaves and depth. `O/design.md:194-196` and `:218`: the input enters by digest (section 6).
- OpenSpec lines: `ML/specs/sherlock-init-admission/spec.md:48` (INIT input forms, I1 to I6: unchanged, the view meets them) and `:55` (the scenario of an input part offered before OD-17 and X11: the view form is decided, X11 stays the gate); `ML/specs/sherlock-init-admission/spec.md:37` and `:44` (genesis records: under AD-17 (b) the genesis memory is empty). `ML/specs/sherlock-node-integration/spec.md` has no view text to change; VIEW-WORK needs a new requirement there when T6 writes the node specs.

## 5. The `ty_d` normal form inside `decl_schema_d` (I-14; AD-14)

Owner ruling: "AD-14: program-independent `ty_d`" (`DA` line 65); the normal form is written before key freeze (`ARCH:3451`). Sources: dissent D-2 (`O/dissent.md:204-208`); Carbon's form `ty_d(Adt(decl_schema_d, [A_0..A_(p-1)])) = H(DOM_TY, 4, decl_schema_d, p, ty_args_d)` with `decl_schema_d` "the canonical Poseidon digest of the ADT declaration's normalized constructor signatures (independent of program-local `TypeIdC` row position)" (`research/sherlock-c6-outcome-ledger9-review-20261110/review.md:60`, `carbon/answer.md:45-49`); a raw `TypeIdC` is canonical only inside one program (`hm/shared/adt.rs:6-13`); ADMIT-WORK check (2) and (4) (`ARCH:2234`).

### 5.1 The source objects

A declaration is `TypeDeclSrc { name, parameters: Vec<String>, ctors: Vec<CtorDeclSrc> }`, a constructor `CtorDeclSrc { name, fields: Vec<CtorFieldSrc> }`, a field `CtorFieldSrc { name, ty: TypeExprSrc }` (`hm/shared/owner_ext_type_builder.rs:46-58`). A field type is `UInt | Bool | List | Tree | Named { name } | Apply { name, args } | Param { name }` (`:17-32`). Type names are unique in a program (`ExtTypeRejectReason::DuplicateTypeName`, `:116`, `:255`, `:272`), as are constructor and field names inside their scope (`:121-122`).

### 5.2 The normal form

All digests are `P2` (FP-4). Tags `DOM_TY` = 45, `DOM_TY_ARGS` = 46, `DOM_TYEXPR` = 47, `DOM_DECL` = 48, `DOM_DECL_SCC` = 49, `DOM_DECL_CTOR` = 50, `DOM_CTOR_FIELD` = 51, `DOM_NAME` = 52, `DOM_DECL_SCC0` = 53, `DOM_DECL_CTOR0` = 54, `DOM_CTOR_FIELD0` = 55, `DOM_FIELD_PAIR` = 56, `DOM_TE_ARGS` = 57 (LEAD picks, next free codes after section 4). Each role has its own tag: a chain head, a chain step, a field pair and each kind of argument list (choice C5-5). The count and position words `p`, `nc`, `m`, `n`, `j`, `i` and the `name_d` length are written `u32` here; they follow O2-1 (`u64` at (b)).

```text
name_d(s)           = P2(DOM_NAME, len: u32, s[0], ..., s[len-1])        one byte per word (pick P-4)

ty_d(UInt)          = P2(DOM_TY, 0: tag(3))      ty_d(Bool) = P2(DOM_TY, 1)
ty_d(List)          = P2(DOM_TY, 2)              ty_d(Tree) = P2(DOM_TY, 3)
ty_d(D<A_0..A_p-1>) = P2(DOM_TY, 4, decl_d(D), p: u32, args_d)
args_d              = P2(DOM_TY_ARGS, ty_d(A_0), ..., ty_d(A_(p-1)))      (tag only when p = 0)

decl_d(D)           = P2(DOM_DECL, scc_d(G), j: u32)      D is member j of its group G in canonical order
scc_d(G)            = chain over the members of G in canonical order:
                        s_0 = P2(DOM_DECL_SCC0, n: u32)
                        s_(i+1) = P2(DOM_DECL_SCC, s_i, sch_d(D_i))
sch_d(D)            = chain over the constructors of D in source order:
                        k_0 = P2(DOM_DECL_CTOR0, name_d(D.name), p: u32, nc: u32)
                        k_(c+1) = P2(DOM_DECL_CTOR, k_c, ctor_d(C_c))
ctor_d(C)           = chain over the fields of C in source order:
                        f_0 = P2(DOM_CTOR_FIELD0, name_d(C.name), m: u32)
                        f_(i+1) = P2(DOM_CTOR_FIELD, f_i, P2(DOM_FIELD_PAIR, name_d(F_i.name), te_d(F_i.ty)))

te_d(UInt | Bool | List | Tree) = P2(DOM_TYEXPR, 0 | 1 | 2 | 3)
te_d(Param name)    = P2(DOM_TYEXPR, 5, i: u32)            i = position of name in D.parameters
te_d(ref to E, args), E in G   = P2(DOM_TYEXPR, 6, j_E: u32, targs_d)
te_d(ref to E, args), E not in G = P2(DOM_TYEXPR, 7, decl_d(E), targs_d)
targs_d             = P2(DOM_TE_ARGS, te_d(a_0), ..., te_d(a_(q-1)))      Named is Apply with q = 0
```

- `G` is the strongly connected component of `D` in the graph "declaration X names declaration Y in a field type". The canonical order of `G` is ascending lexicographic order of the name bytes; names are unique (5.1), so the order is total.
- `decl_d(D)` is a function of the declarations reachable from `D` only: the members of `G` are reachable from `D`, and every `E` outside `G` that `D` names is hashed through `decl_d(E)`, which is computed first (the components form a DAG). So two programs that contain the same reachable declarations give the same `decl_d(D)` and the same `ty_d`, which is the program-independence the ruling asks for. Program-local indices (`TypeIdC`, `CtorIdC`, row positions, name-table indices) enter no preimage.
- Recursion: a reference inside `G` is by canonical position `j_E`, so a self-reference or a mutual reference needs no digest of itself. Every chain step is a fixed-length `P2`, so the hash is well founded.

**Choices.**

- C5-1, components hashed as a group. Alternative: reference other declarations by `name_d` only (no group). Reason: with names only, a declaration `Bar` with a field of type `Foo` has one `decl_d` in two programs whose `Foo` differ, so `Bar` values of one program pass the `in_ty` check of the other with a different field type. The group form makes `decl_d(Bar)` depend on `Foo`'s declaration.
- C5-2, names in the preimage (type, constructor and field names). Alternative: structural form without names. Reason: the language is nominal (unique names, `Named` references); two declarations with equal shapes and different names are different types, and a value carries only the ordinal `c` (FP-4 `vd` row), so a structural `ty_d` would let `A | B` of one program stand for `X | Y` of another.
- C5-3, chains of two-input `P2` steps for the schema. Alternative: one flat sponge over the whole group. Reason: ADMIT-WORK works in bounded units (`ARCH:2234`); a chain step is one HASH request with a fixed preimage, so a unit can stop between any two steps and carry one digest. A flat sponge over a group of unbounded size cannot be split without carrying the 16-word sponge state.
- C5-4, Carbon's builtin and ADT forms kept, with `DOM_TY` codes 0 to 4 (`carbon/answer.md:45-49`).
- C5-5, one tag per role. Alternative: one tag per chain, with the roles told apart by preimage length (premise A5). Reason: the lengths do not always differ. The step `f_(i+1)` and the pair inside it are both 17 words, and at O2-1 (b) the head `k_0` (1 + 8 + 4 + 4) and the step `k_(c+1)` (1 + 8 + 8) are both 17 words. A tag per role costs no word.

### 5.3 Where it is checked

- ROM rows: `types` row `ty_d: digest` (section 2.2, table 9), `decl_templates` row `decl_d: digest` (table 15), `names` row `name_d` (table 21).
- ADMIT-WORK check (2) for each `types` row with `decl = Some(t)`: `ty_d = P2(DOM_TY, 4, decl_d(t), p, args_d)` with `args_d` over the `ty_d` of the `type_args` rows of the row's `args` slice; each arg row is checked before it (check (4)). For each `decl_templates` row: recompute `decl_d` from the template as a witness, chain step by chain step, carrying the chain digest across units. For each `constructors` and `ctor_fields` row of a type instance: the constructor count, the arity and each field's `ty_d` equal the template's field type `te_d` instantiated with the instance arguments. This ties the program's own tables to `decl_d`; without it a program could claim the `ty_d` of another declaration.
- VALUE DIGEST reads `ty_d` through `AuthRom` from table 9 (`ARCH:1863`); `in_ty` in `image_d` is a `ty_d` (FP-4, `ARCH:462`).

### 5.4 OPEN items

- O5-1, the order of `types` rows. Check (4) needs every argument row before the rows that use it. Whether the owner's type builder (T1/T2 FIFO, `hm/shared/owner_ext_type_builder.rs:1-3`) already emits arguments first is not read here. Options: (a) require it as a PROG canonicality rule (the admission refuses another order); (b) let ADMIT-WORK walk rows in term-height order from a witness permutation. Recommendation (a).
- O5-2, the ADMIT-WORK carry. `adm_state_d = H(SHK_ADM_STATE, prog_d, prog_root, tbl, row)` (`ARCH:2232`) has no field for a chain digest or for the template position. It needs `acc: digest` and a position `(member, ctor, field): 3 x u32`. This changes the AW payload preimage of kit NODE INTEGRATION (I-13 keeps one `adm_state_d` word).
- O5-3, polymorphic recursion. If a declaration names itself with different arguments, the `types` table of a program is still finite, but this draft does not show that the builder refuses an infinite instance family. The normal form does not depend on it.
- O5-4, the `Fr` builtin. FP-3 fact 1 added the `vd` form `Fr` (`ARCH:464`); no source gives `Fr` a type row. If a program can declare a field of type `Fr`, `ty_d(Fr) = P2(DOM_TY, 5)` and `te_d(Fr) = P2(DOM_TYEXPR, 4)` are the next codes; otherwise nothing is added.

### 5.5 Closure test (I-14)

- Test `ty_d_normal_form_vectors` (new): `ty_d` of the four builtins; of `Option<UInt>`; of a self-recursive `Nat = Z | S(Nat)`; of a mutually recursive pair `Even = E0 | ES(Odd)`, `Odd = OS(Even)`; each by the Rust reference and the AIR of the ADMIT-WORK unit.
- Positives: `pos_ty_d_program_independent` (new: one declaration in two programs with different table order, unrelated extra declarations and different `TypeIdC` values gives one `ty_d`; a value of program A passes `in_ty` of program B, the GP-2 acceptance (a) case); `rom_ty_d_matches_admission` (`ARCH:1560`, host differential).
- Refused mutants: existing `admit_ty_d_forged`, `admit_ctor_arity_forged` (`ARCH:1560`); new `ty_d_name_ignored` (two declarations that differ only in a constructor name give equal `ty_d`; refused because the ADMIT-WORK recomputation differs), `ty_d_external_by_name` (the control of C5-1: `Bar` naming two different `Foo` gives one `decl_d` in the weakened form and two in the correct one), `ty_d_local_index_hashed` (a `TypeIdC` in a preimage: two programs with different table order give different `ty_d`), `admit_ctor_field_type_other_decl` (a `ctor_fields` row whose `ty_d` is not the template's).

### 5.6 Lines that change

- ARCH FP-4: the tag list (`ARCH:454`) gains codes 45 to 57; a row for `ty_d` and `decl_d` in the digest table (`ARCH:458-477`). ARCH 1.12 I-14 (`ARCH:558`), kit rows BOUNDARY COMMITMENT (`ARCH:599`, the `ty_d` part) and REFERENCE BRIDGE (`ARCH:608`). AD-14 (`ARCH:3451`); the note on `decl_schema_d` at `ARCH:3608`; kit VALUE DIGEST `ARCH:1892`; ADMIT-WORK `ARCH:2232-2234` (O5-2).
- `O/requirements.md:47` and `O/specs/sherlock-recursion/spec.md:250` (SRC-022): replace "[OPEN: the normal form of `decl_schema_d` ...]" by a pointer to this normal form; `O/design.md:190`; `O/dissent.md:204-208` (D-2): status decided, Astra's dissent kept.
- OpenSpec lines: `ML/specs/sherlock-value-digest/spec.md:40` (the mutant "a raw type id enters the digest": unchanged; this normal form is what makes it refusable). No `ML` spec states `ty_d` or `decl_schema_d` yet, so the form needs a new requirement in `ML/specs/sherlock-value-digest/spec.md` (the `ty_d` and `decl_d` preimages of 5.2) and one in `ML/specs/sherlock-reference-bridge/spec.md` (the vectors of 5.5).

## 6. The SRC-021 amendment (I-15 and kit INIT; AD-18 (a))

Owner rulings: "AD-18 (a)" (`ARCH:3455`) and "SRC-021 (AD-18): amend SRC-021 in `sherlock-vm-consensus-20261107` to the one-arena Init field list of kit INIT" (`DA` line 71); AD-17 (b) with an empty genesis memory (`DA` line 66). The one-arena list is kit INIT (`ARCH:2752-2776`). Three texts change: `O/requirements.md:46` (the SRC-021 row), `O/specs/sherlock-recursion/spec.md:234-248` (the requirement and its scenarios) and `O/design.md` 3.2(5) (`:176`, `:194-196`, `:218-270`: the openings, the input bullets, "Where the input enters", the Init table and its notes).

Reconciliations (section 1.11 of ARCH wins over the kit texts, `ARCH:307`):

- `input_d` has the salt: `P2(SHK_IN, salt: 8 words, val_d, fuel_lo: u64, fuel_hi: u64, stack_limit: u64)` (FP-4, `ARCH:466`). Kit INIT (`ARCH:2750`) and the GP-2 hook (`ARCH:2507`) omit the salt; both are edited.
- `view_root` is a header word (FP-5, `ARCH:493-501`); "body word" at `ARCH:139`, `:1027`, `:3454` is edited.
- The digests are the BabyBear `sherlock-value/2` forms (FP-4). SRC-022 still names `sherlock-value/1` over BLS12-381 (`O/requirements.md:47`). This amendment refers to SRC-022 as is; the parallel amendment of SRC-022 to the BabyBear form is OPEN O6-1 and is not part of AD-18.
- The sentence "When the first execution segment begins, the proof SHALL authenticate the input value, the dependency claims and every resource-policy parameter that can affect the outcome" is kept word for word. Under AD-20 (a) the dependency claim of an input is authenticated at the seal by the input link, not at genesis (`ARCH:2780`, `:3457`); the owner's reading of this sentence is AD-20's open point and this amendment does not settle it (O6-2).

### 6.1 Replacement text of the SRC-021 row (`O/requirements.md:46`, middle column)

> (Added 2026-11-10; amended 2026-11-13 by the owner rulings AD-17 (b) and AD-18 (a) of 2026-11-12: one memory arena, by-digest input admission with an empty genesis memory; the text is the one-arena field list of `FD/ARCHITECTURE.md` kit INIT.) If a run's initial state is not `Init(prog_d, input_d)`, the run SHALL NOT be accepted as a canonical run; equality with a supplied `init_d` alone SHALL NOT count as that check. When `h_pre = 0`, the first segment leaf SHALL prove in circuit that its state header, its persistent registers, both stacks and its memory state equal `Init(prog_d, input_d)` field by field (the one-arena field list of design §3.2(5)): control `Eval` at `e0`, or at the reserved `ENTRY` that applies `in_fn` to the input; fuel (u128, two 64-bit limbs) in both fuel registers and the stack limit from `input_d`; outcome `Running`; every other counter zero; environment pointer `None`; both stacks empty, with count 0, the chain heads `EMPTY_F` and `EMPTY_T` and zero cached tops; helper, result and scratch registers zero; the one memory arena empty: `len = 0`, the empty tree root of depth `TOP + 3` and the empty tail digest; `prog_root` from `prog_d`; the header words `input_d` (opened), `deps_d = EMPTY_DEPS` and `view_root`, which is `VIEW_NONE` when the input is absent. `input_d` SHALL be `P2(SHK_IN, salt, val_d, fuel_lo, fuel_hi, stack_limit)` with each limb and the stack limit below 2^64 and `salt` 8 words fixed for the run. `val_d` SHALL be `vd(x)` of SRC-022 for an input `x`, and `VAL_NONE` when the input is absent. The input SHALL enter by digest: no input record is in the genesis memory, and a step SHALL read an input node only through the read port under `view_root`; the seal SHALL require the `VIEW(val_d, view_root)` claim for every `val_d` other than `VAL_NONE`. The Init check SHALL NOT scan the input. When the first execution segment begins, the proof SHALL authenticate the input value, the dependency claims and every resource-policy parameter that can affect the outcome. The gate SHALL be `h_pre = 0`, not a witness flag. The seal SHALL require `h_lo = 0` and a terminal outcome. Off-chain recomputation of `init_d` (option G2) MAY be recorded as a diagnostic and SHALL NOT be acceptance authority.

Acceptance column (replacement): "The host differential (row A37) computes `Init` for fact(3), list123 and one run with input and matches `Machine::new` and, for fields without a reference counterpart, the C6 `genesis()`; for the run with input, fields 2, 23 to 29 and the memory triple are compared by decoding to the reference value and control; the Quint model `models/sherlock/quint/init.qnt`, extended to the one-arena list, accepts exactly `Init` and catches every single-field mutant; in circuit, a non-`Init` register, stack or memory word, a non-canonical fuel limb, a swapped `input_d` opening, a `view_root` other than `VIEW_NONE` for an absent input, and a seal with `h_lo != 0`, a `Running` outcome or no `VIEW` claim are refused at the first leaf or the seal (row A13); the record names the circuit that performs the check."

The cost sentence of the old row (27,736 rows on the C6 digest leaf) is moved to design §3.2(5) as history: it is a four-root halo2 figure, not of this design (`ARCH:2804`).

### 6.2 Replacement Init table (`O/design.md:222-270`)

The table keeps the Quint field numbers 1 to 33 and 38 to 39 and replaces 34 to 37. Header words are marked (H). Under AD-17 (b) the genesis memory is empty, so `n_d = n_f = 0`.

| # | Field | Chip word | Init value |
|---|---|---|---|
| 1 | `ctl_tag` | control | `Eval` (tag 0) |
| 2 | `ctl_expr` | control | `e0` for `Expr(e0)`; `ENTRY` for `Main` |
| 3 | `ctl_rest` | control | 0 |
| 4 | `ctl_origin` | control | 0 |
| 5 | `outcome` | `outcome_tag`, `outcome_payload` (H) | `Running`, 0 |
| 6 | `initial_fuel` | register | `fuel_lo + 2^64 fuel_hi` |
| 7 | `remaining_fuel` | register | the same |
| 8 to 11 | consumed work, helper work, helper visits, reference cursor | registers | 0 |
| 12 | `stack_limit` | register | from the `input_d` opening |
| 13, 14 | `frame_count`, `task_count` | stack cells | 0 |
| 15 | `env_ptr` | register | `None` |
| 16, 17 | `fp`, `tp` | `frames_below`, `tasks_below` | `EMPTY_F`, `EMPTY_T` |
| 18 to 21 | top frame, top task, helper, result | cached records | 0 |
| 22 | `vregs` | scratch `v0..v2` | 0 (never read, CORE invariant 8) |
| 23 to 25 | allocation counters | registers | 0 |
| 26 to 29 | per-kind active lengths, where the machine keeps them as registers | registers | 0 |
| 30 to 32 | segment counters | registers | 0 |
| 33 | `prog_root`, `prog_d` | body words | the opened `prog_root`; the lane `prog_d` |
| 34 | `mem_len` (replaces `root_data`) | MEM frontier `len` | 0 |
| 35 | `mem_root` (replaces `root_fields`) | token word `root` | `empty_at(TOP + 3)` (`chip/dense.rs:86-119`) |
| 36 | `mem_tail` (replaces `root_env`) | token word `tail_d` | `tail_d([])` (`chip/dense.rs:144-152`) |
| 37 | (removed; was `root_slots`) | | |
| 38 | `fin` | (pure profile) | absent, 0 |
| 39 | `deps` | `deps_d` (H) | `EMPTY_DEPS` |
| 40 | `input_d` (new) | `input_d` (H) | the opened `input_d` |
| 41 | `view_root` (new) | `view_root` (H) | `VIEW_NONE` when `val_d = VAL_NONE`; otherwise the witness word, tied to `val_d` at the seal by `VIEW` |
| 42 | constructor fold cells (new) | body words | 0 |

Replacement notes: "`NIL` is replaced by the hash-chain constants `EMPTY_F`, `EMPTY_T` (kit STACKS). The one memory arena holds every record kind with a kind tag (L-9, M-1); its state is `(len, root, tail_d)` and replaces the four `(root, length)` pairs; the sentence 'its root SHALL open to these four `(root, length)` pairs' is withdrawn. The input enters by digest (AD-17 (b)): fields 23 to 29 and 34 to 36 do not depend on `x`."

"Where the input enters" (`O/design.md:218`): replace "The cells of `x` are the initial DATA and FIELDS prefix" by "The input is held by digest: `ARG` decodes to a reference to record 0 of the input view, and a step reads an input node through `read_input` under `view_root` (`FD/ARCHITECTURE.md` kit CORE)"; keep the [OPEN: `ENTRY`/`ARG` rule coverage] item, which now also names the reference variant O1-1. The [OPEN since the C6 review] items of `O/design.md:196` and the SRC-021 row close for the DATA and FIELDS roots and lengths and for `ref_x`; the `val_d` of an absent input is `VAL_NONE = P2(SHK_VALUE_V2)` (FP-4 constants, `ARCH:456`).

### 6.3 Requirement text in the spec (`O/specs/sherlock-recursion/spec.md:234-248`)

The requirement body is the text of 6.1. Scenario "Single-field deviation" reads "differs from `Init(prog_d, input_d)` in exactly one field of the one-arena list (design §3.2(5))" in place of "one of the 39 fields", and its last clause reads "for the run with input, fields 2, 23 to 29 and 34 to 36 by decoding". A new scenario:

> #### Scenario: Input by digest
> - **WHEN** a first leaf with an input carries an input record in its genesis memory, or a `view_root` other than `VIEW_NONE` with `val_d = VAL_NONE`, or a run with an input is sealed without the `VIEW(val_d, view_root)` claim.
> - **THEN** the first leaf or the seal refuses it.

### 6.4 OPEN items

- O6-1, SRC-022 over BabyBear (`sherlock-value/2`, FP-4) is a separate amendment. Recommendation: amend SRC-022 with T5 item 5, in the same change.
- O6-2, the "dependency claims" sentence and AD-20 (above). Recommendation: keep it; the owner's AD-20 reading decides whether the genesis binding is required work.
- O6-3, fields 26 to 32 under one arena: whether the per-kind active lengths and the segment counters stay registers or follow from `len` is set by the state schema (section 1.6). The Init value is 0 in every case.

### 6.5 Closure test (AD-18 part of I-15; kit INIT)

- `openspec validate sherlock-vm-consensus-20261107` passes on the amended text (F.2 gate, `ML/tasks.md:35`). Run on a scratch copy of `openspec/` with the texts of 6.1 to 6.3 applied: see 6.6.
- Test `init_schema_coverage` (`ARCH:2808`): every word of the frozen boundary schema (section 1.6) has a row in the table of 6.2.
- Positives: `init_expr_entry_no_input`, `init_main_entry_with_input`, `init_fuel_limbs_boundaries`, `init_non_genesis_leaf_any_state` (`ARCH:2808`).
- Refused mutants (`ARCH:2809`): `init_memory_not_genesis`, `init_stack_head_not_empty_const`, `init_deps_not_empty`, `init_input_d_swapped_opening`, `init_entry_input_mismatch`, `in_read_outside_view`; new `init_view_root_with_absent_input` and `seal_input_without_view_claim`. `init_genesis_record_kind_out_of_range` is dropped: AD-17 (b) gives no genesis record.
- The Quint model `models/sherlock/quint/init.qnt` extended to fields 1 to 42 of 6.2 with `InitFieldsCoverSchema` (`ARCH:2807`).

### 6.6 Validation run

MEASURED, 2026-11-13. A scratch copy of `~/Projects/Moriarty/openspec/` was made, and a script placed the text of 6.1 into the SRC-021 row of `requirements.md` and into the SRC-021 requirement body of `specs/sherlock-recursion/spec.md`. It also inserted the scenario "Input by digest" and the edited wording of "Single-field deviation". Then `openspec validate sherlock-vm-consensus-20261107` ran with the node 26.13.0 `openspec` binary. Result: `Change 'sherlock-vm-consensus-20261107' is valid`, exit code 0. The run gave 26 warnings, the same 26 lines as the unamended baseline: the amendment adds no warning. The overlong-requirement warning on SRC-021 was already in the baseline. The design.md table edit of 6.2 was not applied, because validate does not parse design.md. The real tree is unchanged.

### 6.7 Lines that change

- `O/requirements.md:46`; `O/specs/sherlock-recursion/spec.md:234-248`; `O/design.md:176`, `:194-196`, `:218-270`; `O/dissent.md` owner question 1 (the input form is decided).
- ARCH kit INIT: `ARCH:2750` (salt), `:2778` (the amendment is made), `:2784-2794` (the form is decided; the candidate list becomes history); AD-18 `ARCH:3455` ("pending" removed); kit rows INIT (`ARCH:607`), I-15 (`ARCH:559`, the SRC-021 part).
- `ML/tasks.md:35` (F.2, there OD-18): done when the owner signs and validate passes. `ML/specs/sherlock-init-admission/spec.md`: the field list.
- OpenSpec lines: `ML/specs/sherlock-init-admission/spec.md:4` (INIT-001, the one-arena field list: unchanged wording, it already names the kit list) and `:26` (INIT-002, `input_d = Poseidon(SHK_IN, val_d, fuel_lo, fuel_hi, stack_limit)`: add `salt` and write `P2`, as in 6.1); `:33-34` (the entry scenarios: unchanged). `ML/tasks.md:36` (F.3) closes with section 4.
- ARCH section 5.2: `ARCH:3925-3930` (INIT requirements; `:3929` gains `salt` in the opened `input_d`).

## Appendix A. State lane table (helper pass, checked by script)

Prefixes: P = `hm/c6_phys.rs`, H = `hm/c6_phys_helper.rs`, S = `hm/shared.rs`, at main `4ea6fa29d`. RES and `RESULTS.md` are the `ENC` file. ARCH lines in this appendix are at fp8 `bcae0f185`. Program indices, program slice lengths and cursors over program slices are written `u32` here, which is `ridx` at `u32` (choice C2-6). At `ridx = u64` the 15 `ridx` lanes of section 1.6 become `u64`: frame lanes 1, 5, 8, 9, 12; task lane 1; helper lanes 12, 17, 19, 21; control lanes 1, 9, 13, 14; register lane 19. Every word count of this appendix is DERIVED (script sums).

### A. Rules applied

- Widest type per lane (RES:103). Words are counted first: `bit`, `tag(w)` and `u16` are 1 word, `u32` 2, `u64` 4, `addr` 5 (TOP = 63, ARCH:320), `u128` 8, `digest` 8. Among the 1-word types the order is `bit` < `tag(w)` (larger w is wider) < `u16`.
- **ROM vs heap comes from the cap that the reference checks for each field.** The table limit names the table:
  - Program sections (ROM rows, `u32`): `max_expressions`, `max_names`, `max_arms`, `max_children`, `max_functions`, `max_parameters`, `max_plan_nodes`, `max_field_paths`, types and ctors. These counts are read from the program section (`c6_phys_program.rs:528,549,556,695,749`).
  - Heap tables (`addr`): `max_value_slots` (a state table, `c6_phys_data.rs:366`), `max_env_cells` and `max_data_nodes`, plus `max_data_field_slots` (`c6_phys_data.rs:51-52`).
- RefC is `(tag bit, Const u32 | Data addr)`, so the lane is `addr`. The cap is at P:452-458 and `c6_phys_data.rs:167-172`. put at P:338-346 (the tag lane stays 0 for Const). EnvRootC = `Option<usize>` (S:725) is `(present bit, addr)`, put at P:322-327. SliceC = `(start, len)` (S:707-710) is put at P:354-357, and `len` is `u32`.
- A helper `Wr::opt` (H:170-173) is `(present bit, payload)`. The payload type comes from that field's cap, because it is not always an env cell. `Wr::opt_ref` (H:174-180) is 3 lanes: present bit, ref tag bit, ref index.
- A cursor bounded by a slice length (`p_cursor`, `a_cursor`, `src_cursor`, `path_cursor`, `Find.row_cursor`) has the type of that slice's length: `ridx` for a program slice, `addr` for a heap slice (choice C2-6). A cursor bounded by `max_arity` (task `cursor`, `ConstructorFieldsF.index`) is `u32`.
- Pick (no word effect): `u8` codes (`Rejected`/`Error`/`ScalarError` code, `Match.field_idx`, S:832) are `tag(8)`, ranged by RANGE `(x, 8)`.

### A. Slice tables (frame record)

| frame field | table | evidence |
|---|---|---|
| `MatchF.arms` | ROM arms | `cap_slice("frame.arms", arms, c.max_arms, None)` P:535 |
| `ArgsF.completed` (`ValSliceC`, S:723) | heap value_slots | `cap_vals` P:547, P:477-479 (`max_value_slots`) |
| `ArgsF.rest` | ROM children | `cap_kids` P:548, P:480-482 (`max_children`) |
| `ConstructorFieldsF.completed` | heap value_slots | P:580 |
| `ConstructorFieldsF.rest` | ROM children | P:581 |

The FrameRowC definition is at S:867-927. Control slices: `Invoke/Enter.args` and `Materialize.fields` are heap value_slots (P:604, P:610, P:661). `Cases.arms` and `CaseResult.rest` are ROM arms (P:629, P:644).

### A. Word totals per record (script output, base picks, `ridx = u32`; DERIVED)

```
parent_resume=u32 counters=addr
frame          42
task           17
helper         89
result         16
control        61
header19_47    149
header19_46    145
outcome23_25   7
    61  control (header lanes 2-18)
   145  scalar registers lanes 19-46 (frame_count lane 47 counted once, as StackCells(frames).count)
    54  StackCells(frames): count u64 + below digest + top Frame
    29  StackCells(tasks): count u64 + below digest + top Task
    89  helper record
    16  result record
    21  memory: len addr + root digest + tail_d digest
    16  prog_root digest + prog_d digest
GRAND body words = 431   (outcome moved to header: 424)
```

The grand total of **431 body words** is the sum of these parts:
- control (61);
- the scalar registers of lanes 19-46 (145);
- `StackCells(frames)` = `count: u64` + `below: digest` + top frame (4 + 8 + 42 = 54);
- `StackCells(tasks)` (4 + 8 + 17 = 29);
- helper (89) and result (16);
- the memory triple `len: addr`, `root: digest`, `tail_d: digest` (21);
- `prog_root` and `prog_d` (16).

Lane 47 `frame_count` is the same register as `StackCells(frames).count` (RES:103: "CORE compares `frame_count <= stack_limit`"), so it is counted once. Lanes 19-47 total 149 words with it and 145 without it.

Left out as codec framing: lanes 0-1 (magic, version, P:1356-1357) and the 4-lane helper section header (H:63).

Variant totals (script output, arguments `parent_resume_type counter_type`, DERIVED):

```
args: u64 addr
GRAND body words = 433   (outcome moved to header: 426)
args: addr addr
GRAND body words = 434   (outcome moved to header: 427)
args: u32 u64
GRAND body words = 417   (outcome moved to header: 410)
```

### A. Tables

#### Frame record: 13 lanes, 42 words

| lane | variants -> field (type) | lane type | words |
|---:|---|---|---:|
| 0 | (all) kind `tag(4)` [10 kinds 0..9, P:691-770] | `tag(4)` | 1 |
| 1 | LetF.binder(name ROM) `u32` [P:679-683]<br>LeftF.op(7 BinaryOp, P:364-373) `tag(3)` [P:691-695]<br>RightF.op `tag(3)` [P:698-701]<br>UnaryF.op(2 UnaryOp, P:387-391) `tag(1)` [P:704-706]<br>IfF.then_expr `u32` [P:714-718]<br>MatchF.arms.start(ROM arms, P:535) `u32` [P:725-728]<br>ArgsF.function_name(name ROM) `u32` [P:737-742]<br>ReturnF.return_type(type ROM) `u32` [P:749-752]<br>ScopeF.saved_env.present `bit` [P:755-757]<br>ConstructorFieldsF.type_id `u32` [P:770-779] | `u32` | 2 |
| 2 | LetF.body(expr ROM) `u32` [P:679-683]<br>LeftF.right(expr ROM) `u32` [P:691-695]<br>RightF.left.tag `bit` [P:698-701]<br>UnaryF.origin `u32` [P:704-706]<br>IfF.else_expr `u32` [P:714-718]<br>MatchF.arms.len `u32` [P:725-728]<br>ArgsF.completed.start(heap value_slots, P:547) `addr` [P:737-742]<br>ReturnF.saved_env.present `bit` [P:749-752]<br>ScopeF.saved_env.cell `addr` [P:755-757]<br>ConstructorFieldsF.ctor_id `u32` [P:770-779] | `addr` | 5 |
| 3 | LetF.saved_env.present `bit` [P:679-683]<br>LeftF.saved_env.present `bit` [P:691-695]<br>RightF.left.idx(Const u32 or Data addr) `addr` [P:698-701]<br>IfF.saved_env.present `bit` [P:714-718]<br>MatchF.saved_env.present `bit` [P:725-728]<br>ArgsF.completed.len `u32` [P:737-742]<br>ReturnF.saved_env.cell `addr` [P:749-752]<br>ScopeF.origin `u32` [P:755-757]<br>ConstructorFieldsF.index(field idx < max_arity, P:578) `u32` [P:770-779] | `addr` | 5 |
| 4 | LetF.saved_env.cell `addr` [P:679-683]<br>LeftF.saved_env.cell `addr` [P:691-695]<br>RightF.origin `u32` [P:698-701]<br>IfF.saved_env.cell `addr` [P:714-718]<br>MatchF.saved_env.cell `addr` [P:725-728]<br>ArgsF.rest.start(ROM children, P:548) `u32` [P:737-742]<br>ReturnF.origin `u32` [P:749-752]<br>ConstructorFieldsF.field_type `u32` [P:770-779] | `addr` | 5 |
| 5 | LetF.origin(expr ROM) `u32` [P:679-683]<br>LeftF.origin `u32` [P:691-695]<br>IfF.origin `u32` [P:714-718]<br>MatchF.origin `u32` [P:725-728]<br>ArgsF.rest.len `u32` [P:737-742]<br>ConstructorFieldsF.checked `bit` [P:770-779] | `u32` | 2 |
| 6 | ArgsF.saved_env.present `bit` [P:737-742]<br>ConstructorFieldsF.completed.start(heap value_slots, P:580) `addr` [P:770-779] | `addr` | 5 |
| 7 | ArgsF.saved_env.cell `addr` [P:737-742]<br>ConstructorFieldsF.completed.len `u32` [P:770-779] | `addr` | 5 |
| 8 | ArgsF.origin `u32` [P:737-742]<br>ConstructorFieldsF.rest.start(ROM children, P:581) `u32` [P:770-779] | `u32` | 2 |
| 9 | ConstructorFieldsF.rest.len `u32` [P:770-779] | `u32` | 2 |
| 10 | ConstructorFieldsF.saved_env.present `bit` [P:770-779] | `bit` | 1 |
| 11 | ConstructorFieldsF.saved_env.cell `addr` [P:770-779] | `addr` | 5 |
| 12 | ConstructorFieldsF.origin `u32` [P:770-779] | `u32` | 2 |

#### Task record: 7 lanes, 17 words

| lane | variants -> field (type) | lane type | words |
|---:|---|---|---:|
| 0 | (all) kind `tag(2)` [3 kinds, H:992-1010] | `tag(2)` | 1 |
| 1 | ValidateTask.r.tag `bit` [H:992-995; caps H:1056-1057]<br>EqualTask.left.tag `bit` [H:1003-1007; caps H:1065-1067]<br>PairTask.binder(name ROM) `u32` [H:1010-1013; caps H:1070-1072] | `u32` | 2 |
| 2 | ValidateTask.r.idx(Const u32 or Data addr) `addr` [H:992-995; caps H:1056-1057]<br>EqualTask.left.idx(Const u32 or Data addr) `addr` [H:1003-1007; caps H:1065-1067]<br>PairTask.sort(type ROM) `u32` [H:1010-1013; caps H:1070-1072] | `addr` | 5 |
| 3 | ValidateTask.visit(2 TaskVisitC, H:148) `tag(1)` [H:992-995; caps H:1056-1057]<br>EqualTask.right.tag `bit` [H:1003-1007; caps H:1065-1067]<br>PairTask.arg.tag `bit` [H:1010-1013; caps H:1070-1072] | `tag(1)` | 1 |
| 4 | ValidateTask.cursor(< max_arity) `u32` [H:992-995; caps H:1056-1057]<br>EqualTask.right.idx(Const u32 or Data addr) `addr` [H:1003-1007; caps H:1065-1067]<br>PairTask.arg.idx(Const u32 or Data addr) `addr` [H:1010-1013; caps H:1070-1072] | `addr` | 5 |
| 5 | EqualTask.visit `tag(1)` [H:1003-1007; caps H:1065-1067] | `tag(1)` | 1 |
| 6 | EqualTask.cursor(< max_arity) `u32` [H:1003-1007; caps H:1065-1067] | `u32` | 2 |

#### Helper record: 23 lanes, 89 words

| lane | variants -> field (type) | lane type | words |
|---:|---|---|---:|
| 0 | (all) kind `tag(4)` [12 values: 0 None, 1..11, H:296-485] | `tag(4)` | 1 |
| 1 | Scalar.site(27 SiteTag, H:125-129) `tag(5)` [H:308-315; caps H:634-640]<br>EnvGet.site(27 SiteTag, H:125-129) `tag(5)` [H:324-328; caps H:648-650]<br>EnvPut.site(27 SiteTag, H:125-129) `tag(5)` [H:338-343; caps H:659-662]<br>Find.site(27 SiteTag, H:125-129) `tag(5)` [H:352-356; caps H:670-672]<br>Bind.site(27 SiteTag, H:125-129) `tag(5)` [H:370-379; caps H:684-700]<br>Append.site(27 SiteTag, H:125-129) `tag(5)` [H:390-396; caps H:710-719]<br>Validate.site(27 SiteTag, H:125-129) `tag(5)` [H:403-405; caps H:721]<br>Equal.site(27 SiteTag, H:125-129) `tag(5)` [H:413-416; caps H:723-724]<br>Match.site(27 SiteTag, H:125-129) `tag(5)` [H:432-443; caps H:738-756]<br>Plan.site(27 SiteTag, H:125-129) `tag(5)` [H:460-472; caps H:770-778]<br>Build.site(27 SiteTag, H:125-129) `tag(5)` [H:485-493; caps H:790-805] | `tag(5)` | 1 |
| 2 | Scalar.origin_reference_cursor(u128) `u128` [H:308-315; caps H:634-640]<br>EnvGet.origin_reference_cursor(u128) `u128` [H:324-328; caps H:648-650]<br>EnvPut.origin_reference_cursor(u128) `u128` [H:338-343; caps H:659-662]<br>Find.origin_reference_cursor(u128) `u128` [H:352-356; caps H:670-672]<br>Bind.origin_reference_cursor(u128) `u128` [H:370-379; caps H:684-700]<br>Append.origin_reference_cursor(u128) `u128` [H:390-396; caps H:710-719]<br>Validate.origin_reference_cursor(u128) `u128` [H:403-405; caps H:721]<br>Equal.origin_reference_cursor(u128) `u128` [H:413-416; caps H:723-724]<br>Match.origin_reference_cursor(u128) `u128` [H:432-443; caps H:738-756]<br>Plan.origin_reference_cursor(u128) `u128` [H:460-472; caps H:770-778]<br>Build.origin_reference_cursor(u128) `u128` [H:485-493; caps H:790-805] | `u128` | 8 |
| 3 | Scalar.parent_resume(OPEN, S:745-746).present `bit` [H:308-315; caps H:634-640]<br>EnvGet.parent_resume(OPEN, S:745-746).present `bit` [H:324-328; caps H:648-650]<br>EnvPut.parent_resume(OPEN, S:745-746).present `bit` [H:338-343; caps H:659-662]<br>Find.parent_resume(OPEN, S:745-746).present `bit` [H:352-356; caps H:670-672]<br>Bind.parent_resume(OPEN, S:745-746).present `bit` [H:370-379; caps H:684-700]<br>Append.parent_resume(OPEN, S:745-746).present `bit` [H:390-396; caps H:710-719]<br>Validate.parent_resume(OPEN, S:745-746).present `bit` [H:403-405; caps H:721]<br>Equal.parent_resume(OPEN, S:745-746).present `bit` [H:413-416; caps H:723-724]<br>Match.parent_resume(OPEN, S:745-746).present `bit` [H:432-443; caps H:738-756]<br>Plan.parent_resume(OPEN, S:745-746).present `bit` [H:460-472; caps H:770-778]<br>Build.parent_resume(OPEN, S:745-746).present `bit` [H:485-493; caps H:790-805] | `bit` | 1 |
| 4 | Scalar.parent_resume(OPEN, S:745-746) `u32` [H:308-315; caps H:634-640]<br>EnvGet.parent_resume(OPEN, S:745-746) `u32` [H:324-328; caps H:648-650]<br>EnvPut.parent_resume(OPEN, S:745-746) `u32` [H:338-343; caps H:659-662]<br>Find.parent_resume(OPEN, S:745-746) `u32` [H:352-356; caps H:670-672]<br>Bind.parent_resume(OPEN, S:745-746) `u32` [H:370-379; caps H:684-700]<br>Append.parent_resume(OPEN, S:745-746) `u32` [H:390-396; caps H:710-719]<br>Validate.parent_resume(OPEN, S:745-746) `u32` [H:403-405; caps H:721]<br>Equal.parent_resume(OPEN, S:745-746) `u32` [H:413-416; caps H:723-724]<br>Match.parent_resume(OPEN, S:745-746) `u32` [H:432-443; caps H:738-756]<br>Plan.parent_resume(OPEN, S:745-746) `u32` [H:460-472; caps H:770-778]<br>Build.parent_resume(OPEN, S:745-746) `u32` [H:485-493; caps H:790-805] | `u32` | 2 |
| 5 | Scalar.mode(10 ScalarMode, H:130-131) `tag(4)` [H:308-315; caps H:634-640]<br>EnvGet.binder(name ROM) `u32` [H:324-328; caps H:648-650]<br>EnvPut.binder(name ROM) `u32` [H:338-343; caps H:659-662]<br>Find.function_name(name ROM) `u32` [H:352-356; caps H:670-672]<br>Bind.function(function ROM) `u32` [H:370-379; caps H:684-700]<br>Append.src.start(heap value_slots, H:713) `addr` [H:390-396; caps H:710-719]<br>Validate.root.tag `bit` [H:403-405; caps H:721]<br>Equal.left.tag `bit` [H:413-416; caps H:723-724]<br>Match.scrutinee.tag `bit` [H:432-443; caps H:738-756]<br>Plan.source_match(expr ROM, H:770) `u32` [H:460-472; caps H:770-778]<br>Build.type_id(type ROM) `u32` [H:485-493; caps H:790-805] | `addr` | 5 |
| 6 | Scalar.operand_a.tag `bit` [H:308-315; caps H:634-640]<br>EnvGet.cursor(env cell heap, H:649).present `bit` [H:324-328; caps H:648-650]<br>EnvPut.value.tag `bit` [H:338-343; caps H:659-662]<br>Find.row_cursor(function ROM row, H:671) `u32` [H:352-356; caps H:670-672]<br>Bind.params.start(ROM parameters, H:688) `u32` [H:370-379; caps H:684-700]<br>Append.src.len `u32` [H:390-396; caps H:710-719]<br>Validate.root.idx(Const u32 or Data addr) `addr` [H:403-405; caps H:721]<br>Equal.left.idx(Const u32 or Data addr) `addr` [H:413-416; caps H:723-724]<br>Match.scrutinee.idx(Const u32 or Data addr) `addr` [H:432-443; caps H:738-756]<br>Plan.scrutinee.tag `bit` [H:460-472; caps H:770-778]<br>Build.ctor_id(ctor ROM) `u32` [H:485-493; caps H:790-805] | `addr` | 5 |
| 7 | Scalar.operand_a.idx(Const u32 or Data addr) `addr` [H:308-315; caps H:634-640]<br>EnvGet.cursor(env cell heap, H:649) `addr` [H:324-328; caps H:648-650]<br>EnvPut.value.idx(Const u32 or Data addr) `addr` [H:338-343; caps H:659-662]<br>Find.found(function ROM, H:672).present `bit` [H:352-356; caps H:670-672]<br>Bind.params.len `u32` [H:370-379; caps H:684-700]<br>Append.value.tag `bit` [H:390-396; caps H:710-719]<br>Validate.status(TriStatus 3, H:135) `tag(2)` [H:403-405; caps H:721]<br>Equal.right.tag `bit` [H:413-416; caps H:723-724]<br>Match.arm(ROM arms, H:739) `u32` [H:432-443; caps H:738-756]<br>Plan.scrutinee.idx(Const u32 or Data addr) `addr` [H:460-472; caps H:770-778]<br>Build.src.start(heap value_slots, H:795) `addr` [H:485-493; caps H:790-805] | `addr` | 5 |
| 8 | Scalar.operand_b.present `bit` [H:308-315; caps H:634-640]<br>EnvGet.found.present `bit` [H:324-328; caps H:648-650]<br>EnvPut.base.present `bit` [H:338-343; caps H:659-662]<br>Find.found(function ROM, H:672) `u32` [H:352-356; caps H:670-672]<br>Bind.args.start(heap value_slots, H:694) `addr` [H:370-379; caps H:684-700]<br>Append.value.idx(Const u32 or Data addr) `addr` [H:390-396; caps H:710-719]<br>Equal.right.idx(Const u32 or Data addr) `addr` [H:413-416; caps H:723-724]<br>Match.saved_env.present `bit` [H:432-443; caps H:738-756]<br>Plan.saved_env.present `bit` [H:460-472; caps H:770-778]<br>Build.src.len `u32` [H:485-493; caps H:790-805] | `addr` | 5 |
| 9 | Scalar.operand_b.tag `bit` [H:308-315; caps H:634-640]<br>EnvGet.found.tag `bit` [H:324-328; caps H:648-650]<br>EnvPut.base.cell `addr` [H:338-343; caps H:659-662]<br>Find.status(TriStatus 3, H:135) `tag(2)` [H:352-356; caps H:670-672]<br>Bind.args.len `u32` [H:370-379; caps H:684-700]<br>Append.src_cursor(<= src.len) `u32` [H:390-396; caps H:710-719]<br>Equal.status(TriStatus 3, H:135) `tag(2)` [H:413-416; caps H:723-724]<br>Match.saved_env.cell `addr` [H:432-443; caps H:738-756]<br>Plan.saved_env.cell `addr` [H:460-472; caps H:770-778]<br>Build.src_cursor(<= src.len) `u32` [H:485-493; caps H:790-805] | `addr` | 5 |
| 10 | Scalar.operand_b.idx(Const u32 or Data addr) `addr` [H:308-315; caps H:634-640]<br>EnvGet.found.idx(Const u32 or Data addr) `addr` [H:324-328; caps H:648-650]<br>EnvPut.new_root(env cell heap, H:662).present `bit` [H:338-343; caps H:659-662]<br>Bind.p_cursor(<= params.len) `u32` [H:370-379; caps H:684-700]<br>Append.dest_start(heap value_slots, H:718) `addr` [H:390-396; caps H:710-719]<br>Match.partial.present `bit` [H:432-443; caps H:738-756]<br>Plan.partial.present `bit` [H:460-472; caps H:770-778]<br>Build.field_start(heap data_fields slot, H:799-802) `addr` [H:485-493; caps H:790-805] | `addr` | 5 |
| 11 | Scalar.expected(type ROM).present `bit` [H:308-315; caps H:634-640]<br>EnvGet.status(TriStatus 3, H:135) `tag(2)` [H:324-328; caps H:648-650]<br>EnvPut.new_root(env cell heap, H:662) `addr` [H:338-343; caps H:659-662]<br>Bind.a_cursor(<= args.len) `u32` [H:370-379; caps H:684-700]<br>Append.staged.present `bit` [H:390-396; caps H:710-719]<br>Match.partial.cell `addr` [H:432-443; caps H:738-756]<br>Plan.partial.cell `addr` [H:460-472; caps H:770-778]<br>Build.staged.present `bit` [H:485-493; caps H:790-805] | `addr` | 5 |
| 12 | Scalar.expected(type ROM) `u32` [H:308-315; caps H:634-640]<br>EnvPut.status(TriStatus 3, H:135) `tag(2)` [H:338-343; caps H:659-662]<br>Bind.partial.present `bit` [H:370-379; caps H:684-700]<br>Append.staged.tag `bit` [H:390-396; caps H:710-719]<br>Match.pat_tag(7 PatternTag, H:146-147) `tag(3)` [H:432-443; caps H:738-756]<br>Plan.node(ROM plan_nodes, H:774) `u32` [H:460-472; caps H:770-778]<br>Build.staged.tag `bit` [H:485-493; caps H:790-805] | `u32` | 2 |
| 13 | Scalar.phase(3, H:132) `tag(2)` [H:308-315; caps H:634-640]<br>Bind.partial.cell `addr` [H:370-379; caps H:684-700]<br>Append.staged.idx(Const u32 or Data addr) `addr` [H:390-396; caps H:710-719]<br>Match.x(name ROM, H:745-748) `u32` [H:432-443; caps H:738-756]<br>Plan.phase(6 PlanPhase, H:142-143) `tag(3)` [H:460-472; caps H:770-778]<br>Build.staged.idx(Const u32 or Data addr) `addr` [H:485-493; caps H:790-805] | `addr` | 5 |
| 14 | Scalar.reg_a(u128) `u128` [H:308-315; caps H:634-640]<br>Bind.dup_cursor(env cell heap, H:700).present `bit` [H:370-379; caps H:684-700]<br>Append.phase(3 AppendPhase, H:139) `tag(2)` [H:390-396; caps H:710-719]<br>Match.y(name ROM, H:749) `u32` [H:432-443; caps H:738-756]<br>Plan.after_walk(2, H:144) `tag(1)` [H:460-472; caps H:770-778]<br>Build.node.present `bit` [H:485-493; caps H:790-805] | `u128` | 8 |
| 15 | Scalar.staged.tag(7 values) `tag(3)` [H:308-315; caps H:634-640]<br>Bind.dup_cursor(env cell heap, H:700) `addr` [H:370-379; caps H:684-700]<br>Match.field_idx(u8, S:832) `tag(8)` [H:432-443; caps H:738-756]<br>Plan.cur.tag `bit` [H:460-472; caps H:770-778]<br>Build.node.tag `bit` [H:485-493; caps H:790-805] | `addr` | 5 |
| 16 | Scalar.staged.payload(U128Value n, bool, NodeKind, u8, sel) `u128` [H:308-315; caps H:634-640]<br>Bind.phase(6 BindPhase, H:136-137) `tag(3)` [H:370-379; caps H:684-700]<br>Match.staged.present `bit` [H:432-443; caps H:738-756]<br>Plan.cur.idx(Const u32 or Data addr) `addr` [H:460-472; caps H:770-778]<br>Build.node.idx(Const u32 or Data addr) `addr` [H:485-493; caps H:790-805] | `u128` | 8 |
| 17 | Bind.status(2 BindStatus, H:138) `tag(1)` [H:370-379; caps H:684-700]<br>Match.staged.tag `bit` [H:432-443; caps H:738-756]<br>Plan.path.start(ROM field_paths, H:776) `u32` [H:460-472; caps H:770-778]<br>Build.phase(2 BuildPhase, H:145) `tag(1)` [H:485-493; caps H:790-805] | `u32` | 2 |
| 18 | Match.staged.idx(Const u32 or Data addr) `addr` [H:432-443; caps H:738-756]<br>Plan.path.len `u32` [H:460-472; caps H:770-778] | `addr` | 5 |
| 19 | Match.phase(8 MatchPhase, H:140-141) `tag(3)` [H:432-443; caps H:738-756]<br>Plan.path_cursor(<= path.len) `u32` [H:460-472; caps H:770-778] | `u32` | 2 |
| 20 | Match.status(TriStatus 3, H:135) `tag(2)` [H:432-443; caps H:738-756]<br>Plan.arm(ROM arms, H:778).present `bit` [H:460-472; caps H:770-778] | `tag(2)` | 1 |
| 21 | Plan.arm(ROM arms, H:778) `u32` [H:460-472; caps H:770-778] | `u32` | 2 |
| 22 | Plan.status(TriStatus 3, H:135) `tag(2)` [H:460-472; caps H:770-778] | `tag(2)` | 1 |

#### Result record: 4 lanes, 16 words

| lane | variants -> field (type) | lane type | words |
|---:|---|---|---:|
| 0 | (all) kind `tag(5)` [20 values: 0 None, 1..19, H:826-887] | `tag(5)` | 1 |
| 1 | Validated.ok `bit` [H:829-830]<br>EnvFound.value.tag `bit` [H:833-834]<br>EnvUpdated.cell(env cell heap) `addr` [H:838-839; cap H:950]<br>FindFound.function(function ROM) `u32` [H:842-843; cap H:951]<br>BindOk.env.present `bit` [H:847-848; cap H:952]<br>AppendDone.seq.start(heap value_slots, H:953-956) `addr` [H:852-853; cap H:953]<br>ScalarValue.value.tag `bit` [H:856-857]<br>ScalarError.code(u8) `tag(8)` [H:860-861]<br>ScalarSelector.sel(3, H:134) `tag(2)` [H:864-865]<br>ScalarTypeTest.ok `bit` [H:868-869]<br>EqualDone.value.tag `bit` [H:872-873]<br>MatchMatched.env.present `bit` [H:876-877]<br>PlanMatched.arm(ROM arms) `u32` [H:881-883; cap H:960-961]<br>BuildDone.value.tag `bit` [H:887-888] | `addr` | 5 |
| 2 | EnvFound.value.idx(Const u32 or Data addr) `addr` [H:833-834]<br>BindOk.env.cell `addr` [H:847-848; cap H:952]<br>AppendDone.seq.len `u32` [H:852-853; cap H:953]<br>ScalarValue.value.idx(Const u32 or Data addr) `addr` [H:856-857]<br>EqualDone.value.idx(Const u32 or Data addr) `addr` [H:872-873]<br>MatchMatched.env.cell `addr` [H:876-877]<br>PlanMatched.env.present `bit` [H:881-883; cap H:960-961]<br>BuildDone.value.idx(Const u32 or Data addr) `addr` [H:887-888] | `addr` | 5 |
| 3 | PlanMatched.env.cell `addr` [H:881-883; cap H:960-961] | `addr` | 5 |

#### Control record: 17 lanes, 61 words

| lane | variants -> field (type) | lane type | words |
|---:|---|---|---:|
| 0 | (all) kind `tag(4)` [12 kinds 0..11, P:896-988] | `tag(4)` | 1 |
| 1 | Eval.expr(ROM) `u32` [P:896]<br>Return.value.tag `bit` [P:898-899]<br>ScheduleNext.frame.kind `tag(4)` [P:902-905]<br>Invoke.function_name(name ROM) `u32` [P:911-913]<br>Enter.function.present `bit` [P:916-921]<br>Bound.binding.ok `bit` [P:928-934]<br>Cases.value.tag `bit` [P:941-944]<br>CaseResult.matching.matched `bit` [P:953-961]<br>PrimResult.Error.is_error `bit` [P:967-969]<br>Error.code(u8) `tag(8)` [P:974-975]<br>Materialize.type_id `u32` [P:982-985]<br>Await.site(u16) `u16` [P:988-990] | `u32` | 2 |
| 2 | Return.value.idx(Const u32 or Data addr) `addr` [P:898-899]<br>ScheduleNext.frame.LetF.binder(name ROM) `u32` [P:902-905]<br>ScheduleNext.frame.LeftF.op(7 BinaryOp, P:364-373) `tag(3)` [P:902-905]<br>ScheduleNext.frame.RightF.op `tag(3)` [P:902-905]<br>ScheduleNext.frame.UnaryF.op(2 UnaryOp, P:387-391) `tag(1)` [P:902-905]<br>ScheduleNext.frame.IfF.then_expr `u32` [P:902-905]<br>ScheduleNext.frame.MatchF.arms.start(ROM arms, P:535) `u32` [P:902-905]<br>ScheduleNext.frame.ArgsF.function_name(name ROM) `u32` [P:902-905]<br>ScheduleNext.frame.ReturnF.return_type(type ROM) `u32` [P:902-905]<br>ScheduleNext.frame.ScopeF.saved_env.present `bit` [P:902-905]<br>ScheduleNext.frame.ConstructorFieldsF.type_id `u32` [P:902-905]<br>Invoke.args.start(heap value_slots, P:604) `addr` [P:911-913]<br>Enter.function(function ROM, P:608) `u32` [P:916-921]<br>Bound.env.present `bit` [P:928-934]<br>Cases.value.idx(Const u32 or Data addr) `addr` [P:941-944]<br>CaseResult.env.present `bit` [P:953-961]<br>PrimResult.Value.value.tag `bit` [P:964-966]<br>PrimResult.Error.code(u8) `tag(8)` [P:967-969]<br>Materialize.ctor_id `u32` [P:982-985]<br>Await.expr(ROM) `u32` [P:988-990] | `addr` | 5 |
| 3 | ScheduleNext.frame.LetF.body(expr ROM) `u32` [P:902-905]<br>ScheduleNext.frame.LeftF.right(expr ROM) `u32` [P:902-905]<br>ScheduleNext.frame.RightF.left.tag `bit` [P:902-905]<br>ScheduleNext.frame.UnaryF.origin `u32` [P:902-905]<br>ScheduleNext.frame.IfF.else_expr `u32` [P:902-905]<br>ScheduleNext.frame.MatchF.arms.len `u32` [P:902-905]<br>ScheduleNext.frame.ArgsF.completed.start(heap value_slots, P:547) `addr` [P:902-905]<br>ScheduleNext.frame.ReturnF.saved_env.present `bit` [P:902-905]<br>ScheduleNext.frame.ScopeF.saved_env.cell `addr` [P:902-905]<br>ScheduleNext.frame.ConstructorFieldsF.ctor_id `u32` [P:902-905]<br>Invoke.args.len `u32` [P:911-913]<br>Enter.args.start(heap value_slots, P:610) `addr` [P:916-921]<br>Bound.env.cell `addr` [P:928-934]<br>Cases.arms.start(ROM arms, P:629) `u32` [P:941-944]<br>CaseResult.env.cell `addr` [P:953-961]<br>PrimResult.Value.value.idx(Const u32 or Data addr) `addr` [P:964-966]<br>Materialize.fields.start(heap value_slots, P:661) `addr` [P:982-985] | `addr` | 5 |
| 4 | ScheduleNext.frame.LetF.saved_env.present `bit` [P:902-905]<br>ScheduleNext.frame.LeftF.saved_env.present `bit` [P:902-905]<br>ScheduleNext.frame.RightF.left.idx(Const u32 or Data addr) `addr` [P:902-905]<br>ScheduleNext.frame.IfF.saved_env.present `bit` [P:902-905]<br>ScheduleNext.frame.MatchF.saved_env.present `bit` [P:902-905]<br>ScheduleNext.frame.ArgsF.completed.len `u32` [P:902-905]<br>ScheduleNext.frame.ReturnF.saved_env.cell `addr` [P:902-905]<br>ScheduleNext.frame.ScopeF.origin `u32` [P:902-905]<br>ScheduleNext.frame.ConstructorFieldsF.index(field idx < max_arity, P:578) `u32` [P:902-905]<br>Enter.args.len `u32` [P:916-921]<br>Bound.return_type `u32` [P:928-934]<br>Cases.arms.len `u32` [P:941-944]<br>CaseResult.value.tag `bit` [P:953-961]<br>Materialize.fields.len `u32` [P:982-985] | `addr` | 5 |
| 5 | ScheduleNext.frame.LetF.saved_env.cell `addr` [P:902-905]<br>ScheduleNext.frame.LeftF.saved_env.cell `addr` [P:902-905]<br>ScheduleNext.frame.RightF.origin `u32` [P:902-905]<br>ScheduleNext.frame.IfF.saved_env.cell `addr` [P:902-905]<br>ScheduleNext.frame.MatchF.saved_env.cell `addr` [P:902-905]<br>ScheduleNext.frame.ArgsF.rest.start(ROM children, P:548) `u32` [P:902-905]<br>ScheduleNext.frame.ReturnF.origin `u32` [P:902-905]<br>ScheduleNext.frame.ConstructorFieldsF.field_type `u32` [P:902-905]<br>Bound.body(expr ROM) `u32` [P:928-934]<br>Cases.saved_env.present `bit` [P:941-944]<br>CaseResult.value.idx(Const u32 or Data addr) `addr` [P:953-961] | `addr` | 5 |
| 6 | ScheduleNext.frame.LetF.origin(expr ROM) `u32` [P:902-905]<br>ScheduleNext.frame.LeftF.origin `u32` [P:902-905]<br>ScheduleNext.frame.IfF.origin `u32` [P:902-905]<br>ScheduleNext.frame.MatchF.origin `u32` [P:902-905]<br>ScheduleNext.frame.ArgsF.rest.len `u32` [P:902-905]<br>ScheduleNext.frame.ConstructorFieldsF.checked `bit` [P:902-905]<br>Cases.saved_env.cell `addr` [P:941-944]<br>CaseResult.body(expr ROM) `u32` [P:953-961] | `addr` | 5 |
| 7 | ScheduleNext.frame.ArgsF.saved_env.present `bit` [P:902-905]<br>ScheduleNext.frame.ConstructorFieldsF.completed.start(heap value_slots, P:580) `addr` [P:902-905]<br>CaseResult.rest.start(ROM arms, P:644) `u32` [P:953-961] | `addr` | 5 |
| 8 | ScheduleNext.frame.ArgsF.saved_env.cell `addr` [P:902-905]<br>ScheduleNext.frame.ConstructorFieldsF.completed.len `u32` [P:902-905]<br>CaseResult.rest.len `u32` [P:953-961] | `addr` | 5 |
| 9 | ScheduleNext.frame.ArgsF.origin `u32` [P:902-905]<br>ScheduleNext.frame.ConstructorFieldsF.rest.start(ROM children, P:581) `u32` [P:902-905]<br>CaseResult.saved_env.present `bit` [P:953-961] | `u32` | 2 |
| 10 | ScheduleNext.frame.ConstructorFieldsF.rest.len `u32` [P:902-905]<br>CaseResult.saved_env.cell `addr` [P:953-961] | `addr` | 5 |
| 11 | ScheduleNext.frame.ConstructorFieldsF.saved_env.present `bit` [P:902-905] | `bit` | 1 |
| 12 | ScheduleNext.frame.ConstructorFieldsF.saved_env.cell `addr` [P:902-905] | `addr` | 5 |
| 13 | ScheduleNext.frame.ConstructorFieldsF.origin `u32` [P:902-905] | `u32` | 2 |
| 14 | ScheduleNext.next(expr ROM) `u32` [P:902-905] | `u32` | 2 |
| 15 | ScheduleNext.env.present `bit` [P:902-905] | `bit` | 1 |
| 16 | ScheduleNext.env.cell `addr` [P:902-905] | `addr` | 5 |

#### Image header scalar registers, lanes 19-47: 149 words (145 without lane 47)

| lane | register | Rust type | BabyBear type | words | source |
|---:|---|---|---|---:|---|
| 19 | control_origin | usize | `u32` | 2 | expr ROM row, cap P:1204-1208 |
| 20 | env_root.present | EnvRootC=Option<usize> | `bit` | 1 | P:1360, S:725 |
| 21 | env_root.cell |  | `addr` | 5 | env cell heap, cap P:1209 |
| 22 | stack_limit | usize | `u64` | 4 | u64 in input_d (RESULTS.md:103); cap P:1210 |
| 23 | outcome.kind | OutcomeC | `tag(3)` | 1 | 5 kinds, P:1294-1312, S:1041-1057 |
| 24 | outcome.payload1 (Halted ref tag bit; Rejected u8; Exhausted tag(3) of 8, P:400-410; Waiting u16) |  | `u16` | 1 | P:1294-1312 |
| 25 | outcome.payload2 (Halted ref idx) |  | `addr` | 5 | P:1297-1299 |
| 26 | initial_fuel | u128 | `u128` | 8 | P:1363, P:1165 |
| 27 | remaining_work | u128 | `u128` | 8 | P:1364, P:1166 |
| 28 | consumed_work | u128 | `u128` | 8 | P:1365, P:1167 |
| 29 | helper_work | u128 | `u128` | 8 | P:1366, P:1168 |
| 30 | helper_visits | u128 | `u128` | 8 | P:1367, P:1169 |
| 31 | reference_cursor | u128 | `u128` | 8 | P:1368, P:1170 |
| 32 | alloc.data_nodes | usize (AllocCounters, S:727-731) | `addr` | 5 | P:1268-1272 |
| 33 | alloc.data_fields | usize | `addr` | 5 | P:1268-1272 |
| 34 | alloc.environment_cells | usize | `addr` | 5 | P:1268-1272 |
| 35 | active.data | usize (ActiveLengths, S:733-738) | `addr` | 5 | P:1273-1278 |
| 36 | active.data_fields | usize | `addr` | 5 | P:1273-1278 |
| 37 | active.env | usize | `addr` | 5 | P:1273-1278 |
| 38 | active.value_slots | usize | `addr` | 5 | P:1273-1278 |
| 39 | seg_active.data | usize | `addr` | 5 | P:1371 |
| 40 | seg_active.data_fields | usize | `addr` | 5 | P:1371 |
| 41 | seg_active.env | usize | `addr` | 5 | P:1371 |
| 42 | seg_active.value_slots | usize | `addr` | 5 | P:1371 |
| 43 | seg_alloc.data_nodes | usize | `addr` | 5 | P:1372 |
| 44 | seg_alloc.data_fields | usize | `addr` | 5 | P:1372 |
| 45 | seg_alloc.environment_cells | usize | `addr` | 5 | P:1372 |
| 46 | seg_consumed_work | u128 | `u128` | 8 | P:1373, P:1175 |
| 47 | frame_count | usize (frames.len()) | `u64` | 4 | P:1374; = StackCells(frames).count u64 (RESULTS.md:103) |

#### Note on counters and the one-arena `len`

The design text does not say that any of `alloc`, `active` or `seg_*` is redundant with `len`. What it says:
- The INIT table keeps them as state rows: ARCH:2766 "23-25 | allocation counters (data nodes, data fields, environment cells)", ARCH:2767 "26-29 | per-kind active lengths, where the machine keeps them as registers", ARCH:2768 "30-32 | segment counters | 0".
- The CORE state list puts "allocation counters" among the "logical counters" and copies `len` separately "from the MEM frontier" (ARCH:1001).
- MEM owns `len`, and the one arena has kind-tagged records (ARCH:1259).

Two facts in the code argue that the per-kind counters stay registers:
- `len` is one total over the kind-tagged arena, so a per-kind count cannot be read from it.
- The reference caps each per-kind counter against its own limit (P:1214-1227), and the outcome has per-kind exhaust reasons `DataNodes`, `DataFields`, `EnvironmentCells` (S:367-371, P:403-405).

At most a sum identity such as `len - len_0 = alloc.data_nodes + alloc.data_fields + alloc.environment_cells` could hold. It is not verified here, because `value_slots` and the physical cells of each record (AD-11, ARCH:3438) are not settled. The question is therefore **OPEN** (item O-2 below).

### A. OPEN lanes

Word-affecting (the delta is against the 431 base):

- **O-1 Helper lane 4 `parent_resume` payload.** `Option<usize>`, and the source says "Bounded width DEFERRED to X-05, not frozen here" (S:745-746). Its lane holds only this field in every variant.
  - Options: `u32` (base), `u64` (+2), `addr` (+3).
  - The script shows 433 for `u64` and 434 for `addr`.
- **O-2 Counters, lanes 32-45** (14 lanes). Reference type `usize` (S:727-738), which is not a word type.
  - Width:
    - (a) `addr`, 5 words (base). A run-wide count can exceed `2^64` because a run can append `3 x 2^64` cells (AD-1, ARCH:3438).
    - (b) `u64`: -14 words, total 417.
    - (c) `u32` for the `seg_*` lanes only, if a segment is bounded.
  - Presence:
    - (a) keep all 14 (base);
    - (b) drop `active` (4 lanes, -20), if the one arena makes it equal to `alloc` (the "where the machine keeps them" wording at ARCH:2767);
    - (c) replace the three `alloc` lanes by a sum identity with `len`. This is not valid as shown above unless the per-kind caps are dropped.
- **O-3 Outcome lanes 23-25** (7 words). The INIT row says "`outcome_tag`, `outcome_payload` (header)" (ARCH:2757), but the reference keeps the outcome in the image body.
  - Options: in the body (base, 431) or in the header (424).
  - A related point: ARCH:2757 names one payload. The reference needs two lanes, because `Halted` carries a RefC (P:1297-1299).
- **O-4 `view_root`.** ARCH:2776 calls it a "header word", while ARCH:139, :1027, :2794 and :3454 call it "the `view_root` body word". It is not in the reference image. Options: header (base, 0) or body (+8).
- **O-5 INIT rows that the reference image does not have** (not in the total):
  - scratch `v0..v2` (ARCH:2765, "the value is never read");
  - "constructor fold cells" (ARCH:2774);
  - "input reference" `(present, val_d, in_ty)` (ARCH:2775), which would be bit + digest + u32 = +11 if it is a body word;
  - `deps_d` and `input_d` are header words (ARCH:2772-2773) and are left out of the body.

No word effect (the type name is open, the width is fixed):

- **O-6** Helper `Match.x`, `Match.y` (lanes 13-14). They are capped as names only when `phase != Arm` (H:743-749), so they are unconstrained in phase `Arm`. Typed `u32`.
- **O-7** `u8` codes are `tag(8)` against `u16`. Both are 1 word.
- **O-8** Lane 47 `frame_count` keeps a lane of its own in the image or is the stack `count` (counted once here; +4 if both are kept).
- **O-9** Helper `staged.payload` (lane 16, Scalar). Its widest variant is `U128Value { n: u128 }` (H:191-204, S:750), so it is `u128`. A design that moves that payload into `reg_a` would save 7 words. This is not proposed in any text.

Count: 9 OPEN items. 5 affect the word count (O-1 to O-5) and 4 do not (O-6 to O-9).


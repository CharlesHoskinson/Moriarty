## ADDED Requirements

### Requirement: FLG-001 Log entries and the frozen operation list
A log entry SHALL be `e = (op, inputs, outputs)`. Each Fr value SHALL be 9 limbs of 30 bits with `dec(x) < r`. The operation list SHALL be `PERM`, `H2`, `SPONGE(n)`, `PATH(d)`, `ECMUL`, `ECMULG`, `PUB(i)`, `DIGEST` and `NOP`. `FADD`, `FMUL`, `FINV` and `ECADD` SHALL NOT be deferred.

#### Scenario: Aliased value
- **WHEN** a logged value has limbs that recompose to `x + r`
- **THEN** the wrap refuses it (mutant `wrap_log_value_noncanon`), and `alias_without_canon` (proved at `bdb92973a`) shows that the check is needed

### Requirement: FLG-002 The onion digest chains through the run
The log digest SHALL be the tagged onion in Poseidon2 over BabyBear: `d_0 = P2(TAG_EMPTY)` and `d_(i+1) = P2(tagStep, d_i, P2(tagEnt, enc(e_i)))`. The pair `(d, n)` SHALL be carried in the boundary state through leaves, continuations, suspension and resume. Nodes SHALL merge child digests by `TAG_NODE`.

#### Scenario: Stop and resume
- **WHEN** a run is suspended and resumed from a checkpoint
- **THEN** the resumed log digest equals the digest of the uninterrupted run, and `onion_empty` (proved at `bdb92973a`) gives `d_0` as a hash output

### Requirement: FLG-003 The log capacity is a set of wrap key constants
The capacity SHALL be four wrap key constants: `B` entries, `V` logged values, `P` permutation slots and `E` Jubjub slots. The values SHALL be `B = V = P = 256` and `E = 0` (OD-6, OD-17).

#### Scenario: Capacity in a universal key
- **WHEN** the wrap key is generated
- **THEN** `B`, `V`, `P` and `E` are constants of the key and do not depend on the shard plan

### Requirement: FLG-004 The `pv` encoding has a fixed length
`pv` SHALL be an entry table of `B` rows `(op, param)` and a value table of `V` rows of 9 limbs, so `W_pv = 2B + 9V` = 2,816 words (OD-17). Offsets SHALL NOT be words of `pv`. Unused entry rows SHALL be `NOP` with `param` 0. No used entry SHALL follow a `NOP`. Unused value rows SHALL be zero.

#### Scenario: Padding forged
- **WHEN** a used entry follows a `NOP` or an unused value row is not zero
- **THEN** the wrap refuses the witness (mutants `wrap_nop_then_used`, `wrap_value_padding_nonzero`), and the encoding of a log stays unique (ZK-L2, open)

### Requirement: FLG-005 Bounds per operation
Each operation SHALL use the value, permutation and Jubjub slots of the table of `FD/ARCHITECTURE.md` section 1.8. `SPONGE(n)` SHALL have `1 <= n <= V - 1`. `PATH(d)` SHALL have `1 <= d <= 32`. `ECMUL` and `ECMULG` SHALL NOT be admitted while `E = 0`. `PUB(i)` SHALL have `0 <= i < 2 + L_TX`. A `DIGEST` tag SHALL come from the frozen tag list (HDM-002).

#### Scenario: Jubjub entry with no slot
- **WHEN** a run asks for `ECMUL` while `E = 0`
- **THEN** the planner proves it inline on JUBJUB-EMU and puts no entry in the log

### Requirement: FLG-006 The reservation `R` comes first
The reservation `R` SHALL be the anchor reservation `R_A` plus the public-digest `DIGEST` entries of the statement. `R` SHALL depend only on the mode and the statement. The planner, `L` and the wrap SHALL place `R` first. Keygen SHALL refuse a capacity that does not hold the largest `R` of 4 entries, 8 values and 4 permutations.

#### Scenario: Full log and a mandatory entry
- **WHEN** the discretionary operations of a run would fill every slot
- **THEN** `R` keeps its slots, the wrap finds every entry of `R` (LW-004), and no mandatory entry goes to tier E

### Requirement: FLG-007 Operations that do not fit spill to tier E
A discretionary operation SHALL enter the log only if every limit and its bound still hold for `R` and the discretionary set together. Every other discretionary Fr operation SHALL be proved inline by tier E. Such an operation SHALL NOT be refused. A consumed claim SHALL close in a discharge shard of a fixed leaf shape.

#### Scenario: Overflow
- **WHEN** a run asks for more Fr operations than the log holds
- **THEN** the overflow is proved by tier E chips, the claim is the same as for any other split (`spill_equiv`, `discharge_sound`, proved at `bdb92973a`), and GP-001 keeps no bound on Fr work

### Requirement: FLG-008 `L` binds `pv` before the first challenge
`L` SHALL recompute the digest from the private flattened log and its tree shape. `L` SHALL bind each table of `pv` to its part of the public-value vector by its own multilinear extension. The Fr transcript of `L` SHALL absorb all of `pv` before the first challenge.

#### Scenario: Late or partial absorb
- **WHEN** a transcript absorbs `pv` after a challenge, or absorbs only a part of it first
- **THEN** the key fails `vkL_ok` (`late_pv_rejected`, `partial_pv_rejected`, proved at `bdb92973a`)

### Requirement: FLG-009 Lean obligations of the field log
The field log SHALL be reported proved only when the two-table encoding is part of a built `wrap_refines` (BR-L2) and ZK-L2 is proved, each with an axiom report that holds only `propext`, `Quot.sound` and `Classical.choice`. Premises A3 (`rootBinds`), A4 (`recSound`) and A5 (`p2Inj`) SHALL stay named hypotheses.

#### Scenario: Proved theorems
- **WHEN** the status is reported
- **THEN** `pvOf_inj`, `encode_append_inj`, `pad_length`, `digest_inj`, `onion_empty`, `canon_check_sound`, `bounded_quotient_sound`, `unbounded_quotient_unsound`, `runWith_all_valid`, `spill_equiv`, `discharge_sound` and `pv_binding_count` are built at `bdb92973a` over a one-table `pv` zero-padded to `pvLen`

#### Scenario: Open obligations
- **WHEN** the open obligations are listed
- **THEN** `encode` on the two-table `pv` (part of BR-L2) and ZK-L2 are open

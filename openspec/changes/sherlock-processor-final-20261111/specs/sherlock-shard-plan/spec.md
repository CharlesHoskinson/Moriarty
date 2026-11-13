## ADDED Requirements

### Requirement: SPL-001 The layer stack
The co-processor SHALL prove a run in the layers of `FD/ARCHITECTURE.md` section 1.6: the per-program plan, leaf shards, nodes, `L`, the wrap and the ledger check. Leaves, nodes and `L` SHALL be STARKs over BabyBear with `EF = BabyBear^4`. The wrap SHALL be midnight-proofs PLONK with KZG over BLS12-381 and the only layer that hides.

#### Scenario: Field change
- **WHEN** a run is proved end to end
- **THEN** the field changes from BabyBear to BLS12-381 Fr only between `L` and the wrap

### Requirement: SPL-002 The static plan is witness-free and once per program
`sherlock plan` SHALL run once per program with no witness. It SHALL output the static plan: the instruction tables, the shape menu, the scheduling policy, the log layout rule with `R` first, the placement rule of discharge shards and of tier E and GP-3 chips, and the per-program keys under `key_root`. It SHALL NOT output the cut points, tree or shard count of a general run (SPL-011, LEAD FZ-5).

#### Scenario: Policy content
- **WHEN** the scheduling policy of the static plan is read
- **THEN** it holds the cut points as a function of `N_e` for Stage 1 Compact, and for general programs the per-rule maximum table, the cut rule and the list of admitted padded shard counts

#### Scenario: Two witnesses of one program
- **WHEN** the plan is run for one program with two different inputs in mind
- **THEN** the plan and `plan_d` are byte-equal

### Requirement: SPL-003 `prog_d` binds the plan
`prog_d` SHALL be `H2(DOM_PROG, image_d, plan_d, key_root, machine_version)` under `sherlock-value/2`. `plan_d` SHALL follow SPL-012. A public program SHALL publish `(prog_d, r_P)` in its manifest. A private program SHALL keep both secret.

#### Scenario: Plan swapped
- **WHEN** a prover uses keys of another plan for the same image
- **THEN** the key commitment has no inclusion path into `key_root`, and the node refuses the child

### Requirement: SPL-004 Per-program keys and universal keys
Per program, the app key, the OpenVM leaf-layer key and the internal-for-leaf key of each chip set SHALL be committed under `key_root`, with the Fr commitment `X` of the child DAG per entry and tree path. The internal-recursive key, the `L` key and the wrap key SHALL be the same for every program. The wrap key, the `L` key and the log capacity SHALL NOT depend on the plan.

#### Scenario: Byte-equal universal keys
- **WHEN** two programs with two chip sets are planned
- **THEN** their internal-recursive keys are byte-equal (frontend M2a)

### Requirement: SPL-005 Key commitments are checked against `key_root`
On the full tree the first universal node SHALL check each child key commitment by a Poseidon2 inclusion path into `key_root`. It SHALL forward the root. On path C, `L` SHALL prove the inclusion of `P2(DOM_KEYDAG, entry, path, enc(X))` with `X` public in `L`. The wrap SHALL check `X` (LW-006).

#### Scenario: Child under an unlisted key
- **WHEN** a child proof is made under a key that `key_root` does not list
- **THEN** the node refuses the child, and on path C the wrap refuses `X`

### Requirement: SPL-006 Shards have fixed shapes from the plan menu
Every leaf and node SHALL have a fixed shape from the plan menu. Nodes SHALL have a fixed arity. Work SHALL grow by adding shards. No key constant SHALL bound the run length or the Fr work (CPR-002).

#### Scenario: Shape outside the menu
- **WHEN** a prover offers a leaf of a height that the plan menu does not list
- **THEN** the node refuses the child

### Requirement: SPL-007 The STARK engine and its production settings
Leaves, nodes and `L` SHALL be proved by OpenVM stark-backend v2.1.0 with its CUDA prover (owner, 2026-11-11). Plonky3 0.9.0-rc.2 with Plonky3-recursion SHALL be the second choice. Production SHALL set `zerocheck_save_memory = false` per AIR set. A program key SHALL list only the AIRs that its leaves use.

#### Scenario: Generic child key with absent AIRs
- **WHEN** a node verifies a child under a generic key with absent preprocessed AIRs
- **THEN** the verifier rejects the node proof with `SumClaimMismatch` (MEASURED)

### Requirement: SPL-008 Machine version
The `R_SL_*` family and its chips SHALL define the machine version `sherlock-k-a/2`. `prog_d` SHALL bind `machine_version`. New rules and chips SHALL change only per-program keys.

#### Scenario: Rule family grows
- **WHEN** a new rule is added to the machine
- **THEN** the keys of old programs stay valid and no universal key changes

### Requirement: SPL-009 What freezes with the universal keys
The log operation list, the capacity with the encoding of `pv` (OD-6, OD-17), the limb encoding, `L_TX` and the public-table layout (OD-7, OD-15), the anchor template words and the anchor statement (OD-16), the hash domains (OD-18), the per-layer parameter sets and shape bounds (OD-4, OD-19), the public value `X` and the constant fields of the internal layer SHALL freeze with the universal keys, and only after the security gate passes (SQG-011) and FP-8 closes (FPR-019).

#### Scenario: Partial freeze
- **WHEN** one of these items changes after the universal keys freeze
- **THEN** a new universal key set is needed, and the change is refused while the old set is in use

### Requirement: SPL-010 Lean obligations of the shard plan
The plan binding SHALL be reported proved only when BR-L2 adds the DAG key check to `wrap_refines`. Premise A4 (`recSound`) SHALL stay a named hypothesis.

#### Scenario: Obligation status
- **WHEN** the status is reported
- **THEN** the DAG key check in `wrap_refines` (part of BR-L2) is open, and no other Lean obligation is listed for this module in the design

### Requirement: SPL-011 The per-run schedule is outside `plan_d`
The prover SHALL make the per-run schedule from a metered run under the scheduling policy: the cut points, the tree, the padded shard count from the admitted list, the positions of the discharge shards, and each suspension and resume. The schedule SHALL stay on the user's machine. Soundness SHALL NOT depend on it: each leaf shape SHALL be checked against `key_root`, and every schedule of one run SHALL give one `claim_pub` (LEAD FZ-5).

#### Scenario: Schedules under one plan
- **WHEN** one program runs at two run lengths, in two partitions of one run, and once with a suspension and resume, under one `plan_d`, one `key_root` and the universal keys
- **THEN** every run is accepted, the runs of one input give one `claim_pub`, and the longer run keeps its `prog_d` (positive `pos_plan_schedules`)

#### Scenario: Shape outside the menu
- **WHEN** a schedule uses a leaf shape that is not in the shape menu
- **THEN** the node refuses the leaf (mutant `plan_schedule_outside_policy`)

### Requirement: SPL-012 The preimage of `plan_d`
`plan_d` SHALL be `P2(DOM_PLAN, 1: u16, itab_d, menu_d, policy_d, layout_rule_d, place_d)`, each digest the `P2` hash of the canonical word list of one part of the static plan, in that order. `plan_d` SHALL NOT bind a per-run schedule. The word lists and the code of `DOM_PLAN` are FREEZE-PENDING ENC-5.

#### Scenario: Schedule in the plan digest
- **WHEN** a planner writes cut points of one run into `policy_d`
- **THEN** two runs of different length get two `plan_d`, and the plan check refuses it (mutant `plan_d_binds_schedule`)

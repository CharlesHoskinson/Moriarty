# Capability: sherlock-vm (machine, layout, dispatch, host interface)

Status: CONSENSUS v2.1 (council vote 6/6 ACCEPT at 2026-11-07 21:29Z; text unchanged from DRAFT v2.1). Refines UIREQ-001..035 and UISEC-001..012 without renumbering them. Amended 2026-11-10 by the C6 outcome-only review (`research/sherlock-c6-outcome-ledger9-review-20261110/review.md`: SVM-010..SVM-012 added); not council text. Design: `../../design.md` §2.

## ADDED Requirements

### Requirement: SVM-001 Fixed reference machine
The interpreter SHALL evaluate every admitted program with the fixed `sherlock-k-a/1` event table and the `sherlock-helper-microstep/1` owner rules, with no program-specific rule and no source unrolling.

#### Scenario: Three programs, one machine
- **WHEN** P1 `fact(3)`, P2 `list123` and P3 (written after key freeze) run under one committed qualification host config through `mori simulate`.
- **THEN** each halts with its reference value and its owner-step event stream names only inventoried `RuleIdC` rules.

### Requirement: SVM-002 StepInv between canonical boundaries
While a run is between canonical boundaries, every accepted step SHALL preserve `StepInv`; canonical decoding SHALL apply only at genesis, chunk store, resume and external claims.

#### Scenario: Publication padding challenge
- **WHEN** a raw witness copies Return reference lanes into Halted with a mutated reference tag or nonzero control padding (astra-2 challenge, `c6_admin_publish_step.rs:88`).
- **THEN** the keyed leaf refuses, or a checked invariant argument shows the state is unreachable from canonical genesis; the record names which.

### Requirement: SVM-003 One-hot dispatch derived from the pre-state
When a leaf proves a step, it SHALL derive a one-hot selector over the supported rules in Core's precedence (P0, P1, P2, `owner_select`) using every value read the guards make, and SHALL gate each fragment and each post-lane write by that selector.

#### Scenario: Equal helper value flip
- **WHEN** two pre-states agree on control tag, site, top frame, helper kind and result presence but differ in the compared UInt values so that Core selects `BHelperEqualDifferent` for one and `BHelperEqualPair` for the other (`c6_dispatch_step.rs:750-806`).
- **THEN** the selector bit differs accordingly and a witness asserting the other rule is refused.

### Requirement: SVM-004 Unsupported rule refusal
If an enabled step's Core selection is a rule outside the supported set, the leaf SHALL be unsatisfiable rather than accept any fragment; a disabled slot SHALL accept only the constrained identity with zero `steps` contribution.

#### Scenario: Financial rule in a pure profile
- **WHEN** a pre-state's Core selection is an `XFin*` or `XAdtHelperPlan*` rule with `enable == 1`.
- **THEN** `sum(sel) == 0`, the leaf refuses, and the native inventory reports the program unsupported.

#### Scenario: Disabled padding slot
- **WHEN** a slot has `enable == 0` and a witness sets `post != pre` or a nonzero step contribution.
- **THEN** the leaf refuses; with `post == pre` it accepts and `steps` is unchanged.

### Requirement: SVM-005 Identity commits layout and bounds
`prog_d` SHALL commit the program image, the layout schema digest, the qualification bounds and the profile id; key manifests SHALL record layout, bounds, ZKIR header, compiler version, zkir binary version, zk-stdlib/circuits/proofs versions, SRS and k.

#### Scenario: Same program, different caps
- **WHEN** one program image is admitted under frames=16 and under frames=256.
- **THEN** the two `prog_d` values differ and a `SherlockRun` sealed to one refuses a decider proof built under the other.

### Requirement: SVM-006 Host checks have no acceptance authority
If a host-side check or a complete host record is offered as acceptance authority for a constraint, proof or ledger claim, the assurance checker SHALL refuse it; public digests are permitted statement components only because the circuit constrains them.

#### Scenario: ProofPreimage check cited as soundness
- **WHEN** an evidence record cites a `ProofPreimage::check` pass for a constraint-rejection claim.
- **THEN** SAS-001 labelling rejects the record.

#### Scenario: Unconstrained public lane
- **WHEN** a statement lane is published with `Impact` but no `ConstrainEq` to the witness (`sherlock_c6_child.rs:598-608` today).
- **THEN** SRC-003 refuses the leaf as a qualified producer.

### Requirement: SVM-007 Single application producer and digest parity
When the owner engine runs a program for proving, the witness stream SHALL be the single application producer for the leaf prover and the digest-parity check, in-circuit digests SHALL equal host digests, and independent expected traces and raw hostile-witness entry points SHALL remain separate from the stream.

#### Scenario: Parity on every step
- **WHEN** `sherlock_digest_parity` runs on the P1-P3 streams at the new pins.
- **THEN** every pre and post digest matches the circuit `TransientHash` arity-4 tree.

### Requirement: SVM-008 No fragment, no key; inventory frozen before P3
If a rule in the frozen admitted inventory has no inventoried fragment with a compiled-and-run receipt, no leaf SHALL be keyed; the inventory, layout, caps and manifests SHALL be frozen before P3 is authored.

#### Scenario: Fresh program needs no key
- **WHEN** P3 is authored from the admitted domain after the freeze.
- **THEN** it proves under the existing leaf, node and decider keys (E2); if it fires a rule outside the inventory, it is unsupported and the freeze stands.

#### Scenario: Inventory gaps at the base commit
- **WHEN** `mori simulate --native-inventory` runs fact3 at `8fd34ed79`.
- **THEN** it reports the gaps `BHelperEqual` x4, `BHelperValidate` x4, `KAdminPublishValue`, and the leaf keygen task is blocked until the gaps are receipted.

### Requirement: SVM-009 Profile /1 only
While this change's qualification runs, `sherlock-helper-microstep/1` SHALL be the only proved profile; `/2` images or tags SHALL be refused by the leaf.

#### Scenario: Await tag in a /1 leaf
- **WHEN** a witness carries control tag 11, outcome tag 4 or exhaust reason 7.
- **THEN** the leaf refuses (SI-02/SI-20) and the record points to the later `/2` change.

### Requirement: SVM-010 Opaque addresses
(Added 2026-11-10 by the C6 outcome-only review, `research/sherlock-c6-outcome-ledger9-review-20261110/review.md:158`.) No machine rule SHALL compute a UInt, a Bool, a branch condition, an error code, an exhaust kind or a published value from a memory address, from `len`, `fp`, `tp` or a segment index. When a leaf closes, fuel and every logical resource counter SHALL remain unchanged. If physical proving capacity is unavailable, then the prover SHALL NOT encode that condition as a `Rejected` or `Exhausted` outcome.

#### Scenario: Address space exhausted
(drafted by lead from review.md §Q1 agreement 3)
- **WHEN** a run needs more segments than the address space of D3 holds.
- **THEN** the prover makes no proof, and no `Rejected` or `Exhausted` outcome is published for the run.

#### Scenario: Counter changed at a leaf close
(drafted by lead from review.md §Q1 verdict 2)
- **WHEN** a leaf witness changes fuel or a logical resource counter at the leaf close.
- **THEN** the leaf refuses it.

### Requirement: SVM-011 Refinement of the logical state
(Added 2026-11-10 by the C6 outcome-only review, `research/sherlock-c6-outcome-ledger9-review-20261110/review.md:160`.) When an enabled chip step executes, its decoded logical post-state SHALL equal the reference transition from its decoded logical pre-state. When a leaf closes, its decoded logical state SHALL remain unchanged.

#### Scenario: Reference without per-leaf allocation
(drafted by lead from review.md §Q1 verdict 5)
- **WHEN** the Quint refinement model runs with a reference that does not allocate per leaf.
- **THEN** every enabled chip step refines one reference step, a leaf close is a stutter step, and each seeded mutant is caught (row F25).

### Requirement: SVM-012 Value digest at creation
(Added 2026-11-10 by the C6 outcome-only review, `research/sherlock-c6-outcome-ledger9-review-20261110/review.md:162`.) When a step creates a data value, it SHALL compute that value's `vd` from its tag and the authenticated `vd` of its fields and SHALL bind it to the value's write-once record. A digest supplied by the host without that constraint SHALL NOT be accepted.

#### Scenario: Forged digest
(drafted by lead from review.md §The canonical encoding and its cost)
- **WHEN** a witness binds a `vd` to a new record that differs from the digest of its tag and its authenticated child digests.
- **THEN** the leaf refuses it (row A39).

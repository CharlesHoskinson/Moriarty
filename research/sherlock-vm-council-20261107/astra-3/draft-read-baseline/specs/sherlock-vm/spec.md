# Capability: sherlock-vm (machine, layout, dispatch, host interface)

Status: DRAFT v1, specified-only. Refines UIREQ-001..035 and UISEC-001..012 without renumbering them. Design: `../../design.md` §2.

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
If Core would select a rule outside the supported set, the leaf SHALL be unsatisfiable rather than accept any fragment.

#### Scenario: Financial rule in a pure profile
- **WHEN** a pre-state's Core selection is an `XFin*` or `XAdtHelperPlan*` rule.
- **THEN** `sum(sel) == 0` under `enable == 1`, the leaf refuses, and the native inventory reports the program unsupported.

### Requirement: SVM-005 Identity commits layout and bounds
`prog_d` SHALL commit the program image, the layout schema digest, the qualification bounds and the profile id; key manifests SHALL record layout, bounds, ZKIR header, compiler version, zkir binary version, zk-stdlib/circuits/proofs versions, SRS and k.

#### Scenario: Same program, different caps
- **WHEN** one program image is admitted under frames=16 and under frames=256.
- **THEN** the two `prog_d` values differ and a `SherlockRun` sealed to one refuses a decider proof built under the other.

### Requirement: SVM-006 Host records are never evidence
If a host-side record is offered as a public statement component or as circuit, proof or ledger evidence, the assurance checker SHALL refuse it.

#### Scenario: ProofPreimage check cited as soundness
- **WHEN** an evidence record cites a `ProofPreimage::check` pass for a constraint-rejection claim.
- **THEN** SAS-001 labelling rejects the record.

### Requirement: SVM-007 Single witness producer and digest parity
When the owner engine runs a program for proving, the witness stream SHALL be the single producer for the leaf prover, the holdout reference and the digest-parity check, and in-circuit digests SHALL equal host digests.

#### Scenario: Parity on every step
- **WHEN** `sherlock_digest_parity` runs on the P1-P3 streams at the new pins.
- **THEN** every pre and post digest matches the circuit `TransientHash` arity-4 tree.

### Requirement: SVM-008 No fragment, no key
If a rule fired by a program has no inventoried fragment with a compiled-and-run receipt, the program SHALL be reported unsupported and no leaf SHALL be keyed for it.

#### Scenario: Inventory gaps at the base commit
- **WHEN** `mori simulate --native-inventory` runs fact3 at `8fd34ed79`.
- **THEN** it reports the gaps `BHelperEqual` x4, `BHelperValidate` x4, `KAdminPublishValue`, and the leaf keygen task is blocked until the gaps are receipted.

### Requirement: SVM-009 Profile /1 only
While this change's qualification runs, `sherlock-helper-microstep/1` SHALL be the only proved profile; `/2` images or tags SHALL be refused by the leaf.

#### Scenario: Await tag in a /1 leaf
- **WHEN** a witness carries control tag 11, outcome tag 4 or exhaust reason 7.
- **THEN** the leaf refuses (SI-02/SI-20) and the record points to the later `/2` change.

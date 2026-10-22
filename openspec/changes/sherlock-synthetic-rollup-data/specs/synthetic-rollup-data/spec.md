# Synthetic rollup data

Status: DRAFT / PROPOSED, 2026-10-21. All scenarios are proposed acceptance contracts, not execution receipts. This additive package preserves the [stagenet experiment](../../../sherlock-stagenet-recursive-rollup/requirements.md) (SRPOC-001 to SRPOC-009), the [benchmark EARS](../../../sherlock-moriarty-benchmarks/specs/benchmarks/spec.md) (SBENCH-001 to SBENCH-010) and the [product contract](../../../../../docs/MORIARTY-PRODUCT-CONTRACT.md).

## ADDED Requirements

### Requirement: SSRD-001 Pinned design cut

WHEN FSYN generates any artefact, the generator SHALL verify the byte count and SHA-256 of every entry of `design-cut.json` and SHALL refuse with exit 2 on any difference.

#### Scenario: SSRD-001-01 current cut

- **GIVEN** every pinned file matches its byte count and SHA-256
- **WHEN** `fsyn generate` runs
- **THEN** it records the cut digest in every `FIXTURE.json`

#### Scenario: SSRD-001-02 stale cut

- **GIVEN** `modules/DA-PROOF.md` changed after the cut
- **WHEN** `fsyn generate` runs
- **THEN** it exits 2, names the changed path, and writes no artefact

### Requirement: SSRD-002 Constants from executable sources

The generator SHALL read every rollup constant, field order and domain tag from the extracted `constants.json`, `layouts.json` and `rules.json`, and the extractor SHALL stop with exit 1 when two sources give different values for one constant.

#### Scenario: SSRD-002-01 agreeing sources

- **GIVEN** `GAP_MAX` is 256 in `cel_auth.qnt` and in CELESTIA-AUTH section 2
- **WHEN** the extractor runs
- **THEN** `constants.json` holds 256 with both citations

#### Scenario: SSRD-002-02 disagreeing sources

- **GIVEN** a Quint constant differs from the module spec table
- **WHEN** the extractor runs
- **THEN** it exits 1, names both sources, and writes no constants file

### Requirement: SSRD-003 Deterministic bytes

WHEN FSYN receives the same cut digest, generator version, seed, profile, scale and mix, it SHALL emit byte-identical artefacts on every supported host.

#### Scenario: SSRD-003-01 two hosts

- **GIVEN** two hosts with the pinned toolchain
- **WHEN** both generate scale S1 with one seed
- **THEN** both `MANIFEST.json` files are byte-identical

#### Scenario: SSRD-003-02 hidden nondeterminism

- **GIVEN** an emitter iterates an unordered map
- **WHEN** the two-host check runs
- **THEN** the check fails and names the first differing file

### Requirement: SSRD-004 Independent random streams

The generator SHALL draw each concern from its own named stream, and a change to one stream SHALL leave the artefacts of the other concerns byte-identical.

#### Scenario: SSRD-004-01 committee change only

- **GIVEN** the committee stream changes and the mix stream does not
- **WHEN** S1 regenerates
- **THEN** transaction records are byte-identical and committee signatures differ

### Requirement: SSRD-005 FRTF v3 reproduction

WHEN FSYN runs scale S0 with seed `0x5354524157`, it SHALL reproduce every FRTF v3 envelope and every `workloads.json` digest byte for byte, and `rtf verify` SHALL pass on its output.

#### Scenario: SSRD-005-01 byte identity

- **GIVEN** the pinned FRTF v3 tree
- **WHEN** FSYN writes S0 and `rtf generate` writes its own output
- **THEN** both trees are byte-identical and b1 profile A digest is `4946d258…488e27d`

#### Scenario: SSRD-005-02 drift

- **GIVEN** an FSYN emitter writes one field in another byte order
- **WHEN** the S0 comparison runs
- **THEN** G0 fails and no S0 fixture is released

### Requirement: SSRD-006 Scale labels

FSYN SHALL label each artefact with its scale S0 to S5, and a catalog case SHALL NOT carry the label of a larger scale than its artefacts.

#### Scenario: SSRD-006-01 relabel refused

- **GIVEN** an S1 fixture
- **WHEN** a catalog case cites it as S3
- **THEN** the catalog bridge exits 1

### Requirement: SSRD-007 Transaction mix coverage

The generator SHALL emit every leaf step kind and every forced outcome code in the selected preset, and SHALL record the realized count of each per batch.

#### Scenario: SSRD-007-01 S1 coverage

- **GIVEN** the `planning` preset at S1
- **WHEN** a batch is generated
- **THEN** MAIN, LATE (when nonzero), FORCED, DEPOSIT, SYS, CLOSE and all nine forced outcomes have counts in `FIXTURE.json`

#### Scenario: SSRD-007-02 cap exceeded

- **GIVEN** the `cap` preset with 513 forced entries requested in one batch
- **WHEN** generation runs
- **THEN** it exits 1 citing `FORCED_MAX` = 512

### Requirement: SSRD-008 Header and blob sizes

WHEN FSYN emits a header, it SHALL size `Records` from the preset, count shares by the sparse-share rule, and refuse a header above `MAX_HEADER_SHARES`.

#### Scenario: SSRD-008-01 planning mix

- **GIVEN** a 3,000,000-byte header payload
- **WHEN** shares are counted
- **THEN** the count is 6,225, as Lean `Da.Budget.share_counts` states

#### Scenario: SSRD-008-02 over the cap

- **GIVEN** a payload of 478 + 8,191 x 482 + 1 bytes
- **WHEN** the header is emitted
- **THEN** generation refuses it at 8,193 shares

### Requirement: SSRD-009 Sealed committee

The generator SHALL model 16 sealed seats with at most f = 4 Byzantine seats a run, SHALL never replace a seat in place, and SHALL switch the run to a migration scenario at 3 permanent seat losses.

#### Scenario: SSRD-009-01 equivocation below quorum

- **GIVEN** 4 Byzantine seats sign two contents at one `(anchor_epoch, h)`
- **WHEN** the validity replay runs
- **THEN** no second content reaches t = 12 and `P-VAL-4` holds

#### Scenario: SSRD-009-02 fifth Byzantine seat

- **GIVEN** a request for 5 Byzantine seats in a non-adversarial scale
- **WHEN** generation runs
- **THEN** it exits 1 citing T1

### Requirement: SSRD-010 Light-client gaps and covers

WHEN FSYN emits a light-client chain, every append SHALL satisfy `0 < B - pred(B) <= GAP_MAX`, and every pointer SHALL have exactly one covering stored header.

#### Scenario: SSRD-010-01 maximal gap

- **GIVEN** a skip of 256 blocks
- **WHEN** the append is replayed in `cel_auth.qnt`
- **THEN** `gapOk` and `coverage` hold

#### Scenario: SSRD-010-02 gap 257

- **GIVEN** a skip of 257 blocks outside the adversarial scale
- **WHEN** generation runs
- **THEN** it exits 1, and the same append in S5 is refused by `advanceCelestiaLC`

### Requirement: SSRD-011 Two profiles, one record set

FSYN SHALL emit the Profile A and Profile B views of a batch from one core record set, and both views SHALL agree on `pre`, `post`, `in_root`, `h_first`, `h_last` and every sum field.

#### Scenario: SSRD-011-01 agreement

- **GIVEN** S0 b1
- **WHEN** both views are emitted
- **THEN** the shared fields are equal and `final_post` is `f3b33453…`

#### Scenario: SSRD-011-02 divergence

- **GIVEN** a Profile B emitter that drops a deposit
- **WHEN** the cross-profile check runs
- **THEN** it fails on `dep_sum_usd`

### Requirement: SSRD-012 Fidelity map

Each artefact type SHALL have a fidelity row naming its interface operation, statement and Quint oracle, and FSYN SHALL NOT emit an artefact type without a row.

#### Scenario: SSRD-012-01 unmapped artefact

- **GIVEN** a new emitter without a fidelity row
- **WHEN** the fidelity lint runs
- **THEN** it exits 1 and names the emitter

### Requirement: SSRD-013 Forward Quint replay

WHEN FSYN emits a scenario, it SHALL emit a Quint replay test of the same actions, and SHALL publish the scenario only if `quint test` passes with every named invariant.

#### Scenario: SSRD-013-01 passing replay

- **GIVEN** an S1 DA miss scenario of reason 3
- **WHEN** its replay runs against `da_accept.qnt` and `liveness_exit.qnt`
- **THEN** `allInv` holds at every step and the scenario enters the manifest

#### Scenario: SSRD-013-02 failing replay

- **GIVEN** a scenario that settles a range without `stmt_da` in COMMITTEE mode
- **WHEN** its replay runs
- **THEN** the replay fails, the scenario goes to `failed/`, and G1 fails

### Requirement: SSRD-014 Reverse Quint replay

FSYN SHALL rebuild every sampled ITF trace of each oracle model through its own rules and compare every observable state variable.

#### Scenario: SSRD-014-01 inexpressible trace

- **GIVEN** a sampled trace with an action FSYN cannot express
- **WHEN** reverse replay runs
- **THEN** it reports a coverage gap with the action name and G1 stays open

### Requirement: SSRD-015 Independent acceptance oracle

The generator's own replay SHALL be labelled internal consistency, and fixture acceptance SHALL use only the pinned Quint models, Lean bounds, `rtf verify` and independently frozen expected states.

#### Scenario: SSRD-015-01 self-agreement only

- **GIVEN** a scenario that passes the reference replay and has no Quint replay
- **WHEN** release runs
- **THEN** the scenario stays unreleased

### Requirement: SSRD-016 Golden vectors

FSYN SHALL keep the golden vectors of design section 7.1, each bound to its source path, line or theorem and the cut hash of that source.

#### Scenario: SSRD-016-01 source change

- **GIVEN** `params-v3.json` changes its relation digest
- **WHEN** the cut check runs
- **THEN** the golden vector and every dependent fixture are stale

### Requirement: SSRD-017 Property tests with Lean bounds

For every property of design section 6.3, the suite SHALL run a property test over generated scenarios and SHALL assert the Lean-proved bound where one exists.

#### Scenario: SSRD-017-01 leaf rows

- **GIVEN** any generated `L_inc` leaf plan
- **WHEN** the P-DA-14 test runs
- **THEN** its modeled rows are at most 477,000 and below 2^19

#### Scenario: SSRD-017-02 counterexample

- **GIVEN** a generated forced queue that drains later than `drain_bound`
- **WHEN** the P-LIV-12 test runs
- **THEN** it fails, shrinks to a minimal seed, and saves a regression fixture

### Requirement: SSRD-018 Mutants killed

Every generator rule and every oracle check SHALL have a registered mutant, and a fixture release SHALL require each mutant to be killed by at least one test.

#### Scenario: SSRD-018-01 surviving mutant

- **GIVEN** the mutant that admits a pad before a real child survives
- **WHEN** release runs
- **THEN** the release stops and names the mutant

### Requirement: SSRD-019 Adversarial coverage

For each of the 14 adversary classes of the security model and the two exit attackers of the exit design note, FSYN SHALL emit at least one case with its expected failing layer, reason code, Quint invariant and killing mutant.

#### Scenario: SSRD-019-01 missing class

- **GIVEN** no case for the activation caller class
- **WHEN** the coverage check runs
- **THEN** G2 fails and names the class

#### Scenario: SSRD-019-02 out-of-model class

- **GIVEN** the Celestia class at 1/3 or more faulty power
- **WHEN** its case runs
- **THEN** validity holds, and the case records residual R6 with no data-availability claim

### Requirement: SSRD-020 Replayable fixtures

Each fixture release SHALL carry `MANIFEST.json`, `FIXTURE.json` with its regeneration command, and the oracle gate record, and `fsyn verify` SHALL reproduce it byte for byte.

#### Scenario: SSRD-020-01 tampered fixture

- **GIVEN** one byte of a committed fixture changed
- **WHEN** `fsyn verify` runs
- **THEN** it exits 1 and names the file

### Requirement: SSRD-021 Benchmark triggers

A benchmark phase SHALL start only after its trigger of design section 9 exists, and a missing trigger SHALL leave the phase open with no substitute workload.

#### Scenario: SSRD-021-01 no recursive proof yet

- **GIVEN** P3's trigger is absent
- **WHEN** a P3 benchmark is requested
- **THEN** the request is refused and P3 stays open

### Requirement: SSRD-022 Benchmark metrics

WHEN a benchmark runs, it SHALL report the SRPOC-005 metrics per leaf type and layer on the SBENCH /2 sample schema, with the FSYN fixture id and scale.

#### Scenario: SSRD-022-01 missing rows

- **GIVEN** the prover exposes no used-row count for a layer
- **WHEN** the sample is written
- **THEN** rows are `unavailable` with a reason, and k and capacity stay measured values

### Requirement: SSRD-023 Measured and projected values

WHEN a report gives a fleet size or a settled-TPS point, it SHALL label each value measured or projected, and SHALL derive projections only from measured unit costs.

#### Scenario: SSRD-023-01 modeled figure as input

- **GIVEN** a projection that uses the RX-52 planning band in place of a measurement
- **WHEN** the report check runs
- **THEN** it exits 1 and names the value

### Requirement: SSRD-024 Stale fixtures

IF the live design differs from the pinned cut, THEN FSYN SHALL mark every dependent fixture stale, and a stale fixture SHALL NOT enter a benchmark sample.

#### Scenario: SSRD-024-01 module spec revision

- **GIVEN** LIVENESS-EXIT moves past revision 7 and revision 20 is regenerated
- **WHEN** `fsyn cut check` runs in DoctorWatson
- **THEN** fixtures that depend on the changed groups are stale until `fsyn cut refresh` and regeneration

#### Scenario: SSRD-024-02 generated file out of sync

- **GIVEN** `build-rev20.py --check` reports a difference
- **WHEN** `fsyn cut check` runs
- **THEN** it exits 2 and no fixture is generated

### Requirement: SSRD-025 Exit kinds and one exit key

FSYN SHALL emit every exit kind of design section 4.4 (L2 spend, forced TRANSFER, forced WITHDRAW with claim, FENC_RELEASE, escape, import, shielded and unshielded payout, foreign-backed escape), and every exit SHALL consume exactly one exit key `xk`.

#### Scenario: SSRD-025-01 escape, import, then spend

- **GIVEN** a note escaped in FX12 and imported by the next settled batch
- **WHEN** a later L2 spend of the same `xk` is generated in an adversarial case
- **THEN** the spend is refused and `exit_core.qnt` `noDoublePay` holds, and `MutEscKeyHash` breaks it

#### Scenario: SSRD-025-02 claim leaf under another batch

- **GIVEN** a withdrawal leaf whose `batch_no` differs from its statement
- **WHEN** the replay runs
- **THEN** the fixed model refuses it and `MutLeafBatch` strands the withdrawal

### Requirement: SSRD-026 Holds at execution

The generator SHALL key each hold by `H(DOM_HT, nf)`, SHALL give each capability at most one hold, and SHALL decide each hold once at execution.

#### Scenario: SSRD-026-01 hold after enqueue

- **GIVEN** a forced TRANSFER enqueued before a consented hold on its note
- **WHEN** the batch drains
- **THEN** the outcome is `FENC_ENCUMBERED`, and `MutQueueTimeHold` pays the held note

#### Scenario: SSRD-026-02 one capability, two notes

- **GIVEN** an adversarial case binding one capability to two notes
- **WHEN** the replay runs
- **THEN** `oneCapOneHold` refuses the second hold

### Requirement: SSRD-027 Boundary disclosure

WHEN FSYN emits an exit, its `public.json` SHALL carry no shielded destination and no value that links the exit to L2 history, and SHALL carry the asset and amount (Q17); an unshielded payout SHALL also carry its destination (Q16).

#### Scenario: SSRD-027-01 shielded payout

- **GIVEN** a shielded claim
- **WHEN** the disclosure check runs
- **THEN** `public.json` holds `xk`, asset, amount and `xcm`, and no destination

#### Scenario: SSRD-027-02 leaked destination

- **GIVEN** an emitter that writes the shielded destination to `public.json`
- **WHEN** the disclosure check runs
- **THEN** it exits 1 and `shieldedPrivate` fails, as `MutShieldedPublic` shows

### Requirement: SSRD-028 Sealed-for-life lifecycle

The generator SHALL emit only key-preserving in-place rotations, and SHALL emit every key or roster change as a migration to a new deployment with wind-down exits of every old note.

#### Scenario: SSRD-028-01 key change requested

- **GIVEN** a scenario that changes the decider key in place
- **WHEN** generation runs
- **THEN** it exits 1 citing Q15, and the migration form of the scenario is the only one emitted

#### Scenario: SSRD-028-02 expired store reset

- **GIVEN** an expired light-client store and a key-preserving rotation
- **WHEN** `activate` replays in `lifecycle.qnt` (`lifecycle_sealed`)
- **THEN** a fresh reset header signed under `DOM_LCRESET2` is installed and no verifier key changes

## ADDED Requirements

### Requirement: SSG-001 D3 on every per-program key
SSG-001: When a per-program R_SL app key is generated, the D3 calculator SHALL print every term of the key at 100 bits or more.

Verification: `fp8_d3` over the app key of each of the 33 suite programs: "FP8D3 all gates: PASS" for each. Exists.

#### Scenario: The largest app
- **GIVEN** the app key of `d04 withdraw` (16,346 columns)
- **WHEN** `fp8_d3` reads the key
- **THEN** every term is at least 100 bits and every grindable term at least 101 bits

#### Scenario: Status of SSG-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP: no D3 record for R_SL app keys. Owner lane: rekey-chips, at T25. Evidence: Tool: openvm `t10mm` 5b7ada8 `crates/sherlock-wrap-export/examples/fp8_d3.rs:9` (usage); two-column rule at `fp8_d3.rs:84`. D3 ran on the 8 chain keys of T10m only (Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:91`, weakest leaf shift 101.0110). RKY-005 covers the universal keys.

### Requirement: SSG-002 The STARK stages hold no BLS12-381 circuit
SSG-002: The crates `sherlock-leaf` and `sherlock-lfr` SHALL have no dependency on a PLONK or KZG proving library.

Verification: NEW: `cargo tree -p sherlock-leaf -p sherlock-lfr` holds no `midnight-proofs` and no `halo2` crate; a line in the T25 gate table. To build.

#### Scenario: A dependency is added
- **GIVEN** a change that adds `midnight-proofs` to `sherlock-lfr`
- **WHEN** the T25 gate runs
- **THEN** the gate fails and names the crate

#### Scenario: Status of SSG-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP: true by construction, not checked. Owner lane: T25. Evidence: Leaves, nodes and `L` use BabyBear configurations (openvm `rekey-chips` dcf1b4c `crates/sherlock-leaf/src/harness.rs:26`, `crates/sherlock-lfr/src/fixture.rs:26`, `:28`); `L_Fr` is a STARK over Fr (openvm `rekey-chips` dcf1b4c `crates/sherlock-lfr/src/testing.rs:23`). The one circuit with a `k` is the wrap (WRP; CPR-001; DG-001). PCS: WHIR in unique decoding (openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/frozen.rs:234`).

### Requirement: SSG-003 A confirmed defect blocks the gate
SSG-003: If the counterexample record holds a row with verdict CONFIRMED and no fix commit, then the security gate SHALL report the gate as not passed.

Verification: NEW: a script that counts the verdicts of `COUNTEREXAMPLES.md` on the T25 tree; the T12 RUNBOOK calls it. To build.

#### Scenario: GP2-C2.1 at the gate
- **GIVEN** the record with GP2-C2.1 CONFIRMED
- **WHEN** the T12 gate runs
- **THEN** the gate reports not passed and names GP2-C2.1

#### Scenario: Status of SSG-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP: 1 CONFIRMED row is open (NAG-007). Owner lane: t12-gate. Evidence: Moriarty 3dc9001a9 `tests/quint-regressions/COUNTEREXAMPLES.md`: CONFIRMED 1 (GP2-C2.1), FIXED 18, GUARD HOLDS 22, NOT REPRODUCED 2, OPEN DESIGN 6, NOT ATTEMPTED 8. Moriarty `quint-reg` 2ad6f48da: CONFIRMED 7 before the fixes. LOOP loop rule "Counterexample for every Quint finding".

### Requirement: SSG-004 A finding without an attempt stays open
SSG-004: If the counterexample record holds a row with verdict NOT ATTEMPTED, then the security gate SHALL list the row as open in the gate record.

Verification: the count script of SSG-003. Exists.

#### Scenario: Eight open rows
- **GIVEN** the record of 2026-11-15
- **WHEN** the gate record is written
- **THEN** the record lists 8 open findings by id

#### Scenario: Status of SSG-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP: 8 rows. Owner lane: t13-cx. Evidence: Moriarty 3dc9001a9 `tests/quint-regressions/COUNTEREXAMPLES.md`: 8 rows NOT ATTEMPTED. LOOP loop rule: "a Quint finding counts only after someone tries to build its counterexample on the real code".

### Requirement: SSG-005 Every circuit change has a row in a change list
SSG-005: When a circuit change lands on a lane branch after openvm c4bd312, the tested-design loop SHALL add one row with the commit, the AIR and the Quint lane to the circuit change list.

Verification: NEW: the file `qa/gemini-quint/CHANGES.md`, checked against `git log c4bd312..` of the openvm T25 tree; each row links a `verdict.md`. To build.

#### Scenario: The FP-3 change
- **GIVEN** the FP-3 commit on openvm
- **WHEN** the change list is checked
- **THEN** the list has a row for FP-3 with a lane name, or the check fails

#### Scenario: Status of SSG-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP. `~/.cache/rekey-t11/LIST.md` is the nearest record and covers the rekey rows only. Owner lane: t13-lanes. Evidence: LOOP T13 names "every circuit change since openvm c4bd312"; no list of these changes exists. Lead verdicts exist for 5 lanes only (Moriarty `t13-lanes` 958a26919 `qa/gemini-quint/`: `c1-dense`, `chipleaf-final`, `chipleaf-hash`, `chipleaf-pairrow-m3`, `reads-result-3x`); `e3-iface`, `wrap-levers`, `lfr-*-r2`, `e5-decode`, `x14-pad`, `x14-identity` have a REPORT and no verdict; `t10m-merge`, `t10q-chunk`, `t10q-fixes`, `t16-sched-r2` have a prompt only. LPC-004, LPC-016.

### Requirement: SSG-006 The gate record names the key set
SSG-006: When G1 to G4, G5 and FP-8 pass on a key set, the security gate SHALL write a gate record with the key manifest sha256 and the log path of each check.

Verification: the T12 gate run on the T25 keys; the record file beside the RUNBOOK. Exists.

#### Scenario: The gate passes
- **GIVEN** the T25 keys with G1 to G5 and FP-8 passed
- **WHEN** the record is written
- **THEN** the key manifest status changes from `prototype` to `qualified`

#### Scenario: G5 names old parameters
- **GIVEN** G1 to G4 and FP-8 passed and a G5 mapping that names the T10m parameter sets
- **WHEN** the gate is evaluated
- **THEN** no gate record is written and the status stays `prototype`

#### Scenario: Status of SSG-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. Owner lane: t12-gate. Evidence: Moriarty `t12-gate` 90f9b4df9 `research/sherlock-tested-design-20261113/t12/RUNBOOK.md`: no step run; G3 legs not built at `:61` to `:63`; UNVERIFIED steps at `:77` and `:87`. Gate definitions G1 to G5: `research/sherlock-final-design-20261111/ARCHITECTURE.md:293` to `research/sherlock-final-design-20261111/ARCHITECTURE.md:297`; SQG-001 requires G1 to G4 with G5. SQG-015, SQG-016, UKS-007. G5 added after Astra-B review R-04.

### Requirement: SSG-007 The security claim follows the gate
SSG-007: While the security gate has not passed on the key set in use, the Sherlock prover SHALL report the security status `not qualified` in every run record.

Verification: NEW: the run record field `security`; checked in the T23 suite table. To build.

#### Scenario: A Preview run before T12
- **GIVEN** a run on prototype keys
- **WHEN** the run record is written
- **THEN** the field reads `not qualified`

#### Scenario: Status of SSG-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP. Owner lane: t23-suite. Evidence: `research/sherlock-final-design-20261111/ARCHITECTURE.md:267` ("Status: not qualified"), `research/sherlock-final-design-20261111/ARCHITECTURE.md:289`. SQG-001. Moriarty 3dc9001a9 `research/sherlock-final-design-20261111/TERMINOLOGY.md:63`.

# Decision index

MADR records of change `sherlock-e2e-prototype-20261115`, in the mapped MADR form of the software-factory profile (`skills/specs/spec-architect/references/madr-template.md`). A record binds only through the EARS IDs and checks of its Confirmation section. Status `accepted` states decision authority, not a passed check.

ID ranges (reserved before the slices wrote, single writer Fable-A): D-001 to D-029 integrator and slice A, D-030 to D-069 slice B, D-070 to D-109 slice C. An unused or cancelled ID is not reused.

| ID | Decision | Slice | Authority | Status | Legacy IDs | Binding EARS IDs (Confirmation) |
|---|---|---|---|---|---|---|
| [D-001](D-001.md) | No circuit above k = 21 | hard rule | OWNER | accepted | hard rule "never k = 22"; CPR-001; memory rule 2026-11-11 | RKY-003, CPR-001, MYC-CHIP-007, MYC-KEY-009, MYC-ACC-002, SSG-002 |
| [D-002](D-002.md) | Zero knowledge end to end | hard rule | OWNER | accepted | GP-ZK; ZK-001 to ZK-019; T11-1 to T11-4; memory rule 2026-11-11 | ZK-015, ZK-014, ERS-002, ZK-012, ZK-013, ZK-016, ZK-017, ZK-019, ZKE-001 to ZKE-007, PDM-005, BXF-001 |
| [D-003](D-003.md) | Proven 100-bit security | hard rule | OWNER | accepted | hard rule; OD-4; OD-19; G1 to G5 | RKY-005, SQG-014 to SQG-016, SQG-001 to SQG-013, SSG-001, SSG-003, SSG-004, SSG-006, SSG-007, LTR-003, NAG-005 |
| [D-004](D-004.md) | One universal key set | hard rule | OWNER | accepted | hard rule; CPR-003; GP-1 | RSL-001, EUI-002, EUI-003, EUF-009, RKY-001, RKY-002, MYC-KEY-001, MYC-KEY-002, MYC-SEAL-015, UKS-001 to UKS-010, LFP-002, LFP-008, LTR-007, ELA-002, GPH-001, PDM-006 |
| [D-005](D-005.md) | The ledger 9.1.0.1 verifier is unchanged | hard rule | OWNER | accepted | hard rule; OD-24 option E; CPR-004; MID-001 | STL-002, EUF-012, MYC-ACC-001, MYC-CHIP-008 |
| [D-006](D-006.md) | Wrap verifier key at most 50,000 bytes | hard rule | OWNER | accepted | hard rule; MYC-ACC-005 | RKY-004 |
| [D-007](D-007.md) | One BLS12-381 wrap over small-field STARK leaves and nodes | hard rule | OWNER | accepted | memory rule 2026-11-11 (small-field STARK); OD-24; hard rule "one BLS12-381 wrap" | STL-002, MYC-ACC-010, MYC-KEY-010, LW-001 to LW-022, LFP-001, NAG-001, LTR-001, LTR-002, SSG-002 |
| [D-008](D-008.md) | GP-1 to GP-4 are never narrowed | hard rule | OWNER | accepted | hard rule; CPR-005; memory rule (GP definition); O10 | LPC-001 to LPC-003, EUI-001, EUI-002, CPR-005, GPH-001 to GPH-008, LFP-003 |
| [D-009](D-009.md) | Midnight writes are batched | hard rule | OWNER | accepted | T20; memory rule 2026-11-14 | LPC-010 |
| [D-010](D-010.md) | Case A only: proving on boxes the user owns | hard rule | OWNER | accepted | OD-9; ZK-018; wiki log 2026-11-15 09:05 | ZK-018, ERS-002, BXF-001, ZKE-004, ELA-006, UKS-005 |
| [D-011](D-011.md) | Chunk cap K_MAX = 2 | hard rule | OWNER | accepted | T20; ANC-012 | ANC-004 to ANC-006, ANC-008, ANC-012, CLB-007 |
| [D-012](D-012.md) | MERGE-WORK bound of 5,400 columns, with the lead condition | hard rule | OWNER | accepted | SQG-013; SHP-008; LPC-002; Carbon afk-picks F-5 | LPC-002, SQG-013, SHP-008, NAG-005, NAG-006, NAG-011 |
| [D-013](D-013.md) | The x_j lock form is canon | hard rule | OWNER | accepted | ANC-002; ANC-017; T10B-LOCK; NT-2, NT-3 | ANC-016, ANC-017, LPC-017, CLB-005, CLB-006 |
| [D-014](D-014.md) | Setup mode replaces F-3 | hard rule | OWNER | accepted | F-3; SPL-002; SPL-011; SETUP-SPEC ST, PF, SC | EUF-007, EUF-008, LPC-006, LPC-007, ST-001 to ST-008, PF-001 to PF-008, SC-001 to SC-014, STM-001 to STM-019, ELA-001 to ELA-007, UKS-008, STM-014 |
| [D-015](D-015.md) | The design stays open until the end-to-end prototype | hard rule | OWNER | accepted | LOOP.md line 101; memory rule 2026-11-14 | LPC-014, UKS-007, SSG-006 |
| [D-016](D-016.md) | Stage 1 of the Compact compiler reads the stock compactc output | A | LEAD PICK for T15 | proposed | CF-001; MYC-FE-001; MYC-FE-008; T18 (b) | ECC-001, ECC-010, MYC-FE-001, MYC-FE-002 |
| [D-017](D-017.md) | R_SL is chip set 3: 16 rules lowered to micro-op AIRs in a 6-or-9 AIR bucket | A | LEAD PICK for T15 | proposed | RSL-002; MYC-CHIP-001; CF-005; ARCHITECTURE.md 1.7; rekey list | RSL-001, RSL-002, EKT-002, EKT-001, ECC-005 |
| [D-018](D-018.md) | Every proof is a fixed-size shard | hard rule | OWNER | accepted | hard rule (LOOP.md line 102); CPR-002 | LPC-007, CPR-002, LFP-005, LFP-008 |
| [D-019](D-019.md) | keccak256 and secp256k1 are R_SL kits inside the bucket | A | OWNER | accepted | T24; RSL-004; RSL-005; CF-013; MYC-BLT-007 | ECS-006, ECC-007, CF-013, RSL-004, RSL-005, EKT-006, EKT-009 |
| [D-020](D-020.md) | rsl-call/1 is the call record between Mycroft and the R_SL executor | A | LEAD PICK for T15 | proposed | ERS-001; `mycroft-call/1` | ERS-005, ERS-011, ERS-001, ERS-013, ERS-014 |
| [D-021](D-021.md) | The anchor slot is the key constant 15 | A | LEAD PICK confirmed | accepted | T12-SLOT15; ANC-001; ANC-013 to ANC-015; MYC-SEAL-005; MYC-SEAL-021; D-072 (T12-SLOT15; MYC-SEAL-021; X1 (a)) | ECC-009, ANC-013 to ANC-015, CLB-004, CLB-008 |
| [D-022](D-022.md) | The prototype settles Compact calls in run form through acceptRun | A | LEAD PICK for T15 | proposed | T23; MYC-CHIP-004; MYC-SEAL; STL-001 | ECS-010, EUF-010, ERS-003, ERS-008 |
| [D-024](D-024.md) | A mixed-hash R_SL program is an open gap with a temporary named refusal | A | LEAD PICK for T15 | proposed | EKT-004; RSL-002; Astra-A review R-01 | EKT-004 |
| [D-025](D-025.md) | processor-final governs where the older universal-interpreter change conflicts with it | A | LEAD PICK for T15 | proposed | UIREQ-001 to UIREQ-035; Astra-A review R-05 | EUI-009 |
| [D-026](D-026.md) | This change cites existing requirements and does not restate them | A | LEAD PICK for T15 | proposed | BRIEF.md; AUDIT.md | none: see record |
| [D-027](D-027.md) | The Jubjub kit is part of the single rekey | A | LEAD PICK confirmed | accepted | RSL-006; LOOP.md row T23 LEAD PICK; EKT-010 | ECC-014, RSL-006, EKT-010, EKT-007 |
| [D-030](D-030.md) | STARK layers run WHIR in unique decoding at five frozen parameter sets | B | OWNER | accepted | OD-4, OD-19, OD-24 option E, OD-18 | LFP-001, NAG-001, LTR-001, LTR-002, SSG-002, SSG-001, RKY-005, SQG-002, SQG-003 |
| [D-031](D-031.md) | OpenVM stark-backend with its CUDA prover is the STARK engine | B | OWNER | accepted | owner 2026-11-11; SPL-007; OD-11 | GPV-001, GPV-003, GPV-004, GPV-002, GPV-005 |
| [D-032](D-032.md) | One rekey from one list regenerates L, L_Fr and the wrap key | B | LEAD PICK confirmed | accepted | T25; RKY-001; the single rekey list | RKY-001, RKY-002, UKS-004, UKS-005, UKS-006, UKS-009, UKS-010, STM-008 |
| [D-033](D-033.md) | Every AIR is required and prog_d with key_root replaces the KeyBind chain | B | OWNER | accepted | O10 (FP8 picks 1 to 7), T3, T7 | NAG-002, NAG-003, NAG-004, LFP-006, LFP-007, LPC-018 |
| [D-034](D-034.md) | Path C checks the child DAG key in L and L_Fr under the same L key | B | LEAD PICK for T15 | proposed | T7c; I-17 path C; lead ruling 2026-11-14 | LTR-007, UKS-003 |
| [D-035](D-035.md) | T8b picks: keys_d with 7 rows and the MERGE-WORK node modes | B | OWNER | accepted | T8b picks; N-7; N-8 | UKS-002, GPH-002, GPH-005, GPH-007 |
| [D-036](D-036.md) | X14 work units use a 17-bit range checker and a 3,300 column bound | B | OWNER | accepted | O1, O2 | GPH-003 |
| [D-037](D-037.md) | Option A (the L_Fr stage at log blowup 3) for the prototype key freeze | B | LEAD PICK confirmed | accepted | O3; E4 | LPC-005, RKY-003 |
| [D-038](D-038.md) | FP-3: L binds call lanes to guest effects through a forward chain | B | LEAD PICK for T15 | proposed | FP-3; ANC-018; ANC-019; NT-1 residual | CLB-002, CLB-003, ANC-018, ANC-019 |
| [D-039](D-039.md) | Blinder lifetimes and the test-seed guard | B | LEAD PICK confirmed | accepted | T11-1, T11-4 | ZK-012, ZK-013, ZK-014, ZKE-001, ZKE-002, ZKE-005, ZKE-006 |
| [D-040](D-040.md) | The daemon keeps OD-13: one-use blinding slots; the slot pool is to build | B | OWNER | accepted | OD-13; PRD-002; ZK-016; T11-2 | PRD-002, ZK-016, PDM-005, ZK-015, PDM-009 |
| [D-041](D-041.md) | No PCS masking while every STARK layer runs on owned boxes | B | LEAD PICK confirmed | accepted | T11-3 | ZKE-003, ZKE-004, ZK-001, ZK-018, BXF-001 |
| [D-042](D-042.md) | Box roles: muninn leads and checks, huginn and odin take the heavy loads | B | OWNER | accepted | owner 2026-11-15 (odin); LOOP compute boxes rule; heavy-slot lead decision | BXF-001 to BXF-004, BXF-007 to BXF-011 |
| [D-043](D-043.md) | Orphan GPU-lock holders are ended under a narrow rule | B | OWNER | accepted | owner 2026-11-15 box-orphan-lock rule | BXF-005, BXF-006 |
| [D-044](D-044.md) | A row is DONE only with a Quint lane verdict and a counterexample for every finding | B | OWNER | accepted | owner 2026-11-14 (two rules); T13 | SSG-003, SSG-004, SSG-005, NAG-006, NAG-007, LPC-004, LPC-016 |
| [D-045](D-045.md) | Stages hand over named records; ProofBundle is the root of a run | B | LEAD PICK for T15 | proposed | new; Fable-B | LFP-009, NAG-010, LTR-006, LTR-008, UKS-004 |
| [D-046](D-046.md) | R_SL key leaves get an index that differs from the DAG leaf | B | LEAD PICK for T15 | proposed | new; LIST "key_root menu id 3" | UKS-003 |
| [D-047](D-047.md) | The Compact adapter class is compact-zkir3-kzg-bls12381-blake2b/1 | B | LEAD PICK confirmed | accepted | O8 | GPH-004 |
| [D-048](D-048.md) | The prover is a resident daemon on a local socket | B | OWNER | accepted | OD-13; T14b | PDM-001 to PDM-004, PDM-006 to PDM-008 |
| [D-049](D-049.md) | T25 runs first; the L collection form follows as a recorded second rekey | B | LEAD PICK for T15 | proposed | T8c; BAT-009; LPC-003; LOOP Finish plan | GPH-008, LPC-003, BAT-009 |
| [D-050](D-050.md) | STM is the OpenSpec home of the setup-mode requirements | B | LEAD PICK for T15 | proposed | ST, PF, SC of SETUP-SPEC.md; T23 profile path pick | STM-001 to STM-019 |
| [D-051](D-051.md) | The R_SL statement bindings join the single rekey | B | LEAD PICK for T15 | proposed | ERS-006, ERS-007, ERS-012 (slice A findings) | LFP-010, LFP-011, LFP-012, ERS-006, ERS-007, ERS-012 |
| [D-070](D-070.md) | One SherlockLedger for each program | C | LEAD PICK confirmed | accepted | N-11; MAP pick 3 | SLC-005, BAT-004, SLC-012 |
| [D-071](D-071.md) | Mycroft call to acceptRun: outcome, input and refused calls | C | LEAD PICK for T15 | proposed | MAP picks 1, 2, 4; FP3-1 (C) | SLC-007, SLC-008, SLC-010 |
| [D-073](D-073.md) | Batched settlement form: collection statement, claims tree, native reads | C | OWNER | accepted | T20 picks W-1, W-2, W-3; O9; FP3-3 (a) | LDG-004, LDG-007, BAT-009, BAT-012, BAT-010, BAT-011 |
| [D-074](D-074.md) | K_MAX = 2 at every TTL, and a 3,600 s settlement TTL | C | LEAD PICK for T15 | proposed | ANC-012; O6; T20 fix 4 | BAT-005, BAT-006 |
| [D-075](D-075.md) | acceptCollection is final | C | LEAD PICK confirmed | accepted | Carbon afk-picks F-8, option A | LDG-003 |
| [D-076](D-076.md) | Governance: sealed contracts, eras in a sealed era table, Fenrir operator verbs | C | LEAD PICK confirmed | accepted | T22 LEAD pick; HFC study | LPC-012, GOV-008, GOV-010, GOV-011, ERT-009, CRR-006, GOV-001 to GOV-011, EGV-001, EGV-002 |
| [D-077](D-077.md) | Governance open items take the recommended options until the owner rules | C | LEAD PICK for T15 | proposed | T22 O1 to O13, P1 to P6; UC-9 | EGV-008, EGV-010, BAT-010, BAT-011 |
| [D-078](D-078.md) | The era check enters the single rekey, or a recorded premise failure selects the fallback | C | LEAD PICK for T15 | proposed | ELD-003, ELD-007; CRR-006; T22 premises A1 to A3; T22 tasks 2 and 3 | EGV-009, EGV-005, EGV-006 |
| [D-079](D-079.md) | Preview: deploys and calls with the existing wallet; Mainnet never without the owner | C | OWNER | accepted | owner 2026-11-14; O11 | PVW-001, PVW-002, PVW-009 |
| [D-080](D-080.md) | Wallet handling: in-process phrase, pinned address, one lock, muninn only | C | LEAD PICK for T15 | proposed | T17 fixes 6 and 8; Finish plan | WAL-004, WAL-009, WAL-002, WAL-003, WAL-005 |
| [D-081](D-081.md) | A refusal counts only with a node error | C | LEAD PICK for T15 | proposed | T17 fixes 8, 9 | PVW-007 |
| [D-082](D-082.md) | Helper machines: case A only; muninn checks every box wrap | C | OWNER | accepted | ZK-018; T11-3; OD-9 | HLP-001, HLP-005 |
| [D-083](D-083.md) | Wrap row gate: 1,782,573 rows at k = 21, option A | C | LEAD PICK confirmed | accepted | O3; T4; OD-24 option E | WRP-003, RKY-003 |
| [D-084](D-084.md) | T23 instance reuse and box profile path | C | LEAD PICK confirmed | accepted | T23 LEAD PICKs | SLC-011 |
| [D-085](D-085.md) | The prototype SDK is the existing tool set behind one runbook | C | LEAD PICK for T15 | proposed | none (new) | SCL-001, SCL-010 |

## Cancelled IDs

| ID | Reason |
|---|---|
| D-023 | merged into D-070 and D-071 (slice C owns the statement to acceptRun map) |
| D-072 | merged into D-021 (body kept there) |

## Authority counts

- LEAD PICK confirmed: 12
- LEAD PICK for T15: 21
- OWNER: 30

## Older identities that look like these IDs

The D-NNN records above have three digits. Older documents use short forms that are different decisions:

| Older ID | Where | Meaning | Record here |
|---|---|---|---|
| D-9, D-10, D-11, D-12 | E3 and E5 RESULTS; LOOP.md rulings O4, O5 | E5 decode checks in the circuit; `R` order and anchor rule; replay scope (resolved by T4: full replay with PATH); OnionAir in `L` | see the legacy column (O4, O5) |
| D-1, D-2, D-3 | T16 `sched.rs` findings (LOOP.md line 106) | three scheduler defects, fixed on openvm `t16-cx` 0b26d74 | defects, not decisions; no record |
| OD-1 to OD-20, OD-24 | `research/sherlock-final-design-20261111/DECISIONS.md` | co-processor decisions of 2026-11-12 | see the next table; DECISIONS.md stays their source |
| O1 to O12 | LOOP.md "Owner rulings that gate rows" | owner rulings of 2026-11-13 | legacy column |
| T11-1 to T11-4, T12-SLOT15 | T11 and T12 lanes; confirmed by the owner 2026-11-15 (wiki log 09:04) | zero-knowledge picks; anchor slot | D-002, D-021, slice B records |
| F-3 | T16 | schedule menu, replaced by setup mode | D-014 |

## Owner decisions OD-1 to OD-24 (source: DECISIONS.md)

These owner decisions of 2026-11-12 keep their records in `research/sherlock-final-design-20261111/DECISIONS.md`. This table maps each one to the record here that applies it (found by its legacy column), to its EARS IDs and to its scope in the run-form prototype. No rejected option is added here that DECISIONS.md does not record.

| Older ID | Decision | Authority | EARS IDs | Records here | Prototype scope |
|---|---|---|---|---|---|
| OD-1, OD-2, OD-3, OD-20 | OOD closure, `L` PoW rule, `L` parameter set, list size | superseded or closed by OD-19 | SQG (processor-final) | no record here | in scope through OD-19 |
| OD-4 | proven rule on inner layers and the margin rule | owner 2026-11-12 | SQG-001 to SQG-013; SSG | D-003, D-030 | in scope |
| OD-5 | E-1 placement V with the V+ layout word | owner 2026-11-12 | CF-008, CF-009, CF-010; MYC-BIND-001 to MYC-BIND-009 | no record here | retained obligation of the sealed form; not built in the run-form prototype (D-022) |
| OD-6 | log capacity in the wrap key (`V = P = 256`) | owner 2026-11-12 | FLG (processor-final); LW | no record here | in scope |
| OD-7, OD-15 | `L_TX` = 1,024 and one call-vector layout of `2 + L_TX` words | owner 2026-11-12 | CF-007; MYC-SEAL-001; LW-003 | no record here | in scope |
| OD-8, OD-16 | hybrid native and Sherlock contracts; anchor lock and native prologue | owner 2026-11-12 | ANC-009; MYC-SEAL-006 | no record here | retained obligation; no hybrid contract in the prototype |
| OD-9 | elastic trust domain: the user's own devices only | owner 2026-11-12 | ZK-018; HLP-005; BXF-001 | D-010, D-082 | in scope |
| OD-10, OD-11 | wait for the Fr Poseidon GPU kernel; no second Fr proving engine | owner 2026-11-12 | none: build-order decisions | D-031 | closed |
| OD-12 | user batches: A-GP-root, opt-in | owner 2026-11-12 | LDG; BAT | no record here | in scope through T20 |
| OD-13 | resident prover daemon, local socket, one-use slots | owner 2026-11-12 | PRD-001 to PRD-005; PDM | D-040, D-048 | in scope |
| OD-14 | Compact `Field` stays emulated | owner 2026-11-12 | CF-005 (audit-fixed); RSL-008; EKT-011 | no record here | in scope: Fr is emulated with 32 byte limbs per word in chip set 3 |
| OD-17 | `pv` encoding, `B = V = 256`, reservation `R` before discretionary operations | owner 2026-11-12 | FLG; MYC-CLM-025, MYC-CLM-026 | no record here | in scope for `L`; the Mycroft claims part is a retained obligation |
| OD-18 | hash domains and transcript algorithm | owner 2026-11-12 | HDM-001 to HDM-010 | D-030 | in scope |
| OD-19 | unique decoding for the internal and `L` layers | owner 2026-11-12 | SQG; SSG | D-003, D-030 | in scope |
| OD-24 | option E: `L_Fr` under one `ZkStdLib`-architecture wrap | owner 2026-11-12 | CPR-004; LW-017; LVR | D-005, D-007, D-030, D-083 | in scope |

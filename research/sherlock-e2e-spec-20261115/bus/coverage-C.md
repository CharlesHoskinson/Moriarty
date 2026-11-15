# COVERAGE.md corrections from slice C (row name | column | new text)

capability table sherlock-e2e-batched-settlement | count | 12
capability table sherlock-e2e-governance | count | 10
capability table sherlock-e2e-helper-machines | count | 8
capability table sherlock-e2e-ledger-contract | count | 12
capability table sherlock-e2e-preview-deployment | count | 13
capability table sherlock-e2e-recovery | count | 15
capability table sherlock-e2e-sdk-cli | count | 10
capability table sherlock-e2e-wallet | count | 10
capability table sherlock-e2e-wrap | count | 11 (WRP-001 to WRP-004, WRP-006 to WRP-012; WRP-005 withdrawn, LW-003 is the ID)
WRAP circuit, decider | EARS IDs | WRP-001 to WRP-004, WRP-006 to WRP-012
WRAP circuit, decider | reused IDs | DG-001 to DG-003, DG-010; LW-001 to LW-022; GPW-001 to GPW-008; DCD-001 to DCD-007; RKY-003, RKY-004; ANC-013, ANC-016
LEDGER CONTRACT | EARS IDs | SLC-001 to SLC-012
LEDGER CONTRACT | reused IDs | LDG-001, LDG-002; STL-001 to STL-006; ANC-014, ANC-015
LEDGER CONTRACT | gap | live settle off until the leaf statement binds prog_d, input_d, outcome_d (SLC-012): slices A, B; MAP.md and accept_args.py staged, not committed
Batched settlement | EARS IDs | BAT-001 to BAT-012
Batched settlement | reused IDs | ANC-012; LDG-003 to LDG-008; LW-009, LW-011, DCD-004 (as amended on t20-batched); LPC-010. CLM-001 to CLM-009 are in-circuit (slice B); on the ledger LDG-004 replaces them
Batched settlement | gap | L collection form, claims tree proof, SHK_FCLAIM tag: no code (BAT-009, BAT-012): slice B, T8c. Registry update entry and N-8 history rule; leaf-list storage: no row (proposed T20b)
Governance | EARS IDs | EGV-001 to EGV-010; BAT-010, BAT-011
Governance | gap | era-check words not in the T25 rekey list and premises A1 to A3 untested (EGV-005, EGV-009): row T22, slice B (UKS). No sealed deploy tool (EGV-001, EGV-002). No Carbon verdict (LPC-012)
Preview deployment and the validium demo | EARS IDs | PVW-001 to PVW-013
Preview deployment and the validium demo | decisions | D-079, D-081
Wallet | EARS IDs | WAL-001 to WAL-010
Wallet | reused IDs | ZK-012
Wallet | gap | derived keys not scrubbed; pins are constants; no mode check on the read path (WAL-002, WAL-003, WAL-010): SDK lane
SDK and CLI (settlement side) | EARS IDs | SCL-001 to SCL-010
Failure and recovery; durability; pushes | EARS IDs | FRC-001 to FRC-015
Failure and recovery; durability; pushes | reused IDs | RKY-002; LPC-010, LPC-014, LPC-015
Failure and recovery; durability; pushes | gap | unknown inclusion state, artifact checks, suite resume not built (FRC-012 to FRC-014): row T23 driver
Helper machines | EARS IDs | HLP-001 to HLP-008
T1 | EARS IDs | FRC-015, OBS-010
T10a | EARS IDs | SLC-001 to SLC-004; LW-003 (in place of WRP-005)
T10 | EARS IDs | WRP-001 to WRP-004, WRP-008 to WRP-010, LVR-001 to LVR-005
T15 | EARS IDs | FRC-011, EGV-008, BAT-012, HLP-006, EUF-013
T17 | EARS IDs | PVW-001 to PVW-010, PVW-012, PVW-013, EUF-001, EUF-009 to EUF-011
T20 | EARS IDs | BAT-001 to BAT-012
T20 | reused IDs | ANC-012; LDG-001 to LDG-008; LPC-010
T22 | EARS IDs | EGV-001 to EGV-010, BAT-010, BAT-011
T25 | EARS IDs | add WRP-012, EGV-005 (C); gap: era-check row missing from the rekey list

# Slice C: adopted existing requirement IDs (for COVERAGE.md)

Sender: Fable-C. The reuse rule of proposal.md forbids a restated requirement. These existing IDs stay normative for slice C. COVERAGE.md lists them beside the new slice C IDs. Check state on 2026-11-15: observed (a named check ran), model (Quint only), open (no check ran).

| Capability | Adopted IDs (short change name) | Check state | New slice C IDs that add the end-to-end gate |
|---|---|---|---|
| sherlock-e2e-wrap | DG-001 to DG-003 (k limit, row gate, VK bound; WRP-002 to WRP-004 add only the tool refusals), DG-010 (a failed gate never narrows a GP target), CPR-004, LW-001 to LW-008, LW-010 to LW-019 (processor-final); DCD-001 to DCD-007 (modular-leaf); LW-022, RKY-003, RKY-004, ANC-013, ANC-014, ANC-016 (completion); GPW-001 to GPW-008 (processor-final) | observed on T10m keys (wrap suite 25/25, E5 decoder lane 6/6 mutants); open on T25 keys | WRP-001 to WRP-004, WRP-006 to WRP-012 (WRP-005 withdrawn: LW-003 is the ID for 1,026 public inputs) |
| sherlock-e2e-ledger-verifier | MID-001, MID-003, MID-004, MID-006, MID-007 (processor-final); MID-002 narrowed to write entries by SLC-003 and SLC-004 | observed (T10m, T17 (a)); open on T25 keys | LVR-001 to LVR-008 |
| sherlock-e2e-ledger-contract | LDG-001, LDG-002 (modular-leaf as amended on `t20-batched`); STL-001 to STL-006 (completion): per call tx hash, block, `runs[claim_pub]` readback, 0 mismatch, blocked reasons, Part A rerun of 33 programs with the hard-fork split; ANC-014, ANC-015 | STL-001 builder observed (staged); STL-005 dry run observed, not a live pass; STL-002 to STL-004 open; STL-006 running (3 of 33 programs) | SLC-001 to SLC-012 |
| sherlock-e2e-batched-settlement | LDG-003 (collection against `[reg_pub, claims_root]`, duplicate root and other registry refused), LDG-004 (membership, no ledger write, once-only at the consumer nullifier across `runs` and several roots), LDG-005 (claims tree, availability), LDG-006 (one write per epoch, none for an empty epoch), LDG-007 (foreign leaf, kind separation), LDG-008 (one registry root per registry epoch); ANC-012 (K_MAX, superseded above 2 by BAT-005); LW-009 and DCD-004 as amended on `t20-batched` to `[reg_pub, claims_root]`, LW-011 (registry root at mid-run consumption); CLM-001 to CLM-009 and FREG-001 to FREG-005 (modular-leaf) stay with slice B for the in-circuit side (GP-2, GP-3); on the ledger they are replaced by LDG-004 (membership) and LDG-007 (no `foreign` map); LPC-010 (T20 merge) | observed on compiled contracts (`t20-ledger-test.out`, 9 tests) and in Quint (`setOpening`, `epochSettle`, `claimsRoot`, registry model); open: collection proving (`L` over SET), the seventh DIGEST tag, a registry update entry and the N-8 history rule, leaf-list storage and retention | BAT-001 to BAT-012 |
| sherlock-e2e-governance | GOV-001 to GOV-011, ERT-001 to ERT-009, ELD-001 to ELD-007, CRR-001 to CRR-007 (governance, `t22-governance@fe7207819`); LPC-012 (Carbon verdict) | model: 13 invariants, 11 of 11 mutants; not modelled: GOV-008, GOV-010, GOV-011, ERT-009 beyond the announce guard, CRR-006; open: premises A1 to A3, sealed deploy tool, Lean, Carbon verdict, forged ledger address and cross-era duplicate tests (CRR-003, CRR-005) | EGV-001 to EGV-010 |
| sherlock-e2e-preview-deployment | MID-004, MID-006; STL-003; LPC-008 (T17 b, c), LPC-009 (T19 validium, whole acceptance test) | T17 (a) observed on T10m keys; (b), (c), T19 open | PVW-001 to PVW-013 |
| sherlock-e2e-wallet | ZK-012 (`r_P` per deployment, box-private) | observed in part (T17 (a)); marker tests open | WAL-001 to WAL-010 |
| sherlock-e2e-sdk-cli | ANC-012 (`sherlock plan` prints k and fee, refuses above K_MAX); LPC-018 (sdk-node suite) | open | SCL-001 to SCL-010 |
| sherlock-e2e-observability | GPW-007 (MEASURED needs a run record and host load); LPC-005, LPC-011 (T21), LPC-016 (lead verdicts), LPC-017 (stale lock form in ARCHITECTURE) | observed for T17 (a) records; open for T21 and the verdicts | OBS-001 to OBS-012 |
| sherlock-e2e-recovery | RKY-002; LPC-014 (T15 freeze presentation and owner sign-off), LPC-015 (pushes) | T0 and T1 records observed; T15 open | FRC-001 to FRC-015 |
| sherlock-e2e-helper-machines | ZK-001, ZK-009 (processor-final as fixed on spec-audit), ZK-018 (completion), PRD-002, PRD-003 | open: no configuration or ownership test | HLP-001 to HLP-008 |

## Rows of slice C and their IDs

| LOOP row | IDs |
|---|---|
| T0, T1, T2 | FRC-010, FRC-015, OBS-010 |
| T4, T10, T10a, T10m | WRP-001 to WRP-004, WRP-008 to WRP-010, WRP-012, LVR-001 to LVR-005, SLC-001 to SLC-003; LW, MID, RKY-003, RKY-004 |
| T12 anchor-slot part (wrap and contract side) | WRP-007, SLC-002; ANC-013, ANC-014, ANC-016 |
| T14b GPU wrap part | WRP-011; GPW |
| T15 freeze and pushes | FRC-011, EGV-008, BAT-012, HLP-006; LPC-014, LPC-015 |
| T17 | PVW-001 to PVW-010, PVW-012, PVW-013, LVR-006 to LVR-008, WAL-001 to WAL-010, OBS-001 to OBS-008, FRC-001 to FRC-006, FRC-013, FRC-014; LPC-008 |
| T19 | PVW-011; LPC-009 |
| T20 | BAT-001 to BAT-012, SLC-001, SLC-004; LDG-001 to LDG-008, ANC-012, LPC-010 |
| T21 page | OBS-012; LPC-011 |
| T22 | EGV-001 to EGV-010, BAT-010, BAT-011; GOV, ERT, ELD, CRR, LPC-012 |
| T23 Part A and settle | SLC-005 to SLC-012, BAT-004, SCL-008, OBS-007, FRC-012, HLP-001 to HLP-004, HLP-008; STL-001 to STL-006 |
| helper machines (owner 2026-11-15) | HLP-005, HLP-006; ZK-018 |

## Gaps of slice C (each with an owner lane)

| Gap | Owner |
|---|---|
| R_SL leaf statement does not bind `prog_d`, `input_d`, `outcome_d`; live settle off (SLC-012) | slices A (ERS-006 to ERS-008, ERS-012) and B (LFP); row T25 |
| Era-check words are not in the T25 rekey list; premises A1 to A3 untested (EGV-005, EGV-009) | row T22 (tasks 2, 3) and slice B (UKS) for the list |
| No sealed deploy tool; the T17 (a) ledger is unsealed (EGV-001 to EGV-003) | row T22 task 4 |
| `L` collection form, claims tree proof, seventh DIGEST tag: no code (BAT-009, BAT-012) | slice B (GPH), row T8c |
| Registry update entry and history rule N-8; leaf-list storage and retention | row T20 follow-up (no row; proposed T20b) |
| Slot 15 not committed in the wrap decoder and not in a compiled contract (WRP-007, SLC-002) | row T12-SLOT15, T25 |
| No VK size guard in the wrap tool (WRP-004); no k = 22 test (WRP-002) | wrap lane, before T25 |
| Final-key gate record and its check at deploy (PVW-013, WRP-012) | slice B produces, row T17 (c) checks |
| huginn or odin prover hook for the final call (PVW-010) | row T17 (c) |
| Settle path: resume, unknown inclusion state, artifact checks, box-result binding (FRC-012 to FRC-014, HLP-008) | row T23 driver |
| Wallet: derived keys not scrubbed, pins are constants (WAL-002, WAL-003, SCL-010) | SDK lane, after T15 for SCL-010 |
| Case A ownership guard has no test (HLP-005, ZK-018) | slice B (ZKE) with row T11 |
| Governance owner items O1 to O13, P1 to P6 | owner at T15 (D-077, recommended options carried) |
| `keyset_d` = the key manifest of UKS; retry of a run reuses its salt (both LEAD PICK for T15, D-077) | slice B (UKS, ZKE) |
| T19 validium and T21 page not started beyond requirements | rows T19, T21 |

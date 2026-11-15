# COVERAGE.md corrections from slice B (row name | column | new text)

capability table sherlock-e2e-universal-keys | count | 10
Universal key set, `key_root`, `keys_d`, the single rekey | EARS IDs | UKS-001 to UKS-010
Universal key set, `key_root`, `keys_d`, the single rekey | gap | rekey list lacks 6 rows: FP-3, slot 15, range checks, R_SL statement bindings, R_SL key leaf index, era-check words (UKS-010); `KEYDAG_IDX = 3` collides with the planned R_SL id 3 (UKS-003); no integrated keygen tree (UKS-009): lanes fp3, t12-slot15, rekey-chips, t22-governance, T25
GP-4 hook (any shape), MERGE-WORK | gap | GAP: the complete kernel width with STACKS and VALUE DIGEST is 5,121 DERIVED, not MEASURED (4,988 MEASURED without them; openvm `rekey-chips` dcf1b4c `frozen.rs:166`); the earlier 5,417 is superseded (NAG-005, LPC-002): row T8c
gap list item 14 | text | **GP-4 hook (any shape), MERGE-WORK.** the complete kernel with STACKS and VALUE DIGEST is 5,121 columns DERIVED against the bound 5,400, not MEASURED (NAG-005, LPC-002): row T8c (no LOOP row), slice B
Leaf: app proof and leaf proof of one shard | gap | R_SL statement, `prog_d` and `input_d` not bound (LFP-010 to LFP-012, HIGH); 137 of 142 calls have no probe (LFP-002); chip set 3 has no named shape vector (LFP-005); both hash kits in one program have no route (LFP-003, D-024): rekey-chips
`L` (bridge layer), `L_Fr`, the tape and the transcript; deferred field log | gap | `L` is at 1,995 of 2,000 columns before FP-3 and slot 15 (LTR-003); `LfrProofReady` is not a built type (LTR-006, LTR-008): T25, t23-suite
Call binding: FP-3, anchors, locks, chunks (iface and `L` side) | gap | FP-3 not committed and one test passes without a refusal (CLB-002, CLB-003); slot 15 not merged with the extended lock (CLB-004, UKS-009); three copies of the slot constant (CLB-008); K_MAX = 2 is a model, not code (CLB-007): lanes fp3, t12-slot15, t20-batched
Box fleet: muninn, huginn, odin; heavy-slot; gpu.lock; box bus | gap | no script enforces the owned-box roster, the digest check, the muninn route or the lock rule (BXF-001, BXF-007, BXF-014, BXF-005); the tools are outside every repository: box tooling (lead)
Box fleet: muninn, huginn, odin; heavy-slot; gpu.lock; box bus | reused IDs | LPC-020; ZK-018
GPU STARK paths | gap | the leaf layer proves on the CPU (GPV-007); `L` on the GPU is not timed (GPV-005); no GPU against CPU public-value test (GPV-002): t14-gpu, t12-gate
GPU STARK paths | decisions | D-007, D-031
Resident prover daemon and warm path | decisions | D-002, D-040, D-048
Resident prover daemon and warm path | gap | no blinding-slot pool (D-040, LEAD PICK for T15, departs from the letter of OD-13); no recorded check of PRD-003, PRD-004 (PDM-007); final timing open (LPC-005): rows T14, T14b
Setup mode and the box profile; shard plan | gap | F-3 menu code still in `sched.rs` and called by the part (b) runner (STM-014); per-GPU calibration open (STM-004); profile path not read by code (STM-016): t16-elastic, t23-suite
Elastic proving across schedules and boxes | gap | muninn `hw` run and muninn join NOT RUN; two run lengths and 1x to 8x open (LPC-006, LPC-007); the record names no device per stage (ELA-007): row T16
ZERO KNOWLEDGE (proving layer) | gap | T11 code and tests untracked, `eq_zk` and `slot_reuse` have no recorded run (ZKE-001, ZKE-007, PDM-009); no code refuses a host outside the roster (ZKE-004, ZK-018); a `test-seed` binary can settle (ZKE-002): T11 lane, box tooling, t23-suite
Security qualification gate, derived gates, freeze prerequisites, Quint lanes | gap | G3 legs not built, no gate run (SSG-006, SQG-014 to SQG-016); GP2-C2.1 CONFIRMED with no fix (SSG-003, NAG-007); 8 findings NOT ATTEMPTED (SSG-004); no circuit change list (SSG-005); no D3 on R_SL app keys (SSG-001): t12-gate, t13
T10q | EARS IDs | NAG-006 to NAG-007, NAG-011, CLB-002 to CLB-006
T11 | EARS IDs | ZKE-001 to ZKE-007, PDM-005, PDM-009
T13 | EARS IDs | SSG-003 to SSG-005, NAG-006, NAG-007, NAG-011, ECC-011
T16 | EARS IDs | ELA-001 to ELA-007, STM-001 to STM-019, BXF-001 to BXF-004, GPH-006, NAG-009, UKS-008
T21 | EARS IDs | OBS-012, PDM-003, PDM-008
T24 | EARS IDs | add LFP-002, LFP-003 (the AIR bucket and the probe with the kits)
T25 | EARS IDs | UKS-001 to UKS-010, LTR-003, NAG-005, CLB-001, CLB-008, WRP-002 to WRP-004, WRP-012, EGV-005, EKT-001
T6 | EARS IDs | LFP-001 to LFP-005, NAG-008, GPH-001, EUI-005, EUI-010
T7 | EARS IDs | LFP-006 to LFP-007, LFP-011, NAG-003 to NAG-004, NAG-008, GPH-003
T7c | EARS IDs | LTR-007, UKS-003, PDM-007
T8c | EARS IDs | GPH-004, GPH-005, GPH-008, NAG-005, BAT-009, BAT-012
T3 | EARS IDs | NAG-002, LTR-004

# Slice C design notes for design.md (Fable-C)

Per capability: scope, the reused IDs that the spec does not restate, the hard rules it states, and the code paths. Each requirement body is the EARS sentence and its Verification line; evidence (`branch@commit:path:line`) and status are in the scenario "Status of <ID>". The full adopted-ID table with check states is `bus/adopted-C.md`. 121 EARS: WRP 11 (WRP-005 withdrawn, number not reused), LVR 8, SLC 12, BAT 12, EGV 10, PVW 13, WAL 10, SCL 10, OBS 12, FRC 15, HLP 8. Status words on 2026-11-15: 24 BUILT, 50 PARTIAL, 47 TO BUILD (counted from the "Status of" scenarios).

Target state of the prototype settlement edge: the T20 batched SherlockLedger (4 entries) with the anchor sealed in slot 15, one ledger for each program, sealed at deploy with a readback gate, write entries on the one wrap key and read entries on native keys; settlement of a user's own run by `acceptRun`; the era check in the ledger if it enters the single rekey, else the consumer fallback on a recorded premise failure (D-078). The three contract forms on unmerged branches are named in SLC-001: T10 (`t17-preview`, with `openClaim`, the T17 (a) record), T20 (`t20-batched@5dc1be2ee`), T22 era ledger (sketch, `t22-governance@fe7207819`).

### sherlock-e2e-wrap (WRP)

Scope: the one BLS12-381 wrap at the settlement edge: the tool guards (k, rows, VK bytes), fresh blinders, the slot-15 pin in the decoder, key equality with the deployed key, the three output files, one key for all programs, GPU and CPU key equality, the final-key qualification gate. Input event `LfrProofReady` (slice B, LTR).
Reused, not restated: DG-001 to DG-003, DG-010, CPR-004, LW-001 to LW-019, LW-022, GPW-001 to GPW-008, DCD-001 to DCD-007, RKY-003, RKY-004, ANC-013, ANC-014, ANC-016.
Hard rules stated: never k = 22 (WRP-002); VK at most 50,000 bytes (WRP-004); one BLS12-381 wrap (WRP-001); one universal key set, wrap side (WRP-010); zero knowledge, wrap blinders (WRP-006).
Code: `tools/sherlock-wrap/` (`t10q-ntfix@95990a6d3`): `src/bin/wrap.rs`, `src/util.rs`, `src/decode.rs`.

### sherlock-e2e-ledger-verifier (LVR)

Scope: the stock ledger 9.1.0.1 verifier as an external system; the local check tool, its tamper controls, the native check before submit, chain identity in the run state.
Reused: MID-001 to MID-007, LW-017, STL-006.
Hard rule stated: ledger 9.1.0.1 verifier unchanged (LVR-001, LVR-002).
Code: `tools/sherlock-wrap/ledger-check/` (rev `a2b30128`), `validate_call` (`~/.cache/xc9`, no commit in Moriarty).

### sherlock-e2e-ledger-contract (SLC)

Scope: the SherlockLedger contract form, slot 15, entry keys, one ledger for each program, the argument builder from the private bundle to `StatementPublics` and the `acceptRun` words, Mycroft outcome and input forms, refused calls, instance reuse, the readiness gate before live settle.
Reused: LDG-001, LDG-002, STL-001 to STL-006, ANC-014, ANC-015, ZK-006, ZK-011, ZK-012; ERS-006 to ERS-008, ERS-010, ERS-012 (slice A).
Hard rules stated: none new; SLC-002 carries the slot-15 pin (D-021).
Code: `tools/sherlock-sdk/sherlock-ledger.compact` (`t20-batched@5dc1be2ee`; T10 form on `t17-preview@9e1e7f398`), `tools/t23-suite/accept_args.py` (staged on `t23-args`), `tools/t23-suite/parta.py` (`t23-suite@16836929b`).

### sherlock-e2e-batched-settlement (BAT)

Scope: epoch close, operator jobs through one collection, write counts in every report (per request, per epoch, per day), T23 suite counts (145 `acceptRun`, at most 106 deploys), `K_MAX` = 2 at every TTL, default TTL 3,600 s, membership data and the off-chain leaf list, the refusal of a collection submit while the `L` collection form has no key, era-aware close and admission, the T15 line for collection settlement.
Reused: LDG-003 to LDG-008, ANC-012, LW-009, LW-011, DCD-004, LPC-010; CLM and FREG stay in slice B for the in-circuit side.
Hard rules stated: Midnight writes batched, one root per checkpoint (BAT-001, BAT-002); transaction counts at scale (BAT-003, BAT-004); `K_MAX` = 2 (BAT-005).
Code: contract and tests on `t20-batched@5dc1be2ee` (`tools/sherlock-sdk/`, `models/sherlock/quint/t20-batched/`); no operator code.

### sherlock-e2e-governance (EGV)

Scope: sealing at deploy and the readback gate, unsealed ledgers as path evidence only, one era for each key set, the era check and the single rekey, the fallback, no key change on a deployed contract, open parameters at T15, the premise test, proof-version migration.
Reused: GOV-001 to GOV-011, ERT-001 to ERT-009, ELD-001 to ELD-007, CRR-001 to CRR-007, LPC-012, RKY-001, RKY-002.
Hard rules stated: one universal key set for each era (EGV-004); verifier unchanged, no upgradable contract (EGV-007).
Code: none deployed; compile-only sketches and `quint/eraTable.qnt` on `t22-governance@fe7207819`.

### sherlock-e2e-preview-deployment (PVW)

Scope: chain pin, no Mainnet, fees from chain parameters, own proof server, one command per step, the four tamper controls and their exclusive verdicts, final deploy after the gate with the gate record of the exact key, the final call on a GPU box of the user, the validium ledger refusals.
Reused: MID-004, MID-006, STL-003, LPC-008, LPC-009, RKY-003 to RKY-005, SQG-014 to SQG-016, ZK-012 to ZK-017.
Hard rules stated: Mainnet is owner-gated (PVW-002).
Code: `tools/sherlock-sdk/t10a-ledger-run.mjs`, `research/sherlock-t17-preview-20261114/t17.sh` (`t17-preview@9e1e7f398`).

### sherlock-e2e-wallet (WAL)

Scope: the phrase read in process, scrubbing, the pinned address, one sender at a time, public-only receipts and balances, one machine holds the wallet, no key material in the repository, no overwrite, permissions before the read. Written from code and the runbook only; no wallet file was opened.
Reused: ZK-012.
Hard rule stated: wallet seeds and keys are never logged, printed or copied (WAL-001, WAL-002, WAL-008).
Code: `t10a-ledger-run.mjs:114-116,487-545` (`t17-preview@9e1e7f398`), `preview-session.mjs` (`preview-deploy@78592bf59`).

### sherlock-e2e-sdk-cli (SCL)

Scope: the seven settlement commands, exit codes, dry run, state folder bound to one key, `E5JSON` records, build from committed source, TTL range, the suite driver, the SDK root, use outside the factory.
Reused: ANC-012 (`sherlock plan`), LPC-018, CF-002.
Hard rules stated: none.
Code: openvm `rekey-chips@dcf1b4c:crates/sherlock-lfr/examples/sherlock_plan.rs`; the tools named above; `tools/t23-suite/t23.py`.

### sherlock-e2e-observability (OBS)

Scope: receipts of included and refused calls (`SettlementReceipt`), privacy scan and its self-test, step log, input manifest, times per call and box, wallet failure causes, MEASURED / DERIVED / ESTIMATE labels, structural validation of the record, the wiki log, the prover GUI page.
Reused: GPW-007, LPC-005, LPC-011, LPC-016, LPC-017, ZK-002, ZK-007; PGUI IDs on branch `prover-gui`.
Hard rule stated: zero knowledge at the published view (OBS-003, OBS-004).
Code: `t10a-ledger-run.mjs:397-403`, `sherlock-ledger-scan.mjs`, `t17.sh:25,117-125`.

### sherlock-e2e-recovery (FRC)

Scope: step markers, one deploy, reuse of a proven call, bounded retries, a refused call proved again, the refusals step, durable units, reboot recovery, a gate that fails twice, durability in git, pushes, suite resume, unknown inclusion state, artifact checks, integration records.
Reused: RKY-002, LPC-010, LPC-014, LPC-015.
Hard rules stated: no push before T15 except `t10q` (FRC-011).
Code: `t17.sh`, `t10a-ledger-run.mjs:556-573,675-726,763-776`; loop rules in LOOP.md.

### sherlock-e2e-helper-machines (HLP)

Scope: the muninn check of every box wrap (`WrapVerdict`), dropped results, the bundle and private parts stay on the box, case A only, case B as its own change, no wallet on a proving box, a box result bound to its job.
Reused: ZK-001, ZK-009, ZK-018, PRD-002, PRD-003; BXF (slice B) for the box bus.
Hard rules stated: case A only, case B through Veritee later (HLP-005, HLP-006); no plain witness to helpers (HLP-004).
Code: none as a guard; loop rules (LOOP.md:96) and `check_prove_result` in `accept_args.py`.

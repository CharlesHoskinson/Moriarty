# Formal findings (Quint lead, Lean lanes, Fable-A)

Format of an entry: `## F-<ctx>-<n> [<from> -> <to>] <title>` then the EARS IDs, the trace path, the disagreement, and the status.

## Context folders (Formal-Quint, 2026-11-15 10:50)

The unit is one bounded context of architecture/DDD.md. Folders under `formal/sherlock-e2e/`, one per context. Lean lanes: put the oracle in `<ctx>/lean/` and the interface in `<ctx>/ORACLE.md`, so the Quint replay finds them.

| Folder | Capability specs | Completion IDs placed here |
|---|---|---|
| leaf-proving | LFP | RSL-001..008, LPC-018 |
| aggregation | NAG, GPH | LPC-001..003 |
| bridge | LTR, CLB | ANC-013..019, LW-020..022 |
| key-management | UKS | RKY-001, RKY-002, RKY-005 |
| scheduling | STM, ELA | LPC-006 |
| box-fleet | BXF, GPV, PDM | LPC-005, LPC-007 |
| assurance | ZKE, SSG | ZK-012..019, SQG-014..016, LPC-004, LPC-016 |
| wrap | WRP | RKY-003, RKY-004 |
| ledger-verification | LVR | - |
| settlement | SLC, BAT | STL-001..006 |
| governance | EGV | LPC-012 |
| network-operations | PVW, WAL, SCL, OBS, FRC | LPC-008..011, LPC-015, LPC-020 |
| placement | HLP | - |
| compact-compilation | ECC | CF-013, CF-014, LPC-013, LPC-019 |
| sherlock-execution | EUI, EKT, ERS | - |
| conformance | ECS | - |
| call-flow | EUF | LPC-014, LPC-017 |

Denominator: 320 e2e IDs + 62 completion IDs = 382 rows in COVERAGE-FORMAL.md, plus one row per adopted ID that an e2e spec cites (rows in a separate section).

## FAL-WRAP-1 [Formal-Lean-AC -> Fable-C] Two pass lists gate one wrap-key deploy
- IDs: WRP-012, PVW-013 (also RKY-003, RKY-004, RKY-005, ZK-012..017, LW-003..006, LW-014).
- Statement: WRP-012 makes the deploy depend on a "wrap qualification record" with passes for LW-003, LW-004, LW-005, LW-006, LW-014 (its Status scenario: "no deploy uses the key"). PVW-013 makes the final deploy depend on a "gate record of the wrap key" with passes for RKY-003, RKY-004, RKY-005, the T12 gate and ZK-012..017. Neither sentence names the other record, so a deploy runner that implements only PVW-013 deploys a key with no LW-004 replay pass, and the WRP-012 scenario "no deploy uses the key" has no EARS trigger of its own.
- Counterexample (oracle `wrap`, spec text): ops `[7000000000, 7000000001, 7000000002, 7000000003]` (four of five WRP-012 bits) then the honest path then `8000000000` -> the wrap oracle refuses (`lastRefusal = 8`); a PVW-013-only runner with a full RKY/T12/ZK record accepts the same key. Pinned as `Wrap.pin_deploy_four_bits`.
- Proposed fix: one record. PVW-013 lists "the 5 wrap checks of WRP-012" beside RKY-003..005, T12 and ZK-012..017, or WRP-012 gets a second sentence "If the record lacks a pass, then the deploy runner SHALL refuse the deploy".
- Verdict: OPEN DESIGN (spec text only; no deploy runner exists to build the counterexample on: PVW-013 and WRP-012 are both TO BUILD per their status scenarios). LEAD PICK for T15: the deploy runner refuses unless both lists pass (the oracle's `deploy` encodes the WRP-012 half; the network-operations oracle encodes PVW-013).

## Slice B Lean oracles (Formal-Lean-B, 2026-11-15)

Seven oracles under `formal/sherlock-e2e/{key-management,leaf-proving,aggregation,bridge,scheduling,box-fleet,assurance}/` (Lean 4.35.0 on odin, `lake build --wfail` rc=0 each, 0 sorry, 325 theorems with pinned `#print axioms` and `#audit_ns` []). Interface per context: `<ctx>/ORACLE.md` (`op = code*16 + arg`, `--op-var lastOp`). Coverage: `formal/sherlock-e2e/COVERAGE-FORMAL-B.md` (347 rows: 127 own + 220 reused). Evidence per finding: `<ctx>/FINDINGS.md`. Verdicts below are PROPOSED: they come from reading the cited code and records (the lead reopened the cited lines of BR-1, BR-2, AGG-1, BF-1, BF-2 and KM-1). No counterexample was built on real code by this lane, so each COUNTEREXAMPLES.md entry is OPEN. No code fix is made by this lane.

## F-B-BR-1 [Formal-Lean-B -> Fable-B] LTR-003 bound 2,000 columns against FP-3 (CLB-002)
- IDs: LTR-003, CLB-002, ANC-018.
- `L` with FP-3 EffectAir (77 columns MEASURED, LIST.md:44) is 2,071 columns DERIVED (LIST.md:47) > 2,000. Code moved to 2,200 (openvm fp3 a2eeece, rekey-int 155d321 `frozen.rs:202`, LEAD PICK FP3-LCOLS). Lean `Bridge.ltr003_fp3_conflict`, `ltr003_fp3_refused`.
- Verdict: CONFIRMED (spec text against committed code). LEAD PICK for T15: LTR-003 text to 2,200 under the FP3-LCOLS condition (T25 measures; k <= 21 and VK <= 50,000 B kept).

## F-B-BR-2 [Formal-Lean-B -> Fable-B, owner] x_j operand list without the FP-3 link
- IDs: CLB-005, ANC-002, ANC-017 (against CLB-003, ANC-019).
- Sentences give 6 operands (`LOCK_LEN = 6`, t10q-ntfix e3b407d `iface.rs:281`); merged FP-3 absorbs `link` (`LOCK_LEN = 7`, rekey-int `iface.rs:290`, `:316`). Read literally CLB-005 refuses every honest FP-3 chunk. Lean `Bridge.clb005_fp3_operands`.
- Verdict: OPEN DESIGN. The x_j form is an owner hard rule (D-013): not changed here; owner confirmation at T15.

## F-B-AGG-1 [Formal-Lean-B -> spec owner of modular-leaf] SHP-003 two children against NAG-011 one child
- SHP-003: MERGE-WORK "SHALL take two enabled prior SEG proofs"; NAG-011 and the built test `afk_f9_work_one_child` (t10q 0082832 `tests/quint_regressions.rs:190`, expect accept) accept one linked child. Oracle follows NAG-011.
- Verdict: CONFIRMED (built code accepts one child). LEAD PICK: SHP-003 text "one or two enabled prior SEG proofs".

## F-B-LFP-1 [Formal-Lean-B -> Fable-B] LFP-003 admits 6 or 7 app AIRs outside the 6-or-9 bucket of RSL-002
- LFP-003 refuses only > 8 app AIRs; the bucket allows 5 or 8 app AIRs (6 or 9 with FP-1). Lean `LeafProving.lfp003_admits_outside_bucket`. Code (`rsl/trace.rs:137`) asserts `<= 8`; `names()` builds only 5 or 8.
- Verdict: GUARD HOLDS (no buildable 6- or 7-AIR program today). LEAD PICK: LFP-003 text "outside the 6-or-9 AIR bucket with FP-1".

## F-B-LFP-3 [Formal-Lean-B -> owner] mixed hash refusal narrows GP-001 (CPR-005)
- LFP-003 scenario refuses SHA-256 + Keccak-256 in one program; CPR-005 forbids a narrowing without an owner decision. Already recorded as D-024 (LEAD PICK for T15). Also: the refusal is an `assert!` panic (`rsl/trace.rs:114`), no named code (F-B-LFP-4, minor).
- Verdict: OPEN DESIGN (owner, T15).

## F-B-BF-1/2 [Formal-Lean-B -> Fable-B] GPV-001 and PDM-004 evidence
- LEAD RECHECK: F-B-BF-1 is NOT REPRODUCED. 0.225 s and 3.595 s are MEASURED times of the calibration table (RESULTS-CDE.md:47-48); only the 16x ratio at :52 is DERIVED. GPV-001 row is MODELED. F-B-BF-2 stands (all.log:42 reads "STARK lfr_call half not run").
- GPV-001 status calls 0.225 s vs 3.595 s MEASURED; the cited `RESULTS-CDE.md:52` (t16-elastic a7a7b8867) says DERIVED (ratio 15.98). PDM-004 cites `~/.cache/box-bus/all.log:42`, which is the T14b wrap baseline ("STARK lfr_call half not run"), not the STARK worker.
- Verdict: CONFIRMED (status text). Fix: GPV-001 "DERIVED"; PDM-004 cite huginn h79 (`log.md:3691`) or mark the MEASURED evidence open.

## Minor status and citation fixes (CONFIRMED, text only; requirement rows not FINDING)
- UKS-010 status "6 rows missing": LIST.md (10:31) has FP-3 and slot 15 rows; 4 missing (`KeyManagement.uks010_rows_missing`).
- LFP-006 cites `core.rs:41` (blank); `PROG_D = 8` is `:46`.
- PDM-001 cites `log.md:3690`; the numbers are on `:3691`.
- MERGE-WORK width: 5,284 MEASURED (t10m RESULTS.md:96), 5,128 in the `frozen.rs:165` comment (t10mm), 4,988 MEASURED / 5,121 DERIVED in NAG-005 status and coverage-B.md (which cites the comment line `frozen.rs:166`; the constant is `:169`). All <= 5,400: GUARD HOLDS; reconcile the texts (lane T8c).
- FVR-004/FVR-006 allow 3,300 columns for FV units while NODE-007 states 3,000 with no exception: read as the owner exception, SPEC-CHECKED, not a finding.

## FAL-EXEC-1 [Formal-Lean-AC -> Fable-A, Mycroft] MYC-KEY-003 and MYC-KEY-008 disagree on whether a universal rekey changes `prog_d`

- IDs: MYC-KEY-003, MYC-KEY-008, ERS-006, MYC-CLM-023, INIT-007 (context sherlock-execution).
- Statement: MYC-KEY-003 says `prog_d` is computed "from the image, the plan, `key_root` and the machine version only". MYC-KEY-008 says that when the universal key set changes, the compiler reports "the new `prog_d`". The universal key set (node, `L`, `L_Fr`, wrap keys; D-004) is none of the four inputs. Both hold only if a universal rekey always changes `key_root` or the machine version, which no requirement states.
- Counterexample (oracle): `SherlockExec.MYC_KEY_003_prog_d_ignores_ukey`: two states that differ only in `ukey` get the same `progD` from the same `admit` op (for example `admit` arg 221007 gives `progD` 221007 under `ukey` 0 and `ukey` 42).
- The ERS-006 status already notes it ("MYC-KEY-003 and MYC-KEY-008 differ on whether a rekey changes `prog_d`; the test fixes the inputs and takes no side").
- Proposed fix: add one sentence that names where the universal key-set identity enters `prog_d`, either as the machine version (a rekey bumps `machine_version`) or as a word committed in `key_root`. Otherwise reword MYC-KEY-008 to "report the `prog_d` and anchor word" without "new".
- Verdict: OPEN DESIGN (spec text; the oracle takes MYC-KEY-003's reading). Owner question for T15.

## FAL-EXEC-2 [Formal-Lean-AC -> Mycroft] MYC-REF-052 still refuses a per-program preprocessing key

- IDs: MYC-REF-052 against ERS-015, CF-005 ("per-program app keys"), CPR-003, MYC-KEY-006.
- Statement: MYC-REF-052 refuses "a preprocessing key per program". The R_SL app key is a per-program preprocessed key under `key_root` (CF-005, ERS-015). The ERS-015 status says D-004 supersedes the clause ("universal means the node, `L`, `L_Fr` and wrap keys"), but the MYC-REF-052 sentence is unchanged, so the two EARS contradict as written.
- Counterexample: any admitted R_SL program with an app key (`admit` arg 1221007, `appKeyCommitted` true, pin `pin_app_key`) is a configuration MYC-REF-052 refuses.
- Proposed fix: amend MYC-REF-052 in `mycroft/spec/specs/refusals/spec.md` to "a node, `L`, `L_Fr` or wrap key per program", per D-004.
- Verdict: CONFIRMED on the spec text. The code check was not run: openvm `src/rsl/mod.rs:12-13`, cited by the status, is not in a checkout on muninn, odin or huginn.

## FAL-EXEC-3 [Formal-Lean-AC -> Mycroft, slice B] MYC-CHIP-001 counts 9 AIRs "in all", EKT-002 names 10

- IDs: MYC-CHIP-001 against EKT-002, CF-005, RSL-002.
- Statement: MYC-CHIP-001 lists WORD, ALU, PUBLIC, RANGE, SHA (four AIRs) and "the keccak AIR", "at most 9 AIRs in all". EKT-002 names KECCAK_REQ and KECCAK_PERM (two keccak AIRs) and BITWISE8 shared, so the chip set has 10 distinct AIRs. Per app the count is 5, or 8 with one hash, or 6 or 9 with FP-1, inside the bucket (`EKT_002_air_bucket`). Only the reading "in all = the chip set" conflicts.
- Proposed fix: reword MYC-CHIP-001 to "at most 8 app AIRs per program (9 with FP-1)" and name KECCAK_REQ and KECCAK_PERM.
- Verdict: OPEN DESIGN (wording; spec text only, code not located).

## FAL-EXEC-4 [Formal-Lean-AC -> Fable-A, owner T15] An R_SL Rejected outcome is never proved, but GP-001 says every outcome is

- IDs: ERS-009, EUI-004 against GP-001, CPR-005 (and EUI-005).
- Statement: EUI-001 puts every program under one family, R_SL included. ERS-009 gives a rejected R_SL call no statement, and EUI-004 gives R_SL no Exhausted. GP-001 says one key set proves "every program of the Sherlock language ... every outcome (`Rejected(code)`, `Exhausted(kind)`)", and CPR-005 forbids any requirement that narrows GP-001. The ERS-009 status limits the rule to "Compact calls of the T23 route only", but the EARS sentence does not.
- Counterexample (oracle): `pin_rsl_reject` (R_SL, execute Rejected, build: phase 8, `stmtCount` 0, nothing to prove) against `pin_interp_reject` (interpreted family, same op list: one statement, `outcomeKind` 2).
- Proposed fix: either (a) scope ERS-009 in its sentence to "a Compact call on the settlement route", and add that the same R_SL program keeps a provable Rejected outcome under the general route, or (b) state in GP-001 or EUI-001 that the R_SL family proves the outcomes native Compact proves, recorded as an owner decision (CPR-005 requires one).
- Verdict: OPEN DESIGN (spec text). The oracle follows ERS-009 as written. Recommended LEAD PICK: (a), so GP-001 is not narrowed.

## F-B-KM-2 [Formal-Lean-B -> T25 lane] the T25 start on recorded values (GUARD HOLDS)
- On openvm rekey-int ca65f27 (11:27) with LIST.md of 11:22 (sha256 8a315c07...), the Key Management oracle refuses the keygen only with `row-uncommitted` (RKY-002): wrap-levers C3.1 is "staged, NOT committed" and the T11 blinders row has a Moriarty "commit pending". UKS-003 (RSL_KEY_IDX0 = 4), UKS-009 (every listed openvm commit is an ancestor of ca65f27) and UKS-010 (all six rows present) hold. Lean `KeyManagement.t25_refused_rekey_int`. UKS-010 status "6 rows missing" is stale (F-B-KM-1).

## Note for Formal-Quint (replay independence)
- Each slice B ORACLE.md has a guard and effect table. Take from it only the state names, the op codes and the abstraction encodings (bits, ids); write the guards and effects from the spec text, so the replay stays an independent check.

## Answer to FAL-EXEC-3 (Formal-Lean-B, slice B side)
- Agreed for the proving side. The leaf-proving oracle counts per program: 5 or 8 app AIRs, 6 or 9 with FP-1 (RSL-002), refusal above 8 app AIRs (LFP-003; CODE `MAX_APP_AIRS = 8`, rekey-chips dcf1b4c `rsl/trace.rs:49`). The chip-set total (10 distinct AIRs with KECCAK_REQ and KECCAK_PERM) enters no slice B bound. The proposed MYC-CHIP-001 wording ("at most 8 app AIRs per program, 9 with FP-1") matches LFP-003 and RSL-002. See also F-B-LFP-1: LFP-003 itself should refuse "outside the 6-or-9 bucket", not only "above 8".

## Leaf Proving (Formal-Quint, 2026-11-15 12:05)

Quint lane `formal/sherlock-e2e/leaf-proving/quint/REPORT.md` (Carbon), rerun on odin. Replay: `formal/sherlock-e2e/replay/project_leaf_proving.py` projects each model run onto the oracle's one-run state.

### F-leaf-proving-Q1 [Formal-Quint -> Fable-A] LFP-003 says "more than 8 app AIRs", RSL-002 says "6 or 9 AIRs with FP-1"
IDs: LFP-003, RSL-002, LFP-002 (scenario d04 with 9 AIRs). Read literally (FP-1 counted) LFP-003 refuses the 9-AIR d04 and s02 calls that LFP-002 and RSL-007 require (Quint instance `leaf_proving_m_lfp003_literal_8_vs_9`). The code counts without FP-1 (`rsl/trace.rs:48-49` MAX_APP_AIRS = 8 "6 to 9 child AIRs, FP-1 included"), and so does the Lean oracle (`appAirs` = without FP-1). Spec text fix proposed: "needs more than 8 app AIRs, the FP-1 AIR not counted". Same family as F-B-LFP-1.

### F-leaf-proving-Q2 [Formal-Quint -> Fable-A] RSL-004 + RSL-007 + GP-001 against the LFP-003 mixed-hash refusal
IDs: LFP-003, RSL-004, RSL-007, GP-001, CPR-005. No requirement gives a route for a program that calls both `persistentHash` and `keccak256`. Same as F-B-LFP-3 (D-024 open gap). Code: typed refusal `Refuse::Unsupported` at `rsl/compile.rs:884` (rekey-chips dcf1b4c), test `rsl_sha_and_keccak_in_one_program_refused`. This also answers F-B-LFP-4: the refusal is typed in the compiler before the `trace.rs:114` assert is reached.

### F-leaf-proving-Q3 [Formal-Quint -> Fable-A] RSL-001 "the universal leaf key" against SPL-004 per-program leaf-layer keys
IDs: RSL-001, SPL-004, LFP-002. DDD.md section 7 defines "universal leaf key" as LOOP wording for the N1 pre-hash property, so no contradiction in meaning; proposed RSL-001 wording: "the program's leaf-layer key under key_root gives an N1 pre-hash equal to the universal N1 pre-hash".

### F-leaf-proving-Q4 [Formal-Quint -> Fable-A] LFP-005 has no height vector for chip set 3
The LFP-005 status scenario already states the GAP. Open design; no new text.

### F-leaf-proving-Q5 [Formal-Quint -> Fable-A] LFP-012 on continuation shards
IDs: LFP-012, LFP-008, INIT-002, INIT-007. LFP-012 compares the tape digest of every R_SL shard with `input_d`; with more than one R_SL shard (LFP-008) the text does not say whether each shard consumes the whole tape or a slice under a run-level `val_d`. Proposed: LFP-012 names the run-level tape digest bound at genesis (`h_pre = 0`, INIT-002) and carried by N1/N2.

### F-leaf-proving-Q6 [Formal-Quint -> Fable-A] LPC-018 is a QA-lane rule; LFP-011 is a prover action
LPC-018 (t3-airs Carbon lane) has no leaf state: SPEC-CHECKED. LFP-011 says only "the leaf prover SHALL set"; the AIR binding is LFP-006, so the pair binds `PROG_D`. No text change needed.

### R-leaf-1 [Formal-Quint, FIXED in the Quint model] N1 merged leaves before LeafProofsReady
Replay disagreement (oracle `not-published`, trace `leaf-proving/traces/regressions/R-leaf-1.itf.json`). DDD.md sections 2 and 4: Aggregation consumes the run's leaves through `LeafProofsReady` (LFP-009). The oracle reads it so; the Quint model was wrong. Fixed in `leaf_proving.qnt` `canN1AcceptLeaf`; regression `lp_R_leaf_1_N1WaitsForLeafProofsReadyTest` fails on the lane model and passes on the fixed one.

## FAL-LEDGER-VERIFICATION-1 [Formal-Lean-AC -> Fable-C] MID-002 puts the wrap key on every entry; SLC-004 puts native keys on the read entries

- IDs: MID-002 against SLC-001, SLC-003, SLC-004, ELD-005 (contexts ledger-verification, settlement).
- Statement: MID-002 says every exported entry of the ledger contract holds the universal wrap key. SLC-004 installs a compiled native key on the two read entries (readRoot, readRun), and ELD-005 says read entries keep native keys. Both cannot hold for the four-entry contract of SLC-001.
- Counterexample (oracle): `Settlement.Consistency.finding_mid002_read_entries`: in every deployed instance, `keyRoot` (2, native) differs from `keyRun` (1, wrap key). Settlement op `1000000007` (deploy) gives `keyRun = keyColl = 1`, `keyRoot = keyRunRead = 2`.
- Proposed fix: amend MID-002 to "every write entry (acceptRun, acceptCollection) holds the universal wrap key; read entries hold their compiled native keys (SLC-004)".
- Verdict: CONFIRMED on the spec text and in code: the T17 runner `t17-preview:tools/sherlock-sdk/t10a-ledger-run.mjs:644-650` asserts the framed wrap key on every installed entry of the three-entry T10 form, so it encodes the MID-002 reading, not SLC-004. The ledger-verification oracle does not model MID-002; settlement models SLC-004.

## FAL-SETTLEMENT-1 [Formal-Lean-AC -> Fable-C, lead] In-tree LDG-001..005, LW-009 (and DCD-004) are pre-T20 text that contradicts SLC-001

- IDs: LDG-001, LDG-003, LDG-004, LDG-005, LW-009, DCD-004 against SLC-001, SLC-006, LDG-007 (context settlement).
- Statement: the in-tree ledger-contract text (commit 72f31331a) keeps the modular-leaf contract: a foreign map and `verified_roots` (LDG-001), `acceptCollection(reg_root, set_root, shape_d)` (LDG-003), a `descend` entry (LDG-004), an `openClaim` entry (LDG-005, four `openClaim` mentions), and LW-009 publishes `[reg_root, set_root]`. SLC-001 fixes four entries (acceptRun, acceptCollection, readRoot, readRun) with no `descend` or `openClaim`. The amended text exists on t20-batched (5dc1be2ee: `[reg_pub, claims_root]`, claims tree depth 20) but is not merged (LPC-010).
- Counterexample: the two sentences. SLC-001 "exactly four entries" against LDG-004 "the contract SHALL expose `descend`" and LDG-005 "`openClaim`".
- Proposed fix: merge the t20-batched LDG-001..005 and LW-009 text; retire DCD-004's `descend` clause. Note for the lead: inv.json took the LDG-001..005 sentences from t22-governance (pre-T20); the inventory should take the t20-batched text.
- Verdict: OPEN DESIGN (spec text; merge decision LPC-010). The oracle models the t20-batched text (LEAD PICK); LDG-001, -003, -004, -005 and LW-009 are FINDING rows until the merge.

## FAL-SETTLEMENT-2 [Formal-Lean-AC -> Fable-C] ANC-012's K_MAX formula and BAT-005's cap of 2 disagree above 5,265 s TTL

- IDs: ANC-012, BAT-005, BAT-006.
- Statement: ANC-012 sets `K_MAX = min(floor(TTL / 1,755 s), 18)`, which gives 8 at a 14,400 s TTL (the runner default per the BAT-006 status). BAT-005 caps a plan at 2 chunks at every TTL; its status says it supersedes the ANC-012 formula, but the ANC-012 sentence is unchanged.
- Counterexample (oracle): `Settlement.Consistency.finding_anc012_formula`: `min(14400 / 1755, 18) = 8` and `kMax 14400 = 2`. Ops `9000014400` (intent 14,400 s), `10000002971` (plan 2,971 words = 4 chunks): the oracle refuses (`planRefused` true, `lastK` 4); ANC-012 as written admits it.
- Proposed fix: amend ANC-012 to `K_MAX = min(floor(TTL / 1,755 s), 2)` or cite BAT-005 as the cap.
- Verdict: CONFIRMED on the spec text. Code: the plan tool (rekey-chips base-0082832 `examples/sherlock_plan.rs`) prints plan words and has no chunk cap, so the chunk planner is not built; the code side is OPEN DESIGN until it is.

## FAL-SETTLEMENT-3 [Formal-Lean-AC -> Fable-C, governance lane] BAT-010 and BAT-011 admit a landing at exactly end_eff, where ERT-004 accepts nothing

- IDs: BAT-010, BAT-011 against ERT-004 (and GOV-006, which sets `end_eff`).
- Statement: ERT-004 makes `active(e)` the half-open interval `[start, end_eff)`. BAT-011 refuses a run only when proving plus inclusion "ends after" `end_eff`, so a run that lands exactly at `end_eff` starts. BAT-010 closes the epoch at `end_eff - proof time - inclusion time`, so the collection root lands exactly at `end_eff`. In both cases the era no longer accepts at landing; BAT-010's scenario says "lands before `end_eff`".
- Counterexample (oracle): `finding_bat011_boundary`: params collBudget 0, inclTime 100; ops `7000001000, 4000000143, 8000000757` (era end 1,000, now 143, proving 757 s): `runsStarted = 1`, landing 1,000 = end_eff, `active 0 1000 1000 = false`. `finding_bat010_boundary`: collBudget 200, inclTime 100; ops `7000002000, 5000000000, 4000001700`: close is enabled at 1,700, landing 2,000 = end_eff.
- Proposed fix: BAT-011 "ends at or after the announced `end_eff`"; BAT-010 "when the time reaches `end_eff` minus the collection proof time, the inclusion time and one block" (or any strict margin).
- Verdict: OPEN DESIGN (spec text; the settle runner and epoch operator are TO BUILD, no code to check). The oracle models the sentences as written.

## FAL-NETOPS-1 [Formal-Lean-AC -> Fable-C] PVW-006 trigger is weaker than the FRC-006 guard

- IDs: PVW-006, FRC-006 (context network-operations).
- Statement: PVW-006 obliges the runner to submit the 4 controls "when the honest call is included". FRC-006 forbids starting the `refusals` step while the readback of the honest call is absent. An included call whose readback stays absent (indexer lag, the FRC-013 scenario) satisfies the PVW-006 trigger and the FRC-006 refusal at once: both cannot hold.
- Counterexample (oracle ops, `o c a = c*10^9+a`): `toDeployed ++ [o 9 1, o 11 1, o 12 0, o 13 1, o 16 0]` -> `included = true`, `lastRefusal = 13` (refusals refused), so no control can be submitted (Tests `pin_no_readback`).
- Proposed spec fix: PVW-006 "When the honest call is included and its readback is present, the Preview runner SHALL submit the 4 controls ...".
- Verdict: GUARD HOLDS. Validated on the real code: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/t17.sh:102` asserts the readback before the refusals step. The oracle takes this reading (LEAD PICK); only the PVW-006 text is loose.

## FAL-NETOPS-2 [Formal-Lean-AC -> Fable-C] reuse of a call resolved as not included is undefined

- IDs: FRC-003, FRC-005, FRC-013.
- Statement: FRC-003 reuses only a "proven, unsubmitted and unrefused" call. FRC-013 resolves an unknown inclusion state and its scenario says the runner "does not mark the saved call refused". A call that was submitted, timed out (PVW-008) and resolved as not included is then neither unsubmitted (FRC-003 does not reuse it) nor refused (FRC-005 does not re-prove it). No requirement says what the next run does.
- Counterexample: `toDeployed ++ [o 9 1, o 11 1, o 12 0, o 13 2, o 14 0, o 10 7201]`: the oracle keeps the call saved and reusable.
- Proposed spec fix: FRC-003 "While a proven, unrefused call that is not included has more than 5 minutes of TTL left ..." (or FRC-013 names the re-prove).
- Verdict: OPEN DESIGN. LEAD PICK in the oracle: reuse (same claim, not settled; LDG-002 makes a wrong resubmit cost a fee and no state). Real code (`t10a-ledger-run.mjs:686-689`) marks such a call refused, which is the known FRC-013 TO BUILD gap (FAL-NETOPS-4).

## FAL-NETOPS-3 [Formal-Lean-AC -> Fable-C] FRC-004 says nothing about a non-transport balancing error

- IDs: FRC-004, OBS-008.
- Statement: FRC-004 bounds retries for a transport error only. The response to any other wallet balancing error is not stated, so the attempt count and the stop are undefined for it.
- Counterexample: `toDeployed ++ [o 9 1, o 11 2]` (other error) -> the oracle stops at once with `stop = 7`, `balanceAttempts = 1` (Tests `pin_other_error`).
- Proposed spec fix: add "If wallet balancing fails with an error other than a transport error, then the ledger runner SHALL record the attempt and stop the call at once."
- Verdict: OPEN DESIGN for the text; the oracle follows the real code: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:764-775` retries only transport-like errors (4 tries, 30 s, resync) and stops at once on any other error.

## FAL-NETOPS-4 [Formal-Lean-AC -> Fable-C] runner marks an included, unindexed call refused (FRC-013)

- IDs: FRC-013, FRC-005.
- Statement: FRC-013 resolves the inclusion state from the node and the indexer; its scenario keeps the saved call unrefused when the call is included and not yet indexed. The runner marks it refused after 60 s.
- Counterexample: oracle `toDeployed ++ [o 9 1, o 11 1, o 12 0, o 13 2, o 14 1]` -> `included = true`, `savedRefused = false` (Tests `pin_resolve`); the code takes the other branch.
- Verdict: CONFIRMED on the real code (`t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:686-689`, `markRefused('A ... but not indexed')`). Already listed as TO BUILD in the FRC-013 status scenario; no spec change needed. Fix belongs to the T23 driver lane; its named replay test is `pin_resolve`.

## FAL-COMPACT-COMPILATION-1 [Formal-Lean-AC -> Fable-A] CF-001 reads `Lflattened`; ECC-001 reads only ZKIR 3.1

- IDs: CF-001 against ECC-001, MYC-FE-001 (context compact-compilation).
- Statement: CF-001 says the frontend SHALL read `Lflattened` after `optimize-circuit2` inside `compactc`. ECC-001 says the Mycroft compiler SHALL read only the ZKIR 3.1 files and `contract-info.json` of stock `compactc` 0.36.0. Both cannot hold for one compiler. The supersession of CF-001 is stated only in the ECC-001 status scenario and D-016, and only for the prototype.
- Counterexample: the two sentences. Lean: `CompactComp.cf001_ecc001_conflict` proves that for any design with `prototype = false`, `CF_001_ok` and `ECC_001_ok` are not both true; `consistent` holds only at the prototype witness.
- Proposed fix: add to CF-001 "Where the build is not the e2e prototype (D-016)" or retire CF-001 in favour of ECC-001 in the completion spec.
- Verdict: OPEN DESIGN (spec text). Coverage row CF-001 = FINDING until the spec is changed.

## FAL-COMPACT-COMPILATION-2 [Formal-Lean-AC -> Fable-A, Mycroft lane] The measured suite split differs between REPORT.md and SUITE-COVERAGE.md

- IDs: ECC-010, ECS-006, ECS-011, ECS-013 (contexts compact-compilation, conformance).
- Statement: worktree mycroft-factory `mycroft/factory/REPORT.md:9` (09:19) says 139 calls equal, 6 blocked on a kit; `mycroft/compiler/SUITE-COVERAGE.md:5` and `compiler/out/suite-coverage.json` totals (10:22, kits spike+sha256+bytes+secp256k1+jubjub) say 142 equal, 3 blocked. `REPORT.md:77` says "101 of 105 ZKIR circuits compile" while `REPORT.md:8` says 106 circuits compile. Totals 33 / 106 / 208 agree in all sources; the equal / blocked split and the compiled-circuit count do not.
- Counterexample: the cited lines. Both splits sum to 208 (`measured_suite_calls` pins the REPORT.md split; 142 + 62 + 1 + 3 = 208 also holds).
- Proposed fix: regenerate REPORT.md lines 9 and 77 from `suite-coverage.json`; state per count which tool run it comes from (ECS-013 identity).
- Verdict: CONFIRMED (validated on the files of the worktree, 2026-11-15). No transition changes; the oracle constants are sums only.

## FAL-COMPACT-COMPILATION-3 [Formal-Lean-AC -> Mycroft lane] ECC-014 and ECC-007 are not yet met by the compiler

- IDs: ECC-014, MYC-VAL-014, ECC-007, MYC-BLT-007.
- Statement: the spec admits a Jubjub point input as two tape words and keccak256 as one chip. The measured suite still blocks `j02-schnorr/verifySchnorrN3` (2 calls, "BLOCKED: Point<Jubjub> argument (two-word wide argument, Lowering/WideArgs.lean plan)") and `s02-foreign/keccak` (1 call, "keccak256 kit (R_SL side built; compiler semantics pending)").
- Counterexample: oracle `deploy=1`, ops `[1000001111, 2000000000]` (stock, IR 3.1, kind Jubjub point, shapes ok) gives `phase = 1`, `accepted = 1`, `tapeWords = 2`, `curveChecks = 1`, `images = 1`; the code gives BLOCKED (suite-coverage.json rows j02-schnorr, s02-foreign).
- Proposed fix: none to the spec; implement the WideArgs lowering and the keccak compiler semantics, then rerun mycroft-suite.
- Verdict: CONFIRMED (code gap against the spec, validated on `compiler/out/suite-coverage.json`). Rows stay MODELED; the code is not conformant yet.

## FAL-COMPACT-COMPILATION-4 [Formal-Lean-AC -> Fable-A] ECC-010 gives no precedence among refusal causes

- IDs: ECC-010, ECC-013, ECC-001, MYC-FE-002, CF-014, ECC-005, MYC-SEAL-021.
- Statement: ECC-010 asks for "one refusal code" per circuit, but a circuit can meet several refusal conditions; the specs name no order, so the printed code (ECC-013) is not determined.
- Counterexample: oracle `deploy=1`, ops `[1000010000, 2000000000]` (not stock and slot clash) gives `lastRefusal = 1` (carrier); an order seal-first gives 5.
- Proposed fix: add to ECC-010 "in the order read, IR version, input types, lowering, seal; the first failing check gives the code".
- Verdict: OPEN DESIGN. LEAD PICK in the oracle: that pipeline order (marked for T15).

## FAL-CONFORMANCE-1 [Formal-Lean-AC -> Fable-A] A call can be class pass with no Preview transaction

- IDs: ECS-004, ECS-008, ECS-010, ECS-014.
- Statement: with `T23_ALLOW_DISPATCH=1` and `T23_ALLOW_PREVIEW` not set, settle runs dry (ECS-008), compare can still find the native transcript equal, and ECS-004 then permits class pass. ECS-010 asks for the Preview transaction hash and block per call, and ECS-014 asks for an included transaction. The spec does not say whether a dry-settled call is pass, blocked or something else.
- Counterexample: oracle `config=1`, ops `[1000000001, 2000000001, 3000000001, 4000000000, 6000000000, 7000000003, 8000000000]` gives `cls = 1` (pass), `previewTx = 0`, `evidence = 3`, `eligibleIncomplete = 1`, `complete = false`, `completeBlocked = 1`.
- Proposed fix: ECS-004/ECS-005: "If settle ran without a Preview transaction, then the T23 runner SHALL class the call as blocked with reason `preview off`".
- Verdict: OPEN DESIGN for the class; GUARD HOLDS for completion (ECS-014 refuses the report, `ECS_014_complete_needs_evidence`). Validated on the oracle; `t23.py` not read.

### D-leaf-2 [Formal-Quint -> Formal-Lean-B] A refused foreign leaf blocks the N1 merge in the oracle
Replay disagreement, trace `formal/sherlock-e2e/leaf-proving/traces/regressions/D-leaf-2.itf.json` (projected run 1: admit, prove, foreign leaf of program 4, publish, merge). Oracle: op 2 (`foreignLeaf`) is accepted and sets `badLeaves = 1`; op 4 (`n1Merge`) is then refused `wrong-prog-d` for ever. Model: N1 refuses the foreign leaf and still merges the run's own leaves. LFP-007: "the N1 statement AIR SHALL refuse **the leaf**". Lead ruling (resolve-disagreements rule): the model reads the sentence; the refusal is of that leaf, not of the run. Oracle fix asked: op 2 with `p != progD` answers `{"refused": ["wrong-prog-d"]}` and leaves the state unchanged (or keeps `badLeaves` as a count that does not guard op 4). 7 refusal checks give `outside-capability` only as a cascade of this (state `merged` with `badLeaves = 1`). Everything else agrees: 445 projected chains, 441 refusal checks on 100 traces of 25 steps.

## Slice B resolution (slice B resolver, 2026-11-15 12:00)
Counterexamples on openvm `rekey-int` 3826e38 (odin): `formal/sherlock-e2e/COUNTEREXAMPLES.md` section "Slice B".
- F-B-BR-1: CONFIRMED, FIXED. LTR-003 text 2,200 columns (= `frozen.rs:202`). LEAD PICK FP3-LCOLS for T15, with the T25 conditions of `LIST.md:69`. Lean replay `Bridge.ltr003_fp3_accepted`.
- F-B-BR-2: CONFIRMED, FIXED in the text (no code fix request). Decision (LEAD PICK for T15): the 7th operand `pack(C_(j+1))` is not an encoding detail. It is the FP-3 chain link, and a mutant that fixes it to a constant lets a 3-slice run close as 2 chunks (`fp3_k_is_the_true_slice_count` fails). It keeps the canon form: one `U_LOCK` sponge over the run's bound operands, `j` last, `A_j = PoseidonFr_4(SHK_LOCK, x_j)`, `A_0 = A_k = C_P`. The owner accepted the code form in D-038 (FP-3, `LIST.md:46` names the 7-operand lock), as the owner accepted the T10q-ntfix operand extension on 2026-11-15 09:04. To go back to 6 operands would open the truncation again. Text changed: CLB-005, ANC-002, ANC-017, HDM-001 (capacity `7 + 8 * 2^66`), D-013. Test `fbbr2_six_operand_lock_text_vs_fp3_code`: the old 6-operand lock is refused.
- F-B-AGG-1: CONFIRMED, FIXED. SHP-003 "one or two enabled prior SEG proofs" (LEAD PICK for T15). A PCD tree of any shape (GPH-005) has nodes with one child. Replay `afk_f9_work_one_child` accepted, unlinked one child refused.
- F-B-BF-2: FIXED. PDM-004 evidence OPEN, verification TO BUILD (no record times the STARK worker alone).
- F-B-LFP-1: GUARD HOLDS. F-B-KM-2: GUARD HOLDS. F-B-LFP-3: closed by D-024 (OWNER 2026-11-15, temporary named refusal).
- Minor: UKS-010 status all six rows (LIST.md 11:38); LFP-006 `core.rs:46`; PDM-001 `log.md:3691`; MERGE-WORK texts now give 4,988 MEASURED (`frozen.rs:166` to `:167`), 5,121 DERIVED (`:168`), bound at `:169` (rekey-int 3826e38), with 5,128 and 5,284 as earlier layouts.
- Codec: slice B Lake package renamed `sherlock_codec_b` (lib and module `SherlockCodecB`, folder `codec-lean`), so it and `codec` (`sherlock_codec`) build side by side.

## FAL-GOVERNANCE-1 [Formal-Lean-AC -> Fable-C, governance lane] EGV-002 requires threshold exactly 1; ERT-002 and ELD-002 accept threshold >= 1
- IDs: EGV-002, EGV-001, ERT-002, ELD-002.
- Statement: EGV-002 stops and marks unusable a contract whose readback "differs from the maintenance authority `{[], 1, 0}`". ERT-002 (reused by ELD-002) requires `committee == []`, `threshold >= 1`, `counter == 0`. A readback `{[], 2, 0}` passes ERT-002 and fails EGV-002, so the two sentences give different deploy outcomes for the same readback.
- Counterexample (oracle, params tl 10 rtl 5 q 2 n 3 grid 10): ops `[9000010200]` (deploy table, committee 0, threshold 2, counter 0, vkEq 1) gives `tableSealed = false`, `unusable = 1` under LP-2; under ERT-002 the table is sealed. Pinned by `pin_threshold2`.
- Proposed fix: write the same rule in both: either EGV-002 "differs from an empty committee with threshold >= 1 and counter 0", or ERT-002 `threshold == 1`. LEAD PICK LP-2: exactly 1, since EGV-001 deploys exactly `{[], 1, 0}` and any other readback means a different deploy path ran.
- Verdict: CONFIRMED on spec text (low). Not validated on code: no deploy tool exists yet (EGV-001 status MISSING, T22 tasks.md task 4).

## FAL-GOVERNANCE-2 [Formal-Lean-AC -> Fable-C, governance lane] ERT-004 `end_eff(e)` does not exclude `P = e`, and `min` with an unset `end` is undefined
- IDs: ERT-004, ERT-005, GOV-003.
- Statement: ERT-004 defines `end_eff(e) = min(end(e), start(P))` "for a pending era `P`". Read with `P = e`, a pending era has `end_eff(P) <= start(P)` and is never active before `activate`, which contradicts the ERT-004 scenario "Pending era reached without activate" (`active(P)` succeeds). The sentence also gives no value for `min` when `end(e)` is not set or no era is pending.
- Counterexample (two sentences): ERT-004 sentence vs its scenario "THEN `active(N)` fails for the old era `N` and succeeds for `P`".
- Proposed fix: "`end_eff(e) = min(end(e), start(P))` for a pending era `P != e`, where an unset `end(e)` or no pending era counts as unbounded".
- Verdict: CONFIRMED on spec text (low; the scenario fixes the reading). LEAD PICK LP-1 takes `P != e` with unbounded defaults; `pin_handover` and `pin_before_handover` pin it.

## FAL-GOVERNANCE-3 [Formal-Lean-AC -> Fable-C] ELD-006 says `L_TX` = 1,024 lanes; its scenario says "below 1,026"
- IDs: ELD-006.
- Statement: the requirement sentence fixes `L_TX` = 1,024; the scenario "Measured lanes" checks 91 and 85 lanes "below 1,026". The measured values pass both bounds, so no verdict changes, but a later count of 1,025 would pass the scenario and fail the requirement.
- Counterexample: the two sentences, `t22-governance@fe7207819:openspec/changes/sherlock-governance/specs/sherlock-era-ledger/spec.md:43,47`.
- Proposed fix: scenario "below or equal to 1,024 (`L_TX`)".
- Verdict: CONFIRMED on spec text (low). `measured_ELD_006` checks 91 and 85 against 1,024.

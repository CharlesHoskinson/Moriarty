# Tasks

Status: CONSENSUS v2.1 (council vote 6/6 ACCEPT at 2026-11-07 21:29Z; text unchanged from DRAFT v2.1; amended 2026-11-08 by the owner decisions in `dissent.md` "Resolved 2026-11-08"). Ordered by milestone. Each task names its requirement IDs, its predicate row(s) and its gate. A checkbox is ticked only when the named predicate passes with its declared dependency closure unchanged (SAS-002) and, for network postconditions, a committed receipt (SWF-005). Rows marked `optional: true` (the S>1 experiment) do not gate milestones (E14). Since 2026-11-08 the B rows are mandatory but form the Fenrir gates (6.6, 7.4), outside the Sherlock C4 closure (SWF-013); Sherlock fee/security evidence always has its own mandatory A/U/F row (SMN-005). None is ticked now.

## M0 Re-arm the factory (T0, bookkeeping; SWF-006)

- [x] 0.1 `scripts/doctorwatson validate`; `scripts/doctorwatson check --set F` (re-earn F4, F11 at the council head); `state-commit`. Record the 295 h STOP and the owner action in `wiki-llm/lessons/`. Not reported as progress. [SWF-006] -> F4, F11
- [x] 0.2 Confirm `scripts/doctorwatson preflight` OK and the heavy lock free. [SWF-004]

## M1 Land this change (T0)

- [x] 1.0 E10 and E14 first: `retired:` and `optional:` row fields; `next` and `check --tier/--set` sweeps skip retired rows and skip optional rows unless `--include-optional`; explicit `check <ID>` still runs both; `validate` refuses a mandatory row depending on a retired or optional row; factory unit test over `next`, `check --tier T3`, `check --set A`, `check A8`, and "an unimplemented optional B4p or a failing existing optional B6/B7 leaves the Preview gate eligible while an unimplemented mandatory A/U/F row blocks it". [SWF-011, SWF-012] -> F16
- [x] 1.1 Then add every new predicate row with `check: scripts/qa/unimplemented.sh <ID>` and `source:` anchors into this change: A2p, A3p, A4p, A6p, A8p, A9p, A10p, A11, A12, A13, U023a-U023f, B4p, B5p, B8p, B9p (`optional: true`), F14, F15, F16; mark the retained B1, B2, B3, B6, B7 rows `optional: true` as well (D14: no B row gates this qualification; B6/B7 are recorded failing in `status.json`); add `retired:` to A8, A9, A10, B4, B5, B8, B9 (stagenet rows; B5 depends on B4 and B9 on B5-B8, `predicates.yaml:141,177`); rewire C4 and the new composites to preview rows. (Amended 2026-11-08: the B rows lost `optional:`, and B4m, B5m, B8m, B9m were added; see 6.6 and 7.4.) [SWF-002, SWF-011] -> `validate`
- [x] 1.2 E1 `scripts/qa/check-traceability.py` + F14 in `t0.sh`. [SWF-002] -> F14
- [x] 1.3 E2 `qa/sherlock/target-pins.json` + `check-target-pins.py` + F15 in `t0.sh`; rows `undeployed`, `preview`, `mainnet`, `stagenet(retired)`. [SWF-007, SMN-001, SMN-004] -> F15
- [ ] 1.4 E12 (evaluator): rejection suite and U023 holdout parametrised by network; `U023.json` contract change; reseal. Owner of this task is the evaluator role. [SWF-003, SRC-007]
- [ ] 1.5 E5/E6 T4 tier, `network:` field, `validate` rule; `qa/tiers/t4.sh` skeleton that refuses without the rehearsal receipt. [SWF-008, SWF-009]
- [ ] 1.6 E7 freshness: each row declares its dependency closure (source paths/globs, spec and contract files, pins row, key manifests, chain identity, receipt hashes); a result is stale iff any declared dependency changed since its run; no blanket directory exclusion; a state-only or receipt-only commit preserves eligibility; HEAD-only flag allowed only as an interim signal, never the gate. Tests: state-only commit -> eligible; source, spec, key, pins, chain-instance change -> stale. [SAS-002]
- [ ] 1.7 Mark MC03 superseded in its README with the obligations crosswalk; add `wiki/midnight-ledger-9-hardfork.md` reference to `wiki/midnight-repositories.md`. [proposal]
- [ ] 1.8 E15 (added 2026-11-08): milestone-scoped tier runners, so that `t2.sh`/`t3.sh` select the C4 closure for the Sherlock M6 gate and the B rows for the Fenrir gates; factory unit test: an unimplemented mandatory B4p leaves the Sherlock M6 selection eligible and blocks the Fenrir Preview selection. [SWF-012, SWF-013]

## M2 Re-pin and re-key (T1/T2; A-set)

- [ ] 2.1 Bump `Cargo.toml` to the ledger 9.1.0.1 line (`a2b30128`), `midnight-circuits 7.3.0`, `midnight-zk-stdlib 2.4.0`, `midnight-zkir` 3.2.0 line; adapt `mori-ledger`/`mori-recursion` to storage 3.0.0 (`EncodingShape`), `AlignedValueChecked`, fallible cost API, unified signing envelope; SDK root to ledger-v9 1.0.0, onchain-runtime-v4 4.0.0, Midnight.js 5.0.0 (remove `setNetworkId` in `rollup-prepare.mjs:59`, `settle-rehearsal.mjs:35`, test harnesses), Wallet SDK 2.0.0, Compact 0.36.0, proof server 9.0.x. [SMN-001] -> A1, F15
- [ ] 2.2 Fix SAS-003 defects in `qa_mutation.rs` / `soundness_mutation.rs` (synthesis error != Rejected; zero-panic requirement; typed outcomes in receipt JSON). [SAS-003] -> T1
- [ ] 2.3 Node fixes before keygen: range checks on `prev_h`, `steps`, `h` (`ivc.rs:308-310`, pattern `parent.rs:665-670`); `steps <= S`; genesis per SRC-005: select realization (a) neutral-encoding constraint or (b) zero/ignore construction with a recorded justification (an unscaled retained carry needs its own security argument; the `24a1eb5a` hash observation is an input, not the argument), confirm at `midnight-circuits 7.3.0`, record the choice before node keygen; negative tests at the constraint boundary with a valid baseline and through the final pairing. [SRC-004, SRC-005] -> A13
- [ ] 2.4 Keygen regression A11: relation and VK hashes for the C6 digest leaf relation (M2 shape, `sherlock_c6_child.rs`; never `standin.rs`), node, decider, parent, contract at the new pins vs manifests; regenerate the changed closure leaf -> node -> decider -> template; write manifests (SRC-008 fields incl. emitted ZKIR header, compiler version, zkir binary version). [SRC-008, SRC-009, SMN-002] -> A11
- [ ] 2.5 Recapture acceptRun template with Compact 0.36.0 / runtime 0.21.0; diff vs `template.rs`; read public-input limit and 256-op cap from `LedgerParameters`; A6p local proof-verifying acceptance with mainnet parameters + five refusals. [SMN-003, SMN-006] -> A6p
- [ ] 2.6 Re-measure M3/M4 (node, decider) at new pins; `budgets.json` rows. [SRC-011] -> A3p, A4p

## M3 Fragments and the dispatch receipt (T1/T2; U-set) [U023 scenario 1]

- [ ] 3.1 Instrument one state x one rule: build time, `ProofPreimage::check` time, IR instruction count per dispatch circuit; record as SAS-014 diagnostic. [SAS-014] -> U023a
- [ ] 3.2 Derive the dispatch-predicate partition mechanically from `c6_dispatch_step.rs` guards (control tag, site, top frame, helper kind/phase, result presence, fuel-zero, depth-zero, PROG facts, `equal_differ` value reads, bounds, canonicality premises); commit the partition; directed cases for rules and boundaries absent from P1-P3. [SAS-013] -> U023b
- [ ] 3.3 Split the dispatch tests: fired-rule acceptance on every step (T1); per-class exclusivity of every supported rule + coordinated/raw mutations (T2, one budget key per class group); receipt `testdata/receipts/dispatch.json` with partition fields and scope; trace under `qa/sherlock/receipt-traces/`. [SAS-013, SVM-003] -> U023c
- [ ] 3.4 Fragments + receipts: `KAdminPublishValue`, `BHelperValidate` x4, `BHelperEqual` x4 (fact3), `BHelperMatch` x10, `BHelperScalarConstruct` (list123); flip `native_inventory.rs` rows. [SVM-008] -> U023d
- [ ] 3.5 `doctorwatson check U023` scenario 1. [UIREQ-023] -> U023 (s1)

## M4 Universal leaf (T2; A-set) [G4]

- [ ] 4.1 M1' measurement: lanes per side at the qualification config; rows and k of three typical fragments. [SRC-013] -> A12
- [ ] 4.2 One-hot dispatch block (selector vector, `sum(sel)==enable`, gated fragments and post-lane writes, unsupported -> unsatisfiable); `rules_d`, `steps`, internal chain constrained (S=1 first). [SVM-003, SVM-004, SRC-003] -> A12
- [ ] 4.3 `prog_d` = digest(image, layout schema, bounds, profile); manifests updated. [SVM-005, SRC-002]
- [ ] 4.4 Freeze the admitted inventory `R`, layout, caps and manifests (SVM-008); key the universal leaf over `R` at S=1; record k, rows, RSS, time; decide U/F (k <= 17) and commit the decision JSON; if F, uniform arch wrapper + in-circuit key-table membership in the node + nonmember-key refusal test. [SRC-013, SVM-008] -> A12
- [ ] 4.5 Seven-lane producer bridge (`sherlock_c6_child.rs`) -> node harness; refuse other shapes. [SRC-001] -> A13
- [ ] 4.6 Hostile raw-witness suite for the leaf: initialization (canonical genesis admitted, non-canonical refused), preservation under `StepInv(pre)`: A4/B002 cases + publication padding + forged selector + disabled-slot write + `/2` tags; shared-carrier premise checks; fresh proofs, SAS-004 record with reached boundary per case. [SVM-002, SVM-003, SVM-004, SVM-009, SAS-004, SAS-010] -> A13
- [ ] 4.7 `sherlock_digest_parity` on P1 and P2 streams at new pins (T0); P3 parity runs in 6.4 after its post-freeze authorship and before OP-PV-CALL-P3. [SVM-007] -> F-set existing test
- [ ] 4.9 Initialization (O1, SAS-010): canonical authenticated initial carriers bound to the committed program/profile and `init_d`; non-canonical genesis, inconsistent `init_d`/`prog_d` and forged initial carriers refused at the leaf/node boundary; the inventory's relied-on premises validated per rule. [SVM-002, SAS-010] -> A13
- [ ] 4.8 (moved after 5.3) S=8 chained leaf as an experiment, only after the S=1 complete-path C0 profile: prover memory and k at the qualification config; short final chunk; own budget key and predicate so the measurement is a `runs.jsonl` line; retain S=1 if no measured benefit. [SRC-014, SRC-011] -> budgets row

## M5 Local chain (T2; A-set) [E1-E6 locally]

- [ ] 5.1 `entry 7;` (6 steps): leaves -> node -> run decider -> `verify_chain` (SRC-010 record) -> `validate_call` on local 9.1.0.1. [SRC-006, SRC-010] -> A2p, A3p, A4p, A6p
- [ ] 5.2 fact(3) (293 steps) and list123 (303 steps) through the same chain; negatives: altered result, altered program digest, forged outcome/result labels with unchanged carriers, replayed stmt, skipped decider; genesis per SRC-005's recorded realization (under (a) a non-neutral carry refused at the node boundary; under (b) a well-formed ignored carry is irrelevant, not refused; invalid leaf at genesis refused through the pairing; malformed carry encoding refused at the decoder, decoder evidence only); under F a valid proof under a nonmember key refused. [SRC-005..007, SRC-013] -> A13
- [ ] 5.3 C0 phase profile on leaf, node, decider of the `entry 7;` chain (extend `scripts/qa/profile-recursion.py` workloads); record shares in `budgets.json`. [SRC-011, RPP] -> budgets
- [ ] 5.4 Quint model of chain invariants (linkage, bounded monotone height, unique genesis, replay set) and of the one-hot selector. [assurance §3] -> model row

## M6 Preview qualification (T3; `network: preview`) [U023 scenario 2, C4]

Operations (SWF-003):

| op | preconditions | command | postcondition | authorization | receipt |
|---|---|---|---|---|---|
| OP-PV-WALLET | F15 | `heavy --id OP-PV-WALLET -- node tools/sherlock-sdk/preview-session.mjs --init` (new wallet, faucet, DUST registration, encrypted backup) | pins row has wallet address | owner | `receipt-traces/OP-PV-WALLET/` |
| OP-PV-DEPLOY | A6p, A11, F15 live | `heavy --id OP-PV-DEPLOY -- node tools/sherlock-sdk/deploy-run.mjs --network preview` (Compact 0.36.0 `SherlockRun`, decider VK installed, maintenance authority held by the owner and equal to the pins row, SMN-007) | A4p | owner | `receipt-traces/OP-PV-DEPLOY/` |
| OP-PV-CALL-P1/P2/P3 | A4p, A13, SRC-010 record | `heavy --id OP-PV-CALL-<P> -- node tools/sherlock-sdk/call-run.mjs --submit` | A8p, A10p | owner | `receipt-traces/OP-PV-CALL-<P>/` |
| OP-PV-REJECT | A8p | `heavy --id OP-PV-REJECT -- node tools/sherlock-sdk/call-run.mjs --reject-suite` | A9p | owner | `receipt-traces/OP-PV-REJECT/` |
| OP-PV-ROLLBACK-REHEARSAL | A4p, A11 | deploy -> bad key -> maintenance replace -> readback (#2375 checked before any key removal) | rehearsal receipt with observed on-chain key state | owner | `receipt-traces/OP-PV-ROLLBACK/` |
| OP-PV-FENRIR-BATCH | B3, A8p | Fenrir 16-transaction batch and prefixes on Preview (command created with 6.6) | B4p, B5p, B8p | owner | `receipt-traces/OP-PV-FENRIR-BATCH/` |

- [ ] 6.1 E13: network-parametrised session module (Midnight.js 5 `providers.config`, no `setNetworkId`), `--network` on `deploy-run.mjs`/`call-run.mjs`/`readback-run.mjs`, reject-suite driver, per-network wallet dir; fixture `crates/mori-recursion/tests/fixtures/preview-verifier.json`; `assemble-u023-record.py` re-pointed. The M6 operation commands above do not exist until this task lands. [SMN-004, SWF-003]
- [ ] 6.2 OP-PV-WALLET, OP-PV-DEPLOY; A4p. Publish the owner-held maintenance authority's public identity in the `target-pins.json` preview row (no secrets) before OP-PV-DEPLOY; that commit runs F15, so it lands after 2.1. [SMN-005, SMN-007]
- [ ] 6.3 OP-PV-CALL-P1, P2; A8p, A10p manifest-bound; `rejections-preview` holdout (E12, evaluator-owned) -> A9p. [SRC-007, SAS-007]
- [ ] 6.4 P3 written after key freeze (mix of recursion, match, arithmetic) from the admitted domain; P3 digest parity, local proof and `verify_chain`/`validate_call` first; then OP-PV-CALL-P3; E2 check: no new key. [SRC-013, SVM-007, SVM-008]
- [ ] 6.5 U023 scenario 2 records (`u023-preview-e2e.json`, `-list123`, `-p3`) against the network-parametrised U023 holdout (E12, evaluator-owned); `doctorwatson check U023 U024 U028 U029 U030 U033 C4`. [UIREQ-023] -> C4
- [ ] 6.6 Fenrir Preview gate (mandatory since 2026-11-08; outside the C4 closure, so Sherlock M6 does not wait on it): B1, B2, B3 local, B6, B7, then OP-PV-FENRIR-BATCH and B4p, B5p, B8p, B9p on Preview. No stagenet run. [SWF-013, SMN-005] -> B1, B2, B3, B6, B7, B4p, B5p, B8p, B9p
- [ ] 6.7 OP-PV-ROLLBACK-REHEARSAL, preceded by a passing A11. [SWF-010, SMN-007]

## M7 Mainnet readiness (T4; owner-only)

- [x] 7.1 Owner decision on `openspec/MORIARTY-COMPLETION-PROGRAM.md:269`: amended 2026-11-08 ("amend the program for mainnet"; now `:270-283`, old sentence kept struck through). Mainnet submissions are permitted only as T4 operations after local Docker at 9.1.0.1 has passed, the Preview qualification receipts and the Preview rollback rehearsal receipt are committed and the pre-deploy checklist has passed. Each mainnet operation still needs explicit owner authorization when it runs, under an owner-set per-operation spend ceiling with no automatic retry. The amendment authorizes no spend. [SWF-009]
- [ ] 7.2 Blockfrost project; mainnet wallet; cNIGHT -> DUST registration about 12 h ahead; `target-pins.json` mainnet row with genesis, the owner-held maintenance authority identity, and a status note that cites the 2026-11-08 amendment instead of the `:269` ban. [SMN-004, SMN-005, SMN-007]
- [ ] 7.3 OP-MN-DEPLOY (`owner+checklist`, maintenance authority per SMN-007), OP-MN-CALL-P1, non-consuming reject cases; A8m, A9m, A10m; committed receipts; each operation separately authorized by the owner when it runs. [SRC-007, SAS-007, SWF-009, SMN-007]
- [ ] 7.4 Fenrir mainnet gate (after 6.6 and 7.3): OP-MN-FENRIR-BATCH (`owner+checklist`, owner-authorized when it runs, owner-set spend ceiling, no automatic retry); committed receipts. [SWF-013, SWF-009, SMN-005] -> B4m, B5m, B8m, B9m
- [ ] 7.5 Rollback plan executed only via §7 of `workflow.md`; every maintenance update preceded by a passing A11 and followed by a committed receipt of the observed on-chain key state; #2375 checked before any key removal. [SWF-010, SMN-007]

## Later changes (not in this change)

- `/2` profile native constraints (Await, Waiting, LogicalCap) and `sherlock-run-anchor` [D8].
- Heavy concurrency lock class (E11).
- Generic ADT instantiation policy, higher-order and letrec extensions (UIREQ-034 refinements).
- Freeze or renounce the `SherlockRun` maintenance authority once the keys are stable (SMN-007, decision 2026-11-08). Until then the authority holder (the owner) can install a verifier key and so change what the contract accepts.

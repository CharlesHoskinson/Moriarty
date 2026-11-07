# Tasks

Status: DRAFT v1. Ordered by milestone. Each task names its requirement IDs, its predicate row(s) and its gate. A checkbox is ticked only when the named predicate passes at the current head with a committed receipt (SWF-005). None is ticked now.

## M0 Re-arm the factory (T0, bookkeeping; SWF-006)

- [ ] 0.1 `scripts/doctorwatson validate`; `scripts/doctorwatson check --set F` (re-earn F4, F11 at the council head); `state-commit`. Record the 295 h STOP and the owner action in `wiki-llm/lessons/`. Not reported as progress. [SWF-006] -> F4, F11
- [ ] 0.2 Confirm `scripts/doctorwatson preflight` OK and the heavy lock free. [SWF-004]

## M1 Land this change (T0)

- [ ] 1.1 Add every new predicate row with `check: scripts/qa/unimplemented.sh <ID>` and `source:` anchors into this change: A2p, A3p, A4p, A6p, A8p, A9p, A10p, A11, A12, A13, U023a-U023f, B4p, B8p, B9p, F14, F15, F16; add `retired:` to A8, A9, A10, B4, B8 (stagenet rows). [SWF-002, SWF-011] -> `validate`
- [ ] 1.2 E1 `scripts/qa/check-traceability.py` + F14 in `t0.sh`. [SWF-002] -> F14
- [ ] 1.3 E2 `qa/sherlock/target-pins.json` + `check-target-pins.py` + F15 in `t0.sh`; rows `undeployed`, `preview`, `mainnet`, `stagenet(retired)`. [SWF-007, SMN-001, SMN-004] -> F15
- [ ] 1.4 E10 `retired:` field, `next` skip, factory unit test. [SWF-011] -> F16
- [ ] 1.5 E5/E6 T4 tier, `network:` field, `validate` rule; `qa/tiers/t4.sh` skeleton that refuses without the rehearsal receipt. [SWF-008, SWF-009]
- [ ] 1.6 E7 stale flag in `next`. [SAS-002]
- [ ] 1.7 Mark MC03 superseded in its README with the obligations crosswalk; add `wiki/midnight-ledger-9-hardfork.md` reference to `wiki/midnight-repositories.md`. [proposal]

## M2 Re-pin and re-key (T1/T2; A-set)

- [ ] 2.1 Bump `Cargo.toml` to the ledger 9.1.0.1 line (`a2b30128`), `midnight-circuits 7.3.0`, `midnight-zk-stdlib 2.4.0`, `midnight-zkir` 3.2.0 line; adapt `mori-ledger`/`mori-recursion` to storage 3.0.0 (`EncodingShape`), `AlignedValueChecked`, fallible cost API, unified signing envelope; SDK root to ledger-v9 1.0.0, onchain-runtime-v4 4.0.0, Midnight.js 5.0.0 (remove `setNetworkId` in `rollup-prepare.mjs:59`, `settle-rehearsal.mjs:35`, test harnesses), Wallet SDK 2.0.0, Compact 0.36.0, proof server 9.0.x. [SMN-001] -> A1, F15
- [ ] 2.2 Fix SAS-003 defects in `qa_mutation.rs` / `soundness_mutation.rs` (synthesis error != Rejected; zero-panic requirement; typed outcomes in receipt JSON). [SAS-003] -> T1
- [ ] 2.3 Node fixes before keygen: range checks on `prev_h`, `steps`, `h` (`ivc.rs:308-310`, pattern `parent.rs:665-670`); `steps <= S`; `prev_acc` scaled by `not_genesis` or the recorded accumulate-binding argument (`ivc.rs:318-364`); negative tests through the pairing. [SRC-004, SRC-005] -> A13
- [ ] 2.4 Keygen regression A11: relation and VK hashes for leaf stand-in, node, decider, parent, contract at the new pins vs manifests; regenerate the changed closure leaf -> node -> decider -> template; write manifests (SRC-008 fields incl. emitted ZKIR header, compiler version, zkir binary version). [SRC-008, SRC-009, SMN-002] -> A11
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
- [ ] 4.4 Key the universal leaf for P1-P3 rule set at S=1; record k, rows, RSS, time; decide U/F (k <= 17); if F, uniform arch wrapper + in-circuit key-table membership in the node. [SRC-013] -> A12
- [ ] 4.5 Seven-lane producer bridge (`sherlock_c6_child.rs`) -> node harness; refuse other shapes. [SRC-001] -> A13
- [ ] 4.6 Hostile raw-witness suite for the leaf: A4/B002 cases + publication padding + forged selector + `/2` tags; fresh proofs, SAS-004 record. [SVM-002, SVM-003, SVM-009, SAS-004] -> A13
- [ ] 4.7 `sherlock_digest_parity` on P1-P3 streams at new pins (T0). [SVM-007] -> F-set existing test
- [ ] 4.8 (after 4.4) S=8 chained leaf as an experiment: prover memory and k at the qualification config; short final chunk. [SRC-014, SRC-011] -> budgets row

## M5 Local chain (T2; A-set) [E1-E6 locally]

- [ ] 5.1 `entry 7;` (6 steps): leaves -> node -> run decider -> `verify_chain` (SRC-010 record) -> `validate_call` on local 9.1.0.1. [SRC-006, SRC-010] -> A2p, A3p, A4p, A6p
- [ ] 5.2 fact(3) (293 steps) and list123 (303 steps) through the same chain; negatives: altered result, altered program digest, replayed stmt, skipped decider, malformed genesis carry. [SRC-005..007] -> A13
- [ ] 5.3 C0 phase profile on leaf, node, decider of the `entry 7;` chain (extend `scripts/qa/profile-recursion.py` workloads); record shares in `budgets.json`. [SRC-011, RPP] -> budgets
- [ ] 5.4 Quint model of chain invariants (linkage, bounded monotone height, unique genesis, replay set) and of the one-hot selector. [assurance §3] -> model row

## M6 Preview qualification (T3; `network: preview`) [U023 scenario 2, C4]

Operations (SWF-003):

| op | preconditions | command | postcondition | authorization | receipt |
|---|---|---|---|---|---|
| OP-PV-WALLET | F15 | `heavy --id OP-PV-WALLET -- node tools/sherlock-sdk/preview-session.mjs --init` (new wallet, faucet, DUST registration, encrypted backup) | pins row has wallet address | owner | `receipt-traces/OP-PV-WALLET/` |
| OP-PV-DEPLOY | A6p, A11, F15 live | `heavy --id OP-PV-DEPLOY -- node tools/sherlock-sdk/deploy-run.mjs --network preview` (Compact 0.36.0 `SherlockRun`, decider VK installed, maintenance authority declared) | A4p | owner | `receipt-traces/OP-PV-DEPLOY/` |
| OP-PV-CALL-P1/P2/P3 | A4p, A13, SRC-010 record | `heavy --id OP-PV-CALL-<P> -- node tools/sherlock-sdk/call-run.mjs --submit` | A8p, A10p | owner | `receipt-traces/OP-PV-CALL-<P>/` |
| OP-PV-REJECT | A8p | `heavy --id OP-PV-REJECT -- node tools/sherlock-sdk/call-run.mjs --reject-suite` | A9p | owner | `receipt-traces/OP-PV-REJECT/` |
| OP-PV-ROLLBACK-REHEARSAL | A4p | deploy -> bad key -> maintenance replace -> readback | rehearsal receipt | owner | `receipt-traces/OP-PV-ROLLBACK/` |

- [ ] 6.1 `preview-session.mjs` from `stagenet-session.mjs` with Midnight.js 5 `providers.config`; fixture `crates/mori-recursion/tests/fixtures/preview-verifier.json`; `assemble-u023-record.py` re-pointed. [SMN-004] 
- [ ] 6.2 OP-PV-WALLET, OP-PV-DEPLOY; A4p. [SMN-005, SMN-007]
- [ ] 6.3 OP-PV-CALL-P1, P2; A8p, A10p manifest-bound; `rejections-preview` holdout -> A9p. [SRC-007, SAS-007]
- [ ] 6.4 P3 written after key freeze (mix of recursion, match, arithmetic); OP-PV-CALL-P3; E2 check: no new key. [SRC-013]
- [ ] 6.5 U023 scenario 2 records (`u023-preview-e2e.json`, `-list123`, `-p3`); `doctorwatson check U023 U024 U028 U029 U030 U033 C4`. [UIREQ-023] -> C4
- [ ] 6.6 B rows: B3 local, B4p, B8p, B9p on Preview (non-gating). [D14]
- [ ] 6.7 OP-PV-ROLLBACK-REHEARSAL. [SWF-010, SMN-007]

## M7 Mainnet readiness (T4; owner-only)

- [ ] 7.1 Owner decision on `openspec/MORIARTY-COMPLETION-PROGRAM.md:269`; until amended, M7 does not start. [SWF-009]
- [ ] 7.2 Blockfrost project; mainnet wallet; cNIGHT -> DUST registration about 12 h ahead; `target-pins.json` mainnet row with genesis. [SMN-004, SMN-005]
- [ ] 7.3 OP-MN-DEPLOY (`owner+checklist`), OP-MN-CALL-P1, non-consuming reject cases; A8m, A9m, A10m, B8m; committed receipts. [SRC-007, SAS-007]
- [ ] 7.4 Rollback plan executed only via §7 of `workflow.md`. [SWF-010]

## Later changes (not in this change)

- `/2` profile native constraints (Await, Waiting, LogicalCap) and `sherlock-run-anchor` [D8].
- Heavy concurrency lock class (E11).
- Generic ADT instantiation policy, higher-order and letrec extensions (UIREQ-034 refinements).

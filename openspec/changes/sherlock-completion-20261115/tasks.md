# Tasks

Status: S2. Tick a task only when its verification passes with a record. A planned command is not evidence.

## 1. Rekey inputs

- [ ] 1.1 Prove RSL-001 on odin r10: the rsl probe gives one N1 pre-hash for a05, t02, s01 and d04. Output: `~/sherlock/logs/rekey-chips/`. Verify: `n1_pre_hash_equals_universal_n1` true for each.
- [ ] 1.2 Prove the 142 R_SL suite calls at app level on odin and huginn (RSL-003). Verify: 142 of 142 accepted by the stock verifier.
- [ ] 1.3 Build the keccak256 and secp256k1 kits (RSL-004, RSL-005) on openvm `rekey-chips`. Verify: s02 calls equal to native, RSL-001 holds.
- [ ] 1.4 Lower `Scalar<Secp256k1>` and `Point<Secp256k1>` in `mycroft/compiler/src/zkir.rs` (CF-013). Verify: mycroft-suite compiles the s02 secp256k1 calls.
- [ ] 1.5 Add the canonical range-check row to `~/.cache/rekey-t11/LIST.md` and build it (RSL-008). Verify: forged non-canonical words refused.
- [ ] 1.6 Commit the T11 guards and record EQ-ZK, the seeded guard, slot reuse and the GPU half (ZK-012 to ZK-017). Verify: T11 RESULTS sections 3, 4, 5 and 7 hold MEASURED values.
- [ ] 1.7 Commit FP-3 on openvm `fp3` (ANC-018, ANC-019). Verify: `nt1_residual_shifted_write_is_fp3` and `fp3_k_is_the_true_slice_count` fail before and pass after.
- [ ] 1.8 Commit T12-SLOT15 in `iface.rs`, `decode.rs` and SherlockLedger (ANC-013, ANC-014). Verify: `t12_anchor_slot_15` and the SherlockLedger slot-15 deploy test.
- [ ] 1.9 Add the T-ANCHOR-DISJOINT census to Mycroft (ANC-015). Verify: every suite contract avoids index 15 or is refused.
- [ ] 1.10 Update LIST.md: C3.1 is committed (Moriarty `t10q-ntfix` 95990a6d3). Add rows for FP-3, T12-SLOT15 and the range checks (RKY-002).

## 2. Rekey and gate

- [ ] 2.1 Run the single rekey (RKY-001). Verify: keygen record with the sha256 of LIST.md.
- [ ] 2.2 Measure wrap rows, VK bytes and D3 (RKY-003 to RKY-005). Verify: at most 1,782,573 rows at k = 21, at most 50,000 bytes, every term at least 100 bits.
- [ ] 2.3 Build the G3 legs and run G3, G4, G1, G2, FP-8 and the ledger check (SQG-014 to SQG-016). Verify: RUNBOOK logs with no UNVERIFIED step.

## 3. Suite and prototype

- [ ] 3.1 Commit the acceptRun argument builder (STL-001). Verify: T17 (a) arguments reproduced.
- [ ] 3.2 Prove, deploy and settle the suite (STL-002 to STL-005). Verify: suite table with mismatch 0.
- [ ] 3.3 Finish Part A (STL-006). Verify: finalize unit commits the report.
- [ ] 3.4 Run T17 (b) and (c) and T19 (LPC-008, LPC-009). Verify: Preview receipts.

## 4. Loop records

- [ ] 4.1 Add row T8c to LOOP.md and commit rows T24 and T25 (LPC-002, LPC-003, LPC-020).
- [ ] 4.2 Merge `t20-batched` and change the ANC-012 status words to owner-accepted (LPC-010).
- [ ] 4.3 Replace the `P2(DOM_LOCK)` form in FINAL-DESIGN.md 1.6 and ARCHITECTURE.md (LPC-017). Verify: no lock value uses `DOM_LOCK`.
- [ ] 4.4 File the missing lead verdicts (LPC-012, LPC-016).
- [ ] 4.5 Commit the T18 (d) deliverables and pass `gates.sh` and `lake build --wfail` (LPC-013, LPC-019).
- [ ] 4.6 Present T15 (LPC-014).

Verification of this change: `openspec validate sherlock-completion-20261115 --strict` and `python3 -I ~/.claude/skills/ears/scripts/ears_lint.py --require-ids --strict openspec/changes/sherlock-completion-20261115/specs/*/spec.md`.

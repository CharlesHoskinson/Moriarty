# T15 interface freeze: items that must land before the freeze

Owner, 2026-11-16: "add the T20 batching fix to the T15 freeze".

Each item below must be closed before T15 is presented, or stated in the presentation as an open item with the owner's ruling. LOOP.md row T15 points here.

## F-1. T20 batched writes in the deployed contract

**Finding** (Midnight deep dive, Compact lane, 2026-11-16, `~/.cache/midnight-dive/compact.md`; checked with git on 2026-11-16):
- The T20 fixes are on branch `t20-batched` (5dc1be2ee, Carbon Quint QA done). They are not in `main`, and they are not in the contract that T23 deploys on Preview: SherlockLedger at 705f1ef95 on `t23-final`.
- That contract still has the per-claim `openClaim` path and inserts one `runs` entry per acceptRun.
- The diff 705f1ef95..t20-batched touches 12 files. The Compact files are `tools/sherlock-sdk/sherlock-ledger.compact`, `sherlock-ledger-t10.compact` and `sherlock-claim-consumer.compact`, plus the two-maps control.
- The owner's batched-writes rule (one root per checkpoint, never per-event writes) therefore holds on the branch, not in the deployed contract.
- The LOOP.md T20 row reads DONE. That is true for the branch only.

**Fix (LEAD PICK, for owner confirmation at T15):**
1. Merge `t20-batched` into the T23 settle line after T23 finishes. Do not merge while the z03 settle runs: the 9 reps keep the contract they were settled with.
2. Merge content:
   - acceptCollection per epoch for operator jobs;
   - acceptRun kept for a user's own run;
   - openClaim replaced by membership against the claims root;
   - foreign claims by membership;
   - chunk cap K_MAX = 2;
   - LDG-003 to LDG-005 without descend;
   - one reg_root write per registry epoch.
3. Rebuild the contract, then re-run the T20 checks on the merged line: the writes before and after (tx and bytes per request, per batch, per second), the Quint model and its mutants, and the counterexample regressions.
4. Run one Preview acceptCollection and one acceptRun on the merged contract. Ledger-check both on muninn and record tx bytes and fees against the T23 baseline (acceptRun 0.340 DUST, about 6.7 kB).
5. Correct the T20 row status to "DONE on branch; merged into the deployed line at <commit>".

**Related changes that the same lane found (candidates, each needs a pick before the freeze; not part of the T20 merge):**
- **Claims root storage:** `HistoricMerkleTree<20, Field>` with `insertHash(upgradeFromTransient(root))` after `kernel.checkpoint()`, Midnight's own Dust/Zswap pattern. Old membership paths stay valid across inserts and the epoch is not revealed (simulated on Preview parameters, block 141264; `~/.cache/midnight-dive/compact-sim/`). The insert moves to the fallible phase.
- **Consumer nullifier:** `H(tag, claim_pub)` lets the operator or DA link each use to its claim. Use owner-bound leaves instead.
- **A `verifyRun` entry that stores nothing,** for a Compact dApp to call cross-contract in the same transaction. It removes the `runs.insert` write (324 net B, about 43 % of the acceptRun fee, DERIVED). This is blocked until the wrap pins lane 1 (`comm`) to the `transient_commit` of the call arguments (T4-WRAP-BUDGET.md:44 says "partial").

**Acceptance:**
- the merged line has no per-claim or per-event write path;
- the T20 Quint and counterexample checks pass on the merged commit;
- Preview acceptCollection and acceptRun are included and ledger-checked;
- the fee and bytes are recorded.

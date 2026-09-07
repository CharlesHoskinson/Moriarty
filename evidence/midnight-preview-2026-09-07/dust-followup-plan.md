# Bounded DUST recovery and call follow-up

User approved the proposed diagnosis/fix/call/finality sequence with “begin”.
Start: 2026-09-07T03:33:36Z. Public diagnostic/proving ceiling: 20 minutes;
existing proof-server container limits remain unchanged. No native R3 run.

Question: can the existing funded Preview wallet recover its DUST view and
settle the existing hello-world call? Keep all failed receipts and original
private state. No new seed, faucet request, deployment or Preprod restore.

Read-only observations show a synced wallet with zero exposed DUST coins and
an empty SDK pending list. A temporary copy processed beyond the grace period
reveals its sequence1 coin, without modifying live state. Wallet and chain DUST
parameters agree. Installed SDK submission catches errors and calls revert;
CoreWallet reconciliation filters pending entries using ledger utxos, which
exclude internally reserved coins. This supports a reservation bookkeeping
failure but does not yet explain error170 itself.

Recovery: replay only DUST from genesis in external preview-dust-recovery-20260907,
using the same seed and copies of the shielded/unshielded snapshots. Retain the
original directory. Bound replay to240seconds. Compare recovered balance and
pending coins, then one instrumented call after fresh sync. A second call is
allowed only if a distinct evidenced cause supports a correction. Stop on
repeated unknown rejection, elapsed budget or success. Do not forcibly expire
reservations in the live wallet, relax node verification, or overwrite receipts.

Acceptance: existing contract deploy and call both indexed SUCCESS, both blocks
canonical and finalized by node RPC, exact message readback. This remains a
Compact network test, not Moriarty PCD or financial-semantics acceptance.

Replay stopped at its240second cap and saved DUST offset171818 of approximately
205000 events. Continue once from that saved offset for at most120seconds,
within the same20minute overall ceiling. This processes the remaining history;
it does not discard/repeat the completed scan. No transaction has been retried.

The instrumented runner initially failed before proving/submission: its dynamic
resolver selected the protocol package CommonJS export, which references a
missing Compact JS CJS artifact. Select the corresponding installed ESM export,
as the working CLI does. No public transaction was generated in that launch.

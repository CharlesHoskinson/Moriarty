# Finalized loan and swap state on the local Midnight node

Actual observation: September 10, 2026. Independent GPT-6 Astra and Grok 4.6 result reviews both returned PASS_SCOPED.

The corrected probe read both deployed contracts at finalized block **20,415**,
`0xe54358be199c4b34f94a74237452c1bb7d78b8a3cb6bd042a199590cbd092aac`.
It retained each full native serialized state, decoded financial fields and
complete native balance map in `probe-result.json`.

| Case | Observed financial state | Full-state SHA-256 |
| --- | --- | --- |
| Loan | Revision 2, remaining 0, principal 4,500,000,000, principal and interest dues 0 | `552d58ff2918332665b179b37a70907c4492a02b5c56d7db05dce5e427e573e0` |
| Swap | Revision 2, remaining 6, closed, both reserves 0 | `fcd0b1621edab64096932b9a602ba3b55c55fb6299f6ff7e5d4e434c59331cfd` |

Zero-valued token entries remain in the native balance maps. Preserve those
entries when comparing full snapshots. Earlier stage summaries that omit zero
balances are not byte-identical full-state records.

The probe completed in 2,802 ms and closed both build loaders. Independent
terminal inspection recorded the node, indexer and proof server stopped and no
diagnostic process or cgroup at 53.819579623 seconds after timer arming. Only
then was the independent stop timer canceled. No transaction, proof, wallet
access or asset debit was performed.

The preceding attempt remains in `../local-finalized-state-01/`: it failed
before RPC readiness, retained no snapshots and established no method support.
Its exact transport cause was not retained. A subsequent real loopback TCP
fixture reproduced `UND_ERR_SOCKET`; the narrow repair admits that error only
during the existing bounded readiness period. Eight controlled tests and four
activation checks passed. The fixture does not establish the first failure's
exact cause.

Both service attempts and all review time remain consumed, along with the
historical ten submission reservations and 3,000,000,000,000,010 SPECK allocation.
No old reservation was reset or reused.

This establishes that the installed node supports the explicit finalized-block
state read used here. These are trusted-node observations, not authenticated
state proofs. No failed transaction was executed, so financial nonmutation,
SP05.2 completion, Preview settlement and mandatory PCD remain open.

Actual review receipts: `result-review-gpt6.json`, `result-review-grok.json`.
The original pending-review draft remains unchanged as `REVIEWED-RESULT.md`.

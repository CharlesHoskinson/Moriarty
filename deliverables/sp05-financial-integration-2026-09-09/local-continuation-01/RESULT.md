# Local continuation stopped at the public owner check

The admitted run reached fresh committee and indexer readiness, then failed with `INVALID_INDEXED_OWNER` while checking the initialized transaction. The launcher was never created and the new private base remained absent. No wallet reservation or transaction was created. Independent inspection found all three original Docker containers stopped with zero PIDs, 63.745 seconds after timer arming; the timer was canceled only after that observation.

The public failure record contains the code, not the failed HTTP payload. Inspection of the pinned indexer and SDK source reconstructs a Bech32m owner where the observer expected raw hex. The adjacent wallet decoder preserves that same encoding. The [source findings](../indexed-owner-01/source-findings.json) distinguish source facts and offline reconstruction from live evidence.

This attempt and its service cost remain consumed. The four prior reserved submissions and 1200000000000004 SPECK remain charged. The original wallet, roles, deployment, initialize bytes, recovered store and password remain in place. Any successor needs a separately reviewed bounded allocation. No complete continuation check, local financial settlement, Preview, SP05 or PCD acceptance is established by this result.

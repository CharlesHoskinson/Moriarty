# Local loan trace matched with actual settlement

The two admitted calls, accrue and settle, were submitted and observed in canonical finalized local blocks 20377 and 20381. Historical deploy and initialize checks also passed. All four financial comparisons match the independent fixed expectations. Settlement moves 533972602 native test-asset units from the borrower to the distinct lender, consuming the 20000000000-unit input and returning 19466027398 units as change. The remaining principal is 4500000000: this is the fixed partial-payment trace, not complete loan extinguishment.

| Call | Transaction ID | Native transaction hash | Block |
| --- | --- | --- | --- |
| accrue | `001ac8252beed46b047b6032710893182db342ce85e195403517738bf47e7248e7` | `473b363b745c4be4864e93a04d475a6f343ff7e56a8f179d86060716ba9a5480` | 20377 |
| settle | `00f40afa7fb4eb3e3daca7b5604ba783e7c1c214151766974848f40daedb474114` | `c201557431820f032743b0fc99c815b59131fa8bde6e298c466009189bce396e` | 20381 |

The unchanged inner driver reports `INCOMPLETE` because its cleanup does not establish outer process containment. Its financial comparator reports `PASS`; wallet shutdown completed with zero pending operations. Independent inspection found an empty/absent launcher cgroup and all three original Docker containers stopped, 167.497 seconds after arming, within the 1350-second allocation. The timer was canceled after that observation. The persisted reservation record remains `stopped:false`, `active:null`; actual process shutdown and the consumed one-shot allocation prevent treating that field as permission to reuse it.

The existing store, password, roles and signing identity passed the continuation gates. Original data, immutable pre-call snapshot, inspection copy and private journals remain outside the repository. Only closed public run records were copied. Two new reservations charge 600000000000002 SPECK and 20000000000 gross test-asset units. Together with retained history this is six reserved submissions and 1800000000000006 SPECK, plus all separately recorded services, failed attempts, builds, proofs, diagnostics and reviews. No refund is claimed.

Independent result reviews are pending. These are trusted local RPC/indexer observations and native-byte comparisons, explicitly uncertified I2. They do not establish swap settlement, Preview financial execution, general ACTUS/DeFi coverage, complete SP05 or PCD.

# Complete fixed local swap trace

Both independent reviewers passed the scoped [actual result](attempt-result.json); [review bindings](reviewed-result.json) preserve their exact responses and limitations. The existing contract completed all four financial comparisons: deployment, initialization, swap and close. Only swap and close were newly submitted.

| New operation | Transaction ID | Finalized block | Financial effect |
| --- | --- | --- | --- |
| Swap | `00059d928925b55197e7d6f08bc64d7a0ac11a16a206a81f55e1909908b623e046` | 20403 | Trader pays 10,000 A net and receives 19,743 B; 100,000 A input includes 90,000 A change. |
| Close | `001617e5090114284dfcf545855d834a4f85d2f7b5080f3c69d36489b32d6d6110` | 20407 | Provider receives 1,010,000 A and 1,980,257 B; both contract reserves become zero. |

The original initialize transaction is now reobserved canonical-finalized at block 20393 and its full financial comparison passes. Earlier failed comparison evidence is preserved. Seventeen closed public capture files retain native bytes, events, stages, reservations and the raw integration result; private plans, credentials, wallet/store data and SDK logs remain private.

The inner runner still reports INCOMPLETE and incomplete cleanup. Independent process/container inspection establishes outer containment at 110.811 seconds after timer arming; only then was the timer cancelled. These separate observations are retained without rewriting the raw diagnostics. Both new reservations are consumed: 600,000,000,000,002 SPECK and 100,000 A gross. Total historical reservations are ten and 3,000,000,000,000,010 SPECK, with every other earlier cost preserved separately.

This is a fixed local `undeployed` trace with one controller holding both capabilities and a distinct provider payout address. It does not establish Preview financial acceptance, independently funded counterparties, full SP05, general AMM conformance or mandatory PCD. No replay or retry is authorized by completion.

# Moriarty loan execution on Midnight Preview

The bounded Moriarty loan completed deployment, initialization, accrual and its
first-period payment on Midnight Preview on September 10, 2026. Independent
GPT-6 Astra and Grok 4.6 audits approve this specific financial result.

The lender received **533,972,602 USD_TEST_ASSET units**: 500,000,000 principal
and 33,972,602 interest. The asset is a minted Preview test token. Outstanding
notional remains **4,500,000,000**; this episode is not a complete loan payoff.

| Stage | Preview block | Observed transaction ID |
| --- | ---: | --- |
| Deploy | 807289 | `00b9a4c77a2609a963404882558462c8c26145ad0477d2ad0697ac76737a1193eb` |
| Initialize | 807293 | `006bf6ff0adeda2dff80937648be6060965b4018a19b95e70d124157c15d1f6bbc` |
| Accrue | 807297 | `008012ed90174c6ce28aab10c814032add101d6b5d1a2058d125c24c804f046365` |
| Settle | 807301 | `00cf2b8c133759457a2bb0f301c5cb427ae32948dbb4d4214b276b8dda380a453e` |

Contract: `323d43b96bca8dc7ddf7d0338c444116d1507b6f464d3fcd38022fc25f1a7ca8`.
Each transaction has two SDK identifiers. All eight are retained in the native
payload sidecars and both audit records, and were posted in the conversation.

[The scoped approval](scoped-approval-01.json) binds both independent audits:
[GPT-6](actual-result-review-gpt6-01.json) and
[Grok](actual-result-review-grok-01.json).
[The actual candidate](actual-result-candidate-01.json) binds the original
native transaction bytes, stage observations, limits and shutdown evidence.
The GPT-6 audit separately deserialized the transactions, verified the settlement
input signature and its initialization output, re-read full historical native
state, and matched every decoded field, balance and deployed verifier key to
the reviewed build. Those checks retain their
[own source and evidence manifest](gpt6-actual-verification-manifest-01.json).

## Scope and remaining gates

The original integration reports `FAILED` and its driver reports `INCOMPLETE`:
the in-process provider cannot attest external process containment. All four
financial comparisons passed. The launcher then exited with code 1. A separate
actual observation established that the process and cgroup were gone and all
three containers were stopped before the independent timer was cancelled.
These raw records and all original false acceptance flags remain unchanged.

The first launcher preflight stopped on a parent-directory permission mismatch
before attempt consumption. Both auditors approved a single mode correction
and renewed preflight; the failure and amendment remain in this directory.
One financial attempt consumed four submission reservations and
1,200,000,000,000,004 SPECK of native DUST debit. The indexer fee-unit
relationship remains unresolved; reservations are not asserted to be paid fees.

**Midnight Preview remains a hard gate.** This evidence establishes the bounded
loan slice under trusted node/indexer observations. Preview swap, complete SP05
financial coverage, mandatory PCD, SP09 verification, SP11 coverage and SP12
release remain open. Local tests or this loan result cannot close those gates.

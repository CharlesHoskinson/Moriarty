# Recovery snapshot sampling

Recovery02 failed its public finality gate before any private recovery or transaction. An unchanged actual-SDK/full-preflight fixture reproduces ordinary head advancement between indexed and finalized-head reads; the exact historical mismatch was not captured.

Candidate05 allows bounded public snapshot sampling after one historical watch. Only validated forward movement retries; malformed data, conflicting hashes, backward finality, changed native state and provider errors fail immediately. Every accepted sample must still have matching indexed/finalized observations and full native-state equality. All 296 ledger tests pass, with no skips. The unchanged actual-SDK fixture passes against final source.

Independent GPT-6 Astra and Grok4.6 high approve source05 and separately approve resource03. Both accept the retained recovery02 result as a failed contained attempt only. Grok completed in 253.937 seconds under a 900-second allowance without cancellation. See the exact candidates, substantive reviews, source snapshots, failed tests and final regression evidence here.

The original deployment, wallet/roles, private store, build and all previous charges remain preserved. One scoped local recovery03 attempt still needs exact admission and actual public/private/financial checks. Latest-state consistency trusts coherent local indexer/RPC observations; SDK state-query cancellation is not guaranteed. No live recovery, financial, Preview, full SP05 or PCD acceptance is inferred.

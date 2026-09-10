# Local loan recovery attempt 01

The admitted attempt stopped at the public preflight. It created no recovery private directory, launched no wallet, and submitted no new transaction. No financial stage completed. The attempt is consumed and cannot be retried under this allocation.

Committee readiness was observed 40.292 seconds after arming. The Node subprocess then failed; the runner kept only `CalledProcessError`. Its JavaScript catch had already discarded the original cause. We cannot identify the failing public predicate from that record.

All three containers were stopped and the launcher was absent when containment was independently observed at arming +105.491 seconds. The timer was canceled after that observation. `timer-terminal-observation.json` records its later inactive state, not an invented cancellation timestamp.

The exact plan expression validates offline. A fixture using the actual pinned indexer SDK and retained deployment bytes passes public verification against synthetic compatible responses, and a synthetic schema rejection produces `IndexerQueryError`. These checks narrow the investigation; they do not prove the real indexer schema, chain state, or actual failure cause.

There are zero new submission or DUST reservations. The prior three local attempts retain three submission reservations and 900000000000003 SPECK in DUST reservations; all other historical charges remain separate. These reservations are not measured fees paid.

The existing deployment remains the transaction recorded in `../local-execution-04/attempt-result.json`, with indexed SUCCESS at block 20313. This attempt adds no independent canonical-finality or current-state confirmation. Recovery, the three loan calls, swap settlement and Midnight Preview financial acceptance remain open.

The next decisive check is a separately admitted, short read-only node/indexer run that preserves the public error. It requires no proof server, wallet, private-state recovery or transaction submission.

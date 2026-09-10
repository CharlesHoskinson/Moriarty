# Source-only verification

Working directory: `/home/charl/Moriarty/.worktrees/sp05-deadline-review`.

- `node --test --test-reporter=tap experiments/moriarty-midnight-financial/ledger/initialized-swap-{driver,public,store}.test.mjs` — RED 0/20 before patch, then 20/20.
- `node --test --test-reporter=tap experiments/moriarty-midnight-financial/ledger/initialized-swap-integration.test.mjs` — composed and diagnostic rounds retained individually.
- `node --test --test-reporter=tap experiments/moriarty-midnight-financial/ledger/initialized-swap-{driver,public,store,integration}.test.mjs experiments/moriarty-midnight-financial/ledger/{run-local,integrate-local,recover-deployment,recover-store,launch-local,continue-loan-plan}.test.mjs` — 269/269, 48.778 seconds.
- `node --test --test-reporter=tap experiments/moriarty-midnight-financial/ledger/initialized-swap-{driver,public,store,integration}.test.mjs` — final focused 49/49, 2.307 seconds, including three additional snapshot/history-wallet tests.
- `git diff --check` — PASS.

The public gate tests use actual retained swap deployment/initialize native bytes and full historical state bytes, actual pinned ledger and compact-runtime state decoders, and the production finalized observer. RPC/indexer transport, current head and decoded financial summary are controlled fixtures; these tests do not establish live finality or execute proof verification. Store tests create synthetic encrypted SDK v2 scratch entries and preserve their bytes; they never inspect the original store. Integration tests exercise production integration and driver with synthetic comparator/provider/transport adapters, a real native initialize receipt output and canonical wallet owner, and durable-stage callback assertions. They establish ordering, not actual filesystem durability or a new live financial comparison. Existing standalone actual SDK wallet wire and real-comparator negative regressions remain separately applicable.

Implementation retains fixed original swap identity, build, source result and store; replays deploy and initialize comparisons once before new swap/close dispatch. No deploy, initialize, constructor, restore, key generation or resource admission is added. New resource/source reviews remain required before operational use.

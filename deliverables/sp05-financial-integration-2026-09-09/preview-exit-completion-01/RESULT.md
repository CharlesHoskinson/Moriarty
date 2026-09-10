# Preview command completion

The CLI can now report `FINANCIAL_COMPLETE` with exit zero after the supported financial trace completes, wallet persistence and stop succeed, public output is retained, no work remains pending, and the deadline still holds. It keeps `containmentComplete: false`: external process and prover containment must still be observed separately. All acceptance flags stay false.

Both independent GPT-6 Astra and Grok 4.6 high reviews approve the [frozen source candidate](source-candidate-01.json). The [approval record](scoped-source-approval-01.json) binds their exact receipts. The reviewer reran 89 Preview and 150 local/recovery tests with no failures or skips. Controls reject over-cap fees, wrong recipients, malformed/missing transaction evidence, pending work, persistence/stop failures, output collisions and late deadlines. Local/recovery behavior remains unchanged.

These checks exercise controlled dependencies and the actual CLI exit selector. They do not observe a new Preview invocation or process exit. An admitted real run remains required; historical loan/swap `FAILED`/`INCOMPLETE` results and exit-one observations are unchanged. Neither source approval nor a zero exit code establishes mandatory proof/history acceptance or full SP05 completion.

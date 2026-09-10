# Preview command completion

The original driver threw DRIVER_CLEANUP_INCOMPLETE after four successful Preview stages when wallet stop succeeded and no operation remained, because external containment was unobserved. `red.tap` reproduces this exact case (17 pass, one fail). The original four source files and RED test are retained. Historical Preview FAILED/INCOMPLETE records remain unchanged.

Preview now distinguishes FINANCIAL_COMPLETE from external containment. It requires completed stages and successful in-process wallet cleanup with zero pending operations. FINANCIAL_COMPLETE keeps containmentComplete=false; PASS retains containmentComplete=true. Local and recovery driver behavior is unchanged. Source adapters continue to return SOURCE_TEST_ONLY, which the CLI success selector rejects.

The integration still rejects every driver/comparison/setup/deadline/loader-cleanup failure. The durable writer and CLI completion validator require matching four ordered receipts and comparison summaries, full retained transaction identifier membership, no failure record, present finality fields, a known four-submission operational record, successful wallet cleanup, and no pending setup work. Native decoder proofVerified/ledgerAccepted and every acceptance flag remain false. These checks preserve the observer/comparator trust boundary; they do not authenticate public RPC or replay proofs.

The bootstrap additionally requires that its own persistence-and-stop closure completed. The actual three-child persistence implementation, durable pending marker and private backup rules are unchanged. Failed/missing persistence or stop cannot reach the result writer or success selector. The original absolute deadline is checked after final durable result retention and again by the CLI selector. A durable financial-completion record alone is not evidence of exit zero: a later deadline/output failure still gives nonzero and remains separately retained. CLI stdout explicitly includes containmentComplete.

`preview-green.tap`: 89 standard Preview tests pass. `local-preserved.tap`: 150 local/recovery/stale controls pass. `connected-03.tap`: 29 focused controls pass before final strengthening. New tests use Node's controlled module interception for the production import branch, the real stage driver/native observer/comparator and retained public Preview loan bytes/effects, then the real writer and actual CLI exit selector. SDK actions/transport are inert; capability commitments alone are rebound to synthetic secrets. Tests cover cumulative excess native fee, wrong recipient, pending/unknown work, missing/failing persistence and stop, incomplete/invalid receipts/IDs, source-only results, durable output collision, and final-directory-fsync deadline crossing.

The positive check establishes selector mapping, not an observed subprocess or new Preview command exit. No wallet/private data, real service, network, prover, compiler, transaction or new financial allocation was used. A future independently admitted actual Preview invocation must still establish the exit-zero requirement. No new orchestration or resource proposal is included.

Earlier logs are preserved: green01 had one obsolete test expecting the original external-containment exception; connected01 found the controlled provider fixture retained only primary IDs instead of the actual full native identifier set; the fixture was corrected to match production. connected02 passed six controls.

The accepted fee comparison source was merged from main b35445d before final checks. To preserve dirty work during that merge, a local unreviewed WIP checkpoint a28b09e was created with parent authorization; it was not published. This candidate remains awaiting independent review.

Commands from the worktree root:

```sh
npm --prefix experiments/moriarty-midnight-financial run test:preview
node --experimental-test-module-mocks --test --test-reporter=tap experiments/moriarty-midnight-financial/ledger/run-local.test.mjs experiments/moriarty-midnight-financial/ledger/integrate-local.test.mjs experiments/moriarty-midnight-financial/ledger/initialized-swap-integration.test.mjs experiments/moriarty-midnight-financial/ledger/stale-loan-integration.test.mjs
git diff --check
```

# Corrected default submission-service boundary

Original candidate01 is preserved, including BLOCKED GPT6 finding and original Grok review scope. Its changed source files resolve by exact original hashes through original-source-preservation.json and original-candidate-01/. Old README and test logs are not overwritten.

The original classifier handled Effect(Fail(node-client.TransactionInvalidError)) but the default wallet service actually produces Effect(Fail(capabilities.SubmissionError(cause=node-client.TransactionInvalidError))). The narrow correction imports the installed capabilities class and unwraps exactly one such wrapper before the existing strict node class/transaction byte tests. Generic transport errors and numeric RPC1010 remain UNKNOWN. No arbitrary nested-cause search or error text is accepted.

The new stale-loan-submission-service.test.mjs runs actual makeDefaultSubmissionService and actual PolkadotNodeClient.sendMidnightTransaction/stream/status callback against a controlled API. It intercepts only the static client-construction boundary, so no WebSocket or HTTP is opened. The real class handles status.isInvalid, the real default service maps its error, and actual Effect.runPromise wraps it. Exact retained public native transaction bytes are serialized through the installed API; this is source-only transport evidence, not a submitted replay. Separate controls cover transport failure and actual RpcError1010, wrong bytes and wrong phase. The actual connected original failure is retained in default-service-red-02.tap.

Final command:

`MORIARTY_LOAN_BUILD_RECEIPT=/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/loan-output/build/build-receipt.json node --experimental-test-module-mocks --test experiments/moriarty-midnight-financial/ledger/stale-loan-rejection.test.mjs experiments/moriarty-midnight-financial/ledger/stale-loan-submission-service.test.mjs experiments/moriarty-midnight-financial/ledger/provider-positive-fee.test.mjs`

12 tests PASS,0fail,0skip (green-corrected-final-02.tap). This candidate contains only the original helper/tests/fee control and the narrow wrapper correction/new connected service test. Concurrent unreviewed operational plan files are excluded. No service/private/proof/compile/submission activity or adverse admission occurred. All operational integration and original scope limits remain open as documented in README.md.

The attempted flag is now `providerSubmissionAttempted` and is set inside the actual submit thunk. It means entry to the guarded provider, never network delivery or confirmation. This prevents a pre-thunk deadline from reporting an attempted provider submission.

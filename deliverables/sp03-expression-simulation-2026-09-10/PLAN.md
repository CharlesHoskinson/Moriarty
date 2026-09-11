# SP03.2 scoped expression simulation plan

## Capability

Developers can run the local, bounded command below for either accepted source
profile and receive the selected source API's successful local evaluation result:

```sh
node experiments/moriarty-language/src/cli.ts simulate --profile PROFILE --schema SCHEMA --snapshots SNAPSHOTS SOURCE.mori
```

This is a local evaluator adapter. It does not compile or run K, create a
proof, contact a service, execute a wallet action, or establish financial or
ledger acceptance.

## Design and order

1. Keep `src/cli.ts` as the sole public adapter. Extend its exact argv parser
   with `simulate --profile PROFILE --schema SCHEMA --snapshots SNAPSHOTS SOURCE.mori`.
   Validate argv and profile before opening a path, then read schema, source,
   and snapshots in that exact order.
2. Reuse the existing opened-descriptor, nonblocking, bounded, fatal-UTF-8
   reader. Its schema and source behavior stays byte-for-byte. Add snapshots
   with the API's 2,000,000-byte limit and `INPUT_BOUND` at 2,000,001 bytes;
   retain BOM bytes for canonical parsing.
3. Select `createExpressionSourceV1` or `createFinancialExpressionSourceV1`
   from the explicit profile. Call only `evaluate(source, snapshotText)`; do
   not accept a Core, pre-check the source, or alter source/evaluator semantics.
4. Treat only `result.status === 'Rejected'` as a source/API rejection. On
   success, first form the documented wrapper using parsed canonical snapshots
   after `evaluate` has succeeded. This avoids leaking snapshot input on every
   rejection and preserves the API's `ExpressionPrepared` success status.
5. Add small subprocess tests first: independent positive financial expectation,
   rejected option, exhausted work, rollback, original-40 fixture, canonical
   snapshot transport failures and current CLI regressions. Then update the
   command contract and README.

## Output contract

Success writes exactly one JSON line to stdout and no stderr, exits 0:

```json
{"judgmentResult":"SourceSimulated","sourceProfile":"PROFILE","pre":"validatedSnapshot.Pre","initialWork":"validatedSnapshot.workInitial","result":"exact successful API result"}
```

The `pre`, `initialWork`, and `result` entries hold values, not the quoted
descriptions shown above. Language rejection writes its exact `Rejected`
record to stderr, stdout remains empty, and exits 1. CLI transport failures use
the current `CliRejected` shape, including `input: "snapshots"` where
applicable. No tentative post-state or descriptors are emitted after rejection.

## Verification sequence

1. Record baseline build and full test output.
2. Add a failing public-CLI simulation test and record its expected failure.
3. Implement the smallest adapter change, rerun the focused test for green,
   then add and verify the rejection and transport cases.
4. Run package build, full package tests, the financial and original-40 demos,
   and save concise command/output records. Do not run a K compile or K trace.

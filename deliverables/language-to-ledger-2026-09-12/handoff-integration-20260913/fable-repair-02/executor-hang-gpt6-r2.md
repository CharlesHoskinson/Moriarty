# Executor hang diagnosis

The reported original timeout was not reproduced on the inspected bytes. The timeout and parent Node processes were gone; PID 2615611 was a zombie, so its former handles could not be examined.

Five selected real-socket tests passed (exit 0; 429.725036 ms): configured handoff, immediate child connection, late directory, late socket, and nonce failure. Command:

```sh
node --test --test-name-pattern='configured handoff|immediate child connection|H3b|H3a' ledger/executor-caller.test.mjs
```

An open socket/server or pending socket-close callback remains a plausible mechanism, but no exact cause or corrective source patch is established. The `handoffHandles` / `afterEach` client teardown already exists in base; it is not evidence of a new fix. Current executor cleanup aborts the protocol, detaches listeners, destroys tracked sockets, and bounds server close.

Do not patch from this hypothesis. Freeze Fable's final candidate and run its exact focused tests with output capture. If the hang recurs, inspect active resource/handle types and socket ownership before timeout. Test teardown must destroy accepted sockets before awaiting server close.

All four inspected hashes were unchanged across this diagnosis and are recorded in the adjacent JSON. Original loaded hashes are unavailable. No source edits, process kills, real accounting changes, or service changes occurred. This is bounded diagnosis, not candidate or release approval.

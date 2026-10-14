# FOREMAN_REPORT

- run_id: moriarty-ll-loop-20260912
- role: implement
- slug: handoff-protocol
- branch: foreman/moriarty-ll-loop-20260912/implement/handoff-protocol
- worktree: /home/charl/Moriarty-wt-moriarty-ll-loop-20260912-implement-handoff-protocol
- base_sha: 056e662254c38936758af9c6f72c019705a1ff09
- status: complete

## Summary

This correction fixes F1 and F2. It strengthens the F3 test without changing the correct production order.

The executor test count increased from 83 to 86. All required suites report zero failures.

## Findings

### F1: measured handoff hang

The executor now installs a paused connection queue before it spawns the child.
The queue retains connections that arrive during the durable invocation append.
After the append, the executor replaces the queue with the handshake handler in one synchronous turn.
It then resumes and dispatches all queued sockets.

Cleanup tracks all accepted sockets and destroys them before server closure.
The server close wait now has a 250 ms upper bound.
The fallback calls `closeAllConnections()` when that Node API is available.

Two new tests prove the repair.

- A 60 ms append with an immediate connection completes the handshake and returns.
- A server whose `close()` callback never runs still returns at the bounded fallback.

The immediate-connect test failed before the repair with `immediate handoff did not return`.
It passed after the repair in the focused test and in `test:executor`.

### F2: boot-mismatch denial

The executor now gives the handshake server the persisted boot ID and a fresh boot-ID reader.
The server calls `verifyBootId()` before peer credentials, ancestry, and state confirmation.
A mismatch produces `HANDOFF_BOOT_ID_MISMATCH`.
The common denial producer durably records this code before it sends denial bytes.

The new test checks the result, wire code, durable denial record, and absence of later state work.
A mutation that ignored the false comparator result changed the outcome to `HANDSHAKE_OK`.
The new test failed on that mutation.

### F3: record-before-answer test

Production already awaited the durable denial record before it wrote denial bytes.
The replay test now uses an unresolved recorder gate controlled by the test.
The test proves that no answer arrives while that gate remains unresolved.

A write-before-record mutation made `answeredEarly` equal `true`.
The new assertion failed with `true !== false`.
The correct production line is restored.

## Denial-case scope

| Enumerated case | Status | Module behavior |
| --- | --- | --- |
| Forged nonce | Enforced | The module performs the fixed-length constant-time comparison. |
| Authority mismatch | Enforced | The module compares the received authority digest. |
| Boot mismatch | Enforced | The module performs a fresh boot-ID comparison and creates a distinct denial. |
| Launcher identity | Enforced | The module verifies PID ancestry and launcher start ticks. |
| Replay | Enforced | The module permits one consumed handshake and denies later connections. |
| Reservation mismatch | Pass-through | `confirmCurrentState()` must detect the predicate and throw its denial code. |
| Stale store event | Pass-through | `confirmCurrentState()` must detect the predicate and throw its denial code. |

The module enforces denial ordering and durable recording for both pass-through cases.
It does not enforce either pass-through predicate.
Peer UID mismatch is also enforced, but it is not one of the seven enumerated cases.

## F1 probe transcripts

Before the repair:

```text
BEFORE slowAppendMs=60 immediateConnect=true outcomeAfter8000ms=pending elapsedMs=8008
Warning: Detected unsettled top-level await at file:///home/charl/Moriarty-wt-moriarty-ll-loop-20260912-implement-handoff-protocol/experiments/moriarty-midnight-financial/[eval1]:51
const result=await execution;
             ^
```

The process exited with status 13 because the awaited executor result never settled.

After the repair:

```text
AFTER slowAppendMs=60 immediateConnect=true handshakeComplete=true returned=true elapsedMs=70 rawExit=exit:0 disposition=PROCESS_UNKNOWN
```

The probe only exercises handoff and return behavior.
The existing fault matrix causes `PROCESS_UNKNOWN` because the synthetic probe has no terminal-evidence persistence.

## Mutation evidence

F1 pre-repair regression:

```text
# Subtest: immediate child connection survives a slow invocation append and the executor returns
not ok 1 - immediate child connection survives a slow invocation append and the executor returns
error: 'immediate handoff did not return'
# tests 1
# pass 0
# fail 1
```

F2 ignored-comparator mutation:

```text
# Subtest: boot mismatch records and returns its distinct denial before state confirmation
not ok 1 - boot mismatch records and returns its distinct denial before state confirmation
+   status: 'HANDSHAKE_OK'
-   code: 'HANDOFF_BOOT_ID_MISMATCH',
-   status: 'HANDSHAKE_DENIED'
# tests 1
# pass 0
# fail 1
```

F3 write-before-record mutation:

```text
# Subtest: successful handshake is one-shot and a replay never answers before its denial record completes
not ok 1 - successful handshake is one-shot and a replay never answers before its denial record completes
Expected values to be strictly equal:
true !== false
# tests 1
# pass 0
# fail 1
```

## Verification

### `experiments/moriarty-language && npm test`

```text
ℹ tests 889
ℹ suites 14
ℹ pass 889
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 20489.691835
```

### `experiments/moriarty-language && npm run typecheck`

```text
> typecheck
> tsc -p tsconfig.json
```

### `experiments/moriarty-midnight-financial && npm test`

```text
1..52
# tests 52
# suites 0
# pass 52
# fail 0
# cancelled 0
# skipped 0
# todo 0
# duration_ms 188.963457
```

### `npm run test:ledger`

```text
1..610
# tests 610
# suites 0
# pass 610
# fail 0
# cancelled 0
# skipped 0
# todo 0
# duration_ms 60879.408896
```

### `npm run test:compiled`

```text
1..3
# tests 3
# suites 0
# pass 3
# fail 0
# cancelled 0
# skipped 0
# todo 0
# duration_ms 1565.893411
```

### `npm run test:compiled-rejection`

```text
1..11
# tests 11
# suites 0
# pass 11
# fail 0
# cancelled 0
# skipped 0
# todo 0
# duration_ms 1991.704362
```

### `npm run test:finalized-state`

```text
1..27
# tests 27
# suites 0
# pass 27
# fail 0
# cancelled 0
# skipped 0
# todo 0
# duration_ms 1434.870735
```

### `npm run test:preview`

```text
1..89
# tests 89
# suites 0
# pass 89
# fail 0
# cancelled 0
# skipped 0
# todo 0
# duration_ms 1764.388804
```

### `npm run test:executor`

```text
1..86
# tests 86
# suites 0
# pass 86
# fail 0
# cancelled 0
# skipped 0
# todo 0
# duration_ms 653.769046
```

## Protected files and dependencies

```text
abc1efa041a9fb6c3936ffcf6708eba24ff6d78a8123b868624ff1696de11c1b  ledger/fault-matrix.mjs
a046a77a17cd38ab4d0a97e3df601add97ebde8443e3303d1dfc1a9bb8967cdb  ledger/prover-lifetime.mjs
```

These hashes equal the pre-change hashes.
All npm dependency maps remain empty.

## Open questions

None.

No work item was blocked.
This round did not use Git, Docker, the network, accounting data, budget data, charge data, or plugin Python files.

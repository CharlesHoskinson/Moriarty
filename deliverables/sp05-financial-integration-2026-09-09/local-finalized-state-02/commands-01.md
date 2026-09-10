# Proposed local finalized-state probe

Draft only: no admission, runtime service or node request has occurred. Source/resource reviews must bind the final candidate and proposal before dispatch.

After admission, the single command is:

```sh
python3 deliverables/sp05-financial-integration-2026-09-09/local-finalized-state-02/execute-once.py
```

Run from `/home/charl/Moriarty/.worktrees/sp05-deadline-review`. Review Python with AST parsing; never import or execute it for syntax checking. The runner rejects optimized Python explicitly, then validates exact source/file hashes, two normalized source verdicts and two resource verdicts from actual GPT6 Astra/Grok4.6 records. Normalized records reference admission-bound actual raw reviews. No fabricated vote, reusable old allocation or unbound candidate permits start.

The runner requires its units, attempt and result absent,12GiBavailable host memory and4GiBfree disk. It inspects all three retained containers as exited and checks their image hashes. Only the existing node starts, with5GiBmemory/10GiBtotal memory-plus-swap/4CPU/restart-no and its writable layer preserved. Indexer/proof server remain stopped; neither appears in a start/stop/kill argv. The one-use attempt record is exclusively written/fsynced before timer or container activation.

The independent `moriarty-sp05-finalized-state-stop-02` timer fires at arming+180seconds. Its exact argv uses a5second cgroup kill,20second Docker stop and5second ExecStopPost Docker kill for the node only, plus at most1second timer accuracy: planned terminal bound211seconds. Actual D-Bus expiry and accuracy are checked before node start. Docker start and acknowledged diagnostic activation must finish within remaining arming+90seconds; late activation aborts. The diagnostic has4GiBmemory, no swap,200%CPU,90second runtime and group-wide forced termination. Its own observation deadline is89seconds including readiness, both loaders and all requests. An independent terminal check must establish node exited, diagnostic cgroup absent and the other two containers still stopped before canceling the timer. Timer cancellation alone is not containment evidence.

`probe.mjs` uses only localhost19944 and the corrected existing RPC transport. Readiness retries only fetch-failed ECONNREFUSED or UND_ERR_SOCKET, bounded by60samples/60seconds; all RPC requests have5second or remaining deadlines with post-completion checks. After readiness it verifies the exact retained genesis, loads the original full loan/swap public build artifacts under their exact receipt/source hashes, and reads both contract states at each capture's explicit finalized anchor. Anchors may differ as the chain advances; no simultaneous two-contract snapshot is claimed. No wallet, original roles, seed, password or contract-private store is opened.

The public output is exclusively retained/fsynced as `probe-result.json`, at most64KiB. It includes full native state hex/hash, all decoded public fields, balances, anchor identities and bounded public errors. Bigints become decimal strings; decoded byte arrays become `{hex:...}`. Native serialized hex retains the complete original representation. A single method/state failure stops without trying the other contract or a fallback; any completed first snapshot remains scoped evidence. `RPC_RESPONSE` means method support UNKNOWN because the transport does not retain the actual error envelope. Successful snapshots establish trusted-node observations, not an authenticated state proof, on-chain rejection, nonmutation, Preview or full SP05 acceptance.

Source checks (no live fetch; actual RPC/native/generated boundary uses controlled transport):

```sh
node --test --test-reporter=tap deliverables/sp05-financial-integration-2026-09-09/local-finalized-state-02/probe.test.mjs
node --check deliverables/sp05-financial-integration-2026-09-09/local-finalized-state-02/probe.mjs
```

Preserve the existing10reservations/3000000000000010SPECK and all historical/runtime/review charges. This new probe authorizes zero transaction/proof attempts and zero DUST or financial debit. Preserve every partial result, error and cleanup observation; no automatic retry.

The manager receives `TimeoutStartSec` from the remaining activation allowance (rounded down, with one second reserved for durable recording and submission). `resolved-launch.json` retains the exact dispatched argv before launch. If recording consumes that reserve, submission fails closed; the independent stop timer remains armed. Original attempt files remain unchanged in sibling `local-finalized-state-01/`.

Attempt 02 requires fresh GPT6 and Grok source/resource approvals. Attempt 01 remains consumed: service 54.56324710899207 seconds and Grok review 278.039 seconds, plus every prior charge (10 reservations / 3000000000000010 SPECK). No budget resets. The original missing socket cause cannot be recovered from timing evidence. The isolated Node v24.18.1 TCP-close reproduction observed UND_ERR_SOCKET; only that code and ECONNREFUSED may be polled before readiness. All later operation errors stop. Closed causeCode is retained, with UNKNOWN for anything outside the finite set. Eight connected tests pass; red evidence is preserved. No operational attempt has run.

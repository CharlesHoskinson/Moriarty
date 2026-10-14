# FOREMAN_REPORT

- run_id: moriarty-orch-remediation-20260912
- role: implement
- slug: runner-round-ownership
- branch: foreman/moriarty-orch-remediation-20260912/implement/runner-round-ownership
- worktree: /home/charl/Moriarty-wt-moriarty-orch-remediation-20260912-implement-runner-round-ownership
- base_sha: 056e662254c38936758af9c6f72c019705a1ff09
- status: complete

## Summary

`execute()` now returns a receipt when a child will not die or will not reap. The function does not raise `subprocess.TimeoutExpired`. An unreapable first wait stays `completionAmbiguous` and records the second-bound exit if that wait reaps. A child that survives both SIGKILL waits is recorded as abandoned with the observed pid and pgid.

## Findings

### Defect 1. Exception loses the receipt

After the read loop, `proc.wait(timeout=5)` could raise `TimeoutExpired`. That exception left `execute()` with no return value. The CLI stored `receipt: null` on `runner_unresolved`.

The fix catches that wait in `_reap()`. A missed first bound does not escape `execute()`. `exitCode` is `None` until a wait observes a status. Zero is not used as a stand-in.

### Defect 2. No second bound

The timeout branch issued one `os.killpg(proc.pid, SIGKILL)` and broke. There was no second kill and no abandonment record.

The fix issues a second SIGKILL after a missed first reap, then waits again for 5 seconds. If that second bound also misses, the receipt sets `abandoned` to true and records `cleanupOwner`.

### New receipt fields

Existing keys stay in place. These fields are added on every receipt:

| Field | Meaning |
|---|---|
| `verified` | True only when a behavioural assertion is observed. False on both new paths. |
| `abandoned` | True only when both reap bounds miss. |
| `cleanupOwner` | `{"pid": ..., "pgid": ...}` when abandoned. `None` otherwise. Unobserved `pgid` is `None`, not 0. |

`completionAmbiguous` is true on both new paths. `behavioralAssertions` is empty on both new paths.

### Tests

Both new tests inject `subprocess.Popen`, `os.killpg`, and `time.monotonic`. They do not use real timing races.

- `test_unreapable_wait_keeps_reservation_unresolved` drives a first-wait timeout then a second-wait exit `-9`. Reservation status stays `active`. CLI exit is 4. `abandoned` is false. Two SIGKILL calls occur.
- `test_abandoned_child_keeps_reservation_unresolved` drives two wait timeouts. Reservation status stays `active`. CLI exit is 4. `abandoned` is true. `exitCode` is `None`. `cleanupOwner.pid` is 4242.

The tests follow the unresolved-reservation shape of `test_supervisor_death_keeps_reservation_unresolved`, `test_launcher_error_keeps_reservation_unresolved`, and `test_encoded_signal_keeps_reservation_unresolved`.

## Evidence

### Digest pin

Stated prior pin:

`0b40d8c3e184f246294188d89107f70bb2b38789de2276fa374792b955b4acb3`

This worktree `hooks/hooks.json` did not contain that pin before this change. It called `hook.py` directly. The new pin is computed from this worktree `scripts/moriarty_dev` `.py` files after the `runner.py` edit:

`b29c951c99f07cca677d986affc56c72bd5114bee12ffb16ca0930ab0b620641`

That value is in all four hook commands. Recomputation matches the written pin.

```text
computed b29c951c99f07cca677d986affc56c72bd5114bee12ffb16ca0930ab0b620641
```

Grep of the four occurrences (lines 10, 22, 34, 46):

```text
10: ... ' b29c951c99f07cca677d986affc56c72bd5114bee12ffb16ca0930ab0b620641 SessionStart; ...
22: ... ' b29c951c99f07cca677d986affc56c72bd5114bee12ffb16ca0930ab0b620641 PreToolUse; ...
34: ... ' b29c951c99f07cca677d986affc56c72bd5114bee12ffb16ca0930ab0b620641 PostToolUse; ...
46: ... ' b29c951c99f07cca677d986affc56c72bd5114bee12ffb16ca0930ab0b620641 Stop; ...
```

Count: 4.

Runtime store created:

`/home/charl/.local/share/moriarty-dev/runtimes/b29c951c99f07cca677d986affc56c72bd5114bee12ffb16ca0930ab0b620641/`

The store digest matches the pin. The store contains the updated `scripts/moriarty_dev` tree.

### Pytest

Command: `cd plugins/moriarty-dev && python3 -m pytest tests/ -q`

Verbatim output:

```text
............................................................ [ 29%]
................................................... [ 55%]
........................................................................ [ 91%]
..................                                                       [100%]
201 passed, 105 subtests passed in 13.73s
```

Zero failures.

## Open questions

This worktree collected 201 tests after the two new tests. The task stated 219 existing tests. This plugin tree has no `test_upgrade.py` and no `bootstrap.py` or `deployment.py` under `scripts/moriarty_dev`. Those files exist in the live `0b40d8c3...` runtime store. I did not copy them into this worktree.

The pre-change digest of this worktree script set, with the old `runner.py` bytes, is `afead559ef39309d470152e6bdee432f0222203e9a4c094e2daefe4036d891be`. It is not `0b40d8c3...` because this tree does not include `bootstrap.py` and `deployment.py`.

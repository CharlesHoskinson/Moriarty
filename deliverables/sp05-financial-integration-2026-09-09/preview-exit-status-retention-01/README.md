# Retain the actual transient-unit exit result

The consumed preview-loan-exit-01 run has FINANCIAL_COMPLETE stdout and all four financial comparisons, but its retained systemd properties say LoadState=not-found and ExecMainCode=0. These are unloaded-unit defaults, not an observed successful main-process exit. Its actual exit remains unknown. No historical result is rewritten.

Installed systemd259 primary manuals explain the failure: systemd-run.1 says successful transient units unload immediately; systemd.unit.5 says unloading discards execution results, including exit codes and signals. systemd.service.5 says RemainAfterExit=yes keeps a service active after its processes exit and applies to every service type. Exact installed document hashes and roff excerpts are retained here. No public web research or manager mutation occurred.

The minimal future executor patch adds RemainAfterExit=yes only to the Type=exec financial launcher, and retains InvocationID in the acknowledged launch observation. It leaves the original stop timer argv, operation/setup/activation limits, financial allowances, Docker limits and language source unchanged. This patch is a reviewed-template delta, not an executable successor allocation: it must only be applied to a separately admitted fresh packet.

Capture the loaded unit before stopping it. Require the expected unique invocation, Type=exec, Transient=yes, RemainAfterExit=yes, active/exited, MainPID=0, Result=success, ExecMainCode=1 and ExecMainStatus=0. Retain the raw selected public systemd fields durably; then stop the launcher explicitly and independently observe its cgroup absent and containers stopped. Missing, failed, signaled, running or unloaded observations cannot establish exit0. Never reconstruct an exit from stdout or financial results.

RemainAfterExit delays ExecStopPost until stop. Therefore successful financial exit is not permission to leave the prover running: explicit stop follows durable main-exit capture, while the original independent forced timer stays armed. The timer is canceled only after independently retained containment. No assumption is made that the exited unit remains queryable beyond runtime-limit/stop/manager events; failure to capture before that point stays unknown.

Pure checks pass: the actual unloaded observation is rejected, retained-success fields pass, missing/nonzero/signaled/wrong-invocation fields fail, and AST equality confirms the original stop timer is unchanged. These are predicate/source checks, not an observed systemd retention test.

A bounded smoke command is prepared but NOT RUN:

```sh
python3 deliverables/sp05-financial-integration-2026-09-09/preview-exit-status-retention-01/smoke-once-proposed.py --run-once
```

It starts only one uniquely named /usr/bin/true user unit (64MiB, no swap,10%CPU,5s runtime cap,2s activation,1s stop), reads selected properties once, then explicitly stops it. Calls share a10s decision deadline plus at most3s final stop; failed/unknown evidence consumes the smoke attempt. It does not touch Docker, wallet/private state, SDK, proof services, compiler, financial allocations or any network endpoint. Current authorization is source-only, so an independently approved bounded smoke decision is required before invocation. A single early read may honestly miss exit and fail; no retry loop is included.

MC02 tasks' two-public-attempt cap is not reset: original Preview loan01 and loan-exit01 consume both loan attempts. No third loan is proposed or admitted here. Any third loan needs an explicit narrow reviewed amendment retaining the third-attempt count and every charge. Swap has one actual attempt; a corrected second swap remains a separate source/resource decision. The language source stays candidate7893d048… unchanged.

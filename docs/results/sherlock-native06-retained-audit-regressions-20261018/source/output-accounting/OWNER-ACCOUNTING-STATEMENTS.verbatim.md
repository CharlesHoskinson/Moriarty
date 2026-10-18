#### 1.1 Reconciliation of Peer R399 Report vs. Authoritative Root Cgroup Readback & Receipt Counters

1. **Recorded vs. Unrecorded Native 06 Resource Facts**:
   - **Directly Measured in `packet/` Receipts & Logs**:
     - Prior phase wall times (`A0_RESULT.json`..`E_RESULT.json`):
       $$2.6567903329996625 + 2.78098472900092 + 1.7633520050003426 + 12.067841101998056 + 7.613954207998177 + 12.315458076998766 = 39.1983804539959241\text{ s}.$$
     - Outer wrapper elapsed time (`RESULT.json`, `3732f46e...`): `157.16370775500036` s.
     - Prior Phase F log bytes: `F_precheck.stdout = 0` B, `F_precheck.stderr = 0` B, `F.stdout = 0` B, `F.stderr = 10,237` B, `F_audit.stdout = 0` B, `F_audit.stderr = 47` B (cumulative `stdout = 0` B, `stderr = 10,284` B against `16,777,216` B caps).
   - **Directly Measured in Kernel Cgroup v2 Readback (`ROOT-CGROUP-READBACK-20261018.json`, sampled `2026-10-18T10:34:24.614823+00:00`)**:
     - Envelope (`/sys/fs/cgroup/moriarty-sherlock-ledger9-20261018-06/cpu.stat`): `usage_usec = 289,006,953` (`user_usec = 235,579,044`, `system_usec = 53,427,909`), `cgroup.procs` empty, `cgroup.events` `populated = 0`.
     - Supervisor leaf (`.../supervisor/cpu.stat`): `usage_usec = 48,374,910` (`48.374910` CPU-s; `ceil = 49` s), `cgroup.procs` empty.
     - Phase parent (`.../phases/cpu.stat`): `usage_usec = 240,632,042` (`240.632042` CPU-s aggregate across all phase leaves `sdk-once-A0`..`sdk-once-F`; `48,374,910 + 240,632,042 = 289,006,952` usec), `cgroup.procs` empty.
   - **Unrecorded Quantities & Residual Scope**:
     - Because Native 06 refused inside `F_audit` (`non-normalized dep-info path`), `F_RESULT.json` was not written and `Group.close()` removed `/sys/fs/cgroup/.../phases/sdk-once-F`. Thus, Phase F-only `wall_seconds` and Phase F-only `usage_usec` were not separately recorded.
     - The difference `157.16370775500036 - 39.1983804539959241 = 117.9653273010044359` s (`ceil = 118` s) is a **residual upper bound** covering Phase F execution plus supervisor startup/inter-phase/teardown overhead, not a directly measured Phase F duration.
     - Similarly, `phases/cpu.stat` (`240,632,042` usec) is the **measured aggregate CPU of all phases `A0..F`**, which is a strict upper bound on Phase F's actual CPU consumption ($\text{CPU}_{\text{F}} \le 240,632,042\ \mu\text{s}$).

2. **Correction of Peer R399 CPU Unit-Conversion Error**:
   - Peer R399 deducted `ceil(117.965328 * 400000) = 47,186,132` usec (`10,800,000,000 - 47,186,132 = 10,752,813,868` usec). That multiplied seconds (`s`) by `cpu_quota_us` (`400,000` usec per `100,000` usec period) without dividing by `cpu_period_us = 100,000` usec and multiplying by `1,000,000` usec/s (off by $10\times$).
   - Because `cpu_quota_us / cpu_period_us = 400,000 / 100,000 = 4` cores (`4,000,000` usec of CPU per wall second), the dimensionally converted 4-core quota product over the residual `117.9653273010044359` s is:
     $$\lceil 117.9653273010044359 \times 4 \times 1,000,000 \rceil = 471,861,310\ \mu\text{s}$$
     (`471,861,312` usec at `117.965328` s), leaving `10,800,000,000 - 471,861,312 = 10,328,138,688` usec.
   - Using the actual kernel-measured `phases/cpu.stat` aggregate (`240,632,042` usec) yields `10,800,000,000 - 240,632,042 = 10,559,367,958` usec (or `10,800,000,000 - 289,006,953 = 10,510,993,047` usec using full `envelope/cpu.stat`).

3. **Reconciling In-Code Accounting, Timer Entry Overhead, and Config Ceiling Bindings**:
   - **Entry Elapsed Deadline (`retained_f_audit`)**: Recording `start = time.monotonic()` at `retained_f_audit` entry and setting `rem_wall = wall_budget - (time.monotonic() - start)` after `bound_identities(c)`, `owned_paths(c)`, `quotas(c, 'F')`, `frozen_source(c)`, and `prior_receipts(...)` charges all entry verification work against `wall_budget` and never extends the initial `ITIMER_REAL` deadline (`start + wall_budget <= start + init_wall`). A `finally: signal.setitimer(signal.ITIMER_REAL, 0)` block ensures no live timer leaks on any exit path, and `need(time.monotonic() - start <= wall_budget, 'whole-job wall cap')` enforces the deadline after `launch` even if `SIGALRM` is mocked in tests.
   - **Live Cgroup CPU + Config Scoping**:
     - In `retained_f_audit`, `sup_cpu = c['supervisor_cpu_seconds'] - (read_numbers(cg_fs(cg_paths(c)[1])/'cpu.stat').get('usage_usec', 0) + 999999) // 1000000` deducts live `supervisor/cpu.stat` usage before `setrlimit(RLIMIT_CPU, (sup_cpu, sup_cpu))` and refuses (`'supervisor CPU cap'`) if `sup_cpu <= 0`.
     - In `launch(..., retained_audit=True)` and `Group.check()`, `group.prior_cpu = read_numbers(group.path.parent/'cpu.stat').get('usage_usec', 0)` reads the phase-parent aggregate (`phases/cpu.stat`) before `group.create()`, refuses immediately if `group.prior_cpu >= v['cpu_time_us']`, and enforces `group.prior_cpu + usage_usec <= v['cpu_time_us']` (`'total cgroup CPU cap'`) throughout execution.
   - **Reconciling `prior_wall` & CPU Config Ceilings (Avoiding Unintended Double-Count vs. Fresh-Cgroup Reset)**:
     - Because `retained_f_audit` subtracts `prior_wall` (`39.1983804539959241` s from `A0_RESULT..E_RESULT`) in code, deducting the unreceipted residual `ceil(117.9653273010044359) = 118` s in config (`total_wall_seconds <= 6602`, `limits.F.wall_seconds <= 2582`) yields an exact non-double-counted whole-job remaining cap of `6602 - 39.1983804539959241 = 6562.801619546004` s. If config also deducts `prior_wall` (`total_wall_seconds <= 6562`, `supervisor_cpu_seconds <= 10642` [or `<= 10751` measured], `limits.F.wall_seconds <= 2582` [or `<= 2542` full outer], `limits.F.cpu_time_us <= 10328138688` [4-core residual quota bound] or `<= 10559367958` [measured `phases/cpu.stat` aggregate]), `limits.F.wall_seconds <= 2582` s remains the binding wall limit (`2582 < 6522.80`) and the reduced config ceilings prevent any reset even if a fresh process/cgroup had `0` prior `cpu.stat`.


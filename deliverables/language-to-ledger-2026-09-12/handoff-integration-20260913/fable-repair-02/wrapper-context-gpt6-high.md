The discrepancy is a reproduced orphan-reaping dependency in the test environment. It is not a PID mapping error.

Under the installed Foreman strong launcher, both remaining child PIDs were zombies (state Z, PPid1) in the same correctly mounted PID namespace as pytest. Foreman Node was namespace PID1. The first test had already passed wrapper exit124 and timing assertions.

At its absolute deadline, the C wrapper kills the child group, performs only nonblocking final waits, then exits. A killed child can become waitable after wrapper exit and be adopted by the outer namespace init. As an ordinary subprocess in these tests, the wrapper does not trigger teardown of that outer namespace.

A test-only subreaper harness under the exact same20-second strong launcher made both unchanged tests pass in1.53seconds. It sent no signals. Its retained wait statuses were child SIGKILL(-9), orphan wrapper124, child SIGKILL(-9), and test process0. The uncorrected context reproduced2failures in5.91seconds.

The minimal correction is a scoped isolated adopter/reaper for these two deadline/orphan tests. Keep original PID-absence assertions and retain reaping/status evidence. Do not apply it indiscriminately to pre-deadline wrapper-own-reaping tests, replace absence with zombie acceptance, lengthen the production deadline or exclude tests.

Keep real PID1 namespace teardown/heartbeat tests and exact production container PID1 configuration as separate acceptance requirements. This diagnosis does not establish Docker/Preview readiness or require a global Foreman change.

Evidence: /tmp/moriarty-wrapper-context-strong.log and /tmp/moriarty-wrapper-context-reaper.log. Exact commands, source hashes and limits are in the accompanying JSON. No source or live service/accounting changes were made.

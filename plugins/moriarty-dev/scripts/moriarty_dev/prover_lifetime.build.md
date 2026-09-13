# Prover lifetime wrapper build

From `plugins/moriarty-dev/scripts/moriarty_dev`, the only supported build is:

```sh
gcc -std=c11 -Wall -Wextra -Werror -O2 -static -o prover_lifetime prover_lifetime.c
```

Write the binary to an untracked build or temporary directory for normal development. Verify the result with:

```sh
file prover_lifetime
ldd prover_lifetime
```

`file` must include `statically linked`, and `ldd` must report `not a dynamic executable`. These checks establish that the executable has no dynamic-library imports. This file documents the only supported build; no other compiler, toolchain, or flags have been validated.

The production CLI is `prover_lifetime <control-file-path> -- <proof-server-path> [proof-server-arg...]`. It opens and validates the closed control record and live clock identity before forking, then uses `execv` on the supplied proof-server path without a shell or `PATH` lookup. CLI errors exit 2 and pre-fork refusals exit 64. A normal tracked-child exit before the wrapper forwards a signal retains child status 0–123 or 125–127; child `execv` failure is status 126. A tracked child that dies from signal N maps to 128+N. If a forwarded signal is caught and the child exits normally, the wrapper still maps the initiating signal to 128+N so a signal-driven stop cannot become success. If the fixed absolute deadline triggers the kill, the wrapper always exits 124. After forwarding SIGTERM, SIGINT, SIGQUIT, or SIGHUP unchanged, the wrapper allows exactly 2 seconds or the remaining deadline, whichever is shorter, before SIGKILL escalation. Its final nonblocking kill-and-reap sweep lasts at most 500 milliseconds and never passes the fixed deadline.

The wrapper is intended to be PID 1 in a private container PID namespace. It creates and kills the tracked child's process group and reaps descendants. Process-group signalling is not the complete containment guarantee: when a descendant changes process group, the wrapper's bounded PID 1 exit tears down every process in that private PID namespace. The container must therefore provide the private PID namespace and `restart=no`; this source artifact does not configure or start a container.

`--selftest-encode <boot-id> <time-namespace-inode> <latest-start-ns> <kill-deadline-ns> <invocation-digest>` and `--selftest-decode` are test-only codec hooks. The decoder accepts the record on standard input and prints its five parsed values; the encoder prints the closed record. These literal modes are dispatched separately from, and are never reachable through, the real `<control-path> -- <executable>` PID 1 form. They perform only codec validation and standard-input/standard-output byte I/O: they do not inspect clocks or `/proc`, fork, or execute a child.

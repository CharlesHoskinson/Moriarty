# Root audit diagnostic

The first read-only audit exited1 because it searched stderr for QNT508.
The original command places QNT508 and the assertion details in stdout.
The audit was corrected to inspect that retained stream. No command receipt,
kernel, test, runtime or original failure was changed or rerun.

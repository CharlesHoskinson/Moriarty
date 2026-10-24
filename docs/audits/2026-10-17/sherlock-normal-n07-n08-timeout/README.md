# N07/N08 partial result: harness timeout

The 2026-10-17 UTC attempt selected the two named N07/N08 prefix/cut tests in
the retained standalone 72-source Sherlock normal-test binary. Its single
harness child timed out after **30.001 seconds**, exited `-9`, and the operator
exited `1`. Preflight and postflight completed successfully.

The complete [137-byte stdout](test-stdout.txt), including its leading newline,
is preserved verbatim. It reports **N07 individually `ok`**. N08 has no terminal
result, and there is no final libtest summary. This is a partial observation:
neither a successful two-test run nor a full 147-test pass is established. No
separate N07 duration was recorded. Runtime stderr was empty.

[`result.json`](result.json) records the two full test names, timeout facts,
source/binary identity and original-evidence digests. The reused unoptimized
binary is `sherlock_b_normal_tests`, 12,836,968 bytes, SHA-256
`a4fdbbbabc2439e98bf3c3210c91e8ddb3ab23cb5c874f0885490fd925e014ad`.
No compile/link child ran in this attempt. Its source cohort remains the
captured 72-file composition; the recorded binary and source hashes were
unchanged, and the operator verified all 82 original bound pins.

The earlier 145-test result for this retained composition remains separate and
was not rerun here. Core47 and its 165-file source snapshot are a different
cohort. Neither result supplies N08's missing terminal observation. Local
hashes do not authenticate third-party producers or establish the unhashed
sysroot/std/libtest, linker/libc, Python and kernel trust boundaries; no sandbox
isolation or memory/disk/process-count bound is claimed.

The attempt is consumed and was not retried. Next recorded work is source-level
investigation of the cut/schedule workload and separate optimization proposals.
This package establishes no fix, timing improvement, timeout increase or
eligible retry. Independent O/complete-prefix, full B/ADT/finance, formal/circuit
and SDK/proof/target acceptance remain open. C4 is unchanged and full native R2
remains unsatisfied. Private controls, raw receipts and provider material are
not part of this public package.

# Existing runner controls, LL01 turn10

The verified run passes all six harmless controls under actual Foreman `posix_pidns_userns_strong` containment. It uses existing runner.execute and store.reserve/finish with an explicit temporary SQLite database. No canonical store, campaign or accounting is modified; this is not a load_runner admission test.

- Exact UTF-8 stdout/stderr recover from stored reservation receipt with matching byte counts and hashes. Launcher diagnostics remain part of real stderr.
- Exit113 and empty stdout produce only the runner's scoped assertion; no native result is inferred.
- Invalid UTF-8 rejects raw recovery despite process exit0.
- Timeout exits124 and rejects recovery.
- Overflow retains a bounded prefix, records all4096 output bytes and rejects recovery.
- Detached child readiness was observed before parent exit. After3.2seconds no delayed survival file and no matching host process remained.
- Thirty mismatched action/candidate/digest/charge/reservation identities reject.

Current evidence is verified-run/result.json. Each case retains exact plan, harmless child source, actual runner packet, exact stored receipt JSON, identity and recovery decision. Accepted streams have recovered binary files; rejected streams are not mislabeled exact raw recovery. The isolated test database is preserved. Runner, store, launcher, Python, Node and reused original recovery-test source hashes match before/after.

The first run also passed six controls, but its sourceUnchanged field compared relative two-file keys against an absolute six-file mapping. That was a test-report bug, not observed source drift. Its files remain intact. run-controls-verified.py corrects the comparison; the second full run supplies current verified evidence. Both invocations were harmless offline controls.

The original recovery predicates were reused from root-runner-consumer-probes-03.py; no process collector, runner or store implementation was added or changed. No K, Java, strace, compile, financial action or live diagnostic was invoked. Source and dispatch admission remain separate.

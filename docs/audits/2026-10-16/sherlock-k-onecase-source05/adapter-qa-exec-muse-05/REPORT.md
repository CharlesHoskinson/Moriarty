**APPROVE_WITH_SCOPE** — execution of the exact caller is eligible; no test outcome is inferred or approved.

Top finding / tail: **`EXACT TAIL K-SOURCE05-QA-CALLER-FULL-END-20261016` seen.** The supplied inline caller, transport, candidate, RED candidate, shared modules, focused tests, test file, and oracle-copy record all terminate completely. Muse used the supplied bytes only and called no tools.

## Scope conditions

1. Execute only this literal command, with no arguments, wrapper, interpreter substitution, or retry:

   `/usr/bin/python3 -I -B /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/run_qa_05.py`

2. Immediately beforehand, the operator must hold the exclusive ordinary slot and confirm stable trusted paths plus these externally supplied pins: caller `19552B / 1f30712d…285125`; transport `9831B / a0e94827…90228`; GREEN candidate `41183B / 5df990ed…77d1`; shared capture test `88920B / 15a4d0bf…ad6`; RED candidate `21483B / 7f99f295…ccbc`; and the supplied pins for `k_observations.py`, `k_terms.py`, `test_k_observations.py`, and `test_k_terms.py`.

3. The operator must also verify the four oracle files against `QA-ORACLE-COPY-RECORD.json` and confirm `QA-RESULTS-05` neither exists nor is a symlink. The caller stability-hashes the caller, oracle files, and 04 `RESULT.json`, but does not internally pin their expected values. Those external checks are therefore mandatory.

4. The transport may be imported only as pinned `_run`; its `main()` and the 04 mutation run must not be invoked. It provides `env={}`, a new session, a 30-second child deadline, process-group `SIGKILL`, and a 5-second drain.

5. RED must be recorded literally as the observed outcome. Only one exact RED result continues: `FAIL` with one matching failure, Ran 1, exit 1, and `FAILED (failures=1)`, or `ERROR` with one matching error, Ran 1, exit 1, and `FAILED (errors=1)`. RED `PASS`, `OTHER`, timeout, drain expiry, nonempty stdout, or source drift stops execution. GREEN failure likewise stops before FOCUSED; there is no delete, retry, or rerun.

6. Continuation requires GREEN Ran 39 ending `OK`, then FOCUSED Ran 57 ending `OK`, both with exit 0, empty stdout, unchanged snapshots, no timeout, and no drain expiry. Return 0 additionally requires complete postflight equality and the exact expected results-directory entries.

7. Preserve `QA-RESULTS-05` after any nonzero or traceback outcome. After execution, independently record the caller’s terminal exit status and hashes of `RESULT.json`, every step receipt, and the raw stdout/stderr logs.

## Residual limits

- Caller directory coverage does not snapshot every parent directory or the interior of the 04 `mutations` tree beyond the recorded `RESULT.json`; the exclusive slot, stable paths, and operator’s before/after hash record cover that boundary.
- Path checking is not race-free against a concurrent local writer. Exact pins, exclusive access, and stable paths are load-bearing.
- The transport cannot contain a descendant that leaves the child process group. The pinned candidate and tests contain no subprocess, network, K, SDK, native, circuit, proof, or engine invocation, so that limitation remains inside this scope only while the pins hold.
- If pre-step drift creates an empty step directory, the final directory listing may differ from the summary’s computed expected entries. That is fail-closed and returns nonzero; preserve the actual directory for audit.

Any failed condition changes the verdict to **BLOCKED**, and any needed source correction requires a complete canonical file from the designated current security owner. Final source/tests, the actual K bundle, independent comparison, and result audit remain required; this approval grants no reliance, W-landing, or publication approval.

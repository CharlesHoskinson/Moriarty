# FeePool tracked Quint integration QA

Both model and test typechecks exited 0. All 14 deterministic tests passed on TypeScript, seed 20261014, one sample each. The bounded 20-trace/24-step TypeScript run completed at seed 20261014 with no counterexample to allInvariants observed. All 23 action targets and the additional late-claim witness were observed; see results.json for every count.

The model executable body is byte-identical to the accepted input; only header comments changed. The test module combines original 13 cases and supplemental positive DA/L2 claims. Frozen runtime model/tests/references remain unchanged.

Commands, stdout, stderr, exits, timings and code hashes are recorded per invocation. All substantive Quint invocations acquired the shared build lock; the earlier metadata-only `quint --version` was outside the lock and is disclosed in integration-inputs.json. No native commands were run. The previous 1000x40 timeout remains an unknown verdict. This integration provides bounded model QA only, without source correspondence, native proof or ledger acceptance.

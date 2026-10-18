# GitHub work in `doctormoriarty<utxo>`

[Work items](work-items.json) and [issue assessments](issue-triage.md) preserve the fifteen-issue/two-PR intake and its EARS links. All fifteen issues remain open. Tracker #89 now records the source merges and retains all thirteen children.

[PR74](merge-20261017/PR74-MERGED.json) and [PR75](merge-20261017/PR75-MERGED.json) are merged after the user selected integration and fresh ordinary plus Opus 4.8 source reviews completed. PR75 uses a conventional nested Rust module; its body and all 90 staged fixture files are unchanged. Git reconstructed the exact combined tree before merge, and main matches it. These are source-only merges. Rust/Cargo, proof and ledger checks remain unrun.

The [redesigned delivery loop](../DELIVERY-LOOP.md) uses the existing active goal continuation and explicit local DoctorWatson QA. [Arming observation](../GOAL-ARMING.json) records runtime state at its timestamp; query the live goal before claiming it is still active. No daemon, replacement scheduler, Foreman or hosted automatic CI was added.

[Publication progress](GITHUB-PUBLICATION.json) links current receipts; older intake and source-preparation records retain their capture-time scope. The combined source passed 230 ordinary Sherlock tests and 10 plugin guidance tests, recorded in [integration QA](../../../../evidence/utxo/doctorwatson-language-loop-2026-10-17/INTEGRATION-QA.json). The separate UTXO reporting cohort passed 62 tests before this integration; that is reporting evidence, not compiler or ledger acceptance. Source-audit currentness must be reconciled against changed merged inputs before reusing affected observations.

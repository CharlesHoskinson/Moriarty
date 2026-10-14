# Logical-work Meter source report

The existing `mori simulate` command now accepts `--logical-cap`. The command applies the existing Meter inside the shared Core interpreter.

Status: tested source candidate. Fresh non-author Sol and Grok audits remain pending. The author made no commit or push.

Base: `5a0c6d4a6d33d59eec97fd2ca2a73027c5804ef1`. Candidate tree: `0ad5d989a0f19908414c032134cb96fc661e0d6b`. The candidate contains eight source and documentation paths.

The profile is `moriarty-reference-logical-cost/1`. Each attempted statement costs one logical unit. Each uncached expression node costs one logical unit per statement. Cache hits add no charge. Evaluation retains eager left-to-right order.

Failed actions expose no partial Execution, effects or poststate. Successful charges remain visible in CostReport. Denied charges leave used unchanged. Structural, action and scenario normalization refusals use zero units.

Metered replay reconstructs once under a separately trusted cap. Existing comparison then reports the first divergence. Comparison work remains outside this profile.

Final verification passed 36 Core tests and four CLI tests. Workspace formatting passed. Commands, final statuses and all logs are in `/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/logical-meter`.

Independent costs are counter 16, payment 4, duties 16, shared DAG 3, distinct literals 4 and cross-statement cache 6. Eager underflow consumes 7 units. Cap 6 refuses before the subtraction RHS. Counter cap 15 refuses at statement 4 with used 15. Payment cap 3 refuses at statement 1 and exposes no transfer.

The original replay oracle used the wrong path for a scalar payload mutation. Existing comparison reports `/steps/0/value/value` for UInt 7 to 8. Removing the optional value reports `/steps/0/value`. Tests now cover both paths and used 16. The root authorized this oracle clarification. The original failure log remains unchanged.

The authorized eighth path adds one logical-cost module binding to `proving::source_closure`. Source-bound contexts, manifests and derived material from earlier bytes become stale. Native authority remains zero. The author regenerated no native material.

The cap measures local primitive interpretation. Dust, service and external costs remain unmeasured. Parsing, validation, normalization, allocation, lookup complexity and replay comparison remain outside the count. CLI operational diagnostics before interpreter entry retain their existing form.

No native VM, query, preparation, SDK execution, proof, key, SRS, private-material, deployment or network path ran. Step, Execution, Scenario, ReplayRecord/1 and signed canonical representations remain unchanged. No U or M milestone exit closes.

`candidate.json` binds the tree, patch, eight full source files, input digests and all logs. `FOREMAN_REPORT.json` contains the test counts, failure history and remaining limitations. Required audits must inspect the full eight-file candidate.

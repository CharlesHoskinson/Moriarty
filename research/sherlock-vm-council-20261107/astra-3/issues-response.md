# astra-3 responses to ISSUES 1–10

1. **Mainnet: accept the chair proposal.** Preview can be this change's implementation acceptance exit. Specify the T4/network/runner/validation additions and the future authorized Mainnet operation here. Preserve broader obligations and the policy-amendment task. This council performs no deployment. Source: `BRIEF.md`, Phase 3 workflow requirements.

2. **Re-pin order: accept the revised proposal.** Select the compatibility baseline first, compare key identities per relation, regenerate the affected closure, then qualify on Preview. Stagenet is optional diagnostic work. My original milestone 1 already selected compatibility before milestone 2. I do not require spending a day requalifying RC artifacts. Sources: [position](position.md), `Cargo.toml:27-42`.

3. **Stagenet: accept for qualification rows.** New production qualification uses local release tests and Preview. Historical stagenet evidence retains its exact scope. Optional diagnostics cannot replace Preview acceptance. Source: [observations](observations.md).

4. **Dispatch partition: object to the listed fields as a sufficient equivalence relation.** The list omits data-dependent guard inputs. `equal_differ` compares selected value kinds, UInt/Bool projections, type and constructor identity. It selects `BHelperEqualDifferent` versus `BHelperEqualPair`. Two states can share every listed tag/phase/presence/zero predicate while differing in those values. Sources: `crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:750-806,1189-1200`.

   Proposed fix: derive the partition from the complete declared guard dependencies, including checked-value-read results, frame payload predicates, helper/task state and StepInv premises. Require directed cases for every admitted rule and priority boundary, even when P1–P3 do not reach them. Show why each partition preserves dispatch decisions on its stated domain, or label it as finite representative coverage without an equivalence claim. Retain fired-rule checks on every trace step and all supported competing rules per selected representative. Preserve malformed-input cases separately, with truthful failure classification.

   I accept bounded per-class/per-rule receipts with this fix. I do not insist that the old all-steps cross-product is the only possible acceptance test. A reduced diagnostic cannot silently take its meaning. Current dispatch is nonproving preprocessing: `crates/mori-backend/src/sherlock_machine/c6_dispatch_consumer.rs:1-8`.

5. **Steps: accept physical owner events, including publication.** Bind charged work independently through state. Require exact active length, range/no-wrap checks and inactive suffix rules for partial chunks. Source: `crates/mori-recursion/src/ivc.rs:191-201,305-320`, universal-interpreter `design.md:25-35`.

6. **Leaf: accept both measured U/F outcomes with S=1 first.** S=8 may be an experiment target. It must not be a forecast or fixed acceptance shape. Measure complete-path cost and memory alongside k. The last partial chunk and early-terminal/exhaustion cases must be part of the contract before S>1 proving. Source: `docs/sherlock/stagenet-gp-recursion.md:184-201`.

7. **B chain: accept non-gating Preview work.** Preserve its requirement lineage, but keep it outside U023/C4 dependencies. Do not silently delete financial obligations when superseding MC03's implementation. Sources: `docs/MORIARTY-PRODUCT-CONTRACT.md:35-49`, MC03 tasks §2–4.

8. **IDs: accept the five proposed prefixes.** Map each old obligation and each ASR/SND/scope clause to a new requirement or a named retained task. The checker must detect omitted clauses, dangling references and evidence without a consuming gate. Source: `BRIEF.md`, traceability deliverable.

9. **Throughput: accept as a measured engineering target.** “Met or shortfall recorded” closes a measurement/reporting task only. It cannot close a requirement that promises the stated latency or memory. Keep performance shortfall separate from semantic and cryptographic acceptance. Source: `wiki-llm/recursion-perf/CONSENSUS.md:19`.

10. **Small path: accept the IVC node with a real C6 leaf.** Keep the old two-child chain for regression and migration diagnosis. It does not gate general-purpose completion. Source: `crates/mori-recursion/src/ivc.rs:1-21`.

Additional constraints: typed negative-test accounting and candidate freshness remain mandatory. Git HEAD alone cannot detect changed keys, a reset chain or a new live runtime. The scout's missing deployed-source mapping remains an explicit reproduction limit. Source: `wiki/midnight-ledger-9-hardfork.md:166`.

## ZKIR version clarification

Fable-2 questions the position's Compact 0.36 → ZKIR 3.1 statement. The primary Compact 0.36 release notes explicitly say the compiler continues to generate ZKIR 3.1 while its bundled `zkir-v3` binary comes from `zkir-3.2.0-rc.2`. A standalone tool's default IR minor version is a separate fact. Keep compiler, tool binary and emitted artifact versions distinct. Verify the actual artifact header during compatibility qualification. Source: [Compact 0.36 release notes, “The zkir-v3 binary is updated”](https://docs.midnight.network/relnotes/compact/toolchain-0.36.0).

## Active predicate selection

Fable-3 correctly notes that adding Preview rows does not stop `sherlock-next` from returning the historical failing A8. The workflow must solve this explicitly. Either migrate/retire the active legacy rows with their historical results preserved, or implement a selected-milestone/target filter. Name the exact addition and acceptance test. Reordering rows only postpones the obsolete failure. Adding a false dependency on Preview completion would misstate the dependency graph. Test that the ordinary factory entrypoint returns the next required Preview task while the historical A8 failure remains inspectable. Sources: `models/doctorwatson/factory/CONTRACT.md`, `models/doctorwatson/factory/status.py:34-47`, `fable-3/issues-response.txt`, issue 3.

# About evidence and the correctness loop

Draft, 2026-10-17. The correctness loop remains required under final03. Subsequent [finite Quint validation](../../results/sherlock-quint-design-20261017/README.md) passed44 selected tests and five directed/sampled runs, with failures and repairs preserved. This supplies bounded model evidence.

Sherlock's correctness work develops the same declared transition slice across K, finite Quint, the existing private Lean/Clean integration and handwritten circuits. The purpose is to expose disagreements while each slice is built. Finishing all host code before beginning comparison would leave semantic and representation mismatches hidden until the most expensive boundary.

Each layer answers a different question. Actual K rule execution supplies concrete semantic traces. Quint explores a declared finite abstraction with explicit actions, bounds and seeds. Pure Lean semantic observations are distinct from theorem evidence. Bounded Clean component execution, witness computation, constraint checks and export each have their own scope. Handwritten components need actual witness/constraint locators on the same bounded interface. Native and target receipts add further, separate observations. The [correctness-loop specification](../../SHERLOCK-CORRECTNESS-LOOP.md) retains these distinctions and the existing private integration; private source, paths and pins stay private.

Independent expectations make this loop informative. They describe initial state, every required prefix, final result and no-next behavior before the subject runs, using normative/input/API facts rather than importing the implementation under test. Ordered effects, preserved fields, absent versus present-zero values and work belong in that expectation. Comparing two wrappers around the same evaluator mostly checks internal agreement. A comparator's ability to detect changed cells tests that comparator; a separate actual producer receipt and comparison are still needed.

Complete records also need continuity: initial equals first.before; adjacent after/before states agree; last.after equals returned state; charges and terminal publication reconcile. Equal arrays with missing required members or fields are not complete evidence. The first differing field, rule and prefix explains a failure more precisely than a final-value mismatch. Correcting an expectation requires an independently reviewed semantic correction, rather than regenerating it from a new output.

The audit baseline has one actual K N01/fuel1 case with two independently expected records; the other 215 fixed inputs remain unobserved in K. Earlier host/image/generic-Quint/pure-Lean captures cover scoped eight-program/216-run comparisons, without proving cross-layer refinement. Existing pure Lean work is present; full bounded Clean/handwritten/native correspondence is not established. The [actual stream attempt](../../results/sherlock-stream-20261017/README.md), `stream-actual-01`, compiled and ran the actual visit_steps producer and eight tests under Rust 1.99.0; its complete supplied host diagnostic artifact matched the independent normal corpus for eight programs and 216 fuel runs. All 26 recorded source pins were unchanged during that attempt. The execution receipt binds the local producer run; the comparison consumer itself does not authenticate producers. Constructor/startup is outside its post-startup scope, and none of the model/circuit/native layers ran in this attempt. Financial Core/2 and Core47 observations keep their different scopes. Frontend checks on later Quint terminal sources are not execution or adopted model revisions.

Final03 reconciles ten Gemini HIGH reports through three Astra MAX reviews. Five Astra HIGH final reviews support ordinary implementation readiness and retain their native/full-completion qualifications. The additional bounded model represents selected interpreter/observer/helper/boundary/financial cases. Its33 A, eight B and three directed finish tests passed; five directed/sampled runs reached all38 requested witnesses. The independent narrow result review closes this model entry condition. Constructor/composition/residual/full-finance/shared-tree cases remain omitted. These results cannot establish all39/all96, full UInt128/field behavior, Clean constraints or settlement. The separate [333-carrier result](../../results/sherlock-composed333-20261017/README.md) adds an actual fixed host capture and independent comparison, without closing generic trace continuity or full-B record production.

This evidence remains attached to source membership, profiles, bounds, projections and actual results. A changed dependency invalidates the comparisons that depend on it; unchanged historical receipts keep their original meaning. Source approval, actual-result review and product acceptance are separate. Full UIREQ/U/M/MC/SP/G exits, current Opus-owned obligations and target qualification remain required.

The [result reference](../reference/profiles-results.md) names evidence classes. [Adding a normal case](../how-to/add-normal-case.md) and [finding a first difference](../how-to/find-first-difference.md) describe the practical tasks.


## The finite projection report now has scoped acceptance

The [original021 adoption](../../results/sherlock-k-quint-f03-actual-20261018/ROOT-ORIGINAL021-ADOPTION.json)
accepts the report-content clause and finite call-return scenario for one actual
F03/fuel93 K/Quint pair. The final review read all95/94 common data in lossless
factored form and complete report metadata/comparison bodies; large raw contexts
remained separately pinned, not claimed manually read in full. Original021 asks
for a declared common domain, rule-to-action projection and honest differences.
It does not turn this finite trace agreement into a semantic refinement theorem.

Source, actual results, reporting acceptance and general tool installation remain
different facts. Seven map-key helper/log guards subsequently passed, but the six
assembler categories, multiline decoder guards and end-to-end chain propagation
remain required before general adoption. Complete captures, failed attempts and
unmeasured finance/B/native scopes remain explicit. Root's accepted count is15
originals after this adoption; no broader correctness-loop completion follows.


## October 18 — audit acceptance, preservation and cleanup are separate facts

The corrected retained R3 audit passed and both independent result reviewers accepted its scoped build-provenance result. Root subsequently verified durable preservation of 3,554 files (3,498 target files, 55 packet files and one external marker), totaling 2,628,344,281 bytes, then completed all six cleanup operations. The three tmpfs mounts are gone, cgroups are absent and the final receipt records `cleanupOwed:false`. Archived files remain read-only/nonexecutable; native outputs remain unaccepted and the SDK target ELF was not executed. Proofs, native target semantics, stagenet and full Sherlock completion remain open. No requirement status changes; the current published count remains 15/35.

The [two independent scoped audit opinions](../../results/sherlock-native06-association-audit-20261018/README.md) support the recorded output association and compile provenance, not execution or promotion of the SDK target. Same-context source/operation/result followups do not create extra independent votes. Root's [later filesystem disposition](../../results/sherlock-native06-association-audit-20261018/actual/preservation/ROOT-PRESERVATION-DISPOSITION.json) verifies preservation, and the [subsequent cleanup receipt](../../results/sherlock-native06-association-audit-20261018/actual/preservation/POST-CLEANUP-RESULT.json) resolves cleanup separately. Preserve raw reports and their factual qualifications: the complete packet has 55 files plus a separate 9-byte marker, not 29 packet files. A current hash binds bytes; it does not extend any earlier host, financial, formal or target claim.


## Evidence for the canonical TypeId migration

The migration has [independent source review](../../results/sherlock-canonical-runtime-types-20261018/reviews/source-and-operation/REPORT.md), three successful compiles and ten passing new selected tests. The ordinary consumer connects actual prepared signatures to live owner states and independent bound-value/result expectations. Nominal checks cover storage only.

The immutable prefix receipt records a controller false-stop after test09 actually passed. [Root accepted the same 326-byte stdout offline](../../results/sherlock-canonical-runtime-types-20261018/actual/offline09-and-tail-selection/ROOT-OFFLINE-09-ADOPTION.json) following [independent readback](../../results/sherlock-canonical-runtime-types-20261018/reviews/offline09-and-tail-eligibility/REPORT.md); [four tail tests](../../results/sherlock-canonical-runtime-types-20261018/actual/tail-10-through-13/RESULT.json) then passed without replaying the prefix. Original failures, historical UNRUN cuts and full streams remain unchanged.

The [Root adoption record](../../results/sherlock-canonical-runtime-types-20261018/ROOT-CANONICAL-TYPEID-COMPATIBILITY-ADOPTION.json) accompanies integration: 63 current source references are rebound, three historical citations retain their old bytes through archive paths, and the two affected live document pins use the composed documents. The 15 existing original-clause scopes are retained; this adds no closure. Original UIREQ034 and all 25 refinements remain open.

One continuing independent reviewer, a0ecce6e, supplied the source and offline-readback opinions; backend-canonical model attestation is false. Root's source-review readback counts 87 unique complete pages, correcting the report's 86. The [result summary](../../results/sherlock-canonical-runtime-types-20261018/ACTUAL-TEN-TESTS-SCOPE.json) separates tested scope from unrun vectors and unbound absolute B clocks.


## Dated evidence classes — October18

The [literal035 adoption](../../results/sherlock-four-lane-progress-20261018/witness035/ROOT-ORIGINAL035-ADOPTION.json) accepts the original deterministic witness-dataflow clause with its finite limits. The matching reader reports16 current adopted originals and19 open; all25 refinements remain open. Preserve earlier15-adopted observations as dated history.

[The new Clean evidence](../../results/sherlock-four-lane-progress-20261018/addenda/20261018-literal035-clean-circuit/clean/sherlock-nominal-proof-scope-20261018.md) consists of four kernel-accepted modules and38 named generic-reference theorems, merged by FenrirPR11. The public final source/result/fork-scope opinion says APPROVE, while its transport terminal remains ERROR. Both facts belong to the evidence record. Theorems retain their stated premises and do not establish complete production/B/native correspondence.

The installed checked-Add handwritten circuit source and its eight authored tests are a separate source increment. Actual SDK compilation, constraint checks, proofs and target acceptance are unrun at this cut. The [circuit concept log](../../../wiki-llm/sherlock-circuit-atlas/README.md) illustrates source construction incrementally; it does not publish a live site or turn an unrun test into execution evidence.


## October 18 final scoped evidence distinctions

The three-file F1/F2/F4 generic correction now has a separate metadata pass: exit 0 in 4.895931652 seconds with 22 warnings, 494 unchanged input pins, 102 emitted dependency sources and all 16 required sources present. Fresh primary Opus approved exactly the five source hunks in 303.650 seconds. Nine other findings remain open, including cfg(test), observation, dynamic-error-policy and resource gaps. The opinion retains its supplied-source limits and is not runtime/QA eligibility. No generic admission or runtime execution is claimed. Metadata checks, source semantics and an actual admitted run remain separate requirements.

Clean's four finite nominal/caller cases now cover 455 complete states. The later three-case invocation contributes 134+134+99 states with complete typed JSON and raw-byte equality to independently prefrozen expectations; the earlier 88-state case remains separately dated. Actual output froze before expected access, and the correlated hand-derived K/Quint oracle basis remains disclosed. Fenrir PR12 merged at 23:22:21Z (`83ee008c3b2b0d07d3da9abb6d14b73d13d3b578`). This is finite logical evidence, not universal production or circuit correspondence.

K's four cases also have complete independent typed equality across 134+134+88+99 states, 455 total. Raw bytes differ; key membership, values, arrays and scalar kinds agree under the declared comparison. The freeze 839e4304 preceded expected-body access. Fresh primary Opus approved the finite result, explicitly decoding roughly 30 states and not individually decoding about 425; Root's complete machine comparison covers all 455. Conditional P1/P2 README corrections were made, and Root retained the historical manifest/status with two hash overrides. PR268 merged at 23:36:04Z (`d37b5f2e02a297aa151cd8e91754cfca90c1451a`). This adds no EARS closure or B-charge/native claim.

Quint B10 retains its reviewed finite frontier-result scope and merged PR267. Its PURE reference operand and all-cuts iteration identities are not an independent full-state oracle or checkpoint ABI. Its permanent owner holds its publication records; this documentation batch duplicates no formal archives.

P8 completed `cargo test --no-run` and built the backend SDK test ELF; Cargo reported 55.60 seconds. The enclosing operation FAILED with exit 2 after 132.4613 seconds. The later fresh Opus diagnosis reconciled the whole metadata as six byte-identical build-script binary recompiles and eight build-script run-unit reexecutions; the guard correctly failed, and this is not a benign-mtime waiver. The report marks recalled Cargo-internal explanations as [C], requiring pinned primary-source confirmation. Root verified that all 3,498 original archive stat mtimes were preserved and requested a concrete seed-order source correction. The guard remains unchanged; no unchanged Cargo rerun is proposed.

All 3,565 files/2,847,417,755 bytes were preserved with exact membership/hashes and nonexec modes. Cleanup completed with cgroup absence and `cleanupOwed:false`; outputs remain UNACCEPTED. No ELF, test body, proof or target transaction executed. PR266 merged the two Rust test-compile fixes into the native feature only at 23:22:18Z (`582288fc769d6374424b974cda4310288eba184f`), preserving the separate source/branch history and strong SDK assertions.

The earlier broad ReturnRight request remains a recorded timeout with no source or vote. Subsequent source work is now concrete: a nine-file combined P1/F4+M1 candidate is materialized, including the 62,630-byte ReturnRight source (`aa141bdc…`). Its full source audit is pending and SDK tests are unrun. Prior staged semantic support is not approval of the combined candidate or a compile result. Scalar.tag005 remains recorded archived/unregistered source, not a native circuit result; this batch adds no website log iteration. Keep handwritten circuits alongside Clean, Quint and K through the existing four-lane recovery contract.

These are distinct evidence classes. Original adoption remains 16/35 and all 25 refinements remain open. See the [dated record](../../results/sherlock-four-lane-progress-20261018/addenda/20261018-admission-and-complete-traces/README.md).


## October20: source wiring, model observations and proof acceptance

The [PR335 carrier result](../../results/sherlock-financial-carrier-unit02-source-20261020/README.md) advances an actual component: real Core builds and sixteen selected carrier tests passed. Financial admission/genesis/owner dispatch and the complete financial requirements remain open. Counting that component as the whole producer would erase a missing connection.

A [separate dated engineering snapshot](../../../wiki/research/sherlock-circuit-atlas/ENGINEERING-018-EVIDENCE-LAYERS.md) keeps the other current layers explicit. Clean B45's strict finite copy/splice proof retains its route, endpoint-avoidance and post-edge-equality premises; it is not the actual SDK assignment bridge. K's46 complete positive representation observations matched both prefrozen common JSON and typed AST after a durable barrier; they are unadmitted representations, not charged helper transitions or a refinement theorem. Both recursive-ADT host captures Halted; the released independent SAFE result now records complete recursive typed values, nominal declarations/generic arguments/constructor fields, source bodies/entry and config/target matching the unchanged prefrozen expectations. This does not establish whole owner images/traces, work/resource counts or universal termination. Native24 passed synthetic SDK-free guard fixtures and private PR53 published the reviewed nine-source cut; Build9 and real keyed MP2 remain unrun. The corrected Quint B40 frontend passed; tests02/03 are not qualified by that frontend result.

A circuit diagram, a proved mathematical relation, a guard fixture, handwritten constraint source, an SDK check, an executed recursive proof and a target receipt answer different questions. Draw or report their connections only where the actual interface/evidence exists. No degree, speedup, proving cost, native recursion or stagenet validation is inferred here. The [circuit concept log](../../../wiki/research/sherlock-circuit-atlas/index.md) remains asynchronous, with designated Muse/Opus creative prose and separate dated engineering facts; its finished website launches only after the full Sherlock goal is finished and accepted.

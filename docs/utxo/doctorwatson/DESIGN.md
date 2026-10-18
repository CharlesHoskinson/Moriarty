# `doctorwatson<UTXO>` — Cardano/Groth16 workstream design

Status: three advisory Astra designs received; bounded implementation authorized by the user's build → EARS update → roadmap audit instruction. The user directs building this adaptation, then updating all EARS and auditing the remaining roadmap. The implementation plan is [here](../../superpowers/plans/2026-10-17-doctorwatson-utxo.md); runtime capability is reported separately. No new loop is armed. The broader Preprod goal remains paused.

## Naming

The user-selected display name is `doctorwatson<UTXO>`. Use `doctorwatson-utxo` for command/file names. The angle-bracket form is a display name, not an unquoted shell command. The existing Sherlock DoctorWatson retains its own entrypoint and evidence namespace.

## What this version should deliver

Give the UTXO workstream one local command that answers three questions: which exact EARS predicate remains open, what actual implementation or check can resolve it next, and what current evidence supports a claimed closure? Every development cycle starts with a requirement and ends with an observed behavior, a reviewed result or a concrete blocker. Documentation, graphs, model availability and growing test counts do not close product requirements.

The inspected Sherlock source is pinned to `0aecf98455dd9abc1635801166151431cba309f7`. The UTXO baseline is `1721457439dbd6d988d00c6c86d1cf648caf2a0c`. [Source inventory](SOURCE-INVENTORY.json) and [assessment](ASSESSMENT.json) record the inspection limits.

## What Sherlock already provides

Sherlock's default DoctorWatson entrypoint passes its current EARS evidence record to the ordinary checker. Its delivery loop chooses a missing requirement predicate, implements or tests that behavior, obtains applicable independent review, then updates the evidence row. A source edit makes affected closure evidence stale.

The reader keeps implementation, validation, owner acceptance and computed currentness separate. A valid report can contain open requirements. Matching hashes do not authenticate a reviewer or establish semantic correctness. The checker preserves named scenarios and distinguishes original requirements from additive refinements and retained families.

The current implementation is specialized: it hard-codes the Sherlock specification paths, 35 original UIREQ IDs, 25 SCON refinement IDs and 68 refinement scenarios. It also runs Sherlock's own corpus. Copying its shell script or pointing it at the UTXO JSON would not produce a working UTXO version.

## Approaches considered

| Approach | Benefit | Cost | Decision |
| --- | --- | --- | --- |
| Add a UTXO profile with its own small entrypoint, inventories and requirement reader | Reuses the established evidence and delivery conventions while leaving Sherlock's working lane intact | Needs a bounded implementation and UTXO-specific regression tests | Recommended |
| Generalize the complete Sherlock checker before starting UTXO work | Could eventually share all catalogue parsing and evidence checks | Changes an active workstream and delays the first UTXO requirement closure | Defer until two concrete consumers demonstrate the useful shared interface |
| Start an independent scheduler/dashboard/factory | Could offer a new interface | Duplicates coordination machinery and does not supply missing compiler or ledger behavior | Reject |

The recommended profile is part of DoctorWatson. It has no agent dispatcher, daemon, provider polling, spending authority or replacement milestone system. Initially it may use a few local parsing/hash helpers rather than import Sherlock's interpreter-specific checker. A later shared helper extraction must preserve both consumers and have its own affected regression checks.

## Retained requirement inventories

The [design baseline](EARS-BASELINE.json) preserves exact headings, clauses, named scenarios and current file hashes. These are separate inventories, not a combined completion percentage:

| Inventory | Requirements | Scenarios | Treatment |
| --- | --- | --- | --- |
| Five existing UTXO packages | 119 | 238 | Keep every EUC/EU-LDG/EU-FIN/EU-NAT/EU-CON ID and all 20 families |
| Cardano first-contract sprint | 11 | 16 | Keep CS01–CS11 and their legacy profile/Preview scope |
| Retained Midnight PCD specifications | 18 titled clauses | 38 | Preserve their source headings; do not turn Midnight proof APIs into Cardano requirements |

The 119 coverage-index rows currently say specified-only. They are not 119 demonstrated current-Rust behaviors. In particular, EUC-001 binds /5 and /6 factories, CS01 names profile/5 and CS10 names Cardano Preview. Neither Rust `moriarty/1` nor Preprod can inherit those bindings.

A profile-reconciliation record must classify each retained clause as current-scope applicable, shared obligation needing an explicit adapter, legacy-profile evidence only, or awaiting a current-profile refinement. Unassessed is the initial disposition. Legacy evidence remains visible. An unsupported feature stays an obligation; it does not become unnecessary or complete. New Rust/Groth16/Preprod requirements need explicit versioned additions under the existing ownership rules. Existing clauses are not silently rewritten.

The retained-family view also references general obligations, ACTUS/DeFi, recursion, private handoff and multi-parent composition. A single escrow, compiler fixture or proof result cannot discharge those families. The missing original 42-predicate catalogue remains a provenance gap.

## Local interface

Proposed entrypoint: `./scripts/doctorwatson-utxo.sh`. It reports UTXO requirement integrity and runs the focused ordinary checker regressions. It does not invoke the Sherlock corpus, SDK, compiler, prover, model engine, wallet, node or network. Explicitly admitted subject checks remain separate commands with their existing limits.

Proposed read-only selectors:

- `--next`: show the next actionable requirement/scenario, actual consumer, independent expected result, owner, missing dependency and the permitted next check. Also show blocked requirements separately. Selection does not execute the displayed command.
- `--requirement EU-NAT-028`: show the complete clause/scenario evidence, source/profile/target identities and remaining predicates.
- `--gate EU-NAT-028`: return nonzero unless the selected complete predicate has current, applicable, reviewed acceptance evidence. It grants no execution authority.

The ordinary report returns 0 for a valid complete inventory even when requirements remain open, 1 for stale/missing/inconsistent declared evidence and 2 for malformed input. A selected closure gate is a distinct check: an open or unassessed row cannot pass it. Its output must say whether integrity or requirement closure was tested.

## Evidence record

Canonical proposed location: `openspec/changes/utxo-doctorwatson/current-evidence.json`, beside a versioned schema and profile reconciliation. Keep each product ID bound to its original normative file and complete named scenario list. Each row carries:

1. Exact predicate and supported domain, original/added lineage, applicable source/semantics profile and target.
2. Implementation state and the actual consumed source paths.
3. Per-scenario independent expectation, positive/refusal input identity, selected consumer and required evidence classes.
4. Captured command, attempt identity, exit, raw result digest and complete relevant comparison.
5. Applicable nonauthor review, owner acceptance, remaining gaps and next discriminating action.
6. Declared source/dependency/profile/oracle/result/review pins and computed currentness.

These are distinct axes. A file can be current while its implementation is absent; a test can pass while a required scenario is unrun; a review can approve source without accepting the actual result. The reader validates the declarations and their references. Semantic acceptance stays with the applicable owner/reviewer process and cannot be generated from file equality.

The record must distinguish universal property certificates, concrete execution proofs and authenticated ledger claims when those predicates are added. It must carry their actual program, semantics, predicate, verifier and anchor identities. A Groth16 verification result alone cannot fill an authenticated-ledger row. The cryptographic owner defines the substantive claim checks; this reporting adaptation does not invent them.

## Currentness and failure handling

A changed consumed source, adapter, schema, profile, oracle, bounds, tool, result or review invalidates affected current closure. Keep historical owner declarations and original failures; remove stale rows from the current-closure subtotal. Track declared dependency edges transitively and report undeclared closure as unestablished. Recheck affected integration changes rather than rerun every unchanged passing cohort.

Evidence references are explicit repository-relative public files. Historical absolute paths in the old coverage index are inert provenance strings, not file-discovery instructions. Private wallet/proving material and provider captures are outside this reader's input set. The checker does not parse commands out of Markdown or execute them.

Missing required evidence affects its consuming predicate. A blocked native stage must not block independent ordinary language/reporting work. Conversely, ordinary QA cannot close native, authenticated-ledger, formal-correspondence or target scenarios. Required input, state, work, fees, collateral, ordered effects and residual duties remain part of the relevant predicate.

## Requirement-driven delivery cycle

1. Select one still-open clause and the exact missing scenario or behavior from the report. Name the real consumer and independent expectation before subject execution.
2. Implement that consumer increment, or add the discriminating check for already implemented behavior, under existing authority.
3. Run only affected ordinary checks and separately admitted semantic/native checks. Retain first divergence, refusals, timeouts and complete result scope.
4. Obtain the applicable fresh nonauthor source/result review. Changed source needs affected fresh evidence.
5. Update the row and its scenario coverage. Close the whole clause only when every required predicate is satisfied. Publish and move to the next actionable row.

A partial subpredicate may be accepted explicitly without closing its parent. A shortened substitute clause, an unsupported refusal or a green factory test cannot be used to inflate completion. A blocked row names the missing premise, responsible owner and observable event that would unblock it.

After this first reporting increment, do not expand the factory unless an actual requirement-closing attempt exposes a concrete defect in it. Normal cycles must change product behavior or resolve a named acceptance predicate. Graphs and progress summaries remain supporting artifacts.

## First delivery and subsequent targets

The first bounded implementation covers the requirement/profile inventory, the current evidence reader and its refusal tests. Its product-facing target is EU-NAT-028: an actual report must include every one of the 20 families with separate source, admission, compiled, native and private statuses. An incomplete or duplicate family list is a discriminating negative. All unobserved layers remain unobserved. A fresh reviewer must assess this exact reporting predicate before any whole-clause closure is recorded.

Carry EU-CON-003 failure-layer reporting in the first actual consumer increment. The current Rust relation → proof → fixed C2/S2′ route requires additive, versioned artifact/publication/consumption predicates linked to EUC-012/013. Those original clauses explicitly require generated Aiken and a generated spending entrypoint; fixed-verifier results cannot close them. The next product batch implements the real selected artifact consumer under the additive route, with original obligations retained. R4 artifact tests or a D7 absent-checker baseline are child evidence, not whole EUC-012/013 closure. The corrected hash-checker, actual builder, generated-spend evaluation and proof/ledger stages retain their actual missing reviews and execution conditions.

Do not automatically choose the easiest reporting row forever. Once the bounded first delivery is accepted, the next selection must advance the current Rust source → Cardano artifact → verified claim output → consumption/composition → settlement path, while retaining every family and broader obligation.

## Proposed factory EARS

These DWU clauses describe the adaptation, not extra product-completion credit.

| ID | Requirement | Discriminating check |
| --- | --- | --- |
| DWU-01 | WHEN the UTXO report loads, it SHALL preserve the exact IDs and named scenarios of each pinned catalogue as separate inventories. | Replace one ID with a duplicate while keeping the count constant; remove a scenario while preserving the aggregate total. |
| DWU-02 | WHEN current-Rust acceptance is requested, the report SHALL require an explicit applicable profile/target mapping and SHALL retain legacy evidence separately. | Supply a /5 or Preview receipt as Rust/Preprod closure and reject the declaration. |
| DWU-03 | WHEN supporting declared inputs change or disappear, the report SHALL mark affected evidence stale/missing and exclude it from current closure while retaining history. | Change one shared dependency used by two rows; both closures become non-current. |
| DWU-04 | WHEN closure is declared, the selected row SHALL name complete scenario coverage, applicable evidence and a scoped independent acceptance record. | Omit a negative scenario, actual-result review or required target observation. |
| DWU-05 | WHEN a report is valid but requirements remain open, it SHALL distinguish integrity success from requirement closure. | A complete all-open record passes integrity and fails the selected closure gate. |
| DWU-06 | WHEN next work is requested, the report SHALL identify an actionable missing predicate and SHALL retain blocked rows with exact missing dependencies. | Block one native row while an ordinary consumer row remains actionable. |
| DWU-07 | WHEN a family report is emitted, it SHALL retain all 20 families and separate source/admission/compiled/native/private status. | Drop or duplicate one family; reject a native pass inferred from source attribution. |
| DWU-08 | WHEN supplied result evidence is reported, the report SHALL preserve the original result identity and failure layer. | Present an expected missing-implementation RED as successful checker behavior and reject that interpretation. |
| DWU-09 | WHILE ordinary UTXO QA runs, it SHALL perform public file/report checks only and SHALL NOT invoke model, SDK, proof, wallet, node or ledger consumers. | A displayed proposed command remains data; default invocation starts no subject command. |
| DWU-10 | WHEN the UTXO profile is integrated, the existing Sherlock entrypoint and scoped results SHALL retain their own behavior and identities. | Run the affected separate ordinary regression lanes and compare the preserved Sherlock contract. |
| DWU-11 | WHEN a first reporting increment is accepted, subsequent cycles SHALL select a missing product predicate unless a concrete closure attempt exposes a factory defect. | A cycle naming only administration or graph counts cannot claim a product closure. |

The first implementation must include meaningful regressions for these cases. Test fixtures may simulate declarations to test the reader; they must be labelled synthetic reporting fixtures and never count as real ledger or product evidence.

## Review and implementation boundary

This document proposes a nonsecurity reporting/QA adaptation. Substantive cryptographic/security requirements and their judgments remain with the designated owner and authorized outage fallback. Model identities, frozen rosters, resource limits and failures remain attached to their original records. No new resource grant, native retry or wallet use follows from the design.

The next implementation plan should name the small owned file set, normal/refusal expectations, affected existing consumers, exact ordinary checks and independent reviewer. Do not start a second orchestration system.

Design checklist: context and actual Sherlock consumer inspected; separate UTXO inventories checked; three approaches compared; proposed interface/data flow/failure behavior/tests specified; implementation follows the user-authorized bounded build and the three advisory designs. The source inventory states partial read limits explicitly.

## User-directed follow-through

The requested order is: build `doctorwatson<UTXO>`; update the retained EARS assessment against actual source/results; audit the remaining roadmap through its complete retained scope. The later audit must enumerate every requirement/scenario or atomic obligation and classify it as demonstrated, contradicted, incomplete, unverified or missing, with the evidence that would resolve it. Updating an assessment is not permission to weaken normative clauses or mark unchecked work complete.

The roadmap audit retains the single U0–U7 sequence, M1–M17, MC01–08, SP01–12, G01–24 and their formal, financial, security, composition and release obligations. `openspec/current-requirement-crosswalk.json` is a 160-heading discovery register, not an atomic completion denominator. The October 14 EARS-preservation baseline retains 388 original lines across 85 files; its inclusion result does not establish semantic acceptance. Keep those registers, the UTXO inventories and Sherlock's original/refinement inventories distinct and cross-linked.

Do not perform a blanket rewrite of the requirements. Preserve full source text, scenarios, binding design references and historical status. Where the current Rust/fixed-Groth16 route differs from a legacy generated-Aiken clause, propose a versioned linked refinement under the applicable owner rather than claiming the routes equivalent. Security requirements remain with the designated security owner.

After the first bounded reporting increment, parallelize the remaining audit by disjoint requirement domains and evidence owners. The integrator combines the results into the existing roadmap and EARS records; no additional delivery roadmap or scheduler is introduced. The audit may be complete while the implementation roadmap remains open. Its completion criterion is exhaustive, source-grounded disposition and concrete remaining tests, not a green product-completion claim.

## Design synthesis

All three Astra proposals recommend a small local reader, preservation of all original clauses and a strict stop after the reporting bootstrap. [Cardano](astra/cardano-design.md) identifies the generated-validator/fixed-verifier mismatch; [Groth16](astra/groth16-design.md) separates five evidence types and eight phase gates; [parallel delivery](astra/parallel-design.md) supplies twelve disjoint lane examples under the 32-worker ceiling. These are advisory designs, not cryptographic acceptance. K7 public VK availability resolves the earlier publication gap only; pairing reproduction and consumer-established relation linkage remain distinct. Preserve the C2 → VI(c2) → VS3(c2,vi) → S2′(c2,vi,vs1,vs2,vs3,vs4) DAG and the recorded FV-02 shared-shard restriction.

## Source correction after design review

The later audit found [K10](../../../evidence/utxo/current-cardano-compiled-claim-wrapper-2026-10-16/README.md): the captured real K6 proof was evaluated through an already compiled/applied C2 mint wrapper, with one success and six semantic EvaluationFailure outcomes. Its root adoption closes only `compiledC2WrapperPrerequisiteClosed`; publication review remains qualified in that package. The Groth16 design annex's G3 “unestablished” disposition must therefore be read as **partial: captured local wrapper prerequisite adopted, authenticated target issuance unestablished**. This correction does not rewrite the original annex, infer instrumented guard identity, qualify ledger ExUnits or establish claim consumption/settlement. Do not rerun this accepted local prefix merely to repair the report.

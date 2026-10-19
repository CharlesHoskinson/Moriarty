# Optimization design

Status: DRAFT / PROPOSED. Source reference `35931362ca435b2ed10014ee052870716c8f10d3`; exact source digests and per-clause consumers are in [traceability.json](traceability.json). The specification is additive to revision03 and the existing recursive experiment.

## Public source basis

`reference.rs::Machine::step` publishes Return/Error before zero-fuel exhaustion and emits 39 event IDs with 36 charged and three free events. `return_publication_step.rs` and `error_publication_step.rs` duplicate the observed utility block. `literal_host.rs` supplies typed schema/layout and source identity machinery; `layout_a/encoding.rs` explicitly provides ordered scalar bytes, not native statement binding. `sherlock_link.rs` has separate A102/A80 layouts and binds source text into relation/context identities. `sherlock_child.rs` is the child statement/proof consumer. These public concepts support this draft; no private source, architecture or research body is reproduced.

## Alternatives and sequence

| Alternative | Benefit to evaluate | Cost and retained obligations | Proposed disposition |
|---|---|---|---|
| Retain isolated leaves | Preserves the current integration and comparison baseline | Duplication remains; no migration cost | Required reference and first recursive stagenet delivery |
| Share source utilities and typed mechanics | Lower maintenance if net maintained lines fall after adapters and validation | Ordered operations/ABI/errors/full state must agree; affected source/context/key identities still change | Recommended first; prepare in isolation, integrate after observed baseline |
| Physical constraint sharing or final fusion | Possible constraint, proof or serialization reduction | Exact accepted-relation/activation evidence, fresh keys and actual cost comparison; k/RSS may increase and fused parents lose reuse | Separate optional measured experiments |

No foreign VM, new ISA, authenticated-memory redesign or compiler rewrite is selected. No speedup or k threshold is invented. Maintenance acceptance is distinct from performance adoption.

## Source sharing boundary

Move only common constructor/emitter mechanics first. Preserve the sequence of `Instruction` operations and compare with independently frozen old-emitter outputs as a pre-change equivalence baseline, not a semantic-correctness oracle, under an explicit checked bijective renaming of temporary identifiers. The renaming is the identity on every non-temporary identifier; its image lies inside the reserved temporary namespace enforced by `TemporaryNameCollision` and is disjoint from every input operand. Do not reorder conjunctions or remove predicates as part of this extraction. Preserve construction errors and refusal locators, including the `expect("fixed L4 column")` panic path, ABI, all complete images and statements. Preserve `is_reserved_{return,error}_publication_variable` and the `%{scope}.local.temp.` prefix or record any change as an explicit ABI change. Count old/new total maintained lines across helpers, both emitters, adapters and validation code; 132 duplicated lines are not automatically 132 net lines saved.

## Typed layout and plans

Share typed declarations/walkers only where public consumers already describe the same fields. Inventory program/pre/post/envelope/context paths, capacities, widths, booleans, maximums, active/inactive padding, domains and provenance. Constraint generation, witness scheduling and export remain separate plans. Child, parent and terminal have separate relations and consumer obligations even where declarations agree. Independently frozen expected images and bytes must not be produced by the shared production descriptor. Common code cannot provide its own independent oracle.

## Organizational rule families

[rule-families.json](rule-families.json) uses the proposed terminal (T), value/environment read (V), expression scheduling (S), pending push (P), continuation (F), primitive/result (O), argument/call (C) and ordered pattern (M) families. All 39 event selectors remain distinct. Guards, deltas, priorities, frame actions, first failures and 36+3 charges are retained. Eight families are a maintenance choice, not eight physical circuits or proofs. This map is a concrete proposed organizational choice.

Every optimization that changes a transition slice covered by UIREQ-032/033 and SCON-014/015 must retain baseline group widths 1/2/4/8, earliest stop and bounded static residualization, even if the candidate omits their implementation. Preserve every intermediate complete boundary, including pending returns, frames/environments, helper cursors, terminal-before-fuel, observer-zero/no-next, integer/range and Bool distinctions. A/B logical work and host diagnostics remain distinct from circuit and prover costs.

## Physical experiments

Same-cell range/preservation sharing may remove a predicate only with the existing owner's exact cell/activation/dominance and accepted-relation evidence at every intermediate boundary. Equal honest witnesses or equal final outputs alone are insufficient. This draft carries that existing owner obligation and supplies no replacement security analysis.

Optional final-parent/terminal fusion preserves the child statement, existing application predicate, accumulation discharge, financial/communication duties and exact ledger-call binding. It creates a separately qualified relation/key candidate. Compare one envelope and repeated envelopes with reusable parents: avoiding one intermediate proof can be outweighed by larger terminal k/RSS or repeated work. Fusion is not unconditional, and equal public vectors do not alone establish replaced operation duties. Here accumulation discharge means the terminal fold and pairing check required for parent-accumulator validity, as recorded in the public [child source obligations](../../../crates/mori-backend/src/sherlock_child.rs). Communication duties mean the selected ledger operation's call-input and communications-commitment obligations: the public [transaction consumer](../../../crates/mori-ledger/src/sherlock_transaction.rs) verifies these before sealing; the child's current LocalPublic relation without communications is a separate scope.

## Baseline and measurement

Integration means any change within the traced source/context identity membership (SOPT-010) of the native recursive child, parent or terminal path; changes outside that membership are unaffected. Before integration, preserve an actual Midnight stagenet recursive child/parent/terminal baseline under the existing [experiment](../../../docs/sherlock/stagenet-recursive-rollup-experiment.md). This is not a claim that such a baseline has already been observed. Isolated source preparation may proceed first. A file, feature, setting or context-input change within an artifact's traced identity membership requires fresh identities and applicable qualification for that affected artifact, including logging-only or expected-equivalent changes. Artifacts whose traced membership excludes the change retain their identities and qualification status.

Use the existing experiment-selected manifest, not a new workload schema. Current source supports full 16-transaction batches and the current six-batch default is 96 transactions. Retain reduced fixtures and unsupported shapes with their own names. Match public seed, input program source, ordered workload and counts, caps, profile, cache state, threads, host hardware/OS identity, build/toolchain identity and target. Baseline and candidate implementation/circuit source identities may differ; record both identities. Deterministic inputs do not promise deterministic proof bytes.

Report child, parent and terminal separately, with parameter loading/generation, prepare/witness/serialization, keygen, prove and independent verify wall/user/system CPU and peak RSS. Report actual k, domain capacity separately from used rows, columns, lookup rows, maximum gate degree separately from k, proof/VK/full transaction bytes and toolchain/context identities, with actual sample count and raw per-sample measurements. Separate submission, inclusion, finality and full settlement readback. RPC elapsed does not measure validator CPU. Each unavailable metric remains unknown with its reason; no invented threshold can turn it into success.

## Correctness and disposition

Use existing Clean, finite Quint, K and handwritten lanes on the same independently specified slice with complete-state/prefix expectations, profile adapters and explicit scope gaps. Local CI only; no new framework or hosted workflow. This draft runs none of those lanes.

Each candidate starts proposed, reaches validated only for evidenced predicates, and then receives retain/reject/defer with exact source/results and limitations. Source cleanup may be retained for maintenance after its own equivalence validation while performance adoption stays open. Such a retain disposition within the SOPT-001 integration membership remains isolated until that baseline is preserved; changes outside that membership are unaffected. Preserve measured regressions, first divergence, terminal failures and the original baseline. No SOPT count closes retained Sherlock or roadmap requirements.

# MIL/2 deep research sprint: obligations before a language freeze

**Prepared:** 2026-09-29. **Baseline:** Moriarty `983a4bb49e3ccae399ee2514da2f6fa3f03593fd`. **Status:** research draft and graph construction; no MIL/2 design adoption or U0 closure.

## Decision this sprint must enable

Determine which MIL/2 constructs can be included in the first signed-intent profile with precise semantics and a feasible route to native enforcement. Produce a corrected, reviewable MIL/2 candidate or explicitly defer any construct whose safety argument remains open. The [initial findings](RESEARCH-FINDINGS.md) identify three immediate corrections and four proof-boundary questions.

## Fixed scope

Research the six obligations in [MIL/2 §17](../../concepts/intent-language/DESIGN-MIL2.md): total Φ evaluation, evidence source preservation, encumbrance accounting, acceptance refinement, conditional recovery viability, and derived-footprint containment. Include Φ₀ decidability and the ZKIR witness/caller boundary because they determine whether the proposed proof routes work. Use the escrow, funded repayment, and contrasting transfer as concrete programs.

Exclude a broad DeFi category survey, new application libraries, a federation service, live financial submissions, and a claim of formal proof based on paper analogies. The existing category map was derived from MIL/1 and requires separate re-derivation after MIL/2 stabilizes.

## Inputs already prepared

- [15 Scrapling captures and digests](sources.json), from three rounds of five public primary-source pages. Captures are under `source-text/`. GitHub ZKIR references are pinned to `47793c8ab042aa5a91d1a4672c6b82de6bdf9dd8`; the IBC `main` URLs are time-stamped captures, not pinned repository snapshots.
- [Research findings](RESEARCH-FINDINGS.md), with repository observations, source facts, and inferences separated.
- [Graphify report](graph-corpus/graphify-out/GRAPH_REPORT.md) and `graph-corpus/graphify-out/graph.json` map 26 nodes and 37 inferred links across three communities. Luna selected the semantic nodes and links. Treat graph edges as navigation and traceability, not independent evidence of a theorem.

The current evidence is enough to select work. Some captures are landing pages or abstracts; the next reading pass must obtain full technical definitions before making theorem-level claims from them. Network work is limited to public primary material. Keep raw captures, hashes, requested URL, retrieved date and any access limitation.

## Work packages and exact exits

| Order | Question and action | Required artifact | Exit condition |
| --- | --- | --- | --- |
| 1 | Specify Φ₀ grammar, widths, errors and authoring-check fragment. Separate QF_IDL guards from general QF_LIA guards. | `phi0-semantics.md`, accepted/rejected expression table, bounded cost record | Every construct has a typed evaluation or named `Reject`; the escrow implication has a sound procedure for its admitted grammar; unknown/timeout rejects; a legal non-IDL guard exposes the old claim. |
| 2 | Give templates with holes a denotation and define completion refinement. | `completion-relation.md`, positive and hostile filling corpus | `Accepted(I)` is a closed, typed set of traces; each permitted filling refines it under an explicit trace projection; wrong recipient, widened budget, weaker evidence and changed recovery rights reject. |
| 3 | Define evidence identities, source-set effects, observations and native binding. | `evidence-relation.md`, operator propagation table, source-to-target field map | Every operator preserves the relevant provenance; imported data cannot satisfy an anchored-only position; time, issuer/status, domain, value and digest are bound at a named enforcement locus. A paper label or host Boolean does not count as that binding. |
| 4 | Define transfer/guard footprint derivation, linear receipts, and encumbrance lifecycle. | `resource-and-footprint.md`, derived showcase footprint, concurrent-candidate examples | Derived read/write sets contain every actual cell, including the MIL/2 showcase branches; fork/join cannot duplicate receipts or shared authority; committed plus reserved locks fit balances and preserve debts. |
| 5 | Separate local escrow safety from conditional cross-domain recovery. | `escrow-transition.md`, explicit late-result and partition traces | A funded pending state is admitted; release/refund are mutually exclusive through a persistent tombstone; timeout alone cannot justify foreign nonexecution; every claimed recovery guarantee names its liveness assumptions and evidence. |
| 6 | Reconcile the first signed stage with pinned ZKIR and U0. | `target-obligations.md`, field-by-field enforcement map and risk register | The intent digest, evidence, effects, predecessor, nonce, failure policy and tombstone each have a concrete proposed enforcement locus and caller obligation; unsupported or unavailable interfaces remain marked open. |

Each package ends with a source ledger row, one positive case, at least one adversarial case, and an explicit result label: `specified`, `checked locally`, `proved`, `native measured`, or `Preview observed`. Do not promote a label merely because a graph path or review agrees with it.

## Review order and stop rules

First correct the Φ₀ decision-procedure claim and the showcase footprint; both are visible in MIL/2 without new backend work. Next settle the denotation of an unfilled intent, because the completion theorem depends on it. Then examine the evidence and recovery relation against the source graph and the pinned target. Keep the five prior owner decisions as provisional design choices until the revised candidate has had a fresh independent review.

Stop a package if a required native interface is unavailable, if the claimed property depends on an unspecified trust premise, or if a counterexample refutes the candidate rule. Record the counterexample and narrow or defer the construct. The sprint closes when all six obligation dispositions and the Φ₀/native boundary are reviewable; this does **not** mean that U0 or MIL/2 has been accepted.

## Research budget and handoff

The preparation used the default bounded research budget: three search/fetch rounds, five captures per round, 15 captured pages total. No canonical wiki merge is included. A later canonical merge would need a separately inspected Obsidian transaction and source/claim ledger updates. The next technical reading pass should prioritize full author papers for information flow, effects and linear contracts, the actual ACTUS rule set rather than its index, and pinned IBC revisions. Retain the current corpus and graph as the research handoff.

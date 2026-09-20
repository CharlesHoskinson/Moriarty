# Closure review: consolidated design v2 and Mina refinements

Verdict: **approved — FINAL CONSOLIDATED DESIGN ONLY**.

Scope: the eleven frozen file payloads in `review-candidate-v2.txt`, identified by `candidate-v2-manifest.json`, plus the appended Mina RESULT, recursion-semantics and developer-integration reports. This is an independent delivery/financial-language design review of the newly expanded scope. It does not approve implementation, source security, native feasibility, proof correctness, deployment or release completion.

I read the entire candidate and all three appended Mina reports. All eleven reconstructed file payloads match the manifest's exact byte counts and SHA-256 hashes. I checked the presence of MPLR-001..035, ZR01..16 and MNR01..08 in the traceability register and compared the named operators with the actual inherited SP10.1 specification. I refreshed read-only development status; the existing operational/accounting blockers persist and no pending public transactions were returned. No candidate/canonical files were edited and no builds, proofs or deployments were performed.

## Prior findings closed

**DREV-01 is resolved.** ROADMAP, One implementation sequence, now requires accepted SP07/SP08 semantic inputs for the claimed successor scope. Its following paragraph distinguishes early U4 subset qualification from full SP09/supported-language MC05, permits semantic preparation before final U6 conformance, and assigns later semantic extensions and affected source/Core/native/consumption/mandatory-acceptance requalification to U6. The corresponding MC05/SP09 traceability rows agree. This preserves the inherited acceptance prerequisites without introducing a cycle or postponing all useful U4 work.

**DREV-02 is resolved.** `requirements.md` now incorporates UNI-001–017, and UNI-017 expressly includes ZR01–ZR16 and MNR01–MNR08.

No required corrective findings remain in this reviewed scope.

## Delivery and financial scope

The U0–U7 schedule remains the single execution sequence. P/C/K records are provenance/detail aliases, not competing queues. All MC01–MC08 and SP01–SP12 retain acceptance mappings, with G01–G24 expressly retained under release. The ACTUS fixture/field, executable-type, taxonomy/source-gap, original DeFi-row, supplemental-case, held-out, intent and regression denominators remain explicit. Grouping by library or phase does not establish a passing row or authorize denominator reduction.

The exact five inherited SP10.1 operators are preserved under U4: **sequential, disjoint parallel, shared-state interleaving, atomic synchronization and asynchronous messaging**. The original file also requires read/write compatibility, explicit ordering and a partitioned global finite work measure; the candidate incorporates the inherited definitions rather than substituting DeFiFormal's distinct operator catalog. Scope remains conditional on actual later evidence.

U0 now owns the canonical price orientation, dimensions, rounding direction and beneficiary policy. U1 certificates bind that numeric profile and its cost relation. UNI-014 correctly distinguishes exact conversion from an unjustified reciprocal of a previously rounded value. Gross debit, fees, minimum net outcomes, liabilities, authority and residual duties remain separate. No source model or token-conservation equation is presented as full financial correctness.

The first general source-to-native slice and the extended conditional exchange remain useful non-vacuous delivery discriminators. U3 permits independently authored candidates without U5 infrastructure. U4 requires separate-party handoff and composition without the federated kernel. This demonstrates optionality at the acceptance boundary rather than merely describing the federation as optional in an introduction.

## Native future and current evidence

The approximately March 2027 comprehensive-recursion horizon is consistently a dated user planning assumption. Full native recursion, portable compliance and private multi-parent composition remain required targets. Ledger induction cannot close MC03/MC06. The target distinguishes bounded per-stage computation and fan-in from growing finite ancestry, while preserving signed episode/lifetime budgets and authorized successor transitions.

Current-source and historical-interface claims remain qualified. U1 must determine actual released support; no present capability is inferred from the six-month forecast. PR17 witness-shape/backend premises, complete final verification, authoritative current-state consumption, signature boundaries and source-to-ledger correspondence remain obligations. Distinct source/Core/ZKIR/key hashes require an authenticated correspondence relation rather than literal equality.

No Lean dependency, foreign proof backend, new curve choice or Mina port is introduced. Midnight's native Halo2-derived PLONK/KZG and pinned ZKIRv3 remain the governing target.

## Mina refinements are appropriate and evidence-grounded

The appended studies provide bounded source observations and concrete motivations for all eight added clauses:

| Refinement | Why it belongs in the existing contract |
|---|---|
| MNR01 | Actual absorption order, challenge context and cross-field encodings refine ZR01/03/06 beyond a mere list of public fields. |
| MNR02 | Step/wrap deferred checks and masks motivate a producer/consumer invariant for every active obligation, legitimate base selection and padding. |
| MNR03 | Dynamic key integrity, compatible shape/features and authorized relation identity are separate obligations; same-shaped weaker relations must not pass policy. |
| MNR04 | Parsing/readable proof fields and auxiliary outputs do not establish verified history or actual effects; state preconditions supply a distinct currentness link. |
| MNR05 | Explicit disabled-proof/dummy modes motivate real-mode assertions and independent final verification, without alleging a deployed Mina bypass. |
| MNR06 | Cache and parameter provenance require authenticated identity or proved compatibility; equal dimensions alone are not the argument. The source TODO remains a motivation, not an exploit claim. |
| MNR07 | Randomized batch equations, cardinality and direct fan-in require a soundness contract and preserved duty/budget accounting under tree decomposition. |
| MNR08 | The older audit's restricted scope motivates a component-by-component specification/constraint/verifier assurance matrix; it cannot qualify current unreviewed gates or versions. |

These are necessary refinements of ZR01–ZR16, with ownership and positive/hostile evidence under UNI-017. They do not create another work queue. Batch acceptance is explicitly subject to a stated probabilistic security guarantee and proof argument; tests are not presented as a universal soundness proof. Sound order-insensitive batching remains permitted.

The evidence boundary is honest: the study reports source/hash/excerpt inspection, bounded documentation acquisition and a historical audit reading, not executed proofs or an integrated build. The acquired Mina/proof-systems gitlink match is preserved without asserting compatibility of the entire independently acquired tuple. Historical audit scope is not transferred to the 2026 heads. I evaluated the supplied source reports and their claimed scope; I did not independently reproduce every cryptographic source claim during this design closure review.

I authored the separately bounded navigation graph in the preceding delegated task. That graph contribution is disclosed here and is not used as independent security/proof evidence for this approval. Its extracted versus inferred labels and OCaml textual-resolution limits are consistent with the candidate's navigation-only description.

## Public authority and internal workflow

UNI-001 and the design/workflow boundary preserve permissionless public authoring, compilation, proving and deployment. Application consent and objective supported semantics still constrain effects. No project review receipt, solver account, federation membership or Pel record becomes public validity input.

The recovery contract now states signed termination/revocation rules and exclusive terminal consumption without imposing perpetual authority. Revocation cannot erase duties; knowledge reconciliation does not authorize a fresh transfer. The optional federation's membership/threshold/ordering/equivocation/availability/epoch policies are explicit service trust conditions, with a required migration test preserving duties and consumed authority. They do not redefine the language's deployment authority.

Pel remains a bounded symbolic internal template: initial candidate, one correction, no review after failed verification, and a non-completion stop. Actual roles, candidate schema, resource/accounting state, immutable inputs and real executable commands must be supplied before use. Missing target commands stay specified-only/interface-blocked. That is an honest design boundary, not a defect requiring runtime implementation before document approval.

Approval applies only to the exact manifest reviewed and the stated consolidation scope. All implementation milestones, backend qualifications, financial conformance, independent proof/ledger evidence and full release acceptance remain open until their own predicates are demonstrated.

## Narrow post-freeze Pel addendum

Root reported that deterministic Pel checking rejected the invented `artifact:consolidated-execution-packet` identifier. I inspected the complete two corrected candidate files and their diffs against frozen v2. The only changes are the input slot to `artifact:approved-spec` and corresponding workflow text requiring that installed slot to be bound to the concrete consolidation execution packet. The bounded verification/review/correction logic and all product semantics are unchanged. **Approved for design consolidation** with these two replacements; no static check is claimed independently rerun by this reviewer. Root reports check/plan passing; this addendum judges the inspected correction, not a newly executed lane.

Exact replacement hashes:

- `openspec/changes/consolidated-language-kernel/implementation.pel`: SHA-256 `fc8a5562e088524e8113961ad06f6851edabdc388cffdea13a531cb137c388f9`.
- `openspec/changes/consolidated-language-kernel/workflow.md`: SHA-256 `34a5552c66061ad21a77fd3cc0bfe1edbb2f46e71626860ae411bee30080c79f`.

# Moriarty 90-day DeFi Kernel research sprint

<!-- markdownlint-disable MD013 MD060 -->

- Date: 2026-09-03 UTC
- Status: superseded by `moriarty-sprint-program-2026-09-03.md`
- Decision target: fund a Moriarty language vertical slice, choose audited Compact
  libraries instead, or stop

> This calendar-based plan is historical. The active plan uses evidence-gated
> sprints. No calendar duration contributes to completion.

## Outcome

At day 90, decide whether Moriarty has a measurable assurance and review
advantage over an audited Compact library. The sprint is not authorized to call
Moriarty a production DeFi Kernel. It must close or explicitly reject the
smallest semantic motions, complete one real proof path, and give all 72
DeFiFormal rows a bounded instance or an honest outside-kernel result.

The first stop test is deliberately small: a Marlowe-shaped finite fragment
with accounts, deposit, bounded choice, timeout, and atomic two-token swap must
generate Compact without an unbounded container or unconstrained witness
callback. It must also produce a client-checkable artifact/disclosure manifest,
a translation-validation certificate, and 1,000 coverage-guided differential
traces. Failure moves the project to audited libraries or stops it before the
Core grows.

## Entry evidence

- DeFiFormal is pinned at
  `8ae0bbfaa3193078d1cabf6999db1382985b7f95`.
- The 72/60 rosters, 1,259/570/689 obligation totals, 61/72 eligibility,
  1,830/185 pair results, 29/72 canonical compression, four collisions, and
  22/22 v3 self-test are locally reproduced.
- 1-NN 47/72, 3-NN 50/72, and the selected Jaccard values are independently
  reproduced with deterministic tie rules. Hierarchical ARI still needs an
  independent harness.
- One escrow has compiled through the pinned Compact/ZKIR tuple and three ZKIR
  circuits passed the mock compiler. Full proof generation, ledger execution,
  correspondence, cost, and audit remain open.
- The agentic panel recommends a gated revision. M2+M3 remains a candidate human
  taxonomy, M5 the internal formal profile, generated Compact the first backend,
  and audited Compact libraries the planned fallback.

## Work packages

| Window | Package | Concrete output | Gate |
|---|---|---|---|
| Days 1-5 | Backend stop test | finite Core fragment, generated Compact, static bound inspection, manifest, 1,000 differential traces | no unbounded collection or unconstrained witness; zero divergence |
| Days 1-15 | Evidence and taxonomy | full Jaccard, clustering/ARI/NMI, exact composition matrix, 689-row residue classification, genuine rater protocol | all calculations reproducible; simulated ratings excluded from agreement claims |
| Days 6-25 | Marlowe delta | construct-by-construct semantics and proof-premise matrix; correction of token, party, input, time, partial-pay, and Merkleization claims | every claimed inheritance has an exact theorem or is marked new proof work |
| Days 15-35 | Semantic motions | decisions on numbers, partial pay, atomic action collection, Attest, authority, mandate, split/merge, and state bounds | one accepted semantics per motion before Core freeze |
| Days 20-45 | Normative assurance choice | executable comparison of Isabelle, Agda, Lean, reference interpreter, and differential implementations | select environment and theorem set or choose library-only |
| Days 25-55 | High-risk composition | binary conditional token plus atomic exchange, invalid oracle, replay, timeout, split/merge, and refund paths | conservation and authorization proofs/checks; deterministic build |
| Days 30-65 | Seven demonstrations | F1-F6/P source, Core, Compact, manifest, client checks, and outside assumptions | seven compile; no full-protocol overclaim |
| Days 35-70 | 72-row coverage | parameterized bounded instance or outside-kernel manifest for every row; retain 13 legacy regressions | 72/72 reviewed rows; unsupported behavior named |
| Days 40-75 | SDK and malicious planner | TypeScript surface API, artifact verifier, disclosure display, state/intent verifier, corrupted-plan suite | every substitution rejected before signing |
| Days 45-80 | Real proof and cost | real proving/verifying path, constraints, keys, proof size, memory, latency, state and transaction costs | product budgets set from measured distributions, not guessed |
| Days 60-85 | Independent audit and user comparison | scoped audit, library-versus-language review study, two pilot specifications | no critical/high findings; measured advantage or fallback |
| Days 80-90 | Council and decision | blinded cross-provider critique, dissent register, scorecards, signed decision record | proceed, library-only, or stop; no ambiguous “continue exploring” result |

## Staffing and planning range

Use 7-9 full-time equivalents for the feasibility sprint: two language/compiler
engineers, two formal-methods engineers, two Compact/Midnight engineers, one
SDK/security engineer, one DeFi/domain and taxonomy researcher, and one product
researcher. Independent audit and prospective-user review are part-time and
outside the authorship chain. Planning range is USD 650,000-1,050,000 for the
90-day phase, including contractor and audit contingency. This is a staffing
estimate, not a vendor quote.

## Armed research loop

Each loop cycle is bounded and evidence-producing:

1. Read the current goal, wiki index, open questions, contradictions, and graph.
2. Select one decision-changing evidence gap; record its acceptance predicate.
3. Use Scrapling for all web search and acquisition; use Git or structured APIs
   for pinned repositories after discovery.
4. Preserve raw input, receipt, digest, date, version, authority, and scope.
5. Inspect code and artifacts; reproduce quantitative claims with deterministic
   commands and preserved outputs.
6. Update the smallest wiki concepts, claim ledger, contradiction register, and
   graph.
7. Add a failing test before any new harness or behavioral change; implement;
   rerun focused and full verification.
8. Ask independent council members only at architecture or commitment
   boundaries. Verify provider/model terminal evidence; record failures as
   abstentions.
9. Apply the current stop and release gates. If a stop fires, write the fallback
   decision instead of silently widening scope.
10. Checkpoint the exact state and start the next highest-value cycle.

The loop is armed by the active Moriarty research goal and the executable
artifacts in this repository. It does not authorize uncontrolled network access,
deployed transactions, external messages, or silent changes to the trusted Core.

## Day-90 decision rules

Choose **Moriarty vertical slice** only if the backend stop test, semantic freeze,
proof-environment choice, high-risk composition, real proof path, client
verification, scoped audit, reproducible artifacts, and two user-supplied pilot
specifications pass.

Choose **audited Compact libraries** if bounded templates, manifests, and client
verification deliver the same measured review and assurance benefit without a
new semantic layer, or if Core-to-Compact correspondence is not maintainable.

Choose **stop** if the backend violates static boundedness, any conservation or
authorization failure is reproducible, artifact substitution is accepted,
privacy claims require undeclared disclosure, real proof cost is outside the
agreed budget, or no qualified team can maintain the normative semantics.

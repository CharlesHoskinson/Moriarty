# Moriarty DeFi Kernel research-prompt council brief

Date: 2026-09-03 UTC

This is a tool-free advisory task. Do not browse, edit files, run code, or claim
independent verification. Work only from the evidence summary below. Identify
which claims the later research sprint must reproduce from primary sources.

## Decision to improve

Develop a decision-grade XML deep-research prompt for Moriarty: a Marlowe-like,
strictly bounded financial-agreement language that generates reviewable Compact,
then uses the pinned Compact compiler to emit TypeScript, ZKIR, and Midnight
proof artifacts. Moriarty is intended to supply safe DeFi primitives whose
obligations are informed by the DeFiFormal corpus. It must not become a
general-purpose or Turing-complete language.

## Reproduced local evidence

- DeFiFormal commit `8ae0bbfaa3193078d1cabf6999db1382985b7f95`
  contains 72 named protocol rows and a 60-construction suite.
- The 60 construction specs contain 1,259 obligations: 570 covered and 689
  residue. No construction has a complete verdict.
- 61/72 element sets satisfy the repository laws and warrants.
- Among the 1,830 pairs of those 61 eligible protocols, 185 fail composition:
  3 of 143 same-category pairs and 182 of 1,687 cross-category pairs. The exact
  failure rates are 2.10% and 10.79%, a 5.14-times association.
- Deterministic Jaccard leave-one-out classification reproduces 47/72 for 1-NN
  and 50/72 for 3-NN. D12 is 0/5 under 1-NN.
- 29/72 protocols have a smaller canonical element form; four identical-form
  groups are reproduced. The v3 self-test passes 22/22 assertions.
- The updated human taxonomy proposal is M2+M3: F1 exchange/price discovery,
  F2 credit/collateralized debt, F3 derivatives, F4 consensus-position claims,
  F5 tokenized off-chain claims, F6 delegated asset management, and a small
  Prediction family, with mandatory execution, settlement, custody, legal,
  collateral, oracle, authorization/mandate, and price-discovery facets.
- The proposed internal profile is M5: actors, transitions, obligations,
  conservation laws, liveness dependencies, and failure modes. The older M4+
  product/instrument/mechanism/trust crosswalk is retained only as a migration
  and benchmark artifact.
- Strong residue candidates are a first-class party/authority sort, bounded
  mandates, a strategy level, and conditional-token split/merge. Legal recourse,
  reserve custody, business calendars, oracle truth, bridge relaying, solver
  auctions, and arbitrary external code are not proved by a financial kernel.

## Reconstructed upstream facts to challenge

- Marlowe V1 Core is finite and centers on `Close`, `Pay`, `If`, `When`, `Let`,
  and `Assert`, with `Deposit`, `Choice`, and `Notify` actions. It has value
  conservation, termination, quiescence, and closure-related proof results with
  explicit premises, but the formal, Haskell, TypeScript, Plutus, Runtime, and
  deployed layers are not one proved artifact.
- Marlowe lacks native token-indexed units, a typed modular source language,
  generalized atomic action sets, a general authorization abstraction, and an
  availability protocol for Merkleized continuations.
- Current Moriarty work has compiled one finite escrow through Compact compiler
  0.34.100, language 0.26.0, runtime 0.19.100, ledger 9.1.0.0-rc.3, and ZKIR 3.
  Three circuits passed a mock compiler. This is S3 feasibility evidence only:
  no full proof-generation benchmark, ledger deployment, audit, or semantic
  correspondence proof exists.
- Compact has static circuit structure and fixed loops, but containers and
  off-chain witness callbacks need Moriarty-level bounds and constraints.
- Direct Moriarty-to-ZKIR compilation is deferred because the current ZKIR
  interface is evolving and ledger-coupled. Generated Compact remains readable
  and uses the supported compiler path.

## Required council response

Give an independent, concise proposal with these exact sections:

1. `Verdict and strongest dissent` — recommend, revise, or reject the proposed
   M2+M3/M5 and generated-Compact direction. State the best no-redesign case.
2. `Taxonomy corrections` — missing families, invalid family/facet boundaries,
   multi-membership rules, promotion thresholds, and temporal maintenance.
3. `Marlowe-to-Moriarty boundary` — what belongs in semantic Core, typed source,
   libraries, SDK/Runtime, capability protocols, or applications.
4. `Seven canonical demonstrations` — the smallest representative application
   per F1-F6 and Prediction, with safety property, required external assumption,
   privacy value, and boundedness rule.
5. `Compact-to-ZKIR assurance` — compiler/translation boundary, privacy and
   disclosure rules, proof/cost gates, artifact identity, client verification,
   and upgrade risks.
6. `Research prompt improvements` — mandatory sources, experiments,
   falsification criteria, status/provenance fields, deliverables, and numeric
   release gates the final XML must contain.
7. `Top ten questions` — questions that could change the architecture or stop
   the project.

Do not average away uncertainty. Mark facts, inferences, proposals, and unknowns.
Do not imply that a circuit proves oracle truth, asset custody, legal claims,
participant liveness, economic soundness, or arbitrary external-script safety.

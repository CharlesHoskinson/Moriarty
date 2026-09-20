---
title: Solvers — search is separate from authority and correctness
diataxis: explanation
status: research-draft
type: research
created: 2026-09-19
updated: 2026-09-19
tags: [moriarty, apss, research]
---
# Solvers: search is separate from authority and correctness

A solver constructs candidate actions satisfying an intent. It may match counterparties, route through liquidity, batch orders, choose timing, or optimize costs. CAKE puts this work between the permission and settlement layers. Winning an auction does not itself authorize asset use or establish final settlement. [CAKE](https://frontier.tech/the-cake-framework), [OFA design](https://frontier.tech/the-orderflow-auction-design-space).

## Three distinct meanings of solver

An economic solver searches execution plans. An SMT solver decides a logical formula within an encoding. A program synthesizer searches program terms. Moriarty may use all three, but their outputs have different meanings. A route proposal is not an SMT proof; an SMT result is not proof that the compiler or ledger implemented the formula; a synthesized program must satisfy the same checks as hand-written code. This is a Moriarty design distinction, informed by the inspected Aeon implementation and the sources below.

Anoma describes solving as matching unbalanced transactions into valid balanced transactions. Its resource account separates hard predicates from preferences, and distinguishes an intent from one concrete transaction representation. That is useful for defining Moriarty proposal interfaces without fixing a single search algorithm. [Intent machine specification](https://specs.anoma.net/main/system_architecture/state/intent_machine/index.html), [resource perspective](https://anoma.net/blog/intents-rm).

## Feasibility, quality and execution are different claims

A candidate can meet a user's floor without being the best available trade. CoW's mathematical description represents acceptable trades as a set and quality as surplus; it then separates routing from bidding. Its model includes non-fill as an acceptable outcome for its supported orders. Moriarty must not generalize that assumption to every financial obligation: a debt payment deadline and an optional swap have different meanings. [CoW problem](https://docs.cow.fi/cow-protocol/reference/core/auctions/the-problem).

Angeris, Chitra, Evans and Boyd formulate CFMM routing as convex optimization under stated assumptions, then add fixed execution costs through mixed-integer activation variables. Their discussion explicitly leaves actual execution sequencing to the underlying chain. A continuously optimal portfolio delta is therefore not an executable, integer-safe, fee-inclusive Moriarty trace. Rounding, intermediate solvency, changed reserves and protocol rejection need independent treatment. PDF pages 1, 5–7 and 12 were visually read. [Routing paper](https://arxiv.org/abs/2204.05238).

A useful Moriarty hierarchy is: supported candidate; semantically valid candidate; candidate satisfying signed hard constraints; candidate with measured or proved objective quality under a snapshot; candidate actually finalized. None of these labels should silently imply the next.

## Open participation is not universal across existing systems

CoW's overview calls the trading protocol permissionless. Its detailed competition rules nevertheless require whitelisted settlement solvers, distinguish on-chain checks from off-chain checks and social rules, and describe discretionary penalties. Its bonding documentation requires team/governance involvement. These claims describe different participation surfaces and should coexist in the literature bank. They are a caution against importing the word permissionless without naming who may do what. [Overview](https://docs.cow.fi/), [rules](https://docs.cow.fi/cow-protocol/reference/core/auctions/competition-rules), [bonds](https://docs.cow.fi/cow-protocol/reference/core/auctions/bonding-pools).

UniswapX's July 2023 paper instead presents permissionless fillers and reactor validation of signed orders. It allows optional brief exclusive filler rights during price discovery and treats RFQ reputation policy separately from its core. This is a dated design, not a claim about every current deployment. PDF pages 1–4, including the single-chain and cross-chain diagrams, were visually read. [UniswapX paper](https://app.uniswap.org/whitepaper-uniswapx.pdf).

Moriarty's public language must not require membership in either kind of market. An application can offer a solver marketplace or bounded user-selected exclusivity. Those policies remain explicit application constraints; they cannot become permission from Moriarty maintainers to deploy a program.

## Information, inclusion and adverse incentives

OFA design distinguishes information shared with bidders, winner selection and eventual inclusion. Revealing a private strategy can create value for an observer even if they never execute it. A proof of correct settlement does not retroactively protect disclosed intent. Conversely, encrypted intent may reduce the information needed to find good solutions. [OFA design](https://frontier.tech/the-orderflow-auction-design-space).

CoW's detailed fairness mechanism uses directed-pair reference outcomes; its short concept page is less precise. Governance-enforced best-execution expectations are not the same as a machine-verified universal optimum. The bank therefore records actual enforcement location for each rule. [Auction](https://docs.cow.fi/cow-protocol/concepts/introduction/fair-combinatorial-auction), [rules](https://docs.cow.fi/cow-protocol/reference/core/auctions/competition-rules).

## Moriarty relationship

Moriarty should express immutable hard constraints, exact arithmetic, complete effects, bounded authority and any explicit objective. Independent solvers propose; proofs and protocol rules verify. A generic candidate interface should bind intent/program hashes, predecessor state, observation versions, expiry, trace and fee attribution. Acceptance should depend on those objects, not the solver's reputation.

Aeon-style refinement checking can reject impossible or unsafe candidate fragments earlier. Typed synthesis may help construct candidates but should follow useful exact encodings and counterexample replay. Neither is a prerequisite for manual program authorship. A solver timeout means no result within the search budget; it does not prove impossibility or authorize weaker constraints.

Open questions: which objective classes admit affordable certificates; how private solving shares minimal information; how stale snapshots and partial fulfillment compose; and how to state ranking quality without promising unavailable liquidity. These are research questions, not implemented Moriarty guarantees.


Evidence archive: [captured source manifest and reading records](evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.

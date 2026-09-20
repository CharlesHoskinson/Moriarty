---
title: Audit a solver integration without trusting the solver
diataxis: how-to
status: proposed-design-procedure
type: research
created: 2026-09-19
updated: 2026-09-19
tags: [moriarty, apss, research]
---
# Audit a solver integration without trusting the solver

Use this procedure when reviewing a proposed Moriarty candidate-production interface. The output is an evidence table, not deployment approval.

1. Identify the signed hard constraints and the separately declared ranking objective.
2. Bind exact assets, recipients, gross debits, fees, liabilities, expiry and allowed partial outcomes.
3. List every candidate field and the object whose hash commits to it.
4. Locate each enforcement rule: program semantics, proof, ledger, service or discretionary governance.
5. Test the candidate through the public validation interface using a fresh developer and independent proposer identity.
6. Mutate recipient, asset domain, fee, predecessor and intent while keeping the old evidence.
7. Require rejection of every mismatched binding.
8. Replay decoded counterexamples against the pinned evaluator before reporting them as valid counterexamples.
9. Check every execution prefix for authority, costs and residual duties.
10. Separate solver timeout, invalid candidate, expired candidate and failed settlement in results.
11. Test direct submission independently of any optional relay.
12. Record what information was disclosed before execution and whether the privacy claim covers that disclosure.

For each rule record: predicate, owner, enforcement location, artifact, negative case and unresolved assumption. Do not describe a social best-execution rule as a cryptographic theorem. Do not replace absent product verification with a reviewer receipt.

References: [CoW rule boundaries](https://docs.cow.fi/cow-protocol/reference/core/auctions/competition-rules), [OFA information/inclusion](https://frontier.tech/the-orderflow-auction-design-space), [Anoma solver interface](https://specs.anoma.net/main/system_architecture/state/intent_machine/index.html). These inform the procedure; this is a proposed Moriarty audit method, not a tested external protocol integration.


Evidence archive: [captured source manifest and reading records](evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.

# Moriarty design sprint — review package

Date: 2026-09-06. Status: proposal ready for review; no new compiler, prover,
formal verification campaign or runnable mock.

**Recommendation:** a typed agreement language over a small bounded transition
Core. ACTUS and DeFi live in versioned financial packages. Intents authorize
bounded work; transaction acceptance requires proof of the financial transition
and compliant predecessor history, together with live ledger checks.

Read these in order:

1. [Target study](../../docs/research/2026-09-06-actus-defi-design-study.md): all
   18 executable ACTUS types and 32 taxonomy entries; all 72 DeFi rows mapped,
   with individual requirements and source/model gaps.
2. [Semantic design](../../docs/superpowers/specs/2026-09-06-moriarty-unified-semantics-design.md):
   three alternatives, proposed types/transitions/effects, finite bounds,
   contract properties, PCD statement and loan/swap examples.
3. [Developer interface](../../docs/superpowers/specs/2026-09-06-moriarty-developer-interface-design.md):
   screen proposal, illustrative authoring, wire objects, signing/proving flow,
   errors and mock scenarios.

The important decisions are concrete:

- Contractual **dues and actual settlement are separate**, including currency
  denomination versus settlement-token identity.
- Long-lived pools/accounts use **explicit finite epochs**; renewal is a new
  authorized contract with a checked migration, not a hidden lifetime extension.
- **Contract property certificates and transaction PCD are connected but
  distinct.** Cross-agreement composition checks each instance's program and a
  joint composition rule. Proofs do not replace oracle trust or ledger consumption.
- The initial partial-fill interface requires **a fresh signature per fill**;
  a successor state does not inherit authorization for an exact old predecessor.

The matrices account for
[all ACTUS rows](../../evidence/moriarty-design-sprint-2026-09-06/actus-32-requirements.csv)
and [all DeFi rows](../../evidence/moriarty-design-sprint-2026-09-06/defi-72-requirements.csv).
They do not mark any new package implemented or verified. ACTUS ANN initialization,
some event/calendar conventions, numeric profiles and actual Compact proof
consumption remain named evidence/implementation obligations. The study records
their dispositions rather than claiming the public source is complete.

This package completes the approved design-proposal work. Review the proposed
architecture and behavioral choices before implementing the mock or selecting
the first bounded compiler/prover experiment. The previous A4/A5 queue remains
superseded. Existing E00 and Candidate A artifacts are retained for scoped reuse.

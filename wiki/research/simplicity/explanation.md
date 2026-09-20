---
title: "Simplicity and a provable Moriarty core"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: explanation
tags: [moriarty, simplicity, research]
---

# Reference semantics before optimization

The [Simplicity paper](https://blockstream.com/simplicity.pdf), pp17–18, defines a jet by the expression it replaces: the optimized execution must preserve the expression's meaning. That is a valuable architecture for Moriarty's certified basis. It does not prove that arbitrary C, Rust, circuit or ZKIRv3 code implements that meaning. Moriarty needs reference-to-host and reference-to-target correspondence, complete side conditions, and composition evidence.

The [execution model](https://docs.simplicity-lang.org/documentation/execution-model/) separates transaction construction from validation. Anyone may propose a transaction; the program checks its conditions. It does not independently obtain documents, call a remote chain, or advance a workflow. State continuity can be enforced through successor-output commitments, while external clients maintain data availability. This supports the separation between solver/orchestrator work and public proof acceptance in Moriarty.

Bounded individual execution is compatible with indefinitely many covenant transitions. Those are different notions of recursion. The draft technical report's delegation extension also warns that late-bound code can defeat commitment-time resource bounds. A Moriarty resource claim therefore needs a fixed artifact/profile or a checked bound on every admitted late-bound component.

Witness data in Simplicity is disclosed in confirmed transactions according to the [witness documentation](https://docs.simplicity-lang.org/documentation/witness/). A witness containing a valid signature is not a zero-knowledge proof of arbitrary financial intent. Midnight privacy and PCD history compliance require separate statements and correspondence.

The recommended adoption is methodological: a small precise core, proof-carrying certified primitives, explicit acceptance conditions, bounded stages, typed temporal/evidence domains and bound program/state identities. Do not substitute a Simplicity runtime for the required Midnight ZKIRv3 target.

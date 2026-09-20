---
title: "Certified Moriarty programs and Simplicity-style jets"
diataxis: explanation
status: research-draft
created: 2026-09-19
updated: 2026-09-19
type: research
tags: [moriarty, apss, research]
---

# Certified programs, jets and ZKIRv3

Moriarty needs a reference meaning and a checked connection to the program that Midnight executes. A jet is useful when an efficient implementation preserves that meaning. Simplicity's design specifies jets using expressions they replace; its denotational transparency does not certify arbitrary native code. [Simplicity paper](https://blockstream.com/simplicity.pdf), visually read pages 1,17,18 with PixelRAG.

The recovered [September 13 workstream audit](../../../../deliverables/aeon-study-2026-09-19/jet-zkirv3-audit.md) preserves useful insights about finite primitives, resource cost and composition. It corrects claims that a shared invariant was already a proof or that no target backend had been named. A proposed count of roughly fifteen primitives remains an exploratory design, not a settled basis or proof-effort estimate.

Three relationships need evidence: reference versus host fast path; reference versus target constraints; and whole-program composition, including every caller's preconditions. Logical work, host speed and circuit cost differ. Optimizing away observable work or changing exhaustion order is a semantics change unless the versioned model explicitly allows it.

[ZKIR PR17](https://github.com/midnightntwrk/midnight-zkir/pull/17), open at head `ebb662c716fef2638ec1b0f42805a8bce23c75dd`, supplies conditional Agda results for an older 34-instruction, 13-type surface. Its module assumptions include backend primitives. Statement soundness also requires substantive `WShape` premises for the witness. A checker run only on an honest generated witness cannot establish those premises for an adversarial satisfying witness. Its conclusion is a sub-realizer with public-input agreement and memory subassignment, not equality everywhere. [Applicability audit](../../../../deliverables/aeon-study-2026-09-19/PR17-APPLICABILITY.md).

Moriarty must compile to ZKIRv3 and run on the actual pinned Midnight ledger. Source properties transfer only through proved correspondence or sound artifact validation. Neither a successful host test nor a valid proof of the wrong circuit establishes formal intention. The exact current instruction surface and deployed verifier remain separate bindings.

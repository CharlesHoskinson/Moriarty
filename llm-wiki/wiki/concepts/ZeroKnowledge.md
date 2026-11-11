---
title: "Zero knowledge end to end"
type: concept
tags: [owner-rule, zero-knowledge, privacy]
sources: [k21-recalibration-brief, sherlock-terminology]
last_updated: 2026-11-11
---

# Zero knowledge end to end

Owner ruling, 2026-11-11: "we need to make this zero knowledge or else its useless" ([[k21-recalibration-brief]]). Zero knowledge end to end is a hard requirement at the same level as GP-1 to GP-4 ([[GeneralPurposeRecursion]]). No optimization may weaken it. Before this ruling the design had no zero-knowledge requirement ([[k21-recalibration-brief]], citing the compact feasibility lanes).

## What the design must cover

From [[k21-recalibration-brief]], section 4a:

1. **Public statement.** `input_d` carries a fresh salt from the user's CSPRNG; `outcome_d` is public effects plus a hiding commitment to the private result; a private-program option makes `prog_d` a hiding commitment under the one key set.
2. **`n_steps`.** It leaks private control flow: remove it from the public statement, or publish a public bound and pad to it with constrained steps. The same rule covers leaf counts, transaction shape, transcript sizes and gas.
3. **Each proof** that leaves the user's machine is zero knowledge: blinding on, CSPRNG blinders, no witness-dependent public input, no unblinded accumulator.
4. **Elastic proving.** A helper host never receives the plain witness: leaves on the user's machine, aggregation on helpers that see only zero-knowledge proofs and hiding commitments.
5. **Determinism.** Seeded byte-identical proofs are a test mode only; production blinders are always fresh.
6. **Compact parity.** Sherlock reveals no more than native Compact: private witnesses, only `disclose`d values public, shielded state through commitments and nullifiers.
7. **Proof of the property.** A simulator argument, Lean obligations, Quint checks and named mutants.

## Status

Designed in the k = 21 recalibration round; no lane has reported. The word "zero-knowledge" joins the category name once this design is in the architecture ([[sherlock-terminology]]). Version 2 of the animation gets a privacy scene ([[AnimationV2]]).

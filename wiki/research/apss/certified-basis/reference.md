---
title: "Certified basis reference"
diataxis: reference
status: research-draft
created: 2026-09-19
updated: 2026-09-19
type: reference
tags: [moriarty, apss, research]
---

# Certified basis reference

| Artifact | Meaning | Evidence limit |
|---|---|---|
| Reference semantics | Typed values, effects, rejection, work | Requires an agreed versioned Core judgment |
| Host equivalence | Fast path preserves reference observations | Not supplied merely by naming a jet |
| Constraint soundness | Every accepted satisfying witness realizes permitted semantics | Must discharge witness-side conditions and chip assumptions |
| Constraint completeness | Every supported valid run has target evidence | Honest witness examples alone are finite evidence |
| Producer check | Source obligations enforced during lowering | Does not imply all adversarial witness conditions |
| Composition theorem | Call-site preconditions and frames preserved | Cannot assume safe reuse from individually valid gadgets |
| Deployment correspondence | Bound target matches actual verifier/ledger behavior | PR merge or version string alone is insufficient |

Primary references: [Simplicity 2017 paper](https://blockstream.com/simplicity.pdf), [2020 jets release](https://blog.blockstream.com/simplicity-jets-release/), [PR17 source README](https://github.com/midnightntwrk/midnight-zkir/blob/ebb662c716fef2638ec1b0f42805a8bce23c75dd/zkir-spec/src/zkir-v3/README.md), [Assumptions](https://github.com/midnightntwrk/midnight-zkir/blob/ebb662c716fef2638ec1b0f42805a8bce23c75dd/zkir-spec/src/zkir-v3/Assumptions.agda), [StatementSoundness](https://github.com/midnightntwrk/midnight-zkir/blob/ebb662c716fef2638ec1b0f42805a8bce23c75dd/zkir-spec/src/zkir-v3/StatementSoundness.agda).

PR17 theorem shape: `producer-WT S → satisfies (synth S) w → WShape S w → SubRealizer S w`. Scope and source locators are retained in the applicability audit. This session did not rebuild the Agda proof or certify a deployed backend.

Certificate fields proposed for Moriarty: specification hash, semantic version, types/ranges, preconditions, reference expression, optimized implementation hash, ZKIRv3 fragment/version, theorem and assumptions, composition obligations, logical-work and target-cost model, and actual evidence status. These are requirements, not an implemented format.


[Local Simplicity paper](../../../attachments/certified-basis/simplicity-paper.pdf). Scrapling captures, PR17 source reconstructions and PixelRAG page tiles are preserved under `.raw/captured/certified-basis-2026-09-19/`; visually inspected paper pages: 1,17,18.

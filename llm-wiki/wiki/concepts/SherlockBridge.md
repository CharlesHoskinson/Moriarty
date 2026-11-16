---
title: "Sherlock bridge"
type: concept
tags: [terminology, bridge, stark, wrap, zero-knowledge]
sources: [sherlock-terminology]
last_updated: 2026-11-11
---

# Sherlock bridge

Owner, 2026-11-11: "name the mathematical structure we are developing or modifying", then "yes". The names live in the terminology file ([[sherlock-terminology]], section "The bridge to Midnight"). The textbook category is proof composition with field switching (STARK-to-SNARK wrapping).

- **Sherlock bridge:** the whole structure, a small-field STARK composed with a PLONK-KZG proof over the BLS12-381 scalar field, joined at one layer.
- **Bridge layer:** the last STARK layer; its trace stays in the small field, while its Merkle commitments and Fiat-Shamir transcript use Poseidon over the BLS12-381 scalar field, with packing, a high blowup and few FRI queries.
- **Wrap:** a blinded PLONK-KZG circuit over BLS12-381 that verifies the bridge layer as a private witness and publishes only the claim. It is the zero-knowledge step ([[ZeroKnowledge]]) and the proof the ledger 9 verifier checks ([[MidnightLedger9]]).
- **Deferred field log:** a commitment to a program's BLS12-381 arithmetic (Compact `Field`, `transientHash`, Jubjub, value digests), recorded by the STARK and checked natively by the wrap.
- **STARK engine:** OpenVM stark-backend with its CUDA prover; owner: "openvm seems to be the winner". Plonky3 is the fallback.

Status: the bridge round designs the bridge layer and the deferred field log; the framework bench measures the STARK engine on this host. The chosen bridge gets a Lean proof and must be GPU friendly. See [[AnimationV2]].

Owner decision, 2026-11-11: internal digests move to `sherlock-value/2` on Poseidon2 over BabyBear ("yes for the small-field hash"); Poseidon over Fr is used only in the bridge layer and the wrap.

---
title: "Loki speed-up backlog: Lean obligations per package"
project: loki
date: 2026-10-25
tags: [loki, lean, speedups, obligations]
---

# Lean obligations for the speed-up packages

Each package says which existing theorem it relies on and which new statement it needs. Existing names
refer to `/home/charles/repos/loki-lean/LokiLean/*.lean` at `99cdba4` (synced to Loki main `8eb0cee`).
"Rust gate" means the obligation is discharged by a test, not by Lean. Package IDs match the backlog in
[LOKI-DESIGN.md](../LOKI-DESIGN.md#speed-up-backlog-from-the-scouts).

## W12 a3-residual (Moriarty side, runnable now)
No Loki prover code changes. `prove` receives identical inputs, so `Refinement.loki_proof_eq_stock` and
`loki_proof_bytes_eq_stock` are untouched.
- **O12.1 `pk-cache-equal` (new, depends on the open `keygen-deterministic` assumption).** A proving key loaded from
  the cache, bound to (VK digest, relation digest, `midnight-proofs`/Loki commit and features, `SerdeFormat`),
  equals `setup_pk(rel, vk)`. Lean: state it as `keygen-deterministic` plus `read ∘ write = id` on `ProvingKey`.
  Rust gate: serialized PK bytes equal (fresh vs cached), and proof bytes equal under a fixed rng and thread count.
- **O12.2 `degree-guard-equiv` (Rust gate only).** Replacing the in-window `cost_model(rel, None).k == vk.k`
  with the memoised `CircuitModel` from `ParentKeygenPlan` gives the same accept and reject decisions.
  Negative test: a key at the wrong k is still rejected.

## W11 extension: multiopen (in flight, files already owned by W11)
`lean-speedups` is already writing `LokiLean/Multiopen.lean`; these extend it.
- **O11.1 `divideByRoots_eq_kateFold`.** A one-pass division of `q` by `prod (X - p_i)` (midnight-zk #565
  `divide_by_roots`) equals the iterated `kate_division` fold, for every point list (including repeated points), as
  coefficient vectors. Basis: uniqueness of the quotient of `q - r_P` by `Z_P`.
- **O11.2 `prover_evals_unused`.** The prover's `multi_open` events (the f-commitment, q-evals and pi) do not
  depend on the `eval` field of `ProverQuery`. So the #554 port, where the prover passes zero evals and borrows the
  polynomials, is sound. `Transcript` event order is unchanged (`SameOrder`).
- **O11.3 (already on W11's list).** Order-preserving chunked linear combinations equal the serial
  `inner_product` (exact field arithmetic; this is the same as `chunkedScan_eq`, but for sums).

## Sequenced packages (not runnable yet; the contract is stated now so Lean can lead)
- **W13 msm-hostpath (after the W9 merge and W10's sequential H0 profile).**
  Existing: `Msm.msmDispatch_meets`, `lokiMsm_meets`, `Field.convertScalar_hostToDevice`.
  New **O13.1 `residentCache_sound`:** an identity-keyed hit (SRS content digest + length + slot) implies that the
  resident bases equal the requested bases elementwise. This needs either an immutable SRS handle or a recomputed
  digest; today the cache has no Lean model ("n/a" in SPEC.md). New **O13.2:** a parallel scalar conversion equals
  the elementwise `convertScalar`. Pinned or async staging: `SameOrder`, with results consumed in transcript order
  (Quint `scheduler.qnt`).
- **W14 cpu-quotient-fusion (after the W9 merge; W10 keeps `evaluation/cuda.rs` and the `compute_nu_poly` dispatch).**
  Existing: `Plonk.numeratorAt`, `EvaluatesNumerator`, `quotient_contract`, `hornerY_swap`, `Ntt.rotate_extended`,
  `Refines.quotientEval`. New **O14.1 `fused_numerator_eq`:** one row-ordered loop that applies gates, then
  permutations, then lookups, then trash per row computes `numeratorAt i` at every extended index. Basis: `values[i]`
  depends only on row `i` of read-only cosets, and the per-row term order equals the stock pass order.
- **W15 shared-cse (after W10's compile step exists).** New **O15.1 `sharedGraph_eval_eq`:** evaluating one
  hash-consed DAG over gates, lookups and trashcans (with consumers that read columns directly, without `Store`
  nodes) equals evaluating each `Expression` separately. Constant folding for 0, 1 and 2 and the
  `Square`/`Double` rewrites are sound.
- **W16 lookup-perm-prep (only if the H0 profile shows at least 5 s of non-MSM work in these phases).**
  Existing: `Refines.lookupPermute_eq`, `loki_lookup_valid`, `loki_lookup_order_free`, `product_column_eq`,
  `chunkedScan_eq`, `batchInvertParallel_eq`. New **O16.1 `lookupPrepare_batch_eq`:** computing the deterministic
  part of every lookup first, then drawing blinding values and writing commitments in lookup order, produces the
  same `Permuted` list, the same rng draw sequence and the same transcript events as the sequential map.
- **W17 keygen and cost-model speed (Loki `dev/cost_model.rs`, `keygen.rs`; after the W9 merge).** Existing
  `keygen-deterministic` (open). New **O17.1:** a parallel or GPU computation of fixed cosets and of the permutation
  pk equals stock (follows from `Ntt.coeffToExtended_eq_eval` and `IsCosetNtt`). **O17.2 (Rust gate):** the
  `circuit_model` output (rows, k, column counts) is unchanged.

## Track B (new keys)
A circuit or protocol change (fused k15 terminal, table cut to k19, LogUp, smaller limbs, a lower quotient degree,
fan-in, in-circuit fixed-base tables) is not a refinement of stock. Its contract is a new relation's soundness and
statement equivalence to `SPEC.md`, which `Refinement.lean` does not cover. List these for later; no Loki Lean work
is requested now.

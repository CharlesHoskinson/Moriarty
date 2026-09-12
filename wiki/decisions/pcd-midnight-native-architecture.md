---
id: moriarty.pcd.midnight-native-decision
type: decision
title: Midnight-native PCD architecture decision
status: active
updated_at: 2026-09-11T21:15:54Z
created: 2026-09-11
updated: 2026-09-11
tags:
  - moriarty
  - pcd
  - midnight
  - decision
sources:
  - SRC-0111
  - SRC-0112
  - SRC-0113
---

# Midnight-native PCD architecture decision

This page records the September 11 decision on how Moriarty implements mandatory proof-carrying financial transactions on Midnight.

| Artifact | Location |
|---|---|
| Full analysis | [decision report](../../deliverables/pcd-midnight-native-2026-09-11/REPORT.md) |
| Staged plan | [PCD roadmap](../../openspec/PCD-ROADMAP-2026-09-11.md) |
| Measurements | [reproduced measurements](../../evidence/pcd-midnight-native-2026-09-11/README.md) |
| Captures | [primary captures](../../raw/sources/pcd-midnight-native-2026-09-11/receipts.jsonl) |

**Sources.**

- SRC-0111: Midnight implementation, pull requests, design records, documentation and network observations.
- SRC-0112: cryptographic literature.
- SRC-0113: local measurements.

The [PCD integration amendment](../../openspec/PCD-INTEGRATION-2026-09-11.md) adopted the roadmap into OpenSpec planning on 2026-09-11 as specified-only work. Atomic F3 rests on the Stage 0 seam, SP04 and SP06 deliver certificates, and verification-enabled mandatory Preview acceptance remains the hard gate. Six design defaults await the user's confirmation, and the certificate k bound needs a user-approved resource amendment.

## Decision

**CLM-0946.** Moriarty should adopt a ledger-anchored certified state machine with bounded native certificates.

1. Each entry point compiles to one contract-call circuit. It proves authorization, transition validity, effect correspondence, intent refinement and per-step invariants together.
2. Each entry point reads the head it replaces, or reads a head's absence when creating one.
3. Operation keys are immutable, and genesis is constrained.
4. History compliance for on-ledger history follows by induction over ledger acceptance.
5. Midnight recursion verifies only bounded certificates: off-ledger segments, third-party attestations and cross-domain imports.
6. General DAG proof-carrying data and per-transaction recursive history are rejected.

**Metadata.** SRC-0111 (report §§1, 10–12), SRC-0112 (report §5), SRC-0113 (report §14.2). Decided 2026-09-11; recommendation derived from source facts and experiments; S2 proposal; not reproduced end to end; confidence medium-high. The user directed that recursion be assumed to reach production within months. See [[wiki/formal-assurance#Midnight-native PCD decision — 2026-09-11|formal assurance]].

## Live acceptance seam

**CLM-0947.** On the ledger 8 generation, the ledger decides contract-call proof validity in `Transaction::well_formed`, against the verifying key stored in `ContractState.operations[entry_point]`.

- **The statement.** The ledger builds it:
  - a SHA-256 binding input over address, entry point, gas, effects, guaranteed program length and the intent binding commitment;
  - the communication commitment;
  - every transcript operation, including reads with their values.
- **Not bound.** Network id, fees, time-to-live and any state root.

**Metadata.** SRC-0111, midnight-ledger `ledger-8` `3fa0d1d15a3cfabd41c806546a006b1ce406b7b2`, `ledger/src/verify.rs:1802-1937`, `ledger/src/structure.rs:435-472`, `transient-crypto/src/proofs.rs:566-583`. Commit 2026-09-10; source fact; S6 for ledger 8 behavior; not reproduced; confidence high.

**CLM-0948.** A Midnight contract-call proof cannot be the inner proof of another contract call's `verify_proof`.

- Ledger-accepted contract proofs use a Blake2b transcript.
- Pull request 738's `verify_proof` reads only Poseidon-transcript zk-stdlib proofs.
- Recursive verification of a predecessor transaction proof is therefore not a native operation, even after recursion ships.

**Metadata.** SRC-0111, pull request 738 head `416da99309cff5f953f029ff6c89211c00cf423c`, `transient-crypto/src/proofs.rs:72,784` and `zkir-v3/src/ir_instructions/verify_proof.rs:74-84`. Observed 2026-09-11; source fact; S3 for the unreleased interface; not reproduced; confidence high.

## Ledger guarantees and their limits

**CLM-0949.** At application the ledger re-runs a call's transcript against current contract state and rejects any changed read with `ReadMismatch`.

- **What this makes linear.** Entry points that read the head they overwrite.
- **What stays outside ledger enforcement.**
  - Compact ledger writes emit no read, so creation writes need explicit absence reads.
  - Placement of operations in the guaranteed or fallible transcript section is chosen by the transaction builder at checkpoints.
  - The pool admits `PartialSuccess`, so a call whose fallible section fails is still admitted and charged fees.

**Metadata.** SRC-0111, `ledger-8` `onchain-vm/src/result_mode.rs:44-59`, `ledger/src/semantics.rs:147-190,1383-1406`, `ledger/src/construct.rs:1111-1162`; LFDT-Minokawa/compact `11e7ec5a` `compiler/midnight-ledger.ss:547-556`; midnight-node `release/node-1.0.2` `67cf566c` `ledger/src/versions/common/mod.rs:959-974`. Source fact; S6; not reproduced on a network; confidence high.

**CLM-0950.** A maintenance authority with an empty committee and threshold 1, the ledger default, can never sign, so its operation keys are immutable.

- Neither deploy nor `ReplaceAuthority` validates the threshold, so an empty committee with threshold 0 accepts unsigned maintenance updates.
- midnight-js deploys with a single-signature authority by default.
- The contracts deployed for the Preview runs carry a one-key committee.

**Metadata.** SRC-0111, `ledger-8` `onchain-state/src/state.rs:710-716`, `ledger/src/verify.rs:354-363,1789-1795`; midnight-docs `guides/deploy-and-operate.mdx:616-628`. Source fact; S6; not reproduced; confidence high.

**CLM-0951.** The ledger atomicity of cross-contract calls runs in one direction only.

- A claimed call must exist in the same intent and section, and no call may be claimed twice.
- Nothing requires a call to be claimed.
- A head-retiring `Release` or `Migrate` can therefore apply without the importing call, and needs a recovery path.
- Compact emits cross-contract calls from toolchain 0.33, which targets ledger 9. The ledger-8 verifier already enforces claimed calls.

**Metadata.** SRC-0111, `ledger-8` `ledger/src/verify.rs:1001-1053,1563-1607`, `construct.rs:1066-1077`; `compactc-v0.34.0` release notes. Source fact; S6 ledger rule, S5 Compact support; not reproduced; confidence high.

## Recursion as it will ship

**CLM-0952.** No released ledger generation soundly supports application-defined recursion.

- Ledgers 8, 9 and 10 verify only the outer proof, and never pairing-check a deferred accumulator.
- Compact and ZKIR up to IR 3.0 have no proof-verification primitive.
- Midnight's problem statement MPS-0014 and draft proof-verification MIP (pull request 198) describe recursion as planned, not available.

**Metadata.** SRC-0111, `ledger-8`/`ledger-9`/`ledger-10` `transient-crypto/src/proofs.rs` and `zkir*/src/ir.rs`; MPS-0014; MIP draft `f49199163`. Observed 2026-09-11; source fact and documented plan; S6 absence, S2 plan; confidence high.

**CLM-0953.** Pull request 738 targets `ledger-10`.

- **Interface.**
  - ZKIR `InnerProof` and `VerifyProof` instructions.
  - Inner keys pinned by SHA-256 and compiled into the outer circuit.
  - `DeciderKind::{None, Collapsed}`.
  - `Proof { bytes, accumulators }`, each accumulator 96 bytes and 12 public inputs.
  - A separate ledger pairing check per accumulator after the outer verify.
- **Hazards at its head.**
  - Fees do not charge accumulator inputs or pairings.
  - A zero guard disables verification.
  - `Collapsed` does not check an IVC proof's own key representation or state decider.
- **Status.** On 2026-09-08 the ledger lead marked its threads resolved for merge, on condition that concerns are fixed before `ledger-10` reaches release candidate.

**Metadata.** SRC-0111, pull request 738 `416da99`, `transient-crypto/src/proofs.rs:146-153,397-404,764-802`, `zkir-v3/src/decider.rs:48-52,142-220`, `ledger/src/structure.rs:2045-2047`. Observed 2026-09-11 (moving fact); source fact; S3; not merged; confidence high.

## Proof system and platform versions

**CLM-0954.** Midnight's implemented proof system is a Halo2-derived PLONKish prover with KZG over BLS12-381 and JubJub as the embedded curve, with no curve cycle. Recursion is same-curve accumulation: emulated G1 arithmetic with the pairing deferred to native verification. The 2023 design record chose Pluto/Eris for its cycle; see [[wiki/contradictions#Midnight PCD source conflicts — 2026-09-11|contradictions]].

**Metadata.** SRC-0111, midnight-zk `695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7` `proofs/src/poly/kzg/mod.rs`, `curves/src`, `circuits/src/verifier/accumulator.rs:13-27`; midnight-architecture `eabbedf` `adrs/0013-proof-system.md`. Source fact; S6; confidence high.

**CLM-0955.** Preview, Preprod and Mainnet all reported node `1.0.2-eb71e64e`, runtime spec 1000000 and transaction version 3, which is the ledger 8 generation. Node 2.1.0-beta.1 is the ledger 8 to 9 hard fork. Recursion targets ledger 10.

**Metadata.** SRC-0111, network runtime observation `observed_at` 2026-09-11T06:29:22Z; node-2.1.0-beta.1 release notes. Moving fact; S6; observed; confidence high.

## Reproduced costs

All three measurements were made on one machine: Intel Core Ultra 7 365, 6 cores, 31 GiB, WSL2. [[wiki/benchmarks#Midnight-native PCD measurements — 2026-09-11|Benchmark summary]].

**CLM-0956. Midnight IVC example** at `695351f`.

- The example as shipped at K = 17 fails key generation.
- At K = 18 it has 163,172 rows, a 5,264 B proof and a 411 MB proving key.
- Over 100 steps, proving took 19.5 s per step with no drift, and native verification about 12.5 ms.
- Peak memory was 4.4 GiB.

**Metadata.** SRC-0113 `bench/logs/ivc_k18_n1000_s{1,10_neg,100}.log`. Experiment observation; S3; reproduced; confidence high for this machine.

**CLM-0957. Pull request 738 end-to-end tests** (in-process test SRS).

| Scenario | Outer circuit | Proving | Memory | Proof on the wire |
|---|---|---|---|---|
| One trivial inner proof | k = 18, 150,966 rows | 34.8 s | 4.1 GiB | 6,550 B |
| Two levels | k = 19, 514,873 rows | 81.9 s | 7.8 GiB | 6,550 B |

- Verification including the deferred pairing took about 6 ms.
- An inner proof that decodes but is invalid, or a valid proof bound to the wrong instance, still yielded an outer proof.
- The ledger's accumulator pairing rejected both.

**Metadata.** SRC-0113 `bench/logs/e2e_verify_proof_*.log`. Experiment observation; S3; reproduced; confidence high for this machine.

**CLM-0958. Moriarty loan `initialize` circuit on the proof server.** The scenario-pinned custody circuit, compiler 0.31.1, proved offline on proof server 8.1.0.

- Circuit size k = 14.
- Proving took 1.16–1.74 s.
- Proofs were 4,508 B, with a 249 MiB container peak.
- The proofs were not verified.

**Metadata.** SRC-0113 `bench/proof-server/`. Experiment observation; S6 component; partially reproduced; confidence high for proving cost.

## Literature constraint

**CLM-0959.** Only atomic accumulation of KZG openings runs on a BLS12-381 KZG Halo2 stack without a new curve or commitment scheme. It supports multiple predecessors per step at roughly linear in-circuit cost.

- **Folding** needs a curve cycle, or about a million non-native gates per fold, plus a new decider.
- **Aggregation** establishes no predecessor validity.
- **Private state.** No surveyed construction lets a successor prove a step that reads predecessor-private state without receiving that state or proving jointly by MPC.

**Metadata.** SRC-0112: BCMS20 §2.4.2, §5.1; KS23 §1.2; KST22 §5; GMN22 §6; OB22. Source facts with stack-fit inference; S2 research; confidence high for definitions, medium-high for stack fit.

## Contradictions

**CLM-0960.** Five source and design conflicts are preserved on the [[wiki/contradictions#Midnight PCD source conflicts — 2026-09-11|contradictions page]]:

1. The proof-system design record versus the implementation.
2. The draft MIP's single batched pairing versus pull request 738's separate pairings.
3. Pull request 738's test comment on published parameters versus the published ceremony files.
4. Moriarty's current PCD proposal versus this decision.
5. The README promise that deployment policy fixes verifier versions versus replaceable keys on the deployed Preview contracts.

## What stays open

These claims establish no Moriarty proof, ledger acceptance or financial settlement. The five ordered experiments in the [PCD roadmap](../../openspec/PCD-ROADMAP-2026-09-11.md) decide the core and certificate routes. Verification-enabled Preview acceptance remains the hard gate. Return to [[wiki/index|the research index]].

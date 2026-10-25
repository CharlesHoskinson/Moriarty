---
title: "Recursion and aggregation in other crypto projects, mapped onto Sherlock"
project: loki
component: other-chains-recursion
date: 2026-10-24
author: haiku-speedup-scout
sources:
  - Loki nodes 05, 06 and 10x/LOKI-10X.md (in this wiki)
  - https://eprint.iacr.org/2024/1651 (Kohrita, Towa, Williamson: one-shot native proofs of non-native ops)
  - https://github.com/succinctlabs/sp1 (README capture raw/other-chains-recursion/sp1-readme.md)
  - https://github.com/axiom-crypto/snark-verifier (README capture raw/other-chains-recursion/axiom-snark-verifier-readme.md)
  - https://barretenberg.aztec.network/api/md_src_2barretenberg_2eccvm_2_r_e_a_d_m_e (ECCVM readme capture raw/other-chains-recursion/aztec-eccvm-readme.md)
  - https://github.com/axiom-crypto/halo2-lib, branch main, halo2-ecc/src/fields/fp.rs (captured raw/other-chains-recursion/halo2lib-halo2-ecc-src-fields-fp.rs via gh api; no bls12_381 module on main)
  - https://docs.telepathy.xyz/protocol/circuits (Succinct Telepathy circuit docs, WebFetch summary)
  - https://docs.succinct.xyz and https://blog.succinct.xyz/sp1-testnet/ (SP1 pipeline, via search)
  - https://dev.risczero.com/api/recursion (RISC Zero pipeline, via search)
  - https://www.trailofbits.com/library/scroll-zkevm-wave-3/ (Scroll audit findings, via search)
  - Zellic UPA audit, https://reports.zellic.io/publications/universal-proof-aggregator (via search)
  - Polygon plonky2 blog repost on telos.net (via search, company claim)
tags: [loki, recursion, aggregation, foreign-field, bls12-381, track-b, sherlock]
---

# Recursion and aggregation in other projects, mapped onto Sherlock

## Read this first

1. **Almost everything here is Track B.** Limb choice, range tables, CRT, ECC chips, fan-in and tree shape all change the in-circuit verifier. That changes the VK, and the ledger-9 verifier pins the VK. The stock-verifier route cannot touch them.
2. **No technique in this survey is a confirmed Track A speed-up for the k20 parent.** The scarce Track A content is prover-side (parallel witness synthesis, lazy reduction in witness computation). None of the sources I could reach document that, so it stays with node 09 and the GPU nodes.
3. **Brief vs. record: children are k13 and k12, not k15 and k12.** The brief says the children are k15 and k12. The recorded run says A102 is k13 and A80 is k12 (`docs/results/sherlock-native-a102-a80-child-producer-20261021/actual/RUN-REPORT.json`, which also records 3,440-byte proofs and 253 instance lanes each). "k15" appears in this wiki only as the target for a proposed fused terminal (astra-2, not built). This note treats the children as k13/k12.
4. **The mapping is insensitive to the child k.** The parent's cost is set by the verifier shape (commitments, evaluations, 253 instance lanes, the two foreign-field MSMs and the pairing gadget), not by the child domain. Sherlock's own parent is already a two-child, shared-fixed-base, single-collapse design (node 05 and `recl9/src/sherlock_link.rs`).
5. **Timing figures conflict in the wiki.** The brief says the k20 parent proves in 220 s on 4 threads. LOKI-10X records 62.3 s setup and 882.6 s proving for the same parent. This note does not reconcile them. Every row below is cited from its own source.

## Already in place in Sherlock (do not propose these again)

| Technique | Where it lives in Sherlock | Evidence |
|---|---|---|
| Pairing deferral: the child verifier outputs an accumulator, not a pairing | `VerifierGadget::prepare` returns an accumulator (node 05) | `midnight-zk` circuits 7.2.4 `verifier_gadget.rs:705` |
| One-shot collapse of the carried accumulator, with fixed bases merged | `sherlock_link.rs` prepares both children, accumulates once, collapses once | node 06 |
| Pairing residue check with no final exponentiation | `recl9/src/pairing.rs` (Fp12/Fp6 witness, constrained Miller loop) | node 05 |
| Width-4 windowed MSM with GLV splitting when appropriate, 1-bit scalar preprocessing | `ecc/foreign/weierstrass_chip.rs:810` | node 06 |
| Poseidon for in-circuit transcript hashing, 20 rows per permutation | `hash/poseidon/poseidon_chip.rs:45` | node 06 |
| Four range-check lanes already selected | `recl9/src/decider.rs:52` | node 06 |
| Accumulator as an accumulation scheme, separate from the decision (PCD-style) | `midnight-zk` `circuits/src/verifier/accumulator.rs` | node 05 |

Any source whose headline is "defer the pairing" or "use a residue check" maps to one of these rows. Its gain is already taken.

## Ranked techniques

Rank is by expected gain on the k20 parent divided by effort. Gain is the claimed or estimated effect on the parent or on the chain. Effort is engineer-weeks. Track A keeps the stock VK, SRS, transcript and proof format. Track B changes at least one of them. Rows are in rank order; the ID is a stable label.

| Rank | ID | Technique (source) | Track | Measured or claimed number | Sherlock hook | Effort | Correctness risk vs. Lean contract |
|---|---|---|---|---|---|---|---|
| 1 | R3 | **Fan-in 3–4 per recursion layer, bounded tree** (SP1 shard compression; zkSync and plonky2 trees) | B | SP1: shards about 2^22, compressed in batches of 3–4 per layer. plonky2: a 1024-proof aggregation lower bound of about 5.2 s (company claim, fully parallel). **Limit:** fan-in amortizes the per-child advice cost, but the 1,032,465-row lookup table does not shrink with arity (node 06), so fan-in 4 does not lower k below 20 on its own | `leaf`/`node` shape in recl9; Sherlock is b=2 today | 1–2 weeks to re-shape; new VK per level | Low–medium: a bounded verifier per level is easy to test; the parent must fold each child's carried accumulator, not just its own proof |
| 2 | R5 | **Endomorphism (GLV) scalar split: 255-bit scalars become two 128-bit halves** (ECCVM readme) | B | The split saves doublings, not window count: about 128 doublings instead of about 255, while the number of table lookups stays about the same. Not measured here | `ForeignWeierstrassEccChip::windowed_msm` (`weierstrass_chip.rs:1788`); GLV is already present "when appropriate", so this is an audit of which scalars and fixed-base terms take the split, not a new feature | 1 week to audit scalar bounds | Low: GLV is already in place; risk is in bounds, which Midnight checks per scalar |
| 3 | R9 | **Multipoint / batched opening with fewer point sets** (Midnight's `multi_prepare` groups by point set; node 06) | B | Node 06 estimate: one redundant lookup removes 3 points and 5 evaluations, about 200 Poseidon rows | `verifier/kzg.rs:338` | 1–2 weeks | Low–medium: valid only if the underlying check is genuinely redundant |
| 4 | R2 | **Fused terminal, one relation over the children's statements** (Loki 10x astra-2; the one-relation form of R1 for this fixture) | B | ESTIMATE from wiki: 20,466 rows, fits k15 capacity 32,761. Not built, not measured | new relation replacing parent + decider | 2–4 engineer-weeks for this fixture (wiki estimate) | Medium–high: must keep all 16 statement fields and decider hashing; needs holdouts and mutation tests |
| 5 | R1 | **Defer non-native EC ops as auxiliary proof information, prove them natively once at the end** (Kohrita, Towa, Williamson, ePrint 2024/1651, Nil/Aztec Labs) | B | Gate count down by "at least one order of magnitude, up to 50×" versus folding-based constructions in Plonk. Conversion of one deferred op costs 4–5 non-native field multiplications. No absolute counts or times reported | `recl9/src/sherlock_link.rs` (child-EC obligations) and `decider.rs:148` | 6–12 weeks (new relation, new keys, decider change, ledger acceptance) | High: soundness is conditional on the auxiliary info being valid, which the stock verifier does not check. Needs a new statement and a new terminal relation |
| 6 | R6 | **Limb choice and range-table size** (compared against Midnight's 7×56-bit and 15-bit range limbs, node 06) | B | No published comparison recovered. halo2-lib (main) has no BLS12-381 module; its `FpChip::new` takes `limb_bits` and `num_limbs` as caller arguments and asserts `limb_bits` in [64, 128) (`halo2-ecc/src/fields/fp.rs`, captured). Midnight: 7 limbs of 56 bits, 15 advice columns for ECC, `Pow2RangeChip` table of Σ2^n entries | `field/foreign/params.rs:277`, `field/decomposition/pow2range.rs:43` | 2–4 weeks; changes the gate set and VK | Medium: limbs must keep integer no-wrap bounds |
| 7 | R8 | **Lazy reduction and CRT tricks** | B | No source in this survey reported a number for these. The halo2-lib `fields/` code is the place to look for the reduction and carry logic (`big_is_zero`, `carry_mod`, `mul_no_carry`), but it is generic, not BLS-specific | `field/foreign/` | unknown | Medium |
| 8 | R10 | **UPA-style outer wrapper with KZG accumulator and Keccak digest** (Nebra; Zellic audit) | B | Vendor claim: verification cost down by "up to five times". No batch size or proving time in sources | Would replace the decider's public output format | 4+ weeks | High: changes the ledger-facing statement |
| 9 | R11 | **Statement batching inside one proof** (Aleo Varuna batching upgrade) | B | None found | none | unknown | Medium |
| 10 | R4 | **Cycle-curve split: move EC ops to a dedicated circuit over a curve whose base field is the outer scalar field** (Aztec Goblin / ECCVM, over BN254 with Grumpkin) | B | Row counts not recovered from sources. ECCVM uses 4-bit wNAF, 8 precompute rows and 4 additions per MSM row (readme capture) | Would replace in-circuit foreign-field MSM with a native-field table | 2–3 months; BLS12-381 has no cycle partner, so this is a PCS change | High: changes the curve and the PCS. Sherlock has no cycle curve (node 05) |
| — | R7 | **Naive cost reference for foreign-field EC** (Succinct Telepathy sync-committee circuit). Reference only, not ranked | — | Telepathy docs: aggregate key uses 512 BLS12-381 additions plus one pairing. Step circuit 27 M constraints (witness 124 s, proving 118 s); Rotate 68 M (witness 180 s, proving 60 s); AWS r6a.8xlarge 32 cores, 256 GB; Circom + Groth16 | Calibration only | 0 | none |
| — | R12 | **Universal aggregation (Electron Labs)**. Not ranked | — | Nothing specific found; the claim is generic aggregation | none | — | — |

**Honest bottom line for the k20 parent:** with the stock VK, none of these techniques reaches it. R2 and R1 are the only ones that cut the parent's work by changing the circuit, and both need a new verifier deployment. R3 is the cheapest structural change, but on its own it does not lower k.

## Details by project

### Mina Pickles (Pasta cycle, accumulation)
- Recursion on the Pasta curves, Pallas and Vesta, forming a cycle. Each curve's scalar field is the other's base field, so no foreign-field arithmetic is needed. Search turned up only a forum thread; no numbers were recovered.
- **Relevance:** Track B and context only. BLS12-381 has no cycle partner, so this route is closed for the KZG outer proof (node 05).

### Zcash halo2 accumulation
- Halo2 uses IPA with an accumulator over the Pallas/Vesta cycle. Its lookup argument is documented in the halo2 book (captured under `raw/06/halo2-lookups.md` in node 06). I did not fetch the accumulation section for this note.
- **Relevance:** same cycle restriction as Pickles. Gap, see below.

### Aztec: Goblin Plonk, ECCVM, ClientIVC, protogalaxy
- **Goblin / ECCVM:** Non-native EC ops are delegated to an ECCVM over BN254 (native field F_q, Grumpkin for commitments). Operations are compiled into a table of F_q values checked by polynomial constraints, multiset-equality checks and lookups. The readme capture confirms the 128-bit endomorphism halves (`z1`, `z2`), 4-bit wNAF with 32 digits plus a skew digit, 8 precompute rows and 4 additions per MSM row, and an `offset_generator` to avoid identity cases (this relaxes completeness, not soundness). **Track B.** Relevant to R4 and R5.
- **ePrint 2024/1651:** the one-shot native proof of non-native ops (R1). Its claim is a gate reduction of at least 10× and up to 50×, against folding-based designs in Plonk. It does not report absolute counts or times.
- **ClientIVC / protogalaxy:** no numbers recovered. Gap.

### Axiom / Scroll / Taiko: snark-verifier aggregation and KZG deferral
- Axiom's `snark-verifier` splits accumulation (`KzgAs`) from the decision (`KzgDecidingKey`), which is the pattern Sherlock already has (node 05). **Already in place.**
- Scroll: its halo2-snark-aggregator repo is archived (last push February 2023). The Trail of Bits audit (wave 3) found two informational items: the accumulator assumes fixed-length limbs, and `MAX_AGG_SNARKS` values other than 10 may misbehave. **Track B** for any change. No k, batch size or time figures were recovered for Scroll or Taiko. Gap.

### zkSync Boojum
- FRI-based STARK over Goldilocks, Plonkish arithmetic. Aggregation uses 15 circuit types across several layers, then a Plonk+KZG or Fflonk+KZG wrap. A recent zkEVM survey says production nodes often merge more than two children. Exact arity was not confirmed. **Track B.** No numbers recovered.

### Polygon zkEVM / plonky2
- Plonky2 on Goldilocks with FRI. A company claim, reposted on telos.net, gives about 5.2 s for aggregating up to 1024 proofs as a fully parallel theoretical lower bound, and about 900 ms to wrap a proof into GoldiBear. These are not independent measurements. **Track B.**

### Succinct SP1 and RISC Zero: STARK to SNARK wrap
- **SP1:** shards of about 2^22 cycles, proved in parallel with STARKs, then compressed in batches of 3–4 per recursion layer. The final STARK is wrapped in PLONK-KZG or Groth16 over BN254 (gnark). An older testnet post reports a Merkle-proof benchmark: core 211 s, recursion 165 s, Groth16 wrap 144 s (the wrap includes a "shrinking" stage). This is old data. **Track B** (inner STARK, different proof system). The wrap is a protocol change the stock verifier cannot accept until the wrap exists.
- **RISC Zero:** each segment is lifted to a succinct receipt, pairs are joined until one remains, then identity_p254 and a Groth16 wrap over BN254. Its recursion circuit is a STARK. No timing recovered. **Track B.**

### Aleo Varuna
- Varuna is Marlin-based. Provable reports an upgrade to its batching protocol that proves several circuit statements together and handles the case where the prover cannot adapt statements to verifier randomness. No numbers. **Track B.**

### Nebra UPA
- The outer universal circuit wraps Halo2's AggregationCircuit, with a KZG accumulator and a Keccak digest (Zellic audit). The vendor says verification cost falls by up to 5×. This is not independently verified. **Track B** (the ledger-facing statement changes).

### Electron Labs
- Search found only a company overview: universal ZK aggregation through recursion and light clients. No halo2 specifics, no numbers. **Gap.**

### Ethereum sync-committee BLS12-381 circuits
- **Succinct Telepathy:** Circom + Groth16. The aggregate key uses 512 BLS12-381 additions (one per committee member), plus one pairing. Step 27 M constraints, Rotate 68 M. The Rotate circuit re-commits to the committee with Poseidon instead of SHA-256, which the page calls "extremely expensive" in SNARKs. Field limbs and emulation are not described on the page. **Track B** for any analogue. As a calibration point, 512 naive additions at Midnight's stock complete-addition cost of 342 rows (node 06) is about 175,000 rows (ESTIMATE, not measured), the same order as Sherlock's 316,833-row pairing gadget.
- **Polyhedra zkBridge / zkLightClient:** no circuit details recovered. Gap.

## Gaps (honest)
- No row, k or time figures recovered for Mina, Zcash accumulation, Aztec ClientIVC/protogalaxy, Scroll, Taiko, zkSync, RISC Zero, Electron Labs, Polyhedra or Aleo Varuna. Several of these need their repos or papers, not summary pages.
- halo2-lib has no BLS12-381 module on `main`, so there is no halo2-ecc limb constant to compare with Midnight's 7×56. The R6 comparison therefore rests on Midnight's own numbers and on the generic limb API above. PSE's halo2-ecc was not checked, per the advisor's instruction not to hunt forks.
- The Telepathy docs give no limb or emulation detail, so R8 cannot be assessed from them.

## Hooks (circuit-crate hooks, not prover-crate hooks)
All of these live in the registry crate `midnight-circuits-7.2.4` and in `recl9`, not in `loki-proofs`:
- `verifier/verifier_gadget.rs` (`prepare`, accumulator), `verifier/msm.rs:481` (fixed-base merge).
- `ecc/foreign/weierstrass_chip.rs` (`windowed_msm` at :1788, table loop at :1840, multi-select at :1405).
- `field/foreign/params.rs:277` (limbs), `field/decomposition/pow2range.rs:43` (range table).
- `recl9/src/decider.rs:148`, `recl9/src/pairing.rs`, `recl9/src/sherlock_link.rs`.

## Track A
No surveyed technique is confirmed Track A for the k20 parent. Track A prover-side candidates (parallel witness synthesis, avoiding double execution of region closures) are in node 09 and the GPU nodes, not here.

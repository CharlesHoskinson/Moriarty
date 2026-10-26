# Recorded stage observations supplied for this research
> Source: file:///home/charl/code/Moriarty-perf/wiki-llm/recursion-perf/stages.json
> Collected: 2026-10-26
> Published: 2026-10-26

{
  "schema": "recursion-perf.stages/1",
  "created": "2026-10-26",
  "purpose": "Single source of truth for the recursion state machine: the Quint model, the three.js state-machine page and the research-cluster launcher all read this file. Costs are recorded observations with their sources; none are new measurements.",
  "pipeline": "Sherlock: children A102 (k13) and A80 (k12) -> parent P (k20, verifies both in-circuit) -> terminal decider D (k20, pairing + Blake2b transcript) -> ledger-9 call on stagenet. Fenrir recl9: 4 leaves (k17) -> node N (k20, arity 4) -> decider D (k20) -> StrawAnchor settle.",
  "proof_states": ["Queued", "KeysReady", "Witnessed", "Committed", "LookupsArgued", "Quotiented", "Opened", "Proved", "Verified", "Consumed", "Decided", "Submitted", "Finalized", "Failed"],
  "stages": [
    {
      "id": "s1-keys",
      "title": "Keys, SRS and parameters",
      "states": ["Queued", "KeysReady"],
      "observed": [
        {"what": "Sherlock k20 parent setup_pk", "value": "62.3 s", "source": "wiki-llm/loki/10x/LOKI-10X.md (receipt sherlock-native-observations-20261022/parent-proof.json)"},
        {"what": "Fenrir recl9 node VK setup", "value": "622.8 s", "source": "projectfenrir recursion/stagenet-strawman/recl9/REPORT.md"},
        {"what": "Proving-key caching gain", "value": "about 1.07x (setup share only)", "source": "LOKI-10X.md correction to LOKI-DESIGN L1"}
      ],
      "seeds": ["wiki-llm/loki/10x/loki-01-prover-profile.md", "wiki-llm/loki/LOKI-DESIGN.md"],
      "questions": ["Persistent prover service with resident keys and SRS: what does warm-start buy across a batch stream?", "Key generation and VK derivation parallelism; why does the recl9 node VK take 623 s?", "SRS loading, memory-mapping and device residency on a 32 GB RTX 5090."]
    },
    {
      "id": "s2-witness",
      "title": "Witness synthesis",
      "states": ["KeysReady", "Witnessed"],
      "observed": [
        {"what": "Witness synthesis share of k20 parent", "value": "unmeasured; Loki notes closures run twice and all advice columns are allocated up front", "source": "LOKI-10X.md plan step 3"},
        {"what": "cuSnark wide k20 circuit: witness and lookup preparation left after GPU", "value": "363.28 s of 463.13 s", "source": "wiki-llm/loki/10x/astra-1/raw (snarkify cusnark capture)"}
      ],
      "seeds": ["wiki-llm/loki/10x/loki-09-cpu-witness-field.md", "wiki-llm/loki/10x/astra-1/PROPOSAL.md"],
      "questions": ["Single-pass synthesis, streaming advice, avoiding duplicate closure evaluation in midnight-proofs.", "Parallel and SIMD field arithmetic for BLS12-381 scalars on 20 cores.", "Generating the K-A reference-machine witness stream (G5) directly in circuit layout order."]
    },
    {
      "id": "s3-commit-msm",
      "title": "Commitments and MSM",
      "states": ["Witnessed", "Committed"],
      "observed": [
        {"what": "MSM dispatch split", "value": "midnight-proofs 0.8.2 switches away from BLST above 2^19 points; 0.8.3 always uses BLST", "source": "LOKI-10X.md, astra-1/PROPOSAL.md"},
        {"what": "Nocy RTX 5090 PLONK server", "value": "k17 12.3 -> 4.4 s, k18 24.6 -> 8.8 s (2.80x)", "source": "astra-1/raw/haiku-01-haiku01-nocy-repo.md"}
      ],
      "seeds": ["wiki-llm/loki/10x/loki-02-gpu-msm.md", "wiki-llm/loki/10x/astra-1/raw/haiku-03-h3-sppark.md", "wiki-llm/loki/10x/astra-1/raw/nocy-msm.md"],
      "questions": ["Bucket/Pippenger GPU MSM for BLS12-381 G1 on Blackwell (sppark, ICICLE, cuZK, yrrid).", "Batching many commitments per kernel launch; avoiding per-commitment synchronization.", "Precomputed bases and SRS residency trade-offs within 32 GB."]
    },
    {
      "id": "s4-lookups",
      "title": "Lookups and grand products",
      "states": ["Committed", "LookupsArgued"],
      "observed": [
        {"what": "k=20 floor of recl9 node and decider", "value": "set by lookup table rows (1,032,465 table rows vs 958,095 advice)", "source": "projectfenrir recursion/stagenet-strawman/CIRCUIT-INVENTORY.md"},
        {"what": "Lookup prover nondeterminism", "value": "HashMap iteration in midnight-proofs-0.8.3 plonk/lookup/prover.rs:399-430", "source": "LOKI-10X.md gotchas"}
      ],
      "seeds": ["wiki-llm/loki/10x/loki-06-recursive-circuit-cost.md"],
      "questions": ["Shrinking or splitting lookup tables to drop the k20 floor (k19 halves the domain).", "logUp / cached-quotient lookups, Lasso/Jolt-style decomposed lookups that fit PLONK/KZG on BLS12-381.", "GPU grand-product (segmented scan) kernels."]
    },
    {
      "id": "s5-quotient-ntt",
      "title": "Quotient and NTT",
      "states": ["LookupsArgued", "Quotiented"],
      "observed": [
        {"what": "cuSnark quotient on GPU", "value": "1,115.43 -> 36.19 s (30.82x)", "source": "astra-1/raw (snarkify cusnark capture)"},
        {"what": "Snarkify BLS12-381 2^22 NTT (warm kernel)", "value": "1.13 ms", "source": "astra-1/raw/snarkify-microbench-code.md"}
      ],
      "seeds": ["wiki-llm/loki/10x/loki-03-gpu-ntt.md", "wiki-llm/loki/10x/astra-1/raw/pipelonk-conference.md"],
      "questions": ["Extended-domain evaluation fully resident on GPU (coset NTTs without host round-trips).", "Pipelonk-style pipelining and fused kernels for the quotient.", "Reducing extended-domain blowup via the constraint degree."]
    },
    {
      "id": "s6-opening",
      "title": "Multiopen and KZG opening",
      "states": ["Quotiented", "Opened", "Proved"],
      "observed": [
        {"what": "fflonk (t=4) in upstream midnight-zk PR #565/#575", "value": "VK 2,258 -> 578 bytes at a small prover cost; postdates our checkout", "source": "LOKI-10X.md plan step 5"}
      ],
      "seeds": ["wiki-llm/loki/10x/loki-04-halo2-gpu-precedents.md"],
      "questions": ["SHPLONK vs fflonk trade-offs for the inner and terminal layers.", "Batch opening across child proofs.", "GPU multiopen."]
    },
    {
      "id": "s7-recursion",
      "title": "Recursion and aggregation",
      "states": ["Verified", "Consumed"],
      "observed": [
        {"what": "Sherlock k20 parent (verifies k13 + k12 children)", "value": "316-382 s, 15.8 GB peak (A3); older receipt 882.6 s prove", "source": "qa/sherlock/runs.jsonl A3; LOKI-10X.md"},
        {"what": "recl9 node N, arity 4", "value": "867.3 s, 14.99 GB; averages 268% CPU on 20 cores", "source": "recl9/REPORT.md; LOKI-10X.md"},
        {"what": "Each extra child in a node", "value": "about +191k rows (arity 2/3/4 = 574k/766k/958k)", "source": "recl9/CIRCUIT-INVENTORY.md"},
        {"what": "Pairing-residue gadget", "value": "316,833 rows: a k19 floor for any terminal that verifies a proof", "source": "LOKI-10X.md"},
        {"what": "Fused terminal (prove A102->A80 directly, about k15)", "value": "chain about 10x plausible; needs new keys and a new verifier", "source": "LOKI-10X.md answer (b)"}
      ],
      "seeds": ["wiki-llm/loki/10x/loki-05-recursion-accumulation.md", "wiki-llm/loki/10x/loki-07-folding-alternatives.md", "wiki-llm/loki/10x/astra-2"],
      "questions": ["Accumulation/deferred pairing (KZG accumulation) instead of full in-circuit verification.", "Folding schemes over BLS12-381 or curve cycles that still terminate in a ledger-9-acceptable PLONK proof (Nova is unsupported on BLS12-381).", "Optimal tree arity and shape for 16-tx batches; the fused-terminal design space."]
    },
    {
      "id": "s8-decider-settle",
      "title": "Decider, ledger-9 and settlement",
      "states": ["Consumed", "Decided", "Submitted", "Finalized"],
      "observed": [
        {"what": "Sherlock terminal decider D (A4)", "value": "325 s, 16.5 GB", "source": "qa/sherlock/runs.jsonl A4"},
        {"what": "recl9 decider D", "value": "586.4 s (6 threads), 14.97 GB; proof 6,432 B; ledger verify 21 ms", "source": "recl9/REPORT.md"},
        {"what": "16-tx batch accepted on stagenet (B4)", "value": "stock verify 125-155 s, 4.2 GB; settle with readback", "source": "qa/sherlock/runs.jsonl B4"},
        {"what": "Wallet DUST registration proof (rc.8 proof server, CPU)", "value": "435 s end to end including sync", "source": "~/code/stagenet-wallet/register-dust.log (2026-10-26)"}
      ],
      "seeds": ["wiki-llm/loki/10x/loki-10-integration-verification.md"],
      "questions": ["Decider cost reduction: cheaper transcript-to-pairing binding, smaller residue gadget.", "Client-side transaction proving latency (wallet proof server) and DUST fee economics per settled batch.", "Ledger-9 verifier constraints that bound how much can change."]
    },
    {
      "id": "s9-orchestration",
      "title": "Scheduling, memory and hardware",
      "states": ["Queued", "Finalized"],
      "observed": [
        {"what": "16-tx batch end to end, local (B3)", "value": "920-1,288 s, 16.2 GB", "source": "qa/sherlock/runs.jsonl B3"},
        {"what": "2/4/6-batch prefixes (B5)", "value": "5,559 s", "source": "qa/sherlock/runs.jsonl B5"},
        {"what": "recl9 16-tx chain", "value": "1,617.3 s total = 2.23 batches/hour", "source": "LOKI-10X.md"},
        {"what": "Throughput scheduler estimate (4-8 CPU witness workers + 1 GPU context)", "value": "7.4-9.7x batches/hour; two heavy proofs peak about 30.7 GB", "source": "LOKI-10X.md answer (c)"}
      ],
      "seeds": ["wiki-llm/loki/10x/loki-08-recursion-tree-scheduling.md", "wiki-llm/loki/10x/astra-3"],
      "questions": ["Pipelining batches: overlap leaves of batch n+1 with node/decider of batch n under a memory admission policy.", "Proof markets/distributed proving across machines; CPU-GPU co-scheduling on one 5090.", "Which measurement (L0 phase profile) unblocks every other decision, and how to instrument it."]
    }
  ]
}

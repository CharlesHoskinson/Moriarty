---
title: "Benchmark-suite debate record"
project: loki
date: 2026-10-25
tags: [loki, benchmark, debate]
---

# Benchmark-suite debate

Accepted design v2. AgentBus run `loki`, channel `#lokibench`; the suite specification is [SUITE.md](SUITE.md). Debate ran on 2026-10-25 from approximately 07:22 to 07:39 UTC. This record covers design, not measurement. No benchmarks/builds/proofs/jobs or locks were run/taken. Only this benchmark directory contains authored documents; no commits were made. Eight explicit accepts, one satisfied conditional acceptance and one abstention; no unresolved required change remains. Execution is still NOT READY.

## Participants and arguments

| Participant | Role and initial position | Critiques and resulting decision |
| --- | --- | --- |
| sol-1 | Moderator/statistics. Proposed n=10 matched independent processes, primary operational chain, median/spread and stock verification | Challenged arbitrary scalar assignment in constrained circuits and unavailable stock phase times. Considered n=7 compromise after cost critiques; restored n=10 paired cold/warm CHAIN after clarifying later execution can span reserved windows. Ratio of medians selected; small-n bootstrap removed |
| sol-2 | Real Sherlock workload. Four serial stages A102, A80, A3 and decider, pinned current capacities; include production orchestration and per-call PK work | Challenged parent-only interpretation, hidden setup subtraction, old child k/lane counts and manufactured sum-of-medians chains. Required actual same-run linkage, stock receipts and terminal semantics. Accepted in suite |
| sol-3 | GPU fairness. Merged legacy sppark arm, same CPU options as paired Loki CPU, transfers/cache/fallback included | Corrected second-process warmup, OS cache dropping, forced phase sync and purported exact peak memory. Identified missing production MSM success/fallback telemetry. Adopted paired operational cold→warm with host-only SRS/VK start, preserving per-call setup |
| sol-4 | CPU fairness. S0/L0/G0 defaults, separate workspaces/lockfiles/invocations and affinity plus Rayon | Identified BLST's independent pool and historical four-Rayon/twenty-CPU mismatch, verifier BLST override, native commit_zk mislabel. Required effective masks, feature source IDs, binary provenance and common Sherlock circuit-params disclosure |
| sol-5 | Adversarial skeptic. Independent stock oracle, independent VK generation, full consumption/pairing and negative controls | Challenged deterministic-Loki byte-equality promise, arbitrary synthetic distributions, setup omission, permissive idle threshold and invalid timing ratios. Required complete attempt ledger and cell invalidation on correctness failure. Supported retaining n=10 primary despite later run cost |
| haiku-1 | Synthetic k/distribution grid; initially proposed parent column-count surrogate, k20 n=3 and deterministic proof equality | Critiqued environment/table proposals. Retracted one-advice description (actual fixture has three advice/two phases), invalid arbitrary witnesses, byte-equality guarantee and n=3. Accepted established fixture plus direct MSM equality diagnostics, scalar statistics outside timing |
| haiku-2 | Memory/energy. RSS, GPU memory and board-power sampling, initially per-phase VmHWM and tight polling | Critiqued clocks, memory gates and weak repetitions. Accepted lifetime versus phase RSS distinction, calibrated NVML sampler, sampled lower-bound GPU peaks, unavailable CPU energy and future-resident estimates not being legacy measurements |
| haiku-3 | Environment/reproducibility. Initial strict idle gate, affinity, cache dropping, fixed GPU clocks and host inventory | Critiqued memory/idle proposals with per-core accounting, VmSwap and finite wait. Withdrew cache dropping, max clocks and mandatory core-speed pilot; fixed common mask with unknown WSL P/E class is acceptable. No vote cast; coordinator instructs recording ABSTAIN |
| haiku-4 | loki-profile. Exclusive phases, stage counters, overhead qualification and separation from stock | Critiqued stock phase column, per-phase VmHWM and run cost. Withdrew forced primary phase sync and dispatch inference from generic arithmetic counters; accepted four-stage diagnostics and outside-create_proof attribution |
| haiku-5 | Table/reporting. Side-by-side prove/verify, provenance, cold/warm rows, n, paired ratios and failures | Critiqued resource-sampler overhead and n=10 affordability. Accepted no stock phase column, invalid diagnostic-only durations, operational PK cost inclusion, explicit CPU mask/OS-thread distinction and separate profile population |

## Decisions and reasons

1. **Owner endpoint:** observed serial four-stage chain through independent stock acceptance/semantic checks, not A3 or MSM alone. Large work outside create_proof and historical kernel-only wins make this necessary.
2. **Validity:** every proof and intermediate stage must have independent registry-stock final pairing/consumption receipts and independently generated equal VK bytes. The old W10 A3 probe has historical artifact pins and BLST no-threads; it is not copied unchanged. Lookup proof-byte inequality is diagnostic because stock HashMap order is not fixed by proof RNG or Loki's sorting flag.
3. **CPU/GPU comparison:** separately built/resolved S0 default stock, L0 default Loki and G0 with registry+NTT, at matched allowed CPUs AND Rayon workers. Default stock BLST keeps its own pool within the same affinity capacity. Budget and multiopen have named secondary CPU/GPU ablations.
4. **Cache populations:** one independent process executes measured cold then measured warm workload using the same allocations. Cold serves as the full warmup for warm and is retained. No OS cache drop or unsupported claim all mixed-k SRS entries remain resident. Existing operational PK rebuilds remain timed in both populations.
5. **Statistics:** final v1 CHAIN n=10 per cold/warm/thread/arm. Expensive standalone parent/decider/k20/ablations n=7, inexpensive/kernel n=10. Run later over predeclared dedicated windows. Median/IQR/min-max/all records; ratio of matched medians headline, paired ratios descriptive. No significance claim from weak bootstrap or best-of selection.
6. **Workloads:** exact current Sherlock capacities and source pin; child/decider degrees measured from actual qualification VKs before execution. Established valid proof k-sweep plus scalar-distribution MSM controls; column counts alone do not create a valid parent surrogate. Native zswap adapter must emit accepted finalized proof bytes. commit_zk is random blinder generation, not a commitment/proof.
7. **Profiling/resources:** Loki-only phase diagnostics, five independent blocks, separate population. Phase-sum identity is bookkeeping. No extra per-phase GPU sync in primary. Production dispatch/fallback telemetry is a missing implementation gate. GPU/RSS sampled peaks are lower bounds; board energy is estimated and CPU energy unavailable, so no system-energy claim.
8. **Host readiness:** dedicated CPU/GPU reservation, fixed idle/noise thresholds, no active swap, effective runner masks and policy capture. Default factory capacity preflight is insufficient to prove idle. Default GPU policy remains unchanged. v2 includes per-core idle accounting, finite 15-minute gate wait, explicit deferral/replacement limits, VmSwap and unknown WSL core-class disclosure. No result-dependent core-mask pilot.
9. **Pin update:** coordinator explicitly superseded the original brief's a256a9a seed revision with main selected at execution-time pre-registration. Current clean candidate verified read-only is `0c4ab9d0961a008dee6340d8eab9dcc90fd64ab4`, with in-tree loki-gpu. Freeze one full SHA plus every dependency/patch/lockfile/binary digest before qualification and all samples; never benchmark a moving main. Source claims from older reads must be rechecked at the execution pin.
10. **Runner:** current cargo-only GPU service does not hold heavy.lock or relay arbitrary environments and locks GPU before cargo when gpu/cuda appears in an arg. v2 chooses a std-only NON-GPU-classified P7 launcher, explicitly auditing all request args and requiring service `gpu:false`, then inner DoctorWatson heavy→GPU exactly once across the whole matched block. No inverse-order request or nested lock acquisition is permitted. Timeout/env/affinity/cleanup/inner-lock receipts are readiness work owned by P7 under `bench/`. No benchmark jobs were requested in debate.
11. **Build-family correction:** v1 enabled Sherlock interface features in every build. sol-4 required truly default SYN/native controls. v2 has independently hashed default-family and circuits-family stock/Loki/verifier targets; Sherlock and ZSWAP explicitly disclose common circuit-params. k18 wrapper support also needs qualification.

## Draft revisions

`DRAFT v1` was posted as `01M5RZJTAR5J8S67KQG808FQ3J`. sol-2 objected to wording that could exclude recurring child SRS/VK reconstruction. sol-3 objected to lock-order ambiguity and insufficiently specified repeated-scalar width. sol-4 objected to topology prerequisites, lock order and circuit-params leaking into supposed default controls. sol-5 independently required the lock-order, wide-scalar and reconstruction fixes. haiku-2 accepted conditionally on a named current-main pin; haiku-1 requested explicit wide repeats and all-one. These were required changes, not dissent silently ignored.

`DRAFT v2 FINAL VOTE` was posted as `01M5RZTFWFHPP4BW8P41X56B94`. It includes all those corrections, coordinator's pin/P7 updates, finite idle deferral and VmSwap accounting, and explicitly records the known stock-rejected mixed-instance multi-proof shape. This is the first of at most two permitted revisions. No measurements informed either revision.

## Protocol and votes

All ten initial proposals were received and ALL ten participants posted critiques of at least two proposals. sol-1 posted source-grounded critiques of haiku-1 and haiku-5 to the channel and a GPU-boundary critique directly to sol-3. Full durable channel messages are available through `agentbus --cwd ~/repos/loki-wt/ids/sol-1 log --since 3h --grep lokibench --all` shortly after this session (use a wider time range later). Coordinator's pin update is `01M5RZKAHJTW0EYVZ3FM2XT5X3`; its vote tally/P7/ABSTAIN instruction is `01M5RZM3WC87355571MY97YWVV` (direct to sol-1). Votes below distinguish actual explicit votes from carried acceptance and conditional resolution.

| Participant | v1 | v2 | v3 / final |
| --- | --- | --- | --- |
| sol-1 | Required revision as moderator | ACCEPT FINAL `01M5RZYH6SB6DBKTHK2W4ANZMH` | ACCEPT v2 |
| sol-2 | OBJECT `01M5RZKHK7QM74JJS2MQRHG4AY` | ACCEPT FINAL `01M5RZV1XDG84NHRVB12149KQN` | ACCEPT v2 |
| sol-3 | OBJECT `01M5RZM2YD975E7PJTR5SKCRB5` | ACCEPT FINAL `01M5RZWKZWVDYF6RJB3ZMPR8BS` | ACCEPT v2 |
| sol-4 | OBJECT `01M5RZMSM3VBAD5TSYB0AAWNP8` | ACCEPT FINAL `01M5RZX5FGWJFE3PPSWWTX9TDT` | ACCEPT v2 |
| sol-5 | OBJECT `01M5RZMZVX8SSRNWHZ8NBV3HA3` | ACCEPT FINAL `01M5RZVYNB6WDF0GQGKRRZSHDN` | ACCEPT v2 |
| haiku-1 | ACCEPT `01M5RZM66XQE8815X4FJBEXFWT`; wide-scalar notes | Accepted proposal retained; requested wording added | Carried ACCEPT, no new v2 vote claimed |
| haiku-2 | Conditional ACCEPT `01M5RZKT923QXCW570DPX8N4FS` | Current-main pin condition addressed | Conditional acceptance satisfied, no new vote claimed |
| haiku-3 | No vote | Coordinator reports ABSTAIN | ABSTAIN |
| haiku-4 | ACCEPT `01M5RZJV2KMH5Y19H7NY551HDK` | Accepted proposal retained; route clarification incorporated | Carried ACCEPT, no new v2 vote claimed |
| haiku-5 | ACCEPT `01M5RZKHHPAC7FF474AYVS57SS` | Ratio-direction/invalid-cell notes incorporated | Carried ACCEPT, no new v2 vote claimed |

## Dissent

No unresolved objection or substantive dissent remains. Each Sol reviewer explicitly confirmed its required changes resolved in its final v2 vote. haiku-1/4/5 accepted v1 and their notes were incorporated; haiku-2's conditional acceptance is satisfied by the explicitly named current-main candidate and frozen execution-pin policy. Those are carried/conditional positions, not fabricated fresh v2 votes. haiku-3's abstention is recorded without interpreting silence as acceptance; its environment positions were incorporated. One revision was needed; no third draft or second revision was necessary.

## Verification and handoff

Read-only document checks confirmed required frontmatter, workload/configuration/readiness/table/dissent sections, balanced Markdown fences, no trailing whitespace and valid shell syntax for every bash example (`bash -n`, examples NOT executed). The durable channel log confirms ten proposals and at least two critiques per participant. Explicit final Sol vote message IDs are recorded above. Only SUITE.md and DEBATE.md were authored in this benchmark directory; no repository commits were made.

The coordinator receives `DONE bench-suite` and the channel receives the same final subject after final document verification. P7 owns subsequent chain-driver/launcher implementation and qualification under `bench/`; no job was submitted as this handoff. Missing exact fixture degrees/VKs, independent driver/verifier builds, runtime dispatch/fallback telemetry, measured caps/timeouts and host reservation/cleanup evidence remain concrete execution gates, not claimed completed work.

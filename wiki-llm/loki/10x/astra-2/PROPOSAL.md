---
title: "Sherlock at 10× with new keys: compose transitions first, shrink recursive proofs second"
project: loki-10x
author: gpt-6-astra
date: 2026-10-24
sources:
  local:
    - "../../LOKI-DESIGN.md"
    - "../../nodes/ (all ten nodes and their linked captures)"
    - "/home/charles/repos/Moriarty-factory/docs/results/sherlock-native-observations-20261022/receipts/parent-proof.json"
    - "/home/charles/repos/Moriarty-factory/docs/results/sherlock-native-a102-a80-child-producer-20261021/actual/RUN-REPORT.json"
    - "/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9"
    - "/home/charles/repos/projectfenrir/recursion/pairing-gadget/lanes/opus/RESULTS.md"
    - "/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/{midnight-proofs-0.8.3,midnight-circuits-7.2.4,midnight-curves-0.3.1,midnight-zk-stdlib-2.3.5}"
    - "/home/charles/repos/midnight-zk @31435e9"
    - "/home/charles/repos/midnight-ledger"
    - "/home/charles/repos/Moriarty-factory/crates/mori-backend"
  web:
    - {url: "https://github.com/midnightntwrk/midnight-zk/pull/480", capture: "raw/pr480.md"}
    - {url: "https://github.com/midnightntwrk/midnight-zk/pull/565", capture: "raw/pr565.md"}
    - {url: "https://github.com/midnightntwrk/midnight-zk/pull/575", capture: "raw/pr575.md"}
    - {url: "https://eprint.iacr.org/2022/1530", capture: "raw/logup.md"}
    - {url: "https://eprint.iacr.org/2022/596", capture: "raw/ecip.md"}
    - {url: "https://eprint.iacr.org/2025/933", capture: "raw/fast-scalar.md"}
    - {url: "https://github.com/microsoft/Nova", capture: "raw/nova.md"}
    - {url: "https://eprint.iacr.org/2023/573", capture: "raw/hypernova.md"}
    - {url: "https://eprint.iacr.org/2023/1192", capture: "raw/cyclefold.md"}
    - {url: "https://eprint.iacr.org/2023/1106", capture: "raw/protogalaxy.md"}
---

## Target definition

**PROPOSAL —** Primary target **(b)** is tenfold lower latency from the same fixed A102→A80 transition witness to an independently verified, ledger-9-compatible terminal proof. Replace the recursive chain with one directly composed terminal relation. This removes the original k20 parent; it is not a tenfold speedup of that unchanged proof under target **(a)**. General interpreter executions and target **(c)**, sustained 16-transaction batches/hour, are separate experiments.

**SOURCE —** The completed [parent receipt](/home/charles/repos/Moriarty-factory/docs/results/sherlock-native-observations-20261022/receipts/parent-proof.json:62) records **882.589555 s proving** and **62.328385 s setup_pk**. It identifies a Source44 executable, not a controlled current-registry benchmark; cache state is unknown, n=1, and the prove interval includes PK destruction. The brief’s killed >500-second attempt is censored. Neither is a qualified warm baseline.

**PROPOSAL —** Compare release builds, identical semantic inputs, the stated 20-core WSL2/90-GB host with RTX 5090 32-GB and CUDA 13.4, and equally cached keys/SRS. Include witness generation, all necessary child/fold/wrap work, transfers, serialization and independent verification. Report cold setup separately. Record ledger acceptance and settlement separately from proving latency. An **88.258956-second** complete replacement is a sufficient tenfold threshold against the historical parent interval alone, but a claim requires a matched complete-chain baseline; use its actual time divided by ten.

## Verdict

**ESTIMATE —** **10× is plausible on the bounded (b) target, with medium confidence in the architecture and no measured speedup yet.** Actual A102 and A80 children are k13 and k12. Paying for a k20 verifier of these small, available-witness computations is the clearest avoidable cost. Spend the first engineering effort on a k15 direct terminal, not a new folding system.

**ESTIMATE —** For unchanged k20 latency (a), 2–4× is an implementation expectation, not a mathematical ceiling. For general recursion and 16-transaction throughput, **10× remains unestablished**. Circuit reduction plus broad GPU acceleration can cross ten under explicit assumptions below. No honest universal ceiling follows from the existing traces; several useful ceilings do follow when particular stages remain unchanged.

**CODE —** Path shorthand: `M=/home/charles/repos/Moriarty-factory`; `F=/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9`; `U=/home/charles/repos/midnight-zk`; `L=/home/charles/repos/midnight-ledger`. `P`, `C`, `Z` mean registry packages `midnight-proofs-0.8.3/src`, `midnight-circuits-7.2.4/src`, `midnight-zk-stdlib-2.3.5/src` under `/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`.

## The plan

### 1. Establish the actual relation and baseline

**SOURCE —** The [child report](/home/charles/repos/Moriarty-factory/docs/results/sherlock-native-a102-a80-child-producer-20261021/actual/RUN-REPORT.json:1) records A102 k13, A80 k12, 3,440-byte proofs and 253 instance lanes each. Program/state widths are 86/81. The producer observation used an unoptimized build; its short phase marks are not portable proof forecasts. The parent receipt pins the same child VK hashes.

**CODE —** `M/crates/mori-backend/src/sherlock_child.rs:244` reconstructs the fixture witness from its public lanes. Both transitions’ complete images are available: fusion does not require acquiring private wallet secrets. `F/src/sherlock_link.rs:452` already shares the program and middle-state cells, prepares both proofs, accumulates once and defers the pairing. Poseidon and pairing deferral are existing optimizations.

**PROPOSAL —** Inventory the actual heterogeneous parent’s advice/table rows, columns, degree, MSM bases/scalar bounds, and phase times before reusing homogeneous-node estimates. Cache PK/SRS at `F/src/sherlock_link.rs:1041`; remove repeat cost modeling at `:1029` from warm service. Cache identity includes circuit/VK/SRS/domain. These changes preserve keys and help every candidate.

### 2. Build a directly composed, ledger-compatible terminal

**PROPOSAL —** Add a bounded `FusedSherlockTerminal` implementation during the implementation phase. Reuse semantic fragments at `M/crates/mori-backend/src/sherlock_machine/literal_bridge.rs:132` and `M/crates/mori-backend/src/sherlock_link.rs:444`. Refactor their surrounding emitters (`:69` and `:390`, respectively) to accept shared operands inside one relation. Existing `LocalPublic` emitters cannot simply be concatenated: both allocate their own public interfaces and output terminators. Introduce a private assignment carrier and expose only the existing terminal template.

**PROPOSAL —** Constrain both enables to one; preserve contexts, envelopes (3,1)/(0,0), program equality and A102.post=A80.pre. Recreate all sixteen fields in `F/src/sherlock_link.rs:519`: keys_id, four context limbs, program/pre/mid/post commitments, two heights, two child-binding lanes, step count, work and ordered trace. Preserve 64-bit height bounds and exact Poseidon domains/order. Pin the old child-key identity as the semantic version whose equations are implemented; the new terminal VK identifies the replacement implementation. This asserts equivalent transition semantics, not that particular child proof bytes were produced.

**CODE / PROPOSAL —** Preserve `F/src/decider.rs:193` statement hashing and `:201` communication opening, including instance_id, batch_no and template ordering. Child binding lanes are committed but **not recomputed as SHA-256 claims in the old circuit** (`F/src/sherlock_link.rs:642`). Preserve that scope explicitly; silently claiming stronger source/provenance authentication would be false. Any strengthening needs its own relation version and accounting.

**ESTIMATE —** Budget the fused terminal at **k15**, not a claimed fit. Existing capacities give `(8192−7)+(4096−7)=12,274` rows for the two separate children. Allocate **8,192** additional rows for composition, commitments and terminal packaging: **20,466 < 32,761** usable k15 rows. Reconfiguration can change fragment costs, so this is a design budget, not an upper-bound proof. Measure advice and lookup-table extents separately; neither may exceed the domain. k14 is a stretch experiment, not the committed target.

**CODE / PROPOSAL —** Ledger compatibility comes from retaining the old outer PLONK/KZG/BLS12-381 protocol, Blake2b transcript, stock gates and official SRS. `Z/lib.rs:1581` reconstructs the constraint system using `configure(architecture,k−1)`; arbitrary gates cannot be installed by uploading a VK. `L/transient-crypto/src/proofs.rs:545` invokes stdlib verification. New terminal keys change the relation/layout, not that verifier. The ledger still checks the outer proof’s KZG pairing; the eliminated *in-circuit* pairing certified child proofs that no longer exist. Qualify against the exact ledger-9 binary, not merely the current ledger checkout.

**ESTIMATE —** Budget **2–4 engineer-weeks** for this fixed fixture: emitter/carrier composition, equivalence and mutation tests, terminal/key-reader integration, measured runs and cold review. A new stagenet deployment is cheap per the brief, but its fee is unmeasured. Price one new key family, deployment/readback, client pins and anchor/replay migration; retain existing wallets/state. No new trusted-setup ceremony is needed when the existing SRS supports the selected degree. General chunking is a separate **6–12+ engineer-week** program, excluding completion of missing interpreter semantics. Keeping keys avoids deployment/state-pin migration; Astra-1 budgets **12–24 engineer-weeks** for a broad GPU backend plus specialized witness acceleration. These are planning estimates, not quotes. Share that backend across both routes rather than paying twice.

### 3. Scale by increasing useful work per proof

| Label | Layer/workload | Advice rows / table rows | k / implication |
|---|---|---|---|
| SOURCE | Actual A102 / A80 | Exact rows unrecorded | 13 / 12 |
| SOURCE | Actual linked parent | Exact rows unrecorded | 20; 882.590 s reference |
| ESTIMATE | Direct fixed-pair terminal | 20,466 budget; tables separately bounded | 15 target |
| SOURCE | Historical synthetic four-transaction leaf | 112,098 / 98,563 | 17 |
| SOURCE | Historical two-child node | 574,040 / 1,032,465 | 20 |
| SOURCE | Historical hardened four-child node | 958,095 / 1,032,465 | 20 |
| SOURCE | Historical terminal decider | 841,682 / 1,032,209 | 20 |
| SOURCE | Historical mono16 synthetic circuit | 449,169 / 360,707 | 19; synthesis/MockProver only |

**SOURCE —** Historical rows come from [recl9 REPORT](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/REPORT.md:106). Its proofs version is 0.8.2, not current 0.8.3; their BLS MSM dispatch differs. The mono16 [cost log](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/mono/logs/cost.log:10) independently supports k19 geometry; its [MockProver acceptance](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/mono/logs/mock-batch.log:1) is not a keyed proof. Its old template leaves communication opening unconstrained (`mono/src/mono.rs:273`), so additional binding work remains. These are geometry references, not current performance predictions. Cost-model “fixed columns” includes permutation columns; do not count both twice.

**PROPOSAL —** For longer executions, compose available-witness transitions into k17–k18 chunks, then recurse only across chunk boundaries. Preserve instruction selection, state openings, fuel, ordering and terminal conditions. Keep independent wallet-proof boundaries where witnesses are unavailable. `F/src/leaf.rs:7` explicitly gives the synthetic prover spend secrets; removing production wallet proofs by centralizing those secrets is not an equivalent optimization.

**ESTIMATE —** Historical node arities suggest roughly `R(b)=190,356+191,842b` before the 371-row hardening offset (190,727 afterward). Five children exceed 1.14M advice rows, so fan-in eight is not a free depth reduction; it requires k21 before glue. Binary trees add verifier instances. This fit applies only to those homogeneous leaf keys, not arbitrary recursive levels. Search fan-in using total measured work, table extent and peak memory.

**SOURCE / CODE —** The [existing residue gadget](/home/charles/repos/projectfenrir/recursion/pairing-gadget/lanes/opus/RESULTS.md:17) has **316,833 rows including subgroup checks**, already above k18’s 262,137 usable rows. Keeping this gadget imposes a **k19 terminal floor**, even with free recursive MSMs. This is an implementation floor, not a cryptographic lower bound. `F/src/pairing.rs:228` already uses residue checking; removing final exponentiation again yields nothing.

### 4. Upgrade inner proof mathematics while retaining the old outer protocol

**CODE —** At U@31435e9, PR #480’s `commit_many` still maps polynomials to individual KZG commitments (`U/proofs/src/poly/kzg/mod.rs:82`). It is an abstraction, not automatic MSM compression. LogUp reads **c+2 commitments and c+3 evaluations** for c chunks (`U/proofs/src/plonk/logup/verifier.rs:57`); the installed lookup reads three/five (`C/verifier/lookup.rs:56`). At c=1 there is no commitment saving. Batch-lookup advantages require a concrete grouping/degree analysis. [PR #480](https://github.com/midnightntwrk/midnight-zk/pull/480), [capture](raw/pr480.md); [LogUp paper](https://eprint.iacr.org/2022/1530), [capture](raw/logup.md).

**SOURCE —** Newer upstream is more promising: [PR #565](https://github.com/midnightntwrk/midnight-zk/pull/565), merged October 14, reports t=4 fflonk at **536 ms versus 415 ms** plain KZG on its k14 zswap example, with VK **578 versus 2,258 bytes**. [Capture](raw/pr565.md). [PR #575](https://github.com/midnightntwrk/midnight-zk/pull/575), merged October 15 as **1e39c12**, adds the in-circuit implementation. [Capture](raw/pr575.md). Both postdate the supplied checkout. The earlier “no in-circuit fflonk” conclusion is obsolete for latest upstream.

**PROPOSAL —** Prototype t=4 inner fflonk/LogUp, translating its complete verifier into stock 7.2.4 outer operations. Chunking reduces commitment points to `sum(ceil(group_size/4))`, not universally one quarter: transcript groups, remainder chunks and opening witnesses remain. The [gadget](https://raw.githubusercontent.com/midnightntwrk/midnight-zk/1e39c12/circuits/src/verifier/fflonk.rs) opens at multiple roots and reads missing evaluations; charge those scalar operations. [Capture](raw/fflonk-gadget.md). Require full-entropy challenges, pinned expanded-SRS provenance, all opening checks and the final pairing. The [PCS parameter loader](https://raw.githubusercontent.com/midnightntwrk/midnight-zk/1e39c12/proofs/src/poly/fflonk/mod.rs) requests k+2 for t=4: k17/k18 inner circuits need 2^19/2^20 parameter domains. [Capture](raw/fflonk-pcs.md). Verify those against the official SRS; the old outer retains its own ordinary KZG sizing. New inner/parent/terminal keys are required. A 29% slower small inner prover can still win if it saves more recursive work; no Sherlock row/time measurements presently establish that tradeoff.

**PROPOSAL —** In parallel, preserve fixed-base information through `C/verifier/msm.rs:481`, shorten bounded-scalar schedules rather than padding every term to the longest (`C/ecc/foreign/weierstrass_chip.rs:1788`), and replan range decompositions. `C/field/decomposition/pow2range.rs:215` already omits unused tags; merely proposing that omission is not new. Retain stock configuration while changing constrained layout. For existing child proofs, merely merging parent and pairing into one terminal is weaker: historical node2 plus the pairing is about 891k rows (k20); node4 plus pairing is about 1.275M (k21), before glue. Eliminating a proof can trigger a domain doubling. k18 requires a 54.3% advice cut for the historical node2, or 72.6% for node4, plus table reduction. Actual Sherlock thresholds remain unknown.

**SOURCE / PROPOSAL —** Test fake-GLV and divisor-certificate MSMs only as bounded research spikes. [Fast scalar multiplication](https://eprint.iacr.org/2025/933) reports 22–53% proving-time improvements in its settings, not a Midnight 75% row reduction; [capture](raw/fast-scalar.md). [ECIP](https://eprint.iacr.org/2022/596), [capture](raw/ecip.md), offers a different verification strategy, but the port needs scalar/decomposition binding, degree bounds, subgroup and exceptional-point checks, and Fiat–Shamir binding of every certificate. Current [Garaga MSM code](https://raw.githubusercontent.com/keep-starknet-strange/garaga/main/hydra/garaga/starknet/tests_and_calldata_generators/msm.py) uses fake-GLV hints; preserved ECIP source is not evidence of a deployed Midnight gadget. [Capture](raw/garaga-msm.md). Give neither an unmeasured multiplicative budget.

### 5. Folding only after charging the complete final wrapper

**SOURCE —** [HyperNova](https://eprint.iacr.org/2023/573) offers CCS folding and multiple-instance folding; [CycleFold](https://eprint.iacr.org/2023/1192) reduces the secondary scalar-multiplication circuit to roughly 1,000–1,500 multiplication gates; [ProtoGalaxy](https://eprint.iacr.org/2023/1106) reduces marginal algebraic verifier work. Captures: [HyperNova](raw/hypernova.md), [CycleFold](raw/cyclefold.md), [ProtoGalaxy](raw/protogalaxy.md). Those quantities are not complete Midnight-decider rows.

**PROPOSAL —** For many repeated chunks, compare a Nova/HyperNova/CycleFold strand with one final old-protocol Midnight wrap. Check both running instances, accumulated satisfiability, commitment openings, recursive input/output hashes and last-fold consistency. A raw-witness final check costs work proportional to the step relation. Compressed verification may use IPA or pairing-based alternatives; [Nova](https://github.com/microsoft/Nova) supports different commitment/compression choices, so a mandatory pairing floor for every folding wrapper would be false. [Capture](raw/nova.md).

**ESTIMATE —** Let an already accelerated tree cost `Na+D`, and folding cost `Nf+W`, including witness/emulation in f and the complete final wrapper in W. Tenfold needs `N(a−10f) ≥ 10W−D`. If f≥a/10 and wrapper overhead is positive enough, tenfold is impossible; asymptotic ceiling is a/f. For a=5 s and f=.8 s, the ceiling is **6.25×**, regardless of strand length. These are illustrative assumptions, not benchmarks. Wrapper rows/k remain **unknown**. An unrelated inner curve adds field-crossing work; BLS12-381/Jubjub is not a recursion cycle.

## Speedup budget

**ESTIMATE —** Use exclusive phase shares, not a product of unrelated headline ratios. For witness share .20, polynomial/MSM share .78, unchanged control .02, plus new overhead .01 of original time, let witness speedup W, domain-work reduction D and GPU bulk speedup G. Then `Tnew/Told=.20/W+.78/(D×G)+.03`. D and G multiply only within the same bulk phase. The original shares sum to one; overhead is additional.

| Label | Scenario | Bulk product | Remaining time | Whole-proof gain |
|---|---|---:|---:|---:|
| ESTIMATE | Same rows; W=1, G=8 | 1×8 | .327500 | 3.053× |
| ESTIMATE | k19; W=2, G=8 | 2×8 | .178750 | 5.594× |
| ESTIMATE | k18; W=4, G=8 | 4×8 | .104375 | 9.581× |
| ESTIMATE | k18; W=5, G=8 | 4×8 | .094375 | **10.596×** |
| ESTIMATE | k18 but twice the effective width; W=5 | 2×8 | .118750 | 8.421× |

**ESTIMATE —** At D=4/G=8, W must reach **4.383562×**. This k18 scenario applies to a parent/chunk, not a terminal retaining the existing pairing gadget. k20→k18 gives fourfold domain shrink at fixed width; FFT work scales approximately 4.44×, but column/degree growth can erase it. Thus “smaller k × GPU” is insufficient without witness savings. Same rows and unchanged witness/control have infinite-bulk ceiling **4.348×** in this model. No measured phase shares support calling that the real Sherlock ceiling. Arithmetic is retained in [budget.json](budget.json).

**SOURCE / ESTIMATE —** Historical synthetic proof-only times are leaves **163.630577**, node **867.279750**, decider **586.381690** seconds (`F/out/{leaves-b1,node-b1,decider-b1}/*.stats.json`): total **1,617.292017**. Parent-only 10× gives **1.933×** overall; free parent gives **2.156×**; free leaves+parent gives **2.758×**; free node+decider with unchanged leaves gives only **9.884×**. Serial reference throughput is **2.22594 batches/hour**; tenfold needs **≤161.7292 seconds/output**. These are exact conditional ceilings for that cohort, not current ledger throughput.

**ESTIMATE / PROPOSAL —** PK caching alone gives `(882.590+62.328)/882.590=1.07062×`, not ten. Direct fusion removes stages and requires a fresh end-to-end budget; do not multiply a 32× domain ratio by that cache factor and a GPU headline. Test its small k15 circuit on CPU first. For remaining large chunks use resident GPU commitments at `P/poly/kzg/mod.rs:74`, quotient evaluation at `P/plonk/evaluation.rs:285`, and witness processing. Smaller domains improve admission on 32-GB VRAM, but synthesis RSS is not device memory and small circuits may lose to launch overhead.

## Risks and soundness

**CODE / PROPOSAL —** The current fixed relation leaves Init, projectA, StepInv, full dispatch and other obligations open (`F/src/sherlock_link.rs:18`). Fusion preserves only the constrained subset. Reconstruct the original child VKs from pinned emitter/SDK versions before composition; compare the complete constrained fragments and their wire mapping, not only generic gate descriptions. Preserve the anchor’s checks of predecessor root/height, instance identity and replay/batch number; the circuit alone does not anchor them. Require differential relation tests and adversarial mutation of every state/program lane, contexts, envelopes, height overflow, trace order, instance/batch identifiers and communication opening. Host success flags and hashes alone are not proof. New-key qualification must independently reconstruct the exact implemented relation and reject wrong key/SRS/template identities.

**PROPOSAL —** Keep canonical encodings, challenge entropy, subgroup checks, blinding and all deferred obligations. GPU verification must run through an independent CPU verifier. Deterministic byte fixtures additionally control lookup HashMap ordering (`P/plonk/lookup/prover.rs:399`) and circuit randomness; production proofs retain fresh randomness. Never expose private folding witnesses. No general-wallet custody change, unsafe subgroup omission or ledger consensus upgrade is budgeted.

## Validation: factory predicates, id/check/tier

**CODE —** Read-only `scripts/sherlock-next` named **A2** first failing. A2–A6 are presently unimplemented checks. A3 specifically requires the k20 recursive parent; fusion does not pass it. A8 names the existing contract; a new deployment does not pass that unchanged predicate either. The factory schema permits A/B/C/U/F, not the design’s proposed L set. No repository code, build, proof, key or deployment was changed in this research session.

**PROPOSAL —** Register separate experimental predicates in set F during implementation; initially use `scripts/qa/unimplemented.sh ID`. Proposed replacement runners below are not existing commands. Invoke each through `scripts/doctorwatson check ID`; retain the original A/B requirements and evaluator-owned holdouts.

| Proposed ID | Proposed check runner | Tier | Required result |
|---|---|---|---|
| F141 | `scripts/qa/check-loki-fusion-equivalence.sh` | T1 | Original fragments versus fused relation; required mutations reject |
| F142 | `scripts/qa/check-loki-fusion-geometry.sh` | T2 | Actual rows, tables, columns, degree; k15 budget survives |
| F143 | `scripts/qa/check-loki-newkey-reader.sh` | T2 | Stock processed-VK round trip; independent verification |
| F144 | `scripts/qa/check-loki-paired-latency.sh` | T2 | Paired complete CPU/new-chain runs; median/p95 and all failures |
| F145 | `scripts/qa/check-loki-inner-fflonk.sh` | T2 | Complete verifier differential; full challenges; total stack win |
| F146 | `scripts/qa/check-loki-fold-wrapper.sh` | T2 | Complete primary/secondary checks; actual wrapper rows/time |
| F147 | `scripts/qa/check-loki-newkey-ledger9.sh` | T2 | Exact local ledger-9 positive and negative acceptance |
| F148 | `scripts/qa/check-loki-newkey-stagenet.sh` | T3 | New contract/key pins, finalized call and state readback |
| F149 | `scripts/qa/check-loki-rollup-throughput.sh` | T2 | Real 16-transaction semantics, fair scheduled CPU baseline |

**PROPOSAL —** Run heavy checks through measured-cap `doctorwatson heavy`; failure means read trace, state a one-line hypothesis, retry with `--hypothesis`, and shrink after two same-kind failures. Apply the stop rule. Require at least three paired repetitions for latency and sustained completed-work runs for throughput; separate startup/fill/drain. Preserve B3–B9 scope, including B7’s under-16-GB host-memory/no-swap requirement. T3 uses the factory’s read-only cold review with its one-hour nonblocking limit. A research budget passing is not product acceptance.

## What the other leads got right/wrong

**ESTIMATE —** Astra-1 correctly rejects a universal 4× ceiling and identifies witness acceleration as necessary: its assumed `.20/8+.78/20+.03=.094` yields 10.638×. I agree with the arithmetic, not an established performance forecast. Our exchange corrected the completed parent receipt’s lineage: Source44 is not automatically today’s registry build. Same-key GPU/witness work remains valuable for workloads that fusion cannot replace.

**ESTIMATE —** Astra-3’s queue model gives **10.454× versus an old serial cohort**, with only about **4.54%** margin; a better CPU scheduler or contention can erase it. Its later eight-worker/20×-bulk scenario reaches 10.472× against an assumed 2× better CPU baseline, but worker service and memory admission remain unmeasured. Its initial universal two-lane/2× overlap limit assumed saturated CPU capacity and was corrected. I agree with its distinction between throughput and latency, and its insistence that fixed-pair fusion does not establish a real 16-transaction rollup result. My own initial upstream-PCS assessment was corrected by live PR #575 evidence.

## Helper contributions used

**CODE / PROPOSAL —** haiku-05 supplied row/table/MSM inspection (`astra-2-1`) and ECIP audits (`-4`, `-7`, `-10`); adopted per-point loop analysis only after correcting mistaken digit counts. haiku-06 supplied LogUp/PCS counts (`-2`), direct-terminal analysis (`-5`), equivalence review (`-9`) and current fflonk follow-up (`-11`). haiku-07 supplied folding boundaries (`-3`), fusion evidence (`-6`, `-8`) and arithmetic audit (`-12`). Reports live in `../helpers/haiku-NN/`. Unsupported folding pairing floors, child-custody claims, proxy-to-Sherlock row transfers and stale fflonk absence claims were rejected. Helpers are research inputs, not validation receipts.

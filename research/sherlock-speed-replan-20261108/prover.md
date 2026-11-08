# prover: make each proof faster and smaller in memory

Agent: `prover`. Date: 2026-11-08. Read-only; no builds, no proofs run. Every number
marked ESTIMATE carries its arithmetic. Web content is cited as data.

## 0. Facts gathered this session

| Fact | Value | Source |
|---|---|---|
| Node at old pins (`04c9c5d9`: proofs 0.8.3, circuits 7.2.4, stdlib 2.3.5) | k=20, 531,989 rows (2^19 = 524,288; over by 7,701 rows = 1.47%), ~50 s keygen, ~95 s/step, 26.2 GB, 63 instance lanes, proof 6,432 B | `docs/sherlock/stagenet-gp-recursion.md:189-193`; `design.md:78` |
| One-fragment digest leaf (M2) | k=13, 2.55 s, 3,824 B, 96 private lanes | `stagenet-gp-recursion.md:185-188` |
| Run decider | k=20, 833,235 rows, ~100 s | `stagenet-gp-recursion.md:194-196` |
| Node arch | `bls12_381 + poseidon`, `nr_pow2range_cols = 4` (the maximum the relation allows), self-verify and leaf-verify folded, one shared collapse MSM | `crates/mori-recursion/src/lib.rs:23-30`, `ivc.rs:137,350-352,367-370`, `tests/sherlock_gp.rs:218,298` |
| Leaf arch (ZKIR) | chips chosen from the IR: poseidon only for a C6 leaf, `nr_pow2range_cols: 4`, `bls12_381: false` | `repos/midnightntwrk/midnight-zkir/zkir/src/ir_vm.rs:1616-1663` |
| Host | aarch64: 10x Cortex-X925 + 10x Cortex-A725 (20 cores, no SMT), 126,120,948 kB RAM, NVIDIA RTX Spark N1X = GB10-class Blackwell, 6,144 CUDA cores, unified LPDDR5X, 273 GB/s | `lscpu`, `/proc/meminfo`, `nvidia-smi`; [Tom's Hardware](https://www.tomshardware.com/pc-components/gpus/nvidia-dgx-spark-review), [cnx-software](https://www.cnx-software.com/2025/03/19/nvidia-dgx-spark-a-desktop-ai-supercomputer-powered-by-nvidia-gb10-20-core-armv9-soc-with-1000-tops-of-ai-performance/) |
| CUDA | driver 615.78.08 with CUDA UMD 13.4; **no `nvcc`, no toolkit installed**; `pacman -Si cuda` offers 13.4.3-1 (5.07 GiB); compute capability 12.1 (`sm_121`), CUDA 13.0+ required | `nvidia-smi`, `pacman -Q`; [NVIDIA forum sm_121](https://forums.developer.nvidia.com/t/request-for-sm-121-tuned-kernels-in-cudnn-cublas-dgx-spark-training-throughput-gap/371094), [dgx-spark-setup](https://github.com/natolambert/dgx-spark-setup) |
| The "95 GB" test | observed live at 21:37: `mori_backend-8a41ff681892bd05` (dev profile, `moriarty-target-m2`), **108,133,508 kB RSS (85.7%), 1805% CPU**; MemAvailable 2.0 GB; it exited by 21:45 | `ps aux --sort=-rss`, `/proc/meminfo` |
| Upstream clone | `repos/midnightntwrk/midnight-zk` at `24a1eb5a` (2026-11-05) "Merge zk stdlib v2 (#616)": circuits 7.3.0, stdlib 2.4.0, proofs 0.8.3 **plus long `[Unreleased]` sections** and a new `aggregation` crate (ivc + multi-circuit aggregator) | `*/CHANGELOG.md`, `aggregation/src/ivc/` |
| crates.io dates | circuits 7.3.0 = 2026-10-30; proofs 0.8.3 = 2026-10-01 (the ledger 9.1.0.1 pin); nothing newer published | [crates.io circuits](https://crates.io/api/v1/crates/midnight-circuits/versions), [crates.io proofs](https://crates.io/api/v1/crates/midnight-proofs/versions) |

Cost model used below (ESTIMATE): prove time per row at the same prover and host.
Node: 95 s / 2^20 rows = 0.091 ms/row. One-fragment leaf: 2.55 s / 2^13 = 0.31 ms/row
(fixed overheads dominate a k13 run). I use 0.09 ms/row as the central value for a lean
ZKIR leaf and 0.3 ms/row as the upper bound. Memory: node 26.2 GB at k20; I scale RSS
linearly with 2^k for the same column set (~13 GB at k19).

## 1. Findings, ranked

### F1. After S=8 the node is no longer the critical path; the universal leaf's k is
`t_total(S) ≈ 293 × t_step + (293/S) × t_node + t_decider`. The node term shrinks with S,
the leaf term does not: every owner step pays one universal step's rows regardless of S.
So the single number that decides whether 2 h is reachable is `k_U`, the universal
leaf's k at S=1 (M4, SRC-013: U if k_U ≤ 17). With `t_step = 0.09 ms × 2^k_U`:
k_U=15 → 3 s; 16 → 6 s; 17 → 12 s; 18 → 24 s per step (upper bound 3.3× these).
Measurement that decides: M4 row count and the C0 `synthesis` (phase 7) share of the leaf.

### F2. S=8 alone brings fact(3) under 2 h on CPU if k_U ≤ 16 (ESTIMATE)
37 leaves (36×8 + one chunk of 5, 3 padded slots = 1.0% waste). Leaf at k_U+3 = k19:
47 s (0.09 ms/row) to 157 s (0.3 ms/row). Node 95 s unchanged.
Serial: 37 × (95 + 47) + 100 = **5,354 s = 1.49 h** (k_U=16); 37 × (95 + 94) + 100 =
7,093 s = 1.97 h (k_U=17, leaf at k20); fails at k_U=18 (S capped at 4 by k ≤ 20:
74 × (95 + 94) + 100 = 14,086 s = 3.9 h). Memory: one prover at a time, ≤ 26 GB.

### F3. The 1.5% at k=20 is inside the noise of the re-pin; measure before trimming
The 531,989 rows were measured at circuits 7.2.4. The 7.3.0 release (what task 2.1
pins) changes `as_public_input` to a canonical encoding for foreign-field elements and
foreign Weierstrass points and makes `assign_as_public_input` run the full `assign`
checks (#612, #598, `circuits/CHANGELOG.md:83-99`). The node carries two BLS12-381 points
and their scalars in its public instance (`ivc.rs:187,370`; `inner.rs:38-49`), so the row
count moves at 7.3.0, and more likely up than down (canonical-encoding checks add
range comparisons; ESTIMATE +0.5k to +3k rows). Task 2.6 must print the row count
(`cost_model`/dev row report) before anyone trims. See §2 for the trim list.

### F4. The node's own proof format is free to change; only the decider's is not
Stock Midnight verifies the decider proof (midnight-proofs 0.8.3 format, ledger 9.1.0.1).
The leaf and node proofs are verified only by our circuits, but those circuits are built
with the same `midnight-circuits` verifier gadget, which only verifies proofs of the
`midnight-proofs` version it is compiled with. One workspace, one version: the pin is
all-or-nothing. Consequence: the upstream `[Unreleased]` verifier-gadget row cuts (F9)
are unusable until a ledger release pins a proofs > 0.8.3. GPU/CPU backend swaps (F5-F7)
do not touch the format at all: an MSM or NTT computed elsewhere must give the same
bytes, and that is checkable against the stock verifier we already run (A5/G10 pattern).

### F5. GPU on this host: real but modest; the memory bus is shared with the CPU
GB10 has 6,144 cores (an RTX 5070's count), FP32 31 TFLOPS, 273 GB/s unified memory.
Field arithmetic runs on the int32 pipeline and NTT is bandwidth-bound; ZKProphet
(arXiv 2509.22684) reports that once MSM is optimised NTT is up to 90% of GPU proof
latency and that performance is capped by integer units, not FLOPS
([abstract](https://arxiv.org/abs/2509.22684)). On a desktop card the CPU and GPU have
separate memory; here every GPU byte competes with the CPU prover for the same 273 GB/s.
ESTIMATE for MSM: a 4090 does a 2^20 BLS12-381 MSM in ~15-25 ms (sppark class); GB10 at
~0.35× the int32 throughput → 50-80 ms; BLST on these 20 ARM cores ≈ 0.4-0.8 s → MSM gain
5-10×. At a 30% MSM share (C0 owner `commitment`, phases 9, 15, 18, 20, 21):
1/(0.7 + 0.3/7) = **1.35×**; at 50%: 1/(0.5 + 0.5/7) = 1.75×. NTT/quotient on GPU:
2-4× on the stage at best because of the shared bus (ESTIMATE, no GB10 measurement found).
Unified memory removes sppark's per-call copy weakness noted by Argon, but every GPU
allocation counts against the same per-process 48 GB and aggregate 100 GB caps.

### F6. GPU library survey (what plugs into midnight-proofs)
Integration point in all cases: `midnight-proofs` `poly/kzg` commit path, where
`msm_specific` always calls BLST `multi_exp_affine` for G1 at 0.8.3 (`proofs/CHANGELOG.md:141`,
PR #536), and `poly/domain.rs` `best_fft`. A `[patch.crates-io]` of midnight-proofs 0.8.3
with a `gpu` feature; proof bytes must be identical.

| Library | State found | Blackwell / aarch64 | BLS12-381 | Integration cost | Risk |
|---|---|---|---|---|---|
| **ICICLE** (Ingonyama) | latest tag v4.0.0 (11 Jul; v3.8.0 added RTX 40/50 builds; v4.0.0 "[FIX] compilation for arm") ([releases](https://github.com/ingonyama-zk/icicle/releases)) | RTX 50 series yes; `sm_121` not named; ARM host fix exists, Linux-aarch64 CUDA backend binary unverified | yes (`icicle-bls12-381` crate) | Rust crates exist (`icicle-runtime`, `icicle-core`, curve crate): 1-2 weeks incl. byte-identity tests | **CUDA backend is closed source and needs a licence**: free R&D licence via Ingonyama's licence server (network), production needs sales ([install_gpu_backend](https://dev.ingonyama.com/start/architecture/install_gpu_backend)). Node-locked licence possible. |
| **sppark** (Supranational) | repo active to 2025-03 ([libraries.io](https://www.libraries.io/cargo/sppark)); no releases; Apache-2.0 | Volta+; README: "primarily supports x86_64 ... only non-GPU portions on ARM64" ([repo](https://github.com/supranational/sppark)) | yes | Rust crate + our own `.cu` glue via `nvcc` (needs the `cuda` package); 2-3 weeks | **aarch64 host unsupported as shipped**; CUDA 13 untested upstream; our own kernels to maintain |
| **cuZK** | 2 commits, CUDA 11.5, V100; 2^20 BLS12-381 MSM 0.19 s vs 0.41 s bellperson ([repo](https://github.com/speakspeak/cuZK)) | no | yes | research code; port to CUDA 13 + Rust FFI: weeks | unmaintained; TCHES'24 follow-up ([Chen et al.](https://tches.iacr.org/index.php/TCHES/article/view/11438)) is 1.8-2× faster but also V100-era |
| **halo2 GPU backends** | PSE halo2 blackboxed MSM/FFT behind halo2curves ([RFC #84](https://github.com/privacy-scaling-explorations/halo2curves/issues/84), [#216](https://github.com/privacy-scaling-explorations/halo2/issues/216)); EZKL+ICICLE enables GPU MSM only for k > 8 and targets the KZG aggregate circuit ([hackmd](https://hackmd.io/@swezkl/HJRCV5MSp)); Ingonyama's halo2 fork benchmarks RTX 4080/4090; Taiko ran GPU provers on modified halo2 | BN254 mostly | mostly BN254 | **Not reusable**: midnight-proofs has its own `midnight-curves`/BLST path and its own transcript and PCS; only the dispatch pattern transfers | none of them produce Midnight-format proofs |

Verdict: ICICLE is the only candidate with a supported Rust path and recent Blackwell
builds; its blockers are the licence server and an unverified Linux-aarch64 CUDA backend.
First step is free: `pacman -S cuda`, build ICICLE's `icicle-bls12-381` example, time a
2^20 G1 MSM on this GPU against BLST on the 20 cores. If the aarch64 backend is missing,
GPU is dead on this host until Ingonyama ships it, and sppark means writing and
maintaining our own kernels.

### F7. CPU-side: parallelism headroom is the biggest unmeasured CPU lever
The recl9 k20 node averaged 268% CPU on 20 cores (`stages.json` s7). If the Sherlock
node is similar, 87% of the machine idles during a proof: halo2 synthesis is
single-threaded by construction, and the verifier gadget's foreign-field witness
(inversions per MSM step) is synthesis. ESTIMATE: synthesis 40% of 95 s = 38 s; a 4×
parallel witness (C4) saves 28 s → 1.42×; at 25% share → 1.23×. Decided by C0 phase 7
(`synthesis`) wall and CPU%. Cheaper first: the dev profile leaves Moriarty's own crates
at opt-level 0 (`Cargo.toml:75-86`); the C6 witness builder and digest code run
unoptimised under `cargo test`. Run heavy proofs with `--release` or add a `[profile.test]`
override: ESTIMATE 1.0-1.3× on leaf witness time, zero risk.

### F8. Memory: 26 GB is the extended-domain quotient; 108 GB was test parallelism
At k20 with a constraint degree d, the quotient evaluator holds every column on the
extended domain 2^20 × 2^ceil(log2 d) (×8 for d ≤ 9): 256 MiB per column; ~60-80 columns
(advice incl. 4 range columns, fixed, permutation, lookup) → 15-20 GB, plus pk ~3 GB and
tables (ESTIMATE; matches 26.2 GB). C7 (coset-tiled quotient) evaluates one coset of 2^20
at a time: column working set ÷8 → node RSS ESTIMATE 8-10 GB at k20, 4-5 GB at k19.
The 108 GB RSS was one `cargo test` binary of `mori-backend` in the dev profile with the
default `--test-threads = nproc = 20`: several k13-k20 provers plus keygens at once. The
QA scripts already serialise (`scripts/qa/check-*.sh` use `--test-threads=1`); the raw
`cargo test` path does not. Fix is procedural, not algorithmic (§3 task P0).

### F9. Upstream after 7.3.0 / 2.4.0 (local clone `24a1eb5a`, 2026-11-05)
- **`midnight-aggregation`**: upstream's own IVC (`aggregation/src/ivc/{circuit,prover,setup,verifier}.rs`) and a multi-circuit aggregator; published 0.1.0 on 2026-09-22 against circuits 7.0.0 ([docs.rs](https://docs.rs/crate/midnight-aggregation/latest/source/CHANGELOG.md)); HEAD adapts it to the unreleased gadget (`aggregation/CHANGELOG.md`). It mirrors our `ivc.rs` shape (`assign_vk_as_public_input`, carried accumulator). Its example row count is a free reference for the self-verification floor at the pinned version.
- **Unreleased verifier-gadget row cuts** (`circuits/CHANGELOG.md:9-82`): #566 linearization removed (no per-selector commitments), #547 fixed columns committed as one group, #582 quotient limbs as one group opened one by one (no full-width scalars), #579 collapsed accumulator with constant scalar 1 and short post-accumulation scalars, #565 barycentric point-set quotients without Lagrange bases, #467 Lim-Lee comb fixed-base MSM, #583 multiopen point-set ordering. ESTIMATE 5-15% fewer verifier rows, enough on its own to put the node under 2^19. All change VKs **and** the proof format (`proofs/CHANGELOG.md:9-138` is all unreleased), so per F4 they wait for the next ledger pin. Worth one experiment branch to measure the row count, not to ship.
- **Unreleased prover-only changes that are backportable**: "Parallelize logup, permutation, and SHPLONK" (#353, `proofs/CHANGELOG.md:104`) and "Store SRS as affine and use an affine MSM path in KZG" (#350, `:101`). Neither changes proof bytes. These are C6 and C9 done upstream; backporting them onto a vendored 0.8.3 is days of work and is confirmed or killed by C0 phases 10-12 and 17.
- Already in 0.8.3: the BLST-always MSM (#536) that removes the 7× cliff above 2^19 points. C8 is moot at our pin.
- No newer crates.io release and no GitHub releases page entries ([releases](https://github.com/midnightntwrk/midnight-zk/releases) is empty). No GPU work anywhere in the upstream tree (`grep -ril gpu|cuda|icicle|sppark` over the clone: nothing).

### F10. C10 (persistent prover) is a shape, not a speedup, at our chain length
Within one process the node pk stays resident across the 37 steps already
(`gp_fold` loop). Loading a k20 pk from disk is ESTIMATE 1-5 s against a 50-95 s proof:
≤ 1.05×. C0 phases 2-3 (`key_load`, `setup_pk`) decide. Keep it as the orchestration
shape (one long-lived process, keys loaded once, two provers in flight) because the
overlap in F11 needs it.

### F11. Leaves are independent: under the 100 GB aggregate cap they vanish behind the node chain
Owner decision 2026-11-08: 48 GB per process, 100 GB aggregate. Leaves do not depend on
each other, only the node chain is sequential. With P leaf workers ahead of the chain:
`T ≈ max(N × t_node, N × t_leaf / P) + t_node + t_decider`. Memory: one node (13 GB at
k19, 26 GB at k20) + P leaves (13-26 GB each) ≤ 100 GB gives P = 2 at k20/k20, P = 3-5
otherwise. With P = 3, 37 leaves at 47-94 s take 580-1,160 s per worker, below the
node chain's 1,850-3,515 s, so **T ≈ 37 × t_node + 150** and k_U drops out of the
wall-clock as long as k_U ≤ 17. CPU contention is the cost: the node prover averages
~270% CPU (recl9) with bursts to 20 cores in MSM/NTT; three leaf provers beside it slow
every phase (ESTIMATE 1.2-1.5× on each proof during overlap; the C0 CPU% per phase
decides). Needs the second heavy-lock class with aggregate RSS accounting (E11).
The same cap lets the tree run 3 node proofs at 26 GB (78 GB); see `tree.md` F11.

## 2. Node at k=19: what to trim, what SRC-004 adds

Gap: 7,701 rows (1.47%) at 7.2.4; unknown at 7.3.0 (F3). Blinding rows are negligible
(single digits). Row costs below are ESTIMATE, derived from "+191k rows per extra k17
child" in recl9 (`stages.json` s7) and the shared windowed MSM in
`circuits/src/verifier/msm.rs:582-633` (`S::msm` over all variable bases at once, so
the per-base cost is the per-window additions, not a full double-and-add chain):
one variable base ≈ 3-4k rows; one absorbed commitment or ~2 absorbed scalars ≈ 60-120
rows of Poseidon; a 64-bit `assert_less_than_pow2` ≈ 8-16 rows.

Where the rows are (ivc.rs): (1) `assign_vk_as_public_input` + `prepare` of the self
proof with 63 absorbed lanes (`:295-301,343-349`); (2) `prepare` of the leaf proof,
7 lanes (`:356-365`); (3) `accumulate` of three accumulators = Poseidon over all
encodings + scalar folds (`:368`); (4) one `collapse` = one shared variable-base MSM over
every variable base of the self proof, the leaf proof and the carried accumulator (`:369`).
Item (4) dominates; items (1)-(2) are second.

Trim candidates, ordered by expected rows per unit of work:

| # | Trim | Rows (ESTIMATE) | Keys / format | Cost | Decided by |
|---|---|---|---|---|---|
| T1 | Re-measure at 7.3.0 (task 2.6) and read the per-gadget row report before any change | ±0.5-3k (direction unknown) | VK changes anyway (re-pin) | 0 | row report of 2.6 |
| T2 | `nr_pow2range_cols`: it is already 4 (max). Try 3: one fewer advice column = one fewer variable base in the self MSM and one fewer absorbed commitment (≈ -3.5k) against more range-check rows (+?k). Only a measurement can sign this | -3.5k + x | node VK | hours (one constructor argument) | synthesize at nr_cols 3 and 4 |
| T3 | Carried-accumulator lanes: the 46 self fixed-base scalar lanes are absorbed as public inputs of the previous proof. Hashing them to one lane moves the same Poseidon work in-circuit; net ≈ 0. Not a trim | 0 | - | - | - |
| T4 | Drop `rule` and `binding` from the node's leaf instance (keep them inside `rules_d`, D5) | ≈ -100 | leaf + node VK | small | - |
| T5 | Leaf column count: ZKIR already enables only the chips the IR uses (`ir_vm.rs:1641-1663`), so no unused advice columns to remove; `nr_pow2range_cols: 4` is hard-coded there | 0 | - | - | - |
| T6 | Unreleased gadget (#582, #579, #566): quotient limbs grouped, short scalars, no selector commitments | -25k to -80k | proof format: blocked (F4) | experiment branch only | row count on the clone HEAD |

Honest reading: T1 decides; T2 is the only in-house trim large enough to matter and it
can go either way. If 7.3.0 lands above 2^19 and T2 fails, k=19 waits for the next
ledger pin (T6). The node at k=19 is worth ESTIMATE 95 → 48-55 s (n log n for NTT,
~n for MSM, so 0.50-0.55×) and 26 → ~13 GB.

**SRC-004 range checks the node adds back** (`requirements.md:26`): `steps ∈ [1, S]`
(bound S+1 is not a power of two: `assert_lower_than_fixed` does one `select`, one
`add_constant` and one `assert_less_than_pow2` at ⌊log2 S⌋ bits, `native_gadget.rs:274-306`;
≈ 10 rows), `prev_h < 2^64` (≈ 8-16 rows), and `h = prev_h + steps < 2^64` (≈ 8-16 rows)
or equivalently `prev_h < 2^64 - S`. Total **ESTIMATE 30-50 rows, 0.006-0.01% of the
node**. They do not threaten k=19. The existing `assert_not_equal_to_fixed(steps, 0)`
(`ivc.rs:313`) becomes redundant once `steps ≥ 1` is range-checked. These checks change
the node VK (and therefore the decider VK and the contract registration), not the format.

## 3. S>1 steps per leaf

**Growth.** Leaf rows ≈ S × rows(one universal step) + (S-1) × lane equalities + two
digests (`design.md §2.3`). So k_leaf = k_U + ⌈log2 S⌉, prove time ≈ S × t_step, RSS ≈
RSS(k_U) × S. Cap: k_leaf ≤ 20 (SRS and the 48 GB cap: a ZKIR k20 leaf is ESTIMATE
15-26 GB), so S ≤ 2^(20 - k_U): k_U=15 → S ≤ 32; 16 → 16; 17 → 8; 18 → 4.

**Best S for 2 h / 48 GB (ESTIMATE).** `T(S) = 293 × t_step + ⌈293/S⌉ × t_node + 100`
is monotone in S, so take the largest S that (a) keeps k_leaf ≤ 20, (b) keeps padding
small (S=8: 3 of 296 slots = 1.0%; S=16: 11 of 304 = 3.6%), and (c) balances
`t_leaf(S) ≈ t_node` for the overlap of F11. With node k19 (50 s) and k_U=16 (6 s/step):
S=8 gives t_leaf = 47 s ≈ t_node. **S=8** is the recommendation at k_U ≤ 17; S=4 if
k_U=18; S=16 only if the node stays at k20 and k_U ≤ 16.

| k_U | S | leaves | serial, node k20 | serial, node k19 | overlapped, node k19 |
|---|---|---|---|---|---|
| 16 | 8 | 37 | 37×(95+47)+100 = 5,354 s (1.49 h) | 37×(50+47)+100 = 3,689 s (1.02 h) | 37×50+150 = 2,000 s (0.56 h) |
| 17 | 8 | 37 | 37×(95+94)+100 = 7,093 s (1.97 h) | 37×(50+94)+100 = 5,428 s (1.51 h) | 37×94+150 = 3,628 s (1.01 h) |
| 18 | 4 | 74 | 74×(95+94)+100 = 14,086 s (3.9 h) | 74×(50+94)+100 = 10,756 s (3.0 h) | 74×94+150 = 7,106 s (1.97 h) |

**What S>1 changes.** Leaf statement: `rule` becomes `rules_d` (Poseidon of the S
selected rule ids, D5); `steps` counts enabled slots; disabled slots are identity on the
lanes; a publishing step is the last enabled one (`design.md §2.3`). Leaf VK changes
(S is a circuit constant), hence the node VK (leaf VK is a node constant), hence the
decider VK and the contract registration. Node: `h = prev_h + steps` already
(`ivc.rs:314`); it gains the SRC-004 checks (§2) and `rules_d` in the leaf instance.
Soundness obligations: SVM-004 (an enabled slot with no `sel` bit refuses), padding
contributes zero to `steps`, and `steps ≤ S` is enforced in the node, not trusted from
the leaf. Format unchanged.

## 4. Lever table

Gain bounds are per-proof unless marked; the stage share each assumes is named with the
C0 phase ids from `scripts/qa/profile-recursion.py:234-247`.

| Lever | Gain (ESTIMATE) | Assumed share | C0 number that confirms or kills | Cost | Keys / format |
|---|---|---|---|---|---|
| S=8 (F2) | node count 293 → 37: 7.9× on the node term; leaf term unchanged | none (counting) | M4 k_U; phase 7 of the leaf | leaf relation change + node lane change; M4 then 4.8 | leaf, node, decider VKs; format no |
| Node k=19 (§2) | node 95 → 48-55 s (0.50-0.55×); RSS 26 → 13 GB | whole proof scales with n | row report at 2.6; `ntt` (22) and `commitment_msm` (18) scale | hours (T2) or wait (T6) | node + decider VK; format no |
| Leaf workers ahead of the chain, P=2-3 (F11) | removes the leaf term: `(t_node + t_leaf)/t_node` = 1.5× (k_U=16, k19 node) to 2.0× (k_U=17), minus contention 1.2-1.5× | leaves ≤ 47-94 s, node ≥ 50 s; ≤ 100 GB aggregate | phase 4 wall and CPU% with 0/2/3 leaf workers; aggregate RSS | orchestration + lock class (E11): days | none |
| GPU MSM via ICICLE (F5-F6) | 1.35× at 30% MSM share, 1.75× at 50%; `1/(1-m+m/7)` | m = owner `commitment` (9, 15, 18, 20, 21) | phases 9/15/18/20/21 share and the 2^20 MSM micro-benchmark on this GPU | `pacman -S cuda`, licence, patched proofs 0.8.3: 1-2 weeks | none (byte-identical proofs required and tested) |
| GPU NTT/quotient | 1.1-1.4× overall; `1/(1-q+q/3)` at q = 25-40% | q = `quotient` (13, 14) + `ntt` (22) + `advice_ntt` (24) | phases 13/14/22/24 | cuSnark-class port: weeks | none |
| Parallel witness, C4 (F7) | 1.23-1.42×; `1/(1-s+s/4)` at s = 25-40% | s = `synthesis` (7) | phase 7 wall and CPU% | weeks inside vendored gadgets; `--release` first (1.0-1.3× free) | none |
| Backport #353 parallel logup/permutation/SHPLONK (F9) ≈ C6 + C9 | 1.1-1.3× at a 25% lookup+permutation+multiopen share | phases 10, 11, 12, 17 | those phases' CPU% < 400% | days | none |
| C7 coset-tiled quotient (F8) | time 1.0-1.2×; RSS 26 → 8-10 GB at k20 | phases 13-14 and `validate_memory` RSS | RSS samples per phase | 1-2 weeks | none |
| C10 resident keys (F10) | ≤ 1.05× | phases 2-3 | phases 2-3 | shape only | none |
| Test-harness memory (F8) | removes the 108 GB spike | - | - | hours | none |

## 5. Recommended plan (ordered tasks)

Each task: requirement change, predicate, measurement.

- **P0 Stop the memory spike.** Requirement: SRC-011 gains "every heavy proof runs under a
  memory cap". Predicate: `RUST_TEST_THREADS=1` via `[env]` in `.cargo/config.toml` for
  the `mori-backend` and `mori-recursion` test targets, heavy tests `#[ignore]`d and
  driven only by `scripts/qa/check-*.sh`, which already pass `--test-threads=1`; heavy
  runs wrapped in `systemd-run --scope -p MemoryMax=48G` so an overrun fails instead of
  swapping the host. Measurement: peak RSS of one QA run ≤ 48 GB in `runs.jsonl`.
- **P1 C0 profile at the new pins (tasks 2.6 and the `recursion-phase-profile` change)**,
  with three additions: (a) print the node row count and the per-gadget breakdown
  (`verifier` prepare self, prepare leaf, accumulate, collapse), (b) CPU% per phase, not
  only wall, (c) RSS per phase. Requirement: RPP rows gain CPU% and row count. Predicate:
  the profile sums to ≥ 95% of wall (the harness's `coverage_bounds`). Measurement:
  `entry 7;` chain, one node step, S=1.
- **P2 M4 first, before any S or GPU choice**: the universal leaf's k_U and RSS at S=1.
  Requirement: SRC-013 as written. Predicate: U if k_U ≤ 17. Measurement: row count and
  prove time at S=1.
- **P3 S=8 (task 4.8 moved before M5, not after 5.3)**. Requirement: SRC-014 "S is a
  committed leaf constant; node refuses `steps ∉ [1, S]`"; SRC-011 becomes mandatory.
  Predicate: fact(3) runs as 37 leaves + 37 nodes + decider, every node refuses
  `steps = 0`, `steps = S+1`, `prev_h = 2^64-1` (SAS-003 boundary). Measurement:
  `budgets.json` rows `leaf-s8`, `node`, `decider`; total wall and peak RSS.
- **P4 Node k=19 (T1 then T2)**. Requirement: none new (design §3.5 already names it).
  Predicate: node rows ≤ 2^19 − blinding at 7.3.0 with SRC-004 checks included.
  Measurement: row report; node prove time and RSS at k19.
- **P5 Leaf workers ahead of the node chain** (P = 2-3) under a second heavy-lock class
  with aggregate RSS accounting (E11). Requirement: SRC-011 "each heavy process ≤ 48 GB,
  concurrent heavy processes ≤ 100 GB aggregate" (owner decision 2026-11-08). Predicate:
  every process peak ≤ 48 GB and the sampled aggregate ≤ 100 GB, enforced by
  `systemd-run -p MemoryMax=48G` per process and a slice `MemoryMax=100G` over all of them.
  Measurement: wall of the 37-step chain vs the serial sum; per-phase CPU% of the node
  with 0, 2 and 3 leaf workers beside it (the contention number).
- **P6 GPU go/no-go in one day**: `pacman -S cuda`; build ICICLE `icicle-bls12-381` on
  aarch64; obtain the free R&D licence; time a 2^20 G1 MSM vs BLST on 20 cores.
  Requirement: none until it passes. Predicate: ≥ 5× on the MSM micro-benchmark and
  bit-identical results. Measurement: micro-benchmark log in `runs.jsonl`. If it fails
  (no aarch64 backend, no licence), close the GPU lever on this host.
- **P7 GPU MSM in a patched midnight-proofs 0.8.3** behind a `gpu` feature, only if P6
  passes and C0 shows the `commitment` owner ≥ 25%. Predicate: proofs byte-identical to
  the CPU path on the same seed and verified by the stock verifier (A5/G10).
- **P8 CPU backports in order of C0 share**: `--release` for heavy paths; #353
  (parallel logup/permutation/SHPLONK); then C4 parallel regions in the verifier
  gadget only if phase 7 ≥ 25% at < 300% CPU.
- **P9 C7 coset-tiled quotient** when memory, not time, blocks a third prover in flight.
- **P10 Experiment branch on the clone HEAD** (`24a1eb5a`): node row count with the
  unreleased gadget, to size the next-pin dividend. Not shippable (F4).

## 6. Combined estimate for fact(3), 293 steps

Baseline at current pins, serial, universal leaf assumed k_U=16 (t_step 6 s):
293 × (95 + 6) + 100 = **29,693 s = 8.2 h** (the brief's 7.7 h counts the node alone).

Ordered levers and the running total (ESTIMATE, k_U=16 unless stated):

1. S=8: 37 × (95 + 47) + 100 = **5,354 s (1.49 h)**. Gain 5.5×.
2. Leaf workers ahead of the chain, P=3 (26 + 3×13 = 65 GB ≤ 100 GB; contention 1.3×
   on the node): 37 × 95 × 1.3 + 95 + 100 = **4,765 s (1.32 h)**; without contention
   3,710 s. Gain 1.1-1.4×. (Listed before k=19 because it needs no circuit change.)
3. Node k=19 (chain-bound, contention 1.3×): 37 × 50 × 1.3 + 50 + 100 = **2,555 s
   (0.71 h)**; 2,000 s without contention. Gain 1.9×.
4. GPU MSM 1.35× on the node: 37 × 48 + 48 + 100 = **1,924 s (0.53 h)**.
5. Parallel witness 1.25×: 37 × 39 + 39 + 100 = **1,580 s (0.44 h)**.
6. #353 backport, C7, C10: ≤ 1.1× each; C7 buys memory for a 4th worker.

Same stack at k_U=17 (leaf k20 at S=8, 94 s; P=2 at 26+2×26 = 78 GB; 37 leaves in
2 × 94 × 19 = 1,786 s per worker, still under the chain): step 1 → 7,093 s (1.97 h);
step 2 → 4,765 s (1.32 h); step 3 → 2,555 s (0.71 h); step 4 → 1,924 s (0.53 h).
At k_U=18 (S=4, 74 leaves at k20, 94 s; P=2): step 1 → 14,086 s (3.9 h); step 2 →
74 × 95 × 1.3 + 195 = 9,334 s (2.6 h); step 3 → 74 × 50 × 1.3 + 150 = 4,960 s (1.38 h);
step 4 → 3,700 s (1.03 h).

**Is 2 h plausible without the tree?** Yes. With k_U ≤ 16, S=8 alone is enough on CPU
(1.49 h); leaf workers (1.32 h) and k=19 (0.71 h) add a 2.8× margin. With k_U = 17 it
needs S=8 plus leaf workers (1.32 h). With k_U = 18 it needs S=4, leaf workers and
node k=19 (1.38 h), with the GPU as margin. Once leaves are proved ahead, the wall clock
is the node chain alone, `37 × t_node`, and the only ways to cut it further are a
faster node (k=19, GPU, C4) or the tree, which divides the chain by the number of
concurrent node proofs (3 at 26 GB = 78 GB, or 4-6 at 13 GB under the 100 GB aggregate;
`tree.md` F11 and §7). The tree is the lever to reach for if k_U ≥ 18 and k=19 fails;
otherwise it trades 1-2 weeks of work for a chain already inside the budget.

## 6a. General-purpose recursion: compliance of each lever

Owner requirement (2026-11-08): one fixed key set serves any committed program and any
step count; no per-program keys or circuits; the design is reasoned about in Lean and Quint.

| Lever | Program-independent? | Step-count cap? | Verdict |
|---|---|---|---|
| S=8 batching | Yes: S is a constant of the universal leaf relation, the program enters only as `prog_d` in the statement; a short last chunk pads with disabled slots | None beyond `h < 2^64` (SRC-004); any step count maps to ⌈n/S⌉ leaves | keep |
| Node k=19 (T2 `nr_pow2range_cols`, SRC-004 checks) | Yes: a node arch constant | None | keep |
| Overlap leaf/node, C10, C7, C6, #353 backport | Orchestration and prover internals; no key or statement change | None | keep |
| GPU MSM/NTT | Backend only; proof bytes identical | None | keep |
| Option F (finite per-rule key family, SRC-013 fallback) | Program-independent (keys per rule of the fixed inventory `R`, not per program) but a leaf then proves one rule, so **S>1 across different rules is impossible under F**; S>1 needs option U | None | flag: S>1 and option F exclude each other; if k_U > 17 forces F, the plan loses lever 1 and must take k=19 + overlap + GPU |
| Fused fixed-pair terminal (C1), leaf partition for 16-tx batches (C2) | No: keys tied to A102/A80 or to a fixed batch shape | - | **reject** for Sherlock general-purpose recursion |
| Leaf trimmed to the fragments fact(3) fires (37 of 46 rules, `stagenet-gp-recursion.md` §6) | No: a program-shaped leaf key | - | **reject**; k_U is measured over the frozen inventory `R` at S=1 (SRC-013) |
| Tree of nodes (reopened by D13 review) | Yes if arity is a fixed constant and the node accepts both leaf and node children | None | neutral; see §6 for why it gains little after S=8 |

## 6b. Formal targets

Quint (state machines, exhaustive small-S runs):
- Q-A **S-batched IVC chain**: states `(prog_d, init_d, cur_d, h, acc)`; actions `leaf(S slots)`,
  `node`, `decide`. Invariants: `h = Σ enabled slots`; a disabled slot leaves the lane state
  fixed and adds 0 to `steps`; a publishing slot is the last enabled one; the node refuses
  `steps ∉ [1, S]`; the decider's `n_steps` equals the owner trace length for every trace
  length 1..N, every S in {1, 2, 4, 8} (padding and short-last-chunk cases).
- Q-B **Genesis**: `prev_h = 0` forces `prev_d = init_d` and scales the previous-proof
  accumulator to the trivial one; no non-genesis path can set `prev_h = 0`.
- Q-C **Leaf-worker scheduler** (F11, E11): P leaf workers and one node chain in flight;
  admission only when every process's recorded peak ≤ 48 GB and the sum of in-flight peaks
  ≤ 100 GB; properties: no deadlock, every step is eventually folded, node i never starts
  before leaf i is proved and node i-1 is proved, the aggregate bound is never exceeded.
- Q-D **Phase-profile accounting**: phases partition the proof interval (the harness's
  `coverage_bounds` ≥ 95%), so the stage shares every lever assumes cannot exceed 100%.

Lean (arithmetic and composition lemmas, over `ZMod r` for the BLS12-381 scalar field):
- L-A **No wrap in the height arithmetic**: `prev_h < 2^64 - S ∧ 1 ≤ steps ≤ S →
  prev_h + steps < 2^64` and the field sum equals the integer sum (the SRC-004 checks of §2
  are sufficient; and the weaker pair `prev_h < 2^64 ∧ steps ≤ S` without the `h` check is
  not).
- L-B **Chunk composition**: if each slot i satisfies the one-step relation on lanes and
  `post_i = pre_{i+1}` lane by lane, the S-slot leaf is a valid transition from `pre_0` to
  `post_{S-1}` for any S ≥ 1 (induction), and padding with identity slots preserves it.
- L-C **Statement independence from S**: the node statement `[vk, prog_d, init_d, cur_d, h, acc]`
  and the decider statement are the same functions of the owner trace for every S, so a
  program proved at S=1 and at S=8 yields the same `(fin_d, n_steps, result_be)`.
- L-D **Accumulator fold**: `accumulate(proof_acc × bit, carried, leaf)` with `bit = 0`
  equals `accumulate(carried, leaf)` (the genesis scale-by-bit lemma), and `collapse`
  preserves the pairing-check value.
- L-E **Range check correctness** of `assert_lower_than_fixed` for a non-power-of-two bound
  (`native_gadget.rs:274-306`): `x < bound ↔ x < 2^k ∨ x - (bound - 2^k) < 2^k` with
  `2^k ≤ bound < 2^(k+1)`; needed because `steps ≤ S` uses a bound of S+1.

Not formal targets (test evidence instead): GPU byte-identity, row counts, timings.

## 7. Risks

- **k_U is unmeasured and everything hangs on it.** An upper-bound per-row cost of
  0.3 ms (the M2 leaf's rate) multiplies every leaf time above by 3.3× and pushes
  k_U=16 serial to 37 × (95 + 157) + 100 = 9,424 s (2.6 h). P2 before P3.
- **7.3.0 may enlarge the node** (canonical public-input encodings, F3). Then k=19 waits
  for the next ledger pin.
- **GPU on aarch64 with a licence server**: ICICLE's CUDA backend is closed source,
  licence-gated and its Linux-aarch64 build is unverified; sppark declares ARM64 hosts
  unsupported. Budget one day (P6), not weeks, before believing any GPU number.
- **Shared memory bus**: any GPU NTT competes with the CPU prover for 273 GB/s; GPU
  gains measured on discrete cards (cuSnark 30× quotient, Nocy 2.8×) do not transfer.
- **Byte-identity of a patched prover**: every GPU/CPU backend change must be checked
  against the stock verifier on the same seed; a silent difference in MSM results would
  only appear as a decider that stagenet refuses (the A4 failure class).
- **Memory accounting with several provers**: peaks add; the 48 GB per-process and
  100 GB aggregate caps include GPU allocations here because the memory is unified; the
  host has 120 GB, so 100 GB of provers leaves 20 GB for the OS, the indexer and
  page cache, and a swap-free OOM kill is the failure mode. The 108 GB spike of F8 is
  what the aggregate cap exists to prevent.
- **CPU contention under concurrency**: three leaf provers beside the node chain slow
  the chain (ESTIMATE 1.2-1.5×); the chain-bound totals in §6 assume 1.3×. Measure
  with 0/2/3 workers before fixing P.
- **Nondeterministic lookups** (`HashMap` in lookup prover, `stages.json` s4) interact
  with any parallel backport; fix the ordering first.

## 8. Open questions

1. k_U and RSS of the universal leaf at S=1 (M4, Q1/Q4 of the design).
2. Node row count and per-gadget breakdown at 7.3.0 (task 2.6).
3. Is there a Linux-aarch64 CUDA backend binary for ICICLE v4, and does the free R&D
   licence permit our use? (Ask Ingonyama; one day.)
4. CPU% per phase of the node prover on this host (F7); nothing in the record measures
   it for Sherlock, only recl9's 268%.
5. The constraint degree of the node's constraint system (sets the extended-domain
   factor and therefore the RSS model in F8).
6. When will a ledger release pin a midnight-proofs above 0.8.3, unlocking the
   unreleased gadget rows (F9) and the `midnight-aggregation` IVC as a reference?
7. Whether `midnight-aggregation`'s IVC example at the same k is smaller than our node
   (a cheap row-count comparison on the clone).

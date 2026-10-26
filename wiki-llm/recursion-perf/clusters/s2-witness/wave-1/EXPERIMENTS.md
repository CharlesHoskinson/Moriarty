# s2-witness — first experiments

SOURCE — No performance experiment, Rust build, proof, installation or submission was run in this research wave. The baseline evidence is [parent-receipt.md](../../../raw/s2-witness/parent-receipt.md) and [cusnark.md](../../../raw/s2-witness/cusnark.md). The historical parent needs its qualified SDK/source closure; the retained source44 files alone are not a standalone reproduction. [Provenance](../../../raw/s2-witness/source44-provenance.md).

ESTIMATE — Run E1 before deciding how much of E2 to implement. E3 is a separate leaf-preparation investigation; it cannot claim a k20 parent gain unless E1 proves that path consumes G5. The steps below are specifications for subsequent implementation. **`s2-witness-bench` and its switches below are a proposed harness interface, not an existing executable.** The corpus manifests are also to be constructed from the pinned local fixtures; this document invents no existing fixture paths or completed measurements.

## Shared setup and measurement contract

1. **SOURCE —** The inspected CPU exposes 20 logical CPUs, ADX/BMI2/AVX2 and no AVX-512; [host capture](../../../raw/s2-witness/host-cpu.md). The target brief specifies 90 GB RAM and RTX 5090 32 GB with CUDA 13.x. **OPEN QUESTION —** Confirm accessible GPU, driver, CUDA/backend versions and usable free memory before a device experiment; no GPU query was made here.
2. **ESTIMATE —** Create a manifest for each of Sherlock P, Sherlock D, Fenrir N, Fenrir D and relevant leaves. Record source commit and patch hashes, Cargo.lock, toolchain/features, release flags, binary hash, public-input digest, circuit/VK/PK/SRS digest, k, advice columns per phase, and transcript type. Use the same loaded keys and inputs across variants. Keep private inputs in their existing protected location; publish metrics and artifact hashes only. An unavailable historical fixture is a missing reproduction, not permission to relabel a synthetic circuit as the baseline.
3. **SOURCE —** Heavy commands must use `scripts/doctorwatson heavy --id ID --budget KEY -- CMD...`; retries require a one-line `--hypothesis`, and repeated failures require a smaller reproducer. [Factory contract](../../../raw/s2-witness/factory-contract.md). **ESTIMATE —** During the present research-only scope, execute none of these future heavy commands: their factory traces write outside the two research directories. Subsequent implementation follows the normal factory loop and Rust checks.
4. **ESTIMATE —** In the future harness, use one heavy proof at a time. Set both the Rayon pool and any separately active BLST pool explicitly; an environment setting for Rayon alone is insufficient. Sweep 1, 4, 8, 12 and 20 witness workers without nested full-size pools. Record actual affinity/core placement, CPU time and context switches; do not assume twenty equivalent cores.
5. **ESTIMATE —** Use two unmeasured warm-up iterations and at least five completed paired baseline/candidate runs in alternating AB/BA order. Run cold key/recipe preparation separately. Report median paired ratio and all pairs, not the best time. Twenty paired runs are appropriate only after a candidate passes correctness and initial timing; use them to estimate uncertainty, not to pretend five samples establish tail latency.
6. **ESTIMATE —** Compare private values in memory and emit only mismatch counts, public operation labels and cell coordinates. Inject test-only deterministic prover/circuit randomness for assignment comparisons; fresh entropy remains mandatory for real proofs. Use randomized nontrivial data and adversarial cases. MockProver is a fast constraint check; each retained candidate must also complete real proof verification through the unchanged terminal decider and pinned local ledger-9 reader. Stagenet submission is a later integration measurement, outside this research task's permitted contacts.

## E1 — Determine the real witness budget and dependency width

OPEN QUESTION — Is witness time dominated by region traversal, arithmetic/inversions, conversions, allocation, or a narrow dependency chain? Is a phase-wise interface required for the actual relation?

ESTIMATE — Implement these exact instrumentation boundaries in a source-identical build of the pinned prover, without changing arithmetic or layout:

1. In `plonk/prover.rs::parse_advices`, bracket configuration, initial output allocation, each phase's rational allocation, `FloorPlanner::synthesize`, `batch_invert_rational`, blinding, advice commitments and transcript/challenge updates **separately**. Source map: [midnight-prover.md](../../../raw/s2-witness/midnight-prover.md). Reconcile their nonoverlapping wall times with the enclosing interval.
2. In `SingleChipLayouter::assign_region`, instrument the shape invocation and assignment invocation independently, count regions/rows and record region offset/column summaries. Count advice `_to` callbacks inside the real value-evaluation path, not merely calls to `assign_advice`; `RegionShape` skips `_to`. Source maps: [floor planner](../../../raw/s2-witness/midnight-floor-planner.md), [shape](../../../raw/s2-witness/midnight-region-shape.md).
3. Add aggregate operation counters at chip APIs for Fr/Fp inversion/multiply, foreign-limb conversion, GLV decomposition, EC additions/doublings and hash rounds. Count value dependencies at typed chip boundaries for one diagnostic run. Do not claim an `Assignment` log alone reveals arithmetic hidden in Rust closures. Estimate weighted work `W`, critical path `D`, and distribution of independent ready operations/inversions per phase. Report incomplete coverage explicitly.
4. Measure allocated/live bytes, output column bytes, denominator density, bytes cloned, RSS and CPU time. Separate active from inactive columns. Record `size_of::<Rational<Fq>>()` and `size_of::<Option<Fq>>()`; do not reuse speculative enum sizes as measurements. Source map: [conversion](../../../raw/s2-witness/midnight-rationals.md).
5. Use aggregated per-region spans for diagnostic captures; measure release latency with fine-grained counters disabled. Compare instrumented versus uninstrumented baseline so profiling overhead is visible. Complete five paired warm runs for each recovered P/N/D fixture; do not sum simultaneously nested timers or add historical key setup to the KeysReady→Witnessed interval.

ESTIMATE — Proposed harness invocation after implementing that contract (from repository root; substitute only paths to recovered fixtures and the built harness):

```bash
scripts/doctorwatson heavy --id s2-witness-profile --budget s2-witness-profile -- \
  /absolute/path/to/s2-witness-bench profile \
  --fixture-manifest /absolute/path/to/pinned-corpus.json \
  --workers 1,4,8,12,20 --warmups 2 --pairs 5 \
  --metrics-only --output wiki-llm/recursion-perf/clusters/s2-witness/wave-1/e1-results.json
```

ESTIMATE — Required output per relation: `total_prove_s`, `witness_s` (sum of synthesis/resolution spans across phases), `shape_s`, `assign_s`, `rational_s`, `allocation_s`, `commit_s`, `transcript_s`, `phase_count`, `cpu_s`, `peak_rss_bytes`, `op_counts`, `W`, `D`, `ready_width_histogram`, `layout_digest`, and verification outcome. Nested shape/assignment scopes are subdivisions of synthesis, not additional time. Record measurement methodology next to W/D because weighted operation counts are models, not timings.

ESTIMATE — **Decision:** compute `f=witness_s/total_prove_s` for every relation. Reject a witness-only 10× claim when the unchanged share already exceeds .10. If removable shape work is <5% of witness, defer layout caching as a major optimization. If the estimated critical path exceeds `witness_s/6.452`, parallel scheduling alone cannot meet the previous joint-plan witness target without accelerating operations on that path. Proceed to E2's selected gadget only when its measured share and frontier widths justify it. If the source closure is unavailable, report E1 unavailable and keep the numerical forecast conditional.

## E2 — Prove that one typed recipe is faster and assignment-equivalent

OPEN QUESTION — Can a frozen-layout recipe plus native values and frontier inversion remove enough real witness work before GPU/compiler investment?

ESTIMATE — Choose the highest-cost *measured* E1 gadget. Prefer a foreign-EC verifier fragment if dominant; otherwise use the measured native-field or hash fragment. Preserve the exact hardened circuit, all intermediate cells and exception checks. Implement the following variants one at a time so their effects remain attributable:

1. **Baseline:** current closure-based synthesis with loaded keys. **Replay:** precomputed region placement and constants/copy plan, existing arithmetic unchanged. Bind the plan to circuit configuration, layout, SDK/patch fingerprint, k, phase, VK and SRS identity; regenerate or reject on mismatch. Never cache witness values or randomness across jobs.
2. **Sequential typed recipe:** extract the selected chip operations to fixed-width Fr/Fp/limb nodes, removing generic BigUint conversion only where equivalent range/reduction semantics are explicit. Evaluate in dependency order, scatter to original cells, and compare all values against the baseline **before blinding**. Check fixed/selectors/copy/instance structure and VK bytes independently. Unsupported operations use the original path; count fallback time.
3. **Parallel recipe:** execute independent blocks with 1/4/8/12/20 workers. Start a task only when all input cells are ready. Preserve original output placement even when execution order changes. Sweep grain 256/1,024/4,096 operations rather than adopting gnark's 50-constraint threshold blindly; [gnark's scheduler](../../../raw/s2-witness/gnark-bls381-solver.md) provides a design precedent, not the correct Midnight grain.
4. **Frontier inversions:** independently benchmark ready sets of 1/32/256/4,096 denominators, separately in Fr and Fp. Compare individual inversion with zero-preserving batch inversion and chunked batches using reusable scratch. Include collection, scatter and consumer wake-up in latency. Test all-zero, mixed-zero, random nonzero and repeated denominators; equality auxiliaries must preserve the original ONE-at-zero convention where applicable. For EC, test identity, equal/inverse operands, GLV/range boundaries and shift/subgroup cases. Do not omit original accumulator prefixes.
5. **Optional GPU variant, only after steps 1–4 pass:** lower supported element-wise groups to a CUDA field program/custom kernel. Check BLS12-381 scalar and base representations explicitly. ICICLE's [Program API](../../../raw/s2-witness/icicle-cpp-program.md) supplies element-wise fusion, not arbitrary dependencies or scans. Sweep batch sizes `2^8,2^12,2^16,2^20`; report warm resident device events **and** host-input-to-host-output latency including transfers, allocation and synchronization. Test on the actual 5090/CUDA 13.x backend. Reject memory oversubscription as a residency result; record free/peak VRAM and concurrent prover allocations.
6. Run the small differential corpus first, then complete real P/N and terminal D proofs on the same pinned fixtures. Run success and corrupted child/public-input/assignment cases through the original verification paths. Record errors as results, never silently discard failing samples. Exercise stale-layout invalidation and changed-shape fixtures.

ESTIMATE — Proposed harness invocation after implementation; each ablation has its own output:

```bash
scripts/doctorwatson heavy --id s2-witness-recipe --budget s2-witness-recipe -- \
  /absolute/path/to/s2-witness-bench recipe \
  --fixture-manifest /absolute/path/to/pinned-corpus.json \
  --variants baseline,replay,sequential,parallel,frontier \
  --workers 1,4,8,12,20 --grains 256,1024,4096 \
  --warmups 2 --pairs 5 --verify-all --metrics-only \
  --output wiki-llm/recursion-perf/clusters/s2-witness/wave-1/e2-results.json
```

ESTIMATE — **Decision:** require zero semantic/layout mismatches and successful reference verification, then target ≥2× improvement in **total measured witness time**, not only the chosen kernel. Report `T_new=T_unmodified+T_recipe+T_scatter+T_fallback`; compare the actual inclusive result. For the ambitious joint plan, witness must reach ≥6.452× only under its assumed .20/.78/.02/.01 budget; recompute from E1 shares. If recipe setup costs `J` seconds and each reuse saves `ΔW`, warm amortization requires more than `J/ΔW` uses. Report cold and warm results separately. Stop the GPU branch if inclusive GPU latency loses to the best CPU recipe at actual frontier sizes.

## E3 — Separate G5 observation cost from circuit witness cost

OPEN QUESTION — Does eliminating JSON/history materialization materially reduce leaf preparation, and can a typed sink generate identical circuit advice without changing the K-A transition stream?

SOURCE — The concrete starting point is `crates/mori-core/src/sherlock/compile_artifact/draft_witness_stream.rs::witness_stream`, with existing `fact3_stream_is_contiguous_and_halts_at_six` and `list123_stream_is_contiguous_and_halts_at_six` tests. [Implementation](../../../raw/s2-witness/g5-stream.md), [tests](../../../raw/s2-witness/g5-stream-tests.md). The existing `stream_owner_steps` API materializes a different owner boundary representation; it must not be substituted without an equivalence argument. [Reference](../../../raw/s2-witness/owner-stream-reference.md).

ESTIMATE — Implement three measured paths over identical public synthetic programs/configuration:

1. **Existing G5:** retain full `Vec<Value>`, display strings, canonical JSON and host SHA-256 digests. Time machine stepping, snapshot conversion, canonicalization/hash and final serialization separately.
2. **Typed observer:** retain `Machine::step` semantics, forward its borrowed rule/before/after/charge/outcome to a callback, discard each record before advancing. Retain only previous post for a checked equality against the next pre; measure this check too. This isolates JSON/history savings while keeping any full-state clone cost inside `Machine::step` visible.
3. **Typed layout sink:** map that event to the already-defined leaf layout with a row/column/phase destination plan and explicit lane/Poseidon encoding. Buffer tiles of 1/64/1,024 events or the relation's smaller public bound. Do not invent a lane mapping when the leaf relation lacks one. On identical events, compare every leaf assignment and public boundary against the existing validated producer; SHA-256 host digest equality alone is not enough.
4. Use the two existing small fixtures, then synthetic runs yielding approximately `10^2,10^3,10^4` actual events, and include empty/deep frames, branches, rejection, exhaustion and terminal publication within each relation's supported domain. Count actual events, not an assumed source-level loop count. Vary public stack bounds; preserve fuel, rule identity, charges, terminal precedence and final result. Check every adjacent state, not just final outputs. Refused/out-of-domain fixtures must remain refused.
5. Measure wall/CPU time, allocation count/bytes, peak RSS, retained events and bytes/event for all paths. Include the typed sink's actual consumed columns and encoding costs. Run at least five paired samples after warm-up and complete a leaf→P/N→D local verification for any circuit-ready variant. Separately test stop/resume only if a resumable API is introduced.

ESTIMATE — Proposed harness invocation after implementation:

```bash
scripts/doctorwatson heavy --id s2-witness-g5 --budget s2-witness-g5 -- \
  /absolute/path/to/s2-witness-bench g5 \
  --fixture-manifest /absolute/path/to/public-g5-corpus.json \
  --variants json,typed,layout --tiles 1,64,1024 \
  --warmups 2 --pairs 5 --compare-events --compare-leaf-advice --metrics-only \
  --output wiki-llm/recursion-perf/clusters/s2-witness/wave-1/e3-results.json
```

ESTIMATE — **Decision:** require complete event/assignment equality and bounded retained history for the typed route. Promote it when measured host preparation improves ≥2× without shifting larger costs into leaf encoding. Claim parent latency improvement only if the measured parent request includes that preparation. Compute full-chain gain from its measured G5 fraction `g` as `1/(1-g+g/s_g5)`; if `g=.05` and `s_g5=10`, the gain is only 1.047×. If no validated K-A→leaf mapping exists, finish the typed-host comparison but mark the circuit-layout experiment incomplete.

ESTIMATE — For any future Rust implementation, run the repository's required format, Clippy and workspace tests through the factory workflow, plus the meaningful differentials above. A source review, successful host stream, or a fast kernel does not replace a complete proof/terminal-verifier check. Each failed heavy run gets a trace-backed hypothesis; shrink after two like failures and obey `stop-rule`.

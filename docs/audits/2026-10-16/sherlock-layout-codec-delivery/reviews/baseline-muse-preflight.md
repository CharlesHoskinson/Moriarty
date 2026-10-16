**Reviewer:** Fresh independent Muse Wonder1.0, high effort, nonsecurity code/contract auditor  
**Verdict:** `APPROVE_CODE_CONTRACT_PREFLIGHT_WITH_LIMITS`

No security verdict is supplied. Compilation and all 35 tests remain **UNRUN**. No pass is claimed.

## Blockers

None identified by reading the exact bytes.

I found no byte-level reason that should prevent the planned compilation from being attempted. In particular:

- [layout_harness.rs](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/layout-codec-01-preparation/candidate/layout_harness.rs) registers only the runtime shim.
- [layout_harness_shim.rs](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/layout-codec-01-preparation/candidate/crates/mori-core/src/sherlock/layout_harness_shim.rs) registers the unchanged `reference_types`, `reference`, `reference_display`, `compare`, and `layout_a` modules under `crate::sherlock`. It does not copy an algorithm.
- Production `lib.rs`/`mod.rs` and unrelated `mori-core` modules are outside the declared closure.
- The declared compile closure contains exactly 24 files. The reachable module and test-module graph matches that closure by inspection.
- The closure uses only Rust standard-library types. No Cargo build or external crate is required by the compiled modules.
- The expected names correspond to $6+8+4+5+8+4=35$ existing `#[test]` functions. No additional test module is reachable in the closure.
- Original test bodies, capacities, literal expectations, and other candidate modules retain the hashes recorded in [SOURCE-ORIGINS.json](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/layout-codec-01-preparation/SOURCE-ORIGINS.json).
- Unused items such as `AUX_PROFILE` and some auxiliary enum variants may produce warnings. The planned rustc invocation does not deny warnings, so they are not identified as compilation blockers.

## Exact conditions

Approval is limited to one fresh run with these conditions:

1. Run only the planned parent invocation:
   `python3 -I -B run_layout.py`
   from `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/layout-codec-01-preparation`.
2. Create only fresh `attempt-01`. Do not reuse an attempt directory.
3. Use only `/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc`.
4. Select only `preflight`, `rustc-version`, `compile-tests --test`, `run-tests`, `candidate-postflight`, and `finish/postflight`.
5. Use the inherited runner controls: direct children, minimal environment, process groups, and 30 seconds per child.
6. Require exact rustc dep-info equality with the 24-file `compile_closure`.
7. Require exactly the 35 fixed names in [CONFIG.json](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/layout-codec-01-preparation/CONFIG.json), with:
   - 35 passed;
   - 0 failed;
   - 0 ignored;
   - 0 measured;
   - 0 filtered out.
8. Require runner source and tool postflight bindings to remain unchanged.
9. On any compilation failure, dep-info mismatch, test-name mismatch, failure, ignore, filter, timeout, or postflight mismatch, stop. Do not retry, alter tests, change a capacity, add a cap, or repair bytes inside this candidate.
10. If bytes do block compilation, register that result against this exact candidate and require a new reviewed candidate. Do not treat this preflight as approval of a replacement.

## Limits

- This approves only an isolated pure-stdlib Rust host compilation and the exact existing 35 normal tests.
- Native authority remains zero.
- The source domain remains H1: trusted, fixed, local, internal, typed source with a retained `PreparedA`.
- Existing Candidate A, F3, and F5 admission conditions remain unchanged and undischarged.
- The zero-dimension auxiliary specimen checks byte width and shape only. It is outside the valid L4 domain and provides no L4 admission evidence.
- Test-fixture capacities are retained examples only. No numerical production capacity, default, adequacy decision, or unfolded-envelope certification is approved.
- Common host comparison, oracle, K, and model files are snapshot/name bindings only in this plan. No producer, comparator, K engine, or model engine is selected or approved.
- Rust sysroot, linker, system libraries, Python standard library, and kernel are unhashed limitations. This is not a hermetic-build or resource-admission claim.
- No production integration, native ABI, byte parsing, theorem, transition or continuity checking, circuit or target acceptance, hostile-test design, B-profile claim, or financial acceptance is approved.
- If compilation or any test fails, that outcome supersedes this preflight expectation for execution purposes; a new candidate is required rather than an in-place correction.

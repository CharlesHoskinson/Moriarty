### Continuing Independent Non-Author Ordinary Source Review — Combined Cargo Discovery Wrapper (`positive` + `contrast`) & Two-Operation Compile/List Proposal (`witness-reference-projection-cargo-discovery-contrast-20261018`)

- **Reviewer Identity:** Continuing independent non-author Gemini 4 Argon (High) ordinary source reviewer (`CID: 21399dd6-faf2-479b-823a-9bfec7feeba7`; standing Opus outage route; same continuing review context, not an additional independent vote).
- **Execution Discipline:** Zero tools, searches, commands, file reads/writes, provider invocations, or subject/model/native actions executed. All verification is performed strictly over the 12 complete inline bodies supplied in this turn.

---

### 1. Explicit Readable Coverage Declaration (All 12 Inline Bodies Read in Full)

All **12 complete inline bodies** supplied in this turn were received and read in full without truncation:

1. `afterimages/crates/mori-core/tests/sherlock_witness_reference_prefix.rs` (`240`B, `sha256=8039696911d04a711c0fafc0461dc30a62293c775d356c4f4c9e63984fc91c7e`)
2. `afterimages/crates/mori-core/tests/support/sherlock_witness_reference_prefix_positive.rs` (`18,254`B, `sha256=7391f8f8302e5441a52742962836bdb291b46f832a35099f38caf8aa6c6551cc`)
3. `afterimages/crates/mori-core/tests/support/sherlock_witness_reference_contrast_positive.rs` (`33,049`B, `sha256=e61953072f7783f3e28258f19f6db0009eda1200cd12f50e5330361cece43f97`)
4. `APPEND-READBACK.json` (`10,358`B, `sha256=29d1584fbb35ce08ec76f05f7ba83b8d221f93d49fcd846663bafd593386fc01`)
5. `SOURCE.patch` (`394`B, `sha256=5f0a8df17df2f5a9ac5f1f63a250791a58c5ec7c40b576a0d5a56886b823397f`)
6. `basis/crates/mori-core/Cargo.toml` (`413`B, `sha256=71d7eeb2520b07edb18033e22a816b55d76ee80cc8ffd6d06bd5e0eca8cf5030`)
7. `SOURCE-FLOW.json` (`7,257`B, `sha256=46c6e6df8dbe4f509a82e7b735bdbf313e090596e361a6dbd04f21402485f598`)
8. `PROPOSED.json` (`15,683`B, `sha256=1660216a386c16e252f78c96d21d38cfb6d42270ff4f296f7bc7c5b1bd2f3cd1`)
9. `witness-reference-projection-cargo-discovery-20261018/provider-ready-01/REPORT.md` (`15,133`B, `sha256=83fcb614c5eadcae1f70764bfde5f8588a38dbf2954cf1405085db22f6b8c9bd`)
10. `witness-reference-projection-cargo-discovery-20261018/actual-two-01/RESULT.json` (`6,895`B, `sha256=a3128e3f665d31a3471442db4c2e0609e1406b11d49a925730d26d58cc97c349`)
11. `witness-reference-contrast-portable-source-20261018/ordinary-QA/actual-portable-compile-01/RESULT.json` (`4,319`B, `sha256=5575aff05de71c07aebf3bf4da05213baeb967b6df453486b564da4b963d3fce`)
12. `435-resolved-signature-execution-preparation/revision02/run_selected_13.py` (`15,371`B, `sha256=ce0482fdc12e4146500fa4421f7e4333b14321bdaa207c123a09ee6cb34c9932`)

---

### 2. Separate Verdicts

1. **`SOURCE` Verdict (Appended 240-Byte Cargo Discovery Wrapper + Module-Safety of Both Unchanged Support Bodies & 7 Opaque Fixtures):** **`APPROVE`**
   - Scoped to `afterimages/crates/mori-core/tests/sherlock_witness_reference_prefix.rs` (`240`B, `sha256=8039696911d04a711c0fafc0461dc30a62293c775d356c4f4c9e63984fc91c7e`), which preserves the exact 157-byte preimage (`sha256=23be731fb53d7f7f60641399f49369a41c84e3e255a119f06c8468b0c2e2ca9a`) and appends the 83-byte suffix `\n#[path = "support/sherlock_witness_reference_contrast_positive.rs"]\nmod contrast;\n`, while preserving unchanged `support/sherlock_witness_reference_prefix_positive.rs` (`18,254`B, `sha256=7391f8f8...`), `support/sherlock_witness_reference_contrast_positive.rs` (`33,049`B, `sha256=e6195307...`), and all 7 opaque JSON fixtures.
2. **`TWO_OPERATIONS_ELIGIBLE` Verdict (`01-compileCombinedCargoDiscoveryWrapper` and `02-listCombinedCargoDiscoveryWrapperTests` in `PROPOSED.json`):** **`APPROVE` (with the order-check qualification noted in Section 3.C for `--list` output)**
   - Eligible for Root-selected single-attempt serial fail-stop execution under the declared 30s compile / 10s list `timeout` and `prlimit` bounds, `131,072`B per-stream cap, exact 3-var environment, static input pin verification, and the existing pure capture helpers from `run_selected_13.py` (`sha256=ce0482fdc12e4146500fa4421f7e4333b14321bdaa207c123a09ee6cb34c9932`).

---

### 3. Detailed Source & Operation Verification

#### A. Exact Prefix Preservation, Module Isolation, Privacy, and `include_str!` Anchoring Across Both Support Files
- **Exact Append Proof (`157`B -> `240`B):**
  ```rust
  //! Cargo integration entry for the existing complete-prefix positive tests.
  #[path = "support/sherlock_witness_reference_prefix_positive.rs"]
  mod positive;

  #[path = "support/sherlock_witness_reference_contrast_positive.rs"]
  mod contrast;
  ```
  Bytes `0..157` match the reviewed 157-byte wrapper (`sha256=23be731fb53d7f7f60641399f49369a41c84e3e255a119f06c8468b0c2e2ca9a`) byte-for-byte; bytes `157..240` (`83` bytes) add only `\n#[path = "support/sherlock_witness_reference_contrast_positive.rs"]\nmod contrast;\n`.
- **Sibling Submodule Safety & Namespace Disjointness (`mod positive` and `mod contrast`):**
  - Both `support/sherlock_witness_reference_prefix_positive.rs` (`18,254`B, `sha256=7391f8f8...`) and `support/sherlock_witness_reference_contrast_positive.rs` (`33,049`B, `sha256=e6195307...`) import external crates strictly from the `--edition=2024` extern prelude (`use mori_core::sherlock as a;`, `use serde_json::{json, Value as J};`, `use std::sync::Arc;`, `std::collections::BTreeSet`) and resolve `mori_core` submodules through their own module-local alias `a` (`use a::helper_machine::{...};`, `use a::helper_machine::shared::*;`, `use a::helper_machine::shared::owner_host::host_owner_engine;`).
  - Neither support file contains `crate::` or `super::` references.
  - Because `positive` and `contrast` are separate sibling modules under the crate root, identically named private helpers (`tree`, `value`, `env`, `pattern`, `arm`, `arms`, `expr`, `function`, `program`, `frame`, `control`, `outcome`, `snapshot`, `literal`, `uint`, `var`, `binary`, `call`, `one_function`, `factorial`, `layout`, `tooling`, `variants`, `exercise`) live in disjoint module namespaces and cannot collide.
- **Privacy & Complete Internal Liveness in `mod contrast`:**
  - All private functions and constants in `mod contrast`—including `FACT2`, `capture_initial`, `distinct_program_cache_contrast`, `LET_LEAF_VAR`, `let_leaf_pattern_var`, `verify_record_counters`, `b_rule_id`, `FACT2_B0`, `b_clock_row`, `compare_b0_schedule`, `exercise`, and all AST/snapshot formatters—are transitively reachable from the two `#[test]` functions in `mod contrast`:
    1. `contrast::factorial_two_complete_prefix_program_cache_and_incomplete_stream`
    2. `contrast::let_leaf_pattern_variable_complete_reference_prefix`
  - `rustc --test` discovers private `#[test]` functions inside private child modules (`mod positive;` and `mod contrast;`) without requiring `pub` visibility, and emits no unused/dead-code warnings.
- **Source-Relative `include_str!` Anchoring (All 7 Opaque Fixtures):**
  - `#[path = "support/..."]` resolves relative to `.../crates/mori-core/tests/` (the crate root directory), whereas `include_str!(...)` resolves relative to `.../crates/mori-core/tests/support/` (the directory of each support source file).
  - The 4 `include_str!("../fixtures/witness-reference-prefix/...")` calls in `positive` and the 3 `include_str!("../fixtures/witness-reference-contrast-independent/...")` calls in `contrast` therefore resolve to the exact 7 staged fixture files under `.../crates/mori-core/tests/fixtures/`:
    1. `witness-reference-prefix/factorial-pending-caller3.json` (`14,726`B, `sha256=48cb2d9d...`)
    2. `witness-reference-prefix/factorial3-complete-A39-prefix.json` (`464,178`B, `sha256=af15b0e5...`)
    3. `witness-reference-prefix/list2-5-complete-A39-prefix.json` (`301,342`B, `sha256=6feb5bb6...`)
    4. `witness-reference-prefix/tree4-1-3-complete-A39-prefix.json` (`716,490`B, `sha256=c9238dee...`)
    5. `witness-reference-contrast-independent/factorial2-normal-program-contrast-A39-prefix-B0-clocks.json` (`28,581`B, `sha256=d384409d...`)
    6. `witness-reference-contrast-independent/factorial2-normal-program-contrast-A39-prefix.json` (`322,314`B, `sha256=11b11e9f...`)
    7. `witness-reference-contrast-independent/let-leaf-pattern-var-complete-A39-prefix.json` (`34,861`B, `sha256=ddd6e079...`)

#### B. Operation `01-compileCombinedCargoDiscoveryWrapper` Verification
- **Command, Externs & Toolchain:** Direct `/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc` (`--sysroot ... --target x86_64-unknown-linux-gnu --edition=2024 --crate-name sherlock_witness_reference_prefix --test`) on `afterimages/crates/mori-core/tests/sherlock_witness_reference_prefix.rs`, linking the retained `libmori_core.rlib` (`bytes=43779842`, `sha256=4afb66098943d7a3da24bcfd4e2930b30944f81d1b06268cf369ace9454fe873`) and `libserde_json-76dc87088753b61e.rlib` with `-C debuginfo=0 -C opt-level=0 -C debug-assertions=yes -C linker=/usr/bin/x86_64-linux-gnu-gcc-15 -C link-arg=-fuse-ld=bfd`.
- **Exact 10 Dep-Info Inputs (`1 + 2 + 7 = 10`):**
  - Emits `build/witness-reference-prefix-combined-discovery` and `build/witness-reference-prefix-combined-discovery.d`.
  - `dep_inputs(path, allowed)` from `run_selected_13.py` resolves `.../tests/support/../fixtures/...` via `Path.resolve(strict=True)` and verifies all **10** inputs (the 240B wrapper root, both support `.rs` files, and all 7 JSON fixtures) against the static input pin inventory (`FLAT-INPUT-PINS.json`, `bytes=142623`, `sha256=f02522a6b7fe008a7dacba1ab1d2d3e013d35ae3ea1c4eeaf012f1f81e01ee42`).
- **Resource & Environment Bounds:** `timeout --signal=KILL 30s`, `prlimit --cpu=25:25 --as=2147483648:2147483648 --fsize=67108864:67108864 --core=0:0`, `keptBytesPerStream: 131072`, isolated `cwd` (`.../witness-reference-projection-cargo-discovery-contrast-20261018`), exact 3-var environment (`PATH=/usr/bin:/bin`, `LANG=C.UTF-8`, `TMPDIR=.../tmp`), `serialFailStopNoRetry: true`.

#### C. Operation `02-listCombinedCargoDiscoveryWrapperTests` Verification & Listing-Order Qualification
- **Command & Resource Bounds:** Direct execution of `build/witness-reference-prefix-combined-discovery --list` under `timeout --signal=KILL 10s`, `prlimit --cpu=8:8 --as=1073741824:1073741824 --fsize=16777216:16777216 --core=0:0`, `outputs: []`, `keptBytesPerStream: 131072`, same isolated `cwd` and exact 3-var environment, `serialFailStopNoRetry: true`.
- **Zero Test-Body Execution:** `--list` only prints the static `libtest` descriptor names and `5 tests, 0 benchmarks` trailer (`runtimeTestBodiesInvoked: 0`). Neither `positive::exercise` nor `contrast::exercise` nor `host_owner_engine()` is called.
- **Concrete Qualification on `--list` Output Line Order (`positive::*` vs. `contrast::*`):**
  - Because `sherlock_witness_reference_prefix.rs` declares `mod positive;` first (line 3) and `mod contrast;` second (line 6), `rustc`'s test harness generator emits the 3 `positive::*` descriptors before the 2 `contrast::*` descriptors in the static slice passed to `test::test_main_static`, and `libtest`'s `list_tests_console` prints descriptors in slice (module declaration) order:
    1. `positive::factorial_three_complete_reference_prefix_and_pending_caller: test`
    2. `positive::list_two_five_complete_reference_prefix: test`
    3. `positive::tree_four_one_three_complete_reference_prefix: test`
    4. `contrast::factorial_two_complete_prefix_program_cache_and_incomplete_stream: test`
    5. `contrast::let_leaf_pattern_variable_complete_reference_prefix: test`
  - Meanwhile, `PROPOSED.json` (`expected.listedTests`), `SOURCE-FLOW.json` (`expected_sorted_listing`), and `APPEND-READBACK.json` (`expected_list_order`) list the 5 names in lexicographical (`sorted`) order (`contrast::*` first, `positive::*` second).
  - **Requirement for Root's post-capture `--list` check:** When checking `stdout.bin` of `02-listCombinedCargoDiscoveryWrapperTests`, verify that:
    - there are **exact 5** `<name>: test` lines with **zero duplicates, zero missing, and zero extra names**,
    - the sorted 5-element list equals `expected["listedTests"]` (matching module-declaration order `positive::*` then `contrast::*` in raw output), and
    - the trailing summary is `5 tests, 0 benchmarks` with `actual_exit == 0`.

---

### 4. Scope Boundaries & Non-Inferred Claims

- **Permissible Claim After Operations `01` and `02`:** Establishes **only** that the 240-byte combined Cargo integration-test root `crates/mori-core/tests/sherlock_witness_reference_prefix.rs` (`sha256=80396969...`) compiles and links cleanly under `rustc --test` against the retained `libmori_core.rlib` (`sha256=4afb6609...`) and `libserde_json` with all **10** expected dep-info inputs (`1` wrapper + `2` support modules + `7` relative-path fixtures), and that the resulting binary registers and lists the exact **5** qualified test names (`3` `positive::*` + `2` `contrast::*`) under `--list`.
- **Non-Inferred Claims:**
  - Zero test bodies are executed (`runtimeTestBodiesInvoked: 0`); `--list` does **not** establish a new runtime `PASS` of any of the 5 tests.
  - No `mori_core` rebuild, full `cargo test` invocation, or Midnight dev-dependency/SDK resolution is performed.
  - The 7 fixture payloads remain opaque static pins here; no new semantic/oracle re-approval of the contrast module or fixtures is performed in this step; and B-clock/counter upper-bound theorems, H1 physical performance/memory claims, canonical `ProjectRel`/`Link` projection, native/proof/target gates, and whole `UIREQ-035` / `SCON013` / `SCON024` acceptance remain **OPEN**.


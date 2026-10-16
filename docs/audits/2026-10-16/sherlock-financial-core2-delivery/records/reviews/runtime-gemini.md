`APPROVE_WITH_SCOPE`

### 1. Sentinel & Complete 14-Member Source Coverage Attestation

- **Reviewer Identity & Role**: Gemini 4 Argon (High) via AGY — fresh independent **non-author** designated-security source/runtime preflight reviewer. Zero tools, actions, files, network calls, commands, writes, or executions were performed (`native_authority = 0`).
- **Phase A Final Sentinel Confirmed**: `END_RUNTIME_PREFLIGHT_A_SENTINEL_093F`
- **Phase B Final Sentinel Confirmed**: `END_RUNTIME_PREFLIGHT_B_SENTINEL_72DA`
- **Approved `RUNTIME-MEMBERSHIP.json` SHA-256**: `d1486e9b56172f6573c2147db56ac927d6d7b1941d39873509de97592393976b`
- **Complete 14-Member Coverage**: Every member of the 14-file `RUNTIME-MEMBERSHIP.json` was received and inspected in full across Phase A and Phase B with zero omissions or truncations, including complete `frozen_f01()` and `frozen_f02()` in `financial_core2_literals.rs`:
  1. `CANDIDATE-MEMBERSHIP.json` (`1828` bytes, sha256 `96521effe3e13486e72a02c4b5be52cba27f154dcd99018c2ef8dcb7f7483091`) — Phase A
  2. `CORE-REUSE-FACTS.json` (`17647` bytes, sha256 `49c3fc89fb43a0031656b64eb3f882dc610456ff31d262627b8723ff18e7c9c1`) — Phase B
  3. `financial_core2_consumer.rs` (`7122` bytes, sha256 `f8df3b5004278b7f105231a64908b17024120e0707ef2b3a86e227dd30cd3620`) — Phase A
  4. `financial_core2_literals.rs` (`112976` bytes, sha256 `96238b989da3eef19e22ae8963d4f78d8091d7549d999d82ef17e20587e017b9`) — Phase A (complete `frozen_f01()` and `frozen_f02()`)
  5. `financial_projection.rs` (`9503` bytes, sha256 `911baa49721bed13c62afdb363ff1355629d7d006e70c4ec6ac29520f20e1c1b`) — Phase A
  6. `frozen-oracle/inputs.json` (`2488` bytes, sha256 `7c9314ff8256d4b2e3b24aa76049423757766d104024d811056e23759207fb4c`) — Phase A
  7. `frozen-oracle/inputs/conditional-grant-tranches.mori` (`1213` bytes, sha256 `37f8dbc6df9149223756e051ae136053dff6fe935141d732b30f229d42ab1427`) — Phase A
  8. `frozen-oracle/inputs/installment-invoice.mori` (`823` bytes, sha256 `298d787de25ec9e2c324dbf3f305557945d201ebfe8d3e7521a8e2c2d4acfd72`) — Phase A
  9. `reuse-context/bounded_source_driver.py` (`11528` bytes, sha256 `6fe358dbfbed5256e4851d67f09fae93804181c53aa83bb3bdc02667b215fd51`) — Phase B
  10. `reuse-context/dependency-manifest.json` (`2049` bytes, sha256 `ce37c42b96cc071280db3c361ad593172aab6f1716419d7cdc9010120ffb2f36`) — Phase B
  11. `reuse-context/production-core-lib.rs` (`1277` bytes, sha256 `df45168fa300562f532577739731c257d016a387fc943b25975ae46e3968a2fa`) — Phase B
  12. `reuse-context/run_core.py` (`18994` bytes, sha256 `81c079df7dd119093ae71fb9aaffff5238be5ad19bd3ed7e995c3b3229311336`) — Phase B
  13. `reuse_core_root.rs` (`327` bytes, sha256 `2fd59425b0fe0155e4d515b44c337615558134b6b4017bf5840ebe6ee1df7b97`) — Phase A
  14. `run_financial_core2.py` (`27124` bytes, sha256 `77c474550d9a6fb1f7531c10208c43b649f8fcfcc8c495c7c20957b66adc01bd`) — Phase B

---

### 2. Substantive Security & Compatibility Findings

1. **Command Eligibility & Source-Closure Compatibility (Zero New Core Compilation)**:
   - **Prior Production `libmori_core.rlib` Reuse**: `run_financial_core2.py` links against the existing compiled production artifact `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/streaming-02-production-preparation/attempt-core-01/libmori_core.rlib` (`15688664` bytes, sha256 `dff1c647ef75602afd9e7635bc57c529dc06697f85dcfdfab4a061b5c622f4e4`) and verifies its compile receipt `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/streaming-02-production-preparation/attempt-core-01/result.json` (`28685` bytes, sha256 `56154a66fb9a6575cee9201338d1a4bbbb49a99a56481a35079d83afc8973e93`, `terminal_state == "success"`).
   - **Producer Closure Identity**: In `CORE-REUSE-FACTS.json` and the verified `01-core` dep-info receipt, 25 of the 26 compiled `mori_core` source files match current working-tree bytes identically, including 100% of the Core/2 producer, stage, parser, IR, logical cost, runtime, digest, replay, policy, intent, JSON, and source-sponsor modules (`crates/mori-core/src/{lib,ir,intent,json,logical_cost,parser,policy,replay,runtime,source_sponsor}.rs` and `crates/mori-core/src/stage/{mod,admit,cost,decode,digest,plan,profile,validate}.rs`). The single differing file (`crates/mori-core/src/sherlock/mod.rs`, compiled `bb0752abf4065903fcd74cccd7c62783b88a17fc706b72c63e6b9e77e0f8aa39` vs current `562f563d6f679d188bdb283d1dc191b34f19459a98d9c8a260911f16e7e367de`) is confined to the `sherlock` submodule and is outside the `mori_core::stage::*` execution and planning path.
   - **Lexical Root & 6-File Compile Closure**: `reuse_core_root.rs` (`pub(crate) use mori_core::*;`) privately aliases the existing public types from `mori_core` so `financial_projection.rs` and `financial_core2_consumer.rs` compile as a standalone `--test` binary without recompiling `mori_core`, without invoking Cargo or any SDK-bearing workspace dev-dependency graph, and without introducing any second interpreter or replacement Core type. The guard enforces that `01-tests.d` dep-info inputs match the exact 6-file `COMPILE_CLOSURE` (`reuse_core_root.rs`, `financial_core2_consumer.rs`, `financial_core2_literals.rs`, `financial_projection.rs`, `frozen-oracle/inputs/installment-invoice.mori`, `frozen-oracle/inputs/conditional-grant-tranches.mori`).

2. **AST Helper Extraction & Process Containment / Resource / Authority Scope**:
   - **Safe AST Extraction**: `load_helpers` parses the verified bytes of `reuse-context/bounded_source_driver.py` (`6fe358dbfbed5256e4851d67f09fae93804181c53aa83bb3bdc02667b215fd51`) and compiles only the 7 named top-level `ast.FunctionDef` nodes (`sha`, `record`, `inventory`, `write_json`, `child`, `source_records`, `depinfo`), ensuring the top-level `bounded-source-01` driver body never executes.
   - **Containment & Resource Policy**: I independently concur with the guard's process and resource scope:
     - `native_authority = 0`, `acceptance = "none"`, H1 and C4 unchanged.
     - Interpreter flags `-I -B` (`sys.flags.isolated and sys.flags.dont_write_bytecode`) enforced at entry; canonical path and symlink rejection (`not is_symlink()`, `resolve(strict=True) == path`) enforced on `R`, `MANIFEST`, all 14 manifest members, `PRIOR_LIB`, `PRIOR_COMPILE_RESULT`, `DM`, `RUST`, `PYTHON`, and all 8 dependency artifacts in `DEPS`.
     - Fresh attempt directory `A = R / 'attempt-financial-core2-reuse-01'` created `0o700`; read-only snapshot files written with `'xb'` and chmodded `0o444` inside `0o555` directories.
     - Exactly 4 children (`00-rustc-version`, `01-compile-financial-core2-tests`, `02-list-exact-tests`, `03-run-exact-tests`), each spawned via `/usr/bin/env -i PATH=/usr/bin:/bin LANG=C.UTF-8 TMPDIR=<A>/tmp` with `cwd=A`, `env={}`, `stdin=subprocess.DEVNULL`, `start_new_session=True`, `timeout=30` seconds per child, process-group `os.killpg(process.pid, signal.SIGKILL)` on timeout, zero retries, and immediate stop on first failure. `host_memory_cap = "none"` and `host_disk_cap = "none"` are explicitly recorded.
     - Postflight verifies `dependencies_unchanged`, `tools_unchanged`, `prior_core_unchanged`, `snapshot_unchanged`, `manifest_unchanged`, and `exactly_four_children` before recording `terminal_state = "success"` and writing `A / 'result.json'` exclusively (`'x'`, mode `0o444`).

3. **Captured Observations & Preserved Metadata Discrepancy Disposition**:
   - **Consumer Equality Checks**: `financial_core2_consumer.rs` prepares both `.mori` sources via `mori_core::stage::load`, asserts full `Program`, `StagePlan`, `program_hash`, and `plan_hash` equality, runs `stage::start` and `stage::resume` threading the **actual** preceding published `Continuation` from `published` (never substituting O's input continuation into `stage::resume`), and asserts equality on the full `Result<StageOutcome, StageRejection>`, borrowed `FinancialObservation` (`project_stage_result` and `financial_observation_matches`), and borrowed `ResumeFinancialObservation` (`project_resume_result` and `resume_financial_observation_matches`).
   - **Sequence Count & `STAGE_LABELS` Verification**: Across `F01` (`start`, `not-yet-accepted`, `accepted` = 1 start + 2 resumes) and `F02` (`start`, `review-not-ready`, `review-ready`, `release-not-ready`, `release-ready` = 1 start + 4 resumes), there are 2 `start` and 6 `resume` invocations (8 total invocations), while all frozen O files remain byte-unchanged (`oracle_source_freeze_sha256`: `6853c508b19b19ff6b4a8c6f081099eb634f38e38660d53d90e36efdd24125d7`, `expected_json_sha256`: `da987c6547bcb4b4785a05108ec4e38198bbf6e2f5566991bdb18089f21b0864`). `verify_test_stdout(tested)` in `run_financial_core2.py` properly derives `expected_resumes` (6 items) and `expected_total_lines` (`18` JSON lines: 2 `load` + 8 `start`/`resume` + 8 `actual_stage_projection_debug`) directly from `EXPECTED_SEQUENCE`, verifies all 6 resume rows report `resume_projection_equal is True`, and verifies `row.get('labels') == [['qualification', 'local-reference-stage'], ['authentication', 'none'], ['consumption', 'untracked']]` matching `stage::STAGE_LABELS`'s JSON serialization as a 3-element list of 2-element string lists.

---

### 3. Coverage & Remaining Limits

- **Execution Status**: **UNRUN**. This preflight review inspects source and metadata only; it does not infer compilation success, test passage, or runtime agreement before execution.
- **Input & Path Scope**: Covers only the two positive normal-path Core/2 financial fixtures (`F01` `InstallmentInvoice` and `F02` `ConditionalGrantTranches`) with `cli_action = None`, `cli_cap = None`, and no `StageRejection` / error-path cases.
- **Provenance & Dependency Limits**: The cached lexer (`libmori_lexer-4eecd0d8cb833138.rlib`, sha256 `5bf3d5f11f818ed96b98e1cb244eecf2f327a2b14f7f274893cbd2d61abf9e8a`), the 5 other extern `.rlib`s (`serde`, `serde_json`, `thiserror`, `sha2`, `k256`), and the 2 proc-macro `.so`s (`serde_derive`, `thiserror_impl`) in `/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target/debug/deps` are pinned and inventoried by binary SHA-256, but their transitive source authenticity and source freshness remain **unestablished**. Host `rustc` (`1.99.0`, commit `b940084d7eb6a299eb4bfeb8e34901bc051e7ac4`), sysroot, linker (`cc`/`ld`/`rust-lld`), `libc`, Python stdlib, and kernel are locally trusted.
- **Non-Acceptance**: Establishes zero Sherlock relation/compiler correspondence, zero financial-agreement/ACTUS/DeFi conformance acceptance, zero native/circuit/proof (PCD) claim, and zero Compact/Preview ledger/target acceptance (`oracle-local` is a fictional local asset-domain string).

---

### 4. Exact Intended Root Invocation & Child Commands

Execution remains gated on **both** this non-author designated-security preflight approval and a fresh Muse exact-runtime preflight approval of `RUNTIME-MEMBERSHIP.json` (`d1486e9b56172f6573c2147db56ac927d6d7b1941d39873509de97592393976b`), followed by a Root-owned execution slot:

- **Intended Root Command**:
  ```bash
  /usr/bin/python3 -I -B /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/financial-core2-binding-01/run_financial_core2.py d1486e9b56172f6573c2147db56ac927d6d7b1941d39873509de97592393976b
  ```
- **Exact 4 Child Commands Spawned by `run_financial_core2.py`** (with `A = /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/financial-core2-binding-01/attempt-financial-core2-reuse-01` and `S = A / 'source'`):
  1. `00-rustc-version`:
     `/usr/bin/env -i PATH=/usr/bin:/bin LANG=C.UTF-8 TMPDIR=<A>/tmp /home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc -vV`
  2. `01-compile-financial-core2-tests`:
     `/usr/bin/env -i PATH=/usr/bin:/bin LANG=C.UTF-8 TMPDIR=<A>/tmp /home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc --edition=2024 --crate-name financial_core2_reuse_check --test <S>/reuse_core_root.rs --extern mori_core=/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/streaming-02-production-preparation/attempt-core-01/libmori_core.rlib -L dependency=/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target/debug/deps -C strip=debuginfo --emit=link,dep-info=<A>/01-tests.d -o <A>/financial-core2-tests --extern k256=/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target/debug/deps/libk256-5362a5e3c63226e9.rlib --extern mori_lexer=/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target/debug/deps/libmori_lexer-4eecd0d8cb833138.rlib --extern serde=/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target/debug/deps/libserde-ca97fc5f093a8961.rlib --extern serde_json=/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target/debug/deps/libserde_json-050e8b0fd297b131.rlib --extern sha2=/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target/debug/deps/libsha2-73f69f806ed827a6.rlib --extern thiserror=/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target/debug/deps/libthiserror-afe06c60c8123fd1.rlib`
  3. `02-list-exact-tests`:
     `/usr/bin/env -i PATH=/usr/bin:/bin LANG=C.UTF-8 TMPDIR=<A>/tmp <A>/financial-core2-tests --list`
  4. `03-run-exact-tests`:
     `/usr/bin/env -i PATH=/usr/bin:/bin LANG=C.UTF-8 TMPDIR=<A>/tmp <A>/financial-core2-tests financial_core2_consumer::independent_core2_financial_two_programs_match_complete_frozen_views --exact --test-threads=1 --nocapture`


# Opus core check classification: four proposed commands

- Reviewer: Claude Opus 5.5 (`claude-opus-5-5`), high effort, sole security owner for this scope. Terminal status: complete.
- Scope: the ordinary execution-closure classification of the four commands in `CORE-PROPOSED-CHECKS.json`. This report does not accept the continuation feature or its security properties. The commands were not run. Native authority stays at 0. No campaign was created and no budget was reset.
- Startup: read the checkout `plugins/moriarty-dev/skills/develop/SKILL.md` and `execution-focus.md` (Opus boundary) plus all of `AGENTS.md`. Guarded `status --json` shows the historical SP01.6 queue with its blockers intact. It is operational history only and was not dispatched.
- Method: read-only source and registry/git-checkout reads, plus `sha256sum` over the pinned paths. No Cargo, build, test, SDK, model or native action was performed.

## Pins

All 25 entries in `CORE-CHECK-SOURCE-PINS.json` match the worktree byte for byte. `normal-oracle.json` = `76075b63…9640`, which matches the prompt.

**Two pins are missing (add them before running; this is not a blocker for the classification).** Both files are compiled into the test binaries through `include_str!`:

- `examples/core/continuations/escrow.mori` `2eb952dd267922c968764154e9d29fbc17f40f5e45fe8b5dac74d229f3d5d60a`
- `examples/core/continuations/reserve.mori` `260f50443d31e58f62d83c8f7f00f3c110e3c116d216feb93f680f0b92224530`

Also assert `git diff --quiet fe01bc8b -- crates/mori-lexer crates/mori-diagnostics crates/mori-span`. `parser::parse_v2` calls `mori_lexer::lex` and `decode_string`, and those crates are unpinned but unmodified in git status.

## Verdicts

| # | Command (`cargo +1.99.0 --offline --locked test -p mori-core --test …`) | Verdict |
|---|---|---|
| 1 | `continuations` | **APPROVE_ORDINARY** |
| 2 | `compat_v1` | **APPROVE_ORDINARY** |
| 3 | `stage_static_cost` | **APPROVE_ORDINARY** |
| 4 | `stage_hostile` | **APPROVE_ORDINARY** |

These verdicts depend on the premises P1–P7 below. If a pin changes, a call path changes, or a premise fails, the classification must be done again.

## Build and link closure (shared by all four commands)

- **Wrapper:** `flock -w 120` on `/tmp/moriarty-language-wave1-build.lock`, which creates the file if it is absent. Then `timeout` 300s with TERM and a 10s kill. Then `env` unsets `RUSTFLAGS` and `MORI_FEE_POOL_CAPTURE_DIR`. `CARGO_TARGET_DIR` points at the shared language-wave1 public object cache, which is not a proving cache. Writes go only to that directory and to the cargo-home locks.
- **Toolchain:** the rustup proxy `+1.99.0` resolves to the installed `~/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu`, and `rust-toolchain.toml` names the same channel. No install happens.
- **Cargo config:** the worktree has no `.cargo/config*`, and neither do `.worktrees/`, `~/Moriarty/` or `~/.cargo/`. So there is no rustc wrapper, runner or linker override (P5 covers `/home/.cargo` and `/.cargo`).
- **Network:** `--offline --locked` means no fetching. Cargo still resolves the whole workspace lock, so the git checkouts for `04c9c5d` and `a01a1ea` must already be present locally. If they are not, the build fails closed.
- **Targets:** only `--test <name>` is built and run. That means the mori-core library (not its `cfg(test)` units), the dependencies, the dev-dependencies and one test binary. No doctests run, no lib units run (`json.rs`/`cost.rs` `mod tests` are excluded) and there is no `--workspace`.
- **Compiled crates:**
  - mori-core normal dependencies: mori-lexer (which pulls in mori-diagnostics, mori-span and soa-rs), serde, serde_json, thiserror, sha2 and **k256** (schnorr and ecdsa).
  - Dev-dependencies: **proptest 1.11.0** (which brings rusty-fork, tempfile and rand 0.9), plus **midnight-base-crypto** and **midnight-serialize** at `04c9c5d`.
  - midnight-base-crypto pulls in reqwest 0.13 with rustls (aws-lc-rs/aws-lc-sys and ring are in the lock), flate2, atomic-write-file, tokio-family crates, k256, rand 0.8 with getrandom, and the proc macros `midnight-base-crypto-derive` and `midnight-serialize-macros`.
- **Compile-time code execution:** if artifacts are not fresh, build scripts run on the host. Examples are the aws-lc-sys and ring C builds through cc/cmake, plus serde, thiserror, typenum and rustix probes. Proc macros also run: serde_derive, thiserror-impl, zeroize_derive, the midnight derive and macros crates, pastey, tracing-attributes and fake derive. I grepped the two midnight proc-macro crates and found no fs, env, process, net or include use. mori-core and mori-lexer have no `build.rs` and none of the crates in that group has `links=`. **Classification: ordinary local compile effects.** The C compilers run offline and produce only object files. No SRS, parameter, key or cache data is read.
- **Link:** none of the four test files references `midnight_base_crypto` or `midnight_serialize`. Their only imports are `mori_core`, `serde_json`, `proptest` and `sha2`. Under edition 2024, an `--extern` crate that is never referenced is not loaded, so **no Midnight SDK, reqwest/rustls, data_provider, parameter-fetch or atomic-write code is linked or executed** (P4). k256 is linked as part of mori-core, but only `intent::check_key`, `prepare_intent` and `verify_*` reach it, and none of the four commands calls them.
- **Mori runtime effects:** I grepped all of `mori-core/src`, `mori-lexer/src`, `mori-diagnostics/src` and `mori-span/src` for fs, process, net, `env::`, `Command`, sockets, libc, unsafe, asm, threads, rand, print and `static mut`. The only filesystem call is `stage::decode::read_bounded` (`File::open`), and no selected test calls it. Every `env` hit is a local variable.

## Per-command call chains

**1 `continuations`** (5 tests, no proptest)
- Compile-time includes: `escrow.mori`, `reserve.mori` and `fixtures/continuations/normal-oracle.json`.
- `stage::load`: `parser::parse_v2` (mori_lexer) → `profile::require_core2` → `validate::validate_v2` → `plan::build` → `plan_invariants` → `cost::fill` → `cost_invariants` → `digest::program_hash`/`plan_hash` (`intent::canonical` sorted serde_json, then `Sha256`) → `intent::hex`.
- `stage::start`: `runtime::scenario_snapshot` → `cost::start_preflight` → `decode::canonical_boundary` → `cost::segment_meter` → **one evaluator, `runtime::run_segment`** → `Admitted::finish` → `emit_checked` (`serde_json::to_string`, then `decode::continuation` → `json::parse_limited`, then `canonical_boundary`).
- `stage::resume`: R0–R7 (format/profile, `digest::matches`, site, capture/event layout, `cost::resume_preflight`, `canonical_boundary`) → `run_segment` → `finish`.
- Everything else is serde_json value comparison against the oracle. Effects: CPU and memory only.

**2 `compat_v1`** (4 tests, no stage code, no proptest)
- serde_json `from_value` into `Program`, `Statement`, `Step` and `Execution` (the `ir::v1` wire), with `Display` comparisons of error strings.
- `mori_core::json::parse`, which is the strict reader.
- `mori_core::simulate`, which is `runtime::simulate`, the legacy evaluator.
- `mori_core::validate` for the profile refusal.
- Effects: CPU and memory only.

**3 `stage_static_cost`** (9 plain tests plus 3 proptests)
- Same load/start/resume chain as command 1, with the CAPTURE, CONSECUTIVE and PLAIN inline sources.
- The test calls `intent::canonical`, `intent::hex` and `sha2::Sha256` directly.
- It also calls `mori_core::parse` (v1), `stage::require_core1`/`require_core2` and the `Statement::Await` constructor.
- Bounded loops: up to 17 awaits, 17 binders and 256 inputs.
- **Correction to the request:** this file sets `ProptestConfig::with_cases(64)`, not 32, so it runs 3 properties × 64 cases. Values come from the default ChaCha RNG seeded from OS randomness and are public sample integers, not keys.

**4 `stage_hostile`** (10 plain tests plus 1 proptest)
- Same load/start/resume chain as command 1.
- `stage::decode::continuation(&str)`, which is the in-memory strict decoder, not `read_bounded`.
- serde_json round trips.
- One property with `with_cases(32)`.

## Proptest output and persistence (commands 3 and 4)

- `Config::default()` reads every `PROPTEST_*` environment variable once. `PROPTEST_FORK` or `PROPTEST_TIMEOUT` would make it re-run the same test binary as a child process (rusty-fork). `PROPTEST_RNG_SEED` and the shrink limits would change behaviour. `CASES` is overridden by `with_cases` (P3).
- **Failure persistence** is `SourceParallel("proptest-regressions")`. No `lib.rs` or `main.rs` exists above `tests/` (checked up to `~`), so proptest prints a warning to stderr and falls back to `WithSource`. On start it reads, and on failure it **appends a seed to**, `crates/mori-core/tests/stage_static_cost.proptest-regressions` or `…/stage_hostile.proptest-regressions` inside the worktree. Neither file exists today. While walking up the directories it only reads file metadata.
- **Classification:** an ordinary, public seed-only artifact that appears only on failure. Root must report it, keep it untracked, and neither commit nor delete it silently. A file that is present before the run changes the replay inputs and requires a recheck (P6).
- Panic output and stdout go to the test harness.

## Premises (each must hold or the classification is redone)

- **P1:** all 25 pins, the two `.mori` pins and the lexer/diagnostics/span baseline diff match at execution time.
- **P2:** exactly these four argument vectors run, one after another, stopping at the first failure. All `MORI_*` variables are cleared without being logged, and `RUSTFLAGS` is unset.
- **P3:** no `PROPTEST_*`, `CARGO_*` overrides other than `CARGO_TARGET_DIR`, `RUSTC_WRAPPER`, `RUSTDOCFLAGS` or `CARGO_BUILD_*` in the child. I recommend unsetting `PROPTEST_*` by name without dumping the environment.
- **P4:** the dev-dependency Midnight crates stay unreferenced in all four test sources, so they are compiled but never linked.
- **P5:** there is no `/home/.cargo/config*` or `/.cargo/config*`, and the toolchain is the installed 1.99.0.
- **P6:** no `*.proptest-regressions` file exists before the run.
- **P7:** the shared target directory holds only public compiled objects (root's premise; not inspected here).

## Not inspected (outside ordinary classification, not waived)

- I did not read the bodies of `runtime::run_segment`/`simulate`, `plan.rs`, `cost.rs`, `validate.rs`, `json.rs` or the lexer line by line. Their effect closure rests on the full-source grep above, not on a correctness audit.
- I did not inspect the build scripts of the third-party registry crates, the aws-lc/ring C sources, or the whole midnight base-crypto source. P4 makes them compile-only.
- The stage outputs carry `authentication: none` and `consumption: untracked`. Tests H8 (stored cap is outside both digests) and H13 (duplicate resume) record known limitations, not security claims.

Old proof, native and target gates keep their full scope. The baseline compiler byte fixtures are not part of these commands. Candidate and actual-result reviews by Muse and Opus are still owed. Test results from these commands are not acceptance evidence.

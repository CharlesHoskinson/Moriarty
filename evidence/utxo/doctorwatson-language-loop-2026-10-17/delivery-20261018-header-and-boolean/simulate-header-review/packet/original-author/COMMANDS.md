# Root-owned execution proposal

No product command below was executed by this author. Root owns integration, the pinned Rust1.99 offline driver, phase timeout and complete process receipts. These commands run only ordinary Rust CLI/library paths with default features disabled. Use a fresh, absent absolute observation directory for each phase; its parent must already exist.

RED, after applying test-only patch to the pinned base:

```sh
MORI_SIMULATE_HEADER_OBSERVATIONS=/ABS/OWNED/FRESH/red cargo +1.99.0 test --offline --locked -j1 -p mori --no-default-features --test simulate_header_dispatch -- --test-threads=1
```

GREEN, after adding production-only patch, with a different fresh capture root:

```sh
MORI_SIMULATE_HEADER_OBSERVATIONS=/ABS/OWNED/FRESH/green cargo +1.99.0 test --offline --locked -j1 -p mori --no-default-features --test simulate_header_dispatch -- --test-threads=1
```

Then justified existing normal Core/2 positive-path regression:

```sh
cargo +1.99.0 test --offline --locked -j1 -p mori --no-default-features --test core2_escrow_complete -- --test-threads=1
```

The new integration target contains one aggregate test, not four Rust tests. RED is expected to fail the first new human stream after all four CLI calls are captured. GREEN should assert all three complete producer JSON errors, their complete exit/stdout/stderr results, and the valid control's complete Execution projection. Existing normal Core/2 target supplies dispatch compatibility; no unrelated reserve/native batch is proposed.

Cargo can build ordinary dependencies and writes its build/cache output. The test creates private input/output files and spawns the actual CARGO_BIN_EXE_mori executable four times via Command (no shell). Each child executes `--color never simulate header.mori --action run --scenario scenario.json` in its own capture directory. No trace, signed, target, resume, prove, wallet, deploy, SDK or native feature/flag is used. The branch reads source and scenario, calls actual Core language/parse/load/start, serializes the actual report, and exits. Error paths return before scenario decoding. There is no host evaluator or AST replacement.

Formatting-only command already performed on private candidate copies and checked, using the workspace's edition2024:

```sh
/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustfmt --check --edition 2024 candidate/crates/mori/src/commands/simulate.rs candidate/crates/mori/tests/simulate_header_dispatch.rs
```

Broad workspace lint/test checks are unrun by this author. Root may apply its already inspected offline/public-registry temporary CARGO_HOME driver; retain exact process bytes and source pins for RED and GREEN. expectationFreezeSha256, sourcePinsBeforeSha256 and sourcePinsAfterSha256 are distinct identities listed in READY.json.

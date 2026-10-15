# Opus 5.5 check-classification deltas (A, B1/B2, C)

Reviewer: Opus 5.5 (runtime model `claude-opus-5-5`; root records the adapter-returned identity). Role: sole security owner, ordinary-execution classification deltas only.
Scope: three independent delta verdicts against completed reports. This is not a correctness, security or feature acceptance.
Actions taken: read-only source inspection; this file is the only write. No cargo, build, test, fmt, SDK, VM, proof, keygen, native verify, deploy, ledger, SRS, key or cache action. Native authority is still 0.
Startup: AGENTS.md and `plugins/moriarty-dev/skills/develop/SKILL.md` were read and applied. The guarded `status --json` invocation was refused by the permission layer, so status was **not** refreshed in this context (repository observation). That does not affect these classifications, which select no registered dispatch.

All premises, preconditions, residuals and C1–C7 from `OPUS-BACKEND-CHECK-CLASSIFICATION.md`, `OPUS-BASELINE-H28-CLASSIFICATION.md` and `OPUS-STATEMENT-ROOT-FIX.md` carry forward unchanged unless a section below names a delta. None is waived.

---

## A. BACKEND-PROPOSED-CHECKS rerun after the `stages.rs` fixture correction: **APPROVE_ORDINARY (delta)**

**Observed (repository/experiment observation):**
- `checks/backend-normal-01/stdout.txt` shows `4 passed; 1 failed; 1 ignored`. The failing test was `generic_scalar_segments_and_zero_await_actions_keep_declaration_order`, which panicked at `stages.rs:45:99`. That location is the `compile()` helper's load `unwrap`, so the failure happened before any assertion ran. This fits the lexer-rejection diagnosis. `result.json` records exit 101, `source_pins_unchanged: true`, `baseline_dependencies_unchanged: true` and `native_actions_requested: 0`.
- Current `stages.rs:120-122` contains exactly the three corrected lines: `flag and enabled`, `when ok and permitted and z >= y`, `requires not (z not = y) or z > y`. The capture assertion at `:131` (`x, flag, y, permitted`) and the test list are unchanged. There are 6 tests, and the only `#[ignore]` is still the baseline comparator at `:157`.
- The rewrite preserves meaning. `!(z != y) || z > y` and `not (z not = y) or z > y` denote the same predicate, and the conjunction order is preserved. The text changed only inside a source-string literal, which is input data to the already-classified pure staged-compile path. The edit adds no I/O, environment read, process, SDK, keygen, proof or native reachability.

**Delta effect:** the command set, argv, reachable call graph, linked closure and residuals are the same ones I classified before. The only security-relevant change is the pinned bytes of one test file.

**Preconditions (A-Δ):**
1. Refresh exactly one pin: `crates/mori-backend/tests/stages.rs` from `fca539fc…869165` to `217c75af42e94b5f01087ccc04d3cb254cf7c50908041a138575ce6e8c51e1e1`. Every other source pin, the six additional input pins and the `fe01bc8b` diff checks must match both before and after the run.
2. Use the same two exact commands, run sequentially, and stop at the first failure. The expected summaries are `5 passed; 0 failed; 1 ignored` for `stages`, then `4 passed; 0 failed; 0 ignored` for `stage_relation`, which has not run yet. Any other summary means stop and diagnose, with no retry loop.
3. Do not pass `--ignored` or `--include-ignored`. The comparator is not part of A.
4. Use the same cleaned environment as the original precondition list.
5. Record this as a new check ID (for example `backend-normal-02`). Keep `checks/backend-normal-01/*` intact as scoped failure evidence.

**Residual (not waived):** a test-only edit by the nonsecurity author is a changed candidate byte. It needs Muse coverage in the owed full result audit. A pass shows that the corrected fixture compiles and that the unchanged assertions hold. It does not provide an independent oracle for staged segmentation.

---

## B. Legacy byte baseline (`LEGACY-BYTE-PROPOSED-CHECKS.json`)

### B1, baseline-binary compile: **APPROVE_ORDINARY** (NEEDS_SOURCE resolved)

**Inspected in the exported old source (`legacy-baseline-source/`, MANIFEST `git_revision` 95df7355…8888):**
- `crates/mori/src/commands/compile.rs:18-60`: `core_io::program`, then `core_io::read_json(target)`. When `signed_policy` is absent, it calls `mori_backend::compile` (unsigned). The manifest is serialized before any filesystem change. `create_dir(out)` fails if the directory exists, and `publish_artifact` writes `NNN-<action>.zkir` per circuit and then `artifact.json` as the completion marker.
- `proof_io.rs:111-140` `publish_new`: a `create_new` staging file in the destination's parent, then `write_all`, `sync_all` and `hard_link` to the final name. It is local, cannot overwrite, and has no network or environment effect.
- Baseline `mori-backend/src/lib.rs:364` `compile` builds an `IrSource` and reloads it with `IrSource::load` (`:1144`, `:1154`). That is a pure parse. The file has no `std::env`, `std::fs`, `Command`, prove, keygen or check call sites.
- `legacy-byte-baseline/target.json` has `network: "local"`, `anchor: "legacy-byte-oracle"` and no `signed_policy`, so the unsigned branch is selected and no Fenrir identity is involved.
- Root recorded an empty diff for Core and backend from 95df7355 to eb2577d6. My earlier backend classification therefore applies to this backend path.

**Not re-read in this pass (limitation):** the bodies of `core_io.rs`, `cli.rs` and `main.rs`. I rely on their MANIFEST pins and on the clap dispatch shape. Correspondence between the binary and this source rests on the R53 provenance of SHA `89ae66c4…f28e`. It was not rebuilt here.

**Preconditions (B1):** `baseline-export` must not exist beforehand. The binary SHA must equal `89ae66c4f8911adaedfec690a0965a88baffc3601c5765fab137c37244eff28e` before and after the run. The `source.mori` and `target.json` SHAs (target `8cd664ce…`) must be re-verified, and the target pin added. MANIFEST.json and every exported source hash must be unchanged. Run once under the stated 30 s timeout. A non-zero exit means stop, with no retry.

### B2, ignored comparator: **APPROVE_ORDINARY**, conditional on B1 success and the fixture-closure step

- The comparator body (`stages.rs:155-170`) is the same as in my earlier reading. The A correction does not touch it. The comparator reads `source.mori`, `target.json`, `artifact.json` and `NNN-*.zkir` from `MORIARTY_STAGE_V1_BASELINE`, recompiles through the current candidate `mori_backend::compile` and asserts byte equality. It writes nothing. It uses `to_vec_pretty`, which is byte-identical to the baseline's `to_string_pretty`, so a format mismatch can't produce a spurious failure or pass.
- **The earlier gap stays open in code and is covered procedurally here.** The test does not prove that the fixture leaf set equals the exported set. Root must therefore do the following:
  1. After B1, compute the leaf set and SHA-256 of every file in `baseline-export`. The set must be exactly `artifact.json` plus one `NNN-<action>.zkir` per `artifact.circuits[]` entry listed in the exported manifest, in index order, with no extra or missing files.
  2. Copy `source.mori`, `target.json` and that leaf set into the fixture root with no overwrite. Hash the fixture root in both directions: every fixture file maps to a source or export hash, and every source or export file maps to a fixture file.
  3. Re-check the same bidirectional set equality after the candidate run, so the fixture cannot change during the run.
  4. The candidate never writes or supplies expected bytes.
- Run the exact single command with `--ignored --exact legacy_artifact_and_every_ir_file_match_baseline_bytes`, once, with the cleaned environment from A. The expected result is `1 passed; 0 failed`, with every other test filtered out.

**Residual (not waived):** a Muse-audited in-test check that the fixture set is closed is still recommended. A byte match establishes /1 non-regression for this one source and target only.

---

## C. H28 `source_closure_coverage` after the statement-root fix: **APPROVE_ORDINARY (delta)**

- The fix is my own `OPUS-STATEMENT-ROOT-FIX.patch`, applied verbatim. Its effect is limited to `statement_root` returning `Result` with a typed `core1_refusal` for `Await`, and the single caller using `?`. It adds no I/O, environment read, process, SDK or proof reachability. The CLI rerun was already approved under D1–D3.
- H28 does not call the `mori_ledger` library. It embeds `proving.rs` via `include_str!` and lists `mori-core/src` paths. `relation/mod.rs` matters only because it is in the compiled dependency closure, which the fix leaves unchanged.

**Preconditions (C-Δ):**
1. Refresh exactly one pin: `crates/mori-ledger/src/relation/mod.rs` from `19f7c4cb…2b0a` to `e1f0fb62f9fa86525e5e621bf799bd374127a9345fae45f20947288bf4898939`. Root must confirm that applying the patch to the old bytes reproduces this hash.
2. All A.4 preconditions from the H28 report still apply: the test is in the source manifest, the `proving.rs` pin and the `mori-core/src/**/*.rs` list and SHAs (including `stage/`) are recorded, there is no Cargo config and no symlink leaving the worktree, and the `CARGO*`/`MORI*`/`PROPTEST*`/wrapper variables are unset without printing them.
3. The 61 `H28-BASELINE-VERIFICATION.json` entries must still match, and the test SHA must be unchanged (`184f35a5…`).
4. No other production file may change relative to the pins in force when the CLI rerun was approved.
5. Run once. A build or test failure means stop and diagnose.

**Residual:** third-party build scripts and proc macros, as before.

---

## Owed, not established here
- Full Muse and Opus code and result audits of the candidate, including the A fixture edit and the C patch.
- No feature, security, product or /1 equivalence acceptance follows from these verdicts.
- Guarded status was not refreshed in this context because the permission layer refused it.

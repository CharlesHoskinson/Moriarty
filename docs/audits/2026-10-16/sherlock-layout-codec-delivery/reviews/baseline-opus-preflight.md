# Security preflight: layout-codec-01 fixed host execution

## Verdict: **APPROVE_EXECUTION_WITH_LIMITS**

This approves one run of the exact 50-member candidate listed in the supplied MANIFEST, under the conditions below. The run is a standalone rustc host compilation of the unwired layout modules, followed by the 35 fixed normal tests.

- **No blocking defect found.** I found nothing in the candidate bytes that would prevent compilation, and nothing in the boundary that grants authority beyond local computation.
- **No guard changes are required.** None are authorized, and Sol must not edit any guard.
- **Nothing has been run.** Compilation and all 35 tests remain UNRUN. I predict no pass.

**Identity and method.** I am Opus 5.5 (`claude-opus-5-5`), reviewing at high effort in a fresh context.
- I made no tool calls, ran nothing, and recomputed no hashes. Digests, file identity and "unchanged" claims are Root transport claims; I accepted them but did not verify them.
- Instructions embedded in the supplied bodies were treated as evidence, not as instructions.
- This verdict does not replace the Muse preflight that `run_layout.py` itself requires.

## Execution boundary as read

Parent: `python3 -I -B run_layout.py`, started from the runtime root. Because of `-I`, only the standard library can be imported, and the script and current directories are not on `sys.path`. `run_layout.py` then:

1. Reads every manifest member once and checks it against `CANDIDATE.json`.
2. Executes the verified `run_host.py` bytes (no re-read, no bytecode) and points `REPO` at `candidate/`.
3. Registers the `stream` realization and creates `attempt-01` exclusively.

The run starts exactly three kinds of child process, each with an argv list (no shell), `PATH=/usr/bin:/bin`, `LANG` and `TMPDIR` only, stdin from `/dev/null`, a new session, and a 30 s process-group kill:

| Child | What it runs |
|---|---|
| Version check | `rustc -vV`; the release must equal the pinned channel `1.99.0` |
| Compile | `rustc --test --edition=2024` on the snapshot copy of `layout_harness.rs`; the system linker is reached via PATH |
| Tests | The test binary with `--test-threads=1 --nocapture` and no filter |

`check.py` is copied into the snapshot and name-scanned for helper loads, but never executed. No producer, comparator, K or model step is selected.

**What the compiled code can do.** I read all 24 compiled sources. They contain no `unsafe`, `include!`/`include_*`, `std::fs`, `std::net`, `std::process` or `std::env`. There are no build scripts, no procedural macros other than built-in derives, and no `#[link]` attributes. The only `#[path]` attribute stays inside the snapshot.

The test binary therefore has pure compute authority plus libtest's own stdout and stderr. Native authority stays 0.

## Guard reuse

`Attempt.preflight`, `version`, `compile`, `tests`, `child`, `postflight` and `finish` are called unchanged.

**Compile-closure check.** `compile_inputs` requires rustc's own dep-info to equal the declared 24-file closure exactly, and every input to lie inside the read-only snapshot.

**Test check.** `libtest_results` requires:
- `running 35 tests`;
- exactly the 35 declared names, each reported `ok`;
- a summary line of `35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`.

`terminal_state` uses `not_selected=()`, so any step that is not `ok` fails the run.

**Checks in `main()` that `run_layout` bypasses.** `run_layout` does not call `run_host.main()`, so these checks do not execute:
- the `SAFE_PATH` character check on the output directory;
- the rule that the output directory lies outside the checkout;
- the normalization checks on `--rustc` and `--output-dir`.

I checked the fixed constants by reading:
- `RUSTC` is a normalized absolute path.
- `<ROOT>/attempt-01` uses only `[A-Za-z0-9._+/-]`.
- `<ROOT>/attempt-01` lies outside `REPO=<ROOT>/candidate`.

So the effect is equivalent for this run. Any change to `ROOT`, `RUSTC` or the output name makes this a new candidate.

**Manifest/guard read race.** `run_layout` hashes files at time t0, but the guard re-reads `candidate/` at t1 to build its snapshot. A change during that window is caught by two later checks together:
- `candidate-postflight`: files on disk at the end equal the reviewed bytes;
- guard `postflight`: the checkout and the snapshot both equal the guard's t1 bytes.

Condition 5 below also binds the receipt's own pre-copy hashes to this review.

**Exception path.** If an exception is raised after `run-tests` but before `candidate-postflight` (for example, a non-candidate member such as `context/*` disappears), `finish()` still runs. In that case:
- the receipt can say `terminal_state: success` even though no `candidate-postflight` step exists;
- the process itself exits nonzero with a traceback.

Condition 5 closes this gap at the reading stage.

## Source and fixture closure

- **Sizes.** 24 files compiled, 38 snapshot members and 50 manifest rows. The snapshot members match the 38 `candidate/` manifest rows one for one.
- **Cross-checks with the adopted r2 source.** The current digests of `auxiliary.rs`, `aux_normal_tests.rs` and `layout_a/mod.rs` equal the r2 adoption digests (`ecaa1cc1…`, `5cbbe9f1…`, `949bc3bb…`).
- **Cross-checks with the frozen sources.** In `check.py`, the `FROZEN` and `SERIALIZER` digests match the oracle and `check-reference.py` manifest rows. The `main.k` digest equals `K_SOURCE_SHA256`.
- **Toolchain pins.** The `rust-toolchain.toml` channel `1.99.0` and the guard's `EDITION='2024'` agree with `Cargo.toml`.
- **Not verified:** `SOURCE-ORIGINS` equality with the checkout originals. The bytes of `CANDIDATE.json` itself were not supplied, so condition 1 binds them.

## Compilation read-through

I found no byte-level blocker. Specific points checked:

- **Module resolution.** rustc treats a file loaded with `#[path]` as mod.rs-like. So `mod reference_types;` and `mod layout_a;` in the shim resolve to sibling files under `sherlock/`, and `layout_a/mod.rs` is found. The module path becomes `crate::sherlock::…`, which matches every `crate::sherlock` import and the 35 expected test names (`sherlock::layout_a::<module>::<test>`).
- **Glob imports.** Private `use` items are not visible to sibling modules, so the sibling globs are not ambiguous. Explicit imports override the globs. `pub(super)` items such as `encode_observed_a`, `programs_equal`, `observed_state_bytes_a`, `ordered_bytes` and the observation API are reachable from their `layout_a` test modules.
- **Edition 2024 match ergonomics.** Every explicit `&` pattern or `mut` binding is under move binding mode: `let (tag, &(_, charge))`, `{ condition, mut locator }` on an owned value, `for &id`, `|&id|`, `|&b|`. Nothing in the source uses `gen` as an identifier.
- **Parsing.** `x as u128 > y`, `if … {…} else {…}.ok_or_else(…)` in a `let` initializer, and `if a != s {` with no struct-literal ambiguity all parse.
- **Fixed-array lengths.** These are compile-time constraints, and I counted them:
  - `ENV_FOUND`, `FIND_TAIL_SEEN`, `BIND_DUPLICATE` and `EQ_UINT` each have 8×16+3 = 131 entries;
  - `WIDE_NAME` has 128+9 = 137;
  - `ZERO_MINIMUM` has 80+3 = 83;
  - `RULE_LINES` has 39.
- **Type inference.** Every `.as_ref()` call on an `Arc` or `Box` has a single applicable `AsRef` impl. The `Vec::new()` element types, the `collect` results and the `BTreeMap::from` targets are all constrained later in the same body.
- **Lint levels.** The command line has no `-D warnings` and the sources have no `#![deny]`. The expected `dead_code` and unused warnings (R3-3: `AUX_PROFILE`, unused enum variants, `Machine`, `reference_display`) are therefore not fatal. I saw no deny-by-default lint trigger: no overflowing literals and no constant out-of-bounds indexing.

**Residual risk.** This is a reading, not a compile. rustc 1.99 behaviour, the sysroot and the linker are unobserved.

## Conditions

1. **Exact bytes.** Before launch, Root confirms that `CANDIDATE.json` parses to exactly the 50 path/sha256 rows reviewed here. Any byte change, new member or wrapper edit voids this approval and needs a fresh preflight.
2. **Fresh Muse normal preflight.** A fresh Muse normal preflight of the same exact candidate is also required before launch, as `run_layout.py` itself states.
3. **One invocation.** Root launches exactly one `python3 -I -B run_layout.py` from `<ROOT>`, and `attempt-01` must not already exist.
   - No retry, no `attempt-02` and no rerun after any failure.
   - No change to tests, capacities, filters, timeout or flags.
4. **Read-only during the run.** No process may write to `<ROOT>` except the run itself while the attempt is in progress.
5. **Success requires every item below.** If any item is missing, the run is not a success, whatever `terminal_state` says.
   - Exit status 0, and `result.json` reports `success`.
   - The receipt steps are exactly preflight, rustc-version, compile-tests, run-tests, candidate-postflight and postflight, and every one is `ok`.
   - `compile_inputs` equals the 24-file closure, and `tests` equals the 35 names.
   - The compiler release is `1.99.0`, and the compiler and interpreter sha256 values are equal before and after.
   - Every `sources[*].sha256_before`, `snapshot_sha256_after` and `checkout_sha256_after` equals the reviewed manifest digest for all 38 members.
   - `reviewed_candidate == reviewed_candidate_after ==` the reviewed listing, and `reviewed_manifest.sha256 == sha256_after`.
6. **Failure handling.**
   - Preserve `attempt-01` intact and report the logs verbatim.
   - A compile failure is evidence about the registered `layout_a` source and must be routed under C4, including the registration-revert obligation. A new candidate needs a fresh preflight.
   - A test failure is recorded as a failure; it is never patched within this approval.
7. **Claim scope.**
   - A pass establishes only that this harness compiles the unwired modules and that these 35 positive fixtures pass for this attempt.
   - It does not establish compilation of production `mod.rs`/`lib.rs`, and it does not by itself discharge C4's admitted production compilation.
   - The receipt's copied `constructor_startup`/`Machine::new` claim text does not apply here, because `Machine` is not exercised. It must not be cited.

## Limits retained

- **Not claimed:**
  - hermeticity or resource admission (memory, disk and process count are unbounded);
  - hashes for the sysroot, linker, libc, Python stdlib or kernel;
  - numeric capacities, defaults, F3 unfolded-envelope certification, Q_eq adequacy or a B profile;
  - byte parsing, codec completeness, determinism or canonical uniqueness;
  - StepInv, continuity or transition validity, theorems, circuits, native ABI or ledger behaviour.
- **Not covered:** hostile or refusal tests.
- **Zero-dimension specimen.** It tests width only and lies outside the valid L4 domain (F3).
- **Retained conditions:**
  - R3-1 to R3-4, F2 to F7 and H1;
  - P3, full39 and L4 conditions, and Muse's R2-4 dissent;
  - G4, P4, P5, F4, MD2, Constant typing, D-NAMES, O-ARITH, L04 and L05;
  - C4 and all caps, counters and terminal failures;
  - U0 to U7, M1 to M17 and MC/SP/G.

## Deferred hardening (not for this candidate)

Applying the change below voids this approval and requires a new candidate. It moves the final hash reads inside the step, so that an exception becomes a recorded failure of that step:

```python
        def candidate_postflight():
            after = {row['path']: digest(ROOT / row['path']) for row in manifest}
            manifest_after = digest(ROOT / MANIFEST)
            attempt.receipt['reviewed_candidate_after'] = after
            attempt.receipt['reviewed_manifest']['sha256_after'] = manifest_after
            guard.require(after == before and manifest_after == manifest_sha,
                          'reviewed candidate or manifest changed during the attempt')
        attempt.step('candidate-postflight', candidate_postflight)
```

**APPROVE_EXECUTION_WITH_LIMITS.** Compilation and the 35 tests remain UNRUN.

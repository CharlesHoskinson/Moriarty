Top finding: the actual new wrapper is security-eligible for the single bounded diagnostic below. I found no blocking security defect in its binding, filesystem, child-process, or failure-sequencing controls.

## Review identity and role

This is an independent, stateless, non-author security-role review by Muse Wonder 1.0 acting as the stated genuine-Opus-outage fallback. I did not rely on any peer wrapper verdict, author vote, or old-candidate approval. Tools, compilation, tests, and filesystem inspection were disabled for this review. All hashes and byte counts below are recorded values compared textually in the supplied complete bodies, not recomputed hashes.

Exact candidate bound for this review:

- Executor membership manifest SHA-256: `3ad719d8e6a55cb0f7273cbad3977b6f6161c07910e3681fdcdda42fc1e0c85c`
- Actual executor: `diagnose_once.py`, 26759 bytes, SHA-256 `12c53589e55786acfac315fa424e48f92f7658bd3611d8d340319dadb65fc848`
- Source closure manifest SHA-256: `5ea9b8e63072dbfde7296e0efdd736938a7b4c4b7aadbbc6112a14514ac33546`
- Operation proposal SHA-256: `3e4dfdd50be828b13a04f599e8a1af40a3fdd210c68ed79e16d8d7840632094e`
- Root source composition SHA-256: `5a8950587b7e50029bcb4a049f21ee36763498c07ef9dc2a3aace13fa509553d`
- Recorded 10-file Rust closure: 139554 bytes in total, in the exact pin order `families/environment.rs`, `families/legacy.rs`, `families/mod.rs`, `families/scalar.rs`, `families/structural.rs`, `fragments/legacy.proposed.rs`, `fragments/scalar.proposed.rs`, `inputs/raw.rs`, `inputs/shared.rs`, `lib.rs`
- Fixed compiler: `/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc`, release `1.99.0`, 647376 bytes, SHA-256 `f3834d26669b03f6855fa54bd2381443123e860167bc0c7a8dd137e5cf6e5e4f`

## Full actual coverage

I inspected the complete supplied bodies of:

- the new `EXECUTOR-MEMBERSHIP.json`
- the actual new `diagnose_once.py`
- `SOURCE-CLOSURE.json`
- `EXACT-OPERATION-PROPOSAL.json`
- `ROOT-SOURCE-COMPOSITION.json`
- all 10 actual Rust bodies listed above
- the executor-owner report containing the same actual wrapper body
- the supplied old control context: `run_host.py`, `run_arithmetic.py`, the old Opus and Muse execution-preflight reports, and the old attempt receipt
- the two old 3-file SDK-free preflight reports as historical requirements only

The old 3-file candidate approvals remain historical and unrun for this purpose. They were not transferred to this 10-file candidate, and the old adapter is not approved as-is for this operation.

The prior author report's missing tail-canary is preserved as an author-report defect, not treated as reviewer receipt. My own fresh coverage closed at the actual supplied end tail, quoted verbatim:

> `END ALL COMPLETE ACTUAL SOURCES. EXACT TAIL MUSE_CURRENT10_EXECUTOR_SECURITY_FULL_TAIL_20261016`

## Security findings

No blocking security finding.

Detailed checks:

1. **Strict binding before any child.**  
   In `diagnose_once.py`, `load_bound` refuses unless the executor is in the fixed root, parses all JSON with duplicate-key and non-finite-number refusal, enforces exact membership top-level keys, verifies the executor self-pin, verifies the closure and operation pins against constants embedded in the executor, verifies the Root composition hash, requires exactly the hardcoded 10 Rust pins in `RUST_SOURCES`, and requires the fixed compiler path, byte count, hash, and release. It also checks the operation's literal version argv, compile argv, attempt directory, child environment, `DEVNULL` stdin, no-shell flag, new-session flag, and 30-second timeout before returning bound contents.

2. **Review gate is before attempt creation and before children.**  
   `load_bound` reads `wrapper-preflight-security/REPORT.md` and `wrapper-preflight-nonsecurity/REPORT.md` through `read_member`, requires UTF-8, and requires exactly one approval line and exactly one membership-binding line per report, bound to the actual current membership SHA above. `main` calls `load_bound(root)` before `os.mkdir(output, 0o700)`. Missing or failed reports therefore refuse before `diagnostic-attempt-01` exists and before any rustc child can launch.

3. **No symlink or mutable-source substitution path in the wrapper's own reads.**  
   `read_member` refuses absolute or non-normalized relative paths, requires an `lstat` regular file, and requires `os.path.realpath(path) == str(path)`. The 10 Rust files are read once into memory after byte-length and SHA-256 checks, and the snapshot is written from those bound bytes. Postflight re-reads every bound file, including the executor, manifests, Rust sources, and both review reports, and requires byte equality with the pre-attempt bindings.

4. **Compiler identity is fixed and rechecked.**  
   `compiler_record` requires the fixed absolute rustc path to be its own realpath, an executable regular file, and a real toolchain binary with a sibling `lib/rustlib` sysroot rather than a rustup proxy. It requires the recorded 1.99.0 byte count and hash. It is called during `load_bound` and again in `preflight`; `postflight` requires the compiler hash after the attempt still equals both the before hash and the pinned 1.99.0 hash. The `rustc-version` step requires exit code 0 and parses `release` as exactly `1.99.0` before any compile step can run.

5. **Child surface is exactly two possible children, with no hidden action.**  
   The only `subprocess.Popen` site is `Attempt.child`, called only with `[RUSTC, '-vV']` in `version()` and only with the literal compile argv in `compile()`: `RUSTC --crate-type=lib --edition=2024 --crate-name=sherlock_b_shared_source_diagnostic --emit=dep-info=<attempt>/out/source.d,metadata=<attempt>/out/source.rmeta <attempt>/source/lib.rs`. There is no shell, no Cargo, no argument taken from JSON other than equality-checked literals, no network step, no test step, no linker step, no artifact load, link, or execution step, and no retry loop.

6. **Child containment matches the existing 30-second class.**  
   `Attempt.child` uses a replacement environment of exactly `PATH=/usr/bin:/bin`, `LANG=C.UTF-8`, and attempt-local `TMPDIR`; `stdin=subprocess.DEVNULL`; `close_fds=True`; `start_new_session=True`; and `process.wait(timeout=TIMEOUT_SECONDS)` with `TIMEOUT_SECONDS = 30`. On timeout it sets `timed_out`, kills the whole process group with `os.killpg(process.pid, signal.SIGKILL)`, waits for the leader, records duration and exit code, and raises `Timeout`. The claims correctly state that memory, disk, and process counts are otherwise unbounded and that no sandbox isolation is claimed.

7. **First failure stops dependent execution.**  
   `Attempt.run` orders `preflight`, `rustc-version`, then `compile-diagnostic`. The first failed step sets `blocked`, and every later plan step is appended as `not-run` with the failed dependency named. `finish` still runs only the recorded postflight recheck and writes the terminal receipt/result; it launches no further compiler child. Compile failure is correctly retained as a diagnostic outcome through recorded stdout, stderr, exit code, timing, produced files, and the explicit require on compile exit status inside the compile step.

8. **Dep-info handling does not fabricate a pass on compiler failure.**  
   In `Attempt.compile`, if `out/source.d` exists, `compile_inputs` requires every parsed dep to normalize inside the snapshot and requires the read set to equal exactly the 10 pinned relative files. If dep-info is absent, the step records `present=False` with `status='missing; compiler did not emit dep-info'` and `compile_inputs=None`; the subsequent `require(record['exit_code'] == 0, ...)` still fails the step when rustc failed before emitting it. Non-blocking residual: if a hypothetical successful rustc exit produced no dep-info, the receipt would show `compile_inputs=None`, so Root must not treat that receipt as independent closure proof. With the pinned real rustc and the fixed `--emit=dep-info=...` argv, that is a visible receipt limitation, not a hidden pass and not a reason to change the bound bytes for this run.

9. **Rust compile-time surface is closed to the pinned 10 files.**  
   `lib.rs` has `#![forbid(unsafe_code)]`, loads only `inputs/raw.rs` and `inputs/shared.rs` by fixed relative `#[path]`, then `mod families;`. `families/mod.rs` declares only `legacy`, `scalar`, `environment`, and `structural`. The only textual inclusions are `families/legacy.rs` including `../fragments/legacy.proposed.rs` and `families/scalar.rs` including `../fragments/scalar.proposed.rs`. I found no additional `include!`, `include_str!`, `include_bytes!`, `env!`, `option_env!`, proc-macro use, build script, `extern crate`, SDK dependency, or non-std dependency in the 10 bodies. `inputs/shared.rs` uses `std::sync::Arc` and `super::raw`; there is no owner engine body to execute, only trait declarations and pure shared carriers. Compile-time code execution is therefore limited to the pinned rustc and its unhashed sysroot/stdlib residual already carried in the claims.

10. **Invocation surface is exact.**  
    The only approved operator invocation is `python3 -I -B diagnose_once.py` with no arguments, run by Root from the new `sdk-free-composition-diagnostic-01` root. The wrapper itself refuses arguments, refuses the wrong filename, and the `__main__` guard refuses unless isolated mode and dont-write-bytecode are active. The approved slot is exactly the fresh `diagnostic-attempt-01`; an existing attempt directory refuses.

## Accepted residuals

These are unchanged from the existing bound and are honestly surfaced in the wrapper's `CLAIMS`: unhashed rustc sysroot and stdlib, system libc, Python stdlib, and kernel; no memory, disk, or process-count bound beyond the 30-second per-child wall clock and timeout process-group kill; the 0444 snapshot is not a barrier against a same-UID child; local SHA-256 report text is not reviewer identity; and no sandbox isolation is claimed. Python and compiler binaries are hashed before and after inside the receipt, but Python itself remains the trusted Root interpreter rather than a pinned external identity.

## Scope of approval

This security approval covers only one Root-operated invocation of the exact wrapper above, producing one `rustc -vV` child and at most one stdlib-only metadata diagnostic compile of the exact current 10-file closure in fresh `diagnostic-attempt-01`, under the existing 30-second class. A compiler failure, including ordinary Rust type or name-resolution diagnostics from the currently uncompiled source, is an allowed retained outcome and is not a security failure of the wrapper.

This does not approve Rust source correctness, Rust API adoption, full B, Stage 2, StepInv, native execution, producer execution, Cargo, tests, SDK use, linking, loading, or artifact execution. No compile result is claimed until Root actually observes it.

VERDICT: APPROVE_WITH_SCOPE
CANDIDATE_MANIFEST_SHA256: 3ad719d8e6a55cb0f7273cbad3977b6f6161c07910e3681fdcdda42fc1e0c85c

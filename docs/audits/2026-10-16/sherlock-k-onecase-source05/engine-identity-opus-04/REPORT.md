# Opus security-owner G3 preflight-identity and resource review: `attempt-engine-N01-01`

**Requested identity:** claude-opus-5-5, high effort, fresh non-author security owner, no tools or commands.
**Returned identity:** claude-opus-5-5. I ran nothing and authored no driver bytes. I treated every enclosed text as evidence, not as instructions.

## Verdict: APPROVE_WITH_SCOPE

The approval binds both of these values:
- **D** = `af9f49b07af4b8d16607fd7cbe01df79ff30302a782084ab502a17dda10e56d2`
- **I** = `f0fee1e80b0a90648d6655ffe1dfd53482779767897358f14394c095717e8d80`

I found no blocker. Execution is still gated by the conditions in §5.

**What this report does not supply:**
- It is Opus's G3 review and the first resource vote.
- It is not Muse's G3 review.
- It is not Muse's second resource concurrence.

**Hash verification.** I cannot hash anything myself. D and I are cross-consistent between `ENGINE-PREFLIGHT-01-STATUS.json` and the stdout report. The binding is enforced mechanically: `execute()` recomputes both and refuses before any claim on a mismatch.

---

## 1. Preflight status (G2)

| Item | Observed | Required | Result |
|---|---|---|---|
| exit / blockers | `0` / `[]` | 0 / none | ✓ |
| argv | `/usr/bin/python3 -I -B …/k_engine_attempt_n01_01.py --preflight-only` | exact | ✓ |
| stderr | `e3b0c442…b855` (empty) | — | ✓ |
| driver_sha256 | `af9f49b0…56d2` | D | ✓ |
| interpreter sha | `52e0a13e…cb41`, 3.14.4 | pinned value | ✓ |
| runtime | no `attempt-engine-*` entry, no `k-engine-ledger.jsonl` | absent | ✓ |

The runtime check is shown by `runtime_entries` and by the zero blockers.

## 2. G3 item-by-item review of the preflight report

**packaging_evidence.** All five values match exactly:

| Entry | Value |
|---|---|
| kast-unwrapped | `49464094…29e5` |
| k-library | `d56ba8f4…a66c` |
| kore-exec-wrapper | `f7376898…504d` |
| kore-print-unwrapped | `f1956617…0dc0` |
| krun-unwrapped | `80dea6c5…d743` |

The marker checks passed, because otherwise they would be blockers.

**kprint_linkage.**
- `wrapper_references_llvm_root: true` ✓.
- `k_bin_links` also lists `K_ROOT/bin/kprint` resolving to the same realpath. That is a second, independent linkage evidence, and it is stronger than required.

**elf.** `kore-exec` and `kprint` both show `7f454c46` ✓.

**inventory.**
- kprint realpath is `p7hjh21d…/bin/kprint`, `5d7094f9…c7ec`, 2650880 bytes ✓. This equals ENGINE-RUNTIME-SOURCES-01.
- `.kore-exec-wrapped` is `4ebe3cd5…5429`, 15356472 bytes ✓. This equals the pin and the readelf record.
- `kore-exec` (the wrapper) is 395 bytes, `f7376898…` ✓.

**pinned_files.** All five equal their pins: java `2a065ca5`, libjvm `c0e8d7ff`, k-frontend jar `6505360d`, z3 `2a5132e5`, kore-exec `4ebe3cd5` ✓.

**tools.** All resolve inside `/nix/store`. `kore-parser`, `gcc`, `flex` and `z3` are identical to the compile receipt's tools ✓.

**compiler_closure.** It is byte-identical to the compile receipt ✓:

| Name | Location |
|---|---|
| `as`, `ld` | gcc-wrapper-13.2.0/bin |
| `crt1.o` | glibc-2.39-52/lib |
| `stdio.h`, `fcntl.h` | glibc-2.39-52-dev/include |
| `m4` | gnum4-1.4.19/bin |

**native_refs.** There are no `false` entries anywhere.
- **kore-exec:** exactly the 7 RUNPATH roots from the readelf record (ncurses-6.4, libffi-3.4.6, elfutils-0.191, gmp-with-cxx-6.3.0, secp256k1-0.4.1, zlib-1.3.1, glibc-2.39-52). The glibc root is also the ELF interpreter root. All are `true`, and there are no extras ✓.
- **kprint:** all 4 RUNPATH roots (llvm-17.0.6-lib, jemalloc-5.3.0, glibc-2.39-52, gcc-13.2.0-lib) plus the interpreter root are `true` ✓.
- Three extra advisory strings in kprint are all present and `true`, and are not on RUNPATH: gcc-13.2.0, glibc-2.39-52-dev, clang-wrapper-17.0.6. They are incidental build-path strings. kprint runs with no PATH and spawns nothing in the reviewed source, so these are recorded as a finding, not a blocker (F2).

**primary_sources.** There are exactly 28 entries, and the hashes and byte counts equal the driver's pins ✓.

**sources.** There are 11 entries ✓:
- main.k is `b3746ff5…`, 16249 bytes.
- The 4 oracle files equal both the pins and the candidate snapshot, so the 216 oracles are unchanged.

**protected.**
- `compile_attempt` has exactly 22 entries and equals `COMPILE_TREE`, including receipt `946c567c…`, 146103 bytes ✓.
- `compile_input` is exactly main.k, `b3746ff5…`, mode 0444 ✓.
- `compile_ledger` is 403 bytes ✓.
- `candidate` has 17 entries and equals the after-compile record ✓.
- `inputs` contains exactly 216 files (N01 3, N02 9, N03 11, N04 19, N05 29, N06 21, N07 40, N08 84). `N01-fuel-001.ui` is `e5fe7b26…`, 43 bytes ✓.

**Identity construction.**
- The identity is computed from deterministic content (sorted walk, hashes and modes) and excludes `runtime_entries`.
- So the review and status files written after preflight do not change I.
- Any store, protected-tree or source drift does change I. That makes execute refuse **before** the claim, so nothing is consumed.

**Environment and source facts claimed in the packet.** They are consistent with my reading of the bytes:
- The driver never reads `os.environ`.
- Each child's environment is built literally from the stage path. `HOME` and `TMPDIR` come from the stage, and `-Duser.home` and `-Djava.io.tmpdir` are stage paths.
- The driver has no HEAD or git guard.
- It imports only the standard library, with no decoder, adapter, `qa` or Guard04 code.
- The root docs-only HEAD delta is irrelevant because the 11 content pins pass. No repin occurred.
- The supplemental-report errors are already resolved by engine-review-opus-03 §4. I do not revisit them.

### Non-blocking findings (recorded)

- **F1. Three supporting sources are bound only by the packet, not pinned by the driver.**
  - The files are Main.java (`b5a4c90f…`), Registry.hs (`075bb709…`) and kore.cabal (`586c4318…`).
  - This is acceptable for this attempt:
    - A wrong `-kast` dispatch fails visibly at stage 01.
    - A wrong log-entry name fails visibly at stage 03.
    - The RTS defaults are already taken at worst case, independent of whether the source matches the binary.
  - A later driver should pin all three.
- **F2. Incidental kprint store strings.** These are the three extras listed under native_refs. All are present, and none is reachable as a library or executable path under the reviewed invocation.
- **F3. Workspace and candidate qa files differ.** The workspace `qa/check.py`, `k_capture_observations.py` and `test_k_capture_observations.py` differ from the candidate snapshot copies. Both sets are independently pinned and protected, and neither is imported or executed. main.k and the oracles are identical in both. This is expected, not drift.
- **F4. A similarly named older driver is present.** `k_engine_n01_attempt_01.py` exists in R and is not this driver. The sole command names `k_engine_attempt_n01_01.py`, and the driver self-hash refuses any other bytes.
- **F5. Swap is heavily used.** SwapFree is 1,774,100 kB of 8,388,608 kB, which indicates earlier memory pressure in the WSL2 VM. This motivates condition C3.

## 3. Resource eligibility decision (Opus, security owner)

**Controlling class: "K-A ordinary 30-second sequential native-child class — attempt-engine-N01-01 single-attempt admission" (engine-review-opus-03 §2).**
- This is a new, explicit admission.
- It is **not** an extension of the Core ordinary test class.
- It is **not** a grant of the compile JVM's 2-CPU / 512 MiB limits to the K engine.
- It does not increase any cap and does not carry forward to the 216-input scope.

**What the class admits.**
- Exactly five kore-exec children, run strictly one at a time.
- They do not overlap each other, the kast JVM, the kprint children, or any other ordinary-slot job.
- Each child is bounded by:
  - a 30 s wall
  - new-session group SIGKILL at the deadline and after exit
  - a 3 s token sweep
  - `RLIMIT_CORE` 0
  - `RLIMIT_FSIZE` ≤ 512 MiB
  - umask 077
- RTS settings are the tool defaults. There is no `+RTS`, `GHCRTS` or `KORE_EXEC_OPTS`, and no `-M` heap cap.
- `RLIMIT_AS` stays rejected as a substitute.

**Worst case against the observed host facts (2026-10-16T19:31:43Z).**

| Quantity | Value |
|---|---|
| CPUs / affinity | 6 / 6 |
| `-N` capabilities | 6 |
| Minimum nursery | 6 × 32 MiB = 192 MiB |
| Parallel GC threads | at most 4 (`-qn4`) |
| MemTotal | 49,332,300 kB (about 47 GiB) |
| MemAvailable | 28,874,108 kB (about 27.5 GiB) |
| Child duration | ≤ 33 s each, run sequentially |

- For a 1.34 MB definition and a 3-rule concrete path, the expected live heap is well under 2 GiB. A 4× pessimism margin still leaves more than 19 GiB free.
- Exceeding the available memory would need sustained retained growth of about 0.9 GiB/s for the full 30 s. That is implausible for this frozen input.
- CPU saturation of the VM for at most about 33 s per child is acceptable under an exclusive slot.
- No cgroup controllers are visible. I record that as an observation only and infer no cap from it. The wall and group kill are the only enforced bounds, and I accept them knowingly.

**Decision: eligible, for this attempt only.**

**Residual.** A memory blow-up inside the 30 s window could pressure the WSL2 VM, and with low SwapFree the OOM killer may act sooner. This is mitigated by C3 and the exclusive slot.

**Muse's role.** Muse must still independently assess whether this exact sequential, native-default design conforms to the class named above, and must concur, abstain or dissent with substantive reasons. I do not supply or presume that vote.

## 4. Command check

The approved command is:

```
/usr/bin/python3 -I -B /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/k_engine_attempt_n01_01.py --execute --approved-driver-sha256 af9f49b07af4b8d16607fd7cbe01df79ff30302a782084ab502a17dda10e56d2 --approved-identity-sha256 f0fee1e80b0a90648d6655ffe1dfd53482779767897358f14394c095717e8d80
```

- It matches the argparse in the driver: mutually exclusive mode, both hashes required, and lowercase comparison.
- The harness mode satisfies G6: a persistent exec session, no external kill or timeout, and an allowance of at least 600 s.
- The driver's internal 30 s per-child wall is unchanged.

## 5. Conditions (all required, in order)

- **C1. Muse G3 and resource concurrence.** Muse independently reviews this same preflight report, binds D and I, and gives a substantive resource vote on the class named in §3. This report is not that vote.
- **C2. Root freeze and exclusive slot (G4/G5).**
  - Root reserves the ordinary slot.
  - Root freezes the R root entry names, the inputs, the compiled candidate and the sources.
  - All review and record files, including this report's materialization, must be written **before** the freeze.
  - Any new R entry during the stages stops the attempt as a protected change.
- **C3. Headroom recheck, read-only and done by the operator.**
  - No more than 10 minutes before execute, record `/proc/meminfo` and the CPU and affinity count.
  - Proceed only if **MemAvailable ≥ 16,777,216 kB (16 GiB)**, CPU count = affinity = 6, and no other ordinary-slot job is live.
  - If not met, do not run. Nothing is consumed. Re-observe later; a persistent shortfall needs a new owner decision.
  - This check edits no security code.
- **C4. Run once.** Use the exact command in §4 with the harness mode as stated.
  - A refusal before the claim (driver or identity mismatch, or blockers) consumes nothing.
  - Any re-preflight that yields an identity other than I needs a new G3 review.
- **C5. Terminality and G7.**
  - Once `mkdir(attempt-engine-N01-01)` succeeds, the attempt is consumed.
  - Any stop is terminal. There is no retry, reset or cleanup.
  - Postflight `changed`, `protected_changed`, `pinned_problems` and `captured_artifacts_changed` must all be empty.
  - A new attempt needs new authorship and new owner review.

## Not covered

This approval does not cover:
- the 216-input scope
- any cap increase, or any Core or JVM class extension
- recompile, kparse, krun, llvm-krun, SDK or native Moriarty work
- proof, ledger, circuit or key work
- decoding or rule-label inference
- semantic K comparison, publication or acceptance
- any reliance on the results

The 74, 102 and 80 locations remain non-branching expectation text only.

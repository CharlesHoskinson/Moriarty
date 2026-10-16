# Opus security-owner review: `attempt-engine-N01-01` driver

**Requested identity:** claude-opus-5-5, high effort, fresh non-author security owner.
**Returned identity:** claude-opus-5-5. This review authored nothing; it ran no tools or commands.

## Verdict: APPROVE_WITH_SCOPE

- **What the approval binds to.** It binds to the bytes of the block headed `FULL …/k_engine_attempt_n01_01.py bytes=53993 sha256=af9f49b07af4b8d16607fd7cbe01df79ff30302a782084ab502a17dda10e56d2`.
- **Hash is unverified by me.** I cannot hash the file on disk. The driver hashes itself, so preflight must print `driver_sha256` = `af9f49b0…56d2`, or nothing proceeds.
- **Embedded copy.** I read the copy inside `engine-author-opus-02/REPORT.md` alongside this block and found no difference. That was a reading comparison, not a byte diff. The binding value is the on-disk self-hash.
- **Remaining conditions.** Approval is still conditional on Muse's independent full-source concurrence and on the preflight identity review (gates G1–G4 below).

I found no blocker. Every finding is either resolved by the new primary sources or admitted below as explicit, recorded scope.

## 1. Author residuals, resolved by the new primary sources

**`-kast` dispatch is evidenced.**
- In `Main.java`, `main` passes `args[0]` to `getInjector`, and `case "-kast"` loads `KastFrontEnd.getModules()` plus the `getKastModules()` plugins.
- The driver's argv after `-kast` uses only options that exist in source:
  - `--definition` (DefinitionLoadingOptions)
  - `--module`, `--sort`, `--input` (KastOptions)
  - `--output` (PrintOptions)
  - one positional file
- The `--input rule` and `--steps` branches are not reached.

**Kast output form is resolved.**
- In `KPrint.prettyPrint`, case KORE calls `addSortInjections(result, s)` with `s` = the `--sort` value, UIRun.
- The root is therefore the `uiRun` constructor, with no outer `inj` and no `kseq`. Inner `uiU` gets `inj{SortUIUInt{},SortUIValue{}}` from the same call.
- This answers concern 7.1 of the definition audit. It also makes `parsed_problem`'s root check correct: the root must start with `UIRUN_OPEN`.
- The driver adds the single outer `inj{SortUIRun{},SortKItem{}}` itself. That matches the only `project:UIRun` equation in the definition.

**Log entry names are evidenced.**
- `Registry.hs` registers both `Proxy @DebugAppliedRewriteRules` and `Proxy @InfoExecDepth`. The text key is `Text.pack . show` of the `SomeTypeRep`.
- `parseCommaSeparatedEntries` uses `sepEndBy` on `,` and requires full consumption, so `DebugAppliedRewriteRules,InfoExecDepth` parses.
- The "stage 03 option-parse failure" residual is closed.

**The rest of the kore-exec argv is valid in source** (`Main.hs`, `SMT.hs`, `KoreLogOptions.hs`):
- `DEFINITION_FILE` is positional.
- `--module`, `--pattern`, `--output` and `--depth` (`auto` over Natural) are defined, so `0` and `1` parse.
- `--smt none` is valid: the options are `map toLower . show` over `[Z3, None]`, and `execute` then calls `runNoSMT`.
- `--log-format standard` and `--disable-log-timestamps` are defined.

**kprint argv is valid** (`main.cpp`):
- The arguments are positional: `<dir>` `<pattern>` `[color]` `[filter_subst bool]`. The values `false` and `false` parse.
- `print_kore` reads `<dir>/syntaxDefinition.kore` and `<dir>/macros.kore`. `macros.kore` is pinned to `// macros\n`, which has zero declarations.
- K's `bin/kore-print` wrapper prepends `p7hjh21d…-llvm-backend-0.1.140/bin`, so `wrapper_references_llvm_root` will be true.
- `bin-unwrapped/kore-print` contains the exact marker `kprint "$kompiledDir" "$input_file" "$color" "$filterSubst"`.

## 2. RTS defaults: ordinary-scope eligibility (owner decision)

**Fact (`kore.cabal`).** The `threaded` flag is `manual: True, default: True`. The common `exe` stanza is `-threaded -rtsopts "-with-rtsopts=-N -T -A32M -qn4"`.
- The Nix build could have overridden the flag, and ELF NEEDED does not show that. I therefore treat the worst case as fact:
  - `-N`: one capability per online CPU
  - `-A32M`: per-capability nursery, so at least 32 MiB × N
  - `-qn4`: 4 parallel-GC threads
  - `-T`: statistics only
  - no `-M` heap cap
- `-rtsopts` also shows that `+RTS` would be accepted. That disproves the author's "support unverified" premise. No change is requested or made now.

**Decision.** All-cores CPU and uncapped memory for kore-exec is eligible for the ordinary 30-second class for this attempt only, as an explicit admission rather than a silent cap increase. The conditions are:
- Five kore-exec children, strictly sequential, never concurrent with each other or with any other ordinary-slot child (gate G5).
- Each is bounded only by the 30 s wall, new-session group SIGKILL, the 3 s token sweep, RLIMIT_CORE 0, RLIMIT_FSIZE ≤ 512 MiB and umask 077.
- An `RLIMIT_AS` cap would not be a valid substitute. The GHC RTS reserves a very large address space at startup, so such a cap would fail every run on launch.
- **Residual:** a memory blow-up inside 30 s could pressure the WSL2 VM. With the frozen N01 input this is improbable. The operator must keep host memory headroom free during the slot.
- The receipt's generic `memory_cpu` string does not name these RTS values. This report is the binding record of them.
- **Not carried forward:** any later attempt, including the 216-input expansion, must re-decide. `+RTS -N<k>` is now known to be available for that.

## 3. Full driver audit (stage by stage)

**Preflight is read-only.**
- Preflight creates no directory, ledger file or child process.
- It gates on:
  - the source content pins (11 files, including the 4 oracle files, so the 216 oracles stay fixed)
  - 28 primary sources
  - 5 pinned binaries
  - the exact 22-entry compile tree, including `receipt.json` `946c567c…`
  - the compile ledger, which must be exactly claim then finish, bound to `05086da9…` and `946c567c…`
  - compile-input, which must be exactly main.k `b3746ff5…`
  - the candidate snapshot against `2a7786ee…`
  - the input `e5fe7b26…` (43 bytes)
  - ASCII content
  - macros being empty
  - the eight definition markers
- Every marker matches the literal declarations supplied. The uiRun marker is `UIRUN_OPEN[:-1]` plus the argument sorts.
- `identity` is computed over deterministic content: `os.walk` with sorted children. It excludes `runtime_entries`, so adding review reports does not change it.

**There is no HEAD guard.**
- The driver pins content hashes only. It reads no git state.
- The root-docs-only HEAD delta (`c9f7cf6a…` / `71dfae47…`) is therefore irrelevant provided the 11 content pins pass. Nothing needs to be repinned.

**The claim is exclusive and terminal.**
- The claim is made with `mkdir(ATTEMPT)`.
- The baseline is taken after the claim and after the ledger file is created, so the engine's own entries are inside the baseline.
- Each stage is claimed in the ledger before its directory exists.
- Every exit path appends `stage-finished`. Any problem stops the attempt, and no code path retries or deletes anything.

**Stage 01, kast.**
- It uses the same JVM flags, PATH and root order as the admitted compile.
- Scanner (flex, gcc, scanner) and TypeInferencer (`z3 -in`) grandchildren inherit the session and the token.
- When the JVM exits, their stdin reaches EOF, and any survivors are swept as in-group processes, which does not count as a breach.
- **Residual (fails visibly):** if K leaves a temporary directory in the working directory, it counts as an unexpected stage file. If K writes into `main-kompiled`, that is detected as a protected change but not prevented. This is the author's admitted decision 10.

**Stage 02, initializer.**
- The driver writes `LblinitGeneratedTopCell{}(Lbl'UndsPipe'-'-GT-Unds'{}(inj{SortKConfigVar{}, SortKItem{}}(\dv{SortKConfigVar{}}("$PGM")),inj{SortUIRun{}, SortKItem{}}(<parsed>)))`.
- Its sorts match the hooked `MAP.element` signature `(SortKItem, SortKItem) → SortMap` and the initializer signature `SortMap → SortGeneratedTopCell`.
- The parsed-term gate admits no variables (no `:`) and no matching-logic connective other than `\dv`. It requires balanced parentheses and a single application, skips string literals correctly, and requires printable ASCII.
- The driver writes no cell, default value or rule label.

**Stages 03–07, kore-exec.**
- They run the direct ELF, the same file the installed wrapper `exec -a`s. PATH is z3/bin only, and there is no `KORE_EXEC_OPTS` or `GHCRTS`.
- `withBugReport` uses `TMPDIR`, which is `stage/tmp`. It copies `definition.kore` and the pattern there, and writes `kore-exec.log` and `result.kore` there. The result is then copied to `stage/state.kore`. All of this is permitted residue or the expected product.
- Logs go to stderr, which is captured raw.
- An error bug report written into the working directory counts as an unexpected file. That stage fails anyway.
- The ELF RUNPATH covers every NEEDED library: ncurses, libffi, elfutils (libelf and libdw), gmp, secp256k1, zlib and glibc.
- **Design consequence (admitted):** if stage 07 fails, stages 08–12 do not run. The raw engine states for 03–07 are still captured.

**Stages 08–12, kprint.**
- They run with no PATH. RUNPATH covers llvm-17-lib, jemalloc, glibc and gcc-lib.
- `filter_subst` is false, so all conjuncts are retained. The raw state with all 10 cells is retained as captured. The pretty print is observation only.

**Containment is unchanged from the compile pattern.**
- The pattern is: new session, group kill at the deadline and after exit, and a `/proc` token sweep.
- A breach is recorded if anything is still alive after the sweep, or if a tagged process was outside the group.

**Imports.** The driver imports only the Python standard library. It does not import any decoder, adapter or `qa` module. It only hashes those files.

## 4. Supplemental definition report: the two noted errors

**1. The receipt hash was called "the compile/main SHA". That is incorrect.**
- `946c567c…9f69` is `attempt-compile-01/receipt.json` (146103 bytes).
- main.k is `b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41` (16249 bytes).
- The binding chain for the spans is:
  - the compiled `Source(…/compile-input/models/sherlock/k/main.k)` attribute
  - compile-input, pinned by preflight to exactly `b3746ff5…`
  - `COMPILED-MAIN-RULE-SPANS-01`, which records `main_sha256` `b3746ff5…`
- I counted lines in the supplied main.k text. Line 6 is `uiU`, line 30 is `uiRun`, line 74 is the startup rule, line 80 is the `uiRet`/`uiNoFrames` publication rule and line 102 is `uiEval(uiLiteral(V))`. These agree with the Location attributes.
- This is a labelling error only. There is no factual incompatibility, and no driver change is needed.

**2. The report asks the depth-0 state to have "no residual `Lbl'Stop'Map`". That criterion is wrong.**
- `initEnvCell` evaluates to `<env>(Lbl'Stop'Map{}())`.
- `Lbl'Stop'Map` is the hooked `MAP.unit`, declared as `unit{}(Lbl'Stop'Map{}())` on `SortMap`.
- An empty `<env>` is therefore expected to appear as `Lbl'Stop'Map{}()`. Its presence is not unreduced residue.
- The meaningful residue indicators would be surviving applications of `LblinitGeneratedTopCell`, `LblinitKCell`, any other `Lblinit*Cell`, `Lblproject'Coln'UIRun` or `LblMap'Coln'lookup`.
- The driver implements no such check and branches on nothing, which is correct.
- Any later reviewer must not apply the `Lbl'Stop'Map` criterion.

**No additional generated context is needed** for this bounded diagnostic. The literal initializer equations, declarations, the three rules on the path and the full syntax definition are sufficient. No whole-language semantic audit is implied. The expected rules 74, 102 and 80 remain expectation text only.

## 5. Explicit per-stage scope

| Stage | Executable | Environment (besides HOME=stage/home, TMPDIR=stage/tmp, LANG=C, LC_ALL=C, token) | Limits |
|---|---|---|---|
| 01 | pinned JRE `java` → `org.kframework.main.Main -kast` | PATH = kore-38afc81/bin : gcc-wrapper/bin : flex/bin : z3/bin | 30 s wall + 3 s sweep, CORE 0, FSIZE min(hard, 512 MiB), umask 077, new session; JVM heap 64m..512m, stack 32m, 2 processors; native grandchildren at defaults |
| 02 | none (driver writes `initializer.kore` with mode 0400) | — | — |
| 03–07 | `.kore-exec-wrapped` (`4ebe3cd5…`), `--smt none` | PATH = z3/bin | as stage 01, without JVM bounds; RTS `-N -T -A32M -qn4`, no heap cap (§2) |
| 08–12 | `kprint` (expected `5d7094f9…`, 2650880 bytes) | no PATH | as stage 01, without JVM bounds; tool defaults |

**Not covered:** recompile, kparse, krun, llvm-krun, SDK, native Moriarty, proof, ledger, circuit or key work, decoding, comparison, retry, install, fetch, network, or any other input. Absent dependencies fail visibly. Any reliance on these results, or any semantic K comparison, remains separate and blocked.

## 6. Gates

**G1. Muse concurrence.** Muse independently concurs on these bytes before preflight is run.

**G2. Preflight.** Run `/usr/bin/python3 -I -B …/k_engine_attempt_n01_01.py --preflight-only`. Required results:
- zero blockers
- `driver_sha256` `af9f49b0…56d2`
- interpreter `52e0a13e…cb41`

**G3. Preflight review by Opus and Muse.** Both review the full report and bind `identity_sha256` I. The required values are:
- **packaging_evidence:**
  - kast-unwrapped `49464094…29e5`
  - k-library `d56ba8f4…a66c`
  - kore-exec-wrapper `f7376898…504d`
  - kore-print-unwrapped `f1956617…0dc0`
  - krun-unwrapped `80dea6c5…d743`
- **kprint_linkage:** `wrapper_references_llvm_root` true.
- **elf:** both `7f454c46`.
- **inventory:** the kprint realpath shows `5d7094f9…` with 2650880 bytes.
- **compiler_closure** equals the compile receipt's values:
  - `as` and `ld`: gcc-wrapper/bin
  - `crt1.o`: glibc-2.39-52/lib
  - `stdio.h` and `fcntl.h`: glibc-2.39-52-dev/include
  - `m4`: gnum4-1.4.19/bin
- **native_refs:**
  - Every RUNPATH root in ENGINE-RUNTIME-SOURCES-01, plus the glibc interpreter root, must be `true`. Any `false` among these blocks execution.
  - Other `false` entries are findings for both reviewers. The advisory regex can capture incidental strings.

**G4. Freeze.** Root reserves the ordinary slot and freezes R root entry names, inputs, the compiled candidate and the sources.

**G5. Exclusive slot, then execute once.**
- No other child may run during the attempt.
- Run once: `… --execute --approved-driver-sha256 af9f49b07af4b8d16607fd7cbe01df79ff30302a782084ab502a17dda10e56d2 --approved-identity-sha256 I`

**G6. Harness timeout (operational).**
- The harness running the execute command must allow at least 600 s and must not kill the driver.
- Worst case is roughly 11 × 33 s plus two preflights and a postflight.
- The driver installs no SIGTERM handler. If it were killed, children in their own sessions would lose wall enforcement, no receipt would be written, and the attempt would still be consumed.

**G7. Postflight.** Postflight `changed`, `protected_changed`, `pinned_problems` and `captured_artifacts_changed` must all be empty. Any stop is a terminal observation, and a new attempt needs new authorship and new review.

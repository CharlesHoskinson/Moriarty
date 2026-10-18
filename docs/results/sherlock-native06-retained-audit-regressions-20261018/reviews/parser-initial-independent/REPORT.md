# R404 — Independent Non-Author Review of Parser-Only Test Module (`8c1c9cc1...`) & Three-Method Ordinary Operation

## 1. Reviewer Identity, Route & Coverage

- **Designated Security Owner**: Seat C (`CID: 3b833d1d-96eb-4d82-a33a-abc28e7d3533`), original cwd `/tmp/moriarty-r304-fresh-independent-disposition-_kt12zfw`, model `gemini-4-argon-high` (`HIGH` thinking) under standing observed `claude-opus-5-5` (`HIGH`) outage fallback.
- **Authorship Independence**: Non-author of [`test_depinfo_retained_audit.py`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/404-native06-parser-only-test-independent-review/source/test_depinfo_retained_audit.py) (`10430` B, `sha256: 8c1c9cc1b68c1b5f9f266e42fa9f6e1a92fdeaf52c168f56a99930287c0e273b`, authored by `6277cc2c-99b8-4b7a-a220-a6c971d1bb98`) and candidate guard [`bounded_sdk_compile.py`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/398-native06-retained-audit-owner-candidate/afterimages/bounded_sdk_compile.py) (`79818` B, `sha256: de3cb754f3483e5100faf1a7313ae08e5dc2ac4dae27ee1d79a97910f4380a53`).
- **Tools Invoked**: **ZERO (`0` tool calls)**.
- **Complete Readable Coverage**: Read 100% of all 3 inline bodies (`0` unread or truncated bytes):
  1. [`test_depinfo_retained_audit.py`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/404-native06-parser-only-test-independent-review/source/test_depinfo_retained_audit.py) (`10430` B, `8c1c9cc1...`, 255 lines, ending at `unittest.main()`)
  2. [`ACTUAL-PARSER-API.py.txt`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/404-native06-parser-only-test-independent-review/inputs/ACTUAL-PARSER-API.py.txt) (`3506` B, `6cdefa10...`, exact unchanged `Refusal`, `Unsupported`, `need`, `absolute`, `regular`, `make_words`, `depinfo_path`, and `parse_depinfo` bodies from `de3cb754...`)
  3. [`EXACT-THREE-PARSER-METHOD-OPERATION.proposed.json`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/404-native06-parser-only-test-independent-review/EXACT-THREE-PARSER-METHOD-OPERATION.proposed.json) (`3099` B, `5fffce41...`)

---

## 2. Summary of Independent Dispositions

1. **Parser Test Module Source Verdict ([`test_depinfo_retained_audit.py`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/404-native06-parser-only-test-independent-review/source/test_depinfo_retained_audit.py), `10430` B, `8c1c9cc1...`)**: **`APPROVE` (PARSER-COHORT TEST SOURCE ONLY — zero fixture defects, zero required edits)**.
2. **Exact Isolated Three-Method Ordinary Operation ([`EXACT-THREE-PARSER-METHOD-OPERATION.proposed.json`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/404-native06-parser-only-test-independent-review/EXACT-THREE-PARSER-METHOD-OPERATION.proposed.json), `5fffce41...`)**: **`ELIGIBLE_FOR_ROOT_ORDINARY_EXECUTION` (ONLY `test_depinfo_retained_audit.DepinfoNormalizationTests`, 3 test methods)**.
3. **Preserved R399 Guard & Lifecycle Scope Qualifications**:
   - The R399 `REVISE` findings on `bounded_sdk_compile.py` (`de3cb754...`) regarding `launch` Phase F log-byte accumulation, `retained_f_audit` wall/CPU budget deduction & receipt-chain qualification (`F_RETAINED_AUDIT_RESULT.json`), and `sandbox_args` flag validation remain **open and unwaived** for guard adoption.
   - Changed-source unit tests for `output_check` (`worktree_files`, `D_FREEZE`, SDK `includes`, host-unit association) and the `--retained-f-audit` lifecycle remain **PENDING** in separate follow-up work and are **not** satisfied by this parser-only cohort.

---

## 3. Technical Audit of `test_depinfo_retained_audit.py` (`8c1c9cc1...`)

### 3.1 Real API Usage, Fixture Coherence & Cleanup
- **Real Guard APIs Only**: Calls only `runner.depinfo_path`, `runner.parse_depinfo`, `runner.make_words`, `runner.Refusal`, and `runner.Unsupported` with their exact signatures (`depinfo_path(word, worktree, roots=())` and `parse_depinfo(text, worktree, roots=())`).
- **Coherent `TemporaryDirectory` Hierarchy (`setUp` / `tearDown`)**:
  - `self._tmp = tempfile.TemporaryDirectory()` and `self.base = Path(self._tmp.name).resolve()` construct a real, symlink-free root directory.
  - Materializes real directories (`crates/mori-backend/src/sherlock_machine` under `self.worktree`; `git/checkouts/midnight-ledger-b2f9c59d942dfdca/04c9c5d/zswap/src` under `self.source_home`) and real `0o644` regular files (`self.lib_rs`, `self.expected_json`, `self.out_rlib`, `self.zswap_lib`, `self.spend_verifier`) via `_write_file`.
  - All files (`0o644`) and directories (`0o700`/`0o755`) remain owner-writable, and the two symlinks created in `test_parse_depinfo_rejects_root_escapes_invalid_parents_and_cross_crate_traversals` (`symlink_parent` and `symlink_target`) live inside `self.worktree`, so `self._tmp.cleanup()` in `tearDown()` unlinks symlinks without following them and cleanly removes all temporary artifacts after each test.

### 3.2 Audit of the 3 Test Methods
1. **`test_parse_depinfo_resolves_relative_and_crate_contained_parent_segments`**:
   - Verifies `runner.depinfo_path` directly on both a 4-level relative `..` path (`crates/mori-backend/src/sherlock_machine/../../../../models/sherlock/machine-circuit/expected.json` -> `str(self.expected_json)`) and an admitted `roots` absolute `..` path (`{self.ledger_root}/zswap/src/../static/spend.verifier.sha256` -> `str(self.spend_verifier)`).
   - Verifies `runner.parse_depinfo` over a multi-line rule with `\\\n` continuations, `# env-dep:RUSTC_BOOTSTRAP=1` comment skipping, dependency-side resolution, and `-MP` phony target lines (`{self.lib_rs}:`, `{rel_dotdot}:`, `{abs_dotdot}:` -> `(target_set, set())`).
2. **`test_parse_depinfo_preserves_make_continuations_and_escaped_filename_tokens`**:
   - Creates a real file named `spec #1:v2\final.json` and tests `spec\ \#1\:v2\\final.json` after `../../../../` on both the right-hand dependency side and the left-hand `-MP` phony target side (`{escaped_rel_dotdot}:`).
   - Exercises `parse_depinfo`'s escaped-colon (`\:`) handling in the delimiter scan as well as `make_words` unescaping of `\ `, `\#`, `\:`, and `\\` before `depinfo_path` resolution.
3. **`test_parse_depinfo_rejects_root_escapes_invalid_parents_and_cross_crate_traversals`**:
   - In all cases with `:`, the left-hand target is `f'{self.out_rlib}'` (a valid normalized absolute path with no `..`), ensuring `paths(line[:delimiter])` succeeds first and the refusal is triggered specifically by the right-hand test case.
   - Verifies **15 distinct `runner.Refusal` branches** with specific expected message assertions (`self.assertIn(expected_msg, str(ctx.exception))`):
     - Lexical stack underflow / empty stack / `.` / `//` / root-escape refusals (`'non-normalized dep-info path'`): `../escape.json`, `crates/../../escape.json`, `crates/..`, `./crates/mori-backend/src/lib.rs`, `crates//mori-backend/src/lib.rs`, cross-crate `{pkg_a}/src/../../pkg-b-1.0.0/src/file.rs` (where `base` binds to `pkg_a` via longest-prefix match in `roots`), and unlisted root `{self.base}/unlisted/sub/../escape.json`.
     - Ascended directory identity refusals (`f'dep-info parent directory identity: {d}'`): nonexistent intermediate directory (`nonexistent_dir/..`), symlink intermediate directory (`symlink_dir/..`), and regular file traversed as directory (`crates/mori-backend/src/lib.rs/../../../../../...`).
     - Terminal target refusals: missing target (`f'dep-info target missing: {...}'`), symlink target (`f'symlink path: {symlink_target}'` from `regular(norm)`), and directory target (`f'not regular: {...}'` from `regular(norm)`).
     - Syntax refusals: missing colon (`'unrecognized dep-info rule'`) and trailing backslash (`'trailing dep-info escape'`).
   - Separately verifies all **3 `runner.Unsupported` characters** (`'$'`, `';'`, `'|'`) raise `runner.Unsupported` with `'unsupported dep-info syntax'`.

---

## 4. Audit of the Proposed Three-Method Ordinary Operation (`5fffce41...`)

- **Isolated Suite & Selector**:
  - Suite directory: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/404-native06-parser-only-test-independent-review/prospective-isolated-parser-suite` containing only `bounded_sdk_compile.py` (`79818` B, `de3cb754...`) and `test_depinfo_retained_audit.py` (`10430` B, `8c1c9cc1...`).
  - Selector: `test_depinfo_retained_audit.DepinfoNormalizationTests` with `assert suite.countTestCases() == 3`.
  - Note: Although `bounded_sdk_compile.py` (`de3cb754...`) requires revision in `launch`/`retained_f_audit`/`sandbox_args` before guard adoption, those functions are never called by `DepinfoNormalizationTests`, which exercises only the already-approved parser/path helpers.
- **Import-Time & Runtime Effects / Resource Bounds**:
  - Importing `bounded_sdk_compile` and `test_depinfo_retained_audit` under `/usr/bin/python3 -I -B` executes only stdlib imports, class/function definitions, and `os.getuid()`/`os.getgid()`.
  - Runtime creates and cleans up 3 sequential `TemporaryDirectory` trees inside `TMPDIR` (the isolated suite directory), writes tiny (<30 B) synthetic files, creates 2 local symlinks, and performs local `stat`/`lstat`/`realpath` calls.
  - Spawns **zero** child processes (`0` forks/subprocesses), performs **zero** external filesystem, cgroup, mount, network, sudo, Cargo, or SDK operations, and is strictly bounded by `uid=1000`, `gid=1000`, `timeout --signal=KILL 30s`, `131072` B capture cap, and `prlimit` (`--cpu=25:25`, `--as=1073741824:1073741824`, `--fsize=1048576:1048576`, `--core=0:0`).

---

## 5. Standing Boundaries

- **Tests Executed in This Turn**: `0` (Root alone selects and executes the approved 3-method ordinary test operation).
- **`native_authority`**: `0` (no retained-audit or native operation selected or approved).
- **`R2`**: `UNSATISFIED` | **`C4`**: `OPEN` | **Amendment 10**: `STOPPED`.


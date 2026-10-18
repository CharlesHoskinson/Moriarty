### Independent Non-Author Parser Test-Delta & Single-Negative-Method Ordinary Eligibility Review (R406)

- **Reviewer Identity & Route**: Conversation `29f6efee-2b5e-4f3f-b663-bad4292ff558` | Model: **Gemini 4 Argon (High)** via `gemini-4-argon-high` standing fallback under Root-observed Opus quota outage (not Opus).
- **Non-Author Independence**: Confirmed non-author of the original parser test module (`6277`), the R405 fixture delta (`3b833`), and `bounded_sdk_compile.py` (`de3cb754...`). Zero tools, commands, or executions invoked.
- **Verdict Summary**:
  1. **Test-Source & Literal Delta Assessment**: **`APPROVE`**
  2. **Single-Negative-Method Ordinary Execution Eligibility**: **`APPROVE`** (strictly scoped to the 1 selected negative method under the declared ordinary resource limits; separate from native/guard/SDK eligibility).

---

### 1. Test-Source & Exact Literal Delta Assessment (`APPROVE`)

#### A. Five-vs-Four `..` Traversal Distinction & R404 Failure Root Cause
In `depinfo_path(word, worktree, roots=())`, relative paths containing `..` are processed segment-by-segment against `stack` and `ascended`:
```python
stack=[];ascended=[]
for part in rel_parts:
    if part=='..':
        need(bool(stack),'non-normalized dep-info path')
        ascended.append(base.joinpath(*stack));stack.pop()
    else:stack.append(part)
need(bool(stack),'non-normalized dep-info path')
norm=base.joinpath(*stack);raw=base/('/'.join(rel_parts))
for d in ascended:
    need(not any(x.is_symlink() for x in [d,*d.parents]) and d.is_dir() and d.resolve(strict=True)==d,'dep-info parent directory identity: '+str(d))
```
- **Why R404 Failed (`exit 1`, `0.071057s`)**: The path prefix `crates/mori-backend/src/lib.rs` pushes **4** lexical components onto `stack`. In R404, the fixture followed those 4 components with **5** `..` segments (`../../../../../models/...`). The first 4 `..` segments popped `stack` to empty (`[]`); the 5th `..` hit `need(bool(stack), 'non-normalized dep-info path')` *during lexical stack unwinding*, raising `Refusal('non-normalized dep-info path')` before the `for d in ascended:` loop ever ran.
- **Correctness of the R406 Literal Delta (`@@ -189,11 +189,20 @@`)**:
  1. **Retained 5-`..` Case (`'five .. segments after four components underflow lexical stack'`)**: Preserves `crates/mori-backend/src/lib.rs/../../../../../models/sherlock/machine-circuit/expected.json\n` with expected substring `'non-normalized dep-info path'`, testing lexical stack underflow after descending into a 4-component path.
  2. **Added 4-`..` Case (`'regular file traversed as parent directory before ..'`)**: Uses `crates/mori-backend/src/lib.rs/../../../../models/sherlock/machine-circuit/expected.json\n` (**4** `..` segments after 4 components). Here `stack` pops cleanly to `[]` and then pushes `models/sherlock/machine-circuit/expected.json`. During the subsequent `for d in ascended:` check, the first ascended path `d = self.worktree / 'crates/mori-backend/src/lib.rs'` (`self.lib_rs`, created as a regular file in `setUp`) fails `d.is_dir()`, raising `Refusal(f'dep-info parent directory identity: {self.lib_rs}')`.
- **Preservation of Passing Methods**: Both positive test methods (`test_parse_depinfo_resolves_relative_and_crate_contained_parent_segments` and `test_parse_depinfo_preserves_make_continuations_and_escaped_filename_tokens`), `setUp`, `tearDown`, and `_write_file` are byte-identical to R404. No guard logic was weakened or altered to fit the fixture.

#### B. Verification of All 16 `Refusal` Subtests & 3 `Unsupported` Subtests
Every subtest in `test_parse_depinfo_rejects_root_escapes_invalid_parents_and_cross_crate_traversals` asserts a specific, meaningful exception type (`runner.Refusal` or `runner.Unsupported`) and exact diagnostic substring (not a generic catch-all assertion):
- **Lexical & root-containment refusals (`'non-normalized dep-info path'`)**:
  - Immediate relative escape (`../escape.json`), post-descent escape (`crates/../../escape.json`), collapse to worktree root (`crates/..`), dot segment (`./crates/...`), empty double-slash segment (`crates//mori-backend/...`), 5-`..` stack underflow after 4 components, cross-crate escape out of `pkg_a` (`{pkg_a}/src/../../pkg-b-1.0.0/src/file.rs`), and absolute `..` path outside admitted roots (`{self.base}/unlisted/sub/../escape.json`).
- **Ascended parent-directory identity refusals (`'dep-info parent directory identity: <path>'`)**:
  - Nonexistent directory before `..` (`nonexistent_dir/..`), symlink directory before `..` (`symlink_dir/..`), and regular file traversed as directory before `..` (`crates/mori-backend/src/lib.rs/../../../..`).
- **Resolved target refusals**:
  - Missing target (`'dep-info target missing: .../missing.json'`), symlink target (`'symlink path: .../symlink_expected.json'`), and directory target (`'not regular: .../machine-circuit'`).
- **Rule & token syntax refusals**:
  - Missing colon delimiter (`'unrecognized dep-info rule'`), trailing backslash escape (`'trailing dep-info escape'`), and shell/make metacharacters `$`, `;`, `|` raising `runner.Unsupported` with `'unsupported dep-info syntax'`.

---

### 2. Single-Negative-Method Ordinary Execution Eligibility (`APPROVE`)

#### A. Import-Time Safety & Finite In-Process Behavior
- **Guard Import (`import bounded_sdk_compile as runner`)**: The supplied top-level AST statements of `bounded_sdk_compile.py` (`de3cb754...`) consist solely of standard-library imports and pure constant assignments (`PHASES`, `CARGO`, `CARGO_PATH`, `BACKEND`, `CLI_OBLIGATION`, `BASE`, `LOCK`, `REQUIRED_MASKS`, `HEX`, `CHUNK`, `GIT_SYMLINK_MODE`, `GIT_SYMLINK_TEXT_BYTES`, `GIT_SYMLINKS`, `TRAMPOLINE`). The `if __name__ == '__main__':` block is inactive on import; no subprocess, mount, seccomp, cgroup, network, or filesystem I/O executes at import time.
- **Filesystem Setup, Symlinks, `chmod`, and Teardown Cleanup**:
  - `setUp()` creates `self._tmp = tempfile.TemporaryDirectory()` (under the isolated `TMPDIR` in the prospective suite directory).
  - All files (`_write_file` with `mode=0o644`), directories (`mkdir(parents=True)`), and symlinks (`symlink_dir` and `symlink_expected.json`) are created strictly inside `self._tmp`.
  - File permissions (`0o644`) and directory permissions permit unprivileged deletion; `tearDown()` unconditionally calls `self._tmp.cleanup()`, leaving no residual files or symlinks.
  - Passing `-I -B` to `/usr/bin/python3` prevents user-site loading and `.pyc` bytecode emission.
- **Resource & Process Bounds**:
  - Runs a single deterministic in-process `unittest` method (`assert suite.countTestCases() == 1`) with zero child processes, threads, or blocking I/O (R404 ran all 3 methods in `0.011s` / `0.071s` wall).
  - Bounded by `/usr/bin/timeout --signal=KILL 30s` and `/usr/bin/prlimit --cpu=25:25 --as=1073741824:1073741824 --fsize=1048576:1048576 --core=0:0` under fixed `uid=1000`, `gid=1000`, and explicit 4-variable environment (`PATH`, `LANG`, `HOME`, `TMPDIR`).
  - Does not rerun the 2 already-passing R404 positive methods or any historical cohorts.

---

### 3. Readable Coverage, Omissions & Scope Boundaries

- **Complete Readable Coverage**:
  - Full materialized `test_depinfo_retained_audit.py` (`10,803` bytes, SHA256 `c1b2e8c4caec6cb4c1a9c0d2ebcbb18e41e08048269886f19acc320ff6cd083b`), exact unified diff (`@@ -189,11 +189,20 @@`), complete guard parser API definitions (`Refusal`, `Unsupported`, `need`, `absolute`, `regular`, `make_words`, `depinfo_path`, `parse_depinfo`), complete guard import-time top-level statements, full R404 failure result/stdout (`76B`, `281696f9...`)/stderr (`2,222B`, `ace4ddbf...`), and the exact proposed one-negative-method operation JSON.
- **Explicit Omissions (Not Reread in Full Here)**:
  - Non-parser function bodies of `bounded_sdk_compile.py` (`79,818` bytes, `de3cb754...`, where parser/parse/output/sandbox/mount/worker were reviewed by `3b833` in R399 `9ed9c540...` while launch/retained-audit accounting/receipt findings remain `REVISE` and are authored separately) and external metadata files listed in local provenance pins (`MATERIALIZATION.json` `73265936...`, R405 `REPORT.md` `19b988e5...`, R405 `RESULT.json` `f26dee96...`, R399 `REPORT.md` `9ed9c540...`).
- **Scope Qualifications**:
  - Source hashes establish file identity only.
  - `prlimit` enforces per-process limits, not an aggregate process-tree bound.
  - All fixture paths are synthetic; no SDK, host filesystem, or native-root facts are inferred.
  - `output_check` and retained-lifecycle unit tests remain pending separately.
  - This review grants no native/SDK execution permission, no production guard installation, and no R2/C4/proof closure; Beta override removes human-only gate requirements while preserving all unchanged caps, refusals, one-shot constraints, and stopped Amendment 10.


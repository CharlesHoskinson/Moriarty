### Review Identity and Execution Posture

- **Task**: `R415` fresh non-author designated security source review and separate ordinary-operation classification
- **Reviewer Conversation ID**: `2246d9e6-0c61-4a77-b793-90541268a34c` (distinct from author `4e8031df-879c-41c4-95d2-622cd6b5df22`)
- **Primary / Requested / Returned Model Identity**: `primary_model: claude-opus-5-5` -> `requested_model: gemini-4-argon-high`, `returned_identity: gemini-4-argon-high`, `effort: high`, `fallback_status: standing_opus_quota_outage_fallback`
- **Tools / Commands / Imports / Mutations Executed**: `0`
- **Input Completeness**: All 3 current source modules (`bounded_sdk_compile.py` `0cb83528078378baf4f6beb48e5fd5e82d7037e82c6bb6440bc0bddb088a619e`, `test_depinfo_retained_audit.py` `cc0652a5038f02a79820d5d868d0aac116d7df046c64e93f34e7901e07d8267a`, `test_output_retained_accounting.py` `3cadfc79dffe6d9218258b98c8ae4a35af0e163390c39acb3cbb98cac71493c2`) and supporting JSON/proposal records are complete inline.

---

### Disposition A — `SOURCE_CORRECTNESS`

**Verdict**: **`APPROVED_SOURCE_WITH_LIMITS` (for the scoped `depinfo_path` hunk in `bounded_sdk_compile.py` and the 4-method parser regression suite in `test_depinfo_retained_audit.py`, with zero mandatory source fixes required)**

#### 1. Evaluation of `depinfo_path` Correction (`bounded_sdk_compile.py`)

1. **Empty and Trailing Segment Refusal (`'.'`, `'..'`, `''`, `'//'`, trailing `'/'`)**:
   - In `depinfo_path`:
     ```python
     need(type(word) is str and '\0' not in word and word!='' and '//' not in word and not word.endswith('/'),'non-normalized dep-info path')
     raw_parts=word[1:].split('/') if word.startswith('/') else word.split('/')
     need(all(p!='' for p in raw_parts) and raw_parts[-1] not in ('.','..'),'non-normalized dep-info path')
     ```
   - Rejects non-strings, NUL bytes, empty words, double slashes `'//'`, trailing slashes `'/'`, and any path whose final segment is `'.'` or `'..'` (including bare `'.'`, bare `'..'`, `'crates/..'`, and trailing `'lib.rs/.'` on a regular file).

2. **Fast-Path Gate (`'.'` and `'..'` Excluded from Unchecked `Path` Construction)**:
   - In `depinfo_path`:
     ```python
     if '.' not in raw_parts and '..' not in raw_parts:
         p=Path(word) if word.startswith('/') else worktree/word
         return str(absolute(str(p)))
     ```
   - Because `pathlib.Path` silently strips `'.'` components during object construction, gating the fast path on `'.' not in raw_parts and '..' not in raw_parts` is essential. Any word containing `'.'` or `'..'` is forced through the slow path (root anchoring, lexical stack walk, `ascended` directory verification, regular target verification, and `raw.resolve(strict=True)==norm`).

3. **Admitted Root Anchoring (`worktree` and `roots`)**:
   - In `depinfo_path`:
     ```python
     if word.startswith('/'):
         candidates=[b for b in (worktree,*(absolute(str(r)) for r in roots)) if word.startswith(str(b)+'/')]
         need(bool(candidates),'non-normalized dep-info path')
         base=max(candidates,key=lambda b:len(str(b)))
         rel_parts=word[len(str(base))+1:].split('/')
     else:
         base=worktree;rel_parts=raw_parts
     ```
   - Relative paths containing `'.'` or `'..'` anchor to `base = worktree`. Absolute paths containing `'.'` or `'..'` must be strict descendants (`word.startswith(str(b)+'/')`) of `worktree` or an admitted root in `roots`, selecting the longest matching root `base`. Unlisted absolute paths with interior `'.'` or `'..'` fail closed with `'non-normalized dep-info path'`.

4. **Lexical Stack Walk, Unchanged Leading `'./'` Refusal, and Root-Boundary Containment**:
   - In `depinfo_path`:
     ```python
     stack=[];ascended=[]
     for part in rel_parts:
         if part in ('.','..'):
             need(bool(stack),'non-normalized dep-info path')
             ascended.append(base.joinpath(*stack))
             if part=='..':stack.pop()
         else:stack.append(part)
     need(bool(stack),'non-normalized dep-info path')
     ```
   - Because `need(bool(stack),'non-normalized dep-info path')` executes whenever `part in ('.','..')`:
     - Leading relative `'./...'` (`stack == []` at index 0) is refused with `'non-normalized dep-info path'`.
     - Absolute root-boundary `'<base>/./...'` (`stack == []` at `rel_parts[0]`) is refused with `'non-normalized dep-info path'`.
     - Post-collapse `'crates/.././...'` (`stack` popped to `[]` by `'..'`, then encountering `'.'`) is refused with `'non-normalized dep-info path'`.
     - Cross-crate traversal (`f'{pkg_a}/src/../../pkg-b-1.0.0/...'`) underflows `stack` at the second `'..'` and is refused with `'non-normalized dep-info path'`.
     - Valid interior `'.'` after descending at least one subdirectory within `base` (`bool(stack)` is `True`) records `base.joinpath(*stack)` in `ascended` without popping `stack`.

5. **Missing / Regular-File / Symlink Parent Identity, Raw Resolution, and Regular Target Verification**:
   - In `depinfo_path`:
     ```python
     norm=base.joinpath(*stack);raw=base/('/'.join(rel_parts))
     for d in ascended:
         need(not any(x.is_symlink() for x in [d,*d.parents]) and d.is_dir() and d.resolve(strict=True)==d,'dep-info parent directory identity: '+str(d))
     need(norm.exists(),'dep-info target missing: '+str(norm));regular(norm);need(raw.resolve(strict=True)==norm,'dep-info path resolution mismatch: '+word)
     return str(norm)
     ```
   - Because `pathlib.Path` collapses `'.'` when constructing `raw = base/('/'.join(rel_parts))`, `raw.resolve(strict=True)` alone would not verify an intermediate component before `'.'` if no `'..'` followed. Appending `base.joinpath(*stack)` to `ascended` when `part in ('.','..')` closes that gap: every directory immediately preceding `'.'` or `'..'` is checked by `for d in ascended:` to ensure `[d, *d.parents]` contains no symlinks, `d.is_dir()` is `True` (rejecting nonexistent directories and regular files traversed via `file.rs/./...`), and `d.resolve(strict=True)==d`.
   - `need(norm.exists(),'dep-info target missing: '+str(norm));regular(norm)` then verifies that the normalized target `norm` exists, has no symlink in `[norm, *norm.parents]`, and is a regular file (`stat.S_ISREG(s.st_mode)`).

#### 2. Downstream Integration (`parse_depinfo`, `output_check`, `worker`)

- **Make Continuations, `# env-dep:` Comments, Escaping, and Set-Per-Rule Semantics**:
  - In `make_words`:
    ```python
    text=text.replace('\\\n','');out=[];word='';esc=False
    ```
    and
    ```python
    need(not esc,'trailing dep-info escape')
    ```
  - In `parse_depinfo`:
    ```python
    for line in text.replace('\\\n','').splitlines():
        if not line or line.startswith('# env-dep:'):continue
    ```
    and
    ```python
    def paths(part):
        return {depinfo_path(word,worktree,roots) for word in make_words(part)}
    rows.append((paths(line[:delimiter]),paths(line[delimiter+1:])))
    ```
  - Comment lines starting with `# env-dep:` (such as `# env-dep:CARGO_PKG_REPOSITORY=https://github.com/bitvecto-rs/bitvec` in `bitvec-570e5a3d029cac83.d`) are skipped before rule parsing. Both rule targets (`line[:delimiter]`) and prerequisites (`line[delimiter+1:]`), including `-MP` empty-prerequisite target lines (`<path>:`), are normalized into sets of canonical path strings via `depinfo_path`.
- **Downstream Manifest, `frozen_source`, `includes`, and `host_units` Binding in `output_check` / `worker`**:
  - In `output_check`:
    ```python
    dep_roots=[source_home/g['relative_root'] for g in c['git_checkouts']]+[source_home/r['source_relative'] for r in c['registry_roots']]+[root,*(Path(t) for t in c['readonly_tools']),Path('/usr')]
    ```
    Because `dep_roots` lists individual git checkout roots (`source_home/g['relative_root']`) and individual registry package roots (`source_home/r['source_relative']`) rather than `source_home` as a whole, `depinfo_path` confines every `source_home` `'.'` or `'..'` walk to its single crate or checkout root.
  - In `output_check`:
    ```python
    for x in all_paths:
        if x==str(worktree) or x.startswith(str(worktree)+'/'):
            need(str(Path(x).relative_to(worktree)) in worktree_files,'unbound worktree dep-info input: '+x)
        elif x==str(source_home) or x.startswith(str(source_home)+'/'):
            rel=str(Path(x).relative_to(source_home))
            need(rel in frozen and not frozen[rel].startswith('git-symlink:'),'unbound source-home dep-info input: '+x)
    inputs=set().union(*(values for _,values in deps[str(p)]))
    sdk=any('/midnight-' in x or '/midnight-ledger-' in x for x in inputs)
    if sdk:
        for x in inputs:
            if x.startswith(c['paths']['source_home']+'/') and not x.endswith('.rs'):need(x in included,'unbound SDK non-Rust dep-info input')
    ```
  - Returning canonical `str(norm)` from `depinfo_path` preserves exact relative path lookup against `worktree_files` (`source_manifest['files']`), `frozen` (`D_FREEZE.json`, rejecting `git-symlink:` entries), `included` (`c['includes']` for non-`.rs` SDK inputs), and `E_RESOLVED.json` `host_units` prefix matching (`any(x.startswith(u['root']+'/') for x in inputs)`).
  - In `worker`:
    ```python
    elif phase=='F':output_check(c,'F_RETAINED_QUARANTINED_OUTPUTS.json' if retained_audit else 'F_QUARANTINED_OUTPUTS.json')
    ```

#### 3. Review of `test_depinfo_retained_audit.py` (4 Test Methods and 7 Added Negative Tuples)

- **2 Preserved Positive Methods**: `test_parse_depinfo_resolves_relative_and_crate_contained_parent_segments` and `test_parse_depinfo_preserves_make_continuations_and_escaped_filename_tokens` are byte-identical to prior bodies and test relative/crate-contained `'..'`, `\\\n` continuations, `# env-dep:`, and escaped ` `, `#`, `:`, `\` characters.
- **1 Added Positive Method**: `test_parse_depinfo_resolves_observed_registry_and_relative_interior_dot_segments` creates real temporary files under `self._tmp` and exercises `runner.parse_depinfo` with `roots=(str(self.ledger_root), str(konst_root), str(typewit_root))` on the three literal `/src/./...` patterns observed in `ROOT-LITERAL-DEPINFO-PATH-OBSERVATION.json` (both as RHS dependencies and as LHS `-MP` empty-dependency rules), `# env-dep:CARGO_PKG_REPOSITORY=https://github.com/bitvecto-rs/bitvec`, relative interior `'crates/mori-backend/src/./lib.rs'`, mixed relative `'crates/mori-backend/src/./sherlock_machine/../../../../models/sherlock/machine-circuit/expected.json'`, and mixed absolute `f'{self.ledger_root}/zswap/src/./../static/spend.verifier.sha256'`.
- **1 Negative Method (`test_parse_depinfo_rejects_root_escapes_invalid_parents_and_cross_crate_traversals`)**: Retains all 16 existing `refusal_cases` (including unchanged `'./crates/mori-backend/src/lib.rs'` -> `'non-normalized dep-info path'`) and adds 7 negative tuples whose expected substrings match exact `depinfo_path` failure precedence:
  1. `f'{self.out_rlib}: {pkg_b}/./src/file.rs\n'` -> fails at `need(bool(stack),'non-normalized dep-info path')` (`stack == []` at crate root boundary).
  2. `f'{self.out_rlib}: crates/.././crates/mori-backend/src/lib.rs\n'` -> `'..'` pops `stack` to `[]`, next `'.'` fails at `need(bool(stack),'non-normalized dep-info path')`.
  3. `f'{self.out_rlib}: crates/mori-backend/src/lib.rs/.\n'` -> fails at `need(all(p!='' for p in raw_parts) and raw_parts[-1] not in ('.','..'),'non-normalized dep-info path')`.
  4. `f'{self.out_rlib}: {self.base}/unlisted/./escape.json\n'` -> fails at `need(bool(candidates),'non-normalized dep-info path')`.
  5. `f'{self.out_rlib}: nonexistent_dir/./expected.json\n'` -> fails in `for d in ascended:` (`d.is_dir()` is `False` before `norm.exists()`) with `f'dep-info parent directory identity: {self.worktree / "nonexistent_dir"}'`.
  6. `f'{self.out_rlib}: symlink_dir/./mori-backend/src/lib.rs\n'` -> fails in `for d in ascended:` (`x.is_symlink()` is `True`) with `f'dep-info parent directory identity: {symlink_parent}'`.
  7. `f'{self.out_rlib}: crates/mori-backend/src/lib.rs/./../../../../models/sherlock/machine-circuit/expected.json\n'` -> fails in `for d in ascended:` (`self.lib_rs.is_dir()` is `False`) with `f'dep-info parent directory identity: {self.lib_rs}'`.

#### 4. Non-Adoption of Unverified Author Inferences and Mandatory/Optional Findings

- **Qualification on Author Diagnosis**: `ROOT-LITERAL-DEPINFO-PATH-OBSERVATION.json` establishes only that `konst-16861531b8e91a42.d` and `typewit-f3a70ec9ea47c182.d` contain literal `/src/./` spans and `bitvec-570e5a3d029cac83.d` contains an `env-dep:` span. This review does **not** assume the author's inference that rustc `#[path = "./..."]` attributes produced those spans (no crate Rust sources are inline), nor does it assume which `.d` file or token caused the first refusal during unordered `root.rglob('*.d')`.
- **Mandatory Fixes / Counterexamples**: **None.**
- **Optional Non-Blocking Observation**: `test_output_retained_accounting.py` is unchanged in this candidate and its synthetic `.d` fixture in `setUp` exercises `output_check` with `'..'` segments (`rel_dotdot` and `abs_dotdot`) rather than `'/./'`; direct `'/./'` coverage is provided by `test_depinfo_retained_audit.py`.

---

### Disposition B — `ORDINARY_OPERATION`

**Verdict**: **`ELIGIBLE` (for the exact 2 proposed SDK-free unit-test commands in `ORDINARY-TWO-COMMAND-OPERATION.proposed.json`, subject to the explicit conditions below)**

#### 1. Command-by-Command Classification

1. **`changed-parser-four`**:
   - **Selector & Count**: `test_depinfo_retained_audit.DepinfoNormalizationTests`, `assert suite.countTestCases() == 4` (matches the 4 `test_*` methods in `test_depinfo_retained_audit.py`: the 2 preserved positive methods re-exercised against modified `depinfo_path`, 1 new positive method for interior `'.'` and mixed `'./..'`, and 1 expanded 23-tuple + 3-char negative method).
   - **Execution Path & Effects**: Imports `bounded_sdk_compile as runner` in `/usr/bin/python3 -I -B`, creates a `tempfile.TemporaryDirectory()` under `TMPDIR` (the isolated prospective suite directory), writes small synthetic files and symlinks, invokes only `runner.depinfo_path`, `runner.make_words`, and `runner.parse_depinfo`, and removes the temporary directory in `tearDown`. No subprocesses, `bwrap`, `cargo`, cgroups, mounts, sockets, or network calls occur.
2. **`dependent-output-one`**:
   - **Selector & Count**: `test_output_retained_accounting.OutputAndRetainedAccountingTests.test_output_check_persists_receipt_and_enforces_manifest_freeze_include_and_host_unit`, `assert suite.countTestCases() == 1` (selects only the single named `output_check`/`worker` method out of the 4 methods in `OutputAndRetainedAccountingTests`; the other 3 accounting/sandbox/launch/retained-audit methods are excluded).
   - **Execution Path & Effects**: `setUp` creates a `tempfile.TemporaryDirectory()` under `TMPDIR` and writes synthetic directory trees, manifest JSONs, a dummy `b'\x7fELF-build'` file (which is only `stat`'ed and SHA-256 hashed by `walk_files`/`digest`, never executed), and a synthetic `.d` file. The test method calls `runner.output_check(self.c)` and `runner.worker(self.c, 'F', retained_audit=True)` and checks 4 `Refusal` cases against `source_manifest`, `frozen_source`, `includes`, and `E_RESOLVED.json`, then cleans up in `tearDown`. No live `launch`, `bwrap`, `cargo`, cgroup manipulation, or native ELF execution is performed.

#### 2. Explicit Conditions for `ELIGIBLE` Execution

1. **Exact Staging Verification**: Before running either command, Root must stage only the 3 pinned afterimages into `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/415-native06-dot-segment-guard-test-candidate/prospective-isolated-parser-output-suite/` and verify exact byte counts and SHA-256 digests (`bounded_sdk_compile.py`: `81707` bytes, `0cb83528078378baf4f6beb48e5fd5e82d7037e82c6bb6440bc0bddb088a619e`; `test_depinfo_retained_audit.py`: `15027` bytes, `cc0652a5038f02a79820d5d868d0aac116d7df046c64e93f34e7901e07d8267a`; `test_output_retained_accounting.py`: `20697` bytes, `3cadfc79dffe6d9218258b98c8ae4a35af0e163390c39acb3cbb98cac71493c2`).
2. **Exact Process, Environment, and Resource Bounds**: Execute as unprivileged `uid=1000`, `gid=1000` with `cwd`, `HOME`, and `TMPDIR` set to the prospective suite directory, environment restricted to declared `PATH=/usr/bin:/bin`, `LANG=C.UTF-8`, `HOME`, and `TMPDIR`, wrapped in `/usr/bin/timeout --signal=KILL 30s` and `/usr/bin/prlimit --cpu=25:25 --as=1073741824:1073741824 --fsize=1048576:1048576 --core=0:0`, `combined_capture_cap_bytes=131072`, `fail_stop=true`, and no auto-retry.
3. **Scope Boundary**: This classification covers only the 2 synthetic SDK-free unit-test commands above. It does not classify or authorize the remaining 3 methods of `test_output_retained_accounting.py`, any retained Phase F audit run, Cargo/SDK invocation, native ELF execution, proof generation, or privilege/mount/cgroup operations. Root is sole executor/adopter, and actual test execution results remain subject to review.

---

### Disposition C — `COVERAGE/GAPS`

1. **Supplied Source Scope and Limits**:
   - Complete current bytes of `bounded_sdk_compile.py` (1153 lines, 81707 bytes), `test_depinfo_retained_audit.py` (381 lines, 15027 bytes), and `test_output_retained_accounting.py` (482 lines, 20697 bytes) were reviewed inline.
   - Material review scope is the single `depinfo_path` owner hunk (`6fcfaffd...` -> `0cb83528...`), its direct integration with `make_words`, `parse_depinfo`, `artifact_dependencies`, `output_check`, and `worker`, the 4 parser unit-test methods in `test_depinfo_retained_audit.py`, and the 1 dependent `output_check` unit-test method in `test_output_retained_accounting.py`.
   - This review does **not** certify the whole SDK, full config/manifest/archive inventories, native compilation outputs, host binaries, proofs, or financial semantics.

2. **Verbatim Source Anchor Summary (for Mechanical Fidelity Check)**:
   - **`depinfo_path` dot/trailing segment and fast-path clauses**:
     - `need(all(p!='' for p in raw_parts) and raw_parts[-1] not in ('.','..'),'non-normalized dep-info path')`
     - `if '.' not in raw_parts and '..' not in raw_parts:`
   - **`depinfo_path` stack-handling clause**:
     - `if part in ('.','..'):`
     - `need(bool(stack),'non-normalized dep-info path')`
     - `ascended.append(base.joinpath(*stack))`
     - `if part=='..':stack.pop()`
   - **`depinfo_path` directory identity and resolution clauses**:
     - `norm=base.joinpath(*stack);raw=base/('/'.join(rel_parts))`
     - `for d in ascended:`
     - `need(not any(x.is_symlink() for x in [d,*d.parents]) and d.is_dir() and d.resolve(strict=True)==d,'dep-info parent directory identity: '+str(d))`
     - `need(norm.exists(),'dep-info target missing: '+str(norm));regular(norm);need(raw.resolve(strict=True)==norm,'dep-info path resolution mismatch: '+word)`
   - **`parse_depinfo` comment-skip and rule-set clauses**:
     - `if not line or line.startswith('# env-dep:'):continue`
     - `return {depinfo_path(word,worktree,roots) for word in make_words(part)}`
   - **`output_check` and `worker` integration clauses**:
     - `regular(p);deps[str(p)]=parse_depinfo(p.read_text(),worktree,dep_roots)`
     - `need(str(Path(x).relative_to(worktree)) in worktree_files,'unbound worktree dep-info input: '+x)`
     - `need(rel in frozen and not frozen[rel].startswith('git-symlink:'),'unbound source-home dep-info input: '+x)`
     - `if x.startswith(c['paths']['source_home']+'/') and not x.endswith('.rs'):need(x in included,'unbound SDK non-Rust dep-info input')`
     - `elif phase=='F':output_check(c,'F_RETAINED_QUARANTINED_OUTPUTS.json' if retained_audit else 'F_QUARANTINED_OUTPUTS.json')`

3. **Incomplete Dependencies and Current Terminal State**:
   - The candidate guard (`0cb83528...`) and parser test suite (`cc0652a5...`) are currently unexecuted.
   - The prior retained audit attempt failed (`exit: 2`, `elapsed_seconds: 15.664219678998052`, `worker_stderr: "TERMINAL_REFUSAL: non-normalized dep-info path\n"`), leaving 4 new audit files in `packet` (`USED_LAUNCH_<nonce>`, `F_retained_audit.stdout`, `F_retained_audit.stderr`, `TERMINAL_FAILURE_RETAINED_AUDIT.json`). Because `retained_f_audit` enforces fresh-output exclusivity (`need(not (packet/name).exists(),'retained audit output already exists: '+name)`), the consumed retained audit cannot be rerun unchanged.
   - `native_outputs_accepted` remains `false`; cleanup remains owed; Amendment 10 / T3 remain terminal; no proof, verification, or target acceptance exists.

4. **Precise Next Mandatory Action**:
   - Obtain the required parallel non-security source review if not already recorded, then have Root stage the 3 pinned afterimages into `prospective-isolated-parser-output-suite` and execute the 2 classified ordinary unit-test commands (`changed-parser-four` and `dependent-output-one`) under the exact bounds in `ORDINARY-TWO-COMMAND-OPERATION.proposed.json`, followed by inspection of the captured test receipts and separate review of any distinct successor route and resource/exclusivity accounting.


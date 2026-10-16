# Security review: Sherlock streaming export (exact source, actual 8-test result, proposed production compile)

Reviewer: Opus 5.5, high, designated security owner. This is a static review of the pasted bytes. I ran nothing and hashed nothing on disk.

## Verdict 1: source contract and public exports — **APPROVE_WITH_SCOPE**

### Production delta
The only production change is `candidate/crates/mori-core/src/sherlock/mod.rs`.

- **Matches BASE plus the patch.** The projected file equals `BASE-mod.rs` plus the three hunks in `future-wiring.patch`. I checked this line by line. By my count the additions are 22 + 52 + 42 = 116 bytes, and 1779 + 116 = 1895, which matches the stated size.
- **No other production change.** `lib.rs` already has `pub mod sherlock;` and is unchanged.
- **Closure is consistent.** The 26-file `compile_closure` matches every non-`cfg(test)` module reachable from `lib.rs`: 9 crate-root files, 8 `sherlock` files including the two `#[path]` modules, `source_sponsor`, and 8 `stage` files.

### `reference_stream.rs` (`9031…f923`)
These bytes are the ones the 8-test capture compiled.

- **Terminal precedence.** A terminal record returns `Terminal` before the observer's `Break` is read.
- **Observer stop.** `Observer` is returned only after a nonterminal event.
- **Zero budget.** A zero budget returns `Budget` without calling `step()` or the callback, including on an already-terminal machine.
- **Already terminal.** When `step()` returns `None`, the result is `Terminal` with zero events.
- **No history retained.** Each record is dropped before the next step.
- **Callback cannot reach the machine.** `&mut self` is held for the whole call, so the closure cannot touch the `Machine`.
- **Fuel and publication unchanged.** Fuel, charges and publication come entirely from the unchanged `step()`.

### Public export
`Machine::visit_steps` becomes a public inherent method on the already-public `mori_core::sherlock::Machine`. `VisitResult` and `VisitStop` are re-exported so its signature can be named.

- **No new authority.** `step`, `run_steps` and `run` are already public, so this adds no capability.
- **No name collisions.** Neither type collides with `reference_types::*` or with the crate-root `pub use` globs.
- **H1 still applies.** `max_events` only groups calls. A single step still clones two complete snapshots without a bound.

### Scope conditions and findings

| # | Finding | Severity / disposition |
|---|---|---|
| S1 | The new public `visit_steps` inherits H1. No CLI, LSP, MCP, `/1`/`/2` source-adapter or other untrusted path may call it. `bounded_scalar_source` still uses `run_steps`. Any future consumer needs its own security review. | Scope condition |
| S2 | If the observer panics, `step()` has already committed and the record unwinds. The machine is left in the delivered post-state with no `VisitResult`. The state is consistent but untested. | Info; document later |
| S3 | `VisitStop` and `VisitResult` are exhaustive public types. Adding a variant later is a breaking API change. | Nonsecurity note |
| S4 | The header of `reference_stream_tests.rs` still says "UNRUN pending preflight". The bytes are frozen after capture, so leave it. The receipt is the record. | Do not edit |
| S5 | Coverage is normal-only: N01, N02, N05, zero fuel, and budget/observer cuts. Rule-82 `Rejected`, the stack-cap `Rejected(4)`, and mid-run `Exhausted` are untested through `visit_steps`. | Retained gap |

## Verdict 2: actual 8-test result — **APPROVE_WITH_SCOPE**

I audited the eight attribution conditions from my prior report against the pasted receipt.

| # | Condition | Observed |
|---|---|---|
| 1 | Outer exit 0, no traceback | `ATTRIBUTION.json`: exit 0, traceback false. This is operator-observed. |
| 2 | Manifest `sha256` and `sha256_after` | Both `73fb5864…a4b1` ✓ |
| 3 | `reviewed_candidate == reviewed_candidate_after == manifest` | All 28 keys match, including `run_streaming.py 7b4a…f410c` ✓ |
| 4 | Every snapshot member's three digests equal `candidate/P` | All 24 members match ✓ |
| 5 | Both postflight steps `ok` | ✓ |
| 6 | Terminal state `success` | ✓ |
| 7 | Exactly the 8 CONFIG test names | The set matches, and stdout says `8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out` ✓ |
| 8 | `result.json` binds the receipt | `1721fa70…e6b8` equals the manifest row ✓ |

Further checks:
- The compiled dep-info inputs equal the 8-file `COMPILED-CLOSURE` ✓.
- The `compile-tests` stderr is empty, so there were no warnings ✓.
- The postflight-exception defect I disclosed earlier was not triggered: the after-fields are present.

I independently re-checked the test expectations against the frozen JSON:
- **N02:** the 9 states and rules `117,135,102,149,102,153,223,80` match.
- **N05:** all 28 rules and all seven selected cuts match, including control, environment, top-first frames and fuel.

These expectations are independent of `step()` and `run_steps()`.

Scope limits:
- This is standalone `#[path]` std-host evidence only.
- It is not production module compilation.
- It does not compile the tests through the production `mod.rs` `cfg(test)` wiring. That would need test modules that are absent from this snapshot.
- I verified the pasted texts' internal consistency, not the on-disk bytes.

## Production compile disposition — **SPECIFIED-ONLY. Not approved for execution.**

The driver below is my authored security guard. Before any run:

1. Root writes it byte-exact to `R/run_core.py`.
2. Root creates a new `R/CANDIDATE.json`: the 53 rows of the manifest you supplied (`f8606b02…6fe46`) plus one `run_core.py` row.
3. That new manifest digest needs a fresh exact-candidate Opus confirmation, plus Muse nonsecurity confirmation under the current routing.

After that, one attempt only: `/usr/bin/python3 -I -B R/run_core.py <approved CANDIDATE.json sha256>`. There is no retry. An independent result audit is required after the attempt.

Two binding points the confirmation reviewer must check:
- **Helper bytes.** `HELPER_SHA = 6fe358db…fd51` must equal the driver digest recorded for the bounded-source-01 run. You assert this; I have not seen it.
- **Prior dep-info.** The driver reads the actual 25 inputs from `bounded-source-01/result.json` and records that file's digest. The auditor must match the digest to the previously audited one.

```python
"""One production Core rlib compile for the projected Sherlock streaming export.

Run once, only after a fresh exact-candidate approval of the manifest pinning this file:
    /usr/bin/python3 -I -B R/run_core.py <approved CANDIDATE.json sha256>

Exactly two children: `rustc -vV`, then one rlib compile of the projected candidate's
real lib.rs in the existing dependency graph. Helpers come from the verified bytes of
bounded_source_driver.py by AST extraction; its top level never runs. No cfg(test),
test execution, Cargo, optimization, retry or fallback. Host diagnostic only:
native authority 0, no acceptance, no memory/disk bound. H1 is unchanged.
"""
from pathlib import Path
import ast, builtins, datetime, hashlib, json, os, re, shutil, signal, stat, subprocess, sys, time, types

R = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/streaming-02-production-preparation')
A = R / 'attempt-core-01'
S = A / 'source'
CAND = R / 'candidate'
MANIFEST = R / 'CANDIDATE.json'
SELF = 'run_core.py'
RUST = Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc')
RUST_SHA = 'f3834d26669b03f6855fa54bd2381443123e860167bc0c7a8dd137e5cf6e5e4f'
RUST_COMMIT = 'b940084d7eb6a299eb4bfeb8e34901bc051e7ac4'
PYTHON = Path('/usr/bin/python3').resolve(strict=True)
DEPS = Path('/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target/debug/deps')
DM = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-reference-20261015/compiler-source-projection-stripped-local-check/dependency-manifest.json')
DM_SHA = 'ce37c42b96cc071280db3c361ad593172aab6f1716419d7cdc9010120ffb2f36'
PRIOR_CORE = R.parent / 'bounded-source-01'
HELPER_SOURCE = 'bounded_source_driver.py'
HELPER_SHA = '6fe358dbfbed5256e4851d67f09fae93804181c53aa83bb3bdc02667b215fd51'
HELPERS = ('sha', 'record', 'inventory', 'write_json', 'child', 'source_records', 'depinfo')
CHILDREN = ('00-rustc-version', '01-compile-actual-core')
LIB = 'crates/mori-core/src/lib.rs'
MOD = 'crates/mori-core/src/sherlock/mod.rs'
STREAM = 'crates/mori-core/src/sherlock/reference_stream.rs'
STREAM_TESTS = 'crates/mori-core/src/sherlock/reference_stream_tests.rs'
EXTERNS = {'mori_lexer', 'serde', 'serde_json', 'thiserror', 'sha2', 'k256'}
PROC_MACROS = {'serde_derive', 'thiserror_impl'}
PINNED = {
    'BASE-mod.rs': '340782cc3bc0d7d95b71377df3f338f47e7a4caf00608f628c45cd0106f88887',
    'CONFIG.json': '919fbe6e37d2b32a4c6722aa7097853983393d1141d04ffa438742b273e2bd83',
    'future-wiring.patch': '0fcab12809c47dd1d8435996065abee1d90e0a05f520ffecb9b481c4bc1a3eaa',
    'dependency-manifest.json': DM_SHA,
    HELPER_SOURCE: HELPER_SHA,
    'candidate/' + MOD: 'bb0752abf4065903fcd74cccd7c62783b88a17fc706b72c63e6b9e77e0f8aa39',
    'candidate/' + STREAM: '90316dea89d6528e17ffdb991d8cf4b226e12714f3a1ed4db06de8c72821f923',
    'candidate/' + STREAM_TESTS: '4e8b66bec3ef365b2fc73c7c65f8d677604cbaac7f9ceb9ae1677f0e9828cdfc',
    'prior-streaming-evidence/CANDIDATE.json': '73fb5864bbb05e7e0d8d1949fb7bc7250c3e1c02d7bb7b3abffbe118cb94a4b1',
    'prior-streaming-evidence/attempt-01/receipt.json': '1721fa709d6a8baa21320bdef924481a7b1de49ce7e52b3e0ca71199d6bfb6e8',
}
TOP = ('BASE-mod.rs', 'CONFIG.json', HELPER_SOURCE, 'dependency-manifest.json', 'future-wiring.patch', SELF)
PRIOR_EVIDENCE = tuple('prior-streaming-evidence/' + p for p in (
    'ATTRIBUTION.json', 'CANDIDATE.json', 'COMPILED-CLOSURE.json', 'CONFIG.json',
    'attempt-01/logs/compile-tests.stderr', 'attempt-01/logs/run-tests.stderr',
    'attempt-01/logs/run-tests.stdout', 'attempt-01/receipt.json', 'attempt-01/result.json',
    'preflight-muse-02/REPORT.md', 'preflight-opus-02/REPORT.md', 'run_streaming.py'))
WIRING = (
    ('mod reference;\npub mod reference_display;\n',
     'mod reference;\nmod reference_stream;\npub mod reference_display;\n'),
    ('pub use reference::Machine;\n',
     'pub use reference::Machine;\npub use reference_stream::{VisitResult, VisitStop};\n'),
)
WIRING_TAIL = '\n#[cfg(test)]\nmod reference_stream_tests;\n'
PATCH_ADDED = ['mod reference_stream;', 'pub use reference_stream::{VisitResult, VisitStop};',
               '', '#[cfg(test)]', 'mod reference_stream_tests;']
SAFE = r'[A-Za-z0-9][A-Za-z0-9._-]*(?:/[A-Za-z0-9][A-Za-z0-9._-]*)*'

result = dict(scope='one production mori_core rlib compile of the projected streaming export only',
              terminal_state='running', steps=[], native_authority=0, acceptance='none',
              limits=dict(child_seconds=30, children=2, host_memory_cap='none', host_disk_cap='none'),
              not_run=['cfg(test) compile', 'test execution', 'Cargo', 'K', 'Quint', 'Lean', 'Clean',
                       'circuits', 'crypto sign/verify', 'SDK/native/proof/ledger/target'],
              provenance='local snapshot and selected tool/dependency inventories only; '
                         'transitive source authenticity unestablished; sysroot/linker/libc/'
                         'Python stdlib/kernel trusted locally')
h = None


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def member(path):
    require(isinstance(path, str) and re.fullmatch(SAFE, path), 'unsafe manifest path')
    full = R / path
    require(not full.is_symlink() and full.resolve(strict=True) == full and full.is_file(),
            'not a plain member: ' + path)
    return full


def load_helpers(raw, snapshot_rows):
    # Compile only the named top-level functions from the verified bytes; the original
    # module body, which would launch the bounded-source-01 jobs, never runs.
    text = raw.decode()
    tree = ast.parse(text, filename=str(R / HELPER_SOURCE), mode='exec')
    definitions = {}
    for node in tree.body:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            require(node.name not in definitions, 'duplicate helper definition: ' + node.name)
            definitions[node.name] = node
    selected = []
    for name in HELPERS:
        node = definitions.get(name)
        require(isinstance(node, ast.FunctionDef) and not node.decorator_list
                and not node.args.defaults and not node.args.kw_defaults and not node.type_params,
                'unexpected helper shape: ' + name)
        selected.append(node)
    namespace = dict(__name__='bounded_source_helpers', __builtins__=builtins,
                     Path=Path, datetime=datetime, hashlib=hashlib, json=json, os=os, re=re,
                     shutil=shutil, signal=signal, stat=stat, subprocess=subprocess, time=time,
                     A=A, S=S, result=result, manifest=snapshot_rows)
    exec(compile(ast.Module(body=selected, type_ignores=[]), str(R / HELPER_SOURCE), 'exec',
                 dont_inherit=True), namespace)
    result['helpers'] = {node.name: dict(lines=[node.lineno, node.end_lineno],
                                         sha256=digest(ast.get_source_segment(text, node).encode()))
                         for node in selected}
    return types.SimpleNamespace(**{name: namespace[name] for name in HELPERS})


def toolchain(host):
    # The same toolchain, linker and sysroot set as bounded-source-01.
    sysroot = RUST.parent.parent
    target = sysroot / 'lib/rustlib' / host
    tools = {RUST, PYTHON, Path(shutil.which('cc', path='/usr/bin:/bin')).resolve(strict=True),
             Path(shutil.which('ld', path='/usr/bin:/bin')).resolve(strict=True), DM,
             Path('/usr/bin/env').resolve(strict=True)}
    tools.update((sysroot / 'lib').glob('librustc_driver-*.so'))
    tools.update(p.resolve(strict=True) for p in (target / 'bin').rglob('*')
                 if p.is_file() and ('rust-lld' in str(p) or 'gcc-ld' in str(p)))
    tools.update(p.resolve(strict=True) for p in (target / 'lib').rglob('*') if p.is_file())
    return sorted(tools)


def run(expected, state):
    global h
    rows = json.loads(state['manifest_raw'])
    require(isinstance(rows, list) and all(isinstance(row, dict) and set(row) == {'path', 'sha256', 'bytes'}
                                           for row in rows), 'manifest shape')
    paths = [row['path'] for row in rows]
    require(len(paths) == len(set(paths)), 'duplicate manifest path')
    # Read each pinned member once; every later use takes these verified bytes.
    contents = {}
    for row in rows:
        raw = member(row['path']).read_bytes()
        require(digest(raw) == row['sha256'] and len(raw) == row['bytes'], 'pinned member differs: ' + row['path'])
        contents[row['path']] = raw
    pinned = state['pinned'] = {row['path']: row['sha256'] for row in rows}
    for path, value in PINNED.items():
        require(pinned.get(path) == value, 'reviewed digest differs: ' + path)
    require(Path(__file__).resolve(strict=True) == R / SELF
            and digest((R / SELF).read_bytes()) == pinned.get(SELF), 'running driver is not the pinned run_core.py')
    result['driver'] = dict(path=str(R / SELF), sha256=pinned[SELF])

    config = json.loads(contents['CONFIG.json'])
    members, closure = config['snapshot_members'], config['compile_closure']
    require(len(members) == len(set(members)) == 36 and len(closure) == len(set(closure)) == 26, 'CONFIG extents')
    require(set(paths) == set(TOP) | set(PRIOR_EVIDENCE) | {'candidate/' + m for m in members},
            'manifest members differ from the reviewed set')
    require(set(closure) <= set(members) and {LIB, MOD, STREAM} <= set(closure)
            and not any(p.endswith('_tests.rs') for p in closure), 'compile closure shape')
    require(config['dependency_manifest_original'] == str(DM), 'dependency manifest origin')

    # The projected mod.rs must be exactly BASE plus the reviewed wiring patch.
    projected = contents['BASE-mod.rs'].decode()
    for old, new in WIRING:
        require(projected.count(old) == 1, 'BASE wiring anchor')
        projected = projected.replace(old, new)
    require(projected.endswith('\nmod layout_a;\n'), 'BASE tail')
    require(contents['candidate/' + MOD] == (projected + WIRING_TAIL).encode(),
            'projected mod.rs is not BASE plus the reviewed wiring')
    added = [line[1:] for line in contents['future-wiring.patch'].decode().splitlines()
             if line.startswith('+') and not line.startswith('+++')]
    require(added == PATCH_ADDED, 'future-wiring.patch differs from the projected wiring')

    snapshot_rows = [dict(path=m, sha256=pinned['candidate/' + m], bytes=len(contents['candidate/' + m]))
                     for m in sorted(members)]
    state['snapshot'] = {row['path']: row['sha256'] for row in snapshot_rows}
    h = load_helpers(contents[HELPER_SOURCE], snapshot_rows)

    # The 26-file closure is the actual prior 25 Core dep-info inputs plus reference_stream.rs.
    prior_raw = (PRIOR_CORE / 'result.json').read_bytes()
    result['prior_core_result'] = dict(path=str(PRIOR_CORE / 'result.json'), bytes=len(prior_raw),
                                       sha256=digest(prior_raw))
    prior = json.loads(prior_raw)
    prefix = str(PRIOR_CORE / 'source') + '/'
    prior_inputs = prior['depinfo']['01-core']['inputs']
    require(prior['terminal_state'] == 'success' and len(prior_inputs) == 25
            and all(i.startswith(prefix) for i in prior_inputs), 'prior bounded-source-01 Core dep-info')
    require(sorted(i[len(prefix):] for i in prior_inputs) == sorted(set(closure) - {STREAM}),
            'closure is not the prior 25 inputs plus reference_stream.rs')

    require(re.fullmatch(r'[A-Za-z0-9/._-]+', str(S)), 'snapshot path')
    for row in snapshot_rows:
        dest = S / row['path']
        dest.parent.mkdir(parents=True, exist_ok=True)
        with dest.open('xb') as stream:
            stream.write(contents['candidate/' + row['path']])
        dest.chmod(0o444)
    h.write_json(A / 'manifest.json', snapshot_rows)
    for directory in sorted((p for p in S.rglob('*') if p.is_dir()), reverse=True) + [S]:
        directory.chmod(0o555)
    result['candidate_before'] = h.source_records(CAND)
    result['snapshot_before'] = h.source_records(S)
    require(result['candidate_before'] == state['snapshot'] == result['snapshot_before'],
            'snapshot or candidate differs from manifest')

    dm_raw = contents['dependency-manifest.json']
    require(DM.read_bytes() == dm_raw, 'original dependency manifest differs')
    dependencies = json.loads(dm_raw)
    require(set(dependencies) == {'externs', 'proc_macros'} and set(dependencies['externs']) == EXTERNS
            and set(dependencies['proc_macros']) == PROC_MACROS, 'dependency selection')
    for group in dependencies.values():
        for name, row in group.items():
            path = Path(row['path'])
            require(path.parent == DEPS and h.sha(path) == row['sha256'], 'dependency digest: ' + name)
    require(RUST.resolve(strict=True) == RUST and h.sha(RUST) == RUST_SHA, 'rustc identity')
    require(Path(sys.executable).resolve(strict=True) == PYTHON, 'python identity')

    version = h.child(CHILDREN[0], [RUST, '-vV']).read_text()
    require(re.search(r'^release: 1\.99\.0$', version, re.M)
            and re.search(r'^commit-hash: ' + RUST_COMMIT + r'$', version, re.M), 'rustc version')
    host = re.search(r'^host: ([A-Za-z0-9_.-]+)$', version, re.M).group(1)
    require(host == 'x86_64-unknown-linux-gnu', 'host')
    state['tools'] = toolchain(host)
    state['tools_before'] = [h.record(p) for p in state['tools']]
    h.write_json(A / 'toolchain-before.json', state['tools_before'])
    print('inventorying complete dependency directory', flush=True)
    state['deps_before'] = h.inventory(DEPS)
    h.write_json(A / 'dependencies-before.json', state['deps_before'])
    by_path = {row['path']: row for row in state['deps_before']}
    for group in dependencies.values():
        for name, row in group.items():
            require(by_path.get(row['path'], {}).get('sha256') == row['sha256'], 'dependency inventory: ' + name)
    result['dependency_file_count'] = len(state['deps_before'])
    result['dependency_bytes'] = sum(row['bytes'] for row in state['deps_before'])
    print('dependency inventory complete', len(state['deps_before']), flush=True)

    # No --extern mori_core and no prior Core library: this compiles the crate itself.
    library = A / 'libmori_core.rlib'
    require(not library.exists(), 'fresh library path')
    command = [RUST, '--edition=2024', '--crate-name', 'mori_core', '--crate-type', 'rlib',
               S / LIB, '-L', 'dependency=' + str(DEPS),
               '--emit=link,dep-info=' + str(A / '01-core.d'), '-o', library]
    for name, row in sorted(dependencies['externs'].items()):
        command.extend(['--extern', name + '=' + row['path']])
    h.child(CHILDREN[1], command)
    h.depinfo('01-core')
    require(result['depinfo']['01-core']['inputs'] == sorted(str(S / p) for p in closure),
            'dep-info is not the exact 26-file closure')
    result['actual_core_library'] = h.record(library)
    diagnostics = (A / (CHILDREN[1] + '.stderr')).read_text(errors='replace')
    result['compile_stderr_bytes'] = len(diagnostics.encode())
    require(not any(p in diagnostics for p in ('sherlock/reference_stream.rs', 'sherlock/mod.rs')),
            'compiler diagnostic in the streaming wiring')


def postflight(expected, state):
    checks = {}

    def check(name, ready, test):
        if not ready:
            checks[name] = None
            return
        try:
            checks[name] = test() is True
        except BaseException as error:
            checks[name] = False
            result.setdefault('postflight_errors', {})[name] = repr(error)

    def dependencies():
        after = h.inventory(DEPS)
        h.write_json(A / 'dependencies-after.json', after)
        return after == state['deps_before']

    def tools():
        after = [h.record(p) for p in state['tools']]
        h.write_json(A / 'toolchain-after.json', after)
        return after == state['tools_before']

    def snapshot():
        result['snapshot_after'] = h.source_records(S)
        result['candidate_after'] = h.source_records(CAND)
        return result['snapshot_after'] == state['snapshot'] == result['candidate_after']

    def manifest():
        raw = MANIFEST.read_bytes()
        result['manifest_sha256_after'] = digest(raw)
        after = {path: digest(member(path).read_bytes()) for path in state['pinned']}
        return digest(raw) == expected and after == state['pinned']

    def children():
        steps = result['steps']
        return ([step['name'] for step in steps] == list(CHILDREN)
                and all(step.get('exit') == 0 and step.get('timed_out') is False for step in steps))

    check('dependencies_unchanged', h is not None and 'deps_before' in state, dependencies)
    check('tools_unchanged', h is not None and 'tools_before' in state, tools)
    check('snapshot_unchanged', h is not None and 'candidate_before' in result, snapshot)
    check('manifest_unchanged', 'pinned' in state, manifest)
    check('exactly_two_children', True, children)
    return checks


def main():
    if not (sys.flags.isolated and sys.flags.dont_write_bytecode):
        raise SystemExit('invoke with /usr/bin/python3 -I -B')
    if len(sys.argv) != 2 or not re.fullmatch(r'[0-9a-f]{64}', sys.argv[1]):
        raise SystemExit('usage: run_core.py <approved CANDIDATE.json sha256>')
    expected = sys.argv[1]
    if R != R.resolve(strict=True):
        raise SystemExit('runtime root is not canonical')
    manifest_raw = MANIFEST.read_bytes()
    if digest(manifest_raw) != expected:
        raise SystemExit('CANDIDATE.json is not the approved manifest')
    if A.exists() or A.is_symlink():
        raise SystemExit('fresh attempt-core-01 required')
    A.mkdir(mode=0o700)
    S.mkdir()
    (A / 'tmp').mkdir()
    result['expected_manifest_sha256'] = expected
    state = dict(manifest_raw=manifest_raw)
    started = time.monotonic()
    compiled = False
    try:
        run(expected, state)
        compiled = True
    except BaseException as error:
        result['error'] = repr(error)
        print('CORE COMPILE STOP', repr(error), flush=True)
    checks = postflight(expected, state)
    result['postflight'] = checks
    if compiled and checks and all(value is True for value in checks.values()):
        result['terminal_state'] = 'success'
    else:
        result['terminal_state'] = 'failed-postflight' if compiled else 'failed'
    result['elapsed_seconds'] = time.monotonic() - started
    print('attempt-core-01', result['terminal_state'], flush=True)
    # Written last and exclusively: nothing after this can raise into a recorded success.
    with (A / 'result.json').open('x') as stream:
        stream.write(json.dumps(result, indent=2, default=str) + '\n')
    (A / 'result.json').chmod(0o444)
    return 0 if result['terminal_state'] == 'success' else 1


if __name__ == '__main__':
    raise SystemExit(main())
```

### What the driver binds
These guards follow from my prior review. They are not a separate security claim.

- **Verified reads.** Every manifest member is read exactly once and checked for digest and length. All later uses take those bytes, with no `.pyc`.
- **Manifest digest.** The digest comes from argv, is checked before `mkdir`, and is checked again in postflight.
- **Driver identity.** The running driver's own hash must equal its manifest row, and the manifest must contain exactly the reviewed member set.
- **Projected `mod.rs`.** It must reconstruct exactly as BASE plus the patch.
- **Prior closure.** The 26-file closure must equal the actual prior 25 Core dep-info inputs plus `reference_stream.rs`.
- **Helper extraction.** Helpers are rejected if they have duplicates, decorators or default arguments. Their segment hashes are recorded.
- **Dep-info.** The dep-info must exactly equal the 26 snapshot paths.
- **Postflight.** Dependency and toolchain inventories, snapshot, candidate, manifest and the two-child check each run in their own exception guard. `success` requires all five to be `True`.
- **Write order.** `result.json` is written last and exclusively, so nothing after it can raise into a recorded success.

### Residual limitations the confirmation must accept
- **Driver self-hash race.** The interpreter reads the script before it hashes itself, so a time-of-check/time-of-use gap remains. It is covered only by the trusted-checkout model and the postflight re-hash.
- **Proc-macro selection.** Which proc-macro `.so` rustc actually loads is selected through serde/thiserror metadata in `DEPS`. It is pinned only by the full-directory inventory.
- **Stderr gate.** The driver gates stderr only for diagnostics in the two streaming-wiring files. The auditor reviews all other compile warnings.

### Post-run attribution
The compile result counts only if all of these hold:

1. The outer process exits 0 with no traceback.
2. `result.json` is mode 0444 and says `success`.
3. `expected_manifest_sha256 == manifest_sha256_after ==` the approved digest.
4. All five postflight values are `true`, and there is no `error` or `postflight_errors` key.
5. There are exactly the two named steps, each with exit 0 and no timeout. Their recorded argv matches the specification.
6. `depinfo['01-core'].inputs` lists exactly the 26 snapshot paths.
7. `candidate_before`, `candidate_after`, `snapshot_before` and `snapshot_after` all equal the pinned digests.
8. `prior_core_result.sha256` equals the digest from the audited bounded-source-01 result.

Any failure is reported as a failure. A second attempt needs a new approval.

## Scope not covered (any verdict)
- Compiling or running the 8 tests through production `mod.rs`, and any whole-crate `cfg(test)` build.
- Any consumer of the produced rlib.
- Hostile or untrusted inputs; the H1 time/heap/stack gaps remain.
- Publication of the raw captures, which is a separate scope. No private Fenrir material.
- Product and native acceptance: native authority stays 0, and C4, resources and counters are unchanged.
- K, Quint, Lean, Clean, handwritten-circuit, native and target layers keep their gaps. No U, M, MC, SP or G exit closes.

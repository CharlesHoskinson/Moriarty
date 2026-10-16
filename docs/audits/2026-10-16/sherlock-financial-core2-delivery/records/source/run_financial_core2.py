"""One SDK-free Core/2 financial typed-oracle consumer compile and run reusing prior libmori_core.rlib.

Run once, only after fresh designated-security and Muse exact-candidate preflight approvals
of the manifest pinning this file:
    /usr/bin/python3 -I -B R/run_financial_core2.py <approved RUNTIME-MEMBERSHIP.json sha256>

Exactly four children: `rustc -vV`, one `--test` compile of `reuse_core_root.rs` linked against
the prior verified `libmori_core.rlib` and pinned cached externs (zero new Core compilation),
`--list` verification of the single test, and `--exact --test-threads=1 --nocapture` execution.
Helpers come from the verified bytes of `reuse-context/bounded_source_driver.py` by AST extraction;
its top level never runs. No Cargo, SDK, optimization, retry or fallback. Host diagnostic only:
native authority 0, no acceptance, no memory/disk bound. H1 and C4 are unchanged.
"""
from pathlib import Path
import ast, builtins, datetime, hashlib, json, os, re, shutil, signal, stat, subprocess, sys, time, types

R = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/financial-core2-binding-01')
A = R / 'attempt-financial-core2-reuse-01'
S = A / 'source'
MANIFEST = R / 'RUNTIME-MEMBERSHIP.json'
SELF = 'run_financial_core2.py'
RUST = Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc')
RUST_SHA = 'f3834d26669b03f6855fa54bd2381443123e860167bc0c7a8dd137e5cf6e5e4f'
RUST_COMMIT = 'b940084d7eb6a299eb4bfeb8e34901bc051e7ac4'
PYTHON = Path('/usr/bin/python3').resolve(strict=True)
DEPS = Path('/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target/debug/deps')
DM = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-reference-20261015/compiler-source-projection-stripped-local-check/dependency-manifest.json')
DM_SHA = 'ce37c42b96cc071280db3c361ad593172aab6f1716419d7cdc9010120ffb2f36'
PRIOR_CORE_ATTEMPT = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/streaming-02-production-preparation/attempt-core-01')
PRIOR_COMPILE_RESULT = PRIOR_CORE_ATTEMPT / 'result.json'
PRIOR_RESULT_BYTES = 28685
PRIOR_RESULT_SHA = '56154a66fb9a6575cee9201338d1a4bbbb49a99a56481a35079d83afc8973e93'
PRIOR_LIB = PRIOR_CORE_ATTEMPT / 'libmori_core.rlib'
PRIOR_LIB_BYTES = 15688664
PRIOR_LIB_SHA = 'dff1c647ef75602afd9e7635bc57c529dc06697f85dcfdfab4a061b5c622f4e4'
HELPER_SOURCE = 'reuse-context/bounded_source_driver.py'
HELPER_SHA = '6fe358dbfbed5256e4851d67f09fae93804181c53aa83bb3bdc02667b215fd51'
HELPERS = ('sha', 'record', 'inventory', 'write_json', 'child', 'source_records', 'depinfo')
CHILDREN = (
    '00-rustc-version',
    '01-compile-financial-core2-tests',
    '02-list-exact-tests',
    '03-run-exact-tests',
)
TEST_NAME = 'financial_core2_consumer::independent_core2_financial_two_programs_match_complete_frozen_views'
EXTERNS = {'mori_lexer', 'serde', 'serde_json', 'thiserror', 'sha2', 'k256'}
PROC_MACROS = {'serde_derive', 'thiserror_impl'}
PINNED = {
    'reuse_core_root.rs': dict(
        bytes=327,
        sha256='2fd59425b0fe0155e4d515b44c337615558134b6b4017bf5840ebe6ee1df7b97',
    ),
    'financial_core2_consumer.rs': dict(
        bytes=7122,
        sha256='f8df3b5004278b7f105231a64908b17024120e0707ef2b3a86e227dd30cd3620',
    ),
    'financial_core2_literals.rs': dict(
        bytes=112976,
        sha256='96238b989da3eef19e22ae8963d4f78d8091d7549d999d82ef17e20587e017b9',
    ),
    'financial_projection.rs': dict(
        bytes=9503,
        sha256='911baa49721bed13c62afdb363ff1355629d7d006e70c4ec6ac29520f20e1c1b',
    ),
    'CORE-REUSE-FACTS.json': dict(
        bytes=17647,
        sha256='49c3fc89fb43a0031656b64eb3f882dc610456ff31d262627b8723ff18e7c9c1',
    ),
    'reuse-context/run_core.py': dict(
        bytes=18994,
        sha256='81c079df7dd119093ae71fb9aaffff5238be5ad19bd3ed7e995c3b3229311336',
    ),
    HELPER_SOURCE: dict(
        bytes=11528,
        sha256=HELPER_SHA,
    ),
    'reuse-context/dependency-manifest.json': dict(
        bytes=2049,
        sha256=DM_SHA,
    ),
    'reuse-context/production-core-lib.rs': dict(
        bytes=1277,
        sha256='df45168fa300562f532577739731c257d016a387fc943b25975ae46e3968a2fa',
    ),
    'CANDIDATE-MEMBERSHIP.json': dict(
        bytes=1828,
        sha256='96521effe3e13486e72a02c4b5be52cba27f154dcd99018c2ef8dcb7f7483091',
    ),
    'frozen-oracle/inputs.json': dict(
        bytes=2488,
        sha256='7c9314ff8256d4b2e3b24aa76049423757766d104024d811056e23759207fb4c',
    ),
    'frozen-oracle/inputs/installment-invoice.mori': dict(
        bytes=823,
        sha256='298d787de25ec9e2c324dbf3f305557945d201ebfe8d3e7521a8e2c2d4acfd72',
    ),
    'frozen-oracle/inputs/conditional-grant-tranches.mori': dict(
        bytes=1213,
        sha256='37f8dbc6df9149223756e051ae136053dff6fe935141d732b30f229d42ab1427',
    ),
}
SNAPSHOT_MEMBERS = (
    'reuse_core_root.rs',
    'financial_core2_consumer.rs',
    'financial_core2_literals.rs',
    'financial_projection.rs',
    'frozen-oracle/inputs.json',
    'frozen-oracle/inputs/installment-invoice.mori',
    'frozen-oracle/inputs/conditional-grant-tranches.mori',
)
COMPILE_CLOSURE = (
    'reuse_core_root.rs',
    'financial_core2_consumer.rs',
    'financial_core2_literals.rs',
    'financial_projection.rs',
    'frozen-oracle/inputs/installment-invoice.mori',
    'frozen-oracle/inputs/conditional-grant-tranches.mori',
)
EXPECTED_LOAD_HASHES = {
    'F01': (
        '696c33c267e34781e13c328544590c179ae7b29ea316900adce52bcb51ac48ed',
        'ae28e72fbe5866aa2a95710eefac35b867e227f7628ca87697812ce1ac9f0511',
    ),
    'F02': (
        '8bdb32ae86af2acc646203f8c1498902673640c7989b5c8f6b59358e069cbe26',
        '74f80be6659f10e9a6b216fa2b2790bd82b3673d7c33f0e106bf9f51cfc8a2d7',
    ),
}
EXPECTED_SEQUENCE = {
    'F01': [('start', 'start'), (' not-yet-accepted'.strip(), 'resume'), ('accepted', 'resume')],
    'F02': [
        ('start', 'start'),
        ('review-not-ready', 'resume'),
        ('review-ready', 'resume'),
        ('release-not-ready', 'resume'),
        ('release-ready', 'resume'),
    ],
}
SAFE = r'[A-Za-z0-9][A-Za-z0-9._-]*(?:/[A-Za-z0-9][A-Za-z0-9._-]*)*'

result = dict(
    scope='one SDK-free Core/2 financial two-program typed-oracle consumer compile and run reusing prior libmori_core.rlib',
    terminal_state='running',
    steps=[],
    native_authority=0,
    acceptance='none',
    limits=dict(child_seconds=30, children=4, host_memory_cap='none', host_disk_cap='none'),
    not_run=[
        'new mori_core rlib compile',
        'Cargo',
        'K',
        'Quint',
        'Lean',
        'Clean',
        'circuits',
        'crypto sign/verify',
        'SDK/native/proof/ledger/target',
    ],
    provenance=(
        'local snapshot, reused attempt-core-01 libmori_core.rlib (25/26 sources matching current; '
        'sole diff crates/mori-core/src/sherlock/mod.rs outside Core/2 stage/parser/IR/runtime/cost closure), '
        'and selected tool/dependency inventories only; cached libmori_lexer.rlib and extern/proc-macro '
        'transitive source authenticity unestablished; sysroot/linker/libc/Python stdlib/kernel trusted locally'
    ),
)
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


def plain_external(path):
    require(isinstance(path, Path) and not path.is_symlink()
            and path.resolve(strict=True) == path and path.is_file(),
            'not a plain external file: ' + str(path))
    return path


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
    namespace = dict(
        __name__='bounded_source_helpers',
        __builtins__=builtins,
        Path=Path,
        datetime=datetime,
        hashlib=hashlib,
        json=json,
        os=os,
        re=re,
        shutil=shutil,
        signal=signal,
        stat=stat,
        subprocess=subprocess,
        time=time,
        A=A,
        S=S,
        result=result,
        manifest=snapshot_rows,
    )
    exec(compile(ast.Module(body=selected, type_ignores=[]), str(R / HELPER_SOURCE), 'exec',
                 dont_inherit=True), namespace)
    result['helpers'] = {
        node.name: dict(
            lines=[node.lineno, node.end_lineno],
            sha256=digest(ast.get_source_segment(text, node).encode()),
        )
        for node in selected
    }
    return types.SimpleNamespace(**{name: namespace[name] for name in HELPERS})


def toolchain(host):
    sysroot = RUST.parent.parent
    target = sysroot / 'lib/rustlib' / host
    tools = {
        RUST,
        PYTHON,
        Path(shutil.which('cc', path='/usr/bin:/bin')).resolve(strict=True),
        Path(shutil.which('ld', path='/usr/bin:/bin')).resolve(strict=True),
        DM,
        PRIOR_COMPILE_RESULT,
        PRIOR_LIB,
        Path('/usr/bin/env').resolve(strict=True),
    }
    tools.update((sysroot / 'lib').glob('librustc_driver-*.so'))
    tools.update(p.resolve(strict=True) for p in (target / 'bin').rglob('*')
                 if p.is_file() and ('rust-lld' in str(p) or 'gcc-ld' in str(p)))
    tools.update(p.resolve(strict=True) for p in (target / 'lib').rglob('*') if p.is_file())
    return sorted(tools)


def verify_test_stdout(tested):
    prefix = 'test ' + TEST_NAME + ' ... '
    require('running 1 test' in tested and prefix in tested
            and re.search(r'(?:^|\.\.\. )ok$', tested, re.M)
            and 'test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;' in tested,
            'exact single-test pass output required')
    rows = []
    for raw_line in tested.splitlines():
        line = raw_line[len(prefix):] if raw_line.startswith(prefix) else raw_line
        if line.startswith('{'):
            rows.append(json.loads(line))
    expected_labels = [
        ['qualification', 'local-reference-stage'],
        ['authentication', 'none'],
        ['consumption', 'untracked'],
    ]
    expected_resumes = [
        (case_id, item[0])
        for case_id, ops in EXPECTED_SEQUENCE.items()
        for item in ops if item[1] == 'resume'
    ]
    expected_total_lines = 2 + 2 * sum(len(ops) for ops in EXPECTED_SEQUENCE.values())
    require(len(rows) == expected_total_lines, 'unexpected JSON observation line count from F01 and F02')
    loads = [r for r in rows if r.get('operation') == 'load']
    require([r.get('case') for r in loads] == ['F01', 'F02'], 'load observation case order')
    for row in loads:
        case_id = row['case']
        prog_hash, plan_hash = EXPECTED_LOAD_HASHES[case_id]
        require(row.get('actual_program_hash') == prog_hash and row.get('actual_plan_hash') == plan_hash,
                'load hash mismatch: ' + case_id)
        require(row.get('labels') == expected_labels,
                'STAGE_LABELS mismatch in load observation: ' + case_id)
    for case_id, expected_ops in EXPECTED_SEQUENCE.items():
        case_ops = [(r.get('id'), r.get('operation'))
                    for r in rows if r.get('case') == case_id and r.get('operation') in ('start', 'resume')]
        require(case_ops == expected_ops, 'invocation sequence mismatch: ' + case_id)
        debug_ids = [r.get('id') for r in rows
                     if r.get('case') == case_id and 'actual_stage_projection_debug' in r]
        require(debug_ids == [item[0] for item in expected_ops], 'stage projection debug sequence: ' + case_id)
    resumes = [r for r in rows if r.get('operation') == 'resume']
    require([(r.get('case'), r.get('id')) for r in resumes] == expected_resumes
            and all(r.get('resume_projection_equal') is True for r in resumes),
            'resume projection equality check failed')
    return dict(
        json_line_count=len(rows),
        load_cases=[r['case'] for r in loads],
        f01_invocations=[item[0] for item in EXPECTED_SEQUENCE['F01']],
        f02_invocations=[item[0] for item in EXPECTED_SEQUENCE['F02']],
        resume_count=len(resumes),
    )


def run(expected, state):
    global h
    rows = json.loads(state['manifest_raw'])
    require(isinstance(rows, list)
            and all(isinstance(row, dict) and set(row) == {'path', 'sha256', 'bytes'} for row in rows),
            'manifest shape')
    paths = [row['path'] for row in rows]
    require(len(paths) == len(set(paths)), 'duplicate manifest path')
    require(set(paths) == set(PINNED) | {SELF}, 'manifest members differ from the reviewed 14-file set')

    contents = {}
    for row in rows:
        raw = member(row['path']).read_bytes()
        require(digest(raw) == row['sha256'] and len(raw) == row['bytes'],
                'pinned member differs: ' + row['path'])
        contents[row['path']] = raw
    pinned = state['pinned'] = {row['path']: row['sha256'] for row in rows}
    state['pinned_bytes'] = {row['path']: row['bytes'] for row in rows}
    for path, spec in PINNED.items():
        require(pinned.get(path) == spec['sha256'] and state['pinned_bytes'].get(path) == spec['bytes'],
                'reviewed digest or byte count differs: ' + path)
    require(Path(__file__).resolve(strict=True) == R / SELF
            and digest((R / SELF).read_bytes()) == pinned.get(SELF),
            'running driver is not the pinned ' + SELF)
    result['driver'] = dict(path=str(R / SELF), sha256=pinned[SELF], bytes=state['pinned_bytes'][SELF])
    result['candidate_membership_sha256'] = pinned['CANDIDATE-MEMBERSHIP.json']

    snapshot_rows = [
        dict(path=m, sha256=pinned[m], bytes=len(contents[m]))
        for m in sorted(SNAPSHOT_MEMBERS)
    ]
    state['snapshot'] = {row['path']: row['sha256'] for row in snapshot_rows}
    h = load_helpers(contents[HELPER_SOURCE], snapshot_rows)

    # Verify CORE-REUSE-FACTS.json and the prior compiled production rlib and receipt.
    facts = json.loads(contents['CORE-REUSE-FACTS.json'])
    require(facts.get('prior_compile_terminal') == 'success'
            and facts.get('compiled_source_count') == 26
            and facts.get('matches_current_count') == 25
            and facts.get('core2_producer_subset_all_match') is True
            and facts.get('current_stage_binding_snapshot_matches') is True,
            'CORE-REUSE-FACTS summary mismatch')
    require(facts.get('prior_compile_result') == dict(
        path=str(PRIOR_COMPILE_RESULT), bytes=PRIOR_RESULT_BYTES, sha256=PRIOR_RESULT_SHA),
        'CORE-REUSE-FACTS prior_compile_result mismatch')
    require(facts.get('prior_library') == dict(
        path=str(PRIOR_LIB), bytes=PRIOR_LIB_BYTES, sha256=PRIOR_LIB_SHA),
        'CORE-REUSE-FACTS prior_library mismatch')
    sources = facts.get('prior_compile_actual_sources', [])
    require(len(sources) == 26 and all(row.get('recorded_matches') is True for row in sources),
            'CORE-REUSE-FACTS prior_compile_actual_sources count or recorded match')
    mismatched = [row['path'] for row in sources if not row.get('current_matches_compiled')]
    require(mismatched == ['crates/mori-core/src/sherlock/mod.rs'],
            'unexpected source diff outside crates/mori-core/src/sherlock/mod.rs')
    lib_rows = [row for row in sources if row.get('path') == 'crates/mori-core/src/lib.rs']
    require(len(lib_rows) == 1
            and lib_rows[0]['compiled_source_sha256'] == PINNED['reuse-context/production-core-lib.rs']['sha256'],
            'production-core-lib.rs does not match compiled lib.rs')

    plain_external(PRIOR_COMPILE_RESULT)
    plain_external(PRIOR_LIB)
    prior_raw = PRIOR_COMPILE_RESULT.read_bytes()
    require(digest(prior_raw) == PRIOR_RESULT_SHA and len(prior_raw) == PRIOR_RESULT_BYTES,
            'prior attempt-core-01 result.json digest mismatch')
    prior = json.loads(prior_raw)
    prior_prefix = str(PRIOR_CORE_ATTEMPT / 'source') + '/'
    prior_inputs = prior.get('depinfo', {}).get('01-core', {}).get('inputs', [])
    require(prior.get('terminal_state') == 'success'
            and prior.get('actual_core_library') == dict(
                path=str(PRIOR_LIB), bytes=PRIOR_LIB_BYTES, sha256=PRIOR_LIB_SHA)
            and prior_inputs == sorted(prior_prefix + row['path'] for row in sources),
            'prior attempt-core-01 receipt or depinfo mismatch')
    state['prior_result_before'] = h.record(PRIOR_COMPILE_RESULT)
    state['prior_lib_before'] = h.record(PRIOR_LIB)
    require(state['prior_lib_before'] == dict(
        path=str(PRIOR_LIB), bytes=PRIOR_LIB_BYTES, sha256=PRIOR_LIB_SHA),
        'prior libmori_core.rlib on-disk bytes differ from pinned digest')
    result['reused_core_library'] = state['prior_lib_before']
    result['prior_core_compile_result'] = state['prior_result_before']

    oracle_inputs = json.loads(contents['frozen-oracle/inputs.json'])
    require(oracle_inputs.get('format') == 'sherlock-independent-core2-financial-inputs/1'
            and oracle_inputs.get('source_profile') == 'moriarty/2'
            and oracle_inputs.get('semantics') == 'moriarty-core/2'
            and [c.get('case_id') for c in oracle_inputs.get('cases', [])] == ['F01', 'F02'],
            'frozen-oracle/inputs.json header or case list mismatch')

    require(re.fullmatch(r'[A-Za-z0-9/._-]+', str(S)), 'snapshot path')
    for row in snapshot_rows:
        dest = S / row['path']
        dest.parent.mkdir(parents=True, exist_ok=True)
        with dest.open('xb') as stream:
            stream.write(contents[row['path']])
        dest.chmod(0o444)
    h.write_json(A / 'manifest.json', snapshot_rows)
    for directory in sorted((p for p in S.rglob('*') if p.is_dir()), reverse=True) + [S]:
        directory.chmod(0o555)
    result['candidate_before'] = h.source_records(R)
    result['snapshot_before'] = h.source_records(S)
    require(result['candidate_before'] == state['snapshot'] == result['snapshot_before'],
            'snapshot or candidate differs from manifest')

    plain_external(DM)
    dm_raw = contents['reuse-context/dependency-manifest.json']
    require(DM.read_bytes() == dm_raw, 'original dependency manifest differs')
    dependencies = json.loads(dm_raw)
    require(set(dependencies) == {'externs', 'proc_macros'}
            and set(dependencies['externs']) == EXTERNS
            and set(dependencies['proc_macros']) == PROC_MACROS,
            'dependency selection')
    for group in dependencies.values():
        for name, row in group.items():
            path = plain_external(Path(row['path']))
            require(path.parent == DEPS and h.sha(path) == row['sha256'], 'dependency digest: ' + name)
    require( plain_external(RUST) == RUST and h.sha(RUST) == RUST_SHA, 'rustc identity')
    require(Path(sys.executable).resolve(strict=True) == PYTHON, 'python identity')

    version = h.child(CHILDREN[0], [RUST, '-vV']).read_text()
    require(re.search(r'^release: 1\.99\.0$', version, re.M)
            and re.search(r'^commit-hash: ' + RUST_COMMIT + r'$', version, re.M),
            'rustc version')
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
            require(by_path.get(row['path'], {}).get('sha256') == row['sha256'],
                    'dependency inventory: ' + name)
    result['dependency_file_count'] = len(state['deps_before'])
    result['dependency_bytes'] = sum(row['bytes'] for row in state['deps_before'])
    print('dependency inventory complete', len(state['deps_before']), flush=True)

    binary = A / 'financial-core2-tests'
    require(not binary.exists(), 'fresh test binary path')
    compile_cmd = [
        RUST,
        '--edition=2024',
        '--crate-name',
        'financial_core2_reuse_check',
        '--test',
        S / 'reuse_core_root.rs',
        '--extern',
        'mori_core=' + str(PRIOR_LIB),
        '-L',
        'dependency=' + str(DEPS),
        '-C',
        'strip=debuginfo',
        '--emit=link,dep-info=' + str(A / '01-tests.d'),
        '-o',
        binary,
    ]
    for name, row in sorted(dependencies['externs'].items()):
        compile_cmd.extend(['--extern', name + '=' + row['path']])
    h.child(CHILDREN[1], compile_cmd)
    h.depinfo('01-tests')
    require(result['depinfo']['01-tests']['inputs'] == sorted(str(S / p) for p in COMPILE_CLOSURE),
            'dep-info is not the exact 6-file financial Core/2 consumer closure')
    result['test_binary'] = h.record(binary)
    diagnostics = (A / (CHILDREN[1] + '.stderr')).read_text(errors='replace')
    result['compile_stderr_bytes'] = len(diagnostics.encode())

    listed = h.child(CHILDREN[2], [binary, '--list']).read_text()
    actual_names = re.findall(r'^(\S+): test$', listed, re.M)
    require(actual_names == [TEST_NAME] and re.search(r'^1 tests?, 0 benchmarks$', listed, re.M),
            'exact 1 financial Core/2 test required')
    result['test_names'] = actual_names

    tested = h.child(CHILDREN[3], [binary, TEST_NAME, '--exact', '--test-threads=1', '--nocapture']).read_text()
    result['observation_summary'] = verify_test_stdout(tested)


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

    def prior_core():
        lib_after = h.record(PRIOR_LIB)
        res_after = h.record(PRIOR_COMPILE_RESULT)
        result['reused_core_library_after'] = lib_after
        result['prior_core_compile_result_after'] = res_after
        return lib_after == state['prior_lib_before'] and res_after == state['prior_result_before']

    def snapshot():
        result['snapshot_after'] = h.source_records(S)
        result['candidate_after'] = h.source_records(R)
        return result['snapshot_after'] == state['snapshot'] == result['candidate_after']

    def manifest():
        raw = MANIFEST.read_bytes()
        result['manifest_sha256_after'] = digest(raw)
        after = {path: digest(member(path).read_bytes()) for path in state['pinned']}
        after_bytes = {path: member(path).stat().st_size for path in state['pinned']}
        return (digest(raw) == expected
                and after == state['pinned']
                and after_bytes == state['pinned_bytes'])

    def children():
        steps = result['steps']
        return ([step['name'] for step in steps] == list(CHILDREN)
                and all(step.get('exit') == 0 and step.get('timed_out') is False for step in steps))

    check('dependencies_unchanged', h is not None and 'deps_before' in state, dependencies)
    check('tools_unchanged', h is not None and 'tools_before' in state, tools)
    check('prior_core_unchanged', h is not None and 'prior_lib_before' in state, prior_core)
    check('snapshot_unchanged', h is not None and 'candidate_before' in result, snapshot)
    check('manifest_unchanged', 'pinned' in state, manifest)
    check('exactly_four_children', True, children)
    return checks


def main():
    if not (sys.flags.isolated and sys.flags.dont_write_bytecode):
        raise SystemExit('invoke with /usr/bin/python3 -I -B')
    if len(sys.argv) != 2 or not re.fullmatch(r'[0-9a-f]{64}', sys.argv[1]):
        raise SystemExit('usage: run_financial_core2.py <approved RUNTIME-MEMBERSHIP.json sha256>')
    expected = sys.argv[1]
    if R != R.resolve(strict=True):
        raise SystemExit('runtime root is not canonical')
    if MANIFEST.is_symlink() or not MANIFEST.is_file() or MANIFEST.resolve(strict=True) != MANIFEST:
        raise SystemExit('RUNTIME-MEMBERSHIP.json is not a plain file')
    manifest_raw = MANIFEST.read_bytes()
    if digest(manifest_raw) != expected:
        raise SystemExit('RUNTIME-MEMBERSHIP.json is not the approved manifest')
    if A.exists() or A.is_symlink():
        raise SystemExit('fresh attempt-financial-core2-reuse-01 required')
    A.mkdir(mode=0o700)
    S.mkdir()
    (A / 'tmp').mkdir()
    result['expected_manifest_sha256'] = expected
    state = dict(manifest_raw=manifest_raw)
    started = time.monotonic()
    completed = False
    try:
        run(expected, state)
        completed = True
    except BaseException as error:
        result['error'] = repr(error)
        print('FINANCIAL CORE2 REUSE STOP', repr(error), flush=True)
    checks = postflight(expected, state)
    result['postflight'] = checks
    if completed and checks and all(value is True for value in checks.values()):
        result['terminal_state'] = 'success'
    else:
        result['terminal_state'] = 'failed-postflight' if completed else 'failed'
    result['elapsed_seconds'] = time.monotonic() - started
    print('attempt-financial-core2-reuse-01', result['terminal_state'], flush=True)
    # Written last and exclusively: nothing after this can raise into a recorded success.
    with (A / 'result.json').open('x') as stream:
        stream.write(json.dumps(result, indent=2, default=str) + '\n')
    (A / 'result.json').chmod(0o444)
    return 0 if result['terminal_state'] == 'success' else 1


if __name__ == '__main__':
    raise SystemExit(main())

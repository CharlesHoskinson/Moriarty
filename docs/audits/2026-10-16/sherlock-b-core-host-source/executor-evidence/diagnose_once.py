#!/usr/bin/env python3
"""One-shot SDK-free rustc metadata diagnostic for the exact 10-file closure.

Invoke as: python3 -I -B diagnose_once.py (no arguments), from
sdk-free-composition-diagnostic-01/. Root-operated only. Runs one rustc -vV
child then at most one --crate-type=lib --edition=2024 --emit=dep-info+metadata
compile of the fresh snapshot's source/lib.rs. Compile failure is a retained
diagnostic outcome. No Cargo, tests, linker, artifact loading/execution, SDK,
or retry.
"""
import hashlib
import json
import os
import re
import signal
import stat
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

ROOT_EXPECTED = '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/b-phase0-convergence/sdk-free-composition-diagnostic-01'
SELF_NAME = 'diagnose_once.py'
MEMBERSHIP = 'EXECUTOR-MEMBERSHIP.json'
MEMBERSHIP_FORMAT = 'sherlock-sdk-free-diagnostic-executor-membership/1'
SOURCE_CLOSURE = 'SOURCE-CLOSURE.json'
SOURCE_CLOSURE_SHA256 = '5ea9b8e63072dbfde7296e0efdd736938a7b4c4b7aadbbc6112a14514ac33546'
OPERATION = 'EXACT-OPERATION-PROPOSAL.json'
OPERATION_SHA256 = '3e4dfdd50be828b13a04f599e8a1af40a3fdd210c68ed79e16d8d7840632094e'
ROOT_COMPOSITION = 'ROOT-SOURCE-COMPOSITION.json'
ROOT_COMPOSITION_SHA256 = '5a8950587b7e50029bcb4a049f21ee36763498c07ef9dc2a3aace13fa509553d'
RUSTC = '/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc'
RUSTC_RELEASE = '1.99.0'
RUSTC_SHA256 = 'f3834d26669b03f6855fa54bd2381443123e860167bc0c7a8dd137e5cf6e5e4f'
RUSTC_BYTES = 647376
EDITION = '2024'
TIMEOUT_SECONDS = 30
ATTEMPT = 'diagnostic-attempt-01'
CRATE = 'sherlock_b_shared_source_diagnostic'
FORMAT = 'sherlock-sdk-free-source-diagnostic/1'
# snapshot-relative path, bytes, sha256 -- the exact current 10 pins
RUST_SOURCES = (
    ('families/environment.rs', 15873, '1d7d7835cffabe2e390817e6d628cfa38e5afabcc3a6693b59ab23056906e40a'),
    ('families/legacy.rs', 315, 'e3da45b9f70b7743d188e672eec5a9d84bceab7cdef7ee8c5492f5b0589ba458'),
    ('families/mod.rs', 2545, '4c28ee00b4c298dd7f4c1d180aee7495027c82f4be7cf14677f6bd6f36470050'),
    ('families/scalar.rs', 315, '259b5b9babe5a481332a57bba5b0136d6413c3ba648217d323f914870fa86dd3'),
    ('families/structural.rs', 22406, 'b356f61eb32e2f35aa1cab1e7b2e66ea32a96871d519f5db36c2c434e7359be6'),
    ('fragments/legacy.proposed.rs', 22227, '96498b8a1e85ce7da24a0cc4bdf24e92b1e4fdb355c2dd1ec92562a53c49c1ce'),
    ('fragments/scalar.proposed.rs', 13694, 'beda415aaf730aeee7b0f07a8188c451a748dd6b9e429010618ad92ea8852511'),
    ('inputs/raw.rs', 18821, 'ef9c70a41933acf2c4310dab566c83fd72087b8ea435869a8c283f6c40aff7d1'),
    ('inputs/shared.rs', 43046, 'b91a60783b9b626ddfa17013ed7aefab587557f414e789662a56d2517aa41631'),
    ('lib.rs', 312, '1a5ab7e2e3701637e698042f564d29d0e0eee6d0823bd2057eaa871c0843717d'),
)
REVIEWS = (('security', 'wrapper-preflight-security/REPORT.md'),
           ('nonsecurity', 'wrapper-preflight-nonsecurity/REPORT.md'))
VERDICT = 'APPROVE_WITH_SCOPE'
VERDICT_LINE = re.compile(r'(?m)^VERDICT:(.*)$')
BINDING_LINE = re.compile(r'(?m)^CANDIDATE_MANIFEST_SHA256:(.*)$')
HEX64 = re.compile(r'[0-9a-f]{64}')
SAFE_PATH = re.compile(r'/[A-Za-z0-9._+/-]*')
CLAIMS = dict(
    observed='one rustc -vV and at most one metadata-only diagnostic compile of the exact 10-file closure, this attempt only',
    acceptance='none', source_adoption=False, native_authority=0, c4='unchanged',
    counters='unchanged', tests='not-selected; none built, run, or reported as passing',
    compile_failure='allowed diagnostic outcome; all rustc stdout/stderr retained',
    metadata_artifact='recorded only; never read as code, executed, linked, or loaded',
    not_run=['Cargo', 'tests', 'linker', 'artifact-execution', 'SDK', 'native', 'proof', 'circuit', 'ledger'],
    wall_clock_seconds_per_child=TIMEOUT_SECONDS, memory_bound='none-claimed',
    disk_bound='none-claimed', process_bound='none-claimed; process group killed only on timeout',
    read_only_snapshot='file modes 0444 only; not a boundary against a same-UID child',
    authenticity='local SHA-256 bindings in a trusted checkout only; report text is not reviewer identity',
    unhashed_inputs=['rustc sysroot (librustc_driver, std)', 'system libc', 'Python stdlib', 'kernel'],
    sandbox_isolation='none claimed')


class Failure(Exception):
    pass


class Timeout(Failure):
    pass


def require(condition, message):
    if not condition:
        raise Failure(message)


def refuse(message):
    raise SystemExit(f'diagnose_once: refused before any attempt: {message}')


def utc_now():
    return datetime.now(timezone.utc).isoformat(timespec='milliseconds')


def sha256(raw):
    return hashlib.sha256(raw).hexdigest()


def digest_file(path):
    digest = hashlib.sha256()
    with open(path, 'rb') as handle:
        for block in iter(lambda: handle.read(1 << 20), b''):
            digest.update(block)
    return digest.hexdigest()


def file_record(path):
    return dict(path=str(path), sha256=digest_file(path), bytes=os.stat(path).st_size)


def read_member(root, relative):
    if os.path.isabs(relative) or os.path.normpath(relative) != relative:
        refuse(f'{relative}: not a normalized relative path')
    path = root / relative
    try:
        regular = stat.S_ISREG(os.lstat(path).st_mode)
    except OSError as error:
        refuse(f'{relative}: {error}')
    if not regular or os.path.realpath(path) != str(path):
        refuse(f'{relative}: not a regular file reached without links')
    return path.read_bytes()


def strict_json(raw, name):
    def pairs(items):
        keys = [key for key, _ in items]
        if len(keys) != len(set(keys)):
            refuse(f'{name}: duplicate JSON key')
        return dict(items)

    def constant(token):
        refuse(f'{name}: non-finite JSON number {token}')

    try:
        return json.loads(raw.decode('utf-8'), object_pairs_hook=pairs, parse_constant=constant)
    except ValueError as error:
        refuse(f'{name}: {error}')


def pin_row(row, name):
    if not (type(row) is dict and set(row) == {'path', 'bytes', 'sha256'}
            and type(row['path']) is str and type(row['bytes']) is int
            and type(row['sha256']) is str and HEX64.fullmatch(row['sha256'])):
        refuse(f'{name}: malformed pin row {row!r}')
    return row['path'], row['bytes'], row['sha256']


def compiler_record():
    require(os.path.realpath(RUSTC) == RUSTC, 'compiler path must not traverse a symlink')
    require(stat.S_ISREG(os.stat(RUSTC).st_mode) and os.access(RUSTC, os.X_OK),
            'compiler path is not an executable regular file')
    sysroot = os.path.dirname(os.path.dirname(RUSTC))
    require(os.path.basename(os.path.dirname(RUSTC)) == 'bin'
            and os.path.isdir(os.path.join(sysroot, 'lib', 'rustlib')),
            'compiler has no sibling sysroot; a rustup proxy is not accepted')
    record = dict(file_record(RUSTC), sysroot=sysroot, sysroot_hashed=False)
    require(record['sha256'] == RUSTC_SHA256 and record['bytes'] == RUSTC_BYTES,
            'compiler identity differs from the pinned 1.99.0 binary')
    return record


def compile_inputs(dep_info, snapshot):
    deps = set()
    for line in dep_info.read_text(encoding='utf-8').splitlines():
        if not line.startswith('#'):
            _, separator, rest = line.partition(': ')
            deps.update(rest.split() if separator else ())
    prefix = str(snapshot) + os.sep
    outside = sorted(dep for dep in deps if not os.path.normpath(dep).startswith(prefix))
    require(not outside, f'compile inputs outside the source snapshot: {outside}')
    read = {os.path.relpath(os.path.normpath(dep), snapshot) for dep in deps}
    expected = {relative for relative, _, _ in RUST_SOURCES}
    require(read == expected,
            f'compile closure mismatch: undeclared {sorted(read - expected)}, unread {sorted(expected - read)}')
    return sorted(read)


def terminal_state(steps):
    failed = next((step for step in steps if step['state'] not in ('ok', 'not-selected')), None)
    if failed is None:
        return 'success'
    return ('interrupted' if failed['state'] == 'running' else failed['state']) + ':' + failed['name']


class Attempt:
    def __init__(self, root, bound):
        self.root = root
        self.bound = bound
        self.snapshot = root / 'source'
        self.out = root / 'out'
        self.logs = root / 'logs'
        for path in (self.snapshot, self.snapshot / 'inputs', self.out, self.logs, root / 'tmp'):
            path.mkdir(mode=0o700)
        self.env = {'PATH': '/usr/bin:/bin', 'LANG': 'C.UTF-8', 'TMPDIR': str(root / 'tmp')}
        self.current = None
        self.receipt = dict(
            format=FORMAT, output_dir=str(root), argv=list(sys.argv),
            child_environment=self.env, timeout_seconds=TIMEOUT_SECONDS, claims=CLAIMS,
            source_closure_sha256=SOURCE_CLOSURE_SHA256, operation_proposal_sha256=OPERATION_SHA256,
            root_composition_sha256=ROOT_COMPOSITION_SHA256,
            executor_membership=dict(path=MEMBERSHIP, sha256=bound['membership_sha256']),
            execution_preflights=bound['reviews'], python=None, compiler=None,
            sources=None, steps=[], terminal_state='running')
        self.save()

    def save(self):
        partial = self.root / 'receipt.json.partial'
        partial.write_text(json.dumps(self.receipt, indent=2, allow_nan=False) + '\n', encoding='utf-8')
        os.replace(partial, self.root / 'receipt.json')

    def step(self, name, action):
        self.current = dict(name=name, state='running')
        self.receipt['steps'].append(self.current)
        self.save()
        try:
            action()
            self.current['state'] = 'ok'
        except Timeout as error:
            self.current.update(state='timeout', error=str(error))
        except Exception as error:
            self.current.update(state='failed', error=f'{type(error).__name__}: {error}')
        self.save()
        return self.current['state'] == 'ok'

    def child(self, argv):
        name = self.current['name']
        stdout = self.logs / (name + '.stdout')
        stderr = self.logs / (name + '.stderr')
        record = dict(argv=[str(arg) for arg in argv], cwd=str(self.root), environment=dict(self.env),
                      timeout_seconds=TIMEOUT_SECONDS, started_utc=utc_now(), timed_out=False)
        self.current['child'] = record
        self.save()
        with open(stdout, 'xb') as out, open(stderr, 'xb') as err:
            began = time.monotonic()
            process = subprocess.Popen(record['argv'], cwd=self.root, env=self.env,
                                       stdin=subprocess.DEVNULL, stdout=out, stderr=err,
                                       close_fds=True, start_new_session=True)
            try:
                process.wait(timeout=TIMEOUT_SECONDS)
            except subprocess.TimeoutExpired:
                record['timed_out'] = True
            finally:
                if process.returncode is None:
                    try:
                        os.killpg(process.pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                    process.wait()
            record.update(duration_seconds=round(time.monotonic() - began, 3),
                          exit_code=process.returncode)
        record.update(stdout=file_record(stdout), stderr=file_record(stderr))
        self.save()
        if record['timed_out']:
            raise Timeout(f'{name}: exceeded {TIMEOUT_SECONDS} s wall clock; process group killed')
        return record

    def preflight(self):
        python = sys.executable
        require(os.path.isabs(python), 'interpreter path unavailable')
        self.receipt['python'] = dict(file_record(python), realpath=os.path.realpath(python),
                                      version=sys.version, isolated=bool(sys.flags.isolated),
                                      dont_write_bytecode=bool(sys.flags.dont_write_bytecode))
        contents = self.bound['rust_contents']
        self.receipt['sources'] = [dict(path=relative, sha256_before=sha)
                                   for relative, (_, sha) in
                                   ((r, (b, s)) for r, b, s in RUST_SOURCES) for _ in [0]][:0] or \
                                  [dict(path=r, sha256_before=s) for r, _, s in RUST_SOURCES]
        for relative, raw in contents.items():
            path = self.snapshot / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            with open(path, 'xb') as handle:
                handle.write(raw)
            os.chmod(path, 0o444)
        self.receipt['compiler'] = compiler_record()

    def version(self):
        record = self.child([RUSTC, '-vV'])
        require(record['exit_code'] == 0, f"rustc-version: exit status {record['exit_code']}")
        text = (self.logs / 'rustc-version.stdout').read_text(encoding='utf-8')
        fields = dict(line.split(': ', 1) for line in text.splitlines() if ': ' in line)
        release = fields.get('release')
        require(release == RUSTC_RELEASE, f'compiler release {release!r} is not {RUSTC_RELEASE!r}')
        self.receipt['compiler'].update(version_text=text, release=release,
                                        commit_hash=fields.get('commit-hash'),
                                        host=fields.get('host'))

    def compile(self):
        dep_info = self.out / 'source.d'
        metadata = self.out / 'source.rmeta'
        argv = [RUSTC, '--crate-type=lib', '--edition=' + EDITION, '--crate-name=' + CRATE,
                f'--emit=dep-info={dep_info},metadata={metadata}',
                str(self.snapshot / 'lib.rs')]
        record = self.child(argv)
        produced = []
        for path in sorted(self.out.rglob('*')):
            if path.is_file():
                produced.append(file_record(path))
        self.current['produced_files'] = produced
        if dep_info.exists():
            self.current['dep_info'] = file_record(dep_info)
            self.current['compile_inputs'] = compile_inputs(dep_info, self.snapshot)
        else:
            # Explicit missing record; never fabricated into a closure pass.
            self.current['dep_info'] = dict(path=str(dep_info), present=False,
                                            status='missing; compiler did not emit dep-info')
            self.current['compile_inputs'] = None
        if metadata.exists():
            self.current['metadata_artifact'] = dict(file_record(metadata),
                                                     disposition='recorded only; never read as code, executed, linked, or loaded')
        require(record['exit_code'] == 0,
                f"compile-diagnostic: exit status {record['exit_code']}; diagnostics retained in compile stdout/stderr")

    def postflight(self):
        root = Path(ROOT_EXPECTED)
        after = {}
        for relative, raw in self.bound['bound_files'].items():
            current = read_member(root, relative)
            after[relative] = sha256(current)
            require(current == raw, f'{relative} changed during the attempt')
        snapshot_after = {relative: digest_file(self.snapshot / relative)
                          for relative, _, _ in RUST_SOURCES}
        expected = {relative: sha for relative, _, sha in RUST_SOURCES}
        require(snapshot_after == expected, 'source snapshot bytes changed during the attempt')
        self.receipt['sources'] = [dict(path=r, sha256_before=s,
                                        snapshot_sha256_after=snapshot_after[r],
                                        checkout_sha256_after=after[r])
                                   for r, _, s in RUST_SOURCES]
        self.receipt['bound_files_after'] = after
        python = self.receipt['python']
        if python is not None:
            python['sha256_after'] = digest_file(sys.executable)
            require(python['sha256_after'] == python['sha256'],
                    'interpreter binary changed during the attempt')
        compiler = self.receipt['compiler']
        if compiler is not None:
            compiler['sha256_after'] = digest_file(RUSTC)
            require(compiler['sha256_after'] == compiler['sha256'] == RUSTC_SHA256,
                    'compiler binary changed during the attempt')
        produced = []
        for path in sorted(self.root.rglob('*')):
            if path.is_file() and path.name not in ('receipt.json', 'result.json'):
                produced.append(file_record(path))
        self.receipt['produced_files'] = produced

    def run(self):
        plan = [('preflight', self.preflight), ('rustc-version', self.version),
                ('compile-diagnostic', self.compile)]
        blocked = None
        for name, action in plan:
            if blocked is None:
                blocked = None if self.step(name, action) else name
            else:
                self.receipt['steps'].append(dict(name=name, state='not-run',
                                                  reason=f'dependency {blocked} did not succeed; no further child launched'))
        self.save()

    def finish(self):
        if self.receipt['sources'] is not None or any(
                step['name'] == 'preflight' for step in self.receipt['steps']):
            self.step('postflight', self.postflight)
        else:
            self.receipt['steps'].append(dict(name='postflight', state='not-run',
                                              reason='no source snapshot was taken'))
        state = terminal_state(self.receipt['steps'])
        self.receipt['terminal_state'] = state
        self.save()
        result = dict(format=FORMAT, terminal_state=state, output_dir=str(self.root),
                      receipt=file_record(self.root / 'receipt.json'))
        with open(self.root / 'result.json', 'x', encoding='utf-8') as handle:
            handle.write(json.dumps(result, indent=2) + '\n')
        print(json.dumps(result, indent=2))
        return 0 if state == 'success' else 1


def load_bound(root):
    if str(root) != ROOT_EXPECTED:
        refuse('executor is not in the fixed sdk-free-composition-diagnostic-01 directory')
    if not SAFE_PATH.fullmatch(ROOT_EXPECTED + '/' + ATTEMPT):
        refuse('attempt path violates SAFE_PATH')
    membership_raw = read_member(root, MEMBERSHIP)
    membership_sha = sha256(membership_raw)
    membership = strict_json(membership_raw, MEMBERSHIP)
    if type(membership) is not dict or set(membership) != {
            'format', 'executor', 'source_closure', 'operation_proposal', 'rust_sources', 'compiler'}:
        refuse(f'{MEMBERSHIP}: unexpected keys')
    if membership['format'] != MEMBERSHIP_FORMAT:
        refuse(f'{MEMBERSHIP}: unexpected format')
    executor = pin_row(membership['executor'], MEMBERSHIP)
    closure_pin = pin_row(membership['source_closure'], MEMBERSHIP)
    operation_pin = pin_row(membership['operation_proposal'], MEMBERSHIP)
    if executor[0] != SELF_NAME or closure_pin[0] != SOURCE_CLOSURE or operation_pin[0] != OPERATION:
        refuse(f'{MEMBERSHIP}: unexpected executor/closure/operation paths')
    if closure_pin[2] != SOURCE_CLOSURE_SHA256 or operation_pin[2] != OPERATION_SHA256:
        refuse(f'{MEMBERSHIP}: closure or operation hash differs from the bound hashes')
    if type(membership['rust_sources']) is not list or len(membership['rust_sources']) != 10:
        refuse(f'{MEMBERSHIP}: rust_sources must list exactly 10 pins')
    rust_pins = tuple(pin_row(row, MEMBERSHIP) for row in membership['rust_sources'])
    if rust_pins != RUST_SOURCES:
        refuse(f'{MEMBERSHIP}: rust source pins differ from the exact 10 pins')
    compiler_pin = membership['compiler']
    if not (type(compiler_pin) is dict and set(compiler_pin) == {'path', 'bytes', 'sha256', 'release'}
            and compiler_pin['path'] == RUSTC and compiler_pin['bytes'] == RUSTC_BYTES
            and compiler_pin['sha256'] == RUSTC_SHA256 and compiler_pin['release'] == RUSTC_RELEASE):
        refuse(f'{MEMBERSHIP}: compiler pin differs from the fixed 1.99.0 identity')
    bound_files = {MEMBERSHIP: membership_raw}
    for relative in (SELF_NAME, SOURCE_CLOSURE, OPERATION, ROOT_COMPOSITION):
        bound_files[relative] = read_member(root, relative)
    if (len(bound_files[SELF_NAME]), sha256(bound_files[SELF_NAME])) != (executor[1], executor[2]):
        refuse('executor bytes differ from the membership self-pin')
    if (len(bound_files[SOURCE_CLOSURE]), sha256(bound_files[SOURCE_CLOSURE])) != closure_pin[1:]:
        refuse(f'{SOURCE_CLOSURE} differs from the membership pin')
    if (len(bound_files[OPERATION]), sha256(bound_files[OPERATION])) != operation_pin[1:]:
        refuse(f'{OPERATION} differs from the membership pin')
    if sha256(bound_files[ROOT_COMPOSITION]) != ROOT_COMPOSITION_SHA256:
        refuse(f'{ROOT_COMPOSITION} differs from the bound Root composition hash')
    closure_doc = strict_json(bound_files[SOURCE_CLOSURE], SOURCE_CLOSURE)
    if type(closure_doc) is not dict:
        refuse(f'{SOURCE_CLOSURE}: expected an object')
    if closure_doc.get('root_composition') != ROOT_COMPOSITION \
            or closure_doc.get('root_composition_sha256') != ROOT_COMPOSITION_SHA256:
        refuse(f'{SOURCE_CLOSURE}: Root composition binding differs')
    files = closure_doc.get('files')
    if type(files) is not list or len(files) != 10:
        refuse(f'{SOURCE_CLOSURE}: expected exactly 10 file rows')
    if any(type(row) is not dict or set(row) != {'source', 'snapshot', 'bytes', 'sha256'}
           for row in files):
        refuse(f'{SOURCE_CLOSURE}: file rows do not retain source/snapshot/bytes/sha256')
    snapshots = {(row.get('snapshot'), row.get('bytes'), row.get('sha256')) for row in files}
    if snapshots != set(RUST_SOURCES):
        refuse(f'{SOURCE_CLOSURE}: file rows differ from the exact 10 pins')
    composition_doc = strict_json(bound_files[ROOT_COMPOSITION], ROOT_COMPOSITION)
    composition_files = composition_doc.get('files') if type(composition_doc) is dict else None
    if type(composition_files) is not list or len(composition_files) != 10:
        refuse(f'{ROOT_COMPOSITION}: expected exactly 10 file rows')
    composition_rows = [(row.get('path'), row.get('bytes'), row.get('sha256'))
                        for row in composition_files if type(row) is dict]
    if composition_rows != list(RUST_SOURCES):
        refuse(f'{ROOT_COMPOSITION}: file rows differ from the exact 10 pins')
    operation_doc = strict_json(bound_files[OPERATION], OPERATION)
    if type(operation_doc) is not dict:
        refuse(f'{OPERATION}: expected an object')
    if operation_doc.get('source_closure_manifest') != SOURCE_CLOSURE \
            or operation_doc.get('source_closure_manifest_sha256') != SOURCE_CLOSURE_SHA256:
        refuse(f'{OPERATION}: source closure binding differs')
    if operation_doc.get('allowed_compiler_read_closure') != \
            ['source/' + relative for relative, _, _ in RUST_SOURCES]:
        refuse(f'{OPERATION}: allowed compiler read closure differs from the exact 10 files')
    attempt_dir = ROOT_EXPECTED + '/' + ATTEMPT
    expected_argv = [RUSTC, '--crate-type=lib', '--edition=' + EDITION, '--crate-name=' + CRATE,
                     f'--emit=dep-info={attempt_dir}/out/source.d,metadata={attempt_dir}/out/source.rmeta',
                     f'{attempt_dir}/source/lib.rs']
    if operation_doc.get('compile_argv') != expected_argv or operation_doc.get('version_argv') != [RUSTC, '-vV']:
        refuse(f'{OPERATION}: argv differs from the literal version/compile argv')
    if operation_doc.get('exclusive_attempt_dir') != attempt_dir or operation_doc.get('cwd') != ROOT_EXPECTED:
        refuse(f'{OPERATION}: cwd or attempt dir differs')
    compiler_doc = operation_doc.get('compiler')
    if not (type(compiler_doc) is dict and compiler_doc.get('path') == RUSTC
            and compiler_doc.get('release') == RUSTC_RELEASE
            and compiler_doc.get('previous_observed_sha256') == RUSTC_SHA256
            and compiler_doc.get('require_current_identity_recheck') is True
            and compiler_doc.get('reject_symlink_or_rustup_proxy') is True):
        refuse(f'{OPERATION}: compiler identity terms differ')
    if operation_doc.get('child_environment') != {
            'PATH': '/usr/bin:/bin', 'LANG': 'C.UTF-8', 'TMPDIR': attempt_dir + '/tmp'}:
        refuse(f'{OPERATION}: child environment differs from the exact 3 keys')
    if not (operation_doc.get('stdin') == 'DEVNULL' and operation_doc.get('shell') is False
            and operation_doc.get('start_new_session') is True
            and operation_doc.get('timeout_seconds_per_child') == TIMEOUT_SECONDS):
        refuse(f'{OPERATION}: stdin/shell/session/timeout terms differ')
    rust_contents = {}
    for relative, size, digest in RUST_SOURCES:
        raw = read_member(root, relative)
        if (len(raw), sha256(raw)) != (size, digest):
            refuse(f'{relative}: bytes differ from the exact pin')
        rust_contents[relative] = raw
        bound_files[relative] = raw
    reviews = {}
    for reviewer, relative in REVIEWS:
        raw = read_member(root, relative)
        bound_files[relative] = raw
        try:
            text = raw.decode('utf-8')
        except UnicodeDecodeError:
            refuse(f'{relative}: not UTF-8')
        verdicts, bindings = VERDICT_LINE.findall(text), BINDING_LINE.findall(text)
        if verdicts != [' ' + VERDICT] or bindings != [' ' + membership_sha]:
            refuse(f'{reviewer} wrapper preflight does not approve {MEMBERSHIP} {membership_sha} '
                   f'(verdict lines {verdicts}, binding lines {bindings})')
        reviews[reviewer] = dict(path=relative, bytes=len(raw), sha256=sha256(raw))
    compiler_record()
    return dict(membership_sha256=membership_sha, reviews=reviews,
                rust_contents=rust_contents, bound_files=bound_files)


def main():
    if len(sys.argv) != 1:
        refuse('expected no arguments')
    if Path(__file__).name != SELF_NAME:
        refuse(f'executor file must be named {SELF_NAME}')
    root = Path(__file__).resolve().parent
    bound = load_bound(root)
    output = root / ATTEMPT
    try:
        os.mkdir(output, 0o700)
    except OSError as error:
        refuse(f'no fresh {ATTEMPT}: {error}')
    attempt = Attempt(output, bound)
    try:
        attempt.run()
    finally:
        status = attempt.finish()
    return status


if __name__ == '__main__':
    if not (sys.flags.isolated and sys.flags.dont_write_bytecode):
        raise SystemExit('invoke with python3 -I -B diagnose_once.py')
    raise SystemExit(main())

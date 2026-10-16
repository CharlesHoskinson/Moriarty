"""Compile and run exactly the nine aux/2 arithmetic/fuel witness and A consumer tests once.

Runtime preparation only. Before any attempt exists it refuses unless the final
candidate manifest matches the materialized bytes and fresh Muse and Opus
execution-preflight reports each approve that manifest's exact SHA-256. The
unchanged run_host.py guard runs from its verified bytes; only its preflight is
adapted here, so no normal-reference producer, check.py or comparison helper is
read, selected or run. Fixed steps: preflight, rustc -vV, one direct
`rustc --test --edition=2024 --emit=dep-info,link` of the fifteen-file closure, one
run of that test binary, then postflights. The first failure stops every later
step. No shell, Cargo, SDK, backend, VM, model, proof, key, ledger or cache.
Invoke as: python3 -I -B run_arithmetic.py (no arguments).
"""
from pathlib import Path
import hashlib
import importlib.util
import json
import os
import re
import stat
import sys

ROOT = Path(__file__).resolve().parent
EXPECTED_ROOT = '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/arithmetic-consumer-01'
CANDIDATE = ROOT / 'candidate'
PREFIX = 'candidate/'
MANIFEST = 'CANDIDATE.json'
SOURCE_MANIFEST = 'SOURCE-CANDIDATE.json'
CONFIG = 'CONFIG.json'
RUNNER = 'run_arithmetic.py'
GUARD = 'candidate/models/sherlock/qa/run_host.py'
RUSTC = '/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc'
CHANNEL, EDITION, TIMEOUT_SECONDS = '1.99.0', '2024', 30
ATTEMPT = 'attempt-01'
REALIZATION = 'arithmetic'
FORMAT = 'sherlock-arithmetic-consumer-host-run/1'
CONFIG_FORMAT = 'sherlock-arithmetic-consumer-run-config/1'
CONFIG_KEYS = {'format', 'source_candidate', 'compile_closure', 'snapshot_members', 'expected_tests'}
HARNESS = 'arithmetic_harness.rs'
CRATE = 'sherlock_arithmetic_consumer_tests'
CLOSURE_SIZE, TEST_COUNT = 15, 9
REVIEWS = (('muse', 'execution-preflight-muse/REPORT.md'), ('opus', 'execution-preflight-opus/REPORT.md'))
VERDICT = 'APPROVE_WITH_SCOPE'
# A report carries exactly one line of each; any second verdict or binding line refuses.
VERDICT_LINE = re.compile(r'(?m)^VERDICT:(.*)$')
BINDING_LINE = re.compile(r'(?m)^CANDIDATE_MANIFEST_SHA256:(.*)$')
HEX64 = re.compile(r'[0-9a-f]{64}')
CLAIMS = dict(
    observed='Standalone rustc --test compile of the fifteen-file aux/2 arithmetic/fuel witness and '
             'private Step-decoded A consumer closure '
             'and exactly 9 fixed normal specimen tests, this attempt only',
    acceptance='none', native_authority=0,
    c4='unchanged; no circuit, emitter or A102 source is compiled or edited',
    scope='honest-witness host functions and the private Step-decoded A consumer only; Step decoding '
          'is not reachability, transition validation, appendData, aux/2 packing, native binding or '
          'constraint acceptance; canonical decode mode is compiled, not exercised; no constraint, '
          'refusal, decoder refusal, serializer, profile, cap, '
          'wire ABI or security judgment is tested',
    tests='owner literal normal specimens only: host-function A1-A5, S1-S4, L1-L5, M1-M6, T1-T4, P1, '
          'F1-F8, plus P1, A3/S3/L4/M6, T1-T4 and owner fuel values through encoder-built, '
          'Step-decoded A images; fuel values use ordinary Eval/Return/Error controls and claim no '
          'rule line; T1/T2 assert selectors, uu and zero limb blocks only, not header lanes; no '
          'hostile, negative (N1-N16), refusal or generated corpus',
    constructor_startup='not-applicable; no Machine or evaluator is constructed; images are encoded '
                        'from hand-built Snapshots, some edited before decoding',
    not_run=['K', 'Quint', 'Lean', 'Clean', 'handwritten-circuit', 'SDK', 'backend', 'VM', 'native-ledger',
             'Cargo', 'normal-reference-producer', 'check.py-comparison', 'proof', 'key'],
    wall_clock_seconds_per_child=TIMEOUT_SECONDS, memory_bound='none-claimed', disk_bound='none-claimed',
    process_bound='none-claimed; the process group is killed only when its leader times out',
    read_only_snapshot='file modes 0444 only; not a boundary against a same-UID child',
    authenticity='local SHA-256 bindings in a trusted checkout only',
    unhashed_inputs=['rustc sysroot (librustc_driver, std)', 'system linker and libc', 'Python stdlib', 'kernel'],
    product_wiring='not-run; production mod.rs/lib.rs are not compiled; this harness compiles '
                   'unwired candidate leaves through private shim modules by #[path]; the crate::sherlock '
                   'paths of the new leaves are not checked against product wiring')


def sha256(raw):
    return hashlib.sha256(raw).hexdigest()


def refuse(message):
    raise SystemExit(f'run_arithmetic: refused before any attempt: {message}')


def read_member(relative):
    """Read one normalized relative path below ROOT once; links, escapes and nonregular files refuse."""
    if os.path.isabs(relative) or os.path.normpath(relative) != relative:
        refuse(f'{relative}: not a normalized relative path')
    path = ROOT / relative
    try:
        regular = stat.S_ISREG(os.lstat(path).st_mode)
    except OSError as error:
        refuse(f'{relative}: {error}')
    if not regular or os.path.realpath(path) != str(path):
        refuse(f'{relative}: not a regular file reached without links')
    return path.read_bytes()


def strict_json(raw, name):
    """UTF-8 JSON without duplicate keys or non-finite numbers."""
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


def pinned_rows(document, name):
    """A manifest is a list of unique {path, bytes, sha256} rows naming files below ROOT."""
    if type(document) is not list:
        refuse(f'{name}: expected a list of rows')
    rows = {}
    for row in document:
        if not (type(row) is dict and set(row) == {'path', 'bytes', 'sha256'}
                and type(row['path']) is str and type(row['bytes']) is int
                and type(row['sha256']) is str and HEX64.fullmatch(row['sha256'])):
            refuse(f'{name}: malformed row {row!r}')
        path = row['path']
        if os.path.isabs(path) or os.path.normpath(path) != path or path in rows:
            refuse(f'{name}: unnormalized, absolute or duplicate path {path!r}')
        rows[path] = (row['bytes'], row['sha256'])
    return rows


def checked_config(config, source):
    """CONFIG pins the reviewed source rows and states one consistent closure, snapshot and suite."""
    if type(config) is not dict or set(config) != CONFIG_KEYS or config['format'] != CONFIG_FORMAT:
        refuse(f'{CONFIG}: unexpected keys or format')
    if pinned_rows(config['source_candidate'], CONFIG) != source:
        refuse(f'{SOURCE_MANIFEST} differs from the source rows pinned by {CONFIG}')
    lists = tuple(config[key] for key in ('compile_closure', 'snapshot_members', 'expected_tests'))
    if not all(type(value) is list and all(type(item) is str for item in value)
               and len(set(value)) == len(value) for value in lists):
        refuse(f'{CONFIG}: closure, members and tests must be lists of unique strings')
    closure, members, tests = (tuple(value) for value in lists)
    if not all(path.startswith(PREFIX) for path in source) or set(members) != {
            path[len(PREFIX):] for path in source}:
        refuse(f'{CONFIG}: snapshot members are not exactly the reviewed candidate/ rows')
    if not (len(closure) == CLOSURE_SIZE and HARNESS in closure and set(closure) <= set(members)
            and {GUARD[len(PREFIX):], 'rust-toolchain.toml'} <= set(members) and len(tests) == TEST_COUNT):
        refuse(f'{CONFIG}: closure, guard, toolchain pin or test count differs from the reviewed boundary')
    return closure, members, tests


def candidate_entries():
    """Every non-directory entry below candidate/ by lstat, as (ROOT-relative path, kind)."""
    def fail(error):
        raise error

    entries = set()
    for top, dirs, files in os.walk(CANDIDATE, onerror=fail):
        for name in dirs + files:
            path = os.path.join(top, name)
            mode = os.lstat(path).st_mode
            if not stat.S_ISDIR(mode):
                entries.add((os.path.relpath(path, ROOT), 'file' if stat.S_ISREG(mode) else 'other'))
    return entries


def review_gate(manifest_sha):
    """Each fresh execution-preflight report approves exactly this final manifest, once."""
    reviews = {}
    for reviewer, relative in REVIEWS:
        raw = read_member(relative)
        try:
            text = raw.decode('utf-8')
        except UnicodeDecodeError:
            refuse(f'{relative}: not UTF-8')
        verdicts, bindings = VERDICT_LINE.findall(text), BINDING_LINE.findall(text)
        if verdicts != [' ' + VERDICT] or bindings != [' ' + manifest_sha]:
            refuse(f'{reviewer} execution preflight does not approve {MANIFEST} {manifest_sha} '
                   f'(verdict lines {verdicts}, binding lines {bindings})')
        reviews[reviewer] = dict(path=relative, bytes=len(raw), sha256=sha256(raw))
    return reviews


def load_guard(raw):
    """Execute the verified guard bytes directly: no file re-read or cached bytecode."""
    path = ROOT / GUARD
    spec = importlib.util.spec_from_file_location('arithmetic_host_guard', path)
    guard = importlib.util.module_from_spec(spec)
    exec(compile(raw, str(path), 'exec', dont_inherit=True), guard.__dict__)
    guard.REPO = CANDIDATE
    return guard


def attempt_class(guard, pins):
    class ArithmeticAttempt(guard.Attempt):
        """The guard's Attempt; only preflight differs, and only as stated in it."""

        def preflight(self):
            # As run_host.Attempt.preflight except: no check.py/helper scan (this realization
            # reads and runs none), snapshot bytes bound to the reviewed rows before use, and
            # the fixed compiler path and channel.
            python = sys.executable
            guard.require(os.path.isabs(python), 'interpreter path unavailable')
            self.receipt['python'] = dict(guard.file_record(python), realpath=os.path.realpath(python),
                                          version=sys.version, isolated=bool(sys.flags.isolated),
                                          dont_write_bytecode=bool(sys.flags.dont_write_bytecode))
            contents = guard.read_sources(guard.REPO, self.realization.sources)
            hashes = {path: sha256(raw) for path, raw in contents.items()}
            guard.require(hashes == pins, 'snapshot members differ from the reviewed candidate bytes')
            self.sources = hashes
            self.receipt['sources'] = [dict(path=path, sha256_before=sha) for path, sha in hashes.items()]
            # Every child reads these exact verified bytes, never the live candidate.
            for relative, raw in contents.items():
                path = self.snapshot / relative
                path.parent.mkdir(parents=True, exist_ok=True)
                with open(path, 'xb') as handle:
                    handle.write(raw)
                os.chmod(path, 0o444)
            self.receipt['comparison_helpers'] = 'not-selected; no check.py, helper, producer or comparison is read or run'
            self.channel = guard.toolchain_channel(contents['rust-toolchain.toml'])
            guard.require(self.channel == CHANNEL, f'rust-toolchain.toml pins {self.channel}, not {CHANNEL}')
            guard.require(self.rustc == RUSTC, 'compiler is not the pinned toolchain binary')
            self.receipt['compiler'] = dict(guard.compiler_record(self.rustc), toolchain_channel=self.channel)

    return ArithmeticAttempt


def main():
    if str(ROOT) != EXPECTED_ROOT or Path(__file__).resolve() != ROOT / RUNNER:
        refuse('runner is not at the fixed candidate root')
    manifest_raw = read_member(MANIFEST)
    manifest = pinned_rows(strict_json(manifest_raw, MANIFEST), MANIFEST)
    # Read each pinned member once; every later check and the guard use only these bytes.
    contents = {path: read_member(path) for path in manifest}
    if {path: (len(raw), sha256(raw)) for path, raw in contents.items()} != manifest:
        refuse(f'{MANIFEST} does not match the materialized bytes')
    for required in (SOURCE_MANIFEST, CONFIG, RUNNER, GUARD):
        if required not in manifest:
            refuse(f'{MANIFEST} omits {required}')
    source = pinned_rows(strict_json(contents[SOURCE_MANIFEST], SOURCE_MANIFEST), SOURCE_MANIFEST)
    closure, members, tests = checked_config(strict_json(contents[CONFIG], CONFIG), source)
    if set(manifest) != set(source) | {SOURCE_MANIFEST, CONFIG, RUNNER} or any(
            manifest[path] != row for path, row in source.items()):
        refuse(f'{MANIFEST} is not exactly the {SOURCE_MANIFEST} rows plus it, {CONFIG} and {RUNNER}')
    try:
        candidate_dir = stat.S_ISDIR(os.lstat(CANDIDATE).st_mode)
    except OSError as error:
        refuse(f'candidate/: {error}')
    if not candidate_dir:
        refuse('candidate/ is not a directory reached without links')
    expected_tree = {(path, 'file') for path in source}
    if candidate_entries() != expected_tree:
        refuse('candidate/ holds entries other than exactly the reviewed regular files')
    manifest_sha = sha256(manifest_raw)
    reviews = review_gate(manifest_sha)
    output = ROOT / ATTEMPT
    guard = load_guard(contents[GUARD])
    if not (guard.EDITION == EDITION and guard.TIMEOUT_SECONDS == TIMEOUT_SECONDS
            and guard.SAFE_PATH.fullmatch(str(output))
            and os.path.commonpath([str(output), str(CANDIDATE)]) != str(CANDIDATE)):
        refuse('attempt path, edition or wall clock differs from the reviewed boundary')
    # compile() roots the crate at realization.producer: here the harness, never a producer.
    guard.REALIZATIONS[REALIZATION] = guard.Realization(
        FORMAT, HARNESS, CRATE, None, closure, tests, members, None, None, None, CLAIMS)
    try:
        output.mkdir(mode=0o700)  # Exclusive: an existing attempt is never reused or overwritten.
    except OSError as error:
        refuse(f'no fresh {ATTEMPT}: {error}')
    pins = {path[len(PREFIX):]: sha for path, (_, sha) in source.items()}
    attempt = attempt_class(guard, pins)(output, RUSTC, REALIZATION)
    attempt.receipt.update(
        reviewed_manifest=dict(path=MANIFEST, bytes=len(manifest_raw), sha256=manifest_sha),
        reviewed_candidate={path: sha for path, (_, sha) in manifest.items()},
        execution_preflights=reviews,
        selected=dict(compile_root=HARNESS, compile_closure=list(closure), snapshot_members=list(members),
                      expected_tests=list(tests),
                      not_selected='normal reference producer, check.py comparison and Cargo have no step'))
    plan = [
        ('preflight', attempt.preflight),
        ('rustc-version', attempt.version),
        ('compile-tests', lambda: attempt.compile(CRATE, closure, '--test')),
        ('run-tests', attempt.tests),
    ]

    def candidate_postflight():
        paths = tuple(manifest) + (MANIFEST,) + tuple(relative for _, relative in REVIEWS)
        after = {path: sha256(raw) for path, raw in guard.read_sources(ROOT, paths).items()}
        attempt.receipt['reviewed_candidate_after'] = after
        guard.require(all(after[path] == sha for path, (_, sha) in manifest.items()),
                      'reviewed candidate changed during the attempt')
        guard.require(after[MANIFEST] == manifest_sha, f'{MANIFEST} changed during the attempt')
        guard.require(all(after[review['path']] == review['sha256'] for review in reviews.values()),
                      'an execution-preflight report changed during the attempt')
        guard.require(candidate_entries() == expected_tree, 'candidate/ membership changed during the attempt')

    try:
        blocked = None
        for name, action in plan:
            if blocked is None:
                blocked = None if attempt.step(name, action) else name
            else:
                attempt.receipt['steps'].append(dict(
                    name=name, state='not-run', reason=f'dependency {blocked} did not succeed'))
        attempt.step('candidate-postflight', candidate_postflight)
    finally:
        status = attempt.finish()
    return status


if __name__ == '__main__':
    if not (sys.flags.isolated and sys.flags.dont_write_bytecode):
        raise SystemExit('invoke with python3 -I -B run_arithmetic.py')
    raise SystemExit(main())

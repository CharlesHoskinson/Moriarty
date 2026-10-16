#!/usr/bin/env python3
"""Run the bounded normal Sherlock host reference once and retain its receipt.

Pure host wrapper, not a scheduler: fixed steps in fixed order. Each child runs
directly (no shell, Cargo, rustup discovery, backend selection or retry) with the
caller-supplied rustc, a minimal environment and a 30 s wall clock, from a
read-only source snapshot inside a fresh attempt directory. The first failure
stops every dependent step. Memory, disk and process counts are not bounded.
Fixed trusted normal typed ASTs only; K/Quint/Lean/Clean/circuit/native layers
are not run and no acceptance is claimed.
"""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import stat
import subprocess
import sys
import time

REPO = Path(__file__).resolve().parents[3]
FORMAT = 'sherlock-normal-host-run/1'
TIMEOUT_SECONDS = 30
EDITION = '2024'  # Workspace edition inherited by mori-core; metadata is snapshotted.
PRODUCER = 'models/sherlock/reference/normal_observations.rs'
CHECK = 'models/sherlock/qa/check.py'
SERIALIZER = 'models/sherlock/reference/check-reference.py'
SHERLOCK = 'crates/mori-core/src/sherlock/'
PRODUCER_CLOSURE = (PRODUCER,) + tuple(SHERLOCK + name for name in (
    'reference_types.rs', 'reference.rs', 'reference_display.rs', 'compare.rs',
    'normal_oracle_inputs.rs', 'extra_oracle_programs.rs'))
TEST_CLOSURE = PRODUCER_CLOSURE + (SHERLOCK + 'reference_bounded_tests.rs',)
# Complete known read+compile closure. Membership is a local pin, not schema authenticity.
SOURCE_PATHS = TEST_CLOSURE + (
    CHECK, 'models/sherlock/qa/host_observations.py', 'models/sherlock/qa/run_host.py',
    'models/sherlock/qa/HOST-OBSERVATIONS.md', SERIALIZER, 'models/sherlock/k/main.k',
    'models/sherlock/oracles/normal-programs.json', 'models/sherlock/oracles/normal-prefixes.json',
    'models/sherlock/oracles/extra-normal-programs.json',
    'models/sherlock/oracles/extra-normal-prefixes.json',
    SHERLOCK + 'normal_oracles.rs', SHERLOCK + 'extra_normal_oracles.rs',
    'rust-toolchain.toml', 'Cargo.toml', 'crates/mori-core/Cargo.toml',
    'scripts/doctorwatson.sh')
EXPECTED_TESTS = tuple('reference_bounded_tests::' + name for name in (
    'zero_events_observes_initial_and_partial_machine_without_advancing',
    'partial_calls_resume_the_actual_current_machine',
    'free_publication_is_a_separate_retained_event_at_zero_fuel',
    'excess_event_budget_stops_at_first_terminal_publication',
    'already_published_normal_outcomes_return_no_further_events',
    'widths_one_two_four_eight_retain_every_independent_normal_prefix',
    'mixed_step_and_group_calls_match_actual_one_step_baseline'))
EXPECTED_PROGRAMS, EXPECTED_RUNS = 8, 216
# rustc dep-info is split on whitespace, so attempt paths stay within this alphabet.
SAFE_PATH = re.compile(r'/[A-Za-z0-9._+/-]*')
CLAIMS = dict(
    observed='Rust-host compile, bounded normal tests, producer execution and diagnostic comparison of this attempt only',
    acceptance='none', constructor_startup='not-executed; Machine::new constructs the post-startup state',
    not_run=['K', 'Quint', 'Lean', 'Clean', 'handwritten-circuit', 'native-ledger'],
    wall_clock_seconds_per_child=TIMEOUT_SECONDS, memory_bound='none-claimed', disk_bound='none-claimed',
    authenticity='local SHA-256 bindings in a trusted checkout only',
    unhashed_inputs=['rustc sysroot (librustc_driver, std)', 'system linker and libc', 'Python stdlib', 'kernel'])


class Failure(Exception):
    """A recorded failure; every dependent step is then not run."""


class Timeout(Failure):
    """A child exceeded the fixed wall clock and its process group was killed."""


def require(condition, message):
    if not condition:
        raise Failure(message)


def utc_now():
    return datetime.now(timezone.utc).isoformat(timespec='milliseconds')


def digest_file(path):
    sha = hashlib.sha256()
    with open(path, 'rb') as handle:
        for block in iter(lambda: handle.read(1 << 20), b''):
            sha.update(block)
    return sha.hexdigest()


def file_record(path):
    return dict(path=str(path), sha256=digest_file(path), bytes=os.stat(path).st_size)


def read_sources(root):
    """Read each declared member once; absent, symlinked or nonregular members fail."""
    contents = {}
    for relative in SOURCE_PATHS:
        path = root / relative
        require(os.path.realpath(path) == str(path), f'{relative}: symlinked source member')
        require(stat.S_ISREG(os.lstat(path).st_mode), f'{relative}: not a regular file')
        contents[relative] = path.read_bytes()
    return contents


def toolchain_channel(raw):
    found = re.findall(rb'(?m)^channel = "([0-9]+\.[0-9]+\.[0-9]+)"$', raw)
    require(len(found) == 1, 'rust-toolchain.toml: expected exactly one numeric channel pin')
    return found[0].decode()


def compiler_record(rustc):
    """Accept only an installed toolchain binary itself, never a proxy or a search."""
    require(os.path.realpath(rustc) == rustc, 'compiler path must not traverse a symlink')
    require(stat.S_ISREG(os.stat(rustc).st_mode) and os.access(rustc, os.X_OK),
            'compiler path is not an executable regular file')
    sysroot = os.path.dirname(os.path.dirname(rustc))
    require(os.path.basename(os.path.dirname(rustc)) == 'bin'
            and os.path.isdir(os.path.join(sysroot, 'lib', 'rustlib')),
            'compiler has no sibling sysroot; a rustup proxy is not accepted')
    return dict(file_record(rustc), sysroot=sysroot, sysroot_hashed=False)


def compile_inputs(dep_info, snapshot, expected):
    """Compare rustc's own dep-info with the declared closure exactly."""
    deps = set()
    for line in dep_info.read_text(encoding='utf-8').splitlines():
        if not line.startswith('#'):
            _, separator, rest = line.partition(': ')
            deps.update(rest.split() if separator else ())
    prefix = str(snapshot) + os.sep
    outside = sorted(dep for dep in deps if not os.path.normpath(dep).startswith(prefix))
    require(not outside, f'compile inputs outside the source snapshot: {outside}')
    read = {os.path.relpath(os.path.normpath(dep), snapshot) for dep in deps}
    require(read == set(expected),
            f'compile closure mismatch: undeclared {sorted(read - set(expected))}, unread {sorted(set(expected) - read)}')
    return sorted(read)


def libtest_results(text):
    """Accept exactly the bounded normal tests, each passing, and nothing filtered."""
    count = len(EXPECTED_TESTS)
    results = re.findall(r'(?m)^test (\S+) \.\.\. (\S+)$', text)
    require(re.search(rf'(?m)^running {count} tests$', text), f'test binary did not announce exactly {count} tests')
    require(sorted(results) == sorted((name, 'ok') for name in EXPECTED_TESTS),
            f'test membership/results differ from the bounded normal tests: {results}')
    require(re.search(rf'(?m)^test result: ok\. {count} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;', text),
            'libtest summary is not a complete pass')
    return [name for name, _ in results]


class Attempt:
    """One fresh attempt directory; every step, child and check is recorded in order."""

    def __init__(self, root, rustc):
        self.root, self.rustc = root, rustc
        self.snapshot, self.build, self.logs = root / 'source', root / 'build', root / 'logs'
        for path in (self.snapshot, self.build, self.logs, root / 'tmp'):
            path.mkdir(mode=0o700)
        self.env = {'PATH': '/usr/bin:/bin', 'LANG': 'C.UTF-8', 'TMPDIR': str(root / 'tmp')}
        self.sources, self.artifacts, self.channel, self.current = {}, {}, None, None
        self.receipt = dict(format=FORMAT, checkout=str(REPO), output_dir=str(root), argv=list(sys.argv),
                            child_environment=self.env, timeout_seconds=TIMEOUT_SECONDS, claims=CLAIMS,
                            python=None, compiler=None, sources=None, steps=[], terminal_state='running')
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
        except Exception as error:  # Recorded, never retried; dependents do not run.
            self.current.update(state='failed', error=f'{type(error).__name__}: {error}')
        self.save()
        return self.current['state'] == 'ok'

    def child(self, argv, *, stdout=None):
        """Run one child directly: no shell, minimal environment, fixed wall clock, no retry."""
        name = self.current['name']
        stdout = stdout or self.logs / (name + '.stdout')
        stderr = self.logs / (name + '.stderr')
        record = dict(argv=[str(arg) for arg in argv], cwd=str(self.root), environment=self.env,
                      timeout_seconds=TIMEOUT_SECONDS, started_utc=utc_now(), timed_out=False)
        self.current['child'] = record
        self.save()
        with open(stdout, 'xb') as out, open(stderr, 'xb') as err:
            began = time.monotonic()
            process = subprocess.Popen(record['argv'], cwd=self.root, env=self.env, stdin=subprocess.DEVNULL,
                                       stdout=out, stderr=err, close_fds=True, start_new_session=True)
            try:
                process.wait(timeout=TIMEOUT_SECONDS)
            except subprocess.TimeoutExpired:
                record['timed_out'] = True
            finally:
                if process.returncode is None:
                    # The unreaped leader still reserves its group ID for this kill.
                    try:
                        os.killpg(process.pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                    process.wait()
            record.update(duration_seconds=round(time.monotonic() - began, 3), exit_code=process.returncode)
        record.update(stdout=file_record(stdout), stderr=file_record(stderr))
        if record['timed_out']:
            raise Timeout(f'{name}: exceeded {TIMEOUT_SECONDS} s wall clock; process group killed')
        require(process.returncode == 0, f'{name}: exit status {process.returncode}')

    def preflight(self):
        python = sys.executable
        require(os.path.isabs(python), 'interpreter path unavailable')
        self.receipt['python'] = dict(file_record(python), realpath=os.path.realpath(python), version=sys.version,
                                      isolated=bool(sys.flags.isolated),
                                      dont_write_bytecode=bool(sys.flags.dont_write_bytecode))
        contents = read_sources(REPO)
        self.sources = {path: hashlib.sha256(raw).hexdigest() for path, raw in contents.items()}
        self.receipt['sources'] = [dict(path=path, sha256_before=sha) for path, sha in self.sources.items()]
        # Every child reads these exact verified bytes, never the live checkout.
        for relative, raw in contents.items():
            path = self.snapshot / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            with open(path, 'xb') as handle:
                handle.write(raw)
            os.chmod(path, 0o444)
        self.channel = toolchain_channel(contents['rust-toolchain.toml'])
        self.receipt['compiler'] = dict(compiler_record(self.rustc), toolchain_channel=self.channel)

    def version(self):
        self.child([self.rustc, '-vV'])
        text = (self.logs / 'rustc-version.stdout').read_text(encoding='utf-8')
        fields = dict(line.split(': ', 1) for line in text.splitlines() if ': ' in line)
        release = fields.get('release')
        require(release == self.channel, f'compiler release {release!r} is not the pinned channel {self.channel!r}')
        self.receipt['compiler'].update(version_text=text, release=release,
                                        commit_hash=fields.get('commit-hash'), host=fields.get('host'))

    def compile(self, crate, closure, *flags):
        binary, dep_info = self.build / crate, self.build / (crate + '.d')
        self.child([self.rustc, *flags, '--edition=' + EDITION, '--crate-name=' + crate,
                    f'--emit=dep-info={dep_info},link={binary}', str(self.snapshot / PRODUCER)])
        self.artifacts[self.current['name']] = file_record(binary)
        self.current.update(artifact=self.artifacts[self.current['name']], dep_info=file_record(dep_info))
        self.current['compile_inputs'] = compile_inputs(dep_info, self.snapshot, closure)

    def execute(self, key, *arguments, stdout=None):
        artifact = self.artifacts[key]
        require(digest_file(artifact['path']) == artifact['sha256'], f'{key} artifact changed before execution')
        self.child([artifact['path'], *arguments], stdout=stdout)

    def tests(self):
        self.execute('compile-tests', '--test-threads=1', '--nocapture')
        text = (self.logs / 'run-tests.stdout').read_text(encoding='utf-8', errors='replace')
        self.current['tests'] = libtest_results(text)

    def produce(self):
        output = self.root / 'host-observations.json'
        self.execute('compile-producer', stdout=output)
        record = file_record(output)
        require(record['bytes'] > 0, 'producer wrote no observations')
        self.artifacts['host-observations'] = self.current['host_observations'] = record

    def compare(self):
        produced = self.artifacts['host-observations']
        artifact = produced['path']
        require(digest_file(artifact) == produced['sha256'], 'host observations changed before comparison')
        output = self.root / 'comparison.json'
        self.child([sys.executable, '-I', '-B', str(self.snapshot / CHECK), '--host-observations', artifact],
                   stdout=output)
        report = json.loads(output.read_bytes())
        host = report.get('supplied_host_observation')
        require(type(host) is dict, 'comparison omitted the host observation result')
        status, host_status = report.get('status'), host.get('status')
        self.current['comparison'] = dict(file=file_record(output), status=status, host_status=host_status,
                                          scope=host.get('scope'), first_difference=report.get('first_difference'),
                                          producer_authenticity=host.get('producer_authenticity'),
                                          constructor_startup=host.get('constructor_startup'))
        require(status == 'pass' and host_status == 'match', f'comparison {status}/{host_status}')
        require(host.get('file') == dict(path=artifact, sha256=produced['sha256']),
                'comparison is not bound to the produced artifact bytes')
        require((host.get('expected_programs'), host.get('expected_prefix_runs')) == (EXPECTED_PROGRAMS, EXPECTED_RUNS),
                'comparison coverage differs from the frozen normal program/run counts')
        transcriptions = report.get('rust_literal_transcriptions', [])
        rows = report.get('frozen_files', []) + transcriptions
        read = [(row.get('path'), row.get('sha256')) for row in rows]
        read += [(SERIALIZER, row.get('serializer_sha256')) for row in transcriptions]
        require(len(rows) == 7 and all(self.sources.get(path) == sha for path, sha in read),
                'comparison did not read exactly the snapshot bytes')

    def postflight(self):
        checkout = {path: hashlib.sha256(raw).hexdigest() for path, raw in read_sources(REPO).items()}
        snapshot = {path: digest_file(self.snapshot / path) for path in SOURCE_PATHS}
        self.receipt['sources'] = [dict(path=path, sha256_before=self.sources[path],
                                        snapshot_sha256_after=snapshot[path], checkout_sha256_after=checkout[path])
                                   for path in SOURCE_PATHS]
        python = self.receipt['python']
        python['sha256_after'] = digest_file(sys.executable)
        compiler = self.receipt['compiler']
        if compiler is not None:
            compiler['sha256_after'] = digest_file(self.rustc)
        require(checkout == self.sources, 'checkout source membership bytes changed during the attempt')
        require(snapshot == self.sources, 'source snapshot bytes changed during the attempt')
        require(python['sha256_after'] == python['sha256'], 'interpreter binary changed during the attempt')
        require(compiler is None or compiler['sha256_after'] == compiler['sha256'],
                'compiler binary changed during the attempt')

    def run(self):
        plan = (('preflight', self.preflight), ('rustc-version', self.version),
                ('compile-producer', lambda: self.compile('sherlock_normal_observations', PRODUCER_CLOSURE)),
                ('compile-tests', lambda: self.compile('sherlock_normal_observations_tests', TEST_CLOSURE, '--test')),
                ('run-tests', self.tests), ('run-producer', self.produce), ('compare', self.compare))
        blocked = None
        for name, action in plan:
            if blocked is None:
                blocked = None if self.step(name, action) else name
            else:
                self.receipt['steps'].append(dict(name=name, state='not-run',
                                                  reason=f'dependency {blocked} did not succeed'))
        self.save()

    def finish(self):
        """Always recheck sources after the attempt, then write the final result once."""
        if self.sources:
            self.step('postflight', self.postflight)
        else:
            self.receipt['steps'].append(dict(name='postflight', state='not-run',
                                              reason='no source snapshot was taken'))
        failed = next((step for step in self.receipt['steps'] if step['state'] != 'ok'), None)
        state = 'success' if failed is None else (
            ('interrupted' if failed['state'] == 'running' else failed['state']) + ':' + failed['name'])
        self.receipt['terminal_state'] = state
        self.save()
        result = dict(format=FORMAT, terminal_state=state, output_dir=str(self.root),
                      receipt=file_record(self.root / 'receipt.json'))
        with open(self.root / 'result.json', 'x', encoding='utf-8') as handle:
            handle.write(json.dumps(result, indent=2) + '\n')
        print(json.dumps(result, indent=2))
        return 0 if failed is None else 1


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument('--rustc', required=True, help='absolute path of the installed pinned rustc binary')
    parser.add_argument('--output-dir', required=True, help='absolute, not yet existing attempt directory')
    args = parser.parse_args(argv)
    if not (sys.flags.isolated and sys.flags.dont_write_bytecode):
        parser.error('run as: python3 -I -B models/sherlock/qa/run_host.py')
    for option, value in (('--rustc', args.rustc), ('--output-dir', args.output_dir)):
        if not os.path.isabs(value) or os.path.normpath(value) != value:
            parser.error(f'{option}: expected a normalized absolute path')
    root = os.path.join(os.path.realpath(os.path.dirname(args.output_dir)), os.path.basename(args.output_dir))
    if not SAFE_PATH.fullmatch(root):
        parser.error('--output-dir: only [A-Za-z0-9._+/-] path characters are accepted')
    if os.path.commonpath([root, str(REPO)]) == str(REPO):
        parser.error('--output-dir: must lie outside the checkout')
    try:
        os.mkdir(root, 0o700)  # Exclusive: an existing attempt is never reused or overwritten.
    except OSError as error:
        parser.exit(2, f'run_host: no fresh attempt directory: {error}\n')
    attempt = Attempt(Path(root), args.rustc)
    try:
        attempt.run()
    finally:
        status = attempt.finish()
    return status


if __name__ == '__main__':
    sys.exit(main())

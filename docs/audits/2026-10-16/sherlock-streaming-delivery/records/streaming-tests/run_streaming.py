"""Select only the fixed streaming tests through the existing host-runner methods.

Runtime preparation only: execute after exact-candidate Opus and Muse preflight.
No production module or execution guard is edited by this adapter.
"""
from pathlib import Path
import hashlib
import importlib.util
import json
import sys

ROOT = Path(__file__).resolve().parent
CANDIDATE = ROOT / 'candidate'
MANIFEST = 'CANDIDATE.json'
GUARD = 'candidate/models/sherlock/qa/run_host.py'
RUSTC = '/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc'


def sha256(raw):
    return hashlib.sha256(raw).hexdigest()


def digest(path):
    return sha256(path.read_bytes())


def main():
    manifest_raw = (ROOT / MANIFEST).read_bytes()
    manifest = json.loads(manifest_raw)
    # Read each pinned member once; CONFIG and the guard use only these verified bytes.
    contents = {row['path']: (ROOT / row['path']).read_bytes() for row in manifest}
    before = {path: sha256(raw) for path, raw in contents.items()}
    if before != {row['path']: row['sha256'] for row in manifest}:
        raise SystemExit('reviewed candidate changed')
    config = json.loads(contents['CONFIG.json'])
    # Compile the verified guard bytes directly: no file re-read or cached bytecode.
    guard_path = ROOT / GUARD
    spec = importlib.util.spec_from_file_location('streaming_host_guard', guard_path)
    guard = importlib.util.module_from_spec(spec)
    exec(compile(contents[GUARD], str(guard_path), 'exec', dont_inherit=True), guard.__dict__)
    guard.REPO = CANDIDATE
    claims = dict(
        observed='Standalone Rust-host streaming API compilation and exactly eight fixed normal tests',
        acceptance='none', native_authority=0,
        constructor_startup='not-executed; Machine::new constructs the post-startup state',
        not_run=['K', 'Quint', 'Lean', 'Clean', 'handwritten-circuit', 'SDK', 'native-ledger'],
        wall_clock_seconds_per_child=30, memory_bound='none-claimed', disk_bound='none-claimed',
        authenticity='local SHA-256 bindings in a trusted checkout only',
        unhashed_inputs=['rustc sysroot (librustc_driver, std)', 'system linker and libc',
                         'Python stdlib', 'kernel'],
        product_wiring='not-run; production mod.rs/lib.rs are not compiled; this harness '
                       'compiles unwired candidate modules by #[path]')
    guard.REALIZATIONS['stream'] = guard.Realization(
        'sherlock-streaming-host-run/1', 'streaming_harness.rs', 'sherlock_streaming',
        None, tuple(config['compile_closure']), tuple(config['expected_tests']),
        tuple(config['snapshot_members']), None, None, None, claims)
    output = ROOT / 'attempt-01'
    output.mkdir(mode=0o700)
    attempt = guard.Attempt(output, RUSTC, 'stream')
    manifest_sha = sha256(manifest_raw)
    attempt.receipt['reviewed_manifest'] = dict(path=MANIFEST, sha256=manifest_sha)
    attempt.receipt['reviewed_candidate'] = before
    plan = [
        ('preflight', attempt.preflight),
        ('rustc-version', attempt.version),
        ('compile-tests', lambda: attempt.compile(
            'sherlock_streaming_tests', tuple(config['compile_closure']), '--test')),
        ('run-tests', attempt.tests),
    ]
    try:
        blocked = None
        for name, action in plan:
            if blocked is None:
                blocked = None if attempt.step(name, action) else name
            else:
                attempt.receipt['steps'].append(dict(
                    name=name, state='not-run', reason=f'dependency {blocked} did not succeed'))
        after = {row['path']: digest(ROOT / row['path']) for row in manifest}
        manifest_after = digest(ROOT / MANIFEST)
        attempt.receipt['reviewed_candidate_after'] = after
        attempt.receipt['reviewed_manifest']['sha256_after'] = manifest_after
        attempt.step('candidate-postflight', lambda: guard.require(
            after == before and manifest_after == manifest_sha,
            'reviewed candidate or manifest changed during the attempt'))
    finally:
        status = attempt.finish()
    return status


if __name__ == '__main__':
    if not (sys.flags.isolated and sys.flags.dont_write_bytecode):
        raise SystemExit('invoke with python3 -I -B')
    raise SystemExit(main())

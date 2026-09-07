#!/usr/bin/env python3
"""One source-reviewed successor attempt. This file is not launch approval.

No native subprocess can start without exact candidate, Fable, GPT-6 and MC01
acceptance receipts. Receipt authenticity is the trusted orchestrator's duty;
JSON files are not signatures or substitutes for the actual auditor outputs.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import selectors
import shutil
import signal
import subprocess
import sys
import time
import tomllib

PIN = '695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7'
SRS_HASH = '4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74'
MEMORY = 8 * 1024**3
SECONDS = 1800
OUTPUT_LIMIT = 256 * 1024**2
LOG_LIMIT = 8 * 1024**2
EXPERIMENT = 'experiments/moriarty-native-ivc-r3'
CHECKS = ['full-relation', 'private-state-recursive-binding', 'unknown-witness-keygen',
          'negative-controls', 'source-pins-lockfile', 'srs', 'resource-envelope']


def sha(data):
    return hashlib.sha256(data).hexdigest()


def encoded(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':')).encode()


def read_regular(path, maximum=64 * 1024**2):
    path = Path(path)
    if path.is_symlink() or not path.is_file() or path.stat().st_size > maximum:
        raise RuntimeError(f'Not a bounded regular input: {path}')
    with path.open('rb') as stream:
        data = stream.read(maximum + 1)
    if len(data) > maximum:
        raise RuntimeError('Input changed beyond byte bound')
    return data


def receipt(path, digest):
    data = read_regular(path, 1024**2)
    if sha(data) != digest:
        raise RuntimeError('Receipt bytes differ from trusted invocation digest')
    return json.loads(data)


def gates(args):
    candidate_bytes = read_regular(args.candidate, 16 * 1024**2)
    candidate = json.loads(candidate_bytes)
    content = {k: v for k, v in candidate.items() if k != 'candidateSha256'}
    if sha(encoded(content)) != args.candidate_sha256 or candidate['candidateSha256'] != args.candidate_sha256:
        raise RuntimeError('Candidate identity mismatch')
    if candidate['nativePin'] != PIN or candidate['srsSha256'] != SRS_HASH:
        raise RuntimeError('Wrong source/SRS pin')
    if candidate['resources'] != {'seconds': SECONDS, 'memoryBytes': MEMORY, 'swapBytes': 0,
       'cpuQuotaPercent': 200, 'jobs': 2, 'k': 17, 'positiveProvingSteps': 2,
       'retainedOutputBytes': OUTPUT_LIMIT, 'automaticRetries': 0}:
        raise RuntimeError('Unreviewed resource envelope')
    for path, digest, model in [(args.fable_audit, args.fable_audit_sha256, 'Fable'),
                               (args.gpt6_audit, args.gpt6_audit_sha256, 'GPT-6')]:
        value = receipt(path, digest)
        required = {'schemaVersion', 'model', 'candidateSha256', 'verdict', 'checkedScopes',
                    'blockingFindings', 'auditorReceiptSha256', 'reviewedAtUtc'}
        if set(value) != required or value['schemaVersion'] != 'moriarty-native-source-audit/1':
            raise RuntimeError('Audit schema mismatch')
        if (value['model'] != model or value['candidateSha256'] != args.candidate_sha256
            or value['verdict'] != 'APPROVED' or value['blockingFindings'] != []
            or value['checkedScopes'] != CHECKS):
            raise RuntimeError('Exact independent implementation/resource audit is not approved')
        # This reference binds the original, unedited auditor output retained by
        # the orchestrator. A digest is provenance binding, not authentication.
        origin = args.fable_original if model == 'Fable' else args.gpt6_original
        if sha(read_regular(origin, 4 * 1024**2)) != value['auditorReceiptSha256']:
            raise RuntimeError('Original auditor output mismatch')
    predecessor = receipt(args.mc01_acceptance, args.mc01_acceptance_sha256)
    if (predecessor.get('package') != 'MC01' or predecessor.get('status') != 'accepted'
        or predecessor.get('sourceProfileOnly') is not False
        or predecessor.get('mandatoryPredicatesComplete') is not True
        or predecessor.get('independentFableApproved') is not True
        or predecessor.get('freshGPT6Approved') is not True):
        raise RuntimeError('Full predecessor MC01 package acceptance is absent')
    root = args.root.resolve()
    for name, digest in candidate['files'].items():
        path = root / name
        if path.resolve().is_relative_to(root) is False or sha(read_regular(path)) != digest:
            raise RuntimeError('Candidate source changed: ' + name)
    runner = root / EXPERIMENT / 'run-checked-encoding.py'
    if sha(read_regular(__file__)) != candidate['files'][str(runner.relative_to(root))]:
        raise RuntimeError('Executing runner differs from reviewed source')
    return candidate


def enforced_limits():
    item = next(x for x in Path('/proc/self/cgroup').read_text().splitlines() if x.startswith('0::'))
    group = Path('/sys/fs/cgroup') / item[3:].lstrip('/')
    values = {name: (group / name).read_text().strip() for name in
              ['memory.max', 'memory.swap.max', 'memory.oom.group', 'cpu.max', 'pids.max']}
    quota, period = values['cpu.max'].split()
    if (values['memory.max'] != str(MEMORY) or values['memory.swap.max'] != '0'
        or values['memory.oom.group'] != '1' or quota == 'max'
        or int(quota) != 2 * int(period) or values['pids.max'] != '128'):
        raise RuntimeError('Required group limits absent')
    return group, values


def checked_copy(source, destination, digest, maximum=64 * 1024**2):
    data = read_regular(source, maximum)
    if sha(data) != digest:
        raise RuntimeError('Input hash mismatch: ' + str(source))
    destination.parent.mkdir(parents=True, exist_ok=True)
    with destination.open('xb') as stream:
        stream.write(data)
    destination.chmod(0o444)
    return len(data)


def prepare(args, candidate, work):
    """Copy pinned inputs; Cargo never compiles a mutable original source tree."""
    source = args.backend.resolve()
    if subprocess.check_output(['git', '-C', str(source), 'rev-parse', 'HEAD'], text=True).strip() != PIN:
        raise RuntimeError('Backend HEAD mismatch')
    repo = work / 'backend'
    for name, digest in candidate['backendFiles'].items():
        checked_copy(source / name, repo / name, digest)
    for original, target in [(EXPERIMENT + '/harness/moriarty_loan_r3.rs', 'aggregation/examples/moriarty_loan_r3.rs'),
                             (EXPERIMENT + '/harness/episode.rs', 'aggregation/examples/moriarty_r3/episode.rs')]:
        checked_copy(args.root / original, repo / target, candidate['files'][original])
    srs = work / 'srs-k17.bin'
    if checked_copy(args.srs, srs, SRS_HASH) != 25166212:
        raise RuntimeError('Wrong SRS size')
    lock = tomllib.loads((repo / 'Cargo.lock').read_text())
    cargo_home = work / 'cargo-home'
    cache_root = args.cargo_cache.resolve()
    registry = 'index.crates.io-1949cf8c6b5b557f'
    # Only content-hashed registry archives are consumed. Existing expanded
    # registry source directories are never copied or trusted.
    for package in lock['package']:
        if package.get('source', '').startswith('registry+'):
            rel = Path('registry/cache') / registry / (package['name'] + '-' + package['version'] + '.crate')
            if str(rel) in candidate['registryArchiveFiles']:
                if candidate['registryArchiveFiles'][str(rel)] != package['checksum']:
                    raise RuntimeError('Archive inventory differs from Cargo.lock')
                checked_copy(cache_root / rel, cargo_home / rel, package['checksum'])
            # Absent archives stay absent in the immutable input inventory.
            # Cargo --offline will reject if the selected target needs them.
    # Index metadata is an offline resolver input; pin its exact bytes in the
    # candidate inventory. --locked prevents choosing another dependency graph.
    for rel, digest in candidate['cargoMetadataFiles'].items():
        checked_copy(cache_root / rel, cargo_home / rel, digest)
    # Git databases are copied as content-addressed objects and checked against
    # both the complete candidate file inventory and Cargo.lock commit identities.
    for item in candidate['gitDependencies']:
        db = cargo_home / item['database']
        actual = subprocess.check_output(['git', '--git-dir', str(db), 'rev-parse', item['commit'] + '^{commit}'], text=True).strip()
        if actual != item['commit']:
            raise RuntimeError('Git dependency commit mismatch')
    toolchain = Path(candidate['toolchainDirectory'])
    for rel, digest in candidate['toolchainFiles'].items():
        if sha(read_regular(toolchain / rel, 512 * 1024**2)) != digest:
            raise RuntimeError('Rust executable changed')
    return repo, srs, cargo_home, toolchain


def inner(args, candidate):
    group, limits = enforced_limits()
    output = args.output.resolve()
    # Wall limit includes verified snapshot preparation, compilation, all
    # exhaustive controls, setup and the two proving steps. No cached target reuse.
    start = time.monotonic()
    work = output / 'disposable'
    work.mkdir()
    (output / 'enforced-limits.json').write_text(json.dumps(limits, indent=2) + '\n')
    repo, srs, cargo_home, toolchain = prepare(args, candidate, work)
    result_dir = output / 'artifacts'; result_dir.mkdir()
    environment = {'PATH': str(toolchain / 'bin') + ':/usr/bin:/bin',
       'HOME': str(Path.home()), 'CARGO_HOME': str(cargo_home), 'CARGO_TARGET_DIR': str(work / 'target'),
       'RUSTC': str(toolchain / 'bin/rustc'), 'RUSTDOC': str(toolchain / 'bin/rustdoc'),
       'CARGO_NET_OFFLINE': 'true', 'CARGO_BUILD_JOBS': '2', 'RAYON_NUM_THREADS': '2',
       'MORIARTY_R3_SRS': str(srs), 'MORIARTY_R3_OUTPUT': str(result_dir),
       'MORIARTY_R3_CANDIDATE_SHA256': args.candidate_sha256}
    command = [str(toolchain / 'bin/cargo'), 'run', '--offline', '--locked', '--release', '--jobs', '2',
       '-p', 'midnight-aggregation', '--features', 'truncated-challenges', '--example', 'moriarty_loan_r3']
    proc = subprocess.Popen(command, cwd=repo, env=environment, stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT, start_new_session=True)
    selector = selectors.DefaultSelector(); selector.register(proc.stdout, selectors.EVENT_READ)
    count = 0; reason = 'process-exit'
    with (output / 'native.stdout.txt').open('xb') as stream:
        while selector.get_map():
            retained = sum(p.stat().st_size for p in result_dir.rglob('*') if p.is_file())
            if time.monotonic() - start > SECONDS - 10 or retained + count >= OUTPUT_LIMIT - LOG_LIMIT or count >= LOG_LIMIT:
                reason = 'wall-limit' if time.monotonic() - start > SECONDS - 10 else 'output-limit'
                os.killpg(proc.pid, signal.SIGTERM)
                try: proc.wait(timeout=3)
                except subprocess.TimeoutExpired: os.killpg(proc.pid, signal.SIGKILL)
                break
            for key, _ in selector.select(timeout=0.25):
                data = os.read(key.fd, min(65536, LOG_LIMIT - count))
                if not data: selector.unregister(key.fileobj)
                else: stream.write(data); stream.flush(); count += len(data)
        code = proc.wait()
    # Every surviving artifact, including the canonical setup VK representation,
    # is bound to this candidate in the terminal inventory, even after failure.
    artifacts = {str(p.relative_to(output)): sha(read_regular(p)) for p in result_dir.rglob('*') if p.is_file()}
    result = {'candidateSha256': args.candidate_sha256, 'exitCode': code, 'reason': reason,
              'elapsedSeconds': time.monotonic() - start, 'command': command, 'artifacts': artifacts,
              'resourceUsage': {name: (group / name).read_text().strip() for name in ['memory.peak', 'memory.events', 'cpu.stat']}}
    (output / 'native-process.json').write_text(json.dumps(result, indent=2) + '\n')
    return code if code else (0 if reason == 'process-exit' else 124)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ['root', 'candidate', 'backend', 'srs', 'cargo-cache', 'output', 'fable-audit', 'fable-original',
                 'gpt6-audit', 'gpt6-original', 'mc01-acceptance']:
        parser.add_argument('--' + name, type=Path, required=True)
    for name in ['candidate-sha256', 'fable-audit-sha256', 'gpt6-audit-sha256', 'mc01-acceptance-sha256']:
        parser.add_argument('--' + name, required=True)
    parser.add_argument('--execute', action='store_true', help='Launch only after every exact gate passes')
    parser.add_argument('--inside-cgroup', action='store_true', help=argparse.SUPPRESS)
    args = parser.parse_args()
    for name, value in vars(args).items():
        if isinstance(value, Path):
            setattr(args, name, value.resolve())
    candidate = gates(args)
    if not args.execute:
        print('All supplied gates checked; no native execution requested.'); return 0
    state_dir = Path.home() / '.local/state/moriarty/mc03-checked-encoding'
    attempt = state_dir / (args.candidate_sha256 + '.json')
    if args.inside_cgroup:
        prior = json.loads(read_regular(attempt))
        if prior['output'] != str(args.output.resolve()):
            raise RuntimeError('Inner invocation does not match consumed attempt')
        return inner(args, candidate)
    if args.output.exists():
        raise RuntimeError('Fresh output required; no overwrite or retry')
    state_dir.mkdir(parents=True, exist_ok=True)
    with attempt.open('x') as stream:
        json.dump({'candidateSha256': args.candidate_sha256, 'output': str(args.output.resolve()),
                   'status': 'attempt-consumed-no-automatic-retry'}, stream)
    args.output.mkdir(parents=True)
    invocation = {name: str(value) if isinstance(value, Path) else value for name, value in vars(args).items()}
    (args.output / 'invocation.json').write_text(json.dumps(invocation, indent=2) + '\n')
    unit = 'moriarty-mc03-encoding-' + args.candidate_sha256[:16]
    command = ['systemd-run', '--user', '--wait', '--pipe', '--unit=' + unit,
       '-p', 'MemoryMax=' + str(MEMORY), '-p', 'MemorySwapMax=0', '-p', 'OOMPolicy=kill',
       '-p', 'RuntimeMaxSec=' + str(SECONDS), '-p', 'KillMode=control-group',
       '-p', 'TasksMax=128', '-p', 'CPUQuota=200%', '-p', 'LimitCORE=0',
       '-p', 'PrivateNetwork=yes', sys.executable, str(Path(__file__).resolve()),
       *[part for name, value in vars(args).items() if name not in {'inside_cgroup', 'execute'}
         for part in ['--' + name.replace('_', '-'), str(value)]], '--execute', '--inside-cgroup']
    started = time.monotonic()
    with (args.output / 'outer.stdout.txt').open('xb') as stream:
        result = subprocess.run(command, stdout=stream, stderr=subprocess.STDOUT)
    # Verified source/dependency/build snapshots are disposable, not retained evidence.
    shutil.rmtree(args.output / 'disposable', ignore_errors=False) if (args.output / 'disposable').exists() else None
    (args.output / 'result.json').write_text(json.dumps({'candidateSha256': args.candidate_sha256,
       'exitCode': result.returncode, 'elapsedSeconds': time.monotonic() - started,
       'nativeReceiptRequiredForSuccess': True, 'command': command}, indent=2) + '\n')
    return result.returncode


if __name__ == '__main__':
    sys.exit(main())

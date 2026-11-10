#!/usr/bin/env python3
"""Resumable equivalent of `cargo test --workspace --locked` for the T1 tier.

The workspace is built once (`cargo test --workspace --no-run`, so feature
unification is the same as one `cargo test --workspace`). Each test binary then
runs as its own process, from its package root with the runtime variables that
cargo sets. Every test that prints `... ok` is recorded at once in a ledger on
disk. The ledger is keyed by the tree under test: HEAD, a hash of
`git diff HEAD`, the untracked file list, and the binary file name (which
changes with the code). A rerun on the same tree skips the tests already
recorded as ok (`--skip NAME --exact`), so a lost or killed run resumes where
it stopped instead of starting again. Doctests run last as one step.

Exit status: 0 when every binary and the doctests pass, 1 on any failure.
"""
import hashlib
import json
import os
import re
import subprocess
import sys
import time
import tomllib
from pathlib import Path

ROOT = Path(subprocess.check_output(['git', 'rev-parse', '--show-toplevel'], text=True).strip())
CACHE = Path(os.environ.get('XDG_CACHE_HOME', Path.home() / '.cache'))
LEDGER_ROOT = Path(os.environ.get('DW_T1_LEDGER', CACHE / 'doctorwatson' / 't1-ledger'))
OK_LINE = re.compile(r'^test (\S+) \.\.\. ok$')


def say(msg):
    print(f'[t1-workspace {time.strftime("%H:%M:%S")}] {msg}', flush=True)


def tree_key():
    head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    h = hashlib.sha256()
    # DoctorWatson rewrites its own status and run records during a run; they are not test inputs.
    h.update(subprocess.check_output(['git', 'diff', 'HEAD', '--binary', '--', '.',
                                      ':(exclude)qa/sherlock/status.json',
                                      ':(exclude)qa/sherlock/runs.jsonl'], cwd=ROOT))
    untracked = subprocess.check_output(
        ['git', 'ls-files', '--others', '--exclude-standard', '--', 'crates', 'Cargo.toml', 'Cargo.lock'],
        cwd=ROOT)
    h.update(untracked)
    for rel in sorted(untracked.decode().split()):
        try:
            h.update((ROOT / rel).read_bytes())
        except OSError:
            pass
    return f'{head[:12]}-{h.hexdigest()[:12]}'


def build():
    """Build every test target once; return [(package, manifest_dir, executable)]."""
    cmd = ['cargo', 'test', '--workspace', '--locked', '--no-run', '--message-format=json-render-diagnostics']
    say('build: ' + ' '.join(cmd))
    proc = subprocess.run(cmd, cwd=ROOT, stdout=subprocess.PIPE, text=True)
    if proc.returncode != 0:
        say(f'build FAILED (exit {proc.returncode})')
        sys.exit(1)
    bins = []
    for line in proc.stdout.splitlines():
        try:
            msg = json.loads(line)
        except ValueError:
            continue
        if msg.get('reason') != 'compiler-artifact' or not msg.get('executable'):
            continue
        if not msg.get('profile', {}).get('test'):
            continue
        manifest = Path(msg['manifest_path']).parent
        try:
            name = tomllib.loads((manifest / 'Cargo.toml').read_text())['package']['name']
        except (OSError, KeyError, ValueError):
            name = manifest.name
        bins.append((name, manifest, msg['executable'], msg['target']['name']))
    return bins


def run_binary(key_dir, pkg, manifest, exe, target):
    tag = Path(exe).name
    ok_file = key_dir / f'{tag}.ok'
    done_file = key_dir / f'{tag}.done'
    if done_file.exists():
        say(f'{pkg}/{target}: PASS (recorded {done_file.read_text().strip()})')
        return True
    done = set(ok_file.read_text().split()) if ok_file.exists() else set()
    args = [exe]
    for name in sorted(done):
        args += ['--skip', name]
    if done:
        args.append('--exact')  # --skip then matches whole test names only
    env = dict(os.environ, CARGO_MANIFEST_DIR=str(manifest), CARGO_PKG_NAME=pkg,
               CARGO_CRATE_NAME=target.replace('-', '_'))
    say(f'{pkg}/{target}: run ({len(done)} tests already ok on this tree)')
    log = key_dir / f'{tag}.log'
    with open(log, 'a') as lf, open(ok_file, 'a') as okf:
        lf.write(f'\n=== run {time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())} skip={len(done)}\n')
        proc = subprocess.Popen(args, cwd=manifest, env=env, stdout=subprocess.PIPE,
                                stderr=subprocess.STDOUT, text=True, bufsize=1)
        for line in proc.stdout:
            sys.stdout.write(line)
            lf.write(line)
            m = OK_LINE.match(line.rstrip('\n'))
            if m:
                okf.write(m.group(1) + '\n')
                okf.flush()
                lf.flush()
        rc = proc.wait()
    if rc == 0:
        done_file.write_text(time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()) + '\n')
        say(f'{pkg}/{target}: PASS')
        return True
    say(f'{pkg}/{target}: FAIL (exit {rc}); log {log}')
    return False


def main():
    key = tree_key()
    key_dir = LEDGER_ROOT / key
    key_dir.mkdir(parents=True, exist_ok=True)
    say(f'ledger {key_dir}')
    bins = build()
    say(f'{len(bins)} test binaries')
    failed = [f'{p}/{t}' for p, m, e, t in bins if not run_binary(key_dir, p, m, e, t)]
    doc_done = key_dir / 'doctests.done'
    if doc_done.exists():
        say('doctests: PASS (recorded)')
    else:
        rc = subprocess.run(['cargo', 'test', '--workspace', '--locked', '--doc'], cwd=ROOT).returncode
        if rc == 0:
            doc_done.write_text(time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()) + '\n')
            say('doctests: PASS')
        else:
            failed.append('doctests')
            say(f'doctests: FAIL (exit {rc})')
    if failed:
        say('FAILED: ' + ', '.join(failed))
        return 1
    say('all test binaries and doctests pass')
    return 0


if __name__ == '__main__':
    sys.exit(main())

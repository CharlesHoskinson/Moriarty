#!/usr/bin/env python3
"""Compile and test arithmetic helpers; does not generate keys or proofs."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
import time

p = argparse.ArgumentParser()
p.add_argument('--runtime-node-modules', type=Path, required=True)
p.add_argument('--output', type=Path, required=True)
args = p.parse_args()
here = Path(__file__).resolve().parent
runtime_modules = args.runtime_node_modules.resolve(strict=True)
runtime_package = runtime_modules / '@midnight-ntwrk/compact-runtime/package.json'
assert json.loads(runtime_package.read_text())['version'] == '0.16.0'
args.output.mkdir(parents=True, exist_ok=True)
commands = []

def run(command, env=None):
    started = time.monotonic()
    result = subprocess.run(command, cwd=here, env=env, capture_output=True,
                            text=True, timeout=90)
    receipt = {'command': command, 'exit_code': result.returncode,
               'elapsed_seconds': time.monotonic()-started,
               'stdout': result.stdout, 'stderr': result.stderr}
    commands.append(receipt)
    (args.output / 'commands.json').write_text(json.dumps(commands, indent=2)+'\n')
    if result.returncode:
        raise RuntimeError(f'{command[0]} failed: {result.stderr or result.stdout}')
    return result.stdout.strip()

assert run(['compact', 'compile', '--version']) == '0.31.1'
assert run(['compact', 'compile', '--language-version']) == '0.23.0'
assert run(['compact', 'compile', '--runtime-version']) == '0.16.0'
with tempfile.TemporaryDirectory(prefix='moriarty-arithmetic-') as temporary:
    paths = [Path(temporary)/'pure', Path(temporary)/'harness']
    for source, target in zip(['arithmetic.compact', 'arithmetic-harness.compact'], paths):
        run(['compact', 'compile', '--skip-zk', source, str(target)])
        (target/'node_modules').symlink_to(runtime_modules, target_is_directory=True)
    env = dict(os.environ, MORIARTY_COMPACT_ARTIFACT=str(paths[0]/'contract/index.js'),
               MORIARTY_COMPACT_HARNESS=str(paths[1]/'contract/index.js'))
    run(['node', '--test', str(here/'arithmetic.test.mjs')], env)
    generated = []
    for target in paths:
        for artifact in sorted(target.rglob('*')):
            if artifact.is_file() and 'node_modules' not in artifact.parts:
                generated.append({'path':str(artifact.relative_to(Path(temporary))),
                                  'bytes':artifact.stat().st_size,
                                  'sha256':hashlib.sha256(artifact.read_bytes()).hexdigest()})
    receipt = {'status':'pass', 'scope':'Compiled Compact runtime arithmetic and ledger-write harness; no native proof, network submission, compiler correspondence or package acceptance',
               'compiler':'0.31.1','language':'0.23.0','runtime':'0.16.0',
               'sources':{f.name:hashlib.sha256(f.read_bytes()).hexdigest()
                          for f in sorted(here.iterdir()) if f.is_file()},
               'generated':generated,'commands_file':'commands.json',
               'keys_generated':False,'proofs_generated':False}
    (args.output/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print(json.dumps({'status':'pass','tests':6,'generated_files':len(generated)}))

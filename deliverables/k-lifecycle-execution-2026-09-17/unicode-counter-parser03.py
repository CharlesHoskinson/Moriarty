#!/usr/bin/env python3
"""krun lexical boundary: pinned KAST JSON-to-KORE conversion only."""
import hashlib
import json
import os
from pathlib import Path
import sys


HERE = Path(__file__).resolve().parents[2] / 'experiments/moriarty-language/formal/k'


def main():
    if len(sys.argv) != 2:
        raise SystemExit(2)
    lock = json.loads((HERE / 'expression-toolchain.lock.json').read_text())
    kast = lock['executables']['kast']
    executable = Path(kast['path'])
    if hashlib.sha256(executable.read_bytes()).hexdigest() != kast['sha256']:
        raise SystemExit('TOOLCHAIN_STALE')
    definition = HERE / '.build-lifecycle-v1' / 'lifecycle-v1-kompiled'
    os.execv(str(executable), [
        str(executable), '--input', 'json', '--output', 'kore', '--sort', 'K',
        '--definition', str(definition), sys.argv[1],
    ])


if __name__ == '__main__':
    main()

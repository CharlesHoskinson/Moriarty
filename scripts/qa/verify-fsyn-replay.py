#!/usr/bin/env python3
"""B2: replay the exact B1 FSYN fixture with the in-repository I1 oracle (i1_replay)."""
import argparse
import json
from pathlib import Path
import shutil
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'models/sherlock/rollup-synth'))
from common import Error, canonical, digest, load_json
from s0 import generate

REQUIRED = ('PASS i1 oracle replay (300 headers, 96 transactions)',
            'PASS batch chain: 6 batches bound to the replayed states and record hashes')
MUTATIONS = (('expected-state', 'expected/000004.json', 'oracle disagrees with expected/000004.json'),
             ('transaction-signature', 'blocks/000004.json', 'oracle header 4: BAD_SIG'))


def run_oracle(binary, corpus):
    return subprocess.run([str(binary), str(corpus)], capture_output=True, text=True, timeout=600)


def mutate(corpus, name, member):
    path = corpus / 'i1' / member
    doc = load_json(path)
    if name == 'expected-state':
        doc['state_after']['fee_total'] += 1
    else:
        doc['txs'][0]['public']['signature']['s'] = '00' * 32
    path.write_bytes(canonical(doc) + b'\n')


def verify_replay(binary, output):
    output = Path(output)
    output.mkdir(parents=True, exist_ok=False)
    fixture = output / 'fixture'
    created = generate(fixture)
    baseline = load_json(ROOT / 'models/sherlock/rollup-synth/baseline.json')
    if digest((fixture / 'MANIFEST.json').read_bytes()) != baseline['manifest_sha256']:
        raise Error('B2 requires the exact pinned B1 fixture manifest')
    corpus = fixture / 'corpus'
    start = time.monotonic()
    result = run_oracle(binary, corpus)
    elapsed = time.monotonic() - start
    (output / 'honest.log').write_text(result.stdout + result.stderr)
    if result.returncode or any(line not in result.stdout for line in REQUIRED):
        raise Error('oracle replay failed; see ' + str(output / 'honest.log'))
    print(result.stdout, end='')
    mutations = []
    for name, member, expected in MUTATIONS:
        mutated = output / name
        shutil.copytree(corpus, mutated)
        mutate(mutated, name, member)
        result = run_oracle(binary, mutated)
        (output / (name + '.log')).write_text(result.stdout + result.stderr)
        if result.returncode == 0 or expected not in result.stderr:
            raise Error('semantic mutation did not reach its expected rejecting oracle: ' + name)
        mutations.append({'case': name, 'exit': result.returncode, 'reason': expected})
        print('PASS semantic mutation ' + name + ' rejected')
    receipt = {'schema': 'sherlock-b2-replay/2', 'fixture_id': created['fixture_id'],
               'fixture_manifest_sha256': baseline['manifest_sha256'],
               'golden_digest': baseline['golden_digest'],
               'oracle': 'crates/mori-recursion/src/rollup/i1 via examples/i1_replay.rs',
               'oracle_binary_sha256': digest(Path(binary).read_bytes()),
               'headers': 300, 'transactions': 96, 'batches': 6,
               'honest_replay_wall_seconds': elapsed, 'semantic_mutations': mutations,
               'scope': ('in-repository I1 state/Poseidon/Merkle/signature replay against the frozen expected states '
                         'and batch-chain binding; the oracle is Moriarty code shared with the Sherlock witness '
                         'builder, not a separate implementation; envelope encoding is checked by B1; '
                         'no recursive proof or ledger acceptance')}
    (output / 'receipt.json').write_bytes(canonical(receipt) + b'\n')
    print(json.dumps(receipt, sort_keys=True))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--out', required=True)
    args = parser.parse_args()
    try:
        verify_replay(args.binary.resolve(), args.out)
    except (Error, OSError, KeyError, subprocess.TimeoutExpired) as exc:
        print(str(exc), file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    raise SystemExit(main())

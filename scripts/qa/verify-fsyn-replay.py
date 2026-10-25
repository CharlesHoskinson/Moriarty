#!/usr/bin/env python3
"""B2: replay the exact B1 corpus with a pinned, separate Rust implementation."""
import argparse
import json
from pathlib import Path
import shutil
import subprocess
import sys
import time

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'models/sherlock/rollup-synth'))
from common import Error, canonical, digest, load_json
from s0 import generate


def run_oracle(binary, corpus):
    # The historical deployment PIN-SET points at absent old home paths and keys.
    # B1 pins data; replay-cut.json pins the compiled oracle instead. No deployment
    # or proof acceptance claim follows from this dataset-only replay.
    return subprocess.run([str(binary), 'verify', '--dir', str(corpus), '--no-pins'],
                          capture_output=True, text=True, timeout=600)


def repair_manifests(corpus, member):
    """Keep byte manifests valid so a semantic mutation reaches the Rust oracle."""
    i1 = load_json(corpus / 'i1/MANIFEST.json')
    i1[member] = digest((corpus / 'i1' / member).read_bytes())
    (corpus / 'i1/MANIFEST.json').write_bytes(canonical(i1) + b'\n')
    manifest = load_json(corpus / 'MANIFEST.json')
    for name in ('i1/' + member, 'i1/MANIFEST.json'):
        manifest[name] = digest((corpus / name).read_bytes())
    (corpus / 'MANIFEST.json').write_bytes(canonical(manifest) + b'\n')


def verify_replay(binary, cut, golden, output):
    output = Path(output)
    output.mkdir(parents=True, exist_ok=False)
    fixture = output / 'fixture'
    created = generate(cut, golden, fixture)
    baseline = load_json(Path(__file__).resolve().parents[2] / 'models/sherlock/rollup-synth/baseline.json')
    if digest((fixture / 'MANIFEST.json').read_bytes()) != baseline['manifest_sha256']:
        raise Error('B2 requires the exact pinned B1 fixture manifest')
    corpus = fixture / 'corpus'
    start = time.monotonic()
    result = run_oracle(binary, corpus)
    (output / 'honest.log').write_text(result.stdout + result.stderr)
    required = ('PASS i1 oracle replay (300 headers, 96 transactions)',
                'PASS envelopes: 6 profile A + 6 profile B + minimal',
                'PASS profile B slots recomputed from i1',
                'PASS anchor model settles profile A',
                'PASS anchor model settles profile B')
    if result.returncode or any(line not in result.stdout for line in required):
        raise Error('independent replay failed; see ' + str(output / 'honest.log'))
    elapsed = time.monotonic() - start
    print(result.stdout, end='')
    mutations = []
    for name, member in (('expected-state', 'expected/000004.json'),
                         ('transaction-signature', 'blocks/000004.json')):
        mutated = output / name
        shutil.copytree(corpus, mutated)
        path = mutated / 'i1' / member
        doc = load_json(path)
        if name == 'expected-state':
            doc['state_after']['fee_total'] += 1
            expected = 'oracle disagrees with expected/000004.json'
        else:
            doc['txs'][0]['public']['signature']['s'] = '00' * 32
            expected = 'oracle header 4: BadSig'
        path.write_bytes(canonical(doc) + b'\n')
        repair_manifests(mutated, member)
        result = run_oracle(binary, mutated)
        (output / (name + '.log')).write_text(result.stdout + result.stderr)
        if result.returncode == 0 or expected not in result.stderr:
            raise Error('semantic mutation did not reach its expected rejecting oracle: ' + name)
        mutations.append({'case': name, 'exit': result.returncode, 'reason': expected})
        print('PASS semantic mutation ' + name + ' rejected after repairing manifests')
    receipt = {'schema': 'sherlock-b2-replay/1', 'fixture_id': created['fixture_id'],
               'fixture_manifest_sha256': baseline['manifest_sha256'],
               'oracle_binary_sha256': digest(Path(binary).read_bytes()),
               'oracle_source_digest': load_json(Path(__file__).resolve().parents[2] /
                   'models/sherlock/rollup-synth/replay-cut.json')['content_digest'],
               'headers': 300, 'transactions': 96, 'batches': 6,
               'profiles': ['A', 'B'], 'minimal_transactions': 2,
               'honest_replay_wall_seconds': elapsed, 'semantic_mutations': mutations,
               'historical_deployment_pins': 'not-checked; data and oracle have separate pinned manifests',
               'scope': 'independent state/Poseidon/Merkle/signature replay and envelope binding; no recursive proof or ledger acceptance'}
    (output / 'receipt.json').write_bytes(canonical(receipt) + b'\n')
    print(json.dumps(receipt, sort_keys=True))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--cut', required=True)
    parser.add_argument('--golden-root', required=True)
    parser.add_argument('--out', required=True)
    args = parser.parse_args()
    try:
        verify_replay(args.binary.resolve(), args.cut, args.golden_root, args.out)
    except (Error, OSError, KeyError, subprocess.TimeoutExpired) as exc:
        print(str(exc), file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    raise SystemExit(main())

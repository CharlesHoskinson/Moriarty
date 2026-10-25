#!/usr/bin/env python3
"""Read actual independent two/four/six-prefix executions and preserve raw scopes."""
import argparse
import importlib.util
import json
from pathlib import Path

SPEC = importlib.util.spec_from_file_location('evidence', Path(__file__).with_name('check-rollup-evidence.py'))
evidence = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(evidence)


def check(root):
    root = Path(root)
    workload = evidence.read(root / 'workload.json', 'sherlock-b5-workload/1')
    if (workload['prefixes'] != [2, 4, 6] or workload['status'] != 'completed'
            or workload['transactions_per_batch'] != 16 or workload['headers_per_batch'] != 50
            or workload['samples_per_prefix'] != 1 or workload['proof_reuse_between_prefixes']):
        raise ValueError('requires three completed independent exact prefix workloads')
    build = evidence.read(root / 'build.json', 'sherlock-b3-build/1')
    if build['prover'] != 'midnight-proofs 0.8.3 registry stock' or build['adapter_version'] != 'sherlock-v3-batch/2':
        raise ValueError('prefix prover/adapter differs from retained stock baseline')
    evidence.require_digest(root / 'rollup.bin', build['binary_sha256'])
    baseline = json.loads((Path(__file__).parents[2] / 'models/sherlock/rollup-synth/baseline.json').read_bytes())
    evidence.require_digest(root / 'fixture/MANIFEST.json', baseline['manifest_sha256'])
    summaries = {}
    fixed = {}
    for count in (2, 4, 6):
        prefix = root / f'prefix-{count}'
        measurements = {}
        for stage in ('leaf', 'parent', 'decider', 'verify'):
            measurement = evidence.read(prefix / f'{stage}.measurement.json', 'sherlock-rollup-measurement/1')
            if measurement['exit'] != 0 or measurement['max_rss_bytes'] <= 0 or measurement['swap_samples'] < 1:
                raise ValueError('failed or unmeasured prefix stage')
            measurements[stage] = measurement
        batches = []
        previous = None
        for batch in range(1, count + 1):
            directory = prefix / f'batch-{batch:02}'
            verified = evidence.read(directory / 'verified.json', 'sherlock-b3-verified/1')
            if not (verified['stock_verification'] and verified['independent_key_reconstruction']
                    and verified['batch'] == batch and verified['transactions'] == 16 and verified['headers'] == 50
                    and verified['layers'] == ['leaf', 'parent', 'decider']
                    and verified.get('shared_key_reconstruction_reused') == (batch > 1)):
                raise ValueError('missing exact independent stock verification')
            fields = json.loads((directory / 'leaf.instance.json').read_bytes())
            parent = json.loads((directory / 'parent.instance.json').read_bytes())
            if len(fields) != 17 or len(parent) != 29 or fields != parent[:17]:
                raise ValueError('prefix leaf/parent statement differs')
            if previous is not None and any(fields[a] != previous[b] for a, b in ((0, 1), (5, 6), (7, 8))):
                raise ValueError('prefix state/deposit chain is broken')
            as_int = lambda value: int.from_bytes(bytes.fromhex(value), 'little')
            if as_int(fields[15]) != (batch - 1) * 50 + 1 or as_int(fields[16]) != batch * 50:
                raise ValueError('prefix header window differs')
            previous = fields
            layers = {}
            for layer in ('leaf', 'parent', 'decider'):
                stats = evidence.read(directory / f'{layer}.stats.json', 'sherlock-b3-stage/1')
                if (stats['prover'] != build['prover'] or stats['layer'] != layer
                        or stats['shared_keygen_proofs'] != count or stats['negative_changed_instance'] != 'rejected'):
                    raise ValueError('incorrect shared-key proof stage')
                for suffix, name in (('vk', 'vk'), ('relation', 'relation'), ('proof', 'proof'), ('instance.json', 'instance')):
                    path = directory / f'{layer}.{suffix}'
                    evidence.require_digest(path, stats[name + '_sha256'])
                    if name in ('vk', 'relation'):
                        expected = fixed.setdefault((layer, suffix), path.read_bytes())
                        if path.read_bytes() != expected:
                            raise ValueError('prefixes changed the fixed relation or verifier key')
                if stats['prove_seconds'] <= 0 or stats['local_verify_seconds'] <= 0:
                    raise ValueError('missing actual proof timing')
                layers[layer] = stats
            if len(json.loads((directory / 'decider.instance.json').read_bytes())) != 30:
                raise ValueError('prefix terminal template differs from the replay-check anchor')
            batches.append({'batch': batch, 'layers': layers, 'verified': verified})
        summaries[str(count)] = {'batches': count, 'transactions': count * 16, 'headers': count * 50,
            'samples': 1, 'measured': measurements, 'artifacts': batches,
            'shape': 'one whole-batch child, single-child verifier parent and terminal per batch; public state chain',
            'shared_keygen': 'one PK per layer within this prefix; rebuilt for each independent prefix',
            'wall_seconds': sum(m['wall_seconds'] for m in measurements.values()),
            'wall_scope': 'sum of four measured sequential child processes; wrapper/file-copy overhead excluded'}
    return {'predicate': 'B5', 'schema': 'sherlock-b5-results/1', 'workload': workload,
            'build': build, 'prefixes': summaries, 'censored_attempts': 'retained in sibling run directories and factory runs.jsonl'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('run', type=Path)
    args = parser.parse_args()
    try:
        result = check(args.run)
    except (OSError, ValueError, KeyError) as error:
        parser.exit(1, f'B5 FAIL: {error}\n')
    (args.run / 'B5.json').write_text(json.dumps(result, indent=2, sort_keys=True) + '\n')
    print('PASS B5: independent 2/4/6-batch runs, exact stock-verified statements and fixed keys')


if __name__ == '__main__':
    main()

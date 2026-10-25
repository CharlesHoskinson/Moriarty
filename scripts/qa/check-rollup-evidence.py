#!/usr/bin/env python3
"""Consume actual B3 artifacts for the B3/B6/B7 predicates; never create a verdict from plans."""
import argparse
import hashlib
import json
from pathlib import Path


def digest(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def require_digest(path, expected):
    if digest(path) != expected:
        raise ValueError(f'artifact digest mismatch: {path}')


def read(path, schema):
    value = json.loads(Path(path).read_bytes())
    if value.get('schema') != schema:
        raise ValueError(f'incorrect evidence schema: {path}')
    return value


def check(kind, root):
    root = Path(root)
    build = read(root / 'build.json', 'sherlock-b3-build/1')
    if not build.get('rustc') or build.get('prover') != 'midnight-proofs 0.8.3 registry stock':
        raise ValueError('requires actual compiler and stock prover identity')
    verified = read(root / 'verified.json', 'sherlock-b3-verified/1')
    if not (verified.get('stock_verification') and verified.get('independent_key_reconstruction')
            and verified.get('transactions') == 16 and verified.get('headers') == 50
            and verified.get('batch') == 1 and verified.get('layers') == ['leaf', 'parent', 'decider']):
        raise ValueError('requires the exact stock-verified B3 workload and independent key reconstruction')
    if (root / 'fixture/MANIFEST.json').exists():
        baseline = json.loads((Path(__file__).parents[2] / 'models/sherlock/rollup-synth/baseline.json').read_bytes())
        require_digest(root / 'fixture/MANIFEST.json', baseline['manifest_sha256'])
    else:
        raise ValueError('requires the exact B1 fixture manifest')
    layers = {}
    for layer in ('leaf', 'parent', 'decider'):
        stats = read(root / f'{layer}.stats.json', 'sherlock-b3-stage/1')
        if stats.get('layer') != layer or stats.get('prover') != 'midnight-proofs 0.8.3 registry stock':
            raise ValueError('prover/layer identity mismatch')
        for suffix, field in [('vk', 'vk'), ('proof', 'proof'), ('relation', 'relation'), ('instance.json', 'instance')]:
            path = root / f'{layer}.{suffix}'
            require_digest(path, stats[field + '_sha256'])
            if field in ('vk', 'proof') and path.stat().st_size != stats[field + '_bytes']:
                raise ValueError('artifact size mismatch')
        if stats.get('negative_changed_instance') != 'rejected':
            raise ValueError('missing negative verification')
        model = stats['model']
        if not (0 < model['k'] <= 20 and stats['domain_capacity'] == 1 << model['k']
                and 0 < model['rows'] < stats['domain_capacity'] and model['table_rows'] > 0
                and model['advice_columns'] > 0 and model['fixed_columns'] > 0 and model['lookups'] > 0):
            raise ValueError('invalid/missing actual circuit layout')
        for field in ('model_seconds', 'parameter_read_downsize_seconds', 'vk_keygen_seconds',
                      'pk_keygen_seconds', 'prove_seconds', 'local_verify_seconds', 'process_wall_seconds',
                      'pk_raw_bytes', 'peak_rss_bytes'):
            if stats[field] <= 0:
                raise ValueError('missing phase/size measurement: ' + field)
        layers[layer] = stats
    measurements = {}
    for stage in ('mock', 'leaf', 'parent', 'decider', 'verify'):
        measurement = read(root / f'{stage}.measurement.json', 'sherlock-rollup-measurement/1')
        if measurement['exit'] != 0 or measurement['max_rss_bytes'] <= 0 or measurement['swap_samples'] < 1:
            raise ValueError('unsuccessful/unmeasured stage: ' + stage)
        measurements[stage] = measurement
        # Retain the measured executable so later builds cannot invalidate or
        # silently substitute the binary behind an earlier run's evidence.
        require_digest(root / 'rollup.bin', build['binary_sha256'])
    if kind == 'B7':
        for stage, measurement in measurements.items():
            if measurement['max_rss_bytes'] >= 16_000_000_000 or measurement['swap_peak_bytes'] != 0:
                raise ValueError(f'{stage} exceeds decimal 16 GB or used swap')
        for layer, stats in layers.items():
            if stats['peak_rss_bytes'] >= 16_000_000_000:
                raise ValueError(f'{layer} process high-water RSS exceeds decimal 16 GB')
    return {'predicate': kind, 'build': build,
            'recursive_shape': 'one 16-transaction/50-header leaf; one single-child verifier parent; one terminal decider per batch',
            'timing_scopes': {'phase_wall_seconds': 'isolated timers for parameters, VK, PK, proof and verification',
                              'process_wall_seconds_in_stats': 'prove_stage only; corpus replay/witness preparation precede this timer',
                              'measurement_wall_seconds': 'entire child process, including corpus replay and witness preparation',
                              'phase_cpu_seconds': 'unavailable; user/system CPU measured for each complete child process',
                              'cache_state': 'shared-host OS cache; not controlled or classified as cold'},
            'layers': {name: {'k': value['model']['k'], 'used_rows': value['model']['rows'],
            'lookup_table_rows': value['model']['table_rows'], 'advice_columns': value['model']['advice_columns'],
            'lookup_query_rows': 'unavailable; library exposes table_rows only',
            'fixed_columns': value['model']['fixed_columns'], 'proof_bytes': value['proof_bytes'],
            'vk_bytes': value['vk_bytes'], 'pk_raw_bytes': value['pk_raw_bytes'],
            'instance_fields': len(json.loads((root / f'{name}.instance.json').read_bytes())),
            'prove_seconds': value['prove_seconds'], 'verify_seconds': value['local_verify_seconds']}
            for name, value in layers.items()}, 'measured': measurements, 'verified': verified}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('predicate', choices=('B3', 'B6', 'B7'))
    parser.add_argument('run', type=Path, nargs='?')
    args = parser.parse_args()
    root = args.run
    if root is None:
        candidates = list(Path('target/sherlock-b3').glob('run-*/verified.json'))
        if not candidates:
            parser.error('no completed B3 run')
        root = max(candidates, key=lambda p: p.stat().st_mtime_ns).parent
    try:
        result = check(args.predicate, root)
    except (ValueError, KeyError, OSError) as exc:
        parser.exit(1, f'{args.predicate} FAIL: {exc}\n')
    (root / f'{args.predicate}.json').write_text(json.dumps(result, indent=2, sort_keys=True) + '\n')
    print(f"PASS {args.predicate}: stock leaf/parent/decider verified; " +
          ', '.join(f'{name} k{value["k"]}, {value["used_rows"]} used rows' for name, value in result['layers'].items()))


if __name__ == '__main__':
    main()

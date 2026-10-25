#!/usr/bin/env python3
"""B9 scoped feasibility report, generated only from successful measured artifacts."""
import argparse
import importlib.util
import json
from pathlib import Path
import tempfile

ROOT = Path(__file__).resolve().parents[2]


def module(name, filename):
    spec = importlib.util.spec_from_file_location(name, ROOT / 'scripts/qa' / filename)
    loaded = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(loaded)
    return loaded


def check():
    status = json.loads((ROOT / 'qa/sherlock/status.json').read_bytes())
    required = ('B3', 'B4', 'B5', 'B6', 'B7', 'B8')
    missing = [key for key in required if not status['predicates'].get(key, {}).get('passes')]
    if missing:
        raise ValueError('missing verified predicates: ' + ', '.join(missing))
    evidence = module('first_batch_evidence', 'check-rollup-evidence.py')
    prefix = module('prefix_evidence', 'check-prefix-evidence.py')
    performance = module('stagenet_performance', 'check-rollup-performance.py')
    b3 = evidence.latest_run('B3', ROOT / 'target')
    first_geometry = evidence.check('B6', b3)
    first_resources = evidence.check('B7', evidence.latest_run('B7', ROOT / 'target'))
    attempts = list((ROOT / 'target/sherlock-b5').glob('run-*/workload.json'))
    if not attempts:
        raise ValueError('no actual prefix benchmark')
    prefix_root = max(attempts, key=lambda path: path.stat().st_mtime_ns).parent
    prefixes = prefix.check(prefix_root)
    network = performance.check(ROOT / 'target')
    b4 = json.loads(Path(network['b4_source']).read_bytes())
    failures = []
    for line in (ROOT / 'qa/sherlock/runs.jsonl').read_text().splitlines():
        run = json.loads(line)
        if run['id'].startswith('B') and run.get('exit') != 0:
            failures.append(run)
    baseline = json.loads((ROOT / 'models/sherlock/rollup-synth/baseline.json').read_bytes())
    return {'schema': 'sherlock-b9-feasibility/1', 'predicate': 'B9',
        'scope': 'S0 synthetic Fenrir v3 rollup experiment; not an unrestricted Sherlock release claim',
        'workload': {'transactions_per_batch': 16, 'headers_per_batch': 50,
            'distinct_corpus_batches': 6, 'distinct_corpus_transactions': 96, 'distinct_corpus_headers': 300,
            'prefix_lengths': [2, 4, 6], 'samples_per_prefix_length': 1,
            'independent_workload_executions': 3, 'independent_history_samples': 1,
            'executed_prefix_batches': 12, 'executed_prefix_transactions': 192, 'executed_prefix_headers': 600,
            'shape': 'one whole-batch child, one single-child verifier parent and one terminal per batch; public state chain'},
        'dataset_baseline': baseline, 'first_batch_geometry': first_geometry,
        'first_batch_resources': first_resources, 'prefix_results': prefixes,
        'stagenet': b4, 'stagenet_performance': network,
        'network_transaction_scope': 'deployment, proof acceptance and settlement are three native transactions carrying one synthetic 16-transaction batch',
        'synthetic_fee_scope': 'the proved fee totals are synthetic corpus state; network fees are separate SDK/indexer receipt fields',
        'failures_and_censored_runs': failures, 'factory_snapshot': status,
        'remaining_gaps': [
            'One deterministic corpus and one sample at each prefix length; no latency distribution is established.',
            'Fixed 16-transaction/50-header batches, era zero and at most 512 initial commitments in the fixed-key adapter.',
            'The anchor records synthetic state observations; escrow and external event authentication are outside this experiment.',
            'OS cache state is uncontrolled; phase CPU, lookup query rows and validator cryptographic execution time are unavailable.',
            'Other Sherlock requirements retain their independent factory status; this experiment does not close them.']}


def render(report):
    rows = ['# Synthetic rollup feasibility', '', report['scope'], '',
            'One deterministic history contains 6 batches, 96 transactions and 300 headers. '
            'Independent executions of its 2/4/6-batch prefixes each have one measured sample. '
            'They execute 192 transactions in total, including repeated prefix data.', '',
            '| Prefix batches | Transactions | Headers | Measured process wall (s) |',
            '|---:|---:|---:|---:|']
    for count, result in report['prefix_results']['prefixes'].items():
        rows.append(f'| {count} | {result["transactions"]} | {result["headers"]} | {result["wall_seconds"]:.3f} |')
    rows += ['', 'Memory is evaluated against the owner’s 2026-10-25 decimal 32 GB cap, with swap zero. '
             'The earlier 16 GB failures remain in the raw history.', '',
             'The JSON report retains per-layer k, used/table rows, columns, key/proof bytes, phase timers, '
             'process CPU/RSS, exact prover/build identity, public receipts and complete readback. '
             'Network fees, finalization and readback latency have separate scopes from local cryptographic verification. '
             'Validator cryptographic execution time is unavailable.', '', 'Remaining gaps:', '']
    rows.extend('- ' + gap for gap in report['remaining_gaps'])
    rows += ['', f'Retained failed/censored factory runs: {len(report["failures_and_censored_runs"])}.', '']
    return '\n'.join(rows)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.parse_args()
    try:
        report = check()
    except (OSError, ValueError, KeyError, ArithmeticError) as error:
        parser.exit(1, f'B9 FAIL: {error}\n')
    reports = ROOT / 'target/sherlock-b9'
    reports.mkdir(parents=True, exist_ok=True)
    destination = Path(tempfile.mkdtemp(prefix='run-', dir=reports))
    (destination / 'B9.json').write_text(json.dumps(report, indent=2, sort_keys=True) + '\n')
    (destination / 'REPORT.md').write_text(render(report))
    print('PASS B9: scoped feasibility report generated from actual complete measurements:', destination)


if __name__ == '__main__':
    main()

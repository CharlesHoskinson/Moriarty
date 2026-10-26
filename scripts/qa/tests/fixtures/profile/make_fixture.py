"""Regenerate the public synthetic accounting fixture, without running a prover."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[5]
schema = json.loads((ROOT / 'openspec/changes/recursion-phase-profile/profile.schema.json').read_text())


def blank(node):
    if '$ref' in node:
        return blank(schema['$defs'][node['$ref'].split('/')[-1]])
    if 'const' in node:
        return node['const']
    if 'enum' in node:
        return None if None in node['enum'] else node['enum'][0]
    kind = node.get('type')
    if isinstance(kind, list):
        return None if 'null' in kind else blank(dict(node, type=kind[0]))
    if kind == 'object':
        return {k: blank(node['properties'][k]) for k in node['required']}
    if kind == 'array':
        return []
    if kind == 'string':
        return '0' * 64 if 'pattern' in node else 'synthetic'
    if kind == 'boolean':
        return False
    return 0


def coverage(total, owners):
    return dict(measured_wall_seconds=total, accounted_exclusive_wall_seconds=total,
                unattributed_wall_seconds=0, unexplained_other_seconds=0,
                overlap_error_seconds=0, relative_sum_error=0, accepted=True,
                owner_shares=[dict(owner=k, exclusive_seconds=v,
                                   fraction_of_named_scope=v / total) for k, v in owners.items()])


def phase(name, start, end, owner, parent, accounting):
    p = blank(schema['$defs']['phase'])
    p.update(span_id=name, parent_span_id=parent, invocation_id='synthetic-command',
             phase=name, owner=owner, accounting=accounting, start_monotonic_ns=start * 10**9,
             end_monotonic_ns=end * 10**9, completed=True, status='ok', reason=None)
    m = p['metrics']
    m.update(wall_seconds=end-start, exclusive_wall_seconds=end-start if accounting else None,
             cpu_core_seconds=2*(end-start), exclusive_cpu_core_seconds=2*(end-start) if accounting else None,
             cpu_scope='isolated_process_all_threads', cpu_null_reason=None)
    m['rss'].update(entry_bytes=10, exit_bytes=8, sampled_peak_bytes=10, peak_delta_bytes=0,
                    net_delta_bytes=-2, samples=0, max_sample_gap_seconds=None,
                    method='proc_status', quality='boundary_only', reason='No interior samples')
    m['bytes'].update(logical_coverage='unavailable', storage_method='unavailable',
                      dram_method='unavailable', device_method='cpu_only',
                      null_reasons=['Counters unavailable in synthetic fixture'], scope='exclusive' if accounting else 'diagnostic')
    m['device'].update(status='not_applicable', reason='CPU fixture', peak_method='not_applicable')
    return p


s = blank(schema)
s.update(run='synthetic', id='B3', invocation_id='synthetic-command', cmd='synthetic only',
         exit=0, recorded_at='2026-10-26T00:00:00Z')
s['identity'].update(resolved_packages=[dict(name='midnight-proofs', version='0.8.3', source='synthetic', checksum=None, features=[])],
                     profile_enabled=True)
s['summary'].update(wall_seconds=120, cpu_core_seconds=240, profile_valid=True,
                    coverage=coverage(120, dict(command=20, quotient=50, multiopen=50)))
s['overhead'].update(qualification='inconclusive', pair_count=5, estimated_fraction=0.005,
                     paired_wall_ratio_upper95=1.015, event_capacity=100, event_count=7)
s['lifecycle']['completion_state'] = 'locally_verified'
s['verification'].update(stock_verifier='pass', negative_controls='pass',
                          official_srs_authenticated=True, terminal_blake2b_preserved=True, protocol_unchanged=True)
s['phases'] = [phase('command', 0, 120, 'command', None, False),
               phase('compile', 0, 10, 'command', 'command', True),
               phase('proof', 10, 110, 'other', 'command', False),
               phase('quotient', 10, 60, 'quotient', 'proof', True),
               phase('multiopen', 60, 110, 'multiopen', 'proof', True),
               phase('verify', 110, 120, 'command', 'command', True),
               phase('worker_ntt', 10, 60, 'quotient', 'quotient', False)]
p = blank(schema['$defs']['proof'])
p.update(proof_id='P', shape='P', k=20, root_span_id='proof',
         coverage=coverage(100, dict(quotient=50, multiopen=50)))
s['proofs'] = [p]
Path(__file__).with_name('valid.json').write_text(json.dumps(s, indent=2) + '\n')

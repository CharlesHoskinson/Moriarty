import copy
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch
from test_profile_identity import ROOT, load

FIXTURE = ROOT / 'scripts/qa/tests/fixtures/profile/valid.json'


class ReduceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.profile = load('profile-recursion')

    def setUp(self):
        self.data = json.loads(FIXTURE.read_text())

    def reject(self, text):
        with self.assertRaisesRegex(ValueError, text):
            self.profile.validate_sidecar(self.data)

    def test_parallel_diagnostics_do_not_double_count(self):
        report = self.profile.validate_sidecar(self.data)
        proof = report['proofs'][0]
        self.assertEqual(proof['total_ns'], 100_000_000_000)
        self.assertEqual(sum(p['wall_ns'] for p in proof['partition']), proof['total_ns'])
        self.assertEqual(proof['cpu_core_ns'], 200_000_000_000)
        self.assertNotIn('worker_ntt', [p['span_id'] for p in proof['partition']])

    def test_command_includes_wrappers(self):
        report = self.profile.validate_sidecar(self.data)
        self.assertEqual(report['command']['total_ns'], 120_000_000_000)
        self.assertIn('compile', [p['span_id'] for p in report['command']['partition']])

    def test_seven_percent_discrepancy(self):
        self.data['proofs'][0]['coverage']['measured_wall_seconds'] = 107
        self.reject('coverage|measured')

    def test_exact_integer_bounds(self):
        f = self.profile.coverage_bounds
        self.assertTrue(f(100, 95, 0))
        self.assertFalse(f(100, 94, 0))
        self.assertTrue(f(100, 95, 5))
        self.assertFalse(f(100, 94, 6))
        self.assertFalse(f(0, 0, 0))
        self.assertFalse(f(100, 100, -1))
        self.assertFalse(f(10**20, 95*10**18-1, 0))

    def test_exact_reconciliation_even_within_five_percent(self):
        self.data['phases'][4]['end_monotonic_ns'] -= 1_000_000_000
        self.data['phases'][4]['metrics']['wall_seconds'] = 49
        self.data['phases'][4]['metrics']['exclusive_wall_seconds'] = 49
        self.reject('reconcil|accounted')

    def test_duplicate_interval_identity(self):
        self.data['phases'].append(copy.deepcopy(self.data['phases'][-1]))
        self.reject('duplicate')

    def test_exclusive_overlap(self):
        p = self.data['phases'][-1]
        p['accounting'] = True
        p['metrics']['exclusive_wall_seconds'] = 50
        p['metrics']['exclusive_cpu_core_seconds'] = 100
        self.reject('overlap')

    def test_boundary_only_and_peak_delta(self):
        report = self.profile.validate_sidecar(self.data)
        self.assertFalse(report['instantaneous_memory_certified'])
        self.data['phases'][3]['metrics']['rss']['peak_delta_bytes'] = 2
        self.reject('peak')

    def test_false_sample_quality(self):
        self.data['phases'][3]['metrics']['rss']['quality'] = 'sampled_lower_bound'
        self.reject('quality')

    def test_dram_absent_is_unsupported(self):
        self.assertFalse(self.profile.validate_sidecar(self.data)['bandwidth_supported'])
        self.data['phases'][3]['metrics']['bytes']['dram_read_bytes'] = 0
        self.reject('unavailable|DRAM')

    def test_absence_needs_reason(self):
        self.data['phases'][3]['metrics']['bytes']['null_reasons'] = []
        self.reject('reason')

    def test_noise_cannot_pass(self):
        self.profile.validate_sidecar(self.data)
        self.data['overhead']['qualification'] = 'pass'
        self.reject('overhead')

    def test_upper_bound_and_pair_edges(self):
        for pairs, upper, verdict, valid in [(5, 1.01, 'pass', True), (4, 1.01, 'pass', False),
                                              (5, 1.010000001, 'pass', False),
                                              (5, 1.01, 'inconclusive', False)]:
            with self.subTest(pairs=pairs, upper=upper, verdict=verdict):
                self.data['overhead'].update(pair_count=pairs, paired_wall_ratio_upper95=upper, qualification=verdict)
                if valid:
                    self.profile.validate_sidecar(self.data)
                else:
                    self.reject('overhead')

    def test_full_and_dropped_buffer_invalid(self):
        for count, dropped in [(100, 0), (100, 1), (7, 1), (101, 0)]:
            with self.subTest(count=count, dropped=dropped):
                self.data['overhead'].update(event_count=count, dropped_events=dropped)
                self.reject('buffer|dropped')

    def test_failed_incomplete_and_invalid_rejected(self):
        for key, value in [('completed', False), ('status', 'failed'), ('end_monotonic_ns', None)]:
            with self.subTest(key=key):
                old = copy.deepcopy(self.data)
                self.data['phases'][3][key] = value
                self.reject('incomplete|failed')
                self.data = old
        self.data['summary']['profile_valid'] = False
        self.reject('invalid')

    def test_secret_fields_and_free_text_rejected(self):
        for field in ['witness_values', 'rng_seed', 'wallet', 'private_key', 'environment']:
            with self.subTest(field=field):
                self.data[field] = 'synthetic marker only'
                self.reject('secret')
                del self.data[field]
        self.data['phases'][0]['metrics']['bytes']['notes'] = 'rng_seed=synthetic'
        self.reject('secret')

    def test_schema_checked(self):
        self.data['host']['logical_cpus_available'] = 'bad'
        self.reject('schema')

    def test_decision_rows_without_candidates(self):
        rows = self.profile.decide_profiles([self.data])['rows']
        self.assertEqual([r['row'] for r in rows], ['C'+str(i) for i in range(1, 11)])
        for row in rows:
            self.assertTrue(row.get('metric') or row.get('reason'))
            self.assertIsNone(row.get('measured_gain'))
        for index in [0, 1, 4]:
            self.assertEqual(rows[index]['status'], 'unresolved')
        self.assertEqual(rows[7]['status'], 'inapplicable')
        self.assertEqual(rows[6]['metric']['numerator_ns'], 50_000_000_000)
        self.assertEqual(rows[6]['metric']['denominator_ns'], 100_000_000_000)

    def test_composed_gain_rejects_shared_credit(self):
        with self.assertRaisesRegex(ValueError, 'overlap'):
            self.profile.reject_overlapping_credit([{'row':'C3', 'spans':['msm']}, {'row':'C9', 'spans':['msm']}])

    def test_cli_reduce_and_decide(self):
        with tempfile.TemporaryDirectory() as tmp:
            for command in ['reduce', 'decide']:
                out = Path(tmp) / (command + '.json')
                result = subprocess.run(['python3', '-B', str(ROOT / 'scripts/qa/profile-recursion.py'), command,
                                         str(FIXTURE), '--output', str(out)], capture_output=True, text=True)
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertTrue(out.is_file())

    def test_invalid_sidecar_not_stored(self):
        with tempfile.TemporaryDirectory() as tmp:
            source, out = Path(tmp) / 'bad.json', Path(tmp) / 'out.json'
            self.data['witness'] = 'synthetic marker'
            source.write_text(json.dumps(self.data))
            result = subprocess.run(['python3', '-B', str(ROOT / 'scripts/qa/profile-recursion.py'), 'reduce',
                                     str(source), '--output', str(out)], capture_output=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertFalse(out.exists())
            self.assertTrue(source.exists())

    def test_planners_never_execute_by_default(self):
        with patch.object(self.profile.subprocess, 'run', side_effect=AssertionError('executed')), patch('sys.stdout'):
            for command in ['one', 'calibrate']:
                self.assertEqual(self.profile.main([command, '--workload', 'A2']), 0)

    def test_a2_plans_both_real_binaries_with_empty_rustc_wrapper(self):
        import io
        import shlex
        from contextlib import redirect_stdout
        out = io.StringIO()
        with patch.object(self.profile.subprocess, 'run', side_effect=AssertionError('executed')), redirect_stdout(out):
            self.assertEqual(self.profile.main(['one', '--workload', 'A2']), 0)
        commands = [shlex.split(line) for line in out.getvalue().splitlines()]
        self.assertEqual(len(commands), 4)
        prefix = ['scripts/doctorwatson', 'heavy', '--budget', 'a2-linked-children', '--']
        for command in commands:
            self.assertEqual(command[:5], prefix)
        for command in commands[:2]:
            self.assertIn('RUSTC_WRAPPER=', command)
            self.assertIn('CARGO_BUILD_JOBS=4', command)
            self.assertIn('profile-a2', command)
            self.assertIn('--offline', command)
        self.assertNotIn('--features', commands[0])
        self.assertEqual(commands[1][-2:], ['--features', 'recursion-phase-profile'])
        self.assertEqual(commands[2][5], 'tools/profile/target/off/release/profile-a2')
        self.assertEqual(commands[3][5], 'tools/profile/target/on/release/profile-a2')

    def test_a2_resident_pk_is_not_falsely_planned(self):
        with patch('sys.stderr'), patch('sys.stdout'):
            self.assertEqual(self.profile.main(['one', '--workload', 'A2', '--cache-mode', 'resident_pk']), 1)

    def test_sampled_rss_predicates(self):
        rss = self.data['phases'][3]['metrics']['rss']
        rss.update(samples=2, sample_points=[{'at_monotonic_ns': 11_000_000_000, 'rss_bytes': 8},
                                            {'at_monotonic_ns': 12_000_000_000, 'rss_bytes': 12}],
                   sampled_peak_bytes=12, peak_delta_bytes=2, quality='sampled_lower_bound')
        self.profile.validate_sidecar(self.data)
        rss['sample_points'][1]['at_monotonic_ns'] = 11_000_000_000
        self.reject('unique')

    def test_boundary_sample_rejected(self):
        rss = self.data['phases'][3]['metrics']['rss']
        rss.update(samples=1, sample_points=[{'at_monotonic_ns': 10_000_000_000, 'rss_bytes': 10}])
        self.reject('interior')

    def test_instantaneous_certificate_rejected(self):
        self.data['phases'][3]['metrics']['rss']['certifies_instantaneous'] = True
        self.reject('instantaneous')

    def test_one_sample_still_boundary_only(self):
        rss = self.data['phases'][3]['metrics']['rss']
        rss.update(samples=1, sample_points=[{'at_monotonic_ns': 11_000_000_000, 'rss_bytes': 12}],
                   sampled_peak_bytes=12, peak_delta_bytes=2)
        self.profile.validate_sidecar(self.data)

    def test_five_percent_unexplained_accepted(self):
        p = self.data['phases'][4]
        p['end_monotonic_ns'] -= 5_000_000_000
        p['metrics'].update(wall_seconds=45, exclusive_wall_seconds=45,
                            cpu_core_seconds=90, exclusive_cpu_core_seconds=90)
        for coverage in (self.data['summary']['coverage'], self.data['proofs'][0]['coverage']):
            coverage['accounted_exclusive_wall_seconds'] -= 5
            coverage['unattributed_wall_seconds'] = coverage['unexplained_other_seconds'] = 5
            for owner in coverage['owner_shares']:
                if owner['owner'] == 'multiopen':
                    owner['exclusive_seconds'] -= 5
                    owner['fraction_of_named_scope'] = owner['exclusive_seconds'] / coverage['measured_wall_seconds']
        proof = self.profile.validate_sidecar(self.data)['proofs'][0]
        self.assertEqual(proof['partition'][-1]['wall_ns'], 5_000_000_000)

    def test_nested_msm_credit_is_shared_once(self):
        opening = self.data['phases'][4]
        msm = copy.deepcopy(opening)
        msm.update(span_id='multiopen_msm', phase='msm', owner='commitment',
                   start_monotonic_ns=100_000_000_000)
        msm['metrics'].update(wall_seconds=10, exclusive_wall_seconds=10,
                              cpu_core_seconds=20, exclusive_cpu_core_seconds=20)
        inclusive = copy.deepcopy(opening)
        inclusive.update(span_id='multiopen_inclusive', accounting=False)
        inclusive['metrics'].update(exclusive_wall_seconds=None, exclusive_cpu_core_seconds=None)
        opening.update(parent_span_id='multiopen_inclusive', end_monotonic_ns=100_000_000_000)
        opening['metrics'].update(wall_seconds=40, exclusive_wall_seconds=40,
                                  cpu_core_seconds=80, exclusive_cpu_core_seconds=80)
        msm['parent_span_id'] = 'multiopen_inclusive'
        self.data['phases'] += [inclusive, msm]
        for c in (self.data['summary']['coverage'], self.data['proofs'][0]['coverage']):
            for owner in c['owner_shares']:
                if owner['owner'] == 'multiopen':
                    owner.update(exclusive_seconds=40, fraction_of_named_scope=40/c['measured_wall_seconds'])
            c['owner_shares'].append(dict(owner='commitment', exclusive_seconds=10,
                                          fraction_of_named_scope=10/c['measured_wall_seconds']))
        rows = self.profile.decide_profiles([self.data])['rows']
        self.assertEqual(rows[2]['metric']['spans'], ['multiopen_msm'])
        self.assertEqual(rows[8]['metric']['numerator_ns'], 40_000_000_000)
        self.assertEqual(rows[8]['metric']['inclusive_multiopen_ns'], 50_000_000_000)
        self.assertEqual(rows[8]['metric']['shared_msm_spans'], ['multiopen_msm'])

    def test_sampled_peak_needs_raw_samples(self):
        self.data['phases'][3]['metrics']['rss']['samples'] = 2
        self.reject('sample points')

    def test_no_cpu_attribution_without_reason(self):
        m = self.data['phases'][3]['metrics']
        m.update(cpu_core_seconds=None, exclusive_cpu_core_seconds=None, cpu_scope='unattributable')
        self.reject('reason')
        m['cpu_null_reason'] = 'Concurrent process CPU cannot be attributed'
        self.assertIsNone(self.profile.validate_sidecar(self.data)['proofs'][0]['cpu_core_ns'])

    def test_subnanosecond_time_rejected(self):
        self.data['phases'][3]['metrics']['wall_seconds'] = 50.0000000001
        self.reject('nanoseconds')

    def test_secret_rejection_does_not_print_value(self):
        with tempfile.TemporaryDirectory() as tmp:
            source = Path(tmp) / 'secret.json'
            self.data['seed'] = 'PUBLIC_SYNTHETIC_CANARY'
            source.write_text(json.dumps(self.data))
            result = subprocess.run(['python3', '-B', str(ROOT / 'scripts/qa/profile-recursion.py'),
                                     'reduce', str(source)], capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertNotIn('PUBLIC_SYNTHETIC_CANARY', result.stdout + result.stderr)

    def test_protocol_and_stock_verification_failures_rejected(self):
        for key, value in [('protocol_unchanged', False), ('stock_verifier', 'fail'),
                           ('official_srs_authenticated', False)]:
            with self.subTest(key=key):
                old = self.data['verification'][key]
                self.data['verification'][key] = value
                self.reject('verification|protocol|SRS')
                self.data['verification'][key] = old

    def test_negative_overhead_mean_is_allowed_by_quint(self):
        self.data['overhead'].update(estimated_fraction=-0.005,
                                     paired_wall_ratio_upper95=1.01, qualification='pass')
        self.profile.validate_sidecar(self.data)

    def test_summary_cpu_must_match_command(self):
        self.data['summary']['cpu_core_seconds'] = 0
        self.reject('CPU')

    def test_resolved_package_version_is_pinned(self):
        self.data['identity']['resolved_packages'][0]['version'] = '0.8.2'
        self.reject('identity')

    def test_proofs_cannot_share_process_cpu_or_intervals(self):
        inner = copy.deepcopy(self.data['phases'][2])
        inner.update(span_id='inner-proof', parent_span_id='proof')
        self.data['phases'].append(inner)
        for p in self.data['phases']:
            if p['span_id'] in ('quotient', 'multiopen'):
                p['parent_span_id'] = 'inner-proof'
        proof = copy.deepcopy(self.data['proofs'][0])
        proof.update(proof_id='duplicate-cpu-owner', root_span_id='inner-proof')
        self.data['proofs'].append(proof)
        self.reject('proof.*overlap')


class EventMappingTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.profile = load('profile-recursion')

    def setUp(self):
        def event(i, parent, phase, accounting, start, end, thread=1):
            return dict(id=i, parent_id=parent, phase_id=phase, accounting=accounting,
                        start_ns=start, end_ns=end, thread_id=thread, status=1,
                        advice_phase=2**64 - 1, circuit_index=2**64 - 1)
        self.data = dict(schema='recursion.phase-events/1', event_capacity=32,
                         dropped_events=0, timer_failures=0, reentrant_events=0,
                         profile_valid=True, flush_wall_ns=10, events=[
            event(1, 0, 1, 0, 1, 101),  # command
            event(2, 1, 4, 0, 2, 100),  # proof
            event(3, 2, 17, 1, 3, 99),  # multiopen
            event(4, 3, 18, 1, 10, 40), # nested MSM
            event(5, 3, 18, 1, 60, 80), # second MSM
            event(6, 3, 22, 0, 4, 90),  # diagnostic earns no credit
        ])

    def test_mapping_schema_and_unique_accounting(self):
        mapped = self.profile.map_events(self.data, 'synthetic')
        phases = mapped['phases']
        self.assertEqual(sum(self.profile.ns(p['metrics']['exclusive_wall_seconds'])
                             for p in phases if p['accounting']), 96)
        owners = [p for p in phases if p['accounting']]
        self.assertEqual(sum(self.profile.ns(p['metrics']['wall_seconds']) for p in owners
                             if p['phase'] == 'commitment_msm'), 50)
        self.assertEqual(sum(self.profile.ns(p['metrics']['wall_seconds']) for p in owners
                             if p['phase'] == 'multiopen'), 46)
        self.assertFalse(mapped['profile_valid'])
        self.assertTrue(mapped['observer_valid'])
        from jsonschema import Draft202012Validator
        schema = json.loads(self.profile.SCHEMA.read_text())
        validator = Draft202012Validator({'$ref': '#/$defs/phase', '$defs': schema['$defs']})
        for phase in phases:
            validator.validate(phase)
            self.assertIsNone(phase['metrics']['rss']['entry_bytes'])
            self.assertIsNone(phase['metrics']['cpu_core_seconds'])
        self.assertIn('not_applicable', [p['status'] for p in phases
                                       if p['phase'] == 'fixed_commitments'])

    def test_invalid_observer_keeps_partial_trace_without_credit(self):
        for field in ['dropped_events', 'timer_failures', 'reentrant_events']:
            with self.subTest(field=field):
                data = copy.deepcopy(self.data)
                data[field] = 1
                mapped = self.profile.map_events(data, 'synthetic')
                self.assertFalse(mapped['observer_valid'])
                self.assertFalse(any(p['accounting'] for p in mapped['phases']))

    def test_full_buffer_and_returned_error(self):
        self.data['event_capacity'] = len(self.data['events'])
        self.assertFalse(self.profile.map_events(self.data, 'synthetic')['observer_valid'])
        self.data['event_capacity'] = 32
        self.data['events'][3]['status'] = 2
        mapped = self.profile.map_events(self.data, 'synthetic')
        self.assertFalse(mapped['observer_valid'])
        self.assertIn('failed', [p['status'] for p in mapped['phases']])

    def test_incomplete_timer_is_retained(self):
        self.data['events'][0].update(status=0, end_ns=0)
        mapped = self.profile.map_events(self.data, 'synthetic')
        self.assertFalse(mapped['observer_valid'])
        self.assertIsNone(mapped['phases'][0]['end_monotonic_ns'])

    def test_invalid_parent_and_overlap_are_rejected(self):
        self.data['events'][3]['parent_id'] = 999
        with self.assertRaisesRegex(ValueError, 'parent'):
            self.profile.map_events(self.data, 'synthetic')
        self.data['events'][3]['parent_id'] = 3
        self.data['events'][4]['start_ns'] = 30
        with self.assertRaisesRegex(ValueError, 'overlap'):
            self.profile.map_events(self.data, 'synthetic')

    def test_secret_metadata_rejected(self):
        self.data['events'][0]['seed'] = 'synthetic'
        with self.assertRaisesRegex(ValueError, 'secret'):
            self.profile.map_events(self.data, 'synthetic')

    def test_cli_maps_raw_events_without_qualifying_them(self):
        with tempfile.TemporaryDirectory() as tmp:
            source = Path(tmp) / 'events.json'
            output = Path(tmp) / 'mapped.json'
            source.write_text(json.dumps(self.data))
            self.assertEqual(self.profile.main(['reduce', str(source), '--output', str(output)]), 0)
            mapped = json.loads(output.read_text())['profiles'][0]
            self.assertFalse(mapped['profile_valid'])
            self.assertTrue(mapped['observer_valid'])

    def test_worker_ntt_is_owned_by_joined_phase_once(self):
        self.data['events'][-1].update(parent_id=0, thread_id=2)
        mapped = self.profile.map_events(self.data, 'synthetic')
        worker = next(p for p in mapped['phases'] if p['span_id'] == '6')
        self.assertEqual(worker['parent_span_id'], '3')
        self.assertEqual(worker['owner'], 'multiopen')
        self.assertFalse(worker['accounting'])
        self.assertEqual(sum(self.profile.ns(p['metrics']['wall_seconds'])
                             for p in mapped['phases'] if p['accounting']), 96)

    def test_public_advice_ordinals_map_to_dimensions(self):
        self.data['events'][2].update(advice_phase=1, circuit_index=2)
        mapped = self.profile.map_events(self.data, 'synthetic')
        phase = next(p for p in mapped['phases'] if p['span_id'] == '3')
        self.assertEqual(phase['dimensions']['advice_phase'], 1)
        self.assertEqual(phase['dimensions']['circuit_index'], 2)

    def test_counter_measurements_map_without_duplicate_cpu_credit(self):
        metrics = copy.deepcopy(json.loads(FIXTURE.read_text())['phases'][3]['metrics'])
        measured = {k: metrics[k] for k in ('cpu_core_seconds', 'cpu_null_reason', 'cpu_scope', 'rss', 'bytes')}
        measured['rss'].update(samples=0, sample_points=[], quality='boundary_only')
        self.data['observation'] = dict(sampler_valid=True, pid=123, measurements=[
            dict(event_id=4, start_ns=10, end_ns=40, metrics=measured)])
        mapped = self.profile.map_events(self.data, 'synthetic')
        inclusive = next(p for p in mapped['phases'] if p['span_id'] == '4')
        exclusive = next(p for p in mapped['phases'] if p['span_id'] == '4:exclusive:0')
        self.assertEqual(inclusive['pid'], 123)
        self.assertIsNone(inclusive['metrics']['exclusive_cpu_core_seconds'])
        self.assertEqual(exclusive['metrics']['exclusive_cpu_core_seconds'], measured['cpu_core_seconds'])
        self.assertFalse(mapped['profile_valid'])

    def test_sampler_failure_keeps_evidence_without_credit(self):
        self.data['observation'] = dict(sampler_valid=False, measurements=[])
        self.data['workload'] = dict(id='A2', exit=1)
        mapped = self.profile.map_events(self.data, 'synthetic')
        self.assertFalse(mapped['observer_valid'])
        self.assertFalse(any(p['accounting'] for p in mapped['phases']))
        self.assertEqual(mapped['workload']['exit'], 1)

    def test_expected_transcript_error_needs_a_successful_verifier_control(self):
        self.data['events'][2]['phase_id'] = 5
        self.data['events'][3].update(phase_id=19, status=2)
        self.data['observation'] = dict(sampler_valid=True, measurements=[], expected_rejections=[4])
        self.data['workload'] = dict(exit=0, verification=dict(negative_controls='pass'))
        mapped = self.profile.map_events(self.data, 'synthetic')
        self.assertTrue(mapped['observer_valid'])
        self.assertIn('Expected verifier rejection', next(p for p in mapped['phases'] if p['span_id'] == '4')['reason'])
        self.data['workload']['verification']['negative_controls'] = 'fail'
        with self.assertRaisesRegex(ValueError, 'expected rejection'):
            self.profile.map_events(self.data, 'synthetic')

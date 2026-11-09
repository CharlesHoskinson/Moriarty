"""GP node-chain workload and the C11-C14 lever rows (task 2.7, F19; RPP task 5.3)."""
import copy
import io
import json
from contextlib import redirect_stdout
from pathlib import Path
import shlex
import shutil
import tempfile
import unittest
from unittest.mock import patch
from test_profile_identity import ROOT, load

FIXTURE = ROOT / 'scripts/qa/tests/fixtures/profile/valid.json'


def node_sidecar(threads, scale=1, invocation=None):
    """The valid fixture as a GP node proof at `threads`, every wall multiplied by `scale`."""
    data = json.loads(FIXTURE.read_text())
    data['id'] = 'GP'
    data['invocation_id'] = invocation or f'gp-t{threads}-{scale}'
    data['host']['rayon_threads'] = threads
    data['proofs'][0]['shape'] = 'node'
    data['proofs'][0]['cache_mode'] = 'resident_pk'
    if scale != 1:
        def scaled(value):
            return value * scale
        for phase in data['phases']:
            phase['start_monotonic_ns'] *= scale
            phase['end_monotonic_ns'] *= scale
            for key in ('wall_seconds', 'exclusive_wall_seconds'):
                if phase['metrics'][key] not in (None, 0):
                    phase['metrics'][key] = scaled(phase['metrics'][key])
            rss = phase['metrics']['rss']
            for sample in rss.get('sample_points', []):
                sample['at_monotonic_ns'] *= scale
        for coverage in [data['summary']['coverage']] + [p['coverage'] for p in data['proofs']]:
            for key in ('measured_wall_seconds', 'accounted_exclusive_wall_seconds',
                        'unexplained_other_seconds', 'unattributed_wall_seconds'):
                coverage[key] = scaled(coverage[key])
            for owner in coverage['owner_shares']:
                owner['exclusive_seconds'] = scaled(owner['exclusive_seconds'])
        data['summary']['wall_seconds'] = scaled(data['summary']['wall_seconds'])
    for phase in data['phases']:
        phase['invocation_id'] = data['invocation_id']
    return data


def plan(*argv):
    profile = load('profile-recursion')
    out = io.StringIO()
    with patch.object(profile.subprocess, 'run', side_effect=AssertionError('executed')), redirect_stdout(out):
        code = profile.main(list(argv))
    return code, [shlex.split(line) for line in out.getvalue().splitlines()]


class GpPlanTests(unittest.TestCase):
    def test_gp_budget_registered(self):
        profile = load('profile-recursion')
        self.assertEqual(profile.BUDGETS['GP'], 'c0-gp-chain')
        budgets = json.loads((ROOT / 'qa/sherlock/budgets.json').read_text())['budgets']
        self.assertIn('c0-gp-chain', budgets)

    def test_gp_plan_builds_and_runs_profile_gp_with_threads(self):
        code, commands = plan('one', '--workload', 'GP', '--threads', '10', '--run-id', 'c0-gp-t10')
        self.assertEqual(code, 0)
        self.assertEqual(len(commands), 4)
        ids = [c[c.index('--id') + 1] for c in commands]
        self.assertEqual(len(set(ids)), 4)
        for build in commands[:2]:
            self.assertIn('profile-gp', build)
            self.assertIn('--offline', build)
            self.assertIn('--locked', build)
            head = build[:build.index('--')]
            self.assertEqual(head[head.index('--max-wall') + 1], '3600')
        self.assertEqual(commands[1][-2:], ['--features', 'recursion-phase-profile'])
        for variant, run in zip(('off', 'on'), commands[2:]):
            head, tail = run[:run.index('--')], run[run.index('--') + 1:]
            self.assertEqual(head[head.index('--budget') + 1], 'c0-gp-chain')
            self.assertEqual(tail[0], 'env')
            self.assertIn('RAYON_NUM_THREADS=10', tail)
            self.assertIn('MORIARTY_GP_STANDIN_STEPS=3', tail)
            binary = next(x for x in tail if x.endswith('profile-gp'))
            self.assertEqual(binary, f'tools/profile/target/{variant}/release/profile-gp')
            args = tail[tail.index(binary) + 1:]
            self.assertEqual(args[args.index('--threads') + 1], '10')
            self.assertEqual(args[args.index('--cache-mode') + 1], 'resident_pk')
            self.assertEqual(args[args.index('--sidecar') + 1],
                             f'qa/sherlock/profiles/c0-gp-t10/GP-resident_pk-t10-0-{variant}.json')

    def test_gp_defaults_to_twenty_threads_and_resident_pk(self):
        code, commands = plan('one', '--workload', 'GP', '--variant', 'on')
        self.assertEqual(code, 0)
        run = commands[-1]
        self.assertIn('RAYON_NUM_THREADS=20', run)
        self.assertIn('resident_pk', run)

    def test_gp_real_leaf_directory(self):
        code, commands = plan('one', '--workload', 'GP', '--variant', 'off', '--leaf-dir', 'target/gp-leaves')
        self.assertEqual(code, 0)
        self.assertIn('MORIARTY_GP_LEAF_DIR=target/gp-leaves', commands[-1])
        self.assertFalse(any(x.startswith('MORIARTY_GP_STANDIN_STEPS=') for x in commands[-1]))

    def test_gp_refusals(self):
        with patch('sys.stderr'):
            for argv in (['--cache-mode', 'process_cold'], ['--threads', '0'],
                         ['--standin-steps', '0'], ['--leaf-dir', 'a b;rm']):
                self.assertEqual(plan('one', '--workload', 'GP', *argv)[0], 1, argv)
            # Thread and leaf options belong to GP only.
            self.assertEqual(plan('one', '--workload', 'A2', '--threads', '4')[0], 1)

    def test_a2_plan_unchanged(self):
        code, commands = plan('one', '--workload', 'A2')
        self.assertEqual(code, 0)
        self.assertEqual(commands[2][commands[2].index('--') + 1], 'tools/profile/target/off/release/profile-a2')


class PinTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.profile = load('profile-recursion')

    def test_profile_workspace_follows_root_pins(self):
        self.assertEqual(self.profile.check_pins(ROOT), [])

    def test_stale_pin_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / 'tools/profile').mkdir(parents=True)
            shutil.copy(ROOT / 'Cargo.toml', root / 'Cargo.toml')
            shutil.copy(ROOT / 'tools/profile/Cargo.lock', root / 'tools/profile/Cargo.lock')
            manifest = (ROOT / 'tools/profile/Cargo.toml').read_text()
            (root / 'tools/profile/Cargo.toml').write_text(manifest.replace('"=2.4.0"', '"=2.3.5"'))
            errors = self.profile.check_pins(root)
            self.assertTrue(any('midnight-zk-stdlib' in e for e in errors), errors)

    def test_lock_without_pinned_version_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / 'tools/profile').mkdir(parents=True)
            shutil.copy(ROOT / 'Cargo.toml', root / 'Cargo.toml')
            shutil.copy(ROOT / 'tools/profile/Cargo.toml', root / 'tools/profile/Cargo.toml')
            lock = (ROOT / 'tools/profile/Cargo.lock').read_text()
            (root / 'tools/profile/Cargo.lock').write_text(
                lock.replace('name = "midnight-circuits"\nversion = "7.3.0"', 'name = "midnight-circuits"\nversion = "7.2.4"'))
            errors = self.profile.check_pins(root)
            self.assertTrue(any('midnight-circuits' in e and 'Cargo.lock' in e for e in errors), errors)

    def test_default_archive_follows_cargo_home(self):
        tail = 'registry/cache/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3.crate'
        self.assertEqual(self.profile.default_archive({'CARGO_HOME': '/c'}), Path('/c') / tail)
        self.assertEqual(self.profile.default_archive({'HOME': '/h'}), Path('/h/.cargo') / tail)


class LeverRowTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.profile = load('profile-recursion')

    def rows(self, sidecars):
        rows = self.profile.decide_profiles(sidecars)['rows']
        return {r['row']: r for r in rows}

    def test_rows_c1_to_c14_each_with_metric_or_reason(self):
        rows = self.profile.decide_profiles([node_sidecar(20)])['rows']
        self.assertEqual([r['row'] for r in rows], ['C' + str(i) for i in range(1, 15)])
        for row in rows:
            self.assertTrue(row.get('metric') or row.get('reason'), row['row'])
            self.assertIsNone(row.get('measured_gain'))

    def test_c12_domain_share_and_k19_factor(self):
        # Fixture proof: 100 s; quotient 50 s and multiopen 50 s are domain-proportional.
        row = self.rows([node_sidecar(20)])['C12']
        m = row['metric']
        self.assertEqual(m['name'], 'domain_proportional_share')
        self.assertEqual((m['numerator_ns'], m['denominator_ns']), (100 * 10**9, 100 * 10**9))
        self.assertEqual(sorted(m['spans']), ['multiopen', 'quotient'])
        # 1 / (1 - d/2) = 2T / (2T - N)
        self.assertEqual(m['predicted_k19_factor'], {'numerator': 200 * 10**9, 'denominator': 100 * 10**9})
        self.assertEqual(m['rayon_threads'], 20)
        self.assertEqual(m['proof_shape'], 'node')

    def test_c14_msm_share_and_amdahl_bound(self):
        data = node_sidecar(20)
        data['phases'][3]['phase'] = 'commitment_msm'
        data['phases'][3]['owner'] = 'commitment'
        for c in (data['summary']['coverage'], data['proofs'][0]['coverage']):
            for owner in c['owner_shares']:
                if owner['owner'] == 'quotient':
                    owner['owner'] = 'commitment'
        row = self.rows([data])['C14']
        m = row['metric']
        self.assertEqual(m['name'], 'msm_share')
        self.assertEqual(m['spans'], ['quotient'])
        self.assertEqual(m['numerator_ns'], 50 * 10**9)
        self.assertEqual(m['amdahl_bound'], {'numerator': 100 * 10**9, 'denominator': 50 * 10**9})

    def test_c11_thread_sweep_keep_twenty(self):
        # wall(20) = 0.5 x wall(4) <= 0.7 x wall(4): keep 20 threads.
        rows = self.rows([node_sidecar(20), node_sidecar(10, 1, 'b'), node_sidecar(4, 2)])
        m = rows['C11']['metric']
        self.assertEqual(m['node_step_wall_ns'], {'4': 200 * 10**9, '10': 100 * 10**9, '20': 100 * 10**9})
        self.assertEqual(m['decision'], 'keep_20_threads')
        # U = CPU / wall at 20 threads: fixture proof 200 core-s over 100 s.
        self.assertEqual(m['average_cores_at_20'], {'numerator': 200 * 10**9, 'denominator': 100 * 10**9})

    def test_c11_serial_bound(self):
        rows = self.rows([node_sidecar(20), node_sidecar(4, 1, 'b')])
        self.assertEqual(rows['C11']['metric']['decision'], 'serial_bound')

    def test_c11_needs_four_and_twenty_threads(self):
        row = self.rows([node_sidecar(20)])['C11']
        self.assertEqual(row['status'], 'unresolved')
        self.assertRegex(row['reason'], '4')

    def test_lever_rows_need_a_node_proof_at_twenty_threads(self):
        data = json.loads(FIXTURE.read_text())
        rows = self.rows([data])
        for name in ('C11', 'C12', 'C13', 'C14'):
            self.assertEqual(rows[name]['status'], 'unresolved', name)
            self.assertTrue(rows[name]['reason'])
        # C1-C10 keep their existing selection.
        self.assertEqual(rows['C7']['metric']['numerator_ns'], 50_000_000_000)

    def test_c13_s_arithmetic_without_leaf_stays_unresolved(self):
        row = self.rows([node_sidecar(20)])['C13']
        self.assertEqual(row['status'], 'unresolved')
        self.assertRegex(row['reason'], 'leaf')
        m = row['metric']
        self.assertIsNone(m['leaf_seconds'])
        # Node-only lower bound per S: ceil(320 / S) x t_node.
        self.assertEqual(m['node_only_ns'], {'1': 320 * 100 * 10**9, '2': 160 * 100 * 10**9, '4': 80 * 100 * 10**9,
                                             '8': 40 * 100 * 10**9, '16': 20 * 100 * 10**9})

    def test_lever_rows_do_not_claim_composable_gain(self):
        rows = self.rows([node_sidecar(20), node_sidecar(4, 2)])
        for name in ('C11', 'C12', 'C13', 'C14'):
            self.assertIsNone(rows[name]['measured_gain'])
            if rows[name]['metric']:
                self.assertFalse(rows[name]['metric']['composable'])


def raw_trace(threads, scale=1):
    """A raw GP on-run trace: one stand-in leaf prove, one node prove of 100 x scale ns
    (quotient 40, MSM 30 inside a commitment phase, witness 28, 2 unexplained, a worker NTT
    of 20 joined inside the MSM) and a handled transcript read error."""
    def event(i, parent, phase, accounting, start, end, status=1, thread=1):
        return dict(id=i, parent_id=parent, phase_id=phase, accounting=accounting,
                    start_ns=start * scale, end_ns=end * scale, thread_id=thread, status=status,
                    advice_phase=2**64 - 1, circuit_index=2**64 - 1)
    return dict(schema='recursion.phase-events/1', event_capacity=64, dropped_events=0,
                timer_failures=0, reentrant_events=0, profile_valid=False, flush_wall_ns=1, events=[
        event(1, 0, 1, 0, 1, 400),
        event(2, 1, 4, 0, 2, 10),           # stand-in leaf prove
        event(3, 2, 7, 1, 3, 9),
        event(4, 1, 4, 0, 100, 200),        # node prove
        event(5, 4, 7, 1, 100, 128),        # synthesis (witness)
        event(6, 5, 19, 1, 110, 111, 2),    # handled read error of the absent previous proof
        event(7, 4, 14, 1, 128, 168),       # quotient
        event(8, 4, 9, 1, 168, 198),        # advice commitments
        event(9, 8, 18, 1, 168, 198),       # its MSM owns all of it
        event(10, 0, 22, 0, 170, 190, thread=2),  # worker NTT (joined, no TLS parent)
    ], observation=dict(sampler_valid=True, measurements=[], expected_rejections=[]),
        workload=dict(id='GP', exit=0, profile_enabled=True, host=dict(rayon_threads=threads),
                      chain=dict(standin=True, leaves=1),
                      verification=dict(stock_verifier='pass', negative_controls='pass'),
                      proofs=[dict(shape='node', step=0, cache_mode='resident_pk', k=20)]))


class RawDecideTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.profile = load('profile-recursion')

    def decide(self, traces, overhead=None):
        with tempfile.TemporaryDirectory() as tmp:
            paths = []
            for i, trace in enumerate(traces):
                path = Path(tmp) / f'run-{i}' / 'GP-on.json'
                path.parent.mkdir()
                path.write_text(json.dumps(trace))
                paths.append(path)
            off = Path(tmp) / 'run-0' / 'GP-off.json'
            off.write_text(json.dumps(dict(raw_trace(20), events=[], event_capacity=0,
                                           workload=dict(raw_trace(20)['workload'], profile_enabled=False))))
            if overhead is not None:
                (Path(tmp) / 'overhead.json').write_text(json.dumps(overhead))
                overhead = Path(tmp) / 'overhead.json'
            output = Path(tmp) / 'decide.json'
            code = self.profile.main(['decide', *map(str, paths + [off]), '--output', str(output)]
                                     + (['--overhead', str(overhead)] if overhead else []))
            self.assertEqual(code, 0)
            return json.loads(output.read_text())

    def test_raw_gp_traces_give_lever_rows_and_shares(self):
        overhead = dict(schema='recursion.overhead/1', run='c0-a2-qual-20261108-n20', qualification='inconclusive',
                        pair_count=20, estimated_fraction=0.007, paired_wall_ratio_upper95=1.017)
        report = self.decide([raw_trace(20), raw_trace(4, 2)], overhead)
        self.assertEqual(report['evidence']['source'], 'raw_events')
        self.assertFalse(report['evidence']['profile_valid'])
        self.assertEqual(len(report['skipped']), 1)
        rows = {r['row']: r for r in report['rows']}
        self.assertEqual([r['row'] for r in report['rows']], ['C' + str(i) for i in range(1, 15)])
        for row in report['rows']:
            self.assertEqual(row['overhead'], 'overhead unqualified, est +0.7%, one-sided 95% bound 1.017 '
                                              '(task 2.8, run c0-a2-qual-20261108-n20)')
            self.assertNotEqual(row['status'], 'confirmed')
        # The stand-in leaf is not a workload proof: the node is the only proof root.
        self.assertEqual(rows['C12']['metric']['proof_id'], 'node-0')
        # d = quotient 40 + MSM 30 of 100; m = 30 of 100.
        self.assertEqual(rows['C12']['metric']['numerator_ns'], 70)
        self.assertEqual(rows['C14']['metric']['numerator_ns'], 30)
        self.assertEqual(rows['C11']['metric']['node_step_wall_ns'], {'4': 200, '20': 100})
        self.assertEqual(rows['C11']['metric']['decision'], 'keep_20_threads')
        node = report['gp'][0]['proofs'][0]
        self.assertEqual((node['wall_ns'], node['unexplained_ns']), (100, 2))
        # q = quotient [128, 168) + worker NTT [170, 190) = 60; m or q = [128, 198) = 70.
        self.assertEqual(node['q']['value'], 0.6)
        self.assertEqual(node['m_or_q']['value'], 0.7)
        self.assertEqual(report['gp'][0]['returned_errors']['by_context'], {'prove': 1})

    def test_without_overhead_report_rows_say_unqualified(self):
        report = self.decide([raw_trace(20)])
        self.assertTrue(all(r['overhead'].startswith('overhead unqualified') for r in report['rows']))

    def test_observer_invalid_or_mismatched_traces_are_refused(self):
        invalid = raw_trace(20)
        invalid['events'][7]['status'] = 2  # a failed advice commitment
        mismatch = raw_trace(20)
        mismatch['workload']['proofs'].append(dict(shape='run_decider', cache_mode='resident_pk'))
        for trace, message in ((invalid, 'observer-invalid'), (mismatch, 'prove spans')):
            with self.subTest(message=message), tempfile.TemporaryDirectory() as tmp:
                path = Path(tmp) / 'GP-on.json'
                path.write_text(json.dumps(trace))
                with patch('sys.stderr', new=io.StringIO()) as err:
                    self.assertEqual(self.profile.main(['decide', str(path), '--output', str(Path(tmp) / 'd.json')]), 1)
                self.assertIn(message, err.getvalue())


class SchemaTests(unittest.TestCase):
    def test_gp_sidecar_validates(self):
        profile = load('profile-recursion')
        report = profile.validate_sidecar(node_sidecar(20))
        self.assertEqual(report['proofs'][0]['shape'], 'node')
        decider = copy.deepcopy(node_sidecar(20))
        decider['proofs'][0]['shape'] = 'run_decider'
        profile.validate_sidecar(decider)


if __name__ == '__main__':
    unittest.main()

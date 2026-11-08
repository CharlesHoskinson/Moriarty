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

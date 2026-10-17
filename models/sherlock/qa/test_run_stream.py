"""Pure Python candidate wiring tests. Never compiles or executes Rust."""
import importlib.util
from pathlib import Path
import re
import tempfile
import unittest
from unittest import mock

QA = Path(__file__).resolve().parent
ROOT = QA.parents[2]
SPEC = importlib.util.spec_from_file_location('candidate_run_host', QA / 'run_host.py')
runner = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(runner)
W = ROOT


class StreamWiring(unittest.TestCase):
    def test_fixed_stream_realization_consumes_new_behavior(self):
        stream = runner.REALIZATIONS['stream']
        self.assertEqual(stream.producer, 'models/sherlock/qa/stream_observations.rs')
        self.assertEqual(stream.compare_flag, '--host-observations')
        self.assertEqual(stream.report_key, 'supplied_host_observation')
        self.assertEqual(stream.output, 'stream-observations')
        text = (QA / 'stream_observations.rs').read_text()
        self.assertIn('machine.visit_steps(width, |record|', text)
        self.assertNotIn('machine.run_steps(', text)
        self.assertIn('const WIDTHS: [usize; 4] = [1, 2, 4, 8];', text)
        self.assertNotIn('reference_bounded_tests.rs', text)

    def test_all_eight_existing_tests_are_selected_exactly(self):
        source = (W / (runner.SHERLOCK + 'reference_stream_tests.rs')).read_text()
        names = re.findall(r'(?m)^#\[test\]\s*\nfn ([a-z0-9_]+)\(', source)
        declared = runner.REALIZATIONS['stream'].tests
        self.assertEqual(len(names), 8)
        self.assertEqual(declared, tuple('reference_stream_tests::' + name for name in names))
        output = '\n'.join(['running 8 tests'] + [f'test {name} ... ok' for name in declared] +
            ['test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s'])
        self.assertEqual(runner.libtest_results(output, declared), list(declared))
        self.assertNotEqual(declared, runner.EXPECTED_TESTS)

    def test_stream_closure_keeps_test_members_out_of_producer(self):
        stream = runner.REALIZATIONS['stream']
        self.assertIn(runner.SHERLOCK + 'reference_stream.rs', stream.producer_closure)
        self.assertNotIn(runner.SHERLOCK + 'reference_stream_tests.rs', stream.producer_closure)
        self.assertEqual(set(stream.test_closure) - set(stream.producer_closure),
                         {runner.SHERLOCK + 'reference_stream_tests.rs'})
        self.assertTrue(set(stream.test_closure) <= set(stream.sources))
        self.assertFalse(any('native' in path or 'helper_machine' in path for path in stream.sources))
        for relative in stream.sources:
            self.assertTrue((W / relative).is_file(), relative)

    def test_preflight_snapshot_contains_both_actual_checker_helpers(self):
        # Real preflight snapshot and checker consumption, with a plain-text fake
        # compiler that is only stat()ed/hashed; no Rust or child process may run.
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            compiler = root / 'toolchain/bin/rustc'
            compiler.parent.mkdir(parents=True)
            (root / 'toolchain/lib/rustlib').mkdir(parents=True)
            compiler.write_bytes(b'not a compiler; this test never executes it\n')
            compiler.chmod(0o700)
            attempt_root = root / 'attempt'
            attempt_root.mkdir()
            attempt = runner.Attempt(attempt_root, str(compiler), 'stream')
            with mock.patch.object(runner.Attempt, 'child',
                                   side_effect=AssertionError('no child may run')):
                attempt.preflight()
            expected = ['models/sherlock/qa/host_observations.py',
                        'models/sherlock/qa/source_reference_facts.py']
            self.assertEqual(attempt.receipt['comparison_helpers'], expected)
            for relative in expected:
                self.assertIn(relative, attempt.sources)
                self.assertEqual((attempt.snapshot / relative).read_bytes(),
                                 (ROOT / relative).read_bytes())
            self.assertFalse(any('child' in row for row in attempt.receipt['steps']))
            check_path = attempt.snapshot / runner.CHECK
            spec = importlib.util.spec_from_file_location('snapshot_checker', check_path)
            checker = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(checker)
            report = checker.run(attempt.snapshot)
            self.assertEqual(report['status'], 'pass')
            self.assertEqual(report['source_reference_facts']['unique_paths'], 8)
            self.assertEqual(report['source_reference_facts']['execution_evidence'], 'none')
            self.assertEqual(checker.host_adapter().PRODUCER, 'sherlock-host-reference')

    def test_stream_claims_keep_local_coverage_separate(self):
        stream = runner.REALIZATIONS['stream']
        self.assertEqual(stream.claims['acceptance'], 'none')
        self.assertEqual(stream.claims['not_run'], runner.CLAIMS['not_run'])
        self.assertIn('visit_steps', stream.claims['observed'])
        self.assertIn('retains', stream.claims['retention'])
        self.assertEqual(stream.claims['memory_bound'], 'none-claimed')
        self.assertEqual(len(runner.REALIZATIONS['host'].tests), 14)
        self.assertEqual(runner.REALIZATIONS['image'].tests, ())


if __name__ == '__main__':
    unittest.main(verbosity=2)

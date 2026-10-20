"""Ordinary wiring checks of the real local shell with tiny Python consumers.

Only the entrypoint is copied from the checkout. Fixture consumers record stage
order and working directories; they neither execute Sherlock nor copy this suite.
"""
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import textwrap
import unittest


REPO = Path(__file__).resolve().parents[3]


class DoctorWatsonEntrypointTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix='doctorwatson-entrypoint-')
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.checkout = self.root / 'checkout with spaces'
        self.outside = self.root / 'other working directory'
        self.outside.mkdir()
        scripts = self.checkout / 'scripts'
        scripts.mkdir(parents=True)
        self.entrypoint = scripts / 'doctorwatson.sh'
        shutil.copy2(REPO / 'scripts/doctorwatson.sh', self.entrypoint)
        self.entrypoint.chmod(0o755)
        qa = self.checkout / 'models/sherlock/qa'
        qa.mkdir(parents=True)
        self.events = self.root / 'stages.jsonl'
        self.auxiliary = self.root / 'auxiliary.jsonl'
        (qa / 'check.py').write_text(textwrap.dedent('''\
            import json
            import os
            from pathlib import Path

            with Path(os.environ['DW_TEST_EVENTS']).open('a') as stream:
                stream.write(json.dumps({'stage': 'checker', 'cwd': str(Path.cwd())}) + '\\n')
            raise SystemExit(int(os.environ.get('DW_TEST_CHECKER_EXIT', '0')))
            '''), encoding='utf-8')
        (qa / 'test_fixture.py').write_text(textwrap.dedent('''\
            import json
            import os
            from pathlib import Path
            import unittest

            class FixtureRegression(unittest.TestCase):
                def test_stage(self):
                    events = Path(os.environ['DW_TEST_EVENTS'])
                    previous = [json.loads(line)['stage'] for line in events.read_text().splitlines()]
                    self.assertEqual(previous, ['checker'])
                    with events.open('a') as stream:
                        stream.write(json.dumps({'stage': 'regression', 'cwd': str(Path.cwd())}) + '\\n')
                    self.assertNotEqual(os.environ.get('DW_TEST_REGRESSION_FAIL'), '1',
                                        'requested ordinary fixture regression failure')
            '''), encoding='utf-8')
        delivery = self.checkout / 'models/doctorwatson'
        delivery.mkdir(parents=True)
        (delivery / 'test_delivery_fixture.py').write_text(textwrap.dedent('''\
            import json
            import os
            from pathlib import Path
            import unittest

            class FixtureDeliveryRegression(unittest.TestCase):
                def test_stage(self):
                    events = Path(os.environ['DW_TEST_EVENTS'])
                    previous = [json.loads(line)['stage'] for line in events.read_text().splitlines()]
                    self.assertEqual(previous, ['checker', 'regression'])
                    with events.open('a') as stream:
                        stream.write(json.dumps({'stage': 'doctorwatson', 'cwd': str(Path.cwd())}) + '\\n')
                    self.assertNotEqual(os.environ.get('DW_TEST_DELIVERY_FAIL'), '1',
                                        'requested ordinary delivery fixture failure')
            '''), encoding='utf-8')
        # Keep the real shell and Python interpreter; auxiliary CLI stand-ins
        # only record an unexpected launch and return a distinctive failure.
        binaries = self.root / 'bin'
        binaries.mkdir()
        for command in ('bash', 'dirname'):
            executable = shutil.which(command)
            self.assertIsNotNone(executable, f'{command} is required for this shell test')
            (binaries / command).symlink_to(executable)
        (binaries / 'python3').symlink_to(sys.executable)
        for command in ('node', 'quint', 'gh'):
            stub = binaries / command
            stub.write_text(textwrap.dedent('''\
                #!/usr/bin/env python3
                import json
                import os
                from pathlib import Path
                import sys

                with Path(os.environ['DW_TEST_AUXILIARY']).open('a') as stream:
                    stream.write(json.dumps(Path(sys.argv[0]).name) + '\\n')
                raise SystemExit(97)
                '''), encoding='utf-8')
            stub.chmod(0o755)
        self.environment = {
            'PATH': str(binaries),
            'LANG': 'C.UTF-8',
            'TMPDIR': str(self.root),
            'DW_TEST_EVENTS': str(self.events),
            'DW_TEST_AUXILIARY': str(self.auxiliary),
            'DOCTORWATSON_NODE': str(binaries / 'node'),
            'DOCTORWATSON_QUINT': str(binaries / 'quint'),
        }

    def install_host_fixture(self):
        self.host_arguments = self.root / 'host-arguments.json'
        self.environment['DW_TEST_HOST_ARGUMENTS'] = str(self.host_arguments)
        wrapper = self.checkout / 'models/sherlock/qa/run_host.py'
        wrapper.write_text(textwrap.dedent('''\
            import argparse
            import json
            import os
            from pathlib import Path
            import sys

            parser = argparse.ArgumentParser()
            parser.add_argument('--rustc', required=True)
            parser.add_argument('--output-dir', required=True)
            args = parser.parse_args()
            events = Path(os.environ['DW_TEST_EVENTS'])
            previous = [json.loads(line)['stage'] for line in events.read_text().splitlines()]
            if previous != ['checker', 'regression', 'doctorwatson']:
                raise SystemExit('host fixture must follow ordinary QA')
            with events.open('a') as stream:
                stream.write(json.dumps({'stage': 'host', 'cwd': str(Path.cwd())}) + '\\n')
            Path(os.environ['DW_TEST_HOST_ARGUMENTS']).write_text(json.dumps({
                'argv': sys.argv[1:],
                'isolated': sys.flags.isolated,
                'dont_write_bytecode': sys.flags.dont_write_bytecode,
            }))
            if not args.rustc or not args.output_dir:
                parser.error('explicit compiler and output directory are required')
            raise SystemExit(int(os.environ.get('DW_TEST_HOST_EXIT', '0')))
            '''), encoding='utf-8')

    def run_entrypoint(self, *arguments, **environment):
        return subprocess.run(
            [str(self.entrypoint), *arguments], cwd=self.outside,
            env={**self.environment, **environment}, text=True,
            capture_output=True, timeout=15, check=False,
        )

    def recorded(self, path):
        if not path.exists():
            return []
        return [json.loads(line) for line in path.read_text(encoding='utf-8').splitlines()]

    def assert_stages(self, *stages):
        self.assertEqual(self.recorded(self.events), [
            {'stage': stage, 'cwd': str(self.checkout)} for stage in stages
        ])

    def test_default_from_another_directory_runs_checker_then_regression_in_checkout(self):
        result = self.run_entrypoint()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assert_stages('checker', 'regression', 'doctorwatson')
        self.assertEqual(result.stderr.count('Ran 1 test'), 2)

    def test_checker_failure_propagates_and_prevents_regressions(self):
        result = self.run_entrypoint(DW_TEST_CHECKER_EXIT='23')
        self.assertEqual(result.returncode, 23, result.stdout + result.stderr)
        self.assert_stages('checker')

    def test_regression_failure_propagates(self):
        result = self.run_entrypoint(DW_TEST_REGRESSION_FAIL='1')
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assert_stages('checker', 'regression')
        self.assertIn('requested ordinary fixture regression failure', result.stderr)

    def test_default_launches_no_node_quint_or_github_cli(self):
        result = self.run_entrypoint()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assert_stages('checker', 'regression', 'doctorwatson')
        self.assertEqual(self.recorded(self.auxiliary), [])

    def test_delivery_failure_propagates_and_prevents_host(self):
        self.install_host_fixture()
        result = self.run_entrypoint('--host', DOCTORWATSON_RUSTC='explicit-compiler',
                                     DOCTORWATSON_OUTPUT_DIR='explicit-output',
                                     DW_TEST_DELIVERY_FAIL='1')
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assert_stages('checker', 'regression', 'doctorwatson')
        self.assertIn('requested ordinary delivery fixture failure', result.stderr)
        self.assertFalse(self.host_arguments.exists())
        self.assertEqual(self.recorded(self.auxiliary), [])

    def test_unknown_option_returns_usage_before_checker(self):
        result = self.run_entrypoint('--unknown-option')
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertIn('Usage:', result.stderr)
        self.assert_stages()
        self.assertEqual(self.recorded(self.auxiliary), [])

    def test_missing_design_tool_returns_127_before_checker(self):
        for variable in ('DOCTORWATSON_NODE', 'DOCTORWATSON_QUINT'):
            with self.subTest(tool=variable):
                result = self.run_entrypoint('--design', **{
                    variable: str(self.root / 'missing-installed-tool'),
                })
                self.assertEqual(result.returncode, 127, result.stdout + result.stderr)
                self.assertIn('requires an already installed tool', result.stderr)
                self.assert_stages()
                self.assertEqual(self.recorded(self.auxiliary), [])


    def test_host_runs_after_ordinary_qa_with_exact_caller_arguments(self):
        self.install_host_fixture()
        compiler = str(self.root / 'compiler path with spaces')
        output = str(self.root / 'output path with spaces')
        result = self.run_entrypoint('--host', DOCTORWATSON_RUSTC=compiler,
                                     DOCTORWATSON_OUTPUT_DIR=output)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assert_stages('checker', 'regression', 'doctorwatson', 'host')
        self.assertEqual(json.loads(self.host_arguments.read_text()), {
            'argv': ['--rustc', compiler, '--output-dir', output],
            'isolated': 1, 'dont_write_bytecode': 1,
        })
        self.assertEqual(self.recorded(self.auxiliary), [])

    def test_host_wrapper_failure_propagates(self):
        self.install_host_fixture()
        result = self.run_entrypoint('--host', DOCTORWATSON_RUSTC='explicit-compiler',
                                     DOCTORWATSON_OUTPUT_DIR='explicit-output',
                                     DW_TEST_HOST_EXIT='29')
        self.assertEqual(result.returncode, 29, result.stdout + result.stderr)
        self.assert_stages('checker', 'regression', 'doctorwatson', 'host')
        self.assertEqual(self.recorded(self.auxiliary), [])

    def test_host_missing_configuration_reaches_wrapper_preflight(self):
        self.install_host_fixture()
        cases = ({}, {'DOCTORWATSON_RUSTC': 'explicit-compiler'},
                 {'DOCTORWATSON_OUTPUT_DIR': 'explicit-output'})
        for configuration in cases:
            with self.subTest(configuration=configuration):
                self.events.unlink(missing_ok=True)
                self.host_arguments.unlink(missing_ok=True)
                result = self.run_entrypoint('--host', **configuration)
                self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
                self.assert_stages('checker', 'regression', 'doctorwatson', 'host')
                self.assertEqual(json.loads(self.host_arguments.read_text())['argv'], [
                    '--rustc', configuration.get('DOCTORWATSON_RUSTC', ''),
                    '--output-dir', configuration.get('DOCTORWATSON_OUTPUT_DIR', ''),
                ])
                self.assertIn('explicit compiler and output directory', result.stderr)
                self.assertEqual(self.recorded(self.auxiliary), [])

    def test_host_and_design_are_mutually_exclusive_before_checker(self):
        self.install_host_fixture()
        for arguments in (('--host', '--design'), ('--design', '--host')):
            with self.subTest(arguments=arguments):
                result = self.run_entrypoint(*arguments)
                self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
                self.assertIn('Usage:', result.stderr)
                self.assert_stages()
                self.assertFalse(self.host_arguments.exists())
                self.assertEqual(self.recorded(self.auxiliary), [])

if __name__ == '__main__':
    unittest.main()

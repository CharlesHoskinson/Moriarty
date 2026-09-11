"""Offline checks: choosing a suite cannot reuse another suite's K binding."""
import contextlib
import io
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch
import run

HERE = Path(__file__).resolve().parent

class RunnerSuiteTests(unittest.TestCase):
    def test_cli_keeps_proof_unimplemented_for_selected_suite(self):
        result = subprocess.run([sys.executable, str(HERE/'run.py'), '--suite', 'initial', 'prove'], capture_output=True, text=True)
        self.assertEqual(result.returncode, 2)
        self.assertIn('PROOF_UNIMPLEMENTED', result.stdout)

    def test_selected_suite_cannot_use_initial_build(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root/'fixtures').mkdir()
            for name in ['moriarty.k', 'codec.py', 'run.py', 'toolchain.lock.json', 'fixtures/cases.json', 'fixtures/branches.json']:
                (root/name).write_text(name)
            binding = root/'binding.json'
            # Intercept compilation only to exercise CLI selection without a K call.
            def check_binding():
                selected = run.sources()
                self.assertIn('fixtures/branches.json', selected)
                self.assertNotIn('fixtures/cases.json', selected)
                previous = {**selected}
                previous.pop('fixtures/branches.json')
                previous['fixtures/cases.json'] = run.sha(root/'fixtures/cases.json')
                binding.write_text(json.dumps({'sources': previous, 'artifacts': {'compiled': 'fixed'}, 'krunInvocations': 0}))
                with patch.object(run, 'toolchain', return_value={}), patch.object(run, 'artifacts', return_value={'compiled': 'fixed'}), patch.object(run, 'command') as command:
                    with self.assertRaisesRegex(run.RunnerError, 'COMPILED_STALE'):
                        run.evaluate({}, 'never-dispatched')
                    command.assert_not_called()
                    self.assertEqual(json.loads(binding.read_text())['krunInvocations'], 0)
            with patch.object(run, 'HERE', root), patch.object(run, 'BINDING', binding), patch.object(run, 'compile_definition', side_effect=check_binding), patch.object(sys, 'argv', ['run.py', '--suite', 'branches', 'compile']), contextlib.redirect_stdout(io.StringIO()):
                run.main()

if __name__ == '__main__':
    unittest.main()

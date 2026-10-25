"""The generated Loki entry point cannot silently fall back to stock identity."""
import importlib.util
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location('loki', Path(__file__).parents[1] / 'prepare-rollup-loki.py')
loki = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(loki)


class LokiIdentityTests(unittest.TestCase):
    def test_actual_generated_guard_requires_identity_and_refuses_reference_modes(self):
        with tempfile.TemporaryDirectory() as tmp:
            source = Path(tmp) / 'stub.rs'
            source.write_text('type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;\n'
                              'fn error(s: &str) -> Box<dyn std::error::Error> { s.into() }\n'
                              'fn main() -> Result<()> { Ok(()) }\n')
            runner = Path(tmp) / 'runner.rs'
            runner.write_text(loki.render_runner(source, 'Loki CPU test-source', False))
            binary = Path(tmp) / 'runner'
            env = {**os.environ}
            env.pop('SHERLOCK_PROVER_ID', None)
            args = ['rustc', str(runner), '--edition=2024', '-o', str(binary)]
            missing = subprocess.run(args, env=env, capture_output=True)
            self.assertNotEqual(missing.returncode, 0)
            self.assertIn(b'Loki build requires', missing.stderr)
            env['SHERLOCK_PROVER_ID'] = 'Loki CPU test-source'
            built = subprocess.run(args, env=env, capture_output=True)
            self.assertEqual(built.returncode, 0, built.stderr.decode())
            self.assertEqual(subprocess.run([binary, 'identity'], capture_output=True).returncode, 0)
            for mode in ('verify', 'prefix-verify'):
                refused = subprocess.run([binary, mode], capture_output=True)
                self.assertNotEqual(refused.returncode, 0)
                self.assertIn(b'stock reference verifier', refused.stderr)

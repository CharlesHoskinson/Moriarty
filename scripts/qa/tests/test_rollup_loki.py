"""The generated Loki entry point cannot silently fall back to stock identity."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location('loki', Path(__file__).parents[1] / 'prepare-rollup-loki.py')
loki = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(loki)


class LokiIdentityTests(unittest.TestCase):
    def test_source_snapshot_is_independent_and_tampering_changes_its_pins(self):
        with tempfile.TemporaryDirectory() as tmp:
            source = Path(tmp) / 'source'
            source.mkdir()
            (source / 'Cargo.toml').write_text('[package]\nname="test"\n')
            (source / 'src').mkdir()
            (source / 'src/lib.rs').write_text('pub fn original() {}\n')
            pins = loki.snapshot_pins(source)
            destination = Path(tmp) / 'snapshot'
            loki.snapshot_sources(source, destination, pins)
            (source / 'src/lib.rs').write_text('pub fn changed() {}\n')
            self.assertEqual(loki.snapshot_pins(destination), pins)
            (destination / 'src/lib.rs').write_text('pub fn tampered() {}\n')
            self.assertNotEqual(loki.snapshot_pins(destination), pins)

    def test_actual_generated_guard_requires_identity_and_refuses_reference_modes(self):
        with tempfile.TemporaryDirectory() as tmp:
            source = Path(tmp) / 'stub.rs'
            source.write_text('type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;\n'
                              'fn error(s: &str) -> Box<dyn std::error::Error> { s.into() }\n'
                              'fn main() -> Result<()> { Ok(()) }\n')
            runner = Path(tmp) / 'runner.rs'
            scripts = Path(tmp) / 'scripts'
            scripts.mkdir()
            doctor = scripts / 'doctorwatson'
            doctor.write_text('#!/usr/bin/env python3\nimport json, os, sys\n'
                              'print(json.dumps({"args":sys.argv[1:],'
                              '"build":os.environ["SHERLOCK_PREFIX_PROVER_BUILD"],'
                              '"threads":os.environ["RAYON_NUM_THREADS"]}))\n')
            doctor.chmod(0o755)
            runner.write_text(loki.render_runner(source, 'Loki CPU test-source', False,
                                                  (Path(tmp), Path(tmp) / 'output')))
            manifest = Path(tmp) / 'Cargo.toml'
            manifest.write_text('[package]\nname="loki-runner-test"\nversion="0.1.0"\nedition="2024"\n'
                                '[workspace]\n[[bin]]\nname="runner"\npath="runner.rs"\n')
            binary = Path(tmp) / 'target/debug/runner'
            env = {**os.environ}
            env.pop('SHERLOCK_PROVER_ID', None)
            args = ['cargo', 'build', '--offline', '--manifest-path', str(manifest)]
            missing = subprocess.run(args, env=env, capture_output=True)
            self.assertNotEqual(missing.returncode, 0)
            self.assertIn(b'Loki build requires', missing.stderr)
            env['SHERLOCK_PROVER_ID'] = 'Loki CPU test-source'
            built = subprocess.run(args, env=env, capture_output=True)
            self.assertEqual(built.returncode, 0, built.stderr.decode())
            self.assertEqual(subprocess.run([binary, 'identity'], capture_output=True).returncode, 0)
            factory = subprocess.run([binary, 'factory-prefixes'], capture_output=True, text=True)
            self.assertEqual(factory.returncode, 0, factory.stderr)
            forwarded = json.loads(factory.stdout)
            self.assertEqual(forwarded['args'][:3], ['check', 'B5', '--hypothesis'])
            self.assertEqual(forwarded['build'], str(Path(tmp) / 'output/build.json'))
            self.assertEqual(forwarded['threads'], '4')
            self.assertNotEqual(subprocess.run([binary, 'factory-prefixes', 'extra'],
                                              capture_output=True).returncode, 0)
            for mode in ('verify', 'prefix-verify'):
                refused = subprocess.run([binary, mode], capture_output=True)
                self.assertNotEqual(refused.returncode, 0)
                self.assertIn(b'stock reference verifier', refused.stderr)

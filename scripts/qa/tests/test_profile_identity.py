import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[3]


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace('-', '_'), ROOT / 'scripts/qa' / (name + '.py'))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class IdentityTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.profile = load('profile-recursion')

    def test_prepare_real_archive_and_patch(self):
        self.profile.prepare(ROOT, self.profile.DEFAULT_ARCHIVE)

    def test_corrupt_archive(self):
        with tempfile.TemporaryDirectory() as tmp:
            archive = Path(tmp) / 'bad.crate'
            archive.write_bytes(b'synthetic invalid archive')
            with self.assertRaisesRegex(ValueError, 'archive'):
                self.profile.prepare(ROOT, archive)

    def test_patch_is_empty_and_pinned(self):
        manifest = json.loads((ROOT / 'tools/profile/identity.json').read_text())
        patch = ROOT / 'tools/profile' / manifest['patch_file']
        self.assertEqual(hashlib.sha256(patch.read_bytes()).hexdigest(), manifest['patch_sha256'])
        self.assertEqual(patch.read_bytes(), b'')

    def test_patch_tamper(self):
        with tempfile.TemporaryDirectory() as tmp:
            import shutil
            root = Path(tmp)
            shutil.copytree(ROOT / 'tools/profile', root / 'tools/profile')
            (root / 'tools/profile/profile.patch').write_text('tampered')
            with self.assertRaisesRegex(ValueError, 'patch'):
                self.profile.prepare(root, self.profile.DEFAULT_ARCHIVE)

    def test_vendor_tamper(self):
        with tempfile.TemporaryDirectory() as tmp:
            import shutil
            root = Path(tmp)
            shutil.copytree(ROOT / 'tools/profile', root / 'tools/profile')
            (root / 'tools/profile/vendor/midnight-proofs-0.8.3/src/lib.rs').write_text('tampered')
            with self.assertRaisesRegex(ValueError, 'vendor'):
                self.profile.prepare(root, self.profile.DEFAULT_ARCHIVE)

    def test_historical_control_rejected(self):
        sidecar = json.loads((ROOT / 'scripts/qa/tests/fixtures/profile/valid.json').read_text())
        old = copy.deepcopy(sidecar)
        old['identity']['proofs_version'] = '0.8.2'
        with self.assertRaisesRegex(ValueError, 'identity'):
            self.profile.validate_comparison(sidecar, old)
        self.assertEqual(self.profile.validate_comparison(sidecar, old, 'version')['phase_transfer'], False)

    def test_workload_identity_mismatch(self):
        sidecar = json.loads((ROOT / 'scripts/qa/tests/fixtures/profile/valid.json').read_text())
        other = copy.deepcopy(sidecar)
        other['proofs'][0]['srs_sha256'] = '1' * 64
        with self.assertRaisesRegex(ValueError, 'identity'):
            self.profile.validate_comparison(sidecar, other)

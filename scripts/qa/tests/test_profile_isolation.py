from pathlib import Path
import tempfile
import unittest
from test_profile_identity import ROOT, load


class IsolationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.check = load('check-profile-isolation')

    def test_repository_isolated(self):
        self.assertEqual(self.check.check_isolation(ROOT), [])

    def test_seeded_leaks(self):
        leaks = [
            ('Cargo.toml', '[features]\nrecursion-phase-profile = []\n'),
            ('Cargo.toml', '[dependencies]\np = { path = "tools/profile/vendor/midnight-proofs-0.8.3" }\n'),
            ('crates/test/Cargo.toml', '[dependencies]\np = { features = ["recursion-phase-profile"] }\n'),
            ('crates/test/Cargo.toml', '[features]\nx = ["p/recursion-phase-profile"]\n'),
            ('Cargo.lock', '[[package]]\nname = "midnight-proofs"\nversion = "0.8.3"\n'),
            ('Cargo.lock', '[[package]]\nname = "p"\nsource = "path+file:///tmp/tools/profile/vendor/midnight-proofs-0.8.3"\n'),
        ]
        for filename, content in leaks:
            with self.subTest(filename=filename, content=content), tempfile.TemporaryDirectory() as tmp:
                root = Path(tmp)
                (root / 'Cargo.toml').write_text('[workspace]\n')
                (root / 'Cargo.lock').write_text('version = 4\n')
                target = root / filename
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text(content)
                errors = self.check.check_isolation(root)
                self.assertTrue(errors)
                self.assertIn(filename, '\n'.join(errors))

    def test_comments_are_not_features(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / 'Cargo.toml').write_text('# recursion-phase-profile\n[workspace]\n')
            (root / 'Cargo.lock').write_text('version = 4\n')
            self.assertEqual(self.check.check_isolation(root), [])

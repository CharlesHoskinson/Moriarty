"""Recover source bytes only when they satisfy the unchanged cut pins."""
import hashlib
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "models/sherlock/rollup-synth"))
from common import Error, canonical, load_json
from sources import prepare_sources


class PrepareSourcesTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.fenrir = self.root / "fenrir"
        self.fenrir.mkdir()
        self.archive = self.root / "archive"
        self.archive.mkdir()
        self.cache = self.root / "cache"
        self.raw = b"exact historical public source"
        self.name = "~/research/rollup-subarch-2026-10-21/modules/VALIDITY.md"
        self.row = {"path": self.name, "bytes": len(self.raw),
                    "sha256": hashlib.sha256(self.raw).hexdigest()}
        self.cut = self.root / "cut.json"
        groups = {"public": [self.row]}
        self.cut.write_bytes(canonical({"schema": "moriarty-fsyn-design-cut/1", "groups": groups,
            "cut_digest_sha256": hashlib.sha256(canonical(groups)).hexdigest()}))
        self.git = patch("sources.subprocess.run", return_value=subprocess.CompletedProcess([], 128, b"", b"missing"))
        self.git.start()
        self.addCleanup(self.git.stop)

    def prepare(self):
        return prepare_sources(self.cut, self.fenrir, self.cache, self.archive)

    def test_matching_relocated_checkout_source_is_used(self):
        p = self.fenrir / "design/rollup/modules/VALIDITY.md"
        p.parent.mkdir(parents=True)
        p.write_bytes(self.raw)
        self.assertEqual(load_json(self.prepare())[self.name], str(p))

    def test_historical_git_bytes_are_checked_and_cached(self):
        with patch("sources.subprocess.run", return_value=subprocess.CompletedProcess([], 0, self.raw, b"")) as git:
            mapping = load_json(self.prepare())
        self.assertEqual(Path(mapping[self.name]).read_bytes(), self.raw)
        self.assertIn("b5e0437362", git.call_args.args[0][-1])

    def test_stale_git_bytes_do_not_create_a_source_map(self):
        with patch("sources.subprocess.run", return_value=subprocess.CompletedProcess([], 0, b"stale", b"")):
            with self.assertRaisesRegex(Error, self.row["sha256"]):
                self.prepare()
        self.assertFalse((self.cache / "source-map.json").exists())

    def test_archive_recovery_checks_hash(self):
        p = self.archive / "research/rollup-subarch-2026-10-21/modules/VALIDITY.md"
        p.parent.mkdir(parents=True)
        p.write_bytes(self.raw)
        self.assertEqual(Path(load_json(self.prepare())[self.name]).read_bytes(), self.raw)
        p.write_bytes(b"changed archive")
        with self.assertRaises(Error):
            self.prepare()

    def test_cached_source_is_rechecked_before_reuse(self):
        self.cache.mkdir()
        p = self.cache / self.row["sha256"]
        p.write_bytes(self.raw)
        self.assertEqual(load_json(self.prepare())[self.name], str(p))
        p.write_bytes(b"tampered cache")
        with self.assertRaises(Error):
            self.prepare()


if __name__ == "__main__":
    unittest.main()

"""Source relocation must preserve identity and reject stale or aliased pins."""
import hashlib
import os
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "models/sherlock/rollup-synth"))
from common import Error, canonical
from cut import check_cut, source_path
from s0 import generate, verify


class SourceRelocationTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.source = self.root / "relocated"
        self.source.write_bytes(b"pinned public input")
        self.name = str(self.root / "missing-original")
        self.row = {"path": self.name, "bytes": self.source.stat().st_size,
                    "sha256": hashlib.sha256(self.source.read_bytes()).hexdigest()}
        self.groups = {"public": [self.row]}
        self.cut = self.root / "cut.json"
        self.aliases = self.root / "source-map.json"
        self.aliases.write_bytes(canonical({self.name: str(self.source)}))
        self.environment = patch.dict(os.environ, {"FSYN_SOURCE_MAP": str(self.aliases)})
        self.environment.start()
        self.addCleanup(self.environment.stop)
        self.write_cut()

    def write_cut(self):
        self.digest = hashlib.sha256(canonical(self.groups)).hexdigest()
        self.cut.write_bytes(canonical({"schema": "moriarty-fsyn-design-cut/1",
            "groups": self.groups, "cut_digest_sha256": self.digest}))

    def test_relocation_keeps_original_cut_identity(self):
        self.assertEqual(source_path(self.name), self.source)
        report = check_cut(self.cut)
        self.assertEqual(report["cut_digest"], self.digest)
        self.assertEqual(report["files"], 1)
        self.assertEqual(report["bytes"], self.row["bytes"])

    def test_relocated_stale_bytes_are_rejected(self):
        self.source.write_bytes(b"changed")
        with self.assertRaisesRegex(Error, "stale source"):
            check_cut(self.cut)

    def test_missing_map_does_not_fall_back(self):
        self.aliases.unlink()
        with self.assertRaises(Error):
            check_cut(self.cut)

    def test_unmapped_source_retains_original_path(self):
        self.aliases.write_bytes(canonical({}))
        with self.assertRaisesRegex(Error, "not a regular source"):
            check_cut(self.cut)

    def test_relative_map_target_is_rejected(self):
        self.aliases.write_bytes(canonical({self.name: "relative"}))
        with self.assertRaises(Error):
            check_cut(self.cut)

    def test_duplicate_physical_sources_are_rejected(self):
        second = dict(self.row, path=str(self.root / "another-original"))
        self.groups["public"].append(second)
        self.write_cut()
        self.aliases.write_bytes(canonical({self.name: str(self.source), second["path"]: str(self.source)}))
        with self.assertRaisesRegex(Error, "duplicate source row"):
            check_cut(self.cut)

    def test_symlink_target_is_rejected(self):
        alias = self.root / "symlink"
        alias.symlink_to(self.source)
        self.aliases.write_bytes(canonical({self.name: str(alias)}))
        with self.assertRaisesRegex(Error, "not a regular source"):
            check_cut(self.cut)

    def test_unmapped_original_still_checks_without_configuration(self):
        with patch.dict(os.environ, {}, clear=True):
            self.groups["public"][0]["path"] = str(self.source)
            self.write_cut()
            self.assertEqual(check_cut(self.cut)["files"], 1)

    def test_generation_binds_relocated_corpus_authorities(self):
        golden = self.root / "golden"
        golden.mkdir()
        members = {"params-v3.json": b"{}\n", "workloads.json": b"{}\n"}
        members["MANIFEST.json"] = canonical({name: hashlib.sha256(raw).hexdigest()
                                              for name, raw in members.items()})
        rows, aliases = [], {}
        for name, raw in members.items():
            target = golden / name
            target.write_bytes(raw)
            original = str(self.root / "original-golden" / name)
            rows.append({"path": original, "bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest()})
            aliases[original] = str(target)
        self.groups = {"public": rows}
        self.write_cut()
        self.aliases.write_bytes(canonical(aliases))
        out = self.root / "fixture"
        created = generate(self.cut, golden, out)
        self.assertEqual(verify(out, self.cut, golden), created)
        self.assertNotIn(str(golden).encode(), (out / "FIXTURE.json").read_bytes())

    def test_unbound_relocated_corpus_is_rejected(self):
        golden = self.root / "unbound-golden"
        golden.mkdir()
        with self.assertRaisesRegex(Error, "golden authority absent"):
            generate(self.cut, golden, self.root / "fixture")


if __name__ == "__main__":
    unittest.main()

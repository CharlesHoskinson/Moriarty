"""Independent P0/S0 requirements frozen before parent production modules."""
import copy
import hashlib
import json
import os
from pathlib import Path
import tempfile
import unittest

from cut import check_cut, refresh_cut
from common import Error, canonical, load_json
from rng import Stream
from s0 import generate, verify

CUT = Path(os.environ.get("FSYN_DESIGN_CUT", str(Path(__file__).resolve().parents[4] / "openspec/changes/sherlock-synthetic-rollup-data/design-cut.json")))
GOLDEN = Path(os.environ.get("FRTF_V3_ROOT", "/home/charl/projectfenrir/recursion/rollup-test-format/v3"))

class CutTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.source = self.root / "source"
        self.source.write_bytes(b"source")
        self.cut = self.root / "cut.json"
        self.doc = {"schema": "moriarty-fsyn-design-cut/1", "groups": {"public": [{"path": str(self.source), "bytes": 6, "sha256": hashlib.sha256(b"source").hexdigest()}]}}
        self.doc["cut_digest_sha256"] = hashlib.sha256(canonical(self.doc["groups"])).hexdigest()
        self.cut.write_bytes(canonical(self.doc))
    def test_exact_member_check(self):
        self.assertEqual(check_cut(self.cut)["files"], 1)
    def test_changed_source_refuses(self):
        self.source.write_bytes(b"changed")
        with self.assertRaises(Error): check_cut(self.cut)
    def test_digest_tamper_refuses(self):
        self.doc["cut_digest_sha256"] = "0" * 64
        self.cut.write_bytes(canonical(self.doc))
        with self.assertRaises(Error): check_cut(self.cut)
    def test_size_bool_refuses(self):
        self.doc["groups"]["public"][0]["bytes"] = True
        self.cut.write_bytes(canonical(self.doc))
        with self.assertRaises(Error): check_cut(self.cut)
    def test_duplicate_row_refuses(self):
        self.doc["groups"]["public"] *= 2
        self.doc["cut_digest_sha256"] = hashlib.sha256(canonical(self.doc["groups"])).hexdigest()
        self.cut.write_bytes(canonical(self.doc))
        with self.assertRaises(Error): check_cut(self.cut)
    def test_refresh_is_explicit_new_cut(self):
        before = self.cut.read_bytes()
        self.source.write_bytes(b"changed")
        target = self.root / "new.json"
        report = refresh_cut(self.cut, target)
        self.assertEqual(self.cut.read_bytes(), before)
        self.assertEqual(report["changed"], [str(self.source)])
        self.assertEqual(check_cut(target)["files"], 1)
        with self.assertRaises(Error): refresh_cut(target, target)
    def test_duplicate_json_keys_rejected(self):
        self.cut.write_text('{"schema":1,"schema":2}')
        with self.assertRaises(Error): load_json(self.cut)
    def test_nonfinite_json_rejected(self):
        self.cut.write_text('{"value":NaN}')
        with self.assertRaises(Error): load_json(self.cut)
    def test_actual_123_source_baseline(self):
        result = check_cut(CUT)
        self.assertEqual(result["files"], 123)
        self.assertEqual(result["cut_digest"], "884f0f1162ea3589f42a39820b8a7a3dc35051ea8e3c0f1bd924c321d3187bfd")
        baseline = load_json(CUT)["baseline"]
        self.assertEqual(baseline["previous_cut_digest"], "345c4f139c724dbf261e7aed343f4537b8b6c636464e0aa1e4af7246bcb98e02")
        self.assertEqual(len(baseline["omitted_unrecoverable_sources"]), 2)

class StreamTests(unittest.TestCase):
    def test_repeatability_and_chunk_independence(self):
        a = Stream(7, "mix")
        b = Stream(7, "mix")
        self.assertEqual(a.read(81), b.read(1) + b.read(80))
    def test_domains_are_independent(self):
        self.assertNotEqual(Stream(7, "mix").read(64), Stream(7, "schedule").read(64))
    def test_literal_tagged_hash_stream_definition(self):
        import struct
        tag = b"moriarty/fsyn/rng/v1"
        expected = hashlib.sha256(struct.pack("<Q", len(tag)) + tag + struct.pack("<Q", 7) + b"mix" + struct.pack("<Q", 0)).digest()
        self.assertEqual(Stream(7, "mix").read(32), expected)
    def test_negative_length_refused(self):
        with self.assertRaises(Error): Stream(7, "mix").read(-1)

class S0Tests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tmp = tempfile.TemporaryDirectory()
        cls.root = Path(cls.tmp.name)
        cls.fixture = cls.root / "first"
        cls.report = generate(CUT, GOLDEN, cls.fixture)
    @classmethod
    def tearDownClass(cls): cls.tmp.cleanup()
    def test_all_659_golden_members_byte_identical(self):
        manifest = load_json(GOLDEN / "MANIFEST.json")
        self.assertEqual(len(manifest), 659)
        for rel in manifest:
            self.assertEqual((self.fixture / "corpus" / rel).read_bytes(), (GOLDEN / rel).read_bytes(), rel)
    def test_regeneration_is_byte_deterministic(self):
        other = self.root / "second"
        generate(CUT, GOLDEN, other)
        files = lambda root: {str(p.relative_to(root)): p.read_bytes() for p in root.rglob("*") if p.is_file()}
        self.assertEqual(files(self.fixture), files(other))
        baseline = load_json(Path(__file__).resolve().parents[1] / "baseline.json")
        manifest = (self.fixture / "MANIFEST.json").read_bytes()
        self.assertEqual(hashlib.sha256(manifest).hexdigest(), baseline["manifest_sha256"])
        self.assertEqual(json.loads(manifest)["content_digest"], baseline["content_digest"])
        self.assertEqual(self.report["fixture_id"], baseline["fixture_id"])
        self.assertEqual(self.report["files"], baseline["files"])
    def test_replay_and_scope_truth(self):
        self.assertEqual(verify(self.fixture, CUT, GOLDEN)["status"], "internal-replay-pass")
        meta = load_json(self.fixture / "FIXTURE.json")
        self.assertEqual(meta["release_status"], "unreleased")
        self.assertEqual(meta["checks"]["rtf_verify"], "not-run")
        self.assertEqual(meta["checks"]["poseidon_recomputation"], "not-run")
        self.assertEqual(meta["positive_envelopes_rebuilt"], 13)
    def test_tampered_payload_refuses(self):
        path = self.fixture / "corpus/params-v3.json"
        old = path.read_bytes()
        try:
            path.write_bytes(old + b" ")
            with self.assertRaises(Error): verify(self.fixture, CUT, GOLDEN)
        finally: path.write_bytes(old)
    def test_extra_member_refuses(self):
        extra = self.fixture / "unlisted"
        extra.write_bytes(b"x")
        try:
            with self.assertRaises(Error): verify(self.fixture, CUT, GOLDEN)
        finally: extra.unlink()
    def test_manifest_tamper_refuses(self):
        path = self.fixture / "MANIFEST.json"
        old = path.read_bytes()
        try:
            path.write_bytes(old + b" ")
            with self.assertRaises(Error): verify(self.fixture, CUT, GOLDEN)
        finally: path.write_bytes(old)
    def test_unknown_seed_refused_without_output(self):
        target = self.root / "badseed"
        with self.assertRaises(Error): generate(CUT, GOLDEN, target, seed=1)
        self.assertFalse(target.exists())
    def test_existing_destination_not_overwritten(self):
        before = (self.fixture / "MANIFEST.json").read_bytes()
        with self.assertRaises(Error): generate(CUT, GOLDEN, self.fixture)
        self.assertEqual((self.fixture / "MANIFEST.json").read_bytes(), before)

class CliTests(unittest.TestCase):
    def test_real_cli_cut_generate_verify_and_stale_exit(self):
        import contextlib
        import io
        from fsyn import main
        with tempfile.TemporaryDirectory() as tmp:
            fixture = Path(tmp) / "fixture"
            with contextlib.redirect_stdout(io.StringIO()) as stdout:
                self.assertEqual(main(["cut", "check", "--cut", str(CUT)]), 0)
            self.assertEqual(json.loads(stdout.getvalue())["files"], 123)
            with contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(main(["generate", "--cut", str(CUT), "--golden-root", str(GOLDEN), "--out", str(fixture), "--seed", "0x5354524157"]), 0)
                self.assertEqual(main(["verify", "--cut", str(CUT), "--golden-root", str(GOLDEN), "--fixture", str(fixture)]), 0)
            (fixture / "corpus/params-v3.json").write_bytes(b"tampered")
            with contextlib.redirect_stderr(io.StringIO()) as stderr:
                self.assertEqual(main(["verify", "--cut", str(CUT), "--golden-root", str(GOLDEN), "--fixture", str(fixture)]), 1)
            self.assertEqual(json.loads(stderr.getvalue())["status"], "refused")

if __name__ == "__main__": unittest.main()

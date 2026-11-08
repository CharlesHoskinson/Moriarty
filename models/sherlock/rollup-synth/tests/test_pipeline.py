"""Independent P0/S0 requirements on the in-repository golden (SSRD-001, SSRD-003, SSRD-005, SSRD-030)."""
import hashlib
import json
from pathlib import Path
import shutil
import tempfile
import unittest

from common import Error, canonical, load_json
import golden
from rng import Stream
from s0 import generate, verify

GOLDEN = golden.ROOT


def golden_copy(root):
    """A private copy of the in-repository golden and its pin, for tamper tests."""
    tree, pin = Path(root) / "s0", Path(root) / "s0.json"
    shutil.copytree(golden.ROOT, tree)
    shutil.copy(golden.PIN, pin)
    return tree, pin


class GoldenTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.tree, self.pin = golden_copy(self.tmp.name)
    def test_in_repository_golden_pins_match(self):
        result = golden.check()
        self.assertEqual(result["members"], 659)
        self.assertEqual(result["golden_digest"], load_json(golden.PIN)["golden_digest"])
    def test_golden_lives_in_the_repository(self):
        repo = Path(__file__).resolve().parents[4]
        self.assertTrue(golden.ROOT.is_relative_to(repo))
        self.assertTrue(golden.PIN.is_relative_to(repo))
    def test_changed_member_refuses(self):
        path = self.tree / "i1/blocks/000004.json"
        path.write_bytes(path.read_bytes() + b" ")
        with self.assertRaisesRegex(Error, "stale golden member: i1/blocks/000004.json"):
            golden.load(self.tree, self.pin)
    def test_changed_manifest_refuses(self):
        path = self.tree / "MANIFEST.json"
        path.write_bytes(path.read_bytes() + b" ")
        with self.assertRaisesRegex(Error, "MANIFEST.json"): golden.load(self.tree, self.pin)
    def test_missing_member_refuses(self):
        (self.tree / "workloads.json").unlink()
        with self.assertRaises(Error): golden.load(self.tree, self.pin)
    def test_unlisted_member_refuses(self):
        (self.tree / "unlisted.json").write_bytes(b"{}")
        with self.assertRaisesRegex(Error, "unlisted golden member"): golden.load(self.tree, self.pin)
    def test_symlink_member_refuses(self):
        (self.tree / "link.json").symlink_to(self.tree / "params-v3.json")
        with self.assertRaises(Error): golden.load(self.tree, self.pin)
    def test_pin_digest_tamper_refuses(self):
        doc = load_json(self.pin)
        doc["members"] = 658
        self.pin.write_bytes(canonical(doc))
        with self.assertRaisesRegex(Error, "golden pin digest mismatch"): golden.load(self.tree, self.pin)
    def test_duplicate_json_keys_rejected(self):
        self.pin.write_text('{"schema":1,"schema":2}')
        with self.assertRaises(Error): load_json(self.pin)
    def test_nonfinite_json_rejected(self):
        self.pin.write_text('{"value":NaN}')
        with self.assertRaises(Error): load_json(self.pin)

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
        cls.report = generate(cls.fixture)
    @classmethod
    def tearDownClass(cls): cls.tmp.cleanup()
    def test_all_659_golden_members_byte_identical(self):
        manifest = load_json(GOLDEN / "MANIFEST.json")
        self.assertEqual(len(manifest), 659)
        for rel in manifest:
            self.assertEqual((self.fixture / "corpus" / rel).read_bytes(), (GOLDEN / rel).read_bytes(), rel)
    def test_regeneration_is_byte_deterministic(self):
        other = self.root / "second"
        generate(other)
        files = lambda root: {str(p.relative_to(root)): p.read_bytes() for p in root.rglob("*") if p.is_file()}
        self.assertEqual(files(self.fixture), files(other))
        baseline = load_json(Path(__file__).resolve().parents[1] / "baseline.json")
        manifest = (self.fixture / "MANIFEST.json").read_bytes()
        self.assertEqual(hashlib.sha256(manifest).hexdigest(), baseline["manifest_sha256"])
        self.assertEqual(json.loads(manifest)["content_digest"], baseline["content_digest"])
        self.assertEqual(self.report["fixture_id"], baseline["fixture_id"])
        self.assertEqual(self.report["files"], baseline["files"])
        self.assertEqual(baseline["golden_digest"], load_json(golden.PIN)["golden_digest"])
    def test_replay_and_scope_truth(self):
        self.assertEqual(verify(self.fixture)["status"], "internal-replay-pass")
        meta = load_json(self.fixture / "FIXTURE.json")
        self.assertEqual(meta["release_status"], "unreleased")
        self.assertNotIn("rtf_verify", meta["checks"])
        self.assertNotIn("cut_digest", meta)
        self.assertEqual(meta["golden_digest"], load_json(golden.PIN)["golden_digest"])
        self.assertEqual(meta["checks"]["poseidon_recomputation"], "not-run")
        self.assertEqual(meta["positive_envelopes_rebuilt"], 13)
    def test_fixture_metadata_names_no_path_outside_the_repository(self):
        meta = (self.fixture / "FIXTURE.json").read_text()
        self.assertNotIn("~", meta)
        self.assertNotIn("/home/", meta)
        self.assertNotIn("design-cut", meta)
    def test_tampered_payload_refuses(self):
        path = self.fixture / "corpus/params-v3.json"
        old = path.read_bytes()
        try:
            path.write_bytes(old + b" ")
            with self.assertRaises(Error): verify(self.fixture)
        finally: path.write_bytes(old)
    def test_extra_member_refuses(self):
        extra = self.fixture / "unlisted"
        extra.write_bytes(b"x")
        try:
            with self.assertRaises(Error): verify(self.fixture)
        finally: extra.unlink()
    def test_manifest_tamper_refuses(self):
        path = self.fixture / "MANIFEST.json"
        old = path.read_bytes()
        try:
            path.write_bytes(old + b" ")
            with self.assertRaises(Error): verify(self.fixture)
        finally: path.write_bytes(old)
    def test_unknown_seed_refused_without_output(self):
        target = self.root / "badseed"
        with self.assertRaises(Error): generate(target, seed=1)
        self.assertFalse(target.exists())
    def test_existing_destination_not_overwritten(self):
        before = (self.fixture / "MANIFEST.json").read_bytes()
        with self.assertRaises(Error): generate(self.fixture)
        self.assertEqual((self.fixture / "MANIFEST.json").read_bytes(), before)
    def test_stale_golden_refuses_without_output(self):
        with tempfile.TemporaryDirectory() as tmp:
            tree, pin = golden_copy(tmp)
            path = tree / "i1/genesis.json"
            path.write_bytes(path.read_bytes() + b" ")
            target = Path(tmp) / "out"
            with self.assertRaises(Error): generate(target, root=tree, pin=pin)
            self.assertFalse(target.exists())

class CliTests(unittest.TestCase):
    def test_real_cli_golden_generate_verify_and_stale_exit(self):
        import contextlib
        import io
        from fsyn import main
        with tempfile.TemporaryDirectory() as tmp:
            fixture = Path(tmp) / "fixture"
            with contextlib.redirect_stdout(io.StringIO()) as stdout:
                self.assertEqual(main(["golden", "check"]), 0)
            self.assertEqual(json.loads(stdout.getvalue())["members"], 659)
            with contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(main(["generate", "--out", str(fixture), "--seed", "0x5354524157"]), 0)
                self.assertEqual(main(["verify", "--fixture", str(fixture)]), 0)
            (fixture / "corpus/params-v3.json").write_bytes(b"tampered")
            with contextlib.redirect_stderr(io.StringIO()) as stderr:
                self.assertEqual(main(["verify", "--fixture", str(fixture)]), 1)
            self.assertEqual(json.loads(stderr.getvalue())["status"], "refused")
    def test_cli_takes_no_design_cut_or_outside_root(self):
        import contextlib
        import io
        from fsyn import main
        for argv in (["generate", "--out", "x", "--cut", "c"], ["generate", "--out", "x", "--golden-root", "g"],
                     ["cut", "check", "--cut", "c"]):
            with self.subTest(argv=argv), contextlib.redirect_stderr(io.StringIO()):
                with self.assertRaises(SystemExit) as raised: main(argv)
                self.assertEqual(raised.exception.code, 2)

if __name__ == "__main__": unittest.main()

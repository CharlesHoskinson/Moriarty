"""Identity and exit-code requirements on the golden pin (SSRD-001, SSRD-003)."""
import contextlib
import io
import json
from pathlib import Path
import shutil
import tempfile
import unittest
from unittest.mock import patch
from common import canonical, load_json
import golden
from fsyn import main
from s0 import generate, verify

PIN = golden.PIN

def tree_bytes(root):
    return {str(p.relative_to(root)): p.read_bytes() for p in root.rglob("*") if p.is_file()}

class ReviewRegressionTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.original = PIN.read_bytes()
        self.addCleanup(lambda: self.assertEqual(PIN.read_bytes(), self.original))
    def test_equivalent_pin_serializations_have_identical_complete_fixtures(self):
        doc = load_json(PIN)
        first = self.root / "compact.json"
        first.write_bytes(canonical(doc))
        second = self.root / "pretty.json"
        second.write_text(json.dumps(doc, indent=4) + "\n")
        a, b = self.root / "a", self.root / "b"
        left, right = generate(a, pin=first), generate(b, pin=second)
        self.assertEqual(left["fixture_id"], right["fixture_id"])
        self.assertEqual(tree_bytes(a), tree_bytes(b))
        self.assertEqual(verify(a, pin=second)["status"], "internal-replay-pass")
    def test_different_golden_pin_changes_fixture_identity(self):
        doc = load_json(PIN)
        doc["provenance"] = doc["provenance"] + " Review regression."
        doc["golden_digest"] = golden._digest(doc)
        changed = self.root / "changed.json"
        changed.write_bytes(canonical(doc))
        a, b = self.root / "original", self.root / "changed"
        left, right = generate(a), generate(b, pin=changed)
        self.assertNotEqual(left["fixture_id"], right["fixture_id"])
        self.assertNotEqual((a / "MANIFEST.json").read_bytes(), (b / "MANIFEST.json").read_bytes())
    def test_cli_tamper_is_one_and_bad_or_stale_pin_is_two(self):
        fixture = self.root / "fixture"
        generate(fixture)
        target = fixture / "corpus/params-v3.json"
        target.write_bytes(target.read_bytes() + b" ")
        args = ["verify", "--fixture", str(fixture)]
        with contextlib.redirect_stderr(io.StringIO()) as stderr:
            self.assertEqual(main(args), 1)
        self.assertIn("corpus/params-v3.json", json.loads(stderr.getvalue())["error"])
        malformed = self.root / "malformed.json"
        malformed.write_text('{"schema":')
        doc = load_json(PIN)
        doc["manifest"]["sha256"] = "0" * 64
        doc["golden_digest"] = golden._digest(doc)
        stale = self.root / "stale.json"
        stale.write_bytes(canonical(doc))
        for pin in (malformed, stale):
            with self.subTest(pin=pin.name), patch("golden.PIN", pin), patch("s0.PIN", pin):
                with contextlib.redirect_stderr(io.StringIO()): self.assertEqual(main(args), 2)
    def test_stale_golden_member_is_two(self):
        tree = self.root / "s0"
        shutil.copytree(golden.ROOT, tree)
        (tree / "i1/genesis.json").write_bytes(b"{}")
        with patch("s0.ROOT", tree), contextlib.redirect_stderr(io.StringIO()) as stderr:
            self.assertEqual(main(["generate", "--out", str(self.root / "out")]), 2)
        self.assertIn("stale golden member: i1/genesis.json", json.loads(stderr.getvalue())["error"])
        self.assertFalse((self.root / "out").exists())

if __name__ == "__main__": unittest.main()

"""Review-driven requirements frozen before the corrected production edits."""
import contextlib
import hashlib
import io
import json
import os
from pathlib import Path
import tempfile
import unittest
from common import canonical, load_json
from fsyn import main
from s0 import generate, verify

CUT = Path(os.environ.get("FSYN_DESIGN_CUT", str(Path(__file__).resolve().parents[4] / "openspec/changes/sherlock-synthetic-rollup-data/design-cut.json")))
GOLDEN = Path(os.environ.get("FRTF_V3_ROOT", "/home/charl/projectfenrir/recursion/rollup-test-format/v3"))

def tree_bytes(root):
    return {str(p.relative_to(root)): p.read_bytes() for p in root.rglob("*") if p.is_file()}

class ReviewRegressionTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.original = CUT.read_bytes()
        self.addCleanup(lambda: self.assertEqual(CUT.read_bytes(), self.original))
    def test_equivalent_cut_serializations_have_identical_complete_fixtures(self):
        doc = load_json(CUT)
        first = self.root / "compact-cut.json"
        first.write_bytes(canonical(doc))
        doc["retrieved_utc"] = "receipt-only-metadata-differs"
        second = self.root / "pretty-cut.json"
        second.write_text(json.dumps(doc, indent=4) + "\n")
        self.assertNotEqual(hashlib.sha256(first.read_bytes()).hexdigest(), hashlib.sha256(second.read_bytes()).hexdigest())
        a, b = self.root / "a", self.root / "b"
        left, right = generate(first, GOLDEN, a), generate(second, GOLDEN, b)
        self.assertEqual(left["fixture_id"], right["fixture_id"])
        self.assertEqual(tree_bytes(a), tree_bytes(b))
        self.assertNotIn("source_cut_sha256", load_json(a / "FIXTURE.json"))
        self.assertEqual(verify(a, second, GOLDEN)["status"], "internal-replay-pass")
    def test_different_canonical_source_groups_change_fixture_identity(self):
        doc = load_json(CUT)
        source = self.root / "new-declared-public-source.txt"
        source.write_bytes(b"additional declared PUBLIC_SYNTHETIC source")
        doc["groups"]["review_regression"] = [{"path": str(source), "bytes": source.stat().st_size,
            "sha256": hashlib.sha256(source.read_bytes()).hexdigest()}]
        doc["cut_digest_sha256"] = hashlib.sha256(canonical(doc["groups"])).hexdigest()
        changed = self.root / "changed-cut.json"
        changed.write_bytes(canonical(doc))
        a, b = self.root / "original", self.root / "changed"
        left, right = generate(CUT, GOLDEN, a), generate(changed, GOLDEN, b)
        self.assertNotEqual(left["fixture_id"], right["fixture_id"])
        self.assertNotEqual((a / "MANIFEST.json").read_bytes(), (b / "MANIFEST.json").read_bytes())
    def test_cli_tamper_is_one_and_bad_or_stale_cut_is_two(self):
        fixture = self.root / "fixture"
        generate(CUT, GOLDEN, fixture)
        target = fixture / "corpus/params-v3.json"
        target.write_bytes(target.read_bytes() + b" ")
        args = ["verify", "--fixture", str(fixture), "--cut", str(CUT), "--golden-root", str(GOLDEN)]
        with contextlib.redirect_stderr(io.StringIO()) as stderr:
            self.assertEqual(main(args), 1)
        self.assertIn("corpus/params-v3.json", json.loads(stderr.getvalue())["error"])
        malformed = self.root / "malformed.json"
        malformed.write_text('{"schema":')
        args[4] = str(malformed)
        with contextlib.redirect_stderr(io.StringIO()): self.assertEqual(main(args), 2)
        doc = load_json(CUT)
        row = next(iter(doc["groups"].values()))[0]
        row["sha256"] = "0" * 64
        doc["cut_digest_sha256"] = hashlib.sha256(canonical(doc["groups"])).hexdigest()
        stale = self.root / "stale.json"
        stale.write_bytes(canonical(doc))
        args[4] = str(stale)
        with contextlib.redirect_stderr(io.StringIO()): self.assertEqual(main(args), 2)

if __name__ == "__main__": unittest.main()

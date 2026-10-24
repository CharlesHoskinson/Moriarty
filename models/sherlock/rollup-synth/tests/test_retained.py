"""One new retained artifact operation; does not rerun the passing unit prefix."""
import json
import os
from pathlib import Path
import unittest
from s0 import generate, verify

class RetainedArtifactTest(unittest.TestCase):
    @unittest.skipUnless(os.environ.get("FSYN_TEST_ARTIFACT"), "retained output path not selected")
    def test_retained_s0_artifact(self):
        cut = Path(os.environ["FSYN_DESIGN_CUT"])
        golden = Path("/home/charl/projectfenrir/recursion/rollup-test-format/v3")
        out = Path(os.environ["FSYN_TEST_ARTIFACT"])
        created = generate(cut, golden, out)
        replayed = verify(out, cut, golden)
        self.assertEqual(created, replayed)
        self.assertEqual(created["golden_members"], 659)
        self.assertEqual(created["positive_envelopes_rebuilt"], 13)
        print(json.dumps({"operation": "retained-sdkfree-S0", **replayed}, sort_keys=True))

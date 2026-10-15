"""Checked-in registrations must equal the generated pinned form; read-only."""
import json
from pathlib import Path
import shlex
import sys
import unittest

PLUGIN = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(PLUGIN / "scripts"))
from moriarty_dev.bootstrap import inventory, runtime_digest
from moriarty_dev.deployment import REQUIRED_MODULES, definition


class RegistrationPinTests(unittest.TestCase):
    def test_checked_in_registration_is_canonical_for_current_runtime(self):
        # Any runtime Python edit changes the digest; hard-coded old pins fail here.
        digest = runtime_digest(PLUGIN)
        text = (PLUGIN / "hooks/hooks.json").read_text()
        self.assertEqual(json.loads(text), definition(PLUGIN, digest))
        self.assertEqual(text, json.dumps(definition(PLUGIN, digest), indent=2) + "\n")

    def test_every_event_embeds_source_bootstrap_and_single_pin(self):
        bootstrap = (PLUGIN / "scripts/moriarty_dev/bootstrap.py").read_text()
        registered = json.loads((PLUGIN / "hooks/hooks.json").read_text())["hooks"]
        self.assertEqual(set(registered), {"SessionStart", "PreToolUse", "PostToolUse", "Stop"})
        for event, groups in registered.items():
            with self.subTest(event=event):
                tokens = shlex.split(groups[0]["hooks"][0]["command"])
                start = tokens.index("-c")
                self.assertEqual(tokens[start - 1], "-I")
                self.assertEqual(tokens[start + 1], bootstrap)
                # The shell separator follows the event: `... DIGEST Event; else ...`.
                self.assertEqual(tokens[start + 2:start + 4], [runtime_digest(PLUGIN), event + ";"])

    def test_runtime_inventory_is_complete(self):
        self.assertTrue(REQUIRED_MODULES.issubset(inventory(PLUGIN)))


if __name__ == "__main__":
    unittest.main()

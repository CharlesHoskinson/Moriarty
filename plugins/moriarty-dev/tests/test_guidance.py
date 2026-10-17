"""Read-only guidance contracts; no product/native execution or agent simulation.

Scenarios cover the previously observed stale plugin instructions. These checks
verify explicit instructions, not model compliance or roadmap acceptance.
"""
import json
import re
import shutil
import tempfile
import unittest
from pathlib import Path


PLUGIN = Path(__file__).resolve().parents[1]


class CurrentGuidanceTestCase(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.skill = (PLUGIN / "skills/develop/SKILL.md").read_text()
        cls.focus = (PLUGIN / "skills/develop/references/execution-focus.md").read_text()
        cls.readme = (PLUGIN / "README.md").read_text()
        cls.manifest = json.loads((PLUGIN / ".codex-plugin/plugin.json").read_text())

    def test_latest_task_routing_survives_recovered_old_roster(self):
        # Current nonsecurity task must not inherit Sept11 or Oct13 review seats.
        for text in (self.skill, self.focus, self.readme):
            with self.subTest(document=text.splitlines()[0]):
                self.assertIn("gpt-6.1-sol", text)
                self.assertIn("fresh non-author Opus 5.5", text)
                self.assertIn("high effort", text)
        self.assertNotIn("Both Fresh Audits", self.skill)
        self.assertNotIn("Both selected auditors", self.skill)

    def test_requested_reconciliation_beats_historical_i2_queue(self):
        self.assertIn("current request before any recovered queue", self.skill)
        self.assertIn("historical campaign", self.skill)
        self.assertNotIn("For the current financial bottleneck, target **I2/MC02", self.skill)
        self.assertNotIn("The near-term target is I2/MC02", self.focus)
        prompt = self.manifest["interface"]["defaultPrompt"]
        self.assertIn("current request", prompt)
        self.assertIn("historical", prompt)

    def test_current_requirements_are_loaded_before_task_selection(self):
        for source in ("ROADMAP.md", "AUDIT-PLAN.md", "MID-ROADMAP-REVIEW.md",
                       "traceability.md", "spec.md", "wiki/index.md",
                       "wiki-llm/README.md", "wiki-llm/RECONCILIATION-2026-10-14.md",
                       "openspec/current-requirement-crosswalk.json",
                       "openspec/changes/consolidated-language-kernel/reconciliation-2026-10-14.md"):
            with self.subTest(source=source):
                self.assertIn(source, self.skill + self.focus)
        self.assertIn("EARS", self.focus)

    def test_local_success_does_not_close_parent_exit(self):
        for text in (self.skill, self.focus):
            self.assertIn("implementation, evidence and acceptance", text)
            self.assertIn("U0–U7", text)
            self.assertIn("M1–M17", text)
        self.assertIn("no whole-language percentage", self.focus)
        self.assertIn("captured-local", self.focus)
        self.assertIn("independent financial/work", self.focus)

    def test_profile_and_target_evidence_are_distinct(self):
        for value in ("moriarty/1", "moriarty-core/1", "Source/6", "Core/5",
                      "mori-backend", "mori-ledger", "Preview", "Fenrir"):
            self.assertIn(value, self.focus)
        self.assertIn("two positive", self.focus)
        self.assertIn("refusal", self.focus)
        self.assertIn("unknown", self.focus)

    def test_guidance_is_not_a_new_runtime_admission_system(self):
        self.assertIn("does not enforce the reviewer roster", self.skill)
        self.assertIn("historical", self.readme)
        self.assertIn("no new campaign", self.focus)
        self.assertIn("do not reset", self.focus)

    def test_release_metadata_reflects_guidance_update(self):
        self.assertEqual(self.manifest["version"], "0.1.1")
        self.assertIn("roadmap", self.manifest["description"].lower())

    def test_checkout_links_resolve_and_skill_stays_compact(self):
        self.assertLess(len(self.skill.split()), 750)
        for path, text in ((PLUGIN / "skills/develop/SKILL.md", self.skill),
                           (PLUGIN / "skills/develop/references/execution-focus.md", self.focus)):
            for target in re.findall(r"\]\(([^)#]+)(?:#[^)]*)?\)", text):
                if not target.startswith(("https:", "http:")):
                    with self.subTest(target=target):
                        self.assertTrue((path.parent / target).is_file(), target)

    def test_installed_package_links_need_no_checkout_ancestors(self):
        # Codex caches this package alone, outside the repository. A link that
        # works only because checkout ancestors exist fails the real install.
        with tempfile.TemporaryDirectory() as directory:
            package = Path(directory) / "moriarty-dev"
            shutil.copytree(PLUGIN, package, ignore=shutil.ignore_patterns("__pycache__"))
            guidance = [package / "skills/develop/SKILL.md"]
            guidance.extend((package / "skills/develop/references").rglob("*.md"))
            for document in guidance:
                for target in re.findall(r"\]\(([^)#]+)(?:#[^)]*)?\)", document.read_text()):
                    if target.startswith(("https:", "http:")):
                        continue
                    with self.subTest(document=document.relative_to(package), target=target):
                        resolved = (document.parent / target).resolve()
                        self.assertTrue(resolved.is_relative_to(package.resolve()), target)
                        self.assertTrue(resolved.is_file(), target)


if __name__ == "__main__":
    unittest.main()

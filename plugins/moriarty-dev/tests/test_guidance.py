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
        cls.rust = (PLUGIN / "skills/develop/references/rust.md").read_text()
        cls.readme = (PLUGIN / "README.md").read_text()
        cls.manifest = json.loads((PLUGIN / ".codex-plugin/plugin.json").read_text())

    def test_skill_points_at_factory_loop(self):
        self.assertIn("AGENTS.md", self.skill)
        self.assertIn("scripts/sherlock-next", self.skill)
        prompt = self.manifest["interface"]["defaultPrompt"]
        self.assertIn("AGENTS.md", prompt)
        self.assertIn("scripts/sherlock-next", prompt)

    def test_skill_carries_no_process_rules(self):
        for phrase in ("independent review", "substantive agreeing", "two substantive",
                       "consumed", "must load", "design vote", "admission",
                       "reviewer", "Opus", "Muse"):
            with self.subTest(phrase=phrase):
                self.assertNotIn(phrase, self.skill)

    def test_profile_reference_keeps_technical_content(self):
        for value in ("moriarty/1", "moriarty-core/1", "Source/6", "Core/5",
                      "mori-backend", "mori-ledger", "Preview", "Fenrir"):
            self.assertIn(value, self.focus)

    def test_readme_points_at_factory_loop(self):
        self.assertIn("AGENTS.md", self.readme)
        self.assertIn("scripts/sherlock-next", self.readme)

    def test_release_metadata(self):
        self.assertEqual(self.manifest["version"], "0.1.2")
        self.assertIn("factory", self.manifest["description"].lower())

    def test_rust_changes_follow_the_checkout_guidelines(self):
        for text in (self.skill, self.readme):
            with self.subTest(document=text.splitlines()[0]):
                self.assertIn("RUST_GUIDELINES.md", text)
                self.assertIn("references/rust.md", text)
        self.assertIn("RUST_GUIDELINES.md", self.rust)
        self.assertIn("crates/", self.rust)
        for check in ("cargo fmt --all --check",
                      "cargo clippy --workspace --all-targets --locked -- -D warnings",
                      "cargo test --workspace"):
            self.assertIn(check, self.rust)
        for rule in ("MoriDiagnostic", "[workspace.dependencies]", "crates/fixtures/",
                     "Never snapshot `--help`"):
            self.assertIn(rule, self.rust)

    def test_checkout_links_resolve_and_skill_stays_compact(self):
        self.assertLess(len(self.skill.split()), 200)
        for path, text in ((PLUGIN / "skills/develop/SKILL.md", self.skill),
                           (PLUGIN / "skills/develop/references/execution-focus.md", self.focus),
                           (PLUGIN / "skills/develop/references/rust.md", self.rust)):
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

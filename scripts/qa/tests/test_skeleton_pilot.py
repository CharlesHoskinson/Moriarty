"""Skeleton-coding pilot driver: sandbox stripping, row names and the blindness audit."""
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path

SPEC = importlib.util.spec_from_file_location(
    "skeleton_pilot", Path(__file__).resolve().parents[1] / "skeleton_pilot.py")
sp = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(sp)


class StripModsTests(unittest.TestCase):
    def test_removes_test_module_by_path_and_by_name(self):
        src = "\n".join([
            "pub mod keep;",
            "#[cfg(test)]",
            '#[path = "c6_x_tests.rs"]',
            "mod c6_x_tests;",
            "#[cfg(test)]",
            "mod c6_dispatch_tests;",
            "#[cfg(test)]",
            "mod other_tests;",
        ])
        out = sp.strip_mods(src, {"c6_x_tests.rs", "c6_dispatch_tests.rs"})
        self.assertEqual(out, "pub mod keep;\n#[cfg(test)]\nmod other_tests;")

    def test_keeps_unrelated_attributes(self):
        src = "#[cfg(test)]\nmod a_tests;\nfn f() {}"
        self.assertEqual(sp.strip_mods(src, set()), src)


class NamesTests(unittest.TestCase):
    def test_dotted_row_name(self):
        self.assertEqual(sp.dotted("KAdminPublishValue"), "k.admin.publish.value")
        self.assertEqual(sp.dotted("BHelperScalarConstruct"), "b.helper.scalar.construct")

    def test_mentions_is_case_insensitive_only_for_row_names(self):
        pats = ["KMachineInvokeFind", "k.machine.invoke.find"]
        self.assertTrue(sp.mentions("// ROW k.machine.invoke.find", pats))
        self.assertTrue(sp.mentions("// b.helper.Scalar.construct", ["b.helper.scalar.construct"]))
        self.assertFalse(sp.mentions("kmachineinvokefind", pats))

    def test_every_component_has_an_emitter_and_rules(self):
        self.assertEqual(len(sp.COMPONENTS), 6)
        for name, (emitter, rules) in sp.COMPONENTS.items():
            self.assertTrue(emitter.startswith("C6") and emitter.endswith("Emitter"), name)
            self.assertTrue(rules, name)


class AuditTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.sandbox = Path(self.tmp.name) / "lane"
        self.sandbox.mkdir()

    def events(self, *cmds):
        return "\n".join(json.dumps({"type": "item.completed",
                                     "item": {"type": "command_execution", "command": c}})
                         for c in cmds)

    def test_clean_lane_has_no_violation(self):
        text = self.events(f"sed -n 1,40p {self.sandbox}/crates/x.rs",
                           "cargo check -p mori-backend --lib --offline")
        self.assertEqual(sp.audit_events(text, self.sandbox, "c6_admin_publish"), [])

    def test_outside_path_git_and_hidden_file_are_flagged(self):
        text = self.events("cat /home/hoskinson/Projects/Moriarty/crates/a.rs",
                           "git log -p", "cat c6_admin_publish_tests.rs",
                           "cat /tmp/claude-1000/x/scratchpad/grader/crates/a.rs")
        v = sp.audit_events(text, self.sandbox, "c6_admin_publish")
        self.assertEqual(len(v), 4)
        self.assertIn("outside path", v[0]["why"][0])
        self.assertIn("forbidden command", v[1]["why"])
        self.assertIn("hidden file", v[2]["why"][0])

    def test_vendor_skills_and_own_tool_results_are_not_violations(self):
        home = Path.home()
        text = self.events(f"cat {home}/.agents/skills/rust-dev/SKILL.md",
                           f"cat {home}/.claude/projects/-tmp-x-lane/abc/tool-results/b1.txt")
        self.assertEqual(sp.audit_events(text, self.sandbox, "c6_admin_publish"), [])

    def test_claude_tool_use_is_read(self):
        e = {"type": "assistant", "message": {"content": [
            {"type": "tool_use", "input": {"file_path": "/home/hoskinson/Projects/other/x"}}]}}
        v = sp.audit_events(json.dumps(e), self.sandbox, "c6_admin_publish")
        self.assertEqual(len(v), 1)


class AdaptorTests(unittest.TestCase):
    def test_adaptor_names_the_component_emitter(self):
        a = sp.adaptor("c6_call_bind")
        self.assertIn("skeleton_pilot_cand_step::C6CallBindEmitter::new(scope)", a)


if __name__ == "__main__":
    unittest.main()

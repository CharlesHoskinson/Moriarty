"""F17 (SWF-014): the T0 guard names every tracked Fenrir or research-tree path read."""
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / 'check-no-fenrir.sh'
# Built from parts so that this test file does not itself trip the guard.
CHECKOUT = 'project' + 'fenrir'
RESEARCH = 'rollup' + '-subarch'


class NoFenrirGuardTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        subprocess.run(['git', 'init', '-q', str(self.root)], check=True)
        (self.root / 'scripts/qa').mkdir(parents=True)
        shutil.copy(SCRIPT, self.root / 'scripts/qa/check-no-fenrir.sh')
        self.write('crates/demo/src/lib.rs', 'pub fn clean() {}\n')

    def write(self, name, text, track=True):
        path = self.root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text)
        if track:
            subprocess.run(['git', '-C', str(self.root), 'add', name], check=True)

    def guard(self):
        return subprocess.run(['bash', str(self.root / 'scripts/qa/check-no-fenrir.sh')], cwd=self.root,
                              capture_output=True, text=True, timeout=60)

    def test_clean_tree_passes(self):
        result = self.guard()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_fenrir_corpus_load_fails_naming_file_and_line(self):
        self.write('crates/demo/tests/rollup.rs',
                   'fn corpus() {}\nconst P: &str = "../../../' + CHECKOUT + '-main/recursion/rollup-test-format/v3";\n')
        result = self.guard()
        self.assertEqual(result.returncode, 1)
        self.assertIn('crates/demo/tests/rollup.rs:2:', result.stdout)

    def test_research_tree_in_each_guarded_root_fails(self):
        for root in ('tools', 'scripts', 'models', 'qa'):
            self.write(root + '/x.py', 'SOURCE = "~/research/' + RESEARCH + '-2026-10-21/x"\n')
        self.write('qa/sherlock/predicates.yaml', 'check: "' + CHECKOUT + '"\n')
        result = self.guard()
        self.assertEqual(result.returncode, 1)
        for name in ('tools/x.py:1:', 'scripts/x.py:1:', 'models/x.py:1:', 'qa/x.py:1:',
                     'qa/sherlock/predicates.yaml:1:'):
            self.assertIn(name, result.stdout)

    def test_holdout_history_untracked_and_unguarded_roots_are_ignored(self):
        self.write('qa/holdout/old.json', CHECKOUT + '\n')
        self.write('qa/sherlock/runs.jsonl', '{"hypothesis": "' + CHECKOUT + '"}\n')
        self.write('docs/history.md', CHECKOUT + '\n')
        self.write('crates/demo/notes.txt', CHECKOUT + '\n', track=False)
        result = self.guard()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)


if __name__ == '__main__':
    unittest.main()

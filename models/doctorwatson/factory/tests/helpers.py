import json
import os
import subprocess
import sys
import tempfile
import unittest

PKG_PARENT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))


class FakeRepo(unittest.TestCase):
    """Temp-dir fake repo driven through `python3 -m factory` with DW_REPO."""

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.repo = os.path.join(self._tmp.name, "repo")
        os.makedirs(os.path.join(self.repo, "qa", "sherlock"))
        self.lock = os.path.join(self._tmp.name, "heavy.lock")
        self.env = dict(os.environ, DW_REPO=self.repo, DW_LOCK=self.lock, DW_SKIP_PREFLIGHT="1")
        self.env.pop("DW_ROLE", None)

    def p(self, *parts):
        return os.path.join(self.repo, *parts)

    def write(self, rel, text, mode=None):
        path = self.p(rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w") as fh:
            fh.write(text)
        if mode:
            os.chmod(path, mode)

    def predicates(self, text):
        self.write("qa/sherlock/predicates.yaml", text)

    def run_cli(self, *args, env=None, timeout=60, input=None):
        e = dict(self.env)
        e.update(env or {})
        return subprocess.run([sys.executable, "-m", "factory", *args], cwd=PKG_PARENT, env=e,
                              capture_output=True, text=True, timeout=timeout, input=input)

    def status(self):
        with open(self.p("qa/sherlock/status.json")) as fh:
            return json.load(fh)

    def runs(self):
        path = self.p("qa/sherlock/runs.jsonl")
        if not os.path.exists(path):
            return []
        with open(path) as fh:
            return [json.loads(line) for line in fh if line.strip()]

    def budgets(self):
        with open(self.p("qa/sherlock/budgets.json")) as fh:
            return json.load(fh)

    def seed_budget(self, key, measured_wall=None, cap_wall=None, cap_rss=None, rss=None):
        with open(self.p("qa/sherlock/budgets.json"), "w") as fh:
            json.dump({"schema": "doctorwatson.budgets/1", "budgets": {key: {
                "measured": {"wallSeconds": measured_wall, "maxRssBytes": rss, "k": 20, "at": None, "run": None},
                "cap": {"wallSeconds": cap_wall, "maxRssBytes": cap_rss}}}}, fh)


def pred(id, set="A", tier="T0", check="true", title=None, **kw):
    lines = [f"  - id: {id}", f"    set: {set}", f'    title: "{title or "title " + id}"',
             f"    tier: {tier}", f'    check: "{check}"']
    for k, v in kw.items():
        if isinstance(v, bool):
            v = "true" if v else "false"
        elif isinstance(v, (list, tuple)):
            v = "[" + ", ".join(v) + "]"
        elif isinstance(v, str):
            v = f'"{v}"'
        lines.append(f"    {k}: {v}")
    return "\n".join(lines)


def yaml_of(*preds):
    return "version: 1\npredicates:\n" + "\n".join(preds) + "\n"

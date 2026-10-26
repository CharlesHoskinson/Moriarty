"""Qualification capacities must preserve the draft owner's halted outcomes."""
import json
import copy
from pathlib import Path
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
QUALIFICATION = ROOT / "docs/sherlock/qualification"
CONFIG = QUALIFICATION / "qualification-host-config.json"
DRAFT = ROOT / "qa/holdout/config/draft-host-config.json"
PROGRAMS = {
    "literal": 'language "moriarty-sherlock-draft/1"; program P { entry 7; }',
    "addition": 'language "moriarty-sherlock-draft/1"; program P { entry 3 + 4; }',
    **{name: (QUALIFICATION / "programs" / f"{name}.mori").read_text()
       for name in ("fact3", "list123")},
}


def simulate(source, config):
    with tempfile.TemporaryDirectory(prefix="sherlock-qualification-") as directory:
        path = Path(directory)
        (path / "program.mori").write_text(source)
        (path / "config.json").write_text(json.dumps(config))
        (path / "target.json").write_text("{}")
        result = subprocess.run(
            [str(ROOT / "target/debug/mori"), "simulate", str(path / "program.mori"),
             "--execution-profile", "sherlock-helper-microstep/1",
             "--source-profile", "moriarty-sherlock-draft/1", "--entry",
             "--execution-config", str(path / "config.json"),
             "--target", str(path / "target.json")],
            capture_output=True, text=True, timeout=120,
        )
        if result.returncode:
            raise AssertionError(f"simulate: {result.stdout}\n{result.stderr}")
        return json.loads(result.stdout)


class QualificationConfig(unittest.TestCase):
    def test_identical_halted_outcomes(self):
        qualification = json.loads(CONFIG.read_text())
        draft = json.loads(DRAFT.read_text())
        for name, source in PROGRAMS.items():
            with self.subTest(program=name):
                baseline = simulate(source, draft)["execution"]
                document = simulate(source, qualification)
                tight = document["execution"]
                self.assertTrue(tight["completed"])
                self.assertIn("Halted", tight["summary"]["outcome"])
                self.assertEqual(baseline["summary"], tight["summary"])
                # References in the outcome must resolve to the same heap contents.
                for field in ("data", "data_fields"):
                    self.assertEqual(baseline["image"][field], tight["image"][field])
                ref = tight["summary"]["outcome"]["Halted"]["value"]
                space, index = next(iter(ref.items()))
                table = (document["preparation"]["admission"]["program_view"]["constants"]
                         if space == "Const" else tight["image"]["data"])
                value = int(table[int(index)]["U128"]["n"])
                self.assertEqual(value, 7 if name in ("literal", "addition") else 6)
                print(name, "result", value, json.dumps(tight["summary"], sort_keys=True))

    def test_one_slot_margin_and_coordinate_minima(self):
        minimum = json.loads(CONFIG.read_text())
        paths = [("bounds", key) for key in minimum["bounds"]]
        paths += [("layout", "capacities", key) for key in minimum["layout"]["capacities"]]
        paths += [(key, "value") for key in ("fuel", "stack")]

        def table(config, path):
            for key in path[:-1]:
                config = config[key]
            return config

        for path in paths:
            row = table(minimum, path)
            row[path[-1]] = str(int(row[path[-1]]) - 1)
        expected = {name: simulate(source, minimum)["execution"]["summary"]
                    for name, source in PROGRAMS.items()}
        self.assertTrue(all("Halted" in s["outcome"] for s in expected.values()))
        for path in paths:
            reduced = copy.deepcopy(minimum)
            row = table(reduced, path)
            if row[path[-1]] == "0":
                continue
            row[path[-1]] = str(int(row[path[-1]]) - 1)
            with self.subTest(capacity=".".join(path)):
                same = True
                for name, source in PROGRAMS.items():
                    try:
                        same &= simulate(source, reduced)["execution"]["summary"] == expected[name]
                    except (AssertionError, KeyError):
                        same = False
                    if not same:
                        break
                self.assertFalse(same, "capacity can shrink further without changing outcomes")


if __name__ == "__main__":
    unittest.main()

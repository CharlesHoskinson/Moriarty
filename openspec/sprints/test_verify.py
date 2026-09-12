#!/usr/bin/env python3
"""Exercise planning retention through the real validator with corrupt copies."""
import json
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


class RetentionTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        shutil.copytree(ROOT / 'openspec', self.root / 'openspec')
        coverage = self.load('coverage.json')
        reports = self.load('report-lessons.json')
        gates = self.load('legacy-release-gates.json')
        paths = set(coverage['sourceInventories']) | set(coverage['sourceDigests'])
        paths |= set(reports['sourceDigests']) | set(gates['sourceSha256'])
        paths |= set(self.load('pcd-integration.json')['sourceDigests'])
        for path in paths:
            dest = self.root / path
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / path, dest)

    def load(self, name):
        return json.loads((self.root / 'openspec/sprints' / name).read_text())

    def save(self, name, value):
        (self.root / 'openspec/sprints' / name).write_text(json.dumps(value))

    def verify(self):
        return subprocess.run(['python3', str(self.root / 'openspec/sprints/verify.py')],
                              capture_output=True, text=True)

    def rejects(self, diagnostic):
        result = self.verify()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(diagnostic, result.stdout + result.stderr)

    def test_valid_plan(self):
        result = self.verify()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_missing_original_task(self):
        data = self.load('package-task-map.json')
        data['rows'].pop()
        self.save('package-task-map.json', data)
        self.rejects('Original task crosswalk differs')

    def test_rewritten_original_task(self):
        data = self.load('package-task-map.json')
        data['rows'][0]['originalText'] = 'Silently replace the original requirement'
        self.save('package-task-map.json', data)
        self.rejects('Original task text differs')

    def test_nonexistent_report_owner(self):
        data = self.load('report-lessons.json')
        data['lessons'][0]['tasks'] = ['SP99.1']
        self.save('report-lessons.json', data)
        self.rejects('Unknown report task owner')

    def test_dropped_report_regression(self):
        data = self.load('report-lessons.json')
        data['modeledRegressions'].pop()
        self.save('report-lessons.json', data)
        self.rejects('Report regression identities differ')

    def test_rewritten_legacy_gate(self):
        data = self.load('legacy-release-gates.json')
        data['gates'][11]['originalPredicate'] = 'One build is enough'
        self.save('legacy-release-gates.json', data)
        self.rejects('Legacy gate text differs')

    def test_stale_report_source(self):
        path = self.root / 'deliverables/modern-defi-taxonomy-2026-09-08/cases.json'
        path.write_text(path.read_text() + '\n')
        self.rejects('Stale report source')

    def test_dropped_sprint_program_requirement(self):
        data = self.load('coverage.json')
        data['requirements'] = [r for r in data['requirements'] if r['package'] != 'SPRINT-PROGRAM']
        self.save('coverage.json', data)
        self.rejects('Requirement crosswalk differs')

    def set_stage_requires(self, stage, requires):
        path = self.root / 'openspec/moriarty-completion-program.json'
        program = json.loads(path.read_text())
        for record in program['reportReconciliation']['stageAdmission']['stages']:
            if record['id'] == stage:
                record['requires'] = requires
        path.write_text(json.dumps(program))
        sprints = self.load('sprints.json')
        for sprint in sprints['sprints']:
            for gate in sprint['entryGates']:
                if gate['stage'] == stage:
                    gate['requires'] = requires
        self.save('sprints.json', sprints)

    def test_core_depends_on_certificate_stage(self):
        self.set_stage_requires('f3', ['atomic-accept', 'i2', 'f2', 'f1'])
        self.rejects('PCD core depends on certificate stage')

    def test_release_lost_certificate_stage(self):
        self.set_stage_requires('release', ['atomic-accept', 'i2', 'f3', 'mandatory', 'composition', 'finance'])
        self.rejects('PCD release lost certificate stage')

    def test_unknown_pcd_crosswalk_reference(self):
        data = self.load('pcd-integration.json')
        data['stageMapping'][0]['tasks'] = ['SP99.9']
        self.save('pcd-integration.json', data)
        self.rejects('Unknown PCD crosswalk reference')


if __name__ == '__main__':
    unittest.main()

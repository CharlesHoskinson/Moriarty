"""Real ordinary checker reporting tests; no evaluator, compiler or child runs."""
import contextlib
import copy
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import re
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]
SPEC = importlib.util.spec_from_file_location('qa_requirements_under_test', HERE / 'check.py')
qa = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(qa)


def pinned(path):
    return {'path': str(path), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest()}


class RequirementEvidenceTests(unittest.TestCase):
    def setUp(self):
        # Synthetic audit/closed declarations test reporting, never real UIREQ acceptance.
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.source = self.root / 'ordinary-source.txt'
        self.source.write_text('original ordinary source\n')
        self.audit = self.root / 'synthetic-audit.txt'
        self.audit.write_text('synthetic reporting fixture only; no semantic acceptance\n')
        catalogue = REPO / 'openspec/changes/sherlock-completion-convergence/traceability.json'
        trace = json.loads(catalogue.read_text())
        original = REPO / 'openspec/changes/sherlock-universal-interpreter-design/specs/universal-interpreter/spec.md'
        refinements = REPO / 'openspec/changes/sherlock-completion-convergence/specs/sherlock-completion/spec.md'
        def row(item, path, closed=False):
            section = next(part for part in re.split(r'(?m)^### Requirement: ', path.read_text())[1:]
                           if part.startswith(item['id']+' '))
            return dict(id=item['id'],
                requirementSource=dict(**pinned(path), heading=section.splitlines()[0],
                                       scenarioNames=re.findall(r'(?m)^#### Scenario: (.+)$', section)),
                predicate=dict(id='fixture-'+item['id'], description='report fixture only',
                               domain='synthetic report test', applicableEvidenceClasses=['source-review']),
                implementation=dict(status='complete' if closed else 'partial', supportedScope='fixture', missing='real scope unassessed'),
                validation=dict(status='observed' if closed else 'partial', evidenceRefs=[pinned(self.audit)], countercaseRefs=[], reviewRefs=[pinned(self.audit)]),
                acceptance=dict(status='closed' if closed else 'open', scope='synthetic fixture only', reason='test declaration'),
                currentness=dict(sourcePins=[pinned(self.source)], comparedToCurrent='identical', limitations=['declared inputs only']),
                nextDiscriminatingCheck=dict(consumer='ordinary fixture', input='fixture', expected='report', missingAuthorityOrOwner='none'),
                citations=[dict(**pinned(self.audit), startLine=1, endLine=1, claim='synthetic fixture')])
        self.ledger = dict(schema='sherlock.current-requirement-evidence/1',
            requirementCatalogue=pinned(original),
            requirements=[row(item, original, index==0) for index,item in enumerate(trace['original35'])],
            refinementInventory=dict(**pinned(refinements), featureIds=[x['id'] for x in trace['refinementFeatures']], scenarioCount=68),
            refinementRequirements=[row(item, refinements) for item in trace['refinementFeatures']],
            retainedInventory=[dict(family='synthetic retained family', catalogue=pinned(original), status='unassessed')],
            assessment=dict(status='PROPOSED_FOR_ROOT_SEMANTIC_ADOPTION', globalV1='open', adoption='pending-root'))
        self.path = self.root / 'current-evidence.json'

    def report(self):
        self.path.write_text(json.dumps(self.ledger))
        return qa.run(REPO, requirement_evidence=self.path)

    def test_actual_consumer_never_reports_stale_source_as_currently_closed(self):
        before = self.report()
        self.assertEqual(before['status'], 'pass')
        self.assertEqual(before['requirement_evidence']['summary']['original35']['current_owner_declared_closed'], 1)
        self.source.write_text('changed ordinary source\n')
        after = self.report()
        self.assertEqual(after['status'], 'mismatch')
        report = after['requirement_evidence']
        self.assertEqual(report['summary']['original35']['current_owner_declared_closed'], 0)
        first = report['requirements'][0]
        self.assertEqual(first['declared']['acceptance']['status'], 'closed')
        self.assertEqual(first['integrity'], 'stale')
        self.assertEqual(first['current_reportability'], 'not-current')
        self.assertTrue(any(x['path']==str(self.source) and x['expected_sha256']!=x['actual_sha256'] for x in first['findings']))
        self.assertEqual(len(report['requirements']), 35)
        self.assertEqual(len(report['refinement_requirements']), 25)
        self.assertEqual(report['inventory']['scenario_count'], 68)

    def test_pending_root_is_visible_and_never_adopted_closure(self):
        report = self.report()['requirement_evidence']
        self.assertEqual(report['summary']['original35']['current_adopted_closed'], 0)
        self.assertEqual(report['assessment']['adoption'], 'pending-root')
        self.assertIn('do not authenticate audits', report['semantic_assessment'])

    def test_current_open_unrun_rows_pass_integrity_without_closing(self):
        for row in self.ledger['requirements'] + self.ledger['refinementRequirements']:
            row['acceptance']['status'] = 'open'
            row['validation'] = dict(status='unrun', evidenceRefs=[], countercaseRefs=[], reviewRefs=[])
        report = self.report()
        self.assertEqual(report['status'], 'pass')
        self.assertEqual(report['requirement_evidence']['summary']['original35']['current_owner_declared_closed'], 0)
        self.assertEqual(report['requirement_evidence']['summary']['original35']['declared_validation']['unrun'], 35)

    def test_missing_review_does_not_support_current_closed(self):
        self.ledger['requirements'][0]['validation']['reviewRefs'][0]['path'] = str(self.root / 'absent-review')
        report = self.report()
        self.assertEqual(report['status'], 'mismatch')
        self.assertEqual(report['requirement_evidence']['requirements'][0]['integrity'], 'missing')
        self.assertEqual(report['requirement_evidence']['summary']['original35']['current_owner_declared_closed'], 0)

    def test_named_refinement_scenario_loss_fails_with_same_aggregate_68(self):
        self.ledger['refinementRequirements'][0]['requirementSource']['scenarioNames'].pop()
        report = self.report()
        self.assertEqual(report['status'], 'mismatch')
        self.assertEqual(report['requirement_evidence']['refinement_requirements'][0]['integrity'], 'invalid')
        self.assertEqual(report['requirement_evidence']['inventory']['scenario_count'], 68)

    def test_duplicate_or_missing_original_row_invalidates_current_subtotal(self):
        self.ledger['requirements'][-1] = copy.deepcopy(self.ledger['requirements'][0])
        report = self.report()
        self.assertEqual(report['status'], 'mismatch')
        self.assertEqual(report['requirement_evidence']['summary']['original35']['current_owner_declared_closed'], 0)

    def test_cli_nonzero_for_changed_pinned_source_and_input_error_is_2(self):
        self.source.write_text('changed source\n')
        self.path.write_text(json.dumps(self.ledger))
        out = io.StringIO()
        with contextlib.redirect_stdout(out):
            code = qa.main(['--repo', str(REPO), '--requirement-evidence', str(self.path)])
        self.assertEqual(code, 1)
        self.assertEqual(json.loads(out.getvalue())['requirement_evidence']['requirements'][0]['integrity'], 'stale')
        self.path.write_text('{}')
        with contextlib.redirect_stderr(io.StringIO()):
            with self.assertRaises(SystemExit) as error:
                qa.main(['--repo', str(REPO), '--requirement-evidence', str(self.path)])
        self.assertEqual(error.exception.code, 2)

    def test_malformed_row_is_reported_without_summary_exception(self):
        self.ledger['requirements'][0] = {'id': 'UIREQ-001', 'acceptance': None}
        report = self.report()
        self.assertEqual(report['status'], 'mismatch')
        self.assertEqual(report['requirement_evidence']['summary']['original35']['current_owner_declared_closed'], 0)

    def test_wrong_heading_and_malformed_citation_span_are_not_current(self):
        source = self.ledger['requirements'][0]['requirementSource']
        other = self.ledger['requirements'][1]['requirementSource']
        source['heading'], source['scenarioNames'] = other['heading'], other['scenarioNames']
        self.ledger['refinementRequirements'][0]['citations'][0]['endLine'] = 99
        report = self.report()
        self.assertEqual(report['status'], 'mismatch')
        self.assertEqual(report['requirement_evidence']['requirements'][0]['integrity'], 'invalid')
        self.assertEqual(report['requirement_evidence']['refinement_requirements'][0]['integrity'], 'invalid')

    def test_bare_checker_reads_no_ledger_and_preserves_existing_corpus_report(self):
        report = qa.run(REPO)
        self.assertEqual(report['status'], 'pass')
        self.assertEqual(report['counts']['prefix_rows'], 216)
        self.assertNotIn('requirement_evidence', report)
        self.assertEqual(report['acceptance'], 'none')


if __name__ == '__main__':
    unittest.main(verbosity=2)

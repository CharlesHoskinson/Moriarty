"""Write the phase-2 explicit read allowlist (lineage files named by the phase-1 manifests).

Data only: reads the already-allowlisted named manifests; opens nothing else.
"""
import json, datetime

OUT = '/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-evidence-repair-independent-source-review-20261015/'
B = '/home/charl/moriarty-eutxo-validation-2026-10-12/'
SR = 'current-core-groth16-compiler-independent-source-review-20261015/'
RR = 'current-core-groth16-compiler-repair-independent-review-20261015/'
RV = 'current-core-groth16-independent-result-review-20261015/'
EX = 'current-core-groth16-env-corrected-offline-execution-20261015/'
PARENT = 'current-core-groth16-compiler-review-repair-source-20261015/'

sr = json.load(open(B + SR + 'REVIEW-MANIFEST.json'))
r2 = json.load(open(B + RR + 'plumbing-r2/R2-MANIFEST.json'))
ex = json.load(open(B + EX + 'OUTPUT-MANIFEST.json'))

never = ('PROMPT.md', 'PROCESS.json', 'FINAL-RESPONSE.md', 'PREPARATION.json', 'run.py',
         'provider-result.private.json', 'process.stderr')
ex_files = [EX + p for p in sorted(ex['files'])
            if not p.endswith('.stderr') and 'listing' not in p]

allow = {
    'phase': 2,
    'at_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'rule': ('Exact paths only. Old-run *.stderr experiment logs, operator.stderr and the two 15 MB '
             'cargo listings are deliberately excluded (not needed; stderr names are never opened). '
             'Parent candidate files are added only as listed by its SOURCE-MANIFEST after that '
             'manifest hash equals a6e17efd... (phase 3).'),
    'never_open_names': list(never),
    'source_review': [SR + 'REPORT.md', SR + 'REVIEW-FAILURES.json', SR + 'INPUT-MANIFEST.json',
                      SR + 'CORRECTED-EXECUTION-REQUEST.json']
                     + [SR + f['path'] for f in sr['files']],
    'repair_review': [RR + 'REPORT.md', RR + 'VOTES.md', RR + 'R2-PLUMBING-EXECUTION-REQUEST.json',
                      RR + 'LOCK-GATE-CONTROLS.json', RR + 'COPY-VERIFY-R2-DRYRUN-CONTROLS.json',
                      RR + 'R2-PLUMBING-DELTA.md']
                     + [RR + 'plumbing-r2/' + f['path'] for f in r2['files']],
    'result_review': [RV + 'proposals/FIX-PROPOSALS.md', RV + 'RECOMPUTE.json', RV + 'RECONCILE.json'],
    'old_execution': ex_files,
    'parent_candidate_manifests': [PARENT + 'SOURCE-MANIFEST.json', PARENT + 'BUILD-MANIFEST.json'],
}
with open(OUT + 'work/ALLOWLIST-PHASE2.json', 'x') as f:
    json.dump(allow, f, indent=1)
print(sum(len(v) for v in allow.values() if isinstance(v, list)))

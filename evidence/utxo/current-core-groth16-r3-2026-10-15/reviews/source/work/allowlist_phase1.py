"""Write the phase-1 explicit read allowlist. Reads only the author's named OUTPUT-MANIFEST.json."""
import json, datetime

OUT = '/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-evidence-repair-independent-source-review-20261015/'
B = '/home/charl/moriarty-eutxo-validation-2026-10-12/'
A = 'current-core-groth16-evidence-repair-resume-source-20261015/'

man = json.load(open(B + A + 'OUTPUT-MANIFEST.json'))
skip = {'inherited/old-partial/FINAL-RESPONSE.md', 'inherited/old-partial/PROCESS.json'}
author = [A + f['path'] for f in man['files'] if f['path'] not in skip]
phase1 = {
    'phase': 1,
    'at_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'rule': ('Only these exact relative paths under BASE may be read or hashed. No directory walking. '
             'Names PROMPT.md, PROCESS.json, FINAL-RESPONSE.md, PREPARATION.json, run.py, run-*.py, '
             'provider-result.private.json, process.stderr, operator.stderr are never opened.'),
    'base': B,
    'author_manifest_files_minus_skipped': author,
    'author_skipped_by_choice': sorted(A + s for s in skip),
    'lineage_manifests_and_named_files': [
        A + 'OUTPUT-MANIFEST.json',
        'current-core-groth16-evidence-repair-root-verification-20261015/VERIFICATION.json',
        'current-core-groth16-independent-result-review-20261015/REPORT.md',
        'current-core-groth16-independent-result-review-20261015/REVIEW-MANIFEST.json',
        'current-core-groth16-compiler-independent-source-review-20261015/REVIEW-MANIFEST.json',
        'current-core-groth16-compiler-repair-independent-review-20261015/OUTPUT-MANIFEST.json',
        'current-core-groth16-compiler-repair-independent-review-20261015/plumbing-r2/R2-MANIFEST.json',
        'current-core-groth16-env-corrected-offline-execution-20261015/OUTPUT-MANIFEST.json',
        'upstream-sherlock-4d969-20261015/MANIFEST.json',
        'upstream-sherlock-4d969-20261015/CORE-BYTE-COMPARISON.json',
        'upstream-sherlock-4d969-20261015/COMMITS.txt',
        'upstream-sherlock-a19b0-20261015/MANIFEST.json',
        'upstream-sherlock-a19b0-20261015/CORE-BYTE-COMPARISON.json',
        'upstream-sherlock-a19b0-20261015/COMMITS.txt',
    ],
    'directory_name_listings_performed_without_stat': [
        'own output dir', 'result-review root', 'compiler-source-review root',
        'compiler-repair-review root', 'plumbing-r2', 'old execution root',
        'root-verification', 'upstream-sherlock-4d969', 'upstream-sherlock-a19b0'],
    'note': 'Later phases add files named by the manifests above before they are read.',
}
with open(OUT + 'work/ALLOWLIST-PHASE1.json', 'x') as f:
    json.dump(phase1, f, indent=1)
print(len(author))

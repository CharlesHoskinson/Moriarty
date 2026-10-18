"""Writes REVIEW-MANIFEST.json over an explicit list of this reviewer's public artifacts.
No directory walk of the task root: only my own subdirectories (work/, work/diffs/, proposals/)
are listed by name to prove nothing of mine is left out. Wrapper/private names are never opened."""
import hashlib, json, os, datetime

OUT = '/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-evidence-repair-independent-source-review-20261015/'
EXCLUDED = ['PROMPT.md', 'PROCESS.json', 'FINAL-RESPONSE.md', 'PREPARATION.json', 'run.py',
            'provider-result.private.json', 'process.stderr', 'REVIEW-MANIFEST.json']
TOP = ['REPORT.md', 'FINDINGS.json', 'VOTES.md', 'READ-COVERAGE.json', 'REVIEW-FAILURES.json',
       'proposals/MINIMUM-REPAIRS.md']
own_dirs = ['work', 'work/diffs', 'proposals']
listed = list(TOP)
for d in own_dirs:
    for name in sorted(os.listdir(OUT + d)):
        rel = d + '/' + name
        if os.path.isdir(OUT + rel):
            continue
        if rel not in listed:
            listed.append(rel)
files = []
for rel in sorted(listed):
    with open(OUT + rel, 'rb') as f:
        b = f.read()
    files.append({'path': rel, 'sha256': hashlib.sha256(b).hexdigest(), 'bytes': len(b)})
top_names = sorted(os.listdir(OUT))  # names only; nothing opened or stat'ed except via isdir on own dirs above
unlisted_top = [n for n in top_names if n not in {p.split('/')[0] for p in listed} and n not in ('work', 'proposals')]
manifest = {
    'format': 'evidence-repair-independent-review-manifest/1',
    'at_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'reviewer': 'claude-opus-5-5 (requested high; fresh non-author; context-only independence)',
    'rule': 'Explicit list of this reviewer\'s finalized public artifacts plus every file in my own work/, work/diffs/ and proposals/ directories. Written last; nothing is appended afterwards. aggregate = SHA-256 of compact sorted-key JSON of files.',
    'excluded_by_name_never_opened': EXCLUDED,
    'top_level_names_not_listed': unlisted_top,
    'count': len(files),
    'files': files,
    'aggregate_sha256': hashlib.sha256(json.dumps(files, sort_keys=True, separators=(',', ':')).encode()).hexdigest(),
    'verdicts': {
        'source': 'ACCEPT-DELTA unit scope; constraint semantics unchanged; no new defect',
        'expectations': 'frozen preserved; AUTH addenda labelled; CHANGE on HW-06/07/23 classes',
        'parser_only_actual_results': 'ACCEPT-QUALIFIED synthetic parser evidence only (13/13, not rerun)',
        'execution_proposal': 'NOT YET ELIGIBLE; NV-11a AGREE, NV-11b CHANGE; r3 operator absent',
        'adoption': 'NOT ADOPTED; no r3 Rust result exists',
    },
}
with open(OUT + 'REVIEW-MANIFEST.json', 'x') as f:
    json.dump(manifest, f, indent=1, sort_keys=True)
print(json.dumps({'count': len(files), 'aggregate': manifest['aggregate_sha256'], 'unlisted_top': unlisted_top}, indent=1))

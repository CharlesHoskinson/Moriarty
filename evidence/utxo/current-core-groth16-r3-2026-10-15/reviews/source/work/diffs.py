"""Reviewer's own unified diffs (difflib) between allowlisted files; compares with author diffs by
changed-line content (header lines ignored). Data only; writes under work/diffs/ (exclusive)."""
import difflib, json, os

OUT = '/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-evidence-repair-independent-source-review-20261015/work/diffs/'
B = '/home/charl/moriarty-eutxo-validation-2026-10-12/'
A = B + 'current-core-groth16-evidence-repair-resume-source-20261015/'
P = B + 'current-core-groth16-compiler-review-repair-source-20261015/'
R2 = B + 'current-core-groth16-compiler-repair-independent-review-20261015/plumbing-r2/'

pairs = [
    ('coverage.rs', P + 'source/mori-r1cs/src/coverage.rs', A + 'candidate/source/mori-r1cs/src/coverage.rs',
     A + 'DIFFS/candidate/source__mori-r1cs__src__coverage.rs.diff'),
    ('COVERAGE.md', P + 'COVERAGE.md', A + 'candidate/COVERAGE.md', A + 'DIFFS/candidate/COVERAGE.md.diff'),
    ('BUILD-IDENTITY-DELTA.json', P + 'BUILD-IDENTITY-DELTA.json', A + 'candidate/BUILD-IDENTITY-DELTA.json',
     A + 'DIFFS/candidate/BUILD-IDENTITY-DELTA.json.diff'),
    ('heldout.rs.r2-r3', R2 + 'proposals/heldout-harness-r2/tests/heldout.rs',
     A + 'review-overlay/proposals/heldout-harness-r3/tests/heldout.rs', A + 'DIFFS/review/heldout.rs.vs-r2.diff'),
    ('Cargo.toml.r2-r3', R2 + 'proposals/heldout-harness-r2/Cargo.toml',
     A + 'review-overlay/proposals/heldout-harness-r3/Cargo.toml', A + 'DIFFS/review/Cargo.toml.vs-r2.diff'),
    ('lib.rs.r2-r3', R2 + 'proposals/heldout-harness-r2/src/lib.rs',
     A + 'review-overlay/proposals/heldout-harness-r3/src/lib.rs', A + 'DIFFS/review/lib.rs.vs-r2.diff'),
    ('copy_verify.r2-r3', R2 + 'r2-tools/copy_verify_r2.py', A + 'review-overlay/r3-tools/copy_verify_r3.py',
     A + 'DIFFS/review/copy_verify_r3.py.vs-r2.diff'),
]


def changed(lines):
    return sorted(l for l in lines if (l.startswith('+') or l.startswith('-'))
                  and not l.startswith('+++') and not l.startswith('---'))


os.makedirs(OUT, exist_ok=False)
summary = {}
for name, a, b, authord in pairs:
    la = open(a, encoding='utf-8').read().splitlines()
    lb = open(b, encoding='utf-8').read().splitlines()
    d = list(difflib.unified_diff(la, lb, 'old', 'new', lineterm='', n=3))
    with open(OUT + name + '.diff', 'x', encoding='utf-8') as f:
        f.write('\n'.join(d) + '\n')
    mine = changed(d)
    theirs = changed(open(authord, encoding='utf-8').read().splitlines())
    summary[name] = {'plus': sum(1 for l in mine if l[0] == '+'),
                     'minus': sum(1 for l in mine if l[0] == '-'),
                     'equal_to_author_diff_changed_lines': mine == theirs}
with open(OUT + 'SUMMARY.json', 'x') as f:
    json.dump(summary, f, indent=1)
print(json.dumps(summary, indent=1))

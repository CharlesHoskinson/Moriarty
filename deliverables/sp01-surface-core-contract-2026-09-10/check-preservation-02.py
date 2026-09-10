"""Check immutable source receipts via their explicit preservation map; no execution."""
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


mapping = json.loads((HERE / 'preservation-map-02.json').read_text())
old_path = ROOT / mapping['originalManifest']
assert digest(old_path) == mapping['originalManifestSha256']
original = json.loads(old_path.read_text())
rows = {r['originalPath']: r for r in mapping['files']}
assert len(rows) == len(mapping['files']) == len(original['files']) == 101
for entry in original['files']:
    row = rows[entry['path']]
    assert row['sha256'] == entry['sha256']
    assert digest(ROOT / row['preservedPath']) == entry['sha256'], entry['path']

decision = json.loads((HERE / 'source-candidate-01.json').read_text())
for name, wanted in decision['files'].items():
    preserved = rows.get(name, {}).get('preservedPath', name)
    assert digest(ROOT / preserved) == wanted, name

# Whole-input admission, finite domains, the financial statement rules and
# mathematical representation are unchanged by the Boolean-only revision.
spec = ROOT / 'experiments/moriarty-language/spec/successor'
archive = HERE / 'strict-contract-0/experiments/moriarty-language/spec/successor'


def section(path, start, end):
    return path.read_text().split(start, 1)[1].split(end, 1)[0]


for filename, start, end in [
    ('semantic-contract.md', '## V — finite domains and values', '## E — evaluation contexts and frames'),
    ('static-semantics.md', '## S0 — judgments and admission order', '## L — literals'),
    ('static-semantics.md', '## STMT — staging and descriptors', '## Cases and acceptance limits'),
]:
    assert section(spec / filename, start, end) == section(archive / filename, start, end)

old_sig = json.loads((archive / 'expression-signatures.json').read_text())
new_sig = json.loads((spec / 'expression-signatures.json').read_text())
assert [r['constructor'] for r in old_sig['constructors']] == [r['constructor'] for r in new_sig['constructors']]
assert len(new_sig['constructors']) == 40
for old, new in zip(old_sig['constructors'], new_sig['constructors']):
    if old['constructor'] not in {'And', 'Or'}:
        assert old == new, old['constructor']

print(json.dumps({
    'scope': 'source-byte-preservation-only',
    'publishedPins': 101,
    'archivedChangedFiles': sum(r['disposition'] == 'archived-before-revision' for r in rows.values()),
    'decisionPinsResolved': len(decision['files']),
    'constructorNamesUnchanged': 40,
    'nonBooleanSignaturesUnchanged': 38,
    'admissionFiniteDomainsFinancialStatementsUnchanged': True,
    'runtimeExecuted': False,
    'status': 'PASS',
}, indent=2))

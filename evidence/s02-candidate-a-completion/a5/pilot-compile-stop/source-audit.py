"""Read-only exact pilot source/plan preservation check, not model checking."""
import hashlib
import json
from pathlib import Path
import re

ROOT = Path.cwd()
MODEL = ROOT / 'specs/quint/s02/factored_verification'
R = ROOT / '.superpowers/sdd/a5-factoring-receipts'
def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

plan = ROOT / 'docs/superpowers/plans/2026-09-05-candidate-a-factored-verification.md'
assert sha(plan) == 'fe96c2343c369e9170952c04cfb847fc826d706a8f0ad6818efc283f37615563'
assert sha(R / 'task3-dispatch.json') == '51ca132878cf7dca7470b9fa5e82ef7e53e6088ea26b8a5b8b2838af4e73aeba'
dispatch = json.loads((R / 'task3-dispatch.json').read_text())
assert dispatch['actualTask3Base'] == '9ccbf0ed571e4055bc05962e5681fdf2fa75ad96'
assert dispatch['runtimeBootstrapBase'] == '900bb2051225b4a3d99bf422c3b2e5e386e3e7bc'
assert sha(ROOT / 'scripts/run_s02_candidate_a_factoring_pilot.py') == dispatch['runtimeHelperSha256']
names = ['candidate_a_funding_pilot', 'candidate_a_funding_pilot_f', 'candidate_a_funding_pilot_test']
sources = {}
for name in names:
    blocks = re.findall(r'```quint\n(module ' + re.escape(name) + r' \{.*?\n\})\n```', plan.read_text(), re.S)
    assert len(blocks) == 1
    path = MODEL / (name + '.qnt')
    assert path.read_bytes() == (blocks[0] + '\n').encode()
    sources[name] = sha(path)

original = (MODEL / (names[0] + '.qnt')).read_text()
factored = (MODEL / (names[1] + '.qnt')).read_text()
expected = original.replace('module candidate_a_funding_pilot {', 'module candidate_a_funding_pilot_f {', 1)
for module, alias in [('candidate_a_authority_swap_fixtures', 'F'),
                      ('candidate_a_authority_swap', 'S'),
                      ('candidate_a_authority_boundary', 'B'), ('candidate_a_core', 'C')]:
    before = f'import {module} as {alias} from "../{module}"'
    assert expected.count(before) == 1
    expected = expected.replace(before, f'import {module}_f as {alias} from "./{module}_f"', 1)
assert expected == factored
assert len([(profile, forged, n) for profile in (0, 1) for forged in (False, True)
            for n in range(5 if forged else 6)]) == 22
frozen = json.loads((R / 'task2-frozen-source.json').read_text())
assert len(frozen['files']) == 30
for entry in frozen['files']:
    assert sha(ROOT / entry['path']) == entry['sha256']
print(json.dumps({'ok': True, 'scope': 'Exact pilot source only; no native or checker verdict',
                  'sourceHashes': sources, 'exactPlanModules': 3, 'declarationAndImportChanges': 5,
                  'scheduledPrefixPairs': 22, 'unchangedTask2Files': 30,
                  'helperSha256': dispatch['runtimeHelperSha256']}, indent=2))

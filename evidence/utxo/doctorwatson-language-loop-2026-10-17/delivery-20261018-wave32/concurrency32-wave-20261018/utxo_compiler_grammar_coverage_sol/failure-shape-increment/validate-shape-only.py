"""Offline JSON/source custody check against an already captured old CLI result.
Does not import or execute Rust/Node/product/model/provider code or change originals.
"""
from pathlib import Path
import hashlib
import json
import subprocess

HERE = Path(__file__).parent
R = HERE.parents[2]
ORACLE = R / 'concurrency32-wave-20261018/two_program_financial_oracles'
REPO = '/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018'
COMMIT = 'de1eaf81eec57c7cec48c2ddae57362ff2c7d498'
freeze = ORACLE / 'FREEZE.sha256'
assert hashlib.sha256(freeze.read_bytes()).hexdigest() == '1b606141dff7ca162bcd132d55d62014187de4f936cfc6528c23c8d22591aefb'
member_checks = []
for line in freeze.read_text().splitlines():
    digest, name = line.split('  ', 1)
    data = (ORACLE / name).read_bytes()
    assert hashlib.sha256(data).hexdigest() == digest
    member_checks.append({'path': name, 'sha256': digest})
report = json.loads((HERE / 'captured-before/reserve-observations/stdout.json').read_text())
execution = report['execution']
steps = execution['steps']
projection = {key: execution[key] for key in ['semantics', 'action', 'pre', 'post', 'effects']}
projection['steps'] = [{key: step[key] for key in ['index', 'value', 'snapshot', 'effects']} for step in steps]
expected_names = ['financial-execution-projection.expected.json', 'source-statement-kinds.expected.json', 'cumulative-prefix-effects.expected.json']
expected = [json.loads((ORACLE / name).read_text()) for name in expected_names]
assert projection == expected[0]
assert [step['statement']['kind'] for step in steps] == expected[1]
before = []
after = []
prefix = []
for step in steps:
    prefix.extend(step['effects'])
    before.append(list(prefix))
    after.append({'index': step['index'], 'effects': list(prefix)})
assert before != expected[2]
assert isinstance(before[0], list) and isinstance(expected[2][0], dict)
assert before[0] == [] and expected[2][0] == {'index': 0, 'effects': []}
assert all(old == record['effects'] for old, record in zip(before, expected[2]))
assert after == expected[2]
fixture_checks = []
for name in expected_names:
    original = (ORACLE / name).read_bytes()
    pinned = subprocess.check_output(['git', '-C', REPO, 'show', COMMIT + ':crates/fixtures/reserve-spend-refund/' + name])
    assert pinned == original
    fixture_checks.append({'name': name, 'oracle_sha256': hashlib.sha256(original).hexdigest(), 'root_git_fixture_sha256': hashlib.sha256(pinned).hexdigest(), 'byte_identical': True})
summary = {
    'format': 'captured-observation-adapter-diagnosis/1',
    'classification': 'Offline mechanical comparison of existing old capture; no product rerun or new pass',
    'old_runtime_exit': 101,
    'first_difference': {'json_pointer': '/0', 'actual_before_type': 'array', 'expected_type': 'object', 'actual_before': before[0], 'expected': expected[2][0]},
    'root_cause': 'Adapter dropped record index and effects wrapper; every complete cumulative effects array already equals the unchanged expected record.effects.',
    'complete_financial_projection_matches_old_cli_capture': True,
    'all_source_kinds_match_old_cli_capture': True,
    'all_20_complete_effect_arrays_match_before_fix': True,
    'all_20_indexed_records_match_after_adapter_on_old_capture': True,
    'indices_observed_not_recreated': [step['index'] for step in steps],
    'expected_root_fixture_bytes_unchanged': fixture_checks,
    'whole_oracle_freeze_sha256': hashlib.sha256(freeze.read_bytes()).hexdigest(),
    'whole_oracle_payloads_verified': len(member_checks),
    'whole_oracle_members': member_checks,
    'runtime_rerun': False,
    'Rust_compile_or_syntax_execution': False,
    'new_product_oracle_or_expected_values': False,
    'payout_target_result': 'Not reached in failed batch; remains unobserved.',
}
(HERE / 'DATA-DIAGNOSIS.json').write_text(json.dumps(summary, indent=2) + '\n')
for name in expected_names:
    dest = HERE / 'oracle-reference' / name
    dest.parent.mkdir(parents=True, exist_ok=True)
    dest.write_bytes((ORACLE / name).read_bytes())
(HERE / 'captured-adapter-before.json').write_text(json.dumps(before, indent=2) + '\n')
(HERE / 'captured-adapter-after.json').write_text(json.dumps(after, indent=2) + '\n')
print(json.dumps({key: summary[key] for key in ['first_difference', 'all_20_complete_effect_arrays_match_before_fix', 'all_20_indexed_records_match_after_adapter_on_old_capture', 'whole_oracle_payloads_verified', 'runtime_rerun']}))

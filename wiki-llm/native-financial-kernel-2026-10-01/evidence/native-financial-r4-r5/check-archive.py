"""Read-only integrity and recorded-result checks. Never executes archived code."""
import hashlib
import json
from pathlib import Path

ARCHIVE = Path(__file__).resolve().parent
ROOT = ARCHIVE.parents[3]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load(name):
    return json.loads((ARCHIVE / name).read_bytes())


manifest = load('archive-manifest.json')
by_original = {}
for entry in manifest['files']:
    path = (ROOT / entry['path']).resolve()
    assert path.is_relative_to(ARCHIVE), entry['path']
    assert path.is_file() and path.stat().st_size == entry['bytes'], entry['path']
    assert digest(path) == entry['sha256'], entry['path']
    assert entry['original_path'] not in by_original
    by_original[entry['original_path']] = path

for name, count in [
    ('NATIVE-FINANCIAL-V10-R5-ACTUAL-RESULT-FREEZE.json', 165),
    ('NATIVE-FINANCIAL-V10-R4-ACTUAL-RESULT-FREEZE.json', 88),
    ('NATIVE-FINANCIAL-V10-R4-PROOF-RESULT-FREEZE.json', 67),
]:
    original = load(name)['sha256']
    assert len(original) == count
    for path, expected in original.items():
        assert path in by_original and digest(by_original[path]) == expected, path

acceptance = load('NATIVE-FINANCIAL-V10-R5-ROOT-ACTUAL-ACCEPTANCE.json')
assert digest(ARCHIVE / 'NATIVE-FINANCIAL-V10-R5-ACTUAL-RESULT-FREEZE.json') == manifest['actual165_freeze_sha256'] == acceptance['actual_result_freeze']['sha256']
assert digest(ARCHIVE / 'NATIVE-FINANCIAL-V10-R5-ROOT-ACTUAL-ACCEPTANCE.json') == manifest['root_acceptance_sha256']
assert acceptance['accepted_after_all_three_fresh_actual_result_reviews'] is True
assert len(acceptance['reviews']) == 3
for review in acceptance['reviews']:
    assert review['verdict'] == 'APPROVE_SCOPED'
    assert digest(by_original[review['report_path']]) == review['report_sha256']
    assert digest(by_original[review['terminal_receipt_path']]) == review['terminal_receipt_sha256']

for phase in ['prove', 'verify']:
    suffix = 'r4' if phase == 'prove' else 'r5'
    receipt = load(f'native-financial-v10-{suffix}-{phase}-receipt.json')
    assert receipt['exit_code'] == 0 and receipt['stop_reason'] is None
    assert receipt['supervisor_error'] is None and receipt['native_postconditions_passed'] is True
    directory = ARCHIVE / ('native-financial-v10-r4-proof' if phase == 'prove' else 'native-financial-v10-r5-verification')
    actual = {str(p.relative_to(directory)) for p in directory.rglob('*') if p.is_file()}
    assert actual == set(receipt['output_artifacts'])
    for name, artifact in receipt['output_artifacts'].items():
        assert digest(directory / name) == artifact['sha256']
        assert (directory / name).stat().st_size == artifact['bytes']

proof = ARCHIVE / 'native-financial-v10-r4-proof'
verified = ARCHIVE / 'native-financial-v10-r5-verification'
receipt = json.loads((proof / 'receipt.json').read_bytes())
assert receipt['result'] == 'Success' and receipt['proof_count'] == 1
assert receipt['strictness'] == 'native default Real; proof-verifying enabled'
fee, allowance, available, remainder = (int(receipt[k]) for k in ['actual_native_fee_consumed', 'allow_fee_payment', 'generationless_available', 'dust_remainder'])
assert 0 < fee <= allowance <= available and fee + remainder == available
assert (proof / 'receipt.json').read_bytes() == (verified / 'independent-ledger-receipt.json').read_bytes()
controls = json.loads((verified / 'history-control-results.json').read_bytes())
assert controls['all_six_refusals'] is True and controls['pristine_good_again'] is True
assert len(controls['controls']) == 8
for record in controls['controls']:
    name = record['case']
    log = verified / 'fault-controls' / ({'canonical-native-alternate-producer': 'alternate', 'pristine-good-again': 'good-again'}.get(name, name) + '.log')
    assert digest(log) == record['log_sha256']
    assert record['exit_code'] == (0 if name in ['canonical-native-alternate-producer', 'pristine-good-again'] else 1)
first = (ARCHIVE / 'native-financial-v10-r5-verify-good-first.log').read_bytes()
assert first == (verified / 'fault-controls/good-again.log').read_bytes()
indices = [int(line.split('NATIVE_CRYPTO_REFUSAL public field ', 1)[1].split()[0].split('+')[0]) for line in first.decode().splitlines() if line.startswith('NATIVE_CRYPTO_REFUSAL public field ')]
assert indices == list(range(1035))
partial = load('native-financial-v10-r4-verify-receipt.json')
assert partial['exit_code'] == 1 and partial['native_postconditions_passed'] is False
source = ARCHIVE / 'native-ledger-reward-funded-v10-r2-candidate'
for name, expected in json.loads((source / 'SOURCE-HASHES.json').read_bytes())['sha256'].items():
    assert digest(source / name) == expected
print(json.dumps({'files_checked': len(manifest['files']), 'actual_frozen_files': 165, 'proof_outputs': 22, 'verification_outputs': 30, 'three_actual_reviews': True, 'recorded_result_invariants': 'pass', 'native_execution': False}))

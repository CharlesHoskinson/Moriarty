"""Read-only proposed interpretation of existing R435 operation09; no subject execution."""
from pathlib import Path
import hashlib, json, re
def parse_exact_stdout(expected, text):
    # Only the fixed serial libtest envelope and the frozen consumer's one diagnostic are allowed.
    diagnostic_cases = {
        'uint_builtin_identity_uses_real_canonical_carriers': 'uint_identity',
        'bool_builtin_identity_uses_real_canonical_carriers': 'bool_identity',
        'list_builtin_identity_uses_real_canonical_carriers': 'list_identity',
        'tree_builtin_identity_uses_real_canonical_carriers': 'tree_identity',
    }
    diagnostic = (re.escape(diagnostic_cases[expected]) +
        r': owner_events=\d+, consumed=\d+, remaining=\d+, helper_work=\d+, helper_visits=\d+, reference_cursor=\d+; absolute independent B clocks UNBOUND') if expected in diagnostic_cases else None
    inline = r'test ' + re.escape(expected) + r' \.\.\. ok'
    body = inline if diagnostic is None else '(?:' + inline + '|test ' + re.escape(expected) + r' \.\.\. ' + diagnostic + r'\nok)'
    pattern = (r'\nrunning 1 test\n' + body +
        r'\n\ntest result: ok\. 1 passed; 0 failed; 0 ignored; 0 measured; (\d+) filtered out; finished in (?:\d+)\.(?:\d+)s\n\n')
    match = re.fullmatch(pattern, text)
    assert match, 'stdout is not the exact one-selected-test envelope with the frozen diagnostic shape'
    assert int(match.group(1)) == 4, 'sealed five-test consumer must filter the other four'
    return dict(exactTest=expected, selected=1, passed=1, failed=0, ignored=0, measured=0,
                filtered_out=4, diagnostic_counts_are_observations_not_independent_B_clock_oracles=True)

def main():
    stdout = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/435-resolved-signature-execution-preparation/actual-thirteen-01/09/stdout.bin', 'bytes': 326, 'sha256': '09ea0fef601d153b353a51a8c4b341ed532039c595c8cfcd2d5360777231b786'}
    raw = Path(stdout['path']).read_bytes()
    assert len(raw) == stdout['bytes'] and hashlib.sha256(raw).hexdigest() == stdout['sha256']
    result = json.loads(Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/435-resolved-signature-execution-preparation/actual-thirteen-01/09/RESULT.json').read_text())
    assert hashlib.sha256(Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/435-resolved-signature-execution-preparation/actual-thirteen-01/09/RESULT.json').read_bytes()).hexdigest() == '06d02b96f9b887e9189599dec3d0ae94a2375f49adf1e7f95c3385481f757140'
    assert result['actual_exit'] == 0 and result['controller_failure'] is None
    assert len(result['streams']) == 2 and all(row['full_stream_retained'] for row in result['streams'])
    for row in result['streams']:
        observed = Path(row['path']).read_bytes()
        assert len(observed) == row['bytes'] and hashlib.sha256(observed).hexdigest() == row['sha256']
    assert Path(result['streams'][1]['path']).read_bytes() == b''
    acceptance = parse_exact_stdout('uint_builtin_identity_uses_real_canonical_carriers', raw.decode())
    print(json.dumps(dict(status='PROPOSED_OFFLINE_INTERPRETATION_PENDING_EXPLICIT_ROOT_ADOPTION',
        original_accepted=result['accepted'], original_failure=result['acceptance_failure'],
        proposed_exact_test_acceptance=acceptance, original_receipt_unchanged=True), indent=2))

if __name__ == '__main__':
    main()

"""Compare complete captured actual nominal tapes to independent frozen JSON.

This process reads data and writes a report to stdout. It runs no K/model or
other child process and never supplies an expected result to the evaluator.
"""
from pathlib import Path
import hashlib
import json
import sys
import nominal_trace_codec as codec

def first_difference(a, e, path='$'):
    if type(a) is not type(e): return path + ': type'
    if isinstance(e, dict):
        if set(a) != set(e): return path + ': fields'
        for key in e:
            if a[key] != e[key]: return first_difference(a[key], e[key], path + '.' + key)
    elif isinstance(e, list):
        if len(a) != len(e): return path + ': length'
        for i, (x, y) in enumerate(zip(a, e)):
            if x != y: return first_difference(x, y, path + '[' + str(i) + ']')
    elif a != e: return path + ': value'
    return None

def main():
    manifest = json.loads(Path(sys.argv[1]).read_text())
    codec.configure(manifest['api'], manifest['syntax'])
    rows = []
    for case in manifest['cases']:
        row = {'case_id': case['case_id'], 'complete_tape_equal': False}
        try:
            expected_bytes = Path(case['expected_json']).read_bytes()
            if hashlib.sha256(expected_bytes).hexdigest() != case['expected_json_sha256']:
                raise ValueError('Independent expected JSON pin changed')
            fixture = json.loads(expected_bytes)
            text_bytes = Path(case['actual_print']).read_bytes()
            kore_bytes = Path(case['actual_kore']).read_bytes()
            actual_term = codec.parse_printed(text_bytes.decode('utf-8'))
            actual = codec.decode(actual_term, {'arms': codec.arms_index(fixture['shared_program'])})
            expected = codec.expanded_expected(fixture)
            row.update({'actual_print_sha256': hashlib.sha256(text_bytes).hexdigest(),
                        'actual_print_bytes': len(text_bytes),
                        'actual_kore_sha256': hashlib.sha256(kore_bytes).hexdigest(),
                        'actual_kore_bytes': len(kore_bytes),
                        'complete_tape_equal': actual == expected,
                        'first_difference': first_difference(actual, expected)})
        except (ValueError, KeyError, IndexError, TypeError, AssertionError, OSError, UnicodeError, RecursionError) as error:
            row['comparison_error'] = type(error).__name__ + ': ' + str(error)
        rows.append(row)
    success = len(rows) == len(manifest['cases']) and all(r['complete_tape_equal'] for r in rows)
    print(json.dumps({'kind': 'COMPLETE_ACTUAL_TAPE_VS_INDEPENDENT_FROZEN_JSON',
                      'comparison': 'initial/every returned state/final, full Program/control/current and saved environments/ordered frames/full event prefixes; no stutter or value projection',
                      'all_cases_equal': success, 'cases': rows}, indent=2), flush=True)
    return 0 if success else 1

if __name__ == '__main__':
    raise SystemExit(main())

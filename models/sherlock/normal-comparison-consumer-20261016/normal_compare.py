#!/usr/bin/env python3
"""Public QA JSON comparator. Private frozen-oracle readers remain separate.
Only supplied JSON data is read; no producer, source evaluator or engine runs.
"""
import argparse
from decimal import Decimal
import json
import sys
from pathlib import Path

def require(ok, path, message):
    if not ok:
        raise ValueError(f'{path}: {message}')


def keys(value, expected, path):
    require(type(value) is dict and set(value) == set(expected), path,
            f'exact fields required {sorted(expected)}; observed {sorted(value) if type(value) is dict else type(value).__name__}')


def read(path):
    def pairs(items):
        out = {}
        for k, v in items:
            require(k not in out, str(path), f'duplicate JSON key {k!r}')
            out[k] = v
        return out
    def no_constant(value):
        raise ValueError(f'{path}: non-integral JSON number {value}; no float transport')
    return json.loads(Path(path).read_text(), object_pairs_hook=pairs,
                      parse_float=Decimal, parse_constant=no_constant)


def ptr(doc, pointer):
    require(pointer.startswith('/') or pointer == '', pointer, 'JSON pointer required')
    for key in pointer.split('/')[1:]:
        key = key.replace('~1', '/').replace('~0', '~')
        doc = doc[int(key)] if type(doc) is list else doc[key]
    return doc


def differences(expected, actual, path=''):
    """Complete exact tree comparison. bool/int, null/zero and all fields differ."""
    out = []
    if type(expected) is not type(actual):
        return [{'path': path, 'kind': 'type', 'expectedType': type(expected).__name__, 'actualType': type(actual).__name__,
                 'expected': expected, 'actual': {'jsonNumericToken': str(actual)} if type(actual) is Decimal else actual}]
    if type(expected) is dict:
        for key in sorted(expected.keys() | actual.keys()):
            p = path + '/' + key.replace('~', '~0').replace('/', '~1')
            if key not in actual:
                out.append({'path': p, 'kind': 'missing_actual', 'expected': expected[key]})
            elif key not in expected:
                out.append({'path': p, 'kind': 'unrepresented_actual', 'actual': actual[key]})
            else:
                out.extend(differences(expected[key], actual[key], p))
    elif type(expected) is list:
        for i in range(max(len(expected), len(actual))):
            p = path + '/' + str(i)
            if i >= len(actual): out.append({'path': p, 'kind': 'missing_actual', 'expected': expected[i]})
            elif i >= len(expected): out.append({'path': p, 'kind': 'unrepresented_actual', 'actual': actual[i]})
            else: out.extend(differences(expected[i], actual[i], p))
    elif (expected.as_tuple() != actual.as_tuple() if type(expected) is Decimal else expected != actual):
        out.append({'path': path, 'kind': 'value', 'expected': expected, 'actual': actual})
    return out


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--expected', type=Path, required=True, help='Private complete QA envelope supplied by the separate frozen reader')
    parser.add_argument('--actual', type=Path, help='Actual JSON/JSONL data only; producer origin requires Root attestation')
    parser.add_argument('--actual-format', choices=('host-jsonl', 'host-series', 'qa-envelope'), default='host-jsonl')
    parser.add_argument('--report', type=Path)
    args = parser.parse_args()
    expected = read(args.expected)
    report = {'status': 'UNOBSERVED', 'engineComparison': 'UNOBSERVED', 'actualRead': False, 'differences': []}
    if args.actual:
        if args.actual_format == 'host-jsonl':
            from capture_jsonl import compare_capture
            result = compare_capture(expected, args.actual)
        elif args.actual_format == 'host-series':
            from proposed_transport import ProposedTransport
            result = ProposedTransport().compare(expected, read(args.actual))
        else:
            diff = differences(expected, read(args.actual))
            result = {'status': 'MISMATCH' if diff else 'EXACT_DATA_MATCH', 'differences': diff}
        report.update(result)
        report['actualRead'] = True
        report['engineComparison'] = 'PRODUCER_ORIGIN_REQUIRES_ROOT_ATTESTATION'
    encoded = json.dumps(report, indent=2, default=lambda v: {'jsonNumericToken': str(v)} if type(v) is Decimal else str(v)) + '\n'
    if args.report: args.report.write_text(encoded)
    else: sys.stdout.write(encoded)
    return 2 if report['status'] in ('GAP', 'UNOBSERVED') else (1 if report['differences'] else 0)


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (ValueError, KeyError, IndexError, TypeError, OSError) as exc:
        print(json.dumps({'status': 'GAP', 'error': str(exc)}), file=sys.stderr)
        sys.exit(2)

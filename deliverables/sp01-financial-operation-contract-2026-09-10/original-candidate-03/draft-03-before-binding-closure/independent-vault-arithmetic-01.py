"""Independent rational oracle for specified vault arithmetic, not Moriarty execution."""
from fractions import Fraction
from pathlib import Path
import json

ROOT = Path(__file__).resolve().parent
MAX = 2**128 - 1


def observe(case):
    q, supply, valuation = (int(case[k]) for k in ('quantity', 'supply', 'valuation'))
    if any(x < 0 or x > MAX for x in (q, supply, valuation)):
        return {'rejection': 'INPUT_UINT128_BOUND'}
    if supply == 0 or valuation == 0:
        return {'rejection': 'OUTSIDE_INITIALIZED_SUBRULE'}
    operation = case['operation']
    assert operation in {'deposit', 'mint', 'withdraw', 'redeem'}
    share_result = operation in {'deposit', 'withdraw'}
    numerator = q * (supply if share_result else valuation)
    denominator = valuation if share_result else supply
    assert 0 <= numerator < 2**256
    ratio = Fraction(numerator, denominator)
    floor = ratio.numerator // ratio.denominator
    result = floor + (operation in {'mint', 'withdraw'} and ratio != floor)
    if result > MAX:
        return {'rejection': 'RESULT_UINT128_BOUND'}
    if case.get('requiresProductWiderThan128'):
        assert numerator > MAX, 'case must distinguish a 128-bit-only implementation'
    return {'output': str(result), 'resultUnit': 'shares' if share_result else 'assets'}


if __name__ == '__main__':
    cases = json.loads((ROOT / 'independent-vault-arithmetic-01.json').read_text())['cases']
    for case in cases:
        actual = observe(case)
        assert actual == case['expect'], (case['id'], actual, case['expect'])
    # These probes make each directional rounding obligation observable.
    indexed = {case['id']: case for case in cases}
    assert observe(indexed['deposit-round-down'])['output'] != observe(indexed['withdraw-round-up'])['output']
    assert observe(indexed['mint-round-up'])['output'] != observe(indexed['redeem-round-down'])['output']
    print(json.dumps({'specifiedCasesChecked': len(cases), 'wideProductDiscriminators': 4,
                      'runtimeExecuted': False, 'sourceExpressibilityEstablished': False}))

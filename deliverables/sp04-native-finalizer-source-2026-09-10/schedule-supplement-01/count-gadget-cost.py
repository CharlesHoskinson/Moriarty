#!/usr/bin/env python3
"""Pure integer port of published auxiliary bounds; illustrative 4x8 layout.

No circuit code is imported or executed. This is not a whole-verifier row bound.
"""
from functools import lru_cache
from pathlib import Path
import json
import math

here = Path(__file__).resolve().parent
c = json.loads((here.parent / 'public-constant-results.json').read_text())
p, r = int(c['baseModulusHex'], 16), int(c['scalarModulusHex'], 16)
B, L, H = 2**56, 7, 2**112
moduli = [2**134, 2**134 - 1]
bp = [pow(B, i, p) for i in range(L)]
dbp = [pow(B, i+j, p) for i in range(L) for j in range(L)]


@lru_cache(None)
def rows(bits):
    if bits == 0:
        return 0
    return 1 + min(rows(bits - n*w) for n in range(1, 5)
                   for w in range(1, 9) if n*w <= bits)


def power_bound(value):
    first = (value-1).bit_length()
    best = first
    for n in range(first+1, first+129):
        if rows(n) < rows(best):
            best = n
    return 2**best


def ceildiv(a, b):
    return -((-a)//b)


def auxiliary(expr, expr_mod):
    lo, hi = expr
    kmin, kmax = ceildiv(lo, p), hi//p
    umax = power_bound(kmax-kmin+1)
    lower, upper = lo-(umax+kmin)*p, hi-kmin*p
    lcm, chosen = r, []
    for m in moduli:
        if lcm > -lower and lcm > upper:
            break
        lcm = math.lcm(lcm, m)
        chosen.append(m)
    assert lcm > -lower and lcm > upper
    widths = [(umax.bit_length()-1)]
    for m, (emin, emax) in zip(chosen, expr_mod):
        km = (kmin*p)%m
        lmin = ceildiv(emin-umax*(p%m)-km, m)
        lmax = (emax-km)//m
        vmax = power_bound(lmax-lmin+1)
        lower = emin-umax*(p%m)-km-(vmax+lmin)*m
        upper = emax-km-lmin*m
        assert r > -lower and r > upper
        widths.append(vmax.bit_length()-1)
    assert all(0 < n < 255 for n in widths)
    return widths


def mul_expr(a, d):
    x = sum(a)*(B-1)
    xy = sum(d)*(B-1)**2
    return -x, xy+2*x


mul_widths = auxiliary(mul_expr(bp, dbp), [
    mul_expr([x%m for x in bp], [x%m for x in dbp]) for m in moduli])
shift = sum(bp)*H
z = sum(bp)*(B-1)
norm_expr = (-z-shift, shift)
norm_mod = []
for m in moduli:
    powers = [x%m for x in bp]
    sm = sum(powers)*H
    zm = sum(powers)*(B-1)
    norm_mod.append((-zm-shift%m, 2*sm-shift%m))
norm_widths = auxiliary(norm_expr, norm_mod)

# All seven output limbs are bounded conservatively by 56 bits, including
# any tighter most significant limb. Power-of-two assignment uses one P2R
# region of optimal row count. Existing-cell assertions add a copy-only
# region, charged one whole row conservatively. Both foreign cores span2.
mul_core = 2 + 7*rows(56) + sum(rows(w)+1 for w in mul_widths)
norm_core = 2 + 7*(rows(56)+1) + sum(rows(w)+1 for w in norm_widths)

# No cache/packing savings credited. Foreign mul asks for zero and one:
# charge all14 native fixed-limb requests one row. Add/sub asks for seven
# zero limbs, and each of seven two-input native linear-combinations asks
# for native zero plus one arithmetic row: 7 + 7*(1+1) =21.
# Negation is explicitly implemented as normalized subtraction from zero
# in this proposed counting schedule, charged like subtraction.
mul_total = mul_core + 14
normalized_linear = 21 + norm_core
schedule = json.loads((here/'schedule-results.json').read_text())
arithmetic = schedule['conservativeArithmeticCore']
linear_count = sum(arithmetic[k] for k in ['addFp', 'subFp', 'negFp'])
bound = arithmetic['mulFp']*mul_total + linear_count*normalized_linear

result = {
    'status': 'PASS_PUBLIC_INTEGER_AUXILIARY_BOUNDS_ONLY',
    'hypotheticalLayout': {'parallelRangeChecks':4,'maxRangeTagBits':8,
        'nativeArithmeticAdviceColumns':5,'foreignArithmeticAdviceColumns':14,
        'notBackendOrBudgetSelection':True},
    'multiplicationAuxiliaryBitBounds':mul_widths,
    'normalizationAuxiliaryBitBounds':norm_widths,
    'rangeRowsFor56Bits':rows(56),
    'twoRowCorePlusOutputAndAuxRangeRegions':{'multiply':mul_core,'normalize':norm_core},
    'perOperationBoundIncludingUncachedFixedRequests':{
        'normalizedInputMultiply':mul_total,
        'addSubOrNegIncludingOutputNormalization':normalized_linear},
    'conditionalArithmeticRegionHeightSumUpperBound':bound,
    'excludes':['input canonicality/on-curve/subgroup and identity derivation',
        'two identity-mask selections, inverse-witness assignment and identity equalities',
        'prepared line and Frobenius constant assignment',
        'lookup-table loading and column-placement/permutation/blinding overhead',
        'transcript preparation, carried-accumulator MSM and outer statement binding'],
    'rowPlacementPremise':'Serial nonoverlapping region placement with no extra gap is the specified accounting layout; summing region heights credits no parallel packing. Actual floor-planner compatibility must be checked before treating it as a full-circuit domain bound.',
    'scope':'Numeric upper bound only for the stated arithmetic-region calls under explicit normalization/layout assumptions. No complete finalizer or verifier row/k/SRS/runtime fit, proof or deployment acceptance.'
}
print(json.dumps(result, indent=2))

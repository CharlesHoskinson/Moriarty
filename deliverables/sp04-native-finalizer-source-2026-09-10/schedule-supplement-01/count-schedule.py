#!/usr/bin/env python3
"""Offline source-count and exponent algebra only. Does not call native code."""
from collections import Counter
from pathlib import Path
import hashlib
import json
import math
import re

here = Path(__file__).resolve().parent
source = (here / "pairing.c").read_text()
constants = json.loads((here.parent / "public-constant-results.json").read_text())
p = int(constants["baseModulusHex"], 16)
r = int(constants["scalarModulusHex"], 16)


def body(name):
    match = re.search(r"static (?:inline )?void " + name + r"\([^;]*?\)\s*\{", source)
    assert match, name
    start = match.end()
    level = 1
    for i in range(start, len(source)):
        level += (source[i] == "{") - (source[i] == "}")
        if level == 0:
            return source[start:i]
    raise AssertionError("Unclosed source function")


def calls(name, text):
    return len(re.findall(r"\b" + name + r"\s*\(", text))


loop = body("miller_loop_lines")
segments = [(int(offset), int(length)) for offset, length in re.findall(
    r"post_add_n_dbl\(ret,\s*&Qlines\[(\d+)\],\s*Px2,\s*(\d+)\)", loop)]
assert segments == [(1, 2), (4, 3), (8, 9), (18, 32), (51, 16)]
cursor = 1
for offset, length in segments:
    assert offset == cursor
    cursor += 1 + length
assert cursor == 68
assert calls("post_line_by_Px2", loop) == 1
assert calls("mul_fp", body("post_line_by_Px2")) == 4
assert calls("sqr_fp12", body("post_add_n_dbl")) == 1
assert calls("mul_by_xy00z0_fp12", body("post_add_n_dbl")) == 2
per_term = {
    "lineEvaluations": cursor,
    "ordinarySquaresFp12": sum(length for _, length in segments),
    "sparseMultiplicationsFp12": cursor - 1,
    "conjugationsFp12": calls("conjugate_fp12", loop),
    "pointPreparationAddsFp": calls("add_fp", loop),
    "pointPreparationNegationsFp": calls("neg_fp", loop),
}
assert per_term == dict(lineEvaluations=68, ordinarySquaresFp12=62,
                       sparseMultiplicationsFp12=67, conjugationsFp12=1,
                       pointPreparationAddsFp=2, pointPreparationNegationsFp=1)

raise_half = body("raise_to_z_div_by_2")
raise_lengths = re.findall(r"mul_n_sqr\(ret, a, ([0-9]+(?:-1)?)\)", raise_half)
lengths = [sum(map(int, x.split("-"))) if "-" not in x else
           int(x.split("-")[0]) - int(x.split("-")[1]) for x in raise_lengths]
assert lengths == [2, 3, 9, 32, 15]
helper_squares = 1 + sum(lengths)
assert calls("mul_fp12", body("mul_n_sqr")) == 1
final = body("final_exp")
full_raises = calls("raise_to_z", final)
half_raises = calls("raise_to_z_div_by_2", final)
final_counts = {
    "cyclotomicSquaresFp12": calls("cyclotomic_sqr_fp12", final)
    + full_raises * (helper_squares + 1) + half_raises * helper_squares,
    "denseMultiplicationsFp12": calls("mul_fp12", final)
    + (full_raises + half_raises) * len(lengths),
    "conjugationsFp12": calls("conjugate_fp12", final)
    + full_raises + half_raises,
    "frobeniusPowers": [int(n) for n in re.findall(
        r"frobenius_map_fp12\([^;]*?,\s*([123])\)", final)],
    "inversionsFp12": calls("inverse_fp12", final),
}
assert final_counts == dict(cyclotomicSquaresFp12=315,
                           denseMultiplicationsFp12=35,
                           conjugationsFp12=10, frobeniusPowers=[2, 3, 2, 1],
                           inversionsFp12=1)

# Exponent-only transcription of pairing.c final_exp. Multiplication adds
# exponents, square doubles, Frobenius multiplies by p^n, conjugation by p^6.
# Work modulo p^12-1 and assume the nonzero field input required by inversion.
group_order = p ** 12 - 1
z_abs = 2
for length in lengths:
    z_abs = (z_abs + 1) << length
z_abs *= 2
assert z_abs == 0xd201000000010000
conj = lambda x: x * p ** 6 % group_order
rz_half = lambda x: conj(x * (z_abs // 2) % group_order)
rz = lambda x: 2 * rz_half(x) % group_order
y1 = conj(1)
y2 = -1 % group_order
ret = (y1 + y2) % group_order
y2 = ret * p ** 2 % group_order
ret = (ret + y2) % group_order
y0 = 2 * ret % group_order
y1 = rz(y0)
y2 = rz_half(y1)
y3 = conj(ret)
y1 = (y1 + y3) % group_order
y1 = conj(y1)
y1 = (y1 + y2) % group_order
y2 = rz(y1)
y3 = rz(y2)
y1 = conj(y1)
y3 = (y3 + y1) % group_order
y1 = conj(y1)
y1 = y1 * p ** 3 % group_order
y2 = y2 * p ** 2 % group_order
y1 = (y1 + y2) % group_order
y2 = rz(y3)
y2 = (y2 + y0) % group_order
y2 = (y2 + ret) % group_order
y1 = (y1 + y2) % group_order
y2 = y3 * p % group_order
ret = (y1 + y2) % group_order
exponent = group_order // r
assert group_order % r == 0 and ret == 3 * exponent and math.gcd(3, r) == 1

# Explicit unoptimized tower schedule: Fp2 = Fp[u]/(u²+1),
# Fp6 = Fp2[v]/(v³-u-1), Fp12 = Fp6[w]/(w²-v).
# Every intermediate Fp add/sub/neg is normalized before its next use.
# Dense products use schoolbook convolution; sparse products and all squares
# deliberately use the same dense routine. Constant products use generic mul.
def scale(c, n):
    return Counter({k: v * n for k, v in c.items()})


M = Counter(mulFp=1)
A = Counter(addFp=1)
S = Counter(subFp=1)
N = Counter(negFp=1)
A2 = scale(A, 2)
nonresidue2 = A + S
M2 = scale(M, 4) + A + S
M6 = scale(M2, 9) + scale(A2, 6) + scale(nonresidue2, 2)
M12 = scale(M6, 4) + scale(A2, 6) + nonresidue2
assert M12 == Counter(mulFp=144, addFp=105, subFp=45)
dense_instances = 2 * (per_term["ordinarySquaresFp12"]
                      + per_term["sparseMultiplicationsFp12"])
dense_instances += 1 + final_counts["denseMultiplicationsFp12"]
dense_instances += final_counts["cyclotomicSquaresFp12"]
dense_instances += 1  # One multiplication checking a * inverseWitness == 1.
assert dense_instances == 610
core = scale(M12, dense_instances)
core += scale(M, 2 * cursor * 4)  # four constant products per line
core += scale(A, 2 * per_term["pointPreparationAddsFp"])
core += scale(N, 2 * per_term["pointPreparationNegationsFp"])
core += scale(N, 6 * (2 + final_counts["conjugationsFp12"]))
for n in final_counts["frobeniusPowers"]:
    # fp12_tower.c:697-731: five Fp2 constant products, four Fp
    # constant products, six Fp conjugations when n is odd.
    core += scale(M2, 5) + scale(M, 4) + scale(N, 6 * (n % 2))
assert core == Counter(mulFp=88480, addFp=64074, subFp=27470, negFp=86)
normalizations = core["addFp"] + core["subFp"] + core["negFp"]
assert normalizations == 91630
result = {
    "status": "PASS_SOURCE_SCHEDULE_AND_EXPONENT_ARITHMETIC_ONLY",
    "blstVersion": "0.3.17", "millerPerTerm": per_term,
    "millerTerms": 2, "finalExponentiation": final_counts,
    "symbolicExponent": {"standardExponentMultiple": 3,
        "coprimeToScalarOrder": True,
        "scope": "Same identity predicate for nonzero Fp12 inputs, not identical target-group value to the standard exponent."},
    "genericTowerProduct": dict(M12),
    "denseProductInstancesIncludingInverseCheck": dense_instances,
    "conservativeArithmeticCore": dict(core),
    "scheduledNormalizationCallsMax": normalizations,
    "extraBoundaries": {"fp12InverseWitnessAssignmentsFp": 12,
        "FpEqualitiesInverseAndFinalIdentity": 24,
        "FpSelectionsForTwoIdentityTermMasks": 24,
        "notNumericallyCosted": ["canonical G1/subgroup/input binding and identity bits",
            "fixed inner SRS line table and Frobenius constants assignment",
            "range-check and normalization auxiliary rows",
            "lookup-table load, column placement, permutation/blinding, full verifier overhead"]},
    "rowUpperBound": None,
    "conditionalRowAccounting": "88480*R_M + 64074*R_A + 27470*R_S + 86*R_neg + 91630*R_norm + 24*R_select + 24*R_equal + 12*R_assign + R_inputs_constants_tables_layout",
    "scope": "No Rust/BLST execution, circuit synthesis, constraints, proof or pairing fixture verification. Symbolic arithmetic assumes the pinned BLS tower field and constrained nonzero inverse relation. No backend, k/SRS, deployment or resource choice.",
    "sourceHashes": {name: hashlib.sha256((here / name).read_bytes()).hexdigest()
                     for name in ["pairing.c", "fp12_tower.c"]},
}
print(json.dumps(result, indent=2))

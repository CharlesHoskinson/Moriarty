"""Small algebra/row-budget checks; no Midnight proof or performance measurement."""
from collections import Counter
from itertools import product
from pathlib import Path
import json
import random


def indexed_permutation(inputs, table):
    """Group raw tuple IDs in fixed table order, reserve one table copy per group."""
    counts = Counter(inputs)
    assert set(counts) <= set(table)
    ap, sp, holes = [], [], []
    for value in dict.fromkeys(table):
        count = counts[value]
        if count:
            start = len(ap)
            ap.extend([value] * count)
            sp.extend([value] + [None] * (count - 1))
            holes.extend(range(start + 1, start + count))
    left = Counter(table)
    for value in counts:
        left[value] -= 1
    for value in dict.fromkeys(table):
        for _ in range(left[value]):
            sp[holes.pop()] = value
    assert not holes
    assert Counter(ap) == Counter(inputs) and Counter(sp) == Counter(table)
    return ap, sp


def blocked_scan(values, p, block):
    local, totals = [], []
    for start in range(0, len(values), block):
        acc, out = 1, []
        for value in values[start:start + block]:
            out.append(acc)
            acc = acc * value % p
        local.append(out)
        totals.append(acc)
    carry, result = 1, []
    for out, total in zip(local, totals):
        result.extend(value * carry % p for value in out)
        carry = carry * total % p
    return result


def run():
    p = 7
    table = [(0, 0), (1, 0), (1, 1), (1, 1)]
    tuples = list(dict.fromkeys(table))
    checks = collision_checks = zero_denominator_cases = product_checks = 0
    for inputs in product(tuples, repeat=len(table)):
        ap, sp = indexed_permutation(inputs, table)
        for theta in range(p):
            compress = lambda v: (theta * v[0] + v[1]) % p
            a, s, aa, ss = [list(map(compress, v)) for v in (inputs, table, ap, sp)]
            assert aa[0] == ss[0]
            assert all((aa[i] - ss[i]) * (aa[i] - aa[i - 1]) % p == 0
                       for i in range(1, len(aa)))
            checks += 1
            collision_checks += len(set(map(compress, tuples))) < len(tuples)
            for beta, gamma in product(range(p), repeat=2):
                den = [(x + beta) * (y + gamma) % p for x, y in zip(aa, ss)]
                if 0 in den:
                    zero_denominator_cases += 1
                    continue  # division recurrence has an exceptional challenge
                z = 1
                for i, d in enumerate(den):
                    numerator = (a[i] + beta) * (s[i] + gamma) % p
                    next_z = z * numerator * pow(d, -1, p) % p
                    assert next_z * d % p == z * numerator % p
                    z = next_z
                assert z == 1
                product_checks += 1
    rejected = 0
    for invalid in [(2, 0), (1, 2), (-1, 0)]:
        try:
            indexed_permutation([invalid] * len(table), table)
        except AssertionError:
            rejected += 1
    assert rejected == 3
    rng = random.Random(41)
    scan_checks = 0
    for length in range(65):
        values = [rng.randrange(p) for _ in range(length)]
        # Inject zeros, including chunk boundaries, to exercise absorbing products.
        if values:
            values[-1] = 0
        serial, acc = [], 1
        for v in values:
            serial.append(acc)
            acc = acc * v % p
        for block in [1, 2, 3, 4, 8, 20]:
            assert blocked_scan(values, p, block) == serial
            scan_checks += 1
    row_counts = {}
    for cap in [15, 16, 17, 18, 19]:
        dp = [0] + [10**6] * 255
        for bound in range(1, 256):
            dp[bound] = 1 + min(dp[bound - lanes * bits]
                                for lanes in range(1, 5)
                                for bits in range(1, cap + 1)
                                if lanes * bits <= bound)
        row_counts[cap] = dp
    result = {
        "classification": "SOURCE: local toy algebra checks, not proving benchmarks",
        "permutation_cases": checks,
        "cases_with_compression_collisions": collision_checks,
        "nonzero_denominator_product_checks": product_checks,
        "exceptional_challenge_cases_skipped": zero_denominator_cases,
        "invalid_tuple_inputs_rejected": rejected,
        "block_scan_checks_including_zero": scan_checks,
        "rows_for_selected_bounds_four_lanes": {
            str(cap): {str(b): rows[b] for b in [56, 64, 128, 255]}
            for cap, rows in row_counts.items()
        },
        "bounds_1_to_255_same_rows_as_cap19": {
            str(cap): sum(a == b for a, b in zip(rows[1:], row_counts[19][1:]))
            for cap, rows in row_counts.items()
        },
        "limits": ["No cryptographic proof generation or BLS arithmetic tested",
                   "No zero-denominator proof acceptance claimed",
                   "Row DP counts decomposition rows only, not synthesis extents",
                   "No CPU/GPU timing or ledger acceptance measured"],
    }
    out = Path(__file__).with_name("model-results.json")
    out.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    run()

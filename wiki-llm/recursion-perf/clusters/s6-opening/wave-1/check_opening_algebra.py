#!/usr/bin/env python3
"""Small public-data algebra experiment; no GPU, proof, SRS or timing claim."""
import random

P = 0x73EDA753299D7D483339D80809A1D80553BDA402FFFE5BFEFFFFFFFF00000001
RNG = random.Random(0x5601)


def evaluate(a, x):
    result = 0
    for c in reversed(a):
        result = (result * x + c) % P
    return result


def divide_linear(a, z):
    q = [0] * (len(a) - 1)
    carry = 0
    for j in range(len(q) - 1, -1, -1):
        carry = (a[j + 1] + z * carry) % P
        q[j] = carry
    return q


def compose(after, before):
    a, b = after
    c, d = before
    return a * c % P, (a * d + b) % P


def scan_division(a, z):
    # Parallel-scan reference, not Axiom's CUDA Brent-Kung implementation.
    states = [(z, c) for c in reversed(a[1:])]
    stride = 1
    while stride < len(states):
        old = states
        states = [compose(pair, old[i - stride]) if i >= stride else pair
                  for i, pair in enumerate(old)]
        stride *= 2
    return [b for _, b in reversed(states)]


def multiply(a, b):
    result = [0] * (len(a) + len(b) - 1)
    for i, ai in enumerate(a):
        for j, bj in enumerate(b):
            result[i + j] = (result[i + j] + ai * bj) % P
    return result


def divide_monic(a, divisor):
    remainder = a[:]
    result = [0] * (len(a) - len(divisor) + 1)
    for j in range(len(result) - 1, -1, -1):
        coefficient = remainder[j + len(divisor) - 1]
        result[j] = coefficient
        for i, di in enumerate(divisor):
            remainder[j + i] = (remainder[j + i] - coefficient * di) % P
    return result


def main():
    scan_cases = eval_cases = sparse_cases = 0
    for n in (1, 2, 3, 4, 7, 8, 16, 31, 32, 64, 128, 256):
        for _ in range(12):
            a = [RNG.randrange(P) for _ in range(n)]
            for z in (0, 1, RNG.randrange(P)):
                q = divide_linear(a, z)
                assert q == scan_division(a, z)
                shifted = a[:]
                shifted[0] = (shifted[0] - evaluate(a, z)) % P
                assert q == divide_linear(shifted, z)
                x = RNG.randrange(P)
                assert (evaluate(a, x) - evaluate(a, z)) % P == (x - z) * evaluate(q, x) % P
                scan_cases += 1
    for n in (2, 4, 8, 16, 32, 64):
        omega = pow(7, (P - 1) // n, P)
        assert pow(omega, n, P) == 1 and pow(omega, n // 2, P) != 1
        domain = [pow(omega, i, P) for i in range(n)]
        for _ in range(8):
            a = [RNG.randrange(P) for _ in range(n)]
            # Exercise ordinary denominators and the exceptional in-domain root.
            for z in (RNG.randrange(P), domain[RNG.randrange(n)]):
                y = evaluate(a, z)
                q = divide_linear(a, z)
                derivative = [i * a[i] % P for i in range(1, n)]
                for x in domain:
                    expected = (evaluate(a, x) - y) * pow((x - z) % P, -1, P) % P if x != z else evaluate(derivative, z)
                    assert evaluate(q, x) == expected
                eval_cases += 1
    for t in (1, 2, 4, 8):
        omega = pow(7, (P - 1) // t, P)
        for _ in range(12):
            root = RNG.randrange(1, P)
            roots = [root * pow(omega, i, P) % P for i in range(t)]
            divisor = [1]
            for z in roots:
                divisor = multiply(divisor, [-z % P, 1])
            assert divisor == [-pow(root, t, P) % P] + [0] * (t - 1) + [1]
            q = [RNG.randrange(P) for _ in range(24)]
            a = multiply(q, divisor)
            repeated = a
            for z in roots:
                repeated = divide_linear(repeated, z)
            assert q == repeated == divide_monic(a, divisor)
            arbitrary = [RNG.randrange(P) for _ in a]
            repeated = arbitrary
            for z in roots:
                repeated = divide_linear(repeated, z)
            assert repeated == divide_monic(arbitrary, divisor)
            sparse_cases += 1
    print(f"PASS: {scan_cases} affine-scan/synthetic-division and constant-subtraction-elision cases")
    print(f"PASS: {eval_cases} evaluation-basis quotient cases, including in-domain roots")
    print(f"PASS: {sparse_cases} sparse vanishing-polynomial/repeated-division cases")
    print("LIMIT: Python field algebra only; no GPU performance, group/MSM, transcript, SRS or ledger acceptance validated.")


if __name__ == '__main__':
    main()

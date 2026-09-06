"""Python reference for ZKIR v3 values: field constants, curve arithmetic and
the public-input encodings (shared by the unit tests and the differential
harness). Everything here is cross-checked against the K definition by
unit_values.py; the K definition is cross-checked against the Rust crate by
diff_test.py.
"""
from __future__ import annotations

import random

R = 52435875175126190479447740508185965837690552500527637822603658699938581184513
RJ = 6554484396890773809930967563523245729705921265872317281365359162392183254199
K256P = 2**256 - 2**32 - 977
K256N = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141
P256P = 0xFFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF
P256N = 0xFFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551
P256B = 0x5AC635D8AA3A93E7B3EBBD55769886BC651D06B0CC53B0F63BCE3C3E27D2604B
C25519P = 2**255 - 19
C25519L = 2**252 + 27742317777372353535851937790883648493
JUBJUB_D = (-10240 * pow(10241, -1, R)) % R
C25519_D = (-121665 * pow(121666, -1, C25519P)) % C25519P
JUBJUB_G_FULL = (44746807950788659978687200207992930935149218647843500701850233404325651525118, 11)
C25519_G = (15112221349535400772501151409588531511454012693041857206046113283949847762202,
            46316835694926478169428394003475163141307993866256225615783033603165251855960)
K256_G = (0x79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798,
          0x483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8)
P256_G = (0x6B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296,
          0x4FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5)


def ed_add(p, q, mod, d):
    (x1, y1), (x2, y2) = p, q
    t = d * x1 * x2 * y1 * y2 % mod
    x3 = (x1 * y2 + y1 * x2) * pow(1 + t, -1, mod) % mod
    y3 = (y1 * y2 + x1 * x2) * pow(1 - t, -1, mod) % mod
    return (x3, y3)


def ed_mul(p, k, mod, d):
    acc = (0, 1)
    while k:
        if k & 1:
            acc = ed_add(acc, p, mod, d)
        p = ed_add(p, p, mod, d)
        k >>= 1
    return acc


def w_add(p, q, mod, a):
    if p is None:
        return q
    if q is None:
        return p
    (x1, y1), (x2, y2) = p, q
    if x1 == x2 and (y1 + y2) % mod == 0:
        return None
    if p == q:
        lam = (3 * x1 * x1 + a) * pow(2 * y1, -1, mod) % mod
    else:
        lam = (y2 - y1) * pow(x2 - x1, -1, mod) % mod
    x3 = (lam * lam - x1 - x2) % mod
    y3 = (lam * (x1 - x3) - y1) % mod
    return (x3, y3)


def w_mul(p, k, mod, a):
    acc = None
    while k:
        if k & 1:
            acc = w_add(acc, p, mod, a)
        p = w_add(p, p, mod, a)
        k >>= 1
    return acc


JUBJUB_G = ed_mul(JUBJUB_G_FULL, 8, R, JUBJUB_D)   # subgroup generator


def enc_foreign(x, mod, log2, nb):
    e = (x - 1) % mod
    per = 254 // log2
    out = []
    left = nb
    while left > 0:
        take = min(left, per)
        out.append(e & ((1 << (log2 * take)) - 1))
        e >>= log2 * take
        left -= take
    return out


def enc_wpoint(p, mod):
    if p is None:
        return enc_foreign(0, mod, 64, 4) * 2 + [1]
    return enc_foreign(p[0], mod, 64, 4) + enc_foreign(p[1], mod, 64, 4) + [0]


ENCODED_LEN = {
    'Scalar<BLS12-381>': 1, 'Bytes<32>': 2, 'Point<Jubjub>': 2, 'Scalar<Jubjub>': 1,
    'Point<Secp256k1>': 5, 'Base<Secp256k1>': 2, 'Scalar<Secp256k1>': 2,
    'Point<Secp256r1>': 5, 'Base<Secp256r1>': 2, 'Scalar<Secp256r1>': 2,
    'Point<Curve25519>': 4, 'Base<Curve25519>': 2, 'Scalar<Curve25519>': 2,
}


def random_encoded(ir_type: str, rng: random.Random) -> list[int]:
    """A valid raw encoding of a random value of the given type."""
    if ir_type == 'Bool':
        return [rng.randrange(2)]
    if ir_type == 'Byte':
        return [rng.randrange(256)]
    if ir_type.startswith('Bytes<') and ir_type != 'Bytes<32>':
        n = int(ir_type[6:-1])
        b = bytes(rng.randrange(256) for _ in range(n))
        return [int.from_bytes(b[i:i + 31], 'little') for i in range(0, n, 31)]
    match ir_type:
        case 'Scalar<BLS12-381>':
            return [rng.randrange(R)]
        case 'Bytes<32>':
            return [rng.randrange(1 << 248), rng.randrange(256)]
        case 'Point<Jubjub>':
            p = ed_mul(JUBJUB_G, rng.randrange(1, RJ), R, JUBJUB_D)
            return [p[0], p[1]]
        case 'Scalar<Jubjub>':
            return [rng.randrange(RJ)]
        case 'Point<Secp256k1>':
            return enc_wpoint(w_mul(K256_G, rng.randrange(1, K256N), K256P, 0), K256P)
        case 'Base<Secp256k1>':
            return enc_foreign(rng.randrange(K256P), K256P, 64, 4)
        case 'Scalar<Secp256k1>':
            return enc_foreign(rng.randrange(K256N), K256N, 64, 4)
        case 'Point<Secp256r1>':
            return enc_wpoint(w_mul(P256_G, rng.randrange(1, P256N), P256P, P256P - 3), P256P)
        case 'Base<Secp256r1>':
            return enc_foreign(rng.randrange(P256P), P256P, 64, 4)
        case 'Scalar<Secp256r1>':
            return enc_foreign(rng.randrange(P256N), P256N, 64, 4)
        case 'Point<Curve25519>':
            p = ed_mul(C25519_G, rng.randrange(1, C25519L), C25519P, C25519_D)
            return enc_foreign(p[0], C25519P, 64, 4) + enc_foreign(p[1], C25519P, 64, 4)
        case 'Base<Curve25519>':
            return enc_foreign(rng.randrange(C25519P), C25519P, 64, 4)
        case 'Scalar<Curve25519>':
            return enc_foreign(rng.randrange(C25519L), C25519L, 51, 5)
    raise ValueError(ir_type)


def small_encoded(ir_type: str, rng: random.Random) -> list[int]:
    """Like random_encoded but native values are small booleans/bytes, which
    keeps guards, bit bounds and assertions in the Compact-generated programs
    plausible."""
    if ir_type == 'Scalar<BLS12-381>':
        return [rng.choice([0, 1, 1, 1, 2, 7, 255])]
    return random_encoded(ir_type, rng)

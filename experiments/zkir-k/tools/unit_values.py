"""Unit tests for ZKIR-FIELD, ZKIR-CURVES and ZKIR-VALUES (milestone 3).

Evaluates K function terms through the ZKIR-TEST definition and compares with
an independent Python implementation of the same textbook formulas and of the
midnight-circuits public-input encodings. Prints one line per check; exit 1 on
any mismatch.

Usage: uv run --group zkir-k python experiments/zkir-k/tools/unit_values.py
"""
from __future__ import annotations

import sys
from pathlib import Path

from pyk.kast.inner import KApply, KInner, KSort, KToken
from pyk.kast.prelude.kint import intToken
from pyk.kore.parser import KoreParser
from pyk.ktool.krun import KRun

DEF = Path(__file__).resolve().parent.parent / 'semantics' / 'zkir-test-kompiled'

# --- Python reference -------------------------------------------------------------
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
JUBJUB_G = (44746807950788659978687200207992930935149218647843500701850233404325651525118, 11)
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


# --- K side ---------------------------------------------------------------------------
krun = KRun(DEF)


def run(term: KInner) -> str:
    kore = krun.kast_to_kore(term, KSort('KItem'))
    res = krun.run_process(kore)
    assert res.returncode == 0, res.stderr
    out = krun.pretty_print(krun.kore_to_kast(KoreParser(res.stdout).pattern()))
    # strip the <k> cell wrapper
    start = out.find('<k>') + 3
    end = out.rfind('~> .K')
    return out[start:end].strip()


_SYMBOLS = list(krun.definition.symbols.keys())


def resolve(label: str) -> str:
    """Symbol name for a production: exact `symbol(_)` name, else the unique
    mangled label whose base name matches (e.g. finv -> finv(_,_)_ZKIR-FIELD_...)."""
    if label in krun.definition.symbols:
        return label
    hits = [s for s in _SYMBOLS if s.startswith(label + '(') or s.startswith(label + '_') or s == label]
    if len(hits) != 1:
        raise KeyError(f'{label}: {hits}')
    return hits[0]


def K(label: str, *args: KInner) -> KInner:
    return KApply(resolve(label), list(args))


def I(n: int) -> KInner:
    return intToken(n)


def pt(x, y):
    return K('pt', I(x), I(y))


def klist(items):
    t = KApply('.List')
    for it in reversed(items):
        t = KApply('_List_', [KApply('ListItem', [it]), t])
    return t


def pretty_pt(p):
    return 'inf ( )' if p is None else f'pt ( {p[0]} , {p[1]} )'


def pretty_list(items):
    return ' '.join(f'ListItem ( {i} )' for i in items) if items else '.List'


checks = []


def check(name, term, expected):
    actual = run(term)
    ok = ' '.join(actual.split()) == ' '.join(expected.split())
    checks.append(ok)
    print(('PASS' if ok else 'FAIL'), name, '' if ok else f'\n   expected: {expected}\n   actual:   {actual}')


# field
check('finv', K('finv', I(12345), K('#r')), str(pow(12345, -1, R)))
check('fsqrt 4', K('fsqrt', I(4), K('#r')), 'sqrtOk ( 2 )' if run(K('fsqrt', I(4), K('#r'))) == 'sqrtOk ( 2 )' else f'sqrtOk ( {R - 2} )')
sq = run(K('fsqrt', I(JUBJUB_D), K('#r')))
check('jubjub d is a non-residue', K('fsqrt', I(JUBJUB_D), K('#r')), 'noSqrt ( )')
check('curve25519 d is a non-residue', K('fsqrt', I(C25519_D), K('#c25519P')), 'noSqrt ( )')
check('legendre 3 mod r', K('legendre', I(3), K('#r')), str(pow(3, (R - 1) // 2, R)))
# curves
check('jubjub d', K('#jubjubD'), str(JUBJUB_D))
check('jubjub G on curve', K('onCurve', K('#jubjub'), pt(*JUBJUB_G)), 'true')
G8 = ed_mul(JUBJUB_G, 8, R, JUBJUB_D)
check('jubjub subgroup generator = 8G', K('#jubjubGenerator'), pretty_pt(G8))
check('jubjub 8G in subgroup', K('inSubgroup', K('#jubjub'), pt(*G8)), 'true')
check('jubjub G not in subgroup', K('inSubgroup', K('#jubjub'), pt(*JUBJUB_G)), 'false')
check('jubjub 12345 * 8G', K('ecMul', K('#jubjub'), pt(*G8), I(12345)), pretty_pt(ed_mul(G8, 12345, R, JUBJUB_D)))
check('curve25519 B on curve', K('onCurve', K('#curve25519'), pt(*C25519_G)), 'true')
check('curve25519 B in subgroup', K('inSubgroup', K('#curve25519'), pt(*C25519_G)), 'true')
check('curve25519 7 * B', K('ecMul', K('#curve25519'), pt(*C25519_G), I(7)), pretty_pt(ed_mul(C25519_G, 7, C25519P, C25519_D)))
check('secp256k1 G on curve', K('onCurve', K('#secp256k1'), pt(*K256_G)), 'true')
check('secp256k1 n * G = inf', K('ecMul', K('#secp256k1'), pt(*K256_G), I(K256N)), 'inf ( )')
check('secp256k1 999 * G', K('ecMul', K('#secp256k1'), pt(*K256_G), I(999)), pretty_pt(w_mul(K256_G, 999, K256P, 0)))
check('secp256r1 G on curve', K('onCurve', K('#secp256r1'), pt(*P256_G)), 'true')
check('secp256r1 n * G = inf', K('ecMul', K('#secp256r1'), pt(*P256_G), I(P256N)), 'inf ( )')
check('secp256r1 31 * G', K('ecMul', K('#secp256r1'), pt(*P256_G), I(31)), pretty_pt(w_mul(P256_G, 31, P256P, P256P - 3)))
# Jubjub decompression: from_xy(x, y) with x's parity only
check('jubjubFromXY exact', K('jubjubFromXY', I(G8[0]), I(G8[1])), f'ptOk ( pt ( {G8[0]} , {G8[1]} ) )')
xneg = (-G8[0]) % R
check('jubjubFromXY negated x', K('jubjubFromXY', I(xneg), I(G8[1])), f'ptOk ( pt ( {xneg} , {G8[1]} ) )')
# a wrong x with the same parity as the real one recovers the real point
wrong = (G8[0] + 2) % R
check('jubjubFromXY wrong x same parity', K('jubjubFromXY', I(wrong), I(G8[1])), f'ptOk ( pt ( {G8[0]} , {G8[1]} ) )')
# encodings
x = 0x1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF % K256P
check('enc secp256k1Base', K('encodeValue', K('secp256k1BaseV', I(x))), pretty_list(enc_foreign(x, K256P, 64, 4)))
check('enc secp256k1Base 0', K('encodeValue', K('secp256k1BaseV', I(0))), pretty_list(enc_foreign(0, K256P, 64, 4)))
check('dec secp256k1Base', K('decodeValue', klist([I(v) for v in enc_foreign(x, K256P, 64, 4)]), K('Secp256k1Base')), f'decOk ( secp256k1Base ( {x} ) )')
s = 0xABCDEF % C25519L
check('enc curve25519Scalar', K('encodeValue', K('curve25519ScalarV', I(s))), pretty_list(enc_foreign(s, C25519L, 51, 5)))
check('dec curve25519Scalar', K('decodeValue', klist([I(v) for v in enc_foreign(s, C25519L, 51, 5)]), K('Curve25519Scalar')), f'decOk ( curve25519Scalar ( {s} ) )')
encG = enc_foreign(K256_G[0], K256P, 64, 4) + enc_foreign(K256_G[1], K256P, 64, 4) + [0]
check('enc secp256k1Point G', K('encodeValue', K('secp256k1PointV', pt(*K256_G))), pretty_list(encG))
check('dec secp256k1Point G', K('decodeValue', klist([I(v) for v in encG]), K('Secp256k1Point')), f'decOk ( secp256k1Point ( pt ( {K256_G[0]} , {K256_G[1]} ) ) )')
encInf = enc_foreign(0, K256P, 64, 4) * 2 + [1]
check('enc secp256k1Point inf', K('encodeValue', K('secp256k1PointV', K('inf'))), pretty_list(encInf))
check('dec secp256k1Point inf', K('decodeValue', klist([I(v) for v in encInf]), K('Secp256k1Point')), 'decOk ( secp256k1Point ( inf ( ) ) )')
encB = enc_foreign(C25519_G[0], C25519P, 64, 4) + enc_foreign(C25519_G[1], C25519P, 64, 4)
check('enc curve25519Point B', K('encodeValue', K('curve25519PointV', pt(*C25519_G))), pretty_list(encB))
check('dec curve25519Point B', K('decodeValue', klist([I(v) for v in encB]), K('Curve25519Point')), f'decOk ( curve25519Point ( pt ( {C25519_G[0]} , {C25519_G[1]} ) ) )')
check('dec jubjubPoint 8G', K('decodeValue', klist([I(G8[0]), I(G8[1])]), K('JubjubPoint')), f'decOk ( jubjubPoint ( pt ( {G8[0]} , {G8[1]} ) ) )')
check('dec jubjubPoint G (not subgroup)', K('decodeValue', klist([I(JUBJUB_G[0]), I(JUBJUB_G[1])]), K('JubjubPoint')), 'decErr ( "Failed to decode as JubjubPoint" )')
check('dec jubjubScalar', K('decodeValue', klist([I(RJ - 1)]), K('JubjubScalar')), f'decOk ( jubjubScalar ( {RJ - 1} ) )')
check('dec jubjubScalar too big', K('decodeValue', klist([I(RJ)]), K('JubjubScalar')), 'decErr ( "Failed to decode as JubjubScalar" )')
b = bytes(range(1, 33))
lo = int.from_bytes(b[:31], 'little')
check('enc bytes32', K('encodeValue', K('bytes32V', KToken(f'b"{"".join(chr(c) for c in b)}"', KSort('Bytes')))), pretty_list([lo, 32]))
check('dec bytes32', K('decodeValue', klist([I(lo), I(32)]), K('Bytes32')), f'decOk ( bytes32 ( {run(KToken("b" + chr(34) + "".join(chr(92) + "x%02x" % c for c in b) + chr(34), KSort("Bytes")))} ) )')
check('dec bytes32 bad high (Rust assert_eq! panics)', K('decodeValue', klist([I(lo), I(256)]), K('Bytes32')), 'decPanic ( "assertion failed: Bytes32 low element uses byte 31 or high element exceeds a byte" )')
check('dec native out of field', K('decodeValue', klist([I(R)]), K('Native')), 'decErr ( "is not a canonical field element" )')
check('dec bytes32 bad high, strict (2ffe2d1 decode_bytes returns None)', K('decodeStrict', klist([I(lo), I(256)]), K('Bytes32'), KToken('true', 'Bool')), 'decErr ( "Failed to decode as Bytes32" )')

# the semantic witness space (2026-09-06 review, item 2): wellTyped is a
# semantic predicate, not a sort check; witnessSpace(.List, M) is false on a
# malformed register and true on a well-formed one
from pyk.kast.prelude.string import stringToken


def mem(v):
    return KApply('_|->_', [stringToken('%x'), v])


def kbytes(b: bytes):
    return KToken('b"' + ''.join('\\x%02x' % c for c in b) + '"', KSort('Bytes'))


check('witnessSpace false: native(r) out of the field', K('witnessSpace', KApply('.List'), mem(K('nativeV', I(R)))), 'false')
check('witnessSpace false: jubjubPoint(pt(0, 0)) off the curve', K('witnessSpace', KApply('.List'), mem(K('jubjubPointV', pt(0, 0)))), 'false')
check('witnessSpace false: a 3-byte bytes32', K('witnessSpace', KApply('.List'), mem(K('bytes32V', kbytes(b'abc')))), 'false')
check('witnessSpace false: secp256k1Base(p) not below the modulus', K('witnessSpace', KApply('.List'), mem(K('secp256k1BaseV', I(K256P)))), 'false')
check('witnessSpace true: native(5)', K('witnessSpace', KApply('.List'), mem(K('nativeV', I(5)))), 'true')
check('witnessSpace true: jubjubPoint(8G) in the subgroup', K('witnessSpace', KApply('.List'), mem(K('jubjubPointV', pt(*G8)))), 'true')

print(f'\n{sum(checks)}/{len(checks)} checks passed')
sys.exit(0 if all(checks) else 1)

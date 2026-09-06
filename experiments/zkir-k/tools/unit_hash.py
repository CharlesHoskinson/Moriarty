"""Known-answer tests for ZKIR-HASH against the Rust oracle (milestone 3/4).

Runs the crate's `preprocess` (zkir-oracle, built from midnight-ledger 92e8bdd3
in repos/_build/ledger-92e8bdd3) on corpus/handmade/{transient_hash,std_hashes}.zkir
and checks that K's poseidonHash, hashToCurve, sha256 and keccak256 give the
same register values. Also checks sha256 against hashlib.

Usage: uv run --group zkir-k python experiments/zkir-k/tools/unit_hash.py
"""
from __future__ import annotations

import hashlib
import json
import subprocess
import sys
import tempfile
from pathlib import Path

from pyk.kast.inner import KApply, KInner, KSort, KToken
from pyk.kast.prelude.kint import intToken
from pyk.kore.parser import KoreParser
from pyk.ktool.krun import KRun

HERE = Path(__file__).resolve().parent
DEF = HERE.parent / 'semantics' / 'zkir-test-kompiled'
CORPUS = HERE.parent / 'corpus' / 'handmade'
ORACLE = Path.home() / 'Moriarty/repos/_build/ledger-92e8bdd3/target/release/zkir-oracle'

krun = KRun(DEF)
_SYMBOLS = list(krun.definition.symbols.keys())


def resolve(label: str) -> str:
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


def klist(items):
    t = KApply('.List')
    for it in reversed(items):
        t = KApply('_List_', [KApply('ListItem', [it]), t])
    return t


def kbytes(b: bytes) -> KInner:
    esc = ''.join('\\x%02x' % c for c in b)
    return KToken(f'b"{esc}"', KSort('Bytes'))


def run(term: KInner) -> str:
    kore = krun.kast_to_kore(term, KSort('KItem'))
    res = krun.run_process(kore)
    assert res.returncode == 0, res.stderr
    out = krun.pretty_print(krun.kore_to_kast(KoreParser(res.stdout).pattern()))
    start = out.find('<k>') + 3
    end = out.rfind('~> .K')
    return ' '.join(out[start:end].split())


def oracle(program: Path, inputs: list[int]) -> dict:
    with tempfile.NamedTemporaryFile('w', suffix='.json', delete=False) as f:
        json.dump({'inputs': [str(i) for i in inputs], 'binding_input': '42'}, f)
    res = subprocess.run([str(ORACLE), str(program), f.name], capture_output=True, text=True, check=True)
    return json.loads(res.stdout)


def pretty_list(items):
    return ' '.join(f'ListItem ( {i} )' for i in items) if items else '.List'


checks = []


def check(name, actual, expected):
    ok = actual == expected
    checks.append(ok)
    print(('PASS' if ok else 'FAIL'), name, '' if ok else f'\n   expected: {expected}\n   actual:   {actual}')


a, b, c = 5, 123456789, 987654321987654321
mem = oracle(CORPUS / 'transient_hash.zkir', [a, b, c])
assert mem['status'] == 'ok', mem
m = mem['memory']
check('poseidon [a,b,c]', run(K('poseidonHash', klist([I(a), I(b), I(c)]))), m['%h3']['encoded'][0])
check('poseidon [a]', run(K('poseidonHash', klist([I(a)]))), m['%h1']['encoded'][0])
check('poseidon []', run(K('poseidonHash', klist([]))), m['%h0']['encoded'][0])
check('poseidon [a,b]', run(K('poseidonHash', klist([I(a), I(b)]))), m['%h2']['encoded'][0])
for name, xs in [('%p3', [a, b, c]), ('%p1', [a]), ('%p0', [])]:
    x, y = m[name]['encoded']
    check(f'hash_to_curve {xs}', run(K('hashToCurve', klist([I(v) for v in xs]))), f'pt ( {x} , {y} )')

# sha256 / keccak256 through the alignment layer
mem = oracle(CORPUS / 'std_hashes.zkir', [a, b, c])
assert mem['status'] == 'ok', mem
m = mem['memory']
digest = hashlib.sha256(bytes([a])).digest()
check('sha256 hashlib', run(K('encodeValue', K('bytes32V', K('sha256Bytes', kbytes(bytes([a])))))),
      pretty_list([int.from_bytes(digest[:31], 'little'), digest[31]]))
check('sha256 bytes(1) vs oracle', run(K('encodeValue', K('bytes32V', K('sha256Bytes', kbytes(bytes([a])))))), pretty_list(m['%s1']['encoded']))
check('keccak256 bytes(1) vs oracle', run(K('encodeValue', K('bytes32V', K('keccak256Bytes', kbytes(bytes([a])))))), pretty_list(m['%k1']['encoded']))
# bytes(32) from [a, c]: stray byte first (a), then 31 bytes of c
pre32 = c.to_bytes(31, 'little') + bytes([a])
digest32 = hashlib.sha256(pre32).digest()
check('sha256 bytes(32) alignment vs hashlib', pretty_list(m['%s32']['encoded']), pretty_list([int.from_bytes(digest32[:31], 'little'), digest32[31]]))
align32 = K('alignment', K('segments', K('atom', K('bytesAtom', I(32))), K('.segments')))
check('alignedBytes bytes(32)', run(K('alignedBytes', align32, klist([I(a), I(c)]))), f'bOk ( {run(kbytes(pre32))} )')
check('sha256 bytes(32) vs oracle', run(K('encodeValue', K('bytes32V', K('sha256Bytes', kbytes(pre32))))), pretty_list(m['%s32']['encoded']))
check('keccak256 bytes(32) vs oracle', run(K('encodeValue', K('bytes32V', K('keccak256Bytes', kbytes(pre32))))), pretty_list(m['%k32']['encoded']))
# field + bytes(6): 32 LE bytes of c, then 6 bytes of a
pref = c.to_bytes(32, 'little') + a.to_bytes(6, 'little')
alignf = K('alignment', K('segments', K('atom', K('fieldAtom')), K('segments', K('atom', K('bytesAtom', I(6))), K('.segments'))))
check('alignedBytes field+bytes(6)', run(K('alignedBytes', alignf, klist([I(c), I(a)]))), f'bOk ( {run(kbytes(pref))} )')
check('sha256 field+bytes(6) vs oracle', run(K('encodeValue', K('bytes32V', K('sha256Bytes', kbytes(pref))))), pretty_list(m['%sf']['encoded']))
# bytes(200) from 7 elements + field: six 31-byte chunks (elements 1..6 reversed), then 200 % 31 = 14 stray bytes from element 0
els = [a, b, c, a, b, c, a]
# bytes_from_field_repr: the 31-byte chunks come first (elements after the stray one, reversed), the stray bytes last
pre200 = b''.join(e.to_bytes(31, 'little') for e in reversed(els[1:])) + els[0].to_bytes(14, 'little') + c.to_bytes(32, 'little')
assert len(pre200) == 232
check('keccak256 bytes(200)+field vs oracle', run(K('encodeValue', K('bytes32V', K('keccak256Bytes', kbytes(pre200))))), pretty_list(m['%k200']['encoded']))
align200 = K('alignment', K('segments', K('atom', K('bytesAtom', I(200))), K('segments', K('atom', K('fieldAtom')), K('.segments'))))
check('alignedBytes bytes(200)+field', run(K('alignedBytes', align200, klist([I(v) for v in els + [c]]))), f'bOk ( {run(kbytes(pre200))} )')

print(f'\n{sum(checks)}/{len(checks)} checks passed')
sys.exit(0 if all(checks) else 1)

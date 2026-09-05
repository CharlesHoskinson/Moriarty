"""Run a ZKIR v3 program through the K semantics (ZKIR-VM) and read the result.

`run_job(program_path, preimage)` builds job(Program, Preimage), runs krun on
the ZKIR definition, and returns a dict shaped like the Rust oracle's output:
  {status: 'ok'|'error', error?: str, memory: {id: {variant, encoded}},
   pis: [str], pi_skips: [None|int], cursors: (pubIn, pubOut, priv),
   constraints: int}
Memory values are re-encoded on the Python side with the same encoding as
`encode_offcircuit` (the encodings are unit-tested against K in unit_values.py).

Usage: zkir_run.py FILE.zkir PREIMAGE.json [--definition DIR]
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any

from pyk.kast.inner import KApply, KInner, KSequence, KSort, KToken, top_down
from pyk.kast.prelude.kint import intToken
from pyk.kore.parser import KoreParser
from pyk.ktool.krun import KRun

sys.path.insert(0, str(Path(__file__).resolve().parent))
import zkir_kast  # noqa: E402

R = 52435875175126190479447740508185965837690552500527637822603658699938581184513
RJ = 6554484396890773809930967563523245729705921265872317281365359162392183254199
K256P = 2**256 - 2**32 - 977
K256N = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141
P256P = 0xFFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF
P256N = 0xFFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551
C25519P = 2**255 - 19
C25519L = 2**252 + 27742317777372353535851937790883648493


def default_definition() -> Path:
    return Path(__file__).resolve().parent.parent / 'semantics' / 'zkir-kompiled'


def default_ext_definition() -> Path:
    return Path(__file__).resolve().parent.parent / 'semantics' / 'zkir-ext-kompiled'


# --- preimage term ---------------------------------------------------------------------

def klist(items: list[KInner]) -> KInner:
    t: KInner = KApply('.List')
    for it in reversed(items):
        t = KApply('_List_', [KApply('ListItem', [it]), t])
    return t


class PreimageError(ValueError):
    """A preimage integer is not a canonical BLS12-381 scalar (the Rust
    `ProofPreimage` holds `Fr` values; the oracle rejects such inputs)."""


def fr(v) -> int:
    n = int(v)
    if not 0 <= n < R:
        raise PreimageError(f'{n} is not a canonical field element')
    return n


def ints(values) -> KInner:
    return klist([intToken(fr(v)) for v in values])


def preimage_term(p: dict[str, Any]) -> KInner:
    cc = p.get('communications_commitment')
    comm = KApply('noComm') if cc is None else KApply('comm', [intToken(fr(cc[0])), intToken(fr(cc[1]))])
    return KApply('preimage', [
        ints(p.get('inputs', [])),
        intToken(fr(p.get('binding_input', 0))),
        comm,
        ints(p.get('private_transcript', [])),
        ints(p.get('public_transcript_inputs', [])),
        ints(p.get('public_transcript_outputs', [])),
    ])


# --- result extraction --------------------------------------------------------------------

def find_cell(term: KInner, name: str) -> KInner:
    found: list[KInner] = []

    def visit(t: KInner) -> KInner:
        if isinstance(t, KApply) and t.label.name == name:
            found.append(t.args[0])
        return t

    top_down(visit, term)
    if not found:
        raise KeyError(name)
    return found[0]


def list_items(term: KInner) -> list[KInner]:
    items: list[KInner] = []

    def walk(t: KInner) -> None:
        if isinstance(t, KApply):
            if t.label.name == '_List_':
                for a in t.args:
                    walk(a)
            elif t.label.name == 'ListItem':
                items.append(t.args[0])
            elif t.label.name == '.List':
                pass
            else:
                raise ValueError(f'not a list: {t.label.name}')
    walk(term)
    return items


def map_items(term: KInner) -> list[tuple[KInner, KInner]]:
    items: list[tuple[KInner, KInner]] = []

    def walk(t: KInner) -> None:
        if isinstance(t, KApply):
            if t.label.name == '_Map_':
                for a in t.args:
                    walk(a)
            elif t.label.name == '_|->_':
                items.append((t.args[0], t.args[1]))
            elif t.label.name == '.Map':
                pass
            else:
                raise ValueError(f'not a map: {t.label.name}')
    walk(term)
    return items


def tok_int(t: KInner) -> int:
    assert isinstance(t, KToken), t
    return int(t.token)


def tok_str(t: KInner) -> str:
    assert isinstance(t, KToken) and t.sort.name == 'String', t
    return json.loads(t.token)


_ESC = {'t': 9, 'n': 10, 'r': 13, 'f': 12, '"': 34, '\\': 92}


def tok_bytes(t: KInner) -> bytes:
    assert isinstance(t, KToken) and t.sort.name == 'Bytes', t
    s = t.token
    assert s.startswith('b"') and s.endswith('"'), s
    body = s[2:-1]
    out = bytearray()
    i = 0
    while i < len(body):
        ch = body[i]
        if ch == '\\':
            nxt = body[i + 1]
            if nxt == 'x':
                out.append(int(body[i + 2:i + 4], 16))
                i += 4
            elif nxt == 'u':
                out.append(int(body[i + 2:i + 6], 16))
                i += 6
            else:
                out.append(_ESC[nxt])
                i += 2
        else:
            out.append(ord(ch))
            i += 1
    return bytes(out)


def enc_foreign(x: int, mod: int, log2: int, nb: int) -> list[int]:
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


def point(t: KInner):
    assert isinstance(t, KApply)
    if t.label.name == 'inf':
        return None
    assert t.label.name == 'pt', t
    return (tok_int(t.args[0]), tok_int(t.args[1]))


def enc_chunks(b: bytes) -> list[int]:
    return [int.from_bytes(b[i:i + 31], 'little') for i in range(0, len(b), 31)]


def type_string(t: KInner, ext: bool) -> str:
    """The Rust `{:?}` of `IrType` for this value (Bytes<n> is `Bytes(n)` at 2ffe2d1)."""
    assert isinstance(t, KApply)
    lbl = t.label.name
    if lbl == 'bytes32V':
        return 'Bytes(32)' if ext else 'Bytes32'
    if lbl == 'bytesV':
        return f'Bytes({len(tok_bytes(t.args[0]))})'
    return {'nativeV': 'Native', 'boolV': 'Bool', 'byteV': 'Byte', 'jubjubPointV': 'JubjubPoint',
            'jubjubScalarV': 'JubjubScalar', 'secp256k1PointV': 'Secp256k1Point', 'secp256k1BaseV': 'Secp256k1Base',
            'secp256k1ScalarV': 'Secp256k1Scalar', 'secp256r1PointV': 'Secp256r1Point', 'secp256r1BaseV': 'Secp256r1Base',
            'secp256r1ScalarV': 'Secp256r1Scalar', 'curve25519PointV': 'Curve25519Point', 'curve25519BaseV': 'Curve25519Base',
            'curve25519ScalarV': 'Curve25519Scalar'}[lbl]


def encode_value(t: KInner, ext: bool = False) -> tuple[str, list[int]]:
    assert isinstance(t, KApply), t
    lbl = t.label.name
    a = t.args
    match lbl:
        case 'nativeV':
            return 'Native', [tok_int(a[0])]
        case 'bytes32V':
            b = tok_bytes(a[0])
            return ('Bytes' if ext else 'Bytes32'), [int.from_bytes(b[:31], 'little'), b[31]]
        case 'boolV':
            return 'Bool', [1 if (a[0].token if isinstance(a[0], KToken) else a[0].label.name) == 'true' else 0]
        case 'byteV':
            return 'Byte', [tok_int(a[0])]
        case 'bytesV':
            return 'Bytes', enc_chunks(tok_bytes(a[0]))
        case 'jubjubPointV':
            p = point(a[0]) or (0, 1)
            return 'JubjubPoint', [p[0], p[1]]
        case 'jubjubScalarV':
            return 'JubjubScalar', [tok_int(a[0])]
        case 'secp256k1PointV' | 'secp256r1PointV':
            mod = K256P if lbl.startswith('secp256k1') else P256P
            p = point(a[0])
            if p is None:
                return lbl[:-1].capitalize().replace('point', 'Point'), enc_foreign(0, mod, 64, 4) * 2 + [1]
            return ('Secp256k1Point' if mod == K256P else 'Secp256r1Point'), enc_foreign(p[0], mod, 64, 4) + enc_foreign(p[1], mod, 64, 4) + [0]
        case 'secp256k1BaseV':
            return 'Secp256k1Base', enc_foreign(tok_int(a[0]), K256P, 64, 4)
        case 'secp256k1ScalarV':
            return 'Secp256k1Scalar', enc_foreign(tok_int(a[0]), K256N, 64, 4)
        case 'secp256r1BaseV':
            return 'Secp256r1Base', enc_foreign(tok_int(a[0]), P256P, 64, 4)
        case 'secp256r1ScalarV':
            return 'Secp256r1Scalar', enc_foreign(tok_int(a[0]), P256N, 64, 4)
        case 'curve25519PointV':
            p = point(a[0]) or (0, 1)
            return 'Curve25519Point', enc_foreign(p[0], C25519P, 64, 4) + enc_foreign(p[1], C25519P, 64, 4)
        case 'curve25519BaseV':
            return 'Curve25519Base', enc_foreign(tok_int(a[0]), C25519P, 64, 4)
        case 'curve25519ScalarV':
            return 'Curve25519Scalar', enc_foreign(tok_int(a[0]), C25519L, 51, 5)
    raise ValueError(lbl)


SYMBOL_TO_TYPE = {v: k for k, v in zkir_kast.TYPE_SYMBOLS.items()}
SYMBOL_TO_TYPE.update({'Bool': 'Bool', 'Byte': 'Byte'})


def type_of_symbol(t: KInner) -> str:
    assert isinstance(t, KApply)
    if t.label.name == 'BytesN':
        return f'Bytes<{tok_int(t.args[0])}>'
    return SYMBOL_TO_TYPE[t.label.name]


def need(t: KInner) -> tuple[str, Any]:
    assert isinstance(t, KApply)
    match t.label.name:
        case 'needPubOut':
            return ('pubOut', type_of_symbol(t.args[0]))
        case 'needPriv':
            return ('priv', type_of_symbol(t.args[0]))
        case 'needPubIn':
            return ('pubIn', tok_int(t.args[0]))
        case 'needComm':
            return ('comm', tok_int(t.args[0]))
    raise ValueError(t.label.name)


def skip(t: KInner):
    assert isinstance(t, KApply)
    if t.label.name == 'skipNone':
        return None
    return tok_int(t.args[0])


class Runner:
    def __init__(self, definition: Path | None = None, ext: bool = False):
        self.ext = ext
        self.krun = KRun(definition or (default_ext_definition() if ext else default_definition()))

    def run(self, program: KInner, preimage: dict[str, Any], depth: int | None = None, gen: bool = False, checked: bool = False) -> dict[str, Any]:
        """checked=True runs the static check `wf` first (checkedJob); the default
        raw entry point models `preprocess` on any program `IrSource::load` accepts."""
        job = KApply('genJob' if gen else ('checkedJob' if checked else 'job'), [program, preimage_term(preimage)])
        kore = self.krun.kast_to_kore(job, KSort('Job'))
        res = self.krun.run_process(kore, depth=depth)
        if res.returncode != 0:
            return {'status': 'krun-failed', 'error': res.stderr[-2000:]}
        cfg = self.krun.kore_to_kast(KoreParser(res.stdout).pattern())
        status = find_cell(cfg, '<status>')
        assert isinstance(status, KApply)
        out: dict[str, Any] = {}
        k_cell = find_cell(cfg, '<k>')
        finished = isinstance(k_cell, KSequence) and len(k_cell.items) == 0
        if not finished:
            # a configuration with a residual computation is not a result: either
            # no rule applied (stuck) or the depth bound was hit
            out['status'] = 'depth-exhausted' if depth is not None else 'stuck'
            out['error'] = self.krun.pretty_print(k_cell).replace('\n', ' ')[:300]
        elif status.label.name == 'ok':
            out['status'] = 'ok'
        elif status.label.name == 'panic':
            out['status'] = 'panic'
            out['error'] = tok_str(status.args[0])
        else:
            out['status'] = 'error'
            out['error'] = tok_str(status.args[0])
        memory = {}
        for k, v in map_items(find_cell(cfg, '<mem>')):
            variant, enc = encode_value(v, self.ext)
            memory[tok_str(k)] = {'variant': variant, 'type': type_string(v, self.ext), 'encoded': [str(e) for e in enc]}
        out['memory'] = memory
        out['pis'] = [str(tok_int(x)) for x in list_items(find_cell(cfg, '<pi>'))]
        out['pi_skips'] = [skip(x) for x in list_items(find_cell(cfg, '<skips>'))]
        out['cursors'] = (tok_int(find_cell(cfg, '<pubInIdx>')), tok_int(find_cell(cfg, '<pubOutIdx>')), tok_int(find_cell(cfg, '<privIdx>')))
        out['constraints'] = len(list_items(find_cell(cfg, '<constraints>')))
        out['k_cell'] = self.krun.pretty_print(find_cell(cfg, '<k>')).strip()
        out['needs'] = [need(x) for x in list_items(find_cell(cfg, '<needs>'))]
        verdicts = list_items(find_cell(cfg, '<verdicts>'))
        bad = []
        allv = []
        for v in verdicts:
            assert isinstance(v, KApply) and v.label.name == 'verdict'
            outcome = v.args[1]
            assert isinstance(outcome, KApply)
            gate = ' '.join(self.krun.pretty_print(v.args[0]).replace('\n', ' ').split())
            rec = (outcome.label.name, tok_str(outcome.args[0]) if outcome.args else '', gate)
            allv.append(rec)
            if outcome.label.name != 'holds':
                bad.append((rec[0], rec[1], gate[:120]))
        out['verdicts'] = len(verdicts)
        out['all_verdicts'] = allv
        out['violations'] = bad
        out['outputs'] = [type_string(v, self.ext) + ':' + ','.join(str(e) for e in encode_value(v, self.ext)[1]) for v in list_items(find_cell(cfg, '<outputs>'))]
        return out

    def run_file(self, path: Path, preimage: dict[str, Any], gen: bool = False, checked: bool = False) -> dict[str, Any]:
        return self.run(zkir_kast.load_program(path, ext=self.ext), preimage, gen=gen, checked=checked)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument('file', type=Path)
    ap.add_argument('preimage', type=Path)
    ap.add_argument('--definition', type=Path, default=None)
    ap.add_argument('--ext', action='store_true')
    ap.add_argument('--checked', action='store_true', help='run the static check first (checkedJob)')
    args = ap.parse_args()
    runner = Runner(args.definition, ext=args.ext)
    with open(args.preimage) as f:
        pre = json.load(f)
    print(json.dumps(runner.run_file(args.file, pre, checked=args.checked), indent=1))
    return 0


if __name__ == '__main__':
    sys.exit(main())

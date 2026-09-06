"""ZKIR v3 JSON -> K term (pyk) preprocessor.

Reads a `.zkir` artifact (JSON, version 3.0) and builds the `Program` term of
semantics/zkir-syntax.k as a pyk KInner. Mirrors the serde layer of
`zkir-v3/src/ir.rs` at midnight-ledger 92e8bdd3:

- operands: strings starting with "%" are variables; "0x…"/"-0x…" are
  little-endian hex immediates that must fit the BLS12-381 scalar field
  (`Fr::from_le_bytes` returns None otherwise); anything else is rejected;
- `guard: null` on public_input/private_input is `noGuard()`;
- alignments: `[{"tag":"atom","value":{"tag":"bytes","length":n}} …]`.

Usage:
  zkir_kast.py kore  FILE.zkir            print the Program term as KORE text
  zkir_kast.py kast  FILE.zkir            print the Program term as KAST JSON
  zkir_kast.py check FILE.zkir [--definition DIR]
                                          run ZKIR-CHECK, print wfOk/wfError
  zkir_kast.py contract FILE.zkir [--definition DIR]
                                          run ZKIR-CONTRACT-MAIN, print the
                                          tier-one obligations as JSON
`--ext` accepts the midnight-zkir 2ffe2d1 surface for any command.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any

from pyk.kast.inner import KApply, KInner, KSequence, KSort, KToken
from pyk.kast.prelude.kbool import boolToken
from pyk.kast.prelude.kint import intToken
from pyk.kast.prelude.string import stringToken

BLS_SCALAR_MODULUS = 52435875175126190479447740508185965837690552500527637822603658699938581184513

TYPE_SYMBOLS = {
    'Scalar<BLS12-381>': 'Native',
    'Bytes<32>': 'Bytes32',
    'Point<Jubjub>': 'JubjubPoint',
    'Scalar<Jubjub>': 'JubjubScalar',
    'Point<Secp256k1>': 'Secp256k1Point',
    'Base<Secp256k1>': 'Secp256k1Base',
    'Scalar<Secp256k1>': 'Secp256k1Scalar',
    'Point<Secp256r1>': 'Secp256r1Point',
    'Base<Secp256r1>': 'Secp256r1Base',
    'Scalar<Secp256r1>': 'Secp256r1Scalar',
    'Point<Curve25519>': 'Curve25519Point',
    'Base<Curve25519>': 'Curve25519Base',
    'Scalar<Curve25519>': 'Curve25519Scalar',
}


class ZkirFormatError(ValueError):
    """The JSON does not deserialize as ZKIR 3.0 (mirrors serde errors)."""


# Surface selection: False = midnight-ledger 92e8bdd3 (34 instructions, 13 types);
# True = midnight-zkir 2ffe2d1 (ZKIR-EXT: Bool, Byte, Bytes<n>, nine more instructions,
# reverse_bytes removed).
EXT = False
EXT_TYPES = {'Bool': 'Bool', 'Byte': 'Byte'}
EXT_OPS = {'and', 'or', 'xor', 'concat', 'slice', 'nth', 'reverse', 'load_constant', 'sha512'}


# --- leaf builders -----------------------------------------------------------

def klist(cons: str, nil: str, items: list[KInner]) -> KInner:
    term: KInner = KApply(nil)
    for item in reversed(items):
        term = KApply(cons, [item, term])
    return term


def ir_type(name: Any) -> KInner:
    if isinstance(name, str) and name in TYPE_SYMBOLS:
        return KApply(TYPE_SYMBOLS[name])
    if EXT and isinstance(name, str):
        if name in EXT_TYPES:
            return KApply(EXT_TYPES[name])
        m = re.fullmatch(r'Bytes<(0|[1-9][0-9]*)>', name, flags=re.ASCII)
        if m:
            n = int(m.group(1))
            if 1 <= n <= (1 << 24):
                return KApply('BytesN', [intToken(n)])
    raise ZkirFormatError(f'unknown IR type {name!r}')


HEX = re.compile(r'^[0-9a-fA-F]+$')


def immediate(text: str) -> int:
    """`Operand::deserialize` for hex immediates: little-endian bytes, optional
    leading '-', value must be < r. `const_hex::decode` accepts only an even
    number of hex digits, no whitespace or other characters."""
    negate = text.startswith('-')
    body = text[1:] if negate else text
    if not (body.startswith('0x') or body.startswith('0X')):
        raise ZkirFormatError(f'invalid operand format: {text!r}')
    hex_str = body[2:]
    if not hex_str:
        raise ZkirFormatError("hex immediate must have at least one digit after '0x'")
    if not HEX.match(hex_str) or len(hex_str) % 2:
        raise ZkirFormatError(f'invalid hex immediate {text!r}: odd length or non-hex character')
    raw = bytes.fromhex(hex_str)
    if len(raw) > 32:
        raise ZkirFormatError(f'immediate {text!r} out of range for field element')
    value = int.from_bytes(raw, 'little')
    if value >= BLS_SCALAR_MODULUS:
        raise ZkirFormatError(f'immediate {text!r} out of range for field element')
    return (-value) % BLS_SCALAR_MODULUS if negate else value


def operand(text: Any) -> KInner:
    if not isinstance(text, str):
        raise ZkirFormatError(f'operand must be a string, got {text!r}')
    body = text[1:] if text.startswith('-') else text
    if body.startswith('0x') or body.startswith('0X'):
        return KApply('imm', [intToken(immediate(text))])
    if not text.startswith('%'):
        raise ZkirFormatError(
            f"invalid operand format: {text!r}. Variables must start with '%', immediates must start with '0x'"
        )
    return KApply('var', [stringToken(text)])


def operands(items: Any) -> KInner:
    if not isinstance(items, list):
        raise ZkirFormatError(f'expected a list of operands, got {items!r}')
    return klist('operands', '.operands', [operand(x) for x in items])


def identifier(text: Any) -> KInner:
    if not isinstance(text, str):
        raise ZkirFormatError(f'identifier must be a string, got {text!r}')
    return stringToken(text)


def identifiers(items: Any) -> KInner:
    if not isinstance(items, list):
        raise ZkirFormatError(f'expected a list of identifiers, got {items!r}')
    return klist('ids', '.ids', [identifier(x) for x in items])


def pair(items: Any, what: str) -> tuple[Any, Any]:
    if not isinstance(items, list) or len(items) != 2:
        raise ZkirFormatError(f'{what} must be a pair, got {items!r}')
    return items[0], items[1]


def u32(value: Any, what: str) -> KInner:
    if isinstance(value, bool) or not isinstance(value, int) or value < 0 or value >= 2**32:
        raise ZkirFormatError(f'{what} must be a u32, got {value!r}')
    return intToken(value)


def guard(value: Any) -> KInner:
    if value is None:
        return KApply('noGuard')
    return KApply('guard', [operand(value)])


def alignment(value: Any) -> KInner:
    if not isinstance(value, list):
        raise ZkirFormatError(f'alignment must be a list of segments, got {value!r}')
    segs = []
    for seg in value:
        tag = seg.get('tag') if isinstance(seg, dict) else None
        if tag == 'atom':
            atom = seg.get('value')
            atag = atom.get('tag') if isinstance(atom, dict) else None
            if atag == 'field':
                segs.append(KApply('atom', [KApply('fieldAtom')]))
            elif atag == 'bytes':
                segs.append(KApply('atom', [KApply('bytesAtom', [u32(atom.get('length'), 'bytes length')])]))
            elif atag == 'compress':
                segs.append(KApply('atom', [KApply('compressAtom')]))
            else:
                raise ZkirFormatError(f'unknown alignment atom {atom!r}')
        elif tag == 'option':
            alts = seg.get('value')
            if not isinstance(alts, list):
                raise ZkirFormatError(f'alignment option must carry a list, got {alts!r}')
            segs.append(KApply('option', [klist('alignments', '.alignments', [alignment(a) for a in alts])]))
        else:
            raise ZkirFormatError(f'unknown alignment segment {seg!r}')
    return KApply('alignment', [klist('segments', '.segments', segs)])


# --- instructions ------------------------------------------------------------

def _need(ins: dict, *keys: str) -> None:
    """serde: every listed field is required (Option fields are listed by the
    callers with `_opt`); unknown fields are ignored, as the Rust structs do
    not carry `deny_unknown_fields`."""
    missing = [k for k in keys if k not in ins]
    if missing:
        raise ZkirFormatError(f"{ins.get('op')}: missing field(s) {missing}")


def instruction(ins: Any) -> KInner:
    if not isinstance(ins, dict) or 'op' not in ins:
        raise ZkirFormatError(f'instruction must be an object with "op", got {ins!r}')
    op = ins['op']
    match op:
        case 'encode':
            _need(ins, 'input', 'outputs')
            return KApply('encode', [operand(ins['input']), identifiers(ins['outputs'])])
        case 'assert':
            _need(ins, 'cond')
            return KApply('assert', [operand(ins['cond'])])
        case 'cond_select':
            _need(ins, 'bit', 'a', 'b', 'output')
            return KApply('cond_select', [operand(ins['bit']), operand(ins['a']), operand(ins['b']), identifier(ins['output'])])
        case 'constrain_bits':
            _need(ins, 'val', 'bits')
            return KApply('constrain_bits', [operand(ins['val']), u32(ins['bits'], 'bits')])
        case 'constrain_eq':
            _need(ins, 'a', 'b')
            return KApply('constrain_eq', [operand(ins['a']), operand(ins['b'])])
        case 'constrain_to_boolean':
            _need(ins, 'val')
            return KApply('constrain_to_boolean', [operand(ins['val'])])
        case 'copy':
            _need(ins, 'val', 'output')
            return KApply('copy', [operand(ins['val']), identifier(ins['output'])])
        case 'impact':
            _need(ins, 'guard', 'inputs')
            return KApply('impact', [operand(ins['guard']), operands(ins['inputs'])])
        case 'ec_mul':
            _need(ins, 'a', 'scalar', 'output')
            return KApply('ec_mul', [operand(ins['a']), operand(ins['scalar']), identifier(ins['output'])])
        case 'ec_mul_generator':
            _need(ins, 'scalar', 'output')
            return KApply('ec_mul_generator', [operand(ins['scalar']), identifier(ins['output'])])
        case 'hash_to_curve':
            _need(ins, 'inputs', 'output')
            return KApply('hash_to_curve', [operands(ins['inputs']), identifier(ins['output'])])
        case 'into_coordinates':
            _need(ins, 'point', 'outputs')
            x, y = pair(ins['outputs'], 'into_coordinates outputs')
            return KApply('into_coordinates', [operand(ins['point']), identifier(x), identifier(y)])
        case 'from_coordinates':
            _need(ins, 'inputs', 'output')
            x, y = pair(ins['inputs'], 'from_coordinates inputs')
            return KApply('from_coordinates', [operand(x), operand(y), identifier(ins['output'])])
        case 'into_bytes32':
            _need(ins, 'input', 'output')
            return KApply('into_bytes32', [operand(ins['input']), identifier(ins['output'])])
        case 'from_bytes32':
            _need(ins, 'bytes', 'type', 'output')
            return KApply('from_bytes32', [operand(ins['bytes']), ir_type(ins['type']), identifier(ins['output'])])
        case 'reverse_bytes':
            if EXT:
                raise ZkirFormatError("unknown instruction op 'reverse_bytes' (removed at midnight-zkir 2ffe2d1; use reverse)")
            _need(ins, 'bytes', 'output')
            return KApply('reverse_bytes', [operand(ins['bytes']), identifier(ins['output'])])
        case 'and' | 'or' | 'xor' if EXT:
            _need(ins, 'inputs', 'output')
            return KApply(op, [operands(ins['inputs']), identifier(ins['output'])])
        case 'concat' if EXT:
            _need(ins, 'inputs', 'output')
            return KApply('concat', [operands(ins['inputs']), identifier(ins['output'])])
        case 'slice' if EXT:
            _need(ins, 'bytes', 'start', 'len', 'output')
            return KApply('slice', [operand(ins['bytes']), u32(ins['start'], 'start'), u32(ins['len'], 'len'), identifier(ins['output'])])
        case 'nth' if EXT:
            _need(ins, 'bytes', 'index', 'output')
            return KApply('nth', [operand(ins['bytes']), u32(ins['index'], 'index'), identifier(ins['output'])])
        case 'reverse' if EXT:
            _need(ins, 'bytes', 'output')
            return KApply('reverse', [operand(ins['bytes']), identifier(ins['output'])])
        case 'load_constant' if EXT:
            _need(ins, 'type', 'encoding', 'output')
            if not isinstance(ins['encoding'], list):
                raise ZkirFormatError('load_constant encoding must be a list of hex immediates')
            enc = klist('operands', '.operands', [KApply('imm', [intToken(immediate(e))]) for e in ins['encoding']])
            return KApply('load_constant', [ir_type(ins['type']), enc, identifier(ins['output'])])
        case 'sha512' if EXT:
            _need(ins, 'alignment', 'inputs', 'output')
            return KApply('sha512', [alignment(ins['alignment']), operands(ins['inputs']), identifier(ins['output'])])
        case 'bytes32_into_low_high':
            _need(ins, 'bytes', 'outputs')
            lo, hi = pair(ins['outputs'], 'bytes32_into_low_high outputs')
            return KApply('bytes32_into_low_high', [operand(ins['bytes']), identifier(lo), identifier(hi)])
        case 'bytes32_from_low_high':
            _need(ins, 'inputs', 'output')
            lo, hi = pair(ins['inputs'], 'bytes32_from_low_high inputs')
            return KApply('bytes32_from_low_high', [operand(lo), operand(hi), identifier(ins['output'])])
        case 'div_mod_power_of_two':
            _need(ins, 'val', 'bits', 'outputs')
            return KApply('div_mod_power_of_two', [operand(ins['val']), u32(ins['bits'], 'bits'), identifiers(ins['outputs'])])
        case 'reconstitute_field':
            _need(ins, 'divisor', 'modulus', 'bits', 'output')
            return KApply('reconstitute_field', [operand(ins['divisor']), operand(ins['modulus']), u32(ins['bits'], 'bits'), identifier(ins['output'])])
        case 'transient_hash':
            _need(ins, 'inputs', 'output')
            return KApply('transient_hash', [operands(ins['inputs']), identifier(ins['output'])])
        case 'persistent_hash':
            _need(ins, 'alignment', 'inputs', 'output')
            return KApply('persistent_hash', [alignment(ins['alignment']), operands(ins['inputs']), identifier(ins['output'])])
        case 'keccak256':
            _need(ins, 'alignment', 'inputs', 'output')
            return KApply('keccak256', [alignment(ins['alignment']), operands(ins['inputs']), identifier(ins['output'])])
        case 'test_eq':
            _need(ins, 'a', 'b', 'output')
            return KApply('test_eq', [operand(ins['a']), operand(ins['b']), identifier(ins['output'])])
        case 'add' | 'mul':
            _need(ins, 'a', 'b', 'output')
            return KApply(op, [operand(ins['a']), operand(ins['b']), identifier(ins['output'])])
        case 'neg' | 'inv' | 'not':
            _need(ins, 'a', 'output')
            return KApply(op, [operand(ins['a']), identifier(ins['output'])])
        case 'less_than':
            _need(ins, 'a', 'b', 'bits', 'output')
            return KApply('less_than', [operand(ins['a']), operand(ins['b']), u32(ins['bits'], 'bits'), identifier(ins['output'])])
        case 'jubjub_scalar_from_native':
            _need(ins, 'native', 'output')
            return KApply('jubjub_scalar_from_native', [operand(ins['native']), identifier(ins['output'])])
        case 'public_input' | 'private_input':
            _need(ins, 'type', 'output')   # guard is Option<Operand>: absent == null
            return KApply(op, [guard(ins.get('guard')), ir_type(ins['type']), identifier(ins['output'])])
        case 'output':
            _need(ins, 'vals')
            return KApply('output', [operands(ins['vals'])])
        case _:
            raise ZkirFormatError(f'unknown instruction op {op!r}')


# --- program -------------------------------------------------------------------

def program(doc: Any) -> KInner:
    """`IrSource::load`: version {major: 3, minor: 0..=0}, then serde fields."""
    if not isinstance(doc, dict):
        raise ZkirFormatError('expected a JSON object')
    version = doc.get('version')
    if not isinstance(version, dict) or 'major' not in version or 'minor' not in version:
        raise ZkirFormatError('expected a version entry')
    for part in ('major', 'minor'):   # serde u8
        v = version[part]
        if isinstance(v, bool) or not isinstance(v, int) or not (0 <= v <= 255):
            raise ZkirFormatError(f'version.{part}: expected u8, got {v!r}')
    if version['major'] != 3 or not (0 <= version['minor'] <= 0):
        raise ZkirFormatError(f"unhandled version: {version['major']}.{version['minor']}")
    for key in ('inputs', 'outputs', 'do_communications_commitment', 'instructions'):
        if key not in doc:
            raise ZkirFormatError(f'missing field {key!r}')
    for key in ('inputs', 'outputs', 'instructions'):   # serde Vec
        if not isinstance(doc[key], list):
            raise ZkirFormatError(f'{key}: expected a sequence')
    inputs = []
    for entry in doc['inputs']:
        if not isinstance(entry, dict) or 'name' not in entry or 'type' not in entry:
            raise ZkirFormatError(f'input must have name and type, got {entry!r}')
        inputs.append(KApply('typedId', [identifier(entry['name']), ir_type(entry['type'])]))
    outputs = [ir_type(t) for t in doc['outputs']]
    if not isinstance(doc['do_communications_commitment'], bool):
        raise ZkirFormatError('do_communications_commitment must be a boolean')
    instrs = [instruction(i) for i in doc['instructions']]
    return KApply('program', [
        intToken(version['minor']),
        klist('typedIds', '.typedIds', inputs),
        klist('irTypes', '.irTypes', outputs),
        boolToken(doc['do_communications_commitment']),
        klist('instrs', '.instrs', instrs),
    ])


def load_program(path: Path, ext: bool | None = None) -> KInner:
    global EXT
    if ext is not None:
        EXT = ext
    with open(path) as f:
        return program(json.load(f))


# --- CLI ------------------------------------------------------------------------

def default_definition(name: str = 'zkir-check') -> Path:
    return Path(__file__).resolve().parent.parent / 'semantics' / f'{name}-kompiled'


def wf_result(pretty: str) -> str:
    if 'wfOk ( )' in pretty:
        return 'wfOk'
    start = pretty.find('wfError (')
    if start >= 0:
        end = pretty.find(')', start)
        return pretty[start:end + 1].replace('\n', ' ')
    return 'unexpected: ' + pretty.replace('\n', ' ')[:200]


# --- the target contract (ZKIR-CONTRACT, semantics/zkir-contract.k) --------------

def k_string(token: KInner) -> str:
    """The Python value of a K String token (quoted, C-style escapes)."""
    if not isinstance(token, KToken):
        raise ValueError(f'expected a String token, got {token}')
    text = token.token
    try:
        return json.loads(text)
    except json.JSONDecodeError:
        return text[1:-1] if len(text) >= 2 and text[0] == text[-1] == '"' else text


def find_label(term: KInner, label: str) -> KInner | None:
    """First subterm (pre-order) whose label is `label`; descends through
    cells, applications and the `~>` sequence of the <k> cell."""
    if isinstance(term, KApply):
        if term.label.name == label:
            return term
        children: tuple[KInner, ...] = term.args
    elif isinstance(term, KSequence):
        children = term.items
    else:
        return None
    for child in children:
        found = find_label(child, label)
        if found is not None:
            return found
    return None


def obligations(term: KInner) -> list[dict[str, Any]]:
    """Walk the `obligations` list of a ZKIR-CONTRACT-MAIN result into dicts
    {name, status, detail}: status is met, failed, notApplicable or info;
    detail is the message of failed/info and None otherwise."""
    out: list[dict[str, Any]] = []
    node = find_label(term, 'obligations')
    if node is None:
        if find_label(term, '.obligations') is not None:
            return out
        raise ValueError('no obligations list in the result')
    while isinstance(node, KApply) and node.label.name == 'obligations':
        ob, node = node.args
        if not (isinstance(ob, KApply) and ob.label.name == 'obligation'):
            raise ValueError(f'unexpected list element {ob}')
        name_tok, status = ob.args
        if not isinstance(status, KApply):
            raise ValueError(f'unexpected status {status}')
        kind = status.label.name
        detail = k_string(status.args[0]) if status.args else None
        out.append({'name': k_string(name_tok), 'status': kind, 'detail': detail})
    return out


def run_contract(krun: 'KRun', term: KInner) -> list[dict[str, Any]]:
    from pyk.kore.parser import KoreParser
    kore = krun.kast_to_kore(term, KSort('Program'))
    res = krun.run_process(kore)
    if res.returncode != 0:
        raise RuntimeError(res.stderr.strip())
    return obligations(krun.kore_to_kast(KoreParser(res.stdout).pattern()))


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('command', choices=['kore', 'kast', 'check', 'contract'])
    ap.add_argument('file', type=Path)
    ap.add_argument('--definition', type=Path, default=None)
    ap.add_argument('--ext', action='store_true', help='accept the midnight-zkir 2ffe2d1 surface (ZKIR-EXT)')
    args = ap.parse_args(argv)
    try:
        term = load_program(args.file, ext=args.ext)
    except ZkirFormatError as e:
        print(f'format error: {e}', file=sys.stderr)
        return 2
    if args.command == 'kast':
        print(json.dumps(term.to_dict()))
        return 0
    from pyk.ktool.krun import KRun
    if args.command == 'contract':
        krun = KRun(args.definition or default_definition('zkir-contract-main'))
        try:
            obs = run_contract(krun, term)
        except RuntimeError as e:
            print(f'krun failed: {e}', file=sys.stderr)
            return 1
        print(json.dumps({
            'program': str(args.file),
            'surface': 'extension' if args.ext else 'base',
            'obligations': obs,
        }, indent=2))
        return 0
    krun = KRun(args.definition or default_definition())
    kore = krun.kast_to_kore(term, KSort('Program'))
    if args.command == 'kore':
        print(kore.text)
        return 0
    from pyk.kore.parser import KoreParser
    res = krun.run_process(kore)
    if res.returncode not in (0,):
        print(res.stderr, file=sys.stderr)
        return res.returncode
    out = krun.pretty_print(krun.kore_to_kast(KoreParser(res.stdout).pattern()))
    print(wf_result(out))
    return 0


if __name__ == '__main__':
    sys.exit(main())

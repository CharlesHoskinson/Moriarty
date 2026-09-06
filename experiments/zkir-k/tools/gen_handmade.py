"""Generate handmade positive programs (corpus/handmade/) with matching test
preimages (manifest.json), so that every value-level instruction has at least
one successful differential run on every type it supports.

Usage: python gen_handmade.py   (idempotent)
"""
import json
from pathlib import Path

import zkir_values as zv

OUT = Path(__file__).resolve().parent.parent / 'corpus' / 'handmade'
NATIVE = 'Scalar<BLS12-381>'
programs = {}


def prog(name, inputs, instructions, raw, outputs=(), comm=False):
    programs[name] = ({'version': {'major': 3, 'minor': 0},
                       'inputs': [{'name': n, 'type': t} for n, t in inputs],
                       'outputs': list(outputs), 'do_communications_commitment': comm,
                       'instructions': instructions},
                      [str(x) for x in raw])


# --- curves: from/into coordinates, ec_mul, ec_mul_generator, add, neg, test_eq, constrain_eq, encode
CURVES = {
    'jubjub': ('Point<Jubjub>', 'Scalar<Jubjub>', NATIVE, zv.JUBJUB_G, lambda k: zv.ed_mul(zv.JUBJUB_G, k, zv.R, zv.JUBJUB_D), [], True),
    'secp256k1': ('Point<Secp256k1>', 'Scalar<Secp256k1>', 'Base<Secp256k1>', zv.K256_G, lambda k: zv.w_mul(zv.K256_G, k, zv.K256P, 0), [], True),
    'secp256r1': ('Point<Secp256r1>', 'Scalar<Secp256r1>', 'Base<Secp256r1>', zv.P256_G, lambda k: zv.w_mul(zv.P256_G, k, zv.P256P, zv.P256P - 3), [], False),
    'curve25519': ('Point<Curve25519>', 'Scalar<Curve25519>', 'Base<Curve25519>', zv.C25519_G, lambda k: zv.ed_mul(zv.C25519_G, k, zv.C25519P, zv.C25519_D), [], False),
}
for name, (pt, sc, base, g, mul, _, has_gen) in CURVES.items():
    p = mul(7)
    q = mul(11)
    if name == 'jubjub':
        raw = [p[0], p[1], q[0], q[1], 5, p[0], p[1]]
        xs = [('%p', pt), ('%q', pt), ('%s', sc), ('%x', NATIVE), ('%y', NATIVE)]
    elif name == 'curve25519':
        raw = zv.enc_foreign(p[0], zv.C25519P, 64, 4) + zv.enc_foreign(p[1], zv.C25519P, 64, 4) + zv.enc_foreign(q[0], zv.C25519P, 64, 4) + zv.enc_foreign(q[1], zv.C25519P, 64, 4) + zv.enc_foreign(5, zv.C25519L, 51, 5) + zv.enc_foreign(p[0], zv.C25519P, 64, 4) + zv.enc_foreign(p[1], zv.C25519P, 64, 4)
        xs = [('%p', pt), ('%q', pt), ('%s', sc), ('%x', base), ('%y', base)]
    else:
        mod, order = (zv.K256P, zv.K256N) if name == 'secp256k1' else (zv.P256P, zv.P256N)
        raw = zv.enc_wpoint(p, mod) + zv.enc_wpoint(q, mod) + zv.enc_foreign(5, order, 64, 4) + zv.enc_foreign(p[0], mod, 64, 4) + zv.enc_foreign(p[1], mod, 64, 4)
        xs = [('%p', pt), ('%q', pt), ('%s', sc), ('%x', base), ('%y', base)]
    ins = [
        {'op': 'add', 'a': '%p', 'b': '%q', 'output': '%sum'},
        {'op': 'neg', 'a': '%p', 'output': '%negp'},
        {'op': 'add', 'a': '%sum', 'b': '%negp', 'output': '%q2'},
        {'op': 'test_eq', 'a': '%q2', 'b': '%q', 'output': '%eq'},
        {'op': 'assert', 'cond': '%eq'},
        {'op': 'constrain_eq', 'a': '%q2', 'b': '%q'},
        {'op': 'ec_mul', 'a': '%p', 'scalar': '%s', 'output': '%ps'},
        {'op': 'into_coordinates', 'point': '%p', 'outputs': ['%px', '%py']},
        {'op': 'from_coordinates', 'inputs': ['%x', '%y'], 'output': '%p2'},
        {'op': 'constrain_eq', 'a': '%p2', 'b': '%p'},
        {'op': 'encode', 'input': '%ps', 'outputs': [f'%e{i}' for i in range(zv.ENCODED_LEN[pt])]},
        {'op': 'cond_select', 'bit': '%eq', 'a': '%p', 'b': '%q', 'output': '%sel'},
    ]
    if has_gen:
        ins.append({'op': 'ec_mul_generator', 'scalar': '%s', 'output': '%gs'})
    if name != 'jubjub':
        ins += [
            {'op': 'add', 'a': '%x', 'b': '%y', 'output': '%bx'},
            {'op': 'mul', 'a': '%x', 'b': '%y', 'output': '%bm'},
            {'op': 'neg', 'a': '%x', 'output': '%bn'},
            {'op': 'inv', 'a': '%x', 'output': '%bi'},
            {'op': 'into_bytes32', 'input': '%x', 'output': '%xb'},
            {'op': 'from_bytes32', 'bytes': '%xb', 'type': base, 'output': '%x2'},
            {'op': 'constrain_eq', 'a': '%x2', 'b': '%x'},
            {'op': 'encode', 'input': '%x', 'outputs': ['%ex0', '%ex1']},
            {'op': 'test_eq', 'a': '%x', 'b': '%y', 'output': '%bxeq'},
        ]
    prog(f'curve_{name}', xs, ins, raw)

# --- native and bytes: arithmetic, bits, bytes32 conversions
b = bytes(range(1, 33))
lo, hi = int.from_bytes(b[:31], 'little'), b[31]
prog('native_bytes', [('%a', NATIVE), ('%b', NATIVE), ('%bs', 'Bytes<32>'), ('%c', 'Scalar<Jubjub>')], [
    {'op': 'add', 'a': '%a', 'b': '%b', 'output': '%s'},
    {'op': 'mul', 'a': '%a', 'b': '%b', 'output': '%m'},
    {'op': 'neg', 'a': '%a', 'output': '%n'},
    {'op': 'inv', 'a': '%a', 'output': '%i'},
    {'op': 'mul', 'a': '%a', 'b': '%i', 'output': '%one'},
    {'op': 'constrain_eq', 'a': '%one', 'b': '0x01'},
    {'op': 'not', 'a': '0x00', 'output': '%t'},
    {'op': 'assert', 'cond': '%t'},
    {'op': 'constrain_to_boolean', 'val': '%t'},
    {'op': 'constrain_bits', 'val': '%a', 'bits': 16},
    {'op': 'less_than', 'a': '%a', 'b': '%b', 'bits': 16, 'output': '%lt'},
    {'op': 'div_mod_power_of_two', 'val': '%b', 'bits': 4, 'outputs': ['%q', '%r']},
    {'op': 'reconstitute_field', 'divisor': '%q', 'modulus': '%r', 'bits': 4, 'output': '%b2'},
    {'op': 'constrain_eq', 'a': '%b2', 'b': '%b'},
    {'op': 'copy', 'val': '%a', 'output': '%a2'},
    {'op': 'into_bytes32', 'input': '%a', 'output': '%ab'},
    {'op': 'from_bytes32', 'bytes': '%ab', 'type': NATIVE, 'output': '%a3'},
    {'op': 'constrain_eq', 'a': '%a3', 'b': '%a'},
    {'op': 'reverse_bytes', 'bytes': '%bs', 'output': '%rev'},
    {'op': 'reverse_bytes', 'bytes': '%rev', 'output': '%rev2'},
    {'op': 'constrain_eq', 'a': '%rev2', 'b': '%bs'},
    {'op': 'test_eq', 'a': '%rev', 'b': '%bs', 'output': '%beq'},
    {'op': 'bytes32_into_low_high', 'bytes': '%bs', 'outputs': ['%lo', '%hi']},
    {'op': 'bytes32_from_low_high', 'inputs': ['%lo', '%hi'], 'output': '%bs2'},
    {'op': 'constrain_eq', 'a': '%bs2', 'b': '%bs'},
    {'op': 'encode', 'input': '%bs', 'outputs': ['%e0', '%e1']},
    {'op': 'jubjub_scalar_from_native', 'native': '%a', 'output': '%js'},
    {'op': 'ec_mul_generator', 'scalar': '%c', 'output': '%gc'},
    {'op': 'encode', 'input': '%c', 'outputs': ['%ec']},
    {'op': 'cond_select', 'bit': '%lt', 'a': '%a', 'b': '%b', 'output': '%sel'},
    {'op': 'transient_hash', 'inputs': ['%a', '%b'], 'output': '%h'},
    {'op': 'hash_to_curve', 'inputs': ['%h'], 'output': '%hp'},
    {'op': 'output', 'vals': ['%s', '%hp']},
], [1234, 60000, lo, hi, 987654321], outputs=[NATIVE, 'Point<Jubjub>'])

# --- transcripts, impacts, guards, commitment
prog('transcripts', [('%a', NATIVE)], [
    {'op': 'public_input', 'guard': None, 'type': NATIVE, 'output': '%p1'},
    {'op': 'private_input', 'guard': None, 'type': 'Bytes<32>', 'output': '%s1'},
    {'op': 'test_eq', 'a': '%a', 'b': '0x01', 'output': '%g'},
    {'op': 'public_input', 'guard': '%g', 'type': 'Point<Jubjub>', 'output': '%pp'},
    {'op': 'private_input', 'guard': '%g', 'type': 'Scalar<Secp256k1>', 'output': '%ps'},
    {'op': 'impact', 'guard': '0x01', 'inputs': ['%a', '%p1']},
    {'op': 'impact', 'guard': '%g', 'inputs': ['0x07', '0x08', '0x09']},
    {'op': 'not', 'a': '%g', 'output': '%ng'},
    {'op': 'impact', 'guard': '%ng', 'inputs': ['0x0a']},
    {'op': 'encode', 'input': '%pp', 'outputs': ['%x', '%y']},
    {'op': 'output', 'vals': ['%p1', '%s1']},
], [1], outputs=[NATIVE, 'Bytes<32>'], comm=True)
prog('transcripts_guard_off', programs['transcripts'][0]['inputs'] and [('%a', NATIVE)], programs['transcripts'][0]['instructions'], [0], outputs=[NATIVE, 'Bytes<32>'], comm=True)

manifest = {'note': 'handmade positive programs generated by tools/gen_handmade.py', 'programs': []}
for name, (doc, raw) in programs.items():
    (OUT / f'{name}.zkir').write_text(json.dumps(doc, indent=1) + '\n')
    manifest['programs'].append({'file': f'{name}.zkir', 'ops': sorted({i['op'] for i in doc['instructions']}), 'test_preimage': {'inputs': raw}})
for extra in ['transient_hash.zkir', 'std_hashes.zkir']:
    manifest['programs'].append({'file': extra, 'test_preimage': {'inputs': ['5', '123456789', '987654321987654321']}})
(OUT / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
print('wrote', len(programs), 'programs')

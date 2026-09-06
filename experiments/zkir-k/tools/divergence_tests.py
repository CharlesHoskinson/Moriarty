"""Milestone 5: the divergence findings as executable tests (oracle 2).

Each case is a small ZKIR v3 program written here, a preimage, and the expected
pattern: the off-circuit status on both sides (K witness and Rust
`preprocess`), and the outcome the constraint checker must assign to one
specific gate. Findings refer to arc-zkir docs/zkir-v3-divergence-review.md at
midnight-ledger 92e8bdd3 (wiki/zkir/zkir-v3-divergence-review.md); "K1" is a
candidate found while writing the K semantics.

Usage: uv run --group zkir-k python experiments/zkir-k/tools/divergence_tests.py
Writes the programs to corpus/divergence/ and exits 0 iff every case matches.
"""
from __future__ import annotations

import json
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import zkir_kast  # noqa: E402
import zkir_values as zv  # noqa: E402
from zkir_run import Runner  # noqa: E402

HERE = Path(__file__).resolve().parent
OUT = HERE.parent / 'corpus' / 'divergence'
ORACLE = Path.home() / 'Moriarty/repos/_build/ledger-92e8bdd3/target/release/zkir-oracle'

NATIVE = 'Scalar<BLS12-381>'


def program(inputs, instructions, outputs=(), comm=False):
    return {'version': {'major': 3, 'minor': 0},
            'inputs': [{'name': n, 'type': t} for n, t in inputs],
            'outputs': list(outputs), 'do_communications_commitment': comm,
            'instructions': instructions}


def hexle(n: int) -> str:
    b = n.to_bytes((n.bit_length() + 7) // 8 or 1, 'little')
    return '0x' + b.hex()


# --- cases -----------------------------------------------------------------------------
# (name, finding, program, raw inputs, expected K status, expected Rust status,
#  gate substring, expected outcome kind, note)
G8 = zv.JUBJUB_G
CASES = []

CASES.append(('f02_assert_non_boolean', 'Finding 2',
    program([('%c', NATIVE)], [{'op': 'assert', 'cond': '%c'}]), [2],
    'error', 'error', 'assert', 'holds',
    'off-circuit demands cond == 1, the circuit only cond != 0'))

CASES.append(('f03_guard_uncoupled', 'Finding 3',
    program([('%g', NATIVE)], [
        {'op': 'public_input', 'guard': '%g', 'type': NATIVE, 'output': '%x'},
        {'op': 'add', 'a': '%x', 'b': '0x01', 'output': '%y'}]), [0],
    'ok', 'ok', 'public_input', 'holds',
    'an inactive guard yields the default value off-circuit; the gate never mentions the guard'))

CASES.append(('f04_less_than_odd_bits', 'Finding 4',
    program([('%a', NATIVE), ('%b', NATIVE)], [
        {'op': 'less_than', 'a': '%a', 'b': '%b', 'bits': 3, 'output': '%lt'}]), [8, 9],
    'error', 'error', 'less_than', 'unknown',
    'a = 8 exceeds 3 bits off-circuit; the chip would accept the padded 4-bit bound (output absent, so unknown)'))

EXTRA_PRE = {}
NONCANON = [((zv.K256P + 5 - 1) & ((1 << 192) - 1)), ((zv.K256P + 5 - 1) >> 192)]
CASES.append(('f05_noncanonical_foreign_limbs', 'Finding 5',
    program([('%x', 'Base<Secp256k1>')], [
        {'op': 'add', 'a': '%x', 'b': '%x', 'output': '%y'}, {'op': 'output', 'vals': ['%y']}], outputs=['Base<Secp256k1>'], comm=True),
    NONCANON,
    'ok', 'ok', 'commGate', 'violated',
    'limbs encoding p + 5 decode (reduced) as 5 off-circuit and the raw-stream commitment is accepted; in circuit the commitment is over the canonical re-encoding, so it differs'))

CASES.append(('f06_cond_select_bytes32', 'Finding 6',
    program([('%bit', NATIVE), ('%a', 'Bytes<32>'), ('%b', 'Bytes<32>')], [
        {'op': 'cond_select', 'bit': '%bit', 'a': '%a', 'b': '%b', 'output': '%o'}]),
    [1, 12345, 7, 999, 0],
    'ok', 'ok', 'cond_select', 'synthErr',
    'cond_select on Bytes32 is accepted off-circuit but has no in-circuit arm'))

CASES.append(('f07_constrain_eq_jubjub_scalar', 'Finding 7',
    program([('%s', 'Scalar<Jubjub>'), ('%t', 'Scalar<Jubjub>')], [
        {'op': 'constrain_eq', 'a': '%s', 'b': '%t'}]), [42, 42],
    'ok', 'ok', 'constrain_eq', 'synthErr',
    'constrain_eq on JubjubScalar is accepted off-circuit but has no in-circuit arm'))

CASES.append(('f08_bytes32_from_low_high_foreign_high', 'Finding 8',
    program([('%lo', NATIVE), ('%hi', 'Base<Secp256k1>')], [
        {'op': 'bytes32_from_low_high', 'inputs': ['%lo', '%hi'], 'output': '%b'}]),
    [12345] + zv.enc_foreign(200, zv.K256P, 64, 4),
    'ok', 'ok', 'bytes32_from_low_high', 'synthErr',
    'a foreign high operand is accepted off-circuit, the circuit requires Native'))

CASES.append(('f10_reconstitute_bits_256', 'Finding 10',
    program([('%d', NATIVE), ('%m', NATIVE)], [
        {'op': 'reconstitute_field', 'divisor': '%d', 'modulus': '%m', 'bits': 256, 'output': '%o'}]), [1, 1],
    'wfError', 'error', 'reconstitute_field', 'n/a',
    'bits >= 256 is rejected statically (Rust off-circuit: Excessive bit count; keygen would underflow)'))

# a Curve25519 point on the curve but outside the prime-order subgroup: (x, -y) of a torsion
# point... simplest: the order-2 point (0, -1)
CASES.append(('f11_curve25519_torsion_point', 'Finding 11',
    program([('%x', 'Base<Curve25519>'), ('%y', 'Base<Curve25519>')], [
        {'op': 'from_coordinates', 'inputs': ['%x', '%y'], 'output': '%p'}]),
    zv.enc_foreign(0, zv.C25519P, 64, 4) + zv.enc_foreign(zv.C25519P - 1, zv.C25519P, 64, 4),
    'error', 'error', 'from_coordinates', 'violated',
    '(0, -1) has order 2: on the curve, not in the subgroup; off-circuit both sides error, in circuit the cofactor-cleared point is unsatisfiable (the crate hint would panic)'))

CASES.append(('f12_ec_mul_generator_p256', 'Finding 12',
    program([('%s', 'Scalar<Secp256r1>')], [
        {'op': 'ec_mul_generator', 'scalar': '%s', 'output': '%p'}]),
    zv.enc_foreign(7, zv.P256N, 64, 4),
    'error', 'error', 'ec_mul_generator', 'synthErr',
    'no generator for secp256r1 scalars on either side'))

CASES.append(('f13_chip_gating_from_bytes32', 'Finding 13',
    program([('%b', 'Bytes<32>')], [
        {'op': 'from_bytes32', 'bytes': '%b', 'type': 'Base<Secp256k1>', 'output': '%x'},
        {'op': 'add', 'a': '%x', 'b': '%x', 'output': '%y'}]), [12345, 0],
    'ok', 'ok', 'from_bytes32', 'synthErr',
    'the secp256k1 chip is never initialised (used_chips ignores from_bytes32)'))

wrong_x = (G8[0] + 2) % zv.R
CASES.append(('k01_jubjub_from_coordinates_parity_only', 'K1 (new)',
    program([('%x', NATIVE), ('%y', NATIVE), ('%enable', 'Point<Jubjub>')], [
        {'op': 'from_coordinates', 'inputs': ['%x', '%y'], 'output': '%p'},
        {'op': 'into_coordinates', 'point': '%p', 'outputs': ['%px', '%py']}]), [wrong_x, G8[1], G8[0], G8[1]],
    'ok', 'ok', 'from_coordinates', 'violated',
    'off-circuit only the parity of x is used (decompression), so a wrong x yields the real point; in circuit (x, y) must lie on the curve (a Jubjub input enables the chip)'))

CASES.append(('k01b_jubjub_from_coordinates_no_chip', 'K1 companion',
    program([('%x', NATIVE), ('%y', NATIVE)], [
        {'op': 'from_coordinates', 'inputs': ['%x', '%y'], 'output': '%p'}]), [G8[0], G8[1]],
    'ok', 'ok', 'from_coordinates', 'synthErr',
    'native from_coordinates uses the Jubjub chip, which used_chips never enables for it: keygen would panic'))

CASES.append(('k03_bytes32_input_assertion_panics', 'K3 (new)',
    program([('%b', 'Bytes<32>')], [{'op': 'reverse_bytes', 'bytes': '%b', 'output': '%r'}]), [1 << 248, 0],
    'panic', 'panic', 'reverse_bytes', 'unknown',
    'a Bytes32 raw input whose low element uses byte 31 trips assert_eq! in the decoder: the crate panics, K reports a panic status'))

CASES.append(('k04_jubjub_scalar_from_native_chip', 'K4 (new)',
    program([('%a', NATIVE)], [
        {'op': 'jubjub_scalar_from_native', 'native': '%a', 'output': '%s'},
        {'op': 'ec_mul_generator', 'scalar': '%s', 'output': '%p'}]), [7],
    'ok', 'ok', 'jubjub_scalar_from_native', 'synthErr',
    'jubjub_scalar_from_native mints a JubjubScalar and its circuit arm calls std.jubjub(); used_chips does not enable the chip (finding 13 says from_bytes32 is the only such entry)'))

CASES.append(('k05_less_than_253_bits_keygen', 'K5 (new)',
    program([('%a', NATIVE), ('%b', NATIVE)], [
        {'op': 'less_than', 'a': '%a', 'b': '%b', 'bits': 253, 'output': '%lt'}]), [1, 2],
    'ok', 'ok', 'less_than', 'synthErr',
    'bits 253 pads to 254 which exceeds MAX_BOUND_IN_BITS = 253 in bounded_of_element: preprocess accepts, keygen panics'))

CASES.append(('k06_empty_impact_guard', 'R3-2',
    program([('%g', NATIVE)], [{'op': 'impact', 'guard': '%g', 'inputs': []}]), [2],
    'error', 'error', 'guardGate', 'violated',
    'an empty impact still converts its guard to a bit in circuit, so a non-boolean guard is unsatisfiable there too'))

CASES.append(('k07_alignment_option_offcircuit', 'astra-R2 #4',
    program([('%x', NATIVE)], [
        {'op': 'persistent_hash', 'alignment': [{'tag': 'option', 'value': [[{'tag': 'atom', 'value': {'tag': 'field'}}], [{'tag': 'atom', 'value': {'tag': 'bytes', 'length': 1}}]]}], 'inputs': ['0x01', '%x'], 'output': '%h'}]), [5],
    'ok', 'ok', 'persistent_hash', 'synthErr',
    'preprocess parses option alignments (selector 1 chooses the byte alternative); the circuit rejects them as not implemented'))

CASES.append(('k02_transcript_too_short_panics', 'K2 (new)',
    program([('%a', NATIVE)], [
        {'op': 'private_input', 'guard': None, 'type': NATIVE, 'output': '%x'},
        {'op': 'add', 'a': '%a', 'b': '%x', 'output': '%y'}]), [1],
    'panic', 'panic', 'private_input', 'unknown',
    'an unguarded private_input with an empty private transcript: the crate indexes the slice and panics (index out of range) instead of returning an error; K reports the same panic'))

CASES.append(('f01_reconstitute_overflow', 'Finding 1 (retired)',
    program([('%d', NATIVE), ('%m', NATIVE)], [
        {'op': 'reconstitute_field', 'divisor': '%d', 'modulus': '%m', 'bits': 8, 'output': '%o'}]),
    [(zv.R - 1) >> 8, 255],
    'error', 'error', 'reconstitute_field', 'unknown',
    'divisor * 2^8 + modulus >= r: off-circuit overflow error; in circuit the sum would wrap mod r'))


def oracle(path: Path, pre: dict) -> dict:
    with tempfile.NamedTemporaryFile('w', suffix='.json') as f:
        json.dump(pre, f)
        f.flush()
        res = subprocess.run([str(ORACLE), str(path), f.name], capture_output=True, text=True)
    if res.returncode == 101:
        return {'status': 'panic', 'error': res.stderr.strip()[-300:]}
    if res.returncode != 0:
        return {'status': 'load-error', 'error': res.stderr.strip()[-300:]}
    return json.loads(res.stdout)


def main() -> int:
    OUT.mkdir(exist_ok=True)
    runner = Runner()
    failures = 0
    for name, finding, prog, raw, exp_k, exp_r, gate_sub, exp_outcome, note in CASES:
        path = OUT / f'{name}.zkir'
        path.write_text(json.dumps(prog, indent=1) + '\n')
        pre = {'inputs': [str(x) for x in raw], 'binding_input': '42'}
        if pre_extra := dict(EXTRA_PRE.get(name, {})):
            pre.update(pre_extra)
        term = zkir_kast.load_program(path)
        if prog['do_communications_commitment']:
            # the raw-stream commitment the crate accepts, taken from a K generation run
            pre['communications_commitment'] = ['0', '7']
            gen = runner.run(term, pre, gen=True)
            comm = [x for kind, x in gen['needs'] if kind == 'comm']
            if comm:
                pre['communications_commitment'] = [str(comm[0]), '7']
        r = oracle(path, pre)
        # the checked entry point: a well-formedness error ends the run before anything executes
        k = runner.run(term, pre, checked=True)
        if k['status'] == 'error' and k.get('error', '').startswith('well-formedness'):
            k_status = 'wfError'
            outcome = 'n/a'
            detail = k['error']
        else:
            k_status = k['status']
            norm = lambda t: t.replace('_', '').replace(' ', '').lower()
            # the target gate must have been emitted and evaluated; select it exactly
            targets = [v for v in k['all_verdicts'] if norm(v[2]).startswith('gate(' + norm(gate_sub) + '(') or norm(v[2]).startswith(norm(gate_sub) + '(')]
            if not targets:
                outcome = 'no-such-gate'
                detail = 'gate not emitted: ' + '; '.join(v[2][:30] for v in k['all_verdicts'][:4])
            else:
                outcome = targets[0][0]
                detail = f'{targets[0][0]}: {targets[0][1]}' if targets[0][1] else targets[0][0]
            if k['status'] != 'ok' and not k.get('error', '').startswith('well'):
                detail = detail + ' | ' + k.get('error', '')[:70]
        ok = (k_status == exp_k) and (r['status'] == exp_r) and (outcome == exp_outcome) and (k_status != 'stuck')
        failures += not ok
        print(f"{'PASS' if ok else 'FAIL'}  {name:42} {finding:20} K={k_status:8} Rust={r['status']:6} gate={gate_sub}:{outcome}")
        if k_status == 'ok' and r['status'] == 'ok' and 'memory' in r:
            km = {n: (v['type'], v['encoded']) for n, v in k['memory'].items()}
            rm = {n: (v['type'], v['encoded']) for n, v in r['memory'].items()}
            if km != rm:
                failures += 1
                print(f"        MEMORY DIFFERS: K={km} Rust={rm}")
        print(f"        {note}")
        print(f"        K: {detail[:100]} | Rust: {r.get('error', 'ok')[:100]}")
        if not ok:
            print(f"        expected K={exp_k} Rust={exp_r} outcome={exp_outcome}")
    print(f'\n{len(CASES) - failures}/{len(CASES)} divergence cases behave as expected')
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())

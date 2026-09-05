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

CASES.append(('f05_noncanonical_foreign_limbs', 'Finding 5',
    program([('%x', 'Base<Secp256k1>')], [
        {'op': 'add', 'a': '%x', 'b': '%x', 'output': '%y'}]),
    [((zv.K256P + 5 - 1) & ((1 << 192) - 1)), ((zv.K256P + 5 - 1) >> 192)],
    'ok', 'ok', 'add', 'holds',
    'limbs encoding p + 5 decode (reduced) as 5 on both sides'))

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
    '(0, -1) has order 2: on the curve, not in the subgroup; off-circuit both sides error, in circuit the cofactor-cleared point is unsatisfiable'))

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
    program([('%x', NATIVE), ('%y', NATIVE)], [
        {'op': 'from_coordinates', 'inputs': ['%x', '%y'], 'output': '%p'},
        {'op': 'into_coordinates', 'point': '%p', 'outputs': ['%px', '%py']}]), [wrong_x, G8[1]],
    'ok', 'ok', 'from_coordinates', 'violated',
    'off-circuit only the parity of x is used (decompression), so a wrong x yields the real point; in-circuit (x, y) must lie on the curve'))

CASES.append(('k02_transcript_too_short_panics', 'K2 (new)',
    program([('%a', NATIVE)], [
        {'op': 'private_input', 'guard': None, 'type': NATIVE, 'output': '%x'},
        {'op': 'add', 'a': '%a', 'b': '%x', 'output': '%y'}]), [1],
    'error', 'oracle-failed', 'private_input', 'unknown',
    'an unguarded private_input with an empty private transcript: the crate indexes the slice and panics (index out of range) instead of returning an error; K reports an error'))

CASES.append(('f01_reconstitute_overflow', 'Finding 1 (retired)',
    program([('%d', NATIVE), ('%m', NATIVE)], [
        {'op': 'reconstitute_field', 'divisor': '%d', 'modulus': '%m', 'bits': 8, 'output': '%o'}]),
    [(zv.R - 1) >> 8, 255],
    'error', 'error', 'reconstitute_field', 'unknown',
    'divisor * 2^8 + modulus >= r: off-circuit overflow error; in circuit the sum would wrap mod r'))


def oracle(path: Path, pre: dict) -> dict:
    with tempfile.NamedTemporaryFile('w', suffix='.json', delete=False) as f:
        json.dump(pre, f)
    res = subprocess.run([str(ORACLE), str(path), f.name], capture_output=True, text=True)
    return json.loads(res.stdout) if res.returncode == 0 else {'status': 'oracle-failed', 'error': res.stderr[-300:]}


def main() -> int:
    OUT.mkdir(exist_ok=True)
    runner = Runner()
    check_runner = Runner(HERE.parent / 'semantics' / 'zkir-check-kompiled')
    failures = 0
    for name, finding, prog, raw, exp_k, exp_r, gate_sub, exp_outcome, note in CASES:
        path = OUT / f'{name}.zkir'
        path.write_text(json.dumps(prog, indent=1) + '\n')
        pre = {'inputs': [str(x) for x in raw], 'binding_input': '42'}
        # static well-formedness first (some findings are rejected there)
        from pyk.kast.inner import KSort
        from pyk.kore.parser import KoreParser
        term = zkir_kast.load_program(path)
        res = check_runner.krun.run_process(check_runner.krun.kast_to_kore(term, KSort('Program')))
        wf = zkir_kast.wf_result(check_runner.krun.pretty_print(check_runner.krun.kore_to_kast(KoreParser(res.stdout).pattern())))
        r = oracle(path, pre)
        if wf != 'wfOk':
            k_status = 'wfError'
            outcome = 'n/a'
            detail = wf
        else:
            k = runner.run(term, pre)
            k_status = k['status']
            norm = lambda t: t.replace('_', '').lower()
            hits = [v for v in k['violations'] if norm(gate_sub) in norm(v[2])]
            if hits:
                outcome = hits[0][0]
                detail = f'{hits[0][0]}: {hits[0][1]}'
            else:
                outcome = 'holds' if any(gate_sub in g for g in [gate_sub]) else 'holds'
                detail = k.get('error', 'ok')
        ok = (k_status == exp_k) and (r['status'] == exp_r) and (outcome == exp_outcome)
        failures += not ok
        print(f"{'PASS' if ok else 'FAIL'}  {name:42} {finding:20} K={k_status:8} Rust={r['status']:6} gate={gate_sub}:{outcome}")
        print(f"        {note}")
        print(f"        K: {detail[:100]} | Rust: {r.get('error', 'ok')[:100]}")
        if not ok:
            print(f"        expected K={exp_k} Rust={exp_r} outcome={exp_outcome}")
    print(f'\n{len(CASES) - failures}/{len(CASES)} divergence cases behave as expected')
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())

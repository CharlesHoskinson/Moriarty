"""Milestone 5: the divergence findings as executable tests (oracle 2).

Each case is a small ZKIR v3 program written here, a preimage, and the expected
pattern: the off-circuit status on both sides (K witness and Rust
`preprocess`), and the outcome the constraint checker must assign to one
specific gate. Findings refer to arc-zkir docs/zkir-v3-divergence-review.md at
midnight-ledger 92e8bdd3 (wiki/zkir/zkir-v3-divergence-review.md); "K1" is a
candidate found while writing the K semantics.

Each case also carries the outcome the circuit oracle (`zkir-circuit-oracle`,
MockProver on the same preimage) must report, from the divergence-cases
section of plan-iter3/circuit-comparison-table.md (`CIRCUIT` below), with the
message class of a panic (D2: the unwrap class requires both "called
`Result::unwrap()`" and "Synthesis("; D1c: "AssignedByte" / "AssignedBounded";
D10: the two preprocess panic messages).

A case in `INJECT` is an injection case (items 5, 10, 11 of the 2026-09-06
review): the circuit oracle runs on the honest preimage given there with
`--inject` of one Native register, the taint walk of circuit_compare.py
predicts the cell from the honest K run, and the prediction must AGREE with
the oracle outcome (and with `CIRCUIT`). The K columns of such a case are
those of the case's own raw inputs, which may put the injected value in the
witness directly so that the K gate is evaluated on it.

A case in `EXT_CASES` is on the extension surface: it runs on the ZKIR-EXT
definition and the two midnight-zkir 2ffe2d1 oracles. A case whose program is
`None` is read from `corpus/divergence/<name>.zkir` with the preimage
`<name>.pre.json` beside it, instead of being written here.

Usage: uv run --group zkir-k python experiments/zkir-k/tools/divergence_tests.py
Writes the programs to corpus/divergence/ and exits 0 iff every case matches.
"""
from __future__ import annotations

import json
import random
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import circuit_compare as cc  # noqa: E402
import diff_test  # noqa: E402
import zkir_kast  # noqa: E402
import zkir_values as zv  # noqa: E402
from zkir_run import Runner  # noqa: E402

HERE = Path(__file__).resolve().parent
OUT = HERE.parent / 'corpus' / 'divergence'
ORACLE = Path.home() / 'Moriarty/repos/_build/ledger-92e8bdd3/target/release/zkir-oracle'
ORACLE_EXT = Path.home() / 'Moriarty/repos/_build/midnight-zkir-2ffe2d1/target/release/zkir-oracle'

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
    'ok', 'ok', 'public_input', 'unconstrained',
    'an inactive guard yields the default value off-circuit; the gate never reads the guard, and the register is a free cell in circuit (plan-iter3 M2)'))

CASES.append(('f04_less_than_odd_bits', 'Finding 4',
    program([('%a', NATIVE), ('%b', NATIVE)], [
        {'op': 'less_than', 'a': '%a', 'b': '%b', 'bits': 3, 'output': '%lt'}]), [8, 9],
    'error', 'error', 'less_than', 'unknown',
    'a = 8 exceeds 3 bits off-circuit; the chip would accept the padded 4-bit bound (output absent, so unknown)'))

EXTRA_PRE = {
    # the guard-1 transcript cases read one public transcript output (and e01b
    # pushes it as a public input): the settled preimage of the honest run
    'e01a_pub_guard1_free': {'public_transcript_outputs': ['1']},
    'e01b_pub_guard1_impact': {'public_transcript_outputs': ['1'], 'public_transcript_inputs': ['1']},
}
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

CASES.append(('k08_load_constant_jubjub_chip', 'K7 (new)',
    None, None,
    'ok', 'ok', 'load_constant', 'synthErr',
    'a load_constant of a Jubjub value with no Jubjub input or transcript entry: used_chips at 2ffe2d1 never enables the chip, so preprocess accepts while the K gate reports the chip uninitialised and synthesis panics (extension surface)'))

# --- injection cases (2026-09-06 review, items 5, 10, 11, 4) ---------------------------------
# e10: a hash operand that overflows its 1-byte atom. Off-circuit both sides reject
# the value (alignment error, status error); in K the circuit-side decoder's range
# failure is `violated` on the hash gate (item 5); the oracle, given the honest
# preimage with the operand injected to 300, truncates in the decomposition hint
# and reports W at the output's mem_insert.
CASES.append(('e10_align_overflow', 'R1 #3 (item 5)',
    program([('%x', NATIVE)], [
        {'op': 'persistent_hash', 'alignment': [{'tag': 'atom', 'value': {'tag': 'bytes', 'length': 1}}], 'inputs': ['%x'], 'output': '%h'}]), [300],
    'error', 'error', 'persistent_hash', 'violated',
    'a value above 255 in a bytes-1 atom: off-circuit alignment error on both sides; in circuit the byte decomposition hint truncates and the digest differs from the witness (W under injection)'))

CASES.append(('e02_inv_zero_inject', 'R2 #1 (item 10)',
    program([('%a', NATIVE)], [{'op': 'inv', 'a': '%a', 'output': '%o'}]), [5],
    'ok', 'ok', 'inv', 'holds',
    'honest run holds; injecting 0 into the operand makes std.inv hint 0, which mem_insert compares with the honest inverse: W, not a constraint failure'))

CASES.append(('e03_b32_high_inject', 'R2 #1 (item 10)',
    program([('%lo', NATIVE), ('%hi', NATIVE)], [
        {'op': 'bytes32_from_low_high', 'inputs': ['%lo', '%hi'], 'output': '%b'}]), [12345, 7],
    'ok', 'ok', 'bytes32_from_low_high', 'holds',
    'honest run holds; a high operand injected to 256 trips the AssignedByte conversion assertion: panic, not a constraint failure'))

CASES.append(('e03b_b32_low_inject', 'R2 #1 (item 10)',
    program([('%lo', NATIVE), ('%hi', NATIVE)], [
        {'op': 'bytes32_from_low_high', 'inputs': ['%lo', '%hi'], 'output': '%b'}]), [12345, 7],
    'ok', 'ok', 'bytes32_from_low_high', 'holds',
    'honest run holds; a low operand injected to 2^248 sets byte 31, which assert_equal_to_fixed(bytes[31], 0) rejects at verify: C'))

CASES.append(('e04_lt_bound_inject', 'R2 #1 (item 10)',
    program([('%a', NATIVE), ('%b', NATIVE)], [
        {'op': 'less_than', 'a': '%a', 'b': '%b', 'bits': 4, 'output': '%lt'}]), [1, 2],
    'ok', 'ok', 'less_than', 'holds',
    'honest run holds; an operand injected to 16 = 2^4 trips BoundedElement::new ("AssignedBounded less than 2^4"): panic'))

# item 11: a violated gate followed by a witness-independent synthErr. The row is
# decided by the synthErr (it fires in optimal_k before MockProver::run), so the
# oracle is X with the unwrap class, not the C/W/X three-way set.
CASES.append(('e11_violated_then_synth', 'R2 #2 (item 11)',
    program([('%x', NATIVE), ('%y', NATIVE), ('%enable', 'Point<Jubjub>'), ('%s', 'Scalar<Jubjub>'), ('%t', 'Scalar<Jubjub>')], [
        {'op': 'from_coordinates', 'inputs': ['%x', '%y'], 'output': '%p'},
        {'op': 'constrain_eq', 'a': '%s', 'b': '%t'}]), [wrong_x, G8[1], G8[0], G8[1], 42, 42],
    'ok', 'ok', 'from_coordinates', 'violated',
    'from_coordinates is violated (wrong x of the right parity) and constrain_eq on JubjubScalar is synthErr; the synthErr decides the row: the unwrap panic in optimal_k precedes any verify'))

# item 4 / 12: transcript registers at guard 1. The gate pins the cell to its type
# only; freedom is decided by the consumers (e1a: none, A; e1b: an impact, W).
CASES.append(('e01a_pub_guard1_free', 'R1 #2 (item 4)',
    program([('%g', NATIVE)], [
        {'op': 'public_input', 'guard': '%g', 'type': NATIVE, 'output': '%v'}]), [1],
    'ok', 'ok', 'public_input', 'holds',
    'a guard-1 public_input that no instruction reads: injecting the register is accepted, the input gate binds the cell to its type only (the transcript is a residual)'))

CASES.append(('e01b_pub_guard1_impact', 'R1 #2 (item 4)',
    program([('%g', NATIVE)], [
        {'op': 'public_input', 'guard': '%g', 'type': NATIVE, 'output': '%v'},
        {'op': 'impact', 'guard': '%g', 'inputs': ['%v']}]), [1],
    'ok', 'ok', 'public_input', 'holds',
    'the same register consumed by an impact: the injected value reaches pi_push and differs from the witness public input: W'))

# item 21: widths above 248 are preprocess-stage failures (ir_vm.rs "Excessive bit
# count"), which K's wf mirrors; keygen keys both programs (tools/provability.py
# lists them as keyed with a preprocess-stage failure).
CASES.append(('e21a_div_mod_249', 'R3 #3 (item 21)',
    program([('%v', NATIVE)], [
        {'op': 'div_mod_power_of_two', 'val': '%v', 'bits': 249, 'outputs': ['%q', '%r']}]), [5],
    'wfError', 'error', 'div_mod_power_of_two', 'n/a',
    'bits 249 > 248: preprocess rejects it on every preimage ("Excessive bit count"), wf mirrors that; keygen keys the circuit (k=9)'))

CASES.append(('e21b_reconstitute_249', 'R3 #3 (item 21)',
    program([('%d', NATIVE), ('%m', NATIVE)], [
        {'op': 'reconstitute_field', 'divisor': '%d', 'modulus': '%m', 'bits': 249, 'output': '%o'}]), [1, 1],
    'wfError', 'error', 'reconstitute_field', 'n/a',
    'bits 249 > 248: preprocess rejects it on every preimage, wf mirrors that; keygen keys the circuit'))

CASES.append(('f01_reconstitute_overflow', 'Finding 1 (retired)',
    program([('%d', NATIVE), ('%m', NATIVE)], [
        {'op': 'reconstitute_field', 'divisor': '%d', 'modulus': '%m', 'bits': 8, 'output': '%o'}]),
    [(zv.R - 1) >> 8, 255],
    'error', 'error', 'reconstitute_field', 'unknown',
    'divisor * 2^8 + modulus >= r: off-circuit overflow error; in circuit the sum would wrap mod r'))


# The expected circuit-oracle outcome of every case (plan-iter3 M1), from the
# "Divergence cases" section of plan-iter3/circuit-comparison-table.md: the
# outcome and, for a panic, the message class of D2 / D1c / D10 (a tuple of
# alternatives, each a tuple of required substrings; see circuit_compare).
CHIP = lambda name: cc.msg_class('must enable ' + name)
CIRCUIT = {
    'f02_assert_non_boolean': (cc.P, None),
    'f03_guard_uncoupled': (cc.A, None),
    'f04_less_than_odd_bits': (cc.P, None),
    'f05_noncanonical_foreign_limbs': (cc.C, None),
    'f06_cond_select_bytes32': (cc.X, cc.MSG_UNWRAP),
    'f07_constrain_eq_jubjub_scalar': (cc.X, cc.MSG_UNWRAP),
    'f08_bytes32_from_low_high_foreign_high': (cc.X, cc.MSG_UNWRAP),
    'f10_reconstitute_bits_256': (cc.P, None),
    'f11_curve25519_torsion_point': (cc.P, None),
    'f12_ec_mul_generator_p256': (cc.P, None),
    'f13_chip_gating_from_bytes32': (cc.X, CHIP('secp256k1')),
    'k01_jubjub_from_coordinates_parity_only': (cc.C, None),
    'k01b_jubjub_from_coordinates_no_chip': (cc.X, CHIP('jubjub')),
    'k03_bytes32_input_assertion_panics': (cc.X, cc.msg_class('assertion `left == right` failed')),
    'k04_jubjub_scalar_from_native_chip': (cc.X, CHIP('jubjub')),
    'k05_less_than_253_bits_keygen': (cc.X, cc.MSG_WIDTH),
    'k06_empty_impact_guard': (cc.P, None),
    'k07_alignment_option_offcircuit': (cc.X, cc.MSG_UNWRAP),
    'k02_transcript_too_short_panics': (cc.X, cc.msg_class('out of range for slice')),
    'k08_load_constant_jubjub_chip': (cc.X, CHIP('jubjub')),
    'f01_reconstitute_overflow': (cc.P, None),
    # injection cases: the oracle runs on INJECT's honest preimage with --inject
    'e10_align_overflow': (cc.W, None),
    'e02_inv_zero_inject': (cc.W, None),
    'e03_b32_high_inject': (cc.X, cc.MSG_BYTE),
    'e03b_b32_low_inject': (cc.C, None),
    'e04_lt_bound_inject': (cc.X, cc.MSG_BOUNDED),
    'e11_violated_then_synth': (cc.X, cc.MSG_UNWRAP),
    'e01a_pub_guard1_free': (cc.A, None),
    'e01b_pub_guard1_impact': (cc.W, None),
    'e21a_div_mod_249': (cc.P, None),
    'e21b_reconstitute_249': (cc.P, None),
}

# name -> (honest raw inputs for the oracle run, {register: injected value}); the
# taint walk of circuit_compare.py must predict the oracle's outcome (AGREE)
INJECT = {
    'e10_align_overflow': ([5], {'%x': 300}),
    'e02_inv_zero_inject': ([5], {'%a': 0}),
    'e03_b32_high_inject': ([12345, 7], {'%hi': 256}),
    'e03b_b32_low_inject': ([12345, 7], {'%lo': 1 << 248}),
    'e04_lt_bound_inject': ([1, 2], {'%a': 16}),
    'e01a_pub_guard1_free': ([1], {'%v': 3}),
    'e01b_pub_guard1_impact': ([1], {'%v': 3}),
}

EXT_CASES = {'k08_load_constant_jubjub_chip'}


def oracle(path: Path, pre: dict, ext: bool = False) -> dict:
    with tempfile.NamedTemporaryFile('w', suffix='.json') as f:
        json.dump(pre, f)
        f.flush()
        res = subprocess.run([str(ORACLE_EXT if ext else ORACLE), str(path), f.name], capture_output=True, text=True)
    if res.returncode == 101:
        return {'status': 'panic', 'error': res.stderr.strip()[-300:]}
    if res.returncode != 0:
        return {'status': 'load-error', 'error': res.stderr.strip()[-300:]}
    return json.loads(res.stdout)


def main() -> int:
    OUT.mkdir(exist_ok=True)
    runners: dict[bool, Runner] = {False: Runner()}
    failures = 0
    for name, finding, prog, raw, exp_k, exp_r, gate_sub, exp_outcome, note in CASES:
        ext = name in EXT_CASES
        if ext and True not in runners:
            runners[True] = Runner(ext=True)
        runner = runners[ext]
        path = OUT / f'{name}.zkir'
        if prog is None:
            prog = json.loads(path.read_text())
            pre = json.loads(path.with_suffix('.pre.json').read_text())
        else:
            path.write_text(json.dumps(prog, indent=1) + '\n')
            pre = {'inputs': [str(x) for x in raw], 'binding_input': '42'}
        if pre_extra := dict(EXTRA_PRE.get(name, {})):
            pre.update(pre_extra)
        term = zkir_kast.load_program(path, ext=ext)
        if prog['do_communications_commitment']:
            # the raw-stream commitment the crate accepts, taken from a K generation run
            pre['communications_commitment'] = ['0', '7']
            gen = runner.run(term, pre, gen=True)
            comm = [x for kind, x in gen['needs'] if kind == 'comm']
            if comm:
                pre['communications_commitment'] = [str(comm[0]), '7']
        r = oracle(path, pre, ext)
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
        exp_c, exp_c_msg = CIRCUIT[name]
        circ_bin = cc.CIRCUIT_ORACLE_EXT if ext else cc.CIRCUIT_ORACLE
        inject_note = ''
        walk_ok = True
        if name in INJECT:
            raw_h, inj = INJECT[name]
            # the honest preimage with its transcripts settled by the generation loop
            pre_h, _ = diff_test.build_preimage(runner, term, prog, random.Random(f'divergence:{name}'), small=True,
                                                seed_pre={'inputs': [str(x) for x in raw_h], 'binding_input': '42', **dict(pre_extra)})
            k_h = runner.run(term, pre_h)
            c = cc.run_circuit_oracle(circ_bin, path, pre_h, inject=inj)
            (reg, val), = inj.items()
            walk = cc.predict_injection(prog, k_h, reg, val)
            walk_ok = cc.judge(walk, c) == 'AGREE'
            inject_note = f" inject={reg}->{val} walk={walk.codes()} {'AGREE' if walk_ok else 'DISAGREE'} ({walk.reason[:90]})"
            if k_h['status'] != 'ok':
                walk_ok = False
                inject_note += f" honest K run is {k_h['status']}: {k_h.get('error', '')[:60]}"
        else:
            c = cc.run_circuit_oracle(circ_bin, path, pre)
        c_ok = c.get('outcome') == exp_c and cc.message_matches(exp_c_msg, c.get('message', ''))
        ok = (k_status == exp_k) and (r['status'] == exp_r) and (outcome == exp_outcome) and (k_status != 'stuck') and c_ok and walk_ok
        failures += not ok
        print(f"{'PASS' if ok else 'FAIL'}  {name:42} {finding:20} K={k_status:8} Rust={r['status']:6} gate={gate_sub}:{outcome} circuit={c.get('outcome')}{inject_note}")
        if k_status == 'ok' and r['status'] == 'ok' and 'memory' in r:
            km = {n: (v['type'], v['encoded']) for n, v in k['memory'].items()}
            rm = {n: (v['type'], v['encoded']) for n, v in r['memory'].items()}
            if km != rm:
                failures += 1
                print(f"        MEMORY DIFFERS: K={km} Rust={rm}")
        print(f"        {note}")
        print(f"        K: {detail[:100]} | Rust: {r.get('error', 'ok')[:100]}")
        print(f"        circuit: {c.get('outcome')} {c.get('message', '')[:110]!r}" + (f" failures={c['failures'][:2]}" if c.get('failures') else '') + f" {c.get('elapsed_ms', 0)}ms")
        if name not in INJECT and k_status == 'ok' and cc.judge(cc.expected_preimage_cell(k), c) != 'AGREE':
            # the row lookup of the comparison table must place the honest run too
            failures += 1
            ok = False
            print(f"        TABLE ROW DISAGREES: expected_preimage_cell={cc.expected_preimage_cell(k).cell} [{cc.expected_preimage_cell(k).codes()}]")
        if not ok:
            print(f"        expected K={exp_k} Rust={exp_r} outcome={exp_outcome} circuit={exp_c}" + (f" (message class {cc.describe_class(exp_c_msg)})" if exp_c_msg else ''))
    print(f'\n{len(CASES) - failures}/{len(CASES)} divergence cases behave as expected')
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())

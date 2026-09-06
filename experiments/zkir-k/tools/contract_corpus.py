"""Target contract (plan-iter3 M5a, M5b) over the whole corpus.

For every program under corpus/: build the Program term with zkir_kast.py (the
base surface for every directory except midnight-zkir-2ffe2d1-tests, which
loads with --ext), run ZKIR-CONTRACT-MAIN, and print one line per program with
the obligations that are not met or not applicable. Programs the preprocessor
rejects never reach K (`IrSource::load` rejects them before `preprocess`) and
are reported as `format`.

The expectation table names the programs whose contract is expected to carry
a failed obligation: the chip-gating divergence cases (finding 13, K4), the
keygen width case (K5), the alignment option case (K7), the reconstitute
256-bit case (f10, which also fails the keygen assertion) and the 249-bit
width cases (e21a, e21b), the alignment field-count and reconstitute bits-0
negatives (2026-09-06 items 19, 20), the programs whose keygen fails on a
witness-independent in-circuit check (the circuit.static obligations: operand
definition, output typing, div_mod_power_of_two arity, boolean gate arity,
constant decoding, nth and slice bounds, unsupported dispatch arms) and the
ill-formed programs. Every other program must have every obligation met or not
applicable. Each failed obligation is printed with its stage
(`name[stage]`), which tools/provability.py reads.

The seven Moriarty transaction contexts (corpus/moriarty-contexts, one real
preimage per artifact) are contract entries with a preimage: tiers two and
three run as well (`zkir_kast.contract` with the preimage), and each must have
every tier-one obligation met or not applicable, an off-circuit run with
status ok and a memory in the modelled witness space, and the circuit oracle
outcome `accepted`.

Usage: uv run --group zkir-k python experiments/zkir-k/tools/contract_corpus.py [--definition DIR] [--no-contexts]
Exit status 0 iff every program matches its expectation.
"""
from __future__ import annotations

import argparse
import json
import sys
import time
from pathlib import Path

from pyk.ktool.krun import KRun

sys.path.insert(0, str(Path(__file__).resolve().parent))
import zkir_kast  # noqa: E402

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent  # experiments/zkir-k
REPO = ROOT.parent.parent
CORPUS = ROOT / 'corpus'
CONTEXTS = CORPUS / 'moriarty-contexts'
# the compiled Moriarty artifacts the contexts belong to (artifact name prefix -> directory)
MORIARTY = {
    'swap': REPO / 'experiments' / 'moriarty-core-swap' / 'output' / 'zkir',
    'escrow': REPO / 'experiments' / 'moriarty-compact-escrow' / 'output' / 'zkir',
}

# directory -> loads with the extension surface
CORPORA = {
    'ledger9-92e8bdd3-tests': False,
    'midnight-zkir-2ffe2d1-precompiles': False,
    'midnight-zkir-2ffe2d1-tests': True,
    'handmade': False,
    'handmade-negative': False,
    'divergence': False,
}
# files of a base-surface directory that load with the extension surface
EXT_FILES = {'k08_load_constant_jubjub_chip.zkir'}

# file name -> the obligations expected to fail (a substring of the detail each),
# or 'format' when the preprocessor rejects the file. Unlisted: every
# obligation met or not applicable.
EXPECTED: dict[str, dict[str, str] | str] = {
    # chip gating: the three cases of 13-known-divergences.md
    'f13_chip_gating_from_bytes32.zkir': {'chips.gating': 'instruction 0 (from_bytes32) needs chip secp256k1'},
    'k04_jubjub_scalar_from_native_chip.zkir': {'chips.gating': 'instruction 0 (jubjub_scalar_from_native) needs chip jubjub'},
    'k01b_jubjub_from_coordinates_no_chip.zkir': {'chips.gating': 'instruction 0 (from_coordinates) needs chip jubjub'},
    'k08_load_constant_jubjub_chip.zkir': {'chips.gating': 'instruction 0 (load_constant) needs chip jubjub'},
    # keygen width and synthesis alignment cases
    'k05_less_than_253_bits_keygen.zkir': {'width.less_than': 'bits 253 pads to 254'},
    'k07_alignment_option_offcircuit.zkir': {'alignment.persistent_hash': 'option segment'},
    'f10_reconstitute_bits_256.zkir': {'wf': 'reconstitute_field: excessive bit count', 'width.reconstitute_field': 'bits 256 exceeds',
                                       'width.reconstitute_field.assertion': 'bits 256: assertion failed: (bit_length as u32) < F::NUM_BITS'},
    'e21a_div_mod_249.zkir': {'wf': 'div_mod_power_of_two: excessive bit count', 'width.div_mod_power_of_two': 'bits 249 exceeds'},
    'e21b_reconstitute_249.zkir': {'wf': 'reconstitute_field: excessive bit count', 'width.reconstitute_field': 'bits 249 exceeds'},
    # a violated gate followed by a synthErr (2026-09-06 item 11): only the dispatch obligation is static
    'e11_violated_then_synth.zkir': {'circuit.static.dispatch': 'instruction 1: Unsupported constrain_eq: JubjubScalar == JubjubScalar'},
    # the alignment field count (item 19) and reconstitute_field bits = 0 (item 20)
    'align_field_short.zkir': {'wf': 'alignment needs 2 field elements but instruction has 1',
                               'alignment.persistent_hash': 'cannot decode field element from no data'},
    'align_bytes_short.zkir': {'wf': 'alignment needs 2 field elements but instruction has 1',
                               'alignment.keccak256': 'cannot decode bytes from to little data'},
    'reconstitute_bits_0.zkir': {'wf': 'reconstitute_field: bits 0',
                                 'width.reconstitute_field.assertion': 'bits 0: assertion failed: (bit_length as u32) < F::NUM_BITS'},
    # witness-independent in-circuit checks of keygen (the ten contradictions of
    # evidence/zkir-k-provability-2026-09-06.txt): the crate tests below occur in
    # both test corpora with the same name
    'output_operand_type_mismatch.zkir': {'circuit.static.output': 'instruction 0: output position 0: signature declares JubjubPoint but operand has static type Native'},
    'test_bool_gate_empty_inputs_fails.zkir': {'circuit.static.bool_gate': 'instruction 0: and: Boolean gate requires at least one input'},
    'test_constant_bad_encoding_rejected.zkir': {'circuit.static.constant': 'instruction 0: load_constant: Failed to decode as Native'},
    'test_nth_out_of_bounds_fails.zkir': {'circuit.static.bytes_bounds': 'instruction 0: nth out of bounds: index 2 into Bytes<2>'},
    'test_slice_out_of_bounds_fails.zkir': {'circuit.static.bytes_bounds': 'instruction 0: slice out of bounds: 2..2+3 into Bytes<4>'},
    'f06_cond_select_bytes32.zkir': {'circuit.static.dispatch': 'instruction 0: Unsupported cond_select: Bytes32 ? Bytes32'},
    'f07_constrain_eq_jubjub_scalar.zkir': {'circuit.static.dispatch': 'instruction 0: Unsupported constrain_eq: JubjubScalar == JubjubScalar'},
    'f08_bytes32_from_low_high_foreign_high.zkir': {'circuit.static.dispatch': 'instruction 0: bytes32_from_low_high high: cannot convert Secp256k1Base to Native'},
    'f12_ec_mul_generator_p256.zkir': {'circuit.static.dispatch': 'instruction 0: Unsupported EcMulGenerator for scalar of type Secp256r1Scalar'},
    # ill-formed by construction (crate tests and handmade negatives); the output
    # arity is checked by wf and again by the in-circuit `I::Output` arm
    'output_arity_mismatch.zkir': {'wf': 'signature declares 1 return values but instruction has 2',
                                   'circuit.static.output': 'output: signature declares 1 return values but instruction has 2'},
    'test_invalid_operand_no_percent_prefix.zkir': 'format',
    'test_invalid_operand_malformed_identifier.zkir': 'format',
    'test_invalid_operand_odd_length_hex.zkir': 'format',
    'reassignment.zkir': {'wf': 'reassignment of %b'},
    # an undefined operand and a div_mod_power_of_two arity defect are rejected by
    # keygen as well (the three contradictions of evidence/zkir-k-provability-2026-09-06d.txt)
    'undefined_variable.zkir': {'wf': 'undefined variable %c',
                                'circuit.static.defined': 'instruction 0: value Identifier("%c") not in memory'},
    'duplicate_input.zkir': {'wf': 'duplicate input %a'},
    'excessive_bits.zkir': {'wf': 'constrain_bits: excessive bit bound'},
    'divmod_outputs.zkir': {'wf': 'div_mod_power_of_two requires exactly 2 outputs',
                            'circuit.static.div_mod_outputs': 'instruction 0: div_mod_power_of_two: Unexpected output length of DivModPowerOfTwo instruction'},
    'immediate_out_of_range.zkir': 'format',
    'wrong_version.zkir': 'format',
}


def summarise(obs: list[dict]) -> str:
    """One line per program: `ok (n met, m n/a)` or `FAILED name[stage]: detail; ...`
    (the stage of every failed obligation, keygen | preprocess | both, is what
    tools/provability.py reads to decide whether keygen must reject the program)."""
    failed = [o for o in obs if o['status'] == 'failed']
    if not failed:
        met = sum(o['status'] == 'met' for o in obs)
        na = sum(o['status'] == 'notApplicable' for o in obs)
        return f'ok ({met} met, {na} n/a)'
    return 'FAILED ' + '; '.join(f"{o['name']}[{o.get('stage') or '?'}]: {o['detail']}" for o in failed)


def matches(expected: dict[str, str] | str | None, obs: list[dict] | None, fmt_error: bool) -> bool:
    if expected == 'format':
        return fmt_error
    if fmt_error or obs is None:
        return False
    failed = {o['name']: o['detail'] or '' for o in obs if o['status'] == 'failed'}
    if expected is None:
        return not failed
    return set(failed) == set(expected) and all(expected[k] in failed[k] for k in expected)


NOT_RUN = 0


def contexts(krun: KRun) -> tuple[list[str], int, int]:
    """The seven Moriarty contexts: contract with preimage. Returns the lines,
    the number of entries and the number that miss their expectation; a tier
    three that could not run (no oracle binary) is counted in NOT_RUN, not as
    a miss."""
    global NOT_RUN
    manifest = json.loads((CONTEXTS / 'manifest.json').read_text())
    lines, total, bad = [], 0, 0
    for art, info in manifest['artifacts'].items():
        total += 1
        program = MORIARTY[art.split('-', 1)[0]] / f"{info['circuit']}.zkir"
        pre_path = CONTEXTS / f'{art}.pre.json'
        pre = json.loads(pre_path.read_text())
        term = zkir_kast.load_program(program, ext=False)
        doc = zkir_kast.contract(program, term, False, krun, pre, pre_path)
        t1, t2, t3 = doc['tier_one'], doc['preimage_dependent'], doc['circuit']
        # a tier three that was not run (no oracle binary) is neither met nor failed
        ok = t1['met'] and t2['met'] and t3['met'] is not False
        not_run = t3['met'] is None
        NOT_RUN += not_run
        bad += not ok
        two = f"off-circuit {t2['status']}" + (f" '{t2.get('error', '')[:60]}'" if t2['status'] != 'ok' else '') +               f", witness_space {str(t2['witness_space']).lower()}" +               (f", unconstrained {','.join(t2['unconstrained'][:4])}" if t2['unconstrained'] else '') +               (f", violations {len(t2['violations'])}" if t2['violations'] else '')
        three = f"circuit {t3['outcome']}" + (f" k={t3['k']}" if 'k' in t3 else '') + (f" {t3['elapsed_ms']}ms" if 'elapsed_ms' in t3 else '') +                 (f" '{t3.get('message', t3.get('reason', ''))[:60]}'" if t3['outcome'] != 'accepted' else '')
        obs = t2.get('observable') or {}
        skips = obs.get('skips')
        skip_note = '' if skips is None else f", {len(skips)} impacts of which {sum(1 for x in skips if x is not None)} skipped"
        ledger = doc.get('ledger', {}).get('commitment', {})
        lines.append(f"moriarty-contexts/{art} ({program.relative_to(REPO)} + {pre_path.name}): {summarise(doc['obligations'])}; {two}; {three}; "
                     f"observable ({obs.get('status', '?')}, {len(obs.get('outputs', []))} output elements, {len(obs.get('pis', []))} public inputs{skip_note}); "
                     f"ledger.commitment {'met' if ledger.get('met') else 'NOT MET'}"
                     + (' (tier three not run)' if not_run else '') + ('' if ok else '   <-- UNEXPECTED'))
    return lines, total, bad


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--definition', type=Path, default=zkir_kast.default_definition('zkir-contract-main'))
    ap.add_argument('--no-contexts', action='store_true', help='skip the Moriarty contexts (tiers two and three)')
    args = ap.parse_args(argv)
    krun = KRun(args.definition)
    t0 = time.time()
    total = clean = expected_failures = unexpected = formats = 0
    for name, ext in CORPORA.items():
        for path in sorted((CORPUS / name).glob('*.zkir')):
            total += 1
            label = f'{name}/{path.name}'
            expected = EXPECTED.get(path.name)
            obs = None
            fmt_error = False
            try:
                term = zkir_kast.load_program(path, ext=ext or path.name in EXT_FILES)
            except zkir_kast.ZkirFormatError as e:
                fmt_error = True
                line = f'format: {e}'
            else:
                try:
                    obs = zkir_kast.run_contract(krun, term)
                    line = summarise(obs)
                except RuntimeError as e:
                    line = f'krun failed: {str(e)[:200]}'
            ok = matches(expected, obs, fmt_error)
            if fmt_error:
                formats += 1
            elif obs is not None and not any(o['status'] == 'failed' for o in obs):
                clean += 1
            elif ok:
                expected_failures += 1
            if not ok:
                unexpected += 1
            flag = '' if ok else '   <-- UNEXPECTED'
            print(f'{label}: {line}{flag}')
    ctx_total = ctx_bad = 0
    if not args.no_contexts:
        lines, ctx_total, ctx_bad = contexts(krun)
        print('\n'.join(lines))
        unexpected += ctx_bad
    print(f'{total} programs, {clean} with every obligation met or not applicable, '
          f'{expected_failures} with an expected failed obligation, {formats} format errors; '
          f'{ctx_total} Moriarty contexts with a preimage, {ctx_total - ctx_bad} with every tier met'
          + (f' ({NOT_RUN} with tier three not run: no oracle binary)' if NOT_RUN else '') + '; '
          f'{unexpected} unexpected, {time.time() - t0:.1f}s')
    return 0 if unexpected == 0 else 1


if __name__ == '__main__':
    sys.exit(main())

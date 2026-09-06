"""Target contract, tier one (plan-iter3 M5a), over the whole corpus.

For every program under corpus/: build the Program term with zkir_kast.py (the
base surface for every directory except midnight-zkir-2ffe2d1-tests, which
loads with --ext), run ZKIR-CONTRACT-MAIN, and print one line per program with
the obligations that are not met or not applicable. Programs the preprocessor
rejects never reach K (`IrSource::load` rejects them before `preprocess`) and
are reported as `format`.

The expectation table names the programs whose contract is expected to carry
a failed obligation: the chip-gating divergence cases (finding 13, K4), the
keygen width case (K5), the alignment option case (K7), the reconstitute
256-bit case (f10) and the ill-formed programs. Every other program must have
every obligation met or not applicable.

Usage: uv run --group zkir-k python experiments/zkir-k/tools/contract_corpus.py [--definition DIR]
Exit status 0 iff every program matches its expectation.
"""
from __future__ import annotations

import argparse
import sys
import time
from pathlib import Path

from pyk.ktool.krun import KRun

sys.path.insert(0, str(Path(__file__).resolve().parent))
import zkir_kast  # noqa: E402

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent  # experiments/zkir-k
CORPUS = ROOT / 'corpus'

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
    'f10_reconstitute_bits_256.zkir': {'wf': 'reconstitute_field: excessive bit count', 'width.reconstitute_field': 'bits 256 exceeds'},
    # ill-formed by construction (crate tests and handmade negatives)
    'output_arity_mismatch.zkir': {'wf': 'signature declares 1 return values but instruction has 2'},
    'test_invalid_operand_no_percent_prefix.zkir': 'format',
    'test_invalid_operand_malformed_identifier.zkir': 'format',
    'test_invalid_operand_odd_length_hex.zkir': 'format',
    'reassignment.zkir': {'wf': 'reassignment of %b'},
    'undefined_variable.zkir': {'wf': 'undefined variable %c'},
    'duplicate_input.zkir': {'wf': 'duplicate input %a'},
    'excessive_bits.zkir': {'wf': 'constrain_bits: excessive bit bound'},
    'divmod_outputs.zkir': {'wf': 'div_mod_power_of_two requires exactly 2 outputs'},
    'immediate_out_of_range.zkir': 'format',
    'wrong_version.zkir': 'format',
}


def summarise(obs: list[dict]) -> str:
    failed = [o for o in obs if o['status'] == 'failed']
    if not failed:
        met = sum(o['status'] == 'met' for o in obs)
        na = sum(o['status'] == 'notApplicable' for o in obs)
        return f'ok ({met} met, {na} n/a)'
    return 'FAILED ' + '; '.join(f"{o['name']}: {o['detail']}" for o in failed)


def matches(expected: dict[str, str] | str | None, obs: list[dict] | None, fmt_error: bool) -> bool:
    if expected == 'format':
        return fmt_error
    if fmt_error or obs is None:
        return False
    failed = {o['name']: o['detail'] or '' for o in obs if o['status'] == 'failed'}
    if expected is None:
        return not failed
    return set(failed) == set(expected) and all(expected[k] in failed[k] for k in expected)


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--definition', type=Path, default=zkir_kast.default_definition('zkir-contract-main'))
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
    print(f'{total} programs, {clean} with every obligation met or not applicable, '
          f'{expected_failures} with an expected failed obligation, {formats} format errors, '
          f'{unexpected} unexpected, {time.time() - t0:.1f}s')
    return 0 if unexpected == 0 else 1


if __name__ == '__main__':
    sys.exit(main())

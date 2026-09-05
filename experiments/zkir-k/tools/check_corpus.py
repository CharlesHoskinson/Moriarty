"""Milestone 2 test: every ZKIR v3 program in the corpus parses and is well formed.

For each program: build the Program term with zkir_kast.py, run ZKIR-CHECK, and
compare the outcome with the expectation table below (default: wfOk). Programs
whose expected outcome is a format error never reach K; that mirrors
`IrSource::load` rejecting them before `preprocess`.

Usage: uv run --group zkir-k python experiments/zkir-k/tools/check_corpus.py [--definition DIR]
Exit status 0 iff every program matches its expectation.
"""
from __future__ import annotations

import argparse
import json
import sys
import time
from pathlib import Path

from pyk.kast.inner import KSort
from pyk.kore.parser import KoreParser
from pyk.ktool.krun import KRun

sys.path.insert(0, str(Path(__file__).resolve().parent))
import zkir_kast  # noqa: E402

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent  # experiments/zkir-k
REPO = ROOT.parent.parent

CORPORA = {
    'ledger9-92e8bdd3-tests': ROOT / 'corpus' / 'ledger9-92e8bdd3-tests',
    'midnight-zkir-2ffe2d1-precompiles': ROOT / 'corpus' / 'midnight-zkir-2ffe2d1-precompiles',
    'moriarty-compact-escrow': REPO / 'experiments' / 'moriarty-compact-escrow' / 'output' / 'zkir',
    'moriarty-core-swap': REPO / 'experiments' / 'moriarty-core-swap' / 'output' / 'zkir',
    'handmade-negative': ROOT / 'corpus' / 'handmade-negative',
}

# Expected outcome when it is not wfOk. 'format' = rejected by the preprocessor
# (serde-level), 'wfError:<substring>' = static check fails with that message.
EXPECTED = {
    'output_arity_mismatch.zkir': 'wfError:signature declares 1 return values but instruction has 2',
    # The type mismatch is dynamic (runtime types); statically the program is fine.
    'output_operand_type_mismatch.zkir': 'wfOk',
    'test_invalid_operand_no_percent_prefix.zkir': 'format',
    'test_invalid_operand_malformed_identifier.zkir': 'format',
    'test_invalid_operand_odd_length_hex.zkir': 'format',
    # handmade negatives (corpus/handmade-negative)
    'reassignment.zkir': 'wfError:reassignment of %b',
    'undefined_variable.zkir': 'wfError:undefined variable %c',
    'duplicate_input.zkir': 'wfError:duplicate input %a',
    'excessive_bits.zkir': 'wfError:constrain_bits: excessive bit bound',
    'immediate_out_of_range.zkir': 'format',
    'divmod_outputs.zkir': 'wfError:div_mod_power_of_two requires exactly 2 outputs',
    'wrong_version.zkir': 'format',
}


def outcome(krun: KRun, path: Path) -> str:
    try:
        term = zkir_kast.load_program(path)
    except zkir_kast.ZkirFormatError as e:
        return f'format: {e}'
    kore = krun.kast_to_kore(term, KSort('Program'))
    res = krun.run_process(kore)
    if res.returncode != 0:
        return f'krun failed: {res.stderr.strip()[:200]}'
    pretty = krun.pretty_print(krun.kore_to_kast(KoreParser(res.stdout).pattern()))
    return zkir_kast.wf_result(pretty)


def matches(expected: str, actual: str) -> bool:
    if expected == 'wfOk':
        return actual == 'wfOk'
    if expected == 'format':
        return actual.startswith('format')
    if expected.startswith('wfError:'):
        return actual.startswith('wfError') and expected[len('wfError:'):] in actual
    return False


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument('--definition', type=Path, default=zkir_kast.default_definition())
    args = ap.parse_args()
    krun = KRun(args.definition)
    rows = []
    failures = 0
    t0 = time.time()
    for corpus, directory in CORPORA.items():
        for path in sorted(directory.glob('*.zkir')):
            doc = json.loads(path.read_text())
            if doc.get('version', {}).get('major') != 3:
                continue
            expected = EXPECTED.get(path.name, 'wfOk')
            actual = outcome(krun, path)
            ok = matches(expected, actual)
            failures += not ok
            rows.append((corpus, path.name, len(doc['instructions']), expected, actual, ok))
    width = max(len(r[1]) for r in rows)
    for corpus, name, n, expected, actual, ok in rows:
        print(f"{'PASS' if ok else 'FAIL'}  {corpus:34} {name:{width}} {n:4d}  {actual}")
    print(f'\n{len(rows)} programs, {len(rows) - failures} as expected, {failures} unexpected, {time.time() - t0:.1f}s')
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())

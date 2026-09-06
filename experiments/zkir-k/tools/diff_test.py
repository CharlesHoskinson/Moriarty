"""Oracle 1: differential test of the K semantics against the Rust crate.

For every version-3 program in the corpus:
  1. draw raw inputs of the declared types (seeded per program);
  2. run the K semantics in generation mode until the transcript needs are
     stable (public/private transcript values of the requested types are
     appended, impact values become public_transcript_inputs, and the
     communications commitment is taken from the run);
  3. run the real K semantics and the Rust `preprocess` (zkir-oracle) on the
     same preimage and compare status, memory (variant and encoding of every
     register), public inputs and skips;
  4. repeat step 3 with one perturbed preimage (first raw input replaced), so
     that error paths are compared too.

Usage: uv run --group zkir-k python experiments/zkir-k/tools/diff_test.py [--only SUBSTR] [--seed N]
Exit 0 iff every comparison agrees.

`--circuit` (plan-iter3 M1) additionally runs `zkir-circuit-oracle` (preprocess,
MockProver::run, verify) on every preimage of steps 3 and 4 and, for a
successful honest run, on Native-register injections into the preprocessed
memory and on an instance-column perturbation. Each outcome is compared with
the cell of plan-iter3/circuit-comparison-table.md (circuit_compare.py); a
comparison outside the table is a blocking finding listed at the end. The
output of the other modes is unchanged.
"""
from __future__ import annotations

import argparse
import json
import random
import subprocess
import sys
import tempfile
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import circuit_compare as cc  # noqa: E402
import zkir_kast  # noqa: E402
import zkir_values as zv  # noqa: E402
from zkir_run import Runner  # noqa: E402

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
REPO = ROOT.parent.parent
ORACLE = Path.home() / 'Moriarty/repos/_build/ledger-92e8bdd3/target/release/zkir-oracle'
ORACLE_EXT = Path.home() / 'Moriarty/repos/_build/midnight-zkir-2ffe2d1/target/release/zkir-oracle'

CORPORA = {
    'ledger9-92e8bdd3-tests': ROOT / 'corpus' / 'ledger9-92e8bdd3-tests',
    'midnight-zkir-2ffe2d1-precompiles': ROOT / 'corpus' / 'midnight-zkir-2ffe2d1-precompiles',
    'moriarty-compact-escrow': REPO / 'experiments' / 'moriarty-compact-escrow' / 'output' / 'zkir',
    'moriarty-core-swap': REPO / 'experiments' / 'moriarty-core-swap' / 'output' / 'zkir',
    'handmade': ROOT / 'corpus' / 'handmade',
}


ORACLE_BIN = ORACLE


def oracle(program: Path, preimage: dict) -> dict:
    with tempfile.NamedTemporaryFile('w', suffix='.json') as f:
        json.dump(preimage, f)
        f.flush()
        res = subprocess.run([str(ORACLE_BIN), str(program), f.name], capture_output=True, text=True)
    if res.returncode == 101:   # Rust panic (unwinding abort)
        return {'status': 'panic', 'error': res.stderr.strip()[-500:]}
    if res.returncode != 0:      # load error or preimage error: reported before preprocess
        return {'status': 'load-error', 'error': res.stderr.strip()[-500:]}
    return json.loads(res.stdout)


# Error classes: both sides' messages are mapped to a small vocabulary and the
# classes must agree; formatting differences (K prints decimals, Rust prints
# little-endian hex) are not compared.
ERROR_CLASSES = [
    ('variable-not-found', r'variable not found|register .* is not in the witness'),
    ('type-conversion', r'cannot convert'),
    ('excessive-bits', r'Excessive bit (count|bound)'),
    ('bit-bound', r'Bit bound failed'),
    ('boolean', r'Expected boolean|Boolean gate'),
    ('assertion', r'Failed direct assertion'),
    ('equality', r'Equality constraint failed'),
    ('unsupported-op', r'Unsupported|Cannot build|Cannot extract|Concat expects|expects Byte'),
    ('decode', r'Failed to decode|not in canonical form|Bytes32 decoding|Not enough raw inputs|Expected \d+ raw inputs|slice out of bounds|nth out of bounds|slice length'),
    ('transcript-short', r'transcript index out of range|range end index|index out of range'),
    ('transcript-unconsumed', r'Transcripts not fully consumed'),
    ('commitment', r'communications? (commitment|randomness)|Communications commitment'),
    ('impact-mismatch', r'Public transcript input mismatch'),
    ('output', r'^Output|Output position|output: signature'),
    ('div-mod-arity', r'DivModPowerOfTwo requires|div_mod_power_of_two requires'),
    ('alignment', r'did not match alignment|alignment'),
    ('inverse', r'cannot invert zero'),
    ('overflow', r'overflows field'),
    ('low-high', r'Bytes32FromLowHigh|low operand|high operand'),
    ('reassignment', r'reassignment of'),
    ('wf', r'wfError|duplicate input|undefined variable|immediate out of field'),
]


def err_class(msg: str) -> str:
    import re
    for name, pat in ERROR_CLASSES:
        if re.search(pat, msg or '', re.I):
            return name
    return 'other:' + (msg or '')[:40]


def strs(xs):
    return [str(x) for x in xs]


def build_preimage(runner: Runner, program, doc: dict, rng: random.Random, small: bool, seed_pre: dict | None = None) -> tuple[dict, int]:
    gen = zv.small_encoded if small else zv.random_encoded
    inputs = [x for entry in doc['inputs'] for x in gen(entry['type'], rng)]
    rand = rng.randrange(zv.R)
    pre = {'inputs': strs(inputs), 'binding_input': str(rng.randrange(zv.R)),
           'private_transcript': [], 'public_transcript_inputs': [], 'public_transcript_outputs': []}
    if seed_pre and seed_pre.get('inputs'):
        # the crate's own test values, when they are literals (manifest test_preimage)
        for key in ('inputs', 'private_transcript', 'public_transcript_outputs', 'binding_input'):
            if key in seed_pre:
                pre[key] = seed_pre[key]
    if doc['do_communications_commitment']:
        pre['communications_commitment'] = ['0', str(rand)]
    passes = 0
    for _ in range(8):
        passes += 1
        res = runner.run(program, pre, gen=True)
        if res['status'] == 'krun-failed':
            raise RuntimeError(res['error'])
        needs = res['needs']
        new_out = [t for kind, t in needs if kind == 'pubOut']
        new_priv = [t for kind, t in needs if kind == 'priv']
        pre['public_transcript_inputs'] = strs(x for kind, x in needs if kind == 'pubIn')
        comm = [x for kind, x in needs if kind == 'comm']
        if comm:
            pre['communications_commitment'] = [str(comm[0]), str(rand)]
        if not new_out and not new_priv:
            break
        pre['public_transcript_outputs'] += strs(x for t in new_out for x in gen(t, rng))
        pre['private_transcript'] += strs(x for t in new_priv for x in gen(t, rng))
    return pre, passes


def compare(k: dict, r: dict) -> list[str]:
    diffs = []
    if k['status'] != r['status']:
        return [f"status K={k['status']} ({k.get('error', '')[:80]}) Rust={r['status']} ({r.get('error', '')[:80]})"]
    if k['status'] in ('error', 'panic'):
        ck, cr = err_class(k.get('error', '')), err_class(r.get('error', ''))
        if ck != cr:
            diffs.append(f"error class K={ck} ('{k.get('error', '')[:60]}') Rust={cr} ('{r.get('error', '')[:60]}')")
        return diffs
    if k['status'] != 'ok':
        return diffs
    if k['pis'] != r['pis']:
        diffs.append(f"pis differ: K={k['pis'][:6]} Rust={r['pis'][:6]}")
    if k['pi_skips'] != r['pi_skips']:
        diffs.append(f"pi_skips differ: K={k['pi_skips']} Rust={r['pi_skips']}")
    km, rm = k['memory'], r['memory']
    for name in sorted(set(km) | set(rm)):
        if name not in km:
            diffs.append(f'{name}: missing in K')
        elif name not in rm:
            diffs.append(f'{name}: missing in Rust')
        else:
            a, b = km[name], rm[name]
            if a['encoded'] != b['encoded'] or a.get('type') != b.get('type'):
                diffs.append(f"{name}: K={a} Rust={b}")
    return diffs


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument('--only', default='')
    ap.add_argument('--seed', type=int, default=2026)
    ap.add_argument('--no-perturb', action='store_true')
    ap.add_argument('--attempts', type=int, default=8)
    ap.add_argument('--ext', action='store_true', help='ZKIR-EXT definition, midnight-zkir 2ffe2d1 oracle and corpus')
    ap.add_argument('--circuit', action='store_true', help='also run the circuit oracle (MockProver) and compare with the comparison table')
    args = ap.parse_args()
    global ORACLE_BIN, CORPORA
    if args.ext:
        ORACLE_BIN = ORACLE_EXT
        CORPORA = {
            'midnight-zkir-2ffe2d1-tests': ROOT / 'corpus' / 'midnight-zkir-2ffe2d1-tests',
            'midnight-zkir-2ffe2d1-precompiles': ROOT / 'corpus' / 'midnight-zkir-2ffe2d1-precompiles',
            'handmade': ROOT / 'corpus' / 'handmade',
        }
    circ_bin = cc.CIRCUIT_ORACLE_EXT if args.ext else cc.CIRCUIT_ORACLE
    circ: dict[int, list] = {}          # row index -> circuit comparisons attached to that row
    circ_findings: list = []            # comparisons outside the table
    circ_ms = 0

    def circuit(row: int, corpus: str, path: Path, label: str, k: dict, pre: dict, inject=None, instance=None, exp=None, detail=''):
        nonlocal circ_ms
        res = cc.run_circuit_oracle(circ_bin, path, pre, inject=inject, instance=instance)
        circ_ms += res.get('elapsed_ms', 0)
        exp = exp or cc.expected_preimage_cell(k)
        verdict = cc.judge(exp, res)
        circ.setdefault(row, []).append((label, cc.k_summary(k), res, exp, verdict, detail))
        if verdict == 'DISAGREE':
            circ_findings.append((corpus, path.name, label, cc.k_summary(k), cc.first_bad(k), res, exp, detail, pre, inject, instance))

    runner = Runner(ext=args.ext)
    rows = []
    failures = 0
    oracle2_flags = []
    ws_rows: list[tuple[str, bool, list[str]]] = []   # (program, witness_space, unconstrained registers) of successful K runs
    t0 = time.time()
    for corpus, directory in CORPORA.items():
        for path in sorted(directory.glob('*.zkir')):
            if args.only and args.only not in path.name:
                continue
            doc = json.loads(path.read_text())
            if doc.get('version', {}).get('major') != 3:
                continue
            try:
                program = zkir_kast.load_program(path, ext=args.ext)
            except zkir_kast.ZkirFormatError as e:
                # the crate's `IrSource::load` must reject it as well
                r = oracle(path, {'inputs': [], 'binding_input': '0'})
                ok = r['status'] == 'load-error'
                failures += not ok
                rows.append((corpus, path.name, 'format', f"K=format-error Rust={r['status']} ({str(e)[:60]})", [] if ok else [f"Rust accepted a program the preprocessor rejects: {r.get('error', '')[:100]}"]))
                continue
            rng = random.Random(f'{args.seed}:{path.name}')
            manifest = directory / 'manifest.json'
            seed_pre = None
            if manifest.exists():
                for entry in json.loads(manifest.read_text()).get('programs', []):
                    if entry['file'] == path.name:
                        seed_pre = entry.get('test_preimage')
            # Up to MAX_ATTEMPTS preimages with small native inputs; stop after the
            # first one both sides accept. Every attempt is compared.
            t1 = time.time()
            for attempt in range(args.attempts):
                try:
                    pre, passes = build_preimage(runner, program, doc, rng, small=True, seed_pre=seed_pre if attempt == 0 else None)
                except RuntimeError as e:
                    rows.append((corpus, path.name, f'gen{attempt}', 'krun failed', [str(e)[-300:]]))
                    failures += 1
                    break
                k = runner.run(program, pre)
                r = oracle(path, pre)
                diffs = compare(k, r)
                failures += bool(diffs)
                note = ''
                if k['status'] == 'error':
                    same = k.get('error', '')[:20] == r.get('error', '')[:20]
                    note = f" msg K='{k.get('error', '')[:70]}'" + ('' if same else f" Rust='{r.get('error', '')[:70]}'")
                viol = k.get('violations', [])
                if k['status'] == 'ok' and viol:
                    oracle2_flags.append((path.name, viol))
                if k['status'] == 'ok':
                    ws_rows.append((path.name, k.get('witness_space', False), k.get('unconstrained', [])))
                vnote = f" verdicts={k.get('verdicts', 0)}" + (f" NON-HOLDING={len(viol)}: " + '; '.join(f'{o}[{m}] {g[:50]}' for o, m, g in viol[:3]) if viol else '')
                if k.get('unconstrained'):
                    vnote += f" witness_space={str(k.get('witness_space', False)).lower()} unconstrained={','.join(k['unconstrained'][:6])}"
                rows.append((corpus, path.name, f'run{attempt}', f"K={k['status']} Rust={r['status']} regs={len(k.get('memory', {}))} pis={len(k.get('pis', []))} passes={passes} {time.time() - t1:.1f}s{note}{vnote}", diffs))
                if args.circuit:
                    run_row = len(rows) - 1
                    circuit(run_row, corpus, path, f'run{attempt}', k, pre)
                if k['status'] == 'ok' and r['status'] == 'ok':
                    if args.circuit and cc.k_summary(k) == 'ok':
                        # the injection and instance columns of the table, on the honest witness
                        crng = random.Random(f'{args.seed}:circuit:{path.name}')
                        for kind, reg, new, exp in cc.choose_injections(doc, k):
                            old = k['memory'][reg]['encoded'][0]
                            circuit(run_row, corpus, path, kind, k, pre, inject={reg: new}, exp=exp, detail=f'{reg} {old} -> {new}; {exp.reason[:110]}')
                        idx, inst, exp = cc.instance_perturbation(k, crng)
                        circuit(run_row, corpus, path, 'instance', k, pre, instance=inst, exp=exp, detail=exp.reason)
                    if not args.no_perturb:
                        variants = []
                        if pre['inputs']:
                            pert = json.loads(json.dumps(pre))
                            pert['inputs'][0] = str(rng.randrange(zv.R))
                            variants.append(('perturbed-raw', pert))
                            # a valid random value of the declared type at a random input position
                            offs, pos = 0, rng.randrange(len(doc['inputs']))
                            for i, entry in enumerate(doc['inputs']):
                                w = zv.ENCODED_LEN.get(entry['type']) or len(zv.random_encoded(entry['type'], rng))
                                if i == pos:
                                    pert2 = json.loads(json.dumps(pre))
                                    pert2['inputs'][offs:offs + w] = strs(zv.random_encoded(entry['type'], rng))
                                    variants.append(('perturbed-typed', pert2))
                                    break
                                offs += w
                        if pre['public_transcript_inputs']:
                            pert3 = json.loads(json.dumps(pre))
                            j = rng.randrange(len(pert3['public_transcript_inputs']))
                            pert3['public_transcript_inputs'][j] = str((int(pert3['public_transcript_inputs'][j]) + 1) % zv.R)
                            variants.append(('wrong-pubin', pert3))
                        if pre.get('communications_commitment'):
                            pert4 = json.loads(json.dumps(pre))
                            pert4['communications_commitment'][0] = str((int(pert4['communications_commitment'][0]) + 1) % zv.R)
                            variants.append(('wrong-comm', pert4))
                        for label, p2 in variants:
                            k2 = runner.run(program, p2)
                            r2 = oracle(path, p2)
                            d2 = compare(k2, r2)
                            failures += bool(d2)
                            rows.append((corpus, path.name, label, f"K={k2['status']} Rust={r2['status']} regs={len(k2.get('memory', {}))}" + (f" msg K='{k2.get('error', '')[:50]}'" if k2['status'] != 'ok' else ''), d2))
                            if args.circuit:
                                circuit(len(rows) - 1, corpus, path, label, k2, p2)
                    break
    for i, (corpus, name, label, summary, diffs) in enumerate(rows):
        print(f"{'PASS' if not diffs else 'FAIL'}  {corpus:34} {name:52} {label:9} {summary}")
        for d in diffs[:8]:
            print(f'        {d}')
        for clabel, ksum, res, exp, verdict, detail in circ.get(i, []):
            print(f"        CIRC  {clabel:16} K={ksum:14} oracle={cc.describe(res):58} expected={exp.cell} [{exp.codes()}] {verdict}" + (f'  {detail}' if detail else ''))
    oks = sum(1 for r in rows if r[3].startswith('K=ok Rust=ok'))
    errs = sum(1 for r in rows if r[3].startswith('K=error Rust=error') or r[3].startswith('K=panic Rust=panic'))
    print(f'\n{len(rows)} comparisons: {oks} successful-run agreements, {errs} error-run agreements (status and error class), {len(rows) - failures} agree, {failures} disagree, {time.time() - t0:.0f}s')
    print(f'oracle 2: {len(oracle2_flags)} successful K runs with a non-holding gate' + (': ' + '; '.join(f'{n} {v[0][0]}[{v[0][1][:40]}]' for n, v in oracle2_flags[:10]) if oracle2_flags else ''))
    in_space = sum(1 for _, ws, _ in ws_rows if ws)
    with_free = [(n, regs) for n, _, regs in ws_rows if regs]
    print(f'witness space: {in_space} of {len(ws_rows)} successful K runs in the modelled witness space, {len(with_free)} with unconstrained registers'
          + (': ' + '; '.join(f"{n} {','.join(regs[:4])}" for n, regs in with_free[:10]) if with_free else ''))
    if args.circuit:
        from collections import Counter
        entries = [e for es in circ.values() for e in es]
        verdicts = Counter(e[4] for e in entries)
        cells = Counter(e[3].cell for e in entries)
        print(f"\ncircuit: {len(entries)} comparisons against the table: {verdicts['AGREE']} agree, {verdicts['DISAGREE']} outside the table, {verdicts['N/C']} not comparable, {circ_ms / 1000:.0f}s in the oracle")
        print('cells hit: ' + '; '.join(f'{cell} x{n}' for cell, n in sorted(cells.items())))
        undecided = [(rows[i][1], e[0], e[5]) for i, es in circ.items() for e in es if e[4] == 'N/C']
        for name, clabel, detail in undecided:
            print(f'  not compared: {name} {clabel}: {detail}')
        if circ_findings:
            print('\nBLOCKING: comparisons outside the table')
            for corpus, name, label, ksum, fb, res, exp, detail, pre, inject, instance in circ_findings:
                print(f'  {corpus}/{name} {label}: K={ksum}' + (f' first non-holding={fb}' if fb else '') + f" oracle={res.get('outcome')} '{res.get('message', '')[:200]}' expected={exp.cell} [{exp.codes()}]" + (f' {detail}' if detail else ''))
                print(f'    preimage={json.dumps(pre, separators=(",", ":"))}')
                if inject:
                    print(f'    inject={json.dumps(inject)}')
                if instance is not None:
                    print(f'    instance={json.dumps(instance, separators=(",", ":"))}')
                if res.get('failures'):
                    print(f"    failures={res['failures'][:4]}")
    return 1 if failures or circ_findings else 0


if __name__ == '__main__':
    sys.exit(main())

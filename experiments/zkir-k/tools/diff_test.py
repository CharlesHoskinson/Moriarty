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
    with tempfile.NamedTemporaryFile('w', suffix='.json', delete=False) as f:
        json.dump(preimage, f)
    res = subprocess.run([str(ORACLE_BIN), str(program), f.name], capture_output=True, text=True)
    if res.returncode != 0:
        return {'status': 'oracle-failed', 'error': res.stderr[-500:]}
    return json.loads(res.stdout)


def strs(xs):
    return [str(x) for x in xs]


def build_preimage(runner: Runner, program, doc: dict, rng: random.Random, small: bool, seed_pre: dict | None = None) -> tuple[dict, int]:
    gen = zv.small_encoded if small else zv.random_encoded
    inputs = [x for entry in doc['inputs'] for x in gen(entry['type'], rng)]
    rand = rng.randrange(zv.R)
    pre = {'inputs': strs(inputs), 'binding_input': str(rng.randrange(zv.R)),
           'private_transcript': [], 'public_transcript_inputs': [], 'public_transcript_outputs': []}
    if seed_pre:
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
        elif km[name] != rm[name]:
            diffs.append(f"{name}: K={km[name]} Rust={rm[name]}")
    return diffs


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument('--only', default='')
    ap.add_argument('--seed', type=int, default=2026)
    ap.add_argument('--no-perturb', action='store_true')
    ap.add_argument('--attempts', type=int, default=8)
    ap.add_argument('--ext', action='store_true', help='ZKIR-EXT definition, midnight-zkir 2ffe2d1 oracle and corpus')
    args = ap.parse_args()
    global ORACLE_BIN, CORPORA
    if args.ext:
        ORACLE_BIN = ORACLE_EXT
        CORPORA = {
            'midnight-zkir-2ffe2d1-tests': ROOT / 'corpus' / 'midnight-zkir-2ffe2d1-tests',
            'midnight-zkir-2ffe2d1-precompiles': ROOT / 'corpus' / 'midnight-zkir-2ffe2d1-precompiles',
            'handmade': ROOT / 'corpus' / 'handmade',
        }
    runner = Runner(ext=args.ext)
    rows = []
    failures = 0
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
            except zkir_kast.ZkirFormatError:
                continue   # rejected before preprocess by both sides
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
                vnote = f" verdicts={k.get('verdicts', 0)}" + (f" NON-HOLDING={len(viol)}: " + '; '.join(f'{o}[{m}] {g[:50]}' for o, m, g in viol[:3]) if viol else '')
                rows.append((corpus, path.name, f'run{attempt}', f"K={k['status']} Rust={r['status']} regs={len(k.get('memory', {}))} pis={len(k.get('pis', []))} passes={passes} {time.time() - t1:.1f}s{note}{vnote}", diffs))
                if k['status'] == 'ok' and r['status'] == 'ok':
                    if not args.no_perturb and pre['inputs']:
                        pert = json.loads(json.dumps(pre))
                        pert['inputs'][0] = str(rng.randrange(zv.R))
                        k2 = runner.run(program, pert)
                        r2 = oracle(path, pert)
                        d2 = compare(k2, r2)
                        failures += bool(d2)
                        rows.append((corpus, path.name, 'perturbed', f"K={k2['status']} Rust={r2['status']} regs={len(k2.get('memory', {}))}", d2))
                    break
    for corpus, name, label, summary, diffs in rows:
        print(f"{'PASS' if not diffs else 'FAIL'}  {corpus:34} {name:52} {label:9} {summary}")
        for d in diffs[:8]:
            print(f'        {d}')
    oks = sum(1 for r in rows if r[3].startswith('K=ok Rust=ok'))
    print(f'\n{len(rows)} comparisons ({oks} successful runs), {len(rows) - failures} agree, {failures} disagree, {time.time() - t0:.0f}s')
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())

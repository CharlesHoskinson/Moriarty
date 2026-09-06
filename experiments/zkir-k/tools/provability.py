#!/usr/bin/env python3
"""Provability receipt for the ZKIR corpus (plan-iter3 M5b).

Runs `zkir-circuit-oracle --keygen` (unknown-witness key generation with the
KZG parameters the crate's `TestParams` provider reads, `bls_midnight_2p{k}`)
on every corpus program whose contract has every obligation met or not
applicable, `--prove` (keygen, prove, verify) on a representative sample, and
`--keygen` on the divergence programs whose contract fails as a negative
control (they must fail with a panic or a synthesis error). One line per
program, then a summary and the list of programs whose keygen outcome
contradicts their contract result.

Contract results come from the JSON-free lines of the contract-corpus receipt
(`evidence/zkir-k-contract-corpus-<date>.txt`); programs absent from it (the
Moriarty contexts, k08) are checked with `zkir_kast.py contract` on the fly.

  uv run --group zkir-k python experiments/zkir-k/tools/provability.py \\
      --out evidence/zkir-k-provability-2026-09-06.txt
"""
from __future__ import annotations

import argparse
import json
import os
import platform
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent  # experiments/zkir-k
REPO = ROOT.parent.parent
CORPUS = ROOT / 'corpus'
SPIKE = HERE / 'circuit-oracle' / 'spike-2026-09-06'
BUILD = Path.home() / 'Moriarty/repos/_build'
ORACLE = {
    'B': BUILD / 'ledger-92e8bdd3/target/release/zkir-circuit-oracle',
    'X': BUILD / 'midnight-zkir-2ffe2d1/target/release/zkir-circuit-oracle',
}
PARAMS_DIR = BUILD / 'params'
PARAMS_URL = 'https://srs.midnight.network/'
DEFAULT_RECEIPT = REPO / 'evidence/zkir-k-contract-corpus-2026-09-06.txt'

# corpus directory -> (contract surface is the extension, oracle binary)
CORPORA = {
    'ledger9-92e8bdd3-tests': (False, 'B'),
    'midnight-zkir-2ffe2d1-precompiles': (False, 'X'),
    'midnight-zkir-2ffe2d1-tests': (True, 'X'),
    'handmade': (False, 'B'),
    'handmade-negative': (False, 'B'),
    'divergence': (False, 'B'),
}
# programs that only the extension surface accepts
SURFACE_OVERRIDE = {'k08_load_constant_jubjub_chip.zkir': (True, 'X')}

# divergence programs whose contract fails and which must fail keygen
NEGATIVE_CONTROLS = [
    'k04_jubjub_scalar_from_native_chip.zkir',
    'k01b_jubjub_from_coordinates_no_chip.zkir',
    'f13_chip_gating_from_bytes32.zkir',
    'k05_less_than_253_bits_keygen.zkir',
    'k07_alignment_option_offcircuit.zkir',
    'k08_load_constant_jubjub_chip.zkir',
]
NEGATIVE_OK = {'panic', 'synthesis-error'}

MORIARTY = {
    'swap': REPO / 'experiments/moriarty-core-swap/output/zkir',
    'escrow': REPO / 'experiments/moriarty-compact-escrow/output/zkir',
}

# the preimages the spike used (program, preimage, binary)
SPIKE_PAIRS = [
    (CORPUS / 'handmade/std_hashes.zkir', SPIKE / 'std_hashes.json', 'B'),
    (CORPUS / 'ledger9-92e8bdd3-tests/test_curve25519_proof.zkir', SPIKE / 'test_curve25519_proof.json', 'B'),
    (CORPUS / 'ledger9-92e8bdd3-tests/test_secp256k1_proof.zkir', SPIKE / 'test_secp256k1_proof.json', 'B'),
    (CORPUS / 'ledger9-92e8bdd3-tests/test_secp256r1_proof.zkir', SPIKE / 'test_secp256r1_proof.json', 'B'),
    (MORIARTY['swap'] / 'expire.zkir', SPIKE / 'expire.json', 'B'),
    (CORPUS / 'midnight-zkir-2ffe2d1-tests/test_curve25519_proof.zkir', SPIKE / 'ext_test_curve25519_proof.json', 'X'),
]


class Receipt:
    def __init__(self, out: Path | None):
        self.lines: list[str] = []
        self.out = out

    def __call__(self, line: str = '') -> None:
        print(line, flush=True)
        self.lines.append(line)

    def save(self) -> None:
        if self.out:
            self.out.write_text('\n'.join(self.lines) + '\n')


def label_of(path: Path) -> str:
    try:
        return str(path.relative_to(CORPUS))
    except ValueError:
        return str(path.relative_to(REPO))


def read_contract_receipt(path: Path) -> dict[str, str]:
    """label -> 'ok (...)' | 'FAILED ...' | 'format: ...'"""
    res = {}
    for line in path.read_text().splitlines():
        if line.startswith('{') or ': ' not in line or not line.split(': ', 1)[0].endswith('.zkir'):
            continue
        label, verdict = line.split(': ', 1)
        res[label] = verdict.replace('   <-- UNEXPECTED', '')
    return res


def contract_via_kast(path: Path, ext: bool) -> str:
    cmd = [sys.executable, str(HERE / 'zkir_kast.py'), 'contract'] + (['--ext'] if ext else []) + [str(path)]
    res = subprocess.run(cmd, capture_output=True, text=True, cwd=REPO)
    if res.returncode != 0 or not res.stdout.lstrip().startswith('{'):
        msg = (res.stdout + res.stderr).strip().splitlines()
        return (msg[-1] if msg else f'exit {res.returncode}')
    obs = json.loads(res.stdout)['obligations']
    failed = [o for o in obs if o['status'] == 'failed']
    if not failed:
        met = sum(o['status'] == 'met' for o in obs)
        na = sum(o['status'] == 'notApplicable' for o in obs)
        return f'ok ({met} met, {na} n/a)'
    return 'FAILED ' + '; '.join(f"{o['name']}: {o['detail']}" for o in failed)


def oracle(binary: str, program: Path, mode: str, preimage: Path | None = None, timeout: int = 1800) -> dict:
    cmd = [str(ORACLE[binary]), str(program)] + ([str(preimage)] if preimage else []) + [mode, '--params', str(PARAMS_DIR)]
    t0 = time.time()
    try:
        res = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return {'outcome': 'timeout', 'message': f'{timeout}s', 'elapsed_ms': int((time.time() - t0) * 1000)}
    if res.returncode != 0:
        return {'outcome': 'load-error', 'message': (res.stderr.strip() or res.stdout.strip())[:300],
                'elapsed_ms': int((time.time() - t0) * 1000)}
    return json.loads(res.stdout.strip().splitlines()[-1])


def fmt(o: dict, keys: tuple[str, ...]) -> str:
    parts = []
    for k in keys:
        if k in o:
            parts.append(f'{k}={o[k]}')
    if o.get('outcome') != 'accepted':
        parts.append('message=' + json.dumps(o.get('message', ''))[:220])
    return ' '.join(parts)


KEYGEN_KEYS = ('k', 'pk_k', 'vk_match', 'elapsed_ms', 'keygen_vk_ms', 'keygen_ms', 'peak_rss_kb')
PROVE_KEYS = ('k', 'n_pis', 'proof_bytes', 'elapsed_ms', 'keygen_ms', 'prove_ms', 'verify_ms', 'peak_rss_kb')


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--contract-receipt', type=Path, default=DEFAULT_RECEIPT)
    ap.add_argument('--out', type=Path, default=None)
    ap.add_argument('--timeout', type=int, default=1800, help='seconds per oracle run')
    args = ap.parse_args(argv)

    r = Receipt(args.out)
    t_start = time.time()
    contracts = read_contract_receipt(args.contract_receipt)

    r('zkir-circuit-oracle provability receipt (plan-iter3 M5b)')
    r(f'Date: {time.strftime("%Y-%m-%d")}')
    r(f'Host: {platform.platform()}, {os.cpu_count()} CPUs')
    r(f'Binaries: B = {ORACLE["B"]}')
    r(f'          X = {ORACLE["X"]}')
    r(f'Parameters: {PARAMS_DIR}/bls_midnight_2p{{k}}, the official Midnight KZG parameters fetched once from')
    r(f'  {PARAMS_URL}bls_midnight_2p{{k}} for k=0..17 (the host midnight-base-crypto downloads from) and verified against')
    r('  the SHA-256 digests in base-crypto/src/data_provider.rs (EXPECTED_DATA); read by the oracle exactly as the crate\'s')
    r('  test provider TestParams reads $MIDNIGHT_PP/bls_midnight_2p{k}. Proofs are verified with the crate\'s embedded')
    r('  PARAMS_VERIFIER (transient-crypto/static/bls_midnight_2p14, same SRS). No unsafe_setup parameters were used.')
    r(f'Contract results: {args.contract_receipt.relative_to(REPO)}; programs absent from it are checked with zkir_kast.py contract (marked computed).')
    r('Columns: keygen = outcome of --keygen (keygen_vk + keygen, unknown witness); prove = outcome of --prove (keygen, prove, VerifierKey::verify).')
    r('Outcomes: accepted | panic | synthesis-error | params-unavailable | load-error | timeout; prove adds preprocess-error | witness-consistency-error | verify-failure.')
    r('  midnight_zk_stdlib::setup_vk unwraps the keygen result ("keygen_vk should not fail"), so an unknown-witness synthesis error')
    r('  surfaces as panic with the Synthesis message; synthesis-error is reserved for an Err returned by keygen_vk/keygen/prove.')
    r('  optimal_k returns the cost-model k, which is below 9 for the small crate tests, so the parameter files k=0..17 are all needed.')
    r()

    findings: list[str] = []
    keygen_counts: dict[str, int] = {}
    prove_counts: dict[str, int] = {}
    not_keyed = 0

    # ---- keygen sweep over the corpus ----
    r('== keygen sweep: every corpus program whose contract has every obligation met or not applicable ==')
    negative_results: dict[str, dict] = {}
    for name, (ext, binary) in CORPORA.items():
        for path in sorted((CORPUS / name).glob('*.zkir')):
            label = label_of(path)
            ext, binary = SURFACE_OVERRIDE.get(path.name, (ext, binary))
            computed = ''
            verdict = contracts.get(label)
            if verdict is None:
                verdict = contract_via_kast(path, ext)
                computed = ' (computed)'
            ok = verdict.startswith('ok')
            if not ok:
                not_keyed += 1
                if path.name in NEGATIVE_CONTROLS:
                    o = oracle(binary, path, '--keygen', timeout=args.timeout)
                    negative_results[path.name] = o
                    r(f'{label} [{binary}]: contract {verdict[:90]}{computed}; negative control keygen={o["outcome"]} {fmt(o, KEYGEN_KEYS)}')
                    if o['outcome'] == 'accepted':
                        findings.append(f'{label}: contract fails but keygen accepted')
                else:
                    r(f'{label} [{binary}]: contract {verdict[:90]}{computed}; not keyed')
                continue
            o = oracle(binary, path, '--keygen', timeout=args.timeout)
            keygen_counts[o['outcome']] = keygen_counts.get(o['outcome'], 0) + 1
            r(f'{label} [{binary}]: contract {verdict}{computed}; keygen={o["outcome"]} {fmt(o, KEYGEN_KEYS)}')
            if o['outcome'] not in ('accepted', 'params-unavailable'):
                findings.append(f'{label}: contract {verdict} but keygen {o["outcome"]}: {o.get("message", "")[:200]}')
            elif o['outcome'] == 'accepted' and o.get('pk_k') != o.get('k'):
                findings.append(f'{label}: pk k {o.get("pk_k")} differs from optimal_k {o.get("k")}')
            elif o['outcome'] == 'accepted' and o.get('vk_match') is not True:
                findings.append(f'{label}: keygen_vk and keygen produced different verifier keys')
    r()

    # ---- Moriarty contexts: keygen and prove ----
    r('== Moriarty contexts (corpus/moriarty-contexts/*.pre.json): contract, keygen, prove-and-verify ==')
    manifest = json.loads((CORPUS / 'moriarty-contexts/manifest.json').read_text())
    for art, info in manifest['artifacts'].items():
        family = art.split('-', 1)[0]
        program = MORIARTY[family] / f'{info["circuit"]}.zkir'
        pre = CORPUS / 'moriarty-contexts' / f'{art}.pre.json'
        verdict = contract_via_kast(program, False)
        ok = verdict.startswith('ok')
        kg = oracle('B', program, '--keygen', timeout=args.timeout)
        keygen_counts[kg['outcome']] = keygen_counts.get(kg['outcome'], 0) + 1
        pv = oracle('B', program, '--prove', pre, timeout=args.timeout)
        prove_counts[pv['outcome']] = prove_counts.get(pv['outcome'], 0) + 1
        r(f'{art} ({label_of(program)}) [B]: contract {verdict} (computed); keygen={kg["outcome"]} {fmt(kg, KEYGEN_KEYS)}')
        r(f'    prove={pv["outcome"]} {fmt(pv, PROVE_KEYS)}')
        if ok and kg['outcome'] not in ('accepted', 'params-unavailable'):
            findings.append(f'{art}: contract {verdict} but keygen {kg["outcome"]}: {kg.get("message", "")[:200]}')
        if not ok and kg['outcome'] == 'accepted':
            findings.append(f'{art}: contract fails but keygen accepted')
        if pv['outcome'] not in ('accepted', 'params-unavailable'):
            findings.append(f'{art}: prove-and-verify {pv["outcome"]}: {pv.get("message", "")[:200]}')
    r()

    # ---- spike preimages: prove ----
    r('== handmade and crate-test programs with the preimages of the spike: prove-and-verify ==')
    for program, pre, binary in SPIKE_PAIRS:
        pv = oracle(binary, program, '--prove', pre, timeout=args.timeout)
        prove_counts[pv['outcome']] = prove_counts.get(pv['outcome'], 0) + 1
        r(f'{label_of(program)} + {pre.name} [{binary}]: prove={pv["outcome"]} {fmt(pv, PROVE_KEYS)}')
        if pv['outcome'] not in ('accepted', 'params-unavailable'):
            findings.append(f'{label_of(program)} + {pre.name}: prove-and-verify {pv["outcome"]}: {pv.get("message", "")[:200]}')
    r()

    # ---- negative controls ----
    r('== negative controls: divergence programs whose contract fails must fail keygen (panic or synthesis-error) ==')
    neg_pass = 0
    for name in NEGATIVE_CONTROLS:
        o = negative_results.get(name)
        if o is None:
            r(f'divergence/{name}: NOT RUN (contract did not fail; see the sweep above)')
            findings.append(f'divergence/{name}: named as a negative control but its contract does not fail')
            continue
        good = o['outcome'] in NEGATIVE_OK
        neg_pass += good
        r(f'divergence/{name}: keygen={o["outcome"]} -> {"as expected" if good else "UNEXPECTED"}; {json.dumps(o.get("message", ""))[:200]}')
    extra = [n for n in negative_results if n not in NEGATIVE_CONTROLS]
    for name in extra:
        o = negative_results[name]
        r(f'divergence/{name} (contract fails, not in the named set): keygen={o["outcome"]}; {json.dumps(o.get("message", ""))[:200]}')
    r()

    # ---- summary ----
    kc = ', '.join(f'{v} {k}' for k, v in sorted(keygen_counts.items()))
    pc = ', '.join(f'{v} {k}' for k, v in sorted(prove_counts.items()))
    summary = (f'{sum(keygen_counts.values())} programs keyed ({kc}); {not_keyed} not keyed (contract not met); '
               f'{sum(prove_counts.values())} proved and verified ({pc}); '
               f'negative controls {neg_pass}/{len(NEGATIVE_CONTROLS)} failed keygen as expected; '
               f'{len(findings)} contradictions; {time.time() - t_start:.0f}s')
    r(summary)
    if findings:
        r('Contradictions between contract and keygen/prove (findings for the contract):')
        for f in findings:
            r(f'  - {f}')
        r('  Each is a program whose contract obligations are all met or not applicable while the circuit cannot be keyed: the')
        r('  in-circuit checks it fails (output signature vs operand type, gate arity, constant decoding, nth/slice bounds, the')
        r('  unsupported in-circuit arms of cond_select/constrain_eq/bytes32_from_low_high/ec_mul_generator) have no obligation yet.')
    else:
        r('No program contradicts its contract result.')
    r.save()
    return 0 if not findings and neg_pass == len(NEGATIVE_CONTROLS) else 1


if __name__ == '__main__':
    sys.exit(main())

#!/usr/bin/env python3
"""Provability receipt for the ZKIR corpus (plan-iter3 M5b; stage-aware since
the 2026-09-06 review, item 21).

Runs `zkir-circuit-oracle --keygen` (unknown-witness key generation with the
KZG parameters the crate's `TestParams` provider reads, `bls_midnight_2p{k}`)
on EVERY parseable corpus program, whatever its contract says, and compares
the outcome with what the contract's stage tags predict:

  every obligation met or not applicable          -> keygen must accept  (class `clean`)
  a failed obligation of stage keygen or both     -> keygen must reject, by a panic or a
                                                     synthesis error whose message names
                                                     the failed check       (class `must-reject`)
  failed obligations of stage preprocess only     -> keygen must accept: the program is
                                                     rejected on every preimage by
                                                     `IrSource::preprocess`, not by keygen
                                                     (class `keyed-despite-preprocess`)
  failed obligations of stage static only (`wf`)  -> the K static layer is strictly stricter
                                                     than keygen and makes no keygen claim;
                                                     keygen must accept unless another failed
                                                     obligation says otherwise
                                                     (class `keyed-despite-static`)

A disagreement in either direction is a contradiction and a finding for the
contract: a program the contract lets key that keygen rejects (the contract is
too weak), or a program the contract says keygen rejects that keys (the
contract is too strict), or a rejection on a check other than the one the
failed obligation names (item 24: the oracle message must name the check,
matched by the Rust text of the obligation's detail or by a per-obligation
rule; the receipt says which). "0 contradictions" is therefore a statement
about every program, not about a curated list.

`--prove` (keygen, prove, VerifierKey::verify) runs on the seven Moriarty
contexts and on the spike preimages. What that tier establishes and what it
does not (item 13) is stated in the receipt header: it is not ledger
acceptance.

Contract results come from the lines of the contract-corpus receipt
(`evidence/zkir-k-contract-corpus-<stamp>.txt`, `FAILED name[stage]: detail`);
programs absent from it (the Moriarty contexts) are checked with
`zkir_kast.py contract` on the fly, which also reports the `ledger.commitment`
fact.

  uv run --group zkir-k python experiments/zkir-k/tools/provability.py \\
      --contract-receipt evidence/zkir-k-contract-corpus-2026-09-06d.txt \\
      --out evidence/zkir-k-provability-2026-09-06d.txt
"""
from __future__ import annotations

import argparse
import json
import os
import platform
import re
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
DEFAULT_RECEIPT = REPO / 'evidence/zkir-k-contract-corpus-2026-09-06d.txt'

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

# The programs the 2026-09-06 review names, as an explicit cross-check of the
# stage-driven classification: each must be classified `must-reject` by its
# contract and fail keygen on the check its failed obligation names.
NEGATIVE_CONTROLS = [
    # chip gating, width and alignment (M5a)
    'divergence/k04_jubjub_scalar_from_native_chip.zkir',
    'divergence/k01b_jubjub_from_coordinates_no_chip.zkir',
    'divergence/f13_chip_gating_from_bytes32.zkir',
    'divergence/k05_less_than_253_bits_keygen.zkir',
    'divergence/k07_alignment_option_offcircuit.zkir',
    'divergence/k08_load_constant_jubjub_chip.zkir',
    # the witness-independent in-circuit checks (circuit.static.*, M5b)
    'ledger9-92e8bdd3-tests/output_operand_type_mismatch.zkir',
    'midnight-zkir-2ffe2d1-tests/output_operand_type_mismatch.zkir',
    'midnight-zkir-2ffe2d1-tests/test_bool_gate_empty_inputs_fails.zkir',
    'midnight-zkir-2ffe2d1-tests/test_constant_bad_encoding_rejected.zkir',
    'midnight-zkir-2ffe2d1-tests/test_nth_out_of_bounds_fails.zkir',
    'midnight-zkir-2ffe2d1-tests/test_slice_out_of_bounds_fails.zkir',
    'divergence/f06_cond_select_bytes32.zkir',
    'divergence/f07_constrain_eq_jubjub_scalar.zkir',
    'divergence/f08_bytes32_from_low_high_foreign_high.zkir',
    'divergence/f12_ec_mul_generator_p256.zkir',
    'divergence/e11_violated_then_synth.zkir',
    # the alignment field count (item 19) and reconstitute_field bits = 0 (item 20)
    'handmade-negative/align_field_short.zkir',
    'handmade-negative/align_bytes_short.zkir',
    'handmade-negative/reconstitute_bits_0.zkir',
    # the three stage mis-tags of the 2026-09-06d sweep, closed by the both-stage
    # obligations circuit.static.defined, circuit.static.div_mod_outputs and
    # width.reconstitute_field.assertion
    'handmade-negative/undefined_variable.zkir',
    'handmade-negative/divmod_outputs.zkir',
    'divergence/f10_reconstitute_bits_256.zkir',
]
# programs the review names as keyed with a preprocess-stage (or K-only) failure (item 21)
KEYED_DESPITE = [
    'handmade-negative/reassignment.zkir',
    'handmade-negative/excessive_bits.zkir',
    'divergence/e21a_div_mod_249.zkir',
    'divergence/e21b_reconstitute_249.zkir',
]
REJECT_OUTCOMES = {'panic', 'synthesis-error'}

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


# --- contract results -----------------------------------------------------------------------

FAILED_RE = re.compile(r'([A-Za-z0-9_.]+)\[([a-z?]+)\]: (.*)')


def parse_failed(verdict: str) -> list[tuple[str, str, str]]:
    """`FAILED name[stage]: detail; name[stage]: detail` -> [(name, stage, detail)]."""
    if not verdict.startswith('FAILED '):
        return []
    body = verdict[len('FAILED '):]
    out = []
    # details never contain "; " followed by an obligation name and a stage
    for part in re.split(r'; (?=[A-Za-z0-9_.]+\[[a-z?]+\]: )', body):
        m = FAILED_RE.match(part)
        if m:
            out.append((m.group(1), m.group(2), m.group(3)))
        else:
            out.append((part.split(':', 1)[0], '?', part))
    return out


def read_contract_receipt(path: Path) -> dict[str, str]:
    """label -> 'ok (...)' | 'FAILED ...' | 'format: ...' | 'krun failed: ...'"""
    res = {}
    for line in path.read_text().splitlines():
        if line.startswith('{') or ': ' not in line or not line.split(': ', 1)[0].endswith('.zkir'):
            continue
        label, verdict = line.split(': ', 1)
        res[label] = verdict.replace('   <-- UNEXPECTED', '')
    return res


def contract_via_kast(path: Path, ext: bool) -> tuple[str, dict]:
    """(verdict line in the receipt's format, the full contract document)."""
    cmd = [sys.executable, str(HERE / 'zkir_kast.py'), 'contract'] + (['--ext'] if ext else []) + [str(path)]
    res = subprocess.run(cmd, capture_output=True, text=True, cwd=REPO)
    if res.returncode != 0 or not res.stdout.lstrip().startswith('{'):
        msg = (res.stdout + res.stderr).strip().splitlines()
        return (msg[-1] if msg else f'exit {res.returncode}'), {}
    doc = json.loads(res.stdout)
    obs = doc['obligations']
    failed = [o for o in obs if o['status'] == 'failed']
    if not failed:
        met = sum(o['status'] == 'met' for o in obs)
        na = sum(o['status'] == 'notApplicable' for o in obs)
        return f'ok ({met} met, {na} n/a)', doc
    return 'FAILED ' + '; '.join(f"{o['name']}[{o.get('stage') or '?'}]: {o['detail']}" for o in failed), doc


def classify(verdict: str) -> tuple[str, list[tuple[str, str, str]]]:
    """The class the stage tags predict: clean | must-reject |
    keyed-despite-preprocess | keyed-despite-static | not-parseable | unknown."""
    if verdict.startswith('format'):
        return 'not-parseable', []
    if not (verdict.startswith('ok') or verdict.startswith('FAILED')):
        return 'unknown', []
    failed = parse_failed(verdict)
    if not failed:
        return 'clean', []
    stages = {s for _, s, _ in failed}
    if stages & {'keygen', 'both'}:
        return 'must-reject', failed
    if stages & {'?'}:
        return 'unknown', failed
    if stages & {'preprocess'}:
        return 'keyed-despite-preprocess', failed
    return 'keyed-despite-static', failed


# --- item 24: does the oracle message name the failed check? ---------------------------------

def _norm(s: str) -> str:
    s = s.replace('\\', '').replace('"', '').replace("'", '').lower()
    s = s.replace('static type', 'type').replace('runtime type', 'type')
    return ' '.join(s.split())


def names_check(name: str, detail: str, message: str) -> str | None:
    """How the oracle message names the failed obligation, or None."""
    d, m = _norm(detail), _norm(message)
    # the Rust text of the detail: strip "instruction N: " / "instruction N (op): " / "op: "
    core = re.sub(r'^instruction \d+( \([a-z0-9_]+\))?: ', '', d)
    core_no_op = re.sub(r'^[a-z0-9_]+( [a-z]+)?: ', '', core)   # "op: " or "op operand: "
    for cand in (core, core_no_op):
        if len(cand) >= 12 and cand in m:
            return 'Rust text of the detail'
    if name == 'chips.gating':
        mm = re.search(r'needs chip ([a-z0-9]+)', d)
        if mm and f'must enable {mm.group(1)}' in m:
            return f'chip {mm.group(1)}'
    if name.startswith('width.'):
        nums = re.findall(r'\b(\d+)\b', core)
        if 'cannot bound' in m and any(f'bound {n} ' in m for n in nums):
            return 'padded bound'
        if 'assertion failed' in d and 'assertion failed' in m and 'num_bits' in m:
            return 'the crate assertion'
    if name.startswith('alignment.') and 'option' in d and 'alignment options' in m:
        return 'option segment'
    if name == 'circuit.static.constant' and 'failed to decode' in d and 'failed to decode' in m:
        ty = re.search(r'as ([a-z0-9]+)$', core)
        if ty and f'as {ty.group(1)}' in m:
            return 'decode failure and type'
    toks = [t for t in re.findall(r'[a-z0-9_]+', core_no_op) if len(t) >= 4 and t != 'instruction']
    if toks and sum(t in m for t in toks) >= max(2, (len(toks) * 3 + 3) // 4):
        return 'significant words of the detail'
    return None


# --- oracle ----------------------------------------------------------------------------------

def oracle(binary: str, program: Path, mode: str, preimage: Path | None = None, timeout: int = 1800) -> dict:
    cmd = [str(ORACLE[binary]), str(program)] + ([str(preimage)] if preimage else []) + [mode, '--params', str(PARAMS_DIR)]
    t0 = time.time()
    try:
        res = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return {'outcome': 'timeout', 'message': f'{timeout}s', 'elapsed_ms': int((time.time() - t0) * 1000)}
    if res.returncode != 0:
        last = res.stdout.strip().splitlines()[-1] if res.stdout.strip() else ''
        if last.startswith('{'):
            try:
                return json.loads(last)
            except ValueError:
                pass
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


def header(r: Receipt, receipt_path: Path) -> None:
    r('zkir-circuit-oracle provability receipt (plan-iter3 M5b; stage-aware sweep, 2026-09-06 review item 21)')
    r(f'Date: {time.strftime("%Y-%m-%d")}')
    r(f'Host: {platform.platform()}, {os.cpu_count()} CPUs')
    r(f'Binaries: B = {ORACLE["B"]}')
    r(f'          X = {ORACLE["X"]}')
    r(f'Parameters: {PARAMS_DIR}/bls_midnight_2p{{k}}, the official Midnight KZG parameters fetched once from')
    r(f'  {PARAMS_URL}bls_midnight_2p{{k}} for k=0..17 (the host midnight-base-crypto downloads from) and verified against')
    r('  the SHA-256 digests in base-crypto/src/data_provider.rs (EXPECTED_DATA); read by the oracle exactly as the crate\'s')
    r('  test provider TestParams reads $MIDNIGHT_PP/bls_midnight_2p{k}. Proofs are verified with the crate\'s embedded')
    r('  PARAMS_VERIFIER (transient-crypto/static/bls_midnight_2p14, same SRS). No unsafe_setup parameters were used.')
    try:
        rel = receipt_path.relative_to(REPO)
    except ValueError:
        rel = receipt_path
    r(f'Contract results: {rel} (FAILED lines carry name[stage]: detail); programs absent from it are checked with')
    r('  zkir_kast.py contract (marked computed), which also reports the ledger.commitment fact.')
    r('Columns: keygen = outcome of --keygen (keygen_vk + keygen, unknown witness); prove = outcome of --prove (keygen, prove, VerifierKey::verify).')
    r('Outcomes: accepted | panic | synthesis-error | params-unavailable | load-error | timeout; prove adds preprocess-error | witness-consistency-error | verify-failure.')
    r('  midnight_zk_stdlib::setup_vk unwraps the keygen result ("keygen_vk should not fail"), so an unknown-witness synthesis error')
    r('  surfaces as panic with the Synthesis message; synthesis-error is reserved for an Err returned by keygen_vk/keygen/prove.')
    r('  optimal_k returns the cost-model k, which is below 9 for the small crate tests, so the parameter files k=0..17 are all needed.')
    r('Stage rule: every parseable program is keyed. A failed obligation of stage keygen or both predicts a keygen rejection whose')
    r('  message names the check; failed obligations of stage preprocess only predict that keygen accepts (the program is rejected')
    r('  on every preimage by IrSource::preprocess instead); a failed wf (stage static, the K static layer, strictly stricter than')
    r('  keygen) makes no keygen claim on its own. A program whose keygen outcome differs from its class is a contradiction, in')
    r('  either direction; a rejection on a check other than the named one is a contradiction too.')
    r('What the prove-and-verify tier establishes (item 13): keygen with the official SRS, Zkir::prove with a fixed seed, and')
    r('  VerifierKey::verify of the proof against the public inputs `prove` itself returned. It is NOT ledger acceptance. Residuals:')
    r('  (1) statement vector: the ledger verifies against [binding_input, communications_commitment, field_repr(guaranteed),')
    r('      field_repr(fallible)] (ledger/src/verify.rs:1956-1970), assembled by the ledger from the transaction, not the `pis`')
    r('      returned by prove; (2) unconditional commitment: the ledger pushes the commitment always, so a program without')
    r('      do_communications_commitment can never satisfy a ledger statement (reported below as the ledger.commitment fact,')
    r('      not as a failed obligation: the program keys and proves); (3) key identity: the verifier key is regenerated here')
    r('      from the program and never compared with a deployed key (`--vk FILE` is specified for when key files exist, D2).')
    r()


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--contract-receipt', type=Path, default=DEFAULT_RECEIPT)
    ap.add_argument('--out', type=Path, default=None)
    ap.add_argument('--timeout', type=int, default=1800, help='seconds per oracle run')
    args = ap.parse_args(argv)

    r = Receipt(args.out)
    t_start = time.time()
    contracts = read_contract_receipt(args.contract_receipt)
    header(r, args.contract_receipt)

    findings: list[str] = []
    keygen_counts: dict[str, int] = {}
    class_counts: dict[str, int] = {}
    prove_counts: dict[str, int] = {}
    not_parseable = 0
    classes: dict[str, str] = {}
    results: dict[str, dict] = {}
    named: dict[str, str | None] = {}

    # ---- keygen sweep over every parseable corpus program ----
    r('== keygen sweep: every parseable corpus program, classified by the stage tags of its failed obligations ==')
    for name, (ext, binary) in CORPORA.items():
        for path in sorted((CORPUS / name).glob('*.zkir')):
            label = label_of(path)
            ext, binary = SURFACE_OVERRIDE.get(path.name, (ext, binary))
            computed = ''
            verdict = contracts.get(label)
            if verdict is None:
                verdict, _ = contract_via_kast(path, ext)
                computed = ' (computed)'
            cls, failed = classify(verdict)
            classes[label] = cls
            class_counts[cls] = class_counts.get(cls, 0) + 1
            if cls == 'not-parseable':
                not_parseable += 1
                o = oracle(binary, path, '--keygen', timeout=args.timeout)
                r(f'{label} [{binary}]: contract {verdict[:90]}{computed}; class not-parseable; keygen={o["outcome"]} {fmt(o, KEYGEN_KEYS)}')
                if o['outcome'] != 'load-error':
                    findings.append(f'{label}: the preprocessor rejects the file but IrSource::load accepted it (keygen {o["outcome"]})')
                continue
            if cls == 'unknown':
                r(f'{label} [{binary}]: contract {verdict[:120]}{computed}; class unknown (not keyed)')
                findings.append(f'{label}: contract result without stage information: {verdict[:120]}')
                continue
            o = oracle(binary, path, '--keygen', timeout=args.timeout)
            results[label] = o
            keygen_counts[o['outcome']] = keygen_counts.get(o['outcome'], 0) + 1
            rejected = o['outcome'] in REJECT_OUTCOMES
            note = ''
            if cls == 'must-reject':
                how = None
                for oname, stage, detail in failed:
                    if stage in ('keygen', 'both'):
                        how = names_check(oname, detail, o.get('message', ''))
                        if how:
                            named[label] = f'{oname}: {how}'
                            break
                if not rejected and o['outcome'] != 'params-unavailable':
                    findings.append(f'{label}: contract says keygen rejects ({"; ".join(f"{n}[{s}]" for n, s, _ in failed if s in ("keygen", "both"))}) but keygen {o["outcome"]} (contract too strict)')
                    note = '  <-- CONTRADICTION (keys)'
                elif rejected and how is None:
                    findings.append(f'{label}: keygen rejects but its message does not name the failed check ({"; ".join(f"{n}: {d[:60]}" for n, s, d in failed if s in ("keygen", "both"))}): {o.get("message", "")[:160]}')
                    note = '  <-- rejected on another check'
                elif rejected:
                    note = f'  names the check ({named[label]})'
            else:
                if rejected:
                    what = 'no failed obligation' if cls == 'clean' else 'failed obligations of stage ' + ', '.join(sorted({s for _, s, _ in failed}))
                    findings.append(f'{label}: contract lets the program key ({what}) but keygen {o["outcome"]}: {o.get("message", "")[:160]} (contract too weak)')
                    note = '  <-- CONTRADICTION (rejected)'
                elif o['outcome'] not in ('accepted', 'params-unavailable'):
                    findings.append(f'{label}: keygen {o["outcome"]}: {o.get("message", "")[:160]}')
                    note = '  <-- not keyed'
                elif o['outcome'] == 'accepted' and o.get('pk_k') != o.get('k'):
                    findings.append(f'{label}: pk k {o.get("pk_k")} differs from optimal_k {o.get("k")}')
                elif o['outcome'] == 'accepted' and o.get('vk_match') is not True:
                    findings.append(f'{label}: keygen_vk and keygen produced different verifier keys')
            r(f'{label} [{binary}]: contract {verdict[:110]}{computed}; class {cls}; keygen={o["outcome"]} {fmt(o, KEYGEN_KEYS)}{note}')
    r()

    # ---- Moriarty contexts: keygen and prove ----
    r('== Moriarty contexts (corpus/moriarty-contexts/*.pre.json): contract, ledger.commitment fact, keygen, prove-and-verify ==')
    manifest = json.loads((CORPUS / 'moriarty-contexts/manifest.json').read_text())
    for art, info in manifest['artifacts'].items():
        family = art.split('-', 1)[0]
        program = MORIARTY[family] / f'{info["circuit"]}.zkir'
        pre = CORPUS / 'moriarty-contexts' / f'{art}.pre.json'
        verdict, doc = contract_via_kast(program, False)
        cls, failed = classify(verdict)
        ledger = doc.get('ledger', {}).get('commitment', {})
        kg = oracle('B', program, '--keygen', timeout=args.timeout)
        keygen_counts[kg['outcome']] = keygen_counts.get(kg['outcome'], 0) + 1
        pv = oracle('B', program, '--prove', pre, timeout=args.timeout)
        prove_counts[pv['outcome']] = prove_counts.get(pv['outcome'], 0) + 1
        r(f'{art} ({label_of(program)}) [B]: contract {verdict} (computed); class {cls}; ledger.commitment {"met" if ledger.get("met") else "NOT MET"}'
          f' ({ledger.get("detail", "?")[:80]}); keygen={kg["outcome"]} {fmt(kg, KEYGEN_KEYS)}')
        r(f'    prove={pv["outcome"]} {fmt(pv, PROVE_KEYS)}')
        if cls in ('clean', 'keyed-despite-preprocess', 'keyed-despite-static') and kg['outcome'] not in ('accepted', 'params-unavailable'):
            findings.append(f'{art}: contract lets the program key but keygen {kg["outcome"]}: {kg.get("message", "")[:160]}')
        if cls == 'must-reject' and kg['outcome'] == 'accepted':
            findings.append(f'{art}: contract says keygen rejects but keygen accepted')
        if pv['outcome'] not in ('accepted', 'params-unavailable'):
            findings.append(f'{art}: prove-and-verify {pv["outcome"]}: {pv.get("message", "")[:160]}')
        if not ledger.get('met'):
            findings.append(f'{art}: presented as a contract entry point without the communications commitment (ledger.commitment not met)')
    r()

    # ---- spike preimages: prove ----
    r('== handmade and crate-test programs with the preimages of the spike: prove-and-verify ==')
    for program, pre, binary in SPIKE_PAIRS:
        pv = oracle(binary, program, '--prove', pre, timeout=args.timeout)
        prove_counts[pv['outcome']] = prove_counts.get(pv['outcome'], 0) + 1
        r(f'{label_of(program)} + {pre.name} [{binary}]: prove={pv["outcome"]} {fmt(pv, PROVE_KEYS)}')
        if pv['outcome'] not in ('accepted', 'params-unavailable'):
            findings.append(f'{label_of(program)} + {pre.name}: prove-and-verify {pv["outcome"]}: {pv.get("message", "")[:160]}')
    r()

    # ---- the review's named programs as a cross-check of the classification ----
    r('== negative controls named by the review and by the 2026-09-06d sweep: class must-reject, keygen fails on the named check ==')
    neg_pass = 0
    for name in NEGATIVE_CONTROLS:
        cls = classes.get(name)
        o = results.get(name)
        if cls is None:
            r(f'{name}: NOT IN THE CORPUS')
            findings.append(f'{name}: named as a negative control but not in the corpus')
            continue
        if cls != 'must-reject':
            r(f'{name}: class {cls} -> UNEXPECTED (its contract does not carry a keygen- or both-stage failure)')
            findings.append(f'{name}: named as a negative control but its contract does not predict a keygen rejection (class {cls})')
            continue
        good = o is not None and o['outcome'] in REJECT_OUTCOMES and named.get(name) is not None
        neg_pass += good
        r(f'{name}: keygen={o["outcome"] if o else "?"} -> {"as expected, " + named[name] if good else "UNEXPECTED"}; {json.dumps((o or {}).get("message", ""))[:160]}')
    r()
    r('== programs the review names as keyed despite a failed preprocess-stage or K-only obligation ==')
    keyed_pass = 0
    for name in KEYED_DESPITE:
        cls = classes.get(name)
        o = results.get(name)
        if cls is None:
            r(f'{name}: NOT IN THE CORPUS')
            findings.append(f'{name}: named as keyed-despite but not in the corpus')
            continue
        good = cls.startswith('keyed-despite') and o is not None and o['outcome'] == 'accepted'
        keyed_pass += good
        r(f'{name}: class {cls}, keygen={o["outcome"] if o else "?"} -> {"as expected" if good else "UNEXPECTED"}')
    r()

    # ---- summary ----
    kc = ', '.join(f'{v} {k}' for k, v in sorted(keygen_counts.items()))
    cc = ', '.join(f'{v} {k}' for k, v in sorted(class_counts.items()))
    pc = ', '.join(f'{v} {k}' for k, v in sorted(prove_counts.items()))
    summary = (f'{sum(keygen_counts.values())} programs keyed ({kc}); classes: {cc}; {not_parseable} not parseable (load-error on both sides expected); '
               f'{sum(prove_counts.values())} proved and verified ({pc}); '
               f'negative controls {neg_pass}/{len(NEGATIVE_CONTROLS)} failed keygen on the named check; '
               f'keyed-despite controls {keyed_pass}/{len(KEYED_DESPITE)} keyed; '
               f'{len(findings)} contradictions; {time.time() - t_start:.0f}s')
    r(summary)
    if findings:
        r('Contradictions between the contract\'s stage tags and keygen/prove (findings for the contract), in both directions:')
        for f in findings:
            r(f'  - {f}')
        r('  "contract too weak": the stage tags let the program key while keygen rejects it (a keygen check the contract does not')
        r('  decide, or an obligation tagged preprocess/static that keygen also performs); "contract too strict": the tags predict a')
        r('  rejection that keygen does not perform; "rejected on another check": the rejection is real but not the named one.')
    else:
        r('No program contradicts its stage tags: every program with no keygen- or both-stage failure keys, every program with one')
        r('is rejected by keygen on the check the obligation names, and every prove-and-verify run verified.')
    r.save()
    return 0 if not findings and neg_pass == len(NEGATIVE_CONTROLS) and keyed_pass == len(KEYED_DESPITE) else 1


if __name__ == '__main__':
    sys.exit(main())

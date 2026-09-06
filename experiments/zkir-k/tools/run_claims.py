"""Run the kprove claims of experiments/zkir-k/claims against the Haskell
definition zkir-symbolic-kompiled and print one line per claim with the
result and the wall time in seconds.

Usage (repository root):
  uv run --group zkir-k python experiments/zkir-k/tools/run_claims.py [--definition DIR]
      [--timeout SECONDS] [--only NAME,...] [--log-dir DIR]

The definition is built with
  cd experiments/zkir-k/semantics && kompile zkir-symbolic.k --backend haskell

Each claim is one K module; kprove is invoked once per module. A claim is
"proved" when kprove exits 0 and prints #Top; otherwise the line says
"not proved" with the reason (kprove error, timeout, or a residual goal) and the
kprove output is kept under --log-dir.

The compiler-obligation template `spec_compiled_observable` writes the claim
module for an observable-semantics obligation: after job(P, Pre) on the program
P and the preimage Pre, the <status>, <outputs> and <pi> cells hold the expected
(status, outputs, pis). The runner instantiates it once
(claims/spec-compiled-observable.k) before running.
"""
from __future__ import annotations

import argparse
import subprocess
import sys
import tempfile
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
CLAIMS = HERE.parent / 'claims'
DEFAULT_DEFINITION = HERE.parent / 'semantics' / 'zkir-symbolic-kompiled'

# (name, file, module), in the order of the receipt
CLAIM_LIST: list[tuple[str, str, str]] = [
    ('add', 'add-spec.k', 'ADD-SPEC'),
    ('mul', 'native-ops-spec.k', 'MUL-SPEC'),
    ('neg', 'native-ops-spec.k', 'NEG-SPEC'),
    ('copy', 'native-ops-spec.k', 'COPY-SPEC'),
    ('cond_select-1', 'native-ops-spec.k', 'COND-SELECT-1-SPEC'),
    ('cond_select-0', 'native-ops-spec.k', 'COND-SELECT-0-SPEC'),
    ('constrain_to_boolean-ok', 'native-ops-spec.k', 'CONSTRAIN-TO-BOOLEAN-OK-SPEC'),
    ('constrain_to_boolean-fail', 'native-ops-spec.k', 'CONSTRAIN-TO-BOOLEAN-FAIL-SPEC'),
    ('assert-ok', 'native-ops-spec.k', 'ASSERT-OK-SPEC'),
    ('assert-fail', 'native-ops-spec.k', 'ASSERT-FAIL-SPEC'),
    ('transient_hash', 'transient-hash-spec.k', 'TRANSIENT-HASH-SPEC'),
    ('spec_compiled_observable', 'spec-compiled-observable.k', 'SPEC-COMPILED-OBSERVABLE'),
]


# --- the compiler-obligation template -----------------------------------------------

OTHER_CELLS = """\
        <mem> .Map => ?_ </mem>
        <skips> .List => ?_ </skips>
        <pubInIdx> 0 => ?_ </pubInIdx> <pubOutIdx> 0 => ?_ </pubOutIdx> <privIdx> 0 => ?_ </privIdx>
        <outTypes> .IrTypes => ?_ </outTypes>
        <doComm> false => ?_ </doComm>
        <pre> preimage(.List, 0, noComm(), .List, .List, .List) => ?_ </pre>
        <constraints> .List => ?_ </constraints>
        <chips> .Set => ?_ </chips>
        <verdicts> .List => ?_ </verdicts>
        <piIdx> 0 => ?_ </piIdx>
        <genMode> false </genMode> <needs> .List </needs> <strictDecode> false </strictDecode>
"""


def spec_compiled_observable(module: str, program: str, preimage: str,
                             status: str, outputs: str, pis: str,
                             requires: str = 'true', comment: str = '') -> str:
    """The claim module text of one observable-semantics obligation.

    program, preimage: K terms of sorts Program and Preimage (the preimage may
    carry symbolic Int variables; constrain them in `requires`).
    status, outputs, pis: the expected (Status, List, List) of the run.
    The claim reads: job(program, preimage) from the initial configuration
    finishes with <status> status, <outputs> outputs and <pi> pis; every other
    cell is left existential.
    """
    head = f'// {comment}\n' if comment else ''
    return f"""{head}requires "../semantics/zkir-symbolic.k"

module {module}
  imports ZKIR-SYMBOLIC
  claim <k> job({program},
                {preimage}) => .K </k>
        <status> ok() => {status} </status>
        <outputs> .List => {outputs} </outputs>
        <pi> .List => {pis} </pi>
{OTHER_CELLS}    requires {requires}
endmodule
"""


def write_observable_instance() -> Path:
    """The one instance of the template: add %a 1 -> %b ; impact 1 [%b], on a
    symbolic input A with the matching public transcript input."""
    text = spec_compiled_observable(
        module='SPEC-COMPILED-OBSERVABLE',
        program='program(0, (typedId("%a", native()), .TypedIds), .IrTypes, false, '
                '(add(var("%a"), imm(1), "%b") ; impact(imm(1), (var("%b"), .Operands)) ; .Instrs))',
        preimage='preimage(ListItem(A:Int), 7, noComm(), .List, ListItem((A +Int 1) modInt #r), .List)',
        status='ok()', outputs='.List', pis='ListItem(7) ListItem((A +Int 1) modInt #r)',
        requires='0 <=Int A andBool A <Int #r',
        comment='Claim (d): instance of the compiler-obligation template spec_compiled_observable\n'
                '// (tools/run_claims.py). Generated; edit the template, not this file. The\n'
                '// program add %a 1 -> %b ; impact 1 [%b] on a symbolic input A, with the\n'
                '// public transcript input equal to the impacted value, finishes with\n'
                '// (status ok, no outputs, public inputs [binding input 7, (A + 1) mod r]).')
    path = CLAIMS / 'spec-compiled-observable.k'
    path.write_text(text)
    return path


# --- running -------------------------------------------------------------------------

def run_claim(name: str, file: Path, module: str, definition: Path, timeout: float, log_dir: Path) -> tuple[str, float]:
    cmd = ['kprove', str(file), '--definition', str(definition), '--spec-module', module]
    start = time.monotonic()
    try:
        proc = subprocess.run(cmd, cwd=file.parent, capture_output=True, text=True, timeout=timeout)
        out = proc.stdout + proc.stderr
        rc: int | None = proc.returncode
    except subprocess.TimeoutExpired as e:
        out = ((e.stdout or b'').decode(errors='replace') if isinstance(e.stdout, bytes) else (e.stdout or '')) + \
              ((e.stderr or b'').decode(errors='replace') if isinstance(e.stderr, bytes) else (e.stderr or ''))
        rc = None
    secs = time.monotonic() - start
    (log_dir / f'{name}.log').write_text(out)
    if rc == 0 and '#Top' in out:
        return 'proved', secs
    if rc is None:
        return f'not proved (timeout after {timeout:.0f}s)', secs
    if '[Error]' in out:
        first = next((line.strip() for line in out.splitlines() if '[Error]' in line), '')
        return f'not proved (kprove error: {first[:120]})', secs
    return f'not proved (residual goal, exit {rc})', secs


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument('--definition', type=Path, default=DEFAULT_DEFINITION)
    ap.add_argument('--timeout', type=float, default=1200.0, help='seconds per claim (default 1200)')
    ap.add_argument('--only', default=None, help='comma-separated claim names to run')
    ap.add_argument('--log-dir', type=Path, default=None, help='where to keep the kprove output (default: a fresh temp dir)')
    args = ap.parse_args()
    log_dir = args.log_dir or Path(tempfile.mkdtemp(prefix='zkir-k-claims-'))
    log_dir.mkdir(parents=True, exist_ok=True)
    if not (args.definition / 'definition.kore').exists():
        print(f'definition not found: {args.definition} (kompile zkir-symbolic.k --backend haskell)', file=sys.stderr)
        return 2
    write_observable_instance()
    selected = set(args.only.split(',')) if args.only else None
    print(f'# definition {args.definition}')
    print(f'# kprove logs {log_dir}')
    print(f'# {"claim":28s} {"result":56s} seconds')
    proved = total = 0
    for name, file, module in CLAIM_LIST:
        if selected is not None and name not in selected:
            continue
        result, secs = run_claim(name, CLAIMS / file, module, args.definition, args.timeout, log_dir)
        print(f'{name:30s} {result:56s} {secs:8.1f}', flush=True)
        total += 1
        proved += result == 'proved'
    print(f'{proved}/{total} claims proved')
    return 0 if proved == total else 1


if __name__ == '__main__':
    sys.exit(main())

"""Run the kprove claims of experiments/zkir-k/claims against the Haskell
definition zkir-symbolic-kompiled and print one line per claim with the
result and the wall time in seconds.

Usage (repository root):
  uv run --group zkir-k python experiments/zkir-k/tools/run_claims.py [--definition DIR]
      [--timeout SECONDS] [--only NAME,...] [--log-dir DIR] [--stamp STAMP] [--no-vacuity]

The definition is built with
  cd experiments/zkir-k/semantics && kompile zkir-symbolic.k --backend haskell

Each claim is one K module; kprove is invoked once per module. A claim is
"proved" when kprove exits 0 and prints a line that is exactly `#Top`;
otherwise the line says "not proved" with the reason: a residual goal (the
prover's "cannot be rewritten further" / WarnStuckClaimState, tested before the
generic `[Error]`), a kprove error, or a timeout. The kprove output of every
run is kept under --log-dir (`make claims` puts it beside the receipt).

Vacuity. A claim whose `requires` is unsatisfiable proves anything, so every
module is also run as a companion probe: the same claim with `ensures false`
(module NAME-VACUITY, written to a temporary directory with an absolute
`requires` path). The probe must NOT prove; a probe that proves means the
left-hand side is unreachable and the claim is reported "vacuous", which counts
as not proved. `--no-vacuity` skips the probes.

Exit status: 0 when every selected claim proved and no probe proved; 1 when a
claim is not proved or vacuous, or when nothing was selected; 2 when --only
names a claim that does not exist or the definition is missing.

The compiler-obligation template `spec_compiled_observable` writes the claim
module for an observable-semantics obligation: after job(P, Pre) on the program
P and the preimage Pre, the <observable> cell holds obs(status, outputs, pis,
skips), the observable semantics [[P]](pre) of zkir-vm.k (`observable`), with
the outputs encoded as field elements; with `witness_space` the claim also
pins <witnessSpace> and <unconstrainedRegs>. The runner writes its observable
instance (claims/spec-compiled-observable.k) before running.
"""
from __future__ import annotations

import argparse
import os
import platform
import re
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
    ('assert-non-boolean', 'native-ops-spec.k', 'ASSERT-NON-BOOLEAN-SPEC'),
    ('transient_hash', 'transient-hash-spec.k', 'TRANSIENT-HASH-SPEC'),
    ('commitment', 'commitment-spec.k', 'COMMITMENT-SPEC'),
    ('spec_compiled_observable', 'spec-compiled-observable.k', 'SPEC-COMPILED-OBSERVABLE'),
]


# --- the compiler-obligation template -----------------------------------------------

OTHER_CELLS = """\
        <status> ok() => ?_ </status>
        <outputs> .List => ?_ </outputs>
        <pi> .List => ?_ </pi>
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
                             status: str, outputs: str, pis: str, skips: str = '.List',
                             requires: str = 'true', comment: str = '',
                             witness_space: bool | None = None, unconstrained: str = '.List') -> str:
    """The claim module text of one observable-semantics obligation.

    program, preimage: K terms of sorts Program and Preimage (the preimage may
    carry symbolic Int variables; constrain them in `requires`).
    status, outputs, pis, skips: the expected (Status, List, List, List) of
    the run; the outputs as the K List of their encoded field elements, the
    skips as the K List of skipNone() / skipSome(n) markers, one per impact.
    witness_space: None leaves <witnessSpace> and <unconstrainedRegs>
    existential; True or False pins <witnessSpace> to that value and
    <unconstrainedRegs> to `unconstrained` (a K List of register names), so
    that the claim also states membership of the final memory in the
    modelled witness space (the modelled side of circuit acceptance; the
    circuit itself is tier three).
    The claim reads: job(program, preimage) from the initial configuration
    finishes with <observable> obs(status, outputs, pis, skips), the
    observable semantics [[P]](pre) (zkir-vm.k `observable`); every other
    cell, including <status>, <outputs>, <pi> and <skips>, is left
    existential. The entry point is `job`, the raw model of `preprocess`, not
    `checkedJob`: the static check `wf` and the rest of tier one,
    targetContract(P), are discharged separately (zkir-contract.k).
    """
    head = f'// {comment}\n' if comment else ''
    if witness_space is None:
        space = '        <witnessSpace> false => ?_ </witnessSpace> <unconstrainedRegs> .List => ?_ </unconstrainedRegs>\n'
    else:
        space = (f'        <witnessSpace> false => {str(witness_space).lower()} </witnessSpace> '
                 f'<unconstrainedRegs> .List => {unconstrained} </unconstrainedRegs>\n')
    return f"""{head}requires "../semantics/zkir-symbolic.k"

module {module}
  imports ZKIR-SYMBOLIC
  claim <k> job({program},
                {preimage}) => .K </k>
        <observable> noObs() => obs({status}, {outputs}, {pis}, {skips}) </observable>
{OTHER_CELLS}{space}    requires {requires}
endmodule
"""


def write_observable_instance() -> Path:
    """The observable instance of the template: add %a 1 -> %b ; impact 1 [%b],
    on a symbolic input A with the matching public transcript input, with the
    witness space pinned."""
    text = spec_compiled_observable(
        module='SPEC-COMPILED-OBSERVABLE',
        program='program(0, (typedId("%a", native()), .TypedIds), .IrTypes, false, '
                '(add(var("%a"), imm(1), "%b") ; impact(imm(1), (var("%b"), .Operands)) ; .Instrs))',
        preimage='preimage(ListItem(A:Int), 7, noComm(), .List, ListItem((A +Int 1) modInt #r), .List)',
        status='ok()', outputs='.List', pis='ListItem(7) ListItem((A +Int 1) modInt #r)', skips='ListItem(skipNone())',
        requires='0 <=Int A andBool A <Int #r', witness_space=True,
        comment='Claim (d), the observable instance of the compiler-obligation template\n'
                '// spec_compiled_observable (tools/run_claims.py). Generated; edit the template,\n'
                '// not this file. The program add %a 1 -> %b ; impact 1 [%b] on a symbolic\n'
                '// input A, with the public transcript input equal to the impacted value, has\n'
                '// the observable result obs(ok, no outputs, public inputs [binding input 7,\n'
                '// (A + 1) mod r], skips [skipNone()]) in the <observable> cell (zkir-vm.k\n'
                '// `observable`) and its final memory in the modelled witness space with no\n'
                '// unconstrained register. targetContract(P) is discharged separately.')
    path = CLAIMS / 'spec-compiled-observable.k'
    path.write_text(text)
    return path


# --- running -------------------------------------------------------------------------

RESIDUAL_MARKS = ('cannot be rewritten further', 'WarnStuckClaimState', 'WarnClaimRHSIsBottom')


def proved(out: str) -> bool:
    return any(line.strip() == '#Top' for line in out.splitlines())


def kprove(file: Path, module: str, definition: Path, timeout: float) -> tuple[int | None, str, float]:
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
    return rc, out, time.monotonic() - start


def classify(rc: int | None, out: str, timeout: float) -> str:
    if rc == 0 and proved(out):
        return 'proved'
    if rc is None:
        return f'not proved (timeout after {timeout:.0f}s)'
    if any(m in out for m in RESIDUAL_MARKS):
        stuck = next((line.strip() for line in out.splitlines() if 'unify with the destination' in line or 'implication check' in line), '')
        return f'not proved (residual goal, exit {rc}: {stuck[:100]})' if stuck else f'not proved (residual goal, exit {rc})'
    if '[Error]' in out:
        first = next((line.strip() for line in out.splitlines() if '[Error]' in line), '')
        return f'not proved (kprove error: {first[:120]})'
    return f'not proved (no #Top line, exit {rc})'


def run_claim(name: str, file: Path, module: str, definition: Path, timeout: float, log_dir: Path) -> tuple[str, float]:
    rc, out, secs = kprove(file, module, definition, timeout)
    (log_dir / f'{name}.log').write_text(out)
    return classify(rc, out, timeout), secs


def module_text(file: Path, module: str) -> str:
    """The text of one `module NAME ... endmodule` block of a claim file."""
    src = file.read_text()
    m = re.search(r'^module\s+' + re.escape(module) + r'\b.*?^endmodule', src, re.S | re.M)
    if not m:
        raise ValueError(f'module {module} not found in {file}')
    return m.group(0)


def write_probe(file: Path, module: str, probe_dir: Path) -> tuple[Path, str]:
    """The vacuity probe of a claim module: the same claim with `ensures false`
    (appended to an existing `ensures`, else added after the claim body)."""
    body = module_text(file, module)
    probe_module = module + '-VACUITY'
    body = re.sub(r'^module\s+' + re.escape(module) + r'\b', 'module ' + probe_module, body, count=1, flags=re.M)
    if re.search(r'^\s*ensures\b', body, re.M):
        body = re.sub(r'^(\s*ensures\s+)(.*?)(\s*)$', lambda m: f'{m.group(1)}({m.group(2)}) andBool false{m.group(3)}', body, count=1, flags=re.M | re.S)
    else:
        body = body.replace('\nendmodule', '\n    ensures false\nendmodule')
    symbolic = (HERE.parent / 'semantics' / 'zkir-symbolic.k').resolve()
    text = f'// vacuity probe of {module} ({file.name}): must not prove\nrequires "{symbolic}"\n\n{body}\n'
    path = probe_dir / f'{file.stem}-{module.lower()}-vacuity.k'
    path.write_text(text)
    return path, probe_module


def run_probe(name: str, file: Path, module: str, definition: Path, timeout: float, log_dir: Path, probe_dir: Path) -> tuple[str, float]:
    try:
        path, probe_module = write_probe(file, module, probe_dir)
    except ValueError as e:
        return f'probe not written ({e})', 0.0
    rc, out, secs = kprove(path, probe_module, definition, timeout)
    (log_dir / f'{name}.vacuity.log').write_text(out)
    if rc == 0 and proved(out):
        return 'VACUOUS (the probe with ensures false proved: the left-hand side is unsatisfiable)', secs
    if rc is None:
        return f'probe timeout after {timeout:.0f}s', secs
    if any(m in out for m in RESIDUAL_MARKS):
        return 'probe refuted as required', secs
    first = next((line.strip() for line in out.splitlines() if '[Error]' in line), '')
    return f'probe not run to a verdict ({first[:80] or f"exit {rc}"})', secs


def header(definition: Path, log_dir: Path, stamp: str | None, vacuity: bool) -> list[str]:
    def sh(*cmd: str) -> str:
        try:
            return subprocess.run(cmd, capture_output=True, text=True, timeout=60).stdout.strip()
        except (OSError, subprocess.TimeoutExpired):
            return '?'
    branch = sh('git', '-C', str(HERE), 'rev-parse', '--abbrev-ref', 'HEAD') or '?'
    commit = sh('git', '-C', str(HERE), 'rev-parse', '--short', 'HEAD') or '?'
    kver = next((line.split(':', 1)[1].strip() for line in sh('kompile', '--version').splitlines() if line.startswith('K version')), '?')
    lines = [
        'ZKIR K semantics: kprove claims (experiments/zkir-k/claims) on the Haskell backend definition zkir-symbolic-kompiled',
        f'Date: {time.strftime("%Y-%m-%d")}' + (f'   Stamp: {stamp}' if stamp else '') + f'   Branch: {branch} ({commit})',
        f'K: {kver}   Host: {os.cpu_count()} cores, {platform.platform()}',
        f'# definition {definition}',
        f'# kprove logs {log_dir}' + (' (NAME.log and NAME.vacuity.log per claim)' if vacuity else ' (NAME.log per claim)'),
        '# proved = kprove exit 0 and a line that is exactly #Top; vacuity = the same claim with `ensures false` must not prove',
        f'# {"claim":28s} {"result":56s} seconds  vacuity probe',
    ]
    return lines


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument('--definition', type=Path, default=DEFAULT_DEFINITION)
    ap.add_argument('--timeout', type=float, default=1200.0, help='seconds per claim (default 1200)')
    ap.add_argument('--only', default=None, help='comma-separated claim names to run (unknown names: exit 2)')
    ap.add_argument('--log-dir', type=Path, default=None, help='where to keep the kprove output (default: a fresh temp dir)')
    ap.add_argument('--stamp', default=None, help='receipt stamp to print in the header (make claims passes $(STAMP))')
    ap.add_argument('--no-vacuity', action='store_true', help='skip the ensures-false companion probes')
    args = ap.parse_args()
    log_dir = args.log_dir or Path(tempfile.mkdtemp(prefix='zkir-k-claims-'))
    log_dir.mkdir(parents=True, exist_ok=True)
    if not (args.definition / 'definition.kore').exists():
        print(f'definition not found: {args.definition} (kompile zkir-symbolic.k --backend haskell)', file=sys.stderr)
        return 2
    known = {name for name, _, _ in CLAIM_LIST}
    selected = None
    if args.only:
        selected = {x.strip() for x in args.only.split(',') if x.strip()}
        unknown = sorted(selected - known)
        if unknown:
            print(f'unknown claim name(s): {", ".join(unknown)}; known: {", ".join(sorted(known))}', file=sys.stderr)
            return 2
    write_observable_instance()
    vacuity = not args.no_vacuity
    for line in header(args.definition, log_dir, args.stamp, vacuity):
        print(line, flush=True)
    proved_n = total = vacuous = 0
    with tempfile.TemporaryDirectory(prefix='zkir-k-vacuity-') as probe_tmp:
        probe_dir = Path(probe_tmp)
        for name, file, module in CLAIM_LIST:
            if selected is not None and name not in selected:
                continue
            result, secs = run_claim(name, CLAIMS / file, module, args.definition, args.timeout, log_dir)
            probe_note = ''
            if vacuity:
                probe, psecs = run_probe(name, CLAIMS / file, module, args.definition, args.timeout, log_dir, probe_dir)
                probe_note = f'  {probe} ({psecs:.1f}s)'
                if probe.startswith('VACUOUS'):
                    vacuous += 1
                    result = 'vacuous (probe proved)' if result == 'proved' else result + '; vacuous'
            print(f'{name:30s} {result:56s} {secs:8.1f}{probe_note}', flush=True)
            total += 1
            proved_n += result == 'proved'
    print(f'{proved_n}/{total} claims proved' + (f', {vacuous} vacuous' if vacuous else '') + ('' if vacuity else ' (vacuity probes skipped)'))
    if total == 0:
        print('no claim selected', file=sys.stderr)
        return 1
    return 0 if proved_n == total else 1


if __name__ == '__main__':
    sys.exit(main())

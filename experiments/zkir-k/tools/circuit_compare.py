"""Circuit oracle comparison (plan-iter3 M1): the executable form of
plan-iter3/circuit-comparison-table.md.

`run_circuit_oracle` runs `zkir-circuit-oracle` (preprocess, optimal_k,
MockProver::run, verify) on a program and preimage, optionally with a
`--inject` memory perturbation or an `--instance` column perturbation, and
returns its JSON line. `expected_preimage_cell` maps a K run to the table
cell for the honest preimage and the four preimage perturbations;
`predict_injection` is the taint walk of D3/D4/D5 that decides the cell of a
Native-register injection from the program and the K memory of the honest
run; `choose_injections` picks the registers; `judge` compares.
"""
from __future__ import annotations

import json
import math
import subprocess
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Any

import zkir_kast
import zkir_values as zv

CIRCUIT_ORACLE = Path.home() / 'Moriarty/repos/_build/ledger-92e8bdd3/target/release/zkir-circuit-oracle'
CIRCUIT_ORACLE_EXT = Path.home() / 'Moriarty/repos/_build/midnight-zkir-2ffe2d1/target/release/zkir-circuit-oracle'

P, W, S, C, X, A = ('preprocess-error', 'witness-consistency-error', 'synthesis-error',
                    'constraint-failure', 'panic', 'accepted')
OUTCOMES = (P, W, S, C, X, A)
CODE = {P: 'P', W: 'W', S: 'S', C: 'C', X: 'X', A: 'A', 'load-error': 'L'}

# in-circuit arms that store their output with `memory.insert`, not `mem_insert`
BYPASS = {'from_bytes32', 'reverse_bytes', 'reverse', 'bytes32_into_low_high', 'bytes32_from_low_high'}
# arms whose output is stored through `mem_insert` and changes whenever an operand changes
INJECTIVE = {'transient_hash', 'persistent_hash', 'keccak256', 'sha512', 'hash_to_curve', 'ec_mul',
             'ec_mul_generator', 'into_coordinates', 'into_bytes32', 'slice', 'nth', 'concat'}
HASHES = {'transient_hash', 'persistent_hash', 'keccak256', 'sha512'}
READ_FIELDS = ('a', 'b', 'bit', 'cond', 'val', 'input', 'inputs', 'bytes', 'scalar', 'point', 'native',
               'divisor', 'modulus', 'guard', 'vals')
FR_BITS = 255


@dataclass
class Expect:
    """The table cell a comparison is expected to land in. `outcomes` is the set
    of acceptable oracle outcomes (None: not comparable), `message` a substring
    the oracle message must contain (panic classes of D2), `cell` the cell
    name, `reason` the reason the harness chose it."""
    outcomes: frozenset[str] | None
    message: str | None
    cell: str
    reason: str = ''

    def codes(self) -> str:
        if self.outcomes is None:
            return 'n/c'
        return '/'.join(CODE[o] for o in OUTCOMES if o in self.outcomes)


def run_circuit_oracle(binary: Path, program: Path, preimage: dict, inject: dict | None = None,
                       instance: list[str] | None = None) -> dict:
    with tempfile.TemporaryDirectory() as d:
        pre = Path(d) / 'pre.json'
        pre.write_text(json.dumps(preimage))
        cmd = [str(binary), str(program), str(pre)]
        if inject:
            inj = Path(d) / 'inject.json'
            inj.write_text(json.dumps({r: {'type': 'Native', 'value': str(v)} for r, v in inject.items()}))
            cmd += ['--inject', str(inj)]
        if instance is not None:
            ins = Path(d) / 'instance.json'
            ins.write_text(json.dumps([str(x) for x in instance]))
            cmd += ['--instance', str(ins)]
        res = subprocess.run(cmd, capture_output=True, text=True)
    if res.returncode != 0:
        return {'outcome': 'load-error', 'message': res.stderr.strip()[-300:], 'elapsed_ms': 0}
    try:
        return json.loads(res.stdout.strip().splitlines()[-1])
    except (ValueError, IndexError):
        return {'outcome': 'load-error', 'message': 'unparseable oracle output: ' + res.stdout[-200:], 'elapsed_ms': 0}


# --- K side ------------------------------------------------------------------------------

SEVERITY = ('violated', 'synthErr', 'unknown', 'unsupported')


def first_bad(k: dict) -> tuple[str, str, str] | None:
    """The first non-holding verdict in emission order, `commGate` last (the crate
    checks the commitment after the last instruction)."""
    bad = [v for v in k.get('all_verdicts', []) if v[0] != 'holds']
    bad.sort(key=lambda v: v[2].startswith('commGate'))
    return tuple(bad[0]) if bad else None


def k_summary(k: dict) -> str:
    st = k.get('status')
    if st != 'ok':
        return st if st in ('error', 'panic') else f'n/r({st})'
    fb = first_bad(k)
    return 'ok' if fb is None else f'ok/{fb[0]}'


def _gate_operands(gate: str) -> list[tuple[str, Any]]:
    import re
    return [('var', m.group(1)) if m.group(1) else ('imm', int(m.group(2)))
            for m in re.finditer(r'var \( "([^"]+)" \)|imm \( (\d+) \)', gate)]


def _sqrt_mod(a: int, p: int) -> int | None:
    """Tonelli-Shanks; None when `a` is a non-residue."""
    a %= p
    if a == 0:
        return 0
    if pow(a, (p - 1) // 2, p) != 1:
        return None
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, t, r = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1:
            t2 = t2 * t2 % p
            i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r


def jubjub_from_xy(x: int, y: int) -> tuple[int, int] | None:
    """midnight-circuits' `CircuitCurve::from_xy` for Jubjub followed by
    `into_subgroup`: the point is decompressed from `y` and the parity of `x`
    (`JubjubAffine::from_bytes`), `x` itself is not checked. None when no
    point has that `y` and parity, or the point is outside the prime-order
    subgroup: the chip's hint then panics."""
    d, r = zv.JUBJUB_D, zv.R
    den = (1 + d * y * y) % r
    if den == 0:
        return None
    u = _sqrt_mod((y * y - 1) * pow(den, -1, r) % r, r)
    if u is None:
        return None
    if (u & 1) != (x & 1):
        u = (-u) % r
    if (u & 1) != (x & 1):
        return None   # u = 0 with a set sign bit is rejected by from_bytes
    p = (u, y % r)
    if zv.ed_mul(p, zv.RJ, r, d) != (0, 1):
        return None
    return p


def _native_from_coordinates_cell(gate: str, k: dict) -> Expect:
    """D1 for native `from_coordinates`: `C` when the hint reproduces a point and
    only the `assert_equal` on x fails, `X` when the hint unwraps."""
    ops = _gate_operands(gate)
    vals = []
    for kind, v in ops[:2]:
        if kind == 'imm':
            vals.append(v)
        elif v in k['memory'] and k['memory'][v]['type'] == 'Native':
            vals.append(int(k['memory'][v]['encoded'][0]))
        else:
            return Expect(frozenset({W}), None, 'ok/violated: W recompute (D1b)', 'foreign from_coordinates')
    if len(vals) < 2:
        return Expect(frozenset({C, X}), None, 'ok/violated: C/X (D1)', 'operands not parsed')
    p = jubjub_from_xy(vals[0], vals[1])
    if p is None:
        return Expect(frozenset({X}), None, 'ok/violated: X hint (D1c)', 'no subgroup point with this y and parity of x')
    return Expect(frozenset({C}), None, 'ok/violated: C assert_equal (D1a)', f'decompressed point has x = {p[0]}, assert_equal on x fails')


def expected_preimage_cell(k: dict) -> Expect:
    """Row lookup for the honest preimage and the four preimage perturbations."""
    st = k.get('status')
    if st == 'error':
        return Expect(frozenset({P}), None, 'error: P (D9)')
    if st == 'panic':
        return Expect(frozenset({X}), None, 'panic: X (D10)')
    if st != 'ok':
        return Expect(None, None, f'n/r ({st})')
    fb = first_bad(k)
    if fb is None:
        return Expect(frozenset({A}), None, 'ok: A')
    kind, msg, gate = fb
    if kind == 'synthErr':
        if 'chip not initialised' in msg:
            return Expect(frozenset({X}), 'must enable', 'ok/synthErr: X chip (D2)', msg[:60])
        if 'MAX_BOUND' in msg:
            return Expect(frozenset({X}), 'Cannot bound', 'ok/synthErr: X width (D2)', msg[:60])
        return Expect(frozenset({X}), 'Synthesis', 'ok/synthErr: X unwrap (D2)', msg[:60])
    if kind == 'violated':
        if gate.startswith('commGate'):
            return Expect(frozenset({C}), None, 'ok/violated: C (D1a)', msg[:60])
        if 'fromCoordinates' in gate:
            return _native_from_coordinates_cell(gate, k)
        return Expect(frozenset({C, W, X}), None, 'ok/violated: C/W/X (D1)', msg[:60])
    if kind == 'unknown':
        return Expect(frozenset(), None, 'ok/unknown: unreachable (D7)', msg[:60])
    return Expect(None, None, 'ok/unsupported: n/c (D8)', msg[:60])


# --- injections: the taint walk of D3 -------------------------------------------------------

def _is_reg(x: Any) -> bool:
    return isinstance(x, str) and x.startswith('%')


def _reads(ins: dict) -> list[str]:
    out: list[str] = []
    for f in READ_FIELDS:
        v = ins.get(f)
        if v is None:
            continue
        if isinstance(v, list):
            out += [x for x in v if isinstance(x, str)]
        elif isinstance(v, str):
            out.append(v)
    return out


def _writes(ins: dict) -> list[str]:
    if 'outputs' in ins and isinstance(ins['outputs'], list):
        return list(ins['outputs'])
    if 'output' in ins:
        return [ins['output']]
    return []


class Undecided(Exception):
    pass


class Walk:
    """In-circuit values relative to the honest K memory M and the injected
    witness W = M[reg := new]. `tainted` holds registers whose in-circuit value
    differs from M; `cval` their value when it is a known Native."""

    def __init__(self, doc: dict, k: dict, reg: str, new: int):
        self.doc, self.mem, self.reg, self.new = doc, k['memory'], reg, new
        self.pis = [int(x) for x in k['pis']]
        self.tainted: set[str] = set()
        self.cval: dict[str, int | None] = {}
        self.violations: list[str] = []
        self.pi_idx = 1 + (1 if doc.get('do_communications_commitment') else 0)
        self.input_names = [e['name'] for e in doc['inputs']]
        self.output_tainted = False
        # a witness-assigned register (declared input) carries the injected value from the start
        if reg in self.input_names:
            self.taint(reg, new)

    def taint(self, r: str, v: int | None) -> None:
        self.tainted.add(r)
        self.cval[r] = v

    def untaint(self, r: str) -> None:
        self.tainted.discard(r)
        self.cval.pop(r, None)

    def honest(self, r: str) -> int | None:
        e = self.mem.get(r)
        return int(e['encoded'][0]) if e and e['type'] == 'Native' else None

    def witness(self, r: str) -> int | None:
        return self.new if r == self.reg else self.honest(r)

    def cur(self, x: str) -> int | None:
        """In-circuit value of an operand, None when unknown or not Native."""
        if not _is_reg(x):
            return zkir_kast.immediate(x)
        if x in self.tainted:
            return self.cval.get(x)
        return self.honest(x)

    def is_t(self, x: str) -> bool:
        return _is_reg(x) and x in self.tainted

    def need(self, x: str, what: str) -> int:
        v = self.cur(x)
        if v is None:
            raise Undecided(f'{what}: value of {x} unknown in circuit')
        return v

    def store(self, o: str, value: int | None, changed: bool | None, at: str) -> str | None:
        """mem_insert of output `o`: W when the in-circuit value differs from the
        witness. `value` known -> compared; else `changed` says whether it differs
        from M[o] (and W[o] = M[o] unless o is the injected register)."""
        w = self.witness(o)
        if value is not None:
            if w is None or value != w:
                return f'W: mem_insert {o} at {at}: circuit {value} != witness {w}'
            self.untaint(o)
            return None
        if changed is None:
            raise Undecided(f'{at}: cannot tell whether {o} changes')
        if changed or o == self.reg:
            return f'W: mem_insert {o} at {at}: recomputed value differs from the witness'
        self.untaint(o)
        return None

    def bool_check(self, x: str, what: str) -> int:
        v = self.need(x, what)
        if v not in (0, 1):
            self.violations.append(f'{what}: {x} = {v} is not boolean')
            raise Undecided(f'{what}: non-boolean {x} = {v}, the in-circuit hint is not modelled')
        return v

    def step(self, i: int, ins: dict) -> str | None:
        op = ins['op']
        at = f'#{i} {op}'
        outs = _writes(ins)
        if op in ('public_input', 'private_input'):
            o = ins['output']
            if o == self.reg:
                self.taint(o, self.new)
            return None
        if op == 'load_constant':
            return self.store(outs[0], None, False, at) if outs else None
        reads = _reads(ins)
        any_t = any(self.is_t(r) for r in reads)
        if op == 'impact' and not any_t:
            self.pi_idx += len(ins['inputs'])   # every impact pushes, tainted or not
        if not any_t:
            if outs and self.reg in outs:
                # computed-register injection: the defining instruction recomputes M[reg]
                if op in BYPASS:
                    return None   # D4: stored with memory.insert, the injection is ignored
                return self.store(self.reg, None, False, at)
            return None
        if op == 'output':
            self.output_tainted = True
            return None
        # arithmetic and control on Natives
        if op in ('add', 'mul', 'neg', 'copy', 'inv', 'not', 'test_eq', 'less_than', 'cond_select'):
            o = outs[0]
            if op == 'add':
                v = (self.need(ins['a'], at) + self.need(ins['b'], at)) % zv.R
            elif op == 'mul':
                v = (self.need(ins['a'], at) * self.need(ins['b'], at)) % zv.R
            elif op == 'neg':
                v = (-self.need(ins['a'], at)) % zv.R
            elif op == 'copy':
                src = ins['val']
                if self.cur(src) is None:
                    return self.store(o, None, True, at)
                v = self.cur(src)
            elif op == 'inv':
                a = self.need(ins['a'], at)
                if a == 0:
                    self.violations.append(f'{at}: inv of 0')
                    raise Undecided(f'{at}: inv of 0, the hint is not modelled')
                v = pow(a, -1, zv.R)
            elif op == 'not':
                v = 1 - self.bool_check(ins['a'], at)
            elif op == 'test_eq':
                a, b = ins['a'], ins['b']
                if self.cur(a) is None or self.cur(b) is None:
                    raise Undecided(f'{at}: test_eq on non-Native or unknown values')
                v = int(self.cur(a) == self.cur(b))
            elif op == 'less_than':
                a, b = self.need(ins['a'], at), self.need(ins['b'], at)
                bits = int(ins['bits'])
                padded = max(bits + bits % 2, 4)
                if a >= 1 << padded or b >= 1 << padded:
                    self.violations.append(f'{at}: operand above the padded bound 2^{padded}')
                    raise Undecided(f'{at}: operand above the padded bound, the gadget hint is not modelled')
                v = int(a < b)
            else:  # cond_select
                bit = self.bool_check(ins['bit'], at)
                sel = ins['a'] if bit == 1 else ins['b']
                if self.cur(sel) is not None:
                    v = self.cur(sel)
                else:
                    # non-Native arm: changed iff the selected arm is tainted or differs from M[o]
                    changed = self.is_t(sel) or (_is_reg(sel) and self.mem.get(sel) != self.mem.get(o))
                    return self.store(o, None, changed, at)
            r = self.store(o, v, None, at)
            if r:
                return r
            if v != self.honest(o):
                self.taint(o, v)
            return None
        if op == 'encode':
            src = ins['input']
            if self.cur(src) is not None and len(outs) == 1:
                v = self.cur(src)
                r = self.store(outs[0], v, None, at)
                if r:
                    return r
                if v != self.honest(outs[0]):
                    self.taint(outs[0], v)
                return None
            return self.store(outs[0], None, True, at)
        if op in HASHES:
            self.check_alignment(ins, at)
            return self.store(outs[0], None, True, at)
        if op == 'from_coordinates':
            xo, yo = ins['inputs']
            types = [self.mem[x]['type'] if _is_reg(x) and x in self.mem else 'Native' for x in (xo, yo)]
            if all(t == 'Native' for t in types):
                # Jubjub: the hint decompresses from y and the parity of x (jubjub_from_xy)
                if self.is_t(yo):
                    raise Undecided(f'{at}: tainted y into the Jubjub decompression hint')
                x, y = self.need(xo, at), self.need(yo, at)
                p = jubjub_from_xy(x, y)
                if p is None:
                    return f'X: {at}: no subgroup point with this y and the parity of x, the hint unwraps (D1c)'
                w = self.mem.get(outs[0])
                if w is None or [str(p[0]), str(p[1])] != w['encoded']:
                    return f'W: mem_insert {outs[0]} at {at}: decompressed point {p[0]} differs from the witness'
                if p[0] != x:
                    self.violations.append(f'{at}: assert_equal x = {x} vs point x = {p[0]}')
                self.untaint(outs[0])
                return None
            return self.store(outs[0], None, True, at)
        if op in INJECTIVE:
            if op in ('slice', 'nth', 'concat'):
                raise Undecided(f'{at}: byte operation over tainted bytes')
            return self.store(outs[0], None, True, at)
        if op == 'jubjub_scalar_from_native':
            s = self.need(ins['native'], at) % zv.RJ
            w = self.mem.get(outs[0])
            wv = int(w['encoded'][0]) if w else None
            if wv != s:
                return f'W: mem_insert {outs[0]} at {at}: circuit {s} != witness {wv}'
            self.untaint(outs[0])
            return None
        if op == 'div_mod_power_of_two':
            v = self.need(ins['val'], at)
            bits = int(ins['bits'])
            for o, val in zip(outs, (v >> bits, v & ((1 << bits) - 1))):
                r = self.store(o, val, None, at)
                if r:
                    return r
                if val != self.honest(o):
                    self.taint(o, val)
            return None
        if op == 'reconstitute_field':
            d, m, n = self.need(ins['divisor'], at), self.need(ins['modulus'], at), int(ins['bits'])
            if d >= 1 << (FR_BITS - n):
                self.violations.append(f'{at}: divisor above 2^{FR_BITS - n}')
            if m >= 1 << n:
                self.violations.append(f'{at}: modulus above 2^{n}')
            v = ((d << n) + m) % zv.R
            r = self.store(outs[0], v, None, at)
            if r:
                return r
            if v != self.honest(outs[0]):
                self.taint(outs[0], v)
            return None
        # constraints without outputs
        if op == 'assert':
            if self.need(ins['cond'], at) == 0:
                self.violations.append(f'{at}: assert of 0')
            return None
        if op == 'constrain_eq':
            a, b = ins['a'], ins['b']
            va, vb = self.cur(a), self.cur(b)
            if va is not None and vb is not None:
                if va != vb:
                    self.violations.append(f'{at}: {a} = {va} != {b} = {vb}')
                return None
            if self.is_t(a) != self.is_t(b):
                # one side changed from an honest run in which both were equal
                self.violations.append(f'{at}: one side tainted')
                return None
            raise Undecided(f'{at}: both sides tainted with unknown values')
        if op == 'constrain_bits':
            v, bits = self.need(ins['val'], at), int(ins['bits'])
            if bits < FR_BITS and v >= 1 << bits:
                self.violations.append(f'{at}: {v} does not fit {bits} bits')
            return None
        if op == 'constrain_to_boolean':
            v = self.need(ins['val'], at)
            if v not in (0, 1):
                self.violations.append(f'{at}: {v} is not boolean')
            return None
        if op == 'impact':
            g = self.need(ins['guard'], at)
            if g not in (0, 1):
                self.violations.append(f'{at}: guard {g} is not boolean')
                raise Undecided(f'{at}: non-boolean guard, the select hint is not modelled')
            for x in ins['inputs']:
                idx = self.pi_idx
                self.pi_idx += 1
                if g == 0:
                    pushed: int | None = 0
                elif self.cur(x) is not None:
                    pushed = self.cur(x)
                elif self.is_t(x):
                    return f'W: pi_push {idx} at {at}: {x} differs from the witness'
                else:
                    raise Undecided(f'{at}: value of {x} unknown')
                if idx < len(self.pis) and self.pis[idx] != pushed:
                    return f'W: pi_push {idx} at {at}: circuit {pushed} != witness {self.pis[idx]}'
            return None
        if op == 'bytes32_from_low_high':
            lo, hi = ins['inputs']
            if self.is_t(lo):
                v = self.need(lo, at)
                if v >= 1 << 248:
                    self.violations.append(f'{at}: low operand uses byte 31')
            if self.is_t(hi):
                v = self.need(hi, at)
                if v >= 256:
                    self.violations.append(f'{at}: high operand above 255')
            self.taint(outs[0], None)
            return None
        if op in BYPASS:
            for o in outs:
                self.taint(o, None)
            return None
        raise Undecided(f'{at}: tainted operand into an instruction the walk does not model')

    def check_alignment(self, ins: dict, at: str) -> None:
        """A byte segment decodes its operands into bytes in circuit; an injected
        value that no longer fits its chunk is not modelled."""
        widths: list[int | None] = []
        for seg in ins.get('alignment', []):
            if seg.get('tag') != 'atom':
                raise Undecided(f'{at}: non-atom alignment segment')
            val = seg['value']
            if val.get('tag') == 'bytes':
                n = int(val['length'])
                while n > 0:
                    widths.append(min(31, n))
                    n -= 31
            else:
                widths.append(None)
        for x, w in zip(ins['inputs'], widths + [None] * len(ins['inputs'])):
            if self.is_t(x) and w is not None:
                v = self.cur(x)
                if v is None or v >= 1 << (8 * w):
                    raise Undecided(f'{at}: injected value does not fit the {w}-byte chunk')

    def run(self) -> tuple[str, str]:
        for i, ins in enumerate(self.doc['instructions']):
            r = self.step(i, ins)
            if r:
                return r[0], r
        comm = self.doc.get('do_communications_commitment')
        if comm and (self.output_tainted or any(x in self.tainted for x in self.input_names)):
            self.violations.append('commitment: poseidon over tainted inputs or outputs')
        if self.violations:
            return 'C', 'C: ' + '; '.join(self.violations[:3])
        return 'A', 'A: no in-circuit consumer detects the injection'


def predict_injection(doc: dict, k: dict, reg: str, new: int) -> Expect:
    try:
        code, reason = Walk(doc, k, reg, new).run()
    except Undecided as e:
        return Expect(None, None, 'undecided (D3)', str(e))
    if code == 'W':
        return Expect(frozenset({W}), None, 'inject: W (D3)', reason)
    if code == 'C':
        return Expect(frozenset({C}), None, 'inject: C (D3)', reason)
    if code == 'X':
        return Expect(frozenset({X}), None, 'inject: X (D1c)', reason)
    return Expect(frozenset({A}), None, 'inject: A (D3)', reason)


def choose_injections(doc: dict, k: dict) -> list[tuple[str, str, int, Expect]]:
    """One declared Native input, one computed Native register (mem_insert),
    one bypass-stored Native register (D4) and one guarded-off Native
    public_input register (D5) when the program has them; the first register
    with a decided prediction of each kind."""
    mem = k['memory']
    natives = {r for r, e in mem.items() if e['type'] == 'Native'}
    chosen: list[tuple[str, str, int, Expect]] = []

    def pick(kind: str, cands: list[str], fixed: Expect | None = None) -> None:
        best = None
        for r in cands:
            new = (int(mem[r]['encoded'][0]) + 1) % zv.R
            exp = fixed or predict_injection(doc, k, r, new)
            if best is None or (best[3].outcomes is None and exp.outcomes is not None):
                best = (kind, r, new, exp)
            if exp.outcomes is not None:
                break
        if best:
            chosen.append(best)

    pick('inject-input', [e['name'] for e in doc['inputs'] if e['name'] in natives])
    computed, bypass, guard_off = [], [], []
    for ins in doc['instructions']:
        op = ins['op']
        if op in ('public_input', 'private_input'):
            g = ins.get('guard')
            if op == 'public_input' and g is not None and ins['output'] in natives:
                gv = zkir_kast.immediate(g) if not _is_reg(g) else (int(mem[g]['encoded'][0]) if g in natives else None)
                if gv == 0:
                    guard_off.append(ins['output'])
            continue
        for o in _writes(ins):
            if o in natives:
                (bypass if op in BYPASS else computed).append(o)
    pick('inject-computed', computed)
    pick('inject-ignored', bypass, Expect(frozenset({A}), None, 'inject-ignored: A (D4)', 'output stored with memory.insert'))
    pick('inject-guard-off', guard_off)
    return chosen


def instance_perturbation(k: dict, rng) -> tuple[int, list[str], Expect]:
    pis = [int(x) for x in k['pis']]
    idx = rng.randrange(len(pis))
    pis[idx] = (pis[idx] + 1) % zv.R
    return idx, [str(x) for x in pis], Expect(frozenset({C}), None, 'instance: C (D6)', f'instance[{idx}] + 1')


def judge(exp: Expect, res: dict) -> str:
    if exp.outcomes is None:
        return 'N/C'
    if res.get('outcome') in exp.outcomes and (exp.message is None or exp.message in res.get('message', '')):
        return 'AGREE'
    return 'DISAGREE'


def describe(res: dict) -> str:
    out = res.get('outcome', '?')
    extra = ''
    if out in (X, S, W, 'load-error'):
        extra = ' ' + repr(res.get('message', '')[:70])
    elif out == C:
        extra = f" {len(res.get('failures', []))} failure(s)"
    return f"{out}{extra} {res.get('elapsed_ms', 0)}ms"

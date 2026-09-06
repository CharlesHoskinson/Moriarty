"""Circuit oracle comparison (plan-iter3 M1): the executable form of
plan-iter3/circuit-comparison-table.md.

`run_circuit_oracle` runs `zkir-circuit-oracle` (preprocess, optimal_k,
MockProver::run, verify) on a program and preimage, optionally with a
`--inject` memory perturbation or an `--instance` column perturbation, and
returns its JSON line. `expected_preimage_cell` maps a K run to the table
cell for the honest preimage and the four preimage perturbations: a
`synthErr` anywhere in the verdicts decides the row (it is witness-independent
and fires inside `optimal_k` before `MockProver::run`), else the first
`violated` in emission order (`commGate` last) is realised per gate (D1: `C`
for an equality assertion, `W` for a `mem_insert` output, `X` with its message
class for a hint that panics); a gate the table does not classify is `n/c`,
never an agreement. `predict_injection` is the taint walk of D3/D4/D5 that
decides the cell of a Native-register injection from the program and the K
memory of the honest run; `choose_injections` picks the registers and, for
each, a second value chosen by the register's first consumer (a boundary of
that consumer's in-circuit hint); `judge` compares.

A K verdict `unconstrained` (plan-iter3 M2) is not a failure: the relation is
satisfied and does not pin the register beyond its type (a `public_input` /
`private_input` at guard 0, where the witness holds the type's default; a
witness-assigned JubjubScalar or foreign field element whose canonicity is not
proven). `k_summary` skips it, so a run whose only non-`holds` verdicts are
`unconstrained` is row `ok` (its `witness_space` flag is true).
`choose_injections` injects one such Native register (column
`inject-unconstrained`, D5): whether the cell is free in the circuit is
decided by its consumers, so `A` is claimed only when no later relation reads
the register (`free_cells` reports that classification for every
`unconstrained` register, Native or not).
"""
from __future__ import annotations

import json
import re
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
IL, IE = 'instance-length-mismatch', 'inject-error'
OUTCOMES = (P, W, S, C, X, A, IL, IE)
CODE = {P: 'P', W: 'W', S: 'S', C: 'C', X: 'X', A: 'A', IL: 'IL', IE: 'IE', 'load-error': 'L'}

# --- message classes (D2, D1c, D10): each alternative is a tuple of substrings the
# oracle message must all contain; a class is a tuple of alternatives.
MSG_UNWRAP = (('called `Result::unwrap()`', 'Synthesis('),)          # D2: Error::Synthesis unwrapped in optimal_k
MSG_CHIP = (('must enable',),)                                        # D2: ZkStdLibArch must enable <chip>
MSG_WIDTH = (('Cannot bound',),)                                      # D2: bounded_of_element MAX_BOUND
MSG_BYTE = (('AssignedByte',),)                                       # D1c: Trying to convert ... to AssignedByte
MSG_BOUNDED = (('AssignedBounded',),)                                 # D1c: Trying to convert ... to an AssignedBounded
MSG_PANIC_PREPROCESS = (('out of range for slice',), ('assertion `left == right` failed',))   # D10


def msg_class(*required: str) -> tuple[tuple[str, ...], ...]:
    return (tuple(required),)


def message_matches(cls: tuple[tuple[str, ...], ...] | None, message: str) -> bool:
    if cls is None:
        return True
    return any(all(s in message for s in alt) for alt in cls)


def describe_class(cls: tuple[tuple[str, ...], ...] | None) -> str:
    if cls is None:
        return ''
    return ' | '.join(' & '.join(repr(s) for s in alt) for alt in cls)


# in-circuit arms that store their output with `memory.insert`, not `mem_insert`
BYPASS = {'from_bytes32', 'reverse_bytes', 'reverse', 'bytes32_into_low_high', 'bytes32_from_low_high'}
# arms whose output is stored through `mem_insert` and changes whenever an operand changes
INJECTIVE = {'transient_hash', 'persistent_hash', 'keccak256', 'sha512', 'hash_to_curve', 'ec_mul',
             'ec_mul_generator', 'into_coordinates', 'into_bytes32'}
HASHES = {'transient_hash', 'persistent_hash', 'keccak256', 'sha512'}
BYTE_OPS = {'slice', 'nth', 'concat'}
BOOL_OPS = {'and', 'or', 'xor'}
READ_FIELDS = ('a', 'b', 'bit', 'cond', 'val', 'input', 'inputs', 'bytes', 'scalar', 'point', 'native',
               'divisor', 'modulus', 'guard', 'vals')
FR_BITS = 255
INT_TYPES = ('Native', 'Bool', 'Byte')


@dataclass
class Expect:
    """The table cell a comparison is expected to land in. `outcomes` is the set
    of acceptable oracle outcomes (None: not comparable), `message` a message
    class (alternatives of required substrings; the panic classes of D2, D1c and
    D10), `cell` the cell name, `reason` the reason the harness chose it."""
    outcomes: frozenset[str] | None
    message: tuple[tuple[str, ...], ...] | None
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
    last = res.stdout.strip().splitlines()[-1] if res.stdout.strip() else ''
    if res.returncode != 0:
        # exit 2 with a JSON line is the oracle's own `inject-error` (a harness
        # error, item 16); anything else is a load / preimage error before preprocess
        if last.startswith('{'):
            try:
                return json.loads(last)
            except ValueError:
                pass
        return {'outcome': 'load-error', 'message': res.stderr.strip()[-300:], 'elapsed_ms': 0}
    try:
        return json.loads(last)
    except ValueError:
        return {'outcome': 'load-error', 'message': 'unparseable oracle output: ' + res.stdout[-200:], 'elapsed_ms': 0}


# --- K side ------------------------------------------------------------------------------

SEVERITY = ('synthErr', 'violated', 'unknown', 'unsupported')


def gate_op(gate: str) -> str:
    """The instruction constructor of a `gate ( op ( ... ) )` string, else the
    constraint constructor (`commGate`, `piGate`, ...)."""
    m = re.match(r'\s*gate\s*\(\s*([A-Za-z0-9_]+)', gate)
    if m:
        return m.group(1)
    m = re.match(r'\s*([A-Za-z0-9_]+)', gate)
    return m.group(1) if m else gate


def first_bad(k: dict) -> tuple[str, str, str] | None:
    """The verdict that decides the row: any `synthErr` (witness-independent,
    fires in `optimal_k` before `MockProver::run`, so it precedes every
    witness-dependent outcome), else the first `violated` / `unknown` /
    `unsupported` in emission order with `commGate` last (the crate checks the
    commitment after the last instruction)."""
    bad = [v for v in k.get('all_verdicts', []) if v[0] not in ('holds', 'unconstrained')]
    synth = [v for v in bad if v[0] == 'synthErr']
    if synth:
        return tuple(synth[0])
    bad.sort(key=lambda v: v[2].startswith('commGate'))
    return tuple(bad[0]) if bad else None


def k_summary(k: dict) -> str:
    st = k.get('status')
    if st != 'ok':
        return st if st in ('error', 'panic') else f'n/r({st})'
    fb = first_bad(k)
    return 'ok' if fb is None else f'ok/{fb[0]}'


def _gate_operands(gate: str) -> list[tuple[str, Any]]:
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


def curve25519_on_curve(x: int, y: int) -> bool:
    """-x^2 + y^2 = 1 + d x^2 y^2 over GF(2^255 - 19)."""
    p, d = zv.C25519P, zv.C25519_D
    return (-x * x + y * y - 1 - d * x * x * y * y) % p == 0


def curve25519_in_subgroup(x: int, y: int) -> bool:
    return zv.ed_mul((x % zv.C25519P, y % zv.C25519P), zv.C25519L, zv.C25519P, zv.C25519_D) == (0, 1)


def _foreign_value(k: dict, name: str) -> int | None:
    """The integer of a foreign base-field register from its limb encoding
    (the inverse of zkir_values.enc_foreign for the 64-bit, 4-limb layout)."""
    e = k['memory'].get(name)
    if not e:
        return None
    ty = e['type']
    mod = {'Curve25519Base': zv.C25519P, 'Secp256k1Base': zv.K256P, 'Secp256r1Base': zv.P256P}.get(ty)
    if mod is None:
        return None
    limbs = [int(x) for x in e['encoded']]
    acc, shift = 0, 0
    per = 254 // 64
    left = 4
    for limb in limbs:
        take = min(left, per)
        acc |= limb << shift
        shift += 64 * take
        left -= take
    return (acc + 1) % mod


def _from_coordinates_cell(gate: str, k: dict) -> Expect:
    """D1 for `from_coordinates`. Native (Jubjub): `C` when the hint reproduces a
    point and only the `assert_equal` on x fails, `X` when the hint unwraps.
    Foreign Edwards (Curve25519, `ecc/foreign/edwards_chip.rs
    point_from_coordinates`): `S` "invalid coordinates" from the hint on an
    off-curve pair, `X` from `into_subgroup` on a torsion point. Foreign
    Weierstrass (secp256k1, secp256r1): `W`, the point is built with
    `from_xy(x, y).unwrap_or(identity)` and stored through `mem_insert`."""
    ops = _gate_operands(gate)
    types = []
    vals: list[int | None] = []
    for kind, v in ops[:2]:
        if kind == 'imm':
            types.append('Native')
            vals.append(v)
        elif v in k['memory']:
            types.append(k['memory'][v]['type'])
            e = k['memory'][v]
            vals.append(int(e['encoded'][0]) if e['type'] == 'Native' else _foreign_value(k, v))
        else:
            return Expect(None, None, 'ok/violated: n/c (D1)', f'operand {v} absent from the memory')
    if len(vals) < 2:
        return Expect(None, None, 'ok/violated: n/c (D1)', 'operands not parsed')
    if all(t == 'Native' for t in types):
        p = jubjub_from_xy(vals[0], vals[1])
        if p is None:
            return Expect(frozenset({X}), None, 'ok/violated: X hint (D1c)', 'no subgroup point with this y and parity of x')
        return Expect(frozenset({C}), None, 'ok/violated: C assert_equal (D1a)', f'decompressed point has x = {p[0]}, assert_equal on x fails')
    if all(t == 'Curve25519Base' for t in types) and None not in vals:
        x, y = vals
        if not curve25519_on_curve(x, y):
            return Expect(frozenset({S}), msg_class('invalid coordinates'), 'ok/violated: S foreign Edwards hint (D1b)', 'off-curve pair: point_from_coordinates returns Error::Synthesis("invalid coordinates")')
        if not curve25519_in_subgroup(x, y):
            return Expect(frozenset({X}), None, 'ok/violated: X foreign Edwards into_subgroup (D1c)', 'torsion point: into_subgroup panics')
        return Expect(None, None, 'ok/violated: n/c (D1)', 'Curve25519 pair on the curve and in the subgroup, yet violated in K')
    return Expect(frozenset({W}), None, 'ok/violated: W recompute (D1b)', 'foreign Weierstrass from_coordinates: from_xy(x, y).unwrap_or(identity) then mem_insert')


# the realisation of a `violated` per gate (item 11): C for an equality
# assertion on cells the circuit holds, W for a mem_insert output, X with a
# message class for a hint that panics. Gates absent here are unclassified.
VIOLATED_C = {'constrainBits', 'constrainEq', 'constrainToBoolean', 'assert', 'guardGate', 'commGate', 'bindGate'}
VIOLATED_W = {'add', 'mul', 'neg', 'copy', 'inv', 'testEq', 'encode', 'transientHash', 'persistentHash', 'keccak256',
              'sha512', 'hashToCurve', 'ecMul', 'ecMulGenerator', 'intoCoordinates', 'intoBytes32',
              'jubjubScalarFromNative', 'divModPowerOfTwo', 'reconstituteField', 'piGate',
              'slice', 'nth', 'concat', 'andI', 'orI', 'xorI', 'loadConstant', 'publicInput', 'privateInput'}


def violated_cell(msg: str, gate: str, k: dict) -> Expect:
    op = gate_op(gate)
    if op == 'commGate':
        return Expect(frozenset({C}), None, 'ok/violated: C commitment (D1a)', msg[:60])
    if op == 'fromCoordinates':
        return _from_coordinates_cell(gate, k)
    if op == 'bytes32FromLowHigh':
        if 'high' in msg:
            return Expect(frozenset({X}), MSG_BYTE, 'ok/violated: X AssignedByte (D1c)', msg[:60])
        return Expect(frozenset({C}), None, 'ok/violated: C byte 31 (D1a)', msg[:60])
    if op == 'lessThan':
        return Expect(frozenset({X}), MSG_BOUNDED, 'ok/violated: X AssignedBounded (D1c)', msg[:60])
    if op == 'inv':
        return Expect(frozenset({W}), None, 'ok/violated: W inv hint (D1b)', msg[:60])
    if op in VIOLATED_C:
        return Expect(frozenset({C}), None, 'ok/violated: C (D1a)', msg[:60])
    if op in VIOLATED_W:
        return Expect(frozenset({W}), None, 'ok/violated: W mem_insert (D1b)', msg[:60])
    return Expect(None, None, 'ok/violated: n/c (D1 unclassified gate)', f'{op}: {msg[:50]}')


def synth_cell(msg: str) -> Expect:
    if 'chip not initialised' in msg:
        return Expect(frozenset({X}), MSG_CHIP, 'ok/synthErr: X chip (D2)', msg[:60])
    if 'MAX_BOUND' in msg:
        return Expect(frozenset({X}), MSG_WIDTH, 'ok/synthErr: X width (D2)', msg[:60])
    return Expect(frozenset({X}), MSG_UNWRAP, 'ok/synthErr: X unwrap (D2)', msg[:60])


def expected_preimage_cell(k: dict) -> Expect:
    """Row lookup for the honest preimage and the four preimage perturbations."""
    st = k.get('status')
    if st == 'error':
        return Expect(frozenset({P}), None, 'error: P (D9)')
    if st == 'panic':
        return Expect(frozenset({X}), MSG_PANIC_PREPROCESS, 'panic: X (D10)')
    if st != 'ok':
        return Expect(None, None, f'n/r ({st})')
    fb = first_bad(k)
    if fb is None:
        return Expect(frozenset({A}), None, 'ok: A')
    kind, msg, gate = fb
    if kind == 'synthErr':
        return synth_cell(msg)
    if kind == 'violated':
        return violated_cell(msg, gate, k)
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


def consumers(doc: dict, reg: str) -> list[tuple[int, str]]:
    """(index, op) of every instruction that reads `reg`."""
    return [(i, ins['op']) for i, ins in enumerate(doc['instructions']) if reg in _reads(ins)]


def free_cells(doc: dict, k: dict) -> list[tuple[str, str, str]]:
    """(register, type, classification) for every register K reports
    `unconstrained`: `free` when no later relation reads it, else `read by #i
    op` (D5: freedom in the circuit is decided by the consumers, not by the
    assigning gate)."""
    out = []
    for r in k.get('unconstrained', []):
        ty = k['memory'].get(r, {}).get('type', '?')
        cs = consumers(doc, r)
        out.append((r, ty, 'free' if not cs else 'read by ' + ', '.join(f'#{i} {op}' for i, op in cs[:3])))
    return out


def honest_bytes(k: dict, r: str) -> bytes | None:
    """The byte string of a Bytes32 / Bytes(n) register from its encoding."""
    e = k['memory'].get(r)
    if not e:
        return None
    ty = e['type']
    enc = [int(x) for x in e['encoded']]
    if ty == 'Bytes32' or ty == 'Bytes(32)':
        return enc[0].to_bytes(31, 'little') + bytes([enc[1]])
    m = re.fullmatch(r'Bytes\((\d+)\)', ty)
    if m:
        n = int(m.group(1))
        out = b''
        for chunk in enc:
            take = min(31, n - len(out))
            out += chunk.to_bytes(take, 'little')
        return out
    return None


class Undecided(Exception):
    pass


class Walk:
    """In-circuit values relative to the honest K memory M and the injected
    witness W = M[reg := new]. `tainted` holds registers whose in-circuit value
    differs from M; `cval` their value when it is a known integer (Native, Bool,
    Byte), `cbytes` their bytes when the register is byte-typed and known."""

    def __init__(self, doc: dict, k: dict, reg: str, new: int):
        self.doc, self.mem, self.reg, self.new = doc, k['memory'], reg, new
        self.k = k
        self.pis = [int(x) for x in k['pis']]
        self.tainted: set[str] = set()
        self.cval: dict[str, int | None] = {}
        self.cbytes: dict[str, bytes | None] = {}
        self.violations: list[str] = []
        self.notes: list[str] = []
        self.pi_idx = 1 + (1 if doc.get('do_communications_commitment') else 0)
        self.input_names = [e['name'] for e in doc['inputs']]
        self.output_tainted = False
        # a witness-assigned register (declared input) carries the injected value from the start
        if reg in self.input_names:
            self.taint(reg, new)

    def taint(self, r: str, v: int | None, b: bytes | None = None) -> None:
        self.tainted.add(r)
        self.cval[r] = v
        if b is not None:
            self.cbytes[r] = b
        else:
            self.cbytes.pop(r, None)

    def untaint(self, r: str) -> None:
        self.tainted.discard(r)
        self.cval.pop(r, None)
        self.cbytes.pop(r, None)

    def honest(self, r: str) -> int | None:
        e = self.mem.get(r)
        return int(e['encoded'][0]) if e and e['type'] in INT_TYPES else None

    def witness(self, r: str) -> int | None:
        return self.new if r == self.reg else self.honest(r)

    def cur(self, x: str) -> int | None:
        """In-circuit value of an operand, None when unknown or not an integer type."""
        if not _is_reg(x):
            return zkir_kast.immediate(x)
        if x in self.tainted:
            return self.cval.get(x)
        return self.honest(x)

    def cur_bytes(self, x: str) -> bytes | None:
        if not _is_reg(x):
            return None
        if x in self.tainted:
            return self.cbytes.get(x)
        return honest_bytes(self.k, x)

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

    def store_bytes(self, o: str, b: bytes | None, at: str) -> str | None:
        """mem_insert of a byte-typed output with a known value."""
        if b is None:
            return self.store(o, None, True, at)
        w = honest_bytes(self.k, o)
        if w is None or b != w:
            return f'W: mem_insert {o} at {at}: recomputed bytes differ from the witness'
        self.untaint(o)
        return None

    def bit(self, x: str, what: str) -> int:
        """`std.convert(AssignedNative -> AssignedBit)` (midnight-circuits
        native_chip.rs): the hint is `v != 0`, and `assert_equal(x, bit)` plus
        the bit cell's booleanity fail at verify when v is not 0 or 1."""
        v = self.need(x, what)
        if v not in (0, 1):
            self.violations.append(f'{what}: {x} = {v} is not boolean (convert hint takes it as 1, assert_equal fails)')
            return 1
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
                    # `std.inv` hints 0 for a zero operand; mem_insert compares it with
                    # the honest inverse (item 10, oracle e2_inv)
                    self.notes.append(f'{at}: inv of 0, the hint stores 0')
                    v = 0
                else:
                    v = pow(a, -1, zv.R)
            elif op == 'not':
                v = 1 - self.bit(ins['a'], at)
            elif op == 'test_eq':
                a, b = ins['a'], ins['b']
                va, vb = self.cur(a), self.cur(b)
                if va is not None and vb is not None:
                    v = int(va == vb)
                else:
                    ba, bb = self.cur_bytes(a), self.cur_bytes(b)
                    if ba is not None and bb is not None:
                        v = int(ba == bb)
                    else:
                        raise Undecided(f'{at}: test_eq on values the walk does not know')
            elif op == 'less_than':
                a, b = self.need(ins['a'], at), self.need(ins['b'], at)
                bits = int(ins['bits'])
                padded = max(bits + bits % 2, 4)
                if a >= 1 << padded or b >= 1 << padded:
                    # BoundedElement::new asserts the value below 2^bound (native_gadget.rs)
                    return f'X: {at}: operand above the padded bound 2^{padded}, BoundedElement::new panics (AssignedBounded)'
                v = int(a < b)
            else:  # cond_select
                bit = self.bit(ins['bit'], at)
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
            # every tainted operand changes the digest; a value that overflows its
            # byte atom is truncated by the decomposition hint and the recomputed
            # digest differs from the witness at the output's mem_insert (item 5)
            return self.store(outs[0], None, True, at)
        if op == 'from_coordinates':
            xo, yo = ins['inputs']
            types = [self.mem[x]['type'] if _is_reg(x) and x in self.mem else 'Native' for x in (xo, yo)]
            if all(t == 'Native' for t in types):
                # Jubjub: the hint decompresses from y and the parity of x (jubjub_from_xy)
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
            return self.store(outs[0], None, True, at)
        if op in BYTE_OPS:
            # 2ffe2d1: slice, nth and concat store through mem_insert
            if op == 'slice':
                b = self.cur_bytes(ins['bytes'])
                if b is None:
                    return self.store(outs[0], None, True, at)
                s, n = int(ins['start']), int(ins['len'])
                return self.store_bytes(outs[0], b[s:s + n], at)
            if op == 'nth':
                b = self.cur_bytes(ins['bytes'])
                if b is None:
                    return self.store(outs[0], None, True, at)
                v = b[int(ins['index'])]
                r = self.store(outs[0], v, None, at)
                if r:
                    return r
                if v != self.honest(outs[0]):
                    self.taint(outs[0], v)
                return None
            parts = []
            for x in ins['inputs']:
                bx = self.cur_bytes(x)
                vx = self.cur(x)
                if bx is not None:
                    parts.append(bx)
                elif vx is not None and self.mem.get(x, {}).get('type') == 'Byte':
                    parts.append(bytes([vx]))
                else:
                    return self.store(outs[0], None, True, at)
            return self.store_bytes(outs[0], b''.join(parts), at)
        if op in BOOL_OPS:
            vals = [self.cur(x) for x in ins['inputs']]
            if any(v is None for v in vals):
                return self.store(outs[0], None, True, at)
            bits = [int(v != 0) for v in vals]
            v = {'and': int(all(bits)), 'or': int(any(bits)), 'xor': sum(bits) % 2}[op]
            r = self.store(outs[0], v, None, at)
            if r:
                return r
            if v != self.honest(outs[0]):
                self.taint(outs[0], v)
            return None
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
            ba, bb = self.cur_bytes(a), self.cur_bytes(b)
            if ba is not None and bb is not None:
                if ba != bb:
                    self.violations.append(f'{at}: bytes of {a} and {b} differ')
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
            self.bit(ins['val'], at)
            return None
        if op == 'impact':
            # the guard is converted with the same hint as a cond_select bit (item 15):
            # a non-boolean guard selects the input and fails booleanity at verify
            g = self.bit(ins['guard'], at)
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
            lo_v, hi_v = self.cur(lo), self.cur(hi)
            if self.is_t(hi) and hi_v is not None and hi_v >= 256:
                # `std.convert(AssignedNative -> AssignedByte)` asserts the value below 256
                # during synthesis, before verify sees any byte-31 failure
                return f'X: {at}: high operand {hi_v} above 255, convert to AssignedByte panics (D1c)'
            if self.is_t(lo) and lo_v is not None and lo_v >= 1 << 248:
                self.violations.append(f'{at}: low operand uses byte 31 (assert_equal_to_fixed(bytes[31], 0) fails)')
            if lo_v is not None and hi_v is not None:
                b = bytearray(lo_v.to_bytes(32, 'little'))
                b[31] = hi_v & 0xff
                self.taint(outs[0], None, bytes(b))
            else:
                self.taint(outs[0], None)
            return None
        if op == 'bytes32_into_low_high':
            b = self.cur_bytes(ins['bytes'])
            if b is None:
                for o in outs:
                    self.taint(o, None)
                return None
            lo_v = int.from_bytes(b[:31], 'little')
            hi_v = b[31]
            for o, v in zip(outs, (lo_v, hi_v)):
                if v != self.honest(o):
                    self.taint(o, v)
                else:
                    self.untaint(o)
            return None
        if op in ('reverse_bytes', 'reverse'):
            b = self.cur_bytes(ins['bytes'])
            self.taint(outs[0], None, bytes(reversed(b)) if b is not None else None)
            return None
        if op == 'from_bytes32':
            b = self.cur_bytes(ins['bytes'])
            if b is not None and ins.get('type') == 'Scalar<BLS12-381>':
                v = int.from_bytes(b, 'little')
                if v < zv.R:
                    if v != self.honest(outs[0]):
                        self.taint(outs[0], v)
                    else:
                        self.untaint(outs[0])
                    return None
            self.taint(outs[0], None)
            return None
        if op in BYPASS:
            for o in outs:
                self.taint(o, None)
            return None
        raise Undecided(f'{at}: tainted operand into an instruction the walk does not model')

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
        return 'A', 'A: no in-circuit consumer detects the injection' + (' (' + '; '.join(self.notes[:2]) + ')' if self.notes else '')


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
        cls = MSG_BOUNDED if 'AssignedBounded' in reason else (MSG_BYTE if 'AssignedByte' in reason else None)
        return Expect(frozenset({X}), cls, 'inject: X (D3 range)' if cls else 'inject: X (D1c)', reason)
    return Expect(frozenset({A}), None, 'inject: A (D3)', reason)


def predict_unconstrained(doc: dict, k: dict, reg: str, new: int) -> Expect:
    """D5: a register K reports `unconstrained` is pinned to its type only by
    its assigning gate. Injecting any value is `A` when no consumer reads it in
    circuit; when a consumer stores a recomputed value through `mem_insert` or
    a constraint reads it, the injected memory is outside the witness space
    and the D3 walk decides."""
    exp = predict_injection(doc, k, reg, new)
    if exp.outcomes == frozenset({A}):
        return Expect(exp.outcomes, None, 'inject-unconstrained: A (D5)', 'free cell, ' + exp.reason)
    if exp.outcomes is None:
        return Expect(None, None, 'undecided (D5)', exp.reason)
    return Expect(exp.outcomes, exp.message, f'inject-unconstrained: {exp.codes()} (D5 consumer, D3)', exp.reason)


def predict_transcript(doc: dict, k: dict, reg: str, new: int) -> Expect:
    """The `inject-transcript` column (item 12): a `public_input` /
    `private_input` register the gate pins to its type only, for every guard;
    what the circuit does with the injected cell is decided by its consumers."""
    exp = predict_injection(doc, k, reg, new)
    if exp.outcomes is None:
        return Expect(None, None, 'undecided (D3 transcript)', exp.reason)
    return Expect(exp.outcomes, exp.message, f'inject-transcript: {exp.codes()} (D3)', exp.reason)


def boundary_value(doc: dict, reg: str, honest: int) -> tuple[int, str] | None:
    """A second injected value chosen by the register's first consumer: the
    boundary of that consumer's in-circuit hint (item 12)."""
    for i, ins in enumerate(doc['instructions']):
        if reg not in _reads(ins):
            continue
        op = ins['op']
        at = f'#{i} {op}'
        if op == 'constrain_bits' or op == 'div_mod_power_of_two':
            return 1 << int(ins['bits']), f'{at}: 1 << bits'
        if op == 'reconstitute_field':
            if ins.get('divisor') == reg:
                return 1 << (FR_BITS - int(ins['bits'])), f'{at}: divisor bound 1 << (255 - bits)'
            return 1 << int(ins['bits']), f'{at}: modulus bound 1 << bits'
        if (op == 'cond_select' and ins.get('bit') == reg) or (op == 'impact' and ins.get('guard') == reg) \
                or op in ('not', 'constrain_to_boolean', 'assert'):
            return 2, f'{at}: non-boolean 2'
        if op in ('cond_select', 'impact'):
            return None   # a selected arm or a pushed value has no boundary of its own
        if op == 'bytes32_from_low_high':
            if ins['inputs'][0] == reg:
                return 1 << 248, f'{at}: low operand with byte 31 set'
            return 256, f'{at}: high operand above 255'
        if op == 'from_coordinates':
            if ins['inputs'][0] == reg:
                return (honest + 2) % zv.R, f'{at}: x + 2 keeps the parity'
            return (honest + 1) % zv.R, f'{at}: y + 1'
        if op == 'inv':
            return 0, f'{at}: inv of 0'
        if op == 'less_than':
            bits = int(ins['bits'])
            return 1 << max(bits + bits % 2, 4), f'{at}: 1 << padded bits'
        if op in ('persistent_hash', 'keccak256', 'sha512'):
            widths: list[int | None] = []
            for seg in ins.get('alignment', []):
                if seg.get('tag') != 'atom':
                    return None
                val = seg['value']
                if val.get('tag') == 'bytes':
                    n = int(val['length'])
                    while n > 0:
                        widths.append(min(31, n))
                        n -= 31
                else:
                    widths.append(None)
            for x, w in zip(ins['inputs'], widths + [None] * len(ins['inputs'])):
                if x == reg and w is not None:
                    return 1 << (8 * w), f'{at}: 1 << (8 * {w}) overflows the {w}-byte atom'
            return None
        return None
    return None


def choose_injections(doc: dict, k: dict) -> list[tuple[str, str, int, Expect]]:
    """One declared Native input, one computed Native register (mem_insert),
    one bypass-stored Native register (D4), one Native register whose assigning
    verdict is `unconstrained` (D5) and one Native `public_input` /
    `private_input` register that is not `unconstrained` (`inject-transcript`,
    item 12) when the program has them; the first register with a decided
    prediction of each kind, and for D5 the first whose cell no consumer reads.
    Every chosen register (except a bypass-stored one, whose injection is
    ignored) is injected twice: `v + 1`, and the boundary value of its first
    consumer (`<kind>:bound`, item 12)."""
    mem = k['memory']
    natives = {r for r, e in mem.items() if e['type'] == 'Native'}
    unconstrained = set(k.get('unconstrained', []))
    chosen: list[tuple[str, str, int, Expect]] = []

    def predict(kind: str, r: str, new: int) -> Expect:
        if kind == 'inject-unconstrained':
            return predict_unconstrained(doc, k, r, new)
        if kind == 'inject-transcript':
            return predict_transcript(doc, k, r, new)
        return predict_injection(doc, k, r, new)

    def with_bound(kind: str, r: str, exp_first: Expect) -> None:
        chosen.append((kind, r, (int(mem[r]['encoded'][0]) + 1) % zv.R, exp_first))
        honest = int(mem[r]['encoded'][0])
        bv = boundary_value(doc, r, honest)
        if bv is not None and bv[0] != chosen[-1][2] and bv[0] != honest:
            exp = predict(kind, r, bv[0])
            chosen.append((kind + ':bound', r, bv[0], Expect(exp.outcomes, exp.message, exp.cell.replace('inject', 'inject-bound', 1) if exp.outcomes is not None else exp.cell, f'{bv[1]}; {exp.reason}')))

    def pick(kind: str, cands: list[str], fixed: Expect | None = None) -> None:
        best = None
        for r in cands:
            new = (int(mem[r]['encoded'][0]) + 1) % zv.R
            exp = fixed or predict(kind, r, new)
            if best is None or (best[1].outcomes is None and exp.outcomes is not None):
                best = (r, exp)
            if exp.outcomes is not None:
                break
        if best:
            if fixed:
                chosen.append((kind, best[0], (int(mem[best[0]]['encoded'][0]) + 1) % zv.R, best[1]))
            else:
                with_bound(kind, best[0], best[1])

    pick('inject-input', [e['name'] for e in doc['inputs'] if e['name'] in natives])
    computed, bypass, transcript = [], [], []
    for ins in doc['instructions']:
        op = ins['op']
        if op in ('public_input', 'private_input'):
            o = ins['output']
            if o in natives and o not in unconstrained:
                transcript.append(o)
            continue
        for o in _writes(ins):
            if o in natives:
                (bypass if op in BYPASS else computed).append(o)
    pick('inject-computed', computed)
    pick('inject-ignored', bypass, Expect(frozenset({A}), None, 'inject-ignored: A (D4)', 'output stored with memory.insert'))
    pick('inject-transcript', transcript)
    # D5: the Native registers K reports unconstrained (the guarded-off inputs; a
    # JubjubScalar or foreign register is not Native and cannot be injected,
    # free_cells reports its consumers instead)
    free = [r for r in k.get('unconstrained', []) if r in natives]
    best = None
    for r in free:
        new = (int(mem[r]['encoded'][0]) + 1) % zv.R
        exp = predict_unconstrained(doc, k, r, new)
        rank = 0 if exp.outcomes == frozenset({A}) else (1 if exp.outcomes is not None else 2)
        if best is None or rank < best[0]:
            best = (rank, r, exp)
        if rank == 0:
            break
    if best:
        with_bound('inject-unconstrained', best[1], best[2])
    return chosen


def instance_perturbation(k: dict, rng) -> tuple[int, list[str], Expect]:
    pis = [int(x) for x in k['pis']]
    idx = rng.randrange(len(pis))
    pis[idx] = (pis[idx] + 1) % zv.R
    return idx, [str(x) for x in pis], Expect(frozenset({C}), None, 'instance: C (D6)', f'instance[{idx}] + 1')


def judge(exp: Expect, res: dict) -> str:
    if exp.outcomes is None:
        return 'N/C'
    if res.get('outcome') in exp.outcomes and message_matches(exp.message, res.get('message', '')):
        return 'AGREE'
    return 'DISAGREE'


def describe(res: dict) -> str:
    out = res.get('outcome', '?')
    extra = ''
    if out in (X, S, W, IL, IE, 'load-error'):
        extra = ' ' + repr(res.get('message', '')[:70])
    elif out == C:
        extra = f" {len(res.get('failures', []))} failure(s)"
    return f"{out}{extra} {res.get('elapsed_ms', 0)}ms"

# Circuit comparison table (M1)

Written before the first `--circuit` run, from the spike receipt
`evidence/zkir-k-circuit-oracle-spike-2026-09-06.txt`, the oracle source
`tools/circuit-oracle/ledger-92e8bdd3/src/main.rs`, `Relation::circuit` in
`ir_vm.rs` of both pinned crates, and chapter 08 (the K outcome model). Every
comparison `tools/diff_test.py --circuit` and `tools/divergence_tests.py` makes
must land in one of the cells below. A comparison outside the table is a
blocking finding, reported in the receipt, not absorbed here. Revised for M2:
the outcome `unconstrained` and the constraint `inputGate` (chapter 08, "What
the witness space establishes") change the `ok` row, the column
`inject-guard-off` (now `inject-unconstrained`), D5 and two divergence cases.
Revised after the 2026-09-06 review (items 3, 4, 10, 11, 12, 14, 15, 17): the
row of a `violated` is realised per gate with `synthErr` precedence, the
injection columns gain `inject-transcript` and a boundary value per register,
D3 names the crate's panics where it used to name constraint failures, the
undecided walk cases are decided, D1(b) splits the foreign Edwards chip from
the Weierstrass chips, and the oracle gains the outcomes
`instance-length-mismatch` and `inject-error`.

## The two sides

K summarises a run by its status and the verdict that decides the row. The
oracle reports one of its outcomes for the same preimage, in the order the
crate produces them: `preprocess` returns `Err`, then `optimal_k` synthesises
with an unknown witness (every witness-independent `Error::Synthesis` is
unwrapped there, as a panic), then `MockProver::run` synthesises with the
known witness (a `mem_insert` or `pi_push` mismatch is the only source of
`witness-consistency-error`, and the oracle requires both its witnesses, the
`error_if_known_and` error the relation stashed and the crate's misalignment
event; any other `Err` is `synthesis-error`; a panic anywhere is `panic`),
then the instance length is compared with the number of pushes
(`instance-length-mismatch`), then `verify` reports the failing constraints
(`constraint-failure`) or `accepted`. A malformed `--inject`, `--pis` or
`--instance` file is `inject-error` with exit status 2: a harness error,
never a cell.

| K summary (row) | Meaning |
|---|---|
| `ok` | status `ok`, every verdict `holds` or `unconstrained` (the run's `witness_space` is true) |
| `ok/synthErr` | status `ok`, some verdict `synthErr` (whatever else is emitted: it is witness-independent and fires first) |
| `ok/violated` | status `ok`, some verdict `violated`, none `synthErr` |
| `ok/unknown` | status `ok`, some verdict `unknown`, none of the above |
| `ok/unsupported` | status `ok`, some verdict `unsupported`, none of the above |
| `error` | status `error` (whatever the verdicts say) |
| `panic` | status `panic` |

When several non-holding kinds occur in one run, a `synthErr` anywhere in the
verdict list decides the row: every `synthErr` is witness-independent and is
raised inside `optimal_k`, before `MockProver::run` reads any witness, so it
precedes every `violated` whatever the emission order. Among the `violated`
verdicts the first in emission order decides, because `MockProver::run` stops
at the first `mem_insert` that fails; `commGate` is evaluated last, where the
crate places the commitment check. Case `e11` (a `violated`
`from_coordinates` followed by a `synthErr` `constrain_eq`) is `X` with the
unwrap message class, and the harness expects exactly that, not the three-way
set of D1.

| Perturbation (column) | What changes | Which side sees it |
|---|---|---|
| honest | the preimage the generation loop settled on | both |
| perturbed-raw | first raw input replaced by a random field element | both |
| perturbed-typed | one declared input replaced by a random value of its type | both |
| wrong-pubin | one `public_transcript_inputs` entry + 1 | both |
| wrong-comm | commitment value + 1 | both |
| inject-input | `--inject`: a declared Native input register of the preprocessed memory set to `v + 1` | oracle only: K has no run on an injected memory |
| inject-computed | `--inject`: a Native register written by an instruction set to `v + 1` | oracle only |
| inject-ignored | `--inject`: a Native register stored by a bypass arm (D4) set to `v + 1` | oracle only |
| inject-transcript | `--inject`: a Native `public_input` / `private_input` register that is not `unconstrained` (guard 1 or no guard) set to `v + 1` | oracle only |
| inject-unconstrained | `--inject`: a Native register whose assigning verdict is `unconstrained` (a `public_input` / `private_input` whose guard evaluated to 0; its witness value is the type default 0) set to `v + 1` | oracle only |
| `<kind>:bound` | `--inject`: the same register as the `<kind>` column above set to the boundary value of its first consumer (item 12): `1 << bits` for `constrain_bits` and `div_mod_power_of_two`, `1 << bits` for the modulus and `1 << (255 - bits)` for the divisor of `reconstitute_field`, `2` for a boolean consumer (`cond_select` bit, `impact` guard, `not`, `constrain_to_boolean`, `assert`), `256` for the high and `1 << 248` for the low operand of `bytes32_from_low_high`, `x + 2` for the `x` of a native `from_coordinates` (keeps the parity), `0` for `inv`, `1 << padded` for `less_than`, `1 << 8w` for an operand of a `w`-byte atom of a byte-aligned hash; not performed for `inject-ignored`, nor when the boundary equals `v` or `v + 1` | oracle only |
| instance | `--instance`: one entry of the MockProver instance column + 1, witness untouched | oracle only |

The four preimage perturbations are runs on a different preimage: K and the
oracle both see it, K's row is the summary of *that* run, and the cell is
the same as the honest column of that row. The injection and instance
columns keep the honest witness on the K side; they are performed only when
the honest run is row `ok` on both sides (K `ok`, `preprocess` ok). The
walk of D3 predicts every injection cell; a cell the walk cannot decide is
printed as not compared with its reason and counted in the receipt's
summary, never as agreement.

## The table

Cell codes: `A` accepted, `P` preprocess-error, `W` witness-consistency-error,
`S` synthesis-error, `C` constraint-failure, `X` panic (with a message class
where one is named). `n/r` = not run (the harness does not perform that
perturbation on that row). `n/c` = not comparable (K makes no claim that the
crate's outcome can meet; printed, never counted as agreement). `D<n>` names
a difference explained below; the code before it is the outcome expected in
the cases the harness reaches.

| K summary \ perturbation | honest | perturbed-raw | perturbed-typed | wrong-pubin | wrong-comm | inject-input | inject-computed | inject-ignored | inject-transcript | inject-unconstrained | `<kind>:bound` | instance |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `ok` | A | A | A | A | A | W / C / X / A, D3 | W (or A under D4) | A, D4 | W / C / A, D3 (D5 on the consumers) | A, D5 (W / C / X under D3 when a consumer reads the cell) | W / C / X / A, D3 | C, D6 |
| `ok/synthErr` | X, D2 (message class) | X, D2 | X, D2 | X, D2 | X, D2 | n/r | n/r | n/r | n/r | n/r | n/r | n/r |
| `ok/violated` | D1: C, W, S or X per gate | D1 | D1 | D1 | D1 | n/r | n/r | n/r | n/r | n/r | n/r | n/r |
| `ok/unknown` | D7 (unreachable) | D7 | D7 | D7 | D7 | n/r | n/r | n/r | n/r | n/r | n/r | n/r |
| `ok/unsupported` | D8 (not comparable) | D8 | D8 | D8 | D8 | n/r | n/r | n/r | n/r | n/r | n/r | n/r |
| `error` | P, D9 | P | P | P | P | n/r | n/r | n/r | n/r | n/r | n/r | n/r |
| `panic` | X, D10 (message class) | X | X | X | X | n/r | n/r | n/r | n/r | n/r | n/r | n/r |

## Named differences

**D1. A K `violated` has four realisations in the crate, decided per gate.**
K says "a circuit for this instruction exists and rejects the witness"; the
crate says where. The harness (`circuit_compare.violated_cell`) maps the
deciding gate to one realisation and a message class; a gate the map does
not classify is `n/c`, never an agreement. (a) A relation realised as an
equality assertion on cells the circuit already holds is a
`constraint-failure`: `commGate` is `std.assert_equal(poseidon(...),
public_inputs[1])` at the end of `Relation::circuit`, so case `f05` is `C`;
`constrain_bits`, `constrain_eq`, `constrain_to_boolean`, `assert`, the
booleanity of an `impact` guard (`guardGate`) and the `binding` gate are `C`.
Native (Jubjub) `from_coordinates` is `C` on case `k01`: midnight-circuits'
`CircuitCurve::from_xy` for Jubjub decompresses the point from `y` and the
parity of `x` (`JubjubAffine::from_bytes`), the same function `preprocess`
uses, so a wrong `x` of the right parity reproduces the witness point,
`mem_insert` agrees, and only the chip's `assert_equal(x, point.x)` fails at
`verify`. The low operand of `bytes32_from_low_high` with byte 31 set is `C`
(`assert_equal_to_fixed(bytes[31], 0)`). (b) A relation whose in-circuit
output is recomputed and stored through `mem_insert` is a
`witness-consistency-error` when the recomputed value differs from the
witness, before `verify` runs: the arithmetic, `copy`, `encode`, `test_eq`,
the hashes (including the range failure of a byte atom, item 5, whose
decomposition hint truncates), the curve operations, `into_bytes32`,
`jubjub_scalar_from_native`, `div_mod_power_of_two`, `reconstitute_field`,
`piGate` (a `pi_push` mismatch), the extension byte and boolean operations,
`load_constant`, and `inv` of zero (`std.inv` hints 0 and `mem_insert`
compares it with the honest inverse, case `e02`). Foreign Weierstrass
`from_coordinates` (secp256k1, secp256r1) builds the point with
`from_xy(x, y).unwrap_or(identity)` and only then asserts on-curve, so a
wrong pair is `W`, not `C`; a native `from_coordinates` whose `x` has the
wrong parity decompresses to the negated point and is `W` too. (c) A
relation realised through a hint that panics is a `panic` with a message
class: native `from_coordinates` calls `from_xy(x, y).expect("Affine
coordinates must satisfy Jubjub equation.").into_subgroup()` inside the
known-witness synthesis, so a `y` with no point, or a decompressed point
outside the prime-order subgroup, is `X`; the high operand of
`bytes32_from_low_high` above 255 is `X` "AssignedByte"
(`std.convert(AssignedNative -> AssignedByte)` asserts, case `e03`); a
`less_than` operand at or above the padded bound is `X` "AssignedBounded"
(`BoundedElement::new` asserts, case `e04`); the foreign Edwards chip's
`into_subgroup` panics on a torsion point. (d) The foreign Edwards chip
(Curve25519, `ecc/foreign/edwards_chip.rs point_from_coordinates`) returns
`Error::Synthesis("invalid coordinates")` from its hint on an off-curve
pair: `S`, the one `synthesis-error` a K row can produce; unreachable today
because `preprocess` rejects the pair first (D9). The harness decides between
(a), (b), (c) and (d) for `from_coordinates` by redoing the decompression
(`circuit_compare.jubjub_from_xy`) or the curve membership. The crate's
granularity does not separate "the relation is unsatisfiable" from "the
prover aborted", and K's `violated` deliberately covers both (chapter 08,
"prover-side panics"). A `violated` on `inv` at zero, on `guardGate` or
`piGate` with a non-boolean guard, or on foreign `from_coordinates` with an
off-curve or torsion pair, is never observed by the oracle on a preimage:
`preprocess` rejects the same preimage first and the row is `error` (D9);
the injection columns reach these gates instead (D3).

**D2. A K `synthErr` is a `panic`, in one of three message classes, never a
`synthesis-error`.** Chips: `ZkStdLib::jubjub()`, `secp256k1()`, `p256()`,
`curve25519()`, `poseidon()`, `sha2_256()`, `sha2_512()` and `keccak_256()`
`expect`/`panic!` when `used_chips` did not enable them, message
"ZkStdLibArch must enable ..." (cases `f13`, `k01b`, `k04`). Widths:
`bounded_of_element` asserts `n <= MAX_BOUND_IN_BITS` with message "Cannot
bound an element with a bound ..." (case `k05`). Every other `synthErr`
(a missing dispatch arm, a type the arm rejects, an alignment option, a
declared-type mismatch on an input gate) is an `Error::Synthesis` that does
not depend on the witness, so it already occurs in `optimal_k`, whose
`cost_model` runs `DevAssembly::run(circuit).unwrap()`; the oracle therefore
reports `panic` and the harness requires both "called `Result::unwrap()`"
and "Synthesis(" in the message (cases `f06`, `f07`, `f08`, `k07`, `e11`).
The oracle's `synthesis-error` class is reachable only by a
known-witness-dependent synthesis error (a `public_input` register absent
from the injected memory, or D1(d)), which no honest K row produces. The
harness checks the panic message class as well as the outcome.

**D3. Injection into a witness-assigned Native register (declared input, or
`public_input` / `private_input` output).** The circuit reads the injected
value as its cell; the outcome is decided by what consumes the register,
and K has no verdict on it (its run is the honest one). The harness
predicts the cell with a taint walk over the instruction list and the K
memory of the honest run, following `Relation::circuit`:
- `W` when the tainted value reaches an output stored through `mem_insert`
  whose recomputed value differs from the witness (arithmetic, `copy`,
  `encode`, `cond_select` selecting the tainted arm, `test_eq` or
  `less_than` whose result flips, hashes (whatever the byte alignment: a
  value that overflows its atom is truncated by the decomposition hint and
  the digest differs, case `e10`), curve operations, `into_bytes32`,
  `jubjub_scalar_from_native`, `div_mod_power_of_two`, `reconstitute_field`,
  `inv` of 0 (the hint stores 0, case `e02`), the extension `slice`, `nth`,
  `concat`, `and`, `or`, `xor` (byte- and bit-exact when the walk knows the
  bytes)), or an `impact` whose guard is 1 (or is itself tainted) so that
  `pi_push` sees a different value. `MockProver::run` stops there, so `W`
  wins over any constraint violation before or after it.
- `X` when the tainted value reaches a hint that asserts, before `verify`:
  `less_than` with an operand at or above the padded bound `2^max(bits +
  bits mod 2, 4)` ("AssignedBounded", case `e04`), `bytes32_from_low_high`
  with a high operand above 255 ("AssignedByte", case `e03`), a native
  `from_coordinates` whose tainted `y` (or `x` parity) yields no subgroup
  point (D1c).
- `C` when the tainted value reaches only constraints and violates one:
  `assert` of 0 (a non-boolean non-zero passes, finding 2), `constrain_eq`
  against an untainted register, `constrain_bits` and
  `constrain_to_boolean` out of range, a non-boolean bit or guard into
  `cond_select`, `not`, `impact` (the `convert` hint takes any non-zero as 1
  and `assert_equal(x, bit)` plus the bit cell's booleanity fail at
  `verify`; when that choice selects the honest arm nothing else happens and
  the outcome is `C`, when it selects the other arm the `mem_insert` or
  `pi_push` is `W` first), `bytes32_from_low_high` with byte 31 set (case
  `e03b`), the commitment equality when a declared input or an output is
  tainted, or a native `from_coordinates` whose tainted `x` keeps the parity
  (the decompressed point agrees, `assert_equal` on `x` fails).
- `A` when nothing in the circuit constrains the cell: the register is
  unused, or every consumer discards it (`cond_select` selecting the other
  arm, `mul` by 0, an `impact` whose guard is 0, a bypass path of D4).
The walk refuses to decide (`undecided`, listed in the receipt and never
counted) only when a tainted value of unknown bytes enters `test_eq` or
`constrain_eq` on both sides; the harness prefers a register with a decided
prediction, and every `<kind>:bound` injection is predicted the same way.

**D4. Ignored injection.** `from_bytes32`, `reverse_bytes` (`reverse` at
2ffe2d1), `bytes32_into_low_high` and `bytes32_from_low_high` store their
outputs with `memory.insert`, not `mem_insert`; an injected value on such
an output is never compared and the in-circuit cell keeps the recomputed
value, so the outcome is `A` (spike finding 4). Of these only a Native
output (`from_bytes32` to Native, both outputs of `bytes32_into_low_high`)
can be injected. The harness injects one such register when the program
has one and expects `A`; no boundary injection is performed on it.

**D5. An `unconstrained` register is pinned to its type only by its gate;
its freedom is decided by its consumers.** In circuit the guard of a
`public_input` or `private_input` is not read (`guard: _`, `Relation::circuit`
`ir_vm.rs:977-1003`): for every guard and every type the instruction assigns
the register from `preproc.memory` through `assign_incircuit`, stores it with
`mem_insert` (whose comparison is against the same witness entry), pushes
nothing and asserts nothing against the transcript. K's gate reports
`unconstrained` at guard 0 (M2), where the witness holds the type's default
and the transcript has no entry, so the run stays in row `ok` and
`tools/zkir_run.py` lists the register under `unconstrained`. The marker is
gate-level: it says the relation does not pin the cell beyond its type, not
that the cell is free in the circuit. Whether it is free is decided by the
consumers, which the walk of D3 follows: the oracle must say `A` when
nothing in the circuit reads the cell (spike finding 3: `%amount.13`, and the
receipt's `%color.11`, both in `expire.zkir`; case `e01a` for a guard-1
register), and when a consumer reads it the injected memory is outside the
modelled witness space and the D3 sub-cells `W`, `C` and `X` apply (case
`e01b`: an `impact` consumer, `W`). The harness prefers a register no
consumer reads, and the receipt's witness-space summary classifies every
`unconstrained` register, Native or not, as `free` or `read by #i op`. The
typical Compact pattern feeds the register to an `impact` under the same
(off) guard or a `cond_select` on the same bit, which discards it. The
`inject-transcript` column applies the same walk to a transcript register
that is not `unconstrained` (guard 1 or no guard): the transcript binding is
a residual (the public transcript is enforced by the ledger's re-execution,
the private transcript by nobody), so the gate never makes the cell hold the
ledger's value; `W` in that column comes from a consumer, never from the
input gate. The other `unconstrained` verdicts, JubjubScalar canonicity and
foreign limb canonicity on `inputGate`, `public_input` and `private_input`
of those types, cannot be tested by injection: `--inject` takes Native values
only, and the crate's typed `IrValue` cannot carry the non-canonical
representative that the free bits or limbs admit.

**D6. Instance perturbation.** Every pushed public input is
`constrain_as_public_input`, an equality between an advice cell and the
instance column; the witness is honest so no `mem_insert`/`pi_push` fires
and `verify` reports the two cells (spike: an advice cell in the region of
the assignment and the instance cell). Always `C` on an `ok` row. The
instance column must have the length of the push sequence: a longer column
leaves the surplus cells unconstrained (`accepted`) and a shorter one fails
on the first missing cell (`C`), neither of which says anything about the
witness, so the oracle reports `instance-length-mismatch` for both (item 14;
the same for a `--pis` vector of another length). `--pis` and
`--binding-input` are witness-side perturbations: the instance column
follows them, a changed entry surfaces in `pi_push` as `W`; they do not
model a verifier handed a wrong statement.

**D7. `ok/unknown` is unreachable.** `unknown` needs a register or
public-input slot the gate reads to be absent, which only happens after an
off-circuit failure, and then the status is `error`. If an `ok/unknown` row
appears it is a finding about the K emission rules, and the oracle outcome
cannot be compared to it.

**D8. `ok/unsupported` cannot be compared.** No in-circuit relation is
modelled, so K makes no claim; the oracle outcome is recorded and neither
counted as agreement nor as disagreement. Not reached on the corpus (no
`[owise]` gate is hit by any corpus program).

**D9. `error` hides the verdicts.** The crate's `preprocess` rejects the
preimage before a circuit exists, so the oracle is `P` regardless of what
the gates say: `inv` of zero is `violated` in K and `P` in the oracle (no
circuit is ever built for that witness; the injection of case `e02` reaches
the circuit instead); `less_than` beyond an odd bound is `unknown` and `P`;
an `output` arity mismatch is `synthErr` on `outputGate` and `P`; a hash
operand above its byte atom is `violated` on the hash gate and `P` (case
`e10`, whose injection is `W`). The verdict layer on an error run is only
checked for internal consistency (`divergence_tests.py`), never against the
crate. The `format-error` rows (a program `IrSource::load` rejects) are
`load-error` on the oracle side, exit 2, and are not circuit comparisons.

**D10. `panic` is a preprocess panic, in one of two message classes.** Cases
`k02` and `k03` panic inside `preprocess`; the oracle's `catch_unwind` wraps
`preprocess` as well, so the outcome is `X` with the crate's message: "range
end index 1 out of range for slice of length 0" (a transcript indexed past
its end) or "assertion `left == right` failed" (the `assert_eq!` of the
Bytes32 decoder), not the D2 message classes; the harness requires one of
the two.

## What the crate cannot distinguish

- `violated` versus a prover abort (D1c): the same K outcome maps to `C`,
  `W`, `S` or `X` depending on how midnight-circuits realises the relation;
  the harness carries the realisation per gate, and a gate it does not
  classify is `n/c`.
- a byte or bounded conversion (D1c, D3): `bytes32_from_low_high` with a
  high operand above 255 and `less_than` with an operand at or above the
  padded bound are panics of the conversion hints ("Trying to convert ... to
  AssignedByte", "... to an AssignedBounded less than 2^n"), not constraint
  failures; K reports both as `violated` on the gate, and the message class
  is what tells them from a D2 unwrap.
- `synthErr` versus `unsupported`-in-crate: K's `synthErr` maps to `X` in
  all three message classes (D2); a K `unsupported` gate would meet the
  crate's `Error::Synthesis("Unsupported ...")` on the same path and also
  be `X`, so `X` alone does not tell the two rows apart; the message does.
- `holds` on an error run: never observed (D9). `holds` on `assert` of a
  non-boolean non-zero (finding 2) is `P` on the honest preimage and can be
  observed only through an injection (D3, `A`).
- The instruction that fails: `verify` names regions and columns, not ZKIR
  instructions (spike finding 7); the harness compares outcomes, not
  locations.
- A non-canonical JubjubScalar: the circuit's 252 assigned bits admit a
  representative in `[r_J, 2^252)`, the typed witness interface does not,
  so the `unconstrained` verdict on a witness-assigned JubjubScalar has no
  oracle column (D5).
- A non-canonical foreign limb tuple (item 3): `FieldChip::assign`
  (`field_chip.rs:424-455`) range-checks the limbs against
  `well_formed_log2_bounds`, chosen so that the assigned integer is below
  twice the modulus, not below it; the typed `IrValue` cannot carry such a
  tuple, so the `unconstrained` verdict on a witness-assigned foreign field
  element or point has no oracle column either (D5). For the foreign field
  chip `assert_equal` is limb equality after conditional normalisation,
  which is value equality on canonical representatives.

## Divergence cases

The expected oracle outcome added to each case of `divergence_tests.py`
(an injection case runs the oracle on its honest preimage with `--inject`
and the walk's prediction must agree):

| Case | K | Oracle | Cell |
|---|---|---|---|
| f02 | error, `assert` holds | P | D9 |
| f03 | ok, `public_input` unconstrained | A | `ok`/honest, D5 |
| f04 | error, `less_than` unknown | P | D9 |
| f05 | ok, `commGate` violated | C | D1a |
| f06 | ok, `cond_select` synthErr | X (unwrap of Synthesis) | D2 |
| f07 | ok, `constrain_eq` synthErr | X (unwrap of Synthesis) | D2 |
| f08 | ok, `bytes32_from_low_high` synthErr | X (unwrap of Synthesis) | D2 |
| f10 | wfError, Rust error | P | D9 |
| f11 | error, `from_coordinates` violated | P | D9 |
| f12 | error, `ec_mul_generator` synthErr | P | D9 |
| f13 | ok, `from_bytes32` synthErr | X (must enable secp256k1) | D2 |
| k01 | ok, `from_coordinates` violated | C (assert_equal on x after decompression) | D1a |
| k01b | ok, `from_coordinates` synthErr | X (must enable jubjub) | D2 |
| k03 | panic | X (assertion `left == right` failed) | D10 |
| k04 | ok, `jubjub_scalar_from_native` synthErr | X (must enable jubjub) | D2 |
| k05 | ok, `less_than` synthErr | X (Cannot bound an element) | D2 |
| k06 | error, `guardGate` violated | P | D9 |
| k07 | ok, `persistent_hash` synthErr | X (unwrap of Synthesis) | D2 |
| k02 | panic | X (range end index out of range) | D10 |
| k08 (extension surface, 2ffe2d1 oracles) | ok, `load_constant` synthErr | X (must enable jubjub) | D2 |
| f01 | error, `reconstitute_field` unknown | P | D9 |
| e10 (inject `%x` -> 300 into a `bytes 1` atom) | error, `persistent_hash` violated | W | D9 on the preimage, D3 `W` under injection |
| e02 (inject `%a` -> 0) | ok, `inv` holds | W | D3 `W` (the hint stores 0) |
| e03 (inject `%hi` -> 256) | ok, `bytes32_from_low_high` holds | X (AssignedByte) | D3 `X` |
| e03b (inject `%lo` -> 2^248) | ok, `bytes32_from_low_high` holds | C | D3 `C` |
| e04 (inject `%a` -> 16, bits 4) | ok, `less_than` holds | X (AssignedBounded) | D3 `X` |
| e11 | ok, `from_coordinates` violated then `constrain_eq` synthErr | X (unwrap of Synthesis) | D2, synthErr precedence |
| e01a (inject a guard-1 `public_input` no instruction reads) | ok, `public_input` holds | A | D5 consumers, `inject-transcript` |
| e01b (the same register consumed by an `impact`) | ok, `public_input` holds | W | D5 consumers, `inject-transcript` |
| e21a, e21b (`div_mod_power_of_two` / `reconstitute_field` with 249 bits) | wfError, Rust error | P | D9 (keygen keys both: provability receipt) |

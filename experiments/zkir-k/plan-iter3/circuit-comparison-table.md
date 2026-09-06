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

## The two sides

K summarises a run by its status and the worst verdict of the emitted
gates. The oracle reports one of six outcomes for the same preimage, in the
order the crate produces them: `preprocess` returns `Err`, then
`optimal_k` synthesises with an unknown witness, then `MockProver::run`
synthesises with the known witness (a `mem_insert` or `pi_push` mismatch is
the only source of `witness-consistency-error`; any other `Err` is
`synthesis-error`; a panic anywhere is `panic`), then `verify` reports the
failing constraints (`constraint-failure`) or `accepted`.

| K summary (row) | Meaning |
|---|---|
| `ok` | status `ok`, every verdict `holds` or `unconstrained` (the run's `witness_space` is true) |
| `ok/violated` | status `ok`, some verdict `violated` (none `synthErr`) |
| `ok/synthErr` | status `ok`, some verdict `synthErr` |
| `ok/unknown` | status `ok`, some verdict `unknown`, none of the above |
| `ok/unsupported` | status `ok`, some verdict `unsupported`, none of the above |
| `error` | status `error` (whatever the verdicts say) |
| `panic` | status `panic` |

When several non-holding kinds occur in one run the first non-holding
verdict in emission order decides the row, because `MockProver::run` stops
at the first instruction that fails; `commGate` is evaluated last, where the
crate places the commitment check.

| Perturbation (column) | What changes | Which side sees it |
|---|---|---|
| honest | the preimage the generation loop settled on | both |
| perturbed-raw | first raw input replaced by a random field element | both |
| perturbed-typed | one declared input replaced by a random value of its type | both |
| wrong-pubin | one `public_transcript_inputs` entry + 1 | both |
| wrong-comm | commitment value + 1 | both |
| inject-input | `--inject`: a declared Native input register of the preprocessed memory set to `v + 1` | oracle only: K has no run on an injected memory |
| inject-computed | `--inject`: a Native register written by an instruction set to `v + 1` | oracle only |
| inject-unconstrained | `--inject`: a Native register whose assigning verdict is `unconstrained` (a `public_input` / `private_input` whose guard evaluated to 0; its witness value is the type default 0) set to `v + 1` | oracle only |
| instance | `--instance`: one entry of the MockProver instance column + 1, witness untouched | oracle only |

The four preimage perturbations are runs on a different preimage: K and the
oracle both see it, K's row is the summary of *that* run, and the cell is
the same as the honest column of that row. The injection and instance
columns keep the honest witness on the K side; they are performed only when
the honest run is row `ok` on both sides (K `ok`, `preprocess` ok).

## The table

Cell codes: `A` accepted, `P` preprocess-error, `W` witness-consistency-error,
`S` synthesis-error, `C` constraint-failure, `X` panic. `n/r` = not run
(the harness does not perform that perturbation on that row). `D<n>` names a
difference explained below; the code before it is the outcome expected in the
cases the harness reaches.

| K summary \ perturbation | honest | perturbed-raw | perturbed-typed | wrong-pubin | wrong-comm | inject-input | inject-computed | inject-unconstrained | instance |
|---|---|---|---|---|---|---|---|---|---|
| `ok` | A | A | A | A | A | W / C / A, D3 | W, or A under D4 | A, D5 (W / C under D3 when a consumer reads the cell) | C, D6 |
| `ok/violated` | D1: C, W or X | D1 | D1 | D1 | D1 | n/r | n/r | n/r | n/r |
| `ok/synthErr` | X, D2 | X, D2 | X, D2 | X, D2 | X, D2 | n/r | n/r | n/r | n/r |
| `ok/unknown` | D7 (unreachable) | D7 | D7 | D7 | D7 | n/r | n/r | n/r | n/r |
| `ok/unsupported` | D8 (not comparable) | D8 | D8 | D8 | D8 | n/r | n/r | n/r | n/r |
| `error` | P, D9 | P | P | P | P | n/r | n/r | n/r | n/r |
| `panic` | X, D10 | X | X | X | X | n/r | n/r | n/r | n/r |

## Named differences

**D1. A K `violated` has three realisations in the crate.** K says "a
circuit for this instruction exists and rejects the witness"; the crate says
where. (a) A relation realised as an equality assertion on cells the
circuit already holds is a `constraint-failure`: `commGate` is
`std.assert_equal(poseidon(...), public_inputs[1])` at the end of
`Relation::circuit`, so case `f05` is `C`. Native (Jubjub)
`from_coordinates` is also `C` on case `k01`: midnight-circuits'
`CircuitCurve::from_xy` for Jubjub decompresses the point from `y` and the
parity of `x` (`JubjubAffine::from_bytes`), the same function `preprocess`
uses, so a wrong `x` of the right parity reproduces the witness point,
`mem_insert` agrees, and only the chip's `assert_equal(x, point.x)` fails at
`verify`. (b) A relation whose in-circuit output is recomputed and stored
through `mem_insert` is a `witness-consistency-error` when the recomputed
value differs from the witness, before `verify` runs: foreign
`from_coordinates` builds the point with `from_xy(x, y).unwrap_or(identity)`
and only then asserts on-curve, so a wrong pair is `W`, not `C`; a native
`from_coordinates` whose `x` has the wrong parity decompresses to the
negated point and is `W` too. (c) A relation realised through a hint that
unwraps is a `panic`: native `from_coordinates` calls
`from_xy(x, y).expect("Affine coordinates must satisfy Jubjub equation.")
.into_subgroup()` inside the known-witness synthesis, so a `y` with no
point, or a decompressed point outside the prime-order subgroup, is `X`.
The harness decides between (a), (b) and (c) for native `from_coordinates`
by redoing the decompression (`circuit_compare.jubjub_from_xy`). The
crate's granularity does not separate "the relation is unsatisfiable" from
"the prover aborted", and K's `violated` deliberately covers both (chapter
08, "prover-side panics"). A `violated` on `inv` at zero, on `guardGate` or
`piGate` with a non-boolean guard, or on foreign `from_coordinates` with an
off-curve or torsion pair, is never observed by the oracle: `preprocess`
rejects the same preimage first and the row is `error` (D9).

**D2. A K `synthErr` is a `panic`, in one of three message classes, never a
`synthesis-error`.** Chips: `ZkStdLib::jubjub()`, `secp256k1()`, `p256()`,
`curve25519()`, `poseidon()`, `sha2_256()`, `sha2_512()` and `keccak_256()`
`expect`/`panic!` when `used_chips` did not enable them, message
"ZkStdLibArch must enable ..." (cases `f13`, `k01b`, `k04`). Widths:
`bounded_of_element` asserts `n <= MAX_BOUND_IN_BITS` with message "Cannot
bound an element with a bound ..." (case `k05`). Every other `synthErr`
(a missing dispatch arm, a type the arm rejects, an alignment option) is an
`Error::Synthesis` that does not depend on the witness, so it already
occurs in `optimal_k`, whose `cost_model` runs `DevAssembly::run(circuit)
.unwrap()`; the oracle therefore reports `panic` with message "called
`Result::unwrap()` on an `Err` value: Synthesis(...)" (cases `f06`, `f07`,
`f08`, `k07`). The oracle's `synthesis-error` class is reachable only by a
known-witness-dependent synthesis error (for example a `public_input`
register absent from the injected memory), which no K row produces. The
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
  `less_than` whose result flips, hashes, curve operations,
  `into_bytes32`, `jubjub_scalar_from_native`, `div_mod_power_of_two`,
  `reconstitute_field`), or an `impact` whose guard is 1 (or is itself
  tainted) so that `pi_push` sees a different value. `MockProver::run` stops
  there, so `W` wins over any constraint violation before or after it.
- `C` when the tainted value reaches only constraints and violates one:
  `assert` of 0 (a non-boolean non-zero passes, finding 2), `constrain_eq`
  against an untainted register, `constrain_bits` and
  `constrain_to_boolean` out of range, `impact` or `cond_select` or `not`
  with a non-boolean guard or bit, `less_than` operands above the padded
  bound, `bytes32_from_low_high` with byte 31 set or a high above 255, the
  commitment equality when a declared input or an output is tainted, or
  an `inv` whose operand became 0.
- `A` when nothing in the circuit constrains the cell: the register is
  unused, or every consumer discards it (`cond_select` selecting the other
  arm, `mul` by 0, an `impact` whose guard is 0, a bypass path of D4).
The walk refuses to decide (`undecided`, not counted) when a tainted value
enters an operation whose in-circuit hint it cannot evaluate: a non-boolean
bit into `cond_select` or `not`, a hash whose byte alignment the new value
no longer fits, `slice`/`nth`/`concat` over tainted bytes, `inv` of 0. The
harness prefers a register with a decided prediction.

**D4. Ignored injection.** `from_bytes32`, `reverse_bytes` (`reverse` at
2ffe2d1), `bytes32_into_low_high` and `bytes32_from_low_high` store their
outputs with `memory.insert`, not `mem_insert`; an injected value on such
an output is never compared and the in-circuit cell keeps the recomputed
value, so the outcome is `A` (spike finding 4). Of these only a Native
output (`from_bytes32` to Native, both outputs of `bytes32_into_low_high`)
can be injected. The harness injects one such register when the program
has one and expects `A`.

**D5. An `unconstrained` register is a free cell.** In circuit the guard of
a `public_input` or `private_input` is not read (`guard: _`); when it is 0
the register is assigned from the witness, pushed to no public input, and
its `mem_insert` trivially agrees. K's gate for the instruction checks the
register's type and chip and then reports `unconstrained` (M2), so the run
stays in row `ok` and `tools/zkir_run.py` lists the register under
`unconstrained`. The harness injects one such Native register: the oracle
must say `A` when nothing in the circuit reads the cell (spike finding 3:
`%amount.13`, and the receipt's `%color.11`, both in `expire.zkir`), which is
the soundness claim of chapter 08 on a free cell. When a consumer of D3
reads it, the injected memory is outside the modelled witness space and the
D3 sub-cells `W` and `C` apply; the harness prefers a register no consumer
reads. The typical Compact pattern feeds the register to an `impact` under
the same (off) guard or a `cond_select` on the same bit, which discards it.
The other `unconstrained` verdict, JubjubScalar canonicity on `inputGate`,
`public_input` and `private_input` of that type, cannot be tested by
injection: `--inject` takes Native values only, and the crate's typed
`IrValue::JubjubScalar(JubjubFr)` cannot carry the non-canonical
representative that the free bits admit.

**D6. Instance perturbation.** Every pushed public input is
`constrain_as_public_input`, an equality between an advice cell and the
instance column; the witness is honest so no `mem_insert`/`pi_push` fires
and `verify` reports the two cells (spike: an advice cell in the region of
the assignment and the instance cell). Always `C` on an `ok` row.

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
circuit is ever built for that witness); `less_than` beyond an odd bound is
`unknown` and `P`; an `output` arity mismatch is `synthErr` on `outputGate`
and `P`. The verdict layer on an error run is only checked for internal
consistency (`divergence_tests.py`), never against the crate. The
`format-error` rows (a program `IrSource::load` rejects) are `load-error`
on the oracle side, exit 2, and are not circuit comparisons.

**D10. `panic` is a preprocess panic.** Cases `k02` and `k03` panic inside
`preprocess`; the oracle's `catch_unwind` wraps `preprocess` as well, so
the outcome is `X` with the crate's message ("range end index 1 out of
range for slice of length 0", the `assert_eq!` of the Bytes32 decoder), not
the D2 message classes.

## What the crate cannot distinguish

- `violated` versus a prover abort (D1c): the same K outcome maps to `C`,
  `W` or `X` depending on how midnight-circuits realises the relation.
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

## Divergence cases

The expected oracle outcome added to each case of `divergence_tests.py`:

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
| k03 | panic | X (decoder assert) | D10 |
| k04 | ok, `jubjub_scalar_from_native` synthErr | X (must enable jubjub) | D2 |
| k05 | ok, `less_than` synthErr | X (Cannot bound an element) | D2 |
| k06 | error, `guardGate` violated | P | D9 |
| k07 | ok, `persistent_hash` synthErr | X (unwrap of Synthesis) | D2 |
| k02 | panic | X (range end index out of range) | D10 |
| k08 (extension surface, 2ffe2d1 oracles) | ok, `load_constant` synthErr | X (must enable jubjub) | D2 |
| f01 | error, `reconstitute_field` unknown | P | D9 |

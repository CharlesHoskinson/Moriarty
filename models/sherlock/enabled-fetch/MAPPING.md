# Separate enabled scalar fetch — G3 source candidate

Status: source only, not registered; constructor tests and all SDK/native checks
are UNRUN. This is the separate enabled prerequisite for generic machine rule
families. It does not implement dispatch, a full machine or an adopted layout.
The unconditional `emit_fetch_block` file remains byte-identical to G2.

## Owner requirements and integration

This implementation materializes Opus 5.5's
`machine-fetch-security-and-atlas-opus-r2/ENABLED-FETCH-OWNER-REQUIREMENTS.md`,
SHA256 `21ab71fa53a05dc9c041d128c02517c84b0a9b1ad3dff2583776bf812a2a1423`.
Security requirements and rejection-test prescriptions are Opus-authored;
this source is not a new security judgment, ABI choice or numeric admission.

`emit_enabled_fetch_block(shape, inputs, scope)` returns an instruction fragment,
ordered enabled typed operands, and exact preserved aliases. It uses the pinned
SDK `Operand` and `Instruction` types at Midnight revision
`04c9c5d9bcebb8d4427d8589fb54d58a55599c14`. The existing backend serializes its
IrSource, sets version 3.0 and loads it; this fragment stops before that boundary.
No serialization, SDK loading/checking/evaluation or proof is performed here.

Bind all input operands first. The dispatch owner must supply `enable` from the
constrained selector of the actual pre-state branch that consumes this fetch,
never a free prover witness or expected data. Constraining it Boolean cannot
establish that provenance. The dispatch owner still owes the composition test
that forcing zero in an active branch is unsatisfiable.

Count and every row cell must be well-formed state/program operands in every
branch. Branch-specific temporaries are outside this constructor's contract
unless the caller ensures their unconditional validity. Consumers must gate
every write or assertion that uses selected values with the returned same enable,
for example `CondSelect(enable, new, old)`. No host-computed after-state is used.

## Operand and instruction map

N is the nonzero table capacity; C is the nonzero ordered column count from the
existing `TableShape`. Kinds remain UInt128 and Bool. Shape/name/overflow errors
retain their existing meanings; concrete resource admission stays external.

| Input field | Instruction use | Returned alias |
|---|---|---|
| enable | unconditional Boolean; effective-index and selector gating; guarded prefix; selector-sum RHS | preserved.enable; every selected[c].enable |
| active_rows | unconditional UInt128; capacity and prefix comparisons | preserved.active_rows |
| index | raw Native operand, only the true arm of effective-index select | preserved.index (raw, never index_eff) |
| rows[r][c] | unconditional column-kind constraint; selected-column chain input | preserved.rows[r][c] |

The fragment emits these instruction edges in order:

1. Boolean constrain enable; UInt128 constrain active_rows; constrain every row
   cell by its column kind, including inactive rows.
2. `index_eff = CondSelect(enable, index, 0)`; UInt128 constrain index_eff.
3. `exceeds = LessThan(N, active_rows, 128)`; Boolean constrain exceeds;
   constrain `exceeds == 0`. This remains unconditional.
4. `in_prefix = LessThan(index_eff, active_rows, 128)`; Boolean constrain it;
   `guarded = CondSelect(enable, in_prefix, 1)`; Assert guarded.
5. For each r in row order, `eq_r = TestEq(index_eff, r)` with Boolean constraint;
   `sel_r = CondSelect(enable, eq_r, 0)` with Boolean constraint; accumulate
   `sum = Add(previous_sum, sel_r)` from zero. Constrain final sum == enable.
6. For each c in column order, start a zero accumulator; for each r emit
   `CondSelect(sel_r, rows[r][c], previous_accumulator)`. Constrain the final
   value by the original column kind. Return it tagged with that exact enable.

Ungated equality results never drive a selected lane. The raw index receives
no separate range check, equality check or comparison. Disabled-zero means
UInt 0 or Bool false produced by the gated construction; it is not a fetched
row, default row or canonical padding. Count zero is permitted in a disabled
branch by the prescribed guarded prefix. Actual SDK acceptance is unobserved.

No public input/output order or binding policy is introduced. Selected vector
order is the existing column order. The preserved struct carries enable, count,
raw index and the complete original row matrix without a rewrite, success flag
or supplied after-state. The caller may compose this fragment into its own
IrSource after the required integration and execution reviews.

## Names and exact temporary arithmetic

Caller Variables must already start with `%`, including enable. The final
reserved namespace is `%{scope}.temp.`; scope is inserted literally after `%`
and is normally supplied without `%`. Empty scope and colliding input Variables
are rejected as before. Immediates have no variable-name rule.

The composer must allocate distinct, non-nested namespaces, including across
enabled and unconditional fetches and enclosing instructions. For example,
`a` and `a.temp.1` have overlapping reserved prefixes and cannot coexist.
The constructor checks only its input aliases; no global registry is provided.

| Temporary serial | Definition |
|---|---|
| 0 | index_eff |
| 1 | exceeds |
| 2 | in_prefix |
| 3 | guarded assertion operand |
| 4+3r | eq_r |
| 5+3r | sel_r |
| 6+3r | row r's cumulative sum |
| 4+3N+cN+r | column c's row r accumulator |

There are exactly `4 + N*(C+3)` fresh identifiers. Checked host multiplication
and addition cover that expression before any fragment identifier allocation.
There are `12 + 2*N*C + 5*N + C` emitted instructions: fixed checks, input and
selected-row chains, selector triples and final column-kind constraints.
These are source-construction counts, not measured circuit rows, degree, k,
memory budgets or numeric resource approval.

## Independent positive data and UNRUN tests

`expected.json` is copied unchanged from Root's independently frozen
`enabled-fetch-independent-normal-r1/expected.json`, SHA256
`53045ce217e8abbfff8eeb6996db996290d20fc058bee28e002f70808e3c2d63`.
It was frozen before this enabled constructor was implemented. Its four positive
cases cover enabled rows 0 and 1, disabled empty prefix with raw index 2^128,
and disabled present row zero. False and numeric zero remain explicit.
Expected selected values are never constructor inputs.

The raw index 340282366920938463463374607431768211456 remains a decimal string
in the fixture. No u128 witness adapter is added. A future admitted actual
consumer must bind this field value without truncation or using G2's UInt128
witness packer. The construction test supplies a raw Variable to inspect alias
preservation; it does not evaluate that fixture case.

Five UNRUN tests inspect effective-index and guarded-prefix edges, all gated
selector/sum/lane chains, unconditional checks, exact aliases and enable tags,
exact temporary order/count, owner-prescribed identifier rejection paths
including enable, and fixture metadata. They do not check satisfiability.

The owner-prescribed SDK checks remain UNRUN: enabled cases matching the G
fixture, disabled zero-count/arbitrary-index all-zero selected values, enabled
out-of-prefix rejection, non-Boolean enable rejection, and active_rows>N
rejection for either enable. Dispatch-level forced-zero rejection belongs to
its future composer. These checks require separate admission; no runner or
execution command is introduced here.

## Retained gaps and complete roadmap

Byte/w-bit lanes and their range checks, enable-safe multi-byte id
reconstitution, canonical padding, ProgramBlock/StateBlock binding and step
chaining (L04), transcript adapter and actual SDK checks remain open. No byte
width, privacy, binding or padding policy is added. Full machine layout and
rule integration are separate from this scalar prerequisite.

All current-A fields and rule families, B helper/microstep work and financial
phase, effects, liability and await behavior remain required. Full U0–U7,
M1–M17 and MC/SP/G obligations, complete ACTUS/DeFi conformance, native proofs,
recursive/private history and separate target qualification remain unchanged.
This source closes no machine, financial, proof or target acceptance gate.

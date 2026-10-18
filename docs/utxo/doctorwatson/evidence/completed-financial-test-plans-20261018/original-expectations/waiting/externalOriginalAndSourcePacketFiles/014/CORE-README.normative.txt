SOURCE crates/mori-core/README.md L1-94
# Native Moriarty core language

`mori-core` parses the explicit `language "moriarty/1";` profile into a public,
serde-compatible flat IR and simulates source-authored actions. The existing
`profile "moriarty-beta/1";` frontend is separate. This crate has no fixture
lookup or application dispatch table.

```mori
language "moriarty/1";
agreement Counter {
  state count: UInt = 0;
  action increment(amount: UInt) {
    let total = count + amount;
    requires total <= 100;
    next count = total;
    ensures count == total;
    emit "count"(count);
  }
}
```

State initializers are UInt or Bool literals. Action inputs appear in the
signature, or as `input name: UInt;` declarations before statements. Names
resolve in statement order. `let` captures a value; `next` updates state
immediately for subsequent expressions. Every `requires` and `ensures` is
checked where written. `emit "name"(expression);` records an explicit event.

UInt arithmetic supports `+`, `-`, `*` with checked u128 semantics. Comparisons
support `==`, `<`, `<=`, `>`, `>=`; Bool supports `and`, `or`, and `not`.
Parentheses control precedence. Boolean operations evaluate both operands.
Values are finite; no loops, recursion, division, native calls or dynamic
account selection are implemented. A condition can guard a payment with
`requires approved;`; branch statements are unsupported.

An asset declaration supplies an exact network, issuer, reference and scale:

```mori
asset USD = {
  network: "preview", issuer: "issuer", reference: "usd", scale: 2
};
```

All financial amounts are integral atomic units. Scale describes the asset
identity and does not rescale expressions. For example, with scale 2, the
amount `125` represents 1.25 display units. Decimal literals refuse.

`transfer(USD, "alice", "bob", amount);` debits and credits that exact asset.
`fee(USD, "alice", "treasury", amount);` has the same balance semantics and a
separate fee effect. Scenarios provide explicit balances for both accounts,
including zero balances. Self-transfers retain their gross effect and require
sufficient funds. No mint, burn or implicit account creation occurs.

`liability("delivery", USD, "bob", "alice", amount);` creates or adds to an
identified residual duty. Its exact asset and parties cannot change.
`discharge("delivery", amount);` reduces that duty with checked subtraction.
Discharge records a duty reduction; authors express any corresponding payment
as a separate transfer. Zero residual duties remain in the snapshot.

`liability_present("delivery")` returns Bool and `liability_amount("delivery")`
returns UInt atomic units. Each read accepts one quoted literal name and requires
an unambiguous source-wide identity from a `liability` statement. A declaration
in another action is sufficient. Unknown, empty or conflicting read identities
refuse validation. Calls with dynamic names or extra arguments refuse parsing.
These spellings remain ordinary identifiers when used without a call.

Both reads inspect the current snapshot at the enclosing statement. A missing
row returns false/0. A present-zero tombstone returns true/0. Reads after creation
or discharge see the updated duty. A `let` captures its earlier value, and a
`next` right-hand side reads before the state update. `requires` and `ensures`
check where written. Reads add no effects and do not authenticate supplied duties.
Legacy Core external duties and discharge-only programs retain their rules.


`Scenario` contains `state`, `inputs`, `balances` and `liabilities`. An empty
state map selects declared initial values; a nonempty map supplies exactly
all declared state values. Inputs must exactly match the selected action.
UInt JSON values and amounts are canonical decimal strings, including values
above JavaScript's integer range. Duplicate balances, unknown exact assets,
wrong types and unknown fields refuse.

`simulate` validates public IR, clones the scenario, and returns canonical
pre/post snapshots, ordered statement snapshots and complete effects. A
failure returns a coded error with the failed step and commits no effects.
`replay` recomputes the action and reports its first step divergence, then
checks the complete effect sequence and poststate. Balance and liability
ordering is canonical, independent of input JSON order.

`replay_detailed` and `compare_execution` add first-step and RFC6901 field
paths through a separate API. They preserve existing `replay` diagnostics and
Step/Execution JSON. Execution comparison visits semantics/action/pre, ordered
steps (index/statement/value/snapshot/effects), full effects and post. Nested
object keys are lexical; arrays retain order. Generic array length differences
report the array path; missing/extra steps report their first ordinal. The
backend's [public replay sidecar](../mori-backend/README.md) prefixes these


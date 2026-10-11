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

The language bounds source to 65,536 bytes, 8,192 tokens and expression nodes,
256 declarations, 256 inputs per action, 1,024 statements per action and
expression depth 64. Scenario bounds and a trace allocation budget limit
snapshot growth. These are local interpreter resource limits.

Run `cargo test -p mori-core --locked`. Tests exercise independently authored
counter and financial programs, late rollback, checked arithmetic, exact
schemas, asset separation, partial liability discharge, tampered replay and
malformed source. The files in `../../examples/core/language` are examples consumed by the
same parser as every developer program.

This interpreter demonstrates local language semantics. It does not establish
Midnight proof generation, ledger settlement, phase rollback, external
attestation correctness or private recursive composition. Owner intent and
resource policy are separate APIs; callers must explicitly apply them.

The current artifact, ABI, relation, proof and binding revisions are explicitly
`moriarty-zkir-artifact/3`, `moriarty-public-abi/3`, `moriarty-zkir-relation/3`,
`moriarty-native-transition-proof/3` and `moriarty-native-transition-binding/3`.
Each carries an explicit authorization mode: `unsigned`, or
`moriarty-jubjub-owner-intent/1` when the trusted Target has a `signed_policy`.
Current consumers refuse revision 1 and 2 artifacts and proofs; unchanged source
can recompile under revision 3. Historical revision 1 and 2 measurements and
native results retain their original scope. The signed profile's contract is
[the native owner-intent design](superpowers/specs/2026-10-11-native-owner-intent-design.md);
its native proof results remain separate from ordinary checks.

Each program registers zero through sixteen unique duty names across all its
actions. A name binds the exact asset network/issuer/reference/scale, debtor and
creditor. Identical declarations share one slot; conflicting tuples, unresolved
discharges and a seventeenth name refuse. Every action publishes every registered
opening and closing presence/UInt128 amount slot, including untouched duties.
Absent means `(false, 0)`; present-zero is distinct and consumes capacity.
Accrual sets presence true, including accrual of zero. Discharge requires presence
and preserves it, including full discharge to zero; discharge implies no payment.
Sequential effects publish before/after presence and amount in source order.
Intermediate UInt128 overflow or underflow refuses the entire local action.
Scenario closure is a compiler profile restriction, not authentication of the

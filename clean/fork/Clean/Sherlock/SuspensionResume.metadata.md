# Suspension and exact resume contracts (U014/U015/U016)

This `Init`-only module separates complete technical checkpoints, ordinary
stepping, observation acceptance and technical chunk boundaries. Source hashes,
theorem names and verification disposition are in the matching JSON. The
committed source base inspected on factory-u is `0699194f9`.

The technical restore interface follows `shared/owner_resume.rs`: finance is
refused, the segment must be genesis, profile/SI checks must succeed, and
re-capture must equal the entire supplied checkpoint. `Whole` is an abstract
complete state, not an encoding of Rust's fields. The conditional exactness
statement does not prove that Rust reconstructs every field or implements SI.
The existing /1 and /2 numeric profile IDs are kept distinct.

Chunking iterates a supplied deterministic complete-state transition. The
addition law proves that an identity cut between events changes no observation
of that iteration. It does not prove Rust determinism, outcome priority,
absorbing terminal behavior or concrete source/circuit correspondence. Those
remain premises to supply, as do the complete observation and state types.

The observation contract tracks **uncommitted** profile /2 work inspected in
factory-u's `owner_suspend.rs` and `compile_artifact/suspension2.rs`. It must be
reconciled when that work lands. R0–R5 are named validation predicates; false and
error guards preserve the complete input checkpoint and report attempted work
separately. Acceptance keeps caller frames and charges `1 + guard_nodes` to
consumed work and reference cursor. Fuel conservation requires enough fuel.
The binder installer and remaining resource checks are supplied parameters;
actual fresh allocation and UInt128/epoch headroom are unproved. Retaining
caller frames does not establish the concrete result 14 for `10+inner(3)`.

No financial restore implementation is introduced. Financial prefix, fee-phase,
failure and exhaustion contracts belong to the sibling FinancialItems work.
No cryptographic/checkpoint-byte injectivity, C6 resume relation, native proof,
stock-verifier or ledger correspondence is established here.

No new `sorry` or custom axiom is declared. The sibling `Axioms.lean` lists all
theorem dependency queries. Sherlock-fable independently ran a capped package
build with Lean 4.33.1; `lean-build-1.log` shows this module failing at the zero
case of `runSteps_add`. Hypothesis: Nat addition recurses on the second argument,
so `0 + second` needs `Nat.zero_add`, rather than definitional reduction. The
source now simplifies with that lemma. **The corrected source has not been
recompiled and its axiom queries have not run.** The JSON pins the failed log
by digest. The agent read that log but did not run the build. Its sandbox still
cannot connect `memcap` to the user scope bus; no uncapped build was attempted.

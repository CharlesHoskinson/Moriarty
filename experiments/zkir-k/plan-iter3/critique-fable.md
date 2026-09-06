# Fable advisor verdict on the draft plan

Verdict: B with MockProver is the right bet, but two facts in the crate change M2 and the oracle's version.

What the API allows. No keygen parameters are needed: `MidnightCircuit::new(&ir, Value::known(pis), Value::known(preproc), None)` then `MockProver::run(&circuit, instances)` (midnight-zk-stdlib `src/lib.rs`, `MidnightCircuit::new`). The two pinned crates resolve different midnight-proofs versions (0.7.3 at 92e8bdd3, 0.8.2 at 2ffe2d1); write one oracle per version.

Injected memory is realisable but nearly useless for M2's targets. `Preprocessed` fields are `pub` and `prove_unchecked` exists for this purpose (`ir.rs:993`). Yet `IrValue` is curve and field typed (`JubjubPoint(JubjubSubgroup)`, `JubjubScalar(JubjubFr)`, `Bytes32([u8;32])`, `ir_types.rs:119`): off-curve, out-of-subgroup, non-canonical and out-of-byte-range values are unrepresentable. Only `Native`, `pis`, `binding_input` and `comm_comm` can be perturbed. `mem_insert` (`ir_vm.rs:769`) turns any injected id that mismatches synthesis into a synthErr, aliasing with the real synthErr class; injected memories must omit all non-injected ids.

Change: cut the off-curve and non-canonical injection from M2's exit criterion. Restate `assignGate` for point and scalar types as discharged by the typing of W(P, pi), test only Native-level perturbations, and merge that residual into M1. Drop M5's dependency on M3; M3 already stalled once and must not sit on the critical path.

Risk to mitigate first: MockProver feasibility. `optimal_k` searches to 25; a 2^20-row MockProver with the stdlib's column count may not fit memory. Spike the largest ledger corpus circuit and record k, time and RSS before committing M1's exit criterion.

Missing from the compiler-correctness statement: it has two directions. Completeness (an honest source run yields an accepted preimage with the expected public inputs) is what the oracle tests. Soundness (every w in W(P, pi) corresponds to a source execution) quantifies over the modelled W, which over-approximates the real accepted set whenever a constraint is unmodelled. That is the safe direction, and the plan should say so explicitly, with the unmodelled list as the statement's stated residual.

# R2: the circuit oracle and the comparison table

Focus: experiments/zkir-k/tools/circuit-oracle/ledger-92e8bdd3/src/main.rs (and the 2ffe2d1 copy), experiments/zkir-k/tools/circuit_compare.py, diff_test.py (`--circuit`), divergence_tests.py (circuit expectations), provability.py, experiments/zkir-k/plan-iter3/circuit-comparison-table.md, and the receipts evidence/zkir-k-circuit-oracle-spike-2026-09-06.txt, zkir-k-circuit-differential-*-2026-09-06c.txt, zkir-k-provability-2026-09-06b.txt.

Questions:
1. Does the oracle build the same circuit the crate proves (the stdlib `MidnightCircuit` wrapper with `used_chips` and lookup tables), with the same instance the verifier would see? Check `k` selection, instance construction, and the `--inject`, `--instance`, `--pis`, `--binding-input` paths for anything that would make an outcome meaningless.
2. Are the six outcomes correctly told apart, in particular witness-consistency error (tracing event) versus synthesis error, and constraint failure versus panic? Is `catch_unwind` hiding anything?
3. Is the comparison table sound: for each cell, is the expected oracle outcome really what the crate does for that K verdict summary and perturbation kind? Look for cells that are too permissive (an expectation that would let a real disagreement pass as agreement), and for the "not comparable" declarations: are they justified or convenient?
4. The taint walk (D3) that predicts which injections the circuit recomputes: is it right for every instruction, including the four in-circuit paths that bypass `mem_insert` and the guarded inputs?
5. Keygen and prove modes: parameter provider, `optimal_k` versus `pk.k()`, verify path; is a verified proof here the same thing as a proof the ledger would accept?

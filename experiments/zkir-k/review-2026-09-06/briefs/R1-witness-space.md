# R1: the modelled witness space and the soundness statement

Focus: experiments/zkir-k/semantics/zkir-constraints.k (header comment: projection, soundness claim, residual list; `witnessSpace`, `unconstrainedRegs`, the `unconstrained` outcome, `inputGate`, the `#canonicity` comment), zkir-vm.k (`#witnessSpace`, `#observable`), and chapter experiments/zkir-k/docs/08-constraints-and-verdicts.md.

Questions to settle against midnight-circuits 7.2.4 and midnight-zk-stdlib 2.3.5 sources (cargo registry) and the two crates' `circuit`:
1. Is the projection statement true: is every named register the value of the cell `mem_insert` binds, are auxiliary cells correctly treated as existential, is copy wiring value equality in every chip the instructions use?
2. Is the residual list complete? Search for any constraint the circuit imposes that no modelled relation, no typing clause and no `unconstrained` marker covers (range checks at assignment, foreign-limb ranges, decomposition hints, lookup-table membership, the hash gadgets' padding).
3. Is the canonicity analysis right: `EccChip::assign` with `enforced_canonical: false` for declared inputs and transcript inputs, versus `jubjub_scalar_from_biguint` and `assign_fixed` being canonical by construction?
4. Is the guarded-off input register really a free cell in the circuit, in every case (public and private, every type), and does K mark exactly those?
5. Does `witnessSpace` over-approximate the accepted set in the direction the plan claims (safe for soundness), and is there any case where it under-approximates (a K `violated` or `synthErr` on a witness the circuit accepts), which would be a completeness defect? Use the receipts evidence/zkir-k-circuit-differential-*-2026-09-06c.txt and the table to look for such cells.

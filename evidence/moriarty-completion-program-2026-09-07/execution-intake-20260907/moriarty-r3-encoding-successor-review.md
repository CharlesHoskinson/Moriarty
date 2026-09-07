# Review of the R3 encoding successor

**Status:** source-only correction. This supersedes `/tmp/moriarty-r3-encoding-successor.{json,md}`. No build, synthesis, MockProver, or proof was run.

The prior bare phase proposal has a binding flaw. Host assertions, a host decoder, and `IvcState::decider` do not put the financial/digest table into the synthesized constraint system. If `circuit_transition` constrains only `0 -> 1 -> 2`, the VK binds a counter rather than the loan episode.

Option (a), phase plus a fixed episode digest, has the same problem unless the circuit recomputes the digest from every canonically encoded field or verifies a checked opening. Merely asserting a public digest equals a constant does not constrain the decoded state. Adding in-circuit SHA256 or Poseidon could repair this, but introduces an encoding specification, a cryptographic assumption, and unknown row cost.

The smallest defensible representation for this fixed episode is option (b) without claiming a general cryptographic commitment: keep the complete state private and constrained, and expose one public phase. `AssignedState` contains phase plus all 54 limbs. The circuit uses phase to select the complete fixed before row, equality-constrains every private input limb, selects every complete after-row limb, and returns phase+1. `IvcIO::constrain_as_public_input`, `as_public_input`, and `format_public_input` expose only phase. The episode constants must remain in `circuit_transition`; because they enter synthesized constraints, they affect the VK.

This preserves exactly the original `genesis -> accrued -> settled` steps. All 11 financial fields and all eight digests for domain, program, specification, intent, predecessor, output, effects, and authority/obligation remain constrained. Each u128 stays two range-constrained u64 limbs; each digest stays four. The application PI falls from 54 elements to one, while the private checked application state is 55 elements: phase plus 54 limbs. VK and accumulator PIs remain unchanged.

Required controls reject noncanonical phase, closed/reversed/skipped/repeated edges, all individual financial/digest limb mutations, raw values at or above 2^64, forged genesis, wrong identity/history/authority fields, malformed or mismatched proofs, and an undischarged accumulator. A refactor that retains table values only in host code must fail review.

Exact owned changes are in `experiments/moriarty-native-ivc-r3/harness/moriarty_loan_r3.rs`; `harness/episode.rs` remains unchanged but must be referenced by synthesized constraints; a successor `run-native.py` manifest must bind both hashes and use a fresh output.

The proposed experiment remains one fresh 480-second attempt at k17, 8 GiB, zero swap, two CPUs/jobs, locked Rust 1.90.0 dependencies, midnight-zk `695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7`, and SRS SHA256 `4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74`. Stop at the first failure; do not retry or raise k automatically.

This is still only the fixed application relation. Native recursive verification/final-decider execution and the Preview ledger adapter remain separate. It is unresolved whether lowering application PI count while retaining private full-state constraints saves enough rows, whether PI constraints caused the k17 failure, and whether the IVC implementation soundly permits private AssignedState components across recursive predecessor linkage. These need code review and executed rejection controls before any feasibility claim.

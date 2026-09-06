# Review R3: the constraint checker against the in-circuit halves, and the divergence tests

Files: semantics/zkir-constraints.k, the constraint emission in zkir-vm.k,
tools/divergence_tests.py, corpus/divergence/*.zkir; ground truth ir_vm.rs
`Relation::circuit` and `used_chips`, ir_instructions/*.rs `*_incircuit`,
midnight-circuits 7.2.4 chip semantics, and
/home/charl/Moriarty/repos/input-output-hk/arc-zkir/docs/zkir-v3-divergence-review.md.

Questions to answer:
- For every gate, is the K relation the relation the circuit enforces? In
  particular: assert (non-zero), cond_select (bit booleanity through convert),
  constrain_bits with bits >= FR_BITS, less_than's padded bound, reconstitute_field
  wrap-around, div_mod canonicity, inv (a * inv = 1), into_coordinates of the
  identity, from_coordinates (point_from_coordinates: on-curve, subgroup?),
  public_input / private_input (assign_incircuit: what does it constrain for
  each type?), encode (encode_incircuit extra assertions for JubjubScalar),
  bytes32_from_low_high, the hashes and the aligned byte decoder in circuit,
  the impact piGates and the commitment.
- Chip gating: is usedChips exactly `used_chips`, and is treating a missing
  chip as a synthesis error the right model of what keygen does?
- The claim "instruction-level relations, not gate rows": what soundness-relevant
  behaviour is lost by that abstraction, and is the page honest about it?
- The 14 divergence cases: do they test what they claim? Are findings 1 to 13
  of the review all represented faithfully (9 is documentation-only)? Are the
  two new findings K1 (Jubjub from_coordinates parity) and K2 (transcript
  panic) real, correctly attributed, and correctly described?
- Constraint emission when the witness has failed: absolute pi indices, the
  deadPis bookkeeping, and gates that read registers that do not exist.

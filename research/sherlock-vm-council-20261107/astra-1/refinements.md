# Refinements after the independent position

1. I adopt astra-2's canonicality distinction. Internal microsteps must preserve `StepInv`, but need not perform canonical projection after every step. External admission and publication must satisfy their canonical boundary contracts. I revised `assurance-proposed.md` accordingly. This narrows overly strong wording in my independent position without removing invariant-preservation obligations. [openspec/changes/sherlock-universal-interpreter-design/security-candidate-b.md:64-90]

2. I adopt astra-3's sequencing distinction. The existing A8–A10 endpoint can be requalified alongside C6 completion. A fully supported generic C6 release still needs its own composition and rule coverage. My milestone order should not make full-rule closure a prerequisite for an honestly labeled legacy recursive-anchor experiment. [qa/sherlock/predicates.yaml:A2-A10,U023; scripts/qa/check-a8.sh; crates/mori-backend/src/sherlock_machine/c6_dispatch_consumer.rs]

3. I verified astra-2's producer/consumer mismatch. `prepare_digest_c6_child` creates binding plus five statement lanes. `IvcNode::leaf_instance` creates seven lanes, and the GP harness requires seven. A shared versioned schema belongs before the real C6 end-to-end qualification. [crates/mori-backend/src/sherlock_c6_child.rs:817-834; crates/mori-recursion/src/ivc.rs:191-201; crates/mori-recursion/tests/sherlock_gp.rs:183-188]

These refinements are proposals for synthesis. They do not amend code or assert a new runtime result.

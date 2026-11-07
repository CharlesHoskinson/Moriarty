# Early requirement corrections from astra-1

These concern the in-progress draft before the formal v1 vote.

1. **SRC-007 must not require a nonzero low result byte.** `sherlock-run.compact:19-22` explains why the current fixed-window observation works for that case. The contract's `acceptRun` at lines 40-47 has no nonzero-result-byte constraint. Requiring it would exclude valid UInt128 outputs such as 0 and 256 without a semantic reason. Require canonical Bytes<16> encoding and complete decoded value readback for the declared output domain. Add zero, 256 and maximum-u128 codec/readback tests. If a diagnostic fixture requires a nonzero low byte, label only that fixture's scope. The invariant should preserve exact values, not select convenient output bytes. [tools/sherlock-sdk/sherlock-run.compact:19-22,40-47; UISEC-005]

2. **SVM-006 must permit constrained public digests.** Its current text refuses a witness-stream digest as a public statement component. That contradicts the seven-lane leaf statement and digest-parity design. Refuse host checks as acceptance authority. Permit the specified public digests when the circuit independently constrains them to the actual witness, profile and state. Complete HostSnapshot records remain private. [requirements.md:SVM-006,SRC-001; design.md:D5,D7; UISEC-009,012]

3. **SAS-013 needs justified equivalence, not merely mechanical origin.** A mechanically generated partition can omit a dependency or an invariant premise. Require a checked completeness/congruence argument for the claim that one representative covers a dispatch class. Otherwise retain the coverage-heuristic label. Directed cases for every admitted rule/priority boundary remain required either way. [crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:34-38,750-806; security-candidate-b.md:A4]

4. **Apply accepted corrections to the requirements and scenarios as well as design prose.** Current SRC-005 still combines zero-scaling with mandatory rejection of nontrivial genesis carry. Zero-scaling can establish irrelevance rather than rejection. Current SVM-007 still makes the production stream the holdout reference, and SWF-011 omits tier/set retirement behavior. These points were already resolved in the issue exchange. [astra-2/draft-early-corrections.md; fable-3/workflow-proposed.md:E10]

No final vote is cast by this note.

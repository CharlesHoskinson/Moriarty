# Sherlock recursion evidence

Source revision: `8fd34ed794eda7186c32495f715609ec54a9a297`.

The A102/A80 parent contains real circuit linkage and fixed child keys. The terminal relations contain the final pairing check. Those mechanisms establish scoped claims under trusted keys. The C6 digest adapter does not currently match the IVC input format, and its outcome metadata requires a trusted relation-construction contract.

“Verified fact” below means confirmed in source at this revision. It does not mean a proof, test or attack was executed.

## 1. The A102/A80 parent constrains linkage under fixed keys

**Verified facts.** A102 verification reconstructs the claim, complete instance and verifying key before accepting the proof: [sherlock_child.rs:432](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_child.rs:432). A80 performs the corresponding checks: [sherlock_link.rs:765](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_link.rs:765). The witness carrier rejects guarded inputs, private inputs, skipped public lanes and mismatched publication order: [sherlock_child.rs:244](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_child.rs:244).

The recursive parent aliases A80 program and pre-state cells to A102 program and post-state cells: [parent.rs:583](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/parent.rs:583). It fixes both contexts and rule/charge envelopes at [parent.rs:608](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/parent.rs:608). It prepares both child proofs under fixed keys at [parent.rs:627](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/parent.rs:627). Its height checks bound `h_pre` and `h_pre + 2` below `2^64`: [parent.rs:665](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/parent.rs:665).

**Scope.** The parent proves two linked transitions. It does not enforce initialization at height zero. `anchor_genesis` performs a host subset of initialization checks: [sherlock_link.rs:1225](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_link.rs:1225). `genesis_root_lanes` checks shape, context and envelope before hashing the supplied pre-state: [parent.rs:321](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/parent.rs:321). An application must authenticate that initial root and the installed parent/terminal key.

**Attack tests.** Re-prove with two individually valid children whose program or intermediate state differs. Exercise the circuit directly so host rejection cannot substitute for a constraint. Change each pinned context and envelope independently. Test `h_pre = u64::MAX - 1`. Submit a valid two-step proof with a different initial root to the actual acceptance path.

## 2. C6 outcome metadata is authorized by key construction, not derived from the carrier

**Verified facts.** `c6_child_relation` replaces public carrier lanes with private inputs while retaining the original relation body: [sherlock_c6_child.rs:459](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_c6_child.rs:459). It then hashes caller-supplied `StateOutcome` tag/value constants with the carrier digest: [sherlock_c6_child.rs:551](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_c6_child.rs:551). The added constraints fix those constants, but do not equate them with outcome or result fields in the carrier. The generic API has no carrier outcome-field locator.

The public `rule` and `h` lanes receive inputs and unconditional publications without equality or range constraints: [sherlock_c6_child.rs:598](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_c6_child.rs:598). `prepare_digest_c6_child` supplies their intended values on the host: [sherlock_c6_child.rs:817](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_c6_child.rs:817).

**Interpretation.** Correct key construction can authorize fixed outcome constants for a suitably restricted relation. Arbitrary constants cannot be changed under that same fixed key. The risk arises if an admitted key binds a carrier relation to an incorrect outcome or result. Acceptance of such a key by a deployed verifier is not established here. The raw verifier also proves a published rule label without proving that the label names the keyed relation. `verify_c6_child` supplies stronger host reconstruction checks: [sherlock_c6_child.rs:1012](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_c6_child.rs:1012).

**Attack tests.** Build a relation whose carrier has a known running outcome, then request a digest wrapper with `StateOutcome::halted(wrong_result)`. Determine whether the key-admission path rejects it. With one fixed digest key, alter the rule lane and re-prove through the raw relation. Test rejection at both the host reconstruction API and the eventual recursive consumer. These tests require new proofs, not mutation of an existing proof's public inputs.

## 3. The C6 digest leaf and current IVC node have incompatible public inputs

**Verified facts.** The digest adapter constructs `[binding, prog_d, pre_d, post_d, rule, h]`: [sherlock_c6_child.rs:817](/home/hoskinson/Projects/Moriarty/crates/mori-backend/src/sherlock_c6_child.rs:817). Its test explicitly expects six lanes: [tests/sherlock_c6_child.rs:107](/home/hoskinson/Projects/Moriarty/crates/mori-backend/tests/sherlock_c6_child.rs:107).

The IVC interface requires seven lanes, adding `steps`: [ivc.rs:58](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/ivc.rs:58). The circuit supplies all seven to the leaf verifier: [ivc.rs:354](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/ivc.rs:354). The general-purpose integration harness also requires exactly seven: [tests/sherlock_gp.rs:184](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/tests/sherlock_gp.rs:184).

**Consequence.** The exported C6 digest leaf cannot directly satisfy the current IVC interface. An adapter must prove the step count under the leaf key. Appending a host-selected number to existing proof bytes does not establish that statement. A scoped search through `crates/` found `prove_c6_child` used by backend tests, with no production bridge to the seven-lane IVC interface.

**Attack tests.** Feed a real C6 export to the current IVC harness and retain the exact shape refusal. After implementing the bridge, prove a valid leaf with an incorrect step count. Require rejection inside the keyed relation and terminal verification.

## 4. IVC integer progress depends on constraints outside the node

**Verified facts.** The node assigns `prev_h` and `steps` as field elements, requires only `steps != 0`, and computes `h = prev_h + steps`: [ivc.rs:308](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/ivc.rs:308). It contains no circuit range check on these three values. The Rust witness uses `u64`, which restricts the provided witness constructor but does not constrain a malicious prover's field assignments: [ivc.rs:99](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/ivc.rs:99).

At genesis, the circuit equates the previous digest with the initial digest and scales away the predecessor proof accumulator: [ivc.rs:315](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/ivc.rs:315), [ivc.rs:343](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/ivc.rs:343). The carried accumulator remains a witness and participates in the fold at [ivc.rs:367](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/ivc.rs:367). No canonical trivial-accumulator equality appears in that branch.

**Hypothesis and limit.** A leaf that leaves its height and step count unconstrained can admit field arithmetic that fails the intended positive-integer progress rule. A practical forged history is not demonstrated. The fixed production leaf could impose the missing bounds. An arbitrary genesis accumulator alone is not a demonstrated cancellation attack against the Fiat–Shamir fold.

The existing IVC test uses a toy leaf with an explicitly unconstrained step count: [tests/sherlock_ivc.rs:67](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/tests/sherlock_ivc.rs:67). Its successful chain therefore cannot establish machine-step accounting for a real leaf.

**Attack tests.** Exercise `steps = 0`, `steps = p - 1`, heights outside `u64`, and additions above `u64::MAX`, where `p` is the scalar-field modulus. Use malicious field assignments under an unchanged verification key. Identify the exact production leaf constraint that rejects each case. Test genesis with a malformed carried accumulator through the final pairing, and separately record whether valid nontrivial accumulators are intentionally permitted.

## 5. Terminal verification closes the accumulator and binds the statement

**Verified facts.** The parent decider folds the child proof with the carried accumulator, reassigns the resulting points through the elliptic-curve chip, equates their encodings and fixes both scalars to one: [decider.rs:179](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/decider.rs:179). It invokes the pairing gadget at [decider.rs:221](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/decider.rs:221). The gadget constrains the residue equation to one rather than accepting its host hint: [pairing.rs:243](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/pairing.rs:243).

The Sherlock constructor includes every parent statement field and enables the bound communication commitment: [parent.rs:1061](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/parent.rs:1061). The circuit hashes the selected fields and constrains the commitment opening: [decider.rs:228](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/decider.rs:228).

The run decider also pins the node's public self-key representation to the actual fixed node key: [run_decider.rs:201](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/run_decider.rs:201). It equates carried-accumulator encodings, performs the final pairing and opens the final digest as a halted state with a range-checked result: [run_decider.rs:229](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/run_decider.rs:229), [run_decider.rs:278](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/run_decider.rs:278).

**Scope.** These are source-level guarantees conditional on the underlying gadgets and an authenticated installed key. The standalone parent verifier reads a caller-supplied `--vk` and checks canonical encoding, without pinning its identity: [sherlock_verify_parent.rs:55](/home/hoskinson/Projects/Moriarty/crates/mori-recursion/src/bin/sherlock_verify_parent.rs:55). Its `verifies` result certifies proof and pairing consistency under that supplied key, not authorization of the Sherlock relation.

**Attack tests.** Keep a valid outer proof with an invalid carried accumulator. Alter the self-key representation, scalar-one lanes, accumulator point encoding, final result, communication opening and statement independently. Require rejection under the installed terminal key. Separately demonstrate that the standalone utility's key parameter is authenticated by its caller before treating its output as semantic acceptance.

## 6. Specification and profiling records do not close the missing proof obligations

**Verified facts.** MC03 requires independent retained-byte verification, constrained genesis, expected fields and the final accumulator. Its negative cases include wrong keys, wrong states, forged genesis and invalid predecessors: [MC03 spec:16](/home/hoskinson/Projects/Moriarty/openspec/changes/mc03-native-recursive-proof/specs/mc03-native-recursive-proof/spec.md:16).

The profiling contract preserves transcript and challenge order, requires the same malformed controls, and isolates the patched dependency: [profile spec:99](/home/hoskinson/Projects/Moriarty/openspec/changes/recursion-phase-profile/specs/recursion-phase-profile/spec.md:99), [profile spec:115](/home/hoskinson/Projects/Moriarty/openspec/changes/recursion-phase-profile/specs/recursion-phase-profile/spec.md:115). Its task file leaves the real on/off proof checks, paired overhead runs and full matrix unchecked: [profile tasks:46](/home/hoskinson/Projects/Moriarty/openspec/changes/recursion-phase-profile/tasks.md:46).

**Interpretation.** The inspected records do not establish proof equivalence or performance qualification for the instrumented path. An unchecked task is not proof that no later receipt exists elsewhere. Historical reviewer and campaign rules in these documents are retired by the supplied repository charter and impose no authorization requirement here.

**Attack tests.** Run the same fresh-proof semantic mutants through observation-on, observation-off and stock verification under identical authenticated keys. Include the C6 outcome and step-count cases after the interface is reconciled. Separate public-input mutation checks from malicious witness generation, which targets missing constraints.

## Limitations

This review used targeted source reads and literal searches. No language-server tool was available. The graph query provided orientation only. The reviewed paths had no working-tree changes, and HEAD matched the stated revision.

No build, proof, test or attack was executed. No private witness, secret or other member's position was read. The full pairing arithmetic, upstream verifier implementation, all machine-rule constraints, deployed key provenance and acceptance contract were not audited. Existing ignored tests are source evidence only. No deployed soundness exploit is established by this report.

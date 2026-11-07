# Circuit and recursion assurance audit

Scope: read-only source inspection at `8fd34ed794eda7186c32495f715609ec54a9a297`. No proof, test, synthesis or build was run. Paths below are relative to `/home/hoskinson/Projects/Moriarty`. Source inspection establishes implementation structure and test intent. It does not establish that the tests passed.

## Findings that should affect the council design

### 1. Separate preprocessing, constraints, proof verification and ledger acceptance

C6 dispatch's `check_c6_dispatch_circuit` constructs a `ProofPreimage` and calls `preimage.check(&circuit.ir)`. It does not synthesize constraints or create a proof (`crates/mori-backend/src/sherlock_machine/c6_dispatch_consumer.rs:244–285`). Its factorial/tree-fold tests select expected rules from owner events and check the dispatch circuits through that function (`crates/mori-backend/src/sherlock_machine/c6_dispatch_tests.rs:47–93`, `:95–148`).

The native inventory explicitly labels dispatch receipts as compiled-and-run **nonproving** `ProofPreimage::check` results (`crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:540–550`). `RECEIPTED` names `dispatch`, but the expected `testdata/receipts/dispatch.json` file was absent when inspected. The handoff records that the 16,000-second dispatch run stopped without a receipt (`docs/sherlock/HANDOFF-2026-10-26.md:15–16`). A table entry alone must never close a gate.

The existing data-canonical tests supply a useful distinction: their `SynthesisRejection` separates unsatisfied constraints from setup/synthesis errors, and panics are not converted into rejection (`crates/mori-backend/src/sherlock_machine/c6_data_canonical.rs:745–798`). Preserve that distinction across the assurance system.

Recommendation: each result records `host_semantics`, `sdk_preprocess`, `mock_constraint`, `native_proof`, `independent_proof_verify` or `ledger_effect`. Evidence at one level cannot satisfy another level's acceptance check.

### 2. The mutation harness currently overstates rejection evidence

`qa_mutation::run_once` maps both `MockProver::verify` constraint failure and `MockProver::run` synthesis errors to `Outcome::Rejected` (`crates/mori-backend/src/qa_mutation.rs:451–477`). `run_suite` also increments `rejected` for `Outcome::Panicked` (`:637–650`). Its callers' `assert_sound` require nonempty samples and no accepted single-cell mutations, but do not require zero panics (`crates/mori-backend/tests/soundness_mutation.rs:227–245`).

Therefore, a passing test can include a panic or synthesis failure without demonstrating an unsatisfied constraint. This is a source-confirmed classification defect, not a demonstrated circuit exploit.

Default coverage is bounded and small: 24 cells/96 mutations for data canonicality, three cells/12 mutations for checked reads, and one cell/two mutations for scalar failure. Larger sweeps are ignored by default (`crates/mori-backend/tests/soundness_mutation.rs:802–860`). The driver samples candidates using a seeded shuffle and supports an allowlist (`crates/mori-backend/src/qa_mutation.rs:598–634`).

Recommendation: define distinct `constraint_reject`, `decoder_reject`, `synthesis_failure`, `panic`, `timeout` and `accept` results. Only the first proves a circuit rejection predicate. Fail or mark inconclusive when a test intended to prove constraint rejection instead crashes. Retain sample size, seed, mutation kind and allowlist digest. Do not describe sampled single-cell rejection as complete soundness coverage. Add coordinated mutations and invalid statements with regenerated proofs.

### 3. Dispatch is a selection component with explicit composition obligations

The dispatch implementation leaves IMAGE and HELPER canonicality to a paired step fragment. It checks DATA canonicality only where a value read is enabled. Recursion/native composition is explicitly a caller obligation (`crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:30–38`).

The native inventory still has explicit missing fragments for scalar construction and several Validate, Equal and Match rules (`crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:215`, `:464–509`). The inventory describes source coverage and grants no native acceptance (`:1–15`).

Recommendation: give every fragment a machine-readable assumption/guarantee contract. The universal leaf must discharge each assumption on the **same constrained carrier lanes**. A successful isolated dispatch check is insufficient. Require a composition test that changes an assumed-canonical lane while preserving all other fragments' nominal inputs, then requires constraint rejection.

### 4. Real recursive proof code exists, but the generic test uses a stand-in leaf

The IVC node fixes the leaf key in its relation, links leaf public fields to program/pre/post/rule/height/step count, prepares the previous proof, accumulates the previous/leaf proof checks and exposes the accumulator (`crates/mori-recursion/src/ivc.rs:90–96`, `:300–371`). The run decider pins the node key, binds the carried accumulator to node public fields, resolves fixed bases and constrains the final pairing (`crates/mori-recursion/src/run_decider.rs:200–276`). It also checks the halted result digest and binds the output values into the call template and communications commitment (`:278–326`). These are valuable implementation components to keep.

The `sherlock_ivc` test explicitly uses `ToyLeaf`. Its binding and step-count lanes are unconstrained in that toy relation (`crates/mori-recursion/tests/sherlock_ivc.rs:1–7`, `:63–89`). The proof test is ignored by default (`:111–113`). It does call real `prove` and `verify`, tests changed state/height public lanes, checks the carried accumulator and rejects altered proof bytes (`:195–224`). Those checks demonstrate the intended recursive plumbing if executed successfully. They do not demonstrate C6 instruction correctness.

The GP harness permits real leaf files or a stand-in path. It verifies imported leaf proofs and checks continuity on the host before folding (`crates/mori-recursion/tests/sherlock_gp.rs:129–153`, `:164–206`). The host checks remain distinct from constraints.

Recommendation: the first release qualification needs a retained full C6 execution, real leaves under a pinned C6 relation/key, IVC, decider, independent re-verification and the candidate-specific ledger effect. Keep stand-in performance results, but label their relation and exclude them from C6 semantic closure.

### 5. Public-input mutation is necessary but does not replace malicious-witness testing

The terminal test verifies a real proof after ledger-key deserialization, mutates each public field against that proof, and flips proof bytes (`crates/mori-recursion/tests/sherlock_terminal.rs:310–330`). The parent test similarly changes a public instance field against an existing proof (`crates/mori-recursion/tests/sherlock_parent.rs:137–150`).

These are useful statement/proof binding checks. They do not test whether a prover can generate a fresh proof for a false statement using unconstrained witness assignments.

Recommendation: keep those controls and add adversarial tests that bypass honest witness builders, mutate the post-preprocessing witness, and attempt to prove the false statement. Cover disabled-selector branches, malformed canonical encodings, disconnected state carriers, incorrect outcomes, invalid predecessor proofs and key substitution. Record the rejection layer.

### 6. Genesis and step-count arguments need explicit assurance obligations

The IVC circuit tests `prev_h == 0`, binds `prev_d` to `init_d` at genesis and scales the previous proof's accumulator by the non-genesis bit. The previous carried accumulator is assigned and folded separately (`crates/mori-recursion/src/ivc.rs:315–368`). Honest harnesses initialize it with `trivial_acc()` (`crates/mori-recursion/tests/sherlock_ivc.rs:156`, `crates/mori-recursion/tests/sherlock_gp.rs:244`).

The node requires nonzero `steps` and computes `h = prev_h + steps` as field arithmetic. In the inspected node circuit there is no explicit integer-range or no-wrap constraint on those fields (`crates/mori-recursion/src/ivc.rs:308–314`). The GP harness instead parses step counts into `u64` and checks them on the host (`crates/mori-recursion/tests/sherlock_gp.rs:61–65`, `:189–196`).

These observations are **open proof obligations**, not established exploits. The design must specify whether the leaf contract establishes ranges and whether accumulator soundness permits arbitrary genesis-carried values. State the induction measure, allowed height bounds and non-wrap argument. Then add hostile-genesis and boundary-step tests. Do not use Rust witness types as an argument that arbitrary circuit assignments satisfy integer constraints.

### 7. Independent verification must pin application meaning outside generic proof validity

`sherlock-verify-parent` pins a specific SRS digest, bounds inputs, checks canonical VK serialization, requires 28 canonical fields and independently checks both the PLONK proof and carried pairing (`crates/mori-recursion/src/bin/sherlock_verify_parent.rs:45–102`). It accepts a supplied VK and supplied statement. It does not compare the sixteen statement fields or VK digest to an application-specific expected manifest (`:58–102`). This is appropriate for a generic proof verifier, but application acceptance needs those pins elsewhere.

The binary returns JSON with `verifies: false` without a nonzero exit on ordinary proof rejection (`:103–112`). The existing holdout wrapper correctly parses this field and distinguishes typed reports from decoder refusals (`qa/holdout/lib/rejections.py:80–103`). Preserve that behavior in new callers.

Recommendation: an independently recomputable manifest binds source, compiler/backend, relation, VK, SRS, semantic profile, public fields, network and expected effect. Verification has two explicit results: cryptographic validity and expected-application match. Process exit alone is not proof acceptance.

### 8. HostSnapshot streams are retained witnesses, not proof authority

The C6 range stream calls itself witness storage, keeps one immutable prefix and prior compact HOST, and reports bounded progress and poison states (`crates/mori-core/src/sherlock/helper_machine/c6_range_stream.rs:1–2`, `:18–43`, `:125–143`). The complete-step capture explicitly leaves storage, continuity and proof obligations to consumers (`crates/mori-core/src/sherlock/helper_machine/c6_complete_step_capture.rs:57`, `:83`).

Recommendation: keep the bounded stream and custody design. Treat checksums as byte integrity. The leaf must bind the encoded program/context, pre-state, post-state, work counters and outcome into its statement. An imported stream requires replay/canonicality checks and complete terminal evidence before it can become a proof input.

## Proposed assurance sequence

1. Define the semantic profile, rule inventory and fragment assumption contracts.
2. Repair evidence classification so crashes cannot count as constraint rejection.
3. Build independently expected host/model trace fixtures, including failure precedence and exact work.
4. Check encoded trace equality and fragment composition under mutated carriers.
5. Require constraint-level positive and adversarial cases for each supported rule family.
6. Prove a small real C6 execution and independently verify the retained leaf, node and decider artifacts.
7. Recheck artifact/version compatibility and candidate-specific ledger effects on the deployment target.
8. Expand program and bound coverage without changing the meaning of earlier evidence classes.

These are recommendations. This helper did not run any acceptance check or establish release readiness.

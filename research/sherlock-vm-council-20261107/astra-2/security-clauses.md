# Proposed security requirements for the consensus change

These clauses are proposed text for the chair. They refine existing requirements and do not replace their IDs. Base: `8fd34ed794eda7186c32495f715609ec54a9a297`.

## SND-001: accepted leaf statement

When a leaf proof is accepted for recursive composition, the verifier SHALL bind the authorized relation, program/configuration, pre-state, post-state, rule and exact step interval through one versioned statement schema.

Acceptance: a real producer feeds the actual consumer. Fresh proofs with an incorrect rule, outcome, result, interval or step count fail under the authorized key. A malformed statement receives an explicit refusal before recursive proving.

Mapping: UISEC-001, UISEC-009, ZR01, ZR03, ZR05, ZR06. Concrete gap: `crates/mori-backend/src/sherlock_c6_child.rs:817` emits six lanes while `crates/mori-recursion/src/ivc.rs:191` requires seven.

## SND-002: state invariant

While the machine executes an admitted profile, every accepted step SHALL preserve its complete state invariant. External state admission SHALL establish that invariant through the declared canonical decoder.

Acceptance: establish `decodeCanonical(genesis) ⇒ StepInv` and `StepInv(pre) ∧ constraints ⇒ StepInv(post)` for every authorized rule. Each omitted proof obligation remains explicit. Test malformed reference tags, inactive references, wrong constructor arity, forward references and nonzero union padding through raw witnesses. Coordinated pre/post mutations must be included. An invariant-free fragment test cannot count as full-run soundness evidence.

This clause permits unreachable internal rows when `StepInv` permits them. It does not require canonical projection after every microstep. Mapping: UISEC-002–005, ZR06, ZR11. Existing invariant: `openspec/changes/sherlock-universal-interpreter-design/security-candidate-b.md:A4`. Source challenge: `crates/mori-backend/src/sherlock_machine/c6_admin_publish_step.rs:88`.

## SND-003: integer progress and genesis

When a recursive node extends a history, its accepted relation SHALL enforce positive bounded step counts, bounded integer addition and the declared authenticated genesis rule. Genesis SHALL introduce no unauthenticated deferred obligation.

Acceptance: test zero steps, out-of-width operands, addition overflow, field wrap, a relabeled base case and a malformed genesis accumulator. Identify the circuit constraints or explicit soundness argument that discharges each premise. Rust integer types and witness constructors supply no circuit credit.

Mapping: UISEC-006, UISEC-010, ZR04, ZR07, ZR08, ZR12. Source: `crates/mori-recursion/src/ivc.rs:308`. This source review did not demonstrate a forged history.

## SND-004: terminal closure

If any mandatory predecessor, accumulator or decider obligation remains unchecked, the verifier SHALL refuse final-valid classification. When it accepts a terminal result, it SHALL derive the declared outcome and result from the proved machine state.

Acceptance: a valid outer proof with a well-formed invalid inner proof, wrong instance, dropped accumulator or false terminal claim fails the final verifier. Prove under the authorized key. Also test result dereferencing and terminal state opening.

Mapping: UISEC-010, ZR05, ZR07, ZR08, ZR16. Mechanism to preserve: `crates/mori-recursion/src/run_decider.rs:206`. Outcome boundary to close: `crates/mori-backend/src/sherlock_c6_child.rs:560`.

## SND-005: key and deployment identity

The acceptance path SHALL authenticate the complete relation/proof/key/SRS/semantic-profile tuple and its deployed operation identity. Where prepared-key caching is enabled, cache reuse SHALL preserve those bindings.

Acceptance: substitute a trivial relation, wrong key, stale layout, different SRS or mismatched template while retaining otherwise valid objects. The relevant acceptance stage must reject. Authorized upgrades preserve state, consumed resources and outstanding duties under the migration policy.

Mapping: ZR01, ZR05, ZR15, MNR06. Existing operation-key check: `crates/mori-ledger/src/sherlock_transaction.rs:201`.

## SND-006: target acceptance and canonical execution

When a target result qualifies a candidate, its evidence SHALL bind that candidate to the chain instance, runtime, installed key, exact finalized call and observed state/effects. Accepted transcript representations SHALL determine one consensus-visible execution behavior.

Acceptance: reject wrong-chain, reset-chain, stale-state and wrong-candidate receipts. Reject byte-distinct values or transcripts that alias in proof inputs but change VM behavior. Include repeated consumption and replay tests where the declared operation consumes a resource. Require real verification, balancing, signatures and limits.

Mapping: UISEC-011, ZR01, ZR03, ZR10, ZR11, ZR16. Current helper boundary: `crates/mori-ledger/src/sherlock_transaction.rs:850`. Strict mode: the same file at line 483. Upstream precedent: <https://github.com/midnightntwrk/midnight-ledger/security/advisories/GHSA-wr7g-rr4v-jmj8>.

## SND-007: honest negative evidence

If a negative test fails before reaching the claimed verifier stage, the assurance reporter SHALL label that stage untested. Panics, synthesis failures, timeouts and preprocessing refusals SHALL NOT receive constraint-rejection credit.

Acceptance: inject each outcome and check distinct labels. Require a valid baseline and a mutation that invalidates the claimed semantics. Valid witness freedom is not an under-constraint by itself.

Mapping: UISEC-012, ZR06, ZR16. Verified source: `crates/mori-backend/src/qa_mutation.rs:458`, `crates/mori-backend/src/qa_mutation.rs:640` and `crates/mori-backend/tests/soundness_mutation.rs:227`. This agrees with astra-1's published position.

## Scope of adoption

These are implementation and qualification requirements. Their adoption does not assert that the source currently satisfies them. A small pure program can qualify its precise slice while unsupported financial or machine semantics remain open. The full security requirement denominator remains unchanged. Source: `openspec/changes/sherlock-universal-interpreter-design/traceability.md:190`.

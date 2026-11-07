# Proposed scope and recovery clauses

These clauses supplement the independent position. The chair can assign final requirement IDs. They are proposed requirements, not implementation claims.

## Candidate and target selection

**Unwanted behavior.** If an evidence record differs from the selected candidate's relevant source, artifact or target dependencies, the qualification checker SHALL refuse current acceptance credit.

**Event-driven.** When an operator selects a candidate for target qualification, every proof, key, statement and deployment lookup SHALL use an explicit candidate manifest.

Acceptance: add changed-input cases for relation, key, statement, network genesis, deployment address and runtime. Test two valid candidate directories with reversed modification times. The selected candidate must remain unchanged. Extend existing factory records and artifact inputs. Do not create a second results database.

Evidence for the gap: `models/doctorwatson/factory/status.py:34-47,77-112`, `scripts/qa/check-a10.sh:7-13`. A10 currently chooses the newest public statement by modification time.

## Exact endpoint acceptance

**Event-driven.** When a target call qualifies a candidate, the qualification checker SHALL bind the finalized call, operation, installed key, statement and complete effects to that candidate.

**Unwanted behavior.** If the observed call belongs to another candidate or chain instance, the qualification checker SHALL reject the receipt.

Acceptance: test an unrelated call at the same address, a correct call at a previous address, a pre-reset receipt, and altered successor effects. Preserve A8 as a narrow inclusion observation unless its checker is deliberately strengthened. Require the complete positive, negative and readback suite for promotion.

Evidence for the gap: `scripts/qa/check-a8.sh:8-35`, `qa/sherlock/predicates.yaml:70-95`, [fresh A8 and target observations](observations.md).

## Failure classification

**Ubiquitous.** The assurance reporter SHALL distinguish semantic rejection, constraint rejection, preparation failure, synthesis failure, panic, resource termination and unknown target outcome.

**Unwanted behavior.** If a negative test fails before its intended validator runs, the reporter SHALL refuse rejection credit for that validator.

Acceptance: inject a synthesis failure, panic and timeout into a valid baseline campaign. Require separate infrastructure outcomes. Then demonstrate a well-formed invalid witness reaching the intended constraint verifier. Keep the successful controls and complete failure details.

Evidence for the gap: `crates/mori-backend/src/qa_mutation.rs:458-486,640-650`, `crates/mori-backend/tests/soundness_mutation.rs:227-245`. Dispatch also converts every checker error to a Boolean refusal at `crates/mori-backend/src/sherlock_machine/c6_dispatch_tests.rs:85-87`.

## Dispatch recovery and U023

**State-driven.** While a dispatch check lacks a successful complete run, its inventory SHALL report the receipt as absent.

**Event-driven.** When a diagnostic uses fewer states, rules or mutations than an acceptance check, its evidence SHALL identify the reduced coverage.

Acceptance: a timeout or reduced run must not satisfy the complete receipt verifier. Complete U023 requires all of its current gates, including exercised-rule consumers, fragment receipts and recursion integration. A successful dispatch-only receipt remains a partial result.

Evidence: `qa/holdout/lib/uireq.py:1025-1042,1195-1237`, `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs:805-813`. The dispatch test is nonproving preprocessing (`crates/mori-backend/src/sherlock_machine/c6_dispatch_consumer.rs:1-8`).

## Sequencing amendment after peer evidence

The six-versus-seven public-lane mismatch is verified in `crates/mori-backend/src/sherlock_c6_child.rs:817-834` and `crates/mori-recursion/src/ivc.rs:191-201`. Make the versioned leaf contract a concrete task before new C6 recursive qualification. Appending a lane is insufficient: the producing constraints must bind its meaning.

Specify whether `steps` counts physical transitions, semantic work or another declared measure. The IVC requires nonzero `steps` and adds it to height (`crates/mori-recursion/src/ivc.rs:305-320`), while the interpreter profiles retain free terminal publication (`openspec/changes/sherlock-universal-interpreter-design/design.md:25-35`). Keep height and charged work distinct unless the mapping proves equality. Require range and no-wrap obligations for the chosen counter domain.

The existing Candidate-A endpoint can be requalified independently of the new C6 leaf integration. Neither result may borrow the other's completion claim. This preserves progress on A8 while making the actual C6 interface gap explicit.

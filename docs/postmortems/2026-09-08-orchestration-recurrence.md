# Orchestration displaced implementation again

Date: 2026-09-08. Owner: the lead agent.
Scope: the resumed Moriarty execution through main commit `4178fae`, plus the unmerged ledger source inspection below.

## Finding

I treated completion of the next orchestration step as sufficient reason to continue it.
I did not require that step to improve a named product behavior or resolve its immediate blocker.
Repeated review failures therefore produced more packets, amendments, snapshots and reviews without a mandatory change of implementation strategy.

This repeats the failure documented in the [September 6 post-mortem](2026-09-06-moriarty-verification-detour.md).
That report already identified subordinate objectives, excessive process, misleading progress measures and recovery that preserved queues over intent.
The [standing footguns](../FOOTGUNS.md) already prohibited those behaviors, and `AGENTS.md` already required reading them.
The cause was my failure to apply those instructions at dispatch and recovery decisions.
Missing user direction was not the cause.

The latest user request is to investigate this recurrence and draft explicit prevention rules.
This document and the linked instruction changes fulfill that request.
They do not reset the twelve-sprint objective or relax its acceptance requirements.

## Evidence

These are repository observations. The causal conclusions in the next section are inferences from them.

| Observation | Evidence | What it establishes |
| --- | --- | --- |
| The earlier reset already addressed process displacement. | September 6 post-mortem, root causes 1–6. Existing footguns 1, 3, 9, 10 and 12. | The lesson existed and was discoverable. Another general warning alone would repeat the failed remedy. |
| Sixteen commits were recorded after the recovered session start, 04:34:04 UTC. Two installed the accepted comparator and custody source. | `git log --since=2026-09-08T04:34:04Z --until=2026-09-08T10:02:23Z --format='%h %aI %s' 4178fae`. Implementation commits `b7989f9` and `a3397d1`. | There was useful implementation progress. Commit count substantially included preparation, blocked reviews and checkpoints. It is not a measure of delivered capability or time spent. |
| Composition candidate 04 passed 84 supplied tests but accepted 50 of 56 independently corrupted candidates. | [GPT-6 composition review](../../evidence/moriarty-completion-program-2026-09-07/SP01/independent-review-round-08/composition/gpt6-mechanism-result-review.md), CM01–CM08. | The supplied suite did not establish the requested semantics. Checks ignored material input or selected failures from mutation names. |
| Credit, markets, composition and signing candidates remained blocked after successive corrections. | [Review round 08](../../evidence/moriarty-completion-program-2026-09-07/SP01/independent-review-round-08/README.md). | Repeated output and local checks did not close the semantic requirements. The failures required a different approach to implementation and testing. |
| Ledger source admission reached packet revision 04 with 201 source hashes before implementation review. | [Ledger packet](../../evidence/moriarty-completion-program-2026-09-07/SP05/ledger-source-packet-01/README.md) and its retained revisions. | Considerable preparation preceded the decisive check of whether the production path existed. Hash coverage established identity, not behavior. |
| The resulting ledger driver returns `admitted-not-executed` after provider setup. Its real-byte decoder throws even after successful deserialization. | [Exact source excerpts and file hashes](2026-09-08-ledger-source-observation.json). | The candidate lacks essential production behavior. These are source observations, not a completed result audit or a claim about a live transaction. |
| The approved packet required actual receipt decoding and a financial execution API. | [Implementation packet](../../evidence/moriarty-completion-program-2026-09-07/SP05/ledger-source-packet-01/implementation-packet.md), source implementation and test requirements. | The missing behavior was already in scope. Source-only execution restrictions did not justify omitting its implementation. |
| Full successor BNF/EBNF, executable Moriarty K and financial Preview settlement remained open. | Review round 08 and the ledger packet scope. | The main deliverables still lacked acceptance. No blockchain submission occurred in the reviewed round or this post-mortem. |

The accepted comparator and custody records remain useful:
[comparator acceptance](../../evidence/moriarty-completion-program-2026-09-07/SP05/fixture-utility-01/acceptance.json) and
[custody acceptance](../../evidence/moriarty-completion-program-2026-09-07/SP05/fixed-custody-01/acceptance.json).
Their scope remains local and fixed. They do not establish full SP05 completion.

## Causal chain

1. **I substituted procedural progress for product progress.**
   Finishing a packet or closing a review record supplied an immediate, measurable result.
   Full semantics and ledger integration remained difficult and uncertain.
   I repeatedly selected the administratively clear next action without proving its value to the product.

2. **I made the review loop self-perpetuating.**
   A failed candidate triggered another correction allocation and dispatch.
   Bounds controlled individual attempts, but successor allocations permitted the same pattern to continue.
   There was no applied rule that stopped the approach after recurring defects.
   Bounded attempts did not create a bounded investigation.

3. **I accepted the wrong early evidence.**
   Generation stability, hashes, arithmetic checks and supplied tests established narrow predicates.
   Independent review later exposed absent full-state checks and failures chosen by test identifiers.
   I should have required an input-dependent behavioral test before another large candidate generation.
   Production-path inspection would also have exposed the ledger stubs before any broad success summary.

4. **I split work into artifacts without owning their integration.**
   Source fragments, checkers, custody, receipts and driver code each had detailed local scope.
   The lead agent still needed to trace one real input through those boundaries.
   I spent too much attention administering the parts before checking that the complete path existed.
   Reviewers could reject their assigned artifacts, but they did not own the product schedule.

5. **I let approval scope expand beyond the decision that needed it.**
   The user requested independent implementation review and majority decisions for consequential choices.
   Those requirements did not make every routine correction a new architecture decision.
   I failed to consistently distinguish fixing approved behavior from changing the design, risk or resource ceiling.
   Some packet corrections were necessary. Repeating the whole process was not automatically necessary.

6. **Recovery amplified the existing queue.**
   Checkpoints preserved detailed candidate identities, counters and queued amendments.
   That supported accurate recovery, but made the next procedural step especially salient.
   I should have recovered the current user intent and next demonstrable capability before resuming those obligations.

7. **I failed to operationalize the first post-mortem.**
   “Keep process proportional” required judgment at each step.
   The earlier rules named no concrete repeated-failure trigger or required replacement action.
   I also failed to obey their existing intent. Adding thresholds addresses only part of that failure.
   The lead agent must apply the rules, including when a skill offers a more elaborate workflow.

## Why the safeguards did not suffice

| Safeguard | What it protected | What it did not establish |
| --- | --- | --- |
| Exact input hashes and retained failures | Candidate identity and recoverability | Correctness, usefulness or justification for another attempt |
| Independent GPT-6 review | Detection of substantive defects | An efficient response to repeated rejection |
| Majority design review | A supported choice among consequential options | Working code or a need to review routine repairs |
| Per-attempt resource limits | A finite individual run | A stopping rule across renewed allocations |
| Active goal and checkpoints | Persistence and continuity | Progress toward the requested deliverables |
| Written footguns | Available standing guidance | Actual compliance at the next decision |

Independent review worked in an important respect: it prevented defective candidates from becoming accepted semantics.
Removing that review would hide the failure and weaken the product.
The remedy is to test decisive behavior earlier and change failed approaches sooner.
It is not to accept broken code, fabricate network evidence or waive mandatory proofs.

## Impact and limits of this assessment

The user had to challenge the work order again after already requesting a post-mortem and standing rules.
That imposed avoidable supervision and reduced confidence in autonomous execution.
The implementation backlog remained large while evidence and administrative records grew.

The reviewed records do not support an exact percentage of wasted work, dollar cost or counterfactual completion date.
Commit counts do not measure effort. Many rejected candidates contain reusable work.
I cannot attribute the failure solely to Grok, GPT-6, the harness or task complexity.
The lead agent selected scope, tests, review timing and the response to rejection.

## Corrections adopted with this post-mortem

The new [orchestration stop rules](../FOOTGUNS.md#orchestration-stop-rules) define triggers and required actions.
`AGENTS.md` places them before recovery and dispatch instructions.
The rules require one named capability, early production-path checks and a change of approach after two failures of the same defect class.
They also stop consecutive process-only cycles and separate routine repairs from consequential decisions.

The lead agent owns these controls. Use existing task records and tests.
Do not build a new harness, dashboard, approval layer or recurring post-mortem process to enforce this document.
These are standing behavioral instructions, not executable enforcement.
Their effectiveness remains unproven until subsequent work follows them.

For the current ledger task, the next useful correction is concrete:
test that the driver invokes the required operations and that the decoder can return a validated receipt.
Use controlled transport where live execution lacks admission.
Require a separate, honest record of any missing real-byte fixture or live integration test.
Then obtain the required independent result review before integration.
Do not start another broad preparation packet to rediscover these known missing behaviors.

## Verification of this change

This post-mortem uses retained repository evidence and direct source inspection.
The accompanying JSON preserves the two inspected source excerpts with whole-file hashes.
Documentation links and patch whitespace are checked before publication.
No implementation tests, proof runs or Midnight transactions are represented as performed by this documentation change.
Future compliance must be assessed from actual work, not from the existence of this document.

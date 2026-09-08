# Moriarty footguns

Standing instructions adopted by the [2026-09-06 user reset](../raw/assignments/moriarty-target-first-reset-2026-09-06.md).
Read with [the postmortem](postmortems/2026-09-06-moriarty-verification-detour.md).
These rules apply to research, planning, implementation and recovery.

The [September 8 recurrence post-mortem](postmortems/2026-09-08-orchestration-recurrence.md)
records a second failure to apply these instructions.
Apply the following stop rules before following a recovered work queue or dispatching more work.

## Orchestration stop rules

These rules implement the user's September 8 request to prevent repeated orchestration displacement.
The lead agent owns their application. Record decisions in the existing task record.
Do not create a new approval system, dashboard or harness for these rules.

1. **Name the capability before the next action.** State what the developer will be able to do.
   Name the command or observable result that demonstrates it.
   A packet, hash manifest, allocation, review dispatch or checkpoint is supporting work.
   None independently counts as a delivered capability.
   When the user explicitly requests documentation, the requested document is the deliverable.

2. **Stop repeated failure of the same approach.** Two failed implementation/result-review cycles with the same defect class trigger this rule.
   Do not dispatch a third broad correction of that approach.
   First reproduce one defect through the public API or smallest executable mechanism.
   Change the implementation strategy or task size, and demonstrate why the change addresses the defect.
   Give Grok that reproducer and a bounded repair under existing authority.
   Retain independent GPT-6 result review and the original acceptance requirements.
   Rewording a packet, changing an identifier or increasing a timeout does not reset this trigger.

3. **Interrupt process-only work.** Two consecutive orchestration cycles without implementation, a decisive experiment or a resolved concrete blocker trigger this rule.
   Thirty minutes spent only on orchestration administration also triggers it.
   An orchestration cycle means preparing, dispatching, receiving and dispositioning one task or review.
   A resolved blocker must identify the previously failing operation now enabled or a decisive technical finding.
   A new packet approval or resource allocation alone does not reset this trigger.
   A genuinely running required build, proof or test is not administration time.
   Stop adding packets and bookkeeping work.
   Inspect or test the production path, repair the current defect, or continue an independent eligible implementation task.
   If none is possible, state the concrete blocker and evidence without inventing another preparatory dependency.

4. **Inspect the production path before reporting candidate success.** Trace one input from the public entry point to its observable result.
   Reject an unconditional throw, constant admission response or unused adapter where execution is required.
   Source-only admission restricts invocation, not implementation of the approved production behavior.
   Test operation order and input-dependent output through that path using controlled transport when necessary.
   Report missing live tests or real transaction fixtures separately.
   Never relabel synthetic transport results as network evidence.

5. **Make tests challenge behavior.** Derive at least one relevant adversarial input independently from the acceptance requirement.
   Check complete material state when the claim requires complete state.
   Reject decisions derived from mutation names, expected errors or copied expected outputs.
   A regression must exercise the defect through the callable mechanism.
   Test counts, stable generation and hashes cannot replace that check.

6. **Separate repairs from consequential decisions.** Repair approved behavior without another design vote when scope and authorized limits remain unchanged.
   Record only the changed requirement, evidence and resource delta in the existing task record.
   Use the user-required majority process for consequential design or resource changes.
   Justify renewed resources with a changed hypothesis or decisive remaining test, not sunk cost or unfinished status.
   Do not exceed an existing limit or waive a failed gate to avoid review.
   A genuine limit stops that run, but does not automatically stop independent eligible work.

7. **Own integration and limit unfinished work.** Keep one primary capability in focus.
   Delegate independent work only when it has a clear boundary and available review capacity.
   Do not create another dependent candidate while its recurring prerequisite defect lacks a reproducer and changed approach.
   Preserve all sprint requirements and dependency gates.
   Work on permitted provisional components without claiming a semantic freeze or broader acceptance.

8. **Recover intent before obligations.** Read the latest user instruction and these rules before resuming checkpoint tasks.
   Keep the current capability, last demonstrated result and next executable action visible in the existing checkpoint.
   Preserve detailed evidence by reference instead of making its queue the objective.
   Verify process liveness before claiming that work continues.
   An active completion loop authorizes persistence, not repetition of a failed approach.

9. **Report the outcome and its limits.** Lead with new usable behavior, the remaining gap and the next demonstration.
   Distinguish author-reported results from independently verified results.
   Report local simulation, proven compilation and finalized financial settlement as separate stages.
   Post every actual blockchain transaction ID and observed status in the conversation.
   If no transaction was submitted, say so when reporting the network milestone.

10. **Respond to a recurrence immediately.** When the user identifies process displacement, stop starting administrative work.
    Fulfill the requested diagnosis or correction before resuming the previous queue.
    Do not replace the product objective with another broad planning campaign.
    Do not claim these rules solved the problem until subsequent execution demonstrates compliance.

These are behavioral controls. They do not automatically enforce themselves.
At each trigger, the lead agent must apply the required action without requesting routine permission again.
Keep required tests, independent audits, bounded resources, mandatory PCD and financial acceptance intact.

## Development plugin integration

The repository development plugin (`plugins/moriarty-dev`) implements automated enforcement of these stop rules:
```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . next
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . run --action <ACTION_ID>
```
If host hook integration is unverified or degraded, use direct CLI execution as the verified fallback procedure.

## Existing product and evidence rules

1. **Recover the purpose before the work queue.** Read the latest user directive
   and current roadmap notice before checkpoint obligations. A stale loop is
   not authority to continue. A4/A5 are unfinished historical experiments; the
   current task is the ACTUS/DeFi/PCD design cycle. Never report them complete to
   clear an obsolete goal.

2. **Start semantics with implementation targets.** Every proposed semantic
   operation must trace to an ACTUS fixture, a named DeFi behavior or an explicit
   developer requirement. Give the smallest example that requires it. Do not
   defer both target families until after selecting and verifying a Core.
   Product types and taxonomy categories do not automatically become Core
   constructors. Keep the complete target coverage matrix visible.

3. **Show what a developer can do.** Before a large formalization campaign,
   show representative authoring, simulation, failure diagnosis, signing and
   proof-consumption flows. Name what works, what is mocked and what is missing.
   A test-count checklist cannot replace this demonstration.

4. **Name the correctness claim.** Separate language metatheorems,
   contract-specific properties, a transaction's valid execution, PCD history
   compliance, and compiler/ledger correspondence. State assumptions, input
   domains and bounds. A passing model check, cryptographic proof or compiler
   invocation establishes only its actual predicate.

5. **Turing-incomplete does not mean automatically correct.** Require explicit
   sizes for values, intermediate arithmetic, collections, schedules, horizons,
   nesting, transaction work and predecessor fan-in. Specify rounding, overflow,
   rejection and termination. Finite state may still be too large to enumerate.
   Prefix checking needs a completeness argument before becoming a lifetime
   claim. Continuations must not silently reset a promised lifecycle bound.
   Admit the exact registered bounds bytes and hash. Matching schema or profile
   labels do not authorize changed limits or hash domains. Reject malformed
   bounds with the defined diagnostic before authentication or consumption.

6. **PCD is part of transaction acceptance.** The design must bind a transaction
   to its semantic/program versions, predecessors, authorization, observations,
   resulting state and effects. Specify constrained genesis, multi-input
   composition and the final verification decision. Hash-linked receipts and
   simulated certificates are not PCD. Missing or invalid required proofs must
   fail closed in the real acceptance path. A signed mandatory-claim root cannot
   be stripped or downgraded by a relay. An optional acceleration fallback must
   preserve the same required claim/history predicate.

7. **History compliance is not global uniqueness or oracle truth.** Define
   ledger consumption/nullifiers, ordering, finality and external-input trust
   separately. Test duplicate consumption and competing valid branches, as well
   as altered proof bytes and public inputs. State what a signature attests.

8. **Check backend compatibility early and narrowly.** Recursive verification,
   folding, accumulation, compression and zero knowledge are different
   properties. Compact's ban on recursive source functions does not rule out
   Midnight-native recursive proofs. Inspect the actual native implementation
   and its application/ledger interface separately. Name the construction and
   final verifier. Do not infer Nova–Compact compatibility from either project's existence. After specifying the
   required relation, use one small positive proof and meaningful rejection
   controls to test the actual pinned deployment interface.

9. **Bound investigations by a decision.** Before an expensive run, record the
   question, smallest decisive input, expected distinguishing outcomes, command,
   resource ceiling and stop condition in the task's existing record. Track
   cumulative effort against that ceiling. After a failure, change a justified
   hypothesis before repeating work. Do not automatically increase heaps,
   widen matrices, rerun consumed stages or arm an open-ended completion loop.
   Use an existing authorized resource budget; if none exists, propose a bounded
   experiment as part of planning rather than inventing unlimited authority.

10. **Keep process proportional.** Preserve authentic inputs, outputs and failed
    runs. Reuse those receipts. Add review or infrastructure only when it closes
    a named correctness or reproducibility gap; do not turn tooling repair,
    provenance, checkpointing or Foreman development into the product. A
    source-only review does not approve product suitability or a native result.

11. **Reuse evidence with its original scope.** E00 generated a narrow Compact
    atomic-swap specialization; its proof compilation was mock. Candidate A
    contains useful interpreter, authorization and lifecycle experiments, with
    incomplete A4/A5 acceptance. Neither is a finished general DSL. Do not erase
    useful code, promote it automatically, or force the new semantics to fit it.

12. **Report progress in user terms.** Lead checkpoints with the current goal,
    usable capability, material gaps and next reviewable deliverable. Include
    ACTUS, DeFi, proofs and developer interface status. Cite test counts only as
    supporting evidence. Report runner counters as counters, not a billing
    invoice or a measured total of wasted compute.

The current [design-cycle plan](superpowers/plans/2026-09-06-actus-defi-pcd-replanning.md)
applies these rules. Changing the plan requires preserving the reason and its
effect on target coverage, not adding another layer of approval machinery.

13. **Separate authority from outcomes and concrete plans.** Exact-plan signing
    is a restricted profile, not solver-independent intent. A refund cannot
    conceal gross over-spending, and a gross receipt before fees is not the net
    promised delivery. Check permitted intermediate recipients and calls.
    Pending progress carries residual authority and obligations; it does not
    establish a terminal goal. A valid signature, keyword certificate or
    taxonomy classification proves none of these semantics. See the
    [intents amendment](research/2026-09-06-intents-report-integration.md).

14. **Apply skills only within their stated scope.** The bridge formal workflow
    applies to the bridge repository. Missing bridge files in Moriarty are not
    an installation failure to repair. Check applicability before obeying a
    foreign repository's stop rule. Worker 04 stopped before edits after this
    exact mistake; preserve its receipt in MC01/profile-02.

15. **Use delegated authority and preserve useful output.** The latest user
    instruction delegates execution decisions and forbids repeated permission
    loops. Record a justified bounded successor when needed, then continue.
    Keep independent correctness reviews and actual completion requirements.
    Do not reuse an inadequate short timeout for a large schema correction.
    Replace documents atomically so interruption cannot delete a required file.
    A worker timeout does not erase completed artifacts, but its report is not
    evidence until the host checks the actual output.

16. **Bind the actual review input before dispatch.** A filename or review tag is
    not candidate identity. Verify the manifest digest, candidate commit, worktree
    HEAD and every source hash before launching the reviewer. Verify the returned
    identity and unchanged bytes before admitting its verdict. The implementation07
    GPT-6 launcher accidentally supplied the implementation06 manifest; that review
    cannot approve implementation07. Preserve the report and recheck useful findings.


## Three-report planning guard

The [report reconciliation](../openspec/REPORT-RECONCILIATION-2026-09-07.md) applies to successor profiles and campaigns. Keep Moriarty Midnight-centric: other-chain examples supply financial requirements, not new backend deliverables. Review nominal liabilities, pending workflows and private successor artifacts before committing an expanded proof relation. A fixed linear proof, an atlas row count or an optional-proof report roadmap cannot close mandatory general-history or financial-coverage obligations.

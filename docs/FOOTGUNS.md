# Moriarty footguns

Standing instructions adopted by the [2026-09-06 user reset](../raw/assignments/moriarty-target-first-reset-2026-09-06.md).
Read with [the postmortem](postmortems/2026-09-06-moriarty-verification-detour.md).
These rules apply to research, planning, implementation and recovery.

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

# Moriarty footguns

Superseded 2026-10-24: the orchestration stop rules, startup-plugin requirement, review
and vote rules, and recovery procedures that this file used to carry are retired. The
process is the factory loop in [AGENTS.md](../AGENTS.md) (`scripts/sherlock-next`,
`scripts/doctorwatson stop-rule`). What remains is technical: known ways the language,
proof and backend work goes wrong.

## Technical footguns

1. **Inspect the production path before reporting candidate success.** Trace one input from the public entry point to its observable result.
   Reject an unconditional throw, constant admission response or unused adapter where execution is required.
   Source-only admission restricts invocation, not implementation of the approved production behavior.
   Test operation order and input-dependent output through that path using controlled transport when necessary.
   Report missing live tests or real transaction fixtures separately.
   Never relabel synthetic transport results as network evidence.

2. **Make tests challenge behavior.** Derive at least one relevant adversarial input independently from the acceptance requirement.
   Check complete material state when the claim requires complete state.
   Reject decisions derived from mutation names, expected errors or copied expected outputs.
   A regression must exercise the defect through the callable mechanism.
   Test counts, stable generation and hashes cannot replace that check.

3. **Start semantics with implementation targets.** Every proposed semantic
   operation must trace to an ACTUS fixture, a named DeFi behavior or an explicit
   developer requirement. Give the smallest example that requires it. Do not
   defer both target families until after selecting and verifying a Core.
   Product types and taxonomy categories do not automatically become Core
   constructors. Keep the complete target coverage matrix visible.

4. **Show what a developer can do.** Before a large formalization campaign,
   show representative authoring, simulation, failure diagnosis, signing and
   proof-consumption flows. Name what works, what is mocked and what is missing.
   A test-count checklist cannot replace this demonstration.

5. **Name the correctness claim.** Separate language metatheorems,
   contract-specific properties, a transaction's valid execution, PCD history
   compliance, and compiler/ledger correspondence. State assumptions, input
   domains and bounds. A passing model check, cryptographic proof or compiler
   invocation establishes only its actual predicate.

6. **Turing-incomplete does not mean automatically correct.** Require explicit
   sizes for values, intermediate arithmetic, collections, schedules, horizons,
   nesting, transaction work and predecessor fan-in. Specify rounding, overflow,
   rejection and termination. Finite state may still be too large to enumerate.
   Prefix checking needs a completeness argument before becoming a lifetime
   claim. Continuations must not silently reset a promised lifecycle bound.
   Admit the exact registered bounds bytes and hash. Matching schema or profile
   labels do not authorize changed limits or hash domains. Reject malformed
   bounds with the defined diagnostic before authentication or consumption.

7. **PCD is part of transaction acceptance.** The design must bind a transaction
   to its semantic/program versions, predecessors, authorization, observations,
   resulting state and effects. Specify constrained genesis, multi-input
   composition and the final verification decision. Hash-linked receipts and
   simulated certificates are not PCD. Missing or invalid required proofs must
   fail closed in the real acceptance path. A signed mandatory-claim root cannot
   be stripped or downgraded by a relay. An optional acceleration fallback must
   preserve the same required claim/history predicate.

8. **History compliance is not global uniqueness or oracle truth.** Define
   ledger consumption/nullifiers, ordering, finality and external-input trust
   separately. Test duplicate consumption and competing valid branches, as well
   as altered proof bytes and public inputs. State what a signature attests.

9. **Check backend compatibility early and narrowly.** Recursive verification,
   folding, accumulation, compression and zero knowledge are different
   properties. Compact's ban on recursive source functions does not rule out
   Midnight-native recursive proofs. Inspect the actual native implementation
   and its application/ledger interface separately. Name the construction and
   final verifier. Do not infer Nova–Compact compatibility from either project's existence. After specifying the
   required relation, use one small positive proof and meaningful rejection
   controls to test the actual pinned deployment interface.

10. **Reuse evidence with its original scope.** E00 generated a narrow Compact
    atomic-swap specialization; its proof compilation was mock. Candidate A
    contains useful interpreter, authorization and lifecycle experiments, with
    incomplete A4/A5 acceptance. Neither is a finished general DSL. Do not erase
    useful code, promote it automatically, or force the new semantics to fit it.

11. **Separate authority from outcomes and concrete plans.** Exact-plan signing
    is a restricted profile, not solver-independent intent. A refund cannot
    conceal gross over-spending, and a gross receipt before fees is not the net
    promised delivery. Check permitted intermediate recipients and calls.
    Pending progress carries residual authority and obligations; it does not
    establish a terminal goal. A valid signature, keyword certificate or
    taxonomy classification proves none of these semantics. See the
    [intents amendment](research/2026-09-06-intents-report-integration.md).

## Headless implementation and review permissions

Process exit zero does not establish a completed model run. Require AGY JSON
`status: SUCCESS` or Grok `stopReason: end_turn`, then inspect the substantive
result and verify the candidate. A cancellation, timeout or progress narration
cannot approve source.

For the authorized plugin work, AGY `accept-edits` did not approve shell test
commands. The successful invocation also used its per-process
`--dangerously-skip-permissions` option. Grok `plan` mode still cancelled a local
fixture command with `--allow Bash` and again with `--always-approve`.
A harmless write/read check succeeded with `--no-plan --always-approve`.
See the [retained invocation and review evidence](../evidence/moriarty-completion-program-2026-09-07/MC08/development-plugin-agy-result-02/README.md).

Before repeating a failed launch, inspect its terminal permission metadata and
verify the intended invocation with one harmless command under the same flags.
Do not change global settings or use this procedure to override a safety denial.

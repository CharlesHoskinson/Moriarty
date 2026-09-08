# FOREMAN_REPORT

Worker: Grok 4.6 high implementation author
Worktree: `/home/charl/Moriarty/.worktrees/sp01-loan-swap-grok`
Task: SP01.6 RP01-MC02 loan and swap semantic design subset
Phase: GPT-6 D1-D5 fidelity correction. Documentation and specified traces only. No command execution.

## Scope

This worker corrected specified design and independent expected traces for the existing atomic loan and swap subset.
The subset covers loan first-period accrue then settle.
It also covers swap exact-input minimum-receive then close.
The artifacts record existing source behavior.
They do not freeze successor semantics.

## Owned files

- `evidence/moriarty-completion-program-2026-09-07/report-reconciliation/semantic-challenges.json`
- `evidence/moriarty-completion-program-2026-09-07/SP01/loan-swap-subset-01/design.md`
- `evidence/moriarty-completion-program-2026-09-07/SP01/loan-swap-subset-01/traces.json`
- `FOREMAN_REPORT.md`
- `FOREMAN_REPORT.json`

The worker did not edit code, specs, tests, the register, or the wiki.

## Work not done by this worker

The worker did not run tests or commands.
The worker did not compute output SHA-256 digests.
The worker did not run the evaluator, Compact, wallets, native proving, or public actions.
Prior static schema check failed on DueCreated `settlement:null`.
Current traces omit that illegal field.
Parent verifies bytes and freezes hashes.
Fresh GPT-6 reviews the correction independently.

## Concrete corrections

D1. Exact DueCreated wire records omit `settlement`. `settlement:null` is illegal on that closed variant. Design states the distinction. Traces already omitted the field. No full traces rewrite.

D2. `negativeFixtureConvention` is top-level, not a `negativeTraces` id. Each negative record references it. `loan-replayed-due` uses after-accrue with one Settled status, not exhausted after-settle remaining 0. `swap-no-reserve-remaining-one` uses revision 7 remaining 1 on lifetime 8 with reseal and rebind. Wrong-role source guard binds actor, principal, authenticatedPrincipal, and genesis to the wrong role, then fails the source guard. Actor/principal mismatch stays `PRINCIPAL_BINDING`. ExactPlan extra and reorder controls use a new simulated statement with a simulation-only signature assumption. Net-goal fixture uses sufficient gross cap `maxUInt128`, not the 101 debit-cap fixture. Present false `checks` booleans give named rejections. Omitted required `checks` field is `INPUT_SCHEMA`. Client booleans are a malformed third `evaluate` argument, not EvaluationInput fields.

D3. Authority prose now separates AuthorityCommon, ExactPlanStatement, and OutcomeStatement. OutcomeStatement does not contain ActionCall or `min_out`. Neither statement signs current ObservationSet. Genesis indirectly commits observation and principal bindings. `actionHash` and `observationsHash` bind the evaluated invocation afterward.

D4. Four row read-footprint paragraphs use `sourcePreStateReads`, `sourceStagedReads`, `sourceConstantsArgumentsObservations`, and `sharedDependencyRef`. They reference `sharedEvaluationDependencies`. Listed source reads do not account for entire evaluator work.

D5. Design and row maps reference `sharedObservationMap`. That map covers Complete, CompleteBody, after StateEnvelope, after StateBody, ProofContext, `authorityConsumption.mode` / `principal` / `nonce` / `statementDigest`, `obligationDelta`, all eight `ResourceCounts` members, and after structural fields. Known counts stay. Uncomputed counts stay pending derivation. Do not invent counts or hashes.

Promotion history. Authored base `f173` is preserved. Current independent local atomic acceptance code `c30aaf5c0d091b1efea7e042d0fcbd539ca6cc51` and evidence `b296ce95afae722177b5bba482d1d195fa334d1e` are bound separately. SP01.8 is no longer pending. This record remains pending-review and does not self-approve.

Network. Assignment `raw/assignments/moriarty-midnight-milestone-testing-2026-09-07.md`. SP05 still needs meaningful loan and swap Preview transactions, finalized receipts, full comparisons, and rejection controls after separately reviewed fixtures, custody, contracts, and campaign. Source, spec, and local checks do not discharge them. Mandatory-proof SP09 and later network gates stay unchanged.

## Gate status written

RP01 status is pending-review.
Subset RP01-MC02 status is pending-review.
Subset RP01-MC03 status is specified-only.
Full RP01 remains specified-only and incomplete.
`reviews` arrays are empty.
`candidateHash` values are null.
Output digests are null with status parent-freeze-pending.

The main record does not contain the required challenge, operator, or theorem rows.

## Unresolved gaps

Full RP01 remains SP01.2 work.
Signing and display successor schema remains SP01.3 work.
Native-statement subset RP01-MC03 remains SP01.7 work.
Preview loan and swap network tests remain SP05 work.
Pool and provider custody authorization remains MC02, MC04, and MC05 work.
The four mandatory judgments remain named and not discharged.
Sample source constants bind expected amounts to this fixture only.
Future loan servicing, general AMM behavior, exact-output, cancellation, races, and private workflow remain unsupported.

## Reviews

Independent GPT-6 final review is pending.
Parent verification is pending.
This writing phase makes no success, passed, or executed claim.

# Six independent developer trials

User steering: after the beta is repaired and verified, three Sonnet5.5 agents
and three Grok4.7 agents at xhigh effort write programs and report DevEx feedback.
Each also proposes a tutorial/getting-started GitHub-hosted documentation website.
The user authorizes installation of required dependencies. No model substitution.

Readiness: exact no-tool Sonnet canary returned READY with
`modelUsage.claude-sonnet-5-5.canonicalModel=claude-sonnet-5-5`. Grok's model list
includes grok-4.7; a no-tool xhigh canary is recorded separately. Foreman installed
runtime verification returned Pass. Detailed receipts retain requested versus
observable model/effort identity, terminal status and source hashes.

## Entry and containment

Start after blocking beta code defects are repaired, fresh checks pass and independent
code review has no unresolved high/medium finding. Tracked low findings remain visible
for the final DevEx repair pass; entry does not claim the full loop complete. Use the actual packed artifact
installed outside Moriarty. Each developer has its own isolated consumer project,
source/scenarios/tests and report. No compiler edit, wallet, signing, proof, ledger
send, host configuration change or maintainer campaign is part of the trial.
At most three active workers alongside the lead, in two waves. Read product docs
and shipped AI authoring pack; no other developer's feedback/programs.

## Tasks

| Seat | Exact requested producer | Program assignment |
| --- | --- | --- |
| S1 | claude-sonnet-5-5 | Merchant invoice with exact decimals, fee/cap sugar; valid and insufficient-authority cases |
| S2 | claude-sonnet-5-5 | Partial and full funded loan repayment; retained residual debt, expiry and overpayment cases |
| S3 | claude-sonnet-5-5 | AMM and oracle authoring; checked nominal units, missing/stale evidence discussion, unsupported execution |
| G1 | grok-4.7, xhigh | Payroll intents for different recipients/nonces; gross/fee accounting and a wrong-asset negative case |
| G2 | grok-4.7, xhigh | Loan servicing with exact quantities and round windows; complete effects, replay, balance and work failures |
| G3 | grok-4.7, xhigh | Bridge plus staking authoring/composition; nominal cross-domain assets, pending duties and unsupported horizon boundaries |

Each writes at least one original complete `.mori` program, closed scenario/case
files when locally executable, and a genuine invalid variant. Run check, fmt,
inspect and relevant expand/simulate/test commands. Use LSP/MCP if helpful; report
actual use separately from recommendations. Do not weaken signed financial limits
to make a failing test pass. Record exact commands/results, errors and attempted
fixes; successful authoring does not mean authenticated execution.

## Feedback contract

1. Artifact list and reproduced outcomes; which features are local/specified/open.
2. Concrete syntax, quantity, names/accounts/assets and intent/scenario friction.
3. Diagnostics, formatting, command discovery, output and editor/AI integration.
4. High/medium/minor product problems, exact reproduction and proposed repair.
5. Separate learnability/design preference from actual bugs; preserve dissent.
6. Tutorial website: landing page, navigation, prerequisites/install, first five
   minutes, first transfer, repayment, units/identity, intent versus scenario,
   first failures, project tests, editor/AI setup, financial support matrix,
   proof/authentication/ledger boundaries, reference and next steps. Recommend
   page sequence and example output based on the developer's actual first run.
7. Content to omit from getting started, missing information and a tutorial
   acceptance checklist. Framework/deployment choices are suggestions, not a
   claim of built or activated website support.

## Reconciliation

The lead reruns every program/case from the installed artifact, checks original
source/receipt digests, and records per-seat completed/unavailable/interrupted
status. Aggregate recurring feedback plus minority views in DEVEX-RESULT.md.
Reproduce and repair real implementation defects, retest and re-review the exact
affected product bytes. Turn tutorial recommendations into a concrete content
plan; user asked for comments, not website deployment. Archive programs and scoped
feedback in wiki-llm with provenance and model identity. Never store private model
reasoning, credentials or unredacted environments.

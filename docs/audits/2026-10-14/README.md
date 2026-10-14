# Moriarty mid-roadmap audit discovery

Source baseline: `cb8280ef54082853dd8bb6c6a52050328d0fc6fa` (tree `ce05a5c777e5e60f9bebfb1f5a80df78a2556096`).

This package contains six Luna mapping passes, five independent GPT proposals,
five independent Opus proposals, a follow-up Opus security review, and the
resulting audit plan and roadmap reconciliation. It is static reconnaissance
and audit design. No product tests, new proof/ledger operations or milestone
acceptance occurred in this review.

Start with:

- [Mid-roadmap review](MID-ROADMAP-REVIEW.md): working child capabilities, integration gaps, unchanged U exits, proposal dispositions and next demonstrations.
- [End-to-end audit plan](AUDIT-PLAN.md): exact output records, independent program/oracle walkthroughs, workstreams and completion criteria.
- [Opus security review](SECURITY-REVIEW.md): Opus-authored source observations, hypotheses, security work and limits. GPT supplies no security verdict.
- [Proposal index](proposal-index.json): all ten original seats, report hashes, requested models and available identity receipts.
- [Retained-obligation index](obligation-index.json): 69 U/M/MC/SP/G IDs; additional full requirements retained. This is not the atomic denominator or a completion score.

## Knowledge graph

The [combined graph](knowledge-graph.json.gz) is gzip-compressed NetworkX node-link
JSON (`nodes` and `links`, directed multigraph). It contains 30,143
tracked-file inventory nodes, 46,421 total nodes and 57,298
relationships. Its [manifest](knowledge-graph-manifest.json) binds the exact bytes
and source tree. Parallel raw relationships are retained rather than silently
collapsed. The [coverage report](graph-coverage-summary.json) separates inventory,
structural extraction, selective semantic inspection, and exclusions.

Six semantic passes produced 231 nodes and 321 relationships over 120 unique
public source/context paths. Hash/citation-location checks establish input
identity and navigable references, not truth of every claim. Original Luna
reports remain unchanged. [Profile corrections](graph-profile-corrections.json)
fix Rust `moriarty/1` / `moriarty-core/1` labels and distinguish the experimental
TypeScript Core/5. The combined graph applies those corrections. The additional
[active import closure](active-import-closure.json) and AST extraction cover the
three experimental modules imported by the shipped Node beta bridge.

Inventory coverage is complete for the baseline's tracked paths. Semantic audit
coverage is bounded: archived, protected and binary artifacts are metadata-only;
`.mori`/`.qnt` files lack generic AST parsing; unresolved references and original
extractor limitations are disclosed. The graph is navigation context, not a
soundness theorem or a whole-repository completion claim.

## Routing and preserved disagreement

Models requested: `gpt-6-luna` high for mapping, `gpt-6.1-sol` high for the five GPT
proposals, and `claude-opus-5-5` high for five Opus proposals and the focused
security follow-up. The five Opus canonical identities are confirmed in filtered
terminal receipts; host-selected GPT/Luna models have no separate returned
provider attestation exposed by the subagent interface.

All security analysis, recommendations, tests and judgments belong to Opus under
the [latest user steering](SECURITY-ROUTING.md). GPT proposals are nonsecurity.
The superseded GPT security draft is not adopted or included as an active
proposal. Initial proposals were independent; reconciliation began only after
their initial scope. Their recommendations are advisory and may disagree with
each other or with the controlling user instruction. The mid-roadmap review and
Opus report identify the adopted corrections; original reports are not edited
to manufacture agreement.

No existing native contract, counter or resource allowance was reset. Native
execution authority stayed zero. Missing actual target configuration and native
acceptance evidence remain blockers for the full end-to-end audit. See the plan
for the distinction between source-static construction, ordinary reference work
and blocked SDK/VM/proof/ledger operations.

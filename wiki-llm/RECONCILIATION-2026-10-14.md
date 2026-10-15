# wiki-llm research and current source — October 14, 2026

This is a nonsecurity source/documentation reconciliation against merged
[PR50](https://github.com/CharlesHoskinson/Moriarty/pull/50), commit
`fc6353bf19b87a76410319c378efe529d52d23d9`. It supplies navigation and corrects
stale current-status prose. It records no new product tests, native execution,
network observation, design adoption or roadmap acceptance. Security analysis
and current security dispositions belong to [Opus](../docs/audits/2026-10-14/SECURITY-ROUTING.md).

The [roadmap](../ROADMAP.md) remains the single U0–U7 delivery sequence. Its
[M1–M17 crosswalk](../openspec/changes/consolidated-language-kernel/traceability.md#fenrir-language-capability-crosswalk--2026-10-14)
records direct user requirements, not proposals awaiting another adoption vote.
The [mid-roadmap review](../docs/audits/2026-10-14/MID-ROADMAP-REVIEW.md) and
[audit plan](../docs/audits/2026-10-14/AUDIT-PLAN.md) control current capability
qualification and next demonstrations.

## Profiles and actual consumers

| Surface | Actual source/consumer | Current qualification |
| --- | --- | --- |
| Rust Core | [`ir.rs`, constants `LANGUAGE` and `SEMANTICS`](../crates/mori-core/src/ir.rs); [`parser`](../crates/mori-core/src/parser.rs); [`runtime`](../crates/mori-core/src/runtime.rs) | `moriarty/1` / `moriarty-core/1`; bounded parsing, validation and reference execution. Calling this Rust profile “Core/5” is incorrect. |
| Rust transition compiler | [`compile` command](../crates/mori/src/commands/compile.rs), [`mori-backend`](../crates/mori-backend/src/lib.rs) | Implemented source-to-ZKIR consumer for its admitted subset. Compilation, proof, target installation and settlement remain separate stages. |
| Rust static call compiler | [`compile-call`](../crates/mori/src/commands/compile_call.rs), [`mori-ledger` source-static compiler](../crates/mori-ledger/src/static_compile.rs) | All-action export/exact reopen and explicit mapped-role mode; captured PR47/49 static scope. It is a different lowering/statement from the transition compiler. |
| Rust local product tools | [`simulate`](../crates/mori/src/commands/simulate.rs), [`test`](../crates/mori/src/commands/test.rs), [`intent`](../crates/mori/src/commands/intent.rs), [`replay`](../crates/mori/src/commands/replay.rs) | Real consumers exist. “All CLI commands are stubs” and “the compiler is missing” are obsolete descriptions of this supported Rust scope. These paths were inspected, not executed by this reconciliation. |
| Node beta | [`package.json`](../packages/moriarty-beta/package.json), [`frontend`](../packages/moriarty-beta/src/frontend.ts), [`bridge` imports](../packages/moriarty-beta/src/bridge.ts) | The beta lowers through the experimental TypeScript Source/6 parser/preparer. Core/5 belongs to that separate experiment; local money preparation remains `PreparedUnqualified`. |
| Node editor and AI tools | [`servers.ts`](../packages/moriarty-beta/src/servers.ts), [`editor/`](../packages/moriarty-beta/editor/), [`ai/`](../packages/moriarty-beta/ai/) | Delivered bounded authoring services/assets. Historical real-client observations are scoped in [RESULT](beta-language-2026-09-30/RESULT.md#executed-checks). |
| Rust editor and AI tools | [`lsp.rs`](../crates/mori/src/commands/lsp.rs), [`mcp.rs`](../crates/mori/src/commands/mcp.rs) | Their command bodies remain `todo!`. Node delivery does not implement these Rust handlers. |

Both distributions expose an executable named `mori`. Select the intended
installation and source profile explicitly. A successful beta check cannot
establish Rust compilation of that source. The [profile corrections](../docs/audits/2026-10-14/graph-profile-corrections.json)
retain exact source locators, and the [native Core guide](../docs/NATIVE-CORE.md)
documents its own grammar, commands and refusal limits. No canonical profile or
lowering unification is selected by this reconciliation.

## Research decisions and later delivery

The ten domain memos are September 30 research. Their manuals, acquisitions,
proposed examples and disagreements keep their original source dates and
coverage. The following dispositions explain how to use them now.

| Research area | Historical record and disposition | Current source or remaining work |
| --- | --- | --- |
| Surface | [01](beta-language-2026-09-30/domains/01-surface.md), [candidate](beta-language-2026-09-30/BETA-DESIGN.md) and [convergence](beta-language-2026-09-30/CONVERGENCE.md) select the bounded brace DSL, prior-only bindings and named calls | [Node frontend](../packages/moriarty-beta/src/frontend.ts) implements the selected beta; general functions/match/imports in mockups remain separate proposed constructs. |
| Types and exact amounts | [02](beta-language-2026-09-30/domains/02-types.md) recommends nominal identities and quantities | The original Quote-per-Base recommendation was rejected in [convergence](beta-language-2026-09-30/CONVERGENCE.md#selection-and-preserved-dissent). Preserve Base-per-Quote; do not silently move a successor numeric convention into another profile. |
| Agreements and complete effects | [03](beta-language-2026-09-30/domains/03-agreements.md) separates fixed intent, completion and supplied state | The Node [bridge](../packages/moriarty-beta/src/bridge.ts) is the concrete S0 consumer. Rust [Core IR/runtime](../crates/mori-core/README.md) is a separate implementation with its own supported statements and effects. |
| Workflows | [04](beta-language-2026-09-30/domains/04-workflows.md) proposes persistent stages/episodes | Rust [policy](../crates/mori-core/src/policy.rs) and [sponsorship](../crates/mori-core/src/source_sponsor.rs) are bounded local precursors; full U3 conditions, joins, late outcomes, recovery and target effects remain open. |
| Distribution and tests | [05](beta-language-2026-09-30/domains/05-toolchain.md) originally proposed a distributable driver | The [beta package](../packages/moriarty-beta/package.json) and [final captured result](beta-language-2026-09-30/RESULT.md) supply the Node delivery; Rust has its own manifest-based [test command](../crates/mori/src/commands/test.rs). General library/linking and U7 acceptance remain separate. |
| Editors | [06](beta-language-2026-09-30/domains/06-editors.md) proposed common analysis plus thin clients | Node services/editor assets exist; Rust server stubs remain. Historical protocol/Neovim observations do not imply current VS Code GUI or all-client activation. |
| AI tools | [07](beta-language-2026-09-30/domains/07-ai.md) proposed portable guidance and three `moriarty_*` tool names | The shipped Node [server](../packages/moriarty-beta/src/servers.ts) exposes `check`, `inspect`, `expand`, `preview`; one historical Claude Code check/inspect observation is not broad provider adoption. |
| Language workbenches | [08](beta-language-2026-09-30/domains/08-meta-tools.md) compares Langium/Xtext/Chevrotain/Tree-sitter/MLIR | The selected Node frontend is handwritten TypeScript. No workbench migration or new backend follows from these comparisons; Rust's separate implementation is not evidence of such a migration. |
| Ecosystems | [09](beta-language-2026-09-30/domains/09-ecosystems.md) compares representation and financial notation | Its external ecosystem examples are research inputs, not Moriarty backend delivery or financial conformance. |
| Popular syntax supplement | [10](beta-language-2026-09-30/domains/10-popular-syntax-addendum.md) adds bounded Java/C#/C++ feature evidence | The feature-documentation gap was addressed; adoption ranking and human comprehension remain unmeasured. |

The [preliminary alternatives](beta-language-2026-09-30/DECISION-CANDIDATE.md)
and [implementation plan](beta-language-2026-09-30/IMPLEMENTATION-PLAN.md) are
completed authoring-beta history. [Developer trials](beta-language-2026-09-30/DEVEX-RESULT.md)
preserve the original six-seat artificial-user cohort, weaker assertion rows,
independent preparations and original failures. Their case counts do not become
new executions or human-usability evidence.

The [programmer mockup](beta-language-2026-09-30/PROGRAMMER-MOCKUP.md) and
[full-language horizon](beta-language-2026-09-30/FULL-LANGUAGE-HORIZON.md)
are hash-bound reviewed design artifacts. They retain original proposed grammar,
local-only rows and pending-review wording. The later [final result](beta-language-2026-09-30/RESULT.md#research-and-design-review)
records the final scoped design reviews. General ADTs, certified arithmetic,
full lifecycle/episode semantics and full financial execution are not supplied
by those design reviews or checked proposal-coverage boxes.

The [kernel recommendation](kernel-api-recommendation-2026-09-29.md),
[consensus candidate](kernel-api-consensus-2026-09-29.md),
[research audit](kernel-api-audit-2026-09-29.md), [Aeon probes](aeon-kernel-experiment-evidence-2026-09-29.md),
[K surge](mil4-k-semantics-surge-2026-09-29.md) and
[S0 executable boundary](mil4-s0-executable-boundary-2026-09-30.md) retain their
September 29–30 research/finite-experiment scope. Their initial recommendations
and dispatch suggestions are not the current execution queue. A compiled K file,
finite comparison or advisory agreement does not close U0–U7.

## Website and historical evidence

The [September 30 content plan](beta-language-2026-09-30/TUTORIAL-SITE-PLAN.md)
was followed by an authorized React/Vite implementation. The
[October 1 plan](website-2026-10-01/PLAN.md),
[polish/local result](website-2026-10-01/POLISH-DECISION.md) and
[root R2 publication decision](website-2026-10-01/publication-reviews/R2-ROOT-PUBLICATION-DECISION.md)
record that progression. Current source includes tutorial, how-to, reference,
explanation and documentation entry pages in [`site/`](../site/).
Website implementation now exists; the old unchecked deployment item is not
evidence that site source is absent. Current public availability and current-candidate
release acceptance were not observed here. Original image prompts, design reviews and browser captures
remain evidence of their original candidates.

All `RESULT`, `REVIEW`, repair-result and receipt narratives retain their original
assertions, identities, failures and measured trees. Original source captures,
provider transcripts, developer-trial archives and native/private artifacts are
not rewritten. Historical absolute acquisition paths are provenance, not a
promise that another checkout contains those files. The maintained nonsecurity
relative navigation checked in this pass resolves against tracked paths; a
resolved historical evidence link does not make that evidence current.

## Actionable remaining work

- Complete the U0 profile/consumer/grammar matrix and independent numeric/work
  expectations; compare both lowerings only on their documented common subset.
- Produce the audit plan's two positive contrasting programs through the same
  claimed pipeline with independent complete effects/state expectations. Preserve
  the separate static/refusal track and all downstream blocked stages.
- Repair Rust LSP/MCP behavior and the retained aggregate test-report size failure
  under their own scoped implementation tasks. Reconcile executable installation
  and profile documentation so the two `mori` tools are distinguishable.
- Preserve full ACTUS and DeFi membership and composition obligations. FeePool's
  captured 23 actions/162 cases are child evidence, not full M11 or U6 acceptance.
- Leave Fenrir network/Anchor, committee/registry and qualified target facts unknown
  until evidence exists. Preview receipts and Fenrir qualification remain distinct.
- Obtain the separately admitted native/target, recursive/private-history and
  independent developer-release evidence required by U1–U7. Native authority
  remains zero; no SDK checks/queries, VM, keygen, prove/verify or ledger work was
  authorized by this documentation task.

No U/M/MC/SP/G exit is closed here, and no whole-language completion percentage
is assigned. Security-specific currentness questions are handed to Opus; this
nonsecurity crosswalk supplies no security verdict.

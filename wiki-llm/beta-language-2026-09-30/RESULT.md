# Moriarty authoring beta: delivery result

2026-09-30. Research, design convergence, beta implementation, final independent implementation review and the six requested developer trials are complete. Publication is being reconciled; see the publication section for the observed state. This report separates repository observations, experiments and proposed future language features.

## Delivered language and tools

[Getting started](../../packages/moriarty-beta/GETTING-STARTED.md) and the [package reference](../../packages/moriarty-beta/README.md) describe `@moriarty-lang/beta`0.1.0-beta.1 (Node24+). The implementation supplies a bounded brace DSL, exact decimal asset quantities, nominal domain/asset/account/economic IDs, prior-declaration constants and checked arithmetic, named financial calls, explicit intent bounds, reusable authoring checks and origin maps. Sugar lowers through the existing Source/6–Core/5 path for transfer and funded AccrualFirst repayment. Financial first-failure ordering and complete ordered effects/post-state/work/replay stay with that implementation.

The standalone package includes CLI init/check/fmt/inspect/expand/simulate/test, transfer and repayment templates, duplicate-safe closed scenario/test schemas, bounded LSP services, highlighting/snippets, VS Code extension assets, Neovim setup and read-only MCP check/inspect tools. AI project adapters are optional inert guidance. The parser, inspect projection, JSON and transports have explicit budgets; no unbounded loop or floating money is introduced.

All eight DeFi areas have readable [examples](PROGRAMMER-MOCKUP.md) and visible locally implemented/specified/open labels. Twenty-four non-S0 operation schemas receive bounded authoring checks; their financial execution returns Unsupported with no published effects. The [typed full-language horizon](FULL-LANGUAGE-HORIZON.md) specifies richer lifecycle, proof, duty, authority, recovery and composed bridge/swap interfaces. It is proposed `moriarty-horizon/0.1`, separate from executable beta grammar.

## Research and design review

[Nine domain memos and the Java/C#/C++ supplement](README.md#domains) cover mainstream syntax, financial resources, agreements/authority, asynchronous stages, packages/testing, editors, AI integrations, language meta tools and blockchain notation. Main acquisition preserved25 targets/26 Scrapling attempts; the bounded supplement adds3 targets/3 captures:28 targets/29 captures total, excluding auxiliary robots checks and previously dated reused sources. [Source receipts](evidence/source-receipts/) preserve URLs/status/digests/limits. Popularity is scoped to the cited survey/platform cohort; manuals establish features, not ranking. FpML access, robots observations, reused Daml evidence and browser-only material retain their stated limitations.

[Synthesis](SYNTHESIS.md) and [convergence](CONVERGENCE.md) explain the selected syntax and alternatives. Five requested GPT-6.1 Sol medium design seats independently challenged the frozen original candidate; original reports remain in [audits](audits/). Host routing acknowledged those model/effort requests but exposes no separate provider attestation. The votes are design reviews, not proofs. Dissent includes price orientation: controlling U0 Base-per-Quote is retained. The meta-tool result selects shared bounded TypeScript services for this beta and defers a frontend/backend framework migration.

Full-horizon revisions preserve every original finding and rejection. Final [PL4 horizon review](audits/04-horizon-v5.md) and [PL5 horizon review](audits/05-horizon-v5.md) agree on the specification after literal arithmetic/lifecycle repairs, with implementation and financial acceptance withheld. Final mockup SHA256: `66a0afd0bc127eaf81ec58de16a86c484852bccdc7b849313302d684a036acc7`; horizon SHA256: `97f7f4b122f3ea59460914a1e577c0da69619006a73b7dc8377b3815b9907a29`.

## Current implementation review

The [v6 manifest](audits/code-candidate-v6.json) binds62 files at aggregate SHA256 `ea4f68158d893d6fa71d3048c3d164ac54912397604b134e6f52ff08a9738da6`, baseline `870f998b36ecda04622fa4274132e74902942d0b`. All final file hashes were recomputed. v6 changes only the guide/convergence wording from v5; production code is identical. The stage/intent retained-duty distinction is now explicit. Earlier audits and repair dispositions remain immutable evidence of their recorded trees.

The [Astra final reconciliation](audits/code-astra-v6.md) approves the complete byte-verified current candidate after whole-production inspection, with no remaining findings. The [fresh full Grok4.6 high review](audits/code-grok-v6.md), with [returned receipt](audits/code-grok-v6.receipt.json), also accepts this freeze. Neither reviewer identifies remaining High/Medium findings. Grok records one Low SpecifiedOnly limitation: when a non-bridge intent omits its asset header, gross/fee cap quantities receive quantity/domain checking but no inferred operation-asset comparison. Use an explicit matching asset header; stricter standalone-hint admission remains open. This does not affect S0 caps or execute a horizon financial relation. Host-routed Astra identity/effort lacks a separate provider attestation. Grok requested/returned model and configured effort must be read from its actual receipt; configured effort is not separately provider-attested.

## Executed checks

| Observation | Result and scope |
| --- | --- |
| [Current beta tests](evidence/beta-tests-final-v6.txt) |79/79; includes real packed external CLI/API/declarations and installed editor-server equality |
| [Existing language regression](evidence/regression-final.txt) |938/938,14 suites; existing Source/6/Core/5 caller modules unchanged |
| [Beta typecheck](evidence/typecheck-code-v5.txt) and [experiment typecheck receipt](evidence/experiments-typecheck-v5.receipt.json) |Strict TypeScript checks successful on byte-identical production code |
| [Neovim client](evidence/nvim-final-v5.txt) |Actual initialize/open/navigation/symbols/format/change/diagnostics/shutdown smoke |
| [Extracted VSIX server](evidence/vsix-protocol-final-v5.txt) |13/13 protocol tests outside checkout; real VS Code GUI activation unperformed |
| [Native Claude Code MCP](evidence/claude-mcp-activation-v5.json) |Real connected server and two correlated check/inspect calls; exact Sonnet5.5, successful exit; no other-provider/adaptor adoption claim |
| [Final artifact hashes](evidence/artifact-hashes-v6.json) |Fresh v6 tarball installed outside checkout; installed CLI/current dist/extracted VSIX server byte-identical |

Final tarball SHA256 is `f6de131ef477c3139ca43a9c5f416daadbc11ef506dbbb4309f85b1a242bbd74`; VSIX SHA256 is `e0eacb55ae09094e1d20b5b872d9612884d2edde538fde2a6b8b9494e61c1c8f`. The executable SHA256 is `4398d03a03d613c1f986a26c883d99b70873ceed4b959f8a9086b21653979f1b`. Evidence names retain the measured revision; older receipts are not renamed as new experiments.

## Six developer trials and tutorial

[DevEx results](DEVEX-RESULT.md) preserve all six original reports, programs, fixtures, command logs, feedback, actual-model receipts and parent reproductions. S1/S2/S3 used exact Sonnet5.5. G1/G2/G3 used exact Grok4.7 with configured xhigh effort and returned Grok4.7-build identities. No substitution is counted. Trials used the same frozen v3 package; parent reruns now use the final installed v6 package.

All65 original cases reproduce, plus6 copied starter cases. Original outcomes are12 PreparedUnqualified,17 CoreRejected,12 AuthoringRejected,21 Unsupported and3 FormationRejected. Of12 successful original preparations,11 assert full effects and10 full post-state; the two weaker S2 rows remain explicit coverage limits. Separate G3 local probes execute transfer/repay, not bridge/staking. The separate four-row S1 deliberately wrong-result control fails correctly. [Independent arithmetic](audits/developer-arithmetic-result.md) checks supplied money expectations without importing the implementation. Fixtures remain independent stipulated preparations, not authenticated chained history.

Feedback drove repayment onboarding, precise mismatch/identity diagnostics, readable status/qualification, compact call formatting, quantity hover, help/init errors, UTF8/bounded-inspection handling and horizon nominal checks. Original severities and declined preferences remain visible. The [tutorial website plan](TUTORIAL-SITE-PLAN.md) incorporates each developer’s comments: versioned Git-backed guide, narrow content column, navigation/search/contents/copy controls and explicit support words. The supplied guide implements transfer/repay/schema content. Website deployment/framework benchmarking and measured human onboarding are unperformed.

## Wiki, scope and open obligations

Research and decisions are filed under `wiki-llm`; canonical navigation was saved through an inspected portable transaction, with [inspection](evidence/save-navigation-inspect.json) and [apply receipt](evidence/save-navigation-result.json). [Strict canonical lint](evidence/wiki-lint-final.txt) reports20 preexisting findings (11 dead links,7 duplicate basenames,2 stale index entries), no new navigation/frontmatter/provenance/orphan findings. No accepted financial claim was promoted.

Every successful local money result remains **PreparedUnqualified/local-stipulation-only**. Four external premises remain canonical intent signature, snapshot-to-head, head extension and atomic ledger compare-and-consume; four unverified bindings remain agreement ID, selected program, asset scale and authenticated predecessor. Source hashes/typed IDs/prose policies do not authenticate claims. Signatures, native proof/correspondence, atomic financial ledger acceptance and full eight-area conformance remain open. Existing campaign/resource stops remain unchanged. This beta advances language authoring and demonstrable local tooling; those separate product obligations are not marked complete.

## Publication

Not yet published. Final current reviewer reconciliation and scoped delivery review precede commit, pull request, main integration and beta artifact publication. No npm registry or tutorial-site deployment is claimed.

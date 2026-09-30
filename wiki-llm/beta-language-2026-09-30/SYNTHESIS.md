# Research synthesis and selected inspirations

## Evidence scope

Nine domain threads inspected official language/protocol/framework material.
The [source inventory](source-inventory.json) records 26 Scrapling attempts for
25 document targets: Kotlin required one same-source canonical redirect followup.
All retained payload/content hashes were checked. Domain manifests record the
actual inspected sections and older corpus reuse; downloading a manual never
means every section was studied. OpenAI browser inspections are separate from
immutable captures. AI robots checks occurred after capture; retain that timing
limitation. No authenticated access or WAF bypass was used.

GitHub's [Octoverse report](https://github.blog/news-insights/octoverse/octoverse-a-new-developer-joins-github-every-second-as-ai-leads-typescript-to-1/)
provides a platform-specific popularity signal: TypeScript overtook Python and
JavaScript in its August 2025 measurement. It does not establish universal
language popularity or a best syntax. Its reporting window is September
2024–August2025; published October28 2025 and updated February28 2026. The first
selector captured sidebar material, so the lead extracted the retained HTML's
main element offline; its separate receipt is preserved. No causal claim about
AI follows from that ranking.

The research cohort emphasizes TypeScript/Python plus Rust/Go/Kotlin/Swift,
Elm/Unison and financial/chain languages. It is not a fresh comparative survey
of every major Java/C#/C++ feature or language's adoption. That gap limits any
claim about the globally most popular design; it does not supply a reason to
copy unchecked semantics into Moriarty.
A later [three-source Java/C#/C++ addendum](domains/10-popular-syntax-addendum.md)
adds scoped record/pattern/value/resource documentation. It closes that named
feature-documentation gap, while leaving comprehensive adoption comparisons open.
The main26-attempt inventory is unchanged; the supplement adds3 captures.

## Decisions by domain

| Domain | Borrowed inspiration | Selected Moriarty choice |
| --- | --- | --- |
| Surface | TS annotations/records; Python named arguments | Braces, semicolons, prior-only immutable const, closed named calls and exact Qty sugar |
| Types | Rust nominal wrappers/checked outcomes; Move resource abilities | Distinct domain/account/asset/obligation claims; no scalar/money coercion; linear rights remain a separate horizon |
| Financial agreements | Daml role/choice distinctions; FpML parties, accounts and lifecycle terms | Source-fixed intent separate from local completion/provider facts; party != account != authority |
| Workflows | Kotlin cancellation and Swift structured concurrency | Ledger-linked one-domain stages and durable episodes; timeouts never prove nonreceipt |
| Packages/tests | Cargo layout and Go explicit tooling | Standalone bundled CLI/library, finite project cases, independently explicit financial oracles |
| Editors | LSP3.17 and VS Code language services | One strict analysis API, Full-sync stdio LSP, TextMate/snippets and thin IDE client |
| AI | Repository instructions/skills and MCP | Project-scoped authoring packs and four bounded read-only source-text tools |
| Meta tools | Langium/Xtext/Chevrotain/Tree-sitter/MLIR | Keep TS frontend; defer migration/second grammar/backend until measured needs justify it |
| Chain ecosystems | Sui UID/Coin/capabilities, Solidity addresses/units, Anchor constraints, Cairo typed addresses | Explicit nominal declarations first; no magic symbol implies authenticated address, value or authority |

These are design inferences from the linked domain evidence, not claims that the
borrowed syntax automatically inherits another platform's verifier guarantees.
In particular Sui's `Coin<T>` and `TreasuryCap<T>` are framework concepts, Solidity
native units do not identify ERC assets, and Anchor declarations do not provide
Midnight signature/account checks. Display symbols and decimals never establish
economic identity. Daml signatories/controllers/observers are separate roles.

FpML's fully current5.13 build8 documentation was login-restricted. The thread
inspected historical5.2 architecture and current coding schemes instead; it does
not claim current full-standard conformance. Daml/ACTUS/Marlowe/Simplicity and
Elm/Unison/Compact reuse retained dated evidence with their manifest scope.

## Converged beta and dissent

[Five independent PL reports](audits/01-syntax-types.md) challenged the frozen
[v1 candidate](BETA-DESIGN.md). The [convergence](CONVERGENCE.md) disposes every
substantive finding; reports01/02 append explicit concurrence on corrected
amendments. Model/effort were explicitly requested in host dispatch; a separate
provider-returned identity receipt is unavailable. Agreement is design evidence,
not proof or product acceptance.

Preserve the domain02 price disagreement: its Quote-per-Base recommendation is
not adopted. Moriarty's controlling Base-per-Quote convention stays explicit.
The initially prose-heavy mockup also failed review; the [typed horizon](FULL-LANGUAGE-HORIZON.md)
repairs those interfaces without pretending the beta implements full finance.

See [programmer mockup](PROGRAMMER-MOCKUP.md), [implementation plan](IMPLEMENTATION-PLAN.md)
and [package](../../packages/moriarty-beta/README.md). Exact release measurements,
independent implementation reviews and remaining obligations belong in RESULT,
not in these immutable research snapshots.

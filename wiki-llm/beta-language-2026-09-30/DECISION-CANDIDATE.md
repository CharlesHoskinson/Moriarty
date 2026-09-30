# Beta decisions under research

**Status:** preliminary alternatives, not yet the five-reviewer candidate.

## Three implementation approaches

| Approach | Benefit | Cost and risk | Current recommendation |
| --- | --- | --- | --- |
| External `.mori` language with a bounded typed TypeScript frontend and shared tooling service | Familiar text files and diffs; direct control over nominal financial types, quantities, source provenance and rejection; reuse the demonstrated local S0 contract | Must implement parser recovery, source maps and editor integration deliberately | Leading candidate; confirm against domain research and PL panel |
| Langium grammar workbench with generated parser, services and LSP | Integrated authoring infrastructure; grammar-oriented maintenance | Grammar framework migration and generated tooling do not supply financial semantics; two parsers can drift | Reserve as a measured later option; meta thread must assess requirements |
| Embedded TypeScript DSL | Host ecosystem and existing editors | Host execution, coercion, interpolation, ambient effects and unsigned runtime values complicate a financial language boundary | Do not make the host language authoritative; inert wrappers may follow separately |

## Proposed decision axes

1. Explicit beta profile; Source/6 and earlier profiles retain their versions.
2. Braces, semicolons, immutable bindings, named arguments and trailing commas.
3. Nominal financial identity and exact checked quantity literals; no floats.
4. Domain/network identity, domain-bound accounts and explicit asset representations.
5. Pure, bounded calculations; economic operations use closed typed interfaces.
6. Full source-to-Core expansion and diagnostics retain source locations.
7. Signed scope, submitted completion and scenario/provider inputs remain distinct.
8. One-domain stages and persistent episodes for asynchronous settlement.
9. User-readable errors preserve the first failing judgment and no-effect rejection.
10. Typed authoring support, local simulation, proof status and ledger status are separate.
11. One analysis service feeds CLI, LSP, formatting, IDEs and read-only AI tools.
12. A language-specific signing display explains recipients, fees, assets and duties.
13. Financial libraries cover all eight families in the design; each operation has
    a precise implemented/specified/open boundary.
14. Daml informs participant authority and contract choices; FpML informs trade,
    party/account references, financial terms and lifecycle event structure.
    Their runtime guarantees and interchange schemas are not Moriarty proofs.

## Early contradiction to resolve

The types memo recommends `Price<Base,Quote>` as Quote units per Base. The
controlling U0/MIL/2/MIL/4 convention is **Base units per Quote**. That memo's
recommendation cannot silently change the owner-selected orientation. The beta
candidate will retain the controlling convention, display both units explicitly,
and require any alternative orientation to be a versioned decision with an
explicit conversion. An inverse price is a different typed value. This finding
must appear in the five-reviewer packet and the final convergence record.

## Research still required before freezing a candidate

Complete all eight domain memos, source locators and disagreement records. Select
the exact grammar and smallest coherent implemented profile, together with broad
eight-family examples. The requested five independent PL reviewers then audit the
same frozen candidate before any production beta code is authored.

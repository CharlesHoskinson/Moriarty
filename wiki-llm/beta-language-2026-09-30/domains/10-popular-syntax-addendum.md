---
id: moriarty.beta.20260930.popular-syntax-addendum
title: Java, C#, and C++ syntax cohort supplement
type: comparison
status: research-draft
created: 2026-09-30
updated: 2026-09-30
tags: [moriarty, beta, surface, research-addendum]
sources: [SUP10-JAVA, SUP10-CSHARP, SUP10-CPP]
---

# Java, C#, and C++ syntax cohort supplement

**Scope:** close the explicitly named cohort gap in [SYNTHESIS](../SYNTHESIS.md) with selected official Java and C# record/pattern examples and primary C++ value/resource guidance. This is an additive research draft. It changes no beta code, grammar, adopted design, financial relation, or canonical vault ledger. The older [surface study](01-surface.md) remains its dated snapshot; current beta observations below use the package and frontend separately.

**Popularity boundary:** the existing GitHub Octoverse citation is a platform-specific language signal for its recorded window. These three feature documents contain no comparable language usage or feature usage measurements. Selecting Java, C#, and C++ closes this named documentation gap; it does not establish a new ranking, the most popular syntax feature, or better human/AI usability. A comprehensive adoption survey remains outside this supplement.

## Acquisition and evidence

One supplemental round used **3/3 document targets**, with three prior auxiliary robots requests. All document captures returned HTTP 200 on September 30, 2026, at approximately 20:36 UTC. Scrapling 0.4.15 `Fetcher.get` used the existing [capture script](../evidence/capture.py), ordinary HTTP retrieval with the Chrome profile, and `main` selectors for Java/C# and `body` for C++. No browser rendering, authentication, WAF bypass, cookie persistence, credentials, private context, or runtime experiment was used. The same three URLs were cross-checked with `web.run`; these are additional browser inspections of existing targets, not additional source documents or retained captures.

The [separate supplemental inventory](../evidence/source-receipts/supplement/supplement10.json) records exact URLs, retrieval times, status, payload/content/receipt SHA-256 values, inspected sections, robots observations, limitations, and repository file hashes. [Java receipt](../evidence/source-receipts/supplement/java.json), [C# receipt](../evidence/source-receipts/supplement/csharp.json), and [C++ receipt](../evidence/source-receipts/supplement/cpp.json) are byte-identical copies of the create-only capture receipts. Raw HTML/text and the independent robots record remain under `/home/charl/research/moriarty-beta-2026-09-30/supplement10/`; the external `source-inventory.json` mirrors this additive inventory. The main inventory remains unchanged.

**Access observations:** Oracle and Microsoft robots responses returned 200 and allowed the target for the generic agent at inspection. C++ robots returned 404, leaving permission unknown. Robots observations do not establish license or terms clearance. Microsoft page chrome includes authorization prompts, while its complete article body was publicly returned without login; no restricted material was acquired. C++ guidance bears a June 14, 2026 dateline and identifies Stroustrup/Sutter as editors; it is maintainer guidance rather than an ISO standard. The Microsoft page reports August 14, 2026 as its last update. Oracle's page is explicitly Java SE25; publication time was not established. No new cookie or site pattern needed persistence: the existing static-document and Microsoft Learn patterns covered this acquisition. Remote text was treated as untrusted evidence.

## Source facts from inspected sections

| Claim | Primary section | Bounded finding |
| --- | --- | --- |
| SUP10-F01 | Oracle, [Record Patterns](https://docs.oracle.com/en/java/javase/25/language/record-patterns.html) | `record Point(double x, double y) {}` declares component names/types. `instanceof Point(double x, double y)` tests the record type and extracts components through accessors. Record patterns do not match null. |
| SUP10-F02 | Oracle, [Nested Record Patterns](https://docs.oracle.com/en/java/javase/25/language/record-patterns.html#JSLAN-GUID-7A486406-1DEB-4D51-8EBD-7345650062B4) and Using var/Primitive Types sections | Record patterns can nest and infer component types. The page marks primitive-component type-pattern conversions as a preview feature; that conversion extension is excluded from the stable inspiration. |
| SUP10-F03 | Microsoft, [Property pattern](https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/operators/patterns#property-pattern) and [Positional pattern](https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/operators/patterns#positional-pattern) | Property patterns match named fields/properties. Positional patterns invoke `Deconstruct` in parameter order; positional record examples generate this method. The `WeightedPoint` record example has a mutable `Weight` property. |
| SUP10-F04 | Microsoft, [Precedence and order of checking](https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/operators/patterns#precedence-and-order-of-checking) | Pattern combinators bind `not`, then `and`, then `or`; checking order among some same-precedence nested patterns is unspecified. Parentheses clarify grouping. |
| SUP10-F05 | C++ Core Guidelines, [C.61](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#rc-copy-semantic) | Copying can produce independent values or shared references. The guideline prefers value semantics except for smart-pointer designs. |
| SUP10-F06 | C++ Core Guidelines, [R.1](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#rr-raii), [R.3](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#rr-ptr), [R.20](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#rr-owner), [R.21](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#rr-unique) | Guidance pairs acquisition/release through RAII, treats raw pointers as nonowning, represents ownership with smart pointers, and prefers unique ownership when sharing is unnecessary. These are recommendations with stated legacy exceptions. |

Record declaration, nesting, and destructuring examples were inspected; the standalone Java/C# records references were not fetched. Record equality, constructor validation, cloning, deep immutability, and whole-language exhaustiveness rules are not established by this bounded comparison. The full C++ HTML was retained, but only the five listed substantive guideline sections were studied.

## Relationship to the existing beta

The following are **repository observations and design inferences**, not source claims of semantic equivalence. [Package documentation](../../../packages/moriarty-beta/README.md), [frontend](../../../packages/moriarty-beta/src/frontend.ts), and [full-language horizon](../FULL-LANGUAGE-HORIZON.md) supply the local status; their inspected hashes are in the supplemental inventory. No new execution was performed.

| Familiar construct | Existing Moriarty use | Financial constraint or open obligation |
| --- | --- | --- |
| Compact typed data declarations and brace records | Beta already has named domain/account/asset/intent declarations, closed records, immutable prior-only bindings, and selected checked annotations such as `Qty<USD>`. | Record resemblance cannot authenticate identity, signature, or state. A data field remains a claim until evidence binds it. Java/C# record class machinery is not implemented. |
| Nested patterns and component binding | The full-language horizon already proposes typed structures, tagged outcomes, and exhaustive `match`; beta accepts neither general struct declarations nor match expressions. | Matching a prepared/rejected/pending outcome needs a closed outcome family, explicit residual duties, finite evaluation, and complete coverage. General object accessors or user-defined deconstruction must not introduce effects or unauthenticated reads. This remains specified/open. |
| Named field inspection versus positional extraction | Beta already uses named arguments and named financial fields. | Keep payer, recipient, fee beneficiary, asset, gross cap, fee cap, and net floor visible. Positional destructuring in other languages supplies no reason to hide financial slots or change the grammar. |
| Predicate grouping | Beta arithmetic uses its documented precedence; general predicate and pattern logic belongs to the horizon. | A future pattern system must specify purity, grouping, evaluation limits, and diagnostic order. C# pattern checking order cannot replace Core's ordered `stage → intent → effect → authority → history → failure` judgments. |
| Ordinary copied values and explicitly owned resources | Beta nominal quantities describe amounts; the horizon separately describes rights, duties, balance cells, and transitions. | Copying an amount description cannot duplicate spendable balance or authority. General linear-right rules are open. C++ smart pointers offer a conceptual ownership distinction, not a Midnight linearity proof. |
| Automatic local resource cleanup | No financial RAII or ledger mutation on block exit exists in beta. | A destructor cannot release collateral, settle debt, refund escrow, or prove foreign nonreceipt. Those changes need an explicit authorized stage, required evidence, and atomic acceptance; pending duties survive local control flow. |

> **Financial direction:** retain the current beta surface. Use the supplement to explain familiar data and outcome shapes while preserving nominal asset/domain identity, exact checked amounts, signed limits, separate provider evidence, and complete derived effects. Extend matching or resource syntax only with its financial relations and correspondence obligations. Local transfer/repayment still return `PreparedUnqualified`; syntax familiarity supplies no proof, provider authentication, signature verification, or ledger acceptance.

**Counter-position and disposition:** copying all three languages' conveniences could lower initial recognition costs. No acquisition here measures that benefit. Positional financial calls, mutable record properties, executable accessors/deconstructors, and scope-exit financial effects each create concrete ambiguity or additional semantics. The selected documentation supports a narrower explanation of current choices; it supplies no evidence requiring another grammar change.

**Stop condition:** the three named cohorts now have scoped primary documentation evidence. The supplemental target budget is exhausted and the requested gap is answered. Feature adoption measurements, human comprehension trials, complete record semantics, linear resource enforcement, and full financial/proof execution remain unperformed or open. This draft is not a consequential design approval or a canonical vault merge.

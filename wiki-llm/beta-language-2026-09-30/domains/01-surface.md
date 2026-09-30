---
id: moriarty.beta.20260930.surface
title: Programmer surface and desugaring options
type: comparison
status: active
created: 2026-09-30
updated: 2026-09-30
tags: [moriarty, beta, surface, specified-only]
sources: [SURF-TS, SURF-PY, SURF-U9, SURF-U10, SURF-E2, SURF-E3]
---

# Programmer surface and desugaring options

**Scope:** research and a beta design recommendation, S2, specified only. No beta parser, type checker, source-map implementation, signed codec, signature verifier, proof, or ledger settlement was implemented or tested by this study. The existing local baseline is Source/6 and Core/5 S0; it prepares a transfer or AccrualFirst repayment as `PreparedUnqualified`. Research at checkout `870f998b36ecda04622fa4274132e74902942d0b` on `feat/moriarty-beta-20260930`. This page is review material in `wiki-llm`, not a canonical vault transaction or adopted language decision.

**Recommendation:** use one external `.mori` language with braces, required semicolons after declarations and statements, `name: Type` annotations, closed comma-delimited records, immutable bindings, nominal financial types, exhaustive matches, and named arguments for financial operations. Keep signature, provider evidence, preparation, proof and ledger acceptance visible as distinct interfaces. Every abbreviation must have one bounded typed elaboration with a source origin; no abbreviation may change signed limits, synthesize authenticated facts or publish partial financial effects.

## Evidence and its scope

Exactly two fresh official pages were captured using Scrapling 0.4.15 on September 30. Other pages below reuse September 9 immutable captures. [Source manifest](01-surface/sources.json) contains URLs, exact receipts, SHA-256 values, section locators and coverage limits. Acquisition produced HTTP 200 and substantial text for both fresh pages; these are acquisition observations, not language execution results. The selector can collect overlapping containers; the original HTML is retained for syntax inspection. No cookies, response headers or credentials were retained. Remote prose was treated as evidence, never instructions. No authentication, bypass or additional fresh capture was used. No new site-pattern or cookie persistence was needed: these are ordinary static public documentation pages covered by the existing workflow.

The following are **source facts**; the design decisions in later sections are Moriarty recommendations.

| Claim | Inspected official section and receipt | Narrow observation |
| --- | --- | --- |
| SURF-F01 | TypeScript, [Type Annotations on Variables; Functions](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#type-annotations-on-variables), [receipt](/home/charl/research/moriarty-beta-2026-09-30/surface-typescript/receipt.json) | Variable and parameter annotations follow the name; return annotations follow the parameter list. Initializers and context can supply inferred types. |
| SURF-F02 | TypeScript, [Object Types; Type Aliases](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#object-types), same receipt | Object types enumerate named properties; optional properties may be absent. Type aliases do not create distinct versions of their underlying type. |
| SURF-F03 | TypeScript, [Union Types; any; Type Assertions](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#union-types), same receipt | Union members require narrowing for operations not shared by all members. `any` disables checking; assertions supply no runtime validation. |
| SURF-F04 | Python 3.14.7 tutorial, [4.7 match Statements](https://docs.python.org/3/tutorial/controlflow.html#match-statements), [receipt](/home/charl/research/moriarty-beta-2026-09-30/surface-python/receipt.json) | The first matching case executes; patterns extract components, `_` matches anything, and an unmatched subject can execute no branch. |
| SURF-F05 | Python, [4.9.2 Keyword Arguments; 4.9.3 Special parameters](https://docs.python.org/3/tutorial/controlflow.html#keyword-arguments), same receipt | Calls can name arguments; definitions can require keyword-only arguments. Unknown names and repeated argument assignments are invalid. |
| SURF-F06 | Python, [4.10 Intermezzo: Coding Style](https://docs.python.org/3/tutorial/controlflow.html#intermezzo-coding-style), same receipt | The tutorial recommends four-space indentation, avoiding tabs, consistent names, and spaces around operators. This is guidance rather than a measured comparison with braces. |
| SURF-F07 | Unison, [Tour, Part 3; Part 4 Syntax notes](https://www.unison-lang.org/docs/tour/), [original receipt](../../../raw/sources/language-design-2026-09-09/official/D09-unison-tour.json), [corrected extraction receipt](../../../raw/sources/language-design-2026-09-09/official/D09-unison-tour-body.json) | Function application uses spaces, signatures precede definitions, and whitespace delimits blocks. Pure watch expressions are cached by content hash. |
| SURF-F08 | Unison, [Defining your own data types, The meaning of unique; What happens if we create identical structural types?](https://www.unison-lang.org/docs/fundamentals/data-types/unique-and-structural-types/), [original receipt](../../../raw/sources/language-design-2026-09-09/official/D10-unison-types.json), [corrected extraction receipt](../../../raw/sources/language-design-2026-09-09/official/D10-unison-types-body.json) | Pipe-separated constructors define variants. Unique types distinguish domain meanings; structurally identical types can be interchangeable. |
| SURF-F09 | Elm, [Commands and Subscriptions, element](https://guide.elm-lang.org/effects/), [receipt](../../../raw/sources/language-design-2026-09-09/official/D02-elm-effects.json) | Programs return command and subscription values to the runtime, which performs external work. |
| SURF-F10 | Elm, [Time, Time.Posix and Time.Zone; subscriptions](https://guide.elm-lang.org/effects/time.html), [receipt](../../../raw/sources/language-design-2026-09-09/official/D03-elm-time.json) | The guide distinguishes absolute time, display time zones and subscriptions that produce messages. |

The TypeScript capture is the substantial **Everyday Types chapter**, not the entire handbook; it does not justify claims about the whole TypeScript grammar or exhaustiveness rules. Python's control-flow chapter is not its lexical or expression reference. The September 9 Unison article-body extraction receipts supersede navigation-only text; use those bodies rather than the original `.txt` files. No cited source establishes that any proposed Moriarty syntax is easiest for humans or AI. [The earlier feature checklist](../../../deliverables/language-design-2026-09-09/FEATURE-CHECKLIST.md), especially F01–F03, F05, F13 and F19–F20, already records the external DSL constraint and the need for human task evidence; it is prior repository research, not an independent primary source.

## Baseline that surface changes must preserve

**Repository observations:** [Source/6 contract](../../../experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md), sections Formation and version gate, Source fields to Core/5, and Admission and observation; [closed EBNF](../../../experiments/moriarty-language/spec/successor/financial-agreement-source-v6-grammar.ebnf); [Core/5 types and preparer](../../../experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts); [lexer contract](../../../experiments/moriarty-language/spec/successor/lexical.md). These were inspected at the full commit above, with no execution claim.

The S0 grammar requires one fixed version header, one agreement and one stage proposal, exact field order, explicit empty unsupported components, and action-dependent authenticated cells. Transfer binds owner, recipient, fee recipient, amount and fee; checks gross cap, fee cap and net floor; derives the complete ordered effect vector. Repayment derives the creditor and denomination from an Outstanding obligation and allocates payment to accrued interest before principal. Generic division, nonidentity conversion, minting, observations, accepted failure, retained effects/duties and recovery are outside S0.

Core judgment order is `stage → intent → effect → authority → history → failure`. Rejection reports the first judgment/code with no published post-state or effects. An abbreviation must preserve that order and complete candidate comparison. The source parser retains agreement ID, asset scale and authenticated predecessor, but the current Core preparer does not bind those three fields or an exact `/3` digest. Its intent version label is a type discriminator, not canonical signed bytes. In particular, a string called `source_hash`, `digest` or `key` is not proof of its contents or authenticity.

Record order independence, constructors, `match`, typed literal helpers, arbitrary pure functions and named arguments in this page are **successor proposals**. They cannot be submitted under the existing Source/6 header. A compatibility elaborator must validate the new AST and deliberately lower supported cases to the closed S0 form or typed Core interface. It must reject every unsupported construct with a feature-specific diagnostic. Reinterpreting the existing profile would silently change a published formation contract.

## Competing syntaxes for the same signed payment

All three snippets below state the same proposed signed terms: agreement `Payment`; domain `Preview`; asset `USD` at scale 2; selected action `TransferLiteralFee`; source identity `payment-source`; policy identity `payment-policy`; signer/owner Alice and key reference `alice-key`; nonce `payment-1`; predecessor head `head-1`; inclusive validity rounds 100 through 200; Bob receives 1000 atoms, Treasury receives a fixed fee of 10; gross cap 1010, fee cap 10, net floor 1000. Only terminal success is permitted; observations, disclosures, retained effects and duties are empty; delegation and recovery are absent. The identity strings are illustrative opaque identifiers, **not** real hashes, signature material or chain fixtures.

These are complete comparisons of the **signed-term fragment**, not complete runnable agreements. Authenticated pre-state, verified signature, submitted candidate, post-head evidence and atomic consumption must arrive through separate typed inputs. None is supplied by the spelling `signed`, `intent` or `authenticated`. All beta spellings are specified only.

### A. Braces, closed records and named operation arguments — recommended

```mori
profile "moriarty-beta-surface/0";
agreement Payment {
  domain Preview;
  settlement USD scale 2;
  selected TransferLiteralFee source_hash "payment-source" digest "payment-policy";
  intent {
    signer: Alice, key: "alice-key",
    nonce: "payment-1", pre_head: "head-1",
    valid: rounds(domain: Preview, start: 100, end: 200),
    gross_cap: atoms(asset: USD, value: 1010),
    fee_cap: atoms(asset: USD, value: 10),
    net_floor: atoms(asset: USD, value: 1000),
    failure: SuccessOnly,
    signed_action: transfer(
      from: Alice, to: Bob, fee_to: Treasury,
      value: atoms(asset: USD, value: 1000),
      fee: atoms(asset: USD, value: 10),
    ),
    observations: [], disclosures: [],
    retained_effects: [], retained_duties: [],
    delegation: None, recovery: None,
  }
}
```

### B. Significant indentation and named arguments

```mori
profile "moriarty-beta-layout/0"
agreement Payment:
    domain Preview
    settlement USD scale 2
    selected TransferLiteralFee source_hash "payment-source" digest "payment-policy"
    intent:
        signer: Alice
        key: "alice-key"
        nonce: "payment-1"
        pre_head: "head-1"
        valid: rounds(domain: Preview, start: 100, end: 200)
        gross_cap: atoms(asset: USD, value: 1010)
        fee_cap: atoms(asset: USD, value: 10)
        net_floor: atoms(asset: USD, value: 1000)
        failure: SuccessOnly
        signed_action: transfer(
            from: Alice, to: Bob, fee_to: Treasury,
            value: atoms(asset: USD, value: 1000),
            fee: atoms(asset: USD, value: 10),
        )
        observations: []
        disclosures: []
        retained_effects: []
        retained_duties: []
        delegation: None
        recovery: None
```

### C. Domain clauses and payment sentence

```mori
profile "moriarty-beta-clauses/0";
agreement Payment on Preview settling USD scale 2 {
  select TransferLiteralFee source_hash "payment-source" policy "payment-policy";
  intent signed_by Alice key "alice-key" {
    nonce "payment-1";
    predecessor "head-1";
    valid Preview rounds 100 through 200;
    debit_at_most 1010 atoms USD;
    fee_at_most 10 atoms USD;
    receive_at_least 1000 atoms USD;
    success_only;
    pay 1000 atoms USD from Alice to Bob fee 10 atoms USD to Treasury;
    observations empty; disclosures empty;
    retained_effects empty; retained_duties empty;
    delegation none; recovery none;
  }
}
```

| Candidate | Advantage, as a design inference | Cost or likely misunderstanding | Required elaboration |
| --- | --- | --- | --- |
| A | Fields stay visible; fixed punctuation supports completion, syntax repair and structural diffs; operation labels expose payer and fee beneficiary. | Looks close to TypeScript, so users may expect JS coercion, mutable objects or host execution. Explain the finite typed DSL in the tour. | Closed record checking, label-to-slot resolution, nominal constructors and constant literal normalization. |
| B | Fewer delimiters and a visually strong financial hierarchy. | Moving or pasting indentation can change containment; incremental parsing needs explicit indentation tokens, tab policy and parenthesis continuation rules. | INDENT/DEDENT formation, same typed records and operations as A, with explicit missing-field errors. |
| C | The payment sentence reads as an economic instruction and separates debit, fee and receipt limits. | Many context-sensitive words and word-order choices; natural-language appearance invites interpretations beyond the grammar. Clauses need separate grammar rules for every family. | A closed clause grammar with no natural-language fallback; every clause maps to A's typed slots before signing. |

Recommendation A follows the existing TypeScript-style research constraint and current brace-based lexer. This is an engineering choice, not a source-proven usability ranking. Retain B and C as specified-only alternatives in a controlled developer comprehension comparison. Do not ship three equal authoring dialects initially: each multiplies documentation, parser recovery, source maps and signing-review cases.

## Exact transfer lowering and its unresolved bindings

**Recommended conceptual lowering**, not an implemented function:

```text
parse beta document
  → closed Payment AST with source origins
  → typed SignedTerms + typed TransferLiteralFee descriptor
  → supported-case Core5Stage(pre, action, signedScope, submittedEffects, premises)
  → stage / intent / effect / authority / history / failure
  → Rejected or PreparedUnqualified
  → future proof and atomic ledger-consumption interfaces
```

The existing TypeScript module represents the conceptual action/scope largely within `S0TransferIntent`; it does **not** export a constructor named `Core5Stage`. The contract uses that name as a proposed semantic term. An implementation must target the actual callable/types seam, rather than fabricate a constructor from this diagram.

| Beta term | S0 contract mapping | Status and obligation |
| --- | --- | --- |
| `Payment`, `Preview`, `USD scale 2` | AgreementId, DomainId, AssetId(scale); Core intent `domain="Preview"`, `asset="USD"` | Domain/asset occupy local intent fields. Agreement and scale need the unverified binding repaired before correspondence or signing claims. USD is a nominal binding, never a ticker lookup. |
| `selected TransferLiteralFee …` | `programId="TransferLiteralFee"`, `sourceHash="payment-source"`, `policyDigest="payment-policy"` | Current `programId` denotes an action ID. Hash validity and selected-program authentication are external obligations. |
| `signer`, `key`, `nonce`, `pre_head` | `signer="Alice"`, `keyRef="alice-key"`, nonce, preHead | Derive replay identity from `(Preview,Alice,payment-1)` exactly as the Core codec specifies; signer ownership and verifier/key scope remain mandatory. |
| `rounds(domain: Preview,start:100,end:200)` | `notBefore="100"`, `notAfter="200"` | No clock read occurs. Domain must equal the agreement domain. Endpoints are inclusive; validate inversion and bounds. A later nonround clock requires a separate typed rule. |
| `atoms(asset: USD,value: n)` | Exact nominal integer text for amount or cap | Constructor verifies the resolved nominal asset, scale and range; it performs no floating-point or exchange conversion. S0 transmits integers, not formatted decimal currency. |
| `transfer(from: Alice, to: Bob, fee_to: Treasury, value:1000, fee:10)` | `kind="Transfer"`, signer/owner Alice, recipient Bob, feeRecipient Treasury, amount `"1000"`, fee `"10"` | Reject a distinct `from` signer, endpoint aliases, mismatched units, changed amount/fee and mismatched signed/submitted action. No solver-filled transfer fields in S0. |
| Gross cap, fee cap, net floor | `grossCap="1010"`, `feeCap="10"`, `netFloor="1000"` | Require value positive, fee ≤ feeCap, checked value+fee ≤ grossCap, value ≥ netFloor; gross debit remains visible even if a future account alias policy changes. |
| SuccessOnly and explicit empty components | TerminalSuccess, empty retainedEffects/retainedDuties and no unsupported source components | Permit only the S0 case. User-defined Pending, rejection-with-fee or recovery clauses require new rules and cannot be lowered to SuccessOnly. |
| Authenticated snapshot and candidate successor | `S0State`, current round, expected successor and external premises | Provider inputs are separate from authored terms. Local stipulation objects do not authenticate their fields. Preserve workRemaining/workSpent, predecessor/head and replay state. |
| Submitted plan's effect vector | Compare with internally derived S0Effects | A solver or caller cannot define the economic effect by supplying a list or hash. Preserve order and compare the whole vector. |

With a sufficient correctly bound pre-state, the payment proposes the exact vector below. `nextHead` is a checked successor input, not a source-generated proof. No preparation was executed for this illustrative fixture.

```text
Debit(Alice, USD, 1010)
Credit(Bob, USD, 1000)
Credit(Treasury, USD, 10)
UseAllowance(Alice, 1010)
UseReplay((Preview, Alice, payment-1))
AdvanceHead(head-1, nextHead)
```

At zero fee the Treasury credit is absent by the existing rule; an explicit zero fee credit is invalid. Amount bounds, sufficient owner funds/allowance, receiver/counter overflow, replay, head and work checks remain independent obligations. Every failed judgment returns no published vector or post-state. A document that visually promises a signature is not a signature; beta signing requires a versioned canonical codec over typed resolved terms, signed policy, all fixed/completed values and required claim roots. Its bytes and scope remain open.

## Construct decisions for a coherent full-language tour

All examples in this section are **specified only** unless an S0 observation is explicitly stated. Syntax familiarity is borrowed; financial meaning and lowering are Moriarty recommendations.

| Area | Recommended spelling and rule | Borrowed mechanic / Moriarty obligation |
| --- | --- | --- |
| Declaration delimiters | `const fee: Quantity<USD> = atoms(asset: USD, value: 10);` and `function gross(v: Quantity<USD>, f: Quantity<USD>): Quantity<USD> { return v + f; }` | TS-style suffix annotations and braces (SURF-F01). Explicit public signatures, pure function body, checked arithmetic, no recursion, no host callbacks. Function examples do not imply S0 support. |
| `let` versus `const` | Prefer only immutable `const` in beta; local type inference may be allowed when unit/domain resolves uniquely. | TS examples offer multiple declaration forms; choosing one immutable binding form is our recommendation. Source/5's historical `let` does not settle beta mutability. Adding both immutable `let` and `const` supplies little value and creates a familiar-language trap. |
| Nominal types | `Quantity<USD>`, `Round<Preview>`, `AgreementId`, `EpisodeId`, `StageId`, `ObligationId`; `type` variant definitions create nominal types. | Adapt Unison's distinction (SURF-F08). TS `type USD = UInt128` alone cannot supply asset identity (SURF-F02). Bind catalog identity and scale separately from display symbol. |
| Records | `record Quote { value: Price<USD, BTC>, observed_at: Round<Preview>, status: QuoteStatus, }` and `Quote { value: q, observed_at: r, status: Final, }` | Borrow named property presentation, add closed exact fields and nominal financial wrappers. Reject unknown/missing/duplicate labels, computed keys and extension at construction. Preserve authored order for diagnostics; encode schema order for commitments. |
| Variants | `type QuoteStatus = Missing | Final { evidence: PriceEvidence } | Disputed { dispute: DisputeId };` | Adapt constructor-based sums (SURF-F08). Tags are closed, payloads typed and bounded; add versions deliberately. An evidence payload must have authenticated provenance, not merely a tag named Final. |
| Pattern matching | `match quote.status { Missing => reject MissingPrice; Final { evidence: e } => use e; Disputed { dispute: d } => reject DisputedPrice; }` | Component extraction resembles Python (SURF-F04), with our exhaustive closed matching requirement. No unmatched fallthrough, implicit success or dynamic class hooks. This is a branch-shape fragment; `use` here denotes a proposed pure evidence consumer, not an implemented kernel operation. |
| Named arguments | `transfer(from: Alice, to: Bob, fee_to: Treasury, value: v, fee: f)` | Adapt Python's named/keyword-only idea (SURF-F05), choosing TS-like `:` for one record/call grammar. Resolve each label to exactly one declared slot; reject repeated/unknown/missing arguments. Financial calls require labels; simple pure unary helpers may use positional arguments. |
| Separators | Commas in records/argument lists/variant fields; permit one trailing comma. Semicolons end declarations/statements; no automatic semicolon insertion. | The TypeScript chapter allows alternate object separators (SURF-F02); choosing one separator per role is ours. Trailing-comma policy is a proposed grammar decision, not inferred support in S0. |
| Comments | Existing `//` line and nonnesting `/* … */` block comments; optional `///` documentation treated as line-comment trivia. | Preserve the inspected lexer contract. Comments carry no authority, oracle provenance or accepted status. Unterminated blocks reject. All source bytes still count toward bounds. |
| State | `pre.balances[Alice]`, a proposed `next` write section, then `post` assertions after completion | Preserve immutable pre-state, derived complete reads/writes and single-stage atomic effects. Bind cell owner/asset/domain. No ambient mutable objects, post reads before derivation, duplicate write or guard after a final-state assertion. S0 authenticates input cells rather than evaluating these expressions. |
| Outcomes | `Success { … }`, `Pending { duties: …, continuation: … }`, `Rejected { judgment: …, code: … }` | Closed variants expose retained duties. Pending is a separate lifecycle state; no abbreviation turns it into terminal success. Only existing S0 Rejected/PreparedUnqualified output has local implementation evidence. |
| Completion holes | Proposed `hole route: Route<…>` inside signed completion constraints | A dedicated typed hole avoids conflating absent, None, wildcard pattern and solver choice. Signed fixed facts cannot become holes. S0 exact transfer/repayment fields cannot use this sugar. |
| Libraries and profiles | Versioned `import` bindings with immutable artifact references, qualified names, explicit profile header | Borrow readable namespace references, retain complete dependency closure and semantic version. No runtime package fetch or mutable alias resolution during preparation. Unison content references inform identities, not authorization. |

The beta profile header above is a proposed namespace, not an allocated release/version. Parser authoring must assign a real version and reject unsupported feature profiles. Labels within known closed records can be contextual words; top-level grammar words should have a short declared reserved-word set. Do not allow arbitrary English phrases to become fields. Duplicate labels reject before normalization rather than silently selecting first/last.

## Quantities, clocks and operator sugar

**Recommendation:** start with explicit exact constructors. `atoms(asset: USD, value: 1000)` is clear about smallest-unit amounts and introduces little lexer work. Expose `Quantity<USD>` and `scale=2` in editor hovers and signing previews. Never map amounts to JS `number`, silently rescale two assets, or infer a currency identity from the string `"USD"`.

Competing convenience forms are `10.00 USD`, `USD"10.00"`, and `amount(asset: USD, decimal: "10.00")`. The first adds decimal/name lexical ambiguity and looks like implicit multiplication; the second adds a dedicated literal token; the third uses the existing JSON-string token. If decimal convenience is admitted later, prefer the third initially, with a fixed locale-independent grammar, exact integer scaling, no exponent/NaN/Infinity, no separators at first, and a rejection for nonzero fractional digits beyond the bound scale. A literal helper cannot invent rounding; rounding belongs in a named economic operation. Show the normalized atoms and scale before signing. No new decimal spelling is a valid Source/6 integer token today.

Time must use `Round<Domain>`, `Timestamp<ClockId>` and `Duration<Unit>` as different types. The round window helper above is finite and explicit. A later `timestamp(clock: UnixUTC, value: "2026-09-30T12:00:00Z")` would need a pinned parser, precision/range and time-scale specification; wall-clock display is not a ledger observation. `duration(unit: Seconds, value: 60)` cannot be added to a ledger round without an explicit certified conversion. `deadline + 1 day`, `now`, `today`, implicit local time zones and month-to-second conversion are forbidden until domain clock and calendar policy are explicit. Elm's distinction between absolute time and display zones motivates clarity (SURF-F10); it does not establish blockchain finality or freshness semantics. ACTUS calendar conventions remain instrument policy, not an unqualified `day` suffix.

| Operator | Recommended meaning | Dangerous competing sugar |
| --- | --- | --- |
| `+`, `-` | Checked same-unit arithmetic with a stated result type/range; Quantity subtraction can fail on underflow, while Delta subtraction follows its declared signed range. | Wraparound, JS string/number coercion, saturating caps, implicit signedness or hidden beneficiary change. |
| `*` | Only statically defined scalar×quantity or certified dimensional operations; reject unsupported dimensions and intermediate overflow. | Multiplying two token amounts as if the output were the same asset or silently taking an exchange rate. |
| `/` | Defer generic division; use a typed named conversion such as `convert(input: q, price: p, rounding: Floor, remainder_to: beneficiary)` in the appropriate certified profile. | Automatic truncation, implicit rate direction, ambiguous rounding, or fees rounded away. S0 permits no division. |
| `==`, `!=`, `<`, `<=`, `>`, `>=` | Nominal same-type comparison; fixed precedence specified in grammar. Reject mixed assets/clocks. | Cross-domain time ordering or comparing display symbols as asset identity. |
| Boolean operators | Retain one declared spelling (`not`, `and`, `or`) consistent with the existing lexer; short-circuit only pure predicates under an explicit rule. | Overloaded truthiness or skipping mandatory authentication, authority, effect and history judgments. |
| `..` | Closed integer-round interval only in an explicit window context. | Creating a lazily unbounded iterable or guessing inclusive/exclusive endpoints. |
| Field access | Typed read projection; qualified immutable module/identity names are resolved separately. | Optional chaining turning stale/missing price into absent data that bypasses a financial constraint. |

No user-defined operators, chained comparisons, compound assignments, implicit multiplication or postfix increment initially. Defer pipe/compose and comprehensions until their exact evaluation order and work charging are specified. A finite collection traversal can be admitted only with static maximum length, per-element work and output bounds; saying `for` or `map` does not supply that proof. User recursion, `while`, infinite subscriptions and ambient IO remain outside the finite execution profile.

## Failure propagation and forbidden abbreviations

**Recommendation:** make failure explicit in the first profile. A pure helper can return `Result<T, E>`, handled by an exhaustive match. Financial-stage rejection remains a protocol result produced by the ordered judgments, not a general exception handler. A match cannot catch an authority error and continue with a smaller unauthorized payment. In S0 there is no fee-bearing accepted failure or recovery fallback.

A later `value?` shorthand would mean a single evaluation followed by `match value { Ok { value: x } => x; Err { error: e } => propagate e; }` in one permitted pure result-returning context, preserving original error identity and source origin. It must not skip later required financial judgments, overwrite the first judgment failure, or publish earlier tentative effects. Because `?` could also denote an optional field or completion hole, defer propagation shorthand and use distinct explicit constructors until the grammar and diagnostic model settle. Recovery is an authorized episode transition with surviving duties and bounded work, rather than language-level catch-and-retry.

Do not admit the following sugar:

- `pay(x)` that chooses an unspecified recipient, fee, asset, clock, signature or policy; a financially meaningful default must be a visible signed fixed term.
- `try swap else repay`, automatic retry, netting/canceling ordered gross effects, best-effort batch acceptance or `ignore error` in a signed relation.
- `price ?? 1`, `balance?.value ?? 0`, missing-cell creation, stale-observation fallback, or an `authenticated: true` flag that stands in for provider verification.
- `await bridge()` or synchronous-looking cross-domain calls that conceal a pending episode, foreign finality assumption or timeout duty. Timeout does not prove destination nonreceipt.
- Object/argument spread, field punning and computed labels in signed terms or financial effects. They obscure completeness, duplicate-field rejection, signing scope and origin attribution. A later pure record shorthand must be separately typed and origin-preserving.
- `as AssetId`, postfix nonnull assertions, unchecked casts, `any`, reflection, eval, arbitrary host callbacks, dynamic import or overload dispatch chosen from runtime strings.
- Hidden forgiveness of residual debt, fee rebates that erase gross debit limits, auto-renewed work allowance, signer inference from a connected wallet, or signatures inferred from a successful local preparation.

Elm's command values offer a useful presentation idea (SURF-F09): show typed requested effects and separate runtime consumption. Moriarty must add signed constraints, proof obligations and atomic consumption; ordinary command-returning code does not establish these. Unison's cacheable pure watches suggest an optional offline preview, but a cached preview must never be presented as current authenticated balances, current signature validity or ledger acceptance.

## Desugaring, source origins and resource obligations

Elaboration should have one explicit pipeline: lex → parse → resolve → type/shape check → normalize literal constants → lower supported operations → derive full footprint/effects. Keep provenance for every phase. A user sees concise terms; reviewers must be able to expand the same terms to fully typed Core requests with all signed fields and effect order visible.

Each generated Core node should carry (1) source file/content identity, (2) half-open UTF-8 byte span, (3) surface AST node identity, (4) lowering-rule/version identity, and (5) referenced declaration/parameter origin where the node has several contributors. A fee-overflow error points to amount and fee literals, the typed operator and policy bound; a missing signed field points to the enclosing intent, with the expected field name. A named argument's reordering must preserve its original span. Compiler-generated replay/head nodes point to nonce/domain/signer/head fields and the derivation rule, not a nonexistent source line. Evidence errors distinguish the authored required source from the supplied provider receipt.

Source origins are debugging metadata; they are not authority and need not be part of semantic Core hashes. Source text, build configuration, typed Core/dependency closure, signed intent, authenticated input commitment, proof claims and ledger receipt need separate identity domains. State exactly which artifacts a signature or proof binds. If formatting or local alpha-renaming preserves Core under its encoding, the source identity may still change. Names that denote parties/assets/domains are semantically material, so do not alpha-rename those away.

Keep the current S0 ceilings as the initial supported-case ceilings unless separately amended: 65,536 UTF-8 source bytes, 8,192 tokens including EOF, 8,192 AST nodes, nesting depth 64, identifier length 64 ASCII characters, decoded strings 1,024 UTF-8 bytes, one stage/effect vector, scale 0–18, nominal quantities 0 through `2^127−1`, balance/allowance/work counters through `2^128−1`, and checked UInt128 intermediates. For the proposed richer profile, those parser limits are insufficient by themselves: also bound elaborated Core nodes, literal mantissa digits, collection cardinality, pattern depth, imported closure size, function inlining/instantiation, stage count, episode lifetime, predecessor fan-in and total continuation work. Shared pure definitions should resolve once, with explicit cost rules; generics cannot create an unbounded expansion through specialization.

For each sugar rule, state an expansion-size bound. Record labels reorder to a fixed schema without combinatorial search. An exact literal is normalized with bounded digit work. Pattern matching examines a bounded closed constructor set; guards are pure and separately bounded. A collection helper requires maximum input and output lengths. Effect derivation charges every emitted node, including fee, replay, allowance and head effects. Splitting a stage or resuming an episode must partition the existing work allocation rather than replenish it. No shorthand can move a check out of proof scope merely because the source looks declarative.

## Lexer, editor and AI consequences

Braces and explicit separators allow error recovery at a known delimiter and make partial field completion predictable. Layout syntax instead needs indentation state across edits and a defined treatment of blank lines, comments and parenthesized continuation. This is an implementation tradeoff, not a benchmark result. Choose one formatter that preserves the parsed AST, nominal binding references and signed semantics; its formatting rules are specified only here.

Reuse fatal UTF-8 decoding, Unicode-scalar validation, ASCII identifiers, JSON strings, nonnesting comments and byte-based spans from the inspected lexer. Keep non-ASCII text inside strings/comments available for descriptions. Reject invisible or confusable characters in identifiers rather than normalizing them silently. Reserve `pre`/`post` structurally in the new grammar if they become state projections; they are currently identifiers. Existing `>=` splitting at generic type boundaries and `..` interval formation need explicit preservation/replacement rules. New `[]`, `=>`, `|` and named-call `:` require lexer/grammar changes; their appearance in a mockup is not present parser support. Do not add decimal tokens before fixing maximal munch and separator/interval ambiguity.

Editor tooling should show the nominal asset/domain and normalized atoms on hover; complete only fields of the current closed record and actions permitted by the selected profile; distinguish fixed terms, holes and provider inputs; render explicit status for locally prepared versus specified-only operations; expose a readable desugared view. Signing previews must display the exact resolved fixed terms, caps, recipients, fee beneficiary, time bounds, policy/program identities, holes and binding commitments. A pretty AST display cannot certify signed-byte fidelity while the codec remains open.

AI authoring benefits only by hypothesis from closed labeled records, clear diagnostic spans and a small grammar. No model was tested on these alternatives. Treat generated `.mori` exactly like hand-authored source: parse, resolve, type check and prepare within limits. Reject unknown capabilities instead of interpreting them from prose comments. Never let an assistant infer a signer, asset identity, proof availability, rounding beneficiary or recovery policy from natural-language intent alone. Structured diagnostics should offer a safe edit that preserves the signed relation; a suggested fix that increases a cap or deletes a duty requires an explicitly amended signed intention.

## Smallest coherent next profile and review obligations

First implement a **new versioned surface** for exact signed transfer and AccrualFirst repayment, with A's record syntax, explicit empty unsupported components, nominal atom and round-window constructors, closed named financial calls, separately supplied authenticated inputs and complete S0 mapping. Keep the wider tour's observations, holes, records/variants, staged updates and eight-family actions visibly specified only until their typed Core/kernel seams exist. Pure helpers, general match, decimal convenience, propagation `?`, aliases, user libraries and loops can follow the supported financial cases; no broad feature is needed to demonstrate the first syntax consumer.

For real beta authoring, allow a bounded sequence of pure **literal constants** before the agreement. Resolve constants to typed values; no runtime expression callbacks or general function definitions are needed initially. In A, replacing the two repeated fee/amount literals with these bindings leaves the signed terms unchanged:

```mori
const payment_value: Quantity<USD> = atoms(asset: USD, value: 1000);
const payment_fee: Quantity<USD> = atoms(asset: USD, value: 10);
// Inside A's signed_action:
// value: payment_value, fee: payment_fee,
```

Here `USD` resolves to the agreement's settlement binding under a deliberately specified scope rule; its nominal identity is retained in both constants. A simpler parser may require constants inside the agreement after the settlement declaration instead. **Recommended implementation choice:** place constants there and resolve declarations in order, avoiding forward asset/type references. Constant aliases can reference earlier constants only, with an explicit expansion-node budget. The signing view expands them to their literal values and resolved asset identity.

The corresponding **repayment authoring target** uses the same record/call policy and separate fixture inputs. This is a complete signed-term fragment with no local execution claim:

```mori
profile "moriarty-beta-surface/0";
agreement Repayment {
  domain Preview;
  settlement USD scale 2;
  const payment_amount: Quantity<USD> = atoms(asset: USD, value: 300);
  selected RepayAccrualFirst source_hash "repay-source" digest "repay-policy";
  intent {
    signer: Alice, key: "alice-key",
    nonce: "repayment-1", pre_head: "head-1",
    valid: rounds(domain: Preview, start: 100, end: 200),
    gross_cap: atoms(asset: USD, value: 300),
    fee_cap: atoms(asset: USD, value: 0),
    net_floor: atoms(asset: USD, value: 0),
    failure: SuccessOnly,
    signed_action: repay(
      obligation: Loan1, payer: Alice, amount: payment_amount,
      conversion: Identity,
    ),
    observations: [], disclosures: [],
    retained_effects: [], retained_duties: [],
    delegation: None, recovery: None,
  }
}
```

`Loan1` is an obligation identifier, not an authored obligation-state record; its debtor, creditor, asset, principal, accrued and outstanding fields come from the separately supplied state. A beta checker may construct a nominal ID from a resolved identifier, while still refusing to treat that ID as evidence that an obligation exists. With principal 900, accrued 100, outstanding 1000 and payment 300, the stipulated S0 allocation is accrued debit 100, principal debit 200, then principal 700, accrued 0, outstanding 700, Outstanding. Debit Alice 300 and credit the bound creditor 300 accompany that obligation update, followed by allowance, replay and head effects. This is arithmetic derived from the inspected rule, not an executed fixture.

**Separate scenario fixtures:** keep account balances, allowance counters, current head/predecessor, current round, existing obligation cells, consumed replay keys, remaining/spent work and expected successor outside authored `.mori` signed terms. Use bounded JSON whose fields map directly to the current local `S0State` and `S0LocalStipulation` seam. Parse numeric text as exact decimal strings. Label the fixture `local-stipulation`; provide no `authenticated=true`, fake signatures or fake proof flags. A local fixture is suitable for deterministic simulation and error controls, with output explicitly `PreparedUnqualified` or `Rejected`. A future provider adapter must authenticate the same cells and binding premises before proof or ledger consumption. Supplying fixture state cannot change a fixed signed repayment creditor, denomination, nonce, program or bounds.

LSP and AI integration can initially serve this same small grammar: lexer-derived highlighting, field/constructor completion, type/unit hovers, unknown-label and source-span diagnostics, and a desugared/simulation preview. The language server must mark rich profile operations as unsupported or specified only, and it must never present an AI-generated example as a runnable fixture until the actual checker accepts it. Inert TypeScript/Python wrappers can invoke compiler commands on files or data; they must not execute user callbacks as financial semantics. A comprehensive eight-family mockup remains a separately annotated design artifact, with the S0 authoring consumer as the executable first profile.

The smallest distinguishing demonstration is a beta transfer and repayment through the actual public parser/preparer path, with field-by-field equivalence to corresponding Source/6 cases on a stated domain. Required negative cases include missing/duplicate labels, reordered named fields, mismatched units/domain, a changed signer or recipient, fee cap/gross cap violation, zero-fee effect mismatch, full obligation repayment conservation, wrong bound creditor, overflow, unsupported Pending/recovery, consumed replay, changed head, changed work counters and unverified bindings. These are **specified tests, not performed results**. Existing S0 tests are not evidence for new syntax.

PL audit should settle: one delimiter policy; whether immutable local bindings use `const`; nominal versus structural record/variant roles; public type annotation rules; named-call syntax; literal normalization; interval endpoint policy; profile/version allocation; exact supported constructor mapping; first-failure preservation; generated-node source origins; expansion/work ceilings; and signed identity domains. K/Quint review must check observation equivalence on the declared supported domain, complete financial footprint/effect derivation and work preservation. Authentication and ledger audits must independently establish the external premises. A source-form preference vote cannot establish any of those predicates.

**Open dispositions:** beta canonical signed bytes and source/Core/scale/agreement/predecessor binding remain blockers to signing or correspondence claims; S0 accepted-failure and lifecycle expansion remain outside the first profile; usability ranking remains unmeasured. These limits do not block a reviewable syntax proposal or the bounded local consumer demonstration.

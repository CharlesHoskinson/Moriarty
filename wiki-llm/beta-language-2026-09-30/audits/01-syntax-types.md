# Independent PL audit 01: syntax, elaboration and types

Date: 2026-09-30. Scope: frozen candidate v1, surface/type memos, programmer mockup requirements, and actual Source/6 and Core/5 modules. No peer audit was read; no new external research or production change was made.

Requested reviewer: GPT-6.1 Sol, medium effort. The parent dispatch selected that configuration; no returned provider identity is available in this context. This report does not assert a separately observed provider/model identity.

Candidate SHA-256: `6af4ca5356ed3fd58818b5717401d5b218b672b65769e342f708d4a4e0cd061a`. `sha256sum -c audits/candidate-v1.sha256` (from the recorded repository-relative path) passed. Baseline: `870f998b36ecda04622fa4274132e74902942d0b`.

Startup observation: read `AGENTS.md` and the checked-in `moriarty-dev:develop` skill, then ran guarded `status --json`. It reported historical financial dispatch blockers and no pending transactions. Those blockers do not block this read-only design audit.

## Verdict and substantive votes

**Conditional agreement; repair the high findings before freezing an implementable elaboration contract.** This is design review, not execution or formal acceptance.

- **Agree:** a standalone brace DSL, closed named calls/records, explicit source profile, and one shared analysis service are a coherent initial authoring architecture. Retaining Source/6 rather than introducing another financial acceptance engine is the correct integration choice.
- **Agree:** bare unsigned scalars distinct from nominal `Qty<A>`, exact decimal-to-atoms conversion, no rounding/division/coercion, checked intermediate arithmetic, and rejecting quantity multiplication give a small teachable expression profile. Add the boundary rules below.
- **Agree:** symbolic account/asset declarations must remain claims; domain-aware identity and explicit representation/scale are necessary. Display symbols must never establish identity.
- **Agree:** preserve the existing Base-per-Quote convention and explicitly record the conflicting memo. A reversal is a separate semantic migration.
- **Agree:** broader profiles can provide authoring support with `SpecifiedOnly` and named unsupported execution. Their grammar must not imply financial validation or authenticated capability construction.
- **Disagree:** the present identity-to-Source/6 and candidate-generation descriptions are precise enough to implement without semantic choices. Findings S1 and S3 explain consequential choices that remain.
- **Disagree:** unspecified finite counters and a generic field-to-source map establish bounded elaboration and origin preservation. Findings S5 and S6 require concrete contracts.

## Findings

### S1 — High: resolved economic identity and Source/6 identifier transport are underspecified

**Repository observation:** the example deliberately distinguishes the declaration name `Buyer` from account ID `"Owner"`, `USD` from asset ID `"A"`, and domain `Preview` from ledger ID `"Midnight"`. The design requires nominal identity but never states which of these reaches Source/6, scenario balance keys, replay keys or inspection. Source/6 accepts ASCII identifiers (with reserved words excluded), not arbitrary opaque account/asset/domain strings; `lowerSource6` builds the replay key from its domain and signer identifiers. Core IDs are also restricted. Agreement ID and selected builtin operation occupy different slots; the beta action selector must not overwrite the builtin selected program.

**Discriminator, specified-only:** rename `Buyer` to `Purchaser` while retaining `id: "Owner"`, and leave the scenario keyed by `Owner`. The request, replay tuple and Core observation must remain equivalent except source origins/display names. Conversely, change only `id` to `"OtherOwner"`; it must change the financial identity. Try IDs `"transfer"`, `"0x01"`, and `"account:alice"`; require a documented formation diagnostic rather than an accidental Source/6 failure or silent substitution of declaration names. Two account declarations in one domain with the same ID must not evade endpoint alias rejection.

**Repair recommendation:** freeze a resolved-ID table and lowering map for each identity category. For the initial executable slice, explicitly restrict economic IDs to the existing Source/6 representable alphabet and reserved-word policy, or define a lossless adapter with a stated comparison domain; the latter is a larger choice. Do not generate replacement financial IDs merely to fit the parser. Use declaration names only for binding/navigation. Define duplicate account/domain/asset treatment and reject economic aliases consistently; defer explicit alias syntax if unsupported. Preserve asset identity as domain plus economic ID plus representation, with scale consistency, and state how the one-asset Source/6 envelope carries its unqualified metadata. This does not close authentication/scale binding.

Evidence: `BETA-DESIGN.md:43–68,132–143`; `financial-agreement-source-v6-frontend.ts:12,166,285–298,324–350`; `mil4-s0-core-v5.ts:8,235`.

### S2 — High: UInt128 expression bounds differ from signed nominal field bounds

**Repository observation:** candidate arithmetic promises UInt128 intermediates and rejection of out-of-range atoms without defining the accepted `Qty` range. Source/6 action amounts, fees and signed caps/floor are bounded by `2^127−1`; balances/counters/effect debit amounts use UInt128. Core repeats those distinct ranges. Thus accepting a literal up to UInt128 does not mean it fits an executable monetary field.

**Discriminator, specified-only:** `atoms(asset: USD, value: 170141183460469231731687303715884105728)` equals `2^127`. Test it as a standalone constant, action value, cap, balance and computed intermediate. Also test an intermediate above `2^127−1` that later returns below it, and an intermediate above UInt128 that later decreases. Expected handling must distinguish permissible intermediates from prohibited overflow and narrowing.

**Repair recommendation:** state that expression scalar/quantity arithmetic uses UInt128 checked intermediates; specify whether final constant quantities also use UInt128. Apply explicit, named nominal range validation when lowering every S0 signed field, while keeping balance/counter ranges unchanged. Document unsigned underflow, legal zero quantities versus positive action values, and decimal scaling before range checks. Use BigInt/exact decimal text throughout; never a JS Number path. Preserve Core checks even after authoring validation.

Evidence: `BETA-DESIGN.md:102–113`; `financial-agreement-source-v6-frontend.ts:7–8,185–190,236–238,266–268`; `mil4-s0-core-v5.ts:230–255`.

### S3 — High: automatic candidate expansion can change first-failure behavior

**Inference from inspected code:** repayment effect generation depends on stipulated debt and can fail arithmetic before it produces a valid Source/6 vector. For example, an amount greater than outstanding makes a naive accrual-first expansion produce negative principal; the strict Source/6 parser cannot encode that integer. Core would instead return `effect/S0_EFFECT_RANGE`, or an earlier Intent failure if validity/caps are also wrong. A frontend arithmetic error, exception, or invented “safe” post-state would change the promised observation. Transfer recipient overflow must similarly reach the right boundary without being masked by construction.

**Discriminator, specified-only:** overpay an otherwise valid repayment; repeat with an expired intent and with stale head. Compare automatic generation against an explicit syntactically valid candidate submitted to the actual Source/6 wrapper. The baseline ordering is Intent before Effect before History, with no published effects. Test receiver UInt128 overflow plus expired validity.

**Repair recommendation:** freeze the automatic-generation comparison domain and failure contract. Either define a total proposal builder that emits a documented syntactically valid candidate even when it cannot be financially accepted, leaving Core to diagnose it, or report a distinct expansion/formation failure and narrow the promised first-failure equivalence domain explicitly. Do not call expansion failure a Core judgment, copy Core acceptance logic into the frontend, or publish partial candidate effects. Preserve hostile explicit candidate controls separately.

Evidence: `BETA-DESIGN.md:166–186`; `mil4-s0-core-v5.ts:250–285`; `mil4-s0-source-v6.ts:35–59`.

### S4 — Medium: quantity tokenization and whitespace-only formatting need one lexical contract

**Repository observation:** EBNF leaves `QUANTITY` and `qualified-name` undefined. Decimal bare scalars, separation of number and asset, and trivia placement inside quantities have no settled lexical rule. Preserving token text alone does not ensure formatting preserves meaning: removal of a newline following `//` can comment out the next declaration; joining number/identifier tokens can change quantity recognition.

**Discriminator, specified-only:** `10.00 USD`, `10.00/*unit*/USD`, `10.00\nUSD`, `10.00USD`, bare `10.00`, `01.00 USD`, `1__000 USD`, `1.0_0 USD`, and comments immediately before delimiters. Require formatter parse equivalence, exact ordered non-whitespace spelling/comment preservation and idempotence, including CRLF and astral characters in strings/comments.

**Repair recommendation:** define numeric regexes and separator placement, disallow bare decimals if scalars are integers, and parse a quantity as a numeric token plus an asset-reference token with an explicit separation policy. Store trivia individually with byte spans. The formatter must preserve line-comment termination and token separation; if input is malformed, return diagnostics or no formatting edit. Define operator associativity, absence of unary minus, case-sensitive builtins and qualified-name syntax. Test resolved typed AST equivalence, not just successful reparsing.

Evidence: `BETA-DESIGN.md:89–128`; Source/6 lexer currently discards comments, so its lexer cannot supply the new formatter's promised retained trivia (`financial-agreement-source-v6-frontend.ts:56–65`).

### S5 — Medium: bounded input is not a complete elaboration resource contract

**Inference:** a declaration DAG can duplicate referenced constants exponentially if materialized before evaluation. The candidate rejects this strategy but gives no work/output budget or failure result. Source limits also do not bound scenario JSON bytes/nesting or expanded output by themselves.

**Discriminator, specified-only:** a chain of constants where each references the preceding record twice; many repeated references to one large record; deeply nested JSON scenario; and source within all limits whose expanded Source/6 exceeds its token/byte limits. All must terminate with an explicit budget result, without exponential allocation or process failure.

**Repair recommendation:** memoize checked values by resolved declaration, retain DAG sharing, cap source/scenario parsing depth and bytes, count expression visits and generated nodes/bytes before materialization, and publish numeric limits and stable diagnostics. Validate generated output against Source/6's independent limits. No recovered/incomplete LSP tree may lower. State whether forward references are prohibited for every declaration category, not only constants, and reject cycles before evaluation.

Evidence: `BETA-DESIGN.md:115,124–128,166`.

### S6 — Medium: field origins must identify scenario and derived inputs, not only source text

**Repository observation/inference:** derived repayment principal/status comes from a scenario obligation plus the source amount; replay key comes from domain, signer and nonce; balances/head/counters originate in scenario claims. A single “source origin” span cannot faithfully explain those fields. Current Source/6 AST does not retain all token spans, so origin data must be maintained by the new layer rather than assumed inherited.

**Discriminator, specified-only:** inspect a partial repayment with amount supplied by a constant expression. Each emitted principal/accrued/outstanding/status field must identify its scenario cell and amount expression/declaration dependencies. An overflow diagnostic must point to the relevant scenario amount and source operation. Definitions reached through references should expose use and declaration locations. Generated builtin selection must be marked generated, not attached to an unrelated user string.

**Repair recommendation:** define origins as a bounded tagged collection: beta source span, scenario span/JSON pointer, and derived origin with dependency references plus rule identifier. Retain emitted Source/6 byte ranges and map wrapper errors back through that relation. Keep UTF-8 offsets distinct from UTF-16 LSP coordinates and version the map with both inputs. Cap origin graph size as part of elaboration bounds.

Evidence: `BETA-DESIGN.md:118–120,166–178,249–254`; `financial-agreement-source-v6-frontend.ts:119–143`.

## Abstentions and evidence limits

I abstain on native target correspondence, cryptographic binding, ledger atomicity, editor activation and AI protocol security beyond the described boundary: this review did not reproduce those predicates. I do not count generic-record authoring of the eight families as complete financial typing. No beta implementation exists in the inspected frozen design, so every discriminator above is specified-only, not a passing test. Existing Source/6/Core/5 behavior cited above is repository observation from code inspection, not a new experiment.

The mockup requirements still require complete eight-family examples, visible per-construct status and field-by-field transfer/repayment comparisons; a family horizon table alone does not satisfy them. I support completing those artifacts alongside the executable slice, with unsupported semantics kept explicit.

## Amendment vote: CONVERGENCE.md v2

2026-09-30. Reviewed only the convergence amendment against my own report, original candidate and already inspected Source/6/Core/5 code. Parent confirms explicit host routing `gpt-6.1-sol`, medium; no separate provider identity/effort receipt is available. Original candidate digest and audit remain unchanged. These are design votes, not reproduced implementation results.

- **Economic ID mapping: concur with the choice; reject its stated Source/6 regex until corrected.** Emitting economic IDs, preserving display/binding names separately, rejecting duplicate identities and deferring aliases resolves S1. However, `[A-Za-z][A-Za-z0-9._-]{0,63}` is the Core/5 regex, not the unchanged Source/6 lexer alphabet. Source/6 reads `[A-Za-z][A-Za-z0-9_]{0,63}`, excluding its reserved words; dots/hyphens cannot appear in its emitted identifier tokens. Use that narrower alphabet for executable transport, including agreement names. Discriminator: `id: "Owner-1"` and `id: "Owner.1"` must reject explicitly before emitting unchanged Source/6; `"Owner_1"` is representable. No generated financial-ID replacement or parser widening is implied. This single correction is required for my implementation-contract concurrence.
- **Numeric widths: concur.** UInt128 expression values and checked intermediates plus explicit S127 field narrowing resolve S2. Syntactically expressible zero amounts and delayed cap/validity/state acceptance preserve the existing Core boundary. The distinction must remain in diagnostics and tests.
- **Total proposal builder: concur.** The documented unchanged-debt placeholder on overpayment produces a syntactically representable candidate without claiming a debt transition. The inspected Core checks Intent before the overpayment Effect range and before vector comparison, so this rule addresses S3 on the stated comparison domain. Require overpayment-alone and expired-intent-plus-overpayment probes against the real wrapper. Do not expose the placeholder as prepared output.
- **Lexical/formatter rules: concur.** Separate numeric/asset tokens, no bare decimal scalar, explicit separator/leading-zero rules, left associativity, no unary minus, prior references and invalid-source formatting rejection resolve S4. Preserving a line comment's terminating newline remains part of comment-preserving semantic tests.
- **Budgets: concur.** Explicit expression-edge, scenario, generated-byte/token and origin limits with memoized declaration DAG values and no partial result resolve S5 at design level. Implement pre-allocation accounting, not post-allocation-only checking. Limit failures are formation/resource outcomes rather than Core financial judgments.
- **Origins: concur.** Input digests, tagged source/scenario/derived origins, JSON pointers, byte spans and one UTF16 boundary helper resolve S6 at design level. Derived dependencies must retain both scenario cells and source expressions; generated rule labels cannot imply authentication.

**Result:** conditional concurrence on the amendment; all six remedy choices receive my substantive agreement once the Source/6 ID alphabet is corrected. No re-review is needed for that exact mechanical narrowing. Other choices or production behavior remain subject to actual checks and independent result review.

**Final concurrence:** inspected the corrected `CONVERGENCE.md:43–45`, which now requires `[A-Za-z][A-Za-z0-9_]{0,63}` with Source/6 reserved-word exclusion. The condition above is satisfied. I concur substantively with all six amended choices (economic ID mapping, numeric widths, total proposal builder, lexical/formatter rules, budgets and origins). This approves the design within its stated comparison domain, not future implementation or financial/native acceptance.

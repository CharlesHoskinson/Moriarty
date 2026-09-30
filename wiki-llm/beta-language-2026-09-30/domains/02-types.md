---
title: Moriarty beta types and financial resources
status: research-proposal
created: 2026-09-30
updated: 2026-09-30
domain: 02
evidence: ../types-sources.json
---

# Types and financial resources

**Recommendation BT2-R01, S2; specified-only:** select nominal financial identities, asset-indexed quantities, immutable data records, closed outcome variants, and a bounded pure expression language. Implement an authoring subset that elaborates exactly to the current S0 transfer and funded AccrualFirst repayment. Keep rights, duties, observations and asynchronous outcomes explicit in the wider design, with unsupported execution rejected. General borrow checking, user effect handlers and programmable resource destruction are later decisions.

This is a domain proposal for the requested five independent PL reviews, not a grammar freeze or implementation approval. New syntax below is **specified-only**. The Source/6 parser and local Core/5 preparer are the **implemented locally** baseline; their successful result is `PreparedUnqualified`. Native proof, authenticated providers, exact signature verification and ledger admission are **open**. Source facts, repository observations and recommendations have separate labels below. The [source manifest](../types-sources.json) records exact paths, hashes, receipts and locators.

## What the evidence supports

| Claim | Kind and evidence | Design implication |
| --- | --- | --- |
| BT2-F01 | **Source fact:** Rust tuple structs distinguish equal-shaped data; newtypes protect units, while aliases merely rename an existing type. [Rust Book, tuple structs and advanced types](https://doc.rust-lang.org/book/print.html#creating-different-types-with-tuple-structs); manifest `rust-book`, lines 6655–6710 and 40092–40202. | Identity and units need real type distinctions, rather than aliases over strings or integers. |
| BT2-F02 | **Source fact:** Rust ownership moves non-`Copy` values and drops an owned value when its scope ends; references borrow without taking ownership. [Rust Book, ownership and borrowing](https://doc.rust-lang.org/book/print.html#ownership-rules); manifest `rust-book`, lines 4662–4669, 5150–5208, 5320–5380. | Borrowing describes access to data. Rust-style automatic dropping alone would permit loss of an outstanding duty. |
| BT2-F03 | **Source fact:** Rust exposes `Option` and `Result` as enum variants; `?` returns an error early and can convert its type. [Rust Book, errors](https://doc.rust-lang.org/book/print.html#the--operator-shortcut); manifest `rust-book`, lines 8250–8330, 12987–13030, 13544–13650. | Model absence and errors explicitly. Restrict propagation so it cannot rewrite a financial rejection or discard committed partial effects. |
| BT2-F04 | **Source fact:** Move separately gates copying, dropping, storage and global-storage keys. Aggregate copying/dropping requires corresponding abilities of contained values; references have copy/drop but cannot reside in global storage. [Move Book, abilities](https://move-language.github.io/move/abilities.html); manifest `move-abilities`, sections “The Four Abilities,” “Builtin Types,” “Annotating Structs” and “Conditional Abilities.” | Resource uniqueness and permitted disposal are separate properties. A container must not launder a non-droppable duty into a disposable record. |
| BT2-F05 | **Source fact, reused September 9 capture:** Unison distinguishes unique domain types from structural types; equal-shaped structural declarations can denote the same type. Its abilities describe required operations, and handlers may ignore or invoke a continuation repeatedly. [Unique and structural types](https://www.unison-lang.org/docs/fundamentals/data-types/unique-and-structural-types/), [abilities](https://www.unison-lang.org/docs/language-reference/abilities/); manifest `unison-types`, `unison-abilities`. | Structural equality is useful for ordinary data but dangerous for domain identities. Unrestricted handlers need additional rules before they can manipulate financial effects. |
| BT2-F06 | **Source fact, reused September 9 capture:** Elm programs return commands and subscriptions for the runtime to interpret; time arrives through the effect interface. [Elm effects](https://guide.elm-lang.org/effects/); manifest `elm-effects`, `elm-time`. | Pure computation can build a typed request whose external evidence is supplied separately. A clock read is an authenticated input to a stage, not host ambient time. |
| BT2-F07 | **Source fact, reused pinned September 19 sources:** SimplicityHL distinguishes type aliases from new types and provides option/sum values and bounded lists. Manifest `simplicity-types`, `simplicity-aliases`, pin `aa20f1318db9c6c4ea5afb20ca4102ac42284ede`. | A bounded core can represent sums/products without a general heap, but readability aliases do not protect economic identity. |
| BT2-F08 | **Source fact, reused September 19 capture:** Daml ledger validity includes consistency, conformance and authorization. Consumption prevents later use of the same contract identity; a required missing transfer consequence violates conformance. [Daml ledger model](https://docs.canton.network/overview/reference/ledger-model-detailed); manifest `daml-ledger`, lines 368–412, 575–591, 1230–1259. | Static rights discipline cannot replace actual ledger consumption or completeness of the economic effect. |

**Scope limitation:** Rust and Move are selected for relevant type/resource mechanisms. This memo makes no survey ranking, adoption, performance or usability claim for either language. Only relevant portions of the Rust print edition were inspected; capturing the edition is not reading the whole book. Move evidence is the captured official Move Book, not a claim about a particular deployed Sui/Aptos runtime. No new compiler, proof, language example or ledger experiment ran.

## Local baseline and gaps

**Repository observation BT2-O01, prototype S3:** the [S0 contract](../../../experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md) requires nominal identity, fixed asset scale, exact signed and submitted actions, authenticated cells, checked UInt128 arithmetic, ordered complete effects and atomic rejection. Nominal amounts are `0..2^127−1`; balances, allowance/work counters and intermediate arithmetic are `0..2^128−1`. Scale is `0..18`. S0 additionally narrows principal, accrued and outstanding to `2^127−1` and requires their accounting equality; W-D4 remains open. These bounds are not interchangeable.

**Repository observation BT2-O02, prototype S3:** the [frontend](../../../experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts) represents IDs and integer payloads with TypeScript strings. Its parser checks kinds, closed shape and ranges, but host types do not make those strings nominal or authenticated. `lowerSource6` uses the selected action ID as Core `programId` and does not carry scale, agreement identity or authenticated predecessor into a bound Core statement. The [wrapper](../../../experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts) explicitly returns four unverified bindings: agreement ID, selected program, asset scale and authenticated predecessor. A nominal authoring checker cannot close these binding gaps.

**Repository observation BT2-O03, prototype S3:** the [implementation contract](../../../experiments/moriarty-language/formal/mil4/s0-implementation-contract.md) orders judgments as `stage → intent → effect → authority → history → failure`. Source formation errors have no Core term. Core rejection reports the first judgment/code and diagnostic work with no published post-state/effects. Transfer omits the zero-fee credit; repayment must credit the bound creditor and preserve any unpaid obligation. S0 admits no pending outcome, division, scale conversion, rounding, reserve, mint or retained-effect failure.

## Three candidate designs

**Recommendations BT2-R02–R04, S2, specified-only:** these choices concern the author language; none changes ledger acceptance automatically.

| Design | Programmer surface | Strengths | Concrete costs and risks | Disposition |
| --- | --- | --- | --- | --- |
| A: records with validation | `{asset: "USD", amount: 10}`; general structural records and runtime checks | Small parser; familiar to JSON/TypeScript users | Equal-shaped agreement/domain/asset IDs can mix; symbols can impersonate identity; omitted fields and numbers carry ambiguous scale; authority and data look alike. Runtime checks still need all S0 obligations. | Retain records for pure data and wire adapters; reject as the financial type model. |
| B: nominal values and closed financial operations | `Quantity<USDC>`, `Round<Preview>`, named `transfer`/`repay`, closed errors and outcomes | Explains identity/unit errors early; each financial operation has a finite footprint and explicit Core mapping; supports the S0 path | Requires first-order declaration resolution, explicit types at interfaces, exact literal rules and a separate binding contract. General programs still need refinements and authentication. | **Recommended beta default.** No subtyping/coercion between financial identities. |
| C: user-programmable resources and effects | Generic resources with copy/drop/store rights, borrow syntax, algebraic effect handlers and typestate | Can encode reusable capabilities and persistent workflows | Requires ownership across branches, authority attenuation, continuation multiplicity, frame rules and proof of handler semantics. A replayable handler could duplicate requests; a handler that returns a default could erase rejection. | Preserve in the full-language design, but first implement sealed operations and explicit continuations. Do not require a general borrow checker for S0. |

## Proposed nominal and data types

**Recommendation BT2-R05, S2, specified-only:** built-in identity categories are `AgreementId`, `EpisodeId`, `StageId`, `ActionId`, `ProgramId`, `DomainId`, `AssetId`, `PoolId`, `ShareClassId`, `InstrumentId`, `ObligationId`, `AccountId`, `KeyRef`, `Nonce`, and `HeadRef`. Equal bytes in different categories never imply type equality. Runtime asset identity additionally names its domain and representation: a chain/address/native-asset identity or opaque canonical reference supplied by an adapter. A display symbol is metadata. Compile-time binding to an asset declaration checks that identity; a different local name referring to the same verified identity is not automatically a different economic asset. Reject duplicate/conflicting declarations or resolve an explicit alias before typing.

**Recommendation BT2-R05a, S2, specified-only:** a first-class `Domain` binds network/ledger identity and account representation, not just the word `Ethereum`, `Midnight` or `Preview`. The full design records canonical network identity, relevant genesis/version or epoch, settlement/finality rules and adapter identity. `Account<D>` carries a validated address/credential for domain D; a logical cross-domain party is a separate `PartyId` whose policy binds its per-domain accounts. Identical address bytes on two networks do not imply account equality or spending authority. `Asset<D, Representation, Scale>` separates domain-native ledger representation from the economic instrument/asset class it may represent. A bridge-wrapped representation is a distinct asset with an explicit conversion/evidence contract; sharing a symbol or underlying asset label never authorizes a cross-chain transfer or arithmetic coercion. S0's existing opaque domain/account/asset strings can carry this metadata only through a proposed adapter/elaboration report; metadata cannot close the unverified identity/scale bindings.

Use `Quantity<A>` for unsigned nominal asset quantities, `Balance<A>` for UInt128 state amounts, and `Delta<A>` for signed changes with an explicit range in the later numeric profile. A debt is a typed liability with debtor, creditor, asset, accounting components and discharge rule, rather than a negative balance. A `Price<Base,Quote>` quotes units of Quote per unit of Base; `Position<I>` and `Shares<C>` carry instrument/share-class identities. Quantity is ordinary copyable information; it is not a spendable right. Do not copy an `Encumbrance`, `SpendRight` or `Duty` merely because its descriptor contains a quantity.

Use `Round<D>` for ledger rounds, `Instant<ClockId>` for an external clock and `Duration<ClockId>` for a duration. `Window<T>` has matching endpoints and an inclusive/exclusive convention fixed by its profile. S0 maps validity to the existing inclusive `notBefore/notAfter` round checks. No implicit conversion from wall time to a chain round, or from absolute time to a relative duration. Elm's effectful time access motivates separation but supplies no authenticated-chain-clock guarantee.

Ordinary fixed records are structural data: `{account: AccountId, amount: Quantity<A>}`. Do not use width subtyping, arbitrary field dictionaries or implicit coercion at a signed/Core boundary. Finance entities are named nominal types even when represented internally by records. Public/library signatures, persistent state schemas, evidence interfaces and capability types require annotations; literals and local pure bindings may infer types. `None` and empty collections require an expected type or explicit annotation. No inferred asset, scale, clock or authority scope comes from a string's spelling.

### Syntax examples and status

**Specified-only proposal; illustrative surface, to reconcile with domain 01:**

```moriarty
domain Preview = domain_ref("midnight-preview");
asset USDC on Preview = asset_ref("asset-id-01") scale 6 symbol "USDC";
asset OtherUSD on Preview = asset_ref("asset-id-02") scale 6 symbol "USDC";

const payment: Quantity<USDC> = atoms(1_250_000, USDC);
const fee: Quantity<USDC> = atoms(10_000, USDC);
const gross: Quantity<USDC> = payment + fee;
const valid: Window<Round<Preview>> = rounds(100, 200, Preview);

const alice: Account<Preview> = account_ref("alice-preview", Preview);

// Full-language type forms; outside initial S0 execution.
record Quote<A, B> {
  price: Price<A, B>;
  observed_at: Instant<OracleClock>;
  source: ProgramId;
}
enum Fixing<T> { Missing; Present(T); Disputed; }
enum LoanStatus { Outstanding; Settled; }
```

`asset_ref`/`domain_ref` declare opaque identity claims; they do not validate a ledger asset. In the local beta, `atoms` accepts a bounded exact integer and the asset declaration fixes scale. A later `units("1.25", USDC)` convenience must parse decimal text exactly, reject excess precision and bind the same scale in signing and execution. No binary floating point or automatic decimal rounding. Generic records/enums in this illustration are part of the broader design; initial beta can expose their built-in equivalents without implementing arbitrary user generics.

## Expressions: bounded choices and defaults

**Recommendation BT2-R06, S2, specified-only:** distinguish pure data computation, mandatory predicates, and financial requests.

| Choice | Default and rule | S0 realization |
| --- | --- | --- |
| Literals | Exact integers; strings only for explicitly typed opaque references; booleans; `atoms(n, A)` and `rounds(lo, hi, D)` | Resolve declarations and literal units before generating the existing literal Source/6 fields. |
| Arithmetic | `+`, `-`, `min`, `max` on compatible dimensions; checked intermediate width and underflow; comparisons on equal types | Initial executable beta folds pure constants only. Existing Core derives repayment `min(n, accrued)` itself. Source expressions must not replace that derivation. |
| Predicates | `==`, `!=`, `<`, `<=`, `>`, `>=`, `and`, `or`, `not`, explicit parentheses; no assignment, ambient I/O or truthiness | Typecheck authoring predicates. Execution rejects any nonconstant predicate that lacks an exact admitted Core operation; it cannot silently discard `requires`/`ensures`. |
| Conditions | Total `if p then x else y` with equal branch types; later exhaustive `match` on closed variants | Fold constant conditions for S0. Branches cannot hide authority/effects; later resource usage must reconcile at joins. |
| Absence | `Option<T>` with `Some`/`None`; fixed vectors use a typed `empty` or expected type | Preserve S0's explicit `empty` observation/disclosure/retained-effect/duty fields and explicit `none` delegation/recovery. Missing syntax is a formation error. |
| Fallible helpers | `Result<T,E>` and exhaustive matching; later restricted `?` within pure fallible functions | Compiler authoring diagnostics are separate from Core rejection. Initial S0 has no programmable exception handlers. |
| Advanced numerics | Named certified operations declaring input/output dimensions, numeric range, rounding and beneficiary | Division, prices, conversions and nonidentity repayment conversion reject as unsupported in S0. Design remains visible in mockup examples. |

A proposed `a + b - b` checks each intermediate: a final value fitting the bound does not excuse an earlier UInt128 overflow. `Quantity<A> - Quantity<A>` is a fallible unsigned operation; a later delta operation has an explicitly signed result. The initial executable beta needs no arbitrary recursion, stateful loops, higher-order functions, implicit numeric promotions or general polymorphic overloading. Preserve S0 byte/token/node/depth limits for both parsing and elaboration output; macro expansion cannot evade them.

## Absence, status and result types

**Recommendation BT2-R07, S2, specified-only:** `Option<T>` answers whether ordinary data is present. A missing authenticated balance is not zero; missing obligation evidence is not evidence of no debt; `None` cannot mean “provider unavailable” and “authenticated nonmembership” simultaneously. Kernel interfaces should return closed variants such as `Found`, `AuthenticatedAbsent`, `Unavailable`, `Stale` and `Conflicting`, with the evidence required for each. Only a supported profile decides whether a variant can authorize a transition. Initial S0 requires every specified cell and rejects missing cells/premises.

Use separate state/outcome ADTs. `ObligationStatus` reports the accounting state, not whether a transaction failed. Later `Outstanding`, `Settled`, `Defaulted` and `Impaired` need explicit equations and rights for their respective transitions. An amount of zero alone does not authorize settled status. For S0, use only its existing Outstanding/Settled semantics and preserve unpaid principal/accrued after a partial repayment; the residual obligation is not erased by S0's empty retained-duty field.

```moriarty
// Full-language proposal, specified-only.
enum StageOutcome {
  AtomicRejected(Rejection);
  SuccessCommitted(CompleteEffects, PostHead);
  PendingCommitted(CompleteEffects, Duties, Continuation);
}

// Local preparation result; never coercible to StageOutcome success.
enum Preparation {
  SourceRejected(SourceError);
  CoreRejected(Rejection);
  PreparedUnqualified(Candidate, UnverifiedBindings);
}
```

`AtomicRejected` has no published financial effects/post-head. `PendingCommitted` records effects already committed plus residual duties and authorized continuation/recovery; returning a generic `Err` must not erase that accounting. A terminal-success constructor requires discharge evidence for applicable duties. Pending/committed outcomes remain outside the initial S0 preparer. A local candidate is ordinary data and does not grant a ledger right.

## Authority, rights, and duty discipline

**Recommendation BT2-R08, S2, specified-only:** a readable snapshot may be borrowed or copied; a verified authority object is sealed and can only be produced by the relevant kernel verification judgment. Never let a public constructor, deserialized record, string or Boolean manufacture `VerifiedSnapshot`, `VerifiedIntent` or `SpendRight`. Two returned verified values must bind the same version, selected artifact, domain, action, intent, snapshot/head and outcome where required.

The future authority interface carries issuer/signers, action/domain/resource scope, permitted recipients and amounts, validity, nonce/revocation reference, allowance/work budgets and exact delegation/recovery policy. Separate the signed policy descriptor from its consumable exercise. Attenuation intersects allowed actions/domains/recipients and narrows validity and limits; it cannot broaden rights. Combining required authorizers is conjunction of their constraints. Budget splitting requires an authorized partition with conservation and unique child identities; copying a numeric remaining budget or adding unrelated capabilities cannot create authority. Merely listing a static effect requirement does not prove consent.

Adopt a sealed resource discipline initially: resources cannot be copied, implicitly dropped, overwritten, or hidden inside a disposable container. A `Duty` can only be discharged by a verified fulfillment, transformed by an authorized transition, or persisted into the accepted successor state. Rights whose expiry allows disposal need an explicit checked operation; discard is never inferred from end of scope. Every live resource at a branch join must have compatible successor ownership and duty accounting. Use stage-scoped immutable read views rather than an exposed general lifetime calculus; views cannot be retained as authenticated live reads for a later stage. Renew authentication against its head.

These are **formal obligations**, not claims of static double-spend prevention across separately produced programs. Static checks can reject reuse of one local right; atomic ledger compare-and-consume still prevents conflicting accepted branches. A copied immutable proof/receipt is not a newly spendable right. Failure during uncommitted preparation leaves the pre-state resources unchanged. Later resource flows must specify rollback, committed effects and residual duties per phase before exposing early-return syntax.

## Error propagation and first failure

**Recommendation BT2-R09, S2, specified-only:** source parse/type/elaboration errors are authoring results. Core financial rejection preserves the first judgment in the existing six-judgment schedule, then the profile's chosen within-judgment precedence. W-D3 still controls the eventual stable diagnostic spelling and schedule. Do not sort all errors globally, catch one failed judgment and resume at a later judgment, or use an `or_else` default that validates only part of the signed relation. Pure `and`/`or` evaluates according to a fixed pure-expression semantics; it cannot call kernel verifiers or choose financial failure precedence.

Restrict future `?` to pure helpers returning an explicitly declared compatible error type. Preserve the original failure tag when wrapping diagnostics; implicit conversions cannot turn unavailable evidence into absence or `Rejected` into successful completion. Financial operations return a stage outcome with any retained accounting. Every required predicate must dominate acceptance: computing a false Boolean and discarding it cannot satisfy a `requires` clause. Diagnostic errors and resource usage may reveal information; an eventual privacy theorem must include that observation or record it as allowed leakage.

## Hostile examples and expected handling

**Specified-only test designs BT2-T01–T09; not reproduced:**

| Hostile input | Required result and enforcement boundary |
| --- | --- |
| `payment + atoms(1, OtherUSD)` where both assets display `USDC` | Authoring type error for different resolved identities. A forged adapter identity additionally rejects at authenticated Stage/binding checks. |
| `DomainId("x") == AgreementId("x")` | Type error; equal spelling never bridges identity categories. |
| `atoms(10, USDC)` interpreted by another component as ten whole units | Signature/Core/asset representation must bind scale and exact atoms. Local typechecking alone does not close the existing scale gap. |
| `let r2 = spend_right; spend(r2); spend(spend_right);` | Resource reuse error in a future resource profile; distinct proposals racing on the same identity still require ledger compare-and-consume. |
| `let pending = settle(...); return Success;` | Cannot discard a non-droppable pending duty/result; later outcome checking preserves effects and requires successor duty accounting. Unsupported in S0. |
| `None` used for missing receiver balance, missing debt, or unavailable oracle | Reject missing required cells or reject unsupported observation; never initialize a balance or erase a liability. Authenticated absence is a separate later evidence variant. |
| Receiver balance `2^128−1`, credit `1`; or an intermediate overflow followed by subtraction | Checked overflow rejects. Constant elaboration obeys intermediate bounds; state arithmetic rejects at Effect. |
| A forged submitted `credit(attacker, n)` or repayment with only `SetObligation` | Intent rejects changed signed endpoints; Effect rejects a missing/mismatched complete vector. Repayment always derives the bound creditor. |
| Bad intent scope together with allowance exhaustion; or an invalid Stage together with replay | Preserve Intent before Authority, Stage before History. Author error propagation cannot choose a later or more convenient rejection. |

Add positive controls using two asset declarations with the same resolved identity through a permitted explicit alias, zero-fee transfer with no fee credit line, legal exact-bound arithmetic, a nonempty unpaid obligation after partial repayment, and a fresh current head with a distinct nonce. Rejection of an unsupported/malformed envelope alone does not demonstrate deeper resource or signature soundness.

## S0 mapping and formal obligations

**Recommendation BT2-R10, S2, specified-only:** desugar the beta into Source/6, not a second financial evaluator. Preserve a field-level elaboration report and return the original unqualified candidate/status. Treat richer full-language syntax as authoring-only until it has an admitted constructor and semantics.

| Author construct | Existing S0/Core representation | Obligation and present limit |
| --- | --- | --- |
| Nominal agreement/domain/program/action declarations | `agreement`, `domain`, `selected` fields and typed stage wrapper | Resolve categories without conflation; binding all identifiers into the verified statement is open. Existing Core `programId` is an action ID. |
| `Quantity<A>` literal and asset declaration | Settlement asset/scale and amount strings | Exact canonical atoms; scale must survive verified lowering/signing. Current scale binding remains unqualified. |
| `Round<D>` window | `valid`, authenticated round | Inclusive bounds and exact domain; authenticated clock-to-head premise. |
| Transfer request | `TransferLiteralFee` | Full signed/submitted equivalence; gross amount, optional positive fee credit, allowance/replay/head order and existing distinct-endpoint policy. |
| Funded repayment | `RepayAccrualFirst` | Debtor/signer and bound creditor/asset; identity conversion; accrual-first equations; remaining debt/status; complete creditor credit. |
| Read views and intent claims | Existing authenticated block, signed scope, external premises | Author data is a claim; no constructor promotion to verified evidence. Whole snapshot/head and exact intent authentication remain external. |
| Explicit empty/none | Existing six empty/none source fields and success-only policy | No implicit defaults. Nonempty unsupported mechanisms reject at source formation, rather than being erased. |
| Error/outcome variants | SourceRejected/CoreRejected/PreparedUnqualified | Preserve absence of published post/effects on rejection and first failing judgment. No manufactured success/ledger admission. |

Required future evidence is: type preservation for resolved IDs/dimensions and resource joins; total bounded elaboration; semantic equivalence of accepted beta elaboration and S0 observations on the explicitly admitted domain; K first-failure and complete-effect correspondence; Quint mutation/branch/race witnesses including actual resource identity; exact signature/snapshot/head/scale/program binding; atomic ledger compare-and-consume; numeric basis correspondence with actual finite target operations; and native source-to-target/proof/effect evidence. Type safety alone establishes none of authenticity, economic conformance, provider truth, privacy, availability, liveness or deployment.

## Phased implementation proposal

**Recommendation BT2-R11, S2, specified-only:**

1. **Beta authoring foundation:** first-order nominal declarations, typed constants, closed records built into S0, explicit empties, asset/clock diagnostics, bounded constant expression evaluation and elaboration source maps. Emit only literal Source/6 transfer/repay documents; reject unsupported execution constructs. Keep broader eight-family examples labeled specified/open.
2. **Exact local S0 integration:** invoke the existing parser/preparer on emitted bytes and compare field by field, including caps/floor, endpoints, nonce, reads/counters, signed/submitted actions, exact effect order and empty policies. Verify contrasting transfer and partial-repayment positive cases and the meaningful hostile cases above. Retain `PreparedUnqualified` and all four unverified bindings.
3. **Binding qualification:** define versioned typed representations/canonical bytes and prove/check asset scale, agreement, selected program/action, source/policy digest, predecessor, snapshot and signature linkage without caller booleans. This requires the actual kernel/target path and independent evidence; it is not a parser task.
4. **Broader financial language:** expose certified arithmetic, typed evidence and closed effect requirements per family; add non-droppable rights/duties, persistent continuation/recovery and sealed resource transformations after branch/phase semantics exist. General resource/effect polymorphism and user handlers require a separate decision and proof obligations.

**Open choices BT2-Q01–Q05:** asset representation canonicalization and declaration aliasing; finite signed-delta profile; exact price/clock conversion and rounding contracts; resource split/merge and authorized disposal; privacy of diagnostics and whether pure `?` is worth adding before library errors exist. Recommended initial dispositions are opaque resolved identity plus explicit alias, no deltas/conversion in executable S0, no programmable resource split/disposal, preserved diagnostics with no privacy guarantee, and exhaustive matching before `?`. The broader language must still show these choices and their planned semantic contracts.

**Research completion boundary:** two fresh official captures were acquired through Scrapling; four existing language families were reused with their historical dates/pins. Robots observation allowed the current Rust Book path; Move's host returned 404 for robots and did not establish an access prohibition. No authentication, bypass, cookies, private egress or new site-specific infrastructure was used. The existing static-document pattern applies; no new cookie or site pattern was discovered. This domain recommendation is ready for review as a proposal, with its unperformed experiments and binding questions explicit.

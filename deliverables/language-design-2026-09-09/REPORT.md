# A TypeScript-style financial language informed by Elm and Unison

## Recommendation and scope

**Panel-recommended direction (S2).** Develop Moriarty as a small, separately specified financial DSL with a TypeScript-style surface, a pure bounded transition core, explicit observations and effects, and content-addressed semantic artifacts. Borrow Elm’s separation of state, input and requested effects, and Unison’s separation of readable names from immutable references. Retain text files, ordinary review diffs and explicit versioned compilation. General FRP streams, arbitrary effect handlers and an entire database-based development environment should remain outside the first language slice.

TypeScript-style syntax and Midnight Compact targeting are design constraints here. Their value does not depend on claiming that JavaScript punctuation is intrinsically easier for everyone. Syntax familiarity can reduce one learning burden while increasing another: a programmer who recognizes `next.balance = ...` may assume that it mutates state immediately, or that `number` and JavaScript coercions are available. Familiar forms need precise boundaries and actionable explanations.

**Repository observation.** At base commit `9157a29`, `moriarty-successor-syntax/0` has a bounded parser and formatter. Its README explicitly denies typing, evaluation, obligation conservation, K emission and financial admission. The older atomic profile supplies narrower executable examples. The current successor design already proposes `pre`, staged `next`, suffix `ensures`, nominal asset amounts, ordered effects and residual duties. This research supplements that direction; it does not freeze a successor profile or change accepted atomic behavior. [R1–R4]

The immediate authoring task remains a partial payment with surviving debt, followed through source, typed Core, K, evaluator and eventual Compact/ledger acceptance. ContractInvariant, IntentRefinement, TransitionValidity and HistoryCompliance remain mandatory. A favorable usability study, a content hash or a causal stream theorem cannot replace any of those predicates. Financial settlement and native recursive proof obligations remain open. [R2–R4]

## Evidence and interpretation

The paper atlas separates empirical studies, formal results, systems designs and practitioner documentation. These answer different questions. A controlled comparison can show task-specific differences for a sampled population. A formal theorem can establish a property of a specified calculus under assumptions. An implementation paper can demonstrate a mechanism. A language team’s guide can explain its current intended workflow. None alone identifies a universally best language.

The corpus covers syntax learning, types and undocumented APIs, error-message use, DSL comprehension, human-centered design, historical Elm FRP, temporal typing, effect typing and reproducible builds. The annotated atlas records the exact version, full-text coverage, methods, page or section locators and limitations for each work. Failed public acquisitions remain in the source inventory; inaccessible text does not support substantive claims. Official web observations were made on September 9, 2026 UTC. [See PAPER-ATLAS.md and SOURCE-INDEX.md.]

**Inference.** Accessibility should be evaluated as successful financial work: understanding a proposed transition, predicting the remaining liability, repairing a unit error, inspecting the complete effect list, and identifying why a signature or claim is insufficient. Short source length is an inadequate objective when omitted information determines who can lose assets. A readable program should expose the distinctions needed for a correct decision, while hiding incidental compiler and proof mechanics until requested.

The initial audience should include TypeScript developers with modest financial-domain familiarity, financial engineers with modest programming experience, and developers experienced in typed functional languages. Results must be reported separately by prior experience. The research contains no new developer experiment and supplies no estimated adoption rate.

## What the empirical studies support

The syntax study by Stefik and Siebert is useful because it challenges the assumption that popular punctuation must be intuitive. It combines two rating studies with short novice programming experiments. In the larger coding experiment, Ruby, Python and Quorum exceeded the artificial Randomo condition on token accuracy; Java and Perl did not show a statistically significant advantage over that condition. Most comparisons among actual languages were not significant. This is not evidence that Java is equivalent to random tokens, and the study did not test TypeScript developers or financial programming. Its small groups and constrained early-learning tasks support testing familiarity rather than asserting it. [^1]

PLIERS is particularly relevant because it combines human-centered design with language mechanisms intended to prevent important errors. Its Obsidian comparison analyzed twenty Java programmers. Auction correctness was seven of ten in Obsidian and two of ten in Solidity; the often-quoted significant comparison used only those claiming completion, while the all-participant comparison had p approximately 0.070. The harder Casino task exposed misuse of the `disown` escape hatch and did not yield a fully correct Obsidian solution. The practical lesson is to include difficult, open-ended tasks and measure whether developers work around the checker. It does not justify weakening ownership or assuming ownership annotations are self-explanatory. [^2]

Mayer and colleagues’ typed-versus-untyped API study supplies useful counterevidence to simple slogans. Its twenty-seven analyzed students worked with Java and Groovy versions of undocumented APIs. Static typing helped on three tasks, while dynamic typing helped on two; the aggregate analysis did not establish a simple overall effect. Annotations and checking were bundled, and the participants’ prior Java experience matters. For Moriarty, visible action signatures and asset types are a defensible starting point, while local inference and editor-displayed types remain a separate usability question. [^3]

Prather and colleagues distinguish understanding a compiler message from successfully repairing a program. Their message quizzes had fewer incorrect interpretations with enhanced messages, but the practical assessment did not demonstrate substantial overall learning improvement. The in-context and historical comparisons have confounders. Moriarty should therefore test the next edit, successful repair and recurrence of the error, not just whether participants say a message is clear. An apparently friendly suggestion that encourages suppressing a claim or discarding a duty is harmful by the financial correctness criterion. [^4]

Kosar and colleagues found better questionnaire performance for a GUI DSL than for a C# GUI library in their student sample. Their detailed table is more informative than the conclusion’s rough improvement summary: learning, perceiving and evolving tasks have different differences. This is a GUI comprehension/edit questionnaire, not a measurement of secure contract authoring, and its reported inferential analysis is not independently reanalyzed here. The transferable hypothesis is that domain notation can reduce translation effort. Any advantage for Moriarty must be measured on its own tasks, without importing the paper’s effect size. [^5]

The cognitive-dimensions questionnaire gives reviewers concrete questions about visibility, hidden dependencies, role expressiveness and the effort needed to make a change. Its eighteen-person exploratory study is not a validated language ranking. It also warns that notation and environment are hard to separate: a hash-based language with poor dependency navigation may be harder to understand than a conventional language with clear tooling. Use the vocabulary to identify friction in source-to-effect inspection and residual-duty tracking, not to generate an arbitrary score declaring Elm or Unison the winner. [^6]

The course-based user-centered design paper provides further qualitative caution. Participants’ invented syntax and preferences revealed assumptions, but the authors distinguish naturalness from learnability and practical effectiveness. Prior exposure can shape what people spontaneously write. For Moriarty, start by asking developers to express a financial condition in their own terms, then observe whether the actual constrained notation supports a correct edit. Preference alone is weak evidence; an independent PL panel is an expert design review, not a substitute for human participant evidence. [^7]

Taken together, these studies support an iterative method more strongly than a fixed bundle of language features. Retain the TypeScript-style constraint, expose financial types and effects, and test the chosen notation under equal semantic and tooling conditions. Preserve negative results and disagreement. The most important outcome is whether developers correctly understand what their program authorizes and what remains owed after an action.

## Surface syntax and embedding

**Recommendation.** Keep one canonical `.mori` source language using braces, named declarations, parenthesized parameters, `name: Type` annotations, familiar record fields and infix arithmetic. Specify the lexer, full grammar, static judgments and lowering separately. This leaves a compact authoring surface without making the TypeScript runtime the definition of financial execution. Compact itself documents JavaScript-like syntax alongside fixed-size types, bounded loops, no recursion and explicit private-data disclosure. Its runner-supplied witnesses are a distinct off-chain interface. [^24]

A direct TypeScript embedding and a TypeScript-style external language are different products. The former can mean a tagged template, a typed builder API or arbitrary callbacks. For the first slice, make the external language authoritative. If an embedding is required, an inert tagged-template form can carry the same source text with interpolation prohibited independently of TypeScript checking. Static extraction must not execute the enclosing module: runtime evaluation of a tag is insufficient because interpolation, imports or surrounding expressions can already execute. Specify raw versus cooked template characters, backtick/backslash escaping and source-offset mapping; commit the exact extracted bytes. Replacing the host tag binding must not confer trust: the checked source/Core artifact is authoritative. This is a nonstandalone explanatory fragment, not a currently supported API or the primary financial teaching example. The [complete proposed data fixture](PAYMENT-FIXTURE.md) supplies authority, custody, duties, policy, observations and expected outputs:

```ts
const payment = moriarty.source`
  action pay(paid: Amount<USD>) {
    requires paid > amount(0, USD);
    requires paid <= pre.outstanding;
    let remaining = pre.outstanding - paid;
    next.outstanding = remaining;
    emit Transfer {
      asset: usd, from: borrower, to: lender, amount: paid
    };
    ensures post.outstanding == remaining;
  }
`;
```

The enclosing agreement would still declare custody, parties, asset binding and quantum, financial allocation policy, authority, claims and exact bounds. This fragment alone does not show that the payment is authorized or funded. In the proposed complete operation, one funded principal-allocation rule derives both the transfer and surviving duty; the displayed subtraction explains that relation rather than requiring the author to state the same financial intent twice. It is not an ACTUS interest/principal allocation model. The object named `payment` is a source artifact, not a function allowed to transfer money from the host process.

A fluent TypeScript builder offers editor completion but can hide control flow in host-language functions. If later added, it should construct a closed, bounded AST using data constructors. It must not admit callbacks, implicit `await`, dynamically selected imports, `eval`, reflection, ambient clocks or host-number arithmetic into the trusted semantic input. A builder and the external syntax must elaborate to the same typed Core and diagnostic contract. Their implementation effort and the cost of supporting two authoring representations justify deferral until the canonical flow is usable.

Familiarity should be deliberate at each token. Existing `let`, `requires`, `ensures`, `and` and `not` carry established successor proposals; changing them to `const`, `assert`, `&&` or `!` requires a versioned grammar decision and matched comprehension tasks. In particular, the proposed `not a == b` precedence differs from a JavaScript reader’s expectation about `!a == b`. Prefer formatter-inserted parentheses and focused diagnostics initially. Do not silently change precedence under an unchanged profile identifier. [R1]

The update discipline is explicit: `pre` is immutable; each field admits at most one staged `next` write; `next` is never readable; unwritten fields persist; `post` is visible only in the final `ensures` suffix. Intermediate dependencies use named locals. Negative cases must cover each restriction, including a guard after `ensures` and failure after a staged effect.

The surface should avoid clever overloading. A transfer remains an explicit financial effect, division names its instrument-dependent rounding rule, and a query does not acquire authority through method-call syntax. The developer should be able to inspect what was read, what tentative state was written, what duties remain and which effects are proposed. A compact DSL should reduce irrelevant repetition rather than compress those distinctions away. Payer-favorable rounding is not inherently an error: reject deviations from the declared instrument policy, and show affected calculations and duties when that policy changes.

## Types, errors and financial vocabulary

**Recommendation.** Make public interfaces explicit and use local inference only where it leaves a unique, visible meaning. `Amount<USD>`, `Amount<EUR>`, `Shares<VaultX>`, a market valuation and a redemption entitlement should not become interchangeable because their runtime representations are integers. The existing DeFi studies distinguish accounting, redemption, market and stressed liquidation values, and distinguish preview calculations from execution authorization. Those distinctions should remain in types or named policies. [R1, R5]

Unison’s documentation gives a concrete structural-typing caution: structurally identical `Book` and `Author` declarations can be interchangeable. Its unique-type reference explains that a generated unique identifier participates in a type’s hash. Thus its beginner description of uniqueness “by name” is a pedagogical shorthand, not a rule that renaming a financial asset should change its identity. For Moriarty, bind nominal unit and resource identifiers explicitly; use structural records only where interchangeability is intended. [^23][^27]

Types prevent some category errors but do not alone establish financial conservation, correct rounding, oracle truth or authority. Rejecting `Amount<USD> + Shares<VaultX>` is useful. It does not prove that an admitted valuation is current, that the owner may redeem now, or that the underlying assets are available. Those remain separate static, dynamic, environmental or proof obligations.

A diagnostic should answer four practical questions: where the mismatch arose, what was expected, what was supplied, and which safe action follows. For example:

```text
UNIT_MISMATCH at payment.mori:8:22
Expected Amount<USD>; found Shares<VaultX>.
A share claim cannot be used as a USD payment.
Use the agreement's explicit redemption operation or supply USD.
No financial effect has been accepted.
```

The last line belongs only at a stage where it is true; a failed network submission may have incurred an external fee. Diagnostics must not suggest an unsafe cast, automatic oracle substitution, implicit share redemption or waiver of a mandatory claim. A source span plus the relevant prior declaration is more useful than exposing an entire internal type derivation by default. An expandable explanation can show the constraint path and exact profile.

The error vocabulary should distinguish parsing, typing, bounds, authority, observations, preparation, proof and ledger acceptance. Developers need to know whether a source is malformed, a proposal is well-formed but inadmissible, or a submitted transaction has unknown finality. These are product states with different next actions. A successful formatter and a valid signature must not share a generic “verified” badge.

## Elm: pure transitions without infinite financial streams

Elm’s current guide presents Model, View and Update, with commands and subscriptions describing interaction with the runtime. Its time example turns time observations into messages; `subscriptions : Model -> Sub Msg` describes active interests. The dated 2016 announcement explicitly replaced signals in Elm 0.17. These current patterns and the earlier FRP papers must be described separately. Ease-of-use claims in the announcement are the author’s observations, not controlled usability results. [^14][^15][^16][^17]

**Recommendation.** Borrow a pure transition boundary: an explicit input and immutable prior state produce either a rejection or a prepared result containing tentative state, ordered effects, residual duties, residual authority and remaining work. Inputs should represent authenticated observations or admitted actions, not arbitrary callbacks. Source evaluation should not open a socket, read the wall clock or silently submit a transaction.

```ts
// Proposed interface shape, not implemented language semantics.
type TransitionResult =
  | { tag: "Rejected"; diagnostic: Diagnostic }
  | {
      tag: "Prepared";
      state: NextState;
      effects: BoundedEffects;
      duties: BoundedDuties;
      authority: ResidualAuthority;
      work: RemainingWork;
    };
```

This adaptation makes simulation and explanation straightforward in principle: the same explicit inputs can be replayed, and the result can be inspected before signing or proving. It does not establish replay safety on a ledger. Nonces, predecessor consumption, expiry, ordering and finality remain acceptance concerns, and the execution statement must bind all relevant inputs and outputs.

Historical asynchronous FRP is valuable precisely because its assumptions reveal the mismatch. The Elm PLDI paper’s model distinguishes graph setup from later event processing; its normalization theorem concerns the first stage, and its asynchronous design permits unbounded FIFO queues. The thesis and PLDI paper share a research lineage rather than constituting independent replications. These works support discussion of reactive separation and scheduling tradeoffs, not a finite lifetime execution claim for a financial contract. [^8][^13]

Simply RaTT offers a stronger formal perspective on productivity, causality and space-leak avoidance through temporal typing. Those properties still differ from a fixed, admitted lifetime work budget. A stream that can keep producing outputs forever may satisfy productivity while violating Moriarty’s finite agreement horizon. A bounded transaction cannot regain allowance merely by returning a continuation. [^9]

**Recommendation.** Represent financial reactivity as a finite admitted event schedule or an explicit bounded sequence of externally proposed actions. State the horizon, observation count, queue capacity, duplicate policy, ordering rule, timeout rule and reserve for recovery. Unsupported overflow must reject or follow an explicitly specified policy; dropping a tick must never erase a payment obligation. A timestamp observation supplies data under a trust policy, not a promise that the ledger will execute at that instant. Under a versioned work model, consumed accepted work plus all distinct live successors’ remaining allowance cannot exceed the originally admitted allowance. Splits partition it; joins combine eligible residuals without renewal; the recovery reserve is included, not minted later. Compiler/proof resource bounds and witness availability remain separate obligations.

Current Elm’s browser runtime is useful inspiration for an off-chain developer interface: updates can render simulated state and status messages, and subscriptions can receive transaction-status notifications. Keep that interface outside the on-chain semantic boundary. The official ports guide is a useful reminder that interoperability crosses a boundary into JavaScript; a port-shaped interface does not make the foreign code formally verified. [^18]

## Effects and authority

Unison abilities describe required effects in types, with a subset-style availability check at calls. The official reference distinguishes explicit absence of effects from ability-polymorphic function types. It also permits handlers to discard a continuation or resume it multiple times. This is an expressive mechanism; importing it wholesale would expose execution multiplicity and resource consumption as additional semantic obligations. [^21]

Koka’s paper demonstrates row-polymorphic effect inference and formal connections between effect absence and runtime properties, including termination in its calculus when divergence is absent. The result is conditional on that formal system and its primitives. It does not establish that a custom financial capability object, a host witness or a new handler satisfies a resource budget. The paper itself describes implementation refinements separately from the proved core. [^11]

**Recommendation.** Start with closed effect schemas and conservative inference over a finite action body. Pure expressions have no financial effects. A compiler-derived conservative effect summary is checked against declared interface limits; authors need not repeat effect-row punctuation at each call, and the actual execution statement binds the ordered, concrete effect list. Do not equate the presence of `Transfer<USD>` in a type with the principal’s permission to transfer a particular amount to a particular recipient.

Authority must remain quantitative and contextual: permitted assets, cumulative gross debit, fees, net goals, recipients, liability changes, expiration, nonce and residual rules. Effects describe possible behavior; authority admits a bounded instance of behavior. An outcome intent can precede route selection while still constraining every concrete execution. Generic handlers must not mint new authority, duplicate a duty or hide an effect from the statement. Authority is visible in its declaration and prepared proposal, but this recommendation does not require an authority token on every `emit`. Effect-stage failures and authority-stage failures remain distinct. An allowed route under an outcome intent needs no compulsory second exact-plan signature; the original signed constraints still govern its complete effects.

For the initial language, defer user-defined resumable handlers, first-class continuations and effect polymorphism across dynamically chosen libraries. If a future abstraction needs handlers, require a separate calculus with single-shot or otherwise quantitatively accounted resumption, bounded operation count, complete effect observation and conservative lowering. Tail-call optimization removes stack growth; it does not prove bounded lifetime work.

## Content addressing and reproducible identity

Unison identifies definitions using hashes of their structure and dependency references; readable names are separate metadata. The hash reference document specifies full and abbreviated forms, built-in references and mutually recursive groups. An abbreviation is meaningful only relative to an environment that resolves it unambiguously. These mechanisms suggest immutable dependency identity, not permission to accept a short hash as a globally unique signed commitment. [^19][^20]

**Recommendation.** Distinguish at least four artifacts: exact source bytes, canonical typed Core, a build context and a claim/evidence context. The source digest identifies what was reviewed, including comments and spans. The Core digest identifies the precise elaborated program under a named canonicalization and semantic profile. A build context binds compiler/backend versions and configuration. A claim context binds the required predicates, bounds, verifier/key lineage and environment assumptions. A transaction then commits to the selected program and actual authorization, observations, predecessors and resulting effects.

A possible conceptual structure is:

```ts
// Proposed commitment fields; serialization is deliberately not frozen.
type ProgramIdentity = {
  sourceDigest: Digest;
  coreDigest: Digest;
  semanticProfile: ProfileRef;
  canonicalization: EncodingRef;
  dependencies: BoundedList<FullDefinitionRef>;
  nominalTypes: BoundedList<NominalTypeRef>;
};
```

The initial canonicalization recommendation normalizes local binders only where specified; public identifiers and nominal bindings retain their identity role until separately defined. A compiler change changes build identity, while Core identity changes only if actual elaboration changes.

The committed encoding needs domain separation, field framing, version tags, ordering rules, size limits and treatment of Unicode and identifiers. Different domains should not share an ambiguous hash input. A rename can preserve a definition identity only for names excluded by the specified canonicalization; changing an exported authorization name or a nominal asset binding may be semantically material. Hash canonicalization is a specification task, not a generic whitespace-removal pass.

Nix’s functional deployment model addresses dependency identification and isolation, with the important distinction between declared build inputs and the behavior of arbitrary build processes. Build Systems à la Carte supplies a vocabulary for scheduling and rebuilding and discusses inconsistencies when caches assume determinism that the build does not have. These sources support explicit dependency closure and cache assumptions, not the claim that hashing source makes every output reproducible. [^10][^12]

For Moriarty, a cache key for a proof or financial evaluation must cover more than code. It needs the exact relevant input state, observations, authorization, bounds, claim statement and verifier context. A prior proof’s existence does not establish current unspent predecessor availability. An unchanged source file with a different compiler, rounding policy or ledger deployment is not automatically the same accepted execution.

**Recommendation.** Begin with content-addressed artifacts stored alongside conventional source and review tools. Resolve readable dependency names to full references before an artifact is admitted. Display both a readable name and a shortened digest for inspection, while using full digests in signed/verified payloads. Do not fetch arbitrary code at execution time merely because its hash is known. Dependency closure must be available, bounded, typechecked and admitted in advance.

## Evolution, metadata and outstanding obligations

Unison’s update workflow explicitly tracks affected definitions and library upgrades; the existing definition remains distinct from its replacement. Its license guide treats authorship and license information as values associated with a project. These are useful workflow ideas. They do not imply that documentation, provenance, a dependency’s license, or the admissibility of an upgrade disappears when code has a content hash. [^25][^26]

Moriarty should show three different changes in review: presentation-only changes; semantic changes creating a new program identity; and deployment or verifier changes creating a new execution context. A tool should explain which names and references changed, which claims need rechecking, and which existing positions still refer to the old version. Even a logically equivalent refactoring may produce a different Core hash unless equivalence is separately established.

A hash-preserving rename is attractive for libraries. A deployed financial obligation needs more: the debtor, creditor, outstanding amount, denomination, claim root, authority and remaining work must survive the transition. Upgrading a function or alias must not silently redirect an existing agreement into new semantics. A migration is an explicitly authorized transition with conservation and correspondence obligations, not a namespace edit. Consumed nonce/history and remaining authority/work survive migration. Unchanged duties carry forward automatically; a changed duty requires a funded allocation, named domain transformation or explicit authorized resolution. Structurally detectable omissions can fail statically, while value-dependent conservation belongs to preparation and proof checking. No general linear calculus or universal static rejection guarantee is established.

The existing vault study supplies a concrete example. A pending redemption request can be partly fulfilled while preserving an outstanding entitlement. Replacing its code definition, changing an oracle policy or exhausting a local work budget must not delete that entitlement. A design that makes refactoring easy but loses outstanding duties fails the core financial requirement. [R5]

## Proposed developer evaluation

**Specified-only study.** Formative sessions can begin with the proposed partial-payment material now. A confirmatory preregistered comparison waits for a stable executable prototype carrying the case through source, typed Core, K and evaluator, plus pilot calibration. No session or comparison was run here. Formative sessions should identify misunderstandings before a broad experiment: ask participants to explain a staged update, repair a unit mismatch, predict a partial payment’s residual debt, and distinguish a prepared proposal from accepted settlement. Record errors and the point at which participants seek documentation, including initial syntax/setup failures that prevent reaching the financial task. Do not convert a small formative sample into a population estimate.

The comparison should hold semantics, tasks, documentation quality and editor support constant while varying one design choice at a time. Candidate comparisons include explicit versus inferred local bindings while keeping public financial interfaces explicit in every condition, domain-specific versus compiler-internal diagnostics, and canonical `.mori` text versus an inert TypeScript template. Changing the language, tool quality and teaching material together would make causal attribution difficult.

Recruit across the three experience groups described above; report prior TypeScript, functional-programming and DeFi experience. Counterbalance task order, use equivalent task variants, include transfer tasks and a delayed comprehension task, and separate successful completion from completion time. A faster answer that authorizes excess spending is a failure, not a usability gain. Record correctness, time, assistance, confidence calibration and qualitative misconceptions.

Predefine financial correctness for each task independently of the candidate implementation. Include adversarial tasks: a share amount passed as a token payment, a stale quote, a partial fill that retains a duty, an apparent refund that conceals cumulative gross debit, and an unchanged source compiled under altered claim context. Reviewers should score against the expected financial relation without knowing the experimental condition where feasible.

Determine the confirmatory sample size from pilot variance and a meaningful effect threshold, with uncertainty intervals and treatment of abandoned tasks declared in advance. Avoid a fabricated power calculation in the absence of those inputs. Preregister the main comparisons, account for multiple outcomes, publish anonymized task materials when consent permits, and preserve null or conflicting findings. Tool telemetry must exclude private keys and witness material.

## Delivery sequence and unresolved decisions

The first slice should demonstrate a source-level partial payment with explicit USD units, checked allocation, one staged state update, a complete transfer proposal and a surviving duty. Give the same case to the source checker, independent expected-result calculation, K and evaluator. Reject wrong units, excess payment, unauthorized recipient, unsupported effect and duty deletion. The source may look TypeScript-like while all four paths use the existing financial and proof constraints.

Next, introduce one immutable dependency reference and compare a pure local rename, a changed arithmetic operation, a changed nominal asset binding and a changed compiler/claim context. The expected identity changes should come from the written encoding contract. Keep the old program available and reject silent reinterpretation of a preexisting obligation. These are proposed acceptance tests; they have not been executed in this research.

The key unresolved design decisions are whether embedding must be a product feature in the first release; which public annotations are mandatory; whether financial reactivity initially needs anything beyond explicit actions and finite schedules; and the precise content-hash domains. The feature checklist records twenty converged disposition labels after blind proposals and peer critique. The [convergence record](CONVERGENCE.md) preserves substantive amendments and dissent; labels alone do not settle all implementation choices. These are recommended requirements, not a semantic freeze or evidence of human usability.

The three-seat review supports the recommended direction but leaves duty-checker teachability, hash display and later database tooling as explicit design questions. Ordinary text suffices initially; permanent exclusion of database editing is not justified. Midnight Preview is the fixed financial target, while successful financial acceptance remains open.

There is no basis yet for claiming that the resulting language is easier for developers, that its full semantics are total, or that it produces accepted proof-carrying financial transactions. The useful result is a constrained design direction and a falsifiable checklist grounded in actual language mechanisms and bounded empirical evidence.

## References

Full academic metadata and locators are in [PAPER-ATLAS.md](PAPER-ATLAS.md); exact captures and hashes are in [SOURCE-INDEX.md](SOURCE-INDEX.md) and [source-manifest.json](source-manifest.json). Source identifiers below refer to those immutable records.

- D01–D03: Elm, [The Elm Architecture](https://guide.elm-lang.org/architecture/), [Commands and Subscriptions](https://guide.elm-lang.org/effects/), and [Time](https://guide.elm-lang.org/effects/time.html), current documentation observed September 9, 2026.
- D04R: Evan Czaplicki, [A Farewell to FRP](https://elm-lang.org/news/farewell-to-frp), May 10, 2016; rendered snapshot.
- D05: Elm, [Ports](https://guide.elm-lang.org/interop/ports.html), current documentation.
- D06–D07: Unison, [The big idea](https://www.unison-lang.org/docs/the-big-idea/) and [Hashes](https://www.unison-lang.org/docs/language-reference/hashes/), current documentation; benefit claims treated as maintainer claims.
- D08: Unison, [Abilities and ability handlers](https://www.unison-lang.org/docs/language-reference/abilities-and-ability-handlers/), current reference.
- D09–D10: Unison, [A tour](https://www.unison-lang.org/docs/tour/) and [Unique and structural types](https://www.unison-lang.org/docs/fundamentals/data-types/unique-and-structural-types/).
- D11: Midnight, [Compact reference](https://docs.midnight.network/compact/reference/compact-reference), current reference observed September 9, 2026.
- D12–D14: Unison, [Update workflows](https://www.unison-lang.org/docs/usage-topics/workflow-how-tos/update-code/), [Author and license](https://www.unison-lang.org/docs/tooling/author-license/) and [Unique types reference](https://www.unison-lang.org/docs/language-reference/unique-types/).
- R1: Moriarty, [Proposed surface and semantics](../defi-language-design-2026-09-07/LANGUAGE-DESIGN.md), September 7, 2026; S2 proposal.
- R2: Moriarty, [Successor syntax profile](../../experiments/moriarty-language/spec/successor/README.md), observed at base `9157a29`.
- R3: Moriarty, [Roadmap](../../ROADMAP.md) and [report reconciliation](../../openspec/REPORT-RECONCILIATION-2026-09-07.md), base snapshot.
- R4: Moriarty, [remaining roadmap](../defi-report8-comparison-2026-09-09/ROADMAP-REMAINING.md), September 9, 2026.
- R5: Moriarty, [Vault accounting, collateral value and Moriarty](../erc4626-vault-report-2026-09-08/DESIGN-IMPLICATIONS.md), September 8, 2026; prior source-scoped research.

[^1]: Andreas Stefik, Susanna Siebert, [An Empirical Investigation into Programming Language Syntax](https://www.vidarholen.net/~vidar/An_Empirical_Investigation_into_Programming_Language_Syntax.pdf), 2013. Capture U01; locators and limitations in the atlas/source index.

[^2]: Michael Coblenz, Gauri Kambhatla, Paulette Koronkevich, Jenna L. Wise, Celeste Barnaby, Joshua Sunshine, Jonathan Aldrich, Brad A. Myers, [PLIERS: A Process that Integrates User-Centered Methods into Programming Language Design](https://www.cs.cmu.edu/~jssunshi/assets/pdf/coblenz2021PLIERS.pdf), 2021. Capture U02; locators and limitations in the atlas/source index.

[^3]: Clemens Mayer, Stefan Hanenberg, Romain Robbes, Éric Tanter, Andreas Stefik, [Static Type Systems (Sometimes) have a Positive Impact on the Usability of Undocumented Software: An Empirical Evaluation](https://www.dcc.uchile.cl/TR/2012/TR_DCC-20120418-005.pdf), 2012. Capture U04; locators and limitations in the atlas/source index.

[^4]: James Prather, Raymond Pettit, Kayla Holcomb McMurry, Alani Peters, John Homer, Nevan Simone, Maxine Cohen, [On Novices’ Interaction with Compiler Error Messages: A Human Factors Approach](https://digitalcommons.acu.edu/cgi/viewcontent.cgi?article=1003&context=info_tech_computing), 2017. Capture U05; locators and limitations in the atlas/source index.

[^5]: Tomaž Kosar, Nuno Oliveira, Marjan Mernik, Maria João Varanda Pereira, Matej Črepinšek, Daniela da Cruz, Pedro Rangel Henriques, [Comparing General-Purpose and Domain-Specific Languages: An Empirical Study](https://www.comsis.org/pdf.php?id=0702), 2010. Capture U06; locators and limitations in the atlas/source index.

[^6]: Alan F. Blackwell, Thomas R. G. Green, [A Cognitive Dimensions Questionnaire Optimised for Users](https://ppig.org/files/2000-PPIG-12th-blackwell.pdf), 2000. Capture U09; locators and limitations in the atlas/source index.

[^7]: Michael Coblenz, Ariel Davis, Megan Hofmann, Vivian Huang, Siyue Jin, Max Krieger, Kyle Liang, Brian Wei, Mengchen Sam Yong, Jonathan Aldrich, [User-Centered Programming Language Design: A Course-Based Case Study](https://arxiv.org/pdf/2011.07565), 2020-11-15. Capture U10; locators and limitations in the atlas/source index.

[^8]: Evan Czaplicki and Stephen Chong, [Asynchronous Functional Reactive Programming for GUIs](https://people.seas.harvard.edu/~chong/pubs/pldi13-elm.pdf), 2013. Capture S01; locators and limitations in the atlas/source index.

[^9]: Patrick Bahr, Christian Uldal Graulund and Rasmus Ejlers Møgelberg, [Simply RaTT](https://arxiv.org/pdf/1903.05879), arXiv:1903.05879v2, 2019-06-11. Capture S03; locators and limitations in the atlas/source index.

[^10]: Eelco Dolstra, Merijn de Jonge and Eelco Visser, [Nix: A Safe and Policy-Free System for Software Deployment](https://eelcovisser.org/publications/2004/DolstraJV04.pdf), LISA XVIII, 2004, pp.79–92. Capture S04; locators and limitations in the atlas/source index.

[^11]: Daan Leijen, [Koka: Programming with Row Polymorphic Effect Types](https://arxiv.org/pdf/1406.2061), EPTCS 153, 2014, pp.100–126. Capture S05; locators and limitations in the atlas/source index.

[^12]: Andrey Mokhov, Neil Mitchell and Simon Peyton Jones, [Build Systems a la Carte](https://www.microsoft.com/en-us/research/wp-content/uploads/2018/03/build-systems.pdf), PACMPL 2(ICFP), Article 79, 2018. Capture S06; locators and limitations in the atlas/source index.

[^13]: Evan Czaplicki, [Elm: Concurrent FRP for Functional GUIs](https://elm-lang.org/assets/papers/concurrent-frp.pdf), Senior thesis, Harvard, 2012-03-30. Capture S07; locators and limitations in the atlas/source index.

[^14]: guide.elm-lang.org, [The Elm Architecture · An Introduction to Elm](https://guide.elm-lang.org/architecture/), not established; dated retrieval snapshot. Capture D01; locators and limitations in the atlas/source index.

[^15]: guide.elm-lang.org, [Commands and Subscriptions · An Introduction to Elm](https://guide.elm-lang.org/effects/), not established; dated retrieval snapshot. Capture D02; locators and limitations in the atlas/source index.

[^16]: guide.elm-lang.org, [Time · An Introduction to Elm](https://guide.elm-lang.org/effects/time.html), not established; dated retrieval snapshot. Capture D03; locators and limitations in the atlas/source index.

[^17]: Project documentation, [A Farewell to FRP](https://elm-lang.org/news/farewell-to-frp), page dated where available; see text. Capture D04R; locators and limitations in the atlas/source index.

[^18]: guide.elm-lang.org, [Ports · An Introduction to Elm](https://guide.elm-lang.org/interop/ports.html), not established; dated retrieval snapshot. Capture D05; locators and limitations in the atlas/source index.

[^19]: www.unison-lang.org, [💡 The big idea · Unison programming language](https://www.unison-lang.org/docs/the-big-idea/), not established; dated retrieval snapshot. Capture D06; locators and limitations in the atlas/source index.

[^20]: www.unison-lang.org, [Hashes · Unison programming language](https://www.unison-lang.org/docs/language-reference/hashes/), not established; dated retrieval snapshot. Capture D07; locators and limitations in the atlas/source index.

[^21]: www.unison-lang.org, [Abilities and ability handlers · Unison programming language](https://www.unison-lang.org/docs/language-reference/abilities-and-ability-handlers/), not established; dated retrieval snapshot. Capture D08; locators and limitations in the atlas/source index.

[^22]: www.unison-lang.org, [🗺 A tour of Unison · Unison programming language](https://www.unison-lang.org/docs/tour/), not established; dated retrieval snapshot. Capture D09; locators and limitations in the atlas/source index.

[^23]: www.unison-lang.org, [Defining your own Unison data types · Unison programming language](https://www.unison-lang.org/docs/fundamentals/data-types/unique-and-structural-types/), not established; dated retrieval snapshot. Capture D10; locators and limitations in the atlas/source index.

[^24]: docs.midnight.network, [Compact reference | Midnight Docs](https://docs.midnight.network/compact/reference/compact-reference), not established; dated retrieval snapshot. Capture D11; locators and limitations in the atlas/source index.

[^25]: Project documentation, [Common workflows for updating Unison code · Unison programming language](https://www.unison-lang.org/docs/usage-topics/workflow-how-tos/update-code/), page dated where available; see text. Capture D12; locators and limitations in the atlas/source index.

[^26]: Project documentation, [Creating an author and license · Unison programming language](https://www.unison-lang.org/docs/tooling/author-license/), page dated where available; see text. Capture D13; locators and limitations in the atlas/source index.

[^27]: Unison Computing, [Unique types](https://www.unison-lang.org/docs/language-reference/unique-types/), undated current reference. Capture D14; locators and limitations in the atlas/source index.

# Annotated paper atlas

Thirteen complete primary academic documents, with related Elm thesis/paper counted as a shared lineage. The authors’ methods, formal claims and limitations were inspected at the named locators; no experiments or proofs were independently reproduced.

## Empirical language usability atlas

This atlas supports a research-stage choice of Moriarty surface and tooling. Seven complete primary works were acquired and their methods, results, and limitations inspected. They support task-specific experiments and a cautious transfer of design ideas. They do not establish that C-style syntax, TypeScript familiarity, functional purity, inferred types, or a domain-specific language universally improves developer performance. All recommendations are S2, specified-only; no participant experiment was reproduced here.

Repository observation: `wiki/index.md`, `wiki/hot.md`, and the existing `deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md` already identify a brace-delimited financial surface, explicit requirements and postconditions, nominal asset units, immutable `pre`, tentative `next`, and final `post`. The present evidence tests assumptions behind that direction without changing its implementation or acceptance status. The guarded status reports operational-history blockers for loan-swap implementation; this source research dispatches no implementation or network action.

## Evidence inventory

PDF page numbers below are one-based positions in the captured file. For U06, the publisher returns the entire issue: the target article is PDF pages 15–32, printed pages 247–264. Raw bytes, extraction, and exact dated SHA-256 receipts are in [`usability/`](../../raw/sources/language-design-2026-09-09/usability/). Machine-readable bibliographic metadata and coverage are in its `source-inventory.json`. Abstracts and search snippets are discovery aids, not counted as inspected works.

| ID | Primary work and captured version | Method / population | Main evidence location |
|---|---|---|---|
| U01 | Stefik and Siebert, *An Empirical Investigation into Programming Language Syntax*, TOCE 13(4), article 19, 2013; complete 40-page author-formatted paper on a public mirror | Two intuition surveys, 166 responses each; two randomized novice coding experiments, 18 and 72 analyzed participants | PDF 6–9, 26–35; Table XXIII on 31 |
| U02 | Coblenz et al., *PLIERS: A Process that Integrates User-Centered Methods into Programming Language Design*, TOCHI 28(4), article 28, 2021; 53-page published author-hosted copy | Iterative formative studies plus Glacier and Obsidian summative studies; Obsidian/Solidity RCT analyzed 20 Java programmers | PDF 14–19, 36–39, 43–46 |
| U04 | Mayer, Hanenberg, Robbes, Tanter and Stefik, *Static Type Systems (Sometimes) have a Positive Impact on the Usability of Undocumented Software: An Empirical Evaluation*, 2012 university technical-report copy | Java/Groovy within-subject API tasks; 33 recruited, 27 analyzed students | PDF 4–11, 13–14 |
| U05 | Prather et al., *On Novices’ Interaction with Compiler Error Messages: A Human Factors Approach*, ICER 2017, 74–82; institutional deposit with repository cover | Two pilots; message-comprehension quizzes, 27 complete cases; 31 one-hour think-aloud sessions including a 35-minute programming task | PDF 6–10 |
| U06 | Kosar et al., *Comparing General-Purpose and Domain-Specific Languages: An Empirical Study*, ComSIS 7(2), 2010, 247–264; publisher issue PDF | XAML versus C# Forms questionnaire, 36 students recruited, 35 retained | PDF 20–28, especially 24–25 |
| U09 | Blackwell and Green, *A Cognitive Dimensions Questionnaire Optimised for Users*, PPIG 2000; 11-page public proceedings file, despite cover's 137–154 bibliographic range | Exploratory questionnaire pilot, 18 respondents across programming and music systems | PDF 3–10 |
| U10 | Coblenz et al., *User-Centered Programming Language Design: A Course-Based Case Study*, arXiv:2011.07565v1, 15 November 2020; 7 pages | Eight student language designers, six completed projects, convenience-sample studies | PDF 2–7 |

## U01 — syntax, intuition and prior experience

**Source fact.** The first two studies ask participants to rate word and syntax choices, rather than measure programming productivity. Both had 166 respondents, with partly overlapping participation. Programming experience is self-reported. Study 3 analyzes 18 novices across Quorum, Perl and Randomo; Study 4 analyzes 72 novices across six languages. Randomo uses deliberately arbitrary tokens. Subjects receive examples for early tasks and later lose that reference; this is constrained early learning, not ordinary professional development. [U01, PDF 6–9, 26–28](https://www.vidarholen.net/~vidar/An_Empirical_Investigation_into_Programming_Language_Syntax.pdf).

**Source fact.** In Study 4, Ruby, Python and Quorum outperform Randomo on the study's token-accuracy outcome; Java and Perl do not achieve a statistically significant advantage over Randomo. Most pairwise comparisons among the real languages are not significant; Ruby versus Java is the exception noted by the authors. The correct reading is neither that Java equals random syntax nor that Python defeats all alternatives. Ratings of generic syntax were inconclusive. [U01, PDF 13, 30–31].

**Limitation.** Small novice groups, short constrained tasks, token-based grading, English-heavy recruitment and whole-language condition bundles limit causal attribution to individual punctuation choices. Failure to reject a null hypothesis does not establish equivalence. Authors explicitly distinguish these tasks from professional programming and even normal college coursework. [U01, PDF 28–29, 34–35].

**Recommendation — adapt.** Treat recognizable words and low-surprise punctuation as hypotheses for a defined audience. A TypeScript-familiar surface may help experienced TypeScript users through transfer; this study neither measures that population nor tests TypeScript. Compare clause comprehension and repair for `requires`, `ensures`, `pre` and `next`, stratified by prior TypeScript and functional-language experience. Defer claims that braces are intrinsically easiest, or that replacing `==` with `=` will improve Moriarty overall. Arithmetic familiarity and semantic clarity need separate measures.

## U02 — user-centered design with strong static constraints

**Source fact.** PLIERS combines interviews, natural programming, lightweight task studies, tutorials with comprehension checks, and later controlled comparisons. It explicitly addresses costly prototypes, variation between programmers, prior-language bias and interaction among features. These methods coexist with formal language design; user suggestions are not accepted merely because users request them. [U02, PDF 14–19, 39–46](https://www.cs.cmu.edu/~jssunshi/assets/pdf/coblenz2021PLIERS.pdf).

**Source fact.** The Obsidian/Solidity RCT recruited 21 Java programmers, excluded one with an unusually long tutorial, and analyzed ten per condition. In Auction, correct completion was 7/10 versus 2/10. The reported p≈0.015 applies to correctness among those claiming completion; the all-participant correct-completion comparison is p≈0.070. These denominators must not be conflated. Two successful Obsidian participants encountered compiler diagnostics about losing assets. [U02, PDF 36–37, Table 6].

**Source fact.** The open-ended Casino task reverses the easy success narrative. Among compiling completed programs, four Obsidian participants all misused `disown`; none had a fully correct solution, compared with one Solidity solution. Their mean time was 64 versus 37 minutes. The authors say the task was too hard for the allocated time. This does not show that ownership is useless; it shows a concrete failure of an escape hatch and a limit on generalizing from simpler tasks. [U02, PDF 37–39, Table 8].

**Source fact.** Glacier's task table also needs care. For enforcing Person immutability, success was 10/10 versus 0/10 with Java `final`. For a programming task about returning references, success among finishers was 7/7 versus 4/8, with p≈0.077; for the hash-map task, 7/7 versus 3/10, p≈0.0098. The paper calls both differences significant in its prose, but p≈0.077 does not meet a conventional 0.05 threshold. [U02, PDF 19, Table 2].

**Recommendation — adopt method; adapt language ideas.** Use tutorial checks and formative task observation before comparative trials. Test typed amounts, residual duties and explicit asset transfer on real financial cases; count wrong recipient, missing fee and discarded obligation errors separately. Treat unchecked casts, suppressions and ambient authority as repair behaviors to observe. Defer a general `disown`-like feature unless its necessary use case and constrained semantics are separately justified. Pure functions and nominal types may be valuable, but PLIERS does not establish Moriarty's soundness or supply a generic usability certificate for strong typing.

## U04 — types as documentation, with task-dependent costs

**Source fact.** The experiment compares typed Java APIs with Groovy versions obtained by removing annotations and related declarations. A deliberately comparable environment avoids unequal IDE support. Five tasks chiefly instantiate and connect objects; they do not exercise a representative mix of algorithms. Thirty-three students were recruited; six were excluded for incompletion or task-order/measurement issues, leaving 27. Participants had Java training. [U04, PDF 4–8](https://www.dcc.uchile.cl/TR/2012/TR_DCC-20120418-005.pdf).

**Source fact.** Static typing favors tasks 1, 4 and 5, while dynamic typing favors tasks 2 and 3 in the authors' task-specific analysis. Their aggregate between-subject analysis does not reveal a simple overall type-system effect. Recursive and multi-class structures offer cases in which annotations expose useful design information, but the authors reject a simple rule based only on number or complexity of types. [U04, PDF 9–11, 13–14].

**Limitation.** Familiarity, carryover, excluded non-completers, stripped-static rather than idiomatic dynamic APIs, and artificial short tasks restrict transfer. Type annotations and static checking are bundled; the paper explicitly proposes future work separating them and testing inference. Its explanatory claims about easier tasks are hypotheses, not a universal law. [U04, PDF 8, 14].

**Recommendation — adapt.** Keep function signatures and asset/amount identities visible at module and action boundaries. Evaluate local inference separately from removing type information entirely. Measure navigation and repair as well as elapsed time. Defer claims that Elm-style inference necessarily outperforms explicit TypeScript-like annotations, or that dynamic local expressions improve financial correctness. The study directly supports asking where visible types help developers reconstruct an API.

## U05 — diagnostic comprehension does not equal successful repair

**Source fact.** Prather and colleagues use Athene, an automated assessment system. Two six-person pilots precede comprehension quizzes and a think-aloud study. Of a class of 31, 27 complete all six message quizzes. Standard messages yield 17.28% incorrect interpretations; enhanced messages yield 6.17%, with paired-test p<0.035. These are message-understanding outcomes outside normal programming context. [U05, PDF 6–8](https://digitalcommons.acu.edu/cgi/viewcontent.cgi?article=1003&context=info_tech_computing).

**Source fact.** The practical assessment uses 31 one-hour sessions with a 35-minute coding task and comparison against previous course semesters. The authors report no substantial improvement in overall learning outcomes. Observations nevertheless show many participants reading and acting usefully on enhanced messages. The lower experimental class score cannot cleanly be attributed to messages: compilation conditions, access to previous programs, teachers, semesters and think-aloud procedure differ. [U05, PDF 7–10].

**Recommendation — adopt evaluation distinction; adapt presentation.** Give a specific source span, the expected and actual financial types, and a local explanation of the violated requirement. Test whether developers fix the underlying amount, recipient or obligation error without weakening a condition. Measure interpretation, first corrective edit, eventual correct repair and recurrence separately. Defer claims that longer or friendlier errors automatically improve productivity. Even the pilot's wording change from an offer of help to neutral additional information reflects observed behavior in a particular educational setting, not a universal copywriting law.

## U06 — domain notation can help, but the headline compresses the data

**Source fact.** Thirty-six second-year computing students compare XAML with C# Forms for GUI understanding; one incomplete questionnaire is excluded. Tutorials and examples are supplied, and cohorts take the two notations in different orders. Participants report much greater prior C# Forms familiarity. This compares a GUI DSL with a GUI library, not all DSLs with all general-purpose languages. [U06, printed 252–254 / PDF 20–22](https://www.comsis.org/pdf.php?id=0702).

**Source fact.** Table 4 gives success means of 57.62% versus 40.95% for learning, 64.57% versus 50.86% for perceiving and 70.95% versus 33.33% for evolving. Total is 64.34% versus 43.37%. These are absolute differences of 16.67, 13.71, 37.62 and 20.97 percentage points. Table 5 reports the total difference with a 95% interval of 13.186 to 28.757. The conclusion's statement of roughly 15% better across all three categories fails to preserve that heterogeneity. Prefer the table and retain the discrepancy. [U06, printed 256–257, 260 / PDF 24–25, 28].

**Limitation.** Questionnaire performance is not production productivity or secure transaction authoring. The paper describes an independent t-test despite participants answering both questionnaires; without reanalysis, take its inferential claim as reported rather than validated. Its cognitive-dimension attribution distributes question performance equally among selected dimensions, an assumption rather than independent identification of causal mechanisms. [U06, PDF 25–27].

**Recommendation — adapt.** Prefer domain vocabulary when it makes recipient, asset, rounding and outstanding obligation visible. Compare realistic clause edits against an equivalent library-style presentation. Defer the assertion that a financial DSL must improve accuracy by 15%, or any other borrowed effect size. Explicit financial blocks can reduce translation distance while introducing their own new learning cost.

## U09 — a vocabulary for critique, not a scalar usability score

**Source fact.** The pilot has 18 respondents with no previous cognitive-dimensions familiarity. Ten complete a questionnaire without supervision; eight use an interview format. Systems range from C++/Emacs and theorem provers to music notation tools. Respondents identify concrete usability problems using concepts such as visibility, hidden dependencies, change effort, role expressiveness and premature commitment. Musicians report more issues, but interview format and questionnaire differences prevent interpreting that as a clean group effect. [U09, PDF 3–10](https://ppig.org/files/2000-PPIG-12th-blackwell.pdf).

**Limitation.** Small convenience sample and qualitative pilot; no randomized comparison of languages and no validated total ranking. Participants sometimes cannot distinguish notation from environment or imagine alternatives to a familiar notation. The paper makes precisely that limitation salient: usability belongs to the combination of notation and environment. [U09, PDF 9–10].

**Recommendation — adopt vocabulary; defer numeric scoring.** Ask reviewers to locate hidden relationships between a nominal amount and its ledger asset, the source clause and its generated diagnostic, and a partial payment and surviving duty. Ask which declarations must be understood simultaneously and how a rounding-policy change propagates. Treat comments and formatting as useful secondary notation while keeping their lack of authority explicit. Evaluate editor navigation and source syntax together. Do not add dimension ratings into an unsupported league table declaring TypeScript, Elm or Unison the winner.

## U10 — natural programming reveals expectations, not optimal syntax

**Source fact.** Eight students complete six language projects after instruction in user-centered methods, including four 80-minute class meetings on those methods. Their convenience-sample studies expose designer assumptions. In a concurrency study, users choose named send/receive functions rather than Go's arrow syntax. An IoT study intended to examine when code runs instead finds confusion about where it runs. In a puzzle study, even a participant preferring SML writes imperative constructs. [U10, PDF 2–6](https://arxiv.org/pdf/2011.07565v1).

**Limitation.** This is a course-based qualitative study of novice language designers, not evidence for a performance advantage of their final languages. End-user sample sizes are not consistently specified across the cases. Syntax that participants invent may reflect exposure rather than intrinsic ease; the authors explicitly distinguish naturalness from learnability and practical effectiveness. [U10, PDF 4–7].

**Recommendation — adapt.** Begin with short tasks in participants' own financial vocabulary, then test a constrained prototype. Ask them to express an asset-bound limit, partial repayment and delayed claim without first teaching the proposed spelling. Follow with comprehension and repair tasks using the actual semantics. Separate unfamiliar notation from a mistaken model of transaction acceptance. Defer treating preference, spontaneous invention or a familiar-looking API as proof of correct behavior.

## Cross-study dispositions and proposed tests

| Candidate decision | Disposition | Evidence-supported question | Specified-only experiment |
|---|---|---|---|
| TypeScript-like declarations and braces | Adapt | Does familiarity transfer for the intended developer population? | Equal-semantics comprehension/edit tasks, stratified by prior language exposure; record wrong-state and punctuation errors separately |
| Elm-style local inference with explicit boundaries | Adapt | Which visible annotations help reconstruct asset and function relationships? | Compare explicit locals with inferred locals plus type display; same signatures, tasks, diagnostics and training |
| Pure locals and explicit state transition clauses | Adapt | Can users predict all changed fields and surviving duties? | Predict post-state after partial repayment, then repair a stale pre-state assumption |
| Nominal amounts and typed transfer records | Adapt | Do diagnostics prevent asset and recipient mistakes without blocking valid edits? | Seed wrong-asset, wrong-recipient, fee and rounding errors; inspect repair rather than compile success alone |
| Friendly structured diagnostics | Adopt method; adapt wording | Is the message understood, and does it lead to the correct edit? | Separate message quiz from task repair; compare recurrence and attempts to suppress checks |
| Unison-like content identity or dependency display | Defer empirical superiority | Can users distinguish a name change from a semantic/dependency change? | Controlled rename/update task with version and dependent-action inspection; none of these papers directly tests content addressing |
| General escape hatches | Defer | Do users erase duties to satisfy the checker? | Attempt valid unusual transfer and invalid duty discard; capture suppression behavior without admitting unsafe semantics |

Inference: the seven works converge more strongly on how to evaluate a language than on which surface to copy. Financial correctness, learning, task time and developer confidence can move independently. A bounded empirical programme should preserve exact semantic equivalence between candidate surfaces, offer equal training and tools, predefine correctness and timeout handling, and include a deliberately open-ended task after smaller tasks. The Obsidian evidence makes the last step particularly consequential. No sample-size or power claim is made here; a pilot must first estimate task variance and identify floor/ceiling effects.

## Access gaps and source integrity

U03, the maintainability paper, and U08, Endrikat et al.'s API documentation paper, have retained 404 receipts from the discovered author host and 403 receipts from the alternate university host. U07, the 1996 Green/Petre paper, redirected from CiteSeerX to an archive response ending in 404. These are not counted as read works. U09 provides a directly inspected primary cognitive-dimensions study, but is not represented as a full substitute for inspecting Green/Petre's original framework paper. Hanenberg's 2010 experiment was discovered but not acquired; no result from it is asserted here.

The complete U06 issue contains unrelated articles; only PDF 15–32 supports its claims. U01's public mirror is a primary paper hosted by a third party; provenance is weaker than its authors' own site, whose discovered page returned 404. U04 is explicitly a technical-report version; identity with a final conference version was not established. U10 identifies v1 on its title page; acquisition used the unversioned PDF endpoint, so the bytes and observed v1, rather than the endpoint alone, identify this capture. No access control was bypassed and no packages were installed.

## Reactive semantics, effects and artifact identity

Six complete primary PDFs support the comparisons below. These are historical research sources and S2 design recommendations. No source result establishes Moriarty acceptance, performance, native proofs, or source/Core/K/ledger correspondence.

## Comparative atlas

| Source | Useful distinction | Mandatory limit |
|---|---|---|
| [S01](https://people.seas.harvard.edu/~chong/pubs/pldi13-elm.pdf) | Use immutable state transitions and explicitly identified inputs for tooling. Keep async results as proposals until authority and ledger checks accept them. | Neither first-stage normalization nor responsiveness establishes finite lifetime execution, finite queues, observation truth, quantitative conservation, or ledger acceptance. |
| [S03](https://arxiv.org/pdf/1903.05879) | Treat temporal availability as an independent typing concern. Define a bounded observation record with identity, effective time, expiry and authority before considering reactive syntax. | Productivity describes potentially infinite execution. Garbage collection does not establish a uniform byte/time bound; explicit state can grow. Fairness extensions are future work. These results say nothing about financial liveness or mandatory proofs. |
| [S04](https://eelcovisser.org/publications/2004/DolstraJV04.pdf) | Use separate source, Core, build and output identities. Include semantic profile, dependency closure and toolchain in artifact manifests. Authorize mutable financial migration separately from changing a code name. | An input-derived identifier assumes adequately captured inputs and appropriate determinism. It is not an independently checked output digest, semantic-equivalence proof, authorization, or migration theorem. The paper’s old hash examples are historical, not a recommended modern construction. |
| [S05](https://arxiv.org/pdf/1406.2061) | Give Moriarty primitive effects explicit typing and operational rules; close admitted effect sets before execution. Keep effect summaries separate from authority quantities and ordered ledger effects. | Termination alone does not give a registered work bound. Effect rows are not payment counts, rights, or affine continuation rules. This 2014 source does not establish the behavior of current Koka or arbitrary user-defined handlers. |
| [S06](https://www.microsoft.com/en-us/research/wp-content/uploads/2018/03/build-systems.pdf) | Cache each proof/compile stage by its actual immediate inputs and output identity; record semantic/compiler changes as dependencies. Specify an invalidation test for changed profile, kernel, observation policy and verifier lineage. | These are simplified executable models, not a certification of contemporary build products. Correct build outputs need not implement correct financial semantics. Shallow storage and nondeterminism change the correctness predicate. |
| [S07](https://elm-lang.org/assets/papers/concurrent-frp.pdf) | Prefer an approachable functional surface over a small explicit Core, with stable diagnostic/source mappings. Keep preview recomputation separate from accepted state and residual obligations. | Historical syntax and runtime claims do not specify current Elm. The thesis and PLDI paper belong to the same research line, so they are not independent replications. Embedding/dynamic-switching discussion does not authorize dynamic or unbounded financial execution. |

## S01: Asynchronous Functional Reactive Programming for GUIs

Evan Czaplicki and Stephen Chong. 2013. [Asynchronous Functional Reactive Programming for GUIs](https://people.seas.harvard.edu/~chong/pubs/pldi13-elm.pdf). Locators: §§3.1–3.3, pp.3–7; §4.3 and §5, pp.9–10.

**Source fact.** FElm separates normalization of functional expressions from ongoing signal execution. Its stratified types exclude signals of signals. Theorem 1 establishes type soundness and normalization of the first stage. The pipelined signal semantics uses unbounded FIFO queues; async preserves order inside a subgraph while relaxing global order between subgraphs. The implementation section explicitly omits direct performance evaluations.

**Moriarty recommendation.** Use immutable state transitions and explicitly identified inputs for tooling. Keep async results as proposals until authority and ledger checks accept them.

**Limits and inference.** Neither first-stage normalization nor responsiveness establishes finite lifetime execution, finite queues, observation truth, quantitative conservation, or ledger acceptance.

## S03: Simply RaTT

Patrick Bahr, Christian Uldal Graulund and Rasmus Ejlers Møgelberg. arXiv:1903.05879v2, 2019-06-11. [Simply RaTT](https://arxiv.org/pdf/1903.05879). Locators: §§2.1–2.3, pp.6–9; §§3.2–3.4, pp.12–15; Theorems 3.1/3.2 and 6.3, Proposition 6.4, pp.13–14,23–25; §8 p.26.

**Source fact.** Simply RaTT uses later and stable modalities with context restrictions, guarded recursion and a two-heap machine. For closed well-typed streams over value types built from unit, Nat, sums and products, Theorem 3.1 gives arbitrarily long finite productive prefixes. Theorem 3.2 gives typed stream-transducer progress; its proof establishes causality. The machine can discard the previous heap. The type system rejects a particular fixed-point-under-delay time leak.

**Moriarty recommendation.** Treat temporal availability as an independent typing concern. Define a bounded observation record with identity, effective time, expiry and authority before considering reactive syntax.

**Limits and inference.** Productivity describes potentially infinite execution. Garbage collection does not establish a uniform byte/time bound; explicit state can grow. Fairness extensions are future work. These results say nothing about financial liveness or mandatory proofs.

## S04: Nix: A Safe and Policy-Free System for Software Deployment

Eelco Dolstra, Merijn de Jonge and Eelco Visser. LISA XVIII, 2004, pp.79–92. [Nix: A Safe and Policy-Free System for Software Deployment](https://eelcovisser.org/publications/2004/DolstraJV04.pdf). Locators: Overview/Nix Store pp.81–83; Implementation/The Store and Building Components pp.84–85; User Environment Policies pp.87–88; Conclusion p.91 (PDF pages subtract 78).

**Source fact.** Nix separates user-facing names from immutable installation objects, identifies variants through hashes of build inputs, lowers high-level expressions to simpler store expressions, and deploys dependency closures. Build inputs include scripts, platform and environment bindings. The paper describes atomic environment changes through POSIX rename and retains old generations. Mutable service state is explicitly outside Nix control.

**Moriarty recommendation.** Use separate source, Core, build and output identities. Include semantic profile, dependency closure and toolchain in artifact manifests. Authorize mutable financial migration separately from changing a code name.

**Limits and inference.** An input-derived identifier assumes adequately captured inputs and appropriate determinism. It is not an independently checked output digest, semantic-equivalence proof, authorization, or migration theorem. The paper’s old hash examples are historical, not a recommended modern construction.

## S05: Koka: Programming with Row Polymorphic Effect Types

Daan Leijen. EPTCS 153, 2014, pp.100–126. [Koka: Programming with Row Polymorphic Effect Types](https://arxiv.org/pdf/1406.2061). Locators: §§2.1–2.7 pp.102–107; §5.2 p.117; §6 Theorems 2–4 pp.118–119 (PDF pages subtract 99).

**Source fact.** Koka uses strict evaluation and inferred row-polymorphic effects with duplicate labels. In its formal calculus, absence of exn excludes an unhandled exception result, not internally caught exceptions; absence of div entails termination. Heap encapsulation prevents local references escaping under its typing side conditions. The formal read primitive conservatively includes divergence; §2.7 distinguishes a finer implementation analysis and an incompletely implemented constraint case.

**Moriarty recommendation.** Give Moriarty primitive effects explicit typing and operational rules; close admitted effect sets before execution. Keep effect summaries separate from authority quantities and ordered ledger effects.

**Limits and inference.** Termination alone does not give a registered work bound. Effect rows are not payment counts, rights, or affine continuation rules. This 2014 source does not establish the behavior of current Koka or arbitrary user-defined handlers.

## S06: Build Systems a la Carte

Andrey Mokhov, Neil Mitchell and Simon Peyton Jones. PACMPL 2(ICFP), Article 79, 2018. [Build Systems a la Carte](https://www.microsoft.com/en-us/research/wp-content/uploads/2018/03/build-systems.pdf). Locators: Definition 2.1 p.3; Definition 3.1 p.11; §§4.2–4.4 pp.14–16; §§6.3–6.6 pp.23–26.

**Source fact.** The framework separates scheduling from rebuilding. Correctness preserves reachable inputs and requires every reachable computed key to agree with recomputation against the final store, assuming acyclic tasks. Verifying traces store hashes; constructive traces also store results. Deep traces omit intermediate dependencies and require determinism. The §6.4 Frankenbuild example combines nondeterministic intermediates with deep caching to yield inconsistent artifacts.

**Moriarty recommendation.** Cache each proof/compile stage by its actual immediate inputs and output identity; record semantic/compiler changes as dependencies. Specify an invalidation test for changed profile, kernel, observation policy and verifier lineage.

**Limits and inference.** These are simplified executable models, not a certification of contemporary build products. Correct build outputs need not implement correct financial semantics. Shallow storage and nondeterminism change the correctness predicate.

## S07: Elm: Concurrent FRP for Functional GUIs

Evan Czaplicki. Senior thesis, Harvard, 2012-03-30. [Elm: Concurrent FRP for Functional GUIs](https://elm-lang.org/assets/papers/concurrent-frp.pdf). Locators: Ch.3 pp.13–21; Ch.4 pp.22–31; Ch.6 pp.39–40; Ch.7 p.41 (PDF pages add 3).

**Source fact.** The thesis explains discrete signals, lift and foldp, a two-tier intermediate representation, and explicit async subgraphs. Let-bound signal representations prevent duplicate runtime nodes. Its examples separate presentation from reactive processing. Chapter 6 records concrete JavaScript backend limitations; Chapter 7 presents filtering and further reactive machinery as future work.

**Moriarty recommendation.** Prefer an approachable functional surface over a small explicit Core, with stable diagnostic/source mappings. Keep preview recomputation separate from accepted state and residual obligations.

**Limits and inference.** Historical syntax and runtime claims do not specify current Elm. The thesis and PLDI paper belong to the same research line, so they are not independent replications. Embedding/dynamic-switching discussion does not authorize dynamic or unbounded financial execution.

## Proposed semantic boundary

A reactive environment may supply a candidate observation or requested action. A bounded, deterministic Core transition prepares state, effects, remaining duties, residual authority and remaining work. A separate acceptance relation checks authenticated authority, admissible observations, predecessor consumption, all four mandatory claims, native verification and exact ledger effects. Reactive arrival and render completion are not acceptance events. This is a proposal constrained by the existing [language design](../defi-language-design-2026-09-07/LANGUAGE-DESIGN.md).

For a partial loan payment, a preview can update immediately when a quote arrives. Accepted progress must still allocate payment according to the agreement, retain unpaid principal and fees, reduce remaining authority/work, and persist predecessor consumption. Neither a pure update function nor a causal stream makes a quoted price true or pays the creditor.

A future handler design must address invocation multiplicity explicitly. A handler that can resume a continuation repeatedly must not thereby duplicate a transfer, owned resource, nonce or residual work. This is a required design question, not a result established by the 2014 Koka paper. Keep the finance-bearing operation set closed and first-order unless a separately reviewed resource argument admits more.

## Identity is a family of commitments

| Identity | Proposed committed content | Does not establish |
|---|---|---|
| Source identity | Exact UTF-8 bytes | Meaning, authorization or correct elaboration |
| Core identity | Canonical typed Core, semantic profile and encoding version | Source/Core correspondence |
| Build identity | Input closure, compiler/kernel/toolchain, flags, target and semantic dependencies | That returned output was honestly built |
| Artifact identity | Exact artifact bytes, distinct from build recipe | Correctness of those bytes |
| Execution statement | Program/profile, signed authority digest, predecessor, observations, complete effects, duties, successors and remaining work | Acceptance before native and ledger checks |

These are proposed domains, not a finalized encoding. A human name can map to a content identifier for authoring convenience, but a signed financial request must bind the permitted identities or an explicit upgrade policy. A code change cannot silently reinterpret a previously authorized obligation. Financial state migration needs its own conservation, authority and residual-duty rules.

## Specified-only validation cases

1. Supply the same bounded observation sequence in different delivery schedules. Compare accepted order, full effects and residuals under the explicit ordering policy; UI update timing may differ.
2. Present two asynchronous results for the same consumed predecessor. Exactly one may accept if the agreement permits only one consumption.
3. Generate inputs faster than computation completes. Check the defined queue/admission bound and failure result; do not infer a bound from a fixed graph.
4. Keep a stream productive while its explicit accumulator grows. The financial profile must reject growth beyond its registered size/work bound.
5. Change a compiler, canonicalization rule, semantic profile or immediate intermediate artifact. A cache entry with the old commitment must not establish the new result.
6. Preserve source bytes while changing only a human alias; preserve meaning under formatting while changing source bytes. Verify that each identity domain behaves according to its own contract.
7. Partially pay a loan, then expire or cancel a continuation. Residual duties must remain represented or be resolved by separately authorized rules.
8. Attempt multiple resumptions of a finance-bearing continuation. Require explicit rejection or separately proved resource-safe behavior; effects cannot replenish spent allowance.

All eight cases are proposals. No execution results are claimed.

## Coverage and provenance

The immutable source directory is [raw semantics](../../raw/sources/language-design-2026-09-09/semantics/). Each PDF has a capture receipt with requested/canonical URL, UTC retrieval time, HTTP status and SHA-256. [Reading manifest](../../raw/sources/language-design-2026-09-09/semantics/reading-manifest.json) adds page count, version, reviewed locators and extraction limits. [Typed claims](semantics-claims.json) distinguish source facts, recommendations and limiting inferences. The six PDFs contain 157 pages in total.

Coverage is substantive full-text source review at the named locators, rather than abstract-only evidence; it is not an independent check of every proof/appendix. PDF extraction loses some modal glyphs and diagram structure. In Simply RaTT, use the arXiv v2 date on the first page, not the manuscript’s stale template footer. Build Systems uses the 2018 paper; its later journal expansion is not included. The Elm thesis and paper are related versions and count as two documents, not two independent confirmations.

Elliott’s push-pull author host failed DNS resolution during the robots request. The [failure receipt](../../raw/sources/language-design-2026-09-09/semantics/S02-access-failure.json) is retained; that work is not counted as read. The 2012 Elm thesis supplies the sixth accessible primary source. Current Elm and Unison behavior requires the separately captured official documentation; historical FRP claims are not transferred to current Elm.

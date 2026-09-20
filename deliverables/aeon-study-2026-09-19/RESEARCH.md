# Aeon concepts and their application to Moriarty

Research date: 2026-09-19. This is an initial source-backed assessment, not an adopted language design or release approval.

## Sources and reproduced scope

- Aeon: https://github.com/alcides/aeon, branch `master`, commit `ef66bd95e6b7d2d5309453ee63640bc7fa1d988f`. Local checkout: `/home/charl/.graphify/repos/alcides/aeon`.
- Moriarty: https://github.com/CharlesHoskinson/Moriarty, branch `main`, commit `1896217a28553e0254b0ba319422ac4c0b39ac0d`. Latest upstream fetched. A separate clean checkout at `/home/charl/Moriarty-aeon-study` preserves the user's original branch and local edits.
- Evidence consists of pinned source, repository documentation, a focused 138-test Aeon run, five explicit trust/refinement probes, one SMT synthesis experiment, and Moriarty's existing local financial lifecycle demo. The full Aeon suite, all synthesizers, full semantic soundness, and live Midnight execution were not tested.

## Main recommendation to examine

Use Aeon's specification-guided synthesis and explicit trust reporting to improve off-chain Moriarty authoring. Keep Moriarty's bounded, exact financial semantics and independent transaction acceptance authoritative. Do not port Aeon's Python runtime, general recursion, or native FFI into the financial execution kernel. A solver result about an encoding is not a Midnight proof.

There are three plausible approaches. First, borrow concepts and implement a narrow Moriarty-native authoring layer. This gives the smallest semantic boundary and is the initial preference. Second, use Aeon as an external proposal engine behind an independently checked translation. This can accelerate an experiment but adds a language correspondence obligation. Third, adopt Aeon as the language substrate. This conflicts with existing bounded financial semantics and creates the largest migration and proof burden. The six-member PL review should challenge this preference and select an explicit scope.

## What Aeon actually does

### Compiler and type system

Aeon parses `.ae` into a surface representation, resolves imports, desugars and elaborates, lowers into Core, binds identifiers, links dependencies, checks types/refinements and linearity, and then supports evaluation, synthesis or export. The current compiler does **not** insert ANF. Older `AGENTS.md` and `CLAUDE.md` pipeline summaries are stale. `docs/design/existentials-replace-anf.md:3` records the removal. `aeon/compilation/compile.py:306` onward and `aeon/typechecking/typeinfer.py:610` onward show elaboration, direct lowering and existential handling.

A refinement `{v:Int | v > 0}` restricts values using a logical predicate. Dependent function results can mention arguments. Bidirectional checking produces verification conditions. Subtyping asks whether the source assumptions imply the target predicate. Liquid inference uses finite qualifier predicates and Horn solving. SMT checks whether the negation of an obligation has a model. The current `smt_valid` returns false on satisfiable counterexamples and on `unknown`, with a 200ms solver timeout (`aeon/verification/smt.py:732-795`). This Boolean conflates refutation and failure to establish a proof. Moriarty should retain those outcomes separately in diagnostics.

Existential binders preserve facts about intermediate expressions without synthetic term-level `let` bindings. That matters for linear consumption and alias accounting. This is a design lesson, not a reason to replace Moriarty's existing IR.

The type system also contains algebraic data types, measures, polymorphism, reflected and uninterpreted functions, refinement polymorphism and optional termination metrics. Horn-context lifting still skips some polymorphic value binders (`aeon/typechecking/entailment.py:45-57`). Source features do not imply complete theorem-proving support.

### Synthesis workflow

A hole has a goal type and a local typing context. The context defines permitted components. A backend proposes a term, substitutes it into the program and invokes validation. `make_validator` in `aeon/synthesis/entrypoint.py:84-89` calls `check_type` on the substituted program. The ordinary synthesis path builds separate evaluators for optimization and sampled properties. Different backends and program-level synthesis have different paths, so this inspection does not certify universal enforcement across every backend.

Aeon offers SMT literal completion, type-directed enumeration, grammar enumeration, genetic programming, LLM proposals, tactic steps, SyGuS, tree automata and abstraction-refinement search. The current CLI defaults to `tdsyn_enumerative`, despite `docs/synthesizers.md` calling GP the default. The simplest useful Moriarty experiment would use arithmetic completion or a tiny typed grammar. Search sophistication should follow measured failures on a financial task corpus.

Optimization is distinct from validity. Examples and sampled properties can guide search but do not establish universal correctness. Time budgets bound search effort. They do not prove a synthesized program's execution cost. CPU/energy fitness is not a bound on ledger fees or circuit size.

### Linear resources, typestate and termination

Aeon tracks multiplicities 0, 1 and unrestricted. Its linearity pass scales usage through function application and compares branch usage (`aeon/typechecking/linearity.py:1-32`). Typestate libraries combine linear handles with measures to describe legal state transitions. This fits the conceptual problem of consuming a funding or authorization capability once. However, compile-time linear use does not establish distributed uniqueness or prevent transaction replay. Moriarty still needs authenticated state, identifiers and ledger consumption checks.

`termination_metric_constraints` returns a trivial true condition when no decreasing metric is supplied (`aeon/typechecking/termination.py:393`). Optional termination annotations must not be mistaken for mandatory totality. Moriarty's finite source bounds, static work bounds, runtime work ledger and closure reserve remain separate requirements.

### Trust and developer experience

`aeon/facade/trust.py` computes the transitive frontier of axioms and refined native bindings for a program or selected function. It explicitly recognizes that native annotations are promises, not verified implementations. Unrefined native bindings are omitted, so it is not a complete security or effect inventory.

The LSP infoview exposes local goal/context information and solver counterexamples (`aeon/lsp/infoview.py:497-562`). Moriarty could show a financial obligation, its source span, a counterexample state, and the exact remaining assumption. Counterexample production is a separate bounded solver task and is not always available.

## Reproduced observations

The command in `aeon-tests.log` exercised trust reporting, undefined arithmetic rejection, SMT validity, termination, linearity, decidability classification and the SMT synthesizer. Result: **138 passed, 3 warnings, 13.71 seconds**, Python 3.13.14. The initial invocation misspelled one test filename and collected no tests. Its failed command log is retained separately.

`probes.py` produced these results in `probes.json`:

| Case | Result | Meaning |
| --- | --- | --- |
| Payment 97, minimum 90, fee 3, budget 100 | Accepted | Closed arithmetic property holds |
| Payment 100 with the same fee and budget | Rejected | Fee breaches the budget |
| Payment 89 with minimum 90 | Rejected | Required minimum fails |
| Native string `100` with the same refined signature | Accepted, native assumption reported | The checker trusts the annotation |
| Integer division by zero | Rejected | Current verifier rejects this undefined arithmetic |

`fee-constrained-hole.ae` asks for an integer at least 90 with `payment + 3 = 100`. `-s smt --budget 5 --strict-decidable` synthesized **97**, exit 0. This is a concrete-constant demonstration, not a synthesized general financial function or a Moriarty integration.

The existing Moriarty `examples/financial-lifecycle-payment.mjs` completed with assertions intact. It checked `/5` source, originated principal 100, accrued 10, repaid 30 with accrued discharge 10 and principal discharge 20, and retained outstanding liability 80. It exposed static action bounds and remaining work. This is local execution, not ledger settlement.

## Moriarty mapping

| Aeon concept | Moriarty application to consider | Existing boundary to preserve |
| --- | --- | --- |
| Refined signatures and VCs | Check pure arithmetic and action pre/post obligations | Exact UInt widths, units, scale, rounding, overflow and error semantics |
| Typed holes | External draft holes completed into ordinary `.mori` source | No unresolved holes in admitted programs |
| Small typed grammars | Search only approved pure constructors and in-scope values | No new FFI, unrestricted recursion or authority-bearing primitives |
| Explicit trust frontier | Explain assumptions per action/artifact | Include snapshot, oracle, kernel, compiler and proof/ledger correspondence assumptions |
| Linear handles | Improve diagnostics for duplicate funding/capability use | Existing kernel and ledger uniqueness checks remain mandatory |
| Typestate | Explain legal loan/lifecycle transitions | Persistent financial obligations survive partial payment and state transitions |
| Counterexamples | Give source-local failed financial obligations | Preserve typed rejection codes and no tentative effects |
| Separate fitness and validity | Rank admissible candidates by size/static work | Fees, signed intent and hard financial limits cannot become soft fitness |
| Explicit undecidable-fragment policy | Start with a documented exact solver fragment | Distinguish unsupported, timeout, unknown and genuine counterexample |
| Property/example corpora | Shared positive/adversarial financial tasks | Finite conformance is not formal correspondence |

The likely insertion point is outside `createFinancialAgreementSourceV5()` in `experiments/moriarty-language/src/successor/financial-agreement-source-v5.ts:56`. An authoring tool can produce source and submit it through `check` and `evaluate`. The shared compiler in `financial-agreement-source-compiler.ts` remains the consumer. Avoid a second evaluator that silently becomes the authority. A static verifier must encode **all** admitted inputs, not merely accept one snapshot.

For a repayment task, specify amount/asset identity, gross-debit limit, fee-inclusive net goal, transfer consumption, allocation order, residual liability and work reserve. A pure-expression pilot should not be credited with those effect-level guarantees. Stage effectful synthesis only after a reviewed relation connects source, kernel state/effects and the target backend.

## Roadmap state that the review must respect

Current README and the September 17 result record an independently reviewed fixed Preview loan payment. It is incorrect to say Moriarty has no financial settlement. It is equally incorrect to claim completion of the newer source-to-ledger lifecycle, swap coverage or mandatory PCD from that result. See `deliverables/preview-loan-2026-09-17/recovery-run01/RESULT.md` and `ROADMAP.md` at the pinned commit.

The generic plugin status still reports unresolved campaign history and stale binding/accounting inputs. No public transaction notifications are pending. This research does not authorize or dispatch a financial campaign. Roadmap cleanup should separate demonstrated capabilities, scoped evidence, active delivery work and deferred obligations. Preserve MC/SP acceptance identities and historical receipts instead of deleting inconvenient open gates.

## Contradictions and limitations

1. Old ANF pipeline summaries disagree with current source. Use direct Core lowering with existential types.
2. GP default documentation disagrees with current CLI. Use the observed type-directed default.
3. The Vericoding report lists 59/99 accepted candidates, including 27 degenerate arithmetic solutions. Current source and this run's tests reject the historical zero-divisor cases. Neither the old headline score nor an unqualified claim of a current vulnerability is justified.
4. Some Synquid benchmark ports weaken original specifications. Port success is not evidence for the original theorem.
5. CI accepts no-synthesis-solution exit status for ordinary examples. A green example job is not a synthesis success rate.
6. `native`, axioms, mutable external resources and unchecked CSV shape declarations expose trust assumptions. They are especially unsuitable as hidden financial safety evidence.
7. No formal soundness theorem, compiler correspondence proof, benchmark replication across all strategies, performance claim or deployment feasibility was established here.

## Graph artifacts

The interactive graph is `/home/charl/.graphify/repos/alcides/aeon/graphify-out/graph.html`. The JSON contains 7,521 nodes and 29,825 directed edges in 250 communities. Because it exceeds 5,000 nodes, HTML uses the aggregated community view. Python AST extraction covered 416 files, and semantic extraction covered 40 documents. Image assets were excluded. `.ae` and `.lark` are outside the extractor's parser support. Selected examples were inspected directly.

The raw extractor reports 790 dangling-endpoint edges, 315 self-loops and 6,427 repeated directed endpoint pairs collapsed in the simple graph. Preserve `health.json` and the raw extraction. This is an imperfect navigation map, not a complete call graph or proof. Graphify's estimated token reduction is 52.2x, derived from synthetic queries and approximate token counts. It is not a measured reduction in this session's cost. Semantic-agent token usage is unavailable.

Initial research is complete enough for the requested PL deliberation. Final integration choices remain open for the six reviewers. No Aeon feature has been adopted or implemented in Moriarty by this study.

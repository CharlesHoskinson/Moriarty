# Moriarty agentic-council advisory record

<!-- markdownlint-disable MD013 MD060 -->

Date: 2026-09-03 UTC  
Decision scope: DeFi taxonomy, Marlowe-to-Moriarty boundary, Compact/ZKIR
assurance, and the deep-research prompt  
Status: advisory evidence, not a formal governance vote or proof

## Provider and terminal evidence

Each provider received the same tool-free round-1 brief. Tools, web search,
memory, and subagents were disabled where the CLI exposed those controls. Sol
ran in a private temporary directory with a read-only sandbox. Provider output
is research advice, not source evidence.

| Round | Provider | Requested model | Provider-reported model | Terminal result | Counted |
|---|---|---|---|---|---|
| 1 | xAI/Grok | `grok-4.6` | `grok-4.6-build` | `end_turn`, one substantive result | yes, with routed-build label |
| 1 | Anthropic | `claude-fable-5-1` | canonical `claude-fable-5-1` | `completed`, zero web/tool calls | yes |
| 1 | OpenAI/Codex | `gpt-5.6-sol` | `gpt-5.6-sol` in CLI terminal record | exit 0, substantive result | yes |
| 2 | xAI/Grok | `grok-4.6` | `grok-4.6-build` | `end_turn`, substantive blind critique | yes, with routed-build label |
| 2 | Anthropic | `claude-fable-5-1` | canonical `claude-fable-5-1` | budget exhausted before a result | **abstention** |
| 2 | OpenAI/Codex | `gpt-5.6-sol` | `gpt-5.6-sol` in CLI terminal record | exit 0, substantive blind critique | yes |

The Fable round-2 failure is not converted into agreement. The Grok provider
envelope exposed an internal reasoning field; that field is not retained or
used. Only the provider's final `text` result informed the synthesis.

## Round 1 positions

### Position A: mechanism-first and aggressive demotion

This position accepts a finite Marlowe-shaped Core, M5 as the internal profile,
and generated Compact as the only initial backend. It rejects F1-F6/P as
exclusive language architecture. Its preferred rival is four primary
mechanisms—exchange, credit, derivative, and consensus claim—with F5 treated as
a claim-source facet, F6 as an authorization or mandate facet, and Prediction
as a conditional-token mechanism. It identifies unbounded generated containers
or unconstrained witness callbacks as an immediate stop condition.

Its strongest contribution is boundary discipline. A tokenized off-chain claim
does not become safer because it is a “family”; a mandate remains an authority
constraint even when it wraps exchange or credit; a binary contingent claim can
share a derivative payoff structure. Its weakness is premature compression:
the supplied metrics reject the old twelve more clearly than they establish the
four-mechanism replacement.

### Position B: provisional human taxonomy and formal internal profile

This position retains M2+M3 as a falsifiable human-facing taxonomy, M5 as the
machine profile, and generated Compact with translation validation. It adds
explicit multi-membership, stronger oracle, custody, sequencing, governance,
liquidity, and insurance tests. It proposes one minimal demonstration per
candidate family and a serious library-only fallback.

Its strongest contribution is separating three questions: whether labels help
people navigate the market, whether a formal behavior profile is adequate, and
whether a construct belongs in Core. Its strongest dissent is that current
evidence may justify only seven audited Compact templates with obligation and
assumption manifests. A new language must show an advantage over that smaller
solution.

### Position C: conditional transfer base and certifying elaboration

This position adds F0 conditional transfer for escrow, vesting, payments, and
streams; renames Prediction to contingent claims; and treats F4/F6 as an ordered
multi-membership case. It proposes a Moriarty Core written as a measured
semantic delta against Marlowe, a certifying elaborator, and an audit object made
of normalized Core, generated Compact, and a translation-validation
certificate.

It also flags three upstream claims for correction through primary-source
research: Marlowe already indexes accounts by party and token but lacks a
token-aware type discipline; `Address | Role` is a closed authorization
abstraction rather than no abstraction; and a Marlowe transaction applies an
ordered list of inputs rather than lacking multi-input transactions entirely.
These corrections are hypotheses until the pinned sources are cited.

Its strongest contribution is the audit triple and the requirement to measure
proof reuse instead of saying that Moriarty “inherits” Marlowe proofs. Its risky
recommendations—bounded-natural representation, reject-on-partial-payment,
atomic Step, and Attest—are observable semantic changes and cannot enter Core
before a recorded motion and proof-impact analysis.

## Blinded review result

The second-round brief removed provider names and exposed the three positions as
A, B, and C. Two terminal ballots agreed on these points:

1. Keep the taxonomy human-facing and falsifiable; M5 is the internal formal
   profile.
2. Use a mandatory facet schema with explicit `none` and `not-applicable`.
3. Use atomic swap as the smallest F1 proof case; use auction as a privacy and
   sequencing stress case.
4. Treat Moriarty as an implementation-independent semantic delta against
   Marlowe, but select a proof environment during the feasibility phase on
   measured proof reuse and maintainer capacity.
5. Treat partial payment, numeric bounds, atomic Step, typed Attest, authority,
   mandate, and split/merge as semantic motions or capabilities, not accepted
   syntax.
6. Require a bounded instance or an honest outside-kernel manifest for each of
   the 72 rows, not 72 imitations of full protocols.
7. Generate Compact first and defer direct ZKIR.
8. Keep library-only as the default fallback if a separate DSL has no measured
   assurance or review advantage.

The reviewers disagreed about the first experiment and release thresholds.
Grok favored an atomic-swap backend stop test with 1,000 traces before any
larger proof program. Sol favored a binary conditional-token plus atomic-swap
composition because it exercises more proposed semantic changes at once. Grok
would defer full proof generation until the semantic motions close; Sol requires
clean proof checking from source before release. These positions are compatible
as staged gates: use the atomic swap as the cheap feasibility stop test, then use
conditional-token exchange as the first high-risk semantic composition, and
require full proof generation before a deployment or support claim.

## Synthesis applied to the XML prompt

The research prompt adopts these decisions:

- F1-F6/P remains a candidate human taxonomy, not normative ontology.
- The four-mechanism model, F0 conditional transfer, Insurance,
  Payments/Streaming, Privacy, and formal-only M5 are explicit rivals or gaps.
- Every facet field must state a value, including `none` or `not-applicable`.
- The first experiment is a finite Marlowe-shaped Core fragment plus atomic
  swap. Generated Compact must contain no unbounded container or unconstrained
  witness callback.
- The feasibility test uses at least 1,000 coverage-guided, materially distinct
  differential traces. Release uses at least 10,000 per canonical application
  plus boundary and failure coverage.
- The audit object is the Core/Compact/translation-certificate triple, with a
  separate analysis of correlated defects.
- The proof environment is selected before the normative Core freeze. It is not
  selected by preference alone.
- Semantic changes are classified and decided before implementation scales.
- The 72-row requirement accepts explicit outside-kernel results.
- Full proof generation, client verification, two-builder reproducibility,
  scoped independent audit, and measured pilot comparison are release gates.
- Library-only is a planned outcome, not a consolation prize.

## Preserved dissent and stop case

The principal minority report remains unresolved: the seven-family taxonomy may
be an unnecessarily complicated human index. Four mechanisms plus explicit
claim-source, mandate, oracle, custody, settlement, and privacy facets might be
cleaner. The taxonomy study must compare them without turning either answer into
Core syntax.

The strongest no-redesign case is also preserved. Implement bounded Compact
templates, a Marlowe-derived obligation checker, a disclosure and assumption
manifest, and a client verifier. Do not build a distinct language if these
artifacts give users the same assurance and review benefit.

The earliest hard stop is backend boundedness. If the smallest generated
Compact fragment requires an unbounded container or an unconstrained witness
callback, do not enlarge Moriarty Core. Choose the audited-library path or stop.

## Advisory decision

**Revise and proceed only through a gated feasibility phase.** Confidence is
medium because the independent proposals converge on the architecture boundary
but no round-2 Fable ballot exists, proof generation is unmeasured, and the
taxonomy replacements have not been tested with genuine independent human
raters. No panel member approved a production DeFi Kernel on current evidence.

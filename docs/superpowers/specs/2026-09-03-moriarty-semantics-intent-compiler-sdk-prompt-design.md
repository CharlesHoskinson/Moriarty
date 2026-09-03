# Moriarty semantics and intent research-prompt design

Status: approved for prompt version 1.3

## Decision

Create one XML supplement to the existing Moriarty DeFi Kernel assignment. Do
not replace the existing taxonomy and feasibility work. Start from semantic
scope `0.0.0-e00.2` and require a semantic motion for every Core change.

## Alternatives

One replacement assignment would repeat settled evidence and blur current
scope. Three independent prompts would let the semantics, compiler, proof, and
SDK models drift. The focused supplement keeps one cross-layer correctness
claim while preserving the existing program.

For ACTUS, compare these three completion boundaries:

- Require all 277 public reference fixtures and classify all 32 taxonomy rows.
- Require all 32 taxonomy rows as executable contracts.
- Require only the 18 technically specified contract types.

Select the first boundary. The second boundary is not falsifiable because 14
taxonomy rows lack public reference fixtures. The third boundary hides the
taxonomy gap.

## Research architecture

The prompt has four linked centers:

- a finite agreement Core and a typed intent envelope
- a formal intent-refinement relation over plans, traces, and effects
- a certifying or translation-validated compiler to readable Compact
- a complete developer interface that verifies intent before signing

The CAKE framework supplies Application, Permission, Solver, and Settlement
boundaries. Current ERC and EIP standards supply comparative patterns for
orders, signatures, replay protection, permissions, batching, accounts, and
DeFi interfaces. They do not define Moriarty Core.

ACTUS supplies the terminal financial-contract test vector. Pin the public
technical specification, dictionary, reference tests, documentation, and
public repository code. Treat the public test suite as 276 contract fixtures
plus one analysis-date fixture. Require all 277 fixtures without exclusions.

Implement the 18 public executable contract types as typed Moriarty packages.
Do not add one Core constructor per contract type. Add a Core feature only
after an accepted semantic motion proves that a package cannot express the
required behavior.

Classify every one of the 32 taxonomy rows. Use these dispositions:
`implemented-and-vector-tested`, `specified-without-reference-vector`,
`taxonomy-only`, `planned`, `superseded-alias`, or `unavailable`. Do not count
a classified row as an implemented contract.

Compare each produced event trace with every field present in its fixture.
The comparison includes ordered event type, event date, payoff, currency, and
all supplied post-event state fields. Parse JSON numbers and numeric strings
into one exact decimal model. Do not down-cast values to binary32. Permit a
tolerance only when a written numeric policy identifies the field and reason.

Keep risk-factor observations, unscheduled events, child-contract references,
business-day rules, end-of-month rules, day-count rules, event priority,
contract-role signs, settlement currency, and analysis horizon explicit.

## Evidence policy

The assignment requires primary sources, Scrapling acquisition, exact versions,
status labels, preserved receipts, and reproducible experiments. It keeps mock
ZKIR compilation separate from real proof evidence. It keeps proof verification
separate from ledger and intent correspondence.

Scrape the complete public ACTUS site and documentation sitemap. Preserve each
requested URL, effective URL, response, digest, and failure. The documentation
sitemap currently contains 220 placeholder-host URLs. Preserve the original
entries and record the deterministic host substitution used for retrieval.

Pin every public repository in the `actusfrf` organization. Read executable
source and tests before using README claims. Also pin and inspect the public
Marlowe Haskell, PureScript, and Marlowe compiler implementations.

The official Java `actus-core` is an optional comparison oracle. It requires
registration and uses the custom ACTUS Core License. Do not bypass access
controls. Do not make private-core access a release prerequisite.

Publish license and attribution dispositions. The public dictionary,
technical specification, and fixtures use CC BY-SA 4.0. Public service code
uses Apache 2.0. Treat repositories without an explicit license as
rights-unknown until reviewed.

Describe the result as ACTUS reference-vector compatibility. Do not claim
ACTUS certification, endorsement, or conformance without the required written
authorization.

## ACTUS completion predicate

The ACTUS profile passes only when all conditions hold:

1. The source lock identifies the exact dictionary, specification, and fixture
   revisions and digests.
2. The harness discovers exactly 277 fixtures and reports no excluded,
   skipped, quarantined, or expected-failure fixture.
3. Two independent Moriarty semantics implementations accept each valid
   fixture and agree on its complete ordered trace.
4. The generated Compact backend agrees with the reference semantics for each
   fixture's observable projection.
5. Negative mutations fail at the parser, type, applicability, observation,
   transition, payoff, compiler, or backend boundary that owns the rule.
6. The 32-row taxonomy matrix contains one evidence-backed disposition per
   row and does not inflate implementation coverage.
7. The release report uses compatibility language and satisfies license and
   attribution requirements.

Fixture success demonstrates coverage of the pinned suite. It does not prove
all possible contracts, all risk-factor models, legal enforceability, economic
fitness, or official ACTUS certification.

## Delivery shape

Prompt version 1.3 adds an ACTUS workstream, data contracts, experiments,
deliverables, sprints, release gates, red-team challenges, and prohibited
shortcuts. It requires OpenSpec changes, immutable semantic-scope snapshots,
and wiki research-journal entries. It never uses calendar duration as a
progress measure.

## Self-review

- The prompt contains no placeholder.
- The frozen semantic digest matches the active scope index.
- The prompt treats standards as evidence or adapters, not Core authority.
- The intent theorem binds hard predicates and keeps optimization separate.
- The SDK work reconciles the existing 65 components and 28 data contracts.
- The ACTUS profile requires all 277 fixtures without exclusions.
- The taxonomy matrix accounts for all 32 rows without inventing semantics.
- The Java reference core remains optional and access-controlled.
- The public claim is compatibility, not certification or endorsement.
- A library-plus-certifier or verifier-only result remains an explicit outcome.

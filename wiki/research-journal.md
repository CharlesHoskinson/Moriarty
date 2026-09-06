---
id: moriarty.research.journal
type: decision
title: Moriarty research journal
status: active
updated_at: 2026-09-06T08:22:00Z
sources:
  - SRC-0016
  - SRC-0017
  - SRC-0018
  - SRC-0023
  - SRC-0024
  - SRC-0025
  - SRC-0026
  - SRC-0029
  - SRC-0030
  - SRC-0031
  - SRC-0032
  - SRC-0033
  - SRC-0034
  - SRC-0035
---

# Moriarty research journal

This journal records decision rationale, evidence changes, semantic scope, and
the next falsification test. It does not record private chain-of-thought. An
entry is complete only when its referenced evidence is preserved.

## Iteration S00.2: replace calendar progress with evidence gates

- Timestamp: 2026-09-03T07:23:21Z
- Repository base: `006c4d91ed09c0a89261861b6e7203b3efa3e2df`
- State: active

### Trigger

The user rejected calendar-based planning. The user required sprint-only
progress, an iteration-by-iteration semantic scope record, evidence that a
Compact DSL is possible, and a complete SDK specification.

### Decision rationale

Elapsed time is not evidence. Replace day windows with sprint gates. Retain the
same dependency order because proof, backend, and coverage packages still have
real input dependencies.

Treat E00 as a narrow feasibility witness. It proves that one finite
Marlowe-shaped Core slice can generate reviewable Compact and ZKIR artifacts.
It does not prove general compilation or production deployment.

Expand the SDK package from transaction verification to the complete authoring,
compilation, analysis, packaging, deployment, client-verification, and
operations toolchain.

### Active semantic scope

E00 contains `Close`, `Pay`, `If`, `When`, `Deposit`, bounded `Choice`, parties,
tokens, accounts, explicit timeout, payments, warnings, errors, and the atomic
swap specialization. All collections and contract paths are finite.

No later candidate construct is frozen. Attest, action sets, mandate,
conditional-token split or merge, external calls, minting, Merkleized
continuations, modules, and packages remain proposed or outside the E00 Core.

### Evidence state

- The positive Compact program compiled with the pinned toolchain.
- Four ZKIR 3.0 circuits passed the mock compiler.
- One thousand unique differential traces produced zero divergence.
- The negative disclosure fixture failed compilation as required.
- Real proving parameters, keys, proofs, ledger execution, and cost remain open.

### Next falsification test

Reproduce E00 from a fresh pinned environment. Then attempt a second canonical
application through the same generic surface-to-Core-to-Compact contracts. A
special-case generator that cannot support another shape is insufficient
evidence for a language.

## Iteration S00.3: reproduce E00 and close the SDK specification boundary

- Timestamp: 2026-09-03T08:21:33Z
- Reviewed instruction base: `f702692895e8be811fb9eac2a1c0bfafca5dea70`
- State: instruction revision verified; WP01 passed at S3

### Trigger

The user required evidence that a Compact DSL is possible and required the
entire development SDK to be specified. The three-seat Council requested changes
to all twelve initial work-package instruction sets.

### Council evidence

Grok, Sol, and exact Fable 5.1 each reviewed WP01 through WP12 in one frozen
bundle. All three returned `changes_requested`. The review was advisory, not a
security audit. The raw result digests and model terminal evidence are recorded
in `deliverables/moriarty-work-package-council-advisory-2026-09-03.md`.

The Sol result self-labels as `GPT-5`, but the authoritative Codex terminal
banner records `model: gpt-5.6-sol`. The Fable result uses exact
`claude-fable-5-1` and also discloses one 19-output-token Haiku helper call. The
project preserves both details. It does not silently upgrade model evidence.

The assurance scorecard was frozen at SHA-256
`a4557e72ebf50645d4b02c9d2ee3eb778fc9292bd754f71530927351ff6d6be2`.
The terminal decision scorecard was frozen at SHA-256
`42de140b02a7ed1e3982ec90ce27b2581486b938ccfe49b4b6e6dbb0429c279e`.

### Compact DSL evidence change

A fresh `git archive` of Moriarty commit
`006c4d91ed09c0a89261861b6e7203b3efa3e2df` created a new Python environment and
reproduced 43 focused tests. The run reproduced the 1,000-trace certificate,
Compact source, compiler manifest, four ZKIR files, and four binary ZKIR files.
All recorded digests matched. The undeclared-disclosure negative control also
reproduced with exit code 255 and the same full-diagnostic digest.

The run found a new reproducibility footgun. An absolute Compact input path
changed only the source-map and compiler-manifest digest. The recorded relative
command reproduced the expected digest. The SDK must build under a canonical
sandbox path or canonicalize source maps.

Moriarty now performs its own visibility-manifest check. A generated source that
omits `disclose(decision)` fails. A generated source that adds
`disclose(phase)` also fails. This check is independent of the third-party
compiler negative control.

### SDK scope change

WP09 now specifies 65 components. Seventeen form the minimum safety spine and 48
remain specified-only. The component inventory SHA-256 is
`b956125d8ac5e6f8de2cc1f22ca383152a0ac2a3c0969b0e2b2c1e0ca175458c`.

WP09 also specifies 28 canonical data and wire contracts. Their inventory
SHA-256 is
`180baf9647a73cd38ca680205ff9f12ce1b71d560aec9a1b636431446bac3195`.
Machine-readable component and data-contract schemas now bind both inventories.

The prover and proof-parameter provider are explicitly untrusted. Private
witnesses cannot enter an unapproved prover request. Proof parameters bind to
proof system, circuit, network, version, source, digest, and revocation state.
The client verifies proofs and transaction effects before it creates an
immutable signing request.

### Semantic scope transition

No semantic motion was accepted in this iteration.

- Previous version: `0.0.0-e00.1`
- Previous snapshot SHA-256:
  `edc69c1f857ab0465f867c03fe459464a3707529fa20a4fa7a6030caacf30f9a`
- New version: `0.0.0-e00.2`
- New snapshot SHA-256:
  `9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`
- Change kind: evidence-only.
- Reason: clean reproduction evidence changed. Core constructors and semantics did not.

### Decision rationale

E00 now gives reproducible evidence that one finite Moriarty fragment can target
Compact and ZKIR. The result remains S3 because it is one specialized prototype
and uses mock proof compilation. The clean run does not justify a general DSL
claim.

The SDK is complete as a specification boundary, not as an implementation. WP10
and WP11 cannot treat its 48 specified-only components as executable or audited.
They require the 17-component safety spine first.

### Next falsification test

Freeze WP04 semantic motions. Then rerun WP01 against the new scope digest and
compile a non-swap canonical application through shared compiler interfaces. If
the implementation needs another application-specific generator, the evidence
supports Compact libraries or templates, not a Moriarty language.

## Iteration S00.4: close evidence-integrity review findings

- Timestamp: 2026-09-03T08:48:41Z
- State: validator remediation verified; no semantic change

### Trigger

An independent review found four evidence-integrity defects. Compact disclosure
syntax with whitespace could bypass the source check. Compiler metadata was not
compared with the Moriarty manifest. The evidence manifest trusted package,
scope, self-hash, and gate fields too readily. The SDK inventory schemas and
component references were not enforced by the sprint validator.

### Corrections

- The Compact scanner now recognizes `disclose ( value )`. It excludes line
  comments, block comments, and string literals from executable disclosures.
  It rejects nested or compound disclosure expressions that the manifest does
  not name.
- Compiler metadata must match the manifest toolchain versions, exported
  circuits, circuit arguments and results, witnesses, and complete ledger
  schemas.
- The clean pinned-commit receipt and current-checkout validation now use two
  separate evidence streams. The current stream has no commit identity and no
  fresh-archive claim. It binds the remediated source files by SHA-256.
- The sprint validator now checks package identity, sprint identity, semantic
  scope, scope-index digest, output digests, manifest self-hash, and the WP01
  gate predicate. WP02 through WP12 fail closed until their package-specific
  gate validators exist.
- The SDK validator applies both Draft 2020-12 schemas. It requires exactly 65
  components and 28 data contracts, unique identifiers, valid specification
  paths, closed component references, and a fully recomputed SDK index.

The corrected data-contract inventory SHA-256 is
`180baf9647a73cd38ca680205ff9f12ce1b71d560aec9a1b636431446bac3195`.
The component inventory is unchanged at
`b956125d8ac5e6f8de2cc1f22ca383152a0ac2a3c0969b0e2b2c1e0ca175458c`.

### Semantic scope transition

No semantic motion was accepted. Version `0.0.0-e00.2` and snapshot SHA-256
`9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`
remain active. This is an evidence-integrity change only.

### Verification

- The 17 focused evidence and OpenSpec tests passed.
- All 12 OpenSpec changes passed strict validation.
- The instruction validator confirmed 12 packages, 65 components, and 28 data
  contracts.
- The WP01 validator recomputed the package gate and passed.
- The repository suite first passed 72 tests. Two graph tests could not find the
  ignored Marlowe Graphify analysis fixture in the isolated worktree. A
  read-only link to the preserved main-worktree fixture let the complete suite
  pass all 74 tests. The temporary link was then removed.
- The final independent re-audit returned `READY`. It reproduced seven focused
  mutation checks, both validators, and the fail-closed WP02 result.

### Decision rationale

A toolchain exit code is not sufficient evidence for a compiler boundary. The
Moriarty manifest must also agree with the compiler-observed interface. A JSON
`gate_passed` value is not sufficient evidence for a sprint boundary. The gate
must bind the package, semantic scope, source or commit identity, outputs, and
package-specific predicate.

### Next falsification test

Compile a second finite financial application through shared compiler
interfaces. Reject the general-DSL claim if it needs a new application-specific
lowerer. Do not expand the Core until an S03 semantic motion passes.

## Prompt iteration P01: freeze the semantics and intent research question

- State: research assignment specified
- Active semantic scope: `0.0.0-e00.2`
- Scope change: none

### Trigger

The user requested a focused study of Moriarty semantics, specification,
compiler construction, proof shape, correctness of intent, the CAKE framework,
ERC standards, and the standard developer interface.

### Evidence and design rationale

The CAKE framework provides useful Application, Permission, Solver, and
Settlement boundaries. Current ERC and EIP documents provide patterns for
resolver-based orders, typed signing, replay handling, account delegation,
permissions, wallet batching, and DeFi interfaces. These sources are
comparative evidence. They do not define Moriarty Core or Midnight authority.

The new assignment defines intent correctness as a refinement relation. A plan
and execution can be correct only when every observed effect is authorized by
the signed intent under verified state, artifacts, proof, and named assumptions.
Solver optimization stays outside this safety predicate.

### Semantic scope transition

No semantic motion was accepted. The active scope version and digest remain
unchanged. This prompt specifies research and release gates only.

### Next falsification test

Write the candidate intent-refinement judgment for the atomic swap. Mutate the
plan with one extra effect. Require the local verifier to reject the plan before
it creates a signing request.

## Prompt iteration P02: separate the deployed intent lifecycle

- State: primary-source corpus acquired and under synthesis
- Active semantic scope: `0.0.0-e00.2`
- Scope change: none

### Trigger

The user required a deep research pass over CAKE, the complete NEAR Intents
documentation, and current Ethereum intent standards before revising the
intent portion of the assignment.

### Intermediate evidence and rationale

Scrapling acquired all 68 pages in the official NEAR Intents documentation
sitemap, its documentation index, the NEAR overview, and both published
OpenAPI specifications. The deployed NEAR stack distinguishes a quote request,
quote, signed Verifier payload, solver relay, internal-ledger execution,
external bridge withdrawal, fill status, payout status, cancellation request,
refund, and final receipt. These objects cannot safely share one `Intent`
type.

The Verifier's `token_diff` is a useful conservation pattern: all signed token
differences in one batch must sum to zero for each token. It does not establish
cross-chain completion. Cross-contract calls are asynchronous, simulations
exclude their effects, and bridge withdrawals add independent trust and
finality assumptions.

NEAR supplies concrete replay defenses: signer, verifying contract, deadline,
and a 256-bit single-use nonce whose four-byte salt is versioned by the
contract. Its supported signature profiles do not provide one unambiguous
chain identity. The documentation states that the same public key can map
wallets from different chains to an indistinguishable implicit account.

Operational evidence also changes the SDK boundary. Order cancellation is
asynchronous. Fill and payout are separate status dimensions. Guaranteed relay
delivery is at-least-once, requires durable deduplication, retains messages for
at most seven days, and is documented as live but not yet exercised by a
solver. Confidential execution uses a permissioned private NEAR fork, private
relay, treasury-backed assets, and a PoA bridge. These are named trust profiles,
not properties inherited by Moriarty Core.

Current ERC-7683 is resolver-based and materially differs from both its prior
draft and the Open Intents Framework implementation vocabulary. The revised
assignment must pin the current revision and keep objective, quote,
authorization, opaque order, resolved plan, wallet execution, fill, proof,
claim, refund, cancellation, and final settlement distinct.

### Candidate prompt changes

- Add the complete CAKE and NEAR corpora plus version-pinned OIF and CAIP
  sources.
- Define a typed lifecycle with actor, authority, state anchor, idempotency,
  observable data, proof, finality, retry, and cancellation rules on every
  transition.
- Add resolver snapshot and time-of-check/time-of-use obligations.
- Split economic partial fill, output completion, transaction atomicity,
  contiguous wallet execution, cross-chain all-or-refund, and end-to-end
  atomicity.
- Add SDK contracts for quote requests, resolution snapshots, assumptions,
  disclosure, fees, fills, fulfillment, refunds, and transport delivery.
- Require adversarial experiments for duplicate relay events, asynchronous
  cancellation, proof-delay races, bridge rollback, and misleading status.

### Semantic scope transition

No semantic motion is accepted by this research. Version `0.0.0-e00.2` and
snapshot SHA-256
`9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`
remain active. Every new object above is a research or interface requirement
until a later semantic motion assigns it to Core, the intent calculus, an
adapter, Runtime, or application code.

### Next falsification test

Construct one cross-chain swap trace in which the NEAR internal-ledger batch
succeeds but the bridge withdrawal fails or is rolled back. Reject any
candidate `fulfilled` predicate that accepts the internal batch alone.

## Prompt iteration P03: bind lifecycle evidence before authorization

- State: council correction pass completed
- Active semantic scope: `0.0.0-e00.2`
- Scope change: none

### Trigger

The user required Grok, Sol, and Fable to review the intent research prompt
after the CAKE, NEAR Intents, and Ethereum standards research sprint.

### Council evidence and decision

All three providers completed the same frozen blind brief. Grok ran as
`grok-4.6-build`, Sol as `gpt-5.6-sol`, and Fable as canonical
`claude-fable-5-1`; Fable also disclosed a Haiku helper. All three requested
changes. Their common finding was that a sequence of well-named objects is not
yet an assurance chain.

Prompt version 1.2 now requires every adjacent lifecycle artifact to bind its
predecessor digest, execution state, authorization domain, resolver and
contract code identity, upgrade state, assumptions, and version. The candidate
theorem evaluates authorization at execution state and separates
`SignAfterResolve` from `SignBeforeResolve`.

The prompt also indexes settlement evidence from internal-ledger transition to
destination spendability. It makes cancel-or-fill, residual authorization,
refund destination, delivery deduplication, typed reversal, and compensation
explicit. Safety verifiers must return a typed certificate or a structured
rejection; a Boolean cannot authorize signing, submission, claim, or refund.

Confidential execution now has a separate trusted-computing-base and leakage
theorem. Operator, relay, treasury, and bridge attestations cannot discharge a
proof obligation. CAKE, NEAR, current ERC-7683, prior ERC-7683, and OIF must
remain five separate evidence and status rows.

### Semantic scope transition

No semantic motion was accepted. All 38 intent data contracts and new SDK
operations are `specified-only`. They do not modify the frozen 65-component SDK
or 28 canonical data-contract inventory until an accepted OpenSpec change.
Version `0.0.0-e00.2` and snapshot SHA-256
`9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`
remain active.

### Next falsification test

Build the lifecycle refinement-chain harness. Substitute, omit, equivocate, or
stale each adjacent artifact. Then run a cross-domain trace in which the NEAR
internal ledger succeeds but destination payout fails. Release the research
specification only if every false authority or settlement claim is rejected at
the correct boundary.

## Prompt iteration P04: use ACTUS as the terminal completeness vector

- State: public source lock and prompt revision completed
- Active semantic scope: `0.0.0-e00.2`
- Scope change: none

### Trigger

The user required one final prompt iteration that uses ACTUS as the test vector
for Moriarty completeness. The user required the work to scrape the public
documentation, inspect ACTUS repository code, and cover every public reference
contract.

### Evidence and design rationale

Scrapling captured 270 public ACTUS URLs and recorded one explicit 404 for the
documentation site's missing `robots.txt`. The documentation sitemap publishes
220 entries under a placeholder origin. The acquisition replaced only that
origin with `documentation.actusfrf.org`, fetched all 220, and recorded each
substitution. Eleven official repositories and three comparative Marlowe
repositories are pinned in the source manifest.

The pinned dictionary contains 32 taxonomy rows. The dictionary and technical
specification define 18 executable contract types. The public test repository
contains 276 per-contract fixtures plus one analysis-date fixture. These counts
define different obligations: every taxonomy row needs a disposition, while all
277 executable vectors must run without skips or exclusions.

Direct code inspection found that the public Haskell `actus-core` declares 14
of the 18 executable types in its test-facing contract enum, excludes seven
fixtures, does not include the analysis-date file in its suite, compares only
event type, date, and payoff, and downcasts payoff to binary32 `Float`. The
official public service depends on a Java core whose source requires an access
token. The service CI uses a secret for that checkout. The private core is
therefore optional evidence, not a release dependency.

Prompt version 1.3 requires all 18 executable types as typed surface packages
over shared abstractions. Every fixture must pass through the same surface,
canonical Core, and Compact compiler. Two independent semantics must compare
every present ordered result field under explicit decimal, calendar, event, and
observation rules. The prompt forbids product-specific Core constructors,
fixture allowlists, exclusions, hidden expected failures, and certification
language.

### Semantic scope transition

No Core constructor, type, or dynamic rule was accepted. ACTUS starts as a
surface-package and conformance obligation. A minimal expressivity
counterexample must pass the semantic-motion process before it can change the
frozen Core. Version `0.0.0-e00.2` and snapshot SHA-256
`9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`
remain active.

### Next falsification test

Implement the lossless ACTUS fixture importer and two independent schedule and
payoff semantics. Discover all 277 vectors from the pinned corpus. Fail the
language path at the first fixture that needs a product-specific compiler
bypass or an unapproved Core extension. Preserve the minimal counterexample and
its full expected and actual ordered trace.

## Iteration S01: freeze the architecture-neutral intent-safety interface

- Timestamp: 2026-09-04T23:45:58Z
- Repository evidence base: `3c87c3bb914c069153e1d700995f8e16971f1987`
- State: ten local package predicates recomputed and passed at S3; Task 7
  verification and independent review remain open

### Trigger

The user's instruction to execute the full XML version 1.3 assignment
authorized execution of the architecture-neutral S01 plan. This record does
not invent a separate human signoff event.

### Frozen interface and local evidence

**CLM-0132.** S01 froze the terminology, typed observations, hard predicates,
optimization preferences, assumptions, and architecture-neutral intent-safety
judgment. The theorem status is `candidate-unmechanized`. The fail-closed
validator recomputed all ten local package predicates successfully, but that
result is an S3 repository experiment rather than a proof. Source: SRC-0032 at
`evidence-manifest.json`, `intent-safety-judgment.json`, and
`validation-report.json`; observed 2026-09-04 at commit
`3c87c3bb914c069153e1d700995f8e16971f1987`; authority experimental repository
evidence; scope S01 specification and local verifier; evidence repository and
experiment observation; reproduction reproduced; confidence high; lifecycle
status S3.

**CLM-0133.** For the local atomic-swap exact-transfer check, the baseline
result is `VALID` and the extra-effect mutant result is
`UNAUTHORIZED_EXTRA_EFFECT`. Both certificates set
`signing_request_permitted` to false. No production signing request was
attempted or authorized; the mutant therefore failed before signing. Source: SRC-0032 at
`atomic-swap-extra-effect.json`; observed 2026-09-04 at commit
`3c87c3bb914c069153e1d700995f8e16971f1987`; authority experimental repository
evidence; scope the local `SignAfterResolve` transfer checker only; evidence
experiment observation; reproduction reproduced; confidence high; lifecycle
status S3.

**CLM-0134.** The S01 G17 resolution separates two human-team preference pilots
from the automated ACTUS G19-through-G24 benchmark. Recording the resolution
passes neither obligation: no pilot or ACTUS result is established. Source:
SRC-0032 at `ambiguity-resolutions.json` entry `AMB-S01-001`; observed
2026-09-04; authority reviewed specification evidence; scope prompt version 1.3
release-gate interpretation; evidence design decision and open question;
reproduction not applicable; confidence high; lifecycle status S2.

### Semantic-scope transition and S02 handoff

**CLM-0135.** S01 is an evidence-only transition. Semantic scope remains
`0.0.0-e00.2` with SHA-256
`9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`.
It establishes no mechanized theorem, authenticated complete-effect verifier,
runtime `SignBeforeResolve` verifier, Compact or ZKIR correspondence,
proof-system result, backend correspondence, ledger execution, ACTUS
compatibility, or human-pilot result. S02 owns the still-open four-way
architecture selection and will use Quint with Apalache; a direct TLA+/TLC
workflow is excluded. Sources: SRC-0032 at `semantic_scope_version` and
`limitations`, and SRC-0033 under `Execution evidence and package gate`;
observed 2026-09-04; authority experimental repository evidence and reviewed
experiment design; scope S01-to-S02 handoff; evidence experiment observation,
open question, and recommendation; reproduction reproduced for S01 and not
applicable for the unexecuted S02 design; confidence high; lifecycle status S3
for the local S01 verifier and S2 for S02 preparation.

### Next falsification test

Complete Task 7's independent S01 verification and review without converting
the ten local package predicates into any of the 24 prompt release gates. Then
execute the four distinct S02 Quint models against the same frozen interface and
preserve Apalache counterexamples before selecting an architecture.

### Review correction: settlement process and receipt

**CLM-0136.** Whole-S01 review found that the XML term `settlement` had been
aliased to receipt evidence. Commit
`5d3863793660d551b3e30a88e322d5c9a497d33c` adds `SettlementProcess` for the
protocol-specific process that turns fills and proofs into final, spendable
outcomes or an authorized recovery path. `SettlementReceipt` remains evidence
about a settlement step and no longer owns that alias. Regression checks
distinguish the process and evidence categories. The revised manifest is
SRC-0034; historical SRC-0032 is preserved byte-for-byte in the
[pre-correction archive](../raw/repository-observations/2026-09-04-s01-pre-settlement-correction-manifest.json).
Sources: SRC-0031 at XML `required_terminology/settlement`, SRC-0034 at
`terminology.json` and the validation report; correction committed 2026-09-04,
observed 2026-09-05T00:00:31Z; authority normative task input and experimental
repository evidence; scope S01 specification correction with unchanged Core
scope `0.0.0-e00.2`; evidence repository and experiment observation;
reproduction reproduced; confidence high; lifecycle status S3. Independent
re-review and Task 7 completion remain open at this observation.

### S01 completion disposition

**CLM-0137.** Independent re-review approved the corrected S01 code and
normative artifacts at commit `5d3863793660d551b3e30a88e322d5c9a497d33c` with
no remaining material finding. S01's seven tasks are complete as a bounded
specification and local experiment package. The correction and provenance
history are preserved in the [review ledger](../docs/superpowers/reviews/2026-09-04-moriarty-s01-review-ledger.md)
and [final verification record](../docs/superpowers/reviews/2026-09-04-moriarty-s01-final-verification.md).
Source: SRC-0034 at the corrected manifest and its committed artifacts;
observed 2026-09-05; authority experimental repository evidence; scope S01
completion only; evidence repository observation; reproduction reproduced;
confidence high; lifecycle status S3. The theorem remains unmechanized and
both local certificates deny signing. S02 now owns the four-model Quint
comparison; all later proof, backend, ledger, ACTUS, pilot, and release
obligations remain open.

## Iteration S02: record branch-only foundations and resume Council preparation

- Timestamp: 2026-09-05T02:24:03Z
- Main repository base: `5f692a4f0192158c955a95584a146d6703d01d3d`
- State: intermediate branch work exists, but runtime implementation and Council review remain pending

### Intermediate progress and user decision

**CLM-0138.** Repository observation: the S02 effect foundation candidate and its honest
receipt correction are committed on `s02-model-comparison` through
`1bd4bff04fc24855b1c45ff95b1eab137909434d`. The exact-parent consumption
foundation is committed at `8b905114c1cec79faf555974c0267f183ca31399`.
Its honest receipt whitespace correction is committed at
`6a60a645c03acad83b7cbc6b85d43996cd40ca65`. None of these commits is an
ancestor of `main` at this observation. They have no Council approval and do
not complete S02.

User decision: on 2026-09-05 UTC, the user approved the narrow Foreman runtime
binding fixes in the [runtime binding intake](../docs/superpowers/reviews/2026-09-05-council-runtime-binding-intake.md).
The user also approved GitHub publication and merge for those fixes. The user
accepted the documented limits of the official requested-versus-observed model
routes and approved resumption of the Council workflow. The runtime change is
not yet implemented, and no Council member has received the review bundle.

Claim metadata:

- Sources: SRC-0031 at XML S02 and W1, and SRC-0033 at the S02 execution-evidence boundary.
- Local locators: the three full branch commits above, the [runtime binding intake](../docs/superpowers/reviews/2026-09-05-council-runtime-binding-intake.md), and the [Council review requirements](../docs/COUNCIL_REVIEWS.md).
- Commit dates: the branch commits were created on 2026-09-04 MDT. The user decision was recorded on 2026-09-05 UTC.
- Authority: normative task input, local repository observation, and direct user decision.
- Scope: unreleased S02 preparation under semantic scope `0.0.0-e00.2`.
- Evidence kind: repository observation and decision.
- Reproduction: commit presence and branch containment inspected. Branch execution evidence was not reproduced for this journal update.
- Confidence: high for repository containment and the user decision.
- Lifecycle status: S3 for the branch-only foundations and S2 for the pending runtime and Council work.

### Program boundary and next falsification test

Full S02 remains incomplete. No four-candidate comparison, architecture
selection, Council verdict, integration decision, or S02 gate follows from the
two foundations. S03 through S15 also remain incomplete.

Implement and verify the approved narrow runtime binding carrier before Council
dispatch. Then bind the requested and observed model-route evidence to the
review record. Resume Council only after the runtime contract can preserve the
immutable review bytes, ready-token hash, contract hash, prompt hash, and
reviewer identity without post-response stamping.

### Council repair candidate, not gate completion

**CLM-0139.** Repository observation: Foreman now has a Task 1 candidate at
`ac7c2deff6e39144c29836611ee85800ffed41c8` on branch
`foreman/council-binding-20260905/implement/binding`. It adds canonical
ready-token hashing and an immutable review-delivery preparation boundary.
The worker round ended incomplete. Test typing errors and missing report
evidence prevent acceptance. The single-review execution path remains unimplemented.
No reviewed fix has reached GitHub, and no Moriarty Council gate passed.

A correction dispatch stopped before provider execution with the scanner's
`bound_exceeded` result. The wrapper described this as secret material, but
the scanner did not report a secret finding. Removing the unused root
dependency install from the worktree restored the unchanged scan to `clean`.
The install remains recoverable outside the worktree. Council dependencies
remain installed. The original execution contract governs the resumed attempt.

Claim metadata:

- Sources: SRC-0031 at the Council and S02 obligations, and SRC-0033 at the execution-evidence boundary.
- Local locators: Foreman commit above and Moriarty checkpoint fact 26.
- External run receipts: `/home/charl/.foreman/runs/council-binding-20260905/`, including `execution-contract.json` and `secret-scan-open.trace`.
- Repository: `https://github.com/CharlesHoskinson/foreman.git`, default branch `main`, experimental branch identified above.
- Observed at: 2026-09-05T03:07:37Z. Commit date: 2026-09-05 UTC.
- Authority: local repository and execution observations.
- Scope: unreleased Council enablement only. Moriarty Core scope remains `0.0.0-e00.2`.
- Evidence kind: repository observation and experiment observation.
- Reproduction: partially reproduced. The scan result was reproduced, but Task 1 acceptance and Council execution remain pending.
- Confidence: high for the committed candidate and scan disposition.
- Lifecycle status: S3. No release, integration, or Council-approval claim.

### Authorized successor and preserved repair candidate

**CLM-0140.** Repository observation and user decision: the user explicitly
authorized a successor after the predecessor contract escalated. Foreman
accepted `council-binding-20260905-successor1`, which cites the predecessor
and binds a new approval digest. It permits three implementation rounds, one
correction round, and separate bounded verification, audit, Council,
integration, and publication actions. Its deadline is 2026-09-05T09:40:00Z.

The fresh repair branch starts from Foreman
`d85b89598cb4e31bb55f67c0d97074ff4eebe0c7`. It preserves the separate
PID-namespace documentation and replays the five prior repair commits with
an unchanged patch-series range comparison. The preserved candidate is
`748da867073a345881023fd0ff327cb3ce26f99d`. Queue task 1459 was dispatched
to Grok 4.6 for the five known lint failures. The per-lane watchdog was armed.
These dispatch and monitoring records are not a claim of continued liveness,
passing checks, independent approval, or S02 completion.

Claim metadata:

- Sources: SRC-0031 at the Council and S02 obligations, and SRC-0033 at the execution-evidence boundary.
- Local locator: Moriarty checkpoint fact 31.
- External receipts: `/home/charl/.foreman/runs/council-binding-20260905-successor1/execution-contract.json` and `events.jsonl`.
- Contract SHA-256: `63b05cb6fb9a38b3563663cd6446fdae6022db3e9508420d7d37d85abdcd895a`.
- Repository: `https://github.com/CharlesHoskinson/foreman.git`, default branch `main`, candidate branch `foreman/council-binding-20260905-successor1/implement/binding-fresh`.
- Observed at: 2026-09-05T03:43:29Z. Commit and decision date: 2026-09-05 UTC.
- Authority: local repository evidence and explicit user decision.
- Scope: unreleased Council enablement; Core remains `0.0.0-e00.2`.
- Evidence kind: repository observation and decision.
- Reproduction: handoff comparison reproduced; final package verification and reviews pending.
- Confidence: high for authorization, contract creation, and preserved commits.
- Lifecycle status: S3. No GitHub merge or Moriarty Council gate follows.

### Review-runtime recovery checkpoint

**CLM-0141.** Repository observation: the successor's unfinished Task 2 draft
is preserved at `1fb1548d02c98e81ce9b7953a26cca2d2750bdf8`. The Grok event
log records `max_turns_reached` with limit 75 at 2026-09-05T04:19:39.568Z.
This terminal record does not establish a PID-namespace failure. Both worker
reports remained incomplete. The controller archived those reports and the
raw test logs before dispatching recovery task 1462 under the same successor
contract's resume allowance. No verification, Council, or release approval
follows from this recovery checkpoint.

Claim metadata:

- Sources: SRC-0031 at the Council obligations and SRC-0033 at the execution-evidence boundary.
- Local locator: Moriarty checkpoint fact 35.
- External receipts: `/home/charl/.foreman/runs/council-binding-20260905-successor1/task2-attempt1-receipts/` and `task2-resume1.md` in the same run directory.
- Repository: `https://github.com/CharlesHoskinson/foreman.git`, default branch `main`, candidate branch `foreman/council-binding-20260905-successor1/implement/binding-fresh`, full commit above.
- Observed at: 2026-09-05T04:39:00Z. Commit date: 2026-09-05 UTC.
- Authority: local repository and vendor terminal metadata.
- Scope: unreleased Council enablement; Core remains `0.0.0-e00.2`.
- Evidence kind: repository observation.
- Reproduction: partially reproduced; recovery and final acceptance remain pending.
- Confidence: high for preserved commit, stop metadata, and dispatch.
- Lifecycle status: S3. No Moriarty Council gate or GitHub merge is claimed.

### S02 alternative coverage before model implementation

**CLM-0142.** Repository observation: XML v1.3 requires all four semantic
alternatives before selection or an evidence-backed stop. The S02 branch
contains shared effects and exact-parent consumption foundations, not these
candidate implementations. All four candidates remain undetermined. Missing
implementation is not a decisive counterexample against a candidate.

| Alternative | Required distinct execution mechanism | Candidate-specific acceptance focus |
| --- | --- | --- |
| A: agreement Core plus intent envelope | Agreement interpreter and independent envelope predicate | Timeout priority, rollback, complete effects, and separate legality and authorization |
| B: intent Core with agreement libraries | Native obligation graph plus separately checked agreement elaboration | Dependency removal and corrupted elaboration must expose failures |
| C: two calculi with refinement bridge | Independently computed successors plus paired commitment | Corrupted or stale bridges and one-sided advancement must fail |
| D: Compact library with local verifier | Application calls plus actual-effect extraction and verification | Artifact substitution and extraction corruption must fail |

Each candidate needs the swap and two-installment workloads under both
signing profiles. The closed registry has eleven witness identifiers. The
design's phrase “all ten registry scenarios” is stale and must not remove a
witness. Every candidate and both profiles also need the separate recovery
paths before any fill and after the first fill. The existing cancellation
bookkeeping does not supply that recovery evidence.

The corrected observation, authorization, and recovery carrier remains a
proposal awaiting explicit signoff. No new Quint logic follows from this
coverage table. Frozen Core remains `0.0.0-e00.2`.

Claim metadata:

- Sources: SRC-0031 at XML workstream W1 and sprint S02, and SRC-0033 at the frozen semantic boundary.
- Local locators: [reviewed comparison design](../docs/superpowers/specs/2026-09-04-moriarty-s02-model-comparison-design.md), [closed registry](../evidence/s02-model-comparison/requirements.json), and [corrected carrier proposal](../docs/superpowers/specs/2026-09-05-moriarty-s02-authorization-recovery-types.md).
- Repository: local Moriarty repository, branch `s02-model-comparison`, commit `6a60a645c03acad83b7cbc6b85d43996cd40ca65`. Corrected main-branch carrier: commit `14b16f3ccd98c50070f7b18c2a608d38a8b29870`.
- Observed at: 2026-09-05T04:56:00Z. Controlling design date: 2026-09-04 UTC.
- Authority: controlling XML and reviewed local design.
- Scope: proposed S02 candidate acceptance, not Core changes or deployed behavior.
- Evidence kind: repository observation and open implementation obligation.
- Reproduction: not applicable to this source comparison. Candidate experiments remain unperformed.
- Confidence: high for the inspected requirements and absent candidate implementations.
- Lifecycle status: S2 for the candidate designs. The shared experimental foundations do not raise candidate status.

### Moriarty-only workflow reset

**CLM-0143.** User decision and repository observation: Foreman development is
closed for this workstream. The fetched Foreman main already contains PR 55 at
`48f0b6eaed0eb25ee04d053317626eb768b60725`. The separate repair branch remains
preserved, not accepted or merged by this reset. Its unfinished work is no longer
a blanket prerequisite for authoring Moriarty models.

The [restart assessment](../docs/MORIARTY_RESTART.md) resumes XML S02 from the
existing effects and consumption foundations. Requested Council acceptance
reviews remain outstanding. The user subsequently delegated type-sketch design
signoff to GPT-6 Astra, Fable 5.1, and Grok 4.6 as formal-methods experts.
Their decision remains outstanding. Neither obligation authorizes renewed Foreman
development, and this reset selects no semantic alternative.

Claim metadata:

- Sources: SRC-0031 at XML S02 and the latest explicit user scope instruction.
- Local locators: restart assessment, checkpoint fact 41, and obligations 2, 7, 9.
- Repository: local Moriarty, baseline main `e37dde97847b9300b9f82c0a13f91885397088fd`, foundation branch `s02-model-comparison` at `6a60a645c03acad83b7cbc6b85d43996cd40ca65`.
- Foreman repository: `https://github.com/CharlesHoskinson/foreman.git`, default branch `main`, full commit above.
- Observed and decision date: 2026-09-05 UTC.
- Authority: user scope decision and local repository inspection.
- Scope: S02 restart, unchanged Core `0.0.0-e00.2`.
- Evidence kind: repository observation and decision.
- Reproduction: repository synchronization reproduced; candidate implementation remains unperformed.
- Confidence: high for scope and repository state.
- Lifecycle status: S2 for the restart plan, not candidate implementation or release.

The user's continuation instruction covers the complete XML program, not S02
alone. The app goal was initially paused. The user resumed it, and a subsequent
goal-tool inspection confirmed `active`. This restores product-owned continuation,
not evidence that any sprint or release gate is complete.

### Delegated S02 common-foundation design

**CLM-0144.** Repository observation and design recommendation: all three requested
experts returned independent amendments against identical frozen inline sources.
The [decision](../docs/superpowers/specs/2026-09-05-moriarty-s02-common-design-decision.md)
adopts a combined foundation by two-to-one preference while preserving Fable's
split-review dissent. Astra and Fable support consumed-parent authority with
explicit exact-parent residual applicability; Grok's registered-through-residual
alternative is preserved but not selected. All agree that the current sketch
needs correction and that neither architecture selection nor implementation
acceptance follows from their advice.

The design adds branch policies, optional Core projections, explicit
operation-specific authorization, per-key and per-operation lifecycle records,
current evidence checks, atomic registry/financial updates, and independently
signed recovery. The complete disposition table also records disagreements about
time, freshness, policy supersession, display binding, and terminal stuttering.
No unanimity, actual model counterexample, or completed Council gate is claimed.

Claim metadata:

- Sources: SRC-0035 public proposal receipt; SRC-0031 XML S02; SRC-0033 frozen comparison design.
- Local locator: [immutable proposals and receipt](../raw/reviews/s02-common-design-2026-09-05/receipt.json).
- Repository: local Moriarty main source `76228d99960a78aba052aa481565d06b7a1762db`; foundation source `6a60a645c03acad83b7cbc6b85d43996cd40ca65`.
- Observed and decision date: 2026-09-05 UTC.
- Authority: experimental model advice and user-delegated local design decision.
- Scope: common S02 foundation, unchanged Core `0.0.0-e00.2`.
- Evidence kind: repository observation and recommendation.
- Reproduction: not applicable to source-level design advice; no model tests were run by the proposal authors.
- Confidence: high for receipt contents; proposed correctness remains to be tested.
- Lifecycle status: S2. A–D implementations and all applicable gates remain open.

The Astra route used the explicitly selected native model after its CLI credential
failed before proposing. Fable metadata reports the requested canonical model
plus auxiliary Haiku usage; Grok reports the requested alias's `grok-4.6-build`
route. These observations are not model-identity attestations or Council admission.

### First implemented unit from the delegated design

**CLM-0145.** Experiment observation: the S02 branch now contains the neutral
observation carrier, structural/emission predicates, concrete example harness,
and deterministic tests. The reviewed correction is committed at
`45883ba949f18f580b14c6cd17303b719ef8f700`. The current receipt reports 37 passing
observation tests and both declared-example witnesses in 1,000 sampled traces.
These observations check supplied examples; they do not execute a candidate,
authorize effects, refund escrow, or establish Core correspondence.

Test-first construction preserved fail-closed stubs and failing positive tests.
Self-review added failing cases for wallet-to-wallet effects and multiple deposits.
An independent GPT-6 source review then found the missing frozen-Core
nonnegative-time constraint and a public rollback coverage gap. Three additional
negative-time tests failed before the guard correction. The final tests cover
that guard and all rollback dimensions through the public validator. The subsequent
narrow re-review confirmed both findings resolved and all correction source/receipt
pins matching, with no new issue in the delta. This remains one source review,
not a Council gate.

Claim metadata:

- Sources: SRC-0035 delegated design; SRC-0033 comparison boundary; frozen Core source at the branch commit below.
- Repository: local Moriarty, branch `s02-model-comparison`, commit `45883ba949f18f580b14c6cd17303b719ef8f700`; worktree `/home/charl/Moriarty/.worktrees/s01-audit-start`.
- Local locators on that branch: `specs/quint/s02/observations.qnt`, `observations_harness.qnt`, `observations_test.qnt`, and `evidence/s02-model-comparison/observations/current.json`.
- Evidence: the current pointer selects `minimum-time-correction/manifest.json`; initial receipts and red commits remain preserved.
- Observed and commit date: 2026-09-05 UTC.
- Authority: local executable tests, sampled structural checks, and independent source review.
- Scope: first common-foundation unit; unchanged Core `0.0.0-e00.2`.
- Evidence kind: experiment observation.
- Reproduction: reproduced at the pinned content; raw command output and exact source hashes are preserved on the branch.
- Confidence: high for the recorded outcomes, not for unperformed correspondence or authorization work.
- Lifecycle status: S3 experimental branch, not main-integrated implementation or release.

Next implement branch policies and neutral plan views, followed by signing,
execution verification, atomic parent transitions, and financial recovery under
both profiles. All four candidate representations, final model checking, S01
Council backfill, S03–S15, and all 24 XML release gates remain open.

### Branch policies implemented and independently reviewed

**CLM-0146.** Experiment observation: S02 branch commit `320dc53` implements
unsigned branch policies and complete neutral plan bindings. The pinned receipts
report 72 passing Quint policy tests, with effects/consumption/observation
regressions of 10/16/37, 286 Python tests, and ten local S01 checks passing.
The preceding fail-closed RED commit `07bf706` preserves five expected assertion
failures. These measurements concern the pinned code, not unperformed lifecycle
execution or exhaustive model checking.

The policies enforce input-based settlement/refund alternatives, all debit-owner
requirements, effects-free cancellation authority, both signing-profile bounds,
complete after-resolution operation matching, and recovery amount constraints.
Neutral context facts and plan views still require authentic derivation by later
lifecycle code and candidate adapters. No signature, atomic commit, financial
recovery trace, Core correspondence, or A–D candidate is implemented by this unit.

An independent native GPT-6 Astra review found no actionable correctness issue
within this scope and confirmed all fourteen manifest pins. A focused TypeScript
REPL probe accepted the correct second-fill context and rejected its predecessor;
an initial Rust REPL probe failed with a loader assertion and is not positive
evidence. The existing Rust test receipts are the test-count evidence. This is
one implementation review, not the requested three-vendor Council gate.

Separately, commit `7cccc5a` adds both mandatory recovery subscenarios to the
closed requirements registry without fabricating traces. Its independent
mechanical transcription review was clean. The registry remains specified-only.

Claim metadata:

- Sources: SRC-0035 adopted design; SRC-0033 comparison boundary.
- Repository: local Moriarty, branch `s02-model-comparison`, implementation `320dc53`, reviewed handoff `a6f9c30`; worktree `/home/charl/Moriarty/.worktrees/s01-audit-start`.
- Local locators: `specs/quint/s02/policies.qnt`, `policies_harness.qnt`, `policies_test.qnt`, and `evidence/s02-model-comparison/policies/manifest.json`.
- Review: `docs/superpowers/reviews/2026-09-05-moriarty-s02-policy-branches.md` on that branch.
- Observed and commit date: 2026-09-05 UTC.
- Authority: local deterministic execution and independent source review.
- Scope: pure policy unit; unchanged frozen Core `0.0.0-e00.2`; Alice-only installment/recovery debit fixture.
- Evidence kind: experiment observation.
- Reproduction: raw outputs and exact source/receipt digests preserved at the pinned commit.
- Confidence: high for recorded outcomes; no claim for unimplemented lifecycle or correspondence.
- Lifecycle status: S3 experimental branch, not main-integrated implementation or release.

Next execute `docs/superpowers/plans/2026-09-05-moriarty-s02-authority-lifecycle.md`:
persistent signing, fresh evidence, and atomic fill/cancel/recovery transitions.
The full XML goal remains active. Foreman repairs remain closed; neither the
finished design proposals nor the completed observation/policy units need restart.

### Persistent symbolic signing executes under both profiles

**CLM-0147.** Experiment observation: S02 branch `cbd1f1d`, corrected at `f2941d4`,
implements persistent symbolic signing with exact checked policy content and
key-local freshness snapshots. Alice and Bob can prepare concurrent checks and
register independently. Relevant state, ledger, environment, own-key, and full
parent dependencies invalidate stale checks. Parent registration retains the
exact signed policy in unclaimed accounting without consuming its nonce or
moving money.

Corrected pinned receipts report 41 passing Quint tests and 1,000 sampled signing
traces: 533 after-resolution and 467 before-resolution. Every trace reaches both
signatures. The signing-only invariant checks unchanged money, structural
registry/parent coherence, nonterminal enabledness, and no further signing action
once both signatures exist. The broader implementation run preserved 72 policy,
10 effect, 16 consumption, 37 observation, and 286 Python test passes, plus the
ten local S01 checks. Those counts describe their pinned runs, not current main
implementation or a complete S02 gate.

Independent native GPT-6 Astra review found one nonblocking admission issue:
unsupported Bob/Mallory parent policies could prepare unusable checks. A new
failing test and early preparation guard resolve it; narrow re-review is clean.
The initial RED correction command yielded before its assertion failure was
collected, and that timing is explicitly disclosed in the receipt. Original
manifests and the pre-correction source commit remain preserved.

Claim metadata:

- Sources: SRC-0035 adopted design; SRC-0033 comparison boundary.
- Repository: local Moriarty, branch `s02-model-comparison`, corrected implementation `f2941d4`; worktree `/home/charl/Moriarty/.worktrees/s01-audit-start`.
- Local locators: `specs/quint/s02/authorization.qnt`, `authorization_harness.qnt`, `authorization_test.qnt`, and `evidence/s02-model-comparison/authorization/current.json`.
- Review: `docs/superpowers/reviews/2026-09-05-moriarty-s02-persistent-signing.md` on the branch.
- Observed and commit date: 2026-09-05 UTC.
- Authority: local deterministic tests, sampled signing paths, independent source review and correction re-review.
- Scope: symbolic signing only; unchanged frozen Core; Alice-only parent fixture.
- Evidence kind: experiment observation.
- Reproduction: exact source/receipt pins, RED commits, and raw outputs preserved.
- Confidence: high for recorded outcomes, not unperformed financial or correspondence work.
- Lifecycle status: S3 experimental branch, not main-integrated implementation or release.

No execution evidence verifier, atomic fill/cancel/recovery commit, candidate
A–D semantics, or Core correspondence follows from this signing unit. Constructed
cancelled contexts in guard tests are not cancellation traces. Next implement
per-operation verification and the common commit boundary. Council gates and the
full XML program remain open; Foreman development remains closed.

### Verified execution envelope reaches atomic settlement

**CLM-0148.** Experiment observation: branch source `46fe589` and evidence
`0d5926b` implement full per-operation evidence bindings and atomic execution.
The concrete swap harness starts with prefunded escrow and unsigned policies,
then executes check, sign, propose, verify, and commit under both profiles.
Commit rechecks the actual current context immediately before one atomic update
of financial effects, authority consumption, candidate state, and attempt status.

Pinned receipts report eight pipeline tests, forty-two separately authored
adversarial tests, and one thousand sampled executions. Every sampled trace
reaches settlement: 494 after-resolution and 506 before-resolution. Invariants
check conservation, exact final money and consumed authorities, nonterminal
enabledness, and no enabled action after settlement. Foundation regressions
10/16/37/72/41, Python 286, and ten local S01 checks also pass at their pinned
runs. These measurements are not claims about unperformed candidate execution.

The separate test author exposed accepted Core projection on effects-free
cancellation. Its failing test is preserved; the corrected guard requires
`NoCoreProjection`. Independent native Astra source and evidence review is clean,
with all forty-seven manifest pins matching. The reviewer additionally checked
constructed parent update results; those checks are not parent lifecycle traces.

Claim metadata:

- Sources: SRC-0035 adopted design; SRC-0033 comparison boundary.
- Repository: local Moriarty, branch `s02-model-comparison`, source `46fe589`, evidence `0d5926b`; worktree `/home/charl/Moriarty/.worktrees/s01-audit-start`.
- Local locators: `specs/quint/s02/execution.qnt`, `execution_harness.qnt`, `execution_test.qnt`, `execution_adversarial_test.qnt`, and `evidence/s02-model-comparison/execution/manifest.json`.
- Review: `docs/superpowers/reviews/2026-09-05-moriarty-s02-execution-envelope.md` on that branch.
- Observed and commit date: 2026-09-05 UTC.
- Authority: deterministic tests, sampled envelope execution, independent source/evidence review.
- Scope: common execution envelope with trusted external evidence dispositions; prefunded swap settlement; unchanged frozen Core.
- Evidence kind: experiment observation.
- Reproduction: source/receipt hashes and complete adversarial RED/GREEN report preserved. The initially misstated RED command was corrected transparently; the author's lost standalone typecheck result is not counted as success.
- Confidence: high for recorded envelope results, not cryptography, candidate semantics, or correspondence.
- Lifecycle status: S3 experimental branch, not main-integrated implementation or release.

`EvidenceValid` is an abstract verifier result, not a candidate interpreter or
cryptographic proof. Generic parent commit helpers exist, but actual stateful
fill/cancel races, rejected-attempt handling, and signed recovery paths remain
next. S02, its Council gates, A–D comparison, and the full XML program stay open.

### Rejected attempts retain evidence without moving funds

**CLM-0149.** Experiment observation: branch source `38cf13d` and evidence
`4f3bb75` implement explicit rejection at verification and commit boundaries.
Each rejection retains the original attempt, supplied evidence, current context,
derived reason, and boundary stage. Rejection changes only the attempt cell;
it cannot reverse a winning commit or count as financial settlement.

Pinned corrected runs report 21 rejection tests, two rejection pipeline tests,
and 1,000 sampled traces from a constructed signed swap context. There are 504
missing-proof rejections and 496 verified attempts followed by an anchor change
and stale-context rejection. The invariant checks preserved money, registry,
parent accounting, candidate state, and exact nonterminal enabledness. Standalone
typecheck, eight settlement regressions, 42 adversarial regressions, 286 Python
tests, and ten local S01 checks passed in these branch runs.

Independent native Astra source review found one diagnostic issue: an accepted
Core projection hid a consumed-slot conflict behind a generic unauthorized
label. The failing regression, correction, and clean narrow re-review are
preserved. Final independent evidence review checked all 25 pins, including
13 Quint sources matching the source commit, with no discrepancy. These reviews
are scoped native checks, not Council acceptance.

Claim metadata:

- Sources: SRC-0035 adopted design; SRC-0033 comparison boundary.
- Repository: local Moriarty, branch `s02-model-comparison`, source `38cf13d`, evidence `4f3bb75`; worktree `/home/charl/Moriarty/.worktrees/s01-audit-start`.
- Local locators: `specs/quint/s02/execution.qnt`, `rejection_test.qnt`, `rejection_harness.qnt`, `rejection_pipeline_test.qnt`, and `evidence/s02-model-comparison/rejection/manifest.json`.
- Review: `docs/superpowers/reviews/2026-09-05-moriarty-s02-rejected-attempts.md` on that branch.
- Observed and commit date: 2026-09-05 UTC.
- Authority: deterministic tests, sampled rejection traces, independent source review and correction re-review.
- Scope: common rejected-attempt records; trusted external evidence; constructed signed swap base; unchanged frozen Core.
- Evidence kind: experiment observation.
- Reproduction: source and receipt pins with complete terminal outputs; disabled-scaffold RED at `8d78662` and classifier RED at `1a23366`.
- Confidence: high for these recorded results, not unperformed candidate execution, cryptography, or exhaustive checking.
- Lifecycle status: S3 experimental branch, not main-integrated implementation or release.

The parallel installment harness remains unfinished at this checkpoint. Its
initial signing/race setup is development work, not full lifecycle evidence.
Continue actual race winners, rejected losers, second-fill completion, and
separately signed recoveries of ten or five. No S02, Council, A–D selection, or
XML release gate is closed. Foreman development remains closed.

### Installment races reach payment or separately signed recovery

**CLM-0150.** Experiment observation: common installment implementation `8d8e8fe`,
independent tests `d1c475f`, and evidence `f4c7bc5` now execute the parent lifecycle
under both signing profiles. Initial state is unsigned and prefunded. Both
first-fill and cancellation attempts must be verified against the same current
revision before either commits. The winner changes financial/authority state
atomically; the loser retains an explicit stale rejection.

The remaining path either completes slot two through the exact consumed-parent
residual or separately checks, signs, verifies, and commits nonce-one recovery.
Cancellation does not refund funds by itself. Recovery reads actual escrow while
the policy independently pins the expected ten or five, and it preserves the
cancelled parent and nonce-zero history.

| Terminal path | Alice refund | Bob payment | Escrow | Root sampled traces |
| --- | ---: | ---: | ---: | ---: |
| Both installments complete | 0 | 10 | 0 | 253 |
| Cancellation wins, recovery ten | 10 | 0 | 0 | 512 |
| First fill wins, fresh cancellation, recovery five | 5 | 5 | 0 | 235 |

The root's 1,000-trace run reached all 28 witnesses and all six profile/outcome
combinations without an `installmentSafety` violation. Root also reran the 12
lifecycle tests and nine separately authored negative tests. Fresh repository
regressions report 286 Python tests and ten local S01 checks. Independent source,
test-semantic, and evidence review is clean: all 18 pins match, including eleven
Quint sources at the recorded source commit.

Claim metadata:

- Sources: SRC-0035 adopted design; SRC-0033 comparison boundary.
- Repository: local Moriarty, branch `s02-model-comparison`, source `d1c475f`, evidence `f4c7bc5`; worktree `/home/charl/Moriarty/.worktrees/s01-audit-start`.
- Local locators: `specs/quint/s02/installment_fixtures.qnt`, `installment_harness.qnt`, `installment_test.qnt`, `installment_adversarial_test.qnt`, and `evidence/s02-model-comparison/installment/manifest.json`.
- Review: `docs/superpowers/reviews/2026-09-05-moriarty-s02-installment-lifecycle.md` on that branch.
- Observed and commit date: 2026-09-05 UTC.
- Authority: separate implementation/test authors, independent root tests and sampling, independent source and evidence review.
- Scope: common lifecycle, fixed time two, prefunded escrow, trusted external-verifier dispositions, unchanged frozen Core.
- Evidence kind: experiment observation.
- Reproduction: all current sources and complete final receipts pinned. Development RED reports are not an archived scaffold-source history; that limit and test-construction corrections are disclosed.
- Confidence: high for recorded bounded results, not exhaustive checking or semantic correspondence.
- Lifecycle status: S3 experimental branch, not main-integrated implementation or release.

This fixture races initial fill/cancellation, then selects one successor branch;
it does not test a second concurrent slot-two/cancellation race. Constructed
refund mutations are guard checks, not the full S02 negative-control inventory.
Candidate A–D interpreters, independent Core correspondence, Quint/Apalache
checking, selection, Council acceptance, and all broader XML gates remain open.
The next semantic implementation is Candidate A's agreement interpreter, not a
shared validity flag standing in for candidate execution.

### Candidate A begins executing the frozen agreement semantics

**CLM-0151.** Experiment observation: source commit
`068b7cdd6d15bbb28659f56c062c8f93bccd290a` implements Candidate A's finite
program/state/input domain, literal canonical swap node table, and exact one-step
Close/Pay interpreter. Close uses canonical first-positive-account refund order.
Pay preserves the frozen partial/nonpositive warnings, paid quantity, remaining
accounts, continuation, and reduction count. Valid If/When nodes remain explicitly
unavailable in this increment; malformed-domain diagnostics are not Core errors.

Root verification at that source reports 26 Quint tests and 305 Python tests
passing, including 19 new independent complete-result reference vectors. Those
vectors cover selected rollback, deadline, warning, choice ordering, and deposit
insertion boundaries; they do not compare Python against Quint yet. Separate
nonauthor native source reviews found no actionable issues in their scoped units.
The two development stages preserve exact failing source import closures and
author terminal outputs, rather than relying on a narrative of earlier failures.

Claim metadata:

- Sources: SRC-0031 XML S02; SRC-0033 comparison boundary; SRC-0035 adopted common design.
- Repository: local Moriarty, branch `s02-model-comparison`, source commit above, evidence `982bdd8db4ab42b9bd9c40e451cdb283a94201ee`.
- Local locators on that branch: `specs/quint/s02/candidate_a_core.qnt`, `candidate_a_types.qnt`, `candidate_a_programs.qnt`, `candidate_a_core_test.qnt`, `tests/test_s02_candidate_a_reference_vectors.py`, and `evidence/s02-model-comparison/candidate-a-close-pay/manifest.json`.
- Plan and review: `docs/superpowers/plans/2026-09-05-moriarty-s02-candidate-a-core.md` and `docs/superpowers/reviews/2026-09-05-moriarty-s02-candidate-a-close-pay.md` on that branch.
- Observed and commit date: 2026-09-05 UTC.
- Authority: deterministic author tests, independent root reruns, scoped nonauthor native source reviews.
- Scope: Candidate A plan Tasks 1–2; frozen Core unchanged; no stateful candidate trace yet.
- Evidence kind: experiment observation.
- Reproduction: complete root receipts, source pins, and both RED closures preserved.
- Confidence: high for recorded deterministic results, not unperformed correspondence or model checking.
- Lifecycle status: S3 experimental branch, not main-integrated implementation or release.

Task 3 is assigned next: If/When semantics, bounded quiescence, and ordered input
application. Full transaction rollback, candidate authority integration, A–D
comparison, exact-provider Council acceptance, and all broader XML gates remain
open. The product goal is active; no uninterrupted-runtime or morning-completion
guarantee is inferred from that status. Foreman development remains closed.

### Candidate A executes control flow and ordered input application

**CLM-0152.** Experiment observation: source
`d3f5dd6e60ca937dfc4840d34204bf00436bdc48` implements the frozen If/When
reductions, ordered input scan, and fixed 22-call reduction-to-quiescence fold.
Every admitted constructor now has implemented reduction behavior. Optional
choices preserve absence versus zero; deadline equality selects timeout before
input application. A bounds mismatch does not prevent a later matching case
from accepting, and nonpositive deposits require exact identity/quantity match.
Finite-domain and internal-precondition diagnostics remain distinct from Core
errors; whole-transaction evaluation is the next unit.

Root receipts record 52 core tests, four independently authored boundary tests,
and 327 Python tests passing. The maximum-path test executes 15 descending
non-Close reductions and six canonical refunds before quiescence on call 22.
It is a boundary witness, not exhaustive proof over the admitted program space.
The separate boundary suite also tests maximum-deposit potential preservation
and actual-clock retention. A wrong test expectation omitted the terminal Close
refund after Pay; only the test was corrected, with no product defect inferred.

The 22 new independent Python workload tests at `c8cedc1` exercise the proposed
two-When installment tree, separate five-unit fills, ten/five owner refunds,
deadline commit/reject pairs, and complete invalid-request rollback. They import
only frozen Core constructors/evaluation, not the Quint evaluator or any authority
layer. Agreement-legal recovery does not establish authorization to recover.

Claim metadata:

- Sources: SRC-0031 XML S02; SRC-0033 comparison boundary; SRC-0035 adopted common design.
- Repository: local Moriarty, branch `s02-model-comparison`, source commit above, evidence `a0fb15f9a3d92c67cdac214bbd87e57ce788adfe`.
- Local locators on that branch: `specs/quint/s02/candidate_a_core.qnt`, `candidate_a_types.qnt`, `candidate_a_core_test.qnt`, `candidate_a_boundary_test.qnt`, `tests/test_s02_candidate_a_installment_reference.py`, and `evidence/s02-model-comparison/candidate-a-control-flow/manifest.json`.
- Review: `docs/superpowers/reviews/2026-09-05-moriarty-s02-candidate-a-control-flow.md` on that branch.
- Observed and commit date: 2026-09-05 UTC.
- Authority: deterministic author tests, independent root reruns, scoped nonauthor native source reviews.
- Scope: Candidate A plan Task 3 and selected independent Python workload vectors; frozen Core unchanged.
- Evidence kind: experiment observation.
- Reproduction: primary-author RED six-file import closure and exact root final receipts preserved; boundary-author original RED test source/full raw output not archived, diagnostic excerpt and correction disclosed.
- Confidence: high for the recorded bounded results, not unperformed correspondence or exhaustive checking.
- Lifecycle status: S3 experimental branch, not main-integrated implementation or release.

Task 4 is assigned: full transaction source ordering, exact rejection rollback,
and before/input/after evaluation. Stateful candidate traces, neutral projections,
independent correspondence, A–D comparison and selection, exact-provider Council
acceptance, and all broader XML gates remain open. Continue Moriarty; do not
reopen Foreman repairs or substitute these native reviews for Council acceptance.

### Candidate A transactions, projections, swap and installment execution

**CLM-0153.** Experiment observation: transaction source
`ea35cada6bf84a016ec568834242628ee8ccfa76` implements the full frozen decision
tree and complete rejection rollback. Projection source
`d4a714ce6013e8237ff9ef07b6b661c52a421e8c` exports every result field and derives
ordered effects, including the deposit between pre-input and post-input payments.
Root receipts at those milestones record 75 core tests, four boundary tests,
327 Python tests, and 26 projection tests passing. Separate native evidence audits
verified all 23 transaction and 34 projection pins; those audits did not rerun tests.

**CLM-0154.** Experiment observation: swap harness source
`3c648ec50c7eb63fd4fc78742846a722446f0892` begins with an empty agreement and
real funding wallets. Actual transactions produce deposits, settlement, voluntary
refunds, deadline cleanup and retained deadline-input rejection. Root independently
ran 19 tests and 1,000 bounded Rust traces; all action witnesses were positive
and every sampled trace reached terminal. There is no unconditional stutter or
initializer-selected financial outcome. Diagnostics fail safety and are not Core
rejections. Those local checks recompute the producer; they are not an independent
semantic oracle or exhaustive verification.

**CLM-0155.** Experiment observation: installment source
`c002a8417f668c44fa8aa76ec5d5f31f6758c422` executes two separate five-unit fills
through distinct When boundaries and actual ten/five-unit recovery refunds.
Root independently ran 23 installment tests, 19 swap regression tests and 1,000
bounded installment traces with every action witness positive and every trace
terminal. A rejected recovery input at the deadline is retained before a real
NoInput cleanup. The separate cancellation adapter retains the agreement, emits
no effects and supplies NoCoreProjection; it does not fabricate a Core result.
The initial minimum time is two, not the Python workload fixture's one; later
correspondence must evaluate the actual exported before-state.

Shared claim metadata for CLM-0153 through CLM-0155:

- Sources: SRC-0031 XML S02; SRC-0033 comparison boundary; SRC-0035 adopted design.
- Repository: local Moriarty, branch `s02-model-comparison`; exact source commits above.
- Evidence commits: transaction `18d8a5ca0ef5fe1434306e4a2a3b45ea744e1c9d`; projection `4dfdb388d5129c5fa730dd7ef0458017bd0a484c`; harnesses `a4677cd985a540d9c560de2ad7d882495fc92b78`, archive audit `2b04d23e4ca33e30ce00ba52cc889a877c606e3f`.
- Local locators on that branch: `evidence/s02-model-comparison/candidate-a-transactions/manifest.json`, `candidate-a-projection/manifest.json`, `candidate-a-swap/manifest.json`, and `candidate-a-installment/manifest.json`.
- Reviews: `docs/superpowers/reviews/2026-09-05-moriarty-s02-candidate-a-transactions-projection-evidence.md`, `2026-09-05-moriarty-s02-candidate-a-swap.md`, `2026-09-05-moriarty-s02-candidate-a-installment.md`, and `2026-09-05-moriarty-s02-candidate-a-harness-evidence.md` on that branch.
- Observed and commit date: 2026-09-05 UTC; updated observation 11:39:46 UTC.
- Authority: author tests, nonauthor root execution/source inspection, scoped native reviews. The harness author audited root-created archives, not its own source independently.
- Scope: Candidate A plan Tasks 4–6, agreement-only semantics and projections; frozen Core/swap unchanged.
- Evidence kind: experiment observation and repository observation.
- Reproduction: exact root receipts, typed RED closures, author reports and six byte-identical raw harness ITFs preserved. All 58 harness manifest pins verified; a tool-version locator was clarified without changing author receipts.
- Confidence: high for recorded bounded results; no unperformed correspondence or exhaustive checking claim.
- Lifecycle status: S3 experimental branch, not main-integrated implementation or release.

Task 7 remains in progress: a serialization-only exporter and independently
implemented Python checker must compare actual raw results, projections and
ordered effects. Early review found checker gaps in residual continuation,
duplicate sparse-map keys, exact node identity and raw provenance linkage;
the required regression controls and final actual-corpus comparison are not yet
complete. No S02 or Council gate follows from a passing checker scaffold.
Authority integration, alternatives B–D, architecture comparison/selection,
exhaustive Quint/Apalache evidence, S03–S15 and all broader XML gates remain open.
No Git remote is configured, so local commits are not GitHub publication.

### Candidate A finite-record correspondence and semantic-unit handoff

**CLM-0156.** Experiment observation: checker source
`2f53817de7dad8553b5ecb43746b676006a2d28e` completes the Candidate A
agreement semantic-unit plan Tasks 1–8 at a local experimental boundary.
The final root receipts record 441 branch Python tests, including 84 checker
tests, and zero differences across all 53 actual exported cases from 14 ITFs
with 76 retained provenance occurrences. These are milestone results, not
all-program correspondence or exhaustive verification. The main branch's
separate test measurement remains 284; implementation remains on the S02 branch.

The checker evaluates frozen Python semantics with a separately implemented
decoder and compares raw results, full projections and ordered effects. The
final default CLI requires the complete pinned ITF inventory: missing records
are rejected. Selected-record comparison requires explicit `--allow-subset`
and reports that weaker scope. Nine archived semantic mutations retain their
modified cases, matching raw trace and failing CLI receipt. Source review
and archive review are preserved. Separate reference implementation is not
independent authorship: the takeover checker author also authored the model.
Root and scoped nonauthor native reviews do not substitute for Council acceptance.

The Apalache preflight produced no verification result. Root stopped its owned
server after observing a wildcard listener; a later offline invocation refused
the input format before checking. Neither outcome is a counterexample or an
architecture failure. Candidate B's fourth draft is preserved as unadopted:
its concrete node table, request/error carrier, finite input domains and exact
Core-result mapping still need correction before implementation.

Claim metadata:

- Sources: SRC-0031 XML S02; SRC-0033 comparison boundary; SRC-0035 adopted common design.
- Repository: local Moriarty, branch `s02-model-comparison`; checker source commit above; producer `02a4e94b604760e54b04f94c0dc089ede0de938b`.
- Evidence: `a9deaecd7fde2c0e7048d0af4e0bbb8d0c760683`; aggregate handoff `0266df2b759b40a1e832875f98f90bf68114bd79`; archive audit `d7384d2c8b2c86ac73d3c9f881a1ba66775f10ab`.
- Local locators on that branch: `evidence/s02-model-comparison/candidate-a-correspondence/manifest.json`, its `export/` and `checker-stages/mutations/` directories, and `evidence/s02-candidate-a-core/evidence-manifest.json`.
- Reviews: `docs/superpowers/reviews/2026-09-05-moriarty-s02-candidate-a-correspondence.md`, `2026-09-05-moriarty-s02-candidate-a-correspondence-evidence.md`, and `2026-09-05-moriarty-s02-candidate-b-draft-intake.md` on that branch.
- Observed and commit date: 2026-09-05 UTC; final journal observation 12:17:54 UTC.
- Authority: deterministic author tests, root execution and source inspection, scoped native source/archive reviews; shared model/checker authorship disclosed.
- Scope: Candidate A agreement semantic-unit Tasks 1–8 only; frozen Python Core and swap unchanged.
- Evidence kind: experiment observation and repository observation.
- Reproduction: reproduced finite-record comparison with committed corpus, exact source pins and retained mutation inputs; final archive audit verified 121 pins, and root verified all 27 aggregate handoff pins.
- Confidence: high for recorded bounded results; no exhaustive or all-program claim.
- Lifecycle status: S3 experimental branch, not main-integrated implementation or release.

This supersedes the preceding Task 7 in-progress status, not the broader open
obligations. Next is a reviewed candidate-specific authority integration contract
that binds actual Core inputs/results/effects to signing, verification and atomic
commitment under both profiles, including installment races and nonce-one recovery.
B–D implementation and comparison, justified selection, exact-provider Council
reviews and S01 backfill, S03–S15, ACTUS and all release gates remain open.
The full product goal remains active. Foreman repairs remain closed by user
direction. There is still no configured Git remote or GitHub publication claim.

### Candidate A authority adapter and actual offline-check resource boundary

**CLM-0157.** Experiment observation: source
`e84f737dc97cf923579f8a59c91ee04c03ff0233` implements the Candidate A authority
observation adapter. Actual request/program/state/input/time and full computed
results/effects are bound by recomputation and equality. Cancellation is a
separate checked identity with empty effects and NoCoreProjection; its evidence
tag is not a Core computation or signing authority. Supplied plan payloads are
carried verbatim, not authorized by this adapter.

Root's terminal reruns passed all 15 named adapter tests and 100 sampled traces,
with no adapterBindingSafety violation and 100 adapterProducedFirstFill witnesses.
This is one deterministic adaptation path, not an authority-commitment or
exhaustive witness. The author's separate terminal receipts record both
typechecks and 441 Python tests. Native nonauthor source review found no blocking
adapter defect. Root verified all 54 artifact/source pins, the external tool pin,
47 byte-identical archived payloads and the 13-file live source closure.

Evidence `46946aa55682090ebfe31e6e1a3e2b477028dc12` explicitly records an original
process deviation: the first-fill RED was a name-resolution failure, while the
genuine cancellation RED lacks retained historical source bytes. Ten added
tests are supplemental regressions, not newly observed REDs. Source review
accepts the tested implementation for integration without retroactively
certifying complete failing-first evidence or waiving any XML/Council gate.

**CLM-0158.** Experiment observation: a direct offline Apalache route accepted
the single-module flattened Quint JSON with the required `.qnt.json` suffix.
The 93,413,860-byte input was parsed and typechecked, but both 4 GiB and 8 GiB
heap runs terminated with exit 255 in InlinePass after 275.18 and 410.265 seconds,
respectively. Neither checked states, produced an invariant result or supplied
a counterexample. No listener was started for these direct runs. The generated
input remains local and hash-pinned, not committed; exact commands, source/tool
pins and byte-preserved command/detailed logs are committed in
`93fce82250cf6f0f68114b4d2b5c373cc9b7390d`. This supersedes the previous
preflight-only status, not the open model-checking obligation. Root stopped
memory escalation; semantics-preserving model factoring remains required.

Claim metadata:

- Sources: SRC-0031 XML S02; SRC-0033 comparison boundary; SRC-0035 adopted common design.
- Repository: local Moriarty, branch `s02-model-comparison`; full source/evidence commits above; no configured remote.
- Locators on that branch: `evidence/s02-model-comparison/candidate-a-authority-adapter/manifest.json`, `root-reruns.json`, `author-report.md`, and `evidence/s02-model-comparison/candidate-a-apalache-offline/manifest.json`.
- Review: `docs/superpowers/reviews/2026-09-05-moriarty-s02-candidate-a-authority-adapter.md`.
- Observed and commit date: 2026-09-05 UTC; this journal entry 13:19 UTC.
- Authority: local experimental implementation, actual author/root execution receipts and native source review, not exact-provider Council.
- Scope: Candidate A adapter only and terminal offline resource failures; frozen Core/swap unchanged. Main's test measurement remains separate from the S02 branch.
- Evidence kind: experiment observation and repository observation.
- Reproduction: reproduced adapter tests/sample and observed offline terminal failures; incomplete historical RED source closure remains explicit; no formal verification result.
- Confidence: high for the recorded source, finite tests and terminal process outcomes; no universal correspondence or architecture-failure inference.
- Lifecycle status: S3 experimental branch, not main-integrated implementation or release.

The next concrete boundary plan is adopted at
`5b1f1fb76c1f17a91335c4c9ea3fb621343a3324`: actual funding under both signing
profiles, full-plan fidelity, guarded verification/atomic commitment and reachable
rejection retaining the original evidence. Implementation has begun, with no
passing boundary result claimed here. Both complete A lifecycles and integrated
exports follow. Candidate B fifth/sixth drafts were rejected and preserved at
`d69ca6999263db400c5e9f7e793fda4e164c274d`; an actual frozen Python diagnostic
disproved the draft's negative-deposit error classification. B–D implementations,
comparison/selection, S02, Council/S01 backfill, S03–S15, ACTUS and 24 XML release
gates remain open. The full goal remains active; Foreman repairs remain closed.

### Candidate B native obligation-graph design adopted after corrections

**CLM-0159.** Repository observation and delegated design decision: branch commit
`fe011a17a5d9e1c4d48bed07fafe4b92c4f2c916` adopts an independently reviewed
native obligation-graph experiment, replacing the rejected draft as B's
implementation contract. Its evaluator must compute from native obligations,
dependencies, exclusions, accounts and choices, not call A or relabel A's answer.
A separate narrower library mapping supports frozen-Python comparison.

The review caught a comparison-domain error: minimumTime1 excluded the original
swap's Time0 deposit. A retained root frozen-Python probe accepts the same
deposit10 at now0 from minimumTime0 but rejects it from minimumTime1. The adopted
design restores initial Time0 and its ordinary successor clocks. Authority
fixture clocks remain a separate boundary. The probe's initial formatting error
is retained and is not counted as a semantic result.

The design also distinguishes financial completion from a structurally exhausted
graph and stranded escrow. An admitted single-deposit graph can exhaust its
frontier while retaining funds; this is a static design consequence awaiting
an actual B experiment, not a reproduced B counterexample. Required negative
witnesses must expose it, and B cannot claim universal generic non-locking.
Deadline/no-timeout priority, complete successor checks and mapping diagnostics
are explicit. Native re-review cleared the exact corrected design bytes; root
adopted them under existing user-delegated authority, not as Council approval.

Claim metadata:

- Sources: SRC-0031 XML S02; SRC-0033 comparison boundary; SRC-0035 common design.
- Repository: local Moriarty, branch `s02-model-comparison`, full commit above.
- Locators on that branch: `docs/superpowers/specs/2026-09-05-moriarty-s02-candidate-b-native-graph-design.md` and `evidence/s02-model-comparison/candidate-b-native-design-review/`.
- Final reviewed design SHA-256: `2cc795c4ad3a83c3b7b5ecf6ff39ef0af486a0ae1a864fa2fe3e0b24fe2f70ac`; original exact bytes and review/adoption dispositions retained.
- Observed and commit date: 2026-09-05 UTC; this entry 14:07 UTC.
- Authority: adopted experimental design, native nonauthor Astra review and a source-pinned frozen-Python probe; not the exact-provider Council.
- Scope: B's staged native-model design and the frozen comparison-clock probe.
- Evidence kind: repository observation, experiment observation and delegated recommendation.
- Reproduction: frozen-Python clock probe reproduced; B implementation and runtime experiments specified-only.
- Confidence: high for the documented design/review and probe; no checked B invariant or architecture choice.
- Lifecycle status: S2 written experimental design; source/evidence committed on the experimental branch, not a main-integrated B implementation.

B implementation starts with concrete carriers, generic validation and literal
graphs. A's authority-boundary implementation is under final verification, not
accepted here. Complete A lifecycles, B–D comparison, model checking, Council,
S03–S15, ACTUS and the XML release gates remain open. Foreman repairs stay closed;
the full Moriarty goal remains active and no GitHub publication is claimed.

### Candidate A A0 independent boundary intake accepted

**CLM-0160.** Experiment and repository observation: root independently executed
all forty boundary tests and one hundred sampled funding traces against source
`d14cfea98a1c9213e5ef5f12c1a088f4e966083d`. Both commands terminated with exit 0.
Each of five action witnesses occurred in all hundred traces; each signing
profile committed in fifty traces. No boundarySafetyA violation was observed.
This is finite execution evidence, not exhaustive verification or cryptography.

Native nonauthor review approved the corrected boundary source/specification.
Root checked sixteen final Quint import-closure files, 134 prior source pins,
and 149 exact archived members. The historical boundary-author Python receipt
lacks a full contemporaneous Python source/test/dependency closure; this remains
disclosed, together with the predecessor adapter's historical RED provenance gaps.
No missing historical evidence was reconstructed or silently passed.

Evidence commits on s02-model-comparison are
`0190cb97270e4775273cb3737dc649d874861e91` and
`95899b37fa03ccb61506e51c099dd6aea96bcc16`. The latter adds the canonical
review.md index required by OpenSpec. Locators are
`evidence/s02-candidate-a-completion/a0/{manifest,validation}.json`,
`review.md`, independent raw runtime receipts, and source/archive inventory.
The existing main snapshot supplies the original exact archive without recopying.

Claim metadata: sources SRC-0031 (XML S02), SRC-0033 (comparison boundary),
SRC-0035 (common design); authority experimental; scope A0 boundary intake only;
observed/commit date 2026-09-05 UTC; reproduction reproduced as scoped above;
confidence high for these finite observations; lifecycle S3 experimental branch;
local Moriarty repository without a configured remote. Main contains this status
record, not integrated Candidate A source. Native review does not fill Council.

A1 corrected installment and swap plans are next. Full lifecycles, integrated
correspondence, bounded model checking, Council, A acceptance, B–D/S02 and all
later XML obligations remain open. Foreman development remains closed.
The product goal was observed marked blocked during this execution turn;
the current turn is working, but no automatic background loop is claimed.

### Candidate A A1 plans adopted; fixture implementation dispatched

**CLM-0161.** Repository and experiment observation: local commit
`82d2c0b1d35ecf2f954603e4b8d54b248bda2663` on `s02-model-comparison`
adopts corrected installment and swap plans. Independent native nonauthor
review approved both after exact-rejection assertion corrections. Root's two
final assembled-plan typechecks terminated with exit0. The evidence binds eight
assembled modules, twelve frozen imports and both exact plan digests. A replayable
diagnostic rejects missing/failed terminal results and altered plan hashes.

The plans cover original-signature fresh cancellation, revision2 after two fills,
new nonce1 recovery, canonical raw swap timing, actual two-party funding and
disposition, complete refusal records and finite action witnesses. The explicit
route bounds are17/19 transitions with command budgets20/22. Sampling is not
proof; arbitrary interleavings remain a separate A5 obligation. PreparedAttempt
actor metadata is not authenticated by the unchanged common contracts.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope local A1 planning/static compatibility; observed date 2026-09-05 UTC;
reproduction reproduced for static checks only; confidence high for those checks;
lifecycle S3 experimental branch. Evidence is
`evidence/s02-candidate-a-completion/a1/{manifest,validation}.json`, `review.md`,
two raw typecheck receipts and exact assembled source bytes at the named commit.
Archived source trailing blank lines are preserved byte-for-byte; the commit
whitespace check excluded that immutable assembled-source directory only.

A2 Task1 installment fixtures and A3 Task1 swap fixtures were dispatched to
separate native owners, each requiring original compiling RED then GREEN before
independent review. Dispatch is not a result or a promise of process liveness.
Both lifecycle packages, A4–A7, Council and broader S02/XML gates remain open.
Main records progress only; Candidate A remains unintegrated with no Git remote.

### Candidate A fixture units implemented and admitted

**CLM-0162.** Experiment and repository observation: installment fixture unit
`35959ae64e157e7c2253fdce786976237532d13b` and swap fixture unit
`3f440d2494cf41db289905b15663559e400a6387` preserve actual compiling assertion
failures followed by two passing corrected tests each. Installment RED omitted
the fourth parent-plan operation; swap RED expected settlement reductions2
instead of3. Tests and common semantics were unchanged during correction.

Nonauthor source review approved both corrected two-file units. Root audited
the exact fourteen-file stage closures, terminal outputs, sole intended source
corrections and byte-identical original evidence archives. Receipts are under
`evidence/s02-candidate-a-completion/a2/task1/` and `a3/task1/` in the named
experimental commits, each with manifest, validation command/result, review and
author-evidence archive. Runtime was executed by authors; root independently
audited it and did not duplicate those focused commands.

The installment recorder added a separate Rust binary identity receipt after
RED and before GREEN. Earlier A0/A3 receipts pin the same digest; this does not
create a retrospective per-command RED backend pin. The unsupported binary
version query is retained, with installed-directory version distinct from a
successful self-report. No original receipt was rewritten.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope two literal fixture units; observed date 2026-09-05 UTC; reproduction
reproduced for named checks only; confidence high for those finite observations;
lifecycle S3 experimental branch. A2/A3 Task2 actual authority routes are
dispatched independently; no full lifecycle or Council acceptance follows.
Preliminary export/checking intake is preserved at `5ab0b55`; it is not an
adopted factoring plan or a successful model check. Main remains status-only.

### Candidate A swap ordinary authority routes execute

**CLM-0163.** Experiment and repository observation: commit
`c3c89faef1b5723c1520a05c88eb26ff4529d6c5` implements swap Task2. Five named tests
and the recursive typecheck pass. The deterministic matrix executes twelve
ordinary scenarios under both signing profiles through actual A guards and
common updates: sequential Alice/Bob funding, nonce1 dispositions, funded
settlement/refunds/timeouts, exact retained supplied-deadline and empty-timeout
refusals, and replay rejection. This is finite guarded-prefix execution, not
the separate stateful harness or arbitrary-interleaving verification.

The compiling unchanged-CommitS scaffold failed fundingTest with QNT508 before
the sole correction restored applyCommit. Nonauthor final source review approved;
root audited both fifteen-file closures, raw terminal results, exact correction
and the original evidence archive. Evidence is committed under
`evidence/s02-candidate-a-completion/a3/task2/` with manifest, validation, review
and author archive. Root audited author runtime, rather than duplicating it.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope ordinary swap authority routes; observed date 2026-09-05 UTC; reproduction
reproduced for the named finite checks; confidence high for those observations;
lifecycle S3 experimental branch. Task3 stale binding/adversarial/action witnesses,
shared regressions, integrated correspondence, explicit model checking and Council
remain open. A2 Task2 corrected checks are in progress; no outcome is inferred.

Verification addendum `a314549` consolidates equivalent typechecking based on
installed Quint source inspection and an unused-invalid-declaration control,
with independent review. Both final lifecycle closures must be frozen and bound
before one shared boundary/adapter/Python regression receipt serves both units.
All own tests, witnesses, properties and original source-plan hashes remain intact.

### Candidate A installment authority routes execute

**CLM-0164.** Experiment and repository observation: commit `955f56b` implements
installment Task2. Seven named tests and the recursive typecheck passed through
actual guarded authority routes under both profiles. Assertions cover parent
registration without money movement, both initial fill/cancel race orders with
retained stale losers, the second fill, fresh cancellation under the original
parent signature, and separately signed nonce1 recovery/refusal cases.

The original compiling freshCancellationTest failed with QNT508 under an extra
AuthorityUnused proposal restriction. Removing only that restriction produced
the observed GREEN; tests and accepted fixture did not change. Nonauthor source
review approved. Root audited all four fifteen-file stage closures, before/after
source and tool pins, raw terminal results and91original archive members.
Evidence at `evidence/s02-candidate-a-completion/a2/task2/` includes the review,
manifest, replayable validation and original author archive. Rust version is an
installation-directory label, not an unsupported self-reported version claim.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope deterministic installment authority routes; observed date 2026-09-05 UTC;
reproduction reproduced for named finite checks; confidence high for those
observations; lifecycle S3 experimental branch. Root audited author runtime
without duplicating it. Task3 is dispatched after admission; adversarial/action
witnesses and shared final regressions remain open. No full A2, correspondence,
model checking, Council, integration or broader XML gate is accepted.

### Candidate A swap adversarial and action unit admitted

**CLM-0165.** Experiment and repository observation: `926b350` implements swap
Task3 with original compiling generic-verifier RED and corrected A-specific
verification. Seventeen tests and the recursive typecheck pass. The actual
hundred-sample run reports no swapSafetyS violation and all24 required witnesses
nonzero. Profile completions are54 after-resolution and46 before-resolution.
This samples the declared finite routes, not arbitrary concurrent interleavings.

The six negative tests cover stale signing/verified records, fully rebound
observation mutations, invalid second operation, wrong Core chooser/signer/nonce,
and stale plan facts. Constructed verified tampering is explicitly a boundary
control, not a reachable successful verification. Nonauthor final source/spec
review approved; root audited43original archive members,32RED/GREEN source
snapshots, the exact sole correction, all terminal receipts and raw witness
counts. Evidence is at `evidence/s02-candidate-a-completion/a3/task3/` in that
experimental commit, with replayable validation, review and author archive.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope finite swap adversarial/action unit; observed date2026-09-05 UTC;
reproduction reproduced for named checks; confidence high for those observations;
lifecycle S3 experimental branch. No per-sample ITF output was requested or
retained. Raw command seed42 and printed reproduction hint0x153 remain distinct.
Shared final regressions and full A3 acceptance remain open.

Bounded A4 export/replay design `effb7af` is independently reviewed and adopted
under the XML's delegated planning authority. Separate producer/checker concrete
plans must freeze exact schema, expanded inventory and behavioral tests before
implementation. The design preserves schema1, actual-vs-claimed computation,
retained rejection histories and explicit denied probes/case boundaries. This
is planning, not integrated correspondence, model checking or Council acceptance.

### Candidate A lifecycle local requirements accepted

**CLM-0166.** Experiment and repository observation: installment Task3 `bfc7832`
passes17tests, its recursive typecheck and100scheduled traces with all25required
witnesses positive, profile completions50/50. The original compiling generic-
verifier RED fails changedUnusedNodeTest; restoring only the A guard produces
GREEN. Nonauthor source review and root audit bind118original archive members,
five sixteen-file stage closures, separate streams and unchanged source/tool pins.

Aggregate acceptance `28d35d8` maps A2-I01–I11 and A3-S01–S08 to deterministic
tests and action-family witnesses. Final shared regressions pass40boundary tests,
15adapter tests and441Python tests. Root audited the2673-file source/input archive
and all8final lifecycle source pins. Exact external graph and DeFiFormal fixture
bytes are included; Python package versions, not a full virtualenv image, are
retained. Git freshness cannot detect later changes in those external fixtures.
Typed measurements26/27/29–33 bind the results to experimental source28d35d8;
measurement28 is retained and superseded33 to correct its omitted match argument.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope local A2/A3 finite lifecycle acceptance; observed date2026-09-05 UTC;
reproduction reproduced for named checks; confidence high for those observations;
lifecycle S3 experimental branch. No per-sample ITF or unrestricted interleaving
proof is claimed. A4 exports/replay, A5 bounded checking, A6 Council and A7
integration remain open. Native reviewers do not substitute for requested vendors.

A5 tool control `fcd28f2` checks only a tiny unrelated nullary-binding example:
the correct invariant passes depth2 and the deliberately wrong invariant yields
a state1 counterexample. It validates that offline command path, not Candidate A
semantics or an expression-sharing performance improvement. An isolated factoring
plan is being prepared before any new expensive Candidate A verification run.

### Candidate A independent replay implementation begins

**CLM-0167.** Experiment and repository observation: `9f9cfda` implements the
first independent replay unit, strict immutable carriers and the frozen Python
Core bridge. Five tests pass; disabling duplicate-map rejection produced the
required compiling assertion failure with the other four tests passing. Root
reviewed the complete source and audited3329original Python environment files,
eight command stages, source archives and unchanged before/after pins. Missing-
module collection errors and an initial recorder-order diagnostic are retained,
not counted as behavioral RED. Evidence is under
`evidence/s02-candidate-a-completion/a4/checker-task1/` on the experimental branch.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope local A4 carrier/Core bridge unit; observed date2026-09-05 UTC;
reproduction reproduced for named checks; confidence high for those observations;
lifecycle S3 experimental branch. Session measurement34 and fact82 record this
unit. It does not implement full authorization replay or admit integrated traces.
The runtime archive is not a full OS image or authenticated execution attestation.

Plan adoption `d4f6196` precedes this unit. Immutable addendum `ab7c827` corrects
a shared arithmetic error: all78unchanged cases total1557events, not1757.
Swap negative controls total391events, not591; no case or scheduled event was
removed. The exact corrected inventory is committed. Producer/A5 plans remain
under review. Released Council capability was inspected without provider calls
or Foreman edits; the required coordinator remains unavailable. A4–A7 and exact
cross-provider Council acceptance remain open.

### Candidate A independent histories and factored kernel implemented

**CLM-0168.** Experiment and repository observation: checkerTasks2–5
(`71db4f2`, `0655354`, `94f5824`, `483013b`) add independent authority rules,
46 ordinary and32 negative/stale expected histories, strict full-state/field
provenance and a direct CLI. All17 current tests pass. Each unit retains a
genuine importable behavioral RED and original GREEN, with root source review
and independent archive/stream/current-runtime audit. The direct-script import
failure is separately retained and corrected. These expected histories do not
constitute actual Quint integrated exports or final mutant acceptance.

A5 kernel `d824fa3`, under adopted plan `900bb20`, passes recursive typecheck
and all six finite kernel tests. The original typed supplied-result omission
fails QNT508; the sole correction restores actual-versus-supplied result checking.
Root verified all38 source archive members in each original command, exact
RED/GREEN change and terminal streams. The one315704660-byte runtime archive is
retained in seven lossless ordered Git-sized parts, with independent concatenated
hash validation. The shared Python snapshot is reused, not duplicated. Full
original/factored corpus comparison is dispatched; the pilot has not run.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope local A4 checker units and A5 kernel; observed date2026-09-05 UTC;
reproduction reproduced for named checks; confidence high for those observations;
lifecycle S3 experimental branch. Evidence is under
`evidence/s02-candidate-a-completion/a4/checker-task2/` through `checker-task5/`
and `evidence/s02-candidate-a-completion/a5/kernel-task1/` on that branch.

A read-only expected-state serialization estimate reports1557events and
1545385554 installment plus546949423 swap bytes, excluding event payloads and
ITF overhead. Installed Node's maximum string length is536870888; installed
Quint0.32.0 assembles and serializes whole Rust/ITF results as single strings.
Thus the adopted two-long-trace transport is infeasible unchanged; no actual
failed export is claimed. A case-sharded addendum is being planned, preserving
all78cases/1557events and full records, original receipts and mutant obligations.
It changes transport, not model semantics. A4–A7 and exact-provider Council stay
open; Foreman development remains closed and no providers were dispatched.

### Candidate A actual computation observer and bounded transport

**CLM-0169.** Experiment and repository observation: producerTask0/1 `5616bcd`
implements the actual second-evaluation observer, cancellation-without-Core
behavior and ordered plan computation records. Two inventory tests and four
Quint observer tests pass; recursive typecheck passes. Root audited all nine
original command stages, every before/after source snapshot,8102runtime pins
and exact stream hashes. Genuine inventory/computation controls are retained.
Two additional parser failures are preserved; a narrow sequential-match syntax
correction changes no predicate. Diagnostic/order tests are supplemental GREEN,
not claimed failing-first controls. This is not full case execution or replay.

Transport plan `5829639` specifies78 native case shards and schema3 admission,
with all1557events, complete records, local adjacency and explicit unsigned
per-case initialization. It does not fabricate cross-shard edges. Separate
staging/root-case-pin/seal phases prevent producer self-admission. Utility plan
`22cdec6` specifies strict standard-library streaming JSON and original controls.
These CT001–010/ST001–009 EARS obligations have corresponding OpenSpec scenarios;
strict A4 specification validation passes on main. That validation is structural,
not runtime evidence. Both implementation tasks are now dispatched.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope local producer observer unit and adopted transport plans;
observed date2026-09-05 UTC; reproduction reproduced for named checks;
confidence high for those observations; lifecycle S3 experimental branch.
Original producer evidence is under
`evidence/s02-candidate-a-completion/a4/producer-task1/` on that branch.
The future shared JSON utility is a disclosed common lexical dependency, not
shared authority semantics. Largest-case native feasibility, complete exports,
independent replay, bounded model checking, Council and integration remain open.

### Candidate A streaming utility and sharded checker implemented

**CLM-0170.** Experiment and repository observation: streaming reader `62b7b30`
and writer/hash unit `fdb81c7` implement strict bounded JSON transport. All 42
utility tests pass. Original duplicate-key and short-write behavioral failures,
corrected results, source snapshots and unchanged runtime pins are retained and
independently audited. This shared lexical dependency does not supply authority
semantics to either implementation.

Sharded checker supplement `26c9cc5` requires schema3, all 78 entries and raw/case
pinsets, full source/import closure, independently reconstructed event/state
histories, global ordinals, local adjacency and strict EOF. It passes 79 focused
tests: 37 checker plus 42 utility tests. Root read the full source diff and audited
ten original command stages, 420 source archive members and 3329 runtime files.
The omitted raw-field equality and early unknown-metadata controls have genuine
behavioral failures. An intermediate suffix-fixture hash-linkage failure and its
fixture-only correction are retained separately. The original report calls
`fdb81c7` its narrative dispatch base; command receipts bind `9d26d9b`, which is
the root admission evidence base. No original receipt was rewritten.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope local strict transport and synthetic checker units; observed date
2026-09-05 UTC; reproduction reproduced for named checks; confidence high for
those observations; lifecycle S3 experimental branch. Original evidence is under
`evidence/s02-candidate-a-completion/a4/stream-reader-task1/`,
`stream-writer-task2/` and `checker-sharded/` on the experimental branch.

Producer validation has resumed with the newly frozen utility in its source
closure. A5 original/factored corpus commands remain active, with both adapter
15-test suites and authority-boundary typechecks reporting terminal success.
These partial results do not admit the complete equivalence corpus. Actual
native feasibility/exports, all 78-case replay and Task6 mutants, A5 model
checking, Council and integration remain open. No Foreman repairs resumed.

### Native transport compatibility corrected; adversarial execution starts

**CLM-0171.** Experiment and repository observation: the producer's original
parameterized wrapper did not export its imported state/actions transitively
(QNT404). A small named-instance re-export also lost its bound constant (QNT500).
Neither failure is a Candidate A semantic negative control. A small literal-body
probe passes its bound-ordinal32 assertion and native one-step run, which exposes
the exact variable order authorityState, caseIndex, cursor, latestEvent.
Amendment `4465863` therefore preserves the full shared driver through mechanically
generated literal wrappers and corrects the transport order expectation.

Correction `5843ff6` changes only the checker tuple and an independent fixture
literal. Both honest-shard tests fail at the old exact-order predicate; all 79
tests pass after correction. Root reviewed the diff and audited four original
stages, 220 source/reference members, retained artifact files and 3329 runtime
files. Evidence is under `evidence/s02-candidate-a-completion/a4/native-order/`
on the experimental branch. The toy probe is not Candidate A lifecycle or
largest-case feasibility evidence.

Plan `cd06756` preserves all 27 semantic triples, one substitution triple, five
locator triples and seven actual-package controls. Its T6S requirements and
native-compatibility NW requirements have OpenSpec counterparts on main; strict
specification validation passes. Implementation has now observed the genuine
omitted-equality substitution failure and restored the admitted checker exactly.
The planned 114-instance pre-export synthetic run is active; no final result or
actual-package acceptance is claimed. Producer Task2 validation runs against the
frozen appended test source. Any correction requires a new coordinated source window.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope local transport compatibility and ongoing finite test execution; observed
date 2026-09-05 UTC; reproduction reproduced for the named completed correction
checks; confidence high for those observations; lifecycle S3 experimental branch.
A5 original/factored adapter and boundary suites each pass 15 and 40 tests;
both installment recursive typechecks remain active. Full A4/A5, requested
Council, integration and publication remain open.

### Admit all native case drivers; retain the regression environment failure

**CLM-0172.** Experiment and repository observation: producer Task2 `8be3cef`
admits the full 78-wrapper aggregate recursive typecheck, all 12 unfiltered
native Quint tests and 13 producer Python tests. Its lowerer preserves ordinary,
negative, refusal, cancellation and recovery selectors. Root reviewed the full
source and audited 15 original stages, 3128 source-byte members and 8102 runtime
pins. The original boundary/ordinal behavioral failures and their two exact
corrections are retained. The separate QNT404/QNT500/QNT202 compatibility failures
are diagnostic history, not semantic negative controls. NW004 checks independent
shard endpoints without claiming cross-shard state carry or paired namespace
initialization. Two match-arm bindings are approved static language corrections,
not an invented parser RED. The original receipt archive has 3230 members and
SHA256 `98b132da47ea00f3261121155bc2223e2c8a7e6bc8e20391e6ab72d4977d5ddc`.

RH001 now has an author-observed bounded-read failure and 15 passing producer
Python tests after the streaming-hash correction. Root admission is pending.
Task6's original broad synthetic mutation run continues on frozen source. No
actual Candidate A native shard has yet been admitted or sealed. Largest-case
resource feasibility, all 78 exports, independent replay and actual-package
controls remain mandatory.

The A5 original corpus has completed all 20 commands and 244 tests. The factored
corpus and six-test equivalence sequence remain active; partial success is not
complete Task2 acceptance. Fresh historical Python collection found 441 tests,
but execution stopped at 440 passes and one failure: an unchanged test invokes
`uv`, which is absent from the controlled recorder PATH. Original failure,
JUnit and source/runtime receipts are retained; Core53 was not run after the
failure. A pinned offline uv/interpreter launch supplement is being planned.
No test exclusion, helper repair, successful retry or Core53 result is claimed.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope local producer Task2 admission and explicitly partial A4/A5 execution;
observed date 2026-09-05 UTC; reproduction reproduced for the named original
Task2 checks; confidence high for those observations; lifecycle S3 experimental
branch. Root evidence is under
`evidence/s02-candidate-a-completion/a4/producer-task2/` on that branch.
Full A4–A7, requested cross-provider Council, implementation integration and
publication remain open. No Foreman repairs resumed.

### Admit structural export and measured invocation units

**CLM-0173.** Experiment and repository observation: RH001 `b08a2da` replaces
whole-native-file hashing on the actual recorder path with the admitted bounded
streaming hash. The original guarded-read failure and all 15 corrected producer
tests are retained; root audited 480 original source members and 8102 runtime
pins. Structural exporter `9a263d7` preserves the exact 78-case/1557-event
inventory, typed carriers, independent local histories and root admission before
sealing. Its two original omission/reorder failures and 55 passing non-package
tests are independently audited. Four actual-package tests remain present and
unexecuted; no native export or parser invocation is inferred from synthetic
parser controls. The archive preserves 515 original files and 13 test symlinks.

Receipt-only runner `1df388a` adds measured GNU time output, strict 23-field and
exact-command validation, bounded wall time and owned-group cleanup on every
post-spawn termination path. Its genuine empty-present completeness failure and
41 passing controls include actual short processes, timeout, ordinary leader
exit with a lingering child, and a command-substitution control. Root audited
20 declared source members, 8089 runtime/reference pins and 21 actual capture
records. Inner receipt presence is never semantic acceptance. The original
archive is SHA256
`c748f2da10df4e9a145127c5f626b3d7a6603c885e89885e24a99c10641f2822`.
These local process controls do not establish Candidate A memory feasibility or
PID-namespace isolation. No Foreman development resumed.

Root's completed A5 corpus intake verifies all 40 original/factored commands,
2720 archived source members and exactly 244 ordered tests per side. The author
also reports terminal success for all six full-record equivalence tests, both
100-sample lifecycle runs with all 25/24 witnesses, the corrected complete
441-test Python suite and the 53-case independent Core comparison. Overall
Task2 source/evidence admission remains in progress with a separate nonauthor
review. These finite checks are not bounded model checking or arbitrary-trace
equivalence.

Both environment failures remain intact: the original missing-uv test failure
and the later lexical python-versus-python3 probe mismatch before pytest. The
adopted offline real-uv supplement and exact symlink amendment change neither
tests nor requested child commands. Fresh collection and all passing JUnit
cases are required to match the original 441 IDs. The current broad Task6 run
continues on its original frozen source; no final synthetic or actual-package
acceptance is claimed.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope admitted local A4 source/process units and explicitly partial A5 intake;
observed date 2026-09-05 UTC; reproduction reproduced for the named root-audited
units; confidence high for those observations; lifecycle S3 experimental branch.
Evidence is under `evidence/s02-candidate-a-completion/a4/recorder-streaming/`,
`exporter-source/` and `native-resources/` on that branch. Largest-case native
pilot, all exports, complete replay, A5 model checking, Council and integration
remain open.

### Admit the complete finite factoring corpus and dispatch its bounded pilot

**CLM-0174.** Experiment and repository observation: experimental commit
`9ccbf0e` admits A5 Task2 after root and nonauthor source/evidence review. Root
independently reverse-transformed all 26 derivatives without using their
generator, checked all 28 original pins, 18 exact identity wrappers, three
delegates and 30 frozen files, and audited all 52 original terminal commands.
The actual source archives retain 3650 members. Original and factored corpora
each passed exactly 244 ordered tests; all six full-record equivalence tests
passed; both 100-sample lifecycle runs reached every required 25/24 witness.

The fresh 441-test Python collection and passing JUnit identities match exactly,
with no skips or exclusions. All 2691 fixture/source inputs remain frozen. The
independent concrete Core comparison reports complete-inventory agreement for
53 cases from 14 original ITFs. Both prior environment failures remain unchanged:
the missing real uv command and the lexical python/python3 probe mismatch before
pytest. The adopted real-uv supplement and exact symlink correction alter
neither test bytes nor requested child argv. These failures are not semantic
counterexamples or suppressed test cases.

The separate nonauthor reviewer checked these source, receipt, fixture/runtime,
JUnit and Core obligations and found no material blocker in this bounded unit.
That reviewer authored earlier dependencies; this overlap remains disclosed.
This is not the requested cross-provider Council. The original author report
and index remain at SHA256 `e035e32986e2cbc19d54aef1055f2994be46949fc23c1629be19e1ed5b8a9322`
and `7db8df0bc6079fec9fcda16fef9c0b45fab887af963f4a81cecb59d631d1e1d7`.

Root's fresh lossless archive audit checked all 515 outer members and every
nested source archive. Fifteen ordered parts preserve the original
744523649-byte tar, SHA256
`c3d2d97103b714c60762853ba087e338aaa901c841e4669e8bccfad3b6868ae1`.
The already admitted shared runtime/Python archives remain explicit references,
not silently replaced snapshots. Root's two terminal-audit schema/regex draft
errors were corrected without changing implementation or original evidence.

Task3 dispatch binds actual base
`9ccbf0ed571e4055bc05962e5681fdf2fa75ad96` separately from the immutable
`900bb20` runtime bootstrap. It authorizes only the exact original/factored
funding modules, complete four-route prefix test, samples and paired compile.
Root must inspect original generated JSON, size/object counts and resources
before either depth-five checker command. No resource escalation, predicate
weakening, full campaign or successful model-checking result is inferred.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope locally admitted finite A5 Task2 and a separately gated next experiment;
observed date 2026-09-05 UTC; reproduction reproduced for the named root-audited
original checks; confidence high for those observations; lifecycle S3
experimental branch. Evidence is under
`evidence/s02-candidate-a-completion/a5/corpus-task2/` on that branch. Full A4
native export/replay, Task6 acceptance, full A5, Council, integration and
publication remain open. No Foreman development resumed.

## CLM-0175 — Funding-pilot compilation stop and reproduced alias control

Experiment observation: resume intake revalidated the preserved Task3 source,
five terminal command receipts and the original archive. Prefix checks and
both sampled runs passed in those receipts. Original compilation failed with
QNT404 `AuthorityKey`, retaining empty stdout. Factored compilation and all
model-checker commands remain unperformed. Empty stdout is not a successful
generated-input baseline.

A separately reviewed generic-alias control reproduced the suspected frontend
mechanism with the same pinned Quint runtime. The original toy typechecks but
fails compilation with QNT404 `Key`. A second driver adds one explicit alias
import and changes only its module name; both commands then exit 0. Root and
an independent reviewer checked original source/runtime pins, archives and
outer/child exits, and inspected the complete generated init/step/safety bodies.
No model checker or simulator ran in this control. Inference: this supports
an explicit-import compilation view for the actual adapters; that proposed
remedy has not been executed or admitted.

Repository observation: experimental commit `a09d34a` preserves both units at
`evidence/s02-candidate-a-completion/a5/pilot-compile-stop/` and
`evidence/s02-candidate-a-completion/a5/alias-visibility-control/`. The control
archive SHA256 is
`9b42e0a060bfad2c6338da4b31111a70ec97588bf8789a062608b4c57287cc36`.
The actual Moriarty model and frozen runtime bytes remain unchanged.

Open question: the earlier A4 Task6 run has no terminal receipt and its old
session cannot be reattached in this resumed turn. Its partial output cannot
close the gate or establish process liveness. Original artifacts remain in
place; no duplicate run was launched.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope bounded compiler diagnosis; observed date 2026-09-05 UTC; reproduction
reproduced for the four frontend commands and receipt intake; confidence high
for those observations; lifecycle S3 experimental branch. Actual-model repair,
paired compilation-size review, A4/A5, Council and integration remain open.

## CLM-0176 — Full compilation-view timeout and individual A4 recovery

Experiment observation: the reviewed full funding-pilot view uses source
dispatch `43f33721da7a3bfb11dd1a91f1f548d5110ab93c`. Its 24 copied modules
preserve every semantic body, with two direct `AuthorityKey` imports as the
only changes. Recursive typecheck, the quantified test covering all 22 prefix
pairs and both 100-sample runs passed. All eight witnesses were positive on
each side. Root independently checked the original outputs, terminal exits
and every member of the five source archives against their recorded pins.

Original compilation timed out at the unchanged 900-second limit with the
4,096-MiB heap settings. Its time-wrapped child exited -15; the helper returned
124 and the actual outer wrapper exited 1. Generated JSON, stderr and the
GNU-time resource file are empty. The helper's 901.218-second duration is not
a completed GNU-time frontend measurement. No maximum RSS, successful alias
resolution or reached compiler phase is inferred. Factored compilation and
all solver commands remain unperformed; H1 and the paired-size gate remain
unresolved. The original handoff SHA256 is
`ac3c3174e191f3f51302b409e2cf15c85a418d0aa5fab921207644e50813e8c2`.
Experimental commit `bcdd627afd89d695f261ce850b11cff0ef1491fe` preserves the
timeout at `evidence/s02-candidate-a-completion/a5/compilation-view-timeout/`.
Its 91 original files form a lossless archive in ten parts, SHA256
`9cd6ff42940804ab9aa68d0e90bc9f5faec28993ad1e1ed186a48f19264dc133`.
The first archival audit failed to find two historical pins in a nested source
archive. Its original source/error remain preserved; corrected traversal and
the final audit with optimization explicitly disabled passed. Root and an
independent reviewer checked the complete package.

Repository observation: A4 recovery preserves the interrupted parent's unknown
exit and its partial eighteenth child. Root and a separate reviewer admitted
exactly 17 original children, binding archived source, exact argv, reports and
authentic zero exits. Root independently rehashed 318 retained files totaling
9,795,796,111 bytes and all 46 source and 3,329 runtime pins. These are delayed
endpoint observations, not historical parent after-pins. Inference: the pinned
worker's mandatory assertion-contained raw-file creation, immediately followed
by an unconditional digest in a new directory, establishes assertion execution
for its successful children under the stated source/runtime and sole-writer
assumptions. No historical optimization flags are supplied.

The new recovery recorder passed four real process controls covering normal
completion, failure after launch, a surviving descendant and interruption.
Forced cleanup remains ineligible even when a leader exits zero. The retained
17 admission is preserved at experimental
`evidence/s02-candidate-a-completion/a4/pre-native-recovery-admission/`, archive
SHA256 `26f922b2c53bafb8a4c7554698fffd6784a174270cabe13669f800563183c2ea`.
The same `bcdd627` evidence commit preserves this package.
Its standalone audit and independent package review passed. The exact remaining
97 preliminary nodes have started in fresh paths after A5 native execution
ended. Their terminal result, the amended 114-node aggregate, actual native
package and final 115-node run remain pending.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope bounded frontend experiment and individual preliminary-test recovery;
observed date 2026-09-06 UTC; reproduction reproduced for the four passing
pilot gates, compilation timeout, lifecycle controls and retained admission;
confidence high for those observations with the stated recovery limitations;
lifecycle S3 experimental branch. Full A4/A5, Council and integration remain
open. No timeout is represented as a model-checker result.


## CLM-0177 — A4 preliminary gate admitted and new compiler interval observed

Repository observation: experimental
`08e426c7163880b9312f1f1f029a4dde9d0e7593` preserves root's independently
reviewed A4 preliminary admission: exactly17 retained original controls plus97
fresh JUnit instances. The original interrupted parent's exit remains unknown.
The fresh pytest, recorder and actual outer exits are zero; all16 fresh child
controls retain their own original report bindings. The complete independent
intake checked both generated trees and frozen source/runtime inventories.
The compact package retains141 original members, archive SHA256
`20f9932bfd8c164e5f9de007452c855ae2f4af9c9e9aaf4380f493bb1224cacc`, under
experimental `evidence/s02-candidate-a-completion/a4/pre-native-aggregate-admission/`.
Independent and root archive audits passed. Large originals remain separately
retained and hash-bound. The fresh87 returned outer responses retain the
explicit intermediate polling-metadata gap; no missing history is invented.
This is the amended preliminary gate, not final115 or native78-case acceptance.

Experiment observation: the once-only compiler phase diagnostic at source
`07c3a5462154d45e733d01d8a8bae856cb0f63c4` timed out after900seconds under
the reviewed4096-MiB heap settings. Seven observer/fault controls and four
actual direct/observed alias controls preceded its separate root dispatch.
The original trace has seven markers: load, parse and typecheck resolved Right,
then compile was entered without a completion marker. Inference: this locates
an incomplete observed public compile interval in this new invocation, including
observer I/O and promise scheduling margins. It does not identify an internal
hotspot, locate the historical timeout, or prove the function was executing at
the interruption instant.

The time-wrapped child exited-15; forced SIGTERM cleanup completed. Recorder and
actual outer exits were zero for evidence preservation only. Original stdout,
stderr and GNU-time resources are empty, so no generated JSON or complete GNU
resource measurements exist. Independent full intake SHA256
`e4b745a1c7d09b0a8cc11a7cd3d3666da1bd8a1da3309bfaa4057d49669fb4f4`
and root original-file/response/group checks admit preservation. Final intake
SHA256 `f52aabfe60056dfce067edefb3cae9e6fd4863761d1fda2275927599ef4bcac6`
binds255 prior originals; its authentic post-index transport remains separate.
Originals are retained at experimental
`.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/`; durable packaging
is underway. H1, paired compilation, bounded verification, full A4/A5,
Council and integration remain open.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope amended preliminary-test admission and bounded compiler diagnostic;
observed date2026-09-06UTC; reproduction reproduced for the admitted original
tests, observer controls and terminal diagnostic; confidence high for those
observations with stated gaps; lifecycle S3 experimental branch.


## CLM-0178 — Parser preserved; largest native export failed

Repository observation: experimental
`320ed9b64afdf468e6bc8d72da99924e3ffaaf71` preserves the independently
reviewed final-source parser package at
`evidence/s02-candidate-a-completion/a4/final-source-parser/`.
The archive SHA256 is
`7233f85c03bfacf80d83d50a657a10f05a23d4863c157ecc81a4c37b13ee8119`.
Its 290 original members retain the authentic parser, recorder and outer zero
exits, both complete 121-source copies, recorded runtime references and raw
38,527,047-byte IR with 99 modules and 122 unique typedefs. Independent and
root standalone audits passed; the independent comparison matched every
archived original to its retained bytes. This admits the parser prerequisite;
the exporter still owes its own fresh parser calls.

Experiment observation: the separately dispatched largest case014 native
export at source `08e426c7163880b9312f1f1f029a4dde9d0e7593` failed.
Original stderr reports JavaScript heap exhaustion. Node exited -6; recorder,
RH002 and actual outer returned 134. There is no ITF, and no invariant or
witness acceptance follows. The 900-second outer bound did not expire and
unforced cleanup completed. Original GNU-time measurements cover the whole
recorder and descendants: wall 5:56.80 and maximum RSS 4,371,180 KiB. They do
not locate an internal phase or establish a configured heap cap. Independent
intake SHA256
`4259ddd09d8073edb158a2140f0d78cfcf7ea1e4c607d1303ccc8aeb38d3924b`
and root admission
`f0676a056c57464478954ecc9ae622acfee14102bfcec8079b6900584dc3f758`
retain failed preservation only. Originals remain under experimental
`.superpowers/sdd/a4-producer-receipts/case-014-export/`,
`.superpowers/sdd/a4-largest-native-pilot-014/` and
`.superpowers/sdd/a4-pilot014-transport-20260906/`.
The canonical stage is consumed; case044, structural intake and all78 have not
run. A separate run-phase observer design is specified-only.

Repository observation: experimental
`4006244885c7cf34562fe54cbb76856142b0d244` now preserves CLM-0177's
compiler diagnostic under `a5/compile-phase-diagnostic-preservation/`.
Archive SHA256
`9b3e13c42c4942f0d2440b49c2340054a38b1811b8165d49721e95b9e3858158`
retains all 282 original members with independent and root package review.
The independently reviewed no-flatten design uses the existing compiler flag
and requires a tiny capability control before one full contrast. Its exact
implementation plan is under review; neither invocation has run.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope parser prerequisite and bounded diagnostic preservation; observed date
2026-09-06 UTC; reproduction reproduced for original parser and failed native
invocations, specified-only for the proposed next diagnostics; confidence high
for terminal observations with the stated limits; lifecycle S3 experimental
branch, S2 for next designs. Native feasibility, final115, H1, bounded solver
verification, full A4/A5, Council and integration remain open.


## CLM-0179 — No-flatten tiny observation and corrected retained-data gate

Repository observation: experimental `8541405` preserves all 325 original
members of the failed case014 package under
`evidence/s02-candidate-a-completion/a4/pilot014-heap-failure/`, archive SHA256
`31023b784fa91b797bd604269cd9e09f4def7d11f8cb8d9075999f2a8c8dcec1`.
Independent and root package audits admit failed preservation only. The
run-phase design is adopted at `85cc805`; the exact plan remains unexecuted
while its parent environment is made explicit.

Experiment observation: the no-flatten tiny control ran once at experimental
`2a33b45f647c4ada426ff840bc9033cbbe913bfa`, under the reviewed original plan
SHA256 `79325d985b5f58a62a5f490b0ac51f60c873afcb36775be3d95e372d144b2260`.
Compiler, recorder and actual outer exited zero; cleanup was unforced and
complete. Original stdout is 13,904 bytes, SHA256
`4638a8d77df74903912b7e023375116b5fdcbf62ea45cfd42246a8b9b1955ff2`.
The strict original intake failed its `Concrete=Box[int]` predicate and remains
unchanged at SHA256
`2c39b46364d588dbe78f1505a657f5df003051b8a59ad67c411193becd44a988`.
Independent and root review first admitted only that failed gate's evidence.

Source fact: the pinned analyzer resolves type applications before compilation
reaches the flatten flag. Its retained `Concrete` is a record whose `key`
field remains the type constant `Key`; the variable annotation remains
`Concrete`. The original expectation of a surviving generic application was
incorrect. The separate diagnosis and correction proposal has SHA256
`6d8a52ea54b5a70d1e2d6f0bd7bc72aa298c3139fc96af34febe31ed7a5703d0`.

Experiment observation: the independently reviewed data-only recipe ran once
in actual tool result f108b4/0. It changed the concrete-type predicate, tightened
the comparison with the existing flattened output and wrote a distinct intake;
all other original predicates and evidence fields remained unchanged. Corrected
intake SHA256 is
`f2d73ee627dd4025b7038149f2a5b99f78abb29a6e55fba44f1f4d8d121ed4aa`.
Independent review SHA256
`82ec3e4aa6aa0ce84c18eb7d2c8df25255076e46f97b67ea6fde96b4bbeef5c4`
validated the exact correction and 746 retained original pins. Root's separate
admission SHA256
`09eeff84e1f750b4ffae73dc44173aae058a5343f3bf64801be4480ab848e0ac`
admits corrected tiny capability only. It binds 284 files, including all 254
native originals. No compiler rerun occurred; the original tiny admission
path remains absent. Originals remain in the experimental worktree under
`.superpowers/sdd/a5-no-flatten-diagnostic-20260906/` and its hash-bound sibling
review files. Durable packaging and a separate reviewed full dispatch
supplement are pending; no full no-flatten invocation has run.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope failed-export preservation and corrected retained tiny capability;
observed date 2026-09-06 UTC; reproduction reproduced for the original tiny
invocation and data-only correction, specified-only for the full contrast and
A4 run-phase controls; confidence high within those evidence limits; lifecycle
S3 experimental branch, S2 for unexecuted plans. H1, paired compilation, solver
verification, canonical A4 exports, final115, Council and integration remain open.


## CLM-0180 — Full no-flatten compile produced oversized output

Experiment observation: under the independently reviewed full supplement at
experimental `a2a4b52a7dac09fd5f95e9531e0b3587faa50459`, the one full
no-flatten compiler completed. Compiler, integrated recorder, RH002 and actual
outer exits are zero. Cleanup was complete and unforced, with no timeout or
signals. All 31 original native responses are preserved, ending at actual
14ac9b/0. The unchanged command used Node old-space4096MiB and the existing
setup-inclusive900-second RH002 bound; no native retry occurred.

Original stdout is 199,868,611 bytes, SHA256
`ebd75615ca4f89ba5cb47c7a535206e990ef9162d53530b35f7c3a95b691cb8d`.
Original full NF005 intake exited2 solely at its128-MiB content limit, before
JSON structural acceptance. It remains immutable at SHA256
`66a4b45f3681592d755c4f85e20a055aa60c142a9f67c6958e86dd1301405f46`.
Neither exit0 nor a complete output hash establishes schema or solver readiness.

Independent review SHA256
`c61259897e4aef3f17b16781c913f6d67de6de6eeaaf7c3aad72b99f6f75d0f3`
checked 1,090 originals, both518-member source archives, four121-source
snapshots, all8,102 runtime file records and bytes, exact command/environment,
all native/data transports and cleanup. Oversized stdout was streamed for its
hash and never JSON-parsed. Independent index SHA256 is
`c5050c72c3b5f41267fb8102fcc2e66624d5adbbc007d24050948875801875c4`;
actual audit8fd6bc/0 is preserved in tool receipt
`bd7cb7fc77494853434b6761ad57a3a78b07ff3fd0cd1022f7d43f3ad2246c38`.
Root executed unchanged NF006 at49554c/0 and separately admitted failed
preservation only, SHA256
`a1e566d6737e18ea6c47249fa7c3bb8ead2c5becd64c16455a47c33b91f2f6b8`.

Whole recorder/descendant resources are4:59.84 elapsed and3,166,104KiB maximum
RSS. The integrated recorder's native interval is295.565673seconds, a distinct
scope. Node old-space is not a total RSS cap. Endpoint agreement is endpoint
evidence, not continuous monitoring. Originals remain under experimental
`.superpowers/sdd/a5-no-flatten-diagnostic-20260906/` and
`.superpowers/sdd/a4-producer-receipts/a5-noflat-full-20260906/` with the named
independent sibling records. Durable full packaging remains pending.

Repository and experiment observation: experimental `219125c` commits the
reviewed A4 run-phase exact plan, its explicit parent-environment correction
and actual materialized-source review. The seven isolated controls ran once:
identity0 and six diagnostic fault74 exits, each with a passing expected-result
intake and settled cleanup. Independent mock admission remains pending; no
paired native control or full observed case014 run has occurred.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope one altered-command execution and failed-size-gate preservation, plus
finite observer controls; observed date2026-09-06 UTC; reproduction reproduced
for those invocations, specified-only for a separate larger retained-data
inspection; confidence high within the stated original evidence limits;
lifecycle S3 experimental branch, S2 for the next inspection design. Original
size-gate failure remains failed. H1, paired compilation, solver verification,
canonical A4 exports, final115, Council and integration stay open.

## CLM-0181 — A4 native controls admitted and A5 tiny package committed

Experiment observation: all seven A4 mocks and four native controls have
separate independent and root admission. Both direct/observed pairs preserve
every compared ITF value and stdout/stderr byte except the exact permitted
timing fields. Successful traces contain 28 states; deliberate invariant
violations contain two. Their original native exits are 0/0 and 1/1, with
complete unforced cleanup. Both observed traces contain the nine required
public-phase records. These are finite observer controls, not canonical exports.

Independent controls review is SHA256
`bcaf5ce7f2de3d9703e93fe68427544da375477d51ef7fdf8cfd37df15075fdb`;
its original-data audit is a4b738/0. Root admission is SHA256
`fdfbe6bc4a968d0a06f4ad7cfbbc6ff8a298c1516a8cb1400c7f58992b76923f`,
actual825b16/0. Originals remain under experimental
`.superpowers/sdd/a4-run-phase-diagnostic-20260906/`. The success pair records
HEAD219125c; the violation pair records HEADe87018f. Their integrated source
and runtime identities match; these different historical HEADs are retained.

Repository observation: experimental
`e87018f3856e3a32b5eab0ab4a6e8c5ba405a914` commits the A5 tiny package at
`evidence/s02-candidate-a-completion/a5/no-flatten-tiny-retained-data/`.
Its 784-member archive is SHA256
`754b21e17d28db9f6e393844a14e020e236cf85acb9f0678c4ad99249c78845f`.
Independent standalone audit126edc/0 and original-byte comparisona985ae/0
preceded root package admission
`a35ae07cead424dd3802d45e44fb0fcfb82ce4be4f8ac330db48e86896902612`.
The original failed tiny predicate and separately corrected retained-data result
are preserved together; the package does not establish full compiler acceptance.

Experimental `39ebe4254c16aa9318c0fa6fab47a6b6ff994a35` adopts the source-only
retained256 design and plan. Independent source review is
`e27c6a6b7ecf5007d0e5b4df83ea4d914a2022d01ed634976f5ae3fa017d894f`;
root preparation adoption is
`96947d989af4c28e39e2e92af5c0053d44197fb19c0176f6b73db22660b81692`.
The proposed data process retains the original full structural predicates,
with a separate 256 MiB input ceiling and explicit memory/CPU/wall limits.
It has not parsed the oversized output. An independently reviewed predecessor
extension must bind the intervening A4 invocations before its dispatch.

At 2026-09-06T05:47:30Z, the one full A4 observed diagnostic had been
dispatched under the unchanged 900-second plan, with original tool
0fe6de/session59740 and root monitor cell556. Dispatch SHA256 is
`7278a93792fe0e6b44ffded85f69165c40ae7729ab3ee65dcbfc54e069799065`.
No terminal result is asserted here. The original transport owns subsequent
responses; an observation timeout does not authorize another invocation.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope finite A4 diagnostic controls and A5 tiny evidence preservation;
observed date2026-09-06 UTC; reproduction reproduced for the recorded controls
and package audits, specified-only for retained256 parsing; confidence high
within those limits; lifecycle S3 experimental branch and S2 inspection plan.
Full A4 exports, final115, A5 bounded verification/H1 and Council remain open.

## CLM-0182 — Full A4 diagnostic and retained A5 inspection failed

Experiment observation: the full A4 observer invocation at experimental
`39ebe4254c16aa9318c0fa6fab47a6b6ff994a35` terminated with authentic
8a48a7/134 after 40 original transport responses. Node exited -6; recorder
and outer exited134. No ITF was produced. Cleanup completed without timeout
or forced signals. The seven phase records show completed load, parse and
typecheck, followed by entry into `runSimulator`; they do not identify its
internal failing operation. Independent review is SHA256
`6af66272c392da2fcbffd0956bf40007b1ef194eb90859d0c9146a4be6dc8925`.
Root admission `577bc22c4c71b3f488bc75fed23c995d6559d1fbb0d9dbbae1c1387af38bda9d`
(actualb73d8c/0) accepts failed preservation only. Originals remain under
experimental `.superpowers/sdd/a4-run-phase-diagnostic-20260906/`.

The separately reviewed A5 retained256 inspection ran once against the
unchanged 199,868,611-byte compiler output. Its authentic terminal is
c51390/2. Strict decoding reached the import predicate, which failed because
the retained compiler declarations are topologically ordered rather than
in source order. Subsequent structural predicates were not admitted. The
multiline Python command also made the RH002 timing report unparsable:
the original resource object remains null and resource completeness false.
The before/after input manifests match; owned cleanup completed without
timeout or forced signals. Independent review is SHA256
`fd877f57823dcc8e77261898568056aa278f4b6358c4ec11d995e2dfafcd62ba`.
Root admission `6419842633fffbe66294c0c2069116df4e456460dfd0bf7baba56722cb807816`
(actual537f3c/0) accepts failed preservation only. Originals remain under
experimental `.superpowers/sdd/a5-no-flatten-retained-256-20260906/`.

Source investigation supports two separate next steps: literal A4 case
wrappers that retain the canonical corpus, and an A5 comparison-order plus
single-line command-envelope correction under unchanged resource limits.
These are source proposals, not successful exports or inspections. The
original failures remain immutable; no native compiler retry occurred.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope one full A4 observer and one retained256 data inspection; observed
date2026-09-06 UTC; reproduction reproduced for preservation audits,
specified-only for remedies; confidence high within those limits; lifecycle
S3 experimental evidence and S2 remedies. Full A4 exports, final115, A5
bounded verification/H1 and Council remain open.

## CLM-0183 — Corrected retained-output inspection admitted

Experiment observation: the separate A5 retained-order256 inspection completed
once with authentic terminal f7e609/0. The finite correction compares the37
original declarations in numeric source-ID order and preserves the final3
generated selections. All original structural predicates passed: exact13
imports,40 declarations, full schema/ID checks, PilotState annotations and
generated-selection bodies. No complete operator-body equivalence is claimed.
The single-line bootstrap also allowed complete original RH002 resource capture.
Child and outer exits were zero, with unforced complete cleanup. The before
and after input maps are identical at SHA256
`aa2ba14eff5e04f4b0e6c3507558108902b9aaddbd44960c06c5a4d5fa9afe55`.

The unchanged compiler output is199,868,611 bytes, SHA256
`ebd75615ca4f89ba5cb47c7a535206e990ef9162d53530b35f7c3a95b691cb8d`.
The genuine corrected intake is SHA256
`63ab5fe7eed460635854fa193e68d2a5abde6c6ae5ff9c59d16884b2111e8945`.
Independent review `35c9e3182b5fe06376791264f71b9c095b2959c3a17e789f962bbfd1f0258361`
and root admission `e2cf3fec6546e57aea5e0206c7757bfa3f5881f39bd7f7216916230a74a7a494`
(actual0e4a78/0) admit retained altered-command structure only. Originals are
under experimental `.superpowers/sdd/a5-no-flatten-retained-order-256-20260906/`.
The original128/256 failures remain unchanged; the compiler was not rerun.

Repository observation: root released the A4 source freeze only after the
successful corrected-data after capture. The adopted literal-wrapper plan
`c515e02bf80bcf3c620c9fbb570d9d9d834ba3256b786bb37bd538f436ae56b7`
is in implementation. Root also reproduced a necessary exporter fix: consuming
raw_events indexed a generator directly and raised TypeError before validating
the trace (actual19cb3b/1). The reviewed one-line list conversion and its direct
regression are part of that source change. No new canonical export is admitted.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope one bounded retained-output inspection; observed date2026-09-06 UTC;
reproduction reproduced for original execution and independent preservation
review, partially reproduced for A4 implementation; confidence high within
those limits; lifecycle S3 experimental branch. Paired flattened compilation,
solver/H1 evidence, full A4 exports/tests and A4/A5 completion remain open.

## CLM-0184 — Literal-wrapper source checks and copied compiler view

Experiment observation: A4's frozen literal-wrapper map
`968076cf4514b07ce153538c82b5d9339748655523797a79e33defdc8c0a45f8`
contains121 files. The one new aggregate parse ended at34053d/0 and produced
18,729,611 bytes of IR. The bounded comparison ended at8990d8/0 and checked
all78 literals,98 modules,1,534 declarations and16,514 resolved references
against the admitted original IR. Separate capture and binding/patch reviews
passed. The fresh non-package exporter regressions passed58 tests; four package
nodes remain deferred, and all174 original node identities remain in177.

The aggregate native typecheck ended at e841a5/0 after620.306 seconds, within
the unchanged900-second limit. Native and recorder exits were zero, streams
empty, all source/runtime pins stable, and cleanup complete and unforced.
Root intake d5d602/0 and independent review
`3b23a9b4c7fde5b65314ec50311de2e463ed7aac552d307937447df7b8442926`
admit that source check. The original12 transport responses are retained under
experimental `.superpowers/sdd/a4-literal-v1-20260906/transport/literal-v1-typecheck/`.
The first root intake's snapshot-path error339fff/1 remains preserved; no native
retry occurred. Other required native source suites and joint Gate1 admission
remain open. No canonical ITF or all78 export acceptance follows from these results.

Repository observation: A5's unused-helper view was prepared once at844074/0.
Its24-file manifest is
`f8387d7848ab38c447c5862f774135305821666744e1a84694c394636b6f74f0`.
The actual three scripts match the independently reviewed derivation. The view
preserves the two prior AuthorityKey imports and removes the same13 unused
declarations,3,835 bytes per swap copy. All other bytes,30 original source pins,
121 A4 source pins and the complete copied import closure remain exact.
Independent materialized-view review
`312a4292e61f94ab083a96c323c8c48d3d15550856f983498b8e3252ff430b92`
and root admission
`c8dda24efb006253e23c9dc7e88942c703b89f6bc022eb2d817bc7b8d5a1a124`
admit copied preparation only. Originals are under experimental
`.superpowers/sdd/a5-factoring-receipts/pilot-unused-helper-view/`.
Its native typecheck, prefix/witness checks, paired default compilation,
size comparison and positive/negative offline checker stages remain unexecuted.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope the cited A4 source checks and A5 copied preparation; observed date
2026-09-06 UTC; reproduction reproduced for the named original runs and scoped
independent checks; confidence high within those limits; lifecycle S3
experimental branch. Final full suites, package controls, A4/A5 acceptance and
H1 remain open.

## CLM-0185 — Source gate admitted; canonical export still fails

Experiment observation: all four required literal-wrapper source checks passed.
The aggregate typecheck is recorded in CLM-0184. Export, installment and swap
test suites ended at 1852ba/0, 545a4c/0 and 3e4d6c/0, with exactly 12, 17 and
17 passing tests. Each has a separate independent capture review and root
intake under experimental `.superpowers/sdd/a4-literal-v1-20260906/`.
Their source/runtime endpoints remained stable, and RH002 cleanup was complete
and unforced. The root source gate `review-source.md` is SHA256
`bf6c86b7c5faec54e65076a66aec5607e1e3f50247e4fee9dfd90739a2e6a5f6`.
Experimental commit `fbea1cee5de497a3e58fe3081bb49b4059edd97b` retains all
121 source-map bytes and includes the 78 literal wrappers, exporter correction,
regressions and adopted source plan. This admits the source gate only.

The first new canonical case014 export ended at actual 2fa302/134. Its native
process exited -6 with an explicit JavaScript heap exhaustion and produced no
ITF. RH002 recorded 329.2869 seconds, complete resource capture, no timeout,
stable source/runtime endpoints and ineligible receipt status. The original
inner receipt is SHA256
`11a405604255e6732717703132907a2224b6d51c1eaed7e377700115ebf20351`;
all original transport responses are retained in `transport/case-014/`.
Independent failure review and source investigation are pending. Case044 and
the remaining canonical exports have not been launched on this source epoch.

Repository observation: the A5 copied view now has a frozen 36-input dispatch
at the same full experimental commit. Its `dispatch.json` is SHA256
`e4497cc8193db857873898b0321c14c78fbafcb671ba59b3f76c87d9cea67d90`.
Independent freeze review
`f088b6da84df9c820660d4743a7602cb379969bc38c46a5307b784a7193219fc`
passed; root read the complete report at 58c54a/0. No native A5 stage follows
from that source-only review. The frozen view and source commit are held for
the finite ten-stage experiment.

Claim metadata: sources SRC-0031, SRC-0033, SRC-0035; authority experimental;
scope four A4 source checks, one failed canonical export and A5 dispatch
preservation; observed date 2026-09-06 UTC; reproduction reproduced for the
named native commands and scoped reviews, specified-only for new A5 native
stages; confidence high within those limits; lifecycle S3 experimental branch.
All78/1557 export acceptance, final full suites, package controls, paired
flattened compilation, solver/H1 evidence and A4/A5 completion remain open.

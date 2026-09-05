---
id: moriarty.research.journal
type: decision
title: Moriarty research journal
status: active
updated_at: 2026-09-05T08:10:00Z
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

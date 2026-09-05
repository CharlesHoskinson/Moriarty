# Wiki log

## [2026-09-04] checkpoint | Complete S01 and begin S02 contract

Independent re-review approved the settlement correction with no remaining
material finding. Closed S01's seven bounded specification tasks and retained
the unmechanized theorem and signing-denied local experiment limits. Updated
the program register and index to the reviewed S02 contract plan. No model
comparison, architecture selection, or later release gate is claimed.

## [2026-09-04] correction | Separate settlement process from receipt

Corrected the S01 alias against XML v1.3: `settlement` names the process, while
`SettlementReceipt` is evidence about a step. Preserved the prior manifest
byte-for-byte and its SRC-0032 provenance; registered the revised manifest as
SRC-0034. The correction does not change Core or semantic scope. Independent
re-review remains required before closing S01. Also clarified that the user
directed Quint instead of TLC and the reviewed design selects Apalache.

## [2026-09-04] experiment | S01 evidence-only intent-safety freeze

Recorded the architecture-neutral S01 terminology, observation model, hard
predicates, preferences, assumptions, and candidate-unmechanized theorem. The
validator recomputed all ten local package predicates. The local swap baseline
was `VALID`, the extra-effect mutant was `UNAUTHORIZED_EXTRA_EFFECT`, and both
certificates denied signing. No production signing request was attempted. The
transition preserves semantic scope `0.0.0-e00.2` and its frozen digest.
Mechanization, authenticated effect completeness, runtime `SignBeforeResolve`
verification, proof and backend correspondence, ledger execution, ACTUS, human
pilots, the 24 prompt release gates, and Task 7 review remain open. S02 retains
the four-way architecture decision and uses Quint with Apalache, not TLA+/TLC.

## [2026-09-04] audit | Prompt v1.3 and initial S01 execution

Audited the focused XML assignment, approved S01 design, and implementation plans.
The [audit](../docs/superpowers/reviews/2026-09-04-moriarty-v1.3-execution-audit.md)
records repository observations, sample-code probes, and unresolved freeze obligations.
The S01 OpenSpec contract begins implementation. The theorem and aggregate
evidence gates remain open. The prompt, approved design, and Core snapshot retain their pins.

## [2026-09-03] research | ACTUS terminal completeness prompt

Captured the complete public ACTUS site and documentation sitemap with
Scrapling, pinned eleven official and three comparative repositories, and
inventoried 32 taxonomy rows, 18 executable contract types, and all 277 public
reference fixtures. Direct code inspection identified the public Haskell
implementation's four missing executable types, seven excluded fixtures,
analysis-date omission, three-field comparator, and binary32 payoff downcast.
The official Java core remained access-controlled and unused. Revised the
focused prompt to version 1.3 with typed ACTUS packages, two independent
semantics, shared Core and Compact compilation, full present-field comparison,
24 release gates, and compatibility-only language. Semantic scope
`0.0.0-e00.2` did not change.

## [2026-09-03] research | CAKE, NEAR Intents, and Ethereum intent lifecycle

Acquired the 14-link CAKE Working Group corpus and all 68 official NEAR Intents
documentation pages with Scrapling, including the full documentation aggregate
and two OpenAPI specifications. Reconstructed the deployed objective, quote,
authorization, Verifier, relay, bridge, fill, payout, cancellation, refund, and
finality boundaries. Revised the focused research prompt to version 1.1 with 33
intent data contracts, current ERC-7683 versus OIF version separation, resolver
snapshot obligations, delivery deduplication, asynchronous cancellation, and
cross-domain fulfillment tests. The semantic scope remains `0.0.0-e00.2`.

## [2026-09-03] design | Moriarty semantics and intent research prompt

Added a focused XML assignment for Moriarty semantics, intent correctness,
verified compilation, Compact and ZKIR realization, and the standard developer
interface. Acquired the CAKE framework and 28 official ERC and EIP pages with
Scrapling. Preserved exact status, source date, path, and SHA-256 evidence. The
assignment uses twelve evidence-gated sprints and makes no semantic change.

## [2026-09-03] audit | Evidence and SDK gate hardening

Closed the independent review findings. The disclosure validator now handles
whitespace, excludes comments and strings, and rejects unnamed compound
expressions. Complete compiler-interface metadata is checked against the
Moriarty manifest. Clean reproduction and current-checkout evidence now have
separate identities. Sprint manifests bind package, scope, outputs, self-hash,
and package-specific gates. The SDK validator applies its schemas, requires
exact counts, closes component references, and recomputes its complete index.

## [2026-09-03] experiment | Clean Compact reproduction and complete SDK contracts

Reproduced E00 from a clean archive and fresh environment. Forty-three focused
tests passed, 1,000 traces had zero divergence, and all semantic, Compact,
compiler-manifest, ZKIR, and negative-control digests matched. Recorded the
Compact source-map path-sensitivity footgun. Added an independent Moriarty
visibility-manifest validator with missing and additional disclosure controls.
Specified 65 SDK components, 28 canonical data contracts, two JSON contract
schemas, and a 17-component minimum safety spine. Preserved the Grok, Sol, and
exact Fable 5.1 Council advisory and its requested changes.

## [2026-09-03] decision | Evidence-gated sprints and complete SDK scope

Superseded calendar-based progress with evidence-gated sprints. Added the
research journal and an iteration-scoped semantic ledger. Classified E00 as a
narrow Compact DSL feasibility result rather than general language evidence.
Expanded the SDK boundary to the complete authoring, compiler, analysis,
packaging, verification, wallet, chain, and operations development system.

## [2026-09-03] experiment | Moriarty Core atomic-swap stop test

Implemented the finite E00 Core subset and canonical two-token swap. Generated
fixed-state Compact, an artifact manifest, and an independent transition
machine. One thousand unique traces produced zero divergence and zero invariant
failures across 18 required deadline-boundary cells and both terminal-expiry
rejections. Follow-up semantic review added manifest-driven time guards and
timeout priority, canonical refund ordering for all accepted party names,
small-deadline coverage, and exact Compact integer validation. The first Compact
compile reproduced an undeclared-disclosure
footgun for the decision argument. The corrected source lists and applies the
public disclosure explicitly; the failing source and diagnostic digest are now
preserved. Compact emitted four ZKIR 3.0 circuits, and the pinned mock compiler
accepted all four. Constructor values, real keys, and proof generation remain
open.

## [2026-09-03] decision | DeFi Kernel prompt, council, and graph

Reproduced the 47/72 1-NN, 50/72 3-NN, Jaccard, and exact pair-rate results.
Ran tool-free round-1 proposals with Grok, exact Fable 5.1, and GPT-5.6 Sol,
then a blinded round 2; recorded Fable's budget-exhausted second round as an
abstention. Added the 12-workstream XML prompt, gated 90-day sprint, seven-family
Marlowe mapping, and the library-only stop path. Built a six-source Moriarty
decision graph with 46 nodes, 45 directed edges, and zero missing endpoints.

## [2026-09-03] ingest | Repository-verified taxonomy update

Preserved the decision-bearing claims from the user-supplied updated run as
SRC-0017. Reproduced the DeFiFormal pair, canonicalization, obligation, roster,
and v3 self-test counts at commit
`8ae0bbfaa3193078d1cabf6999db1382985b7f95`. Replaced M4+ as the top-level
decision with M2+M3 human-facing families and facets plus an M5 formal behavior
profile. Added a 72-row family/facet crosswalk while preserving the M4+ file as
historical input.

## 2026-09-02 — Moriarty decision packet

- Added the consolidated decision study and the standalone stakeholder
  pre-read.
- Added an experiment handoff for the compiled Compact/ZKIR escrow vertical
  slice.
- Kept Marlowe as the upstream and migration name; Moriarty is the new language
  and repository name.

## [2026-09-02] ingest | Marlowe organization and live documentation graph

Refreshed and pinned all 36 `marlowe-lang` repositories, acquired all 100 live
documentation sitemap paths with Scrapling, added the official online entry
points, and built the combined AST, semantic, documentation, and provenance
graph. The graph contains 8,864 nodes, 10,888 edges, and 120 hyperedges. Its
1,510 dangling endpoints and 265 undirected relation collapses are recorded as
health limitations rather than hidden.

## [2026-09-02] ingest | DeFi taxonomy report and local corpus reconciliation

Preserved and extracted the user-supplied 25-page taxonomy PDF. Reconciled its
public-access limitation against the authorized clean local DeFiFormal checkout
at commit `8ae0bbfaa3193078d1cabf6999db1382985b7f95`. Reproduced the 72-row corpus,
60 constructions, 1,259 obligations, 570 covered obligations, and 689 residue;
generated a one-to-one construction roster and draft M4+ crosswalk.

## [2026-09-02] design | Canonical Moriarty patterns for the 72-row roster

Assigned every DeFiFormal row to a bounded canonical application pattern and
added 13 surface-language strawmen. D01–D11 each receive one reference pattern;
D12 splits into an event-contingent market and a delegated-curator vault. The
sketches separate provable Core obligations from oracle, bridge, custody,
identity, solver, validator, and legal capabilities.

## [2026-09-02] ingest | Research assignment and LLM Wiki method

Initialized the research repository around the upstream Marlowe modernization
assignment, preserved the controlling prompt, installed the acquisition
environment, and ingested Karpathy's original LLM Wiki idea file as `SRC-0001`.

## [2026-09-02] decision | Rename to Moriarty

Renamed the repository and proposed language to Moriarty. Marlowe now refers
only to the upstream source language, implementation, and migration baseline.

## [2026-09-02] ingest | Midnight and active Compact repositories

Acquired the official Midnight documentation corpus with Scrapling, enumerated
and cloned all 74 public `midnightntwrk` repositories, followed the official
Compact relocation, and cloned all three public LFDT Minokawa repositories.

## [2026-09-02] experiment | Moriarty escrow to Compact and ZKIR 3

Built Compact compiler 0.34.100 from pinned source, compiled a finite Moriarty
escrow lowering with the ZKIR 3 backend, passed six acceptance tests and 44 ZKIR
library tests, and mock-compiled all three generated circuits. Full proof tests
remain dependent on external `MIDNIGHT_PP` parameter files.

## [2026-09-03] research | Complete NEAR Intents documentation and prompt council

Acquired all 68 official NEAR Intents sitemap pages plus both published OpenAPI
documents with Scrapling. Separated internal-ledger execution, source fill,
claim, destination finality, and destination spendability. Grok, Sol, and
Fable reviewed one frozen prompt draft. The single correction pass produced
prompt version 1.2 with execution-state authorization, cryptographic lifecycle
bindings, typed verification certificates, and a separate confidential trust
profile. Semantic scope `0.0.0-e00.2` did not change.

## [2026-09-05] checkpoint | Record branch-only S02 progress and Council restart decision

Recorded the effect foundation candidate and corrected receipt commit
`1bd4bff04fc24855b1c45ff95b1eab137909434d`. Recorded the exact-parent
consumption foundation at `8b905114c1cec79faf555974c0267f183ca31399` and its
honest receipt whitespace correction at
`6a60a645c03acad83b7cbc6b85d43996cd40ca65`. All three commits remain only on
`s02-model-comparison`. They are not integrated or Council-approved.

Recorded the 2026-09-05 user decision to authorize the narrow Foreman runtime
binding fixes, GitHub publication and merge, the documented official model-route
limitations, and Council resumption. Runtime implementation remains pending.
No Council review was dispatched. Full S02 and S03 through S15 remain
incomplete. Historical log entries remain unchanged above this entry.

## [2026-09-05] assignment | Investigate Foreman PID-namespace degradation

Created the [Fable research assignment](../deliverables/foreman-grok-4-6-pidns-deep-research-prompt-2026-09-05.xml)
for the user's separate session. It includes the observed launcher warning,
source and binary context, protected live work, and the official 42-page Grok
4.6 card with its preserved PDF digest. The assignment requires read-only
diagnosis, complete-card coverage, and explicit scrutiny of cleanup claims.
The XML parser accepted the assignment. No root cause, remediation, Council
approval, or containment guarantee follows from creating this prompt.

## [2026-09-05] checkpoint | Resume the original v1.3 workstream

Recorded Foreman candidate `ac7c2deff6e39144c29836611ee85800ffed41c8`
and its incomplete status in research-journal claim CLM-0139. Task 1 needs
correction. Task 2 and all requested Council gates remain pending.
Recorded the scanner resource-limit refusal separately from a secret finding.
The unchanged scanner passed after the unused root dependency install left
the worktree. The install remains recoverable in the external run directory.
No security policy changed, no GitHub merge occurred, and no gate passed.

## [2026-09-05] checkpoint | Record Council repair escalation

Foreman correction candidate `b65d8ad7d30dcc9d1563750fc45608ddd8f67219`
completed its focused worker gate. The independent full package check then
failed with five ESLint errors. Foreman Endstop recorded `Escalated` with
reason `verify_blocking_after_correction` at 2026-09-05T03:21:48Z.
The contract permits no further repair dispatch without successor authority.
No GitHub merge or Council gate passed.

Direct filesystem inspection found the ignored Task 1 and correction test
receipts. Earlier absence claims based on file discovery are superseded.
The original worker report and receipts remain preserved in the external run
archive. The controller did not reconstruct historical test execution.

Added the unapproved S02 authorization/recovery type sketch. Its main-review
hold names missing transaction-time bindings, premature choice identifiers,
and incomplete Core carrier comparison. No Quint model logic was added.

## [2026-09-05] execution | Authorize and dispatch the successor repair

Recorded the explicit successor authorization and contract in CLM-0140.
Preserved the predecessor worktree and reports. Created a fresh branch from
Foreman main with the separate PID-namespace documentation. Replayed all five
repair commits with an unchanged range comparison. Dispatched queue task 1459
for the known lint repair and armed the per-lane stall watchdog. The monitor
does not grant gate approval or reset contract limits. S02 type-sketch review
continues in parallel. No S02 model logic, GitHub merge, or Council verdict is
claimed by this entry.

## [2026-09-05] planning | Correct the S02 observation carrier proposal

Corrected the three issues in the authorization/recovery type sketch. The
observation and verification record now bind transaction time. Core choice
identifiers remain strings, with explicit absence distinct from zero in a
bounded initialized map. Error and warning carriers preserve the declared
frozen Python fields; separate proposed validators constrain emitted results.
Compared the proposal with `moriarty/core.py` and the reviewed observation
design. Requested explicit type-sketch signoff under the Quint workflow.
No model logic, simulation, model check, or Council verdict was produced.

Foreman lint repair is committed at
`2007bad9e466ccc4ea51a20ee6708493af093e50`. The worker's nonzero exit remains
recorded separately from its successful deterministic gate and fresh reports.
Independent queue task 1460 passed the complete Council package check.
Queue task 1461 starts the missing single-review execution implementation.
These intermediate results do not complete the Foreman repair or S02.

## [2026-09-05] recovery | Preserve the unfinished review runtime

Recorded CLM-0141 and checkpoint fact 35. Preserved the 25-file Task 2 draft
in Foreman commit `1fb1548d02c98e81ce9b7953a26cca2d2750bdf8` and archived
its incomplete reports and raw test receipts. Selected vendor terminal
metadata identifies the 75-turn limit as the stop reason. Dispatched bounded
completion task 1462 through the same successor contract's resume allowance.
The watchdog was armed. These are historical dispatch facts, not continuing
liveness, passing checks, independent review, or publication claims.

## [2026-09-05] synthesis | Map the four S02 semantic alternatives

Added CLM-0142 after comparing XML W1/S02, the reviewed four-representation
design, the closed registry, and the branch foundation boundary. Retained
all eleven witness identifiers and both additional recovery paths. Recorded
the stale ten-scenario phrase without changing the pinned review candidate.
All four alternatives remain unimplemented and undetermined. This source
comparison is not model execution, correspondence evidence, or Council advice.

Foreman fixture recovery is preserved at
`940034d008c094d2bbce3f6a954e7692327216cd`. Correction task 1463 addresses
confirmed dispatch-boundary and public-output findings. Checkpoint fact 36
records the handoff. Neither that dispatch nor its watchdog grants approval.

## [2026-09-05] reset | Return the workstream to Moriarty

Recorded CLM-0143 and the Moriarty restart assessment after explicit user direction
to stop Foreman development. Fetched Foreman main and confirmed it already matches
GitHub. Preserved its repair branch and unrelated changes. Dropped the obsolete
repair obligations without marking their work complete. Retained Moriarty Council
reviews as a separate obligation and identified S02 common authorization/recovery
as the next implementation, followed by four distinct semantic alternatives.
The user subsequently delegated the corrected type-sketch decision to the three
requested formal-methods experts and reaffirmed full-program completion.
That decision remains pending. No model logic, architecture
selection, Council verdict, or release gate was added by this reset.

## [2026-09-05] correction | Remove superseded direct-approval instructions

Aligned the type sketch and wiki index with the user delegation already recorded
in CLM-0143 and `docs/MORIARTY_RESTART.md`. The three-expert design decision remains
pending; the old direct-human-approval request must not block continuation.
This is an instruction correction, not design consensus or implementation evidence.

## [2026-09-05] correction | Distinguish the current full program from its predecessor

Corrected the wiki schema's obsolete 18-output count and labeled the index's
twelve-package link as the preceding program. XML v1.3 `required_deliverables`
and `sprint_sequence` require D01–D22 and S01–S15. Preserved the historical package
manifest and reports; their presence is not evidence that the current program,
including ACTUS, is complete. This is source-fact alignment with SRC-0031,
not a new experiment or a passed gate.

## [2026-09-05] decision | Reconcile the three S02 design proposals

Added SRC-0035 and CLM-0144. Preserved the identical frozen prompt, all three final
public proposals, selected metadata, and source digests. Adopted the common design
with explicit majority choices and preserved dissent. Superseded the earlier type
sketch and queued observation tests before authorization implementation. Private
reasoning and credentials were not imported. This is delegated design signoff,
not strict Council admission, architecture selection, or an implementation gate.

## [2026-09-05] implementation | Execute the first common Quint unit

Recorded CLM-0145 for S02 branch commit
`45883ba949f18f580b14c6cd17303b719ef8f700`. Preserved initial failing tests,
emission regressions, independent review findings, and the corrected 37-test
observation carrier with sampled declared-example witnesses. The source remains
on the experimental branch pending further review and integration. Authorization,
recovery, candidate execution, correspondence, and all affected gates remain open.

## [2026-09-05] implementation | Enforce branch policies and complete plan bindings

Recorded CLM-0146 for branch `320dc53` and clean independent Astra review at
`a6f9c30`. Preserved meaningful RED and passing policy/regression receipts.
Recovery requirements are now explicit at `7cccc5a`, with a clean independent
transcription review. These are unsigned constraints and required scenarios,
not executed signing/recovery or passed S02 gates. Next work is the authority
lifecycle, with the full XML goal active and Foreman development still closed.

## [2026-09-05] implementation | Execute persistent symbolic signing

Recorded CLM-0147 for corrected branch implementation `f2941d4`. Both signing
profiles and independent concurrent signer checks execute; exact freshness
constraints and parent registration are tested. Independent Astra review and
admission-correction re-review are clean. Preserved RED/green source commits,
raw sampled evidence, and the asynchronous RED timing disclosure. Next is
per-operation verification and atomic lifecycle commitment, not another design
vote or Foreman repair. No financial recovery or S02 gate is claimed.

## [2026-09-05] implementation | Execute the atomic settlement envelope

Recorded CLM-0148 for source `46fe589` / evidence `0d5926b`. Both profiles execute
symbolic signing through verified atomic settlement from prefunded swap escrow.
Separate adversarial tests exposed an invented Core result on cancellation;
the correction and complete RED/GREEN reports are preserved. Independent source
and evidence review is clean. Candidate semantics and cryptography are not
derived by trusted validity flags. Next are classified rejection, parent races,
and signed recovery, with no S02 or Council gate claimed.

## [2026-09-05] implementation | Retain classified rejected attempts

Recorded CLM-0149 for corrected source `38cf13d` and evidence `4f3bb75` on the
S02 branch. Rejection preserves original observations and proofs without moving
money. A review-found diagnostic issue was reproduced before correction; source
re-review is clean. Corrected deterministic, sampled, and regression receipts
are pinned. The parallel installment lifecycle remains in progress, with no
financial recovery, S02 acceptance, Council gate, or architecture choice claimed.

## [2026-09-05] implementation | Execute installment races and signed recovery

Recorded CLM-0150 for source `d1c475f` / evidence `f4c7bc5`. Both profiles execute
the initial race, retain the stale loser, and reach either full payment or
separately signed recovery of ten or five. Independent root runtime checks and
source/evidence review are clean. Explicitly retained the prefunding, fixed-time,
trusted-verifier, sampling, and development-archive limitations. Prepared the
transition to actual Candidate A semantics; Council and all XML gates remain
open. Released-tool inspection is recorded in `docs/COUNCIL_REVIEWS.md` without
reopening Foreman repairs.

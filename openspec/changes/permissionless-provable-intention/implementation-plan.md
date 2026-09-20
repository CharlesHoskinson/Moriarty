# Implementation packages

**Consolidated schedule, 2026-09-19:** This file retains detailed requirements and historical planning. The [single U0–U7 roadmap](../../../ROADMAP.md) and [consolidated proposal](../consolidated-language-kernel/proposal.md) control current design and scheduling. Resolve former P/C/K owners through [traceability](../consolidated-language-kernel/traceability.md); do not dispatch this as a separate queue. Objective requirements and evidence remain in force.


Status: specified-only. This plan supplies bounded briefs, not an active campaign or completed compiler. Dependencies express technical needs, not administrative permission to use Moriarty.

## P0 — Recover the target and freeze the correspondence matrix

**Objective:** establish the exact source/Core-to-ZKIRv3-to-Midnight route and its proof premises.

**Inputs:** product contract; consensus v4; PR17 at `ebb662c`; existing source `/5`; preserved September13 jet/ZKIR studies. The original architecture worktree contains research absent from current main. Recover only identified source artifacts with original commit and hashes; do not merge unrelated branch changes.

**Owned outputs:** proposed `experiments/moriarty-language/spec/certified/target-manifest.json`, `correspondence.md`, `phase-effects.md` and a new P0 deliverable. These paths do not exist yet. The manifest records actual compiler/ledger/verifier versions, emitted instruction surface, trusted boundaries, source producer checks and adversarial witness premises.

**Acceptance:** identify every required public-input/commitment binding, type/range constraint and phase effect. Compare older PR17 semantics with actual emitted instructions. Establish the exact reproduction commands from the pinned toolchain. Do not call an unrun or unavailable command passing.

**Stop/result:** every unmatched boundary has a named open obligation. A version string, honest witness or Agda module parameter is not proof of correspondence. P0 may reveal a target blocker without weakening MOR requirements.

## P1 — One certified arithmetic implementation

**Objective:** certify checked UInt128 addition as a first tractable primitive candidate, including overflow rejection and logical work.

**Inputs:** P0 target/observation relation; existing exact arithmetic evaluator; declared input ranges and field embedding. The primitive choice is a proposed first experiment, not a frozen whole-language basis.

**Owned outputs:** proposed certificate/specification and target-fragment files under `spec/certified/` and `src/certified/`, focused tests, and a P1 deliverable. No plugin, custody, campaign, historical evidence or roadmap writes.

**Acceptance:** preserve host/reference outputs, definedness, rejection and logical work. Establish target constraint soundness for arbitrary accepted witnesses and completeness for valid inputs. Check zero, maximum, carry boundary, overflow, out-of-range inputs and altered witnesses. Discharge caller constraints and PR17 applicability premises. Publish theorem assumptions and target cost separately from finite tests.

**Verification:** use the existing `npm --prefix experiments/moriarty-language run typecheck` and `npm --prefix experiments/moriarty-language test` for affected host code. P0 defines the pinned target/proof-checker commands. Host regressions alone cannot close the certificate.

**Stop/result:** report tested-only or conditional evidence where a theorem/backend premise remains open. Never relabel a benchmark as certification.

## P2 — General supported-program proof and execution path

**Objective:** compile a new supported financial program to ZKIRv3 and verify its complete intent-bound effects on Midnight.

**Inputs:** P0; certified primitives or explicit remaining trust assumptions; MOR-001..012; current source/Core/kernel contracts and source-to-ledger lifecycle plan.

**Owned outputs:** separately enumerated lowerer, proof-statement, verifier-binding and ledger-adapter files. Freeze exact ownership after P0 identifies the actual route; do not invent an existing adapter package. Keep custody/network changes in a distinct reviewed implementation package.

**Acceptance:** a clean external developer environment needs no project metadata. Bind program/semantics, relation, intent, domain, predecessor/history and all phase effects. Include a program beyond fixed loan/swap fixtures and two independent proposal paths. Check authority/replay/fees and persistent residual liabilities for full, partial and failed outcomes. Retain native transaction, finality and readback evidence for the selected Preview target.

**Stop/result:** unsupported proof enforcement, target compatibility or phase mapping stays open. Local success does not authorize a public claim of complete proof-carrying settlement. Project-funded test runs use internal resource controls; independent developers do not inherit those records.

## P3 — Obligation reports and exact advisory checks

**Objective:** expose missing assurances and provide useful refinement diagnostics over one frozen Core judgment.

**Inputs:** P0 semantic bindings; AEO-001..003; consensus fragment/domain/status definitions.

**Owned outputs:** declared new report/checker files under the successor or certified source tree, their tests, the fragment specification and a P3 deliverable. The brief names exact files before execution.

**Acceptance:** versioned extraction coverage; missing categories fail completeness; unestablished correspondence stays visible. Freeze positive, negative and mutation fixtures. Preserve checked UInt64/128 arithmetic, short-circuit semantics and feasibility/vacuity. Replay counterexamples. A disagreement on the same judgment fails the checker. Publish unsupported/timeout coverage and usefulness against the stated corpus.

**Verification:** existing package typecheck/tests plus newly implemented targeted commands, named before a completion claim. SMT discharge remains advisory Core-formula evidence, with separate target assurance.

**Stop/result:** defer unhelpful diagnostics or any work that displaces the primary compiler/proof path on a single implementation track.

## P4 — Resource and private composition

**Objective:** preserve consumable receipts, affine authority, persistent liabilities and private predecessor guarantees across supported program composition.

**Inputs:** P0/P2 relations; APSS findings; MOR-006..008/012.

**Owned outputs:** explicitly versioned resource rules, composition proofs, kernel integration and test corpus in a separate package.

**Acceptance:** no duplicated receipt, enlarged authority, dropped debt, unframed effect or hidden assumption. State public disclosures and private witness premises. Check denied/unused authority, partial discharge and concurrent replay. Show which privacy and history properties are proved versus assumed.

**Stop/result:** avoid blanket exactly-once treatment of permissions and promises of unconditional liveness.

## P5 — Typed holes and bounded synthesis

**Objective:** improve authoring only after P3 demonstrates useful exact checks.

**Inputs:** immutable specifications, typed-hole fragment, fixed search budget and manual-template baseline.

**Owned outputs:** a scoped authoring-tool package and its evaluation corpus.

**Acceptance:** generated terms pass the ordinary pipeline; no weakening of signed intent, fees, reserve or hard constraints. Measure success, timeout, unsupported cases and effort relative to manual templates. Search failure is not proof of impossibility.

**Stop/result:** retain manual authorship as a first-class public path. Defer synthesis when the measured benefit is absent.

## P6 — ACTUS and broad DeFi conformance

**Objective:** implement and qualify the full retained financial behavior inventory through the public language, not a closed catalog of admitted programs.

**Inputs:** P0 target matrix; P2 general relation; P4 composition for dependent cases; MC07 and SP07/SP08/SP11; pinned ACTUS fixtures, 32 taxonomy dispositions, DS-01..DS-07, 72 original DeFi rows, DA01..DA24 and adopted supplemental cases. Source and oracle analysis can proceed before target proofs; final proof/ledger qualification depends on the relevant P2/P4 artifacts. P3 and P5 are not prerequisites.

**Owned outputs:** the existing language library/specification paths assigned by SP07/SP08, proposed `experiments/moriarty-conformance/coverage.json`, `expected/`, `proof/relation-spec.md`, and `deliverables/permissionless-p6-conformance/`. These proposed paths must be reconciled with the actual tree before implementation, with exact per-batch file ownership. Preserve source-pinned inventories and historical results; do not rewrite them as newly passing.

**Bounded tasks:**

- [ ] P6.1 freezes an identity-preserving coverage manifest: source/version, original row/fixture ID, complete expected observation, independent derivation, supported domain, constructor/certificate mapping and separate semantic/proof/local-ledger/Preview evidence fields. All 277 ACTUS fixtures across 18 executable types and all 72 original DeFi rows remain required; 32 taxonomy dispositions and seven source gaps remain visible rather than counted as passing fixtures.
- [ ] P6.2 implements library and necessary versioned semantic batches for ACTUS event ordering/calendars/rounding, NAM19 zero-payoff capitalization, authorized refinance, and DeFi swap/liquidity/credit/liquidation/vault/request/derivative/conditional-claim behaviors. Derive each expected trace independently. Pending redemption must retain unfilled claims; exact output and minimum output remain different predicates. Additional modern cases supplement rather than replace original rows.
- [ ] P6.3 requalifies every extended domain through Core/K/evaluator, host and target certificates, compiler correspondence and acceptance. Include omitted-field, wrong-recipient, hidden-fee, debt-erasure, duplicate-receipt, stale-oracle and permissive-tolerance controls. Reuse proof evidence only through explicit proved domain instantiations. A sample or taxonomic similarity is not a reduction proof.
- [ ] P6.4 runs the finite episode batches and publishes failures and uncovered cells. Derive required episodes before project-resource allocation; retain the historical 349-episode envelope and its accounting rather than treating it as a coverage denominator or permission to discard cases. Actual qualified Preview evidence remains separate from local acceptance and retained proofs.

**Verification:** inventory identity/count and required-field coverage checks; independent expected traces versus observed outputs; host typecheck/tests; the actual target/proof/ledger commands established in P0/P2. Each batch records exact runnable commands before execution. No nonexistent conformance runner is represented as usable today. Full native episodes need not equal fixture count, but any many-to-one coverage argument must be proved and checked.

**Stop/result:** a missing required fixture, source resolution, field, constructor, proof domain or acceptance link leaves that row and full conformance open. Finite tests do not establish a semantic theorem. Do not restrict other developers' supported programs because this project's library qualification is incomplete.

## P7 — Independent developer release and reproducible evidence

**Objective:** deliver a public developer toolchain and an evidence-backed release with all retained release obligations accounted for.

**Inputs:** P2 public proof/deployment path, P4 privacy/composition where advertised, P6 complete qualification, MC08/SP12 and legacy G01..G24. Documentation and clean-builder preparation can proceed early; release completion requires the claimed domains' evidence. P5 synthesis is optional and cannot block a correct manual-authoring release.

**Owned outputs:** developer command/API/guide paths assigned by SP12, proposed `deliverables/permissionless-p7-release/requirements-crosswalk.json`, `builders/`, `developer-episode/`, and `RELEASE-SCOPE.md`. Freeze the actual CLI/package paths from P2 before implementation. Canonical wiki edits use the repository's vault transaction mechanism. The Pel sample does not publish a release or dispatch a financial transaction.

**Bounded tasks:**

- [ ] P7.1 records a clean external developer walkthrough: author a new supported non-fixture program, format/check/simulate, inspect canonical signing terms, prove, submit in a developer-owned Midnight environment and inspect finality/full effects. Require no project account, registry entry, RP03 receipt or internal review. Compare manual and independent proposal sources under the same predicate. Include absent proof, unbound tampering, unsupported semantics, unavailable witness, replay and signed partial/failure-policy controls.
- [ ] P7.2 reproduces advertised deterministic artifacts on two independent clean builders with pinned acquisition digests, including required SRS inputs. Separate byte-for-byte reproducibility claims from randomized proof generation: specify which artifacts must be identical and independently verify any retained proof bytes. Never copy hidden local caches or private keys to manufacture reproducibility.
- [ ] P7.3 maps every MC01..MC08 and G01..G24 obligation individually to evidence or an explicit unresolved release blocker. Retain the two non-toy pilots, full ACTUS comparison with the audited Compact-library baseline, licenses/attribution, trusted-dependency inventory, confidential leakage obligations and unresolved high/critical finding checks. Missing internal audits block this maintainer release only, never public program validity.
- [ ] P7.4 assembles an immutable release candidate and evidence manifest, verifies retained proof bytes and canonical chain receipts independently, records actual fresh Preview results separately, and obtains current independently selected result reviews. Publish only under applicable release authorization; this planning package grants no network or publication action. Reconcile installed tooling/documentation with the exact candidate before claiming delivery.

**Verification:** clean-install command transcript, independent builder artifact comparisons, retained proof verification, canonical receipt/readback verification and negative controls; an individual evidence disposition for every legacy release obligation. Actual executable commands come from the implemented P2/qualifying packages and are frozen in the release brief. A documentation validator cannot discharge them.

**Stop/result:** missing pilots, licensing, reproducibility, privacy evidence, required conformance or proof/ledger correspondence prevents a full release claim. A scoped preview may state its actual limitations but cannot silently reduce the user-requested release scope. No additional implementation, publication or goal completion is asserted by this plan.

## Pel execution contract

For each package, bind this five-part brief to immutable `artifact:approved-spec`, actual `role:implementer`/`role:reviewer`, and a registered `candidate-full` verification command. Enforce owned paths before verification. The [Pel program](implementation.pel) verifies, independently reviews and permits one correction. No publish or financial dispatch occurs in this sample. Static check/plan results establish syntax and effect structure only; host policy bindings must be real before execution.

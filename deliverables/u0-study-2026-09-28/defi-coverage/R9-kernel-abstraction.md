# R9 — The DeFi kernel concept at the abstraction level, and whether Moriarty's language design can express it

Reviewer R9 of nine. Cross-cutting assignment: the kernel's core model, not any one DeFi category.
Status: **independent read-only review**. Nothing here closes a U0 predicate, accepts a proposal, or
establishes a capability. Under the repository rule work is accepted by evidence, not by review.

---

## 1. Scope and pins

### Commits

| Repo | Command run | Output |
|---|---|---|
| defiformal | `git -C /home/charl/projects/defiformal log -1 --format='%H %ad %s'` | `8c5dd103cd40369a763b02b1504441acce0ce3c2 Thu Sep 10 17:38:12 2026 -0600 Prepare independent Curve source-entry review` |
| Moriarty | `git -C /home/charl/Moriarty rev-parse HEAD` | `8f73784042bd692733c296d0d49f5173be96725e` (`U0 T7: add U0 exit gate`, Wed Sep 23 21:00:14 2026 -0600) |

Moriarty working tree at review time was **not clean**: `git status --porcelain` reported modified
`AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `docs/FOOTGUNS.md`, `plugins/moriarty-dev/README.md`,
`plugins/moriarty-dev/skills/develop/SKILL.md`,
`plugins/moriarty-dev/skills/develop/references/execution-focus.md`; deleted
`docs/COUNCIL_REVIEWS.md`; untracked `.aeon-staging/`, `deliverables/u0-study-2026-09-28/`,
`openspec/changes/aeon-refinement-integration/`. Every Moriarty line citation below is against the
**working tree**, which for the tracked files I cite (schemas, `docs/*.md`, `ROADMAP.md`,
`experiments/`, `deliverables/u0-semantic-contract-2026-09-23/`) matches HEAD, since none of those
paths appears in the dirty list. `deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md` is
**untracked** — it exists only in the working tree and is not at any commit.

A re-run of `git status --porcelain` on completion showed five further modified files
(`wiki/contradictions.md`, `wiki/hot.md`, `wiki/index.md`, `wiki/log.md`, `wiki/open-questions.md`)
and one further untracked file (`wiki/sessions/u0-unified-proposal-2026-09-28.md`) that were **not**
present at review start. They were written by other agents working in this checkout concurrently, not
by this review. This review's only write is this file. None of the newly-changed paths is cited
above, so no citation in this report is affected; but a reader reproducing these line numbers should
pin the tree rather than trust `HEAD`.

### Required startup (AGENTS.md)

The host did not expose `moriarty-dev:develop`. Per `AGENTS.md:5-11` I read and applied the
checked-in skill at `plugins/moriarty-dev/skills/develop/SKILL.md`, and read `AGENTS.md` and
`docs/FOOTGUNS.md` in full before forming conclusions.

I **ran** `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json`
(exit 0). Actual output, abridged: `capability: "SP01.6 loan-swap-subset"`;
`blockedAction: "implementation/repair of loan-swap-subset"`; `nextAction: "sp01-loan-report"`;
`missingEvidence: [binding-input-stale:openspec/sprints/sp01-…, candidate-input-stale:…,
current-accounting-missing:.moriarty-dev/runtime/current-accounting.json,
resource-live-state-unavailable:sp01-loan-swap-grok-01, operational-history]`; seven
`pendingTransactions` IDs beginning `00a91ec05fd1ae…`. This is a read-only diagnostic. **This review
posts no transaction notification**: it observed no new public submission, and the `status` command
does not query Midnight. Per SKILL.md §1 a notification line requires a *selected real public
submission observation*, which this review did not make.

### What I did and did not execute

- **Ran:** `git log`/`rev-parse`/`status`; the guarded `status --json` above; `cat`/`sed`/`awk`/`grep`
  over both repos; four `python3 -c` one-liners that parse Moriarty's own JSON artifacts
  (`stage-relation.schema.json`, `enforcement-map.json`, `source-core-embeddings.json`,
  `numeric-profile.json`, `k-reconciliation.json`) and print counts. Those counts are reported as
  *my own executed observations* and are labelled as such.
- **Did not run:** `lake build` or any Lean command in defiformal; the TypeScript evaluator; the K
  definition; any U0 checker script; any test suite; any network call. Every claim about Lean
  *proving* something, about the evaluator's runtime behaviour, or about K conformance is read from
  source text or from a recorded receipt in the repository, and is labelled accordingly.
- **Did not:** run campaign dispatch, edit repository source, commit, or submit anything.

### What I read

**defiformal** (`8c5dd10`): `lean/README.md`; `lean/DefiKernel/Core.lean` (all 185 lines);
`lean/DefiKernel/Typed/{Types,Transition}.lean` in full and `Typed/Authority.lean:1-130`;
`lean/DefiKernel/Composition/{Interfaces.lean:1-120, Execution.lean:1-150, Sequence.lean:1-110}`;
`lean/DefiKernel/Parallel/{Compatibility.lean:1-90, Execution.lean:1-80}`;
`lean/DefiKernel/Interleaving/{Schedule.lean:1-75, Execution.lean:1-80}`;
`lean/DefiKernel/Atomic/{Policy.lean, Execution.lean:1-85}`;
`lean/DefiKernel/Arithmetic/{Word.lean, Rounding.lean:1-70}`; `algebra/REQUIREMENTS.md`;
`algebra/THEOREM-LEDGER.md`; `openspec/ROADMAP.md`; `docs/research/semantic-kernel-progress.md`
(headings plus sprints 4–8 and the tail); and `docs/UNIFIED-DEFI-ELEMENT-TABLE.md` — §§1, 6, 9,
12.4, 13, 14, 15, 16.2, 17 and Appendix C read directly, with the full 1367-line read delegated to a
subagent whose findings I spot-checked against my own reads of `:530-540`, `:729-733`, `:737-760`,
`:775-808`, `:841-856`, `:882-895`, `:933-948`, `:410-430`, `:1300-1340`. I did **not** open
`algebra/MODEL.md`, `docs/unified-v0.1.md` or `wiki-llm/`.

**Moriarty** (`8f73784`): `deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md` (all 288 lines);
`docs/MORIARTY-CONSOLIDATED-DESIGN.md` (all 123); `docs/MORIARTY-PRODUCT-CONTRACT.md:1-50`;
`ROADMAP.md` (all 56); `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md` (all 63);
`openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json` (parsed in full,
plus `:452-560` read as text); all nine artifacts in
`deliverables/u0-semantic-contract-2026-09-23/` (`EXIT-GATE.md` and `judgments.json` read in full,
the other seven parsed); `experiments/moriarty-language/spec/successor/composition-proposal.md`
(headings, `:1-60`, `:72-105`, `:173-194`);
`experiments/moriarty-language/src/successor/financial-lifecycle.ts:44-139, 208-224, 1456-1463,
1565-1572`; `financial-expression-types-v1.ts:1-43, 84-112`;
`deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md:92-124`;
`wiki/moriarty-architecture.md:93-107`. The remaining breadth of
`experiments/moriarty-language/` (grammars, `financial-expression-v1.ts`, `formal/k/*.k`),
`wiki/defiformal-taxonomy.md`, `wiki/defi-kernel-sdk-interface.md`,
`wiki/defi-kernel-protocol-graph.md` and the rest of
`deliverables/defi-language-design-2026-09-07/` was read by a delegated subagent; where I rely on it
without having opened the file myself, I say so inline and mark it **[delegated]**. I did **not**
read `docs/MORIARTY-BACKEND-REQUIREMENTS.md`.

### Evidence-strength labels used throughout

Per `docs/FOOTGUNS.md` "Existing product and evidence rules" §4 and the report's own instruction, I
separate four strengths and never collapse them:

- **designed** — named in prose in a design document, with no grammar, schema or rule.
- **specified** — a rule, judgment or schema field exists that fixes the meaning.
- **implemented** — code in the evaluator or K definition computes it.
- **demonstrated** — an executed run with recorded output in the repository.

A name in a schema is not coverage. A checker exiting 0 is not a capability
(`UNIFIED-PROPOSAL.md:264`: "Do not treat a checker exit of 0, a cited identifier, or a matching
fixture as coverage or enforcement.").

---

## 2. The kernel abstraction, stated precisely

`lean/DefiKernel/Core.lean` is **not** the kernel at the level this review is about. It is the
first-increment pilot: four hard-coded accounts and four hard-coded assets
(`Core.lean:15-21`), one `Policy` of two Boolean predicates (`Core.lean:44-47`), and a `Transition`
whose guard and effect are arbitrary Lean functions (`Core.lean:49-54`). `lean/README.md:33-39`
disclaims it directly: "The supplied policy is a trust assumption. It does not authenticate callers
or implement capability issuance/revocation."

The kernel proper is `DefiKernel.Typed` + `DefiKernel.Composition` + the four mode packages. Each
element below is stated with its defining file:line.

### 2.1 State

- **Cell index is a triple.** `abbrev Cell (Party Asset Domain : Type) := Domain × Party × Asset`
  — `lean/DefiKernel/Typed/Types.lean:25`. **Domain is the outermost index.**
- **State is a total nonnegative balance function.** `structure State … balance : Cell … → ℚ` with
  `nonneg : ∀ c, 0 ≤ balance c` — `Types.lean:32-34`. Nonnegativity is a *field of the state value*,
  so no state can exist that violates it.
- **Exact rationals.** Every quantity is `ℚ` — `Types.lean:30, 33`. `Core.lean:7` states the intent:
  "A finite reference ledger with exact rational arithmetic."
- **Per-(domain, asset) total.** `total s domain asset = ∑ party, s.balance (domain, party, asset)`
  — `Types.lean:36-38`.
- **Dimensioned units, asset-indexed.**
  `inductive Unit (Asset) := amount (asset) | price (base quote) | scalar | bool` —
  `Types.lean:41-46`, with a `NumericUnit` subset excluding `bool` (`Types.lean:49-53`). A price is
  **quote-asset units per one base-asset unit** (`Types.lean:40`). `Value : Unit → Type` maps
  `bool ↦ Bool` and the three numeric units ↦ `ℚ` (`Types.lean:60-62`), so an `amount usd` and an
  `amount share` are distinct *types* even though both carry a rational.
- **Nonnegative quantity type.** `structure Quantity (asset) where amount : ℚ; nonneg : 0 ≤ amount`
  — `Types.lean:28-30`.
- **Debt is a token, not a negative balance.** `Core.lean:38`: "Debt is a separate nonnegative
  obligation token, not a negative cash balance."

### 2.2 Transition — a registered template

`structure Template (Party Asset Domain)` — `lean/DefiKernel/Typed/Transition.lean:19-29` — carries
exactly nine fields:

| Field | Line | Meaning |
|---|---|---|
| `signature : List (Unit Asset)` | `:20` | typed argument list |
| `domain : Domain` | `:21` | the one domain this operation may touch |
| `partyArity : Nat` | `:22` | how many party arguments |
| `guard : Expr … .bool` | `:23` | execution condition, a closed expression |
| `deltas : List (CellDelta …)` | `:24` | balance changes, each `{asset, target : CellRef, amount : Expr … (.amount asset)}` (`:9-12`) |
| `supplyDeltas : List (SupplyDelta …)` | `:25` | minting/burning, each `{domain, asset, amount}` (`:14-17`) |
| `stateReads : List (PackedCellRef …)` | `:26` | **declared read footprint** |
| `envReads : List (EnvRead Domain)` | `:27` | **declared external data** |
| `writes : List (PackedCellRef …)` | `:28` | **declared write footprint** |

`abbrev Registry := OperationId → Option Template` — `Transition.lean:30`. Templates are looked up
by ID; the registry is trusted and also supplies the operation→domain map used at capability issue
time (`Transition.lean:33-35`).

The *required* footprint is computed from the expressions, not declared by hand:
`Template.requiredStateReads` unions the guard's, deltas' and supplyDeltas' `stateReads`
(`Transition.lean:80-82`); `requiredEnvReads` likewise (`:84-86`). The check is that required ⊆
declared (`Evaluated.stateReadsOK` `:121-122`, `envReadsOK` `:124-125`).

### 2.3 Authority — capabilities

- **Rights are three-way.** `inductive Right := invoke | debit (cell) | changeSupply (domain, asset)`
  — `lean/DefiKernel/Typed/Authority.lean:8-12`. A debit right names **one exact cell**; a supply
  right names **one exact (domain, asset)**.
- **A capability is a scoped, revocable grant.**
  `structure Grant := holder : Party; domain : Domain; operation : OperationId; right : Right`
  (`Authority.lean:21-26`); `structure Capability extends Grant where live : Bool` (`:28-30`).
- **Store is an append-only list; the index is the permanent ID.**
  `CapabilityStore.nextId = ⟨entries.length⟩` (`Authority.lean:44-45`), `lookup = entries[id]?`
  (`:47-48`). `Authority.lean:4`: "List positions are permanent IDs: revoked entries remain as
  tombstones."
- **Issue.** `issueCapability` requires the caller to be the domain admin *for the grant's domain*,
  the operation's registry domain to equal the grant domain, and the right's resource to lie in that
  domain — else `unauthorizedAdmin` / `operationDomain` / `resourceDomain`
  (`Authority.lean:65-74`). "A grant cannot choose its ID or reactivate an existing entry" (`:64`).
- **Revoke.** Idempotent, ID-preserving, admin-gated: `entries.set id.value {cap with live := false}`
  (`Authority.lean:77-85`).
- **Use.** `authorizesId` demands `live ∧ holder = ctx.principal ∧ domain = ctx.domain ∧
  operation = requested ∧ right = requested ∧ right.inDomain ctx.domain`
  (`Authority.lean:88-95`). `hasAuthority ids … = ids.any (authorizesId …)` (`:98-101`) —
  "Existential use means duplicate request IDs confer no additional rights" (`:97`).
- **Caller identity is context, not request.**
  `structure InvocationContext := principal : Party; domain : Domain` — `Types.lean:131-134`, with
  the caveat at `:130`: "Adapter-supplied identity. Constructing this value is not signature
  verification."
- **Request.** `structure Request := operation : OperationId; parties : List Party;
  arguments : List PackedValue; capabilityIds : List CapabilityId; claimedActor : Option Party`
  — `Transition.lean:37-42`. The `claimedActor` is checked *against* `ctx.principal`, never trusted
  (`Transition.lean:178-179`).
- **What the model does not cover.** `Authority.lean:5`: "Authentication, registry truth, allowances
  and replay prevention are outside this model." `Transition.lean:5-6` repeats: checks concern
  aggregate net effects and "do not model intermediate debit order, consumable allowances, replay
  prevention, or observation truth."

### 2.4 Execution and refusal

`execute` (`Transition.lean:171-187`) is a fixed pipeline, and the order is documented at `:168-170`:

1. registry lookup → `unknownOperation`
2. `claimedActor` vs `ctx.principal` → `actorMismatch`
3. `ctx.domain` vs `template.domain` → `domainMismatch`
4. `parties.length` vs `partyArity` → `partyArity`
5. `Args.check signature arguments` → `evaluation`
6. `hasAuthority … .invoke` → `unauthorizedInvoke`
7. `template.evaluate` → `evaluation`
8. `applyEvaluated` (`:152-166`), in order: `guard`, `stateReadFootprint`, `envReadFootprint`,
   `crossDomain`, `unauthorizedDebit`, `unauthorizedSupply`, **nonnegativity**, `accounting`,
   `writeFootprint`.

Fifteen refusal reasons, enumerated at `Transition.lean:44-60`. The four structural checks:

- **Per-asset conservation.** `Evaluated.accountingOK = decide (∀ d a, ∑ p, e.effect (d, p, a) =
  e.supply d a)` — `Transition.lean:145-146`. *Total balance change on an asset within a domain
  equals the declared supply change.* The theorem
  `applyEvaluated_accounting : total post d a = total state d a + e.supply d a`
  (`Transition.lean:248-257`) is the consequence.
- **Nonnegativity.** `if hn : ∀ c, 0 ≤ state.balance c + e.effect c` — `Transition.lean:162`. The
  proof `hn` *is* the new state's `nonneg` field (`:165`), so the post-state cannot be built without
  it. Refusal `insufficientFunds` (`:166`).
- **Write locality.** `writesOK = ∀ c, c ∉ e.writes → e.effect c = 0` — `Transition.lean:148-149`.
- **Domain locality.** `domainOK` requires every required read, every nonzero effect and every
  nonzero supply change to sit in the invocation domain — `Transition.lean:129-131`.

**Refusal carries no post-state.** `execute : … → Except Refusal (ExecutionResult …)`
(`Transition.lean:171-174`); the error branch returns only a reason. `Core.lean:117` states the
principle for the pilot: "Refusal has no post-state; successful execution constructs a nonnegative
state."

Success is characterised biconditionally by `execute_ok_iff` (`Transition.lean:219-246`), and the
derived guarantees are separate named theorems: `execute_invocation_authority` (`:326-334`),
`execute_debit_authority` (`:336-346`), `execute_supply_authority` (`:348-359`),
`execute_accounting_and_locality` (`:361-372`), `execute_reads_and_domain` (`:376-398`),
`execute_nonnegative` (`:318-324`), `execute_preserves_capabilities` (`:312-316`).

### 2.5 Structure — components, ports, workflows, traces

`lean/DefiKernel/Composition/Interfaces.lean`:

- `structure Component := id; privateCells; exports : List ResourcePort; imports :
  List ResourceImport; operations : List OperationInterface` — `:49-55`.
- `ResourcePort := {id : PortId, cell : Cell, writable : Bool}` — `:31-35`. An import re-exports a
  source port's cell with possibly reduced write rights — `:37-41`.
- `InputPort := {id, unit : Typed.Unit Asset}` — `:21-24`; `OutputPort := {id, cell : Cell}` —
  `:26-29`. An `OperationInterface` binds an `OperationId` to its input and output ports — `:43-47`.
- Access is decided structurally: `canRead` = private ∪ exports ∪ imports (`:81-85`);
  `canWrite` adds the `writable` condition (`:87-91`).
- `validateCatalog` enforces no duplicate component IDs, no operation offered twice, disjoint private
  cells, disjoint exported cells, per-component port-ID uniqueness, and that no export or import
  aliases anyone's private cell — `:106-120`. The docstring `:104-105`: "Private ownership excludes
  all shared ports, including the owner's own exports. An import may reduce write access, but cannot
  grant more rights than its exact export."

**Workflow.** `inductive InputSource := literal (value) | priorOutput (step : Nat) (port :
QualifiedPort)` — `Interfaces.lean:69-71` — and `OutputObservation := {step, port, value}` (`:73-76`).
A later step's argument may be a **recorded output of an earlier step at a named port**. That is the
workflow primitive. `Composition/Execution.lean:65-76` (`prepareInvocation`) resolves those inputs
against the history and checks component access before building the kernel `Request`.

**Step and trace.** `inductive Step := invoke | issue (grant) | revoke (id)` —
`Composition/Execution.lean:27-30`; `Receipt := invoked (request) (evaluated) | issued | revoked`
(`:40-43`); `StepResult := {world, receipt, outputs}` (`:45-48`). `Composition/Sequence.lean:8-24`
adds `Event {index, step, before, result}`, `LocatedFailure {index, step, reason}` and a `Cursor`
holding world, events, outputs, `nextIndex` and an optional failure. `Sequence.lean:3-4`: "A refusal
commits no new event, preserves the successful prefix, and makes every continuation inert."

### 2.6 Composition — the four modes

| Mode | Defining file:line | Admission | Failure behaviour |
|---|---|---|---|
| **Sequential** | `Composition/Sequence.lean:33-51` | `validateCatalog` at `startCursor` (`:29-31`) | `advance` sets `failure` and every later step is a no-op (`:35-36`, and `continueRun_failed` `:85-92`). The **successful prefix is retained** (`continueRun_order` `:95-105`). |
| **Disjoint parallel** | `Parallel/Execution.lean:39-46` | `admit` = catalog + per-branch structural analysis + `checkCompatibility` (`Parallel/Compatibility.lean:75-82`) | Admission failure ⇒ `refused reason initial` — **no branch runs** (`Execution.lean:41-42`). Otherwise both branches run from the *same* initial world with isolated histories and are **joined** by `mergeWorld`, which takes each cell from whichever branch's declared write set contains it, else from the initial world (`:22-33`). |
| **Shared-state interleaving** | `Interleaving/Execution.lean` over `Interleaving/Schedule.lean` | `admit` = catalog + structural analysis + `checkSchedule` (`Schedule.lean:33-40`). **Compatibility is deliberately not required**: `Schedule.lean:3-4` "Overlapping footprints are admitted here; compatibility remains a separate premise for disjoint recovery." | One evolving world; each branch has its own `LocalState {consumed, events, outputs, nextIndex, failure}` (`Execution.lean:10-15`); a branch's refusal is permanent for that branch while the peer continues (`:5-6`). |
| **Atomic** | `Atomic/Execution.lean` over `Atomic/Policy.lean` | `admit` = catalog + structural + `checkPolicy` + `checkSchedule` (`Atomic/Execution.lean:55-63`) | Three outcomes: `refused` (nothing ran), `aborted` (ran speculatively, published nothing), `committed` (`:31-36`). `Result.publicWorld` returns `entryWorld` for both `refused` and `aborted`, and the speculative world only for `committed` (`:78-81`). `committedHistory` publishes `[]` unless committed (`:84-85`). `Atomic/Execution.lean:4-5`: "Diagnostic receipts describe speculation; only a committed result publishes them." |

**Compatibility is a three-way disjointness test.** `checkCompatibility` refuses on write/write,
left-write/right-read, and right-write/left-read overlap, naming the exact conflicting cell —
`Parallel/Compatibility.lean:66-74`, with `ConflictKind` at `:21-25` and the logical form `Compatible`
at `:86-89`. The analysed footprint is computed without evaluating any financial value
(`Compatibility.lean:35-52`), and covers **every branch suffix even when a prefix would refuse**
(`:53`).

**Atomic settlement policy.** `structure Lane := {domain, asset, vault : Party}` and
`structure Policy := {lanes : List Lane; participants : List Party}` — `Atomic/Policy.lean:8-20`.
`checkPolicy` rejects a duplicate `(domain, asset)` lane, a duplicate participant, and any static
invocation whose boundary principal is not a listed participant — `Atomic/Policy.lean:56-70`, with
coverage visiting "every static invocation, including suffixes that might never execute" (`:46`).
Obligations are receipt-derived: `updateOutstanding` decrements a participant's lane balance by that
receipt's effect on the lane cell (`:79-82`); unsettled residuals are enumerated lane-major
(`:85-87`) and abort the atomic block via `AbortReason.unsettled` (`Atomic/Execution.lean:21`).
`checkSupply` (`Policy.lean:89-91`) forbids a supply change on a settlement lane, aborting with
`laneSupply` (`Atomic/Execution.lean:19-20`).

### 2.7 Arithmetic — the kernel's *own* integer layer

The kernel does not only have ℚ. `lean/DefiKernel/Arithmetic/` adds a checked bounded-integer layer:
`structure Word (w : Nat) := {value : Nat, bound : value < 2^w}` (`Arithmetic/Word.lean:6-9`),
a ten-case `Failure` including `addOverflow`, `subUnderflow`, `mulOverflow`, `quotientOverflow`,
`nonIntegralQuantity` (`:11-15`), `inductive Rounding := down | up` (`:17-18`), and
`Rounding.mulDiv` performing a **full-width product then a directed division then a checked
narrowing** (`Arithmetic/Rounding.lean:18-22`). Floor and ceiling are characterised by independent
specifications, not by their implementations — `floor_characterization` (`Rounding.lean:25-33`) and
`ceil_characterization` (`:35-60`). `docs/research/semantic-kernel-progress.md:368` records a
"Checked integer arithmetic accepted" sprint.

**This matters for question 5:** the exact-rational/bounded-integer split is a split the kernel makes
*internally*, not a gap between the two projects.

### 2.8 The Atlas vocabulary, and its residue

`docs/UNIFIED-DEFI-ELEMENT-TABLE.md` is a different artifact from the Lean kernel: a classification
vocabulary for whole protocols, not an execution semantics. Its self-description at `:12-14`:
"48 core elements · 10 candidates · 10 provisional · 16 groups · 5 dependency strata · 4 typed bond
classes · 29 required-bond laws · 19 hazard rules · an atomicity phase diagram · four overlays."

- **Element** = "An independently recurring, role-substitutable financial state-transition mechanism
  with a distinct failure signature" (`:74`), admitted by a seven-criterion test (`:99-107`).
- **Four bond classes** (`:535-540`): Interface `—i→`, Economic `—e→`, Trust `—t→`, Informational
  `—n→`. "Each must be satisfied **separately**. A protocol is not valid because its contracts can
  call each other" (`:532-533`). Authority is **not** a fifth bond: "authority is a bond property
  carried by `—t→`" (`:567`).
- **29 required-bond laws** L1–L29 (`:741-769`), each with an async-safe flag; L9 is the only
  conservation identity written as an equation: `Xf → debit(source) = credit(destination)` (`:749`).
  L17 is the capability law: `Au → bounded scope + revocation + expiry + nonce/domain separation`
  (`:757`).
- **19 hazard rules** X1–X19 (`:789-808`), classed F (contradiction) / H (survivors exist) / U
  (unverifiable) (`:784-785`).
- **Nine reaction conditions** (`:937-945`) — environmental bounds a molecule's validity is
  conditional on.
- **Five dependency strata** S0–S4 by *category of prerequisite*, explicitly **not** graph depth
  (`:416-420`, correction at `:424-430`).
- **Atomicity phase diagram**, six levels from "Same transaction" to "Optimistic", each with a named
  **repair** (`:843-850`).
- **Residue** is a mandatory per-molecule field in the grammar (`:632-636`), a formula is ill-formed
  without it (`:689-692`), and §12.4 defines it: "Per molecule, and always present. Residue is a
  *finding*: it is what the vocabulary could not express about this protocol" (`:731-733`).
  The quickstart states the discipline: "Residue is reported, never hidden" (`:30`).

Two Atlas-level admissions matter for §4.6 below: "**Nothing in the element vocabulary can see**"
implementation integrity (`:905`, and the framing at `:884-887`), and the hazard table "has **no
denominator**" with the double standard "named here rather than hidden" (`:777-782`).

Separately, `algebra/THEOREM-LEDGER.md` records that composition is **not** validity-preserving in
this vocabulary: P10, "`⊕` is not a congruence for validity — refuted on all four closures with
witnesses" (`THEOREM-LEDGER.md:37`), sharpened at K4 (`:80`): "observational equivalence is preserved
by composition and *admissibility is not*." The best positive result is partial: a union-closed
fragment covering 47.8% of admissible sets certifies 59% of live protocol pairs
(`THEOREM-LEDGER.md:63`; requirement R5 at `algebra/REQUIREMENTS.md:77-83`).

### 2.9 What the kernel's own records claim

`docs/research/semantic-kernel-progress.md` records one accepted sprint per layer: Sprint 4 typed
transition IR and capability authority (`:108`), Sprint 5 typed interfaces and sequential composition
(`:149`), Sprint 6 disjoint parallel (`:196`), Sprint 7 shared-state interleaving (`:236`), Sprint 8
atomic synchronization (`:299`). Each records a full Lean build, runtime comparison counts, mutation
detection and an axiom audit restricted to `propext`, `Classical.choice`, `Quot.sound` — e.g. Sprint
4 "1005-job full build; 189/189 typed comparisons" and "24/24 real source mutants detected"
(`:118-121`). **I did not re-run any of this**; these are the repository's recorded receipts.

The kernel's own limits are stated at `lean/README.md:59-63`: "The pilot accepts Lean functions for
guards and effects. It is not yet a closed, serialized IR or a checker for untrusted external proof
packages. It does not prove general operational composition, intermediate-effect authority,
machine-width arithmetic refinement, deployed-contract correspondence, economic solvency, or
asynchronous liveness."

---

## 3. Moriarty against each abstraction

Reading key. `file:line` for Moriarty is against the working tree at `8f73784` (see §1).
**[delegated]** marks a row whose Moriarty evidence I did not open myself.

| # | Kernel abstraction (file:line) | Moriarty counterpart (file:line) | Verdict | What is lost or gained |
|---|---|---|---|---|
| 1 | Cell indexed by `Domain × Party × Asset` (`Typed/Types.lean:25`) | `Balance {party, asset, amount}` (`experiments/moriarty-language/src/successor/financial-lifecycle.ts:44-48`); `domain.domainId` is a top-level scalar string in the stage relation (`schemas/stage-relation.schema.json`, dumped leaf `domain.domainId :: string`) | **partial** | The **domain index is lost**. Moriarty has one domain per stage, so `crossDomain` (`Typed/Transition.lean:129-131`) is vacuous rather than checked, and per-(domain,asset) totals are not expressible. Gained: nothing. |
| 2 | Nonnegative balance as a *field of the state value* (`Types.lean:33-34`) | `UInt128Text` balances; `subU128` returns `null` and the run rejects with `INSUFFICIENT_BALANCE` (`financial-lifecycle.ts:1459-1461`) | **expressed** | Same guarantee, different mechanism: unsigned width plus a checked subtract, vs a carried proof. No loss at the value level. Lost: the kernel's *static* impossibility of an ill-formed state — a Moriarty `LifecycleState` parsed from JSON could in principle carry any string until admitted. |
| 3 | Asset-indexed dimensioned units: `amount(asset) \| price(base,quote) \| scalar \| bool` (`Types.lean:41-46`) | `typeShape`/`resolve`: `Amount<asset>`, `Shares<vault,party>`, `Price<base,quote,scale>`, `Rate<scale>`, `Quantity<units,scale>`, `ScaledAmount<asset,scale>`, `AmountProduct<a,b>`, `SignedAmount`, `NetAmount` (`financial-expression-types-v1.ts:27-43`, resolved against declared asset/vault/party sets at `:84-106`) | **expressed differently — gain** | Moriarty's surface is **richer**: it distinguishes shares by *vault and holder* (`:87`), forbids a degenerate `Price<A,A>` (`:92`), and bounds every scale to 0..18 (`:95, :101`). A share amount and a dollar amount are not interchangeable, which is the kernel's requirement. Gained over the kernel: explicit scale, signed/net variants, unit-vector `Quantity`. Lost: nothing at this layer. **But see row 4** — the dimensions live in the *expression type checker*, not in the stage relation. |
| 4 | Exact rational quantities `ℚ` (`Types.lean:30, 33`; `Core.lean:7`) | `UInt128Text`/`UInt64Text` decimal strings over BigInt; `Conversion {mantissa, scale, rounding}`; `AccrualTerms {numerator, denominator, rounding, …}` (`financial-lifecycle.ts:23-42` **[delegated]**, ranges I read at `:69-83`); `units.representation: "exact-domain-qualified-integer"`, `fieldElementCoercion: "forbidden"` (`deliverables/u0-semantic-contract-2026-09-23/numeric-profile.json`) | **expressed differently — real loss, bounded** | Ratios exist only as *parameters with a rounding mode*, never as values. There is no rational sort. The kernel's own `Arithmetic/` layer makes the same move (`Word.lean:6-9`, `Rounding.lean:18-22`), so the two projects agree on the target; the loss is the **remainder**, which the kernel's `mulDiv` characterises with floor/ceil specifications (`Rounding.lean:25-60`) and Moriarty currently cannot post at all (row 22). |
| 5 | Registered `Template` with signature, guard, deltas, supplyDeltas, declared reads/writes (`Typed/Transition.lean:19-29`); `Registry : OperationId → Option Template` (`:30`) | Four fixed action constructors `Transfer \| Repay \| Originate \| Accrue` (`financial-lifecycle.ts:85-127`); no registry. `programIdentity.{programId, entryPoint}` names a *program*, not a registered template (schema leaves) | **partial** | Execution conditions and balance changes exist (as hand-written TypeScript per action). Lost: **templates are not first-class data**. A new operation is new evaluator code, not a registry entry; there is no operation signature, no party arity check, no declared domain. The `emit` descriptor registry is the nearest thing and is explicitly *not* callable: "The operation registry supplies an operand schema, **not a callable semantic function**" (`spec/successor/static-semantics.md:234-241` **[delegated]**). |
| 6 | Declared **read** footprint, and required ⊆ declared check (`Transition.lean:26, 80-82, 121-122`; refusal `stateReadFootprint` `:52`) | None. Executed grep: `grep -ci "read\|reads\|footprint"` over `schemas/stage-relation.schema.json` returns **0, 0, 0** | **absent** | The kernel's read footprint is what makes disjoint-parallel admission decidable *without evaluating values* (`Parallel/Compatibility.lean:35-52`). Moriarty has no field in which to record it, so no compatibility check is even statable. This is the single largest structural gap. |
| 7 | Declared **write** footprint and locality (`Transition.lean:28, 148-149`; theorem `applyEvaluated_locality` `:259-269`) | None; same grep, 0 hits | **absent** | No frame property is statable. `docs/MORIARTY-CONSOLIDATED-DESIGN.md:48` requires "Bind actual complete effects rather than a convenient projection selected by a prover" — that is the *converse* property (completeness), not locality. |
| 8 | `supplyDeltas` and `Right.changeSupply` — minting/burning as a separate authorised quantity (`Transition.lean:14-17, 25`; `Authority.lean:11`) | `effects.supplyChanges[]{asset, account, amount}` exists as a schema leaf; its Core/source embedding is **absent**: "No supply-change account exists in source/5 or core/1" ×3 (`deliverables/u0-semantic-contract-2026-09-23/source-core-embeddings.json`, rows `effects.supplyChanges[].{account,amount,asset}`, `realisation: "absent"`) | **partial — schema name only** | The name is present; nothing computes it. Subagent grep found `mint` appearing once in the language layer and as a *disclaimer* (`spec/successor/financial-expression-source.md:174` **[delegated]**), and `burn` zero times. `deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md:94` states the design intent and stops: "Mint/burn requires a defining authority and accounting rule." |
| 9 | `Request {operation, parties, arguments, capabilityIds, claimedActor}` (`Transition.lean:37-42`) | `LifecycleInput` carrying a list of actions (`financial-lifecycle.ts:141` **[delegated]**); `signedIntent.*` fifteen leaves in the stage relation | **expressed differently** | Moriarty's signed intent carries *outcome constraints* the kernel has no analogue for — `grossDebitCap`, `feeCap`, `minNetOutcome`, `validity`, `recipients[]`, `assetIdentities[]` (schema leaves). That is a **gain** over the kernel, which authorises only per-cell debits. Lost: the request does not name a template or carry capability IDs. |
| 10 | `Capability {holder, domain, operation, right, live}` with `issueCapability`/`revokeCapability` (`Authority.lean:21-30, 65-85`) | `Allowance {party, asset, remaining, spent}` (`financial-lifecycle.ts:50-55`); `authority.{consumed, remaining, replayState}` as three plain `"type": "string"` fields (`stage-relation.schema.json:452-470`, read directly) | **expressed differently — real loss** | An allowance is a *spend budget on (party, asset)*. It is not a capability: it has no holder-vs-owner distinction, no operation scope, no `invoke` right, no issuance identity, no tombstone, no revocation. Gained: an allowance is **consumable and replay-tracked**, which `Authority.lean:5` says the kernel deliberately excludes. The two models are complementary, not nested. |
| 11 | Three rights `invoke \| debit(cell) \| changeSupply(domain,asset)` (`Authority.lean:8-12`) | No `invoke` analogue; no supply right (row 8); debit approximated by the allowance | **partial** | The kernel's `unauthorizedInvoke` (`Transition.lean:183-184`) has no Moriarty counterpart — anyone who can submit a `LifecycleInput` may run any of the four actions. The *authorisation* question in Moriarty is answered entirely by the signed intent and the allowance. |
| 12 | Caller identity from `InvocationContext.principal`, with `claimedActor` cross-check (`Types.lean:131-134`; `Transition.lean:178-179`) | `signedIntent.signer` (schema leaf, `"type": "string"`). No caller context in the evaluator: `LifecycleState` (`financial-lifecycle.ts:129-139`) has no principal | **partial** | The schema names a signer; the evaluator has no notion of who is calling. `UNIFIED-PROPOSAL.md:196-201` (T6) records that in-circuit intent authentication is a *proposed owner decision*, not a decided one, and `enforcement-map.json` `limitation` states "Signed-intent authentication was not found in a circuit, a bound ledger primitive, or another native boundary under the declared native roots." |
| 13 | Refusal with a reason and **no post-state** (`Transition.lean:44-60, 171-174`; `Core.lean:117`) | `RejectedLifecycle {status: 'Rejected', code, actionIndex \| null}` (`financial-lifecycle.ts:217-221`), constructed by `rejected()` (`:336-337`); 30+ codes observed via grep | **expressed** | Exact match, including the located index (kernel `LocatedFailure {index, step, reason}`, `Sequence.lean:14-17`). Moriarty's code taxonomy is finer at the arithmetic level (`OVERFLOW`, `DUST`, `INEXACT_CONVERSION`, `LIABILITY_CAP_EXCEEDED`) and coarser at the authority level. **Gain**: K has a four-layer rejection taxonomy — expression / adapter / kernel / suffix (`formal/k/lifecycle-v1.k:5-10` **[delegated]**). |
| 14 | Per-asset conservation `∀ d a, ∑_p effect(d,p,a) = supply d a` (`Transition.lean:145-146`), with theorem `:248-257` | Proposed law E1 "per-asset conservation: Σ gross = Σ supply changes = 0" and E2 "net is derived from gross and fees" (`deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md:90-91`). Not in `judgments.json` — the `effect` judgment there is one prose sentence (`judgments.json:52`) | **partial — proposed** | F5 of the proposal states the position exactly: "The six judgments are prose. None states conservation, a cap inequality, non-negativity, liability roll-forward or replay freshness" (`UNIFIED-PROPOSAL.md:36`). See §4.3. |
| 15 | Nonnegative-result check gating state construction (`Transition.lean:162-166`) | `subU128 → null → INSUFFICIENT_BALANCE` (`financial-lifecycle.ts:1459-1461`) | **expressed** | Same predicate, checked at the arithmetic operation rather than over the whole cell space. |
| 16 | Domain locality: reads, effects and supply changes confined to `ctx.domain` (`Transition.lean:129-131`; refusal `crossDomain` `:54`) | None (row 1). `domain.{chainId, domainId, stateFrameRef}` are stage-level schema strings | **absent** | `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:8` concedes it: "The current same-name source-unit/settlement-asset restriction is narrower than the target's domain-qualified financial model." |
| 17 | `Component {id, privateCells, exports, imports, operations}` (`Composition/Interfaces.lean:49-55`) | None. `writeClass ∈ {'ordinary','financial'}` is a two-valued write-permission tag on flat fields (`financial-expression-types-v1.ts:60-61` **[delegated]**) | **absent** | Modules are *designed*: "The surface language contains modules, named definitions, schedules, … and templates. These features must elaborate away" (`wiki/moriarty-architecture.md:102-104`, read directly). No grammar production exists — `spec/successor/grammar.ebnf:104-106` lists `composition` among forms that are "not productions of this profile" **[delegated]**. |
| 18 | Named typed ports, input and output (`Interfaces.lean:21-35, 43-47`) | None. My own executed check: `grep -rEoh "\b[Pp]orts?\b"` over `experiments/moriarty-language/{src,spec}/successor` returns **zero matches** | **absent** | Nothing to lose yet — but ports are what make a `priorOutput` reference *typed* (row 20) and what `checkAccess` uses to decide component read/write rights. |
| 19 | `validateCatalog` — structural well-formedness of the component graph (`Interfaces.lean:106-120`) | None | **absent** | Consequence of 17–18. |
| 20 | Workflow: `InputSource.priorOutput (step, port)` (`Interfaces.lean:69-71`) resolved against recorded `OutputObservation`s (`:73-76`, `Composition/Execution.lean:71`) | None in the language. Chaining is done in the test harness: `examples/loan-lifecycle.mjs:21-30` manually feeds `predecessor.post` into the next `evaluate()` call **[delegated]** | **absent** | The workflow concept is defined in prose — "A workflow connects actions across one or more transactions" (`deliverables/defi-language-design-2026-09-07/README.md:25` **[delegated]**) — and `outcome.continuations[]` is a schema leaf whose embedding is **absent**: "No stage-continuation identifier exists in source/5 or core/1" (`source-core-embeddings.json`). |
| 21 | Trace: `Event {index, step, before, result}` + `Cursor` + `TraceSound` (`Sequence.lean:8-24, 56-66`) | `PreparedLifecycle {status, schemaVersion, post, effects}` (`financial-lifecycle.ts:210-215`) — an effect list, plus `actionIndex` on refusal | **partial** | The effect list is an ordered record of successful steps, so the *shape* is there. Lost: the per-step **before-world**, so no step can be independently re-checked against its own pre-state (the kernel's `extractReceipt` re-evaluates against the same pre-state, `Composition/Execution.lean:78-85`), and `issue`/`revoke` are not steps at all. |
| 22 | Sequential mode retaining the committed prefix (`Sequence.lean:3-4, 40-43`; `continueRun_order` `:95-105`) | First failure aborts the whole batch and publishes nothing (`runActions`, `financial-lifecycle.ts:1902-1904` **[delegated]**); `failurePolicy.{phasePolicy, retainedEffects, retainedFees}` are schema strings whose embeddings are **absent**, noted "A failed suffix publishes no state" (`source-core-embeddings.json`) | **expressed differently — real loss** | Moriarty's rejection is **atomic**, the kernel's is **prefix-retaining**. Both are coherent; they are different operators. The gap is acknowledged twice: `docs/MORIARTY-PRODUCT-CONTRACT.md:47` "Local evaluator rejection can be atomic. Midnight ledger phase semantics must be modeled separately: a failed fallible phase can retain guaranteed-phase effects and fees"; and `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:37` "Local all-or-nothing rejection cannot implement those distinctions by itself." |
| 23 | Disjoint parallel with access-compatibility admission and a join (`Parallel/Compatibility.lean:66-82`; `Parallel/Execution.lean:22-46`) | Named in prose only: "Disjoint parallel composition requires checked independence and complete joining" (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:72`) and "disjoint parallel composition" in the five-operator list (`deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md:122`). Executed grep: `parallel` returns **0 hits** in `spec/successor/`, `src/successor/`, `formal/k/*.k` **[delegated]** | **absent** | ROADMAP.md:24 assigns "joins" to **U3**; `ROADMAP.md:25` assigns "every retained composition operator demonstrated" to **U4**. |
| 24 | Shared-state interleaving under a supplied schedule (`Interleaving/Schedule.lean:8-40`; `Interleaving/Execution.lean:10-15`) | Named in prose only: `MORIARTY-CONSOLIDATED-DESIGN.md:72` "Shared-state interleaving requires interference reasoning"; `LANGUAGE-DESIGN.md:122`. `interleav`/`concurren` → **0 hits** **[delegated]** | **absent** | Same owners as row 23. |
| 25 | Atomic mode: one published commit, speculative receipts withheld (`Atomic/Execution.lean:31-36, 78-85`) | Named in prose only: `MORIARTY-CONSOLIDATED-DESIGN.md:72` "Atomic publication applies only within a declared domain and its actual phase semantics"; `LANGUAGE-DESIGN.md:122` "atomic synchronization". The token `atomic` in the language layer refers to the historical profile name `moriarty-bounded-atomic/1` **[delegated]** | **absent** | Moriarty's single-batch all-or-nothing (row 22) is *one instance* of atomic execution, but it is not a composition operator over two branches. |
| 26 | Atomic settlement policy naming vault accounts and participants; receipt-derived outstanding; residual abort (`Atomic/Policy.lean:8-20, 56-70, 79-87`) | None | **absent** | This is the kernel's most financially specific structure — a clearing lane `(domain, asset, vault)` and a participant roster, with obligations derived from receipts and an abort on any nonzero residual. Moriarty has no analogue at any layer. |
| 27 | Capability store carried through execution and preserved by `invoke` (`Transition.lean:73-76`; `execute_preserves_capabilities` `:312-316`); mutated only by `issue`/`revoke` steps (`Composition/Execution.lean:98-105`) | None. Executed grep: `capabilit` → **0 hits** in `spec/successor/` and `src/successor/` **[delegated]** | **absent** | `wiki/moriarty-architecture.md:99-100` designs it — "every oracle or external effect has a named capability and assurance boundary" — but no grammar, type or evaluator field exists. `wiki/defiformal-taxonomy.md:189` uses "capabilities" for things that *stay outside Core* **[delegated]**, i.e. the opposite sense. |
| 28 | Environment observations `{value : PackedValue, timestamp}` keyed by `(domain, ObservationId)` with declared env-read footprint (`Types.lean:114-128`; `Transition.lean:84-86, 124-125`) | `observations[]{kind, issuer, domain, time, finality}` — five schema leaves, all embeddings **absent** (`source-core-embeddings.json`) | **partial** | **Gain**: Moriarty's schema carries `issuer` and `finality`, which the kernel does not model (`Types.lean:124-126`). **Loss**: no declared env-read footprint, so `envReadFootprint` (`Transition.lean:53`) has no counterpart, and nothing computes any of the five fields. |
| 29 | Lean proof terms as the evidence format (`lean/README.md:65`) | `docs/MORIARTY-CONSOLIDATED-DESIGN.md:17`: "Do not introduce a second mandatory semantics in Lean. DeFiFormal's Lean development supplies valuable reference contracts and examples; neither its tooling nor its theorems automatically become Moriarty dependencies or native certificates." Also `docs/MORIARTY-PRODUCT-CONTRACT.md:23`: "Lean is not a Moriarty compiler, developer-tool, proof-production or deployment dependency." | **out-of-scope-by-design** | Correctly excluded. The *content* of the kernel's theorems is still a specification target; only the mechanism is out of scope. |

### Verdict counts

| Verdict | Count | Rows |
|---|---|---|
| `expressed` | 3 | 2, 13, 15 |
| `expressed differently` | 4 | 4, 9, 10, 22 |
| `expressed differently — gain` | 1 | 3 |
| `partial` | 8 | 1, 5, 8, 11, 12, 14, 21, 28 |
| `absent` | 12 | 6, 7, 16, 17, 18, 19, 20, 23, 24, 25, 26, 27 |
| `out-of-scope-by-design` | 1 | 29 |
| **Total** | **29** | |

Combining the two "expressed differently" lines: **expressed 3 · expressed differently 5 · partial 8
· absent 12 · out-of-scope-by-design 1**.

### The distinction that carries the verdict

Not one of the 29 rows is **demonstrated** at kernel scope in Moriarty. The strongest Moriarty
evidence is the recorded K-vs-TypeScript lifecycle run: "125 expression cases, 104 lifecycle cases …
Both complete suites exited zero", four-step loan lifecycle with outstanding 0 → 100 → 110 → 80 → 0
(`deliverables/k-lifecycle-execution-2026-09-17/RESULT.md:3-5` **[delegated]**), whose own scope
caveat is "These are finite native conformance results. They do not discharge determinism, progress,
preservation, termination or general correspondence theorems" (`:32-34` **[delegated]**). That
demonstrates rows 2, 13, 15 and 22's atomic-rejection behaviour. It demonstrates nothing about
capabilities, footprints, components, ports or composition modes, because none of those exists to
demonstrate.

On the Moriarty contract side the position is uniform and self-reported: `enforcement-map.json` has
**84 rows, all `status: "NOT_ENFORCED"`** (my own executed count over the JSON);
`source-core-embeddings.json` is **1 present, 17 partial, 66 absent** (my own executed count; the one
`present` leaf is `profiles.semanticProfile`); `k-reconciliation.json` is **0 covered, 5 partial, 18
not-covered** (my own count). `EXIT-GATE.md:20-26` records the same numbers from checker runs that
all exited 0 — which, per `UNIFIED-PROPOSAL.md:264`, establishes nothing.

---

## 4. Answers

### 4.1 What the stage relation expresses, differently, and not at all

**Directly expressed (3 of 29).** Nonnegative balances, refusal-with-reason-and-no-post-state, and
the nonnegativity gate. These are the parts of the kernel that survive contact with a
single-transaction, single-signer model.

**Expressed differently, with a real loss (5).** Four of the five are real losses and one is a gain.

- *Gain:* the dimensioned value types (row 3). Moriarty's `financial-expression-types-v1.ts:27-43`
  is strictly more expressive than `Types.lean:41-46`: shares indexed by vault *and* holder, explicit
  0..18 scales, a forbidden `Price<A,A>`. Where the kernel needs no scale because it has ℚ, Moriarty
  needs one and has it. This directly answers "a share amount and a dollar amount are not
  interchangeable without an explicit conversion": **yes, in the expression type checker**.
  `UNIFIED-PROPOSAL.md:41` (F10) records the catch — "'Unit dimensions' are implemented in the type
  checker but recorded only as widths" — i.e. the stage relation loses what the type checker knows.
- *Loss:* rationals → bounded integers with a rounding policy (row 4, §4.5).
- *Loss:* capability → allowance (row 10, §4.4).
- *Loss:* prefix-retaining sequencing → atomic batch rejection (row 22, §4.2).
- *Neutral-to-gain:* request → signed intent (row 9). Moriarty's signed intent carries outcome
  constraints (`grossDebitCap`, `feeCap`, `minNetOutcome`, `validity`, `recipients[]`) that the
  kernel has no vocabulary for at all. `docs/MORIARTY-CONSOLIDATED-DESIGN.md:56` is explicit about
  why: "Track gross debit and fee limits separately from net outcomes… A refund cannot replenish
  authority to evade a gross cap unless that replenishment was explicitly authorized." The kernel's
  authority model, being per-cell-debit and non-consumable, cannot state that.

**Not expressed at all (12).** These fall into three clusters:

1. **Footprints** (rows 6, 7, 16). The stage relation has no read set, no write set, no domain
   locality. My executed grep over `schemas/stage-relation.schema.json` for
   `read / reads / footprint / balance / preState / postState` returned **0 hits each**. A stage
   records *deltas* (`effects.gross[]`, `effects.net[]`, `effects.fees[]`, `effects.supplyChanges[]`)
   and *liability endpoints* (`liabilities.opening[]`, `liabilities.closing[]`), but never the state
   it read or the region it may write.
2. **Structure** (rows 17, 18, 19, 20, 27). No components, ports, catalog, workflow, or capability
   store. `wiki/moriarty-architecture.md:102-104` and
   `deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md:122` design them; nothing
   specifies them.
3. **Composition and settlement** (rows 23, 24, 25, 26). The four modes are *named verbatim* in
   Moriarty's design — `docs/MORIARTY-CONSOLIDATED-DESIGN.md:72` reads "Sequential composition
   retains committed prefixes. Disjoint parallel composition requires checked independence and
   complete joining. Shared-state interleaving requires interference reasoning. Atomic publication
   applies only within a declared domain and its actual phase semantics." That sentence is a
   one-to-one restatement of the kernel's four modes. `LANGUAGE-DESIGN.md:122` adds a fifth
   (asynchronous messaging) and then says: "**Exact spellings remain to be specified with examples.**
   Each requires different conflict, custody, authority and residual rules. A generic `and` is only
   Boolean conjunction and cannot stand for financial composition."

**The honest summary.** Moriarty's design has *already decided* it needs the kernel's model. What it
lacks is not the concept but the syntax, the schema fields and the rules. Every absent row is an
absence at the **specified** level, not at the **designed** level — except rows 6, 7 and 16
(footprints), where even the design says nothing.

### 4.2 Granularity: is a kernel workflow a sequence of Moriarty stages?

**Partly, and the losses are precise.**

A kernel workflow is `run cfg boundaries world steps` over `steps : List (Step P A D)`
(`Composition/Sequence.lean:49-51`). A Moriarty stage is one instance of the stage relation, with
`lifecycleIds.{lifecycleId, stageId, logicalRequestId}`, `predecessorCommitments[]` and
`outcome.{kind ∈ {continuation, terminal}, continuations[]}` (schema, read at `:530-551`). So a
sequence of stages linked by predecessor commitments *is* the intended encoding of a workflow, and
`ROADMAP.md:25` calls it "ledger induction".

What is lost, in order of severity:

1. **The typed output→input edge.** The kernel's workflow primitive is
   `InputSource.priorOutput (step, port)` (`Interfaces.lean:69-71`) — step *n*'s argument is
   literally step *m*'s recorded output *at a named typed port*. Moriarty has
   `outcome.continuations[]` as an array of **strings** (`:545-550`) whose embedding is **absent**
   ("No stage-continuation identifier exists in source/5 or core/1"). There is no port, so there is
   nothing to name; there is no `OutputObservation`, so there is nothing to carry. The chaining in
   the executed loan demo is hand-written JavaScript in the harness
   (`examples/loan-lifecycle.mjs:21-30` **[delegated]**), not a language construct. **This is the
   sharpest granularity loss: a kernel workflow's data flow is not representable.**
2. **Intermediate commitment.** The kernel commits after each successful step: `advance` returns a
   new `Cursor` whose `world` is the post-world and whose `events` grew by one
   (`Sequence.lean:41-43`), and a later refusal cannot undo it (`continueRun_failed` `:85-92`). A
   Moriarty batch commits once or not at all: `runActions` aborts on first failure and publishes
   nothing (`financial-lifecycle.ts:1902-1904` **[delegated]**). A *sequence of stages* recovers
   intermediate commitment at stage granularity, so the loss is confined to **within a stage**: a
   kernel workflow whose step 3 fails after steps 1–2 committed is representable only by splitting it
   into three Moriarty stages, which changes the authority, replay and history accounting (each stage
   needs its own `signedIntent`, its own `authority.consumed` and its own predecessor commitment).
3. **Shared-state interleaving.** Not representable at all. A schedule is a *first-class input* in the
   kernel (`Schedule ::= List BranchId`, `Schedule.lean:8`; checked complete at `:17-20`) and the
   kernel deliberately *admits* overlapping footprints in this mode (`Schedule.lean:3-4`). A sequence
   of Moriarty stages linearises; it cannot express "these two branches ran under *this* schedule and
   here is what each saw." Nor can it express a branch that refused while its peer continued
   (`Interleaving/Execution.lean:5-6`).
4. **Disjoint-parallel joins.** Not representable. The kernel's join is `mergeWorld`
   (`Parallel/Execution.lean:22-33`), which reconstructs one ledger from two independently executed
   branches by consulting the *declared write footprints*. Without footprints (row 6/7) the join is
   not even definable. Note the kernel's admission is **conservative and pre-execution**: a
   compatibility conflict refuses *before either branch runs* (`Execution.lean:41-42`), which is a
   different failure mode from anything Moriarty has.
5. **Partial commitment on refusal.** The kernel's refusal is `LocatedFailure {index, step, reason}`
   with the prefix retained. Moriarty's is `RejectedLifecycle {code, actionIndex}` with nothing
   retained. The *design* wants the kernel's behaviour — `docs/MORIARTY-PRODUCT-CONTRACT.md:47`
   demands modelling "a failed fallible phase [that] can retain guaranteed-phase effects and fees",
   and `failurePolicy.{phasePolicy, retainedEffects, retainedFees}` are schema leaves for exactly
   that. But all three are `"type": "string"` (schema `:510-528`, read directly), all three have
   **absent** embeddings, and `UNIFIED-PROPOSAL.md:74` scopes S0 to "Atomic rejection with an empty
   retained-effects set" — i.e. the first slice deliberately picks the *degenerate* case.

**Net.** A kernel workflow whose steps are sequential, whose inputs are literals, and which has no
capability administration steps is representable as a sequence of Moriarty stages at the cost of
re-issuing intent per stage. Everything else — typed output chaining, parallel joins, interleaving,
partial commitment — is not.

### 4.3 Conservation and supply

**The kernel.** One rule, decided, and proved:
`accountingOK = ∀ d a, ∑_p effect(d,p,a) = supply d a` (`Transition.lean:145-146`), refusal
`accounting` (`:58`), consequence `total post d a = total state d a + e.supply d a`
(`applyEvaluated_accounting`, `:248-257`). Supply change is **separately authorised** by a
`changeSupply(domain, asset)` capability (`Authority.lean:11`), and the theorem
`execute_supply_authority` (`Transition.lean:348-359`) shows any change in the per-(domain,asset)
total forces such a capability. Conservation and authority are two independent checks that happen to
mention the same quantity.

**Moriarty.** Three layers, none of them closed:

- *Schema:* `effects.supplyChanges[]{asset, account, amount}` exists, with `amount` matching
  `^-?[0-9]+$` (schema `:288-301` shape, verified by grep). Note the kernel's supply is keyed
  `(domain, asset)`; Moriarty's is keyed `(asset, account)` — **a supply change in Moriarty names an
  account**, which is a different object. The kernel deliberately does *not* attribute supply to an
  account: `supplyChange : Asset → ℚ` in the pilot (`Core.lean:52`) and
  `SupplyDelta {domain, asset, amount}` in the typed layer (`Transition.lean:14-17`).
- *Law:* E1 "per-asset conservation: Σ gross = Σ supply changes = 0" and E2 "net is derived from
  gross and fees" (`UNIFIED-PROPOSAL.md:90-91`). **E1 as written is not the kernel's rule.** The
  kernel's rule is `Σ effect = supply`; E1 sets *both* sides to 0. As stated, E1 is only sound for a
  slice with no supply changes — which is exactly S0's scope ("Empty supply changes … bound as an
  explicit empty set that rejects any non-empty value", `UNIFIED-PROPOSAL.md:74`). For a slice with
  minting, E1 would have to become `Σ gross = Σ supplyChanges`, which is the kernel's rule. **This is
  a substantive defect in the proposed law, not a presentational one, and I flag it as such.**
- *Implementation:* none. All three `effects.supplyChanges[].*` embeddings are `absent`
  ("No supply-change account exists in source/5 or core/1"), and `financial-lifecycle.ts`
  `LifecycleState` (`:129-139`) has no supply field. Value only moves between existing `Balance`
  rows; `Originate`/`Accrue` change obligation amounts, not token supply **[delegated]**.

**The design does state the kernel's separation.** `docs/MORIARTY-PRODUCT-CONTRACT.md:43`: "Complete
effects include an authenticated domain/frame and per-asset accounting for transfers, fee recipients,
custody/reserves and authorized mint/burn supply changes. Liabilities have a separate typed
evolution: opening plus creation/accrual minus explicit discharge equals closing. **Debt is not token
supply.**" That last sentence is the same commitment as `Core.lean:38`. And
`docs/MORIARTY-CONSOLIDATED-DESIGN.md:54` adds what the kernel lacks: "Token conservation alone
cannot prove these properties" — i.e. Moriarty knows conservation is necessary and insufficient,
which is correct and which the kernel does not claim either.

**Verdict.** Conservation is **designed correctly**, **specified wrongly** (E1 as written),
**unimplemented**, and **unenforced** (the `effect` judgment is one prose sentence,
`judgments.json:52`; all 84 enforcement rows are `NOT_ENFORCED`). The kernel's *authority* half —
that a supply change requires its own right — has no Moriarty counterpart at any layer.

### 4.4 Capability IDs vs signed intent

These are **two different authority models solving two different problems**, and neither subsumes
the other. That is the central finding of this section.

| Axis | Kernel capability | Moriarty signed intent + allowance |
|---|---|---|
| Granularity | one exact cell or one `(domain, asset)` (`Authority.lean:10-11`) | a set of assets and recipients plus scalar caps (`signedIntent.assetIdentities[]`, `.recipients[]`, `.grossDebitCap`, `.feeCap`, `.minNetOutcome`) |
| Quantity | **none** — a debit right is unbounded in amount | `Allowance {remaining, spent}` (`financial-lifecycle.ts:50-55`) is consumable; caps bound the total |
| Issuance | `issueCapability` by the domain admin, fresh ID, cannot reactivate (`Authority.lean:64-74`) | no issuance operation; an allowance is state, not a grant |
| Revocation | `revokeCapability`, idempotent, ID retained as a tombstone (`Authority.lean:76-85`) | none; `signedIntent.recoveryPolicy` and `.replayPolicy` are schema strings |
| Replay | explicitly **out of scope** (`Authority.lean:5`) | `authority.replayState`; `usedTransferIds`/`usedAllocationIds`/`usedOriginationIds`/`usedAccrualIds` in `LifecycleState` (`financial-lifecycle.ts:134-137`), enforced by `DUPLICATE` rejections |
| Invocation right | `Right.invoke`, checked before evaluation (`Transition.lean:183-184`) | none |
| Binding to a caller | `cap.holder = ctx.principal` (`Authority.lean:93`) | `signedIntent.signer` is a schema string; no caller in the evaluator |
| Outcome constraints | none | `minNetOutcome`, `validity`, `consentPolicy`, `delegationPolicy` |

So the kernel is **strong on scope and lifecycle, silent on quantity and replay**; Moriarty is
**strong on quantity, replay and outcome, silent on scope and lifecycle**. The kernel says so itself
(`Authority.lean:5`: "allowances and replay prevention are outside this model"). Moriarty says so
too, in the opposite direction: `docs/MORIARTY-CONSOLIDATED-DESIGN.md:68` "Initiate, complete,
reconcile, recover, disclose and amend rights have separate scopes" — six *kinds* of right, which is
the kernel's `Right` inductive generalised, and which nothing implements.

The Atlas's own law is closer to Moriarty's target than to the kernel's implementation: L17,
`Au → bounded scope + revocation + expiry + nonce/domain separation`
(`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:757`). The kernel supplies bounded scope and revocation; Moriarty
supplies nonce (replay state) and expiry (`signedIntent.validity`). **Neither project satisfies L17
alone.** Hazard X16, "Unbounded delegated authority — `Au` without scope bounds, or unlimited token
approvals" (`:805`), is the failure both models are half-protected against.

Three Moriarty-specific facts to carry forward:

1. `authority.{consumed, remaining, replayState}` are three untyped strings
   (`schemas/stage-relation.schema.json:452-470`, read directly). `UNIFIED-PROPOSAL.md:35` (F4)
   calls this out: "authority as opaque strings". S2 proposes "Authority is per-asset vectors plus a
   replay-ID set" (`:115`).
2. Their embeddings are `partial`, mapping to `Allowance.spent`, `Allowance.remaining` and
   `usedTransferIds` (`source-core-embeddings.json`), with the note "there is no
   `authority.consumed` record… `Work.spent` is a computation budget." So the mapping is by name,
   not by meaning.
3. The proposed laws A1/A2 — "authority remaining = previous remaining − consumed, and ≥ 0; replay
   freshness" (`UNIFIED-PROPOSAL.md:93`) — are the *allowance* laws. Nothing proposes a capability
   law. **If capabilities are wanted, no current U0 workstream introduces them.**

### 4.5 Exact rationals vs checked bounded integers with a rounding policy

**Where it is only a representation choice.** For every operation the kernel performs whose result is
exact — addition, subtraction, a 1:1 conversion, a `min`/subtract split — the two models agree
pointwise. The kernel itself provides the bounded-integer layer (`Arithmetic/Word.lean:6-9`,
`Rounding.lean:18-22`) precisely so that a machine-width refinement is expressible, and
`lean/README.md:63` lists "machine-width arithmetic refinement" as an *open migration obligation*,
not a settled property. Moriarty's S0 slice is chosen to sit entirely in the exact fragment:
"Arithmetic used: checked add and subtract on u128, construction of checked amounts, comparison, and
the AccrualFirst `min`/subtract. **That is no open numeric gap.**" (`UNIFIED-PROPOSAL.md:76`). So for
S0 this is purely representational.

**Where it bites — four places, in order of severity.**

1. **The remainder has nowhere to go.** The kernel's `mulDiv` returns a checked quotient and drops
   the remainder, and its *specification* pins the direction (`Rounding.lean:25-60`). Moriarty
   requires a beneficiary and has none: `numeric-profile.json` `reserveMechanism` reads
   `"status": "absent", "citations": []` with the note "Successor sources declare no protocol-reserve
   type, field, or function… **Every ceil or floor primitive is an open conformance gap because the
   rounding remainder has nowhere to post.**" My own executed count over `numeric-profile.json`:
   **17 primitives, 11 `conforms`, 6 `open-gap`** — the six being `accrual-interest`,
   `expression-obligation-division`, `expression-receipt-division`,
   `origination-settlement-conversion`, `prorata-principal-share`, `repayment-settlement-conversion`.
   Every one of the six is a division. The kernel has no such gap because ℚ has no remainder.
   `UNIFIED-PROPOSAL.md:141-142` proposes five remainder classes (`none`, `conserved-split`,
   `charged-increment`, `sub-unit-residual`, `protocol-reserve`) — a **decision**, not yet made
   (`:226`, Phase A).
2. **Rounding direction becomes part of the financial meaning.** `defaultPolicy` is
   `{obligation: "ceil", receipt: "floor", exact: "none", remainderBeneficiary: "protocol-reserve"}`
   (`numeric-profile.json`). The kernel has no such policy because it has no rounding. The
   consequence is a genuine conflict recorded by a single reviewer and not yet resolved: "one field,
   `Conversion.rounding`, is required to be both floor (origination) and ceil (repayment) at
   `convertNominal`" (`UNIFIED-PROPOSAL.md:149, 277`). A ℚ kernel cannot even pose that question.
3. **Price orientation is inverted between the two projects, and the fix is arithmetic, not a
   rename.** Kernel: "A price is quote-asset units per one base-asset unit" (`Types.lean:40`).
   Moriarty: `priceOrientation.canonical = "base-per-quote"` (`numeric-profile.json`).
   `docs/MORIARTY-CONSOLIDATED-DESIGN.md:52` states the consequence: "DeFiFormal prices are
   quote-per-base while the current Moriarty convention is base-per-quote: adaptation requires an
   explicit dimensioned conversion and directed rounding, **not a rename**." Moriarty has already
   done the work: `numeric-profile.json` `defiformalConversion` gives the formula ("the directed
   reciprocal `10^(s+t)/M`… A receipt uses floor: `numerator//M`. An obligation uses ceil:
   `(numerator+M-1)//M`") plus four test vectors, including `3@scale1 → 34` (obligation) vs `33`
   (receipt). **In ℚ these two numbers are the same number.** This is the cleanest demonstration that
   the difference is not representational: the rational reciprocal of 0.3 is a single value, and the
   integer model has two, and which one you get is a financial decision about who bears the
   fraction. I did **not** execute these vectors.
4. **Conservation stops being automatic.** The kernel's accounting law is an equation over ℚ that
   holds or does not. Under directed rounding, `Σ effect` and `supply` can differ by a sub-unit
   residue that no party holds. `UNIFIED-PROPOSAL.md:142` names the only class that keeps the
   equation closed — "`protocol-reserve`, only for a whole atomic unit of an identified asset **that
   would otherwise leave the conservation equation**" — and `:141` names the one that cannot be
   closed: "`sub-unit-residual`: not value, not posted, and not rounded again." Opus L3's point is
   recorded at `:279`: "sub-quantum remainders cannot be posted." **So E1 (§4.3) and the rounding
   policy interact, and no document in this study reconciles them.**

**Summary.** Representation-only for S0. Load-bearing the moment any division appears — which is
every AMM, every interest accrual, every pro-rata allocation, i.e. the subject matter of six of the
other eight reviewers.

### 4.6 Residue — the Atlas's, and Moriarty's

**What the Atlas says it cannot express.** Residue is a mandatory, per-molecule, structural field:
required by the grammar (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:632-636`), a formula is ill-formed
without it (`:689-692`), and §12.4 (`:731-733`) makes it a *finding*: "what the vocabulary could not
express about this protocol. Recurring residue across three unrelated protocols is the primary
evidence for a new element." It is produced by pass 4 of decomposition, where "Every entrypoint not
mapped in pass 1 appears here" (`:964`). The discipline is stated once, at `:30`: "Residue is
reported, never hidden."

The Atlas's named structural blind spots, each an admission about the vocabulary itself:

1. **Implementation integrity.** "v0.1 had no representation anywhere — not in elements, bonds,
   conditions or residue — of smart-contract control-flow safety. A design could pass every rule in
   this document and still ship a reentrancy-class or share-inflation-class loss" (`:884-887`);
   closing with "**Nothing in the element vocabulary can see that**" (`:905`). Handled as a mandatory
   *companion program*, explicitly "not a set of elements" (`:889`).
2. **Operational security.** Failure category (d), "Outside the table — operational: Key compromise,
   phishing, front-end injection, coerced signer" (`:928`), and "by 2025 dollars that is where the
   losses are" (`:60-61`).
3. **Legal instruments.** "Not a description of a legal instrument. A molecule describes mechanism,
   not obligation" (`:1107-1122`). The fifth `—l→` legal bond is **unresolved dissent**, not a
   settled exclusion (`:569-575`, `:1159-1178` item 1).
4. **Class-vs-instance granularity.** "two formulas can compare equal while requiring wholly
   incompatible integrations. That makes the near-isomer procedure (§18.5) unsound at deployment
   granularity" (`:660-666`).
5. **Hazard thresholds.** Eight rows "contain undefined terms — *hard* redemption, *exogenous*
   capital, *dominates*, *illiquid*, *stale*, *unrestricted*, *mostly*, *high-frequency*"
   (`:829-837`), and the table "has **no denominator**" (`:777-782`).
6. **No numeric semantics at all.** There is no statement anywhere in the Atlas about rationals,
   integers, fixed point, units or overflow at the vocabulary level; the only touch-point is one
   checklist row in the *non-element* integrity overlay, "Direction of rounding always favours the
   pool" (`:897`), and the acknowledgement that a "per-element parameter registry with units and
   bounds" is owed work (`:1153`). Criterion P (`:106`) deliberately abstracts away "a specific VM's
   call, storage, gas or ordering model."
7. **Composition does not preserve validity.** Not in the Atlas itself but in the algebra that scores
   it: `algebra/THEOREM-LEDGER.md:37` (P10) "`⊕` is not a congruence for validity — refuted on all
   four closures with witnesses", clarified at `:80` (K4): "observational equivalence is preserved by
   composition and *admissibility is not*." The best partial result certifies 59% of live protocol
   pairs (`:63`; requirement `algebra/REQUIREMENTS.md:77-83`).

**Does Moriarty inherit those limits?** Item by item:

- *(1) Implementation integrity* — **inherited, and differently located.** Moriarty's analogue is
  compiler and lowering correctness, and it is already a named obligation rather than a blind spot:
  `docs/MORIARTY-PRODUCT-CONTRACT.md:31` "constrained execution must exclude invalid witnesses as
  well as admit source-valid executions"; `docs/FOOTGUNS.md` rule 4 requires inspecting the
  production path. So Moriarty has the concept the Atlas lacks. It does not yet have the evidence.
- *(2) Operational security* — **inherited unchanged.** `docs/MORIARTY-CONSOLIDATED-DESIGN.md:97`
  "A foreign destination that accepts only a threshold signature can be bypassed if that threshold is
  compromised, even when honest signers check proofs"; `:99` "A TEE attestation cannot replace a
  missing program proof." Key compromise is outside both models.
- *(3) Legal* — **inherited by declaration.** `algebra/REQUIREMENTS.md:126-127` puts "Obligor,
  custody, register of record and legal recourse… out of scope by declaration, not by oversight";
  `wiki/defiformal-taxonomy.md:186-189` puts "legal enforcement, identity providers, validator
  duties, oracle data acquisition… outside Core as explicit capabilities and assumptions"
  **[delegated]**. Both projects exclude it deliberately and say so.
- *(4) Class-vs-instance* — **not inherited in the same form, and Moriarty is better placed.** The
  Atlas's problem is that a symbol collapses many deployed markets. Moriarty's stage relation is
  per-*instance* by construction (`lifecycleIds`, `programIdentity`, `circuitIdentity`), so the
  collapse cannot happen. Moriarty's *own* granularity problem is the opposite one (row 21: no
  per-step before-world).
- *(5) Thresholds* — **not inherited.** This is a property of a classification vocabulary, not of an
  execution semantics.
- *(6) Numerics* — **not inherited; this is where Moriarty is ahead of both.** The Atlas has no
  numeric layer and admits the parameter registry is owed; the kernel has ℚ and an unrefined
  bounded-integer sidecar; Moriarty has a decided numeric profile with a price orientation, a rounding
  policy, 17 classified primitives, a directed defiformal conversion with test vectors, and an
  explicit `fieldElementCoercion: "forbidden"`. §4.5's four bite points are the *cost* of having done
  this work, not evidence of not having done it.
- *(7) Composition does not preserve validity* — **inherited, and harder.** The Atlas's own algebra
  refutes compositional validity with witnesses. The Lean kernel sidesteps it by making composition
  *operational* rather than *predicate-level*: each mode re-runs the actual per-step checks, so
  nothing is inferred about a composite from its parts. Moriarty must do the same or worse, because
  its acceptance relation is a *proof obligation* — the composite must be provable, not merely
  checkable. `docs/MORIARTY-CONSOLIDATED-DESIGN.md:87` already refuses the shortcut: "Native proof
  aggregation alone does not establish historical compliance." So the limit is inherited, and P10 is
  the reason no future Moriarty composition operator may be given a "validity is compositional" law.

**Moriarty's own residue discipline.** Moriarty has no per-stage residue field. It has something
structurally equivalent at the *contract* level: the three checker artifacts that record, per leaf,
what is not realised — `source-core-embeddings.json` with its classification rule ("absent = no
declaration found") and its limitation ("absence means not found by this recorded search; it is not a
proof of non-realisation", `EXIT-GATE.md:30`); `enforcement-map.json` with 84 `NOT_ENFORCED` rows and
per-row notes; `trust-premises.json` with 9 premises, 6 open. That is the same discipline — report
what the vocabulary cannot express rather than hide it — applied to the language contract instead of
to a protocol decomposition. **It is the strongest thing in the U0 package**, and the proposal keeps
it: `UNIFIED-PROPOSAL.md:263` "Do not grow the name-matching embedding table… to raise the counts."

What Moriarty **lacks** relative to the Atlas: a residue field on the *stage relation itself*. A
Moriarty stage cannot say "this program did something the stage relation could not record." The
schema is `"additionalProperties": false` at top level and in every nested object (verified by
parsing), so an unrepresentable effect makes the instance *invalid* rather than *residual*. The
Atlas's insight is that ill-formedness and residue are different failures (`:689-692` vs `:731-733`),
and Moriarty currently has only the first.

---

## 5. Structural gaps that would change the language design

Ranked by how much of the kernel model each unblocks. Each is tagged with what it needs and with the
Moriarty milestone that owns it **on the evidence I found**; `unassigned` where no document assigns
one.

### G1 — No declared read/write footprint anywhere in the language or the stage relation
**Needs: new language + new judgment clauses + schema.** **Owner: unassigned.**

Evidence: `Template.stateReads/envReads/writes` (`Typed/Transition.lean:26-28`) with checks at
`:121-125, 148-149`; my executed grep over `schemas/stage-relation.schema.json` for
`read|reads|footprint` → 0 hits each. No Moriarty document I read proposes one:
`UNIFIED-PROPOSAL.md` §5 S2 lists six schema changes (`:111-117`) and a footprint is not among them.

Why it ranks first: footprints are the **enabling abstraction for three other gaps**. Disjoint
parallel admission (G3) is decidable only because footprints are computable without evaluating
values (`Parallel/Compatibility.lean:35-52`). The join `mergeWorld` is *defined* by the write sets
(`Parallel/Execution.lean:24-26`). Frame reasoning — the kernel's `applyEvaluated_frame`
(`Core.lean:138-145`) — needs the write set as its hypothesis. And a ZK circuit needs to know which
state it reads: `docs/MORIARTY-CONSOLIDATED-DESIGN.md:46` already demands "for each field the actual
circuit constraint, authenticated state read, signature commitment or ledger check", which is a
footprint obligation stated without a footprint construct.

### G2 — The four composition operators are named in three documents and specified in none
**Needs: new language + new kernel semantics.** **Owner: U3 for joins (`ROADMAP.md:24`), U4 for
"every retained composition operator demonstrated" (`ROADMAP.md:25`). The *specification* is
unassigned.**

Evidence: `docs/MORIARTY-CONSOLIDATED-DESIGN.md:72` names all four in the kernel's own words;
`docs/MORIARTY-CONSOLIDATED-DESIGN.md:32` names them again in the responsibility table ("Distinguish
atomic batch, committed prefix, independent fork/join and interleaving");
`deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md:122` names five and says "Exact
spellings remain to be specified with examples." Against: `parallel`/`interleav`/`concurren` → 0 hits
in the language layer **[delegated]**; the only composition semantics in the repository is
`experiments/moriarty-language/spec/successor/composition-proposal.md`, which is **sequencing only**
and self-declared "Status: **PROPOSAL**, not registered. No runtime implements this document"
(`:3-4`), closing with "No correspondence is claimed between this rule and the TypeScript evaluator,
the K definition or the Compact compiler" (`:192-193`).

Why it ranks second: the kernel demonstrates that these four are *separable and separately provable*
(five accepted sprints, `docs/research/semantic-kernel-progress.md:149, 196, 236, 299`). Moriarty
currently has one operator (atomic single-batch) and calls it four in prose. The mode a program
wants determines its authority accounting, its failure behaviour and its proof shape, so this is a
language-design decision, not a library one.

### G3 — Capabilities do not exist; the allowance is not a substitute
**Needs: new language + new judgment clauses + enforcement machinery.** **Owner: unassigned** —
`UNIFIED-PROPOSAL.md:132` (S6) proposes a `SignedIntent` abstract syntax, not capabilities; A1/A2
(`:93`) are allowance laws.

Evidence: `Authority.lean:8-12, 21-30, 65-85`; `capabilit` → 0 hits in `spec/successor/` and
`src/successor/` **[delegated]**; `wiki/moriarty-architecture.md:99-100` designs the concept;
`authority.*` are three untyped strings (`stage-relation.schema.json:452-470`);
`docs/MORIARTY-CONSOLIDATED-DESIGN.md:68` names six right-scopes nothing implements. Atlas L17
(`:757`) requires "bounded scope + revocation + expiry + nonce/domain separation" — Moriarty supplies
the last two, the kernel the first two.

### G4 — `effects.supplyChanges` is a schema name with no meaning, and the proposed conservation law E1 is wrong for any slice that uses it
**Needs: new judgment clauses + numeric-profile work.** **Owner: U0 S3 for the clause
(`UNIFIED-PROPOSAL.md:119`); the E1 defect is unassigned.**

Evidence: `Transition.lean:14-17, 145-146`; Moriarty's three `effects.supplyChanges[].*` embeddings
`absent`; `UNIFIED-PROPOSAL.md:90` states E1 as `Σ gross = Σ supply changes = 0`, which is the
kernel's rule only when supply is empty. Interacts with G6: under directed rounding, the equation
cannot close without a decided remainder class (`UNIFIED-PROPOSAL.md:141-142`).

### G5 — No components, ports or catalog, so there is no structural boundary and no typed workflow edge
**Needs: new language.** **Owner: unassigned** — `wiki/moriarty-architecture.md:102-104` designs
modules; `experiments/moriarty-language/spec/successor/grammar.ebnf:104-106` excludes `composition`
from the profile **[delegated]**.

Evidence: `Composition/Interfaces.lean:21-55, 69-76, 106-120`; `port` → 0 matches (my own executed
grep); `outcome.continuations[]` embedding `absent`. Consequence: kernel workflow data flow is not
representable (§4.2 item 1), and the only chaining in the executed demo is harness JavaScript
**[delegated]**.

### G6 — Rounding remainders have no beneficiary, so six of seventeen numeric primitives are open gaps
**Needs: numeric-profile work + an owner decision.** **Owner: U0, N1 and N2
(`UNIFIED-PROPOSAL.md:136-153`), sequenced as a Phase A owner decision (`:226`).**

Evidence: `numeric-profile.json` `reserveMechanism.status: "absent"`; my executed count 17 primitives
/ 6 `open-gap`, all six divisions; `UNIFIED-PROPOSAL.md:141-142` proposes five remainder classes.
Ranked here rather than higher because it is the **only** gap in this list with a named owner, a
named workstream and a named decision date. Also note the live hazard: `UNIFIED-PROPOSAL.md:219, 267`
record that declaring an unused reserve type would close all six gaps in the current checker — a
gap-closing mechanism that is not a capability.

### G7 — No domain index on the ledger
**Needs: new language + schema.** **Owner: unassigned**;
`docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:8` records the limitation without assigning it.

Evidence: `Typed/Types.lean:25`; `Balance {party, asset, amount}` (`financial-lifecycle.ts:44-48`).
Blocks `crossDomain` (`Transition.lean:129-131`), per-(domain,asset) totals, and the atomic
settlement lane `(domain, asset, vault)` (`Atomic/Policy.lean:8-15`). Ranked below G1–G6 because
Moriarty's near-term target is a single Midnight domain (`docs/MORIARTY-PRODUCT-CONTRACT.md:21`), so
the cost is deferred rather than immediate — but every cross-domain requirement in
`docs/MORIARTY-CONSOLIDATED-DESIGN.md:72, 85` presupposes it.

### G8 — Rejection is atomic with nothing retained, but the product contract requires phase-retained effects
**Needs: new kernel semantics + new judgment clauses.** **Owner: U3
(`ROADMAP.md:24`, "partial fill, persistent duty and continuation… actual phase effects").**

Evidence: `financial-lifecycle.ts:1902-1904` **[delegated]** vs `Sequence.lean:3-4`;
`failurePolicy.*` three strings with `absent` embeddings ("A failed suffix publishes no state");
`docs/MORIARTY-PRODUCT-CONTRACT.md:47`; `UNIFIED-PROPOSAL.md:74` scopes S0 to the degenerate case.

### G9 — The stage relation has no residue field, so an unrepresentable effect is invalid rather than reported
**Needs: schema + new judgment clauses.** **Owner: unassigned.**

Evidence: `docs/UNIFIED-DEFI-ELEMENT-TABLE.md:689-692` (ill-formedness) vs `:731-733` (residue) — the
Atlas separates them; `stage-relation.schema.json` is `"additionalProperties": false` throughout
(verified by parsing). The U0 package has the residue *discipline* at the contract level
(`source-core-embeddings.json`, `enforcement-map.json`, `trust-premises.json`) and should extend it
one level down.

---

## 6. Limits of this review

**What I could not verify.**

- **No Lean build.** I did not run `lake build`, `lake env lean DefiKernel/Audit.lean`, or
  `VerifyAxioms.lean`. Every statement that a kernel theorem *holds* is read from the source text of
  its statement and proof script. The build counts, mutation counts and axiom audits I quote from
  `docs/research/semantic-kernel-progress.md` are **the repository's recorded receipts, not my
  observations**. I did not check that the recorded candidate hashes match the checked-out tree.
- **No evaluator or K run.** I did not execute `prepareFinancialLifecycle`, the K definition or any
  test. The rejection behaviour in rows 13, 15 and 22 is read from code. This matches the study's own
  limit: "**Evaluator rejection not executed.** No reviewer ran the callable evaluator against the
  repay counterexample" (`UNIFIED-PROPOSAL.md:284`).
- **No U0 checker run.** I did not run `check_u0_*.py`. The counts I report (84 enforcement rows all
  `NOT_ENFORCED`; 1/17/66 embeddings; 17 primitives with 6 gaps; 0/5/18 K rows) are **my own
  executed `python3` parses of the JSON artifacts**, which agree with `EXIT-GATE.md:20-26`. Agreement
  between an artifact and a summary of that artifact establishes nothing about the world.
- **No numeric vectors executed.** The four `defiformalConversion` test vectors and the
  33-vs-34 example in §4.5 are read from `numeric-profile.json`, not computed by me.
- **Delegated reads.** Rows and claims marked **[delegated]** rest on two subagent reports I did not
  fully re-derive. I spot-checked the load-bearing ones myself —
  `LANGUAGE-DESIGN.md:122`, `wiki/moriarty-architecture.md:93-107`,
  `financial-expression-types-v1.ts:84-106`, `composition-proposal.md:3-4, 173-194`,
  `UNIFIED-DEFI-ELEMENT-TABLE.md:530-540, 729-733, 737-760, 775-808, 841-856, 882-895, 933-948` — and
  all matched. I did **not** independently re-derive: the 0-hit greps inside
  `experiments/moriarty-language/spec/successor/` and `formal/k/`; the contents of
  `financial-expression-v1.ts`, `lifecycle-kernel.k`, `lifecycle-v1.k`; the
  `k-lifecycle-execution-2026-09-17/RESULT.md` figures; `wiki/defiformal-taxonomy.md`;
  `wiki/defi-kernel-sdk-interface.md`; `wiki/defi-kernel-protocol-graph.md`; the non-
  `LANGUAGE-DESIGN.md` artifacts in `deliverables/defi-language-design-2026-09-07/`. I did
  re-derive the `port`/`capabilit`/`parallel`/`atomic`/`mint` greps over
  `src/successor` and `spec/successor` myself, and they returned zero as reported.
- **Files I did not open at all:** `docs/MORIARTY-BACKEND-REQUIREMENTS.md`, `algebra/MODEL.md`,
  `docs/unified-v0.1.md`, `wiki-llm/`, `lean/DefiKernel/Typed/Expr.lean`,
  `lean/DefiKernel/Composition/Contracts.lean`, `Metatheory/`, `Nary/`, `Interface/`,
  `CapabilityProvenance/`, `ConcentratedLiquidity/`. **`CapabilityProvenance/` and `Interface/` in
  particular may refine the capability and port abstractions beyond what I characterise in §2.3 and
  §2.5**; my verdicts for rows 10, 18 and 27 are therefore conservative on the kernel side. A reader
  extending this review should start there.

**What I assert by inference rather than by direct evidence.**

1. That `Core.lean` is superseded by `Typed/` as "the kernel" is my reading of the module layering
   plus `lean/README.md:22-39` and the Sprint 4 record. The repository does not say it in those
   words.
2. That E1 as written (`UNIFIED-PROPOSAL.md:90`) is wrong for a slice with nonempty supply changes
   (§4.3) is my derivation from comparing it with `Transition.lean:145-146`. No reviewer in the study
   raised it, and I did not test it against an instance.
3. That footprints are the enabling abstraction for the parallel and interleaving modes (G1) is
   inferred from the kernel's construction (`Compatibility.lean:35-52`, `Execution.lean:24-26`),
   not from a statement in either repository.
4. The verdict assignments in §3 are judgments against the four evidence-strength labels. Two rows
   are close calls: **row 3** could be read as `partial` if one holds that a dimension known only to
   the expression type checker and not to the stage relation is not "expressed at kernel level" —
   `UNIFIED-PROPOSAL.md:41` supports that reading; I graded it `expressed differently — gain` because
   the *language* expresses it. **Row 22** could be read as `absent` rather than
   `expressed differently`, on the grounds that atomic rejection is a different operator rather than
   a lossy encoding of the kernel's; I graded it `expressed differently` because both are coherent
   sequencing semantics and the product contract explicitly chooses one
   (`docs/MORIARTY-PRODUCT-CONTRACT.md:47`).
5. The gap ranking in §5 is mine. Only G6 and G8 have owners stated in a repository document; the
   G2 ownership split (specification unassigned, demonstration at U3/U4) is my reading of
   `ROADMAP.md:24-25`.
6. Nothing in this review establishes that any Moriarty capability exists, that any gap is closed, or
   that the UNIFIED-PROPOSAL should be adopted. It is a proposal (`UNIFIED-PROPOSAL.md:4`), it is
   untracked in git, and this review does not change that.

---

## Architecture axis (follow-up)

Added after the coordinator's clarification. **This section does not revise anything above.** The
earlier axis asked whether the kernel abstractions are built or specified; this one asks whether the
roadmap and design say *how they will be built and how the components fit together*. Nothing has to
exist; there has to be a design. Re-read in full for this section: `ROADMAP.md` (56 lines) and
`docs/MORIARTY-CONSOLIDATED-DESIGN.md` (123 lines). Same pins as §1.

### A1. The architecture as stated

Seven components, reconstructed from the design.

1. **The small core** — `MORIARTY-CONSOLIDATED-DESIGN.md:15`. Eight enumerated capacities: exact
   typed values · bounded evaluation · explicit state and effects · checked authority · assertions
   and refinements · authenticated evidence · persistent obligations and continuations · **defined
   composition operators**. This list is the architecture's load-bearing sentence: it is the only
   place the core's responsibilities are enumerated.
2. **Source libraries over the core** — `:15` ("Financial applications are source libraries over that
   core") and `:93`, nine families, each to receive "a scope matrix, independent source-derived
   expectations and held-out compositions".
3. **A language version** — `:15`: "A language version states exactly which constructors, bounds,
   numeric semantics and evidence relations its compiler supports." This is the designated carrier of
   the core/library interface (see A4).
4. **The public pipeline** — `:19`: author and inspect → elaborate/type/**effect**/bound check →
   expose proof obligations → compile to pinned ZKIRv3 → generate native evidence → submit → check
   actual ledger results. Compact is an optional intermediate under pinned correspondence.
5. **The acceptance relation**, whose content is the **canonical stage statement** (`:42`) and whose
   structure is the **four judgments** (`:48`: contract properties, intent refinement, valid
   state/effect transition, compliant history), with the **enforcement map** (`:44`, `:46`) as the
   artifact tying each bound field to "the actual circuit constraint, authenticated state read,
   signature commitment or ledger check".
6. **Certified primitives / jets** — `:91`, Simplicity-style: a certificate covers "values, failure
   behavior, complete effects, preconditions and the declared cost relation, plus valid-execution
   completeness and invalid-witness exclusion", and "**Call sites discharge preconditions and
   framing**".
7. **The three-column responsibility boundary** — `:23-36`, twelve concerns each split across
   Moriarty language / Federated DeFi Kernel / Midnight, mapped to CAKE's APSS at `:38`.

Plus a **K executable reference** (`:17`), an **authoring-tools ladder** (`:95`) and a **backend
evolution contract** (`:111` → `MORIARTY-BACKEND-REQUIREMENTS.md`). `ROADMAP.md:3` frames the whole
as "a permissionless language of provable financial intention, compiled to Midnight ZKIRv3", with the
Federated DeFi Kernel **optional**.

**This is a real architecture.** The three-column table is genuinely good: every concern that could be
smuggled between layers has a row, and each row says what each layer may and may not claim
(`:32` atomicity, `:34` recovery, `:35` privacy).

### A2. Kernel abstraction → designated Moriarty component

Scoring the 29 rows of §3 by *whether the design names a component responsible*, not by whether it
works.

| Kernel abstraction (§3 row) | Designated component |
|---|---|
| Ledger state, nonneg balances (1, 2) | core "explicit state and effects" `:15`; `:42` "chain/domain and authenticated state frame" |
| Dimensioned units, rationals→bounded ints (3, 4) | numeric profile, frozen at U0, bound by U1 certificates — `:52` |
| **Template registry (5)** | **none designated.** `:15` has no registry or operation-table capacity; `:42` binds "source/Core/program identity and deployed entry point" — a program, not a template |
| **Read / write footprint (6, 7)** | **none designated** — see A3/H1 |
| Supply changes (8) | effect side: `:42` "supply changes"; **authority side: none designated** (`:68`'s six scopes exclude issuance) |
| Request / signed intent (9) | `:42` "signed intention"; Intent row `:25` |
| Capability, rights, store (10, 11, 27) | core "checked authority" `:15`; six right-scopes `:68`; revocation and tombstones `:70` |
| Caller identity (12) | split by design: Permission row `:27` (Moriarty) and `:25` (Midnight "Signature and state consumption mechanisms actually used") |
| Refusal (13) | `:48` "Rejection/partial failure is itself a specified transition"; failure judgment |
| Conservation, nonneg result (14, 15) | "valid state/effect transition" judgment `:48` |
| Domain locality (16) | `:72` "Atomic publication applies only within a declared domain" |
| **Components, ports, catalog (17, 18, 19)** | **none designated** — `:15` has no module/interface capacity at all |
| Workflow edge (20) | *partial*: `:42` "resulting continuations", `:62`, `:66` name the carrier; the typed output→input edge has none |
| **Trace (21)** | **none designated** — the unit of binding is the stage (`:42`) |
| Sequential prefix (22) | `:72`, `:32` "committed prefix" |
| Disjoint parallel, interleaving, atomic (23, 24, 25) | *operator* designated — core "defined composition operators" `:15`, Atomicity row `:32` "Distinguish atomic batch, committed prefix, independent fork/join and interleaving", spelled out at `:72`. *Premises* not designated — see A3 |
| **Settlement policy, lanes/participants (26)** | **none designated** |
| Observations (28) | core "authenticated evidence" `:15`; `:42`; Conditions row `:30` |
| Lean (29) | excluded by design `:17` |

**Counts: designated 16 · partially designated 4 · none designated 8 · out-of-scope 1.** Compare the
implementation axis (expressed 3 / absent 12). **The architecture covers substantially more than the
specification does, which is the right direction.** The composition modes in particular are *not* an
architectural hole: `:15` names "defined composition operators" as a core capacity and `:32` assigns
mode-distinction to the Moriarty language column. What is missing is narrower and sharper.

### A3. Architectural holes — behaviour asserted, no component named

A hole is where the design *asserts* a property and names no component that produces or checks it.
This is strictly narrower than a specification gap: capabilities (§3 row 10) are a **spec gap**, not
a hole, because `:15` "checked authority" plus `:68`'s six scopes plus `:70`'s tombstones do
designate a home. Six holes, in dependency order.

**H1. Independence has no input.** `:72`: "Disjoint parallel composition requires checked
independence and complete joining." The *operator* is designated (`:15`, `:32`); the *premise* is
not. Independence is decidable only from a read/write footprint, and the core's eight capacities
(`:15`) contain no region, frame or footprint notion. The pipeline's "effect check" (`:19`) is the
nearest candidate and is the wrong object: throughout the design "effects" means financial deltas
(`:42` "complete gross and net effects including fees and supply changes"), not a touched region.
**This is not an omission from a list that could grow — it is a missing kind of thing.** Compare
`Parallel/Compatibility.lean:35-52`, where the footprint is computed *without evaluating any
financial value*, which is exactly why admission is decidable.

*Refinement, added after the A4 search.* A per-constructor `frame` field **does** exist, in
`experiments/moriarty-language/spec/successor/expression-signatures.json` (one of the 13 keys on each
of the 40 constructors). I parsed its value set: it has **four** distinct values — `unchanged`,
`extend locals exactly once`, `stage one ordinary field; no financial mutation`, and `append ordered
operation descriptor only`. These classify *which machine component* a constructor mutates, not
*which ledger cells* it touches. Two transfers between disjoint accounts have identical frames.
**This strengthens H1 rather than weakening it:** a frame notion exists, at a granularity that can
never decide independence, and no architecture document reconciles it with `:91`'s call-site framing
obligation or with `:72`'s independence requirement.

**H2. Interference reasoning has no reasoner.** `:72`: "Shared-state interleaving requires
interference reasoning." No capacity at `:15`, no pipeline stage at `:19`, no judgment at `:48`. Same
root cause as H1.

**H3. Framing is assigned to call sites with no frame construct.** `:91`: "Call sites discharge
preconditions and framing." This is the design's only use of "framing", and it makes framing a
*call-site obligation* while providing nothing to frame with. `ROADMAP.md:22` assigns U1 "caller
preconditions and declared logical/target cost relation" — preconditions and cost, **not framing**.
H1/H2/H3 are one hole with three faces: **no region notion anywhere in the architecture.**

**H4. The scope matrix has no producer.** `:93`: "Each family receives a scope matrix" — passive
voice, no component, no milestone. See A4; this is the largest hole by consequence.

**H5. Nine workflow states, no carrier.** `:62` enumerates submitted-unfunded, funded-into-escrow,
partially fulfilled, waiting-for-evidence, eligible-for-release, in-flight, delivered, unresolved,
recovering, and says "The product must distinguish these facts, even if the final syntax represents
them with several typed records rather than one enum." *The product* is not a component, and the
representation decision is explicitly deferred. The canonical stage statement offers one slot —
"resulting continuations or terminal outcome" (`:42`) — a binary that cannot carry nine states.

**H6. Supply change is bound as an effect with no authority scope.** `:42` binds "supply changes";
`MORIARTY-PRODUCT-CONTRACT.md:43` says "**authorized** mint/burn supply changes"; but `:68`'s
enumeration — "Initiate, complete, reconcile, recover, disclose and amend rights have separate
scopes" — contains no issuance right. So the design says supply changes are authorized and names no
authority that authorizes them. Contrast `Authority.lean:11`, where `changeSupply (domain, asset)` is
a first-class right.

**H7. A claimed recovery guarantee has no viability checker.** `:70`: "the program/ledger acceptance
predicate for any claimed recovery guarantee must establish a viable supported closure/recovery path
under named assumptions or reject a workflow claiming that property." That is a demanding static
analysis. No component is named; none of the four judgments (`:48`) covers it.

**Not holes** (correctly designated, merely unspecified): bounded termination — `:66` asserted, `:19`
"bound check" designated. Reserve work — `:70` asserted, `:42` "resource certificate and cumulative
reservations" designated. Netting's gross-preserving relation — `:56` asserted, `:48`
transition-validity judgment designated. Evidence/observations — `:15`, `:30`, `:42` all designate.
Numeric profile — `:52` designates *and* assigns milestones. Privacy and cross-domain compensation —
`:35`, `:72`, three-column rows designate.

### A4. The library/core interface — the biggest architectural gap

The design says libraries sit over the core (`:15`), names nine families (`:93`), and designates the
**language version** as the artifact that "states exactly which constructors, bounds, numeric
semantics and evidence relations its compiler supports" (`:15`). So the interface *has* a designated
carrier. The gap is that **no instance of that carrier exists or is required by any milestone.**

What I verified myself:

- **The U0 exit criteria contain no constructor inventory.** `ROADMAP.md:21` requires embeddings,
  six judgments, numeric profile, K reconciliation, target pins, enforcement map, trust premises.
  **No list of what the core's constructors are.**
- **The nearest artifact has no constructor column.** `deliverables/defi-language-design-2026-09-07/action-targets.csv:1`
  is `target_id,economic_family,actions_or_workflow,source_keys,locator,semantic_requirement,distinguishing_test,implementation_disposition`.
  It maps 24 DeFi targets to **prose semantic requirements** and a disposition — not to constructors.
- **The two family lists do not agree.** Design `:93` lists exact arithmetic/fees · payments and
  escrow · loans and claims · swaps and liquidity · shares/vaults · ACTUS · redemption/loss
  allocation · margin · asynchronous and conditional claims. `ROADMAP.md:48` lists ACTUS ·
  loans/claims · payments/escrow · AMMs · vaults · **netting** · redemption/loss allocation · margin
  · async/conditional workflows. Both are nine long; **"exact arithmetic/fees" is a family in the
  design and not in the roadmap, and "netting" is a family in the roadmap and not in the design.**
  The library inventory is not a single list.
- **Milestone coverage is slice-shaped, not family-shaped.** The only place the architecture connects
  a target to a primitive set is `ROADMAP.md:22`, U1: "every primitive needed by **the initial
  slice**". U6 owns the families (`ROADMAP.md:27`) but its evidence is "ACTUS fixtures/fields, DeFi
  action rows and held-out behaviors" — behaviours, not constructors. U2's "Generality argument
  covers the admitted grammar" (`:23`) is grammar-shaped, not family-shaped.

**Correction, from a delegated exhaustive search that returned after the paragraphs above were
written, with the load-bearing items re-verified by me.** My first framing was too strong in one
direction and not strong enough in the other.

- **The constructor inventory *does* exist.** `experiments/moriarty-language/spec/successor/expression-signatures.json`
  holds **40** base Core constructors — I parsed it: `len(d['constructors']) == 40`, each with keys
  `constructor, operands, result, rule, evaluatedOperands, typing, reduction, rejections, frame,
  entryWork, sources, decisionStatus, caseIds`. Eight financial constructors (F-CA…F-SEL) at
  `experiments/moriarty-language/spec/successor/financial-expression-source.md:67-90` make **48**,
  the number declared authoritative at `openspec/changes/aeon-refinement-integration/design.md:19-20`
  **[delegated]** — note that package is **untracked** in git, though the two files it points at are
  tracked. So `:15`'s designated carrier has a real instance. **A4's problem is not the inventory;
  it is the join.**
- **No scope matrix exists, and the promise has exactly two occurrences in the repository**, both
  the aspirational sentence itself: `docs/MORIARTY-CONSOLIDATED-DESIGN.md:93` and its draft
  `deliverables/consolidated-design-2026-09-19/design-draft.md:80` **[delegated]**. Nothing
  downstream implements it.
- **The closest artifact is family-keyed and has the right column name and the wrong content.**
  `deliverables/defiformal-study-2026-09-19/libraries-review.md:5` — header verified by me:
  `| Family | Current source / roadmap | Language primitive | Reusable library | External kernel /
  adapter responsibility and gap |`, nine rows (`:7-15`). Those nine rows are visibly the source of
  the `:93` family list. But the "Language primitive" cells are one-line prose capability sketches,
  and the table classifies **defiformal's** Lean families, not Moriarty's core.
- **There is exactly one worked precedent, and it is the template.**
  `deliverables/sp02-financial-pure-expression-2026-09-10/inputs/financial-pure-extensions.md:44-50`
  runs the right direction — financial need → constructors — lowering the ERC-4626 rule
  `floor(a*S/Va)` to an exact 9-node Core tree and concluding that the full financial interface needs
  8 added constructors, giving 48 **[delegated]**. **That is one library rule, never generalised to
  the other eight families.**
- **The obligation is filed, and its owner is an alias rather than a milestone.**
  `docs/ROADMAP-RECONCILIATION-2026-09-19.md:145` — verified by me — lists "enumerate exact supported
  constructors" as an obligation whose owner column reads "P0/P2". `ROADMAP.md:5` says P0–P7 "are
  retained requirement/provenance aliases, **not parallel work queues**". So the obligation is
  assigned to something the roadmap says is not a work queue.
- **The repository already records the consequence.** `UNIFIED-PROPOSAL.md:42` (F11, 6 of 6
  reviewers) — read by me in §1 — "ROADMAP.md:38 says 'connect the smallest useful signed financial
  stage' but never names it, so U1's 'every primitive needed by the initial slice' **has no
  denominator**."

**Revised verdict: the core's constructor inventory exists and is well-formed; the artifact that
joins it to the nine library families does not, has one worked single-rule precedent, and is owned
only by a provenance alias.** Until it exists, no one — including the eight category reviewers — can
ask "does this core suffice to build a perpetual / a CDP / a bridge?" and get an answer that is not
prose. That is the single biggest architectural gap in this pass, and it is upstream of all eight
category reports.

### A5. The canonical stage statement as an architecture — structural exclusions

`:42` opens "Every accepted stage binds:" and enumerates exhaustively. Three things it **structurally
excludes** — the shape forbids them, so no added field fixes it — as against things it merely omits.

1. **Everything below stage granularity.** The unit is one stage with one value per field
   (`lifecycle/stage/logical-request IDs`, one `signed intention`, one `selected failure/phase
   policy`). The kernel's `Event {index, step, before, result}` (`Sequence.lean:8-12`) and
   `LocatedFailure {index, step, reason}` (`:14-17`) have no place, because a trace is a *sequence of
   stages-worth of data inside one stage*. Excluded: intra-stage trace, per-step receipt, intermediate
   commitment, located refusal.
2. **Everything above stage arity.** The statement ends "and resulting continuations **or** terminal
   outcome" — one outcome. A fork/join produces two branch outcomes merged into one world
   (`Parallel/Execution.lean:22-33`), and interleaving produces two local states plus a schedule
   (`Interleaving/Execution.lean:10-15`, `Schedule.lean:8`). Neither is a *field of a stage*; both are
   properties of a composite the architecture does not have. Excluded: the join, the schedule, two
   simultaneous branch outcomes.
3. **Residue.** The enumeration is closed ("Every accepted stage binds:") and its schema realisation
   is `additionalProperties: false` throughout. An effect the relation cannot record makes the
   instance *invalid*, not *residual* — the Atlas separates those two failures
   (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:689-692` vs `:731-733`) and Moriarty has only the first.

**Deliberately excluded, and correctly reasoned:** multi-domain-within-one-stage — one
"chain/domain and authenticated state frame", with cross-domain pushed to a sequence of stages
(`:72`, "must never claim global rollback from a local model"). **Merely omitted, additively
fixable:** read footprint, capability IDs, supply-authority reference. These are new fields, not a
new shape.

### A6. Ownership of each hole

| Hole | Architecture work owned by |
|---|---|
| H1/H2/H3 — no region/footprint/frame notion | **unassigned.** `ROADMAP.md:24` gives U3 "joins"; `:25` gives U4 "every retained composition operator demonstrated". Both own *demonstration*; neither owns designing the premise. `ROADMAP.md:22` gives U1 "caller preconditions", explicitly not framing |
| H4 — scope matrix has no producer | **unassigned.** U6 (`ROADMAP.md:27`) owns family *evidence*, not the matrix |
| H5 — nine workflow states, no carrier | **partial: U3** (`ROADMAP.md:24`, "partial fill, persistent duty and continuation, joins"). The nine-way distinction itself unassigned |
| H6 — supply change has no authority scope | **unassigned** |
| H7 — recovery-guarantee viability checker | **partial: U3** (`ROADMAP.md:24`, "separate recovery authority… objective hostile controls"); the static predicate unassigned |
| A4 — constructor inventory and family→constructor map | **unassigned.** Nearest: U2's "Generality argument covers the admitted grammar" (`ROADMAP.md:23`) |

Six of seven are unassigned or half-assigned. Note the pattern: **`ROADMAP.md` assigns demonstrations
and evidence; it does not assign designs.** Every U row's "Evidence required to close" column is
phrased as artifacts and receipts. No row says "produce the architecture for X".

### A7. Minimum additions, ranked

The smallest set that lets all eight categories be built later without redesigning the core.

1. **A region notion in the core's capacity list (`:15`) — a declared read/write footprint per
   operation.** One addition closes H1, H2 and H3, and is the premise for joins, interference and
   framing. It is first because it is the only item that cannot be added later: independence checking
   is not a field you add to a stage, it is a thing operations must *carry*, so every constructor
   designed before it would need revisiting. Needed by bridges (fork/join), AMMs+lending (atomic
   composition), derivatives (multi-leg).
2. **One family→constructor map, and a U-milestone owner for it.** The inventory already exists (48
   constructors, A4); what is missing is the join and an owner that is not a provenance alias. The
   method is already demonstrated once, at
   `deliverables/sp02-financial-pure-expression-2026-09-10/inputs/financial-pure-extensions.md:44-50`
   **[delegated]** — pick a rule, lower it to a Core tree, read off the constructors it needs. Repeat
   for one rule per family and the artifact exists. Without it "libraries over the core" (`:15`) is
   not a checkable claim and no category reviewer can test sufficiency. Needed by all eight.
3. **A composite unit above the stage.** A named architectural object that a schedule, a join and two
   branch outcomes can belong to. A5 shows the canonical stage statement cannot grow into this.
   Needed by derivatives, bridges, and any atomic cross-protocol action.
4. **A supply/issuance authority scope, added to `:68`'s six.** One word in an existing enumeration;
   closes H6. Needed by stablecoins, staking/yield (share issuance), AMMs (LP tokens).
5. **A decision on intra-stage structure** — either an ordered step record inside a stage, or an
   explicit architectural statement that there is none and that lending liquidations and margin
   settlements must be multi-stage. Either resolves A5(1); silence does not.
6. **A residue slot on the stage statement.** Cheapest of the six and the one that makes the other
   eight reports honest: it lets a category reviewer record "this protocol does X and the relation
   could not represent it" without the instance becoming invalid. The U0 package already applies this
   discipline one level up (`source-core-embeddings.json`, `enforcement-map.json`).

**Bottom line for this axis.** Moriarty has an architecture, and a better one than the implementation
axis suggests: 16 of 29 kernel abstractions have a designated home, and the three-column boundary
(`:23-36`) is genuinely strong. It has one structural blind spot — **no notion of a region**, which
costs it independence, interference and framing at once — and one missing artifact — **the
constructor inventory and family map**, without which "financial applications are source libraries
over that core" (`:15`) cannot be evaluated by anyone, including the other eight reviewers.

**Limits of this axis.** I read `ROADMAP.md` and `docs/MORIARTY-CONSOLIDATED-DESIGN.md` in full and
verified every citation above directly. I did **not** read `docs/MORIARTY-BACKEND-REQUIREMENTS.md`,
`docs/ROADMAP-RECONCILIATION-2026-09-19.md`, `openspec/changes/consolidated-language-kernel/{proposal,design,requirements,traceability}.md`,
or `docs/superpowers/`; an architecture for any hole above could exist in one of those and I would
not have seen it. A4's negative now rests on a delegated exhaustive search (its recorded searches
include `grep -rn -i "scope matrix"` repo-wide → 2 hits, and a repo-wide sweep of every family-keyed
markdown table header) plus six items I re-verified myself: the 40-constructor parse, the
`libraries-review.md:5` header, `ROADMAP-RECONCILIATION-2026-09-19.md:145`, `ROADMAP.md:21`,
`action-targets.csv:1`, and the two divergent family lists. It should still be read as "not found by
this recorded search", not as proof of non-existence — the standard `EXIT-GATE.md:30` applies to
Moriarty's own artifacts. Items marked **[delegated]** in A4 I did not open:
`openspec/changes/aeon-refinement-integration/design.md`,
`deliverables/consolidated-design-2026-09-19/design-draft.md`, and
`deliverables/sp02-financial-pure-expression-2026-09-10/inputs/financial-pure-extensions.md`.

The hole/spec-gap boundary in A3 is my judgment; the closest call is H6, which could be read as a
spec gap if one holds that "amend" in `:68` covers issuance.

**One cross-reviewer correction.** `docs/UNIFIED-DEFI-ELEMENT-TABLE.md` **does not exist in the
Moriarty repository** — I ran `find . -name "UNIFIED-DEFI-ELEMENT-TABLE*"` from the Moriarty root and
it returned nothing. It is a **defiformal**-side path
(`/home/charl/projects/defiformal/docs/UNIFIED-DEFI-ELEMENT-TABLE.md`, commit `8c5dd10`), which is
how §2.8 and §4.6 above cite it. A peer report in this directory was observed citing it as a
Moriarty-relative path **[delegated]**; any such citation should carry the defiformal repo and commit,
or it will not resolve.

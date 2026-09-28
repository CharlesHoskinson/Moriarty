# R5 — oracles and external observations: DeFi Kernel abstraction versus Moriarty language design

Reviewer R5 of nine. Read-only review. This file is the only artifact written.

---

## 1. Scope and pins

**Pins (commands run, actual output).**

- `git -C /home/charl/projects/defiformal log -1 --format='%H %ad %s'`
  → `8c5dd103cd40369a763b02b1504441acce0ce3c2 Thu Sep 10 17:38:12 2026 -0600 Prepare independent Curve source-entry review`
- `git -C /home/charl/Moriarty rev-parse HEAD` → `8f73784042bd692733c296d0d49f5173be96725e`

**Startup.** The host did not expose `moriarty-dev:develop`, so I read and applied the checked-in skill
(`plugins/moriarty-dev/skills/develop/SKILL.md`), plus `AGENTS.md` and `docs/FOOTGUNS.md`, before forming
conclusions. I ran the guarded CLI in read-only mode:
`python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json`. Actual output:
capability `SP01.6 loan-swap-subset`; `blockedAction` = "implementation/repair of loan-swap-subset";
`nextAction` = `sp01-loan-report`; `missingEvidence` = binding/candidate-input-stale on
`openspec/sprints/sp01-financial-contract-and-execution-admission.md`,
`current-accounting-missing:.moriarty-dev/runtime/current-accounting.json`,
`resource-live-state-unavailable:sp01-loan-swap-grok-01`, `operational-history`; seven pending
transaction IDs listed. I dispatched nothing (`next`/`run` were not invoked).

**What I read in defiformal** (all read, not assumed): `README.md`; `lean/README.md`;
`lean/DefiKernel/Typed/{Types,Expr,Transition,Examples,Acceptance,ExprTests,TransitionTests}.lean` (the
first four in full or in the cited ranges, the test files by grep); `lean/DefiKernel/Examples.lean`
(untyped `Oracle`); `lean/DefiKernel/ContractExamples.lean` (by grep with context);
`docs/UNIFIED-DEFI-ELEMENT-TABLE.md` §§3–4 (G08), 7.2, 8.2, 9, 10, 12.3–12.4, 13, 15, 17;
`openspec/ROADMAP.md`; `openspec/specs/sequential-workflow-execution/spec.md`; `corpus50/vocab.md`
(oracle rows); `algebra/MODEL.md` by grep.

**What I read in Moriarty:** `deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md` (full);
`deliverables/u0-semantic-contract-2026-09-23/{trust-premises.json, judgments.json, enforcement-map.json,
numeric-profile.json, EXIT-GATE.md}`; `openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json`;
`docs/decisions/u0-numeric-profile-decision.md`; `docs/MORIARTY-CONSOLIDATED-DESIGN.md`,
`docs/MORIARTY-PRODUCT-CONTRACT.md`, `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md`, `ROADMAP.md`;
`experiments/moriarty-language/spec/successor/{README.md, grammar.ebnf, syntax-profile.json,
static-semantics.md, representation.md}`; `experiments/moriarty-language/spec/typed-schemas.md`;
`experiments/moriarty-language/spec/examples/loan.mori`;
`experiments/moriarty-language/src/{evaluate.ts, validate.ts, lower-compact.ts, runtime-types.ts}`;
`experiments/moriarty-language/src/successor/{financial-lifecycle.ts, financial-expression-v1.ts,
financial-agreement-source-compiler.ts}`; `experiments/moriarty-language/compact/generated/loan/kernel.compact`;
`experiments/moriarty-language/formal/k/{lifecycle-kernel.k, lifecycle-binding.k, lifecycle-infer.k,
lifecycle-schema.k}` (by grep); `wiki/security.md`, `wiki/defi-kernel-sdk-interface.md`,
`wiki/defiformal-taxonomy.md`; `deliverables/defi-language-design-2026-09-07/{README.md, LANGUAGE-DESIGN.md,
action-targets.csv}`; `deliverables/moriarty-decision-study-2026-09-02.md` (threat table);
`deliverables/u0-study-2026-09-28/briefs/`.

**What I did NOT execute.** I did not build or run Lean (`lake build`, `lake env lean`), did not run the K
semantics, did not run the TypeScript evaluator or `check_expression_contract.py`, did not run any U0
checker, did not compile Compact, did not prove or verify anything, and made no network request. Every
claim below about behaviour is read from source, not observed at runtime, unless it says otherwise.

**Method note.** I use the develop skill's and FOOTGUNS' distinctions throughout: *designed* (a document
states the intent), *specified* (a normative artifact fixes the rule), *implemented in the evaluator*
(callable code computes it), *demonstrated by executed evidence* (a retained run shows it). A field name in
a schema is none of these.

---

## 2. What the kernel abstraction requires for this category

The DeFi Kernel's oracle abstraction is a *typed, keyed, timestamped external environment that a template
must declare before it may read*. It is deliberately thin about truth and thick about footprint, dimension
and refusal. The Atlas (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md`) adds the classification and required-bond
layer above it.

### 2.1 Kernel (Lean pilot)

**K1 — Observations are a separate input space, not state and not arguments.**
`Environment Asset Domain := ObservationKey Domain → Option (Observation Asset)`, with
`ObservationKey = {domain, id}` and `Observation = {value : PackedValue, timestamp : Nat}`
(`lean/DefiKernel/Typed/Types.lean:114-128`). The environment is passed to `execute` as its own parameter,
beside the ledger, the capability store, the caller context and `now`
(`lean/DefiKernel/Typed/Transition.lean:171-174`). The workflow spec restates this: the system "SHALL
receive authenticated context, observations, and time separately from workflow requests"
(`openspec/specs/sequential-workflow-execution/spec.md:9-11`).

**K2 — An observation's value is dimensioned, and the delivered dimension is checked at lookup.**
The expected unit is intrinsic to the reference (`ObservationRef ... (unit : Unit Asset)`,
`Types.lean:119-122`); `readObservation` returns `.error .observationUnit` when the environment delivers a
different unit (`Typed/Expr.lean:117-124`). A price is a first-class dimension with explicit base and
quote: `Unit.price (base quote : Asset)`, documented as "quote-asset units per one base-asset unit"
(`Types.lean:40-46`).

**K3 — Time is two separate reads: the observation's own timestamp and the ambient `now`.**
`Expr.timestamp (key)` and `Expr.now` are distinct constructors (`Typed/Expr.lean:84-86`), evaluated
separately (`:136-139`), and produce distinct `EnvRead` entries (`EnvRead.observation key` vs
`EnvRead.currentTime`, `:98-101`, `:154-162`).

**K4 — Every template declares its external-read footprint, and execution refuses when a required read is
undeclared.** `Template.envReads : List (EnvRead Domain)` (`Typed/Transition.lean:27`);
`Template.requiredEnvReads` is computed syntactically from the guard, the cell deltas and the supply deltas
(`:84-86`); `Evaluated.envReadsOK` demands `required ⊆ declared` (`:124-125`); `applyEvaluated` returns the
dedicated refusal `.envReadFootprint` (`:52-60`, `:158`). The acceptance suite exercises this with a
mutant whose `envReads` is emptied (`Typed/Acceptance.lean:88`, `:238`).

**K5 — Missing and wrong-unit observations are distinct, total refusals, not defaults.**
`EvalFailure.missingObservation` and `.observationUnit` (`Types.lean:84-87`); refusal cases
`reference_oracle_missing`, `reference_oracle_wrong_feed` and `reference_oracle_wrong_dimension`
(`Typed/Acceptance.lean:216-222`). Binary operators including `and`/`or` evaluate both operands, so a
missing observation in a short-circuitable position still refuses (`Typed/Expr.lean:126-127`, tested at
`ExprTests.lean:115-120`).

**K6 — Feed identity is the key; another feed cannot substitute for the declared one.**
"Supplying a different feed creates a different key; it cannot replace feed seven"
(`Typed/Examples.lean:202-204`); `reference_oracle_wrong_feed` refuses with `.missingObservation`
(`Acceptance.lean:216-217`).

**K7 — Staleness, heartbeat and no-future-timestamp are in-language guard predicates over
(timestamp, now).** The borrow guard requires `timestamp(priceKey) ≤ now` and
`now ≤ timestamp(priceKey) + 5` (`Typed/Examples.lean:133-134`), with accepted boundary, stale and future
cases (`Acceptance.lean:205-207`, `:215`) and a theorem `stale_oracle_refused` (`:287-288`).

**K8 — A value sanity bound is part of the same guard.** `0 < collateralPrice`
(`Typed/Examples.lean:132`), with zero and negative refusal cases including an independent
zero-exposure control (`Acceptance.lean:208-214`).

**K9 — An observation becomes a premise of a financial decision only through a dimensioned conversion.**
`convert : Amount base → Price base quote → Amount quote` and its inverse `unconvert`
(`Typed/Expr.lean:44-46`, evaluated at `:69-71`); `collateralValue` converts the caller's collateral
balance by the observed price (`Examples.lean:127-128`) and the result appears in the borrow guard's
200%-collateral inequality (`:135-136`) and therefore gates the effect and the debt mint (`:138-149`).

**K10 — Determinism and framing over declared reads.** `Expr.eval_congr`: equal arguments, equal declared
state reads and equal declared env reads imply an identical evaluation result, "including missing-input,
wrong-unit and zero-division refusal behavior" (`Typed/Expr.lean:177-186`). `EnvRead.Agree` defines
agreement per read (`:170-173`).

**K11 — Observations may cross domains while state effects may not.** `Evaluated.domainOK` constrains
required state reads, net effects and supply changes to the invocation domain; the accompanying comment
says "Environment observations may explicitly refer to other domains; their truth is adapter supplied"
(`Typed/Transition.lean:127-131`).

**K12 — Oracle conditions can be placed in an independent layer that an untrusted proposal's own guard
cannot remove.** `BorrowConditions` restates feed, positivity, non-future and age-≤-5 plus the
collateral inequality "never read from the proposal guard" (`lean/DefiKernel/ContractExamples.lean:34-50`),
with `borrow_constructor_accepts_iff` and the post-state consequence theorem (`:103-121`).
`lean/README.md:41-47`: "Borrow contracts independently check the declared oracle and collateral rules, so
replacing a proposal's own guard with `true` cannot remove those rules."

**K13 — The trust boundary is stated, not proved away.** "An oracle supplies external data such as a price;
checking its type, sign, and age does not establish its truth" (`README.md:163-165`); "Oracle feed and
timestamp fields are declared inputs; checking them does not establish provenance or market truth"
(`lean/README.md:54-57`); "observation truth … [is an] external assumption" (`Typed/Types.lean:5-6`,
`Typed/Transition.lean:5-6`). Accounting preservation "does not establish market solvency"
(`README.md:170-171`).

**K14 — Exact rationals; no rounding on the observed price.** Balances, prices and conversions are `ℚ`
(`README.md:25-27`, `Typed/Expr.lean:3`); division checks its denominator explicitly (`Expr.lean:58-71`).
Rounding is listed as needing an additional model (`README.md:168-171`).

### 2.2 Atlas (unified element table)

**K15 — Oracles are four distinct elements in one group, G08 "Truth · where does external fact enter?":
`Ex` external data oracle, `Tp` on-chain time-weighted price, `Oa` optimistic assertion oracle, `At`
reserve/NAV/financial attestation** (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:267-281`).

**K16 — `Ex` carries a mandatory `delivery` isotope {push, pull, medianizer}; the failure difference it
preserves is named "staleness mode"** (`:276`, `:452`). `At` carries mandatory `subject` and `assurance`
(`:276-281`, `:462`, `:505`).

**K17 — Required-bond laws.** `L13: Ex → freshness validation` (`:753`);
`L1: (Pl|Im|Cd|Pf|Op) → (Ex|Tp|At) + Ct + (Li|Ad|Sl|Bs)`, marked **not async-safe** because cross-domain
use "also needs a named finality assumption and a liquidation-latency bound" (`:741`);
`L27: At → named attester + independence from the obligor + stated scope and assurance + periodicity and
staleness bound + recourse against the attester` (`:767`).

**K18 — The trust bond is a separate satisfaction obligation.** Four bond types, "each must be satisfied
separately"; `—t→` asks "Who can attest, censor, upgrade, pause, mint, freeze?" with failure "false
attestation" (`:530-540`). Valence derives the counterpart set from the laws: `Vₜ(Pl) = Ex|Tp|At`
(`:588-593`).

**K19 — Oracle latency is a *reaction condition*, carried per molecule, not a program field.**
"Oracle latency and confidence | `Pl` `Cd` `Pf` `Pm` `Ct` | Is the value current relative to volatility and
closeout time?" (`:933-939`); the Aave v3 molecule carries `conditions: oracle heartbeat vs volatility;
executable depth at liquidation size; gas` (`:708-715`).

**K20 — Manipulability is a hazard rule, priced against pool depth, not a language check.**
X2 `Fl* + manipulable Cp/Cl price + Pl/Cd, where manipulation cost < position value`; X3 protocol token as
collateral **and** oracle market **and** backstop; X10 `Pm` with stale reference and unrestricted inventory
(`:790-798`); screening step 4 "price the manipulation against *pool depth*" (`:1023`).

**K21 — Residue is a mandatory per-molecule finding**, and "derived observations are not elements" — a
health factor reports the result of `Ct` and changes nothing (`:729-733`, `:155-156`).

---

## 3. Coverage verdict per requirement

Strictness rules applied: a schema field name is not coverage; a checker exit 0 is not a capability; I
separate *designed / specified / implemented in the evaluator / demonstrated by executed evidence*.

A structural fact governs several rows: Moriarty currently has **two language profiles**, and they differ
on observations.

- The **atomic profile** (`moriarty-bounded-atomic/1`, `experiments/moriarty-language/src/evaluate.ts`,
  `spec/typed-schemas.md`, example `spec/examples/loan.mori`) **has** observation declarations, genesis
  provider bindings, per-observation evidence digests and an authenticity gate.
- The **successor financial profile** (`source/5`, `src/successor/`), which the U0 proposal's S0 slice is
  built on, **has no observations at all**: `observation` is an unsupported declaration
  (`experiments/moriarty-language/spec/successor/syntax-profile.json:53-70`, esp. `:60`;
  `spec/successor/grammar.ebnf:104-106`; `spec/successor/README.md:33-41`) and the compiler hard-codes
  `observations: {}` (`src/successor/financial-agreement-source-compiler.ts:596-608`, esp. `:606`).

| # | Requirement (kernel) | Verdict | Where in Moriarty (file:line) | What is missing |
|---|---|---|---|---|
| K1 | Observations are a separate typed input space, distinct from state and arguments | **covered** (specified + implemented in the evaluator; demonstrated in the generated circuit) | Core schema namespace `observations: {name:t(T),...}` `experiments/moriarty-language/spec/successor/representation.md:112`, `:139-140`; `ReadObs` typing `src/successor/financial-expression-v1.ts:282`, snapshot validation `:489`; K rule `formal/k/lifecycle-infer.k:80`, snapshot phase `formal/k/lifecycle-v1.k:75`; atomic-profile input `src/evaluate.ts:128`; lowered to `KernelObservations` `src/lower-compact.ts:28`, `:33`, `compact/generated/loan/kernel.compact:7`, `:13` | Nothing at this level. The namespace exists in both TS and K and reaches Compact. |
| K2 | Observation value is dimensioned; delivered dimension checked at lookup; price carries base and quote | **partial** | Price is a real dimensioned type: `Price<A,B,S>` with base/quote/scale, `numeric-profile.json:5-6` and its note; literal constructor `src/successor/financial-expression-v1.ts:230`; dimension rule `Amount<B> × Price<A,B,S> → ScaledAmount<A,S>` at `:341` (requires the Price's quote to equal the Amount's asset) | No observation can *carry* a Price. `source/5` has no observations (`financial-agreement-source-compiler.ts:606`); the atomic profile's observation types are the restricted local types, and the Compact lowering rejects `Text` observations (`src/lower-compact.ts:17`) and requires `now:UInt128` (`src/validate.ts:94`). There is no unit check *at observation lookup* comparable to `.observationUnit`. |
| K3 | Per-observation timestamp, separate from ambient `now` | **absent** | `now` is itself an observation, mandatory by name and type (`src/validate.ts:94` → `OBSERVATION_SCHEMA`; `spec/typed-schemas.md:476-478`); it is compared to the horizon and the validity window at `src/evaluate.ts:207` | `ObservationValue = {evidenceDigest, name, provider, value}` has **no** observed-at field (`spec/typed-schemas.md:462-464`; `src/runtime-types.ts:13`). `observations[].time` exists in the stage-relation schema as a free string (`openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json:261-263`), which is a name, not a capability. `now − obs.observedAt ≤ H` is therefore inexpressible. |
| K4 | Per-operation declared external-read footprint, with `required ⊆ declared` refused | **partial** | Program-level declaration is enforced: undeclared observation names reject (`spec/successor/expression-cases.json:509`); genesis `observationBindings` must cover the manifest schema exactly and uniquely (`src/evaluate.ts:192-193`); Compact observation struct fields are generated from the schema (`src/lower-compact.ts:28`) | The declaration is whole-program, not per-action/per-template. There is no analogue of `Template.envReads` with a computed `requiredEnvReads` and an `envReadFootprint` refusal, so one action cannot narrow its own read set, and a footprint mutant cannot be detected. |
| K5 | Missing vs wrong-unit observation as distinct total refusals | **partial** | Shape/type conformance of the supplied `Obs` snapshot is checked (`src/successor/financial-expression-v1.ts:489`; `src/evaluate.ts:191`); name resolution failure is `NAME_RESOLUTION` (`src/validate.ts:61`); missing mandatory `now` is `OBSERVATION_SCHEMA` (`:94`) | No dimension/unit failure class for observations (there is no observation dimension — see K2). No documented distinction between "feed absent" and "feed present with wrong unit". |
| K6 | Feed/issuer identity binds the read; another issuer cannot substitute | **partial** (implemented in the evaluator, not enforced natively, not demonstrated) | `observationBindings: [{authenticationPolicy, name, provider}]` in genesis (`src/evaluate.ts:84`; `spec/typed-schemas.md:439-440`); per-observation `provider` must equal the genesis-bound provider or `OBSERVATION_UNAUTHENTICATED` (`src/evaluate.ts:203`) | The check is **positional** (`observations[j]` against `observationBindings[j]`), compares free-text strings, and its premises are host booleans: `checks.observationsAuthentic` and `checks.genesisValid` are supplied by the caller and merely required to be `true` (`src/evaluate.ts:128`, `:202`). `evidenceDigest` is "an opaque external digest. The profile defines no preimage or algorithm for it" (`spec/typed-schemas.md:469-474`) — nothing in the evaluator verifies it. `authenticationPolicy` is never interpreted anywhere I could find. `observations[].issuer` is `NOT_ENFORCED` (`deliverables/u0-semantic-contract-2026-09-23/enforcement-map.json:350`). |
| K7 | Staleness / heartbeat / no-future-timestamp bound on the observation | **partial** | A *stage-level* time bound exists and is the one piece with executed lowering: `guard obs.now < uint(2000000000)` (`experiments/moriarty-language/spec/examples/loan.mori:74`, `:90`) lowers to `assert(observations.o0 < 2000000000, "HORIZON_EXPIRED")` (`compact/generated/loan/kernel.compact:16`, emitted at `src/lower-compact.ts:47`); host side `HORIZON_EXPIRED` + `VALIDITY_INTERVAL` at `src/evaluate.ts:207` | This bounds the *intent's validity window against a single `now` observation*, not the *age of a price feed*. No heartbeat, no max-age, no future-timestamp rejection, no per-feed bound. The successor `source/5` path is worse: the only time premise, `observedTime`, is an ordinary action argument checked only from below — `if (observed < eligibleAt) return bad('PERIOD_NOT_ELIGIBLE')` (`src/successor/financial-lifecycle.ts:1767-1771`), same in K (`formal/k/lifecycle-kernel.k:36`, argument typed at `formal/k/lifecycle-binding.k:30`). Nothing caps how far in the future a caller may claim it is. |
| K8 | Value sanity bound (e.g. price > 0) as part of the same guard | **partial** | Guards are general boolean expressions over observations in the atomic profile (`spec/examples/loan.mori:74`), so `guard obs.x > 0` is expressible in principle | Nothing to apply it to: no price-valued observation exists (K2). No profile-level obligation to bound an observed value; the kernel's positivity case is a named acceptance row, Moriarty has no equivalent corpus row. |
| K9 | Observation → decision only through a dimensioned conversion | **partial** (specified + implemented for the *conversion*; the *observation* leg is absent) | Price multiplication is dimension-checked (`src/successor/financial-expression-v1.ts:341`) and the scale division with directed rounding sits at `:404-407`, recorded as profile rows `expression-obligation-division` / `expression-receipt-division` (`numeric-profile.json` primitives) | The price operand can only be a literal or an argument, never an observation. So "an oracle price gates an effect" is currently unreachable in either profile: `source/5` has no observations, and the atomic profile's observations reach only the horizon guard. |
| K10 | Determinism/frame: equal declared reads ⇒ equal result, refusals included | **partial** | The observation set is hashed into the accepted body: `observationsHash: hash('OBSERVATIONS', i.observations)` (`src/evaluate.ts:299`); a TS-vs-K differential over the lifecycle is the strongest existing semantic evidence and is the proposal's S5 task (`deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md:131`) | No congruence/frame statement anywhere: nothing corresponds to `Expr.eval_congr`. The hash binds the inputs to the receipt; it does not state that the result is a function of the declared reads. |
| K11 | Observations may cross domains; state effects may not | **partial (designed)** | `observations[].domain` in the stage relation (`schemas/stage-relation.schema.json:258-260`) and `domain.stateFrameRef` (`:111-113`); design intent "typed observations with issuer, domain, time and finality" (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:42`) | Both are free strings with no rule relating them. `observations[].domain` is `NOT_ENFORCED` (`enforcement-map.json:338`). I found no Moriarty rule stating that effects stay in the invocation domain while observations may name another; the nearest is UNI-011's adapter boundary for *external effects* (`trust-premises.json:187`), which is a different obligation. |
| K12 | Independent, non-bypassable oracle conditions outside the program's own guard | **absent** | — | There is no library/contract layer that re-checks an oracle condition independently of the program's guard. The nearest independent checks are the signed-intent caps I1–I5 (`UNIFIED-PROPOSAL.md:92`), which are about recipients, caps and validity, not observations. The consequence in kernel terms: replacing a Moriarty program's guard with `true` removes its oracle rule. |
| K13 | Explicit trust boundary: type/sign/age checks do not establish truth | **covered (specified)** — the strongest-covered row | TP03 "Issuers, oracles and signers remain explicit trust assumptions for observations, finality and oracle honesty", status **open** (`deliverables/u0-semantic-contract-2026-09-23/trust-premises.json:104-157`, status at `:156`); "External observations and attestations remain named assumptions; they cannot substitute for mandatory program/transition proofs" and "Safety proofs do not supply liquidity, inclusion, oracle honesty, bridge security or unconditional liveness" (`docs/MORIARTY-PRODUCT-CONTRACT.md:47`); "A TEE attestation cannot replace a missing program proof; threshold signatures cannot establish the truth of an oracle" (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:99`); "these schemas bind those external checks; they do not implement cryptography, oracle truth, ledger ordering, or custody" (`spec/typed-schemas.md:478-480`); "ReadObs is a pure read of the supplied snapshot: it emits no Observe effect and makes no oracle truth/currentness assertion" (`spec/successor/static-semantics.md:91-94`); "a valid oracle signature does not establish economic truth" (`wiki/security.md:51`); `wiki/defi-kernel-sdk-interface.md:21` | TP03 carries no `blocks`, `owner` or `closeCondition` — that is a proposed addition only (`UNIFIED-PROPOSAL.md:40` F9, `:202-212` T7). |
| K14 | Exact arithmetic on the observed value | **covered by different means** | Exact domain-qualified integers, field-element coercion forbidden, checked widths (`numeric-profile.json` `units`); exactness policy D2/D4 (`docs/decisions/u0-numeric-profile-decision.md:11-30`) | Not a gap, but a consequence: where the kernel needs no rounding on a `ℚ` price, Moriarty's integer+scale model *must* round at every price conversion, and the rounding direction is author-selectable at the single division site (`numeric-profile.json` rows `expression-obligation-division`/`expression-receipt-division`, both `open-gap`), while the remainder beneficiary is undeliverable: `reserveMechanism.status = "absent"` (`numeric-profile.json:55`). |
| D1 | Orientation reconciliation with the kernel's price convention | **covered (specified), not demonstrated** | "Moriarty's canonical price orientation is **base-per-quote** … DeFiFormal and other quote-per-base sources are adapted only through an explicit, dimensioned conversion with directed rounding at the boundary. A reciprocal, rename or implicit reinterpretation is not a conversion" (`docs/decisions/u0-numeric-profile-decision.md:5-9`); the directed-reciprocal formula `10^(s+t)/M` with floor for a receipt and ceil for an obligation, plus four test vectors, at `numeric-profile.json:301`; restated in `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:57` | The four vectors are recorded expectations, not executed output — the proposal's N3 exists precisely to turn recorded values into observed ones (`UNIFIED-PROPOSAL.md:154`). I did not execute them. The kernel's own statement of its convention is `lean/DefiKernel/Typed/Types.lean:40` ("quote-asset units per one base-asset unit"), which is consistent with Moriarty's "quote-per-base" label for it. |
| K15/K16 | Oracle element taxonomy (`Ex`/`Tp`/`Oa`/`At`) with a mandatory delivery isotope (push/pull/medianizer) | **absent** | The closest is one target row: DA20 "observe price / time / external event … typed observations with provenance, freshness and domain … stale or unauthorized evidence rejects … **simulated observations only**" (`deliverables/defi-language-design-2026-09-07/action-targets.csv:21`) | No push/pull distinction, no TWAP/accumulator element, no optimistic-assertion element, no attestation element with `subject`/`assurance`. `wiki/defiformal-taxonomy.md:90` lists oracles as a covered *facet* of the source taxonomy, not as Moriarty language constructs. |
| K17 | Required-bond laws over observations (L13 freshness, L1 collateral→oracle+liquidation, L27 attestation) | **absent** | — | The six U0 judgments are prose; none states a conservation, cap, non-negativity, roll-forward or freshness law (the study's own finding F5, `UNIFIED-PROPOSAL.md:36`). The `effect` judgment nominally lists all five `observations[].*` leaves among its `schemaFields` (`deliverables/u0-semantic-contract-2026-09-23/judgments.json:50-52`, `:76-80`) but its definition is one sentence asserting that the observations "are the authorized state transition" (`:52`). That is a name in a schema, not a law. |
| K18 | Trust bond as a separately satisfied obligation with a named attester per counterpart | **partial (designed)** | TP03 names issuers/oracles/signers as a category (`trust-premises.json:106`); the condition/evidence line in the design table names "typed condition/evidence policies, freshness, disclosure and conjunction/threshold rules" with "Issuers/oracles/signers remain explicit trust assumptions" (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:30`) | One global premise, not a per-observation attester obligation. No independence-from-obligor, scope/assurance, periodicity or recourse requirement (kernel L27). |
| K19 | Oracle latency/confidence as a recorded reaction condition per program | **absent** | — | No construct or document field records "is the value current relative to volatility and closeout time" for a Moriarty program. Unassigned. |
| K20 | Manipulation hazards and reflexivity (X2/X3/X10) | **out-of-scope-by-design, and both projects agree** | "a valid oracle signature does not establish economic truth" (`wiki/security.md:51`); "a price bound or time-weighted observation can reduce a specified attacker's feasible strategies, but cannot automatically remove economic risk … the four mandatory claims remain mandatory and must name those assumptions" (`deliverables/defi-taxonomy-papers-2026-09-08/DESIGN-IMPLICATIONS.md:62`); the kernel's matching disclaimer is `README.md:170-171` | Nothing to add for the language. The Atlas keeps these as hazard rows outside the kernel too (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:790-798`). |
| — | Non-price observations (proof-of-reserve, rates, events) | **partial** | The observation namespace is name→arbitrary declared type (`spec/successor/representation.md:112`), so a reserve or event observation is representable in principle; the design lists "collect documents, signatures, attestations and observations" (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:30`) | No attestation structure (subject/assurance/attester/recourse), no proof-of-reserve construct, no event observation in any example. The atomic profile's only observation in any checked-in example is `now` (`spec/examples/loan.mori:24`). |
| — | Oracle manipulation response inside the language: circuit breakers, fallback, multi-source | **absent (designed once, never specified)** | "Oracle manipulation | Stale, equivocated, replayed, or unit-confused statement | **Signed typed observation: source, feed, unit, timestamp, freshness, sequence, bounds, fallback** | Source can still lie; governance and dispute paths remain external" (`wiki/security.md:32-33`); "Oracle staleness/equivocation/replay | Typed signed statement, feed/source/unit/time/freshness/sequence/bounds, multi-source or fallback policy" (`deliverables/moriarty-decision-study-2026-09-02.md:712`) | These two rows are the most complete oracle design statement in the repository, and **none of their eight fields** (source, feed, unit, timestamp, freshness, sequence, bounds, fallback) exists in the U0 stage relation, whose observation item is `{kind, issuer, domain, time, finality}`, all free strings with no value (`schemas/stage-relation.schema.json:239-268`). No multi-source, aggregation, medianizer or breaker construct exists anywhere. My repository-wide grep for `medianizer|twap|time-weighted|heartbeat|circuit breaker|multi-source|proof-of-reserve` over `docs/ wiki/ deliverables/ openspec/ experiments/moriarty-language/spec/` returned only research captures and third-party paper text — no Moriarty design or spec hit. |
| — | An observation is actually a premise of a financial decision, end to end | **partial (one demonstrated instance, for time only, unauthenticated)** | `loan.mori:24` declares `observation now: UInt128`; `:74`/`:90` guard on it; `src/lower-compact.ts:47` emits the assert; `compact/generated/loan/kernel.compact:13` takes `observations: KernelObservations` as a circuit parameter and `:16-20` asserts the horizon | The enforcement map itself disowns the link: "Generated kernels assert `observations.o0 < 2000000000` at loan kernel.compact:16. That **anonymous** observation is not this stage-relation field" (`enforcement-map.json:362-365`). `o0` has no issuer, no timestamp, no unit, and is an unconstrained circuit input. And in the `source/5` path that S0 is built on, the analogous premise `observedTime` is a plain caller argument (`financial-lifecycle.ts:1767-1771`). |
| — | U0's own disposition of observations | **out-of-scope-by-design for S0, but the scoping is under-justified** | "Empty supply changes, observations and disclosures, each bound as an explicit empty set that rejects any non-empty value" (`UNIFIED-PROPOSAL.md:74`); "Excluded: accrual, ProRata, price or division, reserve posting, partial phases, non-ledger-induced history" (`:75`); TP03 "blocks U3, or U2 only if a program claims an observation" (`:206`) | Binding `observations[]` to `{}` with rejection is a defensible first slice. But no document I found records that the atomic profile *already had* observation bindings, providers, evidence digests and an authenticity gate (`src/evaluate.ts:84`, `:128`, `:202-203`) while `source/5` dropped them (`financial-agreement-source-compiler.ts:606`). None of the nine study briefs mentions oracles or observations (grep over `deliverables/u0-study-2026-09-28/briefs/*.md` returned nothing), so no reviewer was pointed at this. |

**Counts.** 24 rows: **covered 4** (K1, K13, K14-by-different-means, D1-as-specified), **partial 12**
(K2, K4, K5, K6, K7, K8, K9, K10, K11, K18, non-price observations, end-to-end premise),
**absent 6** (K3, K12, K15/K16, K17, K19, manipulation response/fallback),
**out-of-scope-by-design 2** (K20, U0's S0 exclusion).

Nothing in this category is *demonstrated by executed evidence* except the one horizon assert lowered into
`compact/generated/loan/kernel.compact:16`, and I did not re-execute that lowering — I read the generated
file, the emitting line (`src/lower-compact.ts:47`) and the source guard (`loan.mori:74`).

---

## 4. Category verdict

**Verdict: partial, and structurally weaker than the row counts suggest.** Moriarty's *stated* trust
boundary for oracles is at least as sharp as the kernel's and in places sharper — TP03, the product
contract's "external observations and attestations remain named assumptions", and
`static-semantics.md:93-94`'s refusal to let `ReadObs` assert currentness are exactly the right commitments
(`trust-premises.json:104-157`; `docs/MORIARTY-PRODUCT-CONTRACT.md:47`;
`spec/successor/static-semantics.md:91-94`). The *machinery on the safe side of that boundary* is where the
gap lies. The kernel makes an observation a declared, dimensioned, timestamped, keyed premise whose
footprint is checked, whose absence or wrong unit is a distinct refusal, whose age is a guard, and whose
only route into an effect is a dimensioned conversion — and then proves that the result is a function of
those declared reads (`Typed/Transition.lean:84-86`, `:124-125`, `:158`; `Typed/Expr.lean:117-124`;
`Typed/Examples.lean:132-136`; `Typed/Expr.lean:177-186`). Moriarty has the namespace (K1) and the
dimensioned price type (K2 partial), and the atomic profile has real provider binding (K6 partial), but no
per-observation timestamp, no per-operation read footprint, no independent non-bypassable oracle condition,
no freshness law, and no way for a price to *be* an observation at all. The successor `source/5` profile —
the one U0's S0 slice sits on — dropped the observation machinery the atomic profile had
(`financial-agreement-source-compiler.ts:606` vs `src/evaluate.ts:84`, `:128`, `:202-203`), and the
category's only executed artifact is an anonymous `Uint<128>` circuit input that the enforcement map itself
declines to identify with the stage-relation field (`enforcement-map.json:362-365`).

**Milestone ownership on the evidence found.**

- **U0** owns the schema and law shape: giving `observations[]` a *value* and a dimension, and writing at
  least one freshness clause as an executable judgment. The proposal's S2/S3 tasks are the right vehicle
  (`UNIFIED-PROPOSAL.md:110-119`) but neither mentions observations, and S0 binds them empty (`:74`).
  This is an ownership claim by structure, not by an assignment I found.
- **U2** owns the boundary case named in the proposal: "TP03 blocks … U2 only if a program claims an
  observation" (`UNIFIED-PROPOSAL.md:206`). Until observations are non-empty, U2 is unaffected.
- **U3** is the assigned owner of the premise: "TP03 blocks U3" (`UNIFIED-PROPOSAL.md:206`), consistent
  with U3's conditional-settlement and document-condition scope (`ROADMAP.md:24`, `:38`) and with
  MPLR-010 "Time finality and unresolved outcomes | U3" (`trust-premises.json:142`).
- **U6** owns the taxonomy and library side — `Ex`/`Tp`/`Oa`/`At`, delivery isotopes, and the L1/L13/L27
  bond laws — as part of "All retained ACTUS fixtures/fields, DeFi action rows and held-out behaviors"
  (`ROADMAP.md:27`).
- **Unassigned**: per-observation timestamp and staleness bound (K3, K7); per-operation read footprint
  (K4); the independent non-bypassable oracle condition (K12); reaction conditions (K19); multi-source,
  fallback and circuit breakers (`wiki/security.md:33` names them and no milestone claims them).
- Also **unassigned**: reconciling the two profiles. No document I found records the `source/5` observation
  regression or names an owner for restoring it.

---

## 5. Gaps that would change the language design

Ranked by how much new language/kernel/judgment/numeric/enforcement machinery they require, rather than a
library or an example.

**G1 — An observation carries no value, so it cannot be a premise. (New schema + new judgment machinery.)**
The stage-relation observation item is `{kind, issuer, domain, time, finality}`, five free strings with no
value, no asset, no unit and no scale (`schemas/stage-relation.schema.json:239-268`). The `effect`
judgment lists all five leaves and then asserts in prose that they "are the authorized state transition"
(`judgments.json:50-52`, `:76-80`). The kernel's minimum is `{key=(domain,id), value : PackedValue,
timestamp}` with the expected unit intrinsic to the reference and checked at lookup
(`Typed/Types.lean:114-128`; `Typed/Expr.lean:117-124`). Fixing this is not a library change: it changes
the frozen relation, adds a dimension to a leaf the numeric profile must then govern, and turns the
`effect` judgment into a clause that reads a value. It should be S2/S3 scope, and S2/S3 currently do not
mention it (`UNIFIED-PROPOSAL.md:110-119`).

**G2 — No per-observation time, so freshness is inexpressible. (New Core type + new judgment clause.)**
`ObservationValue` has no observed-at field (`spec/typed-schemas.md:462-464`; `src/runtime-types.ts:13`);
`now` is a mandatory observation compared only to a global horizon and the intent validity window
(`src/validate.ts:94`; `src/evaluate.ts:207`). The kernel's `Expr.timestamp key` and `Expr.now` are two
different reads (`Typed/Expr.lean:84-86`, `:154-162`) precisely so that
`timestamp ≤ now ≤ timestamp + H` is a guard (`Typed/Examples.lean:133-134`). Without a per-observation
time, Moriarty cannot state L13 "`Ex → freshness validation`"
(`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:753`) about anything, and `wiki/security.md:33`'s designed
"timestamp, freshness, sequence" fields have nowhere to live.

**G3 — Nothing binds an oracle condition independently of the program's own guard. (New kernel layer.)**
The kernel's contract layer restates the oracle conditions so that "replacing a proposal's own guard with
`true` cannot remove those rules" (`lean/README.md:41-47`;
`lean/DefiKernel/ContractExamples.lean:34-50`, `:103-121`). Moriarty's independent checks are the signed
intent's caps and recipients (`UNIFIED-PROPOSAL.md:92`), which say nothing about observations. This
matters more in Moriarty than in the kernel, because Moriarty's whole acceptance story is that a prover
cannot choose its own predicate (`docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:29`) — yet today an
oracle premise would live only in the program's own guard.

**G4 — The observation read footprint is whole-program, not per-operation, and has no refusal.**
Moriarty declares observations once per program and binds providers positionally
(`src/evaluate.ts:192-193`, `:203`). The kernel computes `requiredEnvReads` from the guard and every
delta and refuses `.envReadFootprint` when the declaration does not cover them
(`Typed/Transition.lean:84-86`, `:124-125`, `:158`), which is what makes a *mutation* of the declaration
detectable (`Typed/Acceptance.lean:88`, `:238`). Adding this is a change to the Core transition shape and
to the diagnostics set, not an example.

**G5 — Provider authenticity is a host boolean and `evidenceDigest` verifies nothing. (Enforcement.)**
`checks.observationsAuthentic` and `checks.genesisValid` are caller-supplied booleans that the evaluator
only requires to equal `true` (`src/evaluate.ts:128`, `:202`); `evidenceDigest` is "an opaque external
digest. The profile defines no preimage or algorithm for it" (`spec/typed-schemas.md:469-474`). FOOTGUNS
rule 4 forbids exactly this shape — "reject an unconditional throw, constant admission response or unused
adapter where execution is required" — and `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:29` says
"a host Boolean, an unused assertion or a verifier chosen by the prover cannot replace that enforcement".
The proposal's T6 settles the analogous question for *signed intent* by making the digest an in-circuit
public input (`UNIFIED-PROPOSAL.md:197-201`); the same decision is owed for observations and no document
makes it. `observations[].issuer` is `NOT_ENFORCED` today (`enforcement-map.json:350`).

**G6 — The price/rounding seam is where orientation, scale and rounding meet, and the reserve is absent.**
D1 fixes base-per-quote and requires an explicit directed-reciprocal conversion from the kernel's
quote-per-base, refusing a rename or reciprocal (`docs/decisions/u0-numeric-profile-decision.md:5-9`;
`numeric-profile.json:301` with four vectors). But the single division site is author-selectable between
floor and ceil (`financial-expression-v1.ts:404-407`; profile rows `expression-obligation-division` and
`expression-receipt-division`, both `open-gap`), and the remainder beneficiary cannot be honoured:
`reserveMechanism.status = "absent"` (`numeric-profile.json:55`). An oracle price is the case where a
one-unit rounding error is chosen by the program author against the protocol. The kernel avoids this by
using `ℚ` (`Typed/Expr.lean:3`) and lists rounding as a separate obligation (`README.md:168-171`); Moriarty
cannot, so the direction must be derived from the result role, not selected. N1/N2 own this
(`UNIFIED-PROPOSAL.md:136-153`), and the four defiformal vectors should be executed under N3 (`:154`).

**G7 — No taxonomy or bond law for oracles, so a program cannot be required to have one.**
The Atlas makes `Ex` carry a mandatory delivery isotope whose preserved failure difference is "staleness
mode" (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:452`), and L1 makes collateralized lending *require*
`(Ex|Tp|At) + Ct + liquidation` with a named finality assumption when cross-domain (`:741`). Moriarty has
one target row that names the intent and marks it "simulated observations only"
(`deliverables/defi-language-design-2026-09-07/action-targets.csv:21`). Turning "this program reads a
price" into "this program must therefore declare a freshness bound and a liquidation path" is judgment
machinery, and it is the thing that would make U6's DeFi conformance mean something for this category.

**G8 — The `source/5` observation regression is unrecorded.** The atomic profile has observation
declarations, genesis provider bindings, evidence digests and an authenticity gate
(`src/evaluate.ts:84`, `:128`, `:202-203`; `spec/typed-schemas.md:439-480`). `source/5` lists `observation`
among unsupported declarations "from the atomic profile"
(`spec/successor/README.md:33-41`; `syntax-profile.json:60`) and hard-codes `observations: {}`
(`financial-agreement-source-compiler.ts:606`). U0's S0 then binds `observations[]` to an empty set with
rejection (`UNIFIED-PROPOSAL.md:74`). Each step is individually reasonable; together they freeze a contract
in which the capability the repository already built has no owner and no restoration condition. This needs
a recorded decision, not code.

---

## 6. Limits of this review

- **Nothing was executed.** No Lean build, no K run, no TypeScript evaluator run, no U0 checker, no Compact
  compilation, no proof, no network call. Every behavioural statement is read from source. Where I say a
  check "refuses", I mean the code path returns that refusal as written, not that I observed it.
- **defiformal Lean coverage is partial.** I read `Typed/{Types,Expr,Transition,Examples}.lean` closely and
  `Typed/Acceptance.lean`, `ContractExamples.lean`, `Examples.lean` by grep with context. I did not read
  `Composition/`, `Parallel/`, `Interleaving/`, `Atomic/`, `Nary/`, `Metatheory/` or `Vault/`, so if an
  observation abstraction exists in the composition or atomic layers that I did not reach, I have missed
  it. I saw no grep hit for `oracle`/`observ` in those directories beyond test-oracle usages, but "test
  oracle" and "external observation" share a word and I did not disambiguate every hit.
- **"openspec/specs `observation`" is mostly a different sense.** Most `observation` hits in
  `defiformal/openspec/specs/` mean "the observed output of a run" or "a test oracle", not external data.
  I relied on `sequential-workflow-execution/spec.md:9-11` for the one hit that is about external inputs.
- **The Atlas is a design document, not the kernel.** I treated `docs/UNIFIED-DEFI-ELEMENT-TABLE.md` as
  stating requirements the kernel *says* the category needs. The document itself flags several of these as
  unsettled (e.g. the informational bond's settling test "is owed by this document and has not been run",
  `:563`), and the hazard base rates are "unknown" throughout (`:789-800`). I did not treat any hazard
  row as established fact.
- **Assertions by inference, labelled.** (a) That `authenticationPolicy` is never interpreted — inferred
  from a repository grep returning only the schema, type and spec declarations
  (`src/evaluate.ts:84`, `src/runtime-types.ts:6`, `spec/typed-schemas.md:439`, `:470`) and no consumer; I
  did not trace every caller. (b) That no Moriarty rule permits cross-domain observations while confining
  effects — inferred from absence in the documents I read. (c) That the `source/5` profile is the one S0
  sits on — inferred from `UNIFIED-PROPOSAL.md:120` naming `src/successor/stage-relation.ts` calling
  `prepareFinancialLifecycle`, and from the numeric profile's "Source/5 uses this lifecycle kernel" notes.
  (d) That no study brief mentioned oracles — from a grep over
  `deliverables/u0-study-2026-09-28/briefs/*.md` returning no hit; I did not read the nine reviewer
  proposals in that directory in full.
- **Counts are mine, not a checker's.** The 4/12/6/2 split in §3 is my classification of my own 24 rows.
  Another reviewer partitioning the category differently would get different denominators.
- **The guarded CLI reported unresolved operational history** (§1). That blocks `sp01` implementation
  dispatch, not this read-only review, and I did not attempt to clear it.

---

## Architecture axis (follow-up)

Read in full for this section: `ROADMAP.md` (56 lines) and `docs/MORIARTY-CONSOLIDATED-DESIGN.md` (123 lines).
Nothing executed. Question: does an *architecture* exist, not an implementation.

**1. Placement.** The architecture places the category in three pieces, and the split is the design. The
Responsibility boundary table's Conditions row (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:30`) assigns to
Moriarty "Typed condition/evidence policies, freshness, disclosure and conjunction/threshold rules"; to the
Federated DeFi Kernel "Collect documents, signatures, attestations and observations"; to the external
domain "Issuers/oracles/signers remain explicit trust assumptions". The core construct list names
"authenticated evidence" (`:15`); the Kernel "obtains evidence … observes finality" (`:11`); settlement
adapters "establish concrete effects and their domain limits" (`:38`). So: *policy* is core, *collection*
is a federated service, *truth* is external. That is a real and defensible architecture — this category is
better designed than specified.

**2. Core mechanisms.** Typed **value** — **not named**: the Canonical stage statement binds "typed
observations with issuer, domain, time and finality" (`:42`) and **no observed value, unit, asset or
scale**, even though `:52` makes prices, units and rounding explicit for arithmetic. **Observed-at time** —
not distinguished from stage time anywhere. **Issuer** — named (`:42`, `:30`). **Freshness** — named as a
Moriarty-column responsibility (`:30`); no component. **Aggregation** — absent; the word appears only for
solver plan aggregation (`:29`). **Conjunction/threshold rules** — named (`:30`); "threshold" elsewhere
means custody/signing (`:11`, `:33`, `:97`), a different mechanism. No component is designated for any of
the six; no U row in `ROADMAP.md:21-28` mentions observations, oracles or evidence policy.

**3. Architectural holes.**
- **H1 (sharpest).** `:46` requires each bound field to identify "the actual circuit constraint,
  authenticated state read, signature commitment or ledger check. A host JSON field or host-computed
  verification Boolean is insufficient." `:44` supplies that locus sentence **for signed intent only**.
  Observations are bound at `:42` with no locus named anywhere.
- **H2.** "Freshness" is assigned at `:30`, but `:42` binds no observed-at time to compute it against — a
  property asserted with no operand.
- **H3.** "Conjunction/threshold rules" (`:30`) have no operator, core construct or library family; `:93`'s
  library list has "asynchronous and conditional claims" but no evidence/conditions family.
- **H4.** "Moriarty can also run on Midnight without this federation" (`:11`) while *collection* of
  observations is a Kernel-column duty (`:30`); `ROADMAP.md:38` says "U3 needs no federation" yet U3 is
  where document conditions live (`ROADMAP.md:24`). Nothing supplies observations federation-free.
- **H5.** "Signed fallback policies must not become an unsigned evidence downgrade" (`:97`) constrains a
  fallback construct that is never defined.
- **H6.** "threshold signatures cannot establish the truth of an oracle" (`:99`) — correct, and the document
  never says what *does* bind an oracle statement.

**4. Canonical stage statement.** `:42` omits, for this category: the observed **value** and its
unit/asset/scale; an **observed-at** time distinct from stage time; a **feed/series identity** distinct
from issuer; any **freshness or heartbeat bound**; a **sequence** number (present in the older threat table
`wiki/security.md:33`, absent here); and the **declared read set** — nothing binds *which* observations the
program was permitted to read. It also omits the `:44` enforcement-locus sentence.

**5. Ownership of the architecture work: unassigned.** U0 freezes price orientation and units but its
evidence list never mentions observations (`ROADMAP.md:21`). U3 is the only plausible host —
"recipient/document conditions" (`:24`), "an authenticated document predicate guard delivery" (`:38`) — and
is scoped to documents, not feeds. U5 owns the collection side via the federated kernel (`:26`). U6 owns
library families, none of which is an evidence family (`:27`, design `:93`). TP03 is correctly a premise,
and the design restates it (`:30`, `:99`) without assigning a designer.

**6. Minimum addition** (no core redesign later): (a) add the observed **value with its declared unit** to
`:42`, which forces the `:52` numeric profile to govern it; (b) add an **observed-at time distinct from
stage time**, giving `:30`'s freshness an operand; (c) extend `:44`'s enforcement-locus sentence to
observations, so `:46` rules out host booleans explicitly rather than by analogy; (d) name one component
per Conditions cell, including what supplies observations on the non-federated path (`:11`); (e) add an
evidence/conditions family to `:93`'s library list so aggregation, multi-source and fallback have a home
outside the core; (f) assign (a)–(e) to a milestone.

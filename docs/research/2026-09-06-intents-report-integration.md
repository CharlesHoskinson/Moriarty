# Intents report integration

Date: 2026-09-06. Status: S2 design amendment, with restricted R2 and R2b S3 local prototypes.
The user supplied `intents.md` during the authorized shared-language sprint and
asked to graph it and integrate its ideas. This amendment extends the
[PCD sequence](2026-09-06-pcd-report-integration.md); it does not restart A4/A5.

## Evidence and interpretation

SRC-0047 is the byte-preserved [report](../../raw/reports/intents-2026-09-06/intents.md),
82,836 bytes, SHA-256
`680f646e244fdd59e72de3718a3a16566f643f599ae30c899f7b82fed6deef87`.
It is secondary design/research evidence with opaque citations. Its embedded
prototype download is not an attached artifact: the claimed checker, Lean file,
tests and ZIP digest at lines 1383–1461 have not been reproduced here.
The [graph package](../../deliverables/intents-report-integration-2026-09-06/README.md)
maps what the report says; extraction does not certify its claims.

The consequential DeFi Kernel claim was checked at the existing SRC-0042 pin,
`8ae0bbfaa3193078d1cabf6999db1382985b7f95`, despite the live checkout advancing.
SRC-0048 preserves [the inspected sources](../../raw/intents-source-check-2026-09-06/receipt.json).
`QSIGMA-VERDICT.md:1–8` explicitly withdraws the four-primitive invariant;
`gate33_cert_check.py` classifies declaration names by keywords and labels its
result a seed. This is a source observation, not a new proof or corpus rerun.
Keep the financial corpus and counterexamples. Neither taxonomy membership nor
keyword tags establish behavioral completeness or a contract certificate.

## Decisions that change the plan

| Report idea and locator | Moriarty disposition | Concrete work |
| --- | --- | --- |
| Intent, permission, plan, execution and receipt are separate, 241–264 and 995–1190 | Adopt | Define separate bounded artifacts and their binding relation. R2 signs a concrete plan; R2b introduces outcome authorization independent of route choice. |
| Quote/plan then sign is a legitimate restricted profile, 1256–1276 | Adopt as R2 scope | Keep the working exact-plan path and call it that. It does not yet support solver choice after signing. |
| Authority and prefix safety precede terminal goals, 595–691 | Adopt | R2 checks gross transfer/fee caps and every named recipient, including refunded movements. R2b adds aggregate account/asset budgets and ordered safety phases. |
| Hard requirements differ from preferences, 435–474 and 1092–1118 | Adopt | Solvers search outside the finite checker. Ranking cannot excuse invalid authority or a failed goal. No global best-price claim. |
| Affine capabilities and residual obligations, 475–553 and 1507–1569 | Adopt for R2b/R4 | Partial completion consumes authority; residual authority cannot grow or reset an epoch. Pending progress has its own judgment and never asserts the terminal goal is already achieved. |
| Compiler refinement plus anti-vacuity, 692–731 | Adopt | Require trace inclusion and a feasible positive witness. Always rejecting does not constitute a working compiler. |
| Domain-qualified assets, assumptions, typed claims, 265–328 and 732–808 | Adopt incrementally | R2 uses explicit local asset IDs and numeric units. R2b defines structured domain/issuer/reference/claim kind and distinguishes receipt tokens from delivered assets. |
| Wallet rendering from canonical signed meaning, 1190–1237 and 1876–1921 | Adopt | R2 exposes the exact local plan and editable policy. R2b adds a deterministic semantic signing summary, expiry, nonce, liability and residual-authority display. |
| Distinct proof classes, 1956–2017 | Adopt with mandatory PCD | Local checks support the relation; contract, transition and predecessor-history obligations remain required for real acceptance. Plan proofs alone are not recursive history compliance. |
| DeFi taxonomy as library coverage, 2095–2153 | Adopt, source-checked | ACTUS and DeFi remain implementation targets. Choose held-out financial cases before extending Core; do not promote the withdrawn four-primitive basis. |
| IKL naming, optional-only ZK, NEAR/EVM-first milestones, unbounded Nat/Int, foreign-call escape | Do not replace user requirements | Keep Moriarty, finite types and bounded lifecycles, ACTUS/DeFi coverage, mandatory PCD, and Midnight Compact/ZKIR with native recursion as the first backend. Uncertified callbacks are outside the certified language. |

Current standards and third-party backend capabilities are not newly verified by
this ingestion. Existing pinned standards research remains the source base.
Any later ERC/NEAR adapter must re-check its exact version and interface rather
than importing the report's moving claims as established compatibility.

## Unified judgments

The following is a proposed semantic contract, not a mechanized theorem:

```text
Agreement + action + observations -> bounded execution trace + effects
IntentIR = principal + scope + authority + requirements + assumptions
         + lifecycle + validity/replay + semantic versions
PlanIR = intent identity + bounded steps + adapter identities + dependencies
Receipt = intent/plan identities + checked effects + status + residual resources

Complete(I, P, trace, evidence) requires:
  authenticated and live I
  typed plan and agreement transition
  gross authority limits and each permitted recipient/call
  safety at every declared observable phase
  lifecycle progress and terminal goal
  complete effect/accounting projection
  required contract, refinement, transition and history evidence

Progress(I, P, trace, evidence) requires:
  the same authority/safety/evidence duties for the performed prefix
  an allowed pending state, residual authority and outstanding obligations
  no assertion of terminal completion
```

PCD's local compliance relation must connect the plan to the signed intent and
carry authority consumption and obligations through predecessor transitions.
An otherwise valid financial transition cannot amplify authority. Ledger
consumption, cancellation/fill races, finality, external observation truth and
private witness availability remain separate obligations. Hidden host calls
cannot be excluded from the footprint merely by omitting them from a receipt.

R2's gross caps apply cumulatively to each signed asset/from/to edge. Incoming
refunds do not restore that edge's cap. Minimum credits are the recipient's net
asset change after all declared transfers and fees. An aggregate budget across
multiple recipients is not implemented by those edge caps; R2b must make that
separate restriction explicit. R2 has no durable nonce registry, expiry check,
solver network, compositional authority theorem or wallet authorization.

## Revised gates and next developer deliverable

| Gate | Exit evidence |
| --- | --- |
| R2: exact-plan executable slice | Loan accrual/settlement and pool swap/closure share one bounded evaluator. Independent effect checks, genuine local signing and fail-closed required-proof status run in the browser. This remains S3. |
| R2b: outcome intent and authority profile | Reviewable bounded IntentIR/PlanIR/Receipt schemas plus the concrete checker and signing summary described below. Preserve positive feasibility and adversarial rejection. |
| R3: native proof boundary | The fixed financial harness compiled and passed application checks, but recursive VK setup exhausted rows at k17; no native proof exists. Local Compact deployment/call settled separately. Public Preprod, native proof-to-ledger compatibility and R2b authority/refinement binding remain open. Review state encoding or resource limits before further native execution. |
| R4: private composition | Independent witness handoff, residual-authority chains and split/join obligations; conflicts and finality checked separately from PCD. |
| R5: certificates and target coverage | Connect named bounded contract certificates, compiler/adapter refinement and held-out workloads; retain all 18 ACTUS types, 277 fixtures and 72 DeFi rows. |
| R6: acceleration | Only measure a concrete repeated workload after the acceptance relation exists. Fallback preserves all mandatory claims. |

The next developer deliverable is an **outcome-intent editor beside the existing
exact-plan mode**. Start with a finite atomic profile: explicit principal and
domain-qualified assets; gross aggregate debit budgets; recipient allowlists;
net minimum credits; fee caps; validity interval and single-use nonce; and a
bounded concrete plan with no arbitrary callbacks. The signing panel renders
these fields from the decoded canonical object. A proposal may change its route
only within that signed envelope. Unsupported semantic extensions refuse signing.

Required controls: a positive swap; an over-debit that still meets the goal; a
refund that hides excess gross spend; an unauthorized intermediate recipient;
net output reduced below minimum by a fee; wrong-domain same-ticker assets;
expiry and nonce replay; and two competing plans attempting one authority token.
The local consumption model must label its durability boundary. Do not call it
ledger safety or turn the always-unavailable proof stub into accepted evidence.

Before broadening Core, freeze these held-out cases from the existing targets:
an ACTUS NAM negative-amortization event, a DeFi lending refinance with a
liability change, and a request/claim redemption with a pending obligation.
The first two target families stay represented; asynchronous redemption is a
later explicit lifecycle profile. Record representable/rejected/needs-extension
with source locators and independent result fields. Their selection is a design
decision, not an implementation or conformance result.

## R2b implementation checkpoint

The [atomic profile](../superpowers/specs/2026-09-06-r2b-outcome-intents-design.md)
is implemented at `/intents`; [evidence](../../evidence/moriarty-r2b-outcomes-2026-09-06/README.md)
records its actual checks. A signed outcome permits either of two registered pool
plans. The same independent authority checker also checks transfer-backed loan
settlement. Gross asset debit across recipients, net goals including fees, exact
asset identities, expiry and principal/domain/nonce consumption are executable.
The signer renders all canonical safety fields before signing. This is a local
key trust record, not wallet or on-chain authority.

The first implementation deliberately permits one registered financial action
and at most four explicit fees. It has no partial fill, pending success or reusable
residual capability. Remaining one-shot allowance is extinguished on commit.
Example changes and reload reset synthetic balances and nonce history; the clock
is an explicit monotone demo input. Concurrency checks protect this local world,
not distributed uniqueness. Mandatory real claims remain unavailable.

[Held-out source records](../../evidence/moriarty-r2b-heldouts-2026-09-06/README.md)
freeze ACTUS NAM19 capitalization, Maple refinance and Huma pending redemption.
All three need extensions; no new Core semantics or conformance result is claimed.
The [R3 experiment](../../evidence/moriarty-native-ivc-r3-2026-09-07/README.md)
now records a concrete k17 recursive-row failure after passing fixed application
checks. No cryptographic proof was produced. Choose a reviewed smaller checked
state encoding or a separately justified resource ceiling before another run.
The [network experiment](../../evidence/moriarty-midnight-network-2026-09-07/README.md)
settled a local Compact deployment/call; public Preprod still requires faucet
CAPTCHA funding and wallet synchronization. These transactions do not consume a
Moriarty history proof. R4 owns private/residual composition, and R5 retains
certificates and full target coverage. No gate is waived by local SDK success.


## Public test-network selection amendment — 2026-09-07

The [official-docs review](../../evidence/midnight-network-review-2026-09-07/README.md)
places local Docker first for routine development, Preview first for new early
public integration experiments, and Preprod at final validation. Preprod remains
a supported direct promotion target if its existing wallet is funded. A faucet
error and a bounded wallet-sync timeout do not establish network failure. Before
another public attempt, observe funding independently of full sync, retain wallet
state, and use the matching documented faucet with its exact server response.
The actual public settlement test remains open. This amendment does not change
the native R3 k17 stop or remove mandatory proofs and target-coverage obligations.

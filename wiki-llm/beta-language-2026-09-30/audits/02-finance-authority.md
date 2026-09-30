# Independent beta audit 02: finance, authority and lifecycle

Candidate: v1, SHA-256 `6af4ca5356ed3fd58818b5717401d5b218b672b65769e342f708d4a4e0cd061a`, recomputed from `BETA-DESIGN.md` and matching `candidate-v1.sha256`. Baseline supplied: `870f998b36ecda04622fa4274132e74902942d0b`. Requested reviewer: GPT-6.1 Sol, medium effort. The parent requested that identity; this agent has no independently observable provider-returned model/effort receipt, so requested identity is recorded without asserting a verified returned identity. No peer audit was read.

Review scope: frozen candidate; domain 02 types, 03 agreements, 04 workflows and 09 ecosystems memos; programmer-facing mockup requirements; Source/6 specification, implementation contract, actual Source/6 frontend/wrapper and Core/5 preparer; existing S0 tests; U0 price orientation. This is a design audit, not full-source authentication, formal correspondence, native proof or ledger review. Startup read `AGENTS.md` and the checked-in development skill; guarded status reported stale campaign bindings and unavailable current accounting, with no pending transactions. Those conditions do not block this authorized read-only design review.

**Verdict: agree with the bounded financial architecture, with the following amendments before claiming the candidate meets the full mockup requirements or freezing implementation behavior.** Findings below distinguish observed specification gaps from possible implementation failures; no beta implementation was available to reproduce the latter.

## Substantive decisions

| Choice | Vote and financial reason |
| --- | --- |
| Separate standalone beta profile; only transfer and funded repayment execute | Agree. This preserves the demonstrated S0 boundary while permitting useful authoring. Each wider operation must reject lowering explicitly. |
| Nominal domain/account/asset identity, representation and scale; no symbol coercion | Agree. A ticker or equal address bytes cannot identify a transferable asset or spending authority. Conflicting same-identity declarations must reject even when scales differ. |
| Owner-fixed operation and bounds; separate untrusted scenario | Agree. Scenario claims cannot replace transfer endpoints or add solver discretion. For repayment, stipulated creditor comes from the obligation, exactly as S0; inspection must identify that provenance. |
| Strict Source/6 then existing Core/5 rederivation | Agree with amendment F02. This avoids a second acceptance engine; the expansion path must not decide economic failure before Core. |
| AccrualFirst funded repayment, complete creditor credit, persistent residual obligation | Agree. Liability reduction alone is not payment. Partial stage success does not settle the whole debt. |
| Gross allowance consumption, work counters, replay and head in complete effect order | Agree. Refund/netting cannot restore gross authority or erase fee accounting. |
| Local atomic rejection and explicit future signed retained-effect relation | Agree. Local no-effect rejection supplies no evidence about Midnight fallible-phase fees/effects. |
| Retain U0 Base-per-Quote orientation | Agree. `semantic-contract.md` and `expression-signatures.json` explicitly use Base per Quote. The domain02 opposite proposal must remain superseded; no implicit reciprocal. |
| Full eight-family horizon and episodes | Agree as scope; disagree that the present table alone satisfies the required complete programmer mockup. See F03. |

## Findings and exact discriminators

### F01 — Medium: nominal numeric range is omitted from the adopted candidate

**Repository observation:** candidate lines 103–115 specify UInt128 intermediates and out-of-range atom rejection, but give no numeric maximum for `Qty`, operation amounts, caps or floors. The Source/6 contract and domain02 BT2-O01 distinguish nominal `0..2^127−1` from balance/counter/intermediate `0..2^128−1`; principal/accrued/outstanding additionally use the narrower bound. Core enforces those separate limits.

**Discriminator:** declare scale-zero USD, `const x = atoms(asset: USD, value: 170141183460469231731687303715884105728)` (`2^127`), then use x as a transfer value or gross cap. It fits UInt128 but cannot form a Source/6 nominal amount. A checker using only UInt128 can misleadingly mark an executable action valid before expansion rejects. Conversely, a receiver balance of `2^128−1` is syntactically valid and must not be narrowed to the nominal range.

**Repair recommendation:** explicitly freeze separate `UInt128` scalar/intermediate/balance/counter bounds and the S0 nominal quantity/liability bounds. State whether every `Qty` value uses the narrower range, or whether S0 field assignment performs that narrowing; emit a source diagnostic at the appropriate assignment. Test boundary `2^127−1`/`2^127`, plus a legal UInt128 receiver and credit overflow. Do not claim W-D4 is closed.

### F02 — High: automatic expansion needs an economic-invalid-request path preserving first Core failure

**Inference from candidate lines 178–184:** default expansion derives a repayment effect vector from source request plus scenario. The straightforward formula can produce negative principal/outstanding for an overpayment. Source/6 requires unsigned effect fields, so emitting those derived negatives yields formation failure before Core's prescribed Effect rejection. Similar premature preparation checks can hide Intent, Effect or Authority ordering.

**Exact discriminator:** valid source intent repays 31 atoms with gross cap 31, fee cap/net floor zero, against a well-formed Outstanding obligation `(principal=20, accrued=10, outstanding=30)`. Provide sufficient balances/allowance/work and fresh replay/head. No explicit hostile vector is supplied. Core checks `n > outstanding` and returns `CoreRejected`, judgment `effect`, code `S0_EFFECT_RANGE`, no published post/effects. Naive default expansion computes principal/outstanding `-1`, which Source/6 instead rejects at formation. Existing S0 test `repayment above outstanding rejects at Effect` demonstrates the intended baseline using a representable comparison vector.

**Repair recommendation:** specify a total candidate-construction result for economically invalid but well-formed requests. Never emit unsigned-invalid derived fields or substitute authoring acceptance for Core. One bounded option is an explicitly labeled, syntactically valid comparison vector for these cases, then invoke the unchanged strict Source/6/Core path; Core range checks precede vector comparison. If expansion cannot construct a valid candidate, expose that fact separately and define a real Core invocation path that preserves the existing judgments. Choose and document one path before freezing bytes. Add the above default-scenario test and combined faults: overpayment plus stale head; bad gross cap plus overpayment; receiver overflow plus omitted fee credit. The expected first codes must match S0, and rejected output must never publish a candidate post-state as effects.

### F03 — High for mockup acceptance: the eight-family table does not supply the requested financial contracts

**Repository observation:** candidate lines 193–220 enumerate operations and hard questions, but contain only a complete transfer source example. The supplied requirement document demands complete readable examples for eight families, a composed example, construct/reference/status coverage, a source→completion→Core→kernel→result trace, and field-by-field transfer and repayment comparisons. The research workflow fragment explicitly abbreviates the signing/provider bundle; it cannot satisfy those complete examples by reference.

**Discriminator:** ask a programmer to identify the stablecoin mint's signed supply/backing limit, authority, precise read/write footprint, peg evidence, rounding beneficiary and emergency-settlement duty from this candidate. No such source is present. The same absence affects AMM LP authority, option collateral/payoff, governance preservation, staking slash priority and paired bridge recovery.

**Repair recommendation:** ship one versioned coherent mockup artifact with all eight complete examples and a bridge/AMM or other cross-family episode. Use the selected brace/record/named-call notation consistently, mark each construct local/specified/open, map every visible operation to its intended typed Core/kernel interface and state the absent semantics. Include a repayment example and field mapping with an explicit residual debt. All wider execution must stay unsupported. This is an artifact-completeness requirement, not a demand to execute eight financial families in beta.

### F04 — Medium: operation schemas for specified-only authoring are not selected

**Repository observation:** candidate declares broader `pool`, `instrument`, `observation`, `policy`, `grant`, `stage`, `episode`, `party`, `share_class` record forms and named calls, but provides no closed catalog identifying which family calls/declaration fields are known. It promises rejection of unknown constructs while promising name/quantity checks on non-S0 operations, without the signatures needed to determine argument roles.

**Discriminator:** compare `amm.exactInput(pool: P, input: 10 USD, recipient: Alice)` with a misspelled `amm.exactInpt(...)`, or put an `Account` in its pool argument. Generic records alone cannot determine whether this is an unknown call or a known specified-only operation with an invalid nominal role. A blanket unknown-call fallback silently turns unrecognized semantics into apparently checked authoring.

**Repair recommendation:** define a finite documented authoring catalog with declaration kinds, operation names, named argument schemas and nominal/quantity roles. Each action reports both financial-validation coverage and execution support. Unknown calls reject formation; known wider calls report `SpecifiedOnly` and reject expansion with `BETA_PROFILE_UNSUPPORTED`. Explicitly describe remaining relational checks as open; catalog membership does not prove supply, reserve, conservation or authorization.

### F05 — Medium: scenario cell and repayment-state contracts need to be explicit in beta documentation

**Repository observation:** candidate lines 166–184 defer to strict Source/6 and mention ordered cells, but omit the zero-fee footprint, endpoint alias restriction and liability equations from the adopted programmer specification. Strict Source/6 currently requires three transfer balances in owner/recipient/fee-recipient order even when the fee is zero, and all three IDs are distinct. Repayment requires exactly payer/creditor balances, one obligation with `outstanding=principal+accrued`, status consistent with zero outstanding, payer=debtor=signer, and identity conversion.

**Discriminators:** a zero-fee transfer with only owner/recipient cells still fails formation; Treasury aliasing Seller fails even when fee is zero. A loan `(principal=20, accrued=10, outstanding=31)` fails typed formation/Stage rather than becoming an adjustable debt. A partial repayment of 5 from `(20,10,30)` must credit creditor 5, leave `(20,5,25,Outstanding)`, publish an empty S0 retained-duty vector, and still display outstanding liability 25.

**Repair recommendation:** document exact per-operation JSON scenario schemas and Source/6 narrowing, including these constraints. Keep source-origin and stipulated-state-origin labels distinct in inspection/field maps; source fixes obligation ID/domain/asset, while local scenario claims its debtor/creditor/components. Distinguish `TerminalSuccess` of this repayment stage from complete agreement settlement. This does not require adding general duties or changing S0.

## Evidence and abstentions

**Experiment observation:** ran `node --test tests/mil4-s0-source-v6.test.mjs` in `experiments/moriarty-language`; exit 0, 28 passed, 0 failed. These existing tests exercise ordered effects, complete funding, partial/full repayment, ranges, effect-before-history faults, strict cell ordering, and unqualified local stipulations. This establishes those scoped local regression observations, not beta expansion behavior, formal correspondence or external authentication.

**Abstention:** no native proof, actual provider signature/snapshot verification, selected-program binding, atomic ledger acceptance, cross-domain finality or eight-family financial invariants were executed or audited. No affirmative safety/equivalence/settlement vote covers them. Source scale metadata remains an unverified binding; exact decimal authoring cannot authenticate it. Wider recovery must retain Unknown reservations/duties and require qualified nonreceipt evidence; I agree with that design requirement without claiming it has been demonstrated.

No production edits, commits, external acquisitions or peer reads occurred. The only authored artifact is this audit.

## Amendment vote: selected contract v2

Reviewed `CONVERGENCE.md` against this seat's own findings on 2026-09-30; no peer audit was inspected. Parent confirms explicit spawn arguments `gpt-6.1-sol`, medium. No separate returned provider identity/effort receipt is available, so the earlier caveat remains.

- **Concur: economic ID mapping.** Emitting resolved domain/account/asset/obligation IDs rather than source declaration names matches Source/6. Duplicate identity rejection, conflicting scale/representation rejection, no implicit aliases, and explicit transport syntax restrictions avoid both invented identities and silent ID rewriting. Chain/network/representation/scale metadata stays unverified.
- **Concur: F01 numeric repair.** UInt128 expression values and intermediates with explicit S127 narrowing at S0 amount/cap/floor/liability fields resolves the ambiguity. Wider balance/counter/round values remain legal. The checker must attribute field narrowing to the actual assignment; it must not narrow all Qty expressions implicitly.
- **Concur: F02 unchanged-debt placeholder proposal.** For well-formed outstanding debt and overpayment, unchanged principal/accrued/outstanding/status is representable Source/6 comparison data. Requested debit/credit and administrative lines remain explicit. Existing Core checks signed bounds before repayment range and range before vector equality, so a bad cap rejects Intent and otherwise overpayment rejects Effect without publishing a post-state. This is a proposed invalid comparison vector, not a prepared obligation transition. Concurrence is a design vote; default expansion and combined-fault tests remain required implementation evidence.
- **Concur: F05 scenario repair.** Closed duplicate-safe raw JSON, canonical decimal strings, exact three/two balance rows, the distinct zero-fee account, no synthesized counters, bound debtor/creditor and strict obligation accounting make the local-stipulation contract explicit. Requiring an Outstanding obligation for executable repayment agrees with S0. Inspection must continue distinguishing stage success from residual debt and source-fixed identity from stipulated-state claims.
- **Concur in design: F04 closed authoring registry.** The finite registry and fixed named argument schemas, with separate structural/quantity coverage and open relational checks, resolve the selected-contract gap. Published schemas and checker behavior still need implementation; registry membership alone does not establish financial validity.
- **F03 remains open.** Convergence expressly owes a typed horizon mockup with eight complete family examples, composed signed bounds, lifecycle/duty contracts, construct status mapping and the required source/Core/kernel trace. I do not approve an absent artifact or count its planned repair as completed mockup acceptance. This outstanding documentation does not reject the amended bounded S0 architecture.

**Scoped amendment verdict:** concur with v2's financial ID, numeric, invalid-proposal and scenario decisions. Preserve the typed-horizon mockup obligation and all execution/authentication/formal/ledger gates. No amended beta execution was observed in this follow-up.

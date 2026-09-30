---
title: Agreements, financial effects and authority
status: research-proposal
created: 2026-09-30
updated: 2026-09-30
---

# Agreements, financial effects and authority

**Recommendation AG-R01:** organize programmer code around a versioned program with named actions, and bind its fixed financial terms and parties in an agreement instance. An intent authorizes one exact action or a constrained class of completions. A solver proposes a completion; the language derives its complete economic effects and checks refinement of every signed condition before preparation. Natural-language requests may help draft an intent, but are not executable authorization.

**Scope:** S2 research proposal for five independent PL reviews. This memo implements no language, authentication, proof or settlement. The [mockup requirements](../../../docs/language/PROGRAMMER-FACING-MOCKUP-REQUIREMENTS-2026-09-30.md) require readable examples across eight financial families. The [Source/6 contract](../../../experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md) remains the only local execution baseline: exact transfer and funded AccrualFirst repayment, returning `PreparedUnqualified`. Broader examples must retain `specified` or `open` labels. Sources, reading ranges and SHA-256 values are in [agreements-sources.json](../agreements-sources.json).

## Financial language evidence

These are **source facts**, separately from the recommendations below.

| ID | Inspected source | Bounded finding |
| --- | --- | --- |
| AG-F01 | Daml [templates](https://docs.canton.network/appdev/modules/m3-contract-templates) and [choices](https://docs.canton.network/appdev/modules/m3-choices), September 19 captures | A template defines a contract type; contracts instantiate it. Active contracts are immutable. Signatories authorize creation; controllers authorize choices. Choices consume their input contract by default, with nonconsuming variants. Observers receive visibility and can separately be controllers. |
| AG-F02 | Daml [detailed ledger model](https://docs.canton.network/overview/reference/ledger-model-detailed), DA01/DA06/DA15 | Consistency, code conformance and authorization are distinct. Child authority comes from immediate parent actors and input signatories. A missing DvP transfer consequence violates conformance. |
| AG-F03 | FpML [5.2 Recommendation 2 overview](https://www.fpml.org/spec/fpml-5-2-6-rec-2/html/confirmation/fpml-5-2-intro.html), §§2.8.4–2.9.3, 3.2.3, 3.2.13, 3.4 | XML components describe products, parties, accounts and lifecycle events. A party can hold several roles; an account names its beneficiary and servicer. Message correlation, acknowledgement, correction and retraction are distinct from consent and confirmation. The messaging design assumes reliable transport. |
| AG-F04 | FpML [coding schemes](https://www.fpml.org/spec/coding-scheme/fpml-schemes.html), catalog 2-26, June 29, 2026 | Schemes use URIs and can evolve independently of schemas. Separate schemes describe currency, party identity, business calendars and day-count fractions. Individual scheme versions have their own dates. |
| AG-F05 | ACTUS [PAM](https://documentation.actusfrf.org/docs/contract-types/PAM), [day count](https://documentation.actusfrf.org/docs/contract-terms/dayCountConvention), [business days](https://documentation.actusfrf.org/docs/contract-terms/businessDayConvention), September 3 captures | PAM includes principal, rate, dates, role and day count. Calendar conventions distinguish shifting before calculation from calculating before shifting. |
| AG-F06 | Marlowe [data types](https://docs.marlowe-lang.org/tutorials/concepts/marlowe-data), September 2 capture | Its financial DSL composes `Close`, `Pay`, `If`, `When`, `Let`, `Assert` and actions including deposit, choice and notification. |
| AG-F07 | Simplicity [execution model](https://docs.simplicity-lang.org/documentation/execution-model/), pinned September 19 documentation | A program approves or rejects a proposed transaction. External clients propose transactions and track state; the validating program does not initiate a remote workflow. |

**Inference:** FpML is a financial interchange and messaging model, not an executable blockchain DSL with a signed-intent refinement judgment. Its representation and lifecycle vocabulary are useful; XML schema validity cannot establish authority or economic execution. Daml supplies an executable contract/action comparison, ACTUS supplies financial schedule semantics, Marlowe supplies a small financial combinator comparison, and Simplicity supplies a transaction-validation boundary. These are complementary evidence families, not interchangeable implementations.

**Coverage limits:** the fresh FpML overview is historical 5.2; the [5.13 Recommendation 2 landing](https://www.fpml.org/spec/fpml-5-13-8-rec-2/) lists documentation requiring login. No authentication or bypass was attempted and no 5.13 conformance is claimed. Only listed sections of the two large captures were inspected. The September 19 Daml bank has 1,427 current pages; this memo reuses selected primary excerpts and the [authorization](../../../deliverables/daml-study-2026-09-19/studies/authorization/REPORT.md) and [settlement](../../../deliverables/daml-study-2026-09-19/studies/settlement/REPORT.md) studies, not a reading of the whole corpus. No Daml runtime experiment was performed. The old Marlowe tutorial uses a public-key representation differing from the [pinned V1 baseline](../../../wiki/marlowe-baseline.md); use its combinator comparison, not its party spelling as a current adapter contract.

## Programmer model and alternatives

**Recommendation AG-R02:** give `ProgramId`, `ActionId`, `AgreementId`, `EpisodeId` and `StageId` different nominal types. A program identifies versioned behavior; an action identifies one callable behavior; an agreement instance binds terms and persistent claims; an episode links stages; a stage proposes one bounded transition in one settlement domain. Current Core `programId` contains an action ID. Preserve that existing mapping explicitly until a reviewed Core revision adds separate identities.

| Candidate | Programmer reading | Disposition |
| --- | --- | --- |
| Generic record plus `execute(payload)` | Everything is data interpreted by a generic engine | Reject for economic operations: hides permitted actions, required authority and complete effects. |
| Daml-style template and choice | Contract type, instantiated data, permissioned choice | Borrow separation and explicit roles. Specify Moriarty duty persistence and bounded stages independently. |
| Program, agreement instance, intent and completion | Reusable action behavior; bound financial terms; signed discretion; concrete proposal | Recommend for the full language. First beta elaborates only the sealed S0 operations. |

**Specified-only syntax candidate**, demonstrating the programmer distinction, not a runnable profile:

```mori
program Loan {
  terms: LoanTerms;
  action repay(amount: Quantity<terms.asset>): ProposedStage {
    return repay_accrual_first(
      obligation: terms.obligation, payer: terms.debtor, amount: amount,
    );
  }
}
agreement Loan1: Loan {
  terms: {
    obligation: LoanDebt, debtor: Borrower, creditor: Lender,
    asset: USD, schedule: NoSchedule,
  };
}
```

This candidate requires a reviewed representation of dependent term types and `ProposedStage`; neither is an existing Core constructor. The initial beta should avoid implementing that dependency: use named, statically resolved asset declarations and sealed `repay` operations with no arbitrary action body. A programmer-facing action library must eventually have a finite semantic contract, footprint, cost bound and source-to-Core mapping.

**Recommendation AG-R03:** terms have three categories with explicit provenance: owner-fixed fields, typed solver-fillable holes and authenticated provider inputs. A completion fills only declared holes; it cannot change recipients, liability rules, fee caps, required evidence, selected semantics or recovery policy. A provider supplies evidence under a named policy, never unbound authority. Completion is a proposal, not signature creation or ledger acceptance.

**Specified-only exact S0 intent fragment**, matching the [surface proposal](01-surface.md), with illustrative opaque identifiers:

```mori
intent {
  signer: Alice, key: "alice-key",
  nonce: "payment-1", pre_head: "head-1",
  valid: rounds(domain: Preview, start: 100, end: 200),
  gross_cap: atoms(asset: USD, value: 1010),
  fee_cap: atoms(asset: USD, value: 10),
  net_floor: atoms(asset: USD, value: 1000),
  failure: SuccessOnly,
  signed_action: transfer(
    from: Alice, to: Bob, fee_to: Treasury,
    value: atoms(asset: USD, value: 1000),
    fee: atoms(asset: USD, value: 10),
  ),
  observations: [], disclosures: [],
  retained_effects: [], retained_duties: [],
  delegation: None, recovery: None,
}
```

This fixes the complete action; S0 admits no solver holes. A later outcome intent could declare `route: Hole<AllowedRoute>` and bind `constraints`, maximum fee, minimum received quantity and every permitted venue. Its completion must satisfy the entire original relation. Signing the exact plan and signing an outcome relation need different explicit modes and canonical objects. Neither the keyword `intent` nor an illustrative `key` establishes a verified signature.

## Roles, accounts and authority

**Recommendation AG-R04:** roles express distinct powers. `signer` grants formal consent; `spender` identifies the debited account; `controller` may propose a named action under delegated scope; `observer` may receive permitted disclosures; `evidence_provider` attests specified facts. An actor can hold multiple roles through explicit bindings. A wallet connection, account name, observer role, contract ID or private witness conveys no additional power.

**Recommendation AG-R05:** follow [domain/type recommendations](02-types.md): a logical `PartyId` binds per-domain `Account<D>` credentials through policy. Domain identity includes a network and ledger representation; assets bind domain, canonical representation and scale. Equal address bytes across networks, a shared ticker, and a bridge-wrapped asset never imply equal identity. An account beneficiary need not control the servicing party’s ledger funds. Custody adapters must specify whose liabilities change and whose consent authorizes those changes.

**Recommendation AG-R06:** capabilities bind issuer, holder, agreement/action scope, allowed accounts/assets, validity, revocation rule, gross allowance and work budget. Nested calls receive only an explicit attenuated scope. Consumption and remaining counters survive successors and concurrent solver reservations; a refund does not restore consumed gross authority. Record replay identity in authenticated state and compare-and-consume atomically with head updates. S0 has no delegation; preserve `None` in executable output.

**Recommendation AG-R07:** authority and duty have independent lifecycles. Consuming a stage or capability cannot erase a loan, encumbrance or pending external claim. New or increased liabilities require the affected party’s exact consent or previously granted bounded authority. Discharge, forgiveness, transfer and amendment each need a named rule. Positive asset receipt can proceed without interactive acceptance where it introduces no unconsented duty. Imported Daml archival authority must not become a universal `drop(debt)` operation.

Canton stakeholders, disclosure projections and validators do not establish Midnight privacy or ZK proofs. Visibility permissions are a proposed disclosure policy whose enforcement remains open. The observer/controller distinction is a design rule, not a claim that storing a role descriptor enforces it.

## Effects, events and audit

**Recommendation AG-R08:** actions derive ordered economic operations and a complete state footprint. Compare a submitted effect candidate with that derivation; reject added, omitted, reordered or substituted effects. Conservation is per domain/asset and also covers liabilities, claims and encumbrances. Custodian credit/debit, mint/burn and loss allocation need explicit equations and authorized sources; a balanced pair of token transfers alone is insufficient.

**Repository observation:** transfer derives owner debit `v+f`, recipient credit `v`, optional positive fee credit, allowance use, replay use and head advance. Repayment derives payer debit, bound-creditor credit, interest-first obligation update, allowance use, replay use and head advance. Source formation requires distinct endpoints and complete authenticated cells. A debt reduction without funding the bound creditor rejects. Preserve this exact order and the first failure order `stage → intent → effect → authority → history → failure`.

**Recommendation AG-R09:** disclose fee payer/beneficiary, rounding direction and beneficiary, conversion orientation, residual debt, losses and priority rules in the signed terms and signing preview. Initial S0 uses exact atoms, identity conversion and no rounding. Later accrual libraries must bind ACTUS contract/calendar/day-count versions and distinguish financial dates from `Round<D>`. Calendar elapsed time cannot be obtained by silently casting blockchain rounds.

**Recommendation AG-R10:** distinguish three event meanings: a contractual scheduled event, an authenticated observation and a committed ledger notification. An action may respond to an event, but a timestamp or emitted log cannot establish its occurrence or economic effect. Logs should identify agreement, stage, selected program/action, signed intent, predecessor, exact result and evidence relation. Bare hashes bind bytes only; an audit needs their resolved meaning and the proof/signature/snapshot/ledger relation. Pending, prepared, committed, rejected, disputed and recovered remain distinct states.

## Hostile cases and required checks

All outcomes here are **specified checks**, not executed experiments.

| Hostile proposal | Required disposition |
| --- | --- |
| Observer supplies owner’s document and requests debit | Reject missing spending authority. |
| Helper reuses outer signer consent beyond its scope | Reject ungranted action/account/asset authority. |
| Correct signer changes recipient, fee or selected policy after signing | Reject exact-action mismatch or failed intent refinement. |
| Two solvers spend the same nonce/head/allowance | At most one atomic consumption; authenticate reservations and cumulative gross accounting. |
| Repayment lowers debt without creditor credit | Reject incomplete economic effect. |
| Refund then new debit exceeds lifetime gross cap | Reject even if net balance is positive. |
| Same ticker or address used on another network | Reject nominal/domain mismatch. |
| Signed amount changes through scale or rounding | Reject changed terms; require explicit conversion and beneficiary. |
| `Settled` label fabricated at genesis or debt archived | Reject invalid history or unauthorized discharge. |
| Timeout used as proof that remote payment never happened | Retain unresolved duty; recovery needs bound evidence/policy. |
| Program upgraded with matching record shape | Require signed semantic-evolution policy and pinned meaning. |
| Log/effect hash substituted for actual settlement evidence | Reject missing effect-to-ledger relation. |

## Beta boundary and eight-family horizon

**Recommendation AG-R11:** implement an authoring frontend that emits complete literal Source/6 documents for transfer and funded repayment, including exact signed limits and explicit unsupported empty fields. Compare lowering field by field and invoke the actual S0 wrapper. Keep its four unverified bindings—agreement, selected program, asset scale and predecessor—and `PreparedUnqualified` visible. A beta parser cannot qualify signatures, native proofs or ledger commitment.

| Financial family | Required later agreement/action contracts |
| --- | --- |
| AMMs and exchanges | Swap conservation, fee/slippage bounds; separate LP mint/redeem; route authority. |
| Lending and borrowing | Collateral locks, accrual schedules, aggregate debt, liquidation/default duties. Only funded S0 repayment is local. |
| Stablecoins and synthetic assets | Backing/supply equations, mint/burn authority, redemption and emergency claims. |
| Derivatives | Fixing/exercise/expiry, collateral, cash settlement; explicit margin/funding extensions. |
| Oracles and observations | Typed value/unit/source/time/finality; stale, missing and disputed outcomes. |
| Governance | Amendment/veto/timelock authority and protection of already signed duties. |
| Bridges and cross-domain settlement | One-domain stages with paired identifiers, foreign evidence and recovery preserving unknown outcomes. |
| Staking, restaking and yield | Shares/rewards/slashes, loss priority, unbonding and withdrawal duties. |

All rows beyond the exact S0 slice are specified/open. Composition requires compatible authority/evidence/effect contracts and episode lineage; overlapping account effects or conflicting liability priority cannot be combined by concatenating logs.

**Open obligations:** canonical signing and complete refinement; authenticated origins and predecessor compliance; bounded elaboration/type preservation; K first-failure/effect correspondence; Quint races and duty persistence; certified library economics; source-to-native proof correspondence; atomic ledger consumption; privacy and external-provider trust. No source comparator discharges these Moriarty obligations. Current FpML interoperability and human comprehension comparisons remain unperformed. Two fresh official pages used Scrapling with the static-document pattern; no cookies, authentication, bypass or new site pattern was required.

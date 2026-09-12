# Independent Task 2 schema and expected-results proposal

Status: **RECOMMENDED, specified-only; not a frozen production contract or implementation approval.** Root agrees in principle with the liability cap, monotone incurred counter, full-transfer origination consumption and checked UInt64 period formula. The approved six-stage package remains unchanged until Task 2's scoped freeze and review. Task 1 remains the only active implementation.

Reviewer: GPT-6 Astra; requested effort medium, fork none. Read approved Task 2, its complete origination/accrual spec, the complete existing repayment kernel (during the preceding independent review, with relevant admission/application functions refreshed), complete September 11 SDK interface design and its predecessor's nominal-liability requirement. SDK designs remain specified-only. No external call, production edit, author dispatch, service start or transaction occurred.

## Design decision and version boundary

Use `moriarty-financial-agreement-source/5`, integrated `moriarty-financial-expression-contract/4`, kernel input `moriarty-financial-lifecycle/1` and explicit state `moriarty-financial-lifecycle-state/1`. Core needs a new version even if it reuses every expression constructor: its protected operation set and financial admission/result contract change. Freeze source/1–/4, Core/1–/3 and repayment/0 without broadening their accepted schemas, reserved names or error priorities.

The lifecycle kernel accepts primitive JSON only, through `prepareFinancialLifecycle(inputJSON)` and `admitFinancialLifecycleStateJSON(stateJSON)`. Input is closed `{schemaVersion,state,actions}` with exactly the kernel version. It returns `{status:"Prepared",schemaVersion,post,effects}` or existing kernel-style `{status:"Rejected",code,actionIndex}`. Actions are nonempty and capped128. State admission returns an owned tree. Source factory retains elaborate/check/four-string evaluate. Integrated Core/4 follows reviewed Core/3 staging and envelopes, binding only primitive schema and lifecycle PRE text. Source/5 and Core/4 return the funded-result shape with versioned lifecycle financialPost. They accept no signed-authority boolean, post-state, callback or migration object.

The source agreement declares exactly four protected operations Transfer, Repay, Originate, Accrue. All must have the exact typed record shape even if a particular action emits only one kind. Each invocation may emit a nonempty subset; an Accrue-only invocation is valid. Emit order is kernel execution order. Ordinary fields remain ordinary and cannot alter any financial record. No new generic financial read is needed: post_* and current pre reads continue against the corresponding admitted lifecycle obligations/balances/allowances.

Recommendation for minimal source scope: retain one declared nominal unit U and settlement asset A shared by all four records, and retain the current agreement adapter restriction U=A as names. The kernel may retain its existing distinct nominal/settlement identifiers with explicit conversion, but source/5 should not introduce mixed-denomination agreements as an incidental feature. If root wants U and A distinct in the source contract, that is one explicit scoped choice at freeze, with matching static and runtime unit tests. No chain-qualified asset identity or multichain semantics is added.

## Exact closed records

All keys below are required; extra keys reject, including nested records. All decimal values are strings, never JSON numbers. No null defaults or optional compatibility fields. Preserve insertion order of state arrays and append successful history in execution order.

```
LifecycleState {
  schemaVersion: "moriarty-financial-lifecycle-state/1",
  balances: Balance[],
  allowances: Allowance[],
  obligations: LifecycleObligation[],
  usedTransferIds: Identifier[],
  usedAllocationIds: Identifier[],
  usedOriginationIds: Identifier[],
  usedAccrualIds: Identifier[],
  work: Work
}
Balance { party, asset, amount }
Allowance { party, asset, remaining, spent }
Work { remaining, spent, closureReserve }
Conversion { mantissa, scale, rounding }
AccrualTerms { numerator, denominator, rounding, periodSeconds, firstPeriodStart }
LifecycleObligation {
  id, debtor, creditor, denomination, settlementAsset,
  principal, accrued, outstanding, allocationRule, conversion, status,
  originationId, originationTransferId, initialPrincipal,
  nominalLiabilityCap, liabilityIncurred,
  accrualTerms, lastAccruedPeriod, nextAccrualAt
}
```

Balance, allowance, work and conversion values preserve existing meanings. Conversion rounding is `none|floor|ceil`; accrual rounding is only `floor|ceil`. Allocation rule remains `AccrualFirst|PrincipalFirst|ProRata`; status remains `Outstanding|Settled`. `initialPrincipal` is immutable principal at birth and retains the baseline for checking principal and lifetime incurred interest; it is generated from Originate.nominalAmount, not a separate caller-selectable amount.

### Domains and whole-state invariants

- Identifier: genuine full-string ASCII `[A-Za-z][A-Za-z0-9_]{0,63}`; reject trailing newline, whitespace, controls and non-ASCII.
- Unsigned money, rates, conversion mantissa and work: canonical UInt128 decimal strings. Numerator may be0; denominator must be positive; conversion mantissa must be positive; conversion scale0..18. All products/sums must fit UInt128 before division.
- `periodSeconds`, `firstPeriodStart`, `lastAccruedPeriod`, `nextAccrualAt`, action `periodIndex` and `observedTime`: canonical UInt64 decimal strings. Duration must be positive. Time means an integer epoch-second value in this local contract; it has no authentication by itself.
- `initialPrincipal`, `nominalLiabilityCap`, `liabilityIncurred`: positive integers at most signed128 maximum M=`170141183460469231731687303715884105727`.
- Principal, accrued, outstanding: nonnegative integers at most M. Require principal+accrued=outstanding, Outstanding iff outstanding>0, Settled iff0. Require principal<=initialPrincipal<=liabilityIncurred<=nominalLiabilityCap<=M; accrued<=liabilityIncurred-initialPrincipal. This prevents state-shape admission from accepting obvious counter inconsistencies without claiming that a supplied history is authentic.
- Sum of allowance remaining+spent, and work remaining+spent+reserve, fit UInt128. Preserve the existing separate reserve rule. Source snapshots still bound spendable work to65536.
- Every collection independently <=128 and state source<=65536 UTF-8 bytes. All balances/allowances unique by party+asset, obligation IDs unique, each history-ID list unique within its own namespace. Old cross-kind ID independence remains; source users can deliberately use the same spelling in separate namespaces.
- Origination IDs are one-to-one with retained obligations: every obligation's originationId appears exactly once in usedOriginationIds, with no extra origination ID. Origination transfer IDs are unique across obligations and all occur in usedTransferIds. All obligations are retained after settlement, so these equations need no deletion or migration exception.
- For each obligation, checked UInt64 `nextAccrualAt = firstPeriodStart + (lastAccruedPeriod+1)*periodSeconds`. Require the result and each intermediate fit UInt64. Cursor0 means no accepted accrual and the first eligibility boundary is start+duration.
- Sum of all lastAccruedPeriod values equals usedAccrualIds.length. This is valid because periods start at1, never skip, every accepted period (including zero interest) records one ID, and no obligation/history is deleted. Consequently each cursor is <=128 even though the wire type is UInt64. History exhaustion is a deliberate bounded stop, not garbage collection.

These invariants validate internal consistency only. An attacker can still supply a different consistent local projection, including a raised cap. Authenticated state/consent binding in Task 5 must reject such substitution; never label local admission a signature check.

### Source protected record fields and types

Use existing record syntax and nested `Record<T>` types; add no general enum declaration feature. Closed string choices are checked by the kernel. Nested records are named by source declarations; operation binding validates their exact shape structurally, not merely a preferred record name.

```
record ConversionFields {
  mantissa: UInt128;
  scale: UInt128;
  rounding: Text;
}
record AccrualTermsFields {
  numerator: UInt128;
  denominator: UInt128;
  rounding: Text;
  periodSeconds: UInt64;
  firstPeriodStart: UInt64;
}
record OriginateFields {
  obligationId: Text;
  transferId: Text;
  originationId: Text;
  debtor: Text;
  creditor: Text;
  nominalAmount: Quantity<Units<Cash,1>,0>;
  denomination: Text;
  settlementAsset: Text;
  conversion: Record<ConversionFields>;
  allocationRule: Text;
  accrualTerms: Record<AccrualTermsFields>;
  nominalLiabilityCap: Quantity<Units<Cash,1>,0>;
}
record AccrueFields {
  accrualId: Text;
  obligationId: Text;
  periodIndex: UInt64;
  observedTime: UInt64;
}
```

TransferFields and RepayFields are unchanged from source/4. Transfer.transferAmount remains Amount<Cash>; Repay.nominalAmount and Originate nominalAmount/cap have exactly the same declared nominal Quantity type. Source literal time/index values need `u64(...)`, since unsuffixed numeric literals are UInt128. Record constructors use existing `record<RecordName>{...}` surface as admitted by the current parser; author must exercise actual parser/formatter syntax instead of treating this declaration sketch as a new construction syntax.

Source binding checks denomination/settlementAsset Text values against the declared U/A when mapping Originate, and checks Repay/Accrue targets against U before accepting a kernel candidate. Accrue has no nominal operand from which to infer denomination; the source adapter must explicitly validate target denomination against the agreement binding. Without that check, source/5 could accrue a different-denomination obligation hidden in an otherwise valid projection. Old unused unrelated rows remain permitted and preserved.

### Kernel actions

```
Transfer { kind:"Transfer", id, from, to, asset, amount }
Repay { kind:"Repay", allocationId, transferId, obligationId, payer, nominalAmount }
Originate {
  kind:"Originate", obligationId, transferId, originationId,
  debtor, creditor, nominalAmount, denomination, settlementAsset,
  conversion, allocationRule, accrualTerms, nominalLiabilityCap
}
Accrue { kind:"Accrue", accrualId, obligationId, periodIndex, observedTime }
```

Kernel nominalAmount and nominalLiabilityCap are nonnegative canonical decimal strings within M; action application rejects zero Originate/Repay nominal and zero cap. Do not widen the source's signed Quantity by interpreting negative strings as unsigned. All action records are admitted before execution; malformed later action shape beats an earlier runtime action failure, as in the existing kernel.

## Funding and origination equations

Originate does not move cash a second time. It consumes the actual already-executed Transfer of this candidate:

1. Require nominal>0, cap>0, nominal<=cap, new obligationId and originationId. Enforce identifier/capacity checks, valid immutable terms and a representable initial boundary before mutation.
2. Look up transferId only in the current step's executed transfers; prior/future/missing transfer fails `TRANSFER_NOT_IN_STEP`.
3. Require transfer.from==creditor, transfer.to==debtor, transfer.asset==settlementAsset. These establish actual disbursement direction, not debtor consent. Existing self-transfer prohibition excludes self-origination funding.
4. Compute settlement using the existing checked conversion function, including pre-division product bound, none/floor/ceil and positive-settlement dust check.
5. Require the converted settlement equal the transfer's **entire original amount** and its internal unallocated remainder equal that entire amount. Reject overfunding, underfunding, partial prior use or any earlier full use; consume the full remainder to0. Repay may still share a transfer under its existing allocation semantics, but Originate cannot share a disbursement or consume prior partially allocated funding.
6. Append obligation with initialPrincipal=principal=outstanding=liabilityIncurred=nominal, accrued0, statusOutstanding, immutable cap/terms/conversion/roles/funding IDs, lastAccruedPeriod0 and nextAccrualAt=start+duration. Append originationId. The preceding Transfer has already appended its own ID and charged the lender's gross allowance.

One transfer cannot both originate one obligation and fund another Originate or Repay. Removing Transfer must remove successful origination. A transfer in a prior accepted invocation is not a fresh witness even though its tombstone is retained. A failed suffix discards the transfer, origin record, both IDs and all work changes together.

A Transfer-only candidate remains legal cash movement with no new liability. An Originate-only candidate cannot use a previously accepted transfer. This distinction preserves actual funded creation rather than fixture relabeling.

## Accrual equations and eligibility

Let p=current principal, a=current accrued, i=liabilityIncurred, c=nominalLiabilityCap, k=lastAccruedPeriod and t=accrualTerms. Application order for an admitted Accrue is:

1. Locate obligation; require Outstanding. Require fresh accrualId; a reused ID is `DUPLICATE` before period checks.
2. Require periodIndex==k+1. Any repeated/reversed/skipped or zero index is `PERIOD_SEQUENCE`. Require observedTime>=nextAccrualAt; otherwise `PERIOD_NOT_ELIGIBLE`.
3. Checked UInt128 product=p*t.numerator. Divide by positive denominator using specified floor/ceil, with checked ceiling increment. Let interest be that result. Compute checked sums a+interest, outstanding+interest, i+interest, and next cursor/boundary.
4. Require all financial results fit signed128 M and i+interest<=c. Require new UInt64 boundary `firstPeriodStart+(periodIndex+1)*periodSeconds` fit. History append must fit capacity128.
5. Set accrued=a+interest, outstanding=p+accrued, liabilityIncurred=i+interest, lastAccruedPeriod=periodIndex and nextAccrualAt=new boundary; append accrualId. Principal, cap, initialPrincipal, terms, conversion, roles, balances, allowances and transfer/allocation/origination history remain unchanged.

Zero rate or floor-rounded zero still consumes one exact period, one ID and one kernel work unit; it is a successful zero-interest event, not a free retry. Accrual on Settled rejects even with zero rate. Payment never resets the cursor or the incurred counter and never releases nominal-cap capacity.

Late catch-up is explicit and bounded: an observation far beyond the next boundary permits only the named next period, not an implicit loop. A candidate may explicitly list consecutive Accrue actions, each charged and independently eligible against its tentative predecessor; all roll back if any action fails. The period-history capacity and actual work bound limit catch-up. For local execution observedTime is caller-supplied and need not prove current wall time. Task 5 must bind it to accepted ledger time in its reviewed mapping. Do not perform host Date.now reads in kernel evaluation.

## Complete effects

Preserve existing complete Transfer and Repayment effects; Repayment continues to expose discharge split and remainingOutstanding. New effect schemas are closed:

```
OriginationEffect {
  kind:"Origination", originationId, transferId, obligationId,
  debtor, creditor, denomination, settlementAsset,
  nominalAmount, settlementAmount, allocationRule, conversion,
  accrualTerms, nominalLiabilityCap, liabilityIncurred,
  lastAccruedPeriod, nextAccrualAt
}
AccrualEffect {
  kind:"Accrual", accrualId, obligationId, debtor, creditor, denomination,
  periodIndex, observedTime, eligibleAt, nextAccrualAt,
  principalBasis, numerator, denominator, rounding, interestAmount,
  previousAccrued, accrued, previousOutstanding, outstanding,
  previousLiabilityIncurred, liabilityIncurred, nominalLiabilityCap
}
```

Origination nominalAmount also identifies initialPrincipal and the initial principal/outstanding values; accrued is fixed0 by this effect kind, so these are not redundant caller fields. Accrual eligibleAt is the old nextAccrualAt. All numeric effect values are canonical decimal strings. Effects occur one per actual kernel action in original order. The full post includes retained conversion/terms/identity metadata even when an effect does not repeat it.

## Liability boundary required by the SDK

SDK L5 says cash spending caps do not bound new nominal debt. Its settlement and admission sections separately require gross debit, net credit, fees and liabilities over the lifecycle. Recommendation: cap represents **lifetime nominal liability incurred by this obligation**, not current outstanding and not net token flow:

```
liabilityIncurred = initialPrincipal + sum(all successful interest accruals)
0 <= outstanding <= liabilityIncurred <= nominalLiabilityCap
```

Repayment may decrease outstanding and cash, but does not decrease liabilityIncurred. Refunds, sponsor funds, lack of fees and a funded transfer cannot enlarge this cap. No service shortfall is converted into accrued interest. A cap110 permits birth100 plus interest10, even after principal payment lowers current outstanding to80; the next positive accrual is refused if it would incur111 or more.

The local cap is untrusted policy data. It is not a signature, aggregate SDK L5 admission or authorization to borrow. Task 5 must authenticate debtor consent binding program/profile, obligation ID, roles, nominal denomination, principal, cap, complete rate/rounding/period/conversion terms and predecessor/head. It must also enforce the signed scope's total nominal liability across all obligations; a per-obligation cap cannot prevent splitting1000 across ten new obligations. No general signed-authority mechanism or multichain ledger is proposed here. Raw JSON changes that remain internally consistent can pass local admission and must be rejected only by the later authenticated boundary.

## Error contract and admission precedence

Retain existing primitive/bounded transport and kernel record-shape errors. New source/5 state faults use the reviewed funded envelope and synthetic source errors as applicable; old versions do not change. Actions of the new kernel reject with `{status:"Rejected",code,actionIndex}`, index null for admission/preflight and 0-based actual action for reduction faults. No rejected output exposes financialPost/post/effects or a mutated work/history fragment.

Proposed new closed runtime codes: `TRANSFER_AMOUNT_MISMATCH`, `TRANSFER_ALREADY_ALLOCATED`, `PERIOD_SEQUENCE`, `PERIOD_NOT_ELIGIBLE`, `LIABILITY_CAP_EXCEEDED`, `NOMINAL_RANGE`. Reuse `DUPLICATE`, `MISSING_OBLIGATION`, `NOT_OUTSTANDING`, `TRANSFER_NOT_IN_STEP`, `TRANSFER_MISMATCH`, `ZERO_AMOUNT`, `OVERFLOW`, `DUST`, `INEXACT_CONVERSION`, `CAPACITY`, `INSUFFICIENT_WORK`, `NOMINAL_UNIT` and existing Transfer/Repay failures. Use INVALID_AMOUNT for malformed numeric domains; INVARIANT for well-formed state/term inconsistencies. New numeric range checks must have an explicit stable priority at the scoped freeze; recommended precedence is checked UInt overflow, signed nominal range, then cap, then capacity.

Full primitive financial admission precedes runtime snapshots/guards. Whole source/schema/selected action checks keep Task 1 order. Kernel admits all records and checks action-count work before running any action. Source/5 post-target nominal checks complete before suffix. Financial PRE never changes within one source action, so a pre lookup of an obligation just originated in that same action fails MISSING_OBLIGATION; post lookup succeeds after successful kernel preparation. Same-call Originate followed by Accrue is permitted only as explicit ordered descriptors with eligibility satisfied, and cannot use a financial pre lookup to synthesize the new debt.

## Shared work invariant and exact kernel oracle

Use the same spendable pool: total committed cost `Eprefix + actionCount + Esuffix`; workRemaining and financialPost.work.remaining agree; prior spent and closureReserve are carried unchanged except spent increases by cost. Liability and period guards cost no extra ad hoc kernel units: one admitted action costs1, as in repayment/0. Admission scans are bounded by record/byte caps and remain outside runtime E. Failed source suffix has diagnostic attempted work including successful tentative kernel work, with no published debit.

The JSON companion contains **independent expected kernel states and complete effects**, calculated from the explicit equations; it does not run or impersonate the proposed implementation. Initial kernel work is remaining256/spent17/reserve16. Four separate kernel invocations have N2,1,2,2 and therefore end at remaining249/spent24/reserve16. These are kernel-only expectations. Final source counts must add actual E from frozen source; copying249 into a source demo is incorrect.

Main trace, Cash and conversion1/none, AccrualFirst, terms numerator1/denominator10/floor/periodSeconds60/firstPeriodStart1000/cap110:

| Stage | Balance lender/borrower | Allowance lender remaining/spent | Borrower remaining/spent | Principal/accrued/outstanding | Incurred/cap | Cursor/next | Kernel remaining/spent |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Seed | 100/10 | 100/0 | 110/0 | absent | absent | absent | 256/17 |
| Transfer D1 + Originate O1 | 0/110 | 0/100 | 110/0 | 100/0/100 | 100/110 | 0/1060 | 254/19 |
| Accrue A1 at1060, index1 | 0/110 | 0/100 | 110/0 | 100/10/110 | 110/110 | 1/1120 | 253/20 |
| Transfer P1 + Repay R1 nominal30 | 30/80 | 0/100 | 80/30 | 80/0/80 | 110/110 | 1/1120 | 251/22 |
| Transfer P2 + Repay R2 nominal80 | 110/0 | 0/100 | 0/110 | 0/0/0 Settled | 110/110 | 1/1120 | 249/24 |

An unrelated Token balance7 and allowance remaining3/spent1 survive every stage. Histories end transfer[D1,P1,P2], allocation[R1,R2], origination[O1], accrual[A1]. Interest repayment split is principal20/accrued10 for30; final repayment principal80/accrued0. No periods or funding identities disappear on settlement.

## Positive and adversarial oracle cases

Every failure compares the entire predecessor including all rows, terms, caps, initial principal, incurred values, cursor/boundary, history arrays and work. No case succeeds by labeling host data authenticated.

- Originate100 with cap110 and exact Transfer100 succeeds; Originate with no current Transfer fails TRANSFER_NOT_IN_STEP. Prior invocation's D1 does not qualify.
- Transfer amount99 or101 with conversion1/nominal100 fails TRANSFER_AMOUNT_MISMATCH; the preceding transfer rolls back. Wrong from/to/asset fails TRANSFER_MISMATCH. Missing balance or allowance retains existing Transfer priority before Originate.
- Second Originate using same disbursement, or after partial Repay allocation from it, fails TRANSFER_ALREADY_ALLOCATED. Fresh unique IDs cannot restore consumed funding. Reuse existing obligationId or originationId fails DUPLICATE. New obligation capacity128 rejects append; exact capacity127 may append128 if byte bound permits.
- Principal101/rate1/10/floor gives interest10; ceil gives11. With cap112, incurred/outstanding111 or112 respectively. Debt arithmetic does not credit anyone's balance on accrual.
- At time1059/index1, PERIOD_NOT_ELIGIBLE. At1060/index1, succeed. At index0 or2 from cursor0, PERIOD_SEQUENCE even if observedTime is large. Reuse A1 rejects DUPLICATE; fresh A2/index1 after accepted period1 rejects PERIOD_SEQUENCE.
- After main trace's pay30, A2/index2/time1120 calculates8 but refuses LIABILITY_CAP_EXCEEDED (would incur118 against110), despite outstanding only80. Failure preserves80 outstanding, incurred110, cursor1 and every ID/work byte. Retrying payment rather than accrual is valid. A consistent alternate **test fixture** originated with cap118 permits that second period and gives principal80/accrued8/outstanding88/incurred118; do not describe editing cap in a live predecessor as an authorized retry.
- Zero numerator, or principal1/numerator1/denominator10/floor, records one zero-interest period and ID/work. Repeat fails; no free nonce loophole. Ceil for1/10 produces1.
- Two explicit consecutive Accrues at time1120 are allowed if sufficient cap/work/history and exact periods1,2; for unchanged principal100 and cap120 they add20, cursor2/next1180, no cash movement. One descriptor with period2 cannot skip period1. No automatic catch-up occurs.
- Settled obligation fails NOT_OUTSTANDING before new positive or zero accrual. Repay Settled retains existing NOT_OUTSTANDING semantics. The state still retains cursor1, cap110 and incurred110.
- Principal2, numeratorUInt128Max, denominatorUInt128Max overflows product and rejects OVERFLOW even though unbounded arithmetic would yield2. No BigInt unlimited-product escape before truncation.
- InitialPrincipal=M, positive interest1 with otherwise valid large cap impossible beyondM rejects NOMINAL_RANGE before any cap comparison when testing the action arithmetic boundary. Originate cap>M is INVALID_AMOUNT at admission. Balance and allowance still support UInt128Max.
- UInt64-max firstPeriodStart with positive duration rejects initial boundary overflow/invariant; a next boundary overflow on Accrue rejects without consuming its valid current period. Max-rate and boundary tests use independently valid preceding states, not malformed seeds that fail earlier for unrelated invariants.
- Wrong action types, unknown fields, invalid nested rounding and denominator0 reject before execution. Invalid unrelated obligations/history/cursor equations reject full-state admission before a false guard.
- No post read can fabricate principal or debt deletion via ordinary Next. A false late Ensure after origination/accrual returns no effects or IDs and leaves the whole predecessor usable. Failed work after a tentative financial event has the same rollback.
- Source protected binding rejects wrong Quantity unit/cap type/UInt128 period fields. Accrue targeting a valid different-denomination obligation rejects NOMINAL_UNIT rather than silently raising unrelated debt.
- Funding refund cannot reset lender's spent allowance, principal repayment cannot reset incurred liability, zero-interest event cannot reset work and settlement cannot erase history.
- Independent authenticated-stage future controls must change head, cap, debtor consent, nominal unit, principal or terms while preserving otherwise valid local shape and verify the actual ledger rejects. Local shape acceptance is expected for some such changes and must not be claimed as a security failure of the pure projection API.

## Alternatives rejected and open scoped choices

Chosen: inline immutable terms/origination provenance with explicit cap/incurred counters, next-boundary equation and four protected operations on one staged interpreter. It adds only bounded financial state required by the approved item.

Rejected: use cash allowances as nominal debt caps; reduce incurred on repayment; allow partially consumed disbursements to originate multiple liabilities; identify periods only with a caller-chosen nonce; infer time from host clock; migrate old state by inventing consent, start date or cap; accept caller financial POST; build a generic scheduler or implement SDK signing/multichain machinery.

Before Task 2 authoring, root must freeze this proposal's version strings, source U=A restriction, numeric error priority and exact field/effect names in the canonical scoped spec. A final source example must be parsed and its exact work independently counted. Those are normal bounded design choices, not missing user authorization. This proposal does not modify the approved package or imply deployment readiness.

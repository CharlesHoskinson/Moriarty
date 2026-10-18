# Fully worked local recovery examples

These are **proposed source and hand-derived design examples**, not supported programs, independent frozen expectations or executed results. Every control/observation input and balance is hypothetical local data. Asset PAY is `(network="local",issuer="recovery-design",reference="PAY",scale=0)`. Snapshot rows use exact asset identities and canonical account/name ordering. Monetary amounts are unsigned integer atomic units; all example counters fit UInt128. Public qualification is local-reference-stage, authentication none, consumption untracked throughout.

## A. Refund still-modeled custody; prior fees and unrelated duties remain

```mori
language "moriarty/3";
agreement RefundExample {
  asset PAY = { network: "local", issuer: "recovery-design", reference: "PAY", scale: 0 };
  state marker: UInt = 77;
  action deliver() {
    fee(PAY, "alice", "treasury", 2);
    transfer(PAY, "alice", "escrow", 60);
    liability("return_due", PAY, "escrow", "alice", 60);
    await release(ready: Bool) when ready
      recover refund cancel(cancel_local: Bool) when cancel_local {
        transfer(PAY, "escrow", "alice", 60);
        discharge("return_due", 60);
        emit "refund_paid"(60);
      };
    transfer(PAY, "escrow", "merchant", 60);
    discharge("return_due", 60);
    emit "delivered"(60);
  }
  action record_warranty(amount: UInt) {
    liability("warranty", PAY, "operator", "alice", amount);
  }
}
```

The selected action is deliver. The unused record_warranty action declares the exact source-fixed warranty identity so that its incoming positive row is not an undeclared duty. Its body is not executed by deliver or recovery. This example's proposed application policy explicitly releases the contingent return_due obligation after authorized delivery or actual local modeled return; that policy and its real-world authorization are not proved here.

Opening state marker77; incoming liability warranty9 from operator to alice; return_due absent. Start uses proposed logical committed cap15. Captures at the release site are empty because deliver has no inputs/lets/prior events. The handler's own cancel_local binder is an invocation input, not a prior capture.

| Account | Opening | Pending after fee/funding | After cancel_local:true recovery |
| --- | ---: | ---: | ---: |
| alice | 100 | 38 | 98 |
| escrow | 0 | 60 | 0 |
| merchant | 0 | 0 | 0 |
| observer | 7 | 7 | 7 |
| operator | 20 | 20 | 20 |
| treasury | 0 | 2 | 2 |
| Total | 127 | 127 | 127 |

Prefix new effects in source order: Fee(alice→treasury,2), Transfer(alice→escrow,60), Liability(return_due,0→60). Recovery adds Transfer(escrow→alice,60), Liability(return_due,60→0), Emit(refund_paid,UInt60). `accounting.effects` is precisely that six-effect concatenation; it does not replace the funding transfer with its net result. The final complete boundary retains present-zero return_due and positive warranty9. Residuals contain exactly warranty9, with full PAY/operator/alice identity. Marker77 and observer7 remain unchanged at every Step.

By asset/account, lifetime gross debit is alice62 and escrow60, aggregate122. Refund credit60 does not reduce alice's original gross62. Fees remain2, net alice−2/treasury+2 and every other account0; dust0. Return liability equation is opening0 + creation60 − discharge60 = closing0; independent warranty equation is9+0−0=9. These are reference arithmetic equations, not authenticated custody/conservation evidence.

The prefix costs fee2 + transfer2 + liability2 + Await-entry1 =7 committed/attempted units. The normal success suffix costs normal guard2 + transfer2 + discharge2 + emit2 =8; complete normal path15. The recovery clause costs guard2 + transfer2 + discharge2 + emit2 =8; complete recovery path15, so the proposed successful-path ceiling is15. The source-declared handler gate is Step0 with Bool true; body Steps1/2/3 have UInt60 values. Handler snapshots0 retain the pending boundary,1 returns the60,2 sets return_due to present-zero,3 preserves that complete state; new deltas respectively[], Transfer60, Liability60→0, Emit60.

Direct recovery: committed7→15, attempted_total7→15. A prior false cancel_local observation costs2 with no segment or effects, committed remains7 and attempted_total becomes9; a later true recovery ends committed15/attempted_total17. The returned control is RecoveryComplete(refund,cancel,release-site), never a pending release successor. It still has warranty9. An old copied pending input is not revoked or globally consumed by this local result.

## B. Forward compensation after payment; no refund or history erasure

```mori
language "moriarty/3";
agreement CompensationExample {
  asset PAY = { network: "local", issuer: "recovery-design", reference: "PAY", scale: 0 };
  state marker: UInt = 77;
  action buy() {
    fee(PAY, "alice", "treasury", 2);
    transfer(PAY, "alice", "merchant", 50);
    liability("remedy", PAY, "insurer", "alice", 50);
    await acknowledgement(delivered: Bool) when delivered
      recover compensate part_payment(deadline_reached: Bool, outcome_known: Bool)
        when deadline_reached and outcome_known {
        transfer(PAY, "insurer", "alice", 20);
        discharge("remedy", 20);
        emit "compensation_paid"(20);
      };
    discharge("remedy", 50);
    emit "accepted_delivery"(50);
  }
}
```

The application has declared a contingent monetary remedy50 owed by insurer to alice. Its proposed policy explicitly discharges this duty on confirmed accepted delivery, or by the actual amount of a compensation payment. Policy adequacy, authority, evidence of delivery/failure and insurance funding remain owner-unassessed. The local source binds no external truth to the Boolean names. Merely naming an input outcome_known cannot authenticate it.

Opening marker77, no liabilities. Selected action buy, committed cap17. At the sole pending Await there are no captured action inputs/lets. Recovery binders are exact ordered Bools deadline_reached and outcome_known, not normal delivered.

| Account | Opening | Pending after purchase | After true/true partial compensation |
| --- | ---: | ---: | ---: |
| alice | 100 | 48 | 68 |
| insurer | 30 | 30 | 10 |
| merchant | 0 | 50 | 50 |
| observer | 7 | 7 | 7 |
| treasury | 0 | 2 | 2 |
| Total | 137 | 137 | 137 |

Prior effects remain Fee(alice→treasury,2), Transfer(alice→merchant,50), Liability(remedy,0→50). Recovery appends Transfer(insurer→alice,20), Liability(remedy,50→30), Emit(compensation_paid,UInt20). Merchant retains50; no inverse merchant transfer is invented. Treasury retains2. The source-designated remedy remains30 with exact insurer debtor/alice creditor/PAY identity; it is the positive residual even though the returned normal control is closed by compensation. Marker77 and observer7 remain unchanged.

Lifetime gross debit alice52 plus insurer20 equals72; fees2. Nets are alice−32, insurer−20, merchant+50, treasury+2, observer0. Remedy equation0+50−20=30. The20 payment is a new debit against insurer's locally modeled balance, not money recovered from the merchant and not an authority unit. If the application owes other duties they would also remain; this specimen opens none.

Prefix costs7 as in A. Normal suffix is normal guard2 + discharge2 + emit2 =6; complete normal path13. Recovery guard costs dispatch1 + deadline Name1 + outcome Name1 + And1 =4, then transfer2 + discharge2 + emit2 =6; recovery cost10 and complete recovery path17. Thus successful-path ceiling17. Direct recovery ends committed17/attempted_total17. The true gate Step0 has Bool true; body Steps1..3 have UInt20. Step1 pays20, Step2 leaves remedy30, Step3 emits20. Full ordered history and residuals must be compared, not only alice's closing68.

## C. Deadline with unresolved outcome remains pending

Use exactly B's source, opening and actual prefix. Invoke part_payment with deadline_reached:true and outcome_known:false. The complete guard is false; both operand visits remain charged. No transfer, discharge, emit or successful gate Step is published. Boundary remains alice48/insurer30/merchant50/observer7/treasury2, marker77, remedy50. Control retains the same action/site/captures; committed remains7; attempted_total7→11. All previously committed fee/funding/duty effects remain in accounting. Residual remedy50 remains unresolved. No timeout-driven automatic cancellation or refund occurs.

A later true/true invocation, under a caller-asserted local known-outcome hypothesis, costs10 and yields B's final amounts with committed17/attempted_total21. Neither invocation establishes reliable external nonexecution. External delivery, late evidence, cancellation races or duplicate funds recovery need designated-owner semantics/evidence. The local result cannot discharge MPLR-010's authentic-outcome requirement.

## D. Failed new payment cannot erase old fees, payment or duty

As a separate ordinary finite body-failure example, open B with insurer10 instead of30, preserving all other rows; total117. Prefix still leaves alice48/insurer10/merchant50/observer7/treasury2 and remedy50, committed7/attempted_total7. Under the proposed statement order, true/true gate costs4, then the transfer attempts20 from insurer10 and fails after its dispatch/literal visits2. Total invocation attempted6; committed remains7; attempted_total becomes13. No successful compensation segment or new committed effect is returned; no discharge/emit executes. The retained complete boundary and residual remedy50 are unchanged. Original fee2 and merchant payment50 remain. Exact typed diagnostic/offset/code precedence must be selected/reviewed before an independent error golden; this is no claim about an executed runtime or security refusal.

These four finite cases distinguish returned control closure, unresolved observation and failed handler execution. They do not establish replay protection, asset control, arbitrary compensation, source-to-target correspondence or a full financial-family result.

# Finite feature predicates, requirement mapping and custody

## Source-author worked finite examples

The preserved [prior examples](prior-proposal/EXAMPLES.md) contain complete A–D account/liability/state/effect/work tables. This comparison retains their hypothetical financial policies and arithmetic; it changes the source pairing/interface and guard trace representation, not the amounts. These are authored design examples, not independent goldens or implementation observations.

The base A source is separately declared as ordinary Core/2:

```mori
language "moriarty/2";
agreement RefundExample {
  asset PAY = { network: "local", issuer: "recovery-design", reference: "PAY", scale: 0 };
  state marker: UInt = 77;
  action deliver() {
    fee(PAY, "alice", "treasury", 2);
    transfer(PAY, "alice", "escrow", 60);
    liability("return_due", PAY, "escrow", "alice", 60);
    await release(ready: Bool) when ready;
    transfer(PAY, "escrow", "merchant", 60);
    discharge("return_due", 60);
    emit "delivered"(60);
  }
  action record_warranty(amount: UInt) {
    liability("warranty", PAY, "operator", "alice", amount);
  }
}
```

Pair this with the refund companion in GRAMMAR-AND-IR.md. Opening account amounts are alice100/escrow0/merchant0/observer7/operator20/treasury0, state marker77 and incoming warranty9. An unused source action declares that exact warranty identity. A's prefix costs7 and leaves alice38/escrow60/merchant0/observer7/operator20/treasury2, present return_due60 and warranty9. Recovery costs8: guard2 plus three body statements at2 each. It appends return transfer60, discharge60 and Emit60; final alice98/escrow0, other accounts unchanged, return_due present-zero and warranty9 residual. Lifetime per-account gross debit remains alice62/escrow60, aggregate122; net alice−2/treasury+2; fees2. The actual base forward plan remains total15; recovery path total15. Direct Process C/T is15/15; false guard then true C/T is15/17. Embedded old Continuation remains unchanged during false while wrapper T increases.

The base B source is likewise ordinary Core/2:

```mori
language "moriarty/2";
agreement CompensationExample {
  asset PAY = { network: "local", issuer: "recovery-design", reference: "PAY", scale: 0 };
  state marker: UInt = 77;
  action buy() {
    fee(PAY, "alice", "treasury", 2);
    transfer(PAY, "alice", "merchant", 50);
    liability("remedy", PAY, "insurer", "alice", 50);
    await acknowledgement(delivered: Bool) when delivered;
    discharge("remedy", 50);
    emit "accepted_delivery"(50);
  }
}
```

Pair it with the compensation companion in GRAMMAR-AND-IR.md. Opening alice100/insurer30/merchant0/observer7/treasury0, marker77, no liabilities. Prefix7 leaves alice48/insurer30/merchant50/observer7/treasury2 and remedy50. True/true recovery guard4 plus transfer/discharge/emit6 costs10; final alice68/insurer10/merchant50/observer7/treasury2, remedy30 residual and unchanged marker77. Original merchant50 and fee2 stay committed. Lifetime gross alice52/insurer20, aggregate72; nets alice−32/insurer−20/merchant+50/treasury+2/observer0. Duty equation0+50−20=30. Actual base normal plan_total is13; recovery path total17 and the paired proposed ceiling17. Never set a forged base committed value17 or label this base StageOutcome::Complete.

| B invocation path | Complete control/boundary effect | Committed C | Total attempts T |
| --- | --- | ---: | ---: |
| Prefix only | Actual pending acknowledgement; merchant50/fee2/remedy50 | 7 | 7 |
| Deadline true, outcome unknown/false | No guard segment/body effect; same complete boundary/control | 7 | 11 |
| Then true/true recovery | New insurer→alice20, remedy50→30, Emit20; RecoveryComplete | 17 | 21 |
| Direct true/true recovery | Same final state/effects/duty without the false observation | 17 | 17 |
| Separate normal delivered:true path | Existing normal forward completion; remedy explicitly discharged50; merchant50 and fee2 remain | 13 | 13 |

For a distinct local body-failure design case, B opening insurer10 makes the requested20 payment unavailable in the modeled snapshot. Under the proposed source evaluation order, guard costs4 and attempted transfer costs2, then body rejects: C7/T13, prior boundary and remedy50 retained, no new successful effect, fee2/merchant50 retained. Exact diagnostic/ordering/authority expectations remain owner-unassessed; this is no new security test or executed refusal claim.

Successful recovery body Steps now use indices0/1/2, with separate source-clause guard metadata; earlier `/3` gate-Step0/body1..3 representation is not carried over. Refund handler roots are1/2/3 after guard root0; compensation body roots3/4/5 follow guard root2. Complete per-step snapshots and financial deltas remain mandatory. A final account equality alone is insufficient.

## Single smallest coherent consumer predicate

After adoption and independent freeze, the public paired load/start/recover library consumer must take A's actual pending state and cancel_local:true, preserve the actual old forward receipt/Continuation shape, and return RecoveryComplete with exact complete new guard/body report, original ordered prefix plus refund effects, fee2, alice gross62, escrow gross60, present-zero return_due, warranty9 residual, C15/T15 and no pending normal successor. A later normal resume on that same Process is not a normal pending operation. This is value-level local closure, not external revocation/once-only evidence.

The fuller finite set adds false guard accounting, B's forward compensation/positive residual and deadline-only unresolved case, then ordinary body-failure coverage after diagnostic premises are adopted. These are new recovery predicates, not repetition of normal multi-condition passing prefixes. Startup/prefix observations are necessary actual input custody, not a new completion count.

## Proposed ordinary EARS crosswalk

The following are review clauses, not canonical edits or a replacement inventory:

| Proposal | Ordinary predicate | Retained requirement |
| --- | --- | --- |
| L03-01 | WHEN an old Core/2 source/API is used, its existing Program/Plan/Continuation/StageOutcome/report semantics and key sets SHALL remain unchanged; the recovery module SHALL use a separately named consumer/profile. | UNI-018/021; adopted finite Core/2/M01/normal report clauses |
| L03-02 | WHEN paired recovery is invoked on an actual pending Process, it SHALL select the source-bound site/handler, use actual boundary/captures and publish a distinct closed recovery result only after that body's normal completion. | MPLR-005; UNI-006/008; missing C0.1 recovery judgment |
| L03-03 | WHILE the guard is false, the pending boundary/control and committed effects/work SHALL remain; only separate cumulative attempted accounting increases. Deadline-only observation SHALL NOT create a local recovery-complete result. | MPLR-010/009; UNI-007/020 |
| L03-04 | WHEN refund or compensation changes finances, explicit source statements SHALL determine complete ordered asset effects, retained fees/gross debit and separately typed liability evolution; control closure SHALL NOT delete positive residual duties. | MPLR-005/017; UNI-005/020 |
| L03-05 | IF a local handler rejects, the result SHALL preserve prior committed state/effects/fees/duties and report actual attempts/full error, without a successful recovery segment. Actual target retained-phase effects SHALL remain separately qualified. | MPLR-009/017; UNI-006/021; product phase distinction |
| L03-06 | WHEN recovery is claimed on a target or as authentic cancellation, missing authority/outcome/control/consumption/history/proof/ledger evidence SHALL remain explicit; finite local wrapper observations SHALL NOT close those requirements. | MPLR-005/006/010/011/017; UNI-019/021/022; U3/U4 and full release exits |

No scope reduction is accepted by a language version. Original 119 UTXO/11 first-contract/18 PCD inventories, all U0–U7/M1–M17/MC/SP/G exits and full financial/composition obligations remain. Pair's local disjoint feature and normal multi-condition results retain their original scopes; they cannot vote or supply cancellation results.

## Independent input custody and decision gate

The current author has inspected the base implementation and the earlier design. New examples and DTOs are source-exposed author material. Root must obtain two substantive agreeing **non-author design votes** plus designated-owner review before implementation. None is present; no provider is dispatched here. Preserve dissent and refresh affected reviews if the exact packet changes.

A fresh non-author custodian, after design adoption, independently derives from adopted ordinary rules and complete source inputs: exact full base Program/Plan; separate companion handler arenas/layouts/costs; complete actual-input scenarios/events; complete new output DTO/key sets and trace/snapshot/delta/effect history; per-asset/account gross/net/fees/duty/work tables; and wrapper/old-base compatibility expectations. No candidate parser/evaluator/model output, shared wrapper or new actual result may generate expected data. Identity and failure goldens require owner-adopted premises before freezing; no guessed hashes/codes are supplied.

Existing M01/M03 and report adoption do not automatically adopt this companion grammar, handler-local arena, new accounting profile, Process control, guard metadata or failure schema. Those are explicit unadopted choices. Authority/actual funding/control, reliable external outcome evidence, cap/boundary provenance and failure ordering, copied-input replay/races, persistence/migration, identity binding and target phase/native/proof interfaces remain owner-dependent. The minimum public in-memory feature is not general authenticated cancellation or a deployed settlement result.

No production/shared source, W/I/J/K/G or Git state is changed. No build, runtime/evaluator/model/SDK/native/provider dispatch or subagent is used. Root's later implementation decision applies to the exact reviewed candidate, not this document's existence.

# Literal derivation: PartialDelivery

Source identity is the independently supplied 634-byte partial-delivery.mori,
SHA256 `1d3103b3e6470fd1c691d8333524556e92c697cd6213d504810401aae7e1612d`.
Normative baseline is existing local Git object main
`9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb`, copied under basis/. Public
adopted design is implementation-source-exposed; its clauses are premises,
not producer output or a successful execution. Current consumers are UNRUN.

The selected positive input has total10, first4 and marker77. The written
Requires is true (4<=10); the captured remainder is 10-4=6. Source has one
asset PAY=(local,partial-delivery-oracle,PAY,scale0), one state marker and one
action deliver(total:UInt,first:UInt). The initializer77 and declaration
metadata allocate no expression nodes under M01. No unused action is added.

## Complete Program representation

Fresh left-to-right postorder, one source-wide expressions arena:

| ExprId | Complete node meaning | Written occurrence |
| ---: | --- | --- |
| 0 | Name first | Requires left |
| 1 | Name total | Requires right |
| 2 | Binary Le(0,1) | Requires root |
| 3 | Name total | Let left |
| 4 | Name first | Let right |
| 5 | Binary Sub(3,4) | Let root |
| 6 | Name total | funding Transfer amount |
| 7 | Name total | Liability creation amount |
| 8 | Name ok | approval guard |
| 9 | Name first | first outgoing Transfer amount |
| 10 | Name first | first Discharge amount |
| 11 | Name seen | receipt guard |
| 12 | Name remainder | final Transfer amount |
| 13 | Name remainder | final Discharge amount |

Statement ordinals0..9 are Requires(predicate2), Let(remainder,value5),
Transfer(alice,escrow,amount6), Liability(delivery,escrow,bob,amount7),
Await(approval,ok:Bool,guard8), Transfer(escrow,bob,amount9),
Discharge(delivery,amount10), Await(receipt,seen:Bool,guard11),
Transfer(escrow,bob,amount12), Discharge(delivery,amount13). Aliases in statements
remain PAY; effects use the complete AssetId. Fourteen nodes are distinct fresh
occurrences; repeated names are not interned. All ten statements stay in their
written source order, including both awaits.

## Filled Plan, frames and work

Approval is site1 at source ordinal4; receipt is site2 at ordinal7. Site1
captures total(Input ordinal0), first(Input ordinal1), remainder(Let ordinal1).
Site2 retains those in order then adds ok(Event ordinal4). No liveness pruning
is performed. The pending await's own binder is excluded: ok is not in site1,
seen is not in site2. Marker is state, not a capture. Runtime captures are
Typed NamedValues total10, first4, remainder6; site2 additionally retains oktrue.
Idle entry/completion is site0,captures[].

The adopted local schedule charges dispatch1 plus distinct expression nodes
with a fresh cache for each statement. Merely reaching a pending await costs
entry1, emits no Step and does not evaluate its guard.

| Successful segment | Charges in source order | Cost | Committed before/after |
| --- | --- | ---: | --- |
| start | Requires1+3; Let1+3; Transfer1+1; Liability1+1; await-entry1 | 13 | 0 / 13 |
| approval true | guard1+1; Transfer1+1; Discharge1+1; next-await-entry1 | 7 | 13 / 20 |
| receipt true | guard1+1; Transfer1+1; Discharge1+1 | 6 | 20 / 26 |

Thus segment_cost=["13","7","6"], committed_before=["0","13","20"],
plan_total="26". The false receipt independently attempts2 (dispatch1+Name1),
retains committed20 and does not contribute to the successful-prefix total.
Across the four chosen invocations attempted work totals13+7+2+6=28;
committed completion is26. These are logical interpreter units, not fees,
circuit measurements, DUST, reservations or lifetime resource authority.

M03 canonical Program hash is `22dc05bcc2fa759bce02cd76c663f4c45e10ddbd2674fee281363a4452ce00b7`.
The filled Plan includes that program_hash before canonicalization. Its hash is
`5d501a362bb09d726c3f105b465e4b41a2e00e86791c32ca6b33bd1ef81b72b0`. Both use the full exact JSON object, recursive
lexical member order, preserved array order, compact UTF-8 bytes with no prefix
or final LF, and SHA256 lowercase hex. The complete preimages are retained.
This mechanically applies the separately designated-owner clause and supplies
no new identity/security judgment.

## Complete snapshots, ordered effects and partial duty

All seven distinct snapshots keep marker77 and PAY account rows in exact order
alice,bob,escrow,observer. Their respective balances and liability lists are:

| Boundary | alice | bob | escrow | observer | delivery duty |
| --- | ---: | ---: | ---: | ---: | --- |
| S0 opening | 20 | 2 | 3 | 9 | absent |
| S1 after funding Transfer | 10 | 2 | 13 | 9 | absent |
| S2 after creation / waiting approval | 10 | 2 | 13 | 9 | present10 |
| S3 after first payment | 10 | 6 | 9 | 9 | present10 |
| S4 after first discharge / waiting receipt | 10 | 6 | 9 | 9 | present6 |
| S5 after final payment | 10 | 12 | 3 | 9 | present6 |
| S6 after final discharge / completion | 10 | 12 | 3 | 9 | present0 |

Every row sums numerically to34 as a static fixture arithmetic check. No fee
statement exists, so totalfees0, no Fee effect and no invented fee recipient.
The original escrow balance3 and observer9 survive every boundary. Funding
adds10 to escrow; first payment removes4; final payment removes6. Bob receives
net10 over its initial2. Alice grossdebit10; escrow grossdebit10 comprises the
two outgoing payments. Transfer and duty discharge remain separate effects.

The six ordered deltas across accepted segments are Transfer10 alice→escrow,
Liability delivery0→10 (creation from ABSENCE), Transfer4 escrow→bob,
Liability delivery10→6, Transfer6 escrow→bob, Liability delivery6→0.
Each Liability effect preserves exact PAY/escrow/bob identity. No declaration,
observer row, input or capture is inserted into state. The final present-zero
row is retained even though residual=[]; it is not the initial absent duty.
Opening0 +creation10 +accrual0 -discharge4 -discharge6 =closing0 describes this
literal duty evolution, without introducing a dedicated accrual construct.

## Four whole outcomes and report bytes

Start emits Steps0..3 with values Booltrue,UInt6,UInt10,UInt10 and complete
snapshots S0,S0,S1,S2. Its ordered effects are only the first two deltas. The
await approval entry has no Step. It waits advancedtrue with frame0→frame1,
boundaryS2, residualdelivery10, attempted13/committed13 and stored capnull.

Approvaltrue emits Steps4..6: accepted Await Booltrue at S2 with empty delta,
Transfer UInt4 at S3, Discharge UInt4 at S4. Reaching receipt produces no entry
Step. It waits advancedtrue with frame1→frame2, boundaryS4, residualdelivery6,
attempted7/committed20 and capnull. Accepted oktrue is captured for site2.

Receiptfalse has statuswaiting, advancedfalse, segmentnull and attempted2.
Its complete continuation is exactly the prior site2 object, including all
hashes/captures/capnull/committed20/boundaryS4, and residualdelivery6. It emits
no Step, effect, new snapshot or capture of seenfalse. A later true receipt
uses the ACTUAL retained continuation; no financial prefix is repeated.

Receipttrue emits Steps7..9: Await Booltrue at S4, Transfer UInt6 at S5,
Discharge UInt6 at S6. Completion has frame2→idle, attempted6/committed26,
residual[], and no root advanced or continuation member. Present-zero duty
stays in post and its final Step snapshot. The complete outcome is local
reference completion, not an authenticated delivery/settlement certificate.

Every report retains exactly qualification=local-reference-stage,
authentication=none,consumption=untracked and the adopted status-specific root
members. Each full segment retains original statement ordinals and exact
Program statements, typed results, complete pre/post/Step snapshots and delta
concatenation. Full reports are explicit standalone JSON, not selected-field
or normalized comparisons. Public stdout is independently specified ASCII
JSON with lexical object keys, two-space indentation, correct arrays, and one
final LF; stderr is exactly empty and successful process exit is0. All strings
in this specimen are ASCII, avoiding unselected Unicode printing questions.

No rejected/hostile/admission/cap-refusal result is invented. All historical
14 gap rows, original freezes/failures/counters and parent exits remain open.
The selected path is conditionally already-admitted normal local observation;
source/DTO arithmetic consistency is not evidence that a consumer has run.

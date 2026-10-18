# Complete authored financial/frame/work example

Status: hand-authored proposed arithmetic and trace specification. No evaluator, compiler, model or build was run. This is NOT independent expected-output custody. Complete production-shaped JSON and exact canonical bytes remain missing until semantics/wire adoption and independent derivation.

## Exact common domains

Every balance/duty below has exact asset `{network:"local",issuer:"shared-tier-design",reference:"CASH",scale:0}`. Every row explicitly retains state marker=77, observer balance=11 and outside liability `{name:"outside",asset:CASH,debtor:"reserve",creditor:"observer",amount:5}`. State contains exactly marker and tier. Balance accounts, in canonical order, are alice, bob, fees-a, fees-b, observer, reserve. Liability rows, when present, are name-sorted delivery-a, delivery-b, outside. Delivery-a identity is reserve→alice; delivery-b is reserve→bob. A dash means the row is absent, not a zero duty. No row is discarded on completion.

All quantities in the tables are UInt atomic units; proposed JSON uses decimal strings. Each positive call advances exactly one head revision; row n is revision n and has exactly n original history envelopes. Reporting/opening invokes no source segment. Invocation count is n. Cumulative committed total equals cumulative attempted work here because every selected guard is true and every invocation succeeds. No other schedule/failure is inferred.

Notation for complete frame/status records: `N` = not-started, site0, captures[], origin=null, last_observation=null. `W(q,r,h)` = waiting, site1, captures exactly `[{name:"quoted",value:{type:"UInt",value:q}}]`, frame_origin_revision=r, last_observation=h. `C(h)` = complete, site0, captures[], origin=null, last_observation=h. Every slot has immutable cap23. No captures are padded. Inputs are empty. Historical zero-based indices h below refer to this session's full log.

## Sequential A then B

| n | Explicit call | tier | marker | reserve | alice | bob | fees-a | fees-b | observer | duty-a | duty-b | outside | A frame/status | B frame/status | committed [A,B] | attempted total | effects length |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|---|---:|---:|
|0|open|1|77|100|0|0|0|0|11|—|—|5|N|N|[0,0]|0|0|
|1|start A|1|77|100|0|0|0|0|11|10|—|5|W(1,1,0)|N|[5,0]|5|1|
|2|resume A|2|77|92|7|0|1|0|11|3|—|5|C(1)|N|[23,0]|23|7|
|3|start B|2|77|92|7|0|1|0|11|3|20|5|C(1)|W(2,3,2)|[23,5]|28|8|
|4|resume B|3|77|78|7|12|1|2|11|3|8|5|C(1)|C(3)|[23,23]|46|14|

Charged amounts A=1/B=2. Quoted emits A=1/B=2. Full ordered effect sequence is `StartA, FinishA(quoted1,charged1), StartB, FinishB(quoted2,charged2)` using the literal effect records below. Residual at each row is exactly all its positive listed duties, including outside.

## Sequential B then A

| n | Explicit call | tier | marker | reserve | alice | bob | fees-a | fees-b | observer | duty-a | duty-b | outside | A frame/status | B frame/status | committed [A,B] | attempted total | effects length |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|---|---:|---:|
|0|open|1|77|100|0|0|0|0|11|—|—|5|N|N|[0,0]|0|0|
|1|start B|1|77|100|0|0|0|0|11|—|20|5|N|W(1,1,0)|[0,5]|5|1|
|2|resume B|2|77|87|0|12|0|1|11|—|8|5|N|C(1)|[0,23]|23|7|
|3|start A|2|77|87|0|12|0|1|11|10|8|5|W(2,3,2)|C(1)|[5,23]|28|8|
|4|resume A|3|77|78|7|12|2|1|11|3|8|5|C(3)|C(1)|[23,23]|46|14|

Charged A=2/B=1. Quoted A=2/B=1. Effects are `StartB, FinishB(1,1), StartA, FinishA(2,2)`.

## Both pending, A resumes first

| n | Explicit call | tier | marker | reserve | alice | bob | fees-a | fees-b | observer | duty-a | duty-b | outside | A frame/status | B frame/status | committed [A,B] | attempted total | effects length |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|---|---:|---:|
|0|open|1|77|100|0|0|0|0|11|—|—|5|N|N|[0,0]|0|0|
|1|start A|1|77|100|0|0|0|0|11|10|—|5|W(1,1,0)|N|[5,0]|5|1|
|2|start B|1|77|100|0|0|0|0|11|10|20|5|W(1,1,0)|W(1,2,1)|[5,5]|10|2|
|3|resume A|2|77|92|7|0|1|0|11|3|20|5|C(2)|W(1,2,1)|[23,5]|28|8|
|4|resume B|3|77|78|7|12|1|2|11|3|8|5|C(2)|C(3)|[23,23]|46|14|

Charged A=1/B=2. Quoted A=1/B=1. Effects are `StartA, StartB, FinishA(1,1), FinishB(1,2)`. B's pending capture stays quoted1 after A changes tier to2. Its original start envelope stays at revision2; its resume execution.pre is the complete revision3 shared head. Reusing B's old continuation boundary would lose A's new tier/payment/discharge; updating B's quoted capture to2 would lose its actual earlier quote. Both are observable failures of this proposed rule.

## Both pending, B resumes first

| n | Explicit call | tier | marker | reserve | alice | bob | fees-a | fees-b | observer | duty-a | duty-b | outside | A frame/status | B frame/status | committed [A,B] | attempted total | effects length |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|---|---:|---:|
|0|open|1|77|100|0|0|0|0|11|—|—|5|N|N|[0,0]|0|0|
|1|start A|1|77|100|0|0|0|0|11|10|—|5|W(1,1,0)|N|[5,0]|5|1|
|2|start B|1|77|100|0|0|0|0|11|10|20|5|W(1,1,0)|W(1,2,1)|[5,5]|10|2|
|3|resume B|2|77|87|0|12|0|1|11|10|8|5|W(1,1,0)|C(2)|[5,23]|28|8|
|4|resume A|3|77|78|7|12|2|1|11|3|8|5|C(3)|C(2)|[23,23]|46|14|

Charged A=2/B=1. Quoted A=1/B=1. Effects are `StartA, StartB, FinishB(1,1), FinishA(1,2)`. A's original frame origin remains revision1 while its resume execution.pre is revision3. The fee recipients differ from A-first, so this is not a commutative/independent-domain example. Reserve+alice+bob+fees-a+fees-b remains100 at every boundary; observer11 is outside that conserved100 and remains present. Duties are tracked separately from balances: opening outside5 + new10+20 − explicit7−12 = closing16.

## Complete source effects and step reconstruction contract

These are literal record patterns with only the quoted/charged/table-row values substituted. This describes authored examples; it does not replace missing independent full JSON. Exact Value tagging, Statement expression indices, Program/Plan identities and canonical bytes must be independently derived after the source/wire rule is adopted.

StartA's one effect is Liability(delivery-a,CASH,reserve,alice,before0,after10). StartB's is Liability(delivery-b,CASH,reserve,bob,before0,after20).

FinishA(q,c) is, in order: Fee(CASH,reserve,fees-a,amount c); State(tier,before c,after c+1); Transfer(CASH,reserve,alice,amount7); Liability(delivery-a,CASH,reserve,alice,before10,after3); Emit(quote-a,UInt q); Emit(charged-a,UInt c).

FinishB(q,c) is, in order: Fee(CASH,reserve,fees-b,amount c); State(tier,before c,after c+1); Transfer(CASH,reserve,bob,amount12); Liability(delivery-b,CASH,reserve,bob,before20,after8); Emit(quote-b,UInt q); Emit(charged-b,UInt c).

For each start, Execution.pre is the prior complete table row; Step0 is the original `let quoted = tier`, value UInt q, full unchanged current Snapshot, effects[]. Step1 is the original liability statement, value UInt10/UInt20, full Snapshot with that entire new duty, effects[StartA/StartB]. Reaching await ordinal2 charges1 and emits no Step for that await yet. Execution.post is the next complete row, frame_pre site0/captures[], frame_post site1/quoted q. Waiting has advanced=true, segment=Some(full segment), continuation.boundary equal full post, logical.cap23/committed5, attempted5, and every positive residual duty in post.

For each successful resume, Execution.pre is the previous complete global row, frame_pre site1/quoted q, frame_post site0/captures[]. The resumed segment includes these complete Steps with original ordinals:

| Ordinal | Statement | Step.value | Snapshot/effect after this step | Cost |
|---:|---|---|---|---:|
|2|await approval_a/b(ok) when ok|Bool true|unchanged complete pre, effects[]|2|
|3|let charged = tier|UInt c|unchanged complete pre, effects[]|2|
|4|fee(... charged)|UInt c|debit reserve c, credit selected fee recipient c; full Fee effect|2|
|5|next tier = tier + 1|UInt(c+1)|replace only tier; full State effect|4|
|6|transfer(... 7/12)|UInt7/UInt12|debit reserve and credit selected recipient; full Transfer effect|2|
|7|discharge(... 7/12)|UInt7/UInt12, NOT remaining duty|duty10→3 or20→8; full Liability effect|2|
|8|emit quote-a/b(quoted)|UInt q|same complete Snapshot; full Emit effect|2|
|9|emit charged-a/b(charged)|UInt c|same complete Snapshot; full Emit effect|2|

No interim Step omits another slot's duty, untouched marker/observer/outside, zero balances or any preceding fee/payment. Complete outcome has committed23, attempted18 and all three current positive liabilities (or the subset present at that exact sequential prefix). History retains full original segments/steps; cumulative effects concatenate these records in exact invocation order. Later history never rewrites earlier outcomes/boundaries or invents Steps for another slot.

Both selected ActionPlans have statement_count10, await site1 at ordinal2, binder ok:Bool, and capture quoted:UInt with origin Let/ordinal0. Cost start=2+2+1=5. Cost resume=2+2+2+4+2+2+2+2=18. Full action23; positive session46. The unused declare_outside action is one liability statement of cost2 with no await and is never selected/included in these per-slot totals.

## Limits of this authored trace

These four schedules do not establish all possible interleavings, false guards, failures, authorization, canonical hashing, an authenticated shared head, split/join, unique consumption, confidentiality, native work adequacy, proof correspondence or ledger settlement. Their full expected reports and Program/Plan/preimages must be derived/frozen by an independent custodian; none exists in this packet. An eventual implementation may be checked against these proposed examples only after the semantic choices and designated-owner questions are resolved.

# Hand derivation and ordinary-example design

Baseline main `74a909d69639df8823a527a4d608a7516ccd69a2`. All outcomes below are expected literals,
not actual consumer output. The source, full Program/Plan, reports and test are
new external proposal artifacts; no shared file, source branch or Git object
was written. Existing public constructs were selected rather than inventing a
new fill operation or reusing a producer-generated trace. A single await gives
two successful stages; one optional false observation tests the pending boundary
without adding an accepted stage.

## Inputs and written bounds

PAY is exact AssetId(local,partial-fill-example,PAY,scale0). State initially
filled0,feesPaid0,grossSpent0,marker77; empty scenario.state selects all four
initializers. No action inputs or lets exist. Opening balances in canonical
account order are maker30,observer9,taker2,treasury1. Opening duties are
unrelated(PAY,observer,treasury)5 and zero(PAY,observer,taker)0, in name order;
delivery is absent. The unused declare action contains exact matching Liability
identities, per the documented boundary-registry premise; it is never executed.

Before fill1, written requires check filled+4<=10,feesPaid+1<=3,
grossSpent+5<=13. Before fill2 they check filled+6<=10,feesPaid+2<=3,
grossSpent+8<=13. Each source counter update occurs after its financial effects.
Final ensures filled==10 is checked at its written position. These are finite
local source rules over the supplied state, not an authenticated signed policy.

## Source ordinals, expression allocation and Plan

Fill has21 statements. Indices0/1/2 are its first three Requires;3 creates
delivery10;4 transfers4 to taker;5 pays fee1;6 discharges4;7/8/9 update
filled/feesPaid/grossSpent;10 is await finish(ok:Bool) when ok;11/12/13 are the
second three Requires;14 transfers6;15 pays fee2;16 discharges6;17/18/19 update
the counters;20 is Ensures. Declare has two Liability statements, not part of
selected runtime history. All23 source statements remain in the full Program.

Under adopted M01, state initializers and all metadata allocate no nodes. The
fresh global arena has61 nodes. Fill statement expression IDs/ranges are:

| Statement | Nodes in left-to-right postorder | Root |
| ---: | --- | ---: |
| 0 | 0filled,1literal4,2Add,3literal10,4Le | 4 |
| 1 | 5feesPaid,6literal1,7Add,8literal3,9Le | 9 |
| 2 | 10grossSpent,11literal5,12Add,13literal13,14Le | 14 |
| 3/4/5/6 | literal nodes15/16/17/18 | 15/16/17/18 |
| 7 | 19filled,20literal4,21Add | 21 |
| 8 | 22feesPaid,23literal1,24Add | 24 |
| 9 | 25grossSpent,26literal5,27Add | 27 |
| 10 | 28Nameok | 28 |
| 11 | 29filled,30literal6,31Add,32literal10,33Le | 33 |
| 12 | 34feesPaid,35literal2,36Add,37literal3,38Le | 38 |
| 13 | 39grossSpent,40literal8,41Add,42literal13,43Le | 43 |
| 14/15/16 | literal nodes44/45/46 | 44/45/46 |
| 17 | 47filled,48literal6,49Add | 49 |
| 18 | 50feesPaid,51literal2,52Add | 52 |
| 19 | 53grossSpent,54literal8,55Add | 55 |
| 20 | 56filled,57literal10,58Eq | 58 |
| declare0/1 | literal0 nodes59/60 | 59/60 |

No repeated occurrence is interned or rebased across statements/actions. The
sole Await site is1 at original ordinal10, binder ok:Bool, captures[]. State
counters are read from the current boundary, not captured; the pending binder
is excluded. Site1/captures[] is a positive pending frame, distinct from idle0.

Local work uses dispatch1+distinct visited nodes, fresh cache per statement.
Start's first three statements cost6 each; Liability/Transfer/Fee/Discharge cost2
each; three Nexts cost4 each; reaching await costs entry1 without guard evaluation:
18+8+12+1=39. True resume costs guard2, Requires18, financial6, Next12, Ensures4:
2+18+6+12+4=42. Plan fill.segment_cost=[39,42],committed_before=[0,39],total81.
Unused declare has one cost4 segment. False finish attempts2 and keeps39
committed; three invocations attempt39+2+42=83 while successful-prefix work is81.
Per-node details are literal WORK-DETAILS.json, not observed meter output.

M03 canonical Program digest is `fd962c266aaa5a660839e0a0a88a37901a39400473efa10157b7d1abfaa29a34`.
Filled Plan digest is `2c9eb53ae5ad63b205873d383d42d7798382e6bd7e414a9ab7938b294eafd3ee`. Complete canonical preimages are
retained: lexical object keys, preserved array order, compact UTF-8 JSON, no
prefix or trailing LF. The Plan includes the complete Program digest and both
action costs. This applies the separately owner-adopted observational clause;
no cryptographic authentication/native identity or actual correspondence follows.

## Fourteen complete snapshot literals

Every snapshot preserves marker77, observer balance9, unrelated5 and zero0.
The complete source-specific state and balances are:

| Snapshot | filled | feesPaid | grossSpent | maker | taker | treasury | delivery |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| S0 | 0 | 0 | 0 | 30 | 2 | 1 | absent |
| S1 creation | 0 | 0 | 0 | 30 | 2 | 1 | 10 |
| S2 first payment | 0 | 0 | 0 | 26 | 6 | 1 | 10 |
| S3 first fee | 0 | 0 | 0 | 25 | 6 | 2 | 10 |
| S4 first discharge | 0 | 0 | 0 | 25 | 6 | 2 | 6 |
| S5 | 4 | 0 | 0 | 25 | 6 | 2 | 6 |
| S6 | 4 | 1 | 0 | 25 | 6 | 2 | 6 |
| S7 first pending | 4 | 1 | 5 | 25 | 6 | 2 | 6 |
| S8 final payment | 4 | 1 | 5 | 19 | 12 | 2 | 6 |
| S9 final fee | 4 | 1 | 5 | 17 | 12 | 4 | 6 |
| S10 final discharge | 4 | 1 | 5 | 17 | 12 | 4 | present0 |
| S11 | 10 | 1 | 5 | 17 | 12 | 4 | present0 |
| S12 | 10 | 3 | 5 | 17 | 12 | 4 | present0 |
| S13 complete | 10 | 3 | 13 | 17 | 12 | 4 | present0 |

Balance sums are42 throughout as static specimen arithmetic. Account order is
maker,observer,taker,treasury; liability order is delivery,unrelated,zero when
delivery exists. Creation is absent→10 even though its effect before is0;
final delivery0 and the unchanged incoming zero0 are both present rows. The
first accepted prefix retains delivery6 plus unrelated5 in residual. Completion
retains unrelated5 only; neither zero row is removed from its complete snapshot.

Start emits original Steps0..9; the Await entry is suppressed. Values are three
Booltrue, UInt10/4/1/4/4/1/5. First three snapshots are S0, then S1..S7. Deltas
are Liability0→10,Transfer4,Fee1,Liability10→6,Statefilled0→4,
StatefeesPaid0→1,StategrossSpent0→5. Full start report waits advancedtrue with
continuation.boundary=S7,site1,captures[],capnull,attempted39,committed39.

False finish reports waiting advancedfalse,segmentnull,attempted2,committed39,
with the ENTIRE actual input continuation unchanged and residual6/5. No Step,
effect or synthetic snapshot is emitted. This is not another partial fill.

True finish uses that actual retained continuation. Steps10..20 start with
Await Booltrue at S7, followed by three Requires Booltrue still at S7, then
UInt6/2/6/10/3/13 and final Ensures Booltrue. Snapshots are S7 for the guard and
requires, S8..S13 for financial/state statements and S13 for Ensures. Deltas are
Transfer6,Fee2,Liability6→0,Statefilled4→10,StatefeesPaid1→3,
StategrossSpent5→13. It completes frame1→idle0,attempted42,committed81,residual5,
with advanced/continuation absent. All three root qualification labels remain.

Cumulative maker gross is5 then13, fees1 then3, taker net4 then10. Initial
maker30→17, taker2→12, treasury1→4 and observer9→9 exactly account for them.
Liability evolution is creation10-discharge4-discharge6=present0; unrelated5
survives. The complete13-effect history concatenates the two accepted segments;
it is derived data, not an extra authenticated report member.

## EARS and unapplied test scope

The original MPLR-001/004 local-prefix and two-payment/residual observation is
represented with source-supported composition. MPLR-009 local work/fee arithmetic
and MPLR-017 explicit typed accounting are observable finite data. Their signed,
authenticated, native and target obligations remain unasserted, as do all other
original EARS and fourteen historical gaps. Precise missing interface constructs
are listed in UNSUPPORTED.json rather than implemented by a constant success flag.

The proposed Rust test performs complete Program/Plan/report JSON comparisons,
then checks exact actual continuation preservation and concatenated actual effect
history. It never feeds a frozen boundary/continuation back into runtime. Its
source/fixtures remain external and unapplied; no Rust formatter/build/test, CLI,
model, SDK, Node, proof, network or provider action ran. A future integrated
candidate needs root review and separately selected ordinary validation.

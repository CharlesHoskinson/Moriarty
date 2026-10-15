# Independent normal continuation arithmetic oracle

Frozen 2026-10-15T04:06:03.116559+00:00 at baseline `fe01bc8b7d889427192c304b0e75d0ec719f63db`. **Specified-only: proposed semantics are not adopted.** Installed develop/AGENTS/status and baseline source cost contract were read. No compiler, evaluator, parser, build, product, model or native operation was executed. No implementation was inspected or written. Python serialized fixed literal frames, manually counted work and public hashes; it did not interpret the source.

The concrete proposed programs are [escrow /2](oracle-escrow-v2.mori), [contrasting reserve /2](oracle-reserve-v2.mori), and [/1 compatibility](oracle-compat-v1.mori). Their exact byte hashes and all literal snapshots, step indices/values/snapshots, effects, events and capture values are in [ORACLE-CONTINUATIONS.json](ORACLE-CONTINUATIONS.json), SHA-256 `76075b6350f83ba2776a4466ed9eb4dada7e9d1da0c509a366026e6b98ec9640`. An implementation mismatch must be reported rather than changing this oracle to match output.

The original Sol-only proposal remains preserved in `DESIGN-SOL.md`, with its hash and file date in `hypothesis_history`. That unadopted alternative charged false observations into cumulative work: escrow13 and reserve42 with the selected false observations. Root then supplied Opus C2/C3: false observation is noncommitting, the continuation stays identical, there is no false-wait circuit, and committed work follows the fixed successful-segment plan. This oracle explicitly supersedes that arithmetic: escrow committed11/total attempted13; reserve committed36/total attempted42. This change predates implementation/execution and is not tuning to results. Revised Sol agreement and Muse audit remain pending.

## Escrow specimen

`deliver(amount10)` transfers10 alice→escrow, creates release duty10, then awaits Bool delivery. It preserves declared marker77 and observer balance42. Opening balances are alice100/escrow0/bob0/observer42, total142; release is absent.

| Invocation | Exact financial outcome | attempted | committed |
| --- | --- | --- | --- |
| Start | Waiting site1. Transfer10 then Liability release0→10. Alice90/escrow10/bob0; captures `[amount UInt10]`; residual release10 | 5 | 5 |
| Resume ready=false | advanced=false, no segment/effects/new continuation; same boundary, captures and canonical continuation | 2 | 5 |
| Resume ready=true | Complete. Transfer10 escrow→bob then Liability10→0. Alice90/escrow0/bob10; release present-zero; residual empty | 6 | 11 |

Start steps are original ordinals0,1. Completion steps are2 (Await true, unchanged snapshot/no effects),3 transfer and4 discharge. The boundary-await entry is charged when reached but does not appear as an evaluated guard step in the start segment. Both opening and completion retain marker77/observer42 exactly. Gross transfer debits are start10+completion10=20, split alice10 and escrow10; fees0, bob net10, dust0. Liability opening0/creation10/accrual0/discharge10/closing0 present. Pending frame is site1/slot10 at the boundary and site0/slot0 at completion. Conceptual pending fields are separate from the declared Snapshot state; no invented JSON state names are added.

Work is independently counted: start Transfer(1+Name1)+Liability(1+Name1)+await-entry1=5; false resume dispatch1+ready-Name1=2; true resume dispatch1+guard-Name1+Transfer2+Discharge2=6. The successful plan is `[5,6]`, total11, with2 advance circuits and no wait branch. Direct start→true attempts11; one false observation adds2 attempts, leaving committed11. The normal stored cap is11; this oracle supplies no cap-refusal tests.

## Contrasting reserve specimen

`allocate(extra2)` uses a different agreement, asset `local/continuation-reserve-oracle/UNIT/scale0`, party names and arithmetic. It captures `held = basis40 + extra2 =42`, changes basis40→35 before awaiting UInt quote, and accrues an **existing** allocation duty5 by42 to47. A quote43 advances: allocated0→42, Transfer42 reserve→target, Discharge42 reduces duty47→5, and basis35+quote43→78. A second Bool await then permits the ordinary source predicate quote≤50, Fee2 reserve→treasury and UInt Emit settled_quote43.

Opening balances are reserve100/target0/treasury0/observer42, total142. Marker77 and observer42 persist in every frame. `basis` is a scalar planning value, not the token custody balance. Captured held42 must survive the change to basis35; recomputing the let would produce37 and contradict the source semantics. Basis itself is not captured: later state reads use the boundary. The accepted UInt event quote43 must survive to the second await and final Emit/predicate.

| Invocation | Exact state, custody, duty and captures | attempted | committed |
| --- | --- | --- | --- |
| Start | Waiting site1. basis35/allocated0; balances100/0/0; allocation47. Captures `[extra2,held42]`; fixed3-slot frame `[2,42,0]` | 11 | 11 |
| First resume quote41 | false guard41≥42; advanced=false, no segment/effects; quote discarded; same site1 | 4 | 11 |
| First resume quote43 | Waiting site2. basis78/allocated42; reserve58/target42/treasury0; duty5. Captures `[extra2,held42,quote43]`; frame `[2,42,43]` | 15 | 26 |
| Second resume ready=false | advanced=false, no segment/effects; same site2 boundary/captures | 2 | 26 |
| Second resume ready=true | Complete. basis78/allocated42; reserve56/target42/treasury2; duty5 remains residual; Emit UInt43; frame clears to site0/three zeros | 10 | 36 |

The complete ordered effects are:

1. State basis40→35.
2. Liability allocation5→47.
3. State allocated0→42.
4. Transfer CREDIT42 reserve→target.
5. Liability allocation47→5.
6. State basis35→78.
7. Fee CREDIT2 reserve→treasury.
8. Emit settled_quote(UInt43).

Start step ordinals0,1,2 contain Let42 (no effect), state update and accrual. First advance steps3,4,5,6,7 include Await true and the four suffix statements, stopping before await8. Final advance steps8,9,10,11 include Await true, business predicate, fee and Emit. Every intermediate per-step snapshot and effects vector is frozen in JSON.

Normal financial totals are gross debit44=transfer42+fee2, target net42, treasury net2, reserve net−44 and custody remaining56, observer unchanged42; balances stay142. Liability opening5/creation0/accrual42/discharge42/closing5 present. Completion retains that positive residual; it does not assert settlement or discharge it. No rounding dust occurs.

Work by successful segment:

- Start: Let1+(Name basis+Name extra+Add)3=4; Next1+(Name basis+literal5+Sub)3=4; Liability1+held1=2; await-entry1. Total11.
- First advance: dispatch1+quote/held/Ge guard3; Next allocated2; Transfer2; Discharge2; Next basis4; second-await entry1. Total15.
- Final advance: dispatch1+ready guard1; Requires1+(quote+literal50+Le)3=4; Fee2; Emit2. Total10.

Plan costs are `[11,15,10]`; committed-before entry sites0/1/2 are0/11/26, plan_total36, normal stored cap36. Three advance circuits are expected. False approval attempts4 and false settlement attempts2 add no committed work. Direct true/true total attempted36; false/true/false/true total attempted42 with committed36.

A separate normal commercial-predicate refusal uses quote51 (first guard still true). The accepted second boundary has basis86/allocated42/reserve58/target42/treasury0/duty5, captures quote51 and committed26. Resuming ready=true reaches `requires quote <=50` at ordinal9 and rejects with attempted6: dispatch1+ready1+Requires1+quote1+literal50 one+Le1. No segment, Fee, Emit or new continuation is returned; committed26 and the prior site2 boundary remain. This is an ordinary source predicate failure, with no altered continuation or hostile-input/security conclusion.

## Existing /1 compatibility and remaining ambiguities

The actual /1 specimen replaces await by positional `requires ready` and has amount/ready as original action inputs. False yields existing atomic rejection at step2 (`mori::core::requires`, message `Action predicate evaluated to false.`), logical6 and no published execution/effects. True yields the full opening→complete escrow frame/effects, logical10. These counts follow three or five statement charges plus one Name visit per statement. Marker/observer preservation adds no expression visits. Do not reinterpret /1 failure as waiting.

No guessed program/plan digest, future Await ExprId allocation or Residual serialization schema is supplied. Future Program/StagePlan canonical hashes and exact continuation output encoding need the adopted implementation; mathematical named captures, pending slots, residual financial identities and step observations are already frozen. Costs exclude serialization, capture copying, circuit/DUST/service/proof/ledger work. Output qualification/authentication/consumption labels are recorded exactly as Opus supplied, without a new security assessment. No hostile/refusal-mutation tests are authored here.

Source/semantics/cost `/2` and unsigned artifact `/4` remain proposed. This is independent normal numeric evidence preparation, not design adoption, execution approval, native support or U3/MPLR acceptance.

# Independent positive staged expectations, 2026-10-15

Classification: independently authored, source-derived, specified-only. There is no evaluator, parser, compiler, adapter, backend or captured-output execution in this packet. The author did not implement the adapter and did not read the existing continuation normal-oracle or prior captured outcomes. Only the prior `/1` packet's file listing and first typed Program portion were inspected for format. The actual raw programs and scenario/event inputs are byte copies of the current checkout's public examples. All output JSON is constructed from manually written expression IDs, amounts, state values and ordered effect rows in `serialize-oracle-data.py`. That script is a data serializer, not a language interpreter. It never imports Moriarty or infers statements from source.

The falsifiable predicate is complete JSON equality for each expected metadata value and each of the eight successful API invocation outcomes in `INVOCATIONS.json`, against the frozen source and inputs. JSON object member ordering is not the predicate; array ordering, numeric/string representations, exact field membership and all values are. Future assertion code must consume these frozen expected values without regenerating them from actual results. No assertions or language runtime executions occurred before the freeze.

The source baseline is `9c63048a18315d7eb3cf114b983bd68c0d842dc6`, remote `https://github.com/CharlesHoskinson/Moriarty.git`. The local candidate has uncommitted changes. `SOURCE-PINS.json` and `raw-source/` pin actual inspected source bytes, including the current untracked wrapper. This packet neither asserts that those changes are committed nor that the reference is authoritative. Required development status was inspected read-only: historical SP01 loan-swap dependent dispatch remains blocked by stale admissions and absent runtime accounting. No campaign was dispatched. Native authority remains zero.

## Wire and identity derivation

Source facts: `ir.rs:179` declares complete Program fields; `ir.rs:31` makes UInt a canonical decimal string; `ir.rs:49` tags Value by type; `ir.rs:77` tags expressions by kind; `ir.rs:127` tags statements by kind. `parser.rs:244-249` assigns each expression its append index. `parser.rs:252-313` appends left and right operands before their binary operator, with no interning. State initial literals (`parser.rs:78-84`) do not enter the expression array. The two `expected-program.json` files manually enumerate these exact nodes and source statement positions.

`stage/plan.rs:56-105` orders sites by source occurrence, starting at 1. Inputs are captured in declaration order with origin Input and input ordinal. Lets are appended with origin Let and source statement ordinal. Event binders are appended only after their own site layout has been recorded, so they appear at later boundaries with origin Event and their defining await's source ordinal. States are read from boundary state and are not implicitly captured. `stage/mod.rs:92-177` specifies named values, logical work, continuations, frames, segments and the externally tagged StageOutcome enum.

`intent.rs:72-86` specifies recursive object-key sorting and compact UTF-8 JSON for canonical identity bytes; `stage/digest.rs:7-44` hashes canonical Program, then canonical costed StagePlan containing that program hash, using SHA-256. The serializer uses only Python's standard JSON and hashlib over the manually constructed JSON objects to derive these metadata hashes. No semantic result is obtained from an implementation call.

Wrapper `staged.rs:10-23,40-54` supplies metadata field names and the three fixed labels. Its start/resume functions return the upstream StageOutcome intact. Thus `*.expected.json` files use raw API `{"Waiting":{...}}` / `{"Complete":{...}}`, with no Result wrapper. CLI `status` output is a different presentation. The upstream raw enum has no extra qualification fields; metadata and `INVOCATIONS.json` preserve the exact labels `qualification: local-reference-stage`, `authentication: none`, `consumption: untracked`. No change of authority is inferred from a matching result.

## Delivery arithmetic and complete step derivation

Source: `delivery.mori:1-12`, specifically lines 6, 7, 8, 9 and 10 for statement ordinals 0..4. Scenario is amount 10, marker 77, alice 100, escrow 0, bob 0, observer 42, no liability.

The expressions are IDs 0 amount, 1 amount, 2 ready, 3 amount, 4 amount. Each successful ordinary statement or resumed guard costs one dispatch plus one Name visit, hence 2. Await entry costs 1 without inspecting the guard (`stage/cost.rs:70-108`). Start costs 2 + 2 + 1 = 5; resume costs 2 + 2 + 2 = 6; plan total is 11 and committed-before is [0,5].

Start statement 0 moves 10 from alice to escrow: alice 100 - 10 = 90, escrow 0 + 10 = 10. Statement 1 adds release 0 + 10 = 10 owed by escrow to bob. Both return UInt 10. The entry to statement 2 produces no Step and captures only amount=10. Frame moves idle site 0 to site 1. Its continuation stores cap 11, committed 5 and the complete post boundary. Residual contains the release duty 10.

False ready charges 1 dispatch + 1 Name = 2 attempted work, publishes no segment and preserves the whole continuation byte-value, pending frame, boundary and committed=5. True ready subsequently resumes the same site. Its first Step is source ordinal 2 with Bool true, unchanged boundary and no effects. Statement 3 moves escrow 10 - 10 = 0 and bob 0 + 10 = 10, then statement 4 discharges release 10 - 10 = 0. The duty stays present with amount zero (`runtime.rs:835-855`), so residual is empty but the complete snapshot contains release=0. Marker 77 and observer 42 survive every step. Final frame is idle; committed is 11; final invocation attempted is 6. Sequence attempted total is 5 + 2 + 6 = 13, separate from committed 11.

Ordered effects are funding Transfer, Liability 0→10, release Transfer, Liability 10→0. Gross debit is alice 10 and escrow 10; fees are zero; net receipts are alice -10, escrow 0, bob +10, observer 0. Asset total stays 142. Full before/after frames and every pre/post and post-step snapshot are literal values in the expected files.

## Reserve arithmetic and complete step derivation

Source: `reserve.mori:1-21`, specifically lines 8..19 for statement ordinals 0..11. Scenario is extra 2, basis 40, allocated 0, marker 77; reserve 100, target 0, treasury 0, observer 42; allocation already present with amount 5.

The manual expression table is: 0 basis, 1 extra, 2 Add(0,1), 3 basis, 4 literal UInt 5, 5 Sub(3,4), 6 held, 7 quote, 8 held, 9 Ge(7,8), 10 held, 11 held, 12 held, 13 basis, 14 quote, 15 Add(13,14), 16 ready, 17 quote, 18 literal UInt 50, 19 Le(17,18), 20 extra, 21 quote.

Start computes held=40+2=42 at ordinal 0, with no snapshot/effect change; ordinal 1 sets basis=40-5=35 (State effect 40→35); ordinal 2 adds allocation=5+42=47 (Liability effect 5→47). It stops entering approval, ordinal 3/site 1. Captures are extra UInt 2 and held UInt 42; quote is not yet captured. The full boundary still has reserve 100, target 0, treasury 0, observer 42, allocated 0, marker 77, basis 35 and duty 47. Work is 4 for addition Let + 4 for subtraction Next + 2 for liability + 1 await entry = 11. Residual is allocation=47.

False approval quote=41 makes 41>=42 false, costing 1 dispatch + 3 expression nodes = 4 attempted; continuation and committed=11 remain unchanged. True approval quote=43 makes 43>=42 true. Ordinal 3 records Bool true with unchanged snapshot and no effects. Ordinal 4 assigns allocated=held=42 (State 0→42). Ordinal 5 transfers held 42: reserve 100-42=58, target 0+42=42. Ordinal 6 discharges allocation 47-42=5. Ordinal 7 sets basis=35+43=78 (State 35→78). Entry to settlement, ordinal 8/site 2, adds no Step. Captures now are extra 2, held 42, quote 43, in that order. In particular held is not recomputed as 78+2, and quote survives the second boundary. Segment cost is 4+2+2+2+4+1=15; committed becomes 11+15=26. Duty 5 remains residual.

False settlement ready=false costs 2 attempted, preserving continuation and committed=26. True settlement first records ordinal 8 Bool true with unchanged boundary/no effects. Ordinal 9 requires 43<=50, returning Bool true and no effects. Ordinal 10 charges fee 2: reserve 58-2=56; treasury 0+2=2. Ordinal 11 emits settled_quote UInt 43, changing no snapshot. Work is 2+4+2+2=10; committed becomes 36, final frame becomes idle. Allocation remains present at 5 and residual contains its complete identity. Sequence attempted total is 11+4+15+2+10=42, separate from committed 36. Segment vector is [11,15,10], committed-before [0,11,26].

Ordered effects are basis State 40→35, Liability 5→47, allocated State 0→42, Transfer 42, Liability 47→5, basis State 35→78, Fee 2, Emit settled_quote 43. Reserve gross debit is 42+2=44; target receives net 42; treasury receives net 2; reserve ends with 56 unspent atomic units. These 56 units are a source balance, not measured protocol dust. Fees here are the source Fee effect, not service fees or DUST. Liabilities reconcile opening 5 + accrued 42 - discharged 42 = closing 5. Marker 77 and observer 42 are preserved; asset total stays 142.

## Source rules used for complete outcomes

- `runtime.rs:528-659`: scenario snapshots, explicit balances and canonical balance/liability order.
- `runtime.rs:660-699`: source debit and destination credit.
- `runtime.rs:702-869`: statement return values and effect shapes, let/env updates, positional assertions, liabilities, discharge and Emit.
- `runtime.rs:934-1022`: resumed guard Step, original source ordinals, unchanged pre snapshot for guard, await entry without Step, capture collection, segment snapshots/effect order.
- `stage/admit.rs:103-126`: false guard preserves continuation and returns no segment.
- `stage/admit.rs:128-223`: frames, committed prefix, full boundary, residual duties and raw outcomes.
- `stage/mod.rs:183-191`: residual filters positive liabilities; present-zero duties stay in Snapshot.
- `stage/cost.rs:17-108,149-162,235-242`: distinct expression-node counting per statement, await dispatch and prefix work arithmetic.
- `docs/CORE-CONTINUATIONS.md:81-88,112-119,153-183`: independent explanatory language contract consistent with the arithmetic above.

No field is marked uncertain after the above static derivation. This is a proposed complete positive oracle, subject to independent review; that status is not a runtime pass. The packet designs no hostile or security cases and gives no security judgment. Native execution, builds, tests, formatter, K, Node, SDK, proofs, keys, network and ledger commands were not run. All full U/M/MC/SP/G acceptance exits remain outside this packet.

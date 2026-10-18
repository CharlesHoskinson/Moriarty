# Concrete source journeys and required independent expectations

All examples are proposed public inputs, not executed programs or frozen independent expected outputs. The author has inspected current producers/consumers and prior related independent oracle packets; this author-worked arithmetic supplies no new independent value or compiler-wire proof. Source grammar/admission is subject to actual existing stage rules and OD1. Two normal-path examples show why the new production boundary is needed; a third source invocation distinguishes gross economics from later net credits.

## Two distinct successful same-asset programs

`examples/first.mori` creates invoice12, transfers6 alice→bob, pays fee1 alice→treasury, discharges6, advances phase to1 and emits first(phase). `examples/second.mori` has a different state schema and an initializer progress999; its explicit binding maps progress to the retained payment.phase. It requires progress1, transfers3, pays fee2, discharges3, advances progress2 and emits second(progress). Its spare state maps to the retained future.quota9. There is no special compiler rule for either source.

The full opening and production manifest are `opening.json` and `two-programs.plan.proposed.json`. Initial global state is payment.phase0, outside.marker77 and future.quota9. The selected exact asset has alice30, bob0, observer11 and treasury0. Another exact asset with the same reference UNIT has vault17. Outside duty5 is present; invoice.main is absent. Distinct issuers are kept separate throughout. The global state/duty names are explicit composition bindings, not modifications to original source DTOs.

| Actual proposed public observation | Complete selected-asset balances (alice,bob,observer,treasury) | invoice.main | outside.duty | Global state (phase,marker,quota) | alice cumulative gross debit / fees | Local committed / sequence attempted,committed | Ordinary remaining / reserve |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Opening, before any stage call | 30,0,11,0 | absent | present5 | 0,77,9 | 0 / 0 | none / 0,0 | 40 / 7 |
| P complete; Q not started | 23,6,11,1 | present6 | present5 | 1,77,9 | 7 / 1 | P12 / 12,12 | 28 / 7 |
| Q complete | 18,9,11,3 | present3 | present5 | 2,77,9 | 12 / 3 | P12,Q14 / 26,26 | 14 / 7 |

The other exact asset's vault17 remains present at every row. All complete global source-Step heads and ordered effect prefixes, including no-effect Requires and untouched rows, are author-worked in `AUTHOR-WORKED-PREFIXES.json`. Those Step rows are not separate public call publications or per-Step committed counters. Source P has six dispatch+one-root statements, cost12. Q's Requires visits comparison/name/literal plus dispatch, cost4; its other five statements cost2 each, total14. Original local work would remain12 and14; their sequence sum would be26 only after actual successful calls. No real result is asserted.

P's final Complete alone would not be a sufficient generic prefix carrier for awaiting programs, even though P in this first example has one segment. The production Sequence design retains all calls for the general case. Q must read the actual completed P head; an independently supplied Q Scenario could fake the final values and would not satisfy this predicate. Existing Pair cannot express this shared exact-asset successor, and its disjoint rule is not weakened.

## Later credit preserves gross spending

`return-one.mori` transfers1 bob→alice and emits credit(1). `later-credit.plan.proposed.json` appends it as a third invocation through the same generic API. No new state/duty bindings are needed; all existing global state and both duties are its outside frame. The resulting selected-asset balances are alice19,bob8,observer11,treasury3; invoice3/outside5 and marker77/quota9/phase2 remain. Alice gross debit remains12 including fee3, gross credit1 and net decrease11. Bob now separately has gross debit1 and transfer credits9, net increase8. Per-asset transfer volume is10 and fee volume3; the other asset's volume remains0. Source R costs4, so the all-success sequence has attempted/committed30, ordinary remaining10 and reserve7. No refund right, authorized compensation, financial netting optimization or restored spending authority is inferred from this ordinary credit.

## Awaiting predecessor then a distinct successor

`staged-first.mori` is the exact current public TwoDeliveries fixture, copied verbatim from pinned main and covered by SOURCE-PINS provenance through its baseline reference in the review manifest. Its source includes the existing unused duty declaration; that source is not rewritten to fit this example. `staged-second.mori` is a distinct new currently scoped scalar program: it requires its mapped progress2, transfers1, pays fee1, advances progress3 and emits successor(progress).

The complete input is `staged-predecessor.plan.proposed.json`, `staged-opening.json`, the two true event files and empty inputs. Opening selected-asset balances are alice30,bob0,observer11,treasury0; mapped invoice.main5 and outside.duty5 are both present. The unselected second asset/state rows remain. Calls are exactly P start, P approval resume, P receipt resume, Q start; there is no Q start while P waits.

| Real-call boundary to be independently specified | Selected balances (alice,bob,observer,treasury) | phase | P/Q local committed | Sequence attempted,committed | Original call/history count | Cumulative effects |
| --- | --- | --- | --- | --- | --- | --- |
| Opening | 30,0,11,0 | 0 | none | 0,0 | 0 | 0 |
| P start/wait approval | 23,7,11,0 | 1 | P5 | 5,5 | 1 | 2 |
| P approval/wait receipt | 20,10,11,0 | 1 | P10 | 10,10 | 2 | 3 |
| P complete, before Q | 20,10,11,0 | 2 | P16 | 16,16 | 3 | 5 |
| Q complete | 18,11,11,1 | 3 | P16,Q12 | 28,28 | 4 | 9 |

Both duties remain5 and the entire outside frame remains equal at every row. Alice final gross debit12 includes fee1; bob net receipts11; ordinary remaining12 and reserve7. P's original waiting frames/captures and all three original outcomes must remain in the final history even though the latest P outcome is Complete. Q's duty view is empty while both whole global duties remain present outside it; a global absence or liability erasure claim would be false. This is a full global observation requirement, not a change to the current local duty/R7 rules. The numbers here are author arithmetic, not new golden output. No concrete false-event golden is invented.

## Meaningful proposed consumer predicates

All names below are future predicates, not tests available at main79de664a. Exact commands must be inspected/pinned by root after adopted source exists; no execution is authorized by this document.

1. `mori-core` integration test `completed_successor_preserves_complete_financial_frame`: load both example sources with actual `stage::load`, compare both **complete** Program and filled StagePlan objects to independent expectations, drive real Sequence plan/begin/next and compare every complete root/call/Step/global-prefix/financial/work field. Require Q.projected_pre to be the explicit projection of the actual P-complete head and require all original nested reports. Expected files do not create that runtime head.
2. Public `mori` test `public_compose_observes_two_sequential_programs`: spawn actual `CARGO_BIN_EXE_mori --color never compose <plan> --json`, require the successful local result, compare the complete report JSON and consumer-specific bytes including LF, and expand all call/effect prefixes against complete independent literals. This verifies the new production public command, not a library wrapper.
3. Public adapter test `public_sequence_test_observes_the_same_complete_run`: invoke actual `mori test` on a sequential-proposed manifest using the identical plan and independent observations. Compare the complete aggregate and all actual production run data. The new consumer path must be observed; existing disjoint passing checks and PR228's one-marker mismatch are not relabeled as this result. No new mismatch test is needed merely to repeat PR228's predicate.
4. `later_credit_preserves_original_gross_debits`: execute the three-invocation production plan and compare the entire independently expected report, including all preserved rows, original tables and distinct debit/credit/fee/duty/work totals. This exercises the same generic interface with a third different program and a financial outcome distinguishable from net balance alone.
5. `completed_awaiting_predecessor_retains_all_original_segments`: execute the concrete staged plan through actual public compose; compare its full original P start/resume/resume history, complete frames/captures/source Steps and Q start from the real completed head. Compare every whole global snapshot/prefix and all original counters. A future normal nonadvancing input must have a separately frozen whole oracle before being selected; no error/security control is authored here.

These positive predicates distinguish source/IR observation, actual production CLI and test-aggregate observation. No proof/ledger/private/target/native result follows. OD rules must be settled before dependent implementation/checks.

## Exact oracle custody required before execution

After SC adoption and applicable OD rules, send an input-only packet containing all source/opening/input/event/manifest bytes, adopted ordinary rules, full current embedded DTO contracts and exact authored outer codec. Exclude producer, test, author-worked prefixes, expected outputs and actual results. A fresh custodian derives financial/work/source/IR/trace values without production evaluation, declares prior source/related-oracle and authored-codec exposure, and freezes the artifacts before author/consumer use.

For the primary two-program plan the required complete subjects are `first.program.json`, `first.plan.json`, `second.program.json`, `second.plan.json`; `opening.observation.json`, `p-complete.observation.json`, `q-complete.observation.json`; `compose.report.json`, `compose.stdout.bin`, `compose.stderr.bin`; and `mori-test.aggregate.json`, `mori-test.stdout.bin`, `mori-test.stderr.bin`. Existing hashes may require the corresponding original four Program/Plan canonical preimages and digest inventory from a separately owner-specified exact identity contract. No new sequence preimage is invented.

For the later-credit plan add `return-one.program.json`, `return-one.plan.json`, `later-credit.observation.json`, its complete report/public bytes and corresponding preimages if that identity predicate is selected. For the staged plan freeze its two complete Programs/Plans, five complete observations (opening plus four calls), complete root/CLI bytes and exact original Program/Plan preimages if selected. Raw call outcomes, Step fields, lifted whole Step snapshots, tables, original local/global frames and every effect attachment are included in those full objects; none may be omitted as derivable merely to make a report smaller.

All observations require exact local/global presence and keys, full Account/AssetId/duty identities, caps and cumulative work, current latest versus whole history, complete prefix lengths, authored table indexes and per-consumer byte rendering. Existing disjoint oracle freeze remains its own conditional finite scope; it cannot supply these new sequential values or outer byte expectations. This proposal's authored fields are not an independent compiler-wire proof. No independent sequential oracle is frozen yet.

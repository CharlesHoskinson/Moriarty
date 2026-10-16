# Independent Core/2 normal financial oracle derivation

Status: specified-only/unexecuted. Independent Oracle O owns this packet.
No source in the checkout or prior scalar-oracle evidence was changed.
The 22 existing normative and representation sources were copied and hash-pinned
in `NORMATIVE-FREEZE.json` before these literals were derived. No candidate
financial projection body, evaluator/compiler output or new capture was read.
The author supplied field shapes only and asked not to receive expected values.

## Predicate and representation

The concrete future predicate is: the same admitted production consumer
`mori_core::stage::load`, then `stage::start/resume`, produces complete
observations equal to every independently frozen literal in `expected.json`.
Matching a final amount alone fails that predicate. Root owns the later binding
of this data into expected typed financial views. Expected values must not be
constructed by projecting the evaluator or candidate output.

`inputs.json` selects two new source programs, exact scenarios, advisory start
caps and ordered events. Each event's `continuation_from` names the preceding
Waiting result, including an ordinary false observation. Individual source,
scenario and event files are real producer inputs, not descriptive placeholders.

`expected.json` contains full manually transcribed source IR, source hash,
canonical program and plan digests, complete StagePlan fields, every start
outcome, and every resume input/event/outcome. Nested Execution retains original
statement IDs, full Statement including expression IDs, typed observed values,
complete pre/post and per-step financial snapshots, ordered local effects and
ordered aggregate effects. Captures, frame pre/post, continuation boundaries,
logical cap/attempted/committed work, present-zero duties and positive residuals
are all literal. The existing externally tagged StageOutcome serde shape is a
fixture representation, not a new borrowed-view wire ABI. Existing CLI labels
are preserved as separate metadata; they are not invented fields on typed views.

Repository observation from the pinned guide/types: state maps contain only
declared state; action inputs and lets are captured by value. Accepted event
binders join captures at later awaits. Await entry ends a segment before guard
evaluation; its entry charge creates no Step. Resume's successful guard creates
one Step at the original await index with the unchanged boundary snapshot.
False guard returns the same continuation, no segment, and separate attempted
work. All positive liabilities appear in `residual`; a zero row remains in the
boundary but is omitted from positive-only residual.

Every Step snapshot/effect inside a segment describes that segment's local
progress. It does not independently commit a prefix. Successful Waiting/Complete
publication yields the local successful segment. Prior published boundaries
are separate from a later segment's tentative effects. No rejected suffix,
hostile case or invented failure state is authored here; those cases remain
with Opus. Technical proof cuts do not publish financial commits.

## F01: InstallmentInvoice

Independent input: invoice40, upfront15, state paid0/marker=true. Exact PAY asset
identity is the declared fictional tuple oracle-local / financial-core2-oracle /
PAY / scale0. Initial balances are buyer100, collector3, observer17, vendor5;
existing invoiceDue5 has buyer/vendor identity. The initial duty is intentional:
completing this action must preserve that opening5 rather than erase it.

The start first adds invoice40, producing duty45. It transfers15 to vendor,
charges fee2 to collector, and discharges15. Thus its boundary has buyer83,
collector5, observer17, vendor20; duty30; paid15. The computed remaining25 is
captured alongside invoice40/upfront15. The guard has not run at this point.

A false accepted event leaves that exact boundary/capture/cap/committed-work
object unchanged. A later true event transfers25, charges fee1, discharges25,
sets paid40, emits remainingDue5, and passes the positional paid==invoice
assertion. Closing buyer57, collector6, observer17, vendor45; duty5 remains
positive and is reported as residual. Completion is not settlement of the
opening duty. Marker and observer balance are preserved at every snapshot.

| Original statement | Root ID / node count | Observed value | Cost when executed | Ordered effect |
| --- | --- | --- | ---: | --- |
| 0 liability invoiceDue | 0 / 1 | UInt40 | 2 | invoiceDue5→45 |
| 1 transfer upfront | 1 / 1 | UInt15 | 2 | buyer→vendor15 |
| 2 fee | 2 / 1 | UInt2 | 2 | buyer→collector2 |
| 3 discharge upfront | 3 / 1 | UInt15 | 2 | invoiceDue45→30 |
| 4 next paid | 6 / 3 | UInt15 | 4 | paid0→15 |
| 5 let remaining | 9 / 3 | UInt25 | 4 | none |
| 6 await acceptance | 10 / 1 | true on advance | 2 on resume; entry1 | none |
| 7 transfer remaining | 11 / 1 | UInt25 | 2 | buyer→vendor25 |
| 8 fee | 12 / 1 | UInt1 | 2 | buyer→collector1 |
| 9 discharge remaining | 13 / 1 | UInt25 | 2 | invoiceDue30→5 |
| 10 next paid | 16 / 3 | UInt40 | 4 | paid15→40 |
| 11 emit remainingDue | 17 / 1 | UInt5 | 2 | emit UInt5 |
| 12 ensures | 20 / 3 | true | 4 | none |

Source-index expressions are appended after children, with no cross-statement
sharing introduced by this transcription. Start work is
`2+2+2+2+4+4+entry1=17`. True resume work is
`2+2+2+2+4+2+4=18`. Plan total35, committed-before vector[0,17].
False guard attempted work2 does not change committed17. It is still reported
as attempted work and is not erased; cumulative invocation attempts across the
chosen sequence are37, while the successful committed prefix costs35.

Complete financial accounting: gross buyer debit43 includes source fees3;
vendor transfer receipts40 and collector fee receipts3; observer unchanged.
Duty equation: opening5 + created40 − discharged40 = closing5. `accounting.json`
retains both segment and cumulative annotations separately from the projection.

## F02: ConditionalGrantTranches

Independent input: first12, second18, state allocated6/marker=false. CREDIT and
OPS are distinct declared exact assets under the same fictional input domain;
neither identifies a live target. Initial CREDIT balances: observer11,
recipient1, vault90. Initial OPS balances: auditor4, observer13, vault9. No duty
row exists initially. The two asset/account arrays are in canonical order.

Start saves opening6 by value, creates grantDue30, transfers12 CREDIT and
discharges12, sets allocated18, then waits before review. First boundary:
CREDIT vault78/recipient13/observer11; all OPS unchanged; duty18. Captures are
first12, second18, opening6, in source order.

A false review with supplement2 is an ordinary not-ready event. Both sides of
`approved and supplement==2` are evaluated under this profile. It has no segment
or committed financial/work delta. A true review captures approved=true and
supplement2, adds2 to grantDue (18→20), charges1 OPS to auditor, sets allocated20,
then waits before release. Second boundary preserves CREDIT78/13/11, has OPS
vault8/auditor5/observer13, and duty20. Prior accepted event fields are in the
second frame's capture layout after the inputs and saved let.

A false release similarly leaves that complete continuation unchanged. A true
release transfers second+supplement20 CREDIT, discharges20, adds second18 to the
already updated allocated20, emits the saved opening6, emits dutyPresent=true,
and passes the positional equality38==6+12+2+18. Closing CREDIT vault58,
recipient33, observer11; OPS vault8, auditor5, observer13; allocated38/marker=false.
GrantDue is PRESENT with amount0, and positive residual is empty. A source read
of liability_present distinguishes this from the initially absent row.

| Original statement | Root ID / node count | Observed value | Cost when executed | Ordered effect |
| --- | --- | --- | ---: | --- |
| 0 let opening | 0 / 1 | UInt6 | 2 | none |
| 1 liability first+second | 3 / 3 | UInt30 | 4 | absent grantDue→30 |
| 2 transfer first | 4 / 1 | UInt12 | 2 | CREDIT vault→recipient12 |
| 3 discharge first | 5 / 1 | UInt12 | 2 | grantDue30→18 |
| 4 next allocated | 8 / 3 | UInt18 | 4 | allocated6→18 |
| 5 await review | 13 / 5 | true on advance | 6 on resume; entry1 | none |
| 6 liability supplement | 14 / 1 | UInt2 | 2 | grantDue18→20 |
| 7 fee OPS | 15 / 1 | UInt1 | 2 | OPS vault→auditor1 |
| 8 next allocated | 18 / 3 | UInt20 | 4 | allocated18→20 |
| 9 await release | 19 / 1 | true on advance | 2 on resume; entry1 | none |
| 10 transfer second+supplement | 22 / 3 | UInt20 | 4 | CREDIT vault→recipient20 |
| 11 discharge second+supplement | 25 / 3 | UInt20 | 4 | grantDue20→0, row retained |
| 12 next allocated | 28 / 3 | UInt38 | 4 | allocated20→38 |
| 13 emit openingAllocation | 29 / 1 | UInt6 | 2 | emit UInt6 |
| 14 emit dutyPresent | 30 / 1 | true | 2 | emit true |
| 15 ensures | 39 / 9 | true | 10 | none |

Start costs `2+4+2+2+4+entry1=15`. Review advance costs
`guard6+2+2+4+entry1=15`. Release advance costs `guard2+4+4+4+2+2+10=28`.
Plan total58, committed-before[0,15,30]. False review/release attempts are6 and2,
with committed15 and30 unchanged. Across the chosen invocation sequence,
attempts total66; successful committed semantic work is58. These are distinct
quantities and neither is native fee/circuit/DUST cost.

Financial totals: CREDIT gross vault debit32, transfer receipts recipient32;
OPS gross vault debit1 is the source fee1, with auditor receipt1. No refund
reduces these gross debit totals. Both observer balances and marker are
preserved. Duty equation: absent opening0 + created/accrued32 − discharged32 =
present closing0. There is no mint/burn; each asset's total is preserved in each
literal step snapshot. Allocated is a source bookkeeping state; its values are
not a token balance or fee reserve.

## Identity and validation

Program IR, StagePlan, all expression IDs, capture layouts and source indices
were transcribed manually. Program and plan digests use only Python SHA256 over
those literal JSON objects with recursively sorted keys and compact UTF-8 JSON,
matching the existing declared canonical encoding. This is data hashing, not
source parsing, compilation, evaluation or expected-from-actual generation.

`validate_literals.py` only parses supplied literal JSON and checks hashes,
record linkage, statement identity, canonical field ordering, preserved asset
totals, accounting arithmetic, copied false continuations and explicit work
sums. It imports no subject code and implements no expression evaluator or
source transition engine. Its successful exit establishes data consistency
only, not acceptance of these sources by any executable consumer.

There are2 programs,8 invocation expectations,3 ordinary false observations,
5 published successful segment expectations,29 complete Steps and22 ordered
effects. All3 distinct pending continuations, all5 resume input/event pairs and every
successful pre/post and per-step Snapshot are explicit data. No scalar N01–N08
or S01–S02 files were edited or repurposed.

## Precise boundaries retained

The represented source subset is scalar UInt/Bool, typed state/input/let/event,
ordered transfer/fee/liability/discharge/next/emit/ensures and top-level awaits.
Existing Core/2 grammar and type rules describe all chosen constructs; actual
admission remains UNRUN. No unsupported construct was silently lowered into
host arithmetic or replaced. If future source admission/lowering refuses a
chosen case, retain that exact boundary and these frozen literals; do not
weaken it into a different program or infer a pass.

The immediate missing observation is admitted production Core/2 execution of
these exact source inputs. Further compiler/source-to-Sherlock/target lowering,
full grammar coverage and universal financial control lowering are separately
unobserved. An independently matching local diagnostic projection would not
complete the Phase7 `financial_compiler_projection_matches` predicate or any
U/M/MC/SP/G exit by itself.

Preparation/encoding work, host resources, Sherlock A/B/helper counters,
circuit/witness cost, DUST, service/proof fees, recovery reservations and target
identity are explicitly unmeasured/null with reasons in `expected.json`.
Source fee1/2 figures are known financial effects and are not renamed native
fees. Existing local qualifications remain exact: local-reference-stage,
authentication none, consumption untracked. Native authority used is0.
Security policy, hostile cases, theorem and review judgments remain Opus-owned.

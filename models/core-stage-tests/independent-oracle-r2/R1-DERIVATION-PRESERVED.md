# Independent normal oracle r1 — partial freeze

Owner: `/root/mori_test_stages_oracle`, independently assigned normal expectation author. Checkout HEAD: `9a7c042d3546331f6529e72f768af996bfdad6f3`. These files contain source-derived expected data, never an observed result. All Moriarty execution and comparisons remain **UNRUN**. Native authority remains **0**.

## Admissible inputs and actual calls

The only input fixture bytes read were the eight leaves listed in `input-sha256.json`. No implementation-author expected reports, preimages, expectation-freeze, new stage-test implementation or WORK_REPORT were read. No private Fenrir/Clean state, environment or credentials were read. No Cargo, Lake, K, Quint, Lean, SDK, parser, evaluator or test was run/imported. `assemble_literal_oracle.py` only assembles the explicit literals authored here and uses Python standard-library JSON/SHA256.

Root supplied an input-only public manifest projection, with all expectation fields removed, SHA256 `09848058f2d1bf32d80d6176ae764b48fd551f6aeaa2c3a676a37d8447bd1ee2`. The exact call premises are `profile:moriarty-tests/2`; case `two-deliveries`, source `two-deliveries.mori`, scenario `two-deliveries-scenario.json`, action `deliver`, start logical cap `"16"`, events `approval-false.json`, `approval-true.json`, `receipt-true.json` in that order; case `quoted-fee`, source `quoted-fee.mori`, scenario `quoted-fee-scenario.json`, action `charge`, start logical cap null, event `quote-true.json`. Resume CLI caps and optional action assertions are absent. Each event uses the immediately previous waiting report's continuation. No copied or observed continuation supplied expected data.

## Derivation

TwoDeliveries transfers 7, sets phase to 1, then stops before the approval guard. The first boundary is alice13/bob7/observer11/treasury0 and phase1. Approval false attempts only dispatch plus the name-node guard (2); its continuation is unchanged and no segment exists. Approval true dispatches that guard (2), transfers3 (2), then pays the next await entry (1). The second boundary is alice10/bob10/observer11/treasury0 and phase1, with accepted event `ok:true` captured by value. Receipt true dispatches its guard (2), sets phase2 (2), emits done2 (2), then becomes idle. Successful segment costs are 5/5/6 and cumulative committed work5/10/16. Transfer is separate from debt discharge: duty5 remains present throughout, with creation/accrual/discharge0. No fee/mint/burn/reserve effect occurs.

QuotedFee captures input amount4, evaluates payable=4+1=5 once (one dispatch plus three distinct expression nodes=4), then pays await entry1. The waiting boundary retains marker9/total0 and observer11/payer20/treasury0. The quote guard true costs2; fee5 costs2 and moves payer20→15/treasury0→5; next total5 costs2; emit fee5 costs2. Successful segment costs5/8 and committed5/13. The input and let captures survive the boundary by value. Marker9, observer11 and duty3 remain present. No liability creation/accrual/discharge occurs. Fee5 is included in gross debit5; treasury net credit5. No mint/burn/reserve effect occurs.

All supplied balances sum to31 before and after each stage. Positive liabilities remain in every residual list. Reserve and rounding dust0 here mean no such effect appears in either source; this does not posit a funded recovery reserve. Circuit cost, DUST, service charges, recovery reservation and wall-clock/allocation costs are unmeasured.

## Explicit serialization premises

`source-premise-sha256.json` pins the original files used. The inspected files were unchanged in this checkout when inspected.

| Source | Fields/premises used |
|---|---|
| `crates/mori-core/src/ir.rs:13–41,69–269` | UInt decimal strings, tagged Value/Expr/Statement/Effect, Program/State/Input/Asset/Snapshot/Step/Execution fields; `Step.value` is Option<Value> |
| `crates/mori-core/src/parser.rs:60–67,244–313,316–343` | Root-authorized static AST allocation inspection only: state initial literals do not enter expression arena; each expression occurrence appends; binary children precede parent; no interning; source statement order retained |
| `crates/mori-core/src/intent.rs:72–90` | Recursively lexically sorted object keys, array order retained, compact UTF-8 JSON, no newline in canonical preimage |
| `crates/mori-core/src/stage/digest.rs` | SHA256 canonical Program; SHA256 costed StagePlan with populated program_hash |
| `crates/mori-core/src/stage/plan.rs` | AwaitSite/CaptureSlot/ActionPlan/StagePlan serialization; input ordinals, source statement ordinals, Input/Let/Event capture order |
| `crates/mori-core/src/stage/cost.rs` | Original static successful cost schedule and dispatch definition; one dispatch plus distinct expression nodes, entry await1, guard belongs to resume |
| `crates/mori-core/src/stage/mod.rs:26–36,92–185` | Exact labels, continuation/frame/logical fields, residual positive liabilities, Waiting/Complete report data |
| `crates/mori/src/commands/simulate.rs:152–199` | Public JSON object field selection for Waiting/Complete, every stage label inserted, decimal attempted/committed |
| `docs/CORE-CONTINUATIONS.md:110–118,153–184,210–213` | False observation unchanged continuation/no segment, lexical captures, state from current boundary, residual duties retained, logical profile, original statement indices |
| `crates/mori-core/README.md` | Sequential `next`, fee/transfer amounts and effects, complete snapshots, canonical balance/liability order, positional assertions |
| Root-supplied original comment at `runtime.rs:981` | “Entry charge is attempted now; guard and its Step belong to resume.” No evaluator loop logic was read. This establishes no entry-await Step and a resumed guard Step. |

AST expression tables are literal independent reconstructions in the two program preimages. TwoDeliveries nodes0..6 are UInt7, UInt1, name ok, UInt3, name seen, UInt2, name phase. QuotedFee nodes0..6 are name amount, UInt1, Add(0,1), name agreed, and three separate occurrences of name payable. Plan preimages bind the independently derived costs, ordinals and capture layouts. SHA256 alone was calculated mechanically; neither parser nor compiler calculated these ASTs/plans.

## GAP-TRACE-1: complete expected report remains blocked

The inspected public types/contracts do not specify the exact Statement→Step.value mapping for Let/Await/Transfer/Fee/Next/Emit. Option<Value> alone does not state when null is serialized. The evaluator loop was intentionally not inspected, invoked or imported. Twelve Step.value fields in the five segment-bearing report drafts therefore contain an explicit `$gap` marker. These files are **partial report drafts**, not valid public report fixtures, and must not be compared or integrated as complete independent expectations.

The false-approval report has no segment and contains no unresolved field; its `.partial.json` name retains the packet-wide incomplete status. All other report fields, ordered steps/statements/snapshots/effects, continuation captures/boundaries, counters, canonical preimages and digest values are independently authored. Root will request an independent trace-format contract from the designated owner. No expectation may be filled by observing implementation output.

## Evidence and acceptance

Falsifiable predicate: for each exact call above, a fresh public staged report equals a complete independent expected report in every field, without normalization. Status: **UNRUN; oracle incomplete** due to GAP-TRACE-1. Source/data authorship is the only completed activity. No source review, ordinary result review, security judgment, theorem, financial acceptance, proof/circuit correspondence, native or target qualification, resource amendment or roadmap gate closure follows.

Read-only startup plugin status retained its existing historical blockers: SP01 source bindings stale, current accounting missing and resource live state unavailable. No campaign dispatch, counter reset or retry was performed.

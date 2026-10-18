# Local Core/2 staged tests

The additive `moriarty-tests/2` manifest runs existing `moriarty/2` programs
through the ordinary Rust `mori test DIRECTORY` command. It calls the actual
Core/2 `stage::load`, `stage::start` and `stage::resume` APIs. The language,
semantics and continuation formats are unchanged. `moriarty-tests/1` keeps its
existing atomic consumer and report format.

Public CLI builds, command execution, CLI tests and CLI actual-output
comparisons remain **UNRUN**. The shared report-library extraction below has
its own narrower, initially unrun check; it does not qualify these CLI paths.
The checked-in
[examples](../models/core-stage-tests/mori-tests.json) now name six complete
normal report candidates copied exactly from the separately authored
[R3 freeze](../models/core-stage-tests/independent-oracle-r3/INDEPENDENT-FREEZE.json),
together with fresh complete program/plan preimages. This source/data integration
is not a runtime comparison or an admission. Review dispositions are recorded
in the [header-correction audit binding](audits/2026-10-16/core-staged-tests-r3/header-correction-r1/REVIEW-BINDING.json)
for the PR75 head before this documentation cleanup. Its source/draft-only
dispositions do not approve these changed documentation bytes, ready-for-review,
merge or execution. The [earlier binding](audits/2026-10-16/core-staged-tests-r3/REVIEW-BINDING.json)
retains its historical candidate scope. This cleanup requires fresh applicable
reviews and a separate execution-admission decision before ready-for-review or merge.

R3 sources append unused matching-duty `declare` actions after their target
actions; target deliver/charge code and incoming duty5/duty3 scenarios remain
unchanged. Historical author/R1/R2 positive expectations and data approvals were
invalidated by the omitted R7 registry premise and remain preserved in
[blocked history](../models/core-stage-tests/history/blocked-r2/) and the r2
capture. The unchanged root `expectation-freeze.json` describes archived old
bytes, not current root reports/preimages. See
[fixture provenance](../models/core-stage-tests/PROVENANCE.md) for current full
program/plan digests, disclosed representation/admission premises, historical
C1–C6/N1–N6 limits and all fourteen open draft gaps.

`mori test` requires an explicit directory and opens only that directory's
`mori-tests.json`; it does not discover manifests recursively. Selecting
`models/core-stage-tests` uses the current root manifest. The public staged-test
regression copies only root regular files and skips subdirectories, including
`history/blocked-r2`. An explicit selection of `history/blocked-r2` would read its
preserved historical manifest; those invalidated positive expectations remain
unusable for current validation. This is a source-inspection finding; no command
or test run established it.

A directory contains `mori-tests.json`:

```json
{
  "profile": "moriarty-tests/2",
  "cases": [{
    "name": "two-deliveries",
    "source": "two-deliveries.mori",
    "action": "deliver",
    "scenario": "two-deliveries-scenario.json",
    "logical_cap": "16",
    "expected": "two-deliveries-start.json",
    "events": [
      {"event": "approval-false.json", "expected": "two-deliveries-observed.json"},
      {"event": "approval-true.json", "expected": "two-deliveries-approved.json"},
      {"event": "receipt-true.json", "expected": "two-deliveries-complete.json"}
    ]
  }]
}
```

All keys shown are required. `logical_cap` is a canonical unsigned decimal
string or explicit `null`. Paths resolve from the manifest directory using the
existing test-command path convention.
paths are not confined to the directory; treat the manifest as trusted input.
There are 1–256 cases with unique,
nonempty names and at most 255 events per case, giving at most 256 invocations
including start. An empty event list tests the start result alone. Each case
starts independently from its scenario.

Under the existing R7 source check, liability identities at an awaiting start
and at a resume boundary must match an exact identity registered from a
`Liability` statement across the program's actions. The examples' unused
`declare` actions provide those matching identities without being executed by
the selected deliver/charge trajectories. Declaration presence alone implies
neither liability creation nor discharge. This is the source premise identified
in the final Opus finding; no runtime pass is claimed.

The start report is compared to `expected`; each ordered event resumes the
actual pending continuation and compares its report to that event's expected
file. Expected values never provide runtime state or continuations. A false
guard retains the actual pending continuation. A rejected resume also retains
its actual input for a later explicitly listed event. After a complete result,
or a rejected start, a further event is a command error because no actual
pending continuation exists.
A rejection before resume admission (event read/decode) reports attempted 0 and committed 0, as public `simulate` does; the retained continuation's committed work is unchanged.

Each expected file contains the **entire** report envelope printed by public
`mori simulate` for that invocation. Waiting reports include status, advanced,
segment (including explicit null), continuation, attempted/committed logical
work and residual liabilities. Complete reports include the full segment,
work and residuals. Rejected reports include the complete error and work.
Every actual report retains these literal qualifications:

```json
{"qualification":"local-reference-stage","authentication":"none","consumption":"untracked"}
```

Segments include action, pending frames, semantics, pre/post snapshots, ordered
steps with serialized statements and expression IDs, and ordered effects.
Continuations include program/plan hashes, site, ordered captures, logical cap
and committed work, and the whole boundary. Supply those fields explicitly;
there are no wildcards, omitted-field defaults, generated expectations or
selected-field comparisons. Object key order and JSON whitespace do not affect
equality. Array order, absent versus present fields, explicit nulls, values and
all additional fields do affect equality. Unknown manifest fields are errors;
additional expected-report fields cause a comparison mismatch.

Manifest, source, scenario and expected files retain the existing 1 MiB reader
ceiling and strict JSON parser. Event files use the existing strict Core/2
64 KiB event decoder. The harness passes actual typed continuations in memory;
it does not read or write continuation files or record their consumption.
Existing language limits and logical-cost semantics still apply. Logical work
is distinct from circuit cost, DUST, service fees and recovery reserves.

The aggregate retains `status` and ordered `cases`. Each case reports `name`,
`passed`, and ordered `stages`; each stage reports zero-based `index`, `passed`
and complete `actual`.
`passed` is a comparison verdict over local-reference-stage reports; the qualification labels are carried by each `actual`.
An expectation mismatch fails that case, retains its
actual report, and continues with actual state through later events and cases.
Any mismatch returns nonzero. A manifest/input error or impossible event
sequence also returns nonzero. Individual case export keeps the existing 1 MiB
export bound; an unexportable case publishes no partial JSON aggregate.

This is local, unauthenticated execution with consumption untracked. Test
agreement does not establish native proofs, settlement, authenticated events,
once-only consumption, circuit correspondence or complete conditional recovery.
U3, M10, the partial/conditional requirements, helper/circuit/native work and
every retained roadmap exit remain open. Security and hostile-test judgments
remain assigned to Opus; this document supplies none.

## Shared library report constructor

`mori_core::stage::outcome_report` renders the complete typed waiting, complete
or rejected outcome without running a program or performing I/O. Both staged
`simulate` output and the staged-test harness call it. The three local
qualification labels, full report fields and existing CLI exit behavior are
preserved. The generic private inspect-report writer remains separate.

The SDK-free library integration target `stage_reports` exercises TwoDeliveries
and QuotedFee through actual `stage::load`, `start` and `resume` calls. Each
successor uses the actual pending continuation. It compares all six reports to
the unchanged complete R3 expectations in
[`crates/fixtures/core-stage-reports`](../crates/fixtures/core-stage-reports/README.md),
then compares pretty serialization plus one newline to those exact file bytes.
The synchronized copy preserves the original model files and their provenance.
Its independent copy freeze is
`885663843203a1fe2f7820884d17f56e98b5dd10a3bbee654829ac649187203e`.

These changed-source tests are **UNRUN** until a separately recorded actual
result binds their exact source tree. The byte assertion is a new finite Core
serialization check, not inherited R3 approval of public stdout formatting.
It does not exercise the CLI writer, aggregate, process exit or I/O failures.
Rejected envelopes and hostile cases are outside these two positive library
cases. All fourteen original draft gaps and the full language, native, proof
and target acceptance obligations remain open.

## Adopted finite Core/2 observation contract

This section defines implementation-source-exposed ordinary observation design adopted on 2026-10-18 under the [recorded design decision](../evidence/utxo/doctorwatson-language-loop-2026-10-17/core2-report-adoption-20261018/DESIGN-ADOPTION.json). Its SHALL clauses are adopted design requirements, not runtime observations, independent goldens or permission for execution. They specify ordinary finite local report observations of already admitted Rust `moriarty/2` / `moriarty-core/2` invocations. They do not redesign financial semantics, admission, cap/refusal policy, continuation validation or security. Existing Core/1 NR-01–06 and financial-report adoption scopes remain separate. Original oracle freezes and all fourteen draft gaps remain unchanged and open; the per-gap dispositions below prevent inferred closure.

M03 local identity design was separately adopted by its designated owner under the [M03 decision](../evidence/utxo/doctorwatson-language-loop-2026-10-17/core2-report-adoption-20261018/M03-DESIGN-ADOPTION.json); the verbatim owner F4 clause appears below. This ordinary author supplies no identity or security decision. The String fields program_hash and plan_hash below retain their report projection. Printed report formatting SHALL NOT become their hash preimage. Global Core/2 expression allocation/sharing is specified in the separately adopted M01 supplement below; this document supplies no concrete Program/Plan identity value.

### Site and capture representation

For one selected action, AwaitSite.site SHALL be its one-based await occurrence in source order; site0 SHALL represent idle entry/completion, with captures:[]. AwaitSite.ordinal SHALL be the zero-based position of that await in the action's complete statements array. These numeric u16 JSON fields SHALL remain distinct from labels, ExprIds and segment-array positions. StagePlan actions SHALL retain program declaration order and statement_count SHALL count all source statements. Site numbering restarts per action.

A site's capture layout SHALL include all action inputs in declaration order, then all preceding let bindings and earlier accepted event binders in source-statement order, each prior event's binders in their declaration order. No liveness pruning is selected. It SHALL exclude state fields and the pending await's own binders. CaptureSlot SHALL contain exactly name, ty, origin and ordinal. Origin Input SHALL use the zero-based input declaration ordinal; Origin Let SHALL use the binding statement's zero-based source ordinal; Origin Event SHALL use the originating await statement's zero-based source ordinal. Multiple event binders at one await therefore share an ordinal and retain binder-array order. StageBinder SHALL contain exactly name and ty. Actual captures SHALL be ordered NamedValue objects with exactly name and typed value, without repeating origin/ordinal; they SHALL match their layout order and retain values captured at their binding positions. Reading current state from a boundary SHALL NOT recapture earlier lets. Empty captures alone do not imply idle; a positive pending site may have no captures. These are layout observations, not an authenticated-input rule or a Program/Plan identity construction.

### Step attachment, values and complete segment snapshots

Every emitted Step SHALL retain exactly index, statement, value, snapshot and effects. Index SHALL be the zero-based original action statement ordinal; it SHALL NOT be renumbered per segment. Statement SHALL be the exact admitted public IR statement, including its numeric ExprId references into that Program's expressions array. Statement amount/value/predicate/guard references SHALL NOT be replaced by evaluated values. This requirement fixes reference meaning and exact comparison, not the parser's global arena allocation.

At start or after an accepted guard, reaching the next await SHALL end the segment before guard evaluation and SHALL emit no Step for that await entry. A normal true resume SHALL begin its segment with exactly one Step for the pending Await: outer value typed Bool true, snapshot equal to the segment pre-boundary and effects:[]. The exact Await statement SHALL retain kind, label, binders and guard ExprId. Its accepted event tuple SHALL NOT replace Step.value. A normal false resume SHALL emit no segment and therefore no public guard Step, value, effects or synthetic snapshot. The next true resume SHALL not repeat the earlier financial prefix. A newly reached later await SHALL again contribute no entry Step.

For each other normally completed statement in an emitted segment, the present outer Step.value SHALL be its typed result: Let's captured RHS; Bool true for Requires/Ensures at their written positions; Next's RHS before assignment; Emit's emitted value; Transfer/Fee's evaluated amount; Liability's added amount; Discharge's discharged amount. UInt SHALL be {type:"UInt",value:canonical unsigned decimal string}; Bool SHALL be {type:"Bool",value:JSON boolean}. Null or omission SHALL NOT replace those results. Liability's result is not its closing amount; Discharge's result is not the remaining duty. This is an adopted Core/2 attachment rule, not inheritance from the Core/1 design or an interest/accrual convention. No Mint, Burn or dedicated accrual construct is introduced; their broader missing financial obligations remain unchanged.

Each Step.snapshot SHALL contain exactly complete state, balances and liabilities after that statement, including preserved rows and present-zero duties. Inputs/captures SHALL NOT become state fields. Balances SHALL retain full account/AssetId/amount rows ordered by exact (asset,account), with AssetId (network,issuer,reference,scale); liabilities SHALL retain full name/AssetId/debtor/creditor/amount rows ordered by name. State values SHALL retain typed values. Empty state SHALL be {}, and empty balance/liability lists []. Segment execution.pre SHALL be the complete starting boundary: the normalized scenario on start, or input continuation.boundary on an accepted resume. Execution.post SHALL equal the last Step.snapshot, or pre if there are no Steps. This local report frame supplies no authenticated-domain or history claim and does not replace boundary admission conditions.

Step.effects SHALL contain only the new statement delta: Let/Requires/Ensures and an accepted Await have []; Next has one State {name,before,after}; Emit one Emit {name,value}; Transfer one Transfer {asset,from,to,amount}; Fee one Fee with those same field names; Liability creation/addition and Discharge each one Liability {name,asset,debtor,creditor,before,after}. The variant name SHALL be the kind field. Effect assets SHALL be full AssetIds, effect amounts/before/after canonical UInt strings; State/Emit values retain typed Value objects. Existing financial transitions determine the values. In already admitted zero/self cases the corresponding effect SHALL remain present even if numeric state does not change. Liability creation SHALL insert its row, addition preserve identity, and discharge retain a closing-zero row; payment remains separate. An effect before zero alone SHALL NOT collapse absent opening duty and present-zero opening duty. Execution.effects SHALL be the ordered concatenation of Step deltas within that segment, retaining repetitions. Cross-segment prefixes are derived histories, not an extra wire field or an authenticated history certificate.

### Status projection and process contract

The public outcome_report envelope SHALL be distinct from derived enum serialization. Every root SHALL have exactly the three qualification members qualification:"local-reference-stage", authentication:"none", consumption:"untracked", plus the applicable members below. All listed members SHALL be present; no others SHALL be added by this projection.

| Typed outcome / ordinary invocation | Additional exact root members and projection |
| --- | --- |
| Waiting after start reaches an await, or true resume reaches a later await | status:"waiting", advanced:true, segment:full StageSegment, continuation:full Continuation, attempted:UInt string, committed:continuation.logical.committed, residual:positive duties of continuation.boundary |
| Waiting after an admitted false resume | status:"waiting", advanced:false, segment:null, continuation:unchanged input Continuation, attempted:UInt string, committed:that continuation.logical.committed, residual:its boundary's positive duties |
| Complete at start or after true resume | status:"complete", segment:full StageSegment, attempted:UInt string, committed:the typed Complete committed amount, residual:positive duties of segment.execution.post; advanced and continuation SHALL be absent, not null |
| Existing StageRejection supplied to the renderer | status:"rejected", error:full typed Error, attempted:its UInt string, committed:its UInt string; advanced, segment, continuation and residual SHALL be absent; no error condition/code/order or refusal-work rule is selected here |

StageSegment SHALL contain exactly action, frame_pre, frame_post and execution. Execution SHALL contain exactly semantics:"moriarty-core/2", action, pre, post, steps and effects. Action names SHALL agree with the selected action. PendingFrame SHALL contain exactly numeric site and ordered captures. Start's frame_pre SHALL be idle; an accepted resume's frame_pre SHALL equal its input pending frame. Waiting frame_post SHALL equal continuation's site/captures and execution.post SHALL equal continuation.boundary. Completion frame_post SHALL be idle. A start with no statements before its first await SHALL still emit a non-null segment with steps:[], effects:[] and pre=post. A normally complete empty action SHALL likewise emit its full empty segment; no synthetic Step is added.

Continuation SHALL retain exactly format:"moriarty-continuation/1", program_hash:String, plan_hash:String, action, site, captures, logical and boundary. StageLogical SHALL retain exactly profile:"moriarty-reference-logical-cost/2", mandatory cap (canonical UInt string or null), and committed (canonical UInt string). A successful waiting start SHALL project the supplied start cap or null; an advancing waiting resume SHALL preserve input continuation.logical.cap, without replacing its reported stored value with a per-call override. This output projection selects no cap admission/editability policy. Positive residual rows SHALL retain the full Liability DTO in boundary order; present-zero rows remain in the complete boundary but are excluded from residual. No report in this scope claims settlement. Error SHALL retain code, message, offset and step, with existing numeric/null representation, without adopting any stage-specific code or admission precedence.

For a successfully printed ordinary public simulate waiting/complete invocation within the existing bounded exporter, stdout SHALL be one complete ASCII JSON report followed by exactly one final LF, with no ordinary status text on stderr, and exit0. Report objects SHALL order member names lexicographically by their original string keys; arrays SHALL retain their prescribed order. Nonempty objects/arrays SHALL use LF-separated entries, two ASCII spaces of indentation per nesting level, comma at each nonfinal entry, colon then one space for object values, and a closing delimiter at the parent's indentation. Empty containers SHALL be {} or []. No trailing spaces, BOM or extra document SHALL appear. Numbers for u16 fields and scale SHALL be unsigned decimal JSON numbers; UInt fields remain strings, booleans lowercase and null literal null. Strings SHALL escape quote/backslash, use the short JSON escapes for backspace/form-feed/LF/CR/tab, encode remaining U+0000–001F and U+007F with lowercase four-digit Unicode escapes, and encode every non-ASCII scalar as lowercase UTF-16 Unicode escape(s), including a surrogate pair when required. Other ASCII characters, including slash, SHALL remain literal. These printing rules concern the report only, not digest preimages. They are adopted independently inspectable rendering choices; no output bytes are generated here. Existing exporter/input bounds and failure handling remain unchanged.

The staged test harness SHALL compare entire JSON values from the same projection while passing actual continuations, not expectations, to successive events. Its aggregate is a separate consumer; no per-invocation byte golden SHALL transfer to the aggregate or vice versa. The existing library pretty-byte check remains separate from public ASCII-export/stdout evidence. Rejected-envelope key projection above adopts no error-byte/process or hostile-case result. Complete stdout/exit/stderr conformance requires its own independently frozen bytes and authorized actual consumer observation.

### Local logical schedule and reporting authority

The adopted schedule for moriarty-reference-logical-cost/2 SHALL charge one unit before each ordinary statement dispatch and each distinct visited expression node, with a fresh node cache per statement. Both Boolean operands SHALL retain existing eager evaluation. Reaching the next await SHALL charge entry1 without visiting its guard and without emitting a Step. Resuming SHALL charge a separate dispatch1 plus the guard's distinct nodes. An admitted false guard SHALL report that invocation's attempted guard work and preserve the input logical.committed; a true guard SHALL charge that guard once and then the executed suffix, including any next await entry. No trace length SHALL substitute for this work count.

For an action with n awaits, segment indices SHALL be0..n: segment0 starts at action entry, segment j>0 begins with the guard dispatch of site j, and a successful segment stops before the next await guard while including its entry charge. ActionPlan.segment_cost SHALL hold n+1 successful segment costs in that order. committed_before[j] SHALL be the sum of earlier successful segment costs, with committed_before[0]=0; plan_total SHALL be their full sum. A normally successful segment j SHALL report attempted=segment_cost[j] and committed=committed_before[j]+segment_cost[j]. A waiting envelope SHALL project committed from its continuation; Complete SHALL use its typed field. False observations SHALL not accumulate into committed, although they remain separately attempted work. This is a local successful-prefix reporting convention, not authenticated lifetime-budget, financial commitment or history authority. No cap-editability, admission, resource ceiling or cap-refusal precedence policy is changed; those decisions remain with their owner.

The previously frozen escrow arithmetic (attempted5/2/6 and committed5/5/11) remains historically CONDITIONAL in its original partial freeze. The [2026-10-18 design adoption](../evidence/utxo/doctorwatson-language-loop-2026-10-17/core2-report-adoption-20261018/DESIGN-ADOPTION.json) establishes this schedule and prefix convention for a NEW independent freeze; it does not upgrade that original record or claim an observed meter pass. A distinct independent custodian SHALL derive and freeze complete new work expectations before an authorized comparison. Original conditional records SHALL remain unchanged. Logical interpretation units SHALL remain separate from circuit cost, DUST, service fees, recovery reserves, wall time and allocation.

### Individual retained gap dispositions

Every row remains OPEN; adopted ordinary design coverage does not establish its actual correspondence predicate or close the gap. Source line pins describe this inspected checkout and do not repair historical claims.

| Original gap | Current individual disposition |
| --- | --- |
| GAP-01 line numbers | OPEN; historical line-number claims not corrected; fresh source pins only |
| GAP-02 stage report internals | OPEN; exact positive envelope/DTO projection design adopted; new independent expectations and actual observations remain separate |
| GAP-03 /2 semantics string | OPEN; adopted design selects literal moriarty-core/2 within emitted segment Execution only |
| GAP-04 key order/format | OPEN; finite public ASCII rendering design adopted; independent byte freeze and real consumer observation remain separate |
| GAP-05 parser ExprId allocation | OPEN; finite M01 allocation/sharing/initializer design adopted; independent Program freeze and actual full-Program checks remain separate |
| GAP-06 stage admission premises | OPEN, designated-owner UNASSESSED; no admission condition or ordering adopted |
| GAP-07 stage-specific error codes | OPEN, designated-owner UNASSESSED; DTO projection supplies no code/precedence or rejection oracle |
| GAP-08 /2 trace bound | OPEN; existing bounds unchanged; no trace-bound design or result |
| GAP-09 normative Step.value source | OPEN; source-exposed typed-result design adopted for every emitted normal kind; actual comparisons remain separate |
| GAP-10 normative Await presence | OPEN; entry suppression/true-guard Step/false null-segment design adopted; actual comparisons remain separate |
| GAP-11 /2 decode/replay | OPEN; output contract supplies no new decoder/replay or compatibility result |
| GAP-12 MOR mapping | OPEN; adopted UNI child clauses are not a complete MOR traceability reconciliation |
| GAP-13 boundary-order provenance/R7 contradiction | OPEN, designated-owner UNASSESSED; adopted row order does not settle admission provenance; original R7 contradiction and invalidated R2 approvals retained |
| GAP-14 false-resume cap refusal | OPEN, designated-owner UNASSESSED; normal admitted false projection supplies no cap-refusal rule/test or precedence change |

The original partial independent escrow freeze retains its original missing-premise status. M01/M02/M04–M07 ordinary design is now adopted under the [recorded decision](../evidence/utxo/doctorwatson-language-loop-2026-10-17/core2-report-adoption-20261018/DESIGN-ADOPTION.json); M03 is separately owner-adopted under its [decision](../evidence/utxo/doctorwatson-language-loop-2026-10-17/core2-report-adoption-20261018/M03-DESIGN-ADOPTION.json) and verbatim F4 below. Those adoptions enable a distinct new independent freeze without rewriting the original partial record. All commitment/authentication/history/soundness/security conditions remain UNASSESSED by this lane. Original P-TRACE-PROJ/draft, C1–C6/N1–N6, all freezes/failures/counters and original gap dispositions SHALL remain unchanged. Proposal, two independent votes, adoption, independent freeze, runtime evidence and acceptance SHALL be recorded separately. This source-exposed adopted design supplies no independent goldens, execution authority or blanket gap closure. All U0–U7, M1–M17, MC/SP/G, partial/conditional, full finance, ACTUS/DeFi and composition exits remain unchanged; public developer use remains permissionless.

## Adopted M01 arena supplement

This separate ordinary representation design was adopted on 2026-10-18 under the [recorded decision](../evidence/utxo/doctorwatson-language-loop-2026-10-17/core2-report-adoption-20261018/DESIGN-ADOPTION.json). Its original frozen report-only proposal deliberately left M01 unselected; both original proposal packets remain unchanged. The SHALL clauses below are adopted and implementation-source-exposed. They describe the complete public Program representation of the finite documented positive source forms below; they grant no admission or execution authority. All fourteen individual gap rows remain OPEN. This supplement supplies an adopted finite representation premise, not a parser result, independent Program golden or completed GAP-05/M01 exit.

The two Core/1 full-Program fixtures from PR203 are finite precedent for source-order arrays, initializer placement and fresh expression occurrences. Their post-adoption derivation does not establish a general Core/1 grammar, general normalization or Core/1/Core/2 equivalence. Core/2 adopts nothing by inheritance here: each choice below was independently adopted by the two substantive design votes recorded on 2026-10-18; new independent expectations remain a separate freeze.

### Whole Program and declaration order

For already admitted source beginning language "moriarty/2", Program SHALL contain exactly language:"moriarty/2", semantics:"moriarty-core/2", name:the agreement identifier, states, assets, expressions and actions. No source hash, selected-action field, Scenario, capture, plan, registry, work counter or runtime result SHALL be inserted. Type SHALL serialize as "UInt" or "Bool". A typed UInt literal SHALL use {type:"UInt",value:its canonical unsigned decimal string}; a Bool literal SHALL use {type:"Bool",value:true or false}. Literal decoding follows the existing admitted numeric/string syntax; no decimal scaling, evaluation or new lexical normalization is selected.

StateDecl SHALL retain exactly name, ty and initial, with the literal initial stored directly as a typed Value. State declarations SHALL occur in Program.states in their own lexical declaration order. AssetDecl SHALL retain exactly name and identity; identity SHALL contain network, issuer and reference as decoded strings and scale as its numeric u8 value. Asset declarations SHALL occur in Program.assets in their own lexical declaration order. Action SHALL retain exactly name, inputs and statements; Program.actions SHALL retain every action, including unused actions, in lexical declaration order. Interleaving declaration kinds does not introduce a combined declaration-order field; each array is the source sequence filtered to its own kind. These representation rules do not authorize forward references or alter existing declaration admission.

A state's initializer, asset scale/identity, agreement/profile/action identifiers, input and binder declarations and statement metadata SHALL allocate no expression nodes. They SHALL NOT be converted into action expressions or synthetic statements. Action.inputs SHALL contain exactly {name,ty} entries: signature inputs in order, followed by any existing admitted leading input declarations in body order. Those leading declarations SHALL NOT count as statements. Await.binders SHALL retain exactly the written {name,ty} entries in binder declaration order, not action inputs. Runtime inputs, event values and captures SHALL NOT be substituted for declarations or expressions in Program.

### One fresh global expression arena

Program.expressions SHALL start empty and be shared across all actions in their lexical declaration order. Within each action, process statements in their written order and allocate only the statement's expression tree as below. Do not reset or rebase the arena at an action, statement, await, segment or selected-action boundary. Retain all nodes from unused actions. An otherwise empty selected action does not imply an empty whole Program arena.

For each source expression occurrence, allocate fresh nodes in left-to-right postorder. To append a node, set its ExprId to the current global array length, append exactly that node, then return that numeric zero-based index. Existing expression bounds and u16 representation remain unchanged. Do not intern repeated names, repeated literal values or structurally equal subexpressions, within or across statements/actions. No constant folding, algebraic reassociation, common-subexpression sharing, inserted coercion or desugared helper node is selected for this source-to-Program representation. Array position SHALL be the ExprId; no extra id member SHALL be inserted in a node.

| Source form | Exact appended node and allocation order |
| --- | --- |
| UInt or Bool literal in an action expression | One fresh Literal {value:typed decoded literal} |
| Ordinary name occurrence | One fresh Name {name:written identifier}; no substitution with a declaration, reference index or runtime value |
| Parenthesized expression | Its inner allocation/root only; no grouping node |
| Prefix not expression | Allocate its operand first, then one Not {value:operand ExprId} |
| Binary expression | Allocate the complete left subtree, then the complete right subtree, then one Binary {op:the selected operator, lhs:left ExprId, rhs:right ExprId} |
| liability_present("literal duty name") | One LiabilityPresent {name:decoded literal duty name}; quoted argument is metadata, not a Literal child |
| liability_amount("literal duty name") | One LiabilityAmount {name:decoded literal duty name}; quoted argument is metadata, not a Literal child |

Every brace form denotes exactly the listed fields plus kind carrying the variant name. Child fields are numeric ExprIds into this Program; they are not typed semantic values or source offsets. For this fresh-postorder source mapping, a child's index is lower than its parent's. Each repeated textual occurrence still allocates its own subtree. No rule here changes the separate admission of manually supplied DAGs or other profiles.

The finite documented expression grammar uses atoms above, parentheses, strongest prefix not, then decreasing binary precedence: * (Mul); + and - (Add/Sub); ==, <, <=, >, >= (Eq/Lt/Le/Gt/Ge); and (And); or (Or). Operators at one binary precedence are left-associative; explicit parentheses determine a different tree before postorder allocation. Minus is binary here; no unary-negative, division, generic function call, branching or implicit decimal node is introduced. This is an adopted finite representation grammar for already admitted forms, not a general frontend normalization claim.

The public DTO also contains BinaryOp::Ne. Current parser source recognizes it through token pair not followed by =, while the public README does not document that spelling and the shared lexer has no exclamation punctuation. This supplement SHALL NOT canonize that parser-specific spelling, invent != support or silently repair the language. Source inequality spelling/admission remains a separate ordinary grammar decision; if the intended spelling is !=, corresponding lexer/parser source repair and independent checks would be required. Such forms are outside this finite grammar's correspondence claim, without retiring any broader language obligation. Their potential future node shape remains Binary with op:"Ne" once source syntax is independently specified. No implementation mismatch is established for the chosen documented forms by this source-only review; conformance still needs real checks.

### Exact statement boundaries, metadata and roots

Each source statement SHALL contribute one Statement in Action.statements in written order, after its expression root is allocated. Statement boundaries SHALL not allocate nodes, reset numbering, append a unit/success result, or insert a statement. A semicolon ends the existing statement form; braces and leading input declarations are not Statement entries. Each expression-bearing field SHALL retain its numeric root ExprId, even where the expression's evaluated amount or boolean happens to have the same numeric spelling.

| Source statement form | Exact public Statement fields in addition to kind | Expression root |
| --- | --- | --- |
| let name = e; | Let: name, value | value is root(e) |
| requires e; | Requires: predicate | predicate is root(e) |
| ensures e; | Ensures: predicate | predicate is root(e) |
| next state = e; | Next: state, value | value is root(e) |
| emit "event name"(e); | Emit: name, value | name is decoded metadata; value is root(e) |
| transfer(asset,"from","to",e); | Transfer: asset, from, to, amount | asset is the alias, parties decoded metadata; amount is root(e) |
| fee(asset,"from","to",e); | Fee: asset, from, to, amount | same metadata roles; amount is root(e) |
| liability("name",asset,"debtor","creditor",e); | Liability: name, asset, debtor, creditor, amount | duty/parties decoded metadata and asset alias; amount is root(e) |
| discharge("name",e); | Discharge: name, amount | duty name decoded metadata; amount is root(e) |
| await label(name:Type,...) when e; | Await: label, binders, guard | label/binder declarations allocate nothing; guard is root(e) |

Ordinary expressions in an Await guard SHALL follow the same fresh allocation rule at that await's lexical statement position. A guard reference to one of its event binders SHALL allocate a Name occurrence; declaring that binder allocates none. Guard allocation SHALL not wait for runtime resume, and SHALL not depend on the event's value or whether a later invocation waits, advances or rejects. Await.guard's ExprId SHALL remain distinct in meaning from the action statement ordinal, plan site number and report Step.index. Name binding/type and financial/await execution semantics remain separate existing obligations.

No selected-action filtering, added declaration for admission, hidden arena node or runtime trace normalization SHALL alter this whole-source Program mapping. Retain unused declarations exactly when actually present in the source. No Mint, Burn or dedicated accrual construct is introduced, and broader missing financial obligations remain. The separately adopted site/capture, trace, envelope, rendering and local-work clauses above retain their own scope; original conditional-work records remain unchanged.

### M01 boundary and independent freeze

This supplement adopts an ordinary representation premise sufficient to derive a complete Program for the original finite escrow source from its actual declarations and expression occurrences. It supplies no such Program fixture, numeric arena specimen, parser output or expected execution. M01 source correspondence and GAP-05 remain OPEN until a distinct custodian derives/freezes the complete Program without using this source-exposed author or parser output as its oracle, and authorized real full-Program checks are observed. Recorded public-premise adoption enables that independent pre-run derivation while the source-correspondence exit remains OPEN awaiting real checks. The original partial escrow freeze, MISSING-PREMISES.json, derivation, all fourteen individual gap dispositions, R7 contradiction, original report candidate, freezes/failures and counters SHALL remain unchanged.

M03 local identity design is separately owner-adopted under its [decision](../evidence/utxo/doctorwatson-language-loop-2026-10-17/core2-report-adoption-20261018/M03-DESIGN-ADOPTION.json); only the verbatim designated-owner F4 text below supplies that normative identity clause. This ordinary author supplies no identity or security assessment. Source-file and proposal-byte digests identify inspected artifacts only, not canonical Program/Plan identity. This supplement supplies no admission, decoder/replay, proof, target or runtime result and closes no U0–U7, M1–M17, MC/SP/G, financial, composition or partial/conditional parent exit.

## Designated-owner M03 local identity clause

The designated owner adopted M03 on 2026-10-18 under the [unaltered M03 decision receipt](../evidence/utxo/doctorwatson-language-loop-2026-10-17/core2-report-adoption-20261018/M03-DESIGN-ADOPTION.json). The following F4 text is copied verbatim from the boundary-owner whole review, reviewToken `doctorwatson-core2-local-identity-boundary-owner-20261018-a1-whole-review`. Original normative authorship remains with that designated owner; GPT-6.1 Sol performed only the mechanical insertion and ordinary adoption promotion. Recorded owner-review provenance is Gemini 4 Argon High under the authorized outage fallback, as retained in the original full review; it is not relabeled as Opus or Sol authorship.

```text
F4 — Normative Observational Clause Text for Core2 Local Program and StagePlan Digests (Resolving M03):
1. Scope and Qualification: Core2 program_hash and plan_hash are deterministic local observational digests over in-memory Program and StagePlan structures (qualification="local-reference-stage", authentication="none", consumption="untracked"). They carry no cryptographic signature, owner authorization, replay protection, or proof/ledger validity.
2. Canonical JSON Byte Encoding (canonical_bytes): For any serializable value V, canonical_bytes(V) is the compact UTF-8 JSON byte sequence produced by converting V to a JSON value tree, recursively sorting the members of every JSON object in strictly ascending lexicographical UTF-8 byte order of their keys while preserving the element order of every JSON array, and formatting with no insignificant whitespace, no domain-separation prefix, no length prefix, and no trailing newline.
3. Scalar and Enum Encoding: Strings use standard RFC 8259 / serde_json compact UTF-8 escaping. UInt values (128-bit unsigned integers) encode as JSON strings matching ^(0|[1-9][0-9]*)$. Fixed-width integers u16 (ExprId, site, ordinal, statement_count) and u8 (scale) encode as bare JSON numbers matching ^(0|[1-9][0-9]*)$. Booleans encode as bare true or false. Type encodes as "UInt" or "Bool"; Origin encodes as "Input", "Let", or "Event"; BinaryOp encodes as "Add", "Sub", "Mul", "Eq", "Ne", "Lt", "Le", "Gt", "Ge", "And", or "Or".
4. Canonical Program Preimage and program_hash: The Program preimage is canonical_bytes(Program) over the complete admitted Program object with root keys in exact order ["actions","assets","expressions","language","name","semantics","states"], where language is "moriarty/2" and semantics is "moriarty-core/2". Nested objects have sorted keys: AssetDecl ["identity","name"]; AssetId ["issuer","network","reference","scale"]; StateDecl ["initial","name","ty"]; Value ["type","value"]; InputDecl ["name","ty"]; Action ["inputs","name","statements"]; Expr by variant ("LiabilityPresent" | "LiabilityAmount" | "Name" -> ["kind","name"]; "Literal" -> ["kind","value"]; "Binary" -> ["kind","lhs","op","rhs"]; "Not" -> ["kind","value"]); Statement by variant ("Await" -> ["binders","guard","kind","label"]; "Let" -> ["kind","name","value"]; "Requires" | "Ensures" -> ["kind","predicate"]; "Next" -> ["kind","state","value"]; "Emit" -> ["kind","name","value"]; "Transfer" | "Fee" -> ["amount","asset","from","kind","to"]; "Liability" -> ["amount","asset","creditor","debtor","kind","name"]; "Discharge" -> ["amount","kind","name"]). program_hash is SHA256(canonical_bytes(Program)) rendered as 64 lowercase ASCII hex characters.
5. Canonical StagePlan Preimage and plan_hash: Before computing plan_hash, StagePlan MUST have format set to "moriarty-stage-plan/1", program_hash set to the 64-character lowercase hex string of program_hash(Program), and every ActionPlan in actions populated with its complete sites layout and static cost fields (segment_cost, committed_before, plan_total). The StagePlan preimage is canonical_bytes(StagePlan) with root keys in exact order ["actions","format","program_hash"] and nested keys in exact order: ActionPlan ["action","committed_before","plan_total","segment_cost","sites","statement_count"]; AwaitSite ["binders","captures","label","ordinal","site"]; StageBinder ["name","ty"]; CaptureSlot ["name","ordinal","origin","ty"]. plan_hash is SHA256(canonical_bytes(StagePlan)) rendered as 64 lowercase ASCII hex characters.
```

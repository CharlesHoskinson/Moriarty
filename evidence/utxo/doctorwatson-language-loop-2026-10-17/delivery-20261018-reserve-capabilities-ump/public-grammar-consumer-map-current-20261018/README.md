# Current public grammar and consumer source map

Source inventory, 2026-10-18. Author: GPT-6.1 Sol, nonsecurity lane. No product process was run by this task.

The current Rust beta emitter accepts checked transfer expressions, while the real TypeScript Source/6 consumer also accepts repayment. Core/1 and Core/2 are separate Rust profiles. They are not interchangeable with Source/6 or the TypeScript Core/5 target. The passing capped QtyMin cases add two exact transfer observations; they do not settle this admission lattice.

## Pins and predicate

Base: `e4d7a2b5709caa18a94f5256d03a0302364b8235`. Explicit PR222 draft: `5acfebe651af47e8edd6741bba298d7336a03b0e`, tree `abb61a07e502d02a2f6b25f8f20e276246a51c80`.

Predicate: each enumerated production/constructor branch has an exact parser, checker, lowering/public entrypoint and real Node-consumer disposition, with finite observed-test evidence or an explicit unknown. This is falsifiable against the full source copies and Git blob/SHA256 inventory.

All 31 base-to-draft paths are test/regression/fixture paths. Production branches in this map are unchanged. SOURCE-INVENTORY.json pins every copied source with full Git blob, SHA256, size and commit. Full source files accompany pointers; short ranges do not replace originals. GRAMMAR-CONSUMER-MATRIX.json/CSV enumerate 150 rows: 34 shared Core syntax branches, 3 Core/2 refinements, 81 beta authoring forms/types/calls, and 32 Source/6 productions/refinements. These row counts are inventory units, not requirements, a denominator, percentages or acceptance.

## Real consumer paths

| Input | Admission and lowering | Public consumer | Real Node disposition |
|---|---|---|---|
| `language "moriarty/1"` / Core/1 | mori-core parser + validate; reference evaluator; backend scalar/financial relation lowerer | Rust `check`, `simulate`, manifest `test`, default `expand`; `compile` uses source_projection::compile_reported | Experimental Source/6 and Node beta frontend do not parse this profile. No corresponding Node compiler path identified in these entrypoints |
| `language "moriarty/2"` / Core/2 | parse_v2 is syntax-only; stage::load validates and plans; stage start/resume share evaluator; backend stage lowerer | Rust `check`, staged `simulate` start/resume and staged manifest `test`; unsigned staged `compile` | No Core/2-to-Source/6 Node adapter identified. Core/1 ledger/static/signed APIs refuse /2; signed staged compilation refuses |
| `profile "moriarty-beta/1"` | Rust mori-parser AST → mori-checker constant evaluation, nominal types and relation checks | Rust `check`; explicit feature-gated `expand --emit-source6-transfer` selects LocalS0 transfer and reads local scenario | Actual unmodified Rust stdout is consumed by the experimental TS parser/lowerer in the two regression scripts |
| Same beta profile through Node package | Separate TS frontend and bridge; operation and scenario shaping; actual Source/6 formation | `packages/moriarty-beta/src/cli.ts` check/inspect/expand/simulate/test | Node expand returns an expansion JSON wrapper containing Source6. It supports transfer and repay; it is not the Rust raw-output emitter |
| `moriarty-financial-agreement-source/6` S0 | Closed EBNF → TS parseSource6 → lowerSource6 → prepareSource6S0Unqualified | Experimental direct TS API; Node beta bridge emits the input; Rust transfer emitter emits a narrower subset | TransferLiteralFee and RepayAccrualFirst are implemented; provisional local consumer only |

Exact entrypoint lines are in every matrix row. Full public CLI files, parser/checker modules, lowerers and TS consumers are copied. `mori compile-call` reaches the distinct mori-ledger path; it does not make its call manifest the backend Artifact. Signed/authorization implementation source is present only to pin a boundary, with no new judgment about it.

## Observed tests retained

OBSERVED-EVIDENCE.json copies seven immutable prior receipts, plus original process logs where present. No receipt is rewritten. HISTORICAL-BLOB-COMPARISON.json compares 29 selected producer/test blobs between each receipt's recorded source and the applicable current base or draft; all 29 match. Whole trees remain distinct and no rerun is claimed.

| Receipt ID | Recorded finite observation | Scope limit |
|---|---|---|
| FINANCIAL-RELATION | 2 tests: Invoice linked observations and Reserve multiasset observation, exit0, source `4f83f0f613464343efd7a6322efacd8f87fdbc1d` | Actual compiler/SDK relation test only; not a public full Artifact golden or all grammar |
| C2-ESCROW | 3 load/library/public CLI tests, exit0, source `718b1b020718c6b23a12846d32d5d2db9f14de7c` | Exact complete escrow specimens |
| C2-PARTIAL | 3 load/library/public CLI tests, exit0, source `078047940fb76e1ef62842f45dadd41a6592f99a` | Exact partial-duty continuation specimens |
| C2-MULTI | 3 load/library/public CLI tests, exit0, source `e0d0b4c7e80f53ead164358a68d804dc18c68d89` | Exact four ordinary outcome combinations and actual continuation links |
| S6-BASE | Library, CLI, build, actual Node regression steps exit0; source `8267ad950742c94f74fd871c266fa995c4e9e893` | Invoice decimal and Workshop referenced-sum transfer only; scoped financial projections |
| S6-QMIN-RUST | 1 Rust public CLI regression checking both actions, exit0; source `34257f3ef95e7a1bf29031417e7f6d33f708f06a` | Both QtyMin choices, zero fee and three opening cells |
| S6-QMIN-NODE | Actual Rust stdout → real TS parser/lowerer/provisional consumer for both capped actions; exit0, draft `5acfebe651af47e8edd6741bba298d7336a03b0e` | Complete independently frozen semantic AST/effects and financial projections only; whole Core/Program/Artifact/work/qualification remain unasserted |

TEST-SOURCE-INVENTORY.json enumerates lexical test declarations. Source-present tests are not reported as runs. In particular the Source6 R-30, full-repay and near-bound repayment tests exist in the actual TS consumer test file, but no matching Rust-to-Node repayment receipt is imported. A row's observed IDs identify enclosing finite specimens, not exhaustive value or alternative coverage. No hostile/security test disposition is supplied here.

## Explicit unsupported and unknown boundaries

- Core admits UInt/Bool, literal state initializers, fixed assets, typed action inputs, expressions and the listed statements. No general decimal Qty, arrays/records, financial named-call families, loops, user functions, recursion, division, dynamic account strings or native calls occur in its admitted grammar. Core asset scale0..38 differs from beta/Source6 scale0..18.
- Core/1 refuses Await. Core/2 adds only the fixed await/binder/guard form; event/continuation inputs are separate DTOs. No join/cancellation/recovery grammar follows from an await. Core/2 finite bounds and staged qualifications remain in source and all fourteen original staged gaps remain individually OPEN.
- Actual Core Ne source spelling is `not =`, not `!=`. The adopted M01 text explicitly declines to canonize this undocumented spelling. This map records source, proposes no semantic change and does not recommend a positive test that would silently decide it.
- Core reference scenarios can contain external duty identities that a source-wide closed backend liability registry cannot infer. Unknown liability reads/discharge-only identities can therefore pass one admission stage and fail the compiler. Registry ceiling16 and target/domain/numeric/circuit limits are separate compiler constraints. An admitted parser node never means every program using it compiles.
- Beta arithmetic is constant evaluation: same-kind/same-asset addition/subtraction; Scalar×Scalar and either Qty×Scalar order; Qty×Qty refuses. `atoms`, `min`, `max`, references and parentheses can feed transfer fields. Parser generic type syntax does not admit arbitrary type constructors. Exact schemas, required/optional fields and roles are in BETA-SCHEMAS.json; relational checks remain necessary.
- All 24 dotted financial calls in the current30-call table are SpecifiedOnly: AMM swap/redeem/mint; lending originate/liquidate/roll_forward; stablecoin mint/redeem/emergency_settle; option fix/exercise/settle; oracle select; governance queue/execute/veto; bridge escrow/claim/recover; staking deposit/reward/slash/unbond/withdraw. They have authoring-role checks but no Source6 S0 constructor or current Rust expansion. This is an ordinary source-support fact, not a financial/security verdict.
- Beta repay is LocalS0 checker-admitted and the Node bridge/Source6 consumer supports it. Current Rust extract explicitly requires operation `transfer`; Rust LocalTransferScenario has three balances and no obligation. Its CLI rejects obligations/candidate_effects. Repayment is thus an explicit missing positive Rust emission path, not an undocumented compiler success.
- Source6 has no general expression language: all amounts are canonical integer tokens. Unknown/duplicate fields and wrong order reject. Transfer requires three ordered distinct cells even with fee0; repay requires two plus one matching duty. Fee0 omits a third Credit. Repay offers identity conversion and accrual-first duty update only. Nonempty observations/disclosures/retained effects/retained duties, delegation/recovery, arbitrary financial families, rounding/division/mint/foreign-evidence/accepted-failure/general Phi/Omega profiles are outside this S0 grammar.
- Source6 parses action/effect shape before the underlying preparation consumer evaluates the proposal. Syntactically representable replay status or submitted effects are not automatically admitted transitions. Bare Source6 nonce is lowered to the Core JSON tuple key. No security conclusion is inferred.
- Full Rust Program representation, local scenarios, TS Core/5 state/intent/effects, backend Target and full backend Artifact are distinct boundaries. These transfer receipts do not supply target IR, all ABI field identities/order, full Artifact serialization or native acceptance. Existing independent full Artifact oracle gaps and four-golden work remain open. No measured work or qualification is reconstructed from fixtures here.

## Next two ordinary positive cases

These are prioritized source proposals, not executions or changed semantics. Their exact source-based predicates and independent arithmetic recipes are in NEXT-POSITIVE-CASES.json.

1. A transfer using `max` after Qty subtraction and left Scalar×Qty, with explicit `atoms` and a positive fee. Existing producer support suffices; the new comparison should use public Rust check/expand output directly through the same real TS consumer and an independently frozen complete Source6 AST/effect/financial recipe. Proposed value800, fee25, gross825 (scale2); this differs from both passing QtyMin actions. No new implementation should be needed if current source behaves as read.
2. A funded partial repayment with accrued interest, contrasting obligation rows and SetObligation with transfer. Current TS consumer and separate Node beta bridge already contain this constructor, but the Rust emitter/scenario boundary refuses it. First obtain a scoped ordinary source extension to reuse that existing semantics, or deliberately label a Node-package-only positive. Do not treat the existing Rust refusal as the required second successful Rust-to-Node program, and do not replace the producer with a fabricated AST or host simulation.

The pair is useful because the first probes an unobserved admitted arithmetic slice without extending syntax; the second exposes a real constructor boundary. They do not replace the existing Invoice/Workshop/min cases or Core staged specimens.

## Requirements and limits

ROADMAP.md:288 U2 retains structurally contrasting programs and a generality argument over admitted grammar. docs/audits/2026-10-14/AUDIT-PLAN.md:7 and :66 require grammar-by-consumer/field-by-boundary coverage and only comparisons over the documented common domain. UNI-018:190, UNI-019:217 and UNI-020:228 require profile/consumer admission, scoped complete comparisons and independent expectations; source-only and observed evidence remain separate. Full originals are copied.

Startup develop skill, pinned AGENTS and product contract were read. The read-only plugin status returned historical SP01.6 blocked dependent dispatch with stale inputs, missing accounting/live resource; no pending transactions. That status did not select this task. No compiler/Cargo/SDK/VM/Node/model/native/proof execution, network/provider call, shared repository edit or Git mutation occurred. Only this new output directory was written.

All U0–U7, M1–M17, MC/SP/G, full finance/composition, target and release exits retain their original dispositions. This packet makes no percentage, acceptance or security claim.

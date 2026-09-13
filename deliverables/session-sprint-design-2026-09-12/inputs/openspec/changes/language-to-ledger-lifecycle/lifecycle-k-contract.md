# Lifecycle K integration contract

Status: proposed for independent design review, September12. Source implementation may proceed only after root adoption and a substantive independent agreeing review. Runtime invocation requires separate current resource admission. No K or ledger acceptance follows from source review.

## Objective and scope

Execute the reviewed Source/5 loan lifecycle through independently checked Core/4 K semantics. Add `financial-lifecycle-v1` to the existing `experiments/moriarty-language/formal/k/run.py`, with complete results compared against Source/5, direct Core/4 TypeScript, and independently derived expectations. Use merged baseline81bed862c60b184e7f5fdb27577157c2bcf39530. Preserve old profiles and their evidence. No separate runner framework or host financial evaluator.

The independent design and38 specified cases in `deliverables/language-to-ledger-2026-09-12/design-review/lifecycle-k-expectations.md/json` are normative acceptance requirements except for decisions fixed here. The canonical `loan-origination-accrual-contract.md` and actual versioned source/Core public contracts determine semantic details and error priority. Reconcile disagreements explicitly before changing accepted semantics. Copy selected expression K source modules from the pinned existing worktree as reviewed integration inputs; do not copy compiled artifacts or grant their historical execution budget.

## Transport and context

The normal CLI evaluation packet has exactly primitive strings `{schema,request,financialPreState}`. Missing factory-context coverage uses the distinct closed packet `{schema,request}` and maps to `lifecycleMissingContext(S,R)`; a present null/non-string is a transport error, never omitted context. The internal normal constructor is `lifecycleRequest(S,R,F)` where each item carries original string and structural parsed representation or parse-failure token. No validity/type/funding/expected-result flag is allowed. Transport parse metadata means lexical parsing only.

Schema and request preserve the public canonical JSON contract. Financial text preserves its original UTF8 bytes and its own public JSON.parse admission behavior, including whitespace and last duplicate object key semantics. Do not reuse a duplicate-rejecting canonical parser for financial text. Numeric lexical forms/JSON number treatment must mirror the actual lifecycle input admission; parsed numeric financial amounts must remain distinguishable from decimal strings. Invalid financial JSON reaches K as parse failure so earlier Core static errors still win. Original byte limits must be checked before normalization. Structural parser tests include duplicate keys, whitespace, escaped Unicode, malformed input and extra fields. Do not derive semantic decisions in Python.

K independently validates selected Core/schema, full financial context and runtime snapshots in actual Core/4 order. Source elaboration/whole-source checking/selection remains an explicitly trusted separately tested frontend. Source-only errors never count as K invocations. The new actual consumer `formal/k/lifecycle-corpus.mjs` calls public source elaboration/check, constructs selected Core requests, captures actual preceding state on every step, and invokes the existing runner only under supplied runtime admission. An offline preparation mode produces source/Core cases without K dispatch. It must not precompute financial POST into the request.

## K representation and complete output

Use separate lifecycle entry and semantic modules importing reusable structural expression modules. Keep immutable original inputs and a single private staged machine: prefix -> admitted ordered kernel -> Ensure suffix -> publication. PRE reads immutable input; POST reads tentative financial result. K owns every state admission rule, protected descriptor mapping, checked arithmetic, state/effect update, work debit, output size check and rejection. Independent K checks include unexecuted branches. K Int requires explicit UInt128/signed128/UInt64 overflow checks before divisions/sums.

Closed output variants are `lcPrepared(Post,FinancialPost,Effects,Remaining)`, `lcExpressionRejected(Code,Span,Path,WorkUsed)`, `lcKernelRejected(Code,ActionIndex)`, and `lcAdapterRejected(Code)`, with separate `lcPending()`. Null kernel actionIndex has its own JSON null representation. Prepared decodes to the exact current funded envelope; rejection decodes to the exact corresponding union. Decode only after empty continuation, one terminal output and equality of immutable input cells with supplied originals. Every output field comes from K; no copying unobserved fields from host input, defaulting or sorting arrays. A native crash, timeout, stuck term, unknown constructor or codec failure is an execution failure, never a semantic Rejected value.

Do not rewrite old expression-v1 semantics when adding financial constructors. Version isolation must prevent widened legacy admission. Port all actual constructors required by the four lifecycle actions and the38 independent financial cases, including static invalid/unselected controls. Inventory every remaining current Core/4 constructor against K; unsupported cases remain explicit uncovered implementation boundaries. This is scoped agreement, not all-Core equivalence. Do not remove any supported acceptance case to achieve a pass.

## Frozen lifecycle observations

Source SHA256a0efd5166fdf27106110c69548d3b3065070e8d759d231296c454d19757bbaaa. Use the exact merged example and root independent full-state templates/probes. Work totals originate99, accrue65, repay88, settle86: prefix36/10/35/33, kernel2/1/2/2, suffix61/54/51/51. Remaining413/348/260/174 and spent116/181/269/355, reserve16. Do not use placeholder work values in final fixtures or derive oracle values from evaluator outputs. Independent expected fields retain debt components, balances, allowances, liability cap/incurred, terms, period, history insertion order, provenance and complete effects. Seed512/spent17/reserve16 is a local fixture, not CPU authority.

Compare complete objects at every actual chained action and rejection. Exercise the38 specified additional cases, exact-work/one-short/kernel/suffix failures, late false ensure rollback and valid same-ID retry. Derive each diagnostic span/path/work from source/Core constructors and actual contract; retain its independent derivation. Comparator mutation controls alter debt split while keeping total, allowance spent, work, cursor, effect metadata and history ordering separately and must all fail comparison.

## Historical crash and execution gates

The two retained trace106 failures require one separately admitted diagnosis before another broad runtime retry. Metadata depth5947/Core65536 remains accepted and5948/Core65547 rejects INPUT_BOUND; metadata is not expression/value depth. No metadata cap1500, omitted trace, erased type metadata, host typing flag, reset invocation counter or unlimited-stack workaround. Preserve old compile markers and charges. Source-only lifecycle integration can proceed while the retained diagnostic grant is reviewed, but no author may compile K or invoke K without root's current exact admitted command.

After diagnosis select and review a semantics-preserving repair supported by actual evidence, then run fresh full applicable expression and lifecycle conformance under a bounded reviewed grant. Historical105 matches are not current approval. Source-only Python/Node structural, codec, fixture, comparator tests and TypeScript regression checks are permitted now. Test doubles never count as K results.

## Files and validation

Author owns formal/k lifecycle entry/modules/codec/parser/corpus and required reconciliation of expression modules plus existing run.py, focused tests, K README and this task's coverage record. May add `tests/lifecycle-k-corpus.test.mjs` and an npm offline corpus command if required by the actual consumer. Do not change TypeScript semantic implementation, immutable source profiles, wallets, accounting, plugin, compiled artifacts or prior evidence. If a semantic discrepancy needs a product change, report it for separate review.

Run meaningful failing offline tests then passing codec/loader/comparator/corpus tests, full language tests/typecheck and relevant old Python runner tests. Retain actual commands/exits and source pins. Final source audit and separate actual K-result audit must be fresh Astra medium. Compact authentication/compilation, Docker, Preview, wider proof and ledger gates stay open.

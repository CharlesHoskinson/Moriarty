# Draft financial validation design

## Status and decisions

This proposal is S2: written proposal or design. Retained source observations describe their original experiments. No source suite, UPLC, or native command was run here. Only this package's OpenSpec structure is checked.

Revision03 is approved only as the T1 design on candidate `dcc62e21e9e351843bd04a3be537ebe260f8c1d0ec606cfcc5e15fd83ca7aff0`. Earlier revision01 and revision02 failures remain preserved. Revision02 candidate `112ccc14dc0e98098557143b5002c880b2dfd1a5d07b0c9940a99ae17b948a26` received CHANGES_REQUIRED. This draft does not rewrite those failures.

T1 is source /5, contract /4, lifecycle /1, quantum 1, Transfer-only, empty observations, and empty obligations. T1 returns NONEMPTY_OBLIGATIONS on E12. Future full finance includes Originate, Accrue, Repay, Fee, lifecycle /2 authority, and source /6 contract /5. Source /6 derives funded-expression-source-v2 and lifecycle-v2. Standalone Allocate is not a selected source descriptor. Repay owns nominal allocation. Fee retains Fee kind under its transfer-shaped binding. These additions require a separate scope amendment. They cannot enter T1 by removing duties.

S0 means historical or superseded. S1 means idea or discussion only. S2 means written proposal or design. S3 means prototype or experimental branch. S4 means implemented and tested but unreleased. S5 means released but not deployed or broadly supported. S6 means deployed and supported. S7 means deployed, independently audited, and operationally evidenced. These are the exact WIKI_SCHEMA meanings. This package creates no wiki claim or vault transaction.

## Inputs and trust boundaries

Financial inputs come from actual versioned factories, source/Core semantics, independent expected records, and packet19 RESULTS.json. Designer observations supply retained evidence, not expected values copied from an author. Every comparison binds source, Core, schema, bounds, action, and input bytes. Source check and elaborate cannot replace evaluate.

The input manifest observes current custody module hashes. These files are mutable partial Grok work. Their presence and hashes establish no implemented or passing finance capability. No implementation18 file is inspected.

Source actors and observations are arguments, not authenticated oracle or role claims. The ledger package owns actual witnesses, reference resolution, required signatories, head consumption, and payout allocation. The native package owns POSIX parity, calibrated siblings, submission, inclusion, race evidence, and readback. The conformance/private package owns final tier reconciliation and mandatory private/PCD/Midnight gates.

Qualified graph navigation was not established for this moving validation corpus. Direct pinned files control this draft. The full 347MB graph was neither loaded nor rebuilt.

## Proposed interfaces

These are proposed data contracts. They are not existing exported APIs. Finance uses field semantics, not invented callable functions.

`FinancialCaseOracle/1` contains `caseId`, `familyId`, `sourcePath`, `sourceSha256`, `sourceProfile`, `contract`, `lifecycleVersion`, `coreHash`, `schemaHash`, `boundsHash`, `actionSelector`, `ordinaryInput`, `financialPre`, `independentExpectedEnvelope`, `actualEvaluatorEnvelope`, and `inputHashes`. Atomic cases carry `invocation`, complete sealed state, writes, effects, and obligationDelta instead of funded-envelope fields.

`FinancialCaseOracle/1.layerResults` contains independent `source`, `admission`, `uplc`, `native`, and `private` records. Each record contains `status`, `predicate`, `receiptPath`, `receiptSha256`, and `notExecutedReason` when applicable. Source diagnostic fields retain code, source text/span, node path, actionIndex, and workUsed wherever returned. No backend diagnostic replaces a source diagnostic.

`EffectAllocationReceipt/1` is produced by the ledger package. It binds consumed head out-references, sorted participating inputs, each effect ordinal/id, exact asset policy/name, recipient credential/address, payout output index, amount, and continuation. Its allocation is injective for the selected separate-output policy. It includes missing-leg, duplicate-input, same-output, and extra-participant refusal records.

`FinancialWalletProjection/1` is produced from the resolved final body. Fields are `normalInputGrossByOwnerAsset`, `allOutputCreditsByOwnerAsset`, `netByOwnerAsset`, `networkFeeADA`, `sponsorshipADA`, `changeByOwnerAsset`, `otherAssetFrame`, and `bodyHash`. Collateral fields are separate wallet-body evidence. The source Fee is a kernel effect. It is not automatically a Cardano fee. Cash never converts to ADA without an explicit admitted conversion.

`ObservationBinding/1` binds source action argument names to a resolved CIP31 out-reference, expected oracle script credential, policy/name NFT quantity 1, datum schema/hash, feed, publisher authority, timestamp, price, and validity endpoints. Its proposed policy requires one matching reference and rejects zero or multiple matches. The reference contributes no business funding. The actual state head remains a consumed input. Oracle authentication proves publisher provenance, not market truth.

`AuthorityBinding/1` binds role text to key/script/bearer authority and signed canonical outcome bytes. Role authentication comes from the ledger package. Fee-inclusive outcome identity includes program, domain, instance, selected action/arguments, predecessor, nonce, asset mapping, gross cap, net goal, and body projection. AP03 is a shared seam, not an assertion that source commitments verify signatures.

Compiler dependency: complete generic source/Core lowering, including nested ConstructRecord and every applicable kernel operation. No source-name or family-ID switch can select a successful envelope. Lowering commits its exact versioned manifest and script. Finance consumes complete outputs and checks semantic correspondence independently.

Ledger proposes compilation order using the ScriptContext own credential to remove the RuntimePins script-hash cycle. That is a separate amendment to revision03. It does not change these financial oracles or inherit existing approval.

## Complete state policies

Existing balance and allowance rows update in place. Missing recipient rows append at the end. Source order is preserved in arrays. Canonical JSON object-key order is a hashing rule, not a list-sorting rule.

Every ordinary field not written by the selected action remains equal. Authority, assets, settlement policies, balances, allowances, obligations, used transfer/allocation/origination/accrual IDs, work, and all unrelated fields compare deeply. Accepted effects include every field in emitted order. Missing, added, reordered, and altered effects fail. Rejection publishes no accepted post, financialPost, effects, or atomic candidate.

Transfer subtracts sender Cash and allowance remaining, adds allowance spent, credits or appends recipient, and appends the ID. Zero amount, self-transfer, missing sender/allowance, inadequate balance/allowance, capacity, duplicate ID, and overflow retain their actual source diagnostics. Refunds do not restore gross allowance.

Originate consumes matching transfer funding once, creates the complete nineteen-field obligation, records origination history, and retains unrelated rows. Repeat-unfunded fails INSUFFICIENT_BALANCE before a duplicate-obligation check. A separately funded duplicate control is required to reach that check.

Accrue requires next period exactly last+1 and observedTime at least nextAccrualAt. Checked principal*numerator determines floor or ceil interest. Accrued, outstanding, and cumulative liability all increase. Repayment does not reset liabilityIncurred. The next time is firstPeriodStart+(periodIndex+1)*periodSeconds under UInt64 checked arithmetic. Zero denominator, period sequence, maturity, range, cap, duplicate, and capacity failures remain distinct.

AccrualFirst uses accrued first. PrincipalFirst uses principal first. ProRata uses floor(n*principal/(principal+accrued)) for principal and assigns the remainder to accrued. Component overpayment rejects. Conversion none requires zero remainder. Floor and ceil follow the pinned conversion policy. Nominal debt remains nonnegative within signed128 maximum.

UInt128 maximum is 340282366920938463463374607431768211455. UInt64 maximum is 18446744073709551615. Nonnegative signed128 maximum is 170141183460469231731687303715884105727. Descriptor count, byte, node, depth, and collection capacities come from the exact admitted source bounds and evaluator. No backend may silently widen them.

Financial Core FloorDiv requires a positive denominator and mathematical floor for signed operands. Atomic policy rounding is a separate admission constraint. Stage00 rounding-none FloorDiv simulation was not native admission. T1 FloorDiv and nonunit quantum remain refused. Future arithmetic support requires declared remainder and divisibility policies plus independent boundary controls.

## Parameterized independent controls

Packet19 contains 46 source cases and supplemental cross-action probes. Its source bindings are actual source results. Its compiledValidatorTemplates have ledgerExecution false.

The complete source oracle includes Lender balance 7→107, Borrower 3→103, retained Witness Cash 5, retained Lender allowance 11/spent 2, prior ID PriorSpend, ReleaseAlt, ordinary revision 4→5, additive work remaining 151/spent 59, exact debit remaining 0/spent 49, closureReserve 1, and a missing recipient row that appends.

Failures include lender balance overflow OVERFLOW, work total invariant INVARIANT, and one-short work WORK_EXHAUSTED at workUsed 48. Early guard failures consume 4, 8, 12, 16, 20, or 24 work units. They do not consume the accepted 49-unit debit. Unknown fields, schema, identifiers, amounts, profile mismatch, and action substitution remain complete source envelopes.

T1 has staticWorkBound 48 and one Transfer kernel unit, giving work 1024→975 and spent 0→49. Transport revision/remaining 0/1→1/0 are independent of ordinary revision 0→1. Finance fixture work debits are 99, 65, 88, 86, 16, and 26 for the six successful finance cases. SB03 cancel/execute debit 31/35, and SB04 batch debits 70. AP01 route debits 31 and AP02 consume debits 22. These are scoped straight-line source counts, not a general static equality claim for conditional execution.

## Time and observations

Original T1 release [1000,1099] contains 100 inclusive milliseconds with endpoint difference 99. Refund [1100,1100] contains one millisecond. These remain Layer S. Native execution cannot relabel them as live positives.

The native package owns a guard-only calibrated sibling after genesis/SDK/V3 POSIX parity. Slot length must be 1000ms. Window slots are 60, inclusion margin is 30 slots, and forecast ceiling is 1800 seconds. Source and program hashes change only for calibrated guards. Bounds stay unchanged when their bytes remain unchanged. Unrepresentable, expired, overflowed, or unbounded windows fail closed.

Oracle freshness covers the whole interval: publishedAt <= lower and upper <= publishedAt+20. Threshold price is at least 10. Source [1010,1020] with publishedAt 1000 passes, while upper 1021 and future timestamp 1011 reject. Actual reference datum values must equal those Args.

## Economic examples and negative controls

SB01 requires both A10 and B25 legs. SB02 uses numerator 19940000000000 and denominator 1009970000. Quotient is 19743 and remainder is 162290000. The next independent successor yields 19356, not the first constant. Product cannot decrease under the admitted fee model. Slippage 19743 passes, 19744 rejects. Asset mappings are injective and recipients match exact admitted credentials.

SB03 terminal execute pays all 100 at 999 and cancel refunds all 100 at 1000. Minimum 70 does not authorize closing with residual 30. A later partial order requires a separate continuation policy.

SB04 settles 30 and 70 into distinct tagged output slots even for one recipient. One output 70 cannot satisfy both orders. Repeated cheap input, duplicate payout slot, wrong sorted index, wallet substitution, omitted extra order, and underpayment reject.

F01 creates principal/outstanding/incurred 100 under cap 110. F02 adds accrued 10 at 1060. F03 partial repayment 30 discharges accrued 10 and principal 20, retaining outstanding 80. Final repayment 80 settles but retains incurred 110 and initialPrincipal 100. All full effects compare.

F04 Fee 1 changes Borrower10→9 and Lender100→101 without reducing debt. Refund5 in each direction leaves balances unchanged but consumes both gross allowances. Native normal input100000 with change90000 remains gross100000. Cap10000 rejects. Collateral5000000 against cap2000000 rejects before signing. An included invalid phase-two transaction can lose collateral while business state stays unchanged.

AP01 route Lender100→Borrower plus Borrower Fee5 yields Borrower net95. Minimum96 rejects. Well-formed network/program/capability substitutions can still prepare source results. Those are retained authentication gaps. AP02 old snapshots can re-evaluate successfully in source. Authenticated head consumption and durable successor history must close the native replay gap.

S03/S04 remain unsupported source actions. Their future split/join/migrate/terminate relation must conserve duties, lifetime work, allowances, and history. Public financial execution cannot satisfy private handoff or general PCD requirements.

## Family matrix

Each row preserves the exact inventory ID. No row inherits prototype native evidence.

| Inventory family | Current source evidence | Current admission | Compiled UPLC | Native | Private | Required future predicate |
| --- | --- | --- | --- | --- | --- | --- |
| S01 | retained source oracle | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | exact frozen family source + full envelope + native context positives and failures |
| S02 | retained source oracle | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | exact frozen family source + full envelope + native context positives and failures |
| S03 | SOURCE_ACTION_UNKNOWN / OPERATION_BINDING | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | separately approved lifecycle relation, conserved duty and authority, authenticated successors |
| S04 | SOURCE_ACTION_UNKNOWN | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | separately approved lifecycle relation, conserved duty and authority, authenticated successors |
| escrow-time | 22 exact matches + 4 named limits in 26 cases | T1 custody subset proposed /5 only, E12 NONEMPTY_OBLIGATIONS, /6 PROFILE_MISMATCH | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | exact frozen family source + full envelope + native context positives and failures |
| interval-boundary | retained source oracle | T1 custody subset proposed /5 only, E12 NONEMPTY_OBLIGATIONS, /6 PROFILE_MISMATCH | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | exact frozen family source + full envelope + native context positives and failures |
| joint-role | retained source oracle | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | exact frozen family source + full envelope + native context positives and failures |
| oracle-observation | retained source oracle | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | exact frozen family source + full envelope + native context positives and failures |
| bilateral atomic escrow swap | retained source oracle | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | exact frozen family source + full envelope + native context positives and failures |
| constant-product exact rounding/slippage with serialized pool continuation | retained source oracle | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | exact frozen family source + full envelope + native context positives and failures |
| order execution versus timeout cancellation | retained source oracle | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | exact frozen family source + full envelope + native context positives and failures |
| two-order batch exact input coverage and injective output allocation / same-output double satisfaction | retained source oracle | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | exact frozen family source + full envelope + native context positives and failures |
| F01-origination | FundedExpressionPrepared / INSUFFICIENT_BALANCE | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | exact frozen family source + full envelope + native context positives and failures |
| F02-interest-period | accrue / PERIOD_NOT_ELIGIBLE / DUPLICATE / LIABILITY_CAP_EXCEEDED | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | exact frozen family source + full envelope + native context positives and failures |
| F03-partial-repayment-residual-liability | partial, final, DUPLICATE | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | exact frozen family source + full envelope + native context positives and failures |
| F04-fee-gross-net-policy | Fee / INTENT_NET_GOAL / INTENT_DEBIT_CAP | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | exact frozen family source + full envelope + native context positives and failures |
| AP01 | retained source oracle | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | exact frozen family source + full envelope + native context positives and failures |
| AP02 | retained source oracle | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | exact frozen family source + full envelope + native context positives and failures |
| AP03 | retained source oracle | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | exact frozen family source + full envelope + native context positives and failures |
| AP04 | OPERATION_BINDING / UNKNOWN_DECLARATION | closed: PROFILE_UNSUPPORTED historical matrix | family not measured, Stage00 distinct predicate | unsupported-measured-gap | unsupported / unverified | fail-closed certificate/private probe and future whole-relation proof gate |

## Failure handling and receipt acceptance

Every receipt binds the exact case, source/Core/bounds, full input bytes, implementation freeze, validator script, and layer. Failure status distinguishes source rejection, admission refusal, UPLC failure, phase-one rejection, included phase-two failure, transport failure, confirmation timeout, and unrun template. Missing receipts remain unverified.

The independent packet hash is `9d71e736e3d3c159a83ff3bb21dbb3f5b2a9661dd90ed1e70414c2690b6552d6`. Stage00 confirms only its public single-head atomic wallet-transfer prototype. The reviewed receipt helper pin is `f3689ef82e078b9331907c4d13bde9ebfc3e2f501d88edb41d394c4f7479bf18`. Native ownership includes import and actual result handling. This draft performs neither.

Missing full finance lowering, native context binding, calibrated source, actual results, or whole-relation private proof blocks its dependent completion claim. A passing OpenSpec structure does not satisfy any of those predicates.

## Source case and requirement crosswalk

Custody E11, E13, and cancel-empty controls map EU-FIN-002 through 006. E12 maps EU-FIN-005 and 022. Interval controls map EU-FIN-006. Oracle and joint-role source cases map EU-FIN-007 and 008. SB01 through SB04 map EU-FIN-009 through 013. F01 through F04 map EU-FIN-014 through 019. AP01, AP02, and AP03 map EU-FIN-020, 021, and 008. S03/S04 duties map EU-FIN-022. AP04 and all tiers map EU-FIN-026.

Packet19 mutable-prestate controls map EU-FIN-001, 003, 023, and 024. Each FIN task number matches its EU-FIN suffix. COVERAGE.json contains every exact inventory ID and shared requirement.

Exact prospective schema: `FinancialCaseOracle/1.sourceProfile` distinguishes atomic /1 from successor /5 and /6. `ordinaryInput` stores the canonical JSON text and digest. `financialPre` stores all admitted lifecycle fields and the original array order. `independentExpectedEnvelope` and `actualEvaluatorEnvelope` are complete records, never an ok boolean. `sourceArguments` retains typed action arguments and selector separately.

Private status remains unverified even if public source, compiled UPLC, and native rows later pass. A missing native runtime pin, calibration receipt, or implementation review keeps that predicate unrun. The parent reports implementation18 has frozen separately. This worker neither inspects nor accepts that implementation result.

# First Cardano contract sprint — OpenSpec/EARS requirements

Status: revision2 proposed expected-outcome re-review input; r1 both sessions CHANGES_REQUIRED; specified-only. Author: GPT-6.1. Two independent fresh Meta Muse Code CLI sessions, both requesting `muse-wonder-1.0` (returned identity `muse-wonder-1.0-contributor`), review expected outcomes; they are separate sessions of the same provider/model, not two vendors; this document is not a full source audit or permission to run a ledger command.

## Purpose and retained scope

Deliver one actual Moriarty source/Core → Aiken/UPLC → Cardano ledger escrow contract before expanding the wider 20-family suite. The first contract is the existing profile/5 public single-head custody escrow with release, refund and cancel. This milestone preserves all20 public pattern families, private/PCD, general obligations and the final reviewed matrix/graph as subsequent obligations. Cardano Preview is the public Cardano testnet exit target; it is distinct from Midnight Preview. This sprint does not claim Midnight acceptance.

Current immutable source417 is the 383-row manifest in `t1-gpt61-implementation-417/source-manifest.json`, published by the parent as commit `1db12527f77c991db52f1de6a5f0fdf256185a4f` on Moriarty `utxo`. No source417 change is authorized during this specification handoff. After independent outcome review, GPT-6.1 remains the sole source writer.

## Normative inherited design

Bootstrap155 is a recipe with two scoped approvals160, not ledger approval. Use era-based Conway create-testnet-data with one pool, one on-disk stake delegator, one UTxO funder and one genesis key. Replace only its Byron output using create-cardano with supply omitted; hash genesis files after replacement. Configuration paths are `genesis/<file>` relative to the node-config parent. Total supply45000000000000000 and delegated supply30000000000000000 lovelace; Shelley slot1 second, epoch3600, security18, fee coefficients1/1, KES3600/64; positive future start, fresh accepted magic and fresh owned run directory. Keep the pinned Alonzo/Conway specs, PlutusV3 cost model and approved ceiling values. An unknown linked ABI is recorded unknown, not infeasibility.

Preserve economic design29: original Layer S SHA256 `00c9893ee5d2cbf41adb619bbe2a4b37e4e3f4de64889aa4567bbfe0174ec957`; source profile/5, contract/4, lifecycle/1; Head13 and Lifecycle9; constructor bound48, debit49, open work1024/0/16, post975/49/16, collection cap128 and nonempty-obligation fence. Kernel Cash100, head thread quantity1, fees and lovelace minimums are distinct quantities.

Approved Cash54 source is SHA256 `3f897ac9ef56c01e11d191dba31736da508dc77982ca484f1684e66013b242c4`: parameter is the consumed Cash seed outref, token name43617368, mint100, burns fail, destination is the seed owner's enterprise payment key and that owner signs; staked/script seed fails. The closed fixture policy332f90... remains closed. Head application order remains head seed, Cash policy, lender, borrower, genesis. Design/hash agreement alone never establishes successful minting.

## ADDED Requirements

### Requirement CS01: Exact source-to-script chain
The system SHALL preserve the original source, elaborate it with the profile/5 factory and bind the generated Core, program, schema, bounds and seed identities to fresh compiled and parameter-applied UPLC. WHEN genuine queried-clock parity is admitted, the system SHALL substitute only the approved time guards and SHALL evaluate every terminal with those substituted arguments before body encoding.

#### Scenario: Calibrated sibling
- WHEN an authentic V3 capture supplies genesis zeroTime/zeroSlot and queried tip on a1000ms clock
- THEN release/cancel use [releaseLower, partition−1], refund uses [partition, refundInclusive], source/program identities change, bounds and economic semantics remain unchanged, and actual compileCustody emits the sibling blueprint used by the caller

#### Scenario: Forgery or stale source
- WHEN a transcript, callback, serialized/copied origin label, source, blueprint, application parameter or current head identity is substituted
- THEN the actual consumer refuses before adopting a production body

### Requirement CS02: Current runtime binding
WHEN the system selects production execution, it SHALL construct one checked context from authentic capture, the current queried seed/head/fee/collateral outrefs, freshly applied Cash/head scripts, current protocol query, current era history and genesis identity. Every downstream encoder/evaluator SHALL consume that context. IF any production field resolves to a UNIT policy, fixture script, fixed UTxO, old cache, offline protocol file or caller-supplied proof flag, THEN the system SHALL refuse and emit no production body.

#### Scenario: One identity everywhere
- WHEN Cash mint, head opening and terminal bodies are prepared
- THEN the Cash policy/name, head script/thread policy/name, program/source/schema/bounds/seed, genesis, instance, role addresses and consumed outrefs agree with the same checked context and actual body CBOR

### Requirement CS03: Production Cash and head opening
WHEN funding an escrow, the system SHALL use the approved Cash source applied to its actual available seed and SHALL observe Cash mint settlement before spending its output to open the head. The head seed SHALL be distinct from the Cash seed. The head opening SHALL mint exactly one Head and transfer the actual100 Cash into the bound script with the Head13 open datum. Private keys SHALL stay in the run-owned private paths; public artifacts SHALL contain only public hashes/addresses/outrefs and transaction results.

#### Scenario: Actual funding dependencies
- WHEN the Cash mint succeeds on the ledger
- THEN only its queried settled output can become the head-opening input; an unsigned-body txid is not an available output

#### Scenario: Forbidden seed/policy
- WHEN the seed is missing, staked, script-locked, already spent, wrong-network or bound to the closed fixture
- THEN funding is refused without reporting Cash mint acceptance

### Requirement CS04: Measured costs, minimum ADA and fee convergence
WHEN preparing each mint/open/release/refund/cancel body, the system SHALL evaluate the actual current scripts and body using the pinned supported calculate-plutus-script-cost command and current ledger context, match each returned script hash and redeemer purpose/index, rebuild with those units and reevaluate the resulting body. The system SHALL measure each output's minimum ADA with calculate-min-required-utxo against the same queried protocol and actual datum/assets/reference script. Every encoded output SHALL meet its measured minimum while preserving the approved sponsorship/payout semantics; a fixed permitted amount below the live minimum SHALL cause refusal, not a semantic flag override. The fee SHALL converge using the actual signing-key witness count, collateral, change and reference-script fee inputs. Receipt provenance SHALL bind body/protocol/script bytes; supplied units or a computed flag SHALL NOT qualify as measurement.

#### Scenario: Valid bounded body
- WHEN final evaluation and output minima match the final CBOR and current protocol, fee convergence succeeds and signed size/exunits fit current limits
- THEN submission eligibility is true for that exact body, with measured per-redeemer units, each output minimum, fee and byte size recorded

#### Scenario: Measurement refusal
- WHEN evaluator stdout is a trace-only failure, script hash/index is wrong, measured body differs, protocol is stale, budget exceeds caps, minimum ADA is missing, fee drifts or witness count changes
- THEN signing/submission is blocked and raw stdout/stderr/status are preserved; no fabricated1/1 or800000/400000 cost is adopted

### Requirement CS05: Terminal semantics and amount projection
WHEN the actual source/Core terminal is evaluated, the consumer SHALL explicitly project the evaluated fixed Cash effect amount into its consumer ABI; missing/null/noncanonical/negative/out-of-UInt128 redeemer amount SHALL be refused. Required obligation quantities SHALL not default to zero. Word64 lovelace and UInt128 kernel quantities SHALL remain distinct. Release SHALL pay100 Cash to Lender; refund/cancel SHALL pay100 Cash to Borrower; every post SHALL preserve Head13 identity, transport1/0, ordinary locked0/revision1, ordered IDs and work975/49/16 with no nonempty obligations admitted.

#### Scenario: Original windows
- WHEN original Layer S1000/1099,1100/1100 or1050/1051 is requested on the zero clock
- THEN FIXTURE_WINDOW_UNREPRESENTABLE is returned with original POSIX bounds and encodeSpendBodyCalled false; aligned40000/99999 is a UNIT40/100 probe only

### Requirement CS06: Signed exact-body settlement
WHEN a final body satisfies CS01–CS05, the system SHALL sign only that body, verify the complete witness-key set and required signers, submit the assembled transaction, retain the public transaction ID and original receipt, and query its actual outputs and input consumption. Final settlement SHALL require full independently expected Head13/Lifecycle9 and payout comparison plus named chain confirmation evidence. A mempool acknowledgement or query presence alone SHALL not establish finality.

#### Scenario: Complete effect
- WHEN release settles on the ledger
- THEN the consumed head outref is gone; the winner head/payout outrefs, policy-qualified quantities, role payout credential, all13 datum fields,9 lifecycle fields and remaining/spent/reserve/IDs match the source evaluator

#### Scenario: Wrong effects
- WHEN payout role, asset policy, amount, source/program/instance, work, transport, obligations or ordered IDs differ
- THEN the result is rejected even if total100 Cash or an outref is present

### Requirement CS07: Original-pair race
WHEN testing release/cancel competition, the system SHALL assemble two valid original bodies from one shared head with disjoint fees/collateral, submit both before any poll, and observe the winner without fixing submission order as the winner. It SHALL retain the original losing submit/outcome BadInputsUTxO receipt and complete queried winner post/payout/identity. It SHALL NOT make a third submission to obtain the losing error.

#### Scenario: Missing outcome
- WHEN both original submits are accepted into mempool and no original-pair loser failure can be observed
- THEN the race is incomplete; no third submit or synthetic negative receipt is allowed

### Requirement CS08: Independent admission and honest evidence
BEFORE any real node/genesis/keygen/funding/mint/sign/submit/query/latch action, two fresh independent Meta Muse Code CLI sessions using `muse-wonder-1.0` (record actual returned `muse-wonder-1.0-contributor` identity) SHALL each issue a FULL audit approving the exact-current compiler AND ledger source candidate. Each selected auditor covers both scopes; these are two total audits. Historical/scoped/compiler-only votes and self-review SHALL not count. AFTER actual execution, separate independent actual-result review SHALL assess source/Core/script correspondence, complete effects and failure receipts. UNIT controls SHALL remain unconfirmed with ledgerExecution false.

#### Scenario: Changed source or missing auditor
- WHEN candidate source changes or either full audit is unavailable/rejects
- THEN dependent native actions remain stopped; authorized source repair and read-only inspection may continue

### Requirement CS09: Local-devnet exit
WHEN the approved isolated Conway bootstrap runs after CS08 source admission, it SHALL demonstrate increasing real tip/block height, queried current protocol/era/genesis binding and authentic V3 parity, then execute approved Cash mint/head opening and source-derived release, refund and cancel on separately funded instances. It SHALL execute the CS07 race on another instance. Gross funding, fees, collateral, outputs and remaining balances SHALL reconcile; cleanup SHALL affect only the owned run. Local exit requires all positives, meaningful rejection controls and independent actual-result review; compilation alone does not close it.

#### Scenario: Actual local exit
- WHEN all4fresh source-derived instances satisfy I1 intermediate dependencies, F18 terminal confirmation, complete effects and original shared-head race, and mandatory L01–L03 negatives have actual classified evidence
- THEN independent actual-result review may close the local milestone; missing observer/depth/negative/receipt keeps it open

### Requirement CS10: Cardano Preview exit
WHEN local CS09 passes and the run uses an authenticated current Cardano Preview context, the same reviewed compiler/caller SHALL consume that network's genesis identity, network magic/address domain, actual funded role outrefs and queried protocol/era parameters without substituting local genesis or UNIT pins. Execution SHALL use fresh testnet-only identities and documented authorized faucet/test-ADA resources, preserve old wallets and public money, and satisfy the prospective bounded Preview dependency/resource contract below before spending; no existing executable Preview authority or numeric fee ceiling is evidenced here. A missing resource or endpoint binding is an explicit execution dependency, not a new design-vote loop.

#### Scenario: Preview demonstration
- WHEN one fresh approved escrow contract is compiled/applied for the actual Preview instance
- THEN real Cash mint, head opening and release settle and pass full CS06 comparison with public txids and chain confirmation evidence; a separate instance demonstrates refund, and another demonstrates cancel, with all source-defined action coverage retained

#### Scenario: Preview race and result
- WHEN two competing valid bodies use a fresh Preview head
- THEN the original-pair race satisfies CS07 or is honestly reported incomplete; the full sprint exit stays open until it satisfies CS07 and independent result review passes

### Requirement CS11: Bounded execution and retained failures
The system SHALL use inherited resource limits and the bootstrap/time recipe (1000ms, margin30 slots, windows60 slots, horizon1800s), with the explicitly proposed revision2 preparationSlots900 scheduling amendment below; no existing Preview resource grant is asserted. Each attempt SHALL have a new owned directory, short socket path, consistent child HOME/TMPDIR, source/tool/input pins captured before execution, bounded command/poll/fee-iteration limits and retained stdout/stderr/exit. Repeating a failed defect class twice SHALL trigger a concrete reproducer and changed repair strategy. This sprint SHALL not add a scheduler, audit framework, dependency reinstall, CML WASM modification or new language feature.

#### Scenario: Four-pass failure and retained limits
- WHEN fee/unit/body measurement has not converged by pass4 or query bundles have not stabilized by attempt4
- THEN the body remains ineligible, no fifth pass or native submission is inferred, and the failed attempt and gross resource accounting are preserved

## Normative revision2 concrete predicates

The following complete predicates strengthen CS01–CS11. They resolve r1 ambiguities without dropping the inherited baseline requirements. All referenced interfaces and test IDs are defined in this packet.

# Revision2 executable predicates and required controls

Status: proposed re-review contract. These predicates strengthen CS01–CS11 and the five PLAN tasks. They do not grant source/native approval or claim an existing Preview resource grant.

## Audit object and deterministic sibling (CS01/CS08; Tasks1/4/5)

Choose deterministic audited emitter derivation, not a post-bootstrap fresh-audit deadlock. Both fresh FULL compiler+ledger audits SHALL explicitly cover the exact frozen implementation/dependency/tool manifest, original source/seed/bounds/Cash/bootstrap inputs, parameter schemas, permitted network adapters, deterministic guard-substitution/elaboration/lowering/application functions and all native/negative drivers. Their public verdicts SHALL approve this dynamic-artifact rule substantively, not merely schema/hash validity.

`auditBinding` SHALL contain candidateManifestSha256, implementationSourceManifestSha256, dependencyManifestSha256 and exactly two audit records: canonical review record reference, provider terminal-result reference/hash, public-review hash, audited candidate hash, distinct actual session ID, requested muse-wonder-1.0, actual muse-wonder-1.0-contributor, FULL compiler AND ledger scope and explicit approval verdict. The actual admission consumer verifies these records against parent-verified provider outcomes in the canonical trusted review store and rehashes every candidate row/tool/input; caller-written JSON, self-review, two copies of one session, missing public approval, mismatched exclusions/scope/hash or a source delta fails before any native action. A hash-shaped or copied approval file does not establish a vote.

A runtime sibling is a DERIVED_AUDITED_EMITTER_ARTIFACT. Before funding, recompute the exact sibling bytes from the audited original source and admitted bounds using the audited function; recompute Core/program/schema/bounds/seed identities through the audited factory; compare the actual written source/lowered Aiken input byte-for-byte; retain the actual pinned compiler invocation/check/build output and blueprint binding; verify the definite parameter CBOR/order and resulting script identities with the existing Aiken/CML/CLI agreement. The original source and all non-time text remain unchanged. Body consumers use only those resulting scripts and current instance identities. An altered derived file, nonpermitted bound/network/parameter or handwritten sibling is refused, not automatically reclassified as an audited artifact. Any change to the frozen implementation/input schemas requires two fresh full audits; a runtime value generated by the explicitly approved parameterized emitter does not by itself change the implementation candidate.

## Scheduling amendment and freshness (CS01/CS02/CS04/CS11; Tasks1/2/5)

R1 historical formula is releaseSlot=captureTip+30, partition=releaseSlot+60, refundExclusive=partition+60. Actual417 compile tests took388854ms and290390ms, so a90slot release cutoff cannot safely contain the current compile/funding sequence. R2 proposes preparationSlots=900; releaseSlot=captureTip+900+30, partition=releaseSlot+60, refundExclusive=partition+60.1000ms clock,30slot minimum inclusion margin,60slot windows, forecast horizon1800s and unchanged guard-only substitution are retained. Refund end is1050slots after capture, within1800. Audits and outcome reviewers must explicitly approve the900slot scheduling input; it is not retrospectively called part of r1. If remaining time is insufficient after compilation/funding, refuse TIMING_BUDGET_EXHAUSTED and record failure; do not silently extend guards or sleep beyond forecast. At submission, current slot must be inside the body's half-open interval and before its end; at least30slots must remain in that interval when a new terminal body is first signed. A release/cancel cannot be signed in the refund partition; refund waits for its own interval. Confirmation may finish after expiry if inclusion occurred inside the valid interval.

Clock/parity capture establishes the admitted conversion against the same genesis/domain/era, not a five-slot lifetime for a several-minute compile. After compilation and each intermediate settlement, refresh protocol/era/tip/all selected UTxOs, retaining the admitted clock, audited emitter/artifacts and scheduled window. A measurement receipt records the tip at its final successful body evaluation. Immediately before signing and again before submission, query a stable tip and current protocol/era/required-input state. A stable query bundle has initial/final tip on the same observed ancestry and ≤5slots drift; at most four query-bundle attempts are allowed. Pre-sign/pre-submit tip must descend from the final measurement tip, have nonnegative drift ≤5slots and satisfy the actual validity-window condition. Protocol bytes/cost model, era bytes, genesis/config/magic/domain, all selected input outrefs+address+value+datum, scripts and instance must equal the bound context. Any missing/spent input or role/network/artifact mismatch refuses. Tip drift>5, protocol/era change or rollback invalidates receipt/signature eligibility and requires refreshed context plus reevaluation/minima/fee checks before signing; an era/clock mapping change requires fresh genuine parity and regenerated sibling under the same audited emitter. No stale receipt remains eligible by copying hashes. An already signed body is not submitted after any gate changes. Any source-code change returns to full source audits.

## Authentic issuing and final-body receipt (CS02/CS04; Tasks1/2)

Production context and measurement provenance are nonserialized in-process identities issued only by the audited actual transport/compiler/query/evaluator consumers after canonical audit admission. Keep issuer metadata in private WeakMaps or equivalent noncaller-settable state; do not export a production issuer or accept caller callbacks as real transport. Metadata records `evidenceClass=PRODUCTION_AUTHENTIC_TRANSPORT`, auditBinding and actual child/connection result references. A JSON/hash copy, constructed transcript, custom compile/spawn callback, fixture CLI, UNIT context, offline cost/minima arithmetic or Boolean marker cannot carry issuer identity. Offline structurally perfect arithmetic/emitter positives always return submitEligible:false and ledgerExecution:false. Successful production context/measurement alone also has ledgerExecution:false; only actual settlement evidence can establish ledger effects.

The final receipt binds candidate and two audit identities, genesis/config/domain/network, clock bounds/capture, exact body CBOR SHA+txid+bytes, scripts/application/source/program/bounds/schema/seed/instance, queried inputs, validity start/end slots and corresponding redeemer POSIX lower/upper, scriptDataHash recomputed from exact purposes/redeemers/datums/language views, fee, ordinary input order, collateral outrefs/values/key hashes, totalCollateral, collateralReturn output/address/assets/datum, every ordinary output/change outref/index/address/policy-qualified asset bundle/datum/reference-script hash, per-purpose/index/hash units, each output's measured minimum, reference-script fee size and complete signing-key witness hash set. Verify final signed bytes/witness coverage/size without changing the body. Receipt equality is necessary but insufficient: assertProductionBody requires authentic issuer metadata, canonical auditBinding and fresh pre-sign/pre-submit state. Interval-only, script-data-hash-only, collateral-only or change-only mutation invalidates eligibility before signing/submission.

Exactly four fee/rebuild/evaluate passes are allowed per body attempt; any fee/unit/body change requires full new final-body reevaluation and output-minimum checks within that bound. Nonconvergence is FEE_DRIFT/MEASUREMENT_NONCONVERGENT, no fifth pass. Units/minima/gross/input/change/fees remain exact quantities; no supplied unit shortcut or transaction-byte minADA formula.

## Cash mint and distinct dependency inputs (CS03; Tasks1/3)

A valid Cash mint consumes the actual approved Cash seed and requires the seed owner's enterprise payment credential in the witness/required-signer set. Its selected holding output pays that enterprise owner, has exactly100 of this instance's applied Cash policy/name43617368, no Head or unapproved foreign assets, no stake/script payment credential, and lovelace≥its measured minimum. Preserve the policy's approved allowance for foreign mint on a separate reviewed purpose; it cannot add unapproved assets to this holding output or confuse this Cash identity. Separate change goes to the actual funding owner's address with an exactly reconciled policy-qualified bundle; bind collateral return separately. Required owner/signing paths are actual current identities, not encoding fixtures.

Cash seed≠head seed. Before intended spends, all selected Cash/head/mintFee/headFee/leftFee/rightFee/cancelFee and collateral inputs are distinct; the only allowed repeats are the settled Cash holding carried into head opening, the settled head carried into a terminal, and the one settled head shared between the two original race members. Each race member has disjoint fee/collateral; ordinary inputs and collateral do not intersect. A funding fanout transaction may create these distinct outputs, but cannot manufacture reused or unobserved inputs. After intermediate confirmation, requery the exact original mint holding outref and owner/value; only that exact confirmed output is consumed in the head body. Different same-value outref, wrong recipient, missing owner witness, same seed, staked/script/wrong-network seed or closed policy refuses before normal encoding/sign/submission.

## Operational confirmation and rollback (CS03/CS06/CS09/CS10; Tasks3/5)

Define I1 intermediate confirmation: actual containing block hash,height,slot and raw block/transaction inclusion proof from an authenticated checked chain observation; the exact submitted txid/body must be in that block, and the current observed canonical chain must contain that block with at least1subsequent block. Full output/input comparison and requery still must pass. Both mint→head and head→terminal use I1. Define F18 terminal confirmation: the same inclusion/ancestry/full effect conditions with at least18subsequent blocks (`tipHeight−inclusionHeight≥18`). Apply F18 to standalone terminal results, race winner and local/Preview final exit. These are operational testing rules, not mathematically irreversible finality, not inferred from securityParam18 and not proof of Cardano's generic security assumptions.

The checked observer records exact inclusion block CBOR/header hash,height,slot, txid/body membership, chain intersection/ancestor path and current tip hash/height/slot. It must use an inspected pinned node-to-client chain observation or checked Preview provider connection; no invented cardano-cli query command, caller-supplied block metadata or unit fixture counts as observation. Implement the actual adapter and audit it; if the toolchain cannot supply the required block/ancestry predicate, native readiness/exit is BLOCKED_BY_CHAIN_OBSERVER rather than presumed finality.

Every observed rollback affecting the inclusion/measurement anchor invalidates confirmations and downstream eligibility; reset confirmation tracking from new actual canonical inclusion, requeries and fresh measurements. Lower height, missing ancestor, disappeared output or conflicting confirmed identity triggers reset/refusal. After an intermediate rollback, do not continue spending its predicted output; report dependent transactions' actual states and preserve any already-sent txids. A visible mempool transaction or output at depth0 is not I1; depth1 permits the dependency only after full comparison; depth17 is not F18; depth18 permits the scoped operational confirmed result. Chain-sync depth errors or unavailable ancestry leave results unconfirmed.

## Original shared-head loser proof (CS07; Tasks3/5)

Both original race bodies must be measured/fresh and valid against the same checked candidate/genesis/instance/shared-head state with overlapping half-open validity intervals before either submission. Fees/collateral are distinct and available at that gate. Exactly two original submissions precede every poll. Preserve each body's exact CBOR/txid/digest, timestamp, required inputs, original command/result and any original-pair outcome record.

A valid loser record is (a) a typed BadInputsUTxO result at that loser's original submit, or (b) an authenticated original-pair outcome explicitly tied to that original txid/body, without resubmitting. Its parsed missing-input set must identify the exact shared head consumed by the confirmed winner, with no unavailable loser fee/collateral/other input as an alternate cause. Bare error-name text, missing cause details, different-input failure or unrelated txid does not qualify. Race complete requires exactly one F18-confirmed full expected winner consuming the same shared head plus that original loser record. Submit-time loser evidence may be recorded before winner confirmation but completes only after F18 winner/source/effect comparison.

Exhaustive classification: one valid confirmed winner+shared-head-specific original loser = COMPLETE_ORIGINAL_PAIR; exactly one winner but both submits accepted and loser silent/dropped/no receipt = INCOMPLETE_LOSER_RECEIPT; both pending/no winner = INCOMPLETE_WINNER; multiple winners/conflicting chain evidence = INCOMPLETE_AMBIGUOUS; wrong state/payout/source or fee/collateral/nonshared-head failure = INCOMPLETE_CAUSE_OR_POST. UNIT remains unconfirmed/ledgerExecution:false regardless of code. No outcome permits a third submission to manufacture a receipt.

## Preview dependency/resource contract (CS10; Task5)

Preview currently has no executable binding/resource grant evidenced in this packet. Status is BLOCKED_BY_PREVIEW_DEPENDENCIES. Before any Preview spend, the parent must bind a prospective `preview-execution-dependency.json` to the actual user-authority reference and a fresh checked endpoint/config, faucet/test-ADA source and runtime identities. Creating a filled record is not itself user authorization and does not invent an existing grant. No repeated design vote is required for routine repair; absent authority/binding is a concrete execution dependency.

Required record: authorityRef+hash+scope; endpoint/config paths+hashes+actual Cardano Preview genesis/networkMagic/domain; faucet/test-ADA source reference and funding receipts; Word64 canonical decimal values for perTxFeeCeilingLovelace, aggregateFeeCeilingLovelace, aggregateGrossDebitCeilingLovelace and aggregateCollateralLiabilityCeilingLovelace; per-instance allocations for release/refund/cancel/race with separate Cash seed/head seed, mint/head/terminal fee and collateral outrefs, ADA output sponsorship/minimum/change reserves, and the race's two separate fee/collateral allocations. Each of4instances has100 kernel Cash under its actual instance's seed-applied policy; total intended kernel allocation400Cash, not400lovelace. The record lists individual exact test-ADA allocations and checked totals, planned maximum transaction counts (including negative tests), per-command/poll timeout limits and four-pass measurement limit. The requested numerical envelope below is prospective and unapproved; the executable dependency record keeps authority/bindings/actual allocations null until supported by actual authority and funding observations. Any missing executable field blocks execution.

Prospective numerical resource request for review (NOT an existing grant): per-transaction fee ceiling5000000lovelace; aggregate gross fee ceiling100000000; aggregate gross debit ceiling2000000000; aggregate worst-case collateral liability ceiling200000000; requested testnet-only faucet liquidity450000000lovelace. Proposed per-standalone-instance ADA allocation90000000 = Cash seed5000000 + head seed5000000 + mint fee10000000/collateral10000000 + head fee10000000/collateral10000000 + terminal fee10000000/collateral10000000 + output-minimum reserve10000000 + separate change reserve10000000. Release/refund/cancel each use that allocation. Race allocation110000000 adds another fee10000000 and collateral10000000 for its second original member. Thus4instance allocations total380000000. Preview phase1 negative allocates fee5000000 and collateral10000000 (15000000), and funding transaction fee/change reserve10000000; planned allocations405000000, remaining liquidity reserve45000000. Planned collateral outpoints total140000000 (four instances120000000, race extra10000000, negative10000000), within the prospective200000000 collateral-liability ceiling. Reserve is not authorization or a refund of gross fees. Planned positive submissions13, negative1, funding fanout maximum2, total maximum16;100000000 aggregate fee ceiling accommodates16×5000000 with20000000 remaining ceiling margin. Before execution, actual measured allocations/minima/collateral percentages and every funding/debit/reserve total must be checked against this request and the supported authority; otherwise remain BLOCKED_BY_PREVIEW_DEPENDENCIES. A reviewer approval of this proposed envelope is not user spending authority. No faucet receipt, endpoint or authority is fabricated.

Check before every sign/submit: measured fee≤perTx ceiling, cumulative gross fees including failed/negative transactions plus reserved outstanding liabilities≤aggregate fee ceiling; gross debits and worst-case collateral liabilities≤their aggregate ceilings. Refunds/change/returned collateral never erase gross accounting. Actual available input balances must cover the complete4instance plan and minima/fees/collateral reserves before beginning; partial funding does not close Preview exit. Fail closed when fees increase beyond reserves or a ceiling is missing/exceeded.

Preview sequence independently queries its real genesis/config/protocol/era/tip/roles, executes a genuine V3 parity probe against the actual Preview context, applies the approved900slot preparation schedule, generates/revalidates/recompiles its sibling through the audited emitter and applies fresh Preview seeds/roles. Never reuse local genesis, capture, seed/outref, scripts/instance or authority record. Refresh measurements as specified above. One deterministic sibling blueprint for one schedule may be applied to4fresh instance seeds/roles under the audited schema; it does not permit sharing issued instance/body receipts. Prepare/fund these instances within that same schedule and reject an exhausted window. Then run4instances and require I1/F18/full race/effect/budget reconciliation and separate independent actual-result review.

## Required test IDs and placement (CS05/CS09/CS11; Tasks1–5)

All offline controls call actual exported consumers; injected outputs remain UNIT and cannot grant eligibility. Mandatory offline IDs:

- O01 wrong candidate, O02 missing/mismatched audit hash, O03 duplicate audit session, O04 public verdict lacks full compiler+ledger scope, O05 byte-perfect serialized UNIT context/measurement, O06 production compile/spawn callback, O07 altered sibling source/program/blueprint/application, O08 context reused across instance/network.
- O09 tip drift6slots (5allowed), O10 protocol/cost-model change, O11 era change, O12 seed/head/fee/collateral spent between measurement and signing, O13 rollback or missing measurement ancestor, O14 interval-only mutation, O15 script-data hash mutation, O16 collateral/return/change-only mutation, O17 witness-key or final signed-size mutation, O18 missing output-minimum or four-pass nonconvergence. Each refuses eligibility and normal sign/submit; invalid source/seed/amount inputs also prevent normal encoding.
- O19 wrong mint recipient, O20 missing seed-owner witness, O21 same Cash/head seed, O22 staked seed, O23 script-locked seed, O24 wrong-network/closed seed, O25 changed same-value mint holding outref, O26 shared fee/collateral or ordinary/collateral collision, O27 wrong holding/change asset bundle.
- O28 amount missing, O29 null, O30 leading-zero/fractional/noncanonical, O31 negative, O32 UInt128 overflow, O33 required obligation quantity absent (not zero), O34 invalid obligation quantity, O35 Word64 overflow coin, O36 valid UInt128 value wrongly used as Word64 coin, O37 valid Word64/kernel boundaries preserving exact decimal bytes; retain68 quantity controls.
- O38 calibrated release/cancel upper=partition−1 accepts exact full release window; upper=partition refuses; O39 release lower=partition refuses, refund lower=partition accepts exact refund window; O40 refund upper=refundInclusive accepts, upper=refundInclusive+1 refuses; O41 cancel partition−1 accepts exact window, partition/refund window refuses; O42 expired schedule/insufficient30remaining slots refuses. Single-point POSIX pairs may be unrepresentable: valid bounds assertions use complete slot-aligned windows and do not weaken rounding fences.
- O43 mint presence/depth0 blocks head; I1depth1+ancestry+full mint comparison permits dependency, O44 final depth17 remains unconfirmed/depth18 permits F18 only with full effect, O45 rollback resets both stages, O46 original submit-time typed shared-head loser+F18 full winner completes, O47 accepted then silent loser stays incomplete, O48 missing-input set names fee/collateral/different head stays incomplete, O49 ambiguous/wrongpost winner, O50 both body measurement contexts/interval overlap mismatch. Post-submission O46–O49 controls assert submitCount2 and no resubmit; O50 preflight mismatch asserts0submits and no resubmit.

Mandatory real local-devnet positives: genuine clock/emitter/measurement and4instances Cash mint/head opening/release/refund/cancel/race, I1 dependencies and F18 terminal winners. Mandatory local on-ledger negatives: L01 replay/shared-head consumption via the original losing race submit; L02 missing role witness/required signer phase1 rejection; L03 wrong payout recipient with an otherwise valid body, source script evaluation/ledger phase2 refusal (record actual collateral disposition). L02/L03 use a separately audited negative-test driver with declared expected invalid predicate and bounded resource accounting; it does not issue normal submit eligibility, count rejected intent as settlement or bypass gates for positive bodies. If pre-submit evaluator catches the wrong payout, retain that as offline/evaluation evidence; obtain actual ledger rejection with the separately audited negative driver to close L03, or leave that mandatory negative open. No fake rejection logs.

Mandatory Cardano Preview: genuine Preview parity/recompile/context;4instances full positives and original-pair shared-head rejection P01; actual missing-role witness P02 phase1 rejection using the audited negative driver. Local L03 is mandatory phase2 rejection evidence; duplicate phase2 rejection on Preview is not required and does not silently stand in for other outcomes. All remaining hostile quantity/forgery/window mutations are mandatory offline, not public collateral-burning tests. Record which named predicate actually rejects each case, real command receipts and collateral effects. O01–O50, L01–L03 and P01–P02 remain mandatory classified controls; failures/execution unavailability stay open, never converted into semantic success.

## Exit matrix

| Stage | Required result | Current417 status |
| --- | --- | --- |
| Expected outcomes | Independent Meta Muse Code/Wonder session review of this spec/plan | Pending; not source admission |
| Offline implementation | Actual consumer positives and independent negatives; full compile/build/apply | Partial, UNIT only |
| Full source audits | Two fresh exact-current full compiler+ledger approvals | Missing |
| Local ledger | CS01–CS09 with actual runtime receipts and result review | Unexecuted |
| Cardano Preview | CS01–CS08 andCS10; explicit binding/authority/faucet/funding/ceilings, genuine Preview parity/recompile and all action/race results/result review | BLOCKED_BY_PREVIEW_DEPENDENCIES; prospective values unresolved |
| Wider program | All20/privatePCD/general obligations/final matrix/graph | Retained open |

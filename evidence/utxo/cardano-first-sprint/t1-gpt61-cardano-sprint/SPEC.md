# First Cardano contract sprint — OpenSpec/EARS requirements

Status: proposed expected-outcome review input; specified-only. Author: GPT-6.1. Two independent fresh Meta Muse Code CLI sessions, both requesting `muse-wonder-1.0` (returned identity `muse-wonder-1.0-contributor`), review expected outcomes; they are separate sessions of the same provider/model, not two vendors; this document is not a full source audit or permission to run a ledger command.

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

### Requirement CS10: Cardano Preview exit
WHEN local CS09 passes and the run uses an authenticated current Cardano Preview context, the same reviewed compiler/caller SHALL consume that network's genesis identity, network magic/address domain, actual funded role outrefs and queried protocol/era parameters without substituting local genesis or UNIT pins. Execution SHALL use fresh testnet-only identities and documented authorized faucet/test-ADA resources, preserve old wallets and public money, and record the existing resource authority and fee ceiling before spending. A missing resource or endpoint binding is an explicit execution dependency, not a new design-vote loop.

#### Scenario: Preview demonstration
- WHEN one fresh approved escrow contract is compiled/applied for the actual Preview instance
- THEN real Cash mint, head opening and release settle and pass full CS06 comparison with public txids and chain confirmation evidence; a separate instance demonstrates refund, and another demonstrates cancel, with all source-defined action coverage retained

#### Scenario: Preview race and result
- WHEN two competing valid bodies use a fresh Preview head
- THEN the original-pair race satisfies CS07 or is honestly reported incomplete; the full sprint exit stays open until it satisfies CS07 and independent result review passes

### Requirement CS11: Bounded execution and retained failures
The system SHALL use existing approved resource limits and the bootstrap/time recipe (1000ms, margin30 slots, windows60 slots, horizon1800s). Each attempt SHALL have a new owned directory, short socket path, consistent child HOME/TMPDIR, source/tool/input pins captured before execution, bounded command/poll/fee-iteration limits and retained stdout/stderr/exit. Repeating a failed defect class twice SHALL trigger a concrete reproducer and changed repair strategy. This sprint SHALL not add a scheduler, audit framework, dependency reinstall, CML WASM modification or new language feature.

## Exit matrix

| Stage | Required result | Current417 status |
| --- | --- | --- |
| Expected outcomes | Independent Meta Muse Code/Wonder session review of this spec/plan | Pending; not source admission |
| Offline implementation | Actual consumer positives and independent negatives; full compile/build/apply | Partial, UNIT only |
| Full source audits | Two fresh exact-current full compiler+ledger approvals | Missing |
| Local ledger | CS01–CS09 with actual runtime receipts and result review | Unexecuted |
| Cardano Preview | CS01–CS08 andCS10 with all action/race results and result review | Specified-only |
| Wider program | All20/privatePCD/general obligations/final matrix/graph | Retained open |

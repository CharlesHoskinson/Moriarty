# Design: source-to-spending correspondence

Status: S2 written design. Recommendations below are specified-only. Repository observations are bound to input-hashes.json.

## Inputs and authoritative observations
Use source text, action name, canonical snapshot, admitted financial prestate, exact profile bounds, and runtime pins. Core is derived from source, never supplied as trusted input.

`financial-agreement-source-v5.ts` routes to expression contract /4 in `financial-expression-v1.ts`. `/6` routes to contract /5 in `financial-expression-v5.ts` and lifecycle-state/2 in `financial-lifecycle-v2.ts`. Contract /5 uses `funded-expression-source-v2.ts` staged lifecycle binding. `financial-lifecycle.ts` defines Transfer, Repay, Originate, and Accrue; `funded-expression-source-v2.ts` admits Transfer, Repay, Originate, Accrue, and Fee descriptors. Fee maps through Transfer-shaped fields, then retains Fee kind in lifecycle-state/2. `financial-lifecycle-v2.ts` applies Fee through its transfer kernel with distinct zero/self-transfer policy. Allocate is not an independent descriptor in these selected source profiles. Allocation identifiers and allocation rules are enforced by Repay. Any separate Allocate selector must remain unsupported until its actual profile binding is demonstrated.

`codec.ts` hashes `UTF8(domain) || 00 || UTF8(canonicalEncode(value))`; raw source uses raw bytes instead. Canonical values permit records, arrays, strings, and booleans. JSON numbers, null, accessors, prototypes, sparse arrays, duplicate canonical keys, and lone surrogates are excluded. Record keys sort; arrays retain order.

Revision03 representation evidence includes full HeadDatum decoding in both independent design reviews. The original ABI text describes narrower author measurements. Preserve that distinction. No design review compiled a spending validator.

`compile-custody.mjs`, `admit-custody.mjs`, `lower-custody.mjs`, and `custody-time.mjs` are moving/partial implementation observations. Their recorded SHA values are observations, not a freeze or approval.

## Proposed interfaces and output
Proposed `routeFinancialSource({sourceText, profile, capabilities})` selects the existing source factory. It returns derived schema/Core or `{ok:false, layer, code, span}`.
Proposed `inventoryFinancialCore(elaborated)` returns ordered constructor paths, type rules, effect schema dependencies, and work rules.
Proposed `lowerFinancial({elaborated, inventory, bounds, bindings, abiVersion})` returns Aiken modules and a correspondence manifest.
Proposed `compileFinancial({sourcePath, seedPath, boundsPath, runtimePinsPath, outputDir, capability})` returns the manifest and checked blueprint only after required checks.
These signatures are planned interfaces. They are not callable today.

The manifest contains profile, contract, lifecycle schema, action tags, constructor rules, source and generated hashes, full dependency pins, bounds preimage, work model, instance identity recipe, supported capabilities, refusal layers, and result evidence paths. It records Aiken source SHA-256 separately from UPLC bytes and Cardano blake2b-224 credentials.

## Core lowering table
Read the selected contract OPERANDS, READ_OPERANDS, and POST_READ_OPERANDS tables. Compare the inventory with source elaboration. Generate rules for every constructor admitted by that capability.

General constructors: Require, Let, NextWrite, Ensure, Emit; LitUInt, LitSInt, LitBool, LitText, LitAmount, LitQuantity, LitShares, LitRate, LitPrice; ReadLocal, ReadPre, ReadArg, ReadObs; ConstructRecord, ConstructEnum, ConstructSome, ConstructNone, ConstructCollection, ConstructAmount, ConstructShares, ConstructVariant; ProjectField, AccessField, ProjectIndex, AccessIndex, ProjectVariant, ProjectSome; ConvertUInt, ScalarValue, Select; Add, Sub, Mul, FloorDiv, CeilDiv; Eq, Lt, Lte, Gt, Gte, Not, And, Or.
Financial reads: ReadOutstanding, ReadPrincipal, ReadAccrued, ReadBalance, ReadAllowanceRemaining, ReadAllowanceSpent, and their ReadPost counterparts.

T1 admits only constructors observed in its actual source: Require, Eq, ReadPre, LitUInt, ReadArg, LitText, Lte, Gte, Lt, Emit, ConstructRecord, ConstructAmount, NextWrite, Add, Ensure. T1 rejects Let, FloorDiv, observations, nonempty obligations, and unsupported extra arguments. General rules above require a separately reviewed amendment; listing them does not enlarge active T1.

Preserve type domains, operation ordering, local scope, write classes, post scope, short-circuit semantics, and path-sensitive runtime cost. Use the source evaluator's arithmetic bounds before narrowing unbounded Plutus integers. Quantity unit vectors, scale, signed magnitude, AmountProduct, scaled amounts, and conversion provenance remain type obligations.

## Closed revision03 ABI
Record constructors use Constr 0. Lifecycle fields are schema_version, balances, allowances, obligations, used_transfer_ids, used_allocation_ids, used_origination_ids, used_accrual_ids, work. Balance fields: party, asset, amount. Allowance fields: party, asset, remaining, spent. Work fields: remaining, spent, closure_reserve.

Obligation fields are id, debtor, creditor, denomination, settlement_asset, principal, accrued, outstanding, allocation_rule, conversion, status, origination_id, origination_transfer_id, initial_principal, nominal_liability_cap, liability_incurred, accrual_terms, last_accrued_period, next_accrual_at. Conversion fields: mantissa, scale, rounding. Accrual fields: numerator, denominator, rounding, period_seconds, first_period_start.

Enum indices: RoundingNone/Floor/Ceil 0/1/2; AccrualFloor/Ceil 0/1; AccrualFirst/PrincipalFirst/ProRata 0/1/2; Outstanding/Settled 0/1. Redeemer Release/Refund/Cancel is 0/1/2, with actor, recipient, transfer_id, lower, upper fields.

HeadDatum fields: program_hash, bounds_hash, source_hash, schema_hash, seed_hash, domain_network, domain_deployment, instance_id, transport_revision, transport_remaining, ordinary_locked, ordinary_revision, lifecycle. No work_hash slot exists. Typed empty obligations are still authenticated data.

Amounts are canonical nonnegative decimal strings before BigInt conversion. Balance, allowance, work, and conversion values use UInt128. Liability fields use nonnegative signed-128 cap. Obligation period fields use UInt64. Identifier syntax is `[A-Za-z][A-Za-z0-9_]{0,63}`. Schema version is exact lifecycle text, not an identifier. Hash byte lengths and UTF-8 checks apply independently of semantic equality. Preserve array order, including a newly appended balance row.

## Kernel and work correspondence
Execute source prefix, staged selected financial kernel, source suffix Ensure, then the source work debit. Preserve all candidate state and complete ordered effects. The current T1 source counts 48 Core nodes plus one Transfer kernel unit. That observation must be recomputed, never compiled as a fixed universal cost.

Early failures retain source workUsed 4, 8, 12, 16, 20, or 24 where the packet observes them. `WORK_EXHAUSTED`, `INVARIANT`, and `OVERFLOW` are different source results. Ledger refusal cannot replace their names. StaticWorkBound is an upper bound; actual visited work is the debit. Plutus CPU/memory budget is a separate measured resource.

T1 transport revision increments and remaining decrements independently. Ordinary locked/revision follow actual NextWrite. Financial work follows the source debit. General sources need a versioned ordinary-field layout; do not squeeze arbitrary fields into T1 locked/revision slots.

## Runtime pins and acyclic self identity
The native-runtime package owns actual generation and pinning. Its future procedure uses a fresh authorized namespace. Generate payment verification keys with `cardano-cli address key-gen`. Keep signing keys outside evidence. Derive public role hashes with `cardano-cli address key-hash --payment-verification-key-file`. Compile policy scripts with pinned Aiken and derive policy ids with `cardano-cli conway transaction policyid --script-file`. Pin public script bytes and vkey digests. Require lowercase 56-hex, nonzero, distinct Cash/thread policies, distinct roles, and fixed asset names Cash `43617368`, Head `48656164`.

Select the actual genesis UTXO from readback. Pin its observed 64-hex transaction id and index. Never generate a fake txhash. Recompute the instance record using domainNetwork, domainDeployment, threadPolicyId, threadAssetNameHex, cashPolicyId, cashAssetNameHex. Recompute the MORIARTY-INSTANCE-financial-agreement/5 digest.

Self identity is a real dependency question. Proposed ledger/compiler amendment: derive the spending credential from authenticated own input ScriptContext. Compile spending first. Derive genesis/thread policy from the spending hash, actual seed, and genesis commitment. Pin actual policies, role hashes, and instance afterward. Bind policy authority through the genesis-authenticated head rather than cyclic literals. Ledger worker proposes PolicyHashWitness/1 for bootstrap authentication. The spender reconstructs canonical applied thread-policy bytes from its pinned policy template, own spending credential, seed outref, and GenesisCommitment. It hashes those bytes with the measured Cardano script-hash domain and matches the input NFT policy. Caller-supplied arbitrary script bytes are insufficient. GenesisCommitment excludes scriptHash, threadPolicyId, and instanceId. The thread policy binds spender hash and full genesis, derives its own policy id from minting purpose, and checks final instance identity. Canonical parameter-application parity and bounded on-chain hashing feasibility remain required measurements. An immutable registry is a separate reviewed alternative. The bootstrap authentication relation needs separate review. This amendment is not part of the approved T1 protocol or current implementation. Compare the retained role mapping at execution. Do not embed the final spending hash into bytes that determine that hash. Policies that depend on the spending hash also need an acyclic reviewed recipe. Stop native compilation on a cycle. This recommendation requires a separate design decision if it changes revision03 pins.

## Trust boundaries and dependency ownership
Ledger-state owns constrained genesis, consumed heads, continuation identity, durable replay across heads, split/join custody, migration allowlists, termination, and gross UTXO debit matching. Finance owns operation policy fixtures, amount quantum conversion, exact rounding, fees, accrual, allocation, repayment, intent net goals, and residual duties. Compiler preserves those semantics and emits binding checks. Native-runtime owns real compilation pins, POSIX parity, actual compiled UPLC execution, submission, and readback. Conformance/private owns independent claim binding, proof authority, private data, mandatory PCD, and global release reconciliation.

ReadObs is a seam, not oracle truth. Require an authenticated reference binding with policy, NFT, datum, freshness, and interval conditions from ledger/native packages. Joint-role Args are not signatures. Require actual extra_signatories. Fee effects are not Cardano txfee. Wallet fees, collateral, sponsorship, and source balances stay distinct.

## Evidence and failure handling
Source envelope equality establishes only source behavior. Lucid/Aiken decode establishes representation. Generated Aiken unit execution establishes that test predicate. Actual compiled UPLC evaluation establishes that script predicate and context. Native readback establishes observed inclusion and resulting UTXOs. None establishes quantified compiler correctness, oracle truth, native recursion, private PCD, or Complete.

Remove only the candidate output's stale success artifacts before compilation. Keep historical failure artifacts intact. Retain stdout, stderr, exit code, pinned tool versions, test names, UPLC bytes, execution budgets, and input contexts. Reject missing tests even when the tool exits zero. Publish a success manifest only after check, build, decoding, and required generated-entrypoint tests pass.

## Independent fixture plan
Consume the immutable packet after implementation freezes. Compare every sourceBindings envelope, not just E11/E13. Positive variants include lender balance 7 yielding 107, borrower balance 3 yielding 103, Witness 5 retained, allowance 11/2 retained, PriorSpend retained, ReleaseAlt, ordinary revision 4 yielding 5, additive work 151/59, exact work 0/49, closure reserve 1, and missing lender row appended. Negative cases include reused id, insufficient balance/allowance, overflow, wrong schema, malformed amount/id, guard failure, wrong tag, and E12 refusal at the T1 fence.

Test sketch, proposed harness only:
```js
for (const binding of packet.sourceBindings) {
  const generated = await executeGeneratedSpending(binding, frozenManifest);
  if (binding.envelope.status === 'FundedExpressionPrepared' && admitted(binding)) {
    assert.deepEqual(generated.ordinaryPost, binding.envelope.post);
    assert.equal(canonicalEncode(admitState(generated.financialPost)),
                 canonicalEncode(admitState(binding.envelope.financialPost)));
    assert.deepEqual(generated.effects, binding.envelope.effects);
    assert.equal(generated.workRemaining, binding.envelope.workRemaining);
  } else assert.equal(generated.spendingSucceeded, false);
}
```
`executeGeneratedSpending`, `admitted`, and `admitState` are proposed test adapters. Their implementation must use the actual generated spending entrypoint and existing lifecycle admission; a helper-only oracle cannot satisfy this test.

## Amendments and unresolved design questions
Preserve revision01/02 CHANGES_REQUIRED and revision03 T1 approval on original bytes. General /6 schema, dynamic ordinary fields, full kernel lowering, quantum support, and Split/Join/Migrate/Terminate are new amendments. Their independent review cannot reuse T1 votes.

Resolve the acyclic script identity recipe before native pin compilation. Decide whether general ordinary fields use a schema-derived record or a versioned typed map. Pin actual per-operation work schedules for repayment allocation and Fee in lifecycle-state/2. Keep standalone Allocate unsupported without demonstrated source binding. Determine the authoritative bounds for unused Text and typed collection encodings. Determine how generated spending tests expose complete effects without trusting a host-computed acceptance flag. These questions block their dependent capabilities, not narrow T1 source drafting.

## Integration dependency updates

Finance proposes FinancialCaseOracle/1. It binds profile, contract, lifecycle, action selector, Core, schema, bounds, input hashes, and complete independent/evaluator envelopes. Compiler C07 and C11 consume that proposed interface. Interface agreement does not establish executed correspondence.

The primary reports implementation18 terminal freeze manifest SHA-256 `aed4090af8f1e94e3904140ac2b23be9aa79e85141806841f1066115e5386680`. Its stated scope is source-UPLC unit only. Native pins remain false, calibrated source is absent, and the blueprint script hash differs from the e5 unit identity. Fresh audits are pending. Earlier moving-source hashes remain timestamped observations. They do not become approval or candidate identity.

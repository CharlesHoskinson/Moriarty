# Successor candidate04 partial source review: BLOCKED

**Source: BLOCKED. Task: BLOCKED** separately for author exit124 and missing actual serving identity. Requested Grok4.6 high does not establish actual serving identity.

Candidate `4c2ab0e340e9eb768e27b330a3af8a013c54116a260687321619e1fa44aa15dd`;704894bytes;8ownedfiles. All8owned+31protected+3atomic pins matched before and after. Scope: proposed bounded contract/schema/generator/fixtures only. No full-map, BNF, K, runtime, native, proof, signature, ledger, network, semantic-freeze or sprint acceptance.

Independent successes: **965 passed / 0 failed**; two generations byte-identical to retained312975bytes; all84catalog hashes/types;613exact ordered display leaves; correct swap cash, accrual41095890 and inherited7→6work. Root902checks are separately attributed and did not execute contextual negatives.

## C04-F1: The bounded Core is still incomplete and some primitive equations are underdetermined

Prior C03-F1. Paths: `candidate-04-partial/experiments/moriarty-language/spec/successor/semantic-contract.md:253`; `candidate-04-partial/experiments/moriarty-language/spec/successor/signing-display-schema.json#/x-coreSignatures`; `candidate-04-partial/experiments/moriarty-language/spec/successor/signing-display-schema.json#/x-exprSignatures`; `candidate-04-partial/experiments/moriarty-language/spec/successor/signing-contract.test.py:920`.

- The 38 Core rows now have named parameters/results, and Genesis arity 2 and vault/holder/share identities are corrected. However, x-exprSignatures has only 8 rows. The 32 other declared constructors, including all literals, four reads, arithmetic/Boolean operators, Ensure and Emit, have no full constructor signatures there. Tests enumerate only five expression constructors.
- RequestFulfill.frameUpdate says 'filled+=fill; maybe Claimable'; no exact status boundary is specified. RequestClaim uses delta/already not bound by its named claimed operand. MessageRefund says 'refund; tombstone unreceived' although Message has no refund asset/amount field. RewardAccount.pendingEntitlement is an Amount but RewardAccrue takes an unqualified UInt128 delta and RewardClaim lacks a complete asset/source update. EventClaimSettle credits claimant without the corresponding source debit or precise payout calculation.
- For a resolved claim with stake 1 AssetA, the listed EventClaimSettle preconditions also describe a payout of 1000000 AssetA (matching asset); the exact admissible payout/source equation is missing. This is an underdetermined proposed interface, not a claim that an implemented evaluator accepted such a payout.

**Required correction:** Complete every declared constructor's operands/results, overloads, bounds and evaluation rule. Replace 'maybe'/unbound variables with exact primitive prerequisites, identity/asset bindings, all state updates and frames. Reconcile these rules with the closed records. Do not defer the requested finite Core contract to a future surface grammar.

## C04-F2: Debt identity metadata still depends on argument position

Prior C03-F2. Paths: `candidate-04-partial/experiments/moriarty-language/spec/successor/generate-signing-examples.py:383`; `candidate-04-partial/experiments/moriarty-language/spec/successor/signing-display-schema.json#/x-metadataResolution`; `candidate-04-partial/experiments/moriarty-language/spec/successor/signing-contract.test.py:985`.

- The retained Price mantissa now correctly displays 'AssetB per AssetA at scale 4'. The retained debt_id at argument 0 also displays ObligationId.
- Independent probe: reverse the two existing accrue arguments (qty, debt_id), recompute executionBodyHash, and validate the ExactPlan schema. Schema passes. project_display returns executionBody.action.arguments.1.value.value = Loan01 with label Value, unit typed-leaf, role payload and no identity. The schema's metadata rule says FieldBinding name debt_id determines metadata; it does not restrict this to index 0.
- The generator condition is path.endswith('action.arguments.0.value.value'), and the test's expected lookup also names index 0.

**Required correction:** Resolve the text leaf through its enclosing named FieldBinding regardless of array index. Add an independent valid reordered-argument discriminator. Preserve the corrected Price unit and the exact typed leaf coverage.

## C04-F3: Initialization and loan admission are not a complete retained funded history

Prior C03-F3. Paths: `candidate-04-partial/experiments/moriarty-language/spec/successor/generate-signing-examples.py:1192`; `candidate-04-partial/experiments/moriarty-language/spec/successor/generate-signing-examples.py:1211`; `candidate-04-partial/experiments/moriarty-language/spec/successor/signing-examples.json#/preimageRegistry/genesisBody/declaredFields`; `candidate-04-partial/experiments/moriarty-language/spec/successor/signing-examples.json#/preimageRegistry/loanGenesisSuccessor`; `candidate-04-partial/experiments/moriarty-language/spec/successor/semantic-contract.md:347`.

- Swap exactWrites now name pool_a/pool_b and cover all five declared genesis fields. Independent transfer replay matches all four swap balances and unchanged financial collections.
- loanGenesisExecutionBody writes loan01 and alive, but loan_gen_doc uses the same genesis_hash as the swap GenesisBody. That body's declaredFields are trader_a,pool_a,pool_b,trader_b,alive: loan01 is undeclared and four declared fields are omitted. Giving the loan a new nonce/successor does not supply a distinct declaration/bootstrap.
- loanGenesisSuccessor.originalIntentDigest is 478d9d511e78ac5e94fcf64c5bbb78cedddf6e115b05f14c384169871866e626. No retained catalog body or valid signed document is its preimage. The generator constructs loan_gen_doc transiently but never retains it.
- Genesis initial writes bind 1100000 AssetA and 2000000 AssetB allocated from an empty unborn balance list; the only effect is Genesis(instanceId, initialWork, claimRoot). No exact funding/input or declared initial-mint rule identifies the source of those balances. Loan initialization creates a 5000000000 liability with no retained funding/creation derivation. The source-only pool-custody assumption is not a complete initialization funding record.

**Required correction:** Retain a separately declared, fully committed loan genesis document/body/history (or the full authorized debt-creation transition) and its funding rule. Bind exact initial allocation/funding for every genesis balance and liability. Replay declaration equality, fields, all financial collections, effects, authority and predecessor identity rather than checking only writes and three work copies.

## C04-F4: Cancellation duty and backing remain disconnected from an admitted transition

Prior C03-F4. Paths: `candidate-04-partial/experiments/moriarty-language/spec/successor/generate-signing-examples.py:1795`; `candidate-04-partial/experiments/moriarty-language/spec/successor/generate-signing-examples.py:1823`; `candidate-04-partial/experiments/moriarty-language/spec/successor/signing-examples.json#/ledgerExamples/shortfallCancellationDuty`; `candidate-04-partial/experiments/moriarty-language/spec/successor/signing-examples.json#/invalid/id=inv-unfunded-cancellation`; `candidate-04-partial/experiments/moriarty-language/spec/successor/semantic-contract.md:777`.

- The signed ActorLedger now represents inflow 0, fees 30, net -30. The example shortfall duty names creditor trader, debtor pool, AssetB 30; these correct the prior recipient-as-debtor error at the record/prose level.
- shortfallCancellationDuty is a standalone ledger example. Its named-balance backing says pool AssetB 1980257, but it has no admitted state/capability/reservation reference, and no full cancellation result retains that duty and backing. The ordinary swap state associated with 1980257 has trader net 19743, hence no 30 deficit; the fee-shortfall state would instead have pool 1980287 and trader net 19713.
- inv-unfunded-cancellation selects cancelPreStateBody: trader AssetB balance/inflow is 0 and the retained goal is 19743, but its isolated unfundedDuty amount is 30. This does not derive the claimed deficit from that state. That state also contains a pool AssetB balance of 2000000; omitting the optional backing property alone does not establish a complete failed backing predicate and all its authority prerequisites.
- The input-fee intent changes top-level netGoals to AssetA zero while residualAuthority.netGoals and the admitted prestate still demand AssetB 19743. The proposed signed-net arithmetic is improved; the contextual authorization/goal match is not complete.

**Required correction:** Provide the actual prestate, authorized payer capability, same-asset deficit calculation and enforceable backing reservation for a funded cancellation and a minimally altered unfunded sibling. Rebuild every dependent state/hash. Make both copies of goals agree. Reject missing/insufficient or multiply committed backing through computed predicates, not fixture flags.

## C04-F5: Inherited work is corrected but charge reconstruction remains self-reported

Prior C03-F5. Paths: `candidate-04-partial/experiments/moriarty-language/spec/successor/semantic-contract.md:824`; `candidate-04-partial/experiments/moriarty-language/spec/successor/signing-display-schema.json#/$defs/CostCertificate`; `candidate-04-partial/experiments/moriarty-language/spec/successor/signing-examples.json#/preimageRegistry/programBody`; `candidate-04-partial/experiments/moriarty-language/spec/successor/signing-contract.test.py:1155`; `candidate-04-partial/experiments/moriarty-language/spec/successor/signing-contract.test.py:1364`.

- Genesis 8→7, swap/outcome 7→6 and loan-accrue 7→6 now agree across the actual referenced before/signed/post/prepared/successor work values. Split and join prose explicitly subtract the charge before assigning outputs.
- ProgramBody still contains only sourceHash, entryActions and coreVersion; sourceUtf8 contains action names without bodies. The certificate lists chosen CoreOp/expression charges but contains no operand/expression structure or linkage from which completeness of that list can be verified. ExprCharge.ctor is any Identifier although the contract requires a finite expression set.
- Independent probe changes swapExecutionBody.costCertificate.coreOps[0].ordinary from 1 to 0 while totalOrdinary stays 1. The certificate remains schema-valid and the dedicated 'independent fixture replay' still passes all 105 checks. expected_post_work subtracts the stored total; it does not recompute the component sum or validate fixed per-op charges.
- There is no positive split/join fixture with complete after-charge authority/cap/duty/identity partition. The clone fixture demonstrates 7+7>7 and duplicate recovery balances, but does not validate a correct sibling partition.

**Required correction:** Retain a complete bounded operation/expression cost derivation, tie it to the selected action and count every component independently, including sidecar charge and fixed operation prices. Validate after-charge work and financial/identity partitions using a valid split/join sibling plus direct charge/omission/partition mutations. Preserve the corrected inherited 7→6 values.

## C04-F6: Negative contexts and their purported replay still accept labels and stale dependencies

Prior C03-F6. Paths: `candidate-04-partial/experiments/moriarty-language/spec/successor/signing-contract.test.py:1218`; `candidate-04-partial/experiments/moriarty-language/spec/successor/signing-contract.test.py:1230`; `candidate-04-partial/experiments/moriarty-language/spec/successor/signing-contract.test.py:1364`; `candidate-04-partial/experiments/moriarty-language/spec/successor/generate-signing-examples.py:1667`; `candidate-04-partial/experiments/moriarty-language/spec/successor/generate-signing-examples.py:1680`; `candidate-04-partial/experiments/moriarty-language/spec/successor/generate-signing-examples.py:1760`; `candidate-04-partial/experiments/moriarty-language/spec/successor/signing-examples.json#/invalid`.

- first_false_stage validates document schema then returns intendedFailureStage. Removing the wrong-network mutation leaves network midnight_preview equal to genesis.network and a schema-valid document, yet the function returns authorization. test_independent_fixture_replay only requires a nonempty named label and accepts laterPredicatesUnreachable=None.
- Six independent in-memory mutations each leave the dedicated replay at PASS 105 / FAIL 0: swap post trader balance 90000→90999 without effect; swap input Transfer amount 10000→1; accrue post debt outstanding→1; add an unbacked prepared duty; arbitrary nonexistent firstFailingPredicate; and change the fee prestate cap 30→0. These probes test the claimed semantic replay only; ordinary hash/schema tests elsewhere can detect some unrehashed corruption.
- Fee and input-fee prestates retain genesisSuccessor while changing fee authority. Both signed Pinned predecessors still point to the original genesis state hash 2d83cf67feb2018a7f180d8494aadc205e3808bc7b5cb3935f814aecb5eabc9f, not the selected fee/input-fee hashes cf6b3712… / 219842c1…. Fee prepared postStateHash 31c0ef6faada3780af39d0a6de984f490a09c97cac8433a15c268cbb5429a16c has no retained preimage. feeSuccessor ledgers say remainingGross=0,remainingFee=0 while its residualCapability restores trader gross10000/fee30.
- Cancellation and unfunded cancellation also pin the original genesis hash but select cancelPreStateBody hash 8c9770fad526a758723b5cfd15b34aaa2644b52bd2dc1ffe85b8caff7e107546. This body adds Request SwapFill01 without a retained creation transition. winningFill is copied from acceptanceOutcome, changes predecessorIds from eb9cdedf… to 1f738e08…, and adds consumedId; independent AcceptanceBody validation rejects that extra field. The referenced prepared/acceptance chain remains the original swap outcome.
- Partial-fill mutation changes minFill to20000 while cumulativeCap stays10000; wrapper is rebuilt but acceptanceOutcome/outcomeSuccessor retain the original document digest. Overdelivery's wrapper/effects deliver19744 but preparedOver points to swapStateBody with19743 and the old successor hash. preparedMigrate points to genesisStateBody with work7 while prepared/successor claim6 and a different successor.
- The input-fee selected action is fee, but signed allowedActions=[accrue,settle,swap,request_cancel]. The cancellation selected action is RequestCancel, but ProgramBody admits request_cancel; no exact alias mapping is retained. These fail prerequisites before the claimed net/backing/currentness predicates.

**Required correction:** Implement bounded independent fixture validation that computes all prior predicates from complete records, identifies the actual first failed predicate, and stops later checks. Rebuild complete admitted histories and all dependent chains for each contextual case. Add direct mutations that invalidate balances/effects/debts/caps/work/labels and confirm the validator rejects them. Keep specified-only/signature-absent status; this does not require a language runtime or ledger.

## All29fixture dispositions

- `inv-extra-property`: **CONFIRMED_SCHEMA_REJECTION**. Reconstructed document rejected by Draft2020-12 before hash.
- `inv-noncanonical-integer`: **CONFIRMED_SCHEMA_REJECTION**. Reconstructed document rejected by Draft2020-12 before hash.
- `inv-unknown-extension`: **CONFIRMED_SCHEMA_REJECTION**. Reconstructed document rejected by Draft2020-12 before hash.
- `inv-principal-trailing-newline`: **CONFIRMED_SCHEMA_REJECTION**. Reconstructed document rejected by Draft2020-12 before hash.
- `inv-cap-trailing-newline`: **CONFIRMED_SCHEMA_REJECTION**. Reconstructed document rejected by Draft2020-12 before hash.
- `inv-principal-trailing-cr`: **CONFIRMED_SCHEMA_REJECTION**. Reconstructed document rejected by Draft2020-12 before hash.
- `inv-principal-unicode-newline`: **CONFIRMED_SCHEMA_REJECTION**. Reconstructed document rejected by Draft2020-12 before hash.
- `inv-duplicate-four-contract-invariant`: **CONFIRMED_SCHEMA_REJECTION**. Reconstructed document rejected by Draft2020-12 before hash.
- `inv-display-mismatch-gross-fee-debt`: **CONFIRMED_DISPLAY_DISCRIMINATOR_ONLY**. Specified missing/extra/changed leaf differs from exact projection; no lifecycle acceptance.
- `inv-display-missing-entry`: **CONFIRMED_DISPLAY_DISCRIMINATOR_ONLY**. Specified missing/extra/changed leaf differs from exact projection; no lifecycle acceptance.
- `inv-display-extra-entry`: **CONFIRMED_DISPLAY_DISCRIMINATOR_ONLY**. Specified missing/extra/changed leaf differs from exact projection; no lifecycle acceptance.
- `inv-display-changed-entry`: **CONFIRMED_DISPLAY_DISCRIMINATOR_ONLY**. Specified missing/extra/changed leaf differs from exact projection; no lifecycle acceptance.
- `inv-uint64-max-ok`: **CONFIRMED_DOMAIN_ONLY**. 2^64-1 is in range; this is not a complete transaction.
- `inv-uint64-max-plus-one`: **CONFIRMED_DOMAIN_REJECTION**. Canonical decimal 2^64 passes pattern but exceeds the inclusive numeric bound before hash.
- `inv-uint128-max-ok`: **CONFIRMED_DOMAIN_ONLY**. 2^128-1 is in range; this is not a complete transaction.
- `inv-uint128-max-plus-one`: **CONFIRMED_DOMAIN_REJECTION**. Canonical decimal 2^128 passes pattern but exceeds the inclusive numeric bound before hash.
- `inv-wrong-network`: **CONTEXT_NOT_ISOLATED**. midnight_mainnet differs from retained genesis midnight_preview; wrapper matches mutated payload. Target local mismatch confirmed; full admitted/contextual predicate ordering is not executed.
- `inv-wrong-deployment`: **CONTEXT_NOT_ISOLATED**. other_deployment differs from preview_deployment_1; rebuilt wrapper matches. Target local mismatch confirmed; full contextual isolation not established.
- `inv-duplicate-predecessor`: **CONTEXT_NOT_ISOLATED**. Two identical consumption IDs are present. Schema permits them; target uniqueness violation confirmed, complete independent history checker absent.
- `inv-revoked-spec`: **CONTEXT_NOT_ISOLATED**. Mutated spec.revoked=true locally rejects revocation predicate; no independently replayed registry/history admission.
- `inv-expired-spec-window`: **CONTEXT_NOT_ISOLATED**. notAfterExclusive1600000000 is earlier than notBefore1690000000 as well as observation now; fixture does not isolate expiration from validity-window consistency.
- `inv-partial-fill-exceeds-cap`: **CONTEXT_NOT_ISOLATED**. minFill20000 > cumulativeCap10000; stale acceptance/successor intent digest and no first-predicate computation.
- `inv-cloned-residual-work`: **CONTEXT_NOT_ISOLATED**. Children ordinary7+7=14 > parent7, recovery2+2=4 > parent2. Selected swap plus copied outcome children is not a complete valid split context; no valid partition sibling.
- `inv-cancellation-race`: **CONTEXT_NOT_ISOLATED**. Selected/pinned state mismatch; RequestCancel action not an exact program entry; winner fails AcceptanceBody schema and differs from retained acceptance.
- `inv-replay-migration`: **CONTEXT_NOT_ISOLATED**. Consumed ID is asserted; poststate still genesis with7 work and old successor versus prepared6. Full chain/history not isolated.
- `inv-exact-output-overdelivery`: **CONTEXT_NOT_ISOLATED**. 19744 differs from Exact19743. Prepared poststate still pays19743 and references old successor, so dependent chain is inconsistent.
- `inv-net-after-fees-shortfall`: **CONTEXT_NOT_ISOLATED**. 19743-30=19713<19743 confirmed. Earlier pinned-state, predecessor-capability and missing poststate preimage failures remain.
- `inv-input-asset-fee-only`: **CONTEXT_NOT_ISOLATED**. 0-30=-30 representable. Action fee is excluded by intent; selected/pinned state and net-goal copies differ; not a net-only failure.
- `inv-unfunded-cancellation`: **CONTEXT_NOT_ISOLATED**. Selected/pinned state and action admission fail first; supplied deficit30 does not follow from goal19743/inflow0; complete missing-backing predicate absent.

## Verification and limits

The frozen tests ran through runpy with PYTHONDONTWRITEBYTECODE=1. Only HERE was redirected for three absent unowned pin files; frozen schema/examples/contract paths and test logic stayed unchanged. Exact stdout SHA256: `3821433f595d2c6e50f60d75d4e4486fa02bf1c3f9336acdb56c27e0242e455b`. Two compact sorted UTF8 generations plus newline match retained SHA256 `18ccb7814863b07bc11ce11ce229a415ab10eb2a12e70fd22ef9b04f5429af` (the exact full digest is in JSON).

The six corruptions test the dedicated105-check semantic replay, not the full965-test suite; ordinary hash/schema tests can detect some unrehashed corruption. The preliminary metadata probe used an invalid UInt tag and was discarded; the valid reversal of existing arguments establishes C04-F2. The JSON records command scopes and results. Read/probe commands exited0. Initial report serialization failed before any file write because TextEncoder was unavailable, then was corrected. Broad read truncations were followed by targeted source reads.

Author completion remains exit124 after1792.872seconds. Retained report prose cannot establish a successful author dispatch or actual serving model. Historical RED144, root69failures, interrupted passes and frozen candidates remain distinct; no raw worker stdout or private reasoning was read.

- No language/runtime evaluator, signature verification, proof, ledger or external funding verification executed or claimed.
- Not every underlying financial corpus input was reinterpreted; all31 protected input hashes were verified.
- No executable complete Core or full contextual validator exists in this candidate; absent/underdetermined semantics cannot be independently accepted by inferring intended behavior.
- No positive split/join full partition fixture exists for verification; complete algebraic proof of every primitive was not attempted.
- No raw author stdout/private reasoning read. Successful author serving identity and terminal completion remain unavailable; historical author command/capture claims were not promoted to verified provenance.
- An exploratory SInt128 bound probe confirmed UInt128 maximum does not fit signed net; no separate blocker is raised without a complete admitted-domain policy analysis.

Substantive review ended after457.958seconds, before780-second cutoff. Native900-second allowance was charged, not cgroup enforced. Final timing is in JSON.

Final elapsed time: 805.162 seconds; 900-second allowance met.

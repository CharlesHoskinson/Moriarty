# Candidate03 independent review: BLOCKED

Candidate `511b179e983daddea2dce93edc0ee62e8a60cf19086b9747d37f5e7857d5c45b`;538023bytes. All8owned and31input SHA256 values matched before and after review. Scope is the proposed contract/generator only. No runtime, network, proof, ledger, finality or sprint acceptance.

All338author tests passed in an independent rerun. Two in-memory generator builds exactly matched the retained225407-byte examples. All52catalog entries independently hash and typecheck. These successes do not establish valid economic transitions.

## C03-F1: Core signature and primitive state semantics remain incomplete

Groups: R1. Paths: `semantic-contract.md:248 (B.1)`; `semantic-contract.md:119`; `semantic-contract.md:178`; `signing-display-schema.json#/$defs/ShareConvertEffect`; `signing-contract.test.py:338`.

- ShareConvert has parameters holder:Party, in, out, rounding without types for three operands; its stated state effect is holder shares swap, but ShareConvertEffect contains two Amount values and no input/output vault identity or share units. Two vaults with equal asset amounts cannot be distinguished by that effect.
- Genesis declares arity1 but lists instanceId and initialWork; the text says arity counts explicit parameters after actor.
- The finite expression set is a list of names rather than constructor signatures and typing/evaluation judgments. Require/Let/NextWrite/ProjectField/ConstructRecord have no complete operand/result types or shape constraints. The Core operation state-effect column does not specify complete primitive update equations and prerequisites for requests, messages, rewards, claims and shares.
- A.2 Shares omits holder while A.3 and schema require holder. Tests check presence of constructor names, headings and selected required fields, which cannot establish the requested full typed Core.

**Required correction:** Complete the bounded typed constructor interface and primitive equations now, including unique financial identities and all operands/results, and reconcile duplicate definitions. Full surface BNF and implementation remain out of scope.

## C03-F2: Display values are preserved, but semantic unit metadata still disagrees with the contract

Groups: R3. Paths: `semantic-contract.md:509 (E)`; `generate-signing-examples.py:278`; `signing-examples.json#/valid/0/displayProjection`; `signing-display-schema.json#/$defs/Price`; `signing-display-schema.json#/$defs/StoredText`.

- E explicitly requires price mantissa unit to identify base per quote at scale N. The actual executionBody.rateOrPrice.mantissa entry says unit=scaled-integer, value=19743; it never resolves AssetB per AssetA at scale4. The generator copies metadata strings without implementing the documented ancestor unit resolution.
- The accrue debt_id argument Loan01 remains label Value, unit typed-leaf, role payload. Its financial obligation identity is not represented in that entry.
- All579 positive display leaves preserve paths and JSON values, including true, empty arrays and integer exponent1. Those improvements resolve the old stringification defect but do not establish full semantic FIELD_META.

**Required correction:** Implement a deterministic metadata resolution rule for context-dependent units/roles/identities or amend the proposed contract to a precise equivalent representation, with independent expected metadata checks rather than nonempty-string checks.

## C03-F3: Generated positive states cannot be replayed from their signed writes and retained history

Groups: R1, R4. Paths: `semantic-contract.md:164`; `signing-examples.json#/preimageRegistry/swapExecutionBody/exactWrites`; `signing-examples.json#/preimageRegistry/genesisStateBody/fields`; `signing-examples.json#/preimageRegistry/genesisExecutionBody`; `signing-examples.json#/preimageRegistry/loanStateBody`; `generate-signing-examples.py:722`.

- Swap and selected outcome plan write reserve_a/reserve_b; genesisStateBody declares pool_a/pool_b, plus trader_a/trader_b/alive. The contract explicitly rejects undeclared selectors. Thus both advertised swap positives fail before the declared poststate can be produced.
- Unborn state has empty fields and balances. Genesis exactWrites only names trader_a=100000 AssetA, pool_a=1000000 AssetA, alive=true. Advertised poststate additionally creates pool_b=2000000 AssetB and trader_b=0; the sole Genesis effect binds instance/work/claimRoot, with no complete initial field or funding record. The exact field creation/frame exception and the additional2000000 AssetB are not derivable from this signed initialization.
- loanStateBody points to the same genesisSuccessor as genesisStateBody but has a5000000000 debt, lender controller and loan-only capability, and work8 instead of successor7. No retained DebtCreate/funding/authority handoff state explains this alternative state under the same predecessor consumption identity.
- All52 catalog hashes and types reproduce. This proves byte commitments, not that the bodies denote a valid transition/history. The13 declared inner edges omit the semantic consistency checks above.

**Required correction:** Generate closed initial field declarations and complete initial financial state/funding commitments; use declared selectors; give the loan its own consistent bootstrap/history or retain the missing creation transition. Replay each full before/effect/write/after tuple and verify predecessor successor/capability/work agreement before labeling it positive.

## C03-F4: Residual net and cancellation rules do not cover authorized fee-only outflows or identify the paying duty

Groups: R6. Paths: `semantic-contract.md:694 (G actor equation)`; `semantic-contract.md:729 (G cancellation)`; `signing-display-schema.json#/$defs/ActorLedger/properties/cumulativeNet`; `signing-display-schema.json#/$defs/Duty`.

- G defines cumulativeNet=inflow_to_actor-cumulativeFees, but cumulativeNet is UInt128Text. An otherwise permitted fee30 AssetA with zero AssetA inflow requires cumulativeNet=-30; the schema rejects that value. Fees on input assets cannot be represented by the stated ledger equation. Independent synthetic ActorLedger validation confirms rejection.
- Cancellation says an unsatisfied net goal becomes a duty owed by the original net-goal actor. For trader receiving19743 AssetB with net19713, that makes trader the debtor for the missing30 rather than identifying the authorized payer. The schema requires both creditor/debtor, but no exact rule supplies the creditor, payer binding or backing for that duty. Remaining trader AssetA gross0 does not fund an AssetB obligation.

**Required correction:** Define signed net deltas or separate inflow/fee ledgers with exact net-goal comparisons. Specify cancellation duty creditor, authorized debtor, asset, deficit and deliverable backing; reject an unfunded cancellation or retain an explicit funded residual. Include input-asset fee and shortfall-cancellation traces.

## C03-F5: Work is refreshed across the actual genesis-to-swap edge

Groups: R7, R4. Paths: `semantic-contract.md:783 (H examples)`; `signing-examples.json#/preimageRegistry/genesisStateBody/remainingWork`; `signing-examples.json#/preimageRegistry/genesisSuccessor/remainingWork`; `signing-examples.json#/valid/0/canonicalUtf8`; `signing-examples.json#/valid/1/canonicalUtf8`; `signing-examples.json#/preimageRegistry/preparedSwap/remainingWork`; `signing-contract.test.py:622`.

- Genesis starts8 ordinary and ends7. Both swap and outcome consume that genesis state but sign ordinaryRemaining8. Their poststate/prepared/successor all retain7. With the declared one-unit Exchange charge, the correct inherited amount is7 and post amount6. This mints one ordinary unit at admission.
- Author tests compare each poststate/prepared/successor triple to hardcoded7, without comparing the actual referenced predecessor7 to signed8.
- H introduces expression charges but the retained ProgramBody contains no Core/expression tree or corresponding cost certificate from which total expression cost can be computed. Join states parent.remaining=sum(children remaining) while also charging1; the postcharge quantity is not specified.

**Required correction:** Derive signed and resulting work from the actual predecessor along every edge. Retain enough typed operation/expression structure to reconstruct charges. Define split/join output balances after charges and test read/write/identity/cap frames independently.

## C03-F6: Contextual negative fixtures still fail prerequisites or do not establish the claimed failure

Groups: R8. Paths: `signing-examples.json#/invalid`; `signing-contract.test.py:664`; `semantic-contract.md:875`.

- Fee shortfall mutates signed fee cap0→30 but preStateRef=genesisStateBody still caps that fee at0. It also writes undeclared reserve_a/reserve_b. It cannot isolate net19713<19743 after earlier authority/type prerequisites.
- Cancellation pins state f4816fee8b527c22b5e9ee7af4defe915e5a190a6ab8da4ffcd891deeec4659d but cancelPreStateBody hashes1a9e5162fed0fe2366dc714a953fb3ff4c4576c3afbecc43c3ebf7236d51aecf. Winning fill consumes eb9cdedf8070f084dc719e71bf9efa8401471156827a8dbd8f90c9c70613318f and creates2b238f98e2c4d5700452be75b02d264ea238e7d1f7990fd2f5f7e01f4f679d4d; the claimed stale intent consumption is1f738e080934d8a887cdd1b21fbc50ed08d73d8e5313f46d01afdaa044575883. The retained winner does not demonstrate consumption of that alleged stale identity.
- Migration action migrate and cancellation action RequestCancel are absent from programBody.entryActions=[genesis,swap,accrue,settle]. No explicit alternative admin-action admission is retained. Migration lists dependent hashes to recompute but provides no rebuilt prepared/successor/acceptance bodies matching that mutation.
- Clone context calls parentWork8 while the actual genesis prestate has7, uses the undeclared-field swap plan, and cites swapSuccessor as cumulative history despite a genesis prestate.
- Exact-output overdelivery recomputes executionBodyHash but retains no matching mutated authority wrapper/dependent chain; the undeclared writes remain. Partial-fill mutation likewise lacks a consistent selected plan/history/authority context.
- The R8 tests inspect selected key presence and predicates; they do not replay all27 fixtures through prerequisites. All fixtures honestly remain specified-only; that honesty does not repair isolation.

**Required correction:** Rebuild each contextual fixture from an independently valid full context. Include signed authorization, admitted action, exact state/currentness identity, selected plan and all dependent bodies. Prove every prior predicate true and exactly the named predicate false with bounded local checks. Preserve the schema/domain/display cases and their scoped successes.

## Group dispositions

- **R1**: BLOCKED C03-F1/F3; Rate/Price stored values, holder and payload fields improved
- **R2**: PRESERVED: true-end patterns, prehash UInt bounds, four distinct mandatory claims
- **R3**: PARTIAL:579typed leaves corrected; metadata C03-F2 remains
- **R4**: PARTIAL:52catalog hashes/types and known13edges corrected; actual transitions/history C03-F3/F5/F6 blocked
- **R5**: CORRECTED proposed arithmetic:41095890 accrual,6000000000cap,5041095890outstanding,period cursor,writeoff components,credit and scale rules; no runtime claim; loan history independently blocked
- **R6**: PARTIAL:swap asset keys/residual0 and accrued debt ledger corrected; C03-F4 remains
- **R7**: BLOCKED C03-F5; read/write conflict prose and recovery alias improved
- **R8**: BLOCKED C03-F6;27individual dispositions retained

## Positive cases

- `exact-plan-swap-min-receive`: schema/hash/display types pass; semantic transition BLOCKED. C03-F3/F5: undeclared selectors and work refresh
- `outcome-intent-swap-and-loan-caps`: schema/hash/display types pass; semantic transition BLOCKED. C03-F3/F5: undeclared selectors and work refresh
- `exact-plan-genesis-no-predecessor`: schema/hash/display types pass; semantic transition BLOCKED. C03-F3: initialization is not fully committed/replayable
- `exact-plan-accrue-nonexchange`: schema/hash/display types pass; semantic transition BLOCKED. C03-F3: loan prestate disagrees with referenced successor/history; accrual arithmetic itself passes

## All27negative fixtures

- `inv-extra-property` (schema): CONFIRMED_SCHEMA_REJECTION. Reconstructed locally; Draft2020-12 rejects.
- `inv-noncanonical-integer` (schema): CONFIRMED_SCHEMA_REJECTION. Reconstructed locally; Draft2020-12 rejects.
- `inv-wrong-network` (authorization): TARGET_PREDICATE_SPECIFIED_NOT_ISOLATED. Target authorization mismatch is apparent; no complete rebuilt authority/hash context proves all preceding checks.
- `inv-wrong-deployment` (authorization): TARGET_PREDICATE_SPECIFIED_NOT_ISOLATED. Target authorization mismatch is apparent; no complete rebuilt authority/hash context proves all preceding checks.
- `inv-display-mismatch-gross-fee-debt` (display): CONFIRMED_DISPLAY_MUTATION_ONLY. Missing, extra or changed leaf differs from retained projection; baseline transition validity remains blocked.
- `inv-duplicate-predecessor` (history): TARGET_DUPLICATE_PRESENT_NOT_ISOLATED. Duplicate consumption is present and schema legal; changed signed body has no rebuilt full authority/hash chain.
- `inv-revoked-spec` (authorization): TARGET_PREDICATE_SPECIFIED_NOT_ISOLATED. Target authorization mismatch is apparent; no complete rebuilt authority/hash context proves all preceding checks.
- `inv-expired-spec-window` (authorization): TARGET_PREDICATE_SPECIFIED_NOT_ISOLATED. Target authorization mismatch is apparent; no complete rebuilt authority/hash context proves all preceding checks.
- `inv-partial-fill-exceeds-cap` (evaluation): BLOCKED_CONTEXT_ISOLATION. See C03-F3/F5/F6 for earlier selector, work, authority, action, currentness or dependent-body failures.
- `inv-cloned-residual-work` (evaluation): BLOCKED_CONTEXT_ISOLATION. See C03-F3/F5/F6 for earlier selector, work, authority, action, currentness or dependent-body failures.
- `inv-cancellation-race` (ledger-currentness): BLOCKED_CONTEXT_ISOLATION. See C03-F3/F5/F6 for earlier selector, work, authority, action, currentness or dependent-body failures.
- `inv-replay-migration` (history): BLOCKED_CONTEXT_ISOLATION. See C03-F3/F5/F6 for earlier selector, work, authority, action, currentness or dependent-body failures.
- `inv-unknown-extension` (schema): CONFIRMED_SCHEMA_REJECTION. Reconstructed locally; Draft2020-12 rejects.
- `inv-exact-output-overdelivery` (evaluation): BLOCKED_CONTEXT_ISOLATION. See C03-F3/F5/F6 for earlier selector, work, authority, action, currentness or dependent-body failures.
- `inv-net-after-fees-shortfall` (evaluation): BLOCKED_CONTEXT_ISOLATION. See C03-F3/F5/F6 for earlier selector, work, authority, action, currentness or dependent-body failures.
- `inv-principal-trailing-newline` (schema): CONFIRMED_SCHEMA_REJECTION. Reconstructed locally; Draft2020-12 rejects.
- `inv-cap-trailing-newline` (schema): CONFIRMED_SCHEMA_REJECTION. Reconstructed locally; Draft2020-12 rejects.
- `inv-principal-trailing-cr` (schema): CONFIRMED_SCHEMA_REJECTION. Reconstructed locally; Draft2020-12 rejects.
- `inv-principal-unicode-newline` (schema): CONFIRMED_SCHEMA_REJECTION. Reconstructed locally; Draft2020-12 rejects.
- `inv-uint64-max-ok` (none): CONFIRMED_DOMAIN_ONLY. Inclusive integer boundary passes domain; not a complete valid authorization/lifecycle fixture.
- `inv-uint64-max-plus-one` (domain): CONFIRMED_DOMAIN_BOUND. Canonical digits pass schema; UInt bound rejects before hashing.
- `inv-uint128-max-ok` (none): CONFIRMED_DOMAIN_ONLY. Inclusive integer boundary passes domain; not a complete valid authorization/lifecycle fixture.
- `inv-uint128-max-plus-one` (domain): CONFIRMED_DOMAIN_BOUND. Canonical digits pass schema; UInt bound rejects before hashing.
- `inv-duplicate-four-contract-invariant` (schema): CONFIRMED_SCHEMA_REJECTION. Reconstructed locally; Draft2020-12 rejects.
- `inv-display-missing-entry` (display): CONFIRMED_DISPLAY_MUTATION_ONLY. Missing, extra or changed leaf differs from retained projection; baseline transition validity remains blocked.
- `inv-display-extra-entry` (display): CONFIRMED_DISPLAY_MUTATION_ONLY. Missing, extra or changed leaf differs from retained projection; baseline transition validity remains blocked.
- `inv-display-changed-entry` (display): CONFIRMED_DISPLAY_MUTATION_ONLY. Missing, extra or changed leaf differs from retained projection; baseline transition validity remains blocked.

## Verification and preserved failures

Tests do not import generator helpers. The prior meaningful RED144 is preserved as author evidence; it was not independently rerun. The root verify3 layout failure remains recorded, followed by verify4 success643checks/49hashes/13declarededges. No failed receipt was overwritten.

- Exit 0: Initial read-only state listing; binding/freeze/prior review/correction/completion prompts and receipt/report inspection. 
- Exit 0: Initial Python SHA256 verification of8owned+31binding inputs. 
- Exit 1: PYTHONDONTWRITEBYTECODE=1 python3 signing-contract.test.py (frozen candidate cwd). FileNotFoundError for unowned bounds.json pin; no source change; not a semantic defect
- Exit 0: sed/rg and Python JSON targeted reads of contract/schema/examples/generator/tests. Multiple read batches; large outputs sometimes truncated and followed by targeted reads
- Exit 0: PYTHONDONTWRITEBYTECODE=1 python3 heredoc: runpy tests, redirect only HERE pin lookup to /home/charl/Moriarty/experiments/moriarty-language/spec/successor, run main. PASS338 FAIL0 GREEN; candidate input path globals unchanged
- Exit 1: Python reconstruct all27 negative docs then inspect positive selectedPlanRef. All27 schema results produced; reviewer TypeError treating typed selectedPlanRef dict as hash key; corrected next probe
- Exit 0: Python corrected selectedPlanRef.name probe, positive before/work/write selectors, schema/display inspection. 
- Exit 1: sed contract plus cat generator-completion-prompt.md from candidate directory. Contract read succeeded; prompt not in candidate; read correctly from state next
- Exit 0: cat graphify and verification-before-completion skills. Graph construction/install would violate explicit read-only scope; not run. Verification guidance applied.
- Exit 0: Python runpy generator build(schema) twice in memory; canonical bytes compare to candidate signing-examples.json. Both builds identical;225407bytes;SHA25656c34ef5104970550bffcd0bb3743d5a62495a25f8783a1851f1a8a5350ececc
- Exit 0: Python independently hash/typecheck all52 catalog entries, inspect loan/cancel successor compatibility and safe structured receipts. All52catalog entries PASS; loan work/capability mismatch confirmed
- Exit 0: rg contract plus cat absolute state/generator-completion-prompt.md. 
- Exit 0: sed contract/test; Python inspect semantic-decisions and preserved interrupted file names (no raw worker stdout). 
- Exit 0: Python inspect Duty/ActorLedger/FieldBinding/StoredText and all13actual edge declarations. 
- Exit 0: Python synthetic fee-only ActorLedger and cancellation identity comparison. Net-30 rejects UInt128 schema; cancellation state/identity mismatches confirmed
- Exit 0: Python recompute candidate manifest digest variants. SHA256 compact sorted JSON file-hash mapping matches511b179e…
- Exit 0: Python final8owned+31input hash comparison and write only gpt6-candidate03-review.json/md. All unchanged; digest verified;538023bytes

## Limits and next gate

- No evaluator/runtime/proof/signature/ledger/finality run or claim. Contextual fixture audit is static plus bounded synthetic checks.
- All31 binding inputs were hash verified; not every financial corpus source was reinterpreted within600sec.
- Full topological financial history cannot be accepted while inconsistent predecessor states and missing mutated dependent bodies remain.
- Complete mathematical proof of every debt/composition equation was not attempted. Corrected local arithmetic is not protocol conformance.
- Receipt prose is attributed evidence; raw worker stdout and private provider reasoning were not read.

Repair these findings and obtain a fresh review. Full map reconciliation and consequential majority semantic freeze remain required. This report grants no downstream acceptance.

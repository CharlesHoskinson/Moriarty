# SP05 production-control scope audit

Read-only audit of the active checkout `/home/charl/Moriarty/.worktrees/sp05-deadline-review`, 2026-09-10. Loaded the Moriarty plugin and queried status; unresolved campaign history remains. No repository edits, services, wallet/private-state reads, proof requests or transactions were performed. Existing tests use public retained bytes and controlled transports; their temporary fixture files are scratch only.

## Conclusion

The wrong-payer, wrong-recipient and omitted-fee controls **already exercise callable production predicates**. They are not mere disconnected arithmetic checks. Controlled network/SDK responses limit them to source-bound rejection evidence, but the current wording does not require every malicious-input control to be a newly submitted on-chain transaction. Do not replace this existing coverage with a new all-negative-cases-on-chain gate.

What is still missing from the inspected SP05.2 evidence is an actual failed local transaction with adequately anchored complete financial nonmutation and its current result reviews. Source rejection, controlled observer responses and successful positive traces do not establish that predicate. An included fallible rollback is not demanded by the reviewed wording.

There is also a **precise excess-fee acceptance-coverage gap to resolve**: the production provider enforces native DUST allowance, but these tests do not demonstrate a positive native DUST debit exceeding its configured cap through `balanceTx`. The comparator itself has no admission-cap input and accepts matching positive native fee fields of arbitrary in-range size. This is distinct from the already passing omitted-fee and mismatched-fee controls. It does not demonstrate an exploitable over-spend of the integrated provider; that production cap check exists and actual positive runs were independently reconciled to their reservations.

## Governing requirements

- `openspec/sprints/sp05-financial-integration-on-preview.md:35–36`: explicit principals, assets/denomination, gross debit, fees, change, residual duties; independently derived expectations; wrong recipient/token/denomination/fee/debt identity must fail comparison.
- Same file `:43–44`: complete decoder and initial/finalized effect comparison, actual test-asset movement for both local cases, and failed transactions with no financial state mutation.
- Same file `:73–75`: actual asset/role/ownership/gross/fee/change/finality bindings; wrong payer, wrong recipient and omitted-fee controls on the callable production path. Preview positive settlement is still separate.
- `openspec/changes/mc02-preview-financial-operation/specs/mc02-preview-financial-operation/spec.md:15–23`: complete comparison rejects wrong recipient/domain, **excess fee**, missing debit, extra approval, undeclared write.
- `openspec/changes/mc02-preview-financial-operation/tasks.md:23`: positive and negative local Docker comparisons precede public submission. `:43`: freeze distinct counterparty identities and funding.

## Existing controls and their exact boundary

| Control | Test and actual production path | Established scope / limitation |
|---|---|---|
| Wallet-derived payer changes after deploy | `integration-negative.test.mjs:76,82–83` calls `integrateLocalFinancialCase` → `runLocalFinancialCase`; `run-local.mjs:79–96` compares role-first address with execution binding. | Exact `LOCAL_EXECUTION_BINDING_MISMATCH`; no next call; previous public failure/charge record retained. SDK/provider implementation is controlled in this composed fixture, so this does not test a live wallet compromise. |
| Wrong decoded financial payer | `integration-negative.test.mjs:81–83`; real observer first decodes retained signed bytes, then fixture changes the observation before real comparator `financial-comparison.mjs:135–148`. | `UNAUTHORIZED_PAYER` in comparison, stopping promotion. Mutation is to decoded observation; it is not proof that a real unauthorized ledger transaction was constructed or rejected. |
| Wrong indexed recipient | `integration-negative.test.mjs:77,82–83` changes wire owner; real `observeFinalizedStage` compares native-decoded outputs to indexed outputs at `receipt.mjs:170–171`. | `INDEXED_OUTPUTS_MISMATCH` before comparison; exercises the actual decoder/observer path, with controlled RPC/indexer transport. |
| Recipient changed during wallet balancing | `providers.test.mjs:262–275` calls actual `createFinancialProviders` and `balanceTx`; actual native `UnshieldedOffer` recipient is changed by controlled wallet adapter. `providers.mjs:355–361` checks approved original semantics. | Rejects before signing and before reservation count increases. This is a real production pre-sign control with native data; it is not on-chain acceptance evidence. |
| Wire fee fields omitted | `integration-negative.test.mjs:78,82–83`; actual observer `receipt.mjs:164–165` parses required paid/estimated strings. | `INVALID_PAID_FEES` before financial comparison; not a live omitted-fee transaction. |
| Observed fee object omitted | `integration-negative.test.mjs:79,82–83`; comparator's exact receipt shape at `financial-comparison.mjs:163`. | `FIELDS_RECEIPT`; real comparator rejection. |
| Native fee understated relative to decoded transaction | `integration-negative.test.mjs:80,82–83`; `financial-comparison.mjs:179–182`. | `NATIVE_FEE_MISMATCH`; preserves both native SPECK debit and unresolved indexer encoding instead of silently equating them. |
| Signatures/ownership | `receipt.test.mjs:38–58,140–150` and actual decoder `receipt.mjs:62–70`; `providers.test.mjs:128–131`; actual provider `providers.mjs:74–105`. | Exact input index/count and cryptographic signature checks, plus payer ownership. Controlled native fixtures establish those predicates, not proof verification or network acceptance. |
| Asset/domain/debt/effect completeness | `financial-comparison.test.mjs:60–89`; comparator `:76–89,113–155`. | Mutations reject missing/extra state, erased residual principal, address/capability/domain/color/network/program changes, hidden effects, wrong payout and missing gross debit. These are direct callable comparator tests. |
| Actual SDK contract-balance producer | `provider-native-balances.test.mjs:23–58`. | Actual GraphQL SDK query and native compact-runtime decoding of the retained 8,150-byte state, with HTTP interception below SDK; no fee admission test and no live state query. |

`integration-negative.test.mjs:1–7,44–59` explicitly discloses all substitutions: public native loan bytes are retained; decoded financial states are reconstructed; synthetic capability preimages replace original private secrets; empty native state containers and RPC are controlled; source-only counters are not allocations. It still uses the actual comparator/driver/observer (`:16–20,51,58,67`). Production public retention rejects its source-only result (`:68`), and all six controls assert `financialAcceptance:false`, stopped/closed fixtures, phase, historical charge preservation and exact call boundaries (`:83`). That boundary disclosure is correct and should be retained.

The production integration dependencies are not merely named in the test: `integrate-local.mjs:113` wires the same provider, comparator, observer and driver by default; `:278,310,318–329` connects comparison and durable-stage retention; `run-local.mjs:110–117` requires comparison PASS. An observer rejection occurs **after an observed transaction**, so it stops acceptance/promotion; do not claim that it retroactively prevents ledger mutation.

## Excess-fee distinction and smallest remaining source test

`providers.mjs:331–334` checks `reservedDustFee + dustFee > maxDustFee`. Calls at `:355,361,367` surround balancing and signing; `:370–374` reserves the full native amount before finalization. `inspect` sums actual native `DustSpend.vFee` at `:93–100`. No refund netting or indexer fee equality is substituted.

However, `financial-comparison.mjs:62–74` accepts no fee admission/cap argument, and `:179–182` checks only native fee completeness/equality/range plus indexer metadata shape. `financial-expectations.json:1403–1408` treats network fees as separate observations. A minimal read-only reproduction using the existing synthetic fixture and actual comparator changed both matching native fee fields to `3000000000000000` SPECK for each of four stages: **both loan and swap still returned PASS**. This proves the standalone comparator is not the fee-cap gate. It does not bypass the real provider, which was not invoked. Reproduction: `/tmp/moriarty-sp05-fee-comparator-scope-repro.mjs` and `.txt`.

The inspected provider suite tests malformed negative fee caps, gross/submission exhaustion and semantic preservation, but has no explicit positive native DUST over-cap case. `provider-native-balances.test.mjs` concerns state balances only. `receipt.test.mjs:110–135` replays the real public historical DUST fixture and proves native debit `300000000000001` SPECK versus indexer `1/1`; this is decoder/observer coverage, not `checkAllowance` coverage.

Recommendation: add one test within the existing provider suite that supplies a **supported native proof/pre-binding transaction with a positive native DUST spend** to actual `walletProvider.balanceTx`, sets the configured allowance below that debit, and asserts exact `DUST fee allowance exceeded` before any wallet call or reservation increase. Reuse public retained native material only if native construction accepts it; first establish that it crosses `Transaction.deserialize('signature','proof','pre-binding', ...)` at `providers.mjs:353`. The historical transaction is bound; passing it directly would fail the wrong predicate. Transplanting proven DustActions into a fresh pre-proof Intent is known to fail native type checks. No fake `inspect`, plain-object `DustSpend`, flag changes, mocked proving or new helper is justified to make the test green. If supported construction is not immediately available, keep this exact gap open while the real local negative transaction proceeds.

For MC02's literal “complete-effect comparator rejects excess fee” requirement, explicitly bind which existing production checker owns the admitted fee limit and preserve evidence through the comparison/result record. Do not claim that equality-only standalone comparison establishes a cap. This can be a narrow integration/evidence correction; it does not imply every excessive-fee input must be publicly submitted.

## Distinct principals and existing positive evidence

The exact-scope interpretation in `/tmp/moriarty-sp05-exact-scope-check.md` is supported. Charter `openspec/MORIARTY-COMPLETION-PROGRAM.md:377` says distinct principals with external keys and explicit fixture funding, not independent humans/controllers/wallet funding sources. `run-local.mjs:79–87` and `financial-comparison.mjs:41–53,80–85` enforce distinct nonzero addresses and role/network/program capability commitments; `financial-comparison.test.mjs:98–100` rejects duplicate participants.

- `local-swap-continuation-01/result-review-gpt6.json:27–35` independently decodes actual input signature/ownership, initialized input origin, trader change, provider close payouts, asset identities, complete states and participant deltas. Close has no unshielded input, so this fixed close does not automatically require a provider-owned input signature. Existing capability authority remains required; a DUST payer signature never becomes some other role's authority.
- `local-continuation-02/result-review-gpt6.json:35–48` independently verifies all four comparisons, actual borrower gross input/signature, exact lender payout/change and residual principal. Test mint funding is not actual funded loan origination; the audited fixed repayment fixture does not gain an additional independent loan-origination requirement.
- Both results retain local-only, uncertified scope, raw inner INCOMPLETE and separate containment observations. Existing positive result audits cannot approve a still-unperformed failed transaction or Preview run.

## Fresh checks and remaining agenda

Fresh command: `node --test` over `integration-negative.test.mjs`, `receipt.test.mjs`, `financial-comparison.test.mjs`, `providers.test.mjs` in this checkout. Result: **86 tests passed, zero failed/skipped**, 2,742.604932 ms. Output: `/tmp/moriarty-sp05-controls-scope-tests.tap`. No live networking or proof body was requested; native proof-free fixture conversion has throwing prover callbacks and no proof-bearing request.

1. Reuse the already passing production payer/recipient/omitted-fee controls, current scoped source checks and positive local result reviews.
2. Retain actual failed local transaction identity/rejection plus complete genuinely-after canonical state/assets and fee accounting; obtain current result reviews. Do not count transport failure or pre-submit Compact rejection as that ledger predicate.
3. Resolve the narrow excess-fee production test/evidence ownership gap above; avoid a new blanket requirement for on-chain negatives.
4. Proceed to separately admitted SP05.3 Preview positive loan/swap, exact complete comparison, finality/readback, transaction ID reporting and package reviews. No new independent-wallet or included-rollback gate is introduced.

## Exact inspected digests

Checkout HEAD at digest capture: `a94cbd314e1373a494afc1b9f117506e5c692652`. This is a scope audit, not approval of unrelated work in the checkout.

- `openspec/sprints/sp05-financial-integration-on-preview.md`: `fd7b0cdbbfe608239c2a91c28a1c4f7d9d15d3bcc9a120eeb52098ab5c1dda2f`
- `openspec/changes/mc02-preview-financial-operation/specs/mc02-preview-financial-operation/spec.md`: `33502e6ac8777a7e133253e6189ca7e0cc3b6e43f18c67048812581565a80b91`
- `openspec/changes/mc02-preview-financial-operation/tasks.md`: `628c9be36bf57dc6822e7a61d0699b72a739a0ff567a10a82bb5cec381502358`
- `openspec/MORIARTY-COMPLETION-PROGRAM.md`: `cee25aaed26757073a0ce1a10f54b47c483500370fe947ec9d1dd1e7224527c1`
- `experiments/moriarty-midnight-financial/ledger/integration-negative.test.mjs`: `8cf88e2c056b881edf430ee2f2f62f642dba457890cc0123c00f9471d789164b`
- `experiments/moriarty-midnight-financial/ledger/financial-comparison.mjs`: `f6ad26dce1cf9f5c2a80200d8e8eedd12645b1d040099151d78cb631f7f8957c`
- `experiments/moriarty-midnight-financial/ledger/financial-comparison.test.mjs`: `1abd2b20aca91e1d75bc386a91864978b44c9df7fb8a5ad1dd8dba1a96803224`
- `experiments/moriarty-midnight-financial/ledger/providers.mjs`: `40c0a894f8176855ae832837c3447c4d71aa357e6159532c0387a35986017c74`
- `experiments/moriarty-midnight-financial/ledger/providers.test.mjs`: `af8cd21c447a5fb40b44585d570403e9e853b240d66dddb1e37fcb07fbc51add`
- `experiments/moriarty-midnight-financial/ledger/receipt.mjs`: `ae0fd41e68e8effe80e02c5c85b331bc564f9c5a2792bdd6f38f3fb4bf5c2a44`
- `experiments/moriarty-midnight-financial/ledger/receipt.test.mjs`: `012ec1fc51be8243d61706ffe76edd17b909513d9ad8ee3a6926a9eb53c86f6a`
- `experiments/moriarty-midnight-financial/ledger/provider-native-balances.test.mjs`: `f816929494c563934c1aba76134a934703213f91b334318660205d5468211c36`
- `experiments/moriarty-midnight-financial/ledger/integrate-local.mjs`: `345aa8985fcc8d0ae01d97517763753cd68e5fa021c7733759077bef9545e8f0`
- `experiments/moriarty-midnight-financial/ledger/run-local.mjs`: `17d504be2dd761cfdfa84ebf8378205ae233b256aec10e99f5c1759a13c1b120`
- `experiments/moriarty-midnight-financial/ledger/financial-expectations.json`: `693a8c60972050b6afbece5f2106682ed9966538eb60a21beaa9218b043df35f`
- `experiments/moriarty-midnight-financial/ledger/fixtures/historical-dust/transaction.bin`: `f79580fe0075dc26ae3f97f10557706f340cdf7a3b3118cd65a72b4fd870110a`
- `deliverables/sp05-financial-integration-2026-09-09/local-swap-continuation-01/result-review-gpt6.json`: `6b42aea96e9436694e1896ebabf3b7a782fbeeb5470de2e5d34d33dea94092a7`
- `deliverables/sp05-financial-integration-2026-09-09/local-continuation-02/result-review-gpt6.json`: `50c0cd58c3fdbece592ed49ccef4ffaf8229ea350ee834e22dbdbd0a31f271df`
- `/tmp/moriarty-sp05-controls-scope-tests.tap`: `4aca4d1633c302d92af4915b674f6fc72932db7b997d375112b18f498d214574`
- `/tmp/moriarty-sp05-fee-comparator-scope-repro.mjs`: `94766368fc94fcdbedaac2033dc5ad93081461c56d310d52325d0a2ec1ccb95d`
- `/tmp/moriarty-sp05-fee-comparator-scope-repro.txt`: `06702a95c8a0daf36adba76046f831a07079b9654c805002e81e523078d3158e`
- `/tmp/moriarty-sp05-exact-scope-check.md`: `fef039985d7941533548f54eefd59247761aa849b6b19dbb512921d05530899c`

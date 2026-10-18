# Public Preprod provider observation — 2026-10-16 14:50 UTC

This proposed capsule records **one completed five-response Scrapling query batch at 14:50 UTC on 2026-10-16**. The [root adoption][adoption] closes only `publicObservationPrerequisiteClosed`: a qualified public provider observation. It does not establish authenticated ledger state or actual Preprod transaction acceptance.

**Funding was not observed in this capture.** The public address query returned raw `[]` with `Content-Range: */*`, so completeness is unknown. It proves neither an unfunded wallet, zero ADA nor absence of the complete UTxO set, and says nothing about the current balance. Later user-reported Preview funding, transaction `9ea68a9b…`, belongs to a separate verification outside this capsule and supplies no Preprod funding status here. The immutable 14:50 response remains a dated observation.

**Publication review is pending.** The [fresh actual-result audit][actual-review] confirms the qualified provider observation, while its [recommendation][recommendation] leaves public export, public verification-input publication and native-roundtrip flags false. This new exact package requires a fresh Muse publication review before root integrates it. No further query or runtime authority follows from it.

The [actual receipt][receipt] records five sequential HTTP 200 responses from the fixed `https://preprod.koios.rest/api/v1` host. The fixed query order was:

| Step | Method and endpoint | Exact captured body | Bytes | SHA-256 |
| --- | --- | --- | ---: | --- |
| 1 | `GET /genesis` | [genesis][raw1] | 7598 | `00e6906f0f0b280aca200b846158d3736852c90d51a656958f148f27cc0a0e40` |
| 2 | `GET /tip` | [tip before][raw2] | 214 | `958eeb27a9c95a9b597d4c53369d07acbac0407f1bfec5ada8926841b9bd2298` |
| 3 | `POST /address_utxos` | [address UTxOs][raw3] | 2 | `4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945` |
| 4 | `GET /epoch_params?_epoch_no=319` | [epoch 319 parameters][raw4] | 7141 | `6df4af0269d7d0ea63f5f0aaafaacffbb68734b33422fa234a9d71526cd39625` |
| 5 | `GET /tip` | [tip after][raw5] | 214 | `958eeb27a9c95a9b597d4c53369d07acbac0407f1bfec5ada8926841b9bd2298` |

The POST was a public address query, not a transaction submission. Its recorded body digest is `381332152b8b7b11088079e639135f1f96f6d286c88cc413aade97210c74bd4e`; the fixed public address/body construction is retained in the [source][source] and [expectations][expectations]. Each step recorded one controller attempt, `retries_arg=1` and no error. For the pinned Scrapling behavior, literal `retries=1` means one attempt. Root recorded **one batch attempt, zero retries**, with no paging or extra query.

At that time the provider genesis response reported `networkmagic="1"`, `networkid="Testnet"` and `systemstart=1654041600`. Both tips reported epoch **319**, era **Conway**, absolute slot **136479015**, block **5299378**, and hash `553229bd5bde27922b1fdafd94fd84c9740a7480440d70a6b29c14f5e6a6e1d1`. The before/after bodies are byte-identical. This is a same-provider bracket, not an atomic or authenticated snapshot.

The epoch-319 response reported protocol **11.0**, `max_tx_size=16384`, `max_tx_ex_mem=17500000` and `max_tx_ex_steps=10000000000`. Its cost-model object contains `PlutusV1`, `PlutusV2` and `PlutusV3`; the coefficient ordering, units and application remain unestablished. These are captured provider fields. No fee, min-ADA, transaction-fit or lovelace conversion is derived, and the raw parameters alone do not qualify the separate K10 local VM budget against ledger limits.

The [root scope/materialization companion][scope] records exactly five unchanged R2 candidate files. The executed `observe.py` is **21,135 bytes**, SHA-256 `ef54a814d99b52409bf6699ff724c950719c1a2cbc3e0f259391d0699661d352`. Source-time proposal placeholders and pending/AUTH metadata are preserved; root filled the actual interpreter and source pin in the companion rather than rewriting audited source. The [outer capture][capture] still says `rawBodyFactsIndependentlyReviewed=false` because it predates the later independent result audit and companion adoption.

The historical source decision has **15 literal author-FOR plus complete Gemini REVIEW-3-AGREE positions**. [Gemini recovery][gemini-recovery] exported three token objects without a provider rerun: one invalid and two valid; REVIEW-2 has truncated prose and REVIEW-3 has the [complete report][gemini-report]. Their core verdicts agree, but they represent **one reviewer**, not two votes. The fresh Muse actual audit did not re-vote those source clauses. The Gemini source reviewer was a separate provider from the Muse author; the actual Muse reviewer was fresh and non-author but from the author's model family. HIGH effort was requested and actual provider effort was not independently observed. Existing fallback routing after the Opus timeout remains disclosed in the frozen records.

The [R1 NOT_READY review][r1] reported cookie-jar reuse, environment/proxy/CA claims, ignored Content-Range completeness, incomplete failure preservation and floating-point overflow handling. R2 addressed those source findings. The [16 pure parser cases][parsers] selected six AST nodes and matched the frozen expectations for Content-Range and JSON/integer parsing. They did not exercise the full reader, Scrapling runtime, cookies/proxies, an SDK or ledger behavior. Failed provider-attempt receipts and schema-diagnosis records remain unchanged; terminal failures with zero structured reviews have no invented cause. Neither source readiness nor parser checks substitute for the separately captured actual query batch.

The recorded launch used Python **3.14 `-B`**, not `-I`, with exactly six environment entries: `PATH`, `HOME`, `LC_ALL`, `TZ`, `PYTHONNOUSERSITE` and a fixed public `PYTHONPATH`. The limits were timeout **120 seconds + 10-second TERM-to-KILL grace**, CPU **15:20 seconds**, address space **1 GiB**, core size **0**, and file size **8 MiB per file**. The reader's **2 MiB** body check occurs after the response has been buffered; it is not a streaming download cap, and the file limit is not an aggregate output limit. Outer wall time was `10.321150281000882` seconds and waited-children CPU `0.22172` seconds; transitive CPU was not measured.

The completed public read batch involved no private material, native SDK, proof production, setup, signing or ledger transaction. Its [attempt is spent][spent]. A future query is a separate scope; archived scripts and internal AUTH/proposal records are fixed experiment evidence, not a rerun instruction or new authority. Public Moriarty compilation, proving and deployment remain permissionless, independent of internal experiment review.

This observation leaves the live prototype path open: qualified funding/real wallet-thread context, a fresh current Rust proof for that context, genuine secp256k1 intent, compiled VI plus four VS pieces, atomic C2 spend/burn settlement, and actual signed Preprod ledger acceptance. All 20 UTxO families, ACTUS/DeFi/general financial obligations, real recursion, private handoff, bounded multi-parent PCD and universal certificates remain required and open. No whole-product compatibility percentage or target acceptance is claimed.

[PUBLICATION.json](PUBLICATION.json) records the proposed destination, scoped result, read coverage and required fresh review. [COPY-MANIFEST.json](COPY-MANIFEST.json) inventories every original/target path, byte count and SHA-256 and supplies a lossless source map. All **89 frozen inputs / 1,174,197 bytes** are mirrored under `payload/inputs/`; eight actual-review public outputs, the [input freeze][freeze] and later root adoption bring the copied total to **99 payloads / 1,298,062 bytes**. Four packaging documents bring the stage to **103 files**. Checkout commit `854d9be532c4641ddc2c6857d29537634adca995` is source context; outside-Git artifacts are independently pinned, not attributed to that commit.

Every copied payload was checked against its pin and preserved byte-for-byte. Packaging semantic inspection covered decisive adoption/review/receipt/history records, small raw responses and selected genesis/parameter fields; it was not a full semantic reading of 89 inputs or an interpretation of cost-model coefficients. The auditor's own [read coverage][coverage] remains separately scoped. Historical absolute links and provenance strings are unchanged; the copy manifest maps their sources to staged locations. No existing integration helper, native test or public network request was executed while preparing this package. The separate withdrawal-source packet and Preview funding records are excluded.

[adoption]: root/PUBLIC-OBSERVATION-ACTUAL-ADOPTION.json
[actual-review]: reviews/actual/REPORT.md
[recommendation]: reviews/actual/ADOPTION-RECOMMENDATION.json
[coverage]: reviews/actual/READ-COVERAGE.json
[freeze]: reviews/actual/ROOT-INPUT-FREEZE.json
[receipt]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-preprod-public-observation-actual-20261016/RESULT.json
[capture]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-preprod-public-observation-root-capture-20261016/RESULT.json
[scope]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-preprod-public-observation-root-records-20261016/SOURCE-ADOPTION-AND-PUBLIC-READ-SCOPE.json
[spent]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-preprod-public-observation-root-records-20261016/ATTEMPT-SPENT.json
[source]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-preprod-public-observer-operator-materialized-20261016/runner/observe.py
[expectations]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-preprod-public-observer-r2-source-candidate-20261016/EXPECTATIONS.json
[r1]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-preprod-public-observer-independent-muse-strict-source-review-20261016/REPORT.md
[parsers]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-preprod-public-observer-r2-ordinary-parser-checks-20261016/RESULT.json
[gemini-recovery]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-preprod-public-observer-r2-independent-gemini-full-source-review-20261016/public-structured-recovery/RESULT.json
[gemini-report]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-preprod-public-observer-r2-independent-gemini-full-source-review-20261016/public-structured-recovery/REPORT-3.md
[raw1]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-preprod-public-observation-actual-20261016/steps/01-genesis.bin
[raw2]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-preprod-public-observation-actual-20261016/steps/02-tip_before.bin
[raw3]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-preprod-public-observation-actual-20261016/steps/03-address_utxos.bin
[raw4]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-preprod-public-observation-actual-20261016/steps/04-epoch_params.bin
[raw5]: payload/inputs/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-preprod-public-observation-actual-20261016/steps/05-tip_after.bin

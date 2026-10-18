# Source request surface, not a new schema

Pinned full source: `source-main/experiments/moriarty-eutxo/src/{cli,native-caller,native-body,compile-custody,native-financial,apply-head}.mjs`. There is **no single declared closed request schema** in this path: `runNativeCaller(request = {})` accepts an open JavaScript object, reads fields and spreads nested objects into helpers. This document preserves that fact. It is a source inventory, not an invented payload standard, input validator or assertion that every field has a public producer. Full copied modules are authoritative for defaults, conditional evaluation, delegated validation and side effects.

## Public CLI construction

`cli.mjs:68–167` selects its integrated native request when run directory, authority A, source or execute is present. The exact object contains the following conditional groups:

| Object / fields | CLI source / conversion |
|---|---|
| `authority.{reviewA,reviewB,candidate}` | authority-a/b and candidate flags, when A present |
| `sourcePath` | source flag or undefined |
| `launch.{startNode,nodeBin,runDir,configDir,port,socketPath}` | execute/start-node, node-bin, run-dir, config-dir, numeric port, socket |
| `magic`, `networkMagic` | network-magic via nonnegative safe-integer parser |
| `cashSource` | text read from cash-source path |
| `cashVotes.{reviewA,reviewB,candidate}` | cash-vote-a/b, cash-candidate when A present |
| `cashDesign.{reviewA,reviewB,candidate,sourcePath}` | cash-design-a/b, cash-design-candidate/source when A present |
| `protocol.{cliPath,socket,execute,magic}` | protocol-cli/socket; execute true |
| `cost.{memory,steps}`, optional `txBytes` | tx-cost-memory/steps strings; numeric tx-bytes |
| `time.{cliPath,socket,execute,magic}` | time-cli/socket; execute true |
| `sibling.{compilerBin,sourcePath,previousProgramHash}` | sibling-compiler/source, previous-program-hash |
| `keys.{cliPath,runDir,magic}` | key-cli plus run directory |
| `utxoQuery.{cliPath,socket,runDir,execute,magic}` | utxo-cli/socket plus run directory; execute true |
| `buildBody`, `financialBody`, `requiredSignerHash`, `cliPath` | build-body enables both booleans and forwards signer-hash/body-cli |
| `race.{submitBin,pollBin,signBin,logPath,leftActor,rightActor,leftResource,rightResource}` | submit-bin selects group; defaults release/cancel and release-resource/cancel-resource |
| `readback.{cliPath,socket,magic}` | readback-cli/socket |
| `receiptStatus` | receipt-status |
| `faucet.{bin,logPath}` | faucet-bin/log |

`detailedData`, `spendScriptPath`, `referenceScriptPath`, public role key material, `ownedProducer` and a serialized source envelope are **not forwarded by this CLI construction**. This is the exact omitted forwarding boundary to answer in Q3, not a proposal to add flags without a binding contract. `cli.mjs:234–295` has a separate nonintegrated native-case path for `pins` and `witness`; it calls `runNativeCase`, not `assembleConsumedFinancial`.

`cli.mjs:299–331` compile-custody has its own explicit source/seed/manifest/cases/expectation/residual/out input set. The complete callable signature at `compile-custody.mjs:202` additionally accepts `oracleResultsPath`, `aikenRun` and `syntheticTime`. The actual compiler performs evaluator and native Aiken work and is not an ordinary read command.

## Internal runNativeCaller fields

The following groups enumerate directly read request fields in `runNativeCaller:2770–3251` and its transaction/body helpers. Nested objects passed through to other modules retain those modules' contracts; no new admissibility judgment is implied.

| Group | Existing fields / source |
|---|---|
| Source and metadata | `sourcePath`, `evidenceRoot`; `sibling.{compilerBin,sourcePath,previousProgramHash}`; internally written `productionSiblingAdmitted`, `calibratedSiblingTerminals.{release,refund,cancel}.{lower,upper}`; optional `representableSpendBounds` same action/bound shape. Sources `225–425,1732–1749,2777–2788,2847–2860,2891–2906`. |
| Process/fixture input | `authority`, `launch`, `cashSource`, `cashVotes`, `cashDesign`, `protocol`, `keys`, `funder`, `utxoQuery`, `utxos`; `ownedProducer` (including executeNow/unitSpawn), `parityCapture`. Delegated calls at `2775–2889`. These fields can trigger native work; copied source is not an invocation. |
| Time and domain | `networkMagic`, `magic`; `time` with cliPath/socket/genesisPath/runDir and optional cliPosix/lucidPosix/scriptContextLower; measured stages can supersede them. Sources `1557–1573,2417,2468–2485,2825–2845`. |
| Body controls and quantities | `buildBody`, `financialBody`, `cliPath`, `outputLovelace`, `feeLovelace`, `requiredSignerHash`, `txBytes`, `cost.{memory,steps}`. `bodySpec:1557–1574`; branch selection `2908–3004`. |
| Public key/recipient inputs | `paymentVkeyCbor`, `payoutVkeyHex`, `payoutVkeyCbor`, `lenderVkeyCbor`, `borrowerVkeyCbor`. Body helper sources `1571–1572,1828,2428,2663–2666`. These are field names only; no key/wallet material is included. |
| Consumed-financial payload | `detailedData`, `spendScriptPath`, `referenceScriptPath`, `costMode`, `costSocket`, `costMagicSource`; optional readback.socket/time.socket participate in cost context. Sources `2408–2485,2667`. See exact shape/selection below. |
| Output/fee helper | `outputFee`, and `cliPath/evidenceRoot/ownedProducer.unitSpawn` in measurement helpers. `adoptHelper77` also has its own separate open request with `kind` and helper-specific fields; its signature is not the top-level public native CLI schema. Sources `2000–2031,2107–2185`. |
| Witness/receipt/race/readback | `witnessKeyPath`, `race` (also signingKeyPath/socket), `readback` (also magic/outputIndex/payoutIndex), `receiptRoot`, `keepReceipt`, `receiptLabel`, `receiptStatus`, `faucet`. Sources `3042–3251`. No signing paths are included or inspected here. |

## Exact consumed-financial selection and immediate inputs

`runNativeCaller` obtains `stages.inputs` from `selectRunInputs(request.utxos or queried utxos)` or, when no such list is selected, `inputsFromDispatch(stages.dispatch)`. The latter at `2041–2059` adds `twelveInputFrameSatisfied:true`, roleSource producer-dispatch, and `inputValues:[{ref,lovelaceText}]` after selecting named financial slots from dispatch results. `selectRunInputs:886–930` alone does not set that flag. The actual consuming branch condition at `2935` requires the flag; a twelve-element caller list by itself does not select it.

The twelve roles in order are cashSeed, head, mintFee, mintCollateral, headFee, headCollateral, releaseFee, releaseCollateral, refundFee, refundCollateral, cancelFee, cancelCollateral. The selection result has left/right/cancel fee/collateral pairs. All are existing source roles; this packet does not judge their authenticity.

`assembleConsumedFinancial(stages,request):2408–2697` directly uses:

- `request.detailedData`: object with constructor exactly 0 and an array of exactly 13 fields; otherwise `DETAILED_DATUM_ABSENT`. Its positional meanings are the closed T1 HeadDatum described in README; the immediate shape check alone is not an authenticated relation.
- `stages.inputs`: role refs plus inputValues; `stages.time.windows`: release/partition slot pair; domain magic from producer stage or request networkMagic/magic. Subsequent stage/helper requirements remain in the full source.
- `request.paymentVkeyCbor`: public payment verification key CBOR; payout selection uses lender/payout for release and borrower/payout for refund/cancel.
- `request.spendScriptPath`: existing V3 script envelope used by encodeSpendBody; `request.referenceScriptPath`: separately optional file measured by output minimum helper. Neither field chooses an authoritative generated producer here.
- Source terminal path: calibrated sibling source when present, otherwise sourcePath or sibling.sourcePath. `loadSourceTerminals` reads the existing designer cases/seed/residual; it does not accept an arbitrary serialized source envelope supplied in this request.
- `request.time.genesisPath`, costMode/socket/magicSource and readback/time socket fallback; the helper still constructs unit era-history and uses `OFFLINE_PROTOCOL_PARAMS` internally.
- Internally selected `UNIT_CASH_POLICY`, `UNIT_HEAD_POLICY`, amount 100 and asset names Cash/Head, with current fee/budget/minimum-output helper flow. An input filename does not replace these bindings.

Final `encodeSpendBody(spec,cliPath)` input contract is its full source at `native-body.mjs:545–720`: head/fee/collateral refs, spendScriptPath, redeemerJson, two outputs (continuing inlineDatumJson and assets; payout assets), network magic, signer/public payout keys, fees/interval, protocol/budget, input values/change and optional output path. It reads/writes artifacts and can spawn cardano-cli. It was not executed.

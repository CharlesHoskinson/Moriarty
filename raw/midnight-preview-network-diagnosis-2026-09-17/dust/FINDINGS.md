# DUST bootstrap source findings

Read-only research, 2026-09-17. No wallet was opened, no transaction was built or submitted, and no package was installed. The parent investigation owns runtime evidence and canonical wiki integration.

## Source facts

- [DUST architecture](https://docs.midnight.network/concepts/dust-architecture#registrations-and-fees), updated September 16, explicitly supports first registration without existing DUST. Only unregistered NIGHT inputs consumed in the guaranteed section provide retroactive capacity. The allowance cannot exceed capacity accrued since each input's creation; excess produces `InsufficientDustForRegistrationFee`. Respending recreates NIGHT outputs under the registration for ongoing generation. Captured HTML and receipt: `dust-architecture.html*`.
- [Acquire tokens](https://docs.midnight.network/guides/acquire-tokens) prescribes synced state, select unregistered NIGHT, `registerNightUtxosForDustGeneration(coins, publicKey, sign, optionalReceiver)`, `finalizeRecipe`, then `submitTransaction`. Wait for synced positive DUST after on-chain registration. The helper follows this sequence. The guide uses Preprod examples; faucet denomination in those examples is not a Preview requirement. Captured HTML and receipt: `acquire-tokens.html*`.
- [Ledger implementation at pinned commit](https://github.com/midnightntwrk/midnight-ledger/blob/5410620fd43bbfe1c11d79499776ae228991f001/ledger/src/dust.rs#L1167): `generationless_fee_availability` selects guaranteed NIGHT inputs owned by the registering key, excludes entries in `night_indices`, reads authoritative UTXO creation metadata, calculates whole elapsed seconds at the transaction's DUST action timestamp, and caps the product of age, value, and rate. `dust.rs` lines 808–829 separately check declared allowance against availability. `error.rs` lines 938–944 render the available/requested distinction. Git remote HEAD and default branch resolve to ledger-8 at this commit; this pin is not proven identical to deployed ledger 8.1.2.

## Repository observations

- The inspected installed facade is version 4.1.0, beneath the umbrella wallet-sdk package. It automatically selects the guaranteed input, rotates NIGHT, attaches registration, checks projected fee sufficiency, and uses `signRecipe` before returning. Snapshot: `installed-sdk/wallet-sdk-facade/dist/index.js`, lines 232–300. Published-tarball equivalence was not checked.
- Its dust implementation selects one input with highest projected DUST for the guaranteed section, placing the remainder in the fallible section. The allowance sums only unregistered guaranteed inputs. Snapshots: `installed-sdk/wallet-sdk-dust-wallet/dist/v1/{CoinsAndBalances,Transacting}.js`.
- The helper's fee estimate includes the configured `additionalFeeOverhead: 300_000_000_000_000n` in `hello-world/src/wallet.ts:116`. The estimate `300000000000001` is not evidence of an enormous fee: it is about 0.3 DUST in ledger units.

## Inferences and next discriminator

With initial parameters, one 5,000 NIGHT input means 5,000,000,000 Stars and generates 41,335,000,000,000 Specks per second. Roughly eight whole seconds covers the stated estimate; live parameters and the authoritative input age must be checked before applying this calculation. Zero wallet DUST before registration is expected and does not establish a bootstrap funding failure.

Possible failure classes remain (1) fee allowance versus actual retroactive capacity, including timestamp/metadata disagreement, (2) malformed or invalid transaction, such as signature or version mismatch, and (3) submission transport failure or unknown finality. None is established by `NestedSubmissionError` alone. Normal WebSocket closure during cleanup does not establish the submission cause.

Minimal discriminating check: inspect the already-retained rejection's nested cause or read-only node preflight response, and the existing finalized transaction's DUST action timestamp, declared allowance, guaranteed input value/identity, and registration-signature verification. Compare the input creation time and registration status from authoritative indexed/ledger data, with actual ledger parameters. Do not issue another submission merely to discover the error. A specific fee error distinguishes allowance failure from signature/serialization and transport cases.

## Acquisition limits

Scrapling fetched the official docs after robots.txt allowed all paths. SHA-256, requested/canonical URL, retrieval time and status are in adjacent receipts. Text files are selector-limited derivatives of the HTML, not independent sources. The unauthenticated GitHub API returned HTTP 403 rate-limit exceeded; Git remote inspection and existing Git objects supplied pinned source instead. No conclusion establishes the deployed runtime's exact source identity, successful registration, or new DUST generation.

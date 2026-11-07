<!-- source: https://docs.midnight.network/relnotes/compact-js/compact-js-3-0-0.md fetched_at: 2026-11-07T20:48:45Z status: 200 untrusted -->

# Compact.js 3.0.0 release notes

> For the complete documentation index, see [llms.txt](https://docs.midnight.network/llms.txt)

* **Version**: 3.0.0
* **Date**: 2026-11-05
* **Environment**: Mainnet, Preview

***

## High-level summary[​](#high-level-summary "Direct link to High-level summary")

Compact.js 3.0.0 is the major release for the [ledger 9 network upgrade](https://docs.midnight.network/relnotes/ledger-9.md). Midnight.js 5 uses it to run contracts on Mainnet after the upgrade and on Preview, which has run ledger 9 since 6 November 2026. It moves the ledger 9 era to Compact runtime 0.21.0 and Compact toolchain 0.36.0, builds against `@midnightntwrk/ledger-v9` 1.0.0 and `@midnightntwrk/onchain-runtime-v4` 4.0.0, and publishes as ESM only. The ledger 8 era is unchanged.

Preprod stays on ledger 8 until a later upgrade. For Preprod, keep using Compact.js 2.5.x with Midnight.js 4.1.1.

***

## Audience[​](#audience "Direct link to Audience")

This release is relevant for developers who:

* Move a DApp to Midnight.js 5 for Mainnet or Preview
* Use `@midnight-ntwrk/compact-js` or `@midnight-ntwrk/compact-js-command` directly

***

## New features[​](#new-features "Direct link to New features")

* The packages expose `/v8`, `/v8/effect`, `/v9`, and `/v9/effect` entry points beside the default entry, so you pick the ledger era at import. Both eras can run in one process.
* Calls that overwrite existing ledger state no longer fail the gas-budget check.

***

## Breaking changes[​](#breaking-changes "Direct link to Breaking changes")

* **Recompile ledger 9 contracts with Compact toolchain 0.36.0.** Artifacts from toolchain 0.35.0 or earlier declare Compact runtime 0.20 and fail the runtime version check on import. Ledger 8 artifacts on the `/v8` entry stay on Compact runtime 0.16 and need no recompile.
* **Recompiling changes a deployed contract's verifier keys.** Don't re-attach a deployed contract with freshly compiled artifacts. Keep its deployed artifacts and the toolchain that built them, add the new verifier keys through its maintenance authority, or redeploy it. Pin the toolchain version in CI so a routine rebuild can't change the keys. See [Recompiling a contract changes its verifier keys](https://docs.midnight.network/relnotes/ledger-9.md#recompiling-a-contract-changes-its-verifier-keys).
* **Circuits declared in a contract type can't be `pure`.** Remove `pure` from those declarations, and disclose the arguments of every cross-contract call.
* **Recompile contracts that make cross-contract calls**, callee side included, to pick up a fix in Compact toolchain 0.36.0.
* **The ledger 9 contract operation version moves from `v3` to `v4`.** Rebuild maintenance updates with Compact.js 3.0.0 rather than resubmitting serialized `v3` updates.
* **The CommonJS build is removed.** Replace `require` call sites with `import`, and set `"type": "module"` in your package or use the `.mjs` extension.
* **Type surfaces are aware of the ledger era.** Code that assumed a single ledger era must select the era explicitly.

***

## Known issues[​](#known-issues "Direct link to Known issues")

* Maintenance updates that remove a verifier key are rejected with `BalanceCheckOverspend` while client packages use ledger 9.1.0.0 ([midnight-node#2375](https://github.com/midnightntwrk/midnight-node/issues/2375)). Inserting keys works. A cross-contract caller that needs the Compact toolchain 0.36.0 fix is redeployed until then. See [Known issues](https://docs.midnight.network/relnotes/ledger-9.md#known-issues) on the ledger 9 upgrade page.
* Building a contract context fails with a `TypeError` when two identical copies of a compiled contract package are resolved in one process ([compact-js#28](https://github.com/midnightntwrk/compact-js/issues/28)). Deduplicate the contract package.
* The CLI `circuit` command rejects a valid null `--input-ps` ([compact-js#30](https://github.com/midnightntwrk/compact-js/issues/30)). Give the contract a non-null initial private state.

***

## Important notes[​](#important-notes "Direct link to Important notes")

Install Compact.js 3.0.0 through Midnight.js 5 where you can, so the versions stay paired. See the [Midnight.js 5.0.0 release notes](https://docs.midnight.network/relnotes/midnight-js/midnight-js-5-0-0.md).

***

## Links and references[​](#links-and-references "Direct link to Links and references")

* [GitHub release: compact-js-v3.0.0](https://github.com/midnightntwrk/compact-js/releases/tag/compact-js-v3.0.0)
* [`@midnight-ntwrk/compact-js` on npm](https://www.npmjs.com/package/@midnight-ntwrk/compact-js/v/3.0.0)
* [Ledger 9 network upgrade](https://docs.midnight.network/relnotes/ledger-9.md)
* [Compact.js 2.5.3 release notes](https://docs.midnight.network/relnotes/compact-js/compact-js-2-5-3.md)

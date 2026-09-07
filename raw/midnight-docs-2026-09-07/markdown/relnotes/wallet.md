> For the complete documentation index, see [llms.txt](/llms.txt)

# Wallet SDK

Midnight Wallet SDK is the SDK of the digital wallet designed for the Midnight blockchain, enabling users to securely store private keys, manage assets, and interact with decentralized applications.

[Link to related documentation](/sdks/official/wallet-developer-guide.md)

To download the component, click the appropriate link under **Artifacts**.

Version numbering

From the 1.0.0 entry dated 23 April 2026 onward, this list tracks the `@midnightntwrk/wallet-sdk` umbrella package version. Earlier entries (up to 3.0.0) used the version of the facade sub-package, so the numbering restarts once below.

***

<!-- -->

Version1.2.0

StatusAll

### [Release <!-- -->1.2.0](/relnotes/wallet/wallet-sdk-1-2-0)LATEST

1 July 2026

#### Artifacts

* [NPM Package](https://www.npmjs.com/package/@midnightntwrk/wallet-sdk/v/1.2.0)

#### Summary

* Fixed a race in `WalletFacade.registerNightUtxosForDustGeneration` where the registration fee could exceed `allow_fee_payment`, causing `BalanceCheckOverspend` rejections; the wallet now estimates the fee at build time and throws before submission.
* Added `WalletFacade.waitForGeneratedDust(utxos, requiredAmount, opts?)` to defer registration until enough DUST has accrued; pair with `estimateRegistration`.
* The SDK now publishes under the `@midnightntwrk` npm scope; 1.2.0 is available under both scopes.

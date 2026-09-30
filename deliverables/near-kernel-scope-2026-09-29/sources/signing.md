[Skip to main content](#content-area)

* [System Status](https://status.near-intents.org/posts/dashboard)
* [Explorer](https://explorer.near-intents.org/)
* [Support](https://t.me/near_intents)
* [Partners](https://partners.near-intents.org/)

[Swaps](/getting-started/what-are-intents)[Market Makers](/integration/market-makers/introduction)[Bridges](/integration/bridging/overview)[Verifier Contract](/integration/verifier-contract/introduction)[Learn](/learn/omni-bridge/overview)[Changelog](/changelog/overview)

[NEAR Intents home page![light logo](https://mintcdn.com/defuselabsltd/H7BXykL_JPKw4xWb/images/logo/light.svg?fit=max&auto=format&n=H7BXykL_JPKw4xWb&q=85&s=e31b086fb76c5f6228ea4020afc8db58)![dark logo](https://mintcdn.com/defuselabsltd/H7BXykL_JPKw4xWb/images/logo/dark.svg?fit=max&auto=format&n=H7BXykL_JPKw4xWb&q=85&s=24c0664a9d5a23125fbdbafce3213d1f)](/)

Search...

⌘KAsk Assistant⌘I

* [System Status](https://status.near-intents.org/posts/dashboard)
* [Explorer](https://explorer.near-intents.org/)
* [Support](https://t.me/near_intents)
* [Partners](https://partners.near-intents.org/)
* [Partners](https://partners.near-intents.org/)

Search...

Navigation

Intents

Signing Intents

[Swaps](/getting-started/what-are-intents)[Market Makers](/integration/market-makers/introduction)[Bridges](/integration/bridging/overview)[Verifier Contract](/integration/verifier-contract/introduction)[Learn](/learn/omni-bridge/overview)[Changelog](/changelog/overview)

Intents

Signing Intents
===============

Copy page

How to sign intents for different wallet types and signing standards

Copy page

After creating intents, they must be signed before submission to the Verifier contract via the `execute_intents` function. The Verifier supports [multiple signature standards](#signature-types) to enable signing from NEAR, Ethereum, TRON, Solana, Stellar, TON, and passkey-based wallets.

* For intents structure before signing, see [Intent Types and Execution](/integration/verifier-contract/intent-types-and-execution).
* For key management, see [Account Abstraction](/integration/verifier-contract/account-abstraction).

**Encoding Requirements for the Verifier Contract****Important:** Compressed public keys are not supported for ECDSA curves (Secp256k1, P256). Public keys must be in uncompressed format (raw 64-byte x || y coordinates without prefix bytes).Signatures must be in raw concatenated byte format, not DER-encoded.

Every public key registered to an account can sign intents on its behalf. See [Account Abstraction](/integration/verifier-contract/account-abstraction) for key management details.

Signature types
---------------

Different wallets use different signing standards. To allow users to sign with their existing wallet, the Verifier supports [multiple verification methods](https://near.github.io/intents/defuse_core/payload/multi/enum.MultiPayload.html) — each corresponding to a specific wallet ecosystem (e.g., ERC-191 for MetaMask, Raw Ed25519 for Phantom). Each signed intent conforms to the [MultiPayload](https://near.github.io/intents/defuse_core/payload/multi/enum.MultiPayload.html) enum.

### NEP-413

The [NEP-413 standard](https://github.com/near/NEPs/blob/master/neps/nep-0413.md) is an off-chain message signing standard recognized by NEAR wallets.

### ERC-191

Compliant with the [ERC-191 standard](https://eips.ethereum.org/EIPS/eip-191) for off-chain message signing (Ethereum wallets like MetaMask).

There is no `public_key` field because it can be recovered from the secp256k1 signature and data.

Ethereum clients shift the recovery byte (`v`) based on chain ID. The Verifier contract expects `v ∈ {0, 1}`, so clients must normalize the recovery byte before submission.

### TIP-191

Compliant with [TIP-191](https://github.com/tronprotocol/tips/blob/master/tip-191.md), TRON’s off-chain message signing standard. TIP-191 is fully compatible with ERC-191.

Like ERC-191, there is no `public_key` field because it can be recovered from the secp256k1 signature and data. The same [recovery byte normalization](#erc-191) applies.

### Raw Ed25519

Used by [Phantom wallet for Solana off-chain message signing](https://docs.phantom.com/solana/signing-a-message).

### WebAuthn (Passkey)

For use with [passkeys](https://en.wikipedia.org/wiki/WebAuthn) and the [Web Authentication standard](https://w3c.github.io/webauthn/). The [signature](https://near.github.io/intents/defuse_webauthn/enum.Signature.html) can use either [Ed25519 or P256 (secp256r1)](https://www.iana.org/assignments/cose/cose.xhtml#algorithms).

### TonConnect

Follows the [standard for data signing](https://docs.tonconsole.com/academy/sign-data) on TON.

### SEP-53

Compliant with [SEP-53](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0053.md), Stellar’s standard for signing arbitrary messages using Stellar key pairs.

Adding more signature types
---------------------------

To support additional key or signature types, contact the NEAR Intents team via [Telegram](https://t.me/near_intents).

Was this page helpful?

YesNo

[Previous](/integration/verifier-contract/intent-types-and-execution)[Simulating Intents

Test intents without modifying blockchain state using the simulate\_intents function

Next](/integration/verifier-contract/simulating-intents)

⌘I

[x](https://x.com/near_intents)[github](https://github.com/defuse-protocol)[telegram](https://t.me/near_intents)

[Powered byThis documentation is built and hosted on Mintlify, a developer documentation platform](https://www.mintlify.com?utm_campaign=poweredBy&utm_medium=referral&utm_source=defuselabsltd)
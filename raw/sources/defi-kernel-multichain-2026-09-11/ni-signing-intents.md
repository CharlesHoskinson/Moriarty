> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Signing Intents

> How to sign intents for different wallet types and signing standards

After creating intents, they must be signed before submission to the Verifier contract via the `execute_intents` function. The Verifier supports [multiple signature standards](#signature-types) to enable signing from NEAR, Ethereum, TRON, Solana, Stellar, TON, and passkey-based wallets.

* For intents structure before signing, see [Intent Types and Execution](/integration/verifier-contract/intent-types-and-execution).
* For key management, see [Account Abstraction](/integration/verifier-contract/account-abstraction).

<Warning>
  **Encoding Requirements for the Verifier Contract**

  | Curve     | Public Key              | Signature                  |
  | --------- | ----------------------- | -------------------------- |
  | Ed25519   | 32 bytes                | 64 bytes                   |
  | Secp256k1 | 64 bytes (uncompressed) | 65 bytes (r \|\| s \|\| v) |
  | P256      | 64 bytes (uncompressed) | 64 bytes (r \|\| s)        |

  **Important:** Compressed public keys are not supported for ECDSA curves (Secp256k1, P256). Public keys must be in uncompressed format (raw 64-byte x || y coordinates without prefix bytes).

  Signatures must be in raw concatenated byte format, not DER-encoded.
</Warning>

<Info>
  Every public key registered to an account can sign intents on its behalf. See [Account Abstraction](/integration/verifier-contract/account-abstraction) for key management details.
</Info>

## Signature types

Different wallets use different signing standards. To allow users to sign with their existing wallet, the Verifier supports [multiple verification methods](https://near.github.io/intents/defuse_core/payload/multi/enum.MultiPayload.html) — each corresponding to a specific wallet ecosystem (e.g., ERC-191 for MetaMask, Raw Ed25519 for Phantom). Each signed intent conforms to the [MultiPayload](https://near.github.io/intents/defuse_core/payload/multi/enum.MultiPayload.html) enum.

### NEP-413

The [NEP-413 standard](https://github.com/near/NEPs/blob/master/neps/nep-0413.md) is an off-chain message signing standard recognized by NEAR wallets.

| Field               | Description                                                                       |
| ------------------- | --------------------------------------------------------------------------------- |
| `standard`          | `"nep413"`.                                                                       |
| `payload.recipient` | Contract address (e.g., `intents.near`). Prevents replay on other contracts.      |
| `payload.nonce`     | Base64-encoded 256-bit nonce.                                                     |
| `payload.message`   | Serialized JSON payload string containing `signer_id`, `deadline`, and `intents`. |
| `public_key`        | Signer's public key. Encoded as key type prefix + base58 (e.g., `ed25519:...`).   |
| `signature`         | Cryptographic signature. Encoded as key type prefix + base58.                     |

```json theme={null}
{
  "standard": "nep413",
  "payload": {
    "recipient": "intents.near",
    "nonce": "Vij2xgAlKBKzgNPJFViEQRgYyS7p2NEiYTTY4XmT8go=",
    "message": "{\"deadline\":\"2025-05-21T10:34:04.254392Z\",\"intents\":[{\"intent\":\"transfer\",\"receiver_id\":\"bob.near\",\"tokens\":{\"nep141:usdc.near\":\"10\"}}],\"signer_id\":\"alice.near\"}"
  },
  "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
  "signature": "ed25519:2ns3eCikZFxFA6e9ApunuDG4XwJUPrB8q82fYViBKjheqiDYcCmLN9RnB9NhoEvCdNoS3L3qYD9FAnKd7UkTnuij"
}
```

### ERC-191

Compliant with the [ERC-191 standard](https://eips.ethereum.org/EIPS/eip-191) for off-chain message signing (Ethereum wallets like MetaMask).

| Field       | Description                                                                       |
| ----------- | --------------------------------------------------------------------------------- |
| `standard`  | `"erc191"`.                                                                       |
| `payload`   | Serialized JSON payload string. The `signer_id` is the Ethereum address.          |
| `signature` | Secp256k1 signature. Encoded as key type prefix + base58 (e.g., `secp256k1:...`). |

<Info>
  There is no `public_key` field because it can be recovered from the secp256k1 signature and data.
</Info>

<Warning>
  Ethereum clients shift the recovery byte (`v`) based on chain ID. The Verifier contract expects `v ∈ {0, 1}`, so clients must normalize the recovery byte before submission.
</Warning>

```json theme={null}
{
  "standard": "erc191",
  "payload": "{\"signer_id\":\"0xca67C1Bb3FD69857E5edaF6aA1c65371bF46A464\",\"verifying_contract\":\"intents.near\",\"deadline\":\"2025-05-26T13:24:16.983Z\",\"nonce\":\"Vij2xgAlKBKzwGNqwogWQxiy87p9jW5Omfg+L9bXBDw=\",\"intents\":[{\"intent\":\"token_diff\",\"diff\":{\"nep141:usdc.near\":\"-1000\",\"nep141:usdt.near\":\"1000\"}},{\"intent\":\"ft_withdraw\",\"token\":\"usdt.near\",\"receiver_id\":\"bob.near\",\"amount\":\"1000\"}]}",
  "signature": "secp256k1:BoR53NKKJ2GfP8bETk427Xav8VMpUcZovLzXeq2EsUD7NnPH2sSmxWFEkDUrguJHHkiu3GwEQxxMo2Rm2ZTDHFygU"
}
```

### TIP-191

Compliant with [TIP-191](https://github.com/tronprotocol/tips/blob/master/tip-191.md), TRON's off-chain message signing standard. TIP-191 is fully compatible with ERC-191.

| Field       | Description                                                                       |
| ----------- | --------------------------------------------------------------------------------- |
| `standard`  | `"tip191"`.                                                                       |
| `payload`   | Serialized JSON payload string. The `signer_id` is the TRON address.              |
| `signature` | Secp256k1 signature. Encoded as key type prefix + base58 (e.g., `secp256k1:...`). |

<Info>
  Like ERC-191, there is no `public_key` field because it can be recovered from the secp256k1 signature and data. The same [recovery byte normalization](#erc-191) applies.
</Info>

### Raw Ed25519

Used by [Phantom wallet for Solana off-chain message signing](https://docs.phantom.com/solana/signing-a-message).

| Field        | Description                                                                                   |
| ------------ | --------------------------------------------------------------------------------------------- |
| `standard`   | `"raw_ed25519"`.                                                                              |
| `payload`    | Serialized JSON payload string. Note: `deadline` uses a Unix `timestamp` instead of ISO 8601. |
| `public_key` | Signer's public key. Encoded as key type prefix + base58.                                     |
| `signature`  | Ed25519 signature. Encoded as key type prefix + base58.                                       |

```json theme={null}
{
  "standard": "raw_ed25519",
  "payload": "{\"signer_id\":\"alice.near\",\"verifying_contract\":\"intents.near\",\"deadline\":{\"timestamp\":1732035219},\"nonce\":\"Vij2xgAlKBKzwGNqwogWQxi9ZuGDNBXlmdy9g3MQSMk=\",\"intents\":[{\"intent\":\"token_diff\",\"diff\":{\"nep141:usdc.near\":\"-1000\",\"nep141:usdt.near\":\"998\"}}]}",
  "public_key": "ed25519:9aqyNsCRrDkq9SceVRCdW1Zs9BmEP8ieJQRJim8iRLF",
  "signature": "ed25519:9RHRenkbYN6cfWXkou47sjAf1PuTc6nYM8amHADApfqio8dcMQu28cgfz6wkktBFoE8J7FsZ3rxifFdMdzTTJi6"
}
```

### WebAuthn (Passkey)

For use with [passkeys](https://en.wikipedia.org/wiki/WebAuthn) and the [Web Authentication standard](https://w3c.github.io/webauthn/). The [signature](https://near.github.io/intents/defuse_webauthn/enum.Signature.html) can use either [Ed25519 or P256 (secp256r1)](https://www.iana.org/assignments/cose/cose.xhtml#algorithms).

| Field                | Description                                                                              |
| -------------------- | ---------------------------------------------------------------------------------------- |
| `standard`           | `"webauthn"`.                                                                            |
| `payload`            | Serialized JSON payload string.                                                          |
| `public_key`         | Signer's public key. Encoded as key type prefix (`p256` or `ed25519`) + base58.          |
| `signature`          | Cryptographic signature. Encoded as key type prefix (`p256` or `ed25519`) + base58.      |
| `client_data_json`   | WebAuthn client data JSON. Contains `type`, `challenge` (base64 URL-safe), and `origin`. |
| `authenticator_data` | Authenticator assertion data. Encoded as base64, URL-safe.                               |

```json theme={null}
{
  "standard": "webauthn",
  "payload": "{\"signer_id\":\"0x3602b546589a8fcafdce7fad64a46f91db0e4d50\",\"verifying_contract\":\"intents.near\",\"deadline\":\"2025-03-30T00:00:00Z\",\"nonce\":\"A3nsY1GMVjzyXL3mUzOOP3KT+5a0Ruy+QDNWPhchnxM=\",\"intents\":[{\"intent\":\"transfer\",\"receiver_id\":\"bob.near\",\"tokens\":{\"nep141:usdc.near\":\"1000\"}}]}",
  "public_key": "p256:2V8Np9vGqLiwVZ8qmMmpkxU7CTRqje4WtwFeLimSwuuyF1rddQK5fELiMgxUnYbVjbZHCNnGc6fAe4JeDcVxgj3Q",
  "signature": "p256:3KBMZ72BHUiVfE1ey5dpi3KgbXvSEf9kuxgBEax7qLBQtidZExxxjjQk1hTTGFRrPvUoEStfrjoFNVVW4Abar94W",
  "client_data_json": "{\"type\":\"webauthn.get\",\"challenge\":\"4cveZsIe6p-WaEcL-Lhtzt3SZuXbYsjDdlFhLNrSjjk\",\"origin\":\"https://defuse-widget-git-feat-passkeys-defuse-94bbc1b2.vercel.app\"}",
  "authenticator_data": "933cQogpBzE3RSAYSAkfWoNEcBd3X84PxE8iRrRVxMgdAAAAAA=="
}
```

### TonConnect

Follows the [standard for data signing](https://docs.tonconsole.com/academy/sign-data) on TON.

| Field          | Description                                                                       |
| -------------- | --------------------------------------------------------------------------------- |
| `address`      | TON wallet address.                                                               |
| `domain`       | Domain associated with the signing request.                                       |
| `timestamp`    | ISO 8601 timestamp of the signing request.                                        |
| `payload.type` | `"text"`.                                                                         |
| `payload.text` | Serialized JSON payload string containing `signer_id`, `deadline`, and `intents`. |
| `public_key`   | Signer's public key. Encoded as key type prefix + base58.                         |
| `signature`    | Ed25519 signature. Encoded as key type prefix + base58.                           |

```json theme={null}
{
  "address": "EXvSRnDlYHziOJRm1MqGLgQB3EN7319eZLYWVinpoPv7LkBd",
  "domain": "example.com",
  "timestamp": "2025-01-01T00:00:00Z",
  "payload": {
    "type": "text",
    "text": "{\"signer_id\":\"alice.near\",\"verifying_contract\":\"intent.near\",\"deadline\":\"2025-05-26T15:19:43.617898Z\",\"nonce\":\"ZnbiFf4tP4cn65XLuZ6T1H6/Vr3o6ucNftdx3pInLnc=\",\"intents\":[{\"intent\":\"ft_withdraw\",\"token\":\"usdc.near\",\"receiver_id\":\"bob.near\",\"amount\":\"1000\"}]}"
  },
  "public_key": "ed25519:G4HVCaJg9vZb2srcLoWxR9grQ3tGLNFMVrZBhTtBi4Q1",
  "signature": "ed25519:5cwYdTNeGy1mApo9RNor9hSXvcG6GbvVm6di6kuf4frnARtVWRpJoPtvFKHMbt7uDGDtgFfn6bPDFGPK5jamqBwC"
}
```

### SEP-53

Compliant with [SEP-53](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0053.md), Stellar's standard for signing arbitrary messages using Stellar key pairs.

| Field        | Description                                                                     |
| ------------ | ------------------------------------------------------------------------------- |
| `standard`   | `"sep53"`.                                                                      |
| `payload`    | Serialized JSON payload string.                                                 |
| `public_key` | Signer's public key. Encoded as key type prefix + base58 (e.g., `ed25519:...`). |
| `signature`  | Ed25519 signature. Encoded as key type prefix + base58.                         |

## Adding more signature types

To support additional key or signature types, contact the NEAR Intents team via [Telegram](https://t.me/near_intents).

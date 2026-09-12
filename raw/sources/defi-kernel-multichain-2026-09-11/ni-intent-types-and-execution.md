> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Intent Types and Execution

> Available intent types and how to structure them for the Verifier contract

An intent is a desired action on your account in the Verifier smart contract, performed by submitting a transaction to the NEAR blockchain. The Verifier allows submitting a list of actions using [the function](https://near.github.io/intents/defuse/intents/trait.Intents.html#tymethod.execute_intents) `execute_intents`.

This page covers how intents are structured, wrapped into signed payloads, and the [available intent types](#available-intent-types) you can submit.

* For cryptographic signing details, see [Signing Intents](/integration/verifier-contract/signing-intents).

* To test intents before submitting, see [Simulating Intents](/integration/verifier-contract/simulating-intents).

## Ordering and atomicity

Multiple intents can be submitted to `execute_intents` in a list, where they execute in the order provided.

<Warning>
  **Important:** Because NEAR is an asynchronous and sharded blockchain, intents submitted in sequence do not guarantee they will *complete* in that sequence. While individual intents execute in order, there is no guarantee that [cross-contract calls](https://docs.near.org/smart-contracts/anatomy/crosscontract) originating from the Verifier will finish in order.
</Warning>

**Example of potential ordering issues:**

1. Intent 1: Perform a [storage deposit](https://nomicon.io/Standards/StorageManagement) on `usdc.near`
2. Intent 2: Withdraw native NEAR to `usdc.near`

The `usdc.near` contract requires a storage deposit before tokens can be deposited. While Intent 1 executes first, there's no guarantee the storage deposit call will complete before Intent 2's withdrawal call is made.

## Intent structure

Intents are submitted as JSON objects in a payload. Here's an example [Transfer](https://near.github.io/intents/defuse_core/intents/tokens/struct.Transfer.html) intent for wNEAR tokens from Alice to Bob:

```json theme={null}
{
  "intent": "transfer",
  "receiver_id": "bob.near",
  "tokens": {
    "nep141:wrap.near": "10"
  }
}
```

<Info>
  The intent does not mention Alice because *the signer* of the intent defines which account performs the transfer.
</Info>

## Payload structure

The intent is wrapped in a payload with these required fields:

| Field                | Description                             |
| -------------------- | --------------------------------------- |
| `signer_id`          | Signer account ID.                      |
| `verifying_contract` | Contract address receiving the intents. |
| `deadline`           | Timestamp in ISO 8601 format.           |
| `nonce`              | 256-bit unique value (see below).       |
| `intents`            | Array of intents to execute.            |

```json theme={null}
{
  "signer_id": "alice.near",
  "verifying_contract": "intents.near",
  "deadline": "2025-05-21T12:23:04.252814Z",
  "nonce": "Vij2xgAlKBKzAMcLzEqKQRhRHXp3ThAEFTYtBmfhzvE=",
  "intents": [
    {
      "intent": "transfer",
      "receiver_id": "bob.near",
      "tokens": {
        "nep141:wrap.near": "10"
      }
    }
  ]
}
```

### Nonce structure

The nonce is a 256-bit (32-byte) value encoded as base64. It must be unique per intent to prevent replay attacks.

**Structure:** `[4-byte salt][28-byte unique data]`

* **Salt (first 4 bytes):** Must match the current contract salt (retrieve via [`simulate_intents`](/integration/verifier-contract/simulating-intents))
* **Unique data (remaining 28 bytes):** Can be random or sequential, as long as it's never reused for the same account

<Info>
  The salt rotates periodically. Always fetch the current salt before constructing intents. See the [nonce documentation](https://github.com/near/intents/tree/main/defuse#nonces) for implementation details.
</Info>

<Tip>
  You can fetch the current salt in two ways:

  1. **Via [`simulate_intents`](/integration/verifier-contract/simulating-intents)** — the response includes the current salt alongside simulation results.

  2. **Directly via `current_salt`** — a dedicated view method on the Verifier contract:

  ```bash cURL theme={null}
  curl -s <NEAR_RPC_URL> \
    -H 'Content-Type: application/json' \
    -d '{
      "jsonrpc": "2.0",
      "id": "dontcare",
      "method": "query",
      "params": {
        "request_type": "call_function",
        "finality": "final",
        "account_id": "intents.near",
        "method_name": "current_salt",
        "args_base64": "e30="
      }
    }' | jq -r '.result.result | implode'
  ```

  The response is a hex string representing the 4-byte salt prefix, for example `"252812b3"`.
</Tip>

## Signed intent format

To create a valid, signed intent for the `execute_intents` function, wrap the payload in a message string and sign it. See [Signing Intents](/integration/verifier-contract/signing-intents) for supported signature types (NEP-413, ERC-191, WebAuthn, and more). Note that the message is the same JSON serialized as a one-liner with escaped quotes:

```json theme={null}
{
  "standard": "nep413",
  "payload": {
    "recipient": "intents.near",
    "nonce": "Vij2xgAlKBKzgNPJFViEQRiSQad02gXP6pv9IQRCFeg=",
    "message": "{\"deadline\":\"2025-05-21T10:34:04.254392Z\",\"intents\":[{\"intent\":\"transfer\",\"receiver_id\":\"bob.near\",\"tokens\":{\"nep141:usdc.near\":\"10\"}}],\"signer_id\":\"alice.near\"}"
  },
  "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
  "signature": "ed25519:2o7Cg7N8bEAKtEDbC9Nja8Ks1bJ9Nunh5Ems51G8oV6n96ckUVFeT81vr3TouE47R24HJSrLyxdeBEbvWeuizVBZ"
}
```

The `recipient` field prevents replay attacks on other copies of the Verifier.

<Info>
  All signed examples on this page use the same test key (`ed25519:C3jX...`). In production, each signer uses their own key pair.
</Info>

## Available intent types

The following [intents](https://near.github.io/intents/defuse_core/intents/enum.Intent.html) can be submitted to the Verifier contract.

<Info>
  Rust `PascalCase` names are converted to `snake_case` in JSON. For example, `TokenDiff` becomes `token_diff`.
</Info>

### add\_public\_key

Adds a public key to an account in the Verifier contract. The added key's private key can sign intents on behalf of this account, including adding new keys.

<Warning>
  Implicit account IDs have their corresponding public keys added by default. If a private key is leaked for an implicit account, you must manually rotate the public key in the Verifier.
</Warning>

Public keys can also be added [via transactions](/integration/verifier-contract/account-abstraction).

| Parameter    | Description                                                 |
| ------------ | ----------------------------------------------------------- |
| `public_key` | The public key to add to the account (e.g., `ed25519:...`). |

```json theme={null}
{
  "intent": "add_public_key",
  "public_key": "ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm"
}
```

<Accordion title="Full signed intent example">
  The `message` field contains the intent JSON above, serialized as a single line with escaped quotes.

  ```json theme={null}
  [
    {
      "standard": "nep413",
      "payload": {
        "recipient": "intents.near",
        "nonce": "Vij2xgAlKBKzwKyKBC58QRjK3paApkNO6LU77m306Vw=",
        "message": "{\"deadline\":\"2025-05-21T08:04:27.483198Z\",\"intents\":[{\"intent\":\"add_public_key\",\"public_key\":\"ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm\"}],\"signer_id\":\"alice.near\"}"
      },
      "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
      "signature": "ed25519:M6gK8WMNBL9vnkuScshNuKF5Yro5gQNRLpTTbruadR54AQvi53xWw8NizyCRbdM3j6y9XZ7MJy4DtQ1JLDz4xGQ"
    }
  ]
  ```
</Accordion>

### remove\_public\_key

Removes a public key from an account. Can also be done [via transactions](/integration/verifier-contract/account-abstraction).

| Parameter    | Description                                |
| ------------ | ------------------------------------------ |
| `public_key` | The public key to remove from the account. |

```json theme={null}
{
  "intent": "remove_public_key",
  "public_key": "ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm"
}
```

<Accordion title="Full signed intent example">
  The `message` field contains the intent JSON above, serialized as a single line with escaped quotes.

  ```json theme={null}
  [
    {
      "standard": "nep413",
      "payload": {
        "recipient": "intents.near",
        "nonce": "Vij2xgAlKBKzAAxeFUp9QRi4FmMX9Jj6kmrjkUTtltk=",
        "message": "{\"deadline\":\"2025-05-21T08:24:47.536976Z\",\"intents\":[{\"intent\":\"remove_public_key\",\"public_key\":\"ed25519:5TagutioHgKLh7KZ1VEFBYfgRkPtqnKm9LoMnJMJugxm\"}],\"signer_id\":\"alice.near\"}"
      },
      "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
      "signature": "ed25519:3E11skwith3ub3w22FnUdcqYzDFy698qBnP8FEPjZyZKbFtqUZK17AXGSLiwnHXppGrV8wbbGafX8fqXUvefoE8p"
    }
  ]
  ```
</Accordion>

### transfer

Transfers tokens from the signer to a specified account within the Verifier contract.

Transfers can also be done [via direct blockchain transactions](https://near.github.io/intents/defuse_nep245/trait.MultiTokenCore.html#tymethod.mt_transfer).

| Parameter     | Description                                                                                                            |
| ------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `receiver_id` | The account to receive the tokens.                                                                                     |
| `tokens`      | Map of token IDs to amounts in [Multi Token format](/integration/verifier-contract/deposits-and-withdrawals/balances). |
| `memo`        | Optional memo for the transfer.                                                                                        |

```json theme={null}
{
  "intent": "transfer",
  "receiver_id": "bob.near",
  "tokens": {
    "nep141:usdc.near": "10"
  }
}
```

<Accordion title="Full signed intent example">
  The `message` field contains the intent JSON above, serialized as a single line with escaped quotes.

  ```json theme={null}
  [
    {
      "standard": "nep413",
      "payload": {
        "recipient": "intents.near",
        "nonce": "Vij2xgAlKBKzgNPJFViEQRhm1GXQz/Vt+TS0PazCsJQ=",
        "message": "{\"deadline\":\"2025-05-21T10:34:04.254392Z\",\"intents\":[{\"intent\":\"transfer\",\"receiver_id\":\"bob.near\",\"tokens\":{\"nep141:usdc.near\":\"10\"}}],\"signer_id\":\"alice.near\"}"
      },
      "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
      "signature": "ed25519:4nY7kjYV11djwsZ9UmezX1eoVMnzTg2wN6jT67o5vyWnibQ7g34zti8wc9imafbAzH5v4rqmksiextQCas14uxm5"
    }
  ]
  ```
</Accordion>

### Withdrawal intents

These intents move tokens from the Verifier contract to an arbitrary address:

* [FtWithdraw](https://near.github.io/intents/defuse_core/intents/tokens/struct.FtWithdraw.html) - Fungible tokens
* [NftWithdraw](https://near.github.io/intents/defuse_core/intents/tokens/struct.NftWithdraw.html) - Non-fungible tokens
* [MtWithdraw](https://near.github.io/intents/defuse_core/intents/tokens/struct.MtWithdraw.html) - Multi tokens
* [NativeWithdraw](https://near.github.io/intents/defuse_core/intents/tokens/struct.NativeWithdraw.html) - Native NEAR

See [Withdrawals](/integration/verifier-contract/deposits-and-withdrawals/withdrawals) for details.

| Parameter         | Description                                                                                                                                                                                            |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `token`           | The token contract to withdraw (e.g., `usdc.near`). No prefix for this field.                                                                                                                          |
| `receiver_id`     | The NEAR account that will receive the withdrawn tokens.                                                                                                                                               |
| `amount`          | The amount in the token's smallest unit.                                                                                                                                                               |
| `memo`            | Optional memo for the withdrawal.                                                                                                                                                                      |
| `msg`             | Optional message to pass to `ft_transfer_call`. If omitted, `ft_transfer` is used instead. See [Withdrawals](/integration/verifier-contract/deposits-and-withdrawals/withdrawals) for refund behavior. |
| `storage_deposit` | Optional wNEAR amount to pay for storage deposit on the token contract for the receiver. Will not be refunded on failure.                                                                              |

**Example:** Withdraw from Alice's account to Bob's account. On success, the tokens will be in the `usdc.near` contract under Bob's account—they have exited the Verifier:

```json theme={null}
{
  "intent": "ft_withdraw",
  "token": "usdc.near",
  "receiver_id": "bob.near",
  "amount": "1000"
}
```

<Accordion title="Full signed intent example">
  The `message` field contains the intent JSON above, serialized as a single line with escaped quotes.

  ```json theme={null}
  [
    {
      "standard": "nep413",
      "payload": {
        "recipient": "intents.near",
        "nonce": "Vij2xgAlKBKzgNPJFViEQRiRYtwetaXk8zFqV//Lq3c=",
        "message": "{\"deadline\":\"2025-05-21T10:45:30.098925Z\",\"intents\":[{\"intent\":\"ft_withdraw\",\"token\":\"usdc.near\",\"receiver_id\":\"bob.near\",\"amount\":\"1000\"}],\"signer_id\":\"alice.near\"}"
      },
      "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
      "signature": "ed25519:XYNGWVRyGFGRVZAAn62k9r8qthzbj4a5Ct9eCjrSUW9hXpPRaBvqpLKXpaeBgsekfLFiTLdXMEitrbsZAmMmdmU"
    }
  ]
  ```
</Accordion>

### storage\_deposit

Makes an [NEP-145](https://nomicon.io/Standards/StorageManagement#nep-145) `storage_deposit` call for an `account_id` on a `contract_id`. The `amount` is subtracted from the user's NEP-141 wNEAR balance and will not be refunded.

| Parameter     | Description                                                                                         |
| ------------- | --------------------------------------------------------------------------------------------------- |
| `contract_id` | The contract to make the storage deposit on (e.g., `usdc.near`).                                    |
| `account_id`  | The account to deposit storage for.                                                                 |
| `amount`      | The wNEAR amount in yoctoNEAR. Subtracted from the signer's wNEAR balance and will not be refunded. |

**Example:** Pay for storage deposit in the `usdc.near` contract. The NEAR token specified will be taken from `alice.near`'s account and paid to `bob.near` in the `usdc.near` contract:

```json theme={null}
{
  "intent": "storage_deposit",
  "contract_id": "usdc.near",
  "account_id": "bob.near",
  "amount": "1250000000000000000000"
}
```

<Accordion title="Full signed intent example">
  The `message` field contains the intent JSON above, serialized as a single line with escaped quotes.

  ```json theme={null}
  [
    {
      "standard": "nep413",
      "payload": {
        "recipient": "intents.near",
        "nonce": "Vij2xgAlKBKzgNPJFViEQRhA/qs878LtXEL9iIjJC14=",
        "message": "{\"deadline\":\"2025-05-21T11:06:28.803408Z\",\"intents\":[{\"intent\":\"storage_deposit\",\"contract_id\":\"usdc.near\",\"account_id\":\"bob.near\",\"amount\":\"1250000000000000000000\"}],\"signer_id\":\"alice.near\"}"
      },
      "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
      "signature": "ed25519:4rAYXBa9UY6Zw32dKcGrXgydParzzdygEwdAaaKRg9TxXSP9541rikLWhVCtJqFHJh4Rcvpp2YrbRQ4LBaay7Xa6"
    }
  ]
  ```
</Accordion>

### token\_diff

The user declares willingness to have a set of changes applied to their tokens. For example, a trade of 100 token A for 200 token B can be represented as `{"A": -100, "B": 200}`. The Verifier resolves matching diffs into transfers between parties.

<Warning>
  When `token_diff` intents are submitted together in a batch, the total diffs across all intents must sum to zero for each token. If the amounts don't balance, the entire batch will fail. For example, if Alice gives 10 USDC, another intent in the batch must receive exactly 10 USDC.
</Warning>

| Parameter  | Description                                                                                                                 |
| ---------- | --------------------------------------------------------------------------------------------------------------------------- |
| `diff`     | Map of token IDs to signed amount strings. Negative values indicate tokens given, positive values indicate tokens received. |
| `memo`     | Optional memo for the trade.                                                                                                |
| `referral` | Optional account ID for referral tracking.                                                                                  |

**Example:** Two users trading USDC for USDT. Alice declares she'll give up 10 USDC to get 10 USDT; Bob declares the opposite. These intents can be matched through the [Message Bus](/integration/market-makers/message-bus/introduction) or any off-chain channel, then submitted together:

<Tabs>
  <Tab title="Alice's intent">
    ```json theme={null}
    {
      "intent": "token_diff",
      "diff": {
        "nep141:usdc.near": "-10",
        "nep141:usdt.near": "10"
      }
    }
    ```
  </Tab>

  <Tab title="Bob's intent">
    ```json theme={null}
    {
      "intent": "token_diff",
      "diff": {
        "nep141:usdc.near": "10",
        "nep141:usdt.near": "-10"
      }
    }
    ```
  </Tab>
</Tabs>

<Accordion title="Full signed intent example">
  The `message` field contains the intent JSON above, serialized as a single line with escaped quotes.

  ```json theme={null}
  [
    {
      "standard": "nep413",
      "payload": {
        "recipient": "intents.near",
        "nonce": "Vij2xgAlKBKzgMh0PGuHQRiOD4cMhPLCgZfb8bCmR7s=",
        "message": "{\"deadline\":\"2025-05-21T11:30:25.042157Z\",\"intents\":[{\"intent\":\"token_diff\",\"diff\":{\"nep141:usdc.near\":\"-10\",\"nep141:usdt.near\":\"10\"}}],\"signer_id\":\"alice.near\"}"
      },
      "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
      "signature": "ed25519:2tKuCpxqtx6YUY6LRkQGeqmAizA8CKhkFtYdsHsAGRjwm4tFcPHHhBRGtuQAAbAkUkwJ6n2UptTP4Mpot1cTvf2u"
    },
    {
      "standard": "nep413",
      "payload": {
        "recipient": "intents.near",
        "nonce": "Vij2xgAlKBKzgMh0PGuHQRhE5M+mHoAocV57AzeIPuQ=",
        "message": "{\"deadline\":\"2025-05-21T11:30:25.054132Z\",\"intents\":[{\"intent\":\"token_diff\",\"diff\":{\"nep141:usdc.near\":\"10\",\"nep141:usdt.near\":\"-10\"}}],\"signer_id\":\"bob.near\"}"
      },
      "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
      "signature": "ed25519:gTrJn3CaESvDJ765SDZ7JdkXupeGApjU6t66XpwrCXKM5K7b4VEeZXxL6NSL7i5zJ2Dn1aracg9xubktcPLwHEq"
    }
  ]
  ```
</Accordion>

As described in the [Introduction](/integration/verifier-contract/introduction), there are many ways to bundle these intents together for submission—the [Message Bus](/integration/market-makers/message-bus/introduction), third parties, or any off-chain communication channel.

## Next steps

<CardGroup cols={2}>
  <Card title="Signing Intents" icon="signature" href="/integration/verifier-contract/signing-intents">
    Learn how to sign intents for different wallet types
  </Card>

  <Card title="Simulating Intents" icon="flask" href="/integration/verifier-contract/simulating-intents">
    Test intents before submitting them on-chain
  </Card>

  <Card title="Deposits & Withdrawals" icon="arrow-right-arrow-left" href="/integration/verifier-contract/deposits-and-withdrawals/deposits">
    Deposit and withdraw tokens from the Verifier
  </Card>

  <Card title="Events" icon="bell" href="/integration/verifier-contract/events">
    Monitor intent execution via on-chain events
  </Card>
</CardGroup>

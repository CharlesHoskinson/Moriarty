> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Account Abstraction

> How the Verifier contract identifies users and manages account keys

Users do not need to create a NEAR account to use NEAR Intents. The Verifier contract supports wallets across [all supported chains](/resources/chain-support). When a user signs with their existing wallet, the Verifier derives an account from their public key. The user retains sole control via their existing private keys.

Under the hood, the Verifier identifies users via a NEAR `AccountId`, which can be either a Named account (like `alice.near`) or an Implicit account derived from a public key. The Verifier maintains its own mapping of account IDs to public keys, allowing it to verify signed intents from any supported wallet.

### NEAR's multi-key model

On NEAR, each account can hold [multiple access keys](https://docs.near.org/protocol/access-keys). There are two types:

* **Full Access Keys** — grant complete account control (transfer tokens, manage keys, deploy contracts)
* **Function Call Keys** — restricted to calling specific contracts, safe to share with applications

Keys can be added or removed at any time, giving accounts flexible, multi-key security. The Verifier contract builds on this model — when you register a public key with the Verifier, it associates that key with your account ID so it can verify your signed intents.

## Account types

### Named accounts

Named accounts are human-readable identifiers like `alice.near`.

To start using a named account with the Verifier, you must register a public key by calling `add_public_key` on `intents.near` from that named NEAR account. This tells the Verifier which keys are authorized to sign intents on behalf of your account.

Others can still deposit or transfer funds to your named account before you register a key - you just won't be able to sign intents until you do.

### Implicit accounts

Users from other chains don't need to create a NEAR account. When they sign with their existing wallet, the Verifier automatically derives an implicit account ID from their public key. No registration, no new wallet — they can start using NEAR Intents immediately.

There is a 1-to-1 relationship between the public key's signing curve and the resulting account format:

| Curve | Account Format                        | Example                                                            |
| ----- | ------------------------------------- | ------------------------------------------------------------------ |
| EdDSA | 64-character hex (Implicit NEAR)      | `8c5cba35f5b4db9579c39175ad34a9275758eb29d4866f395ed1a5b5afcb9ffc` |
| ECDSA | Ethereum-style address (Implicit Eth) | `0x85d456B2DfF1fd8245387C0BfB64Dfb700e98Ef3`                       |

For example, users logging in with a Cosmos (ECDSA) wallet will have an *Implicit Eth address* in `intents.near`, whereas Solana or TON (EdDSA) wallets will yield *Implicit NEAR addresses*.

<Info>
  It's not feasible to differentiate these addresses by chain, since only the signature and public key are known. Even when differentiating based on the signing standard (NEP-413, EIP-712), ambiguity remains when importing the same seed phrase into multiple wallets.
</Info>

## Account keys

Once an account is created, you can add multiple public keys to authorize actions on that account. Each key has full control and can add or remove other keys—either directly via NEAR transactions or via signed intents.

<Info>
  Public keys and signatures must use specific encoding formats depending on the curve type. See [Signing Intents](/integration/verifier-contract/signing-intents) for the full encoding requirements table.
</Info>

### Adding a public key via transaction

Here is an [example transaction](https://nearblocks.io/txns/FBTRk6jRUSW3E1SjBfYbA71DhN5xTX1yE2foy98TafrM#execution) for adding a public key to a Named Account.

<Tabs>
  <Tab title="NEAR CLI">
    ```bash theme={null}
    near contract call-function as-transaction \
      intents.near add_public_key json-args '{
        "public_key": "ed25519:<base58>"
      }' prepaid-gas '100.0 Tgas' attached-deposit '1 yoctoNEAR' \
      sign-as <ACCOUNT_ID> network-config mainnet sign-with-keychain send
    ```
  </Tab>

  <Tab title="NEAR API JS">
    ```typescript theme={null}
    import { Account, JsonRpcProvider, teraToGas, KeyPairString } from "near-api-js";

    const accountId = "your-account.near";
    const privateKey = "ed25519:3D4YudU..." as KeyPairString;

    const provider = new JsonRpcProvider({ url: "https://rpc.fastnear.com" });
    const account = new Account(accountId, provider, privateKey);

    await account.callFunction({
      contractId: "intents.near",
      methodName: "add_public_key",
      args: {
        public_key: "ed25519:<base58>",
      },
      gas: teraToGas("100"),
      deposit: 1n, // 1 yoctoNEAR
    });
    ```

    See contract interaction example in [near-api-examples](https://github.com/near-examples/near-api-examples/blob/main/near-api-js/examples/contract-interaction.ts) repository.
  </Tab>
</Tabs>

### Adding a public key via signed intent

You can also submit a signed intent to add a public key:

```json theme={null}
{
  "signer_id": "<ACCOUNT_ID>",
  "intents": [
    {
      "intent": "add_public_key",
      "public_key": "<PUBLIC_KEY_OF_USER>"
    }
  ]
}
```

## Authentication flow for frontends

Wallets store private keys and allow key rotation. For the Verifier to verify signatures, it needs to know which keys are associated with which "named" accounts. Therefore, `intents.near` maintains a mapping of `account_ids` to their `public_keys` (each account can have multiple public keys registered).

### Step-by-step flow

<Steps>
  <Step title="Connect wallet">
    Prompt your user to connect their wallet, establishing a session and providing their account ID or address.
  </Step>

  <Step title="Check key registration">
    For Named accounts only: check whether the public key is registered with the Verifier contract. If not, register it by calling `add_public_key` on `intents.near`. Implicit accounts skip this step — their account ID is derived directly from their public key.
  </Step>

  <Step title="Sign and submit intents">
    The wallet signs the intent payload. The signed intent is published to the Solver Relay, which coordinates with market makers to settle on-chain via `execute_intents`.
  </Step>
</Steps>

### Key rotation

When a user removes a Full Access Key from their NEAR account, it should also be unregistered on `intents.near` by calling `remove_public_key(public_key)` from that NEAR account.

You can automate this by adding a `FunctionalKey` to the account on NEAR and calling it whenever you detect that a key has been deleted on-chain.

## Real transaction examples

View these key management transactions on NEAR mainnet:

| Operation         | Transaction                                                                                      |
| ----------------- | ------------------------------------------------------------------------------------------------ |
| Add public key    | [FBTRk6jR...](https://nearblocks.io/txns/FBTRk6jRUSW3E1SjBfYbA71DhN5xTX1yE2foy98TafrM#execution) |
| Remove public key | [69XiJcRt...](https://nearblocks.io/txns/69XiJcRtipZ2i8bkHHgCaSAkswd8Lt6VxiJuM8dKq9qn#execution) |

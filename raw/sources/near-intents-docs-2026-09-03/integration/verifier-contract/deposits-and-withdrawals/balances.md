> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Balances and Token IDs

> How to check your balance and identify tokens in the Verifier contract

## Multi Token Standard (NEP-245)

After a successful deposit, the Verifier contract manages tokens using the [Multi Token Standard (NEP-245)](https://nomicon.io/Standards/Tokens/MultiToken/Core). This enables uniform handling of all supported token types.

## Token ID format

Each token is identified by a string-based token ID, prefixed with its standard type.

| Token Type             | Prefix    | Example                          |
| ---------------------- | --------- | -------------------------------- |
| Fungible (NEP-141)     | `nep141:` | `nep141:wrap.near`               |
| Non-fungible (NEP-171) | `nep171:` | `nep171:coolnfts.near:rock.near` |
| Multi Token (NEP-245)  | `nep245:` | `nep245:mygame.near:shield.near` |

## Checking your balance

After a successful deposit, query your balance using [mt\_balance\_of](https://near.github.io/intents/defuse_nep245/trait.MultiTokenCore.html#tymethod.mt_balance_of), which adheres to the NEP-245 standard.

<Tabs>
  <Tab title="NEAR CLI">
    ```bash theme={null}
    near contract call-function as-read-only intents.near mt_balance_of \
      json-args '{"account_id": "your-account.near", "token_id": "nep141:wrap.near"}' \
      network-config mainnet now
    ```
  </Tab>

  <Tab title="NEAR API JS">
    ```typescript theme={null}
    import { JsonRpcProvider } from "near-api-js";

    const provider = new JsonRpcProvider({ url: "https://rpc.fastnear.com" });

    const balance = await provider.callFunction({
      contractId: "intents.near",
      method: "mt_balance_of",
      args: {
        account_id: "your-account.near",
        token_id: "nep141:wrap.near",
      },
    });

    console.log(balance);
    ```

    See contract interaction example in [near-api-examples](https://github.com/near-examples/near-api-examples/blob/main/near-api-js/examples/contract-interaction.ts) repository.
  </Tab>
</Tabs>

| Parameter    | Description                                                                                                |
| ------------ | ---------------------------------------------------------------------------------------------------------- |
| `account_id` | The NEAR account to check the balance of.                                                                  |
| `token_id`   | The token ID with its standard prefix (e.g., `nep141:wrap.near`). See [Token ID format](#token-id-format). |

### Batch balance query

To check multiple token balances at once, use [mt\_batch\_balance\_of](https://near.github.io/intents/defuse_nep245/trait.MultiTokenCore.html#tymethod.mt_batch_balance_of):

<Tabs>
  <Tab title="NEAR CLI">
    ```bash theme={null}
    near contract call-function as-read-only intents.near mt_batch_balance_of \
      json-args '{"account_id": "your-account.near", "token_ids": ["nep141:wrap.near", "nep141:usdt.tether-token.near"]}' \
      network-config mainnet now
    ```
  </Tab>

  <Tab title="NEAR API JS">
    ```typescript theme={null}
    const balances = await provider.callFunction({
      contractId: "intents.near",
      method: "mt_batch_balance_of",
      args: {
        account_id: "your-account.near",
        token_ids: [
          "nep141:wrap.near",
          "nep141:usdt.tether-token.near",
        ],
      },
    });

    console.log(balances);
    ```
  </Tab>
</Tabs>

Response:

```json theme={null}
[
  "3000004000004000006",
  "10"
]
```

<Info>
  Balances are returned in the smallest unit of each token (e.g., yoctoNEAR for wrapped NEAR).
</Info>

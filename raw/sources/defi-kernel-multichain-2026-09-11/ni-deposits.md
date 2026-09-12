> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Deposits

> Depositing fungible and non-fungible tokens into the Verifier contract

Before you can swap or transfer tokens on the Verifier contract, you need to deposit them. Depositing moves tokens from your NEAR account into the contract's internal ledger, where they can be used for [intents](/integration/verifier-contract/intent-types-and-execution).

The contract accepts:

* [NEP-141](https://nomicon.io/Standards/Tokens/FungibleToken/Core) fungible tokens
* [NEP-171](https://nomicon.io/Standards/Tokens/NonFungibleToken/Core) non-fungible tokens
* [NEP-245](https://nomicon.io/Standards/Tokens/MultiToken/Core) multi tokens

<Danger>
  Do not send native NEAR directly to the Verifier contract.

  You must wrap your NEAR into wNEAR first as the contract does not accept native NEAR. See ["Using NEAR Tokens"](/integration/verifier-contract/deposits-and-withdrawals/near-token) for more details.
</Danger>

## Depositing fungible tokens (NEP-141)

The Verifier contract implements the [FungibleTokenReceiver](https://docs.near.org/primitives/ft) interface, part of the [NEP-141 standard](https://nomicon.io/Standards/Tokens/FungibleToken/Core).

To deposit tokens, call `ft_transfer_call` ([function signature](https://github.com/near/near-sdk-rs/blob/611e01ebf6c226f4e1e820a2f50f4a9acf8f1215/examples/fungible-token/ft/src/lib.rs#L103)) on the token contract, with an `msg` parameter specifying who will own the tokens.

Any NEP-141 token can be deposited. For a list of NEP-141 token contract addresses deposited into the Verifier contract, run:

```bash theme={null}
curl -s https://1click.chaindefuser.com/v0/tokens | jq -r '.[] | select(.blockchain == "near") | [.symbol, (.price | tostring), .contractAddress] | @tsv' | column -t -s $'\t'
```

### How to deposit tokens

Replace `<token-contract>` with the token's contract address and adjust the `amount` for the token's decimal precision.

<Tabs>
  <Tab title="NEAR CLI">
    ```bash theme={null}
    near call <token-contract> ft_transfer_call \
      '{"receiver_id": "intents.near", "amount": "<amount>", "msg": ""}' \
      --deposit 0.000000000000000000000001 \
      --gas 100000000000000 \
      --useAccount your-account.near \
      --networkId mainnet
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
      contractId: "<token-contract>",
      methodName: "ft_transfer_call",
      args: {
        receiver_id: "intents.near",
        amount: "<amount>",
        msg: "",
      },
      gas: teraToGas("100"),
      deposit: 1n, // 1 yoctoNEAR
    });
    ```

    See contract interaction example in [near-api-examples](https://github.com/near-examples/near-api-examples/blob/main/near-api-js/examples/contract-interaction.ts) repository.
  </Tab>
</Tabs>

| Parameter          | Description                                                                                                                                                                                                                     |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `<token-contract>` | The NEP-141 token contract you're depositing from (e.g., `wrap.near` for wNEAR, `eth.bridge.near` for ETH). Any NEP-141 token can be deposited.                                                                                 |
| `receiver_id`      | Always `intents.near` (the Verifier contract).                                                                                                                                                                                  |
| `amount`           | The amount in the token's smallest unit. Each token has different decimals (e.g., wNEAR has 24, USDC has 6).                                                                                                                    |
| `msg`              | Controls token ownership after deposit. See [The `msg` parameter](#the-msg-parameter) below.                                                                                                                                    |
| `--deposit`        | 1 yoctoNEAR, required by the NEP-141 standard for `ft_transfer_call`.                                                                                                                                                           |
| `--gas`            | Depends on the `msg` content. Should be calculated to cover the cross-contract call to the Verifier contract plus any promises triggered by the deposit (e.g., `execute_intents`). 100 TGas is sufficient for a simple deposit. |

### The `msg` parameter

The `msg` parameter supports three formats:

<Tabs>
  <Tab title="Empty (default)">
    Leave `msg` empty or omit it to assign ownership to the transaction sender.

    ```json theme={null}
    {
      "receiver_id": "intents.near",
      "amount": "1000",
      "msg": ""
    }
    ```
  </Tab>

  <Tab title="Account ID string">
    Specify an account ID to assign ownership to a different account.

    ```json theme={null}
    {
      "receiver_id": "intents.near",
      "amount": "1000",
      "msg": "bob.near"
    }
    ```
  </Tab>

  <Tab title="DepositMessage object">
    Use a JSON object for advanced options:

    | Field             | Description                                                                                          |
    | ----------------- | ---------------------------------------------------------------------------------------------------- |
    | `receiver_id`     | Account ID taking ownership                                                                          |
    | `execute_intents` | List of intents to execute after deposit                                                             |
    | `refund_if_fails` | When `false` (default), executes intents as a detached promise, decoupling failures from the deposit |

    ```json theme={null}
    {
      "receiver_id": "intents.near",
      "amount": "1000",
      "msg": "{\"receiver_id\": \"charlie.near\", \"execute_intents\": [...], \"refund_if_fails\": false}"
    }
    ```
  </Tab>
</Tabs>

<Warning>
  The `msg` field is always a string, even when it contains a JSON-encoded object. Proper character escaping is required.
</Warning>

## Real transaction examples

View these deposit transactions on NEAR mainnet:

| Operation                 | Transaction                                                                                     |
| ------------------------- | ----------------------------------------------------------------------------------------------- |
| Deposit tokens            | [DWv4AkrL...](https://nearblocks.io/txns/DWv4AkrLxnbJV6paqSFR3Tt42SdVAtunBbEi2XATdXTW#enhanced) |
| Deposit + execute intents | [Bn3iC9B1...](https://nearblocks.io/txns/Bn3iC9B1uUJrX59x7cfxngY1E159HspzTCyyDVwx5DMJ)          |
| Deposit + swap + withdraw | [BMFcWFRe...](https://nearblocks.io/txns/BMFcWFReAzbH8okweUio2nNTuVXtMr1hXeaaNR4UhEzS)          |

## Depositing non-fungible tokens (NEP-171)

The Verifier contract implements the [NonFungibleTokenReceiver](https://docs.near.org/primitives/nft) interface ([NEP-171](https://nomicon.io/Standards/Tokens/NonFungibleToken/Core) standard).

To transfer NFTs to the Verifier, use `nft_transfer_call` with the same `msg` format rules as fungible tokens.

## Depositing multi tokens (NEP-245)

The Verifier contract implements the [MultiTokenReceiver](https://nomicon.io/Standards/Tokens/MultiToken/Core) interface ([NEP-245](https://nomicon.io/Standards/Tokens/MultiToken/Core) standard).

To deposit multi tokens, use `mt_batch_transfer_call` with the same `msg` format rules as fungible tokens.

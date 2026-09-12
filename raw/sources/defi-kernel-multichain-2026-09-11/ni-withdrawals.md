> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Withdrawals

> How to withdraw tokens from the Verifier contract

After a successful [deposit](/integration/verifier-contract/deposits-and-withdrawals/deposits), tokens are assigned to your account in the Verifier contract. You retain full ownership and control, and can withdraw at any time.

## Withdrawal methods

There are two ways to withdraw tokens:

1. **Direct function call** on the Verifier contract
2. **Signed intent** submitted via `execute_intents`

<Info>
  When using direct function calls, the token ID is used **without** a prefix (e.g., `wrap.near`).

  When using signed intents, the token ID is prefixed (e.g., `nep141:wrap.near`).
</Info>

## Withdrawing fungible tokens (NEP-141)

The Verifier contract exposes [ft\_withdraw](https://near.github.io/intents/defuse/tokens/nep141/trait.FungibleTokenWithdrawer.html#tymethod.ft_withdraw) for direct withdrawals and [FtWithdraw](https://near.github.io/intents/defuse_core/intents/tokens/struct.FtWithdraw.html) for intent-based withdrawals.

### Via direct function call

Replace `<token-contract>` with the token's contract address and adjust the `amount` for the token's decimal precision.

<Tabs>
  <Tab title="NEAR CLI">
    ```bash theme={null}
    near call intents.near ft_withdraw \
      '{"token": "<token-contract>", "receiver_id": "your-account.near", "amount": "<amount>"}' \
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
      contractId: "intents.near",
      methodName: "ft_withdraw",
      args: {
        token: "<token-contract>",
        receiver_id: "your-account.near",
        amount: "<amount>",
      },
      gas: teraToGas("100"),
      deposit: 1n, // 1 yoctoNEAR
    });
    ```

    See contract interaction example in [near-api-examples](https://github.com/near-examples/near-api-examples/blob/main/near-api-js/examples/contract-interaction.ts) repository.
  </Tab>
</Tabs>

| Parameter     | Description                                                                                                  |
| ------------- | ------------------------------------------------------------------------------------------------------------ |
| `token`       | The token contract to withdraw (e.g., `wrap.near` for wNEAR). No prefix for direct calls.                    |
| `receiver_id` | The NEAR account that will receive the withdrawn tokens.                                                     |
| `amount`      | The amount in the token's smallest unit. Each token has different decimals (e.g., wNEAR has 24, USDC has 6). |
| `--deposit`   | 1 yoctoNEAR, required for the withdrawal call.                                                               |
| `--gas`       | 100 TGas to cover the cross-contract call.                                                                   |

### Via signed intent

Submit a signed [FtWithdraw](https://near.github.io/intents/defuse_core/intents/tokens/struct.FtWithdraw.html) intent through `execute_intents`. Withdraw intents (`ft_withdraw`, `nft_withdraw`, `mt_withdraw`) use unprefixed token IDs (e.g., `usdc.near`). Token prefixes like `nep141:` are only used in `transfer` and `token_diff` intents.

<Warning>
  Your public key must be registered with the Verifier contract before you can submit signed intents. For named accounts, call `add_public_key` on `intents.near`. Implicit accounts skip this step. See [Account Abstraction](/integration/verifier-contract/account-abstraction) for details.
</Warning>

```json theme={null}
{
  "standard": "nep413",
  "payload": {
    "recipient": "intents.near",
    "nonce": "Vij2xgAlKBKzgNPJFViEQRhPvveC+P9xRwOXFW/IK5w=",
    "message": "{\"deadline\":\"2025-05-20T13:29:34.360380Z\",\"intents\":[{\"intent\":\"ft_withdraw\",\"token\":\"wrap.near\",\"receiver_id\":\"alice.near\",\"amount\":\"1000\"}],\"signer_id\":\"user1.test.near\"}"
  },
  "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
  "signature": "ed25519:5op5APf9YaRznvMVwZBw6GGk9ychU3QtNRKJSxuVajHFMQths4ZZMRoAENU1zWhjinqz8KuLNyM9589c86opopyT"
}
```

See [Intent Types and Execution](/integration/verifier-contract/intent-types-and-execution) for the full signed intent format and how to submit intents.

## Real transaction examples

View these withdrawal and intent transactions on NEAR mainnet:

| Operation                 | Transaction                                                                                     |
| ------------------------- | ----------------------------------------------------------------------------------------------- |
| Withdraw by transaction   | [3E8TDSLq...](https://nearblocks.io/txns/3E8TDSLq2Xn8JZEAdbeX8NGS7Ps6f1EwLeoFxhT7YY3G#enhanced) |
| Deposit + swap + withdraw | [BMFcWFRe...](https://nearblocks.io/txns/BMFcWFReAzbH8okweUio2nNTuVXtMr1hXeaaNR4UhEzS)          |
| Execute intents           | [FxpbspXj...](https://nearblocks.io/txns/FxpbspXjRQg3gDii18ibp3w7yvbe7MWaDHQfVqVfq7xN#enhanced) |

## Withdrawal functions by token type

| Token Type            | Direct Function                                                                                                                  | Intent                                                                                                 |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| NEP-141 (Fungible)    | [ft\_withdraw](https://near.github.io/intents/defuse/tokens/nep141/trait.FungibleTokenWithdrawer.html#tymethod.ft_withdraw)      | [FtWithdraw](https://near.github.io/intents/defuse_core/intents/tokens/struct.FtWithdraw.html)         |
| NEP-171 (NFT)         | [nft\_withdraw](https://near.github.io/intents/defuse/tokens/nep171/trait.NonFungibleTokenWithdrawer.html#tymethod.nft_withdraw) | [NftWithdraw](https://near.github.io/intents/defuse_core/intents/tokens/struct.NftWithdraw.html)       |
| NEP-245 (Multi Token) | [mt\_withdraw](https://near.github.io/intents/defuse/tokens/nep245/trait.MultiTokenWithdrawer.html#tymethod.mt_withdraw)         | [MtWithdraw](https://near.github.io/intents/defuse_core/intents/tokens/struct.MtWithdraw.html)         |
| Native NEAR           | N/A                                                                                                                              | [NativeWithdraw](https://near.github.io/intents/defuse_core/intents/tokens/struct.NativeWithdraw.html) |

## Withdrawing non-fungible tokens (NEP-171)

To withdraw NFTs from the Verifier contract, use [nft\_withdraw](https://near.github.io/intents/defuse/tokens/nep171/trait.NonFungibleTokenWithdrawer.html#tymethod.nft_withdraw) for direct calls or [NftWithdraw](https://near.github.io/intents/defuse_core/intents/tokens/struct.NftWithdraw.html) for intent-based withdrawals, following the same patterns as fungible token withdrawals above.

## Refunds on failed withdrawals

<Warning>
  **Important:** This section applies to all withdrawal types except `NativeWithdraw`, since native NEAR transfers cannot fail.
</Warning>

A refund restores your balance when a withdrawal fails on the target smart contract. The behavior depends on the `msg` parameter in your withdrawal request.

### How `msg` affects refunds

The [withdraw function](https://near.github.io/intents/defuse/tokens/nep141/trait.FungibleTokenWithdrawer.html#tymethod.ft_withdraw) has a `msg` parameter from the [NEP-141 standard](https://nomicon.io/Standards/Tokens/FungibleToken/Core):

| `msg` Value                       | Transfer Method    | Refund on Failure? |
| --------------------------------- | ------------------ | :----------------: |
| Not specified or `null`           | `ft_transfer`      |          ✅         |
| Any string value (including `""`) | `ft_transfer_call` |          ❌         |

<Warning>
  An empty string (`""`) still counts as a specified `msg` value and will use `ft_transfer_call` with no refund. Only omitting `msg` entirely or setting it to `null` enables refund protection.
</Warning>

<Info>
  This same logic applies to Multi Token and NFT transfers with their corresponding functions: `nft_transfer`, `nft_transfer_call`, `mt_transfer`, and `mt_transfer_call`.
</Info>

### Why refunds don't always work

Due to the asynchronous and sharded nature of the NEAR blockchain, refunds can only be processed when `ft_transfer` is used (i.e., when `msg` is not specified or is `null`).

The refund issue occurs when an asynchronous call to another smart contract (e.g., a USDC contract) fails. If an error happens within the Verifier contract itself, your balance won't change.

### Summary

| Withdrawal Request                          | `msg` Parameter | Refund Possible? |
| ------------------------------------------- | :-------------: | :--------------: |
| `FtWithdraw` intent or `ft_withdraw` call   |  No (or `null`) |         ✅        |
| `FtWithdraw` intent or `ft_withdraw` call   |  Yes, as string |         ❌        |
| `MtWithdraw` intent or `mt_withdraw` call   |  No (or `null`) |         ✅        |
| `MtWithdraw` intent or `mt_withdraw` call   |  Yes, as string |         ❌        |
| `NftWithdraw` intent or `nft_withdraw` call |  No (or `null`) |         ✅        |
| `NftWithdraw` intent or `nft_withdraw` call |  Yes, as string |         ❌        |

<Tip>
  **Recommendation:** To ensure a successful withdrawal with refund protection, avoid specifying `msg` or set it explicitly to `null`. For programmable actions on the receiving contract, perform them from your own account using `ft_transfer_call`.
</Tip>

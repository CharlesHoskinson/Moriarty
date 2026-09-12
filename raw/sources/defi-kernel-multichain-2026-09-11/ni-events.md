> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Events

> Events emitted by the Verifier smart contract for tracking on-chain actions

The Verifier smart contract emits [events](https://github.com/near/NEPs/blob/master/neps/nep-0297.md) to log information about every intent execution and balance change. You can use these events to track on-chain actions, build indexers, and verify intent outcomes.

* For intent types that trigger these events, see [Intent Types and Execution](/integration/verifier-contract/intent-types-and-execution).
* To preview events before execution, see [Simulating Intents](/integration/verifier-contract/simulating-intents).

## Event structure

Based on [NEP-297](https://github.com/near/NEPs/blob/master/neps/nep-0297.md), NEAR events are prefixed with the string `EVENT_JSON`. The Verifier contract emits events using the DIP-4 standard, which wraps event data in a JSON object with `standard`, `version`, `event`, and `data` fields.

Here is a `token_diff` event emitted after a successful trade:

```json theme={null}
EVENT_JSON:{
  "standard": "dip4",
  "version": "0.3.0",
  "event": "token_diff",
  "data": [
    {
      "account_id": "charlie.near",
      "intent_hash": "5GpL6PsUQVHFYAk5FWEwBUaEQqcZkc2SjTvPYHgHAnx8",
      "diff": {
        "nep141:usdc.near": "-100",
        "nep141:usdt.near": "100"
      }
    }
  ]
}
```

| Field      | Description                                              |
| ---------- | -------------------------------------------------------- |
| `standard` | Always `"dip4"`.                                         |
| `version`  | DIP-4 version (currently `"0.3.0"`).                     |
| `event`    | Event name (e.g., `"token_diff"`, `"intents_executed"`). |
| `data`     | Array of event-specific payloads.                        |

## Intent events

These events are directly related to [submitted intents](/integration/verifier-contract/intent-types-and-execution). See the [full list of available events](https://near.github.io/intents/defuse_core/events/enum.DefuseEvent.html).

| Event                | Description                                               |
| -------------------- | --------------------------------------------------------- |
| `public_key_added`   | Emitted when a public key is added to an account.         |
| `public_key_removed` | Emitted when a public key is removed from an account.     |
| `transfer`           | Emitted when a transfer between two accounts executes.    |
| `token_diff`         | Emitted when a `token_diff` intent executes successfully. |
| `intents_executed`   | Emitted after successfully executing the listed intents.  |
| `ft_withdraw`        | Emitted when a fungible token withdrawal intent is made.  |
| `nft_withdraw`       | Emitted when an NFT withdrawal intent is made.            |
| `mt_withdraw`        | Emitted when a multi-token withdrawal intent is made.     |
| `native_withdraw`    | Emitted when a native NEAR withdrawal intent is made.     |
| `storage_deposit`    | Emitted when a storage deposit intent is submitted.       |

<Info>
  For withdrawal events, it is often difficult to emit only on success because the outcome depends on cross-contract calls.
</Info>

## Multi Token events

Multi Token events indicate changes in the internal balances of the Verifier contract. See the [available events](https://near.github.io/intents/defuse_nep245/enum.MtEvent.html).

| Event                                                                                     | Description                                                                            |
| ----------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| [`mt_mint`](https://near.github.io/intents/defuse_nep245/struct.MtMintEvent.html)         | Emitted when the balance of a given account increases (due to deposits).               |
| [`mt_burn`](https://near.github.io/intents/defuse_nep245/struct.MtBurnEvent.html)         | Emitted when the balance of a given account decreases (due to withdrawals).            |
| [`mt_transfer`](https://near.github.io/intents/defuse_nep245/struct.MtTransferEvent.html) | Emitted when tokens move between accounts (after executing intents like `token_diff`). |

### Why "mint" and "burn"?

You can think of the Verifier as a set of isolated balances that can increase, decrease, or move between accounts. Although balances are conserved and simply moved between accounts, from the Verifier contract's perspective:

* **Increasing** token amounts is equivalent to minting
* **Decreasing** token amounts is akin to burning

This design stems from how all tokens use the [Multi Token standard](/integration/verifier-contract/deposits-and-withdrawals/balances) for storage inside the Verifier smart contract.

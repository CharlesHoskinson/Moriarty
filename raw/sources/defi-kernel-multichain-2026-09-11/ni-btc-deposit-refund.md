> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Refund a stuck BTC deposit

> Recover Bitcoin you sent to a bridge deposit address that never finalized on NEAR

If you send BTC to a bridge deposit address but the deposit never finalizes on NEAR, the Bitcoin isn't lost. You can pull it back to a Bitcoin address you control using a manual refund flow on the BTC connector contract.

<Note>
  Most deposits finalize automatically — you only need this flow if yours didn't. **Confirm the deposit actually failed before you start** (see [Before you start](#before-you-start-confirm-the-deposit-never-finalized)). If you already received your bridged BTC, the deposit completed and no refund is needed.
</Note>

Use this flow only when **all** of the following are true:

* You sent BTC to a bridge deposit address.
* The deposit never completed on NEAR — you never received the bridged BTC.
* You want the BTC returned to a Bitcoin address you control.

<Warning>
  A deposit can only be refunded if it never finalized. The contract rejects the refund request if the deposit was already completed via `verify_deposit` or `safe_verify_deposit`.
</Warning>

## Before you start: confirm the deposit never finalized

A refund only works if your deposit never completed on NEAR. The contract has no single "is it finalized" view method, so check the *result* of finalization instead:

1. **Did you receive your bridged BTC?** Finalization mints nBTC to the recipient. If the recipient holds the nBTC, the deposit completed — you don't need a refund.
2. **The contract is the final word.** Step 1 below (`request_refund`) is rejected if the deposit already finalized via `verify_deposit` or `safe_verify_deposit` — so a successful step 1 confirms it never completed.

## How it works

The refund runs as three on-chain operations:

1. **Request the refund** — pin the original Bitcoin transaction to a refund address (`bridge-cli`).
2. **Execute the refund** — call the BTC connector contract after a timelock passes (`near-cli`).
3. **Sign the Bitcoin transaction** — trigger MPC signing so the relayer can broadcast the refund on Bitcoin (`bridge-cli`).

Steps 1 and 3 go through `bridge-cli`. Step 2 is a direct contract call through `near-cli`, because it only becomes callable after a timelock and can be invoked by anyone — not just the original depositor.

The contract that owns this flow is [`satoshi-bridge`](https://github.com/Near-One/btc-bridge), deployed on mainnet as `btc-connector.bridge.near`.

## Prerequisites

* **`bridge-cli`** — download a binary from the [releases page](https://github.com/Near-One/bridge-sdk-rs/releases/latest), or build it from source:

  ```bash theme={null}
  git clone https://github.com/near-one/bridge-sdk-rs.git
  cd bridge-sdk-rs
  cargo build --release
  # binary at ./target/release/bridge-cli
  ```

* **[`near-cli`](https://github.com/near/near-cli-rs)** — used for the `execute_refund` call in step 2.

* **The Bitcoin transaction hash** of your original deposit, and the output index (`vout`) of the deposit address within it.

* **A funded NEAR account** to sign the transactions and cover gas and storage.

Configure the NEAR signer for `bridge-cli` through environment variables (the preferred method) or a `.env` file:

```bash theme={null}
NEAR_SIGNER=<signer-account-id>
NEAR_PRIVATE_KEY=<signer-private-key>
```

## Refund the deposit

<Steps>
  <Step title="Submit the refund request">
    The minimal invocation is the chain and the BTC transaction hash. The CLI asks the bridge indexer which output of the transaction is a tracked deposit address, recovers the original `DepositMsg`, and uses its `refund_address` as the refund destination:

    ```bash theme={null}
    bridge-cli mainnet btc-request-refund \
        --chain btc \
        --btc-tx-hash <btc-tx-hash>
    ```

    **Optional arguments:**

    | Argument                                                          | When to use it                                                                                                                                                                                                       |
    | ----------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `--vout N`                                                        | Pick a specific output when the transaction has more than one tracked deposit address. The CLI tells you which vouts to choose from.                                                                                 |
    | `--refund-address X`                                              | Only used when the original `DepositMsg` has no `refund_address`. If the deposit message carries one, that address is used as the refund destination and this flag is ignored.                                       |
    | `--recipient-id`, `--fee`, `--msg`, `--no-deposit-refund-address` | Supply the original deposit args manually instead of relying on the indexer lookup. These must match the values used at deposit time — the contract recomputes the deposit address from them and rejects mismatches. |
    | `--dry-run`                                                       | Print the unsigned `request_refund` NEAR transaction (base64 borsh) for offline or hardware-wallet signing instead of submitting it. Requires `--near-public-key`.                                                   |

    Example with manual args (the `safe_deposit.msg` path, where `receiver_id` inside `--msg` is the intents account the deposit was routed to):

    ```bash theme={null}
    bridge-cli mainnet btc-request-refund \
        --chain btc \
        --btc-tx-hash <btc-tx-hash> \
        --recipient-id intents.near \
        --refund-address bc1q.... \
        --msg '{"receiver_id":"your_account.near"}'
    ```
  </Step>

  <Step title="Wait for the timelock, then execute the refund">
    After the refund request, call `execute_refund` directly on the BTC connector contract. Anyone can call it — the Bitcoin transaction is already pinned to your `refund_address` by step 1.

    The wait depends on whether a refund address was set on the original deposit:

    | Timelock | Condition                                                          |
    | -------- | ------------------------------------------------------------------ |
    | 2 days   | `refund_address` was provided in the original deposit              |
    | 14 days  | `refund_address` was **not** provided in the original deposit      |
    | Instant  | The caller holds the DAO or `RefundOperator` role on the connector |

    `utxo_storage_key` is `<btc_tx_hash>@<vout>` of the original deposit. Attach a deposit to cover storage for the `BTCPendingInfo` entry — the connector exposes the amount it expects through the `required_balance_for_execute_refund` view method (1 NEAR on mainnet at the time of writing):

    ```bash theme={null}
    near contract call-function as-read-only btc-connector.bridge.near \
        required_balance_for_execute_refund json-args '{}' \
        network-config mainnet now
    ```

    Pass that amount as the attached deposit:

    ```bash theme={null}
    near contract call-function as-transaction btc-connector.bridge.near \
        execute_refund \
        json-args '{"utxo_storage_key":"<btc-tx-hash>@0"}' \
        prepaid-gas '100.0 Tgas' \
        attached-deposit '1 NEAR' \
        sign-as your-account.near \
        network-config mainnet sign-with-keychain send
    ```

    <Note>
      Per the contract, this deposit covers storage for the refund and is **not** returned to you — treat it as a fee on the refund, not part of the recovered amount.
    </Note>
  </Step>

  <Step title="Trigger MPC signing of the refund transaction">
    `execute_refund` creates a `BTCPendingInfo` and emits a `GenerateBtcPendingInfo` event. Find `btc_pending_id` in the event logs of the `execute_refund` transaction (NEAR explorer or `near tx-status`) and pass it below. Once signed, the relayer broadcasts the Bitcoin transaction:

    ```bash theme={null}
    bridge-cli mainnet near-sign-btc-transaction \
        --chain btc \
        --btc-pending-id <btc_pending_id from execute_refund logs>
    ```
  </Step>
</Steps>

<Tip>
  If you run this flow and attach a sufficient fee, the relayer has a good chance of handling it for you starting from step 2.
</Tip>

## Notes

* If the original deposit's `DepositMsg.refund_address` was set, that address is the refund destination — `--refund-address` is ignored. The contract enforces that the refund transaction pays out to exactly that address.
* `request_refund` is rejected if the deposit was already finalized via `verify_deposit` or `safe_verify_deposit`.
* Replace all placeholder values (transaction hashes, addresses, account IDs) with your own before running any command.

## Next steps

<CardGroup cols={2}>
  <Card title="Token Bridges" icon="bridge" href="/integration/bridging/overview">
    See which bridges route assets between NEAR Intents and external chains
  </Card>

  <Card title="Withdrawals" icon="arrow-right-from-bracket" href="/integration/verifier-contract/deposits-and-withdrawals/withdrawals">
    Learn how withdrawals move assets out through bridges
  </Card>
</CardGroup>

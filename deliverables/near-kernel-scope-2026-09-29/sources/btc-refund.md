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

Bridges

Refund a stuck BTC deposit

[Swaps](/getting-started/what-are-intents)[Market Makers](/integration/market-makers/introduction)[Bridges](/integration/bridging/overview)[Verifier Contract](/integration/verifier-contract/introduction)[Learn](/learn/omni-bridge/overview)[Changelog](/changelog/overview)

Bridges

Refund a stuck BTC deposit
==========================

Copy page

Recover Bitcoin you sent to a bridge deposit address that never finalized on NEAR

Copy page

If you send BTC to a bridge deposit address but the deposit never finalizes on NEAR, the Bitcoin isn’t lost. You can pull it back to a Bitcoin address you control using a manual refund flow on the BTC connector contract.

Most deposits finalize automatically — you only need this flow if yours didn’t. **Confirm the deposit actually failed before you start** (see [Before you start](#before-you-start-confirm-the-deposit-never-finalized)). If you already received your bridged BTC, the deposit completed and no refund is needed.

Use this flow only when **all** of the following are true:

* You sent BTC to a bridge deposit address.
* The deposit never completed on NEAR — you never received the bridged BTC.
* You want the BTC returned to a Bitcoin address you control.

A deposit can only be refunded if it never finalized. The contract rejects the refund request if the deposit was already completed via `verify_deposit` or `safe_verify_deposit`.

Before you start: confirm the deposit never finalized
-----------------------------------------------------

A refund only works if your deposit never completed on NEAR. The contract has no single “is it finalized” view method, so check the *result* of finalization instead:

1. **Did you receive your bridged BTC?** Finalization mints nBTC to the recipient. If the recipient holds the nBTC, the deposit completed — you don’t need a refund.
2. **The contract is the final word.** Step 1 below (`request_refund`) is rejected if the deposit already finalized via `verify_deposit` or `safe_verify_deposit` — so a successful step 1 confirms it never completed.

How it works
------------

The refund runs as three on-chain operations:

1. **Request the refund** — pin the original Bitcoin transaction to a refund address (`bridge-cli`).
2. **Execute the refund** — call the BTC connector contract after a timelock passes (`near-cli`).
3. **Sign the Bitcoin transaction** — trigger MPC signing so the relayer can broadcast the refund on Bitcoin (`bridge-cli`).

Steps 1 and 3 go through `bridge-cli`. Step 2 is a direct contract call through `near-cli`, because it only becomes callable after a timelock and can be invoked by anyone — not just the original depositor.
The contract that owns this flow is [`satoshi-bridge`](https://github.com/Near-One/btc-bridge), deployed on mainnet as `btc-connector.bridge.near`.

Prerequisites
-------------

* **`bridge-cli`** — download a binary from the [releases page](https://github.com/Near-One/bridge-sdk-rs/releases/latest), or build it from source:
* **[`near-cli`](https://github.com/near/near-cli-rs)** — used for the `execute_refund` call in step 2.
* **The Bitcoin transaction hash** of your original deposit, and the output index (`vout`) of the deposit address within it.
* **A funded NEAR account** to sign the transactions and cover gas and storage.

Configure the NEAR signer for `bridge-cli` through environment variables (the preferred method) or a `.env` file:

Refund the deposit
------------------

1

Submit the refund request

The minimal invocation is the chain and the BTC transaction hash. The CLI asks the bridge indexer which output of the transaction is a tracked deposit address, recovers the original `DepositMsg`, and uses its `refund_address` as the refund destination:**Optional arguments:**Example with manual args (the `safe_deposit.msg` path, where `receiver_id` inside `--msg` is the intents account the deposit was routed to):

2

Wait for the timelock, then execute the refund

After the refund request, call `execute_refund` directly on the BTC connector contract. Anyone can call it — the Bitcoin transaction is already pinned to your `refund_address` by step 1.The wait depends on whether a refund address was set on the original deposit:`utxo_storage_key` is `<btc_tx_hash>@<vout>` of the original deposit. Attach a deposit to cover storage for the `BTCPendingInfo` entry — the connector exposes the amount it expects through the `required_balance_for_execute_refund` view method (1 NEAR on mainnet at the time of writing):Pass that amount as the attached deposit:

Per the contract, this deposit covers storage for the refund and is **not** returned to you — treat it as a fee on the refund, not part of the recovered amount.

3

Trigger MPC signing of the refund transaction

`execute_refund` creates a `BTCPendingInfo` and emits a `GenerateBtcPendingInfo` event. Find `btc_pending_id` in the event logs of the `execute_refund` transaction (NEAR explorer or `near tx-status`) and pass it below. Once signed, the relayer broadcasts the Bitcoin transaction:

If you run this flow and attach a sufficient fee, the relayer has a good chance of handling it for you starting from step 2.

Notes
-----

* If the original deposit’s `DepositMsg.refund_address` was set, that address is the refund destination — `--refund-address` is ignored. The contract enforces that the refund transaction pays out to exactly that address.
* `request_refund` is rejected if the deposit was already finalized via `verify_deposit` or `safe_verify_deposit`.
* Replace all placeholder values (transaction hashes, addresses, account IDs) with your own before running any command.

Next steps
----------

Token Bridges
-------------

See which bridges route assets between NEAR Intents and external chains

Withdrawals
-----------

Learn how withdrawals move assets out through bridges

Was this page helpful?

YesNo

[Previous

Overview

Bridges that route assets between NEAR Intents and external blockchains](/integration/bridging/overview)

⌘I

[x](https://x.com/near_intents)[github](https://github.com/defuse-protocol)[telegram](https://t.me/near_intents)

[Powered byThis documentation is built and hosted on Mintlify, a developer documentation platform](https://www.mintlify.com?utm_campaign=poweredBy&utm_medium=referral&utm_source=defuselabsltd)
> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# FAQs

> Frequently Asked Questions

## General

<AccordionGroup>
  <Accordion title="Is there a testnet deployment?">
    There is no testnet deployment and no plans for one. We recommend testing on NEAR mainnet using separate dev/test NEAR accounts.
  </Accordion>

  <Accordion title="What is Earn on the 1Click API?">
    Earn routes users into third-party yield protocols through normal 1Click quotes. Receipt tokens can be held in Intents or delivered to a destination-chain wallet. See [Earn](/integration/distribution-channels/1click-api/earn).
  </Accordion>

  <Accordion title="What are Limit Orders on the 1Click API?">
    Limit Orders rest a confidential swap at a price the user sets. See [Limit Orders](/integration/distribution-channels/1click-api/orders).
  </Accordion>
</AccordionGroup>

## Deposits

<AccordionGroup>
  <Accordion title="How does the deposit process work?">
    The deposit process begins once the transfer transaction on the foreign network has been completed. When the balance of the user's unique deposit address becomes positive, our indexer generates a deposit event and assigns it a `PENDING` status.

    The next step is collecting the current tokens in storage. The result of this process will be either a `COMPLETED` or `FAILED` status. Deposits with a `FAILED` status are currently handled manually and eventually updated to the `COMPLETED` status.

    On EVM networks, deposits can bypass the `PENDING` status due to faster processing and transfer completion times.

    **BTC deposits:** If you want to make a deposit to an account that hasn't yet been connected to the application, this is possible but requires caution. You can request a deposit address by calling the bridge API (`deposit_address`) and specifying the `account_id` parameter. The `account_id` can be a NEAR account, an EVM address, or a SOL address to which you have access.

    <Tip>
      We recommend starting with a small amount for experimentation. After the deposit is completed, you can connect your wallet and check the tokens.
    </Tip>
  </Accordion>

  <Accordion title="Is there support for native NEAR deposits?">
    Only `ft_transfer_call` can be used to deposit NEP-141 tokens from NEAR to `intents.near`:

    ```javascript theme={null}
    <TOKEN_ACCOUNT_ID>::ft_transfer_call({
      "receiver_id": "intents.near",
      "amount": "1234",
      "msg": "{\"receiver_id\": \"<ACCOUNT_ID>\"}"
    })
    ```

    Here is an example [receipt](https://nearblocks.io/txns/EwmeXzZJStA6e5JB49vgxNYJDemqeYCFGvPH7zapP1Fw#execution#4tyaF4MnMcNQVqrg3kXzsH9277ErDeCXS9g3c2keV38G).

    The `msg` parameter can also be empty, so that funds will be deposited to `sender_id` (i.e. the caller of `ft_transfer_call`). Here is an example of such a [transaction](https://nearblocks.io/txns/HoWpAR8dF5azsUVaQWrBW5VsRve5X4dwr9GGiHWj3R1P#execution).
  </Accordion>

  <Accordion title="Can deposits be made via custom smart contract calls on EVM chains?">
    For EVM chains, NEAR Intents does not impose limitations on how deposits are executed at the smart contract level. Our listeners track the chain for:

    * ERC-20 `Transfer` events
    * Regular or internal native transfers

    to any of the supported deposit addresses.

    As long as the resulting transaction emits a valid `Transfer` event (or performs a native value transfer) to the deposit address, it will be detected—even if it originates from a custom contract method (for example, a custom `withdraw()` function instead of `transfer` / `transferFrom`).
  </Accordion>

  <Accordion title="Why is tx_hash empty in the recent_deposit response for SOL deposits?">
    This information is not available for Solana because the mechanism of deposit tracking works differently on that chain.
  </Accordion>

  <Accordion title="Why aren't my BTC UTXOs being swept? I sent a small amount.">
    Very small amounts (e.g., 5,000 sats) are considered "dust" and there is special business logic to process such small amounts. This may cause delays in sweeping.
  </Accordion>
</AccordionGroup>

## ETH Connector Migration

<AccordionGroup>
  <Accordion title="What changed with the ETH connector split?">
    Because of the split of ETH-connector, the `aurora` contract now [acts](https://github.com/aurora-is-near/aurora-engine/blob/3416d0d170bf3dcaaac3ad3b9ba751d002ea1b3f/engine/src/contract_methods/connector/mod.rs#L209-L210) as a NEP-141 proxy to `eth.bridge.near`. This leads to some changes:

    1. **All new deposits** will be [treated](https://nearblocks.io/txns/ExLxkbRjKbSDSTVozPpSsTfRL8Aqfs8kvLxFLEx4qDzt#enhanced) as `eth.bridge.near`, even if triggered from the `aurora` smart contract—even from inside Aurora Mainnet (example [tx](https://explorer.aurora.dev/tx/0x9c20d9f76443ec3c12f8eb41a65caa0c1391210c539a5924215b9bdf9e0b1fd2?tab=index)). All withdrawals of legacy `aurora` will be received on NEAR as new `eth.bridge.near`.

    2. **Migration of already deposited ETH\@NEAR (`aurora`)** can be done permissionlessly by withdrawing from `intents.near` and depositing it back right away. This can be done in a single transaction thanks to [this patch](https://github.com/Near-One/aurora-eth-connector/blob/58d3f39cebcf6266514de3dd04efec5bafb6274e/eth-connector/src/lib.rs#L132-L143) on the eth-connector side.
  </Accordion>

  <Accordion title="How do I migrate legacy aurora tokens to eth.bridge.near?">
    Users can migrate in two ways:

    **Option 1: Via `ft_withdraw()` transaction**

    [Example transaction](https://nearblocks.io/txns/GHrRbGsDuv86u72jHNqofZYTUssfuXyYwcJ8mYyLrq2v#enhanced) with the following params:

    ```json theme={null}
    {
      "token": "aurora",
      "receiver_id": "intents.near",
      "amount": "1234",
      "memo": "Migrate ETH: aurora -> eth.bridge.near",
      "msg": "<USER_ACCOUNT_ID>"
    }
    ```

    **Option 2: Via `ft_withdraw` intent**

    Use the same parameters as above.

    <Tip>
      The [front-end](https://app.near-intents.org) automatically detects legacy tokens on your balance and prompts you to sign a migration intent.
    </Tip>
  </Accordion>
</AccordionGroup>

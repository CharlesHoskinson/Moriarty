> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Fees

> All fees that apply to NEAR Intents transactions and integrations

## Protocol Fee

* **0.0001% (1 pip)** per transaction
* Collected on-chain by the [`intents.near`](https://nearblocks.io/address/intents.near) smart contract
* Applies to every transfer, swap, or transaction
* Fees are sent to the [`fee_collector`](https://near-intents.org/account?user=near:7066024d3f20f94de601c003163367873cca78507eeca4df66d9be645f197f05)

## Near-Intents.org Fee

* **0.2%** fee on swaps executed through [near-intents.org](https://near-intents.org)
* Collected by the proprietary distribution channel in addition to the protocol fee
* Fees are sent to [`fefundsadmin.sputnik-dao.near`](https://nearblocks.io/address/fefundsadmin.sputnik-dao.near)

## Withdrawal Fees

* **0.1%** fee for **NEAR**, **ZEC**, and **STRK** tokens withdrawn to the **Solana** network

## 1Click Swap API Fees

| Authentication      | Fee                           |
| ------------------- | ----------------------------- |
| **With API key**    | Only the 0.0001% protocol fee |
| **Without API key** | Additional **0.2%** fee       |

<Tip>
  [Apply for an API key](https://partners.near-intents.org/) to avoid the 0.2% unauthenticated fee.
</Tip>

### Quote improvement fee (1Click / NEAR Intents)

**Definition:** If a swap execution gets **filled at a better price than the quoted price**, that difference is split **50/50** between you and the protocol.

**Eligible orders:** Real cross-asset swaps (not same-token moves). Only applies while the quote is still **fresh—within about 30 minutes** of when it was issued.

**Fee calculation:** **Direct 50/50 split** of the measured improvement, or **zero** if execution did not beat the quote.

### Distribution Channel Fees

Developers and distribution channels can add their own fees using the [`appFees`](/integration/distribution-channels/1click-api/fee-config) parameter when requesting quotes.

```json theme={null}
{
  "appFees": [{
    "recipient": "your-wallet.near",
    "fee": 50
  }]
}
```

This example charges a 0.5% fee (50 basis points) from the input token.

### Revenue Share

All partners participate in a **50/50 revenue share** by default. Half of the fee amount specified via `appFees` is automatically sent to the 1Click protocol address, and the other half goes to the partner's `recipient` address.

For example, if you set `"fee": 10` (10 basis points), 5 bps go to your partner address and 5 bps go to the 1Click address.

<Info>
  Revenue is shared equally between partners and NEAR Intents, with a 50/50 split.
</Info>

<Card title="Fee Configuration Guide" icon="wallet" href="/integration/distribution-channels/1click-api/fee-config">
  Learn how to configure and collect fees from your integration
</Card>

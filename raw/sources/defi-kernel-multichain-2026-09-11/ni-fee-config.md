> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Fee Configuration

> Configure fee collection and aggregation

Configure how fees are collected and distributed when using the [1Click Swap API](/integration/distribution-channels/1click-api/about-1click-api). As a distribution channel, you can add your own fees on top of the base platform fees.

<Tip>
  This guide covers **distribution channel fees** (fees you collect). For an overview of all platform fees including protocol fees and API fees, see [Fees](/resources/fees).
</Tip>

***

## Fee Parameters

Include these parameters in the `appFees` array when requesting a quote:

| Parameter   | Type   | Description                                                                                         |
| ----------- | ------ | --------------------------------------------------------------------------------------------------- |
| `recipient` | string | Any NEAR-supported address (named account like `alice.near`, implicit account, or EVM-like address) |
| `fee`       | number | Fee in basis points. `100` = `1.00%`. Fee is charged from the input token.                          |

* **Conversion formula:** `percentage = fee / 10,000` (e.g. `fee: 50` equals 0.5% fee)
* **Constraints:** Fee range: `0` to `500` (where `500` = 5%)

<Info>
  A **50/50 revenue share** applies by default — half of the `fee` amount goes to your `recipient` address and half goes to the 1Click protocol address. For example, `"fee": 10` means 5 bps to you and 5 bps to 1Click.
</Info>

***

## How Fees Are Applied

Fees are calculated differently depending on the swap type:

<Tabs>
  <Tab title="EXACT_INPUT">
    For EXACT\_INPUT swaps, the fee is deducted from your input amount before the swap:

    1. `net_in = amount_in * (1 - p)` where `p = fee / 10,000`
    2. The quote calculates `amount_out` from `net_in`
    3. `fee_amount = amount_in - net_in` (deducted in input token)

    **Example:**

    * Input: `amount_in = 1,000,000`, `fee = 100` (1%)
    * Calculation: `net_in = 1,000,000 * (1 - 0.01) = 990,000`
    * User deposits `1,000,000` and quote calculates output from `990,000`
    * Fee: `10,000` units (in input token)
  </Tab>

  <Tab title="EXACT_OUTPUT">
    For EXACT\_OUTPUT swaps, the fee increases the required input amount:

    1. `net_in = min_amount_in * (1 + p)` where `p = fee / 10,000`
    2. `fee_amount = net_in - min_amount_in` (deducted in input token)
    3. If user deposits more than `min_amount_in`, fees are deducted from actual `amount_in`

    **Example:**

    * Input: `min_amount_in = 500,000`, `fee = 100` (1%)
    * Calculation: `net_in = 500,000 * (1 + 0.01) = 505,000`
    * User must deposit `505,000` to receive the exact output amount
    * Fee: `5,000` units (in input token)
  </Tab>

  <Tab title="FLEX_INPUT">
    FLEX\_INPUT is input-side like EXACT\_IN, so the fee is deducted from the input token. Because the deposit amount can vary within the quoted band, the fee is applied to whatever is actually deposited:

    1. `net_in = amount_in * (1 - p)` where `p = fee / 10,000` and `amount_in` is the actual deposit
    2. The quote calculates `amount_out` from `net_in`
    3. `fee_amount = amount_in - net_in` (deducted in input token)

    **Example** (quote band `0.99–1.00 NEAR`, `fee = 100` / 1%):

    * User deposits `amount_in = 1,000,000`
    * Calculation: `net_in = 1,000,000 * (1 - 0.01) = 990,000`
    * Output is calculated from `990,000`
    * Fee: `10,000` units (in input token)

    A smaller deposit within the band pays a proportionally smaller fee, since the percentage applies to the actual amount deposited.
  </Tab>
</Tabs>

<br />

<Accordion title="Example Quote with Fees">
  ```json theme={null}
  {
    "dry": false,
    "swapType": "EXACT_INPUT",
    "originAsset": "nep141:wrap.near",
    "destinationAsset": "nep141:usdt.tether-token.near",
    "amount": "1000000000000000000000000",
    "recipient": "user.near",
    "recipientType": "INTENTS",
    "refundTo": "user.near",
    "refundType": "INTENTS",
    "appFees": [{
      "recipient": "your-fee-wallet.near",
      "fee": 50
    }]
  }
  ```
</Accordion>

***

## Fee Aggregation

For high-volume integrations that collect fees in many different tokens, the `ANY_INPUT` swap type aggregates them all into a single destination asset and withdraws automatically.

<Info>
  To compare `ANY_INPUT` with the other swap types, see the [Swap Types](./swap-types) page.
</Info>

### How it works

* Deposits in **any supported token** are sent to one dedicated `depositAddress` and accumulate there.
* A background job continuously swaps everything to your chosen `destinationAsset` and withdraws it to `recipient` once the pool reaches **\$1,000 USD**.
* The `deadline` is checked only when the quote is **created** — after that the quote runs **indefinitely**, so a single quote keeps collecting and withdrawing without being refreshed.
* Failed swaps retry every 5 minutes.

<Warning>
  **No refunds.** If a swap fails it retries automatically rather than returning funds — set `refundTo` to an address you control as a safety measure.
</Warning>

### Set it up

<Info>
  Every request must include an `Authorization: Bearer YOUR_JWT_TOKEN` header to receive the quote and its `depositAddress`. Get your API key from the [Partner Dashboard](https://partners.near-intents.org/).
</Info>

<Steps>
  <Step title="Create an ANY_INPUT quote">
    Request a quote with `originAsset: "1cs_v1:any"` and `amount: "0"` to get a dedicated `depositAddress` for fee collection.

    <Accordion title="Example ANY_INPUT request">
      ```json theme={null}
      {
        "dry": false,
        "swapType": "ANY_INPUT",
        "slippageTolerance": 0,
        "originAsset": "1cs_v1:any",
        "depositType": "INTENTS",
        "destinationAsset": "nep141:eth-0xdac17f958d2ee523a2206206994597c13d831ec7.omft.near",
        "amount": "0",
        "refundTo": "your-wallet.near",
        "refundType": "INTENTS",
        "recipient": "0x1ddA60d784483FBB54304c68830d42A706327C6d",
        "recipientType": "DESTINATION_CHAIN",
        "deadline": "2025-01-01T00:00:00.000Z",
        "referral": "YOUR_REFERRAL",
        "quoteWaitingTimeMs": 10000
      }
      ```
    </Accordion>

    | Field              | Description                                    |
    | ------------------ | ---------------------------------------------- |
    | `originAsset`      | Must be `"1cs_v1:any"` for fee aggregation     |
    | `destinationAsset` | Token you want to collect fees in              |
    | `recipient`        | Address where converted fees will be withdrawn |
    | `recipientType`    | `"DESTINATION_CHAIN"` or `"INTENTS"`           |
    | `refundTo`         | Set to an address you control                  |

    <Tip>
      Set `quoteWaitingTimeMs` to 5000–10000ms for optimal performance (3–4s is often enough).
    </Tip>
  </Step>

  <Step title="Wire the deposit address into your fees">
    Use the `depositAddress` from the quote response as the `appFees.recipient` in your user quotes (see [Fee Parameters](#fee-parameters)). Collected fees now flow to the aggregation address and convert automatically.
  </Step>
</Steps>

### Track withdrawals

Use the `/v0/any-input/withdrawals` endpoint to retrieve withdrawal records for your deposit address.

**Request:**

```bash theme={null}
GET /v0/any-input/withdrawals?depositAddress=YOUR_DEPOSIT_ADDRESS
```

Records are filtered by `depositAddress` and sorted by `timestamp` (newest first). Page through results with these query parameters:

| Parameter        | Description                                                         |
| ---------------- | ------------------------------------------------------------------- |
| `depositAddress` | **Required.** The deposit address whose withdrawals you're fetching |
| `depositMemo`    | Memo, if the deposit address requires one                           |
| `timestampFrom`  | Only return withdrawals from this ISO timestamp onward              |
| `page`           | Page number (default `1`)                                           |
| `limit`          | Records per page (max and default `50`)                             |
| `sortOrder`      | `asc` or `desc` by `timestamp` (default `desc`)                     |

**Response:**

```json theme={null}
{
  "recipient": "0x88b9da55b59a59751424b357cf8aea239770944c",
  "affiliateRecipient": "2fd5aba4dc4729dcaec040242fc3b0b1faf8c81f74831c7c7843c6a96ad28add",
  "asset": "nep141:base-0x833589fcd6edb6e08f4c7c32d4f71b54bda02913.omft.near",
  "withdrawals": [
    {
      "status": "SUCCESS",
      "amountOut": "140735",
      "amountOutFormatted": "0.140735",
      "amountOutUsd": "0.140693483175",
      "withdrawFee": "2400",
      "withdrawFeeFormatted": "0.0024",
      "withdrawFeeUsd": "0.0023992919999999995",
      "timestamp": "2025-10-07T10:16:19.702Z",
      "hash": "0xcc005d3e3340c61b1240905c4e383d8b8ecb114a827bef3e0394293997406621"
    }
  ]
}
```

**Response Fields:**

| Field                  | Description                                                                                    |
| ---------------------- | ---------------------------------------------------------------------------------------------- |
| `recipient`            | The `destinationAsset` address these fees are withdrawn to (top-level, applies to all records) |
| `affiliateRecipient`   | Internal identifier for this fee-collection deposit address (top-level)                        |
| `asset`                | The `destinationAsset` fees are converted into (top-level)                                     |
| `status`               | Withdrawal status (e.g., `SUCCESS`)                                                            |
| `amountOut`            | Raw amount withdrawn                                                                           |
| `amountOutFormatted`   | Human-readable amount                                                                          |
| `amountOutUsd`         | USD value at time of withdrawal                                                                |
| `withdrawFee`          | Fee charged for withdrawal, in the smallest unit                                               |
| `withdrawFeeFormatted` | Human-readable withdrawal fee                                                                  |
| `timestamp`            | When the withdrawal occurred                                                                   |
| `hash`                 | Transaction hash                                                                               |

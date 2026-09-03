> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Hyperliquid

> Send USDC to Hyperliquid, or deposit USDC from Hyperliquid, through the 1Click Swap API

You can send USDC to Hyperliquid, and deposit USDC from Hyperliquid, through the [1Click Swap API](./about-1click-api). Same [quote, deposit, and status](./quickstart/making-a-request) flow as any other swap.

### Hyperliquid USDC

`1cs_v1:hypercore:hip1:0x6d1e7cde53ba9467b783cb7c530ce054` (8 decimals)

***

## Send USDC to Hyperliquid

Set `destinationAsset` to Hyperliquid USDC and `recipient` to the user's Hyperliquid `0x` address. USDC is credited to their Hyperliquid **perps** balance.

| Field              | Value                               |
| ------------------ | ----------------------------------- |
| `destinationAsset` | Hyperliquid USDC                    |
| `recipient`        | User's Hyperliquid EVM `0x` address |
| `recipientType`    | `DESTINATION_CHAIN`                 |
| `swapType`         | `EXACT_INPUT` or `EXACT_OUTPUT`     |

<Warning>
  Funds do not arrive as HyperEVM ERC-20 or in Hyperliquid spot.
</Warning>

Example: Ethereum USDT → Hyperliquid USDC. Highlighted lines are the Hyperliquid fields.

<CodeGroup>
  ```bash cURL {9,12,13} theme={null}
  curl -X POST https://1click.chaindefuser.com/v0/quote \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer YOUR_JWT_TOKEN" \
    -d '{
      "dry": false,
      "swapType": "EXACT_INPUT",
      "slippageTolerance": 100,
      "originAsset": "nep141:eth-0xdac17f958d2ee523a2206206994597c13d831ec7.omft.near",
      "destinationAsset": "1cs_v1:hypercore:hip1:0x6d1e7cde53ba9467b783cb7c530ce054",
      "amount": "5000000",
      "depositType": "ORIGIN_CHAIN",
      "recipient": "0xYOUR_HYPERLIQUID_ADDRESS",
      "recipientType": "DESTINATION_CHAIN",
      "refundTo": "0xYOUR_ETHEREUM_ADDRESS",
      "refundType": "ORIGIN_CHAIN",
      "deadline": "2026-12-31T00:00:00.000Z"
    }'
  ```

  ```typescript TypeScript {12,15,16} theme={null}
  const quote = await fetch('https://1click.chaindefuser.com/v0/quote', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': 'Bearer YOUR_JWT_TOKEN'
    },
    body: JSON.stringify({
      dry: false,
      swapType: 'EXACT_INPUT',
      slippageTolerance: 100,
      originAsset: 'nep141:eth-0xdac17f958d2ee523a2206206994597c13d831ec7.omft.near',
      destinationAsset: '1cs_v1:hypercore:hip1:0x6d1e7cde53ba9467b783cb7c530ce054',
      amount: '5000000',
      depositType: 'ORIGIN_CHAIN',
      recipient: '0xYOUR_HYPERLIQUID_ADDRESS',
      recipientType: 'DESTINATION_CHAIN',
      refundTo: '0xYOUR_ETHEREUM_ADDRESS',
      refundType: 'ORIGIN_CHAIN',
      deadline: '2026-12-31T00:00:00.000Z'
    })
  });
  ```
</CodeGroup>

`amount` `5000000` is 5 USDT on Ethereum (6 decimals, origin). Then send `quote.amountIn` to `quote.depositAddress` and track status as in the [Quickstart](./quickstart/making-a-request).

***

## Deposit USDC from Hyperliquid

Same `POST /v0/quote` as above, with origin and destination swapped.

| Field         | Value            |
| ------------- | ---------------- |
| `originAsset` | Hyperliquid USDC |
| `depositType` | `ORIGIN_CHAIN`   |

Example: Hyperliquid USDC → Ethereum USDT. Highlighted lines are the Hyperliquid fields.

<CodeGroup>
  ```bash cURL {8,11} theme={null}
  curl -X POST https://1click.chaindefuser.com/v0/quote \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer YOUR_JWT_TOKEN" \
    -d '{
      "dry": false,
      "swapType": "EXACT_INPUT",
      "slippageTolerance": 100,
      "originAsset": "1cs_v1:hypercore:hip1:0x6d1e7cde53ba9467b783cb7c530ce054",
      "destinationAsset": "nep141:eth-0xdac17f958d2ee523a2206206994597c13d831ec7.omft.near",
      "amount": "500000000",
      "depositType": "ORIGIN_CHAIN",
      "recipient": "0xYOUR_ETHEREUM_ADDRESS",
      "recipientType": "DESTINATION_CHAIN",
      "refundTo": "0xYOUR_HYPERLIQUID_ADDRESS",
      "refundType": "ORIGIN_CHAIN",
      "deadline": "2026-12-31T00:00:00.000Z"
    }'
  ```

  ```typescript TypeScript {11,14} theme={null}
  const quote = await fetch('https://1click.chaindefuser.com/v0/quote', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': 'Bearer YOUR_JWT_TOKEN'
    },
    body: JSON.stringify({
      dry: false,
      swapType: 'EXACT_INPUT',
      slippageTolerance: 100,
      originAsset: '1cs_v1:hypercore:hip1:0x6d1e7cde53ba9467b783cb7c530ce054',
      destinationAsset: 'nep141:eth-0xdac17f958d2ee523a2206206994597c13d831ec7.omft.near',
      amount: '500000000',
      depositType: 'ORIGIN_CHAIN',
      recipient: '0xYOUR_ETHEREUM_ADDRESS',
      recipientType: 'DESTINATION_CHAIN',
      refundTo: '0xYOUR_HYPERLIQUID_ADDRESS',
      refundType: 'ORIGIN_CHAIN',
      deadline: '2026-12-31T00:00:00.000Z'
    })
  });
  ```
</CodeGroup>

`amount` `500000000` is 5 USDC (8 decimals).

### Deposit fee

A flat **0.2 USDC** is deducted from each USDC transfer to the deposit address.

Example: **5 USDC** arrives → **4.8 USDC** is credited to Intents, then paid out to `recipient`.

Send `quote.amountIn`. Do not add 0.2 on top.

### Minimum deposit

Minimum deposit is **0.5 USDC**. Smaller amounts are not processed.

### Transfer methods

Hyperliquid network/gas fees are paid on the transfer and are **not** part of the USDC figures.

<Tabs>
  <Tab title="sendAsset">
    Supported. Spot or perp — the standard **Send** in the Hyperliquid app.

    Send `quote.amountIn`.
  </Tab>

  <Tab title="spotSend">
    Supported. Spot only — USDC transfer on Hyperliquid spot.

    Send `quote.amountIn`.
  </Tab>

  <Tab title="usdSend">
    Partial. Perp only. Extra **1 USDC** is deducted on top of the 0.2 USDC deposit fee.

    Send `quote.amountIn` **+ 1 USDC**. Sending only `amountIn` (e.g. 10) credits **8.80** (10 − 1 − 0.2) and can leave the quote in `INCOMPLETE_DEPOSIT`.
  </Tab>

  <Tab title="Unsupported">
    NEAR Intents gives no guarantees of processing or crediting of funds for:

    * Any bridge transfer directly to the deposit address (CCTP, the native Arbitrum ↔ Hyperliquid bridge, and similar)
    * Any transfer method not listed above
  </Tab>
</Tabs>

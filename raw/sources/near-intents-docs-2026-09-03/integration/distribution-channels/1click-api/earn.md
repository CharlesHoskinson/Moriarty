> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Earn

> Offer multichain yield through the 1Click Swap API

Earn lets wallets and apps route users into third-party yield protocols through the [1Click Swap API](./about-1click-api). The user pays a supported asset; NEAR Intents coordinates execution; the user receives **fungible receipt tokens** (for example vault shares) that represent the position.

<Info>
  NEAR Intents does not run these yield protocols, set their APY, or guarantee principal. Protocols are operated by independent providers. See [§7 Special asset types](/security-compliance/terms-of-service#7-special-asset-types-and-disclaimers).
</Info>

***

## Integration model

Earn uses the standard 1Click lifecycle:

1. Resolve `assetId`s with [`GET /v0/tokens`](#yield-and-receipt-assets).
2. Request a quote with [`POST /v0/quote`](./quickstart/making-a-request).
3. Complete the deposit and track status with [`GET /v0/status`](./quickstart/making-a-request).

Deposit quotes use a payment asset as `originAsset` and a receipt token as `destinationAsset`. Withdraw quotes reverse that. Use the same [swap types](./swap-types) as other 1Click quotes.

These fields control where value is taken from and where it is delivered:

| Field                         | On deposit                                                         | On withdraw                                                  |
| ----------------------------- | ------------------------------------------------------------------ | ------------------------------------------------------------ |
| `depositType`                 | Where the **payment** comes from (`ORIGIN_CHAIN` or `INTENTS`)     | Where the **shares** come from (`ORIGIN_CHAIN` or `INTENTS`) |
| `recipient` / `recipientType` | Where the **receipt tokens** go (`DESTINATION_CHAIN` or `INTENTS`) | Where the **payout** goes (`DESTINATION_CHAIN` or `INTENTS`) |

Deposit and withdraw examples below cover both delivery options.

***

## Yield and receipt assets

```bash theme={null}
curl https://1click.chaindefuser.com/v0/tokens
```

Only instruments that appear in this list and return quotes are available for Earn via 1Click. Look up the receipt token and payment asset by `symbol` (or other fields you care about) and use the returned `assetId` values in quotes.

You can request additional protocol support. Protocols already listed in the [Yield.xyz DeFi yields](https://docs.yield.xyz/docs/defi-yields) catalog are usually faster to add than those outside it.

For the examples on this page we use two vault-share symbols that are listed today — **`TLO`** and **`gtUSDCp`** — plus **USDC** as the payment asset. Treat the `assetId` strings in the snippets as illustrative snapshots; always take current IDs from `/v0/tokens` before you ship.

<Warning>
  Listings and liquidity can change. A quote may return `No liquidity available` or reject an unsupported delivery mode for a given asset.
</Warning>

See [Asset support](/resources/asset-support).

***

## Deposit — destination-chain wallet

Use this when the user should receive receipt tokens on the protocol’s chain.

Set `recipient` to their address on that chain and `recipientType` to `DESTINATION_CHAIN`. Highlighted lines are the delivery fields.

Example below: Ethereum USDC → `TLO`, delivered to an Ethereum address. Resolve both `assetId`s from `/v0/tokens` (symbols `USDC` on `eth`, `TLO` on `eth`).

<CodeGroup>
  ```bash cURL {12,13} theme={null}
  curl -X POST https://1click.chaindefuser.com/v0/quote \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer YOUR_JWT_TOKEN" \
    -d '{
      "dry": false,
      "swapType": "EXACT_INPUT",
      "slippageTolerance": 100,
      "originAsset": "nep141:eth-0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.omft.near",
      "destinationAsset": "nep141:eth-0x0f38f1ce62776d4a0038bc6cac66877a5687383b.omft.near",
      "amount": "100000000",
      "depositType": "ORIGIN_CHAIN",
      "recipient": "0xYourEvmAddress",
      "recipientType": "DESTINATION_CHAIN",
      "refundTo": "0xYourEvmAddress",
      "refundType": "ORIGIN_CHAIN",
      "deadline": "2026-12-31T00:00:00.000Z"
    }'
  ```

  ```typescript TypeScript {16,17} theme={null}
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
      originAsset: 'nep141:eth-0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.omft.near',
      destinationAsset: 'nep141:eth-0x0f38f1ce62776d4a0038bc6cac66877a5687383b.omft.near',
      amount: '100000000',
      depositType: 'ORIGIN_CHAIN',
      recipient: '0xYourEvmAddress',
      recipientType: 'DESTINATION_CHAIN',
      refundTo: '0xYourEvmAddress',
      refundType: 'ORIGIN_CHAIN',
      deadline: '2026-12-31T00:00:00.000Z'
    })
  });
  ```
</CodeGroup>

`amount` `100000000` is 100 USDC (6 decimals). The same pattern works with other listed receipt tokens that quote for destination-chain delivery.

***

## Deposit — Intents balance

Use this when the user should hold receipt tokens in Intents (bridged to NEAR) instead of on the protocol’s chain.

Set `recipient` to their Intents account and `recipientType` to `INTENTS`. Highlighted lines are what differs from destination-chain delivery.

Example below: Ethereum USDC → `TLO`, held in Intents. Same symbols as above; only delivery fields change.

<CodeGroup>
  ```bash cURL {12,13} theme={null}
  curl -X POST https://1click.chaindefuser.com/v0/quote \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer YOUR_JWT_TOKEN" \
    -d '{
      "dry": false,
      "swapType": "EXACT_INPUT",
      "slippageTolerance": 100,
      "originAsset": "nep141:eth-0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.omft.near",
      "destinationAsset": "nep141:eth-0x0f38f1ce62776d4a0038bc6cac66877a5687383b.omft.near",
      "amount": "100000000",
      "depositType": "ORIGIN_CHAIN",
      "recipient": "user.near",
      "recipientType": "INTENTS",
      "refundTo": "0xYourEvmAddress",
      "refundType": "ORIGIN_CHAIN",
      "deadline": "2026-12-31T00:00:00.000Z"
    }'
  ```

  ```typescript TypeScript {16,17} theme={null}
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
      originAsset: 'nep141:eth-0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.omft.near',
      destinationAsset: 'nep141:eth-0x0f38f1ce62776d4a0038bc6cac66877a5687383b.omft.near',
      amount: '100000000',
      depositType: 'ORIGIN_CHAIN',
      recipient: 'user.near',
      recipientType: 'INTENTS',
      refundTo: '0xYourEvmAddress',
      refundType: 'ORIGIN_CHAIN',
      deadline: '2026-12-31T00:00:00.000Z'
    })
  });
  ```
</CodeGroup>

You can use the same Intents delivery with other listed shares (for example `gtUSDCp`). Replace `recipient`, `refundTo`, and the JWT as needed. After the quote, deposit and track status as in the [Quickstart](./quickstart/making-a-request).

***

## Withdraw — payout to destination-chain wallet

Quote from the receipt token back to a payment asset. Use the user’s share balance as `amount`.

Example below: redeem `TLO` held in Intents (`depositType: INTENTS`) to Ethereum USDC on a chain wallet. `amount` `10000000000` is **100 `TLO`** at 8 decimals — confirm decimals from `/v0/tokens`. Highlighted lines are the payout delivery fields.

<CodeGroup>
  ```bash cURL {12,13} theme={null}
  curl -X POST https://1click.chaindefuser.com/v0/quote \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer YOUR_JWT_TOKEN" \
    -d '{
      "dry": false,
      "swapType": "EXACT_INPUT",
      "slippageTolerance": 100,
      "originAsset": "nep141:eth-0x0f38f1ce62776d4a0038bc6cac66877a5687383b.omft.near",
      "destinationAsset": "nep141:eth-0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.omft.near",
      "amount": "10000000000",
      "depositType": "INTENTS",
      "recipient": "0xYourEvmAddress",
      "recipientType": "DESTINATION_CHAIN",
      "refundTo": "user.near",
      "refundType": "INTENTS",
      "deadline": "2026-12-31T00:00:00.000Z"
    }'
  ```

  ```typescript TypeScript {16,17} theme={null}
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
      originAsset: 'nep141:eth-0x0f38f1ce62776d4a0038bc6cac66877a5687383b.omft.near',
      destinationAsset: 'nep141:eth-0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.omft.near',
      amount: '10000000000',
      depositType: 'INTENTS',
      recipient: '0xYourEvmAddress',
      recipientType: 'DESTINATION_CHAIN',
      refundTo: 'user.near',
      refundType: 'INTENTS',
      deadline: '2026-12-31T00:00:00.000Z'
    })
  });
  ```
</CodeGroup>

## Withdraw — payout to Intents balance

Use this when the payout should stay in Intents. Same redeem as above; only delivery fields change. Highlighted lines are what differs.

<CodeGroup>
  ```bash cURL {12,13} theme={null}
  curl -X POST https://1click.chaindefuser.com/v0/quote \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer YOUR_JWT_TOKEN" \
    -d '{
      "dry": false,
      "swapType": "EXACT_INPUT",
      "slippageTolerance": 100,
      "originAsset": "nep141:eth-0x0f38f1ce62776d4a0038bc6cac66877a5687383b.omft.near",
      "destinationAsset": "nep141:eth-0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.omft.near",
      "amount": "10000000000",
      "depositType": "INTENTS",
      "recipient": "user.near",
      "recipientType": "INTENTS",
      "refundTo": "user.near",
      "refundType": "INTENTS",
      "deadline": "2026-12-31T00:00:00.000Z"
    }'
  ```

  ```typescript TypeScript {16,17} theme={null}
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
      originAsset: 'nep141:eth-0x0f38f1ce62776d4a0038bc6cac66877a5687383b.omft.near',
      destinationAsset: 'nep141:eth-0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.omft.near',
      amount: '10000000000',
      depositType: 'INTENTS',
      recipient: 'user.near',
      recipientType: 'INTENTS',
      refundTo: 'user.near',
      refundType: 'INTENTS',
      deadline: '2026-12-31T00:00:00.000Z'
    })
  });
  ```
</CodeGroup>

Quote from the balance you intend to redeem; do not assume a fixed share-to-underlying rate. The same shapes work for other listed shares such as `gtUSDCp`. If the shares sit on the origin chain, set `depositType` (and matching refund fields) to `ORIGIN_CHAIN` and keep choosing payout with `recipientType` as above.

***

## APY

The 1Click API does not return protocol metadata such as APY. When you show rates in your UI, load them from the Yield.xyz aggregator API:

1. Use [List yields](https://docs.yield.xyz/reference/yieldscontroller_getyields) to find the yield that matches your receipt token (for example by protocol or token symbol).
2. Use [Get a yield](https://docs.yield.xyz/reference/yieldscontroller_getyield) for that yield’s APY and other metadata.

Yield.xyz data is third-party; it is not returned or guaranteed by 1Click.

***

## Fees and disclosures

* 1Click platform fees apply as for other quotes ([Fee configuration](./fee-config), [Fees](/resources/fees)).
* Third-party protocols may charge their own fees.
* Yield, share price, and principal are not guaranteed by NEAR Intents.
* End-user disclosures for yield assets should align with [Terms of Service §7](/security-compliance/terms-of-service#7-special-asset-types-and-disclaimers).

***

## Related

<CardGroup cols={2}>
  <Card title="Making a request" icon="paper-plane" href="./quickstart/making-a-request">
    Quote, deposit, and status for 1Click swaps
  </Card>

  <Card title="Swap types" icon="right-left" href="./swap-types">
    EXACT\_INPUT, EXACT\_OUTPUT, and deposit handling
  </Card>

  <Card title="Asset support" icon="coins" href="/resources/asset-support">
    Live token catalog from the 1Click API
  </Card>

  <Card title="Terms of Service" icon="file-contract" href="/security-compliance/terms-of-service">
    Yield-bearing assets and Earn
  </Card>
</CardGroup>

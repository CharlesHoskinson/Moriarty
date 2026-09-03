> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Limit Orders

> Offer a swap at the user's price through the 1Click Swap API

Limit Orders let wallets and apps rest a swap at a price the user sets, through the [1Click Swap API](./about-1click-api).

<Info>
  Limit orders run as confidential 1Click swaps. They are not Perpetuals on near.com. See [Confidential Swaps](./quickstart/confidential-swaps) and the [1Click Terms of Service](/security-compliance/terms-of-service).
</Info>

***

## Place a limit order

Sell **1 NEAR** for Ethereum USDC at **5 USDC** per NEAR. The user pays on NEAR; filled USDC goes to their Ethereum address. Partial fills are allowed.

1. Look up both `assetId`s with [`GET /v0/tokens`](#tokens).
2. Create the order with `POST /v0/orders`.
3. Send the amount on `swapView` to `depositAddress`, then poll [`GET /v0/orders/{orderId}`](#watch-the-order).

You send `quantity`, `side`, and `price`. `quantity` is the base in smallest units. `price` is quote per one base — `"5"` is 5 USDC per 1 NEAR.

This page uses NEAR (24 decimals) and Ethereum USDC (6 decimals):

|             | `SELL`              | `BUY`               |
| ----------- | ------------------- | ------------------- |
| `quantity`  | 1 NEAR (`10^24`)    | 1 NEAR (`10^24`)    |
| You deposit | 1 NEAR (`10^24`)    | 5 USDC (`5 × 10^6`) |
| You receive | 5 USDC (`5 × 10^6`) | 1 NEAR (`10^24`)    |

The quote-side amount is `quantity × price × 10^quoteDecimals / 10^baseDecimals`, rounded up to a whole smallest unit. [After create](#create), send the amount on `swapView`.

***

## Tokens

```bash theme={null}
curl https://1click.chaindefuser.com/v0/tokens
```

Look up both sides of the pair by `symbol` (or other fields you care about) and use the returned `assetId` as `baseAsset` and `quoteAsset`. Take `decimals` from the same object.

For the examples on this page:

|       | `symbol` | `blockchain` | `decimals` |
| ----- | -------- | ------------ | ---------- |
| Base  | `wNEAR`  | `near`       | 24         |
| Quote | `USDC`   | `eth`        | 6          |

`baseAsset` is `nep141:wrap.near`. `quoteAsset` is `nep141:eth-0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.omft.near`. Treat those `assetId` strings as snapshots; take current IDs from `/v0/tokens` before you ship.

See [Asset support](/resources/asset-support).

***

## Authorization

Send a [partner JWT](./authentication) as `X-API-Key` for your app. If the user is signing in, send their [User-Session](./authentication#authenticating-end-users-confidential-intents) as `Authorization: Bearer`.

Use that same identity on create, get, list, and cancel so you can keep managing the order.

***

## Create

The user pays on the origin chain and receives filled output on a destination-chain address.

<CodeGroup>
  ```bash cURL theme={null}
  curl -X POST https://1click.chaindefuser.com/v0/orders \
    -H "Content-Type: application/json" \
    -H "X-API-Key: YOUR_JWT_TOKEN" \
    -d '{
      "data": {
        "type": "orders",
        "attributes": {
          "orderType": "LIMIT",
          "baseAsset": "nep141:wrap.near",
          "quoteAsset": "nep141:eth-0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.omft.near",
          "quantity": "1000000000000000000000000",
          "side": "SELL",
          "price": "5",
          "depositType": "ORIGIN_CHAIN",
          "refundTo": "your-account.near",
          "refundType": "ORIGIN_CHAIN",
          "recipient": "0xYourEvmAddress",
          "recipientType": "DESTINATION_CHAIN",
          "confidentiality": "basic"
        }
      }
    }'
  ```

  ```typescript TypeScript theme={null}
  const created = await fetch('https://1click.chaindefuser.com/v0/orders', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'X-API-Key': 'YOUR_JWT_TOKEN'
    },
    body: JSON.stringify({
      data: {
        type: 'orders',
        attributes: {
          orderType: 'LIMIT',
          baseAsset: 'nep141:wrap.near',
          quoteAsset: 'nep141:eth-0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.omft.near',
          quantity: '1000000000000000000000000',
          side: 'SELL',
          price: '5',
          depositType: 'ORIGIN_CHAIN',
          refundTo: 'your-account.near',
          refundType: 'ORIGIN_CHAIN',
          recipient: '0xYourEvmAddress',
          recipientType: 'DESTINATION_CHAIN',
          confidentiality: 'basic'
        }
      }
    })
  });
  const order = await created.json();
  const orderId = order.data.id;
  const { depositAddress, depositMemo, swapView } = order.data.attributes;
  ```
</CodeGroup>

| Response         | Next step                                            |
| ---------------- | ---------------------------------------------------- |
| `id`             | Poll and cancel                                      |
| `depositAddress` | Send the deposit here                                |
| `swapView`       | How much to send                                     |
| `depositMemo`    | Include it on the transfer when the response has one |

Send `swapView.amountIn` on a SELL, or `swapView.maxAmountIn` on a BUY, to `depositAddress`. You can optionally notify 1Click with [`POST /v0/deposit/submit`](/api-reference/oneclick/submit-deposit-transaction-hash).

<Tip>
  `swapView.swapType` is `EXACT_INPUT` on a SELL and `EXACT_OUTPUT` on a BUY. See [Swap types](./swap-types).
</Tip>

The order stays open until it fills, you cancel it, or `deadline` (default 7 days). `timeInForce` is `GTC`.

| Optional      | Default                                                                   |
| ------------- | ------------------------------------------------------------------------- |
| `timeInForce` | `GTC` (only value)                                                        |
| `deadline`    | 7 days from creation                                                      |
| `appFees`     | None. If set, deducted from input. See [Fee configuration](./fee-config). |

***

## Watch the order

Poll until `isPayoutStatusFinal` is `true`. That's when filled output has been withdrawn and any unfilled amount refunded (`COMPLETED` or `FAILED`).

```bash theme={null}
curl "https://1click.chaindefuser.com/v0/orders/ORDER_ID" \
  -H "X-API-Key: YOUR_JWT_TOKEN"
```

While you wait, `fillStatus` is the matching state:

| `fillStatus`       | Meaning                                       |
| ------------------ | --------------------------------------------- |
| `AWAITING_DEPOSIT` | Waiting for the deposit                       |
| `OPEN`             | Funded, resting                               |
| `PARTIALLY_FILLED` | Some quantity filled, rest still working      |
| `FILLED`           | Fully matched                                 |
| `PENDING_CANCEL`   | Cancel requested; a last slice can still fill |
| `CANCELED`         | Canceled                                      |
| `EXPIRED`          | Deadline passed                               |

When payout is final, `payouts` has the `withdrawal` and `refund` legs (`txHash` on completed legs). `partialFills` are fills that already executed. `depositedAmount` is how much was funded.

Once the order is in flight, extra deposits are not applied to it. They come back separately from the unfilled refund (`payouts.refund`).

***

## Cancel

```bash theme={null}
curl -X POST "https://1click.chaindefuser.com/v0/orders/ORDER_ID/cancel" \
  -H "X-API-Key: YOUR_JWT_TOKEN"
```

Cancel is asynchronous. Matching stops; filled output is still withdrawn and the unfilled remainder is refunded. A last slice can still fill, so the order may end `FILLED` instead of `CANCELED`.

***

## List

`GET /v0/orders` returns your orders, newest first. Page with `page[after]` and `page[size]` (1–50, default 50). `links.next` is `null` on the last page.

| Query                         | Purpose                                       |
| ----------------------------- | --------------------------------------------- |
| `filter[fillStatus]`          | One or more fill statuses (repeat the param). |
| `filter[isPayoutStatusFinal]` | `true` when payout has finished.              |
| `page[after]`                 | Cursor from the previous page.                |
| `page[size]`                  | Page length, max 50.                          |

```bash theme={null}
curl "https://1click.chaindefuser.com/v0/orders?filter[isPayoutStatusFinal]=false&page[size]=20" \
  -H "X-API-Key: YOUR_JWT_TOKEN"
```

***

## If the user already holds an Intents balance

Use this when the payment already sits in Intents or Confidential Intents — an in-app balance, for example [near.com](https://near.com).

On create, set `depositType`, `refundType`, and `recipientType` to `INTENTS` or `CONFIDENTIAL_INTENTS`. Then fund the order with [signed intent execution](./quickstart/signed-intent-execution): generate an intent against the order's `depositAddress`, have the user sign it, and submit it.

```bash theme={null}
curl -X POST https://1click.chaindefuser.com/v0/orders \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer USER_ACCESS_TOKEN" \
  -d '{
    "data": {
      "type": "orders",
      "attributes": {
        "orderType": "LIMIT",
        "baseAsset": "nep141:wrap.near",
        "quoteAsset": "nep141:eth-0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.omft.near",
        "quantity": "1000000000000000000000000",
        "side": "SELL",
        "price": "5",
        "depositType": "CONFIDENTIAL_INTENTS",
        "refundTo": "user.near",
        "refundType": "CONFIDENTIAL_INTENTS",
        "recipient": "user.near",
        "recipientType": "CONFIDENTIAL_INTENTS",
        "confidentiality": "basic"
      }
    }
  }'
```

| Step                                          | Who                                                |
| --------------------------------------------- | -------------------------------------------------- |
| Generate against the order's `depositAddress` | `X-API-Key` with a [partner JWT](./authentication) |
| Sign                                          | User                                               |
| Submit                                        | `X-API-Key` with a [partner JWT](./authentication) |

Each generate call covers the remaining unfunded amount.

***

## Fees

1Click platform fees apply as for other flows ([Fee configuration](./fee-config), [Fees](/resources/fees)). Optional `appFees` on create are deducted from input.

***

## If it fails

Errors come back as `errors[]` with a stable `code`. Use `code` when you handle them. Include `meta.correlationId` if you report a problem.

| Situation                                                        | `code`                |
| ---------------------------------------------------------------- | --------------------- |
| `confidentiality` was `public`, or a field was invalid           | `validation-failed`   |
| Amount below the USD minimum, or create/cancel could not proceed | `order-rejected`      |
| Unknown order, or a different key created it                     | `order-not-found`     |
| Too many requests                                                | `rate-limit-exceeded` |

***

## Related

<CardGroup cols={2}>
  <Card title="Making a request" icon="paper-plane" href="./quickstart/making-a-request">
    Quote, deposit, and status for 1Click swaps
  </Card>

  <Card title="Confidential Swaps" icon="user-secret" href="./quickstart/confidential-swaps">
    `confidentiality` and `CONFIDENTIAL_INTENTS`
  </Card>

  <Card title="Signed Intent Execution" icon="signature" href="./quickstart/signed-intent-execution">
    Fund Intents deposits without an on-chain transfer
  </Card>

  <Card title="Fee configuration" icon="wallet" href="./fee-config">
    `appFees` on create
  </Card>
</CardGroup>

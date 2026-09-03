> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Confidential Swaps

> Add privacy to a swap with the confidentiality parameter or an embedded Confidential Intents balance

Confidential Intents has **two integration paths**. Most partners only need the first.

<CardGroup cols={2}>
  <Card title="Foreign-to-foreign (most partners)" icon="globe">
    Run a normal `ORIGIN_CHAIN` → `DESTINATION_CHAIN` swap and set the `confidentiality` parameter to `basic` or `advanced`. You do **not** need the `CONFIDENTIAL_INTENTS` type fields or signed-intent execution — the user deposits and receives on external chains exactly as a standard swap.
  </Card>

  <Card title="Embedded account / wallet" icon="wallet">
    When the funds already live inside a user's Confidential Intents balance, set `depositType`, `recipientType`, and/or `refundType` to `CONFIDENTIAL_INTENTS` and authorize the swap with [Signed Intent Execution](./signed-intent-execution). This is the advanced, wallet-style use case — integrators that keep user balances in Intents, for example [near.com](https://near.com).
  </Card>
</CardGroup>

To request a foreign-to-foreign confidential quote, add `confidentiality` to a standard quote request. Everything else in the request/response cycle is identical to [Making a Request](./making-a-request):

```json theme={null}
{
  "dry": false,
  "swapType": "EXACT_INPUT",
  "originAsset": "nep141:wrap.near",
  "depositType": "ORIGIN_CHAIN",
  "destinationAsset": "nep141:arb-0x912ce59144191c1204e64559fe8253a0e49e6548.omft.near",
  "amount": "100000000000000000000000",
  "recipient": "0xYourArbitrumAddress",
  "recipientType": "DESTINATION_CHAIN",
  "refundTo": "your-account.near",
  "refundType": "ORIGIN_CHAIN",
  "confidentiality": "basic",
  "deadline": "2025-01-01T00:00:00.000Z"
}
```

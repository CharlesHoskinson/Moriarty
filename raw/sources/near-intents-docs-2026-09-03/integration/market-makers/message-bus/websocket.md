> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# WebSocket Reference

> Subscribe and respond to quote requests in real-time

NEAR Intents exposes a WebSocket endpoint for market makers (solvers) to receive and respond to quote requests in real-time.

**Endpoint:** `wss://solver-relay-v2.chaindefuser.com/ws`

## Authentication

This endpoint requires a Partner JWT, sent as an `Authorization: Bearer` header on the WebSocket handshake request (not `X-API-Key`, that header is only for the [REST RPC endpoint](/integration/market-makers/message-bus/rpc#authentication)):

```
Authorization: Bearer <your_partner_jwt>
```

To obtain a Partner JWT, register through the [Partner Portal](https://partners.near-intents.org).

***

## subscribe

Subscribe to quote requests or quote status events.

<ParamField body="params[0]" type="string" required>
  Subscription name: `"quote"` or `"quote_status"`
</ParamField>

<ParamField body="params[1]" type="object">
  Optional quote filters. Second positional param. Omit, `null`, or `{}` = no filter. Set fields are ANDed; empty array on a field = no restriction for that field.

  Put the `defuse_asset_identifier` values for the tokens you support in `tokens_in` and `tokens_out` (same IDs as on quote events).
</ParamField>

| Field        | Type       | Keep the quote if                                            |
| ------------ | ---------- | ------------------------------------------------------------ |
| `tokens_in`  | `string[]` | input token (`defuse_asset_identifier_in`) is in this list   |
| `tokens_out` | `string[]` | output token (`defuse_asset_identifier_out`) is in this list |

Also optional: `params[2]` `guaranteed` (see [Guaranteed Delivery](/integration/market-makers/message-bus/guaranteed-delivery)).

<Info>
  For `"quote_status"` with [guaranteed delivery](/integration/market-makers/message-bus/guaranteed-delivery), leave filters empty (`null` or `{}`).
</Info>

<Accordion title="Example request">
  Native NEAR pair:

  ```json theme={null}
  {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "subscribe",
    "params": [
      "quote",
      {
        "tokens_in": ["nep141:usdt.tether-token.near", "nep141:wrap.near"],
        "tokens_out": ["nep141:usdt.tether-token.near", "nep141:wrap.near"]
      }
    ]
  }
  ```

  Cross-chain assets (still `nep141:` IDs on the bus):

  ```json theme={null}
  {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "subscribe",
    "params": [
      "quote",
      {
        "tokens_in": [
          "nep141:eth-0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.omft.near",
          "nep141:eth-0xdac17f958d2ee523a2206206994597c13d831ec7.omft.near"
        ],
        "tokens_out": [
          "nep141:eth-0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.omft.near",
          "nep141:eth-0xdac17f958d2ee523a2206206994597c13d831ec7.omft.near"
        ]
      }
    ]
  }
  ```
</Accordion>

<Accordion title="Example response">
  ```json theme={null}
  {
    "jsonrpc": "2.0",
    "id": 1,
    "result": "00000000-0000-0000-0000-000000000000"
  }
  ```

  Response fields:

  * `result` - Subscription ID used for receiving events and unsubscribing
</Accordion>

### `quote` events

After subscribing to `"quote"`, you'll receive events like this:

```json Event theme={null}
{
  "jsonrpc": "2.0",
  "method": "subscribe",
  "params": {
    "subscription": "00000000-0000-0000-0000-000000000000",
    "quote_id": "00000000-0000-0000-0000-000000000000",
    "defuse_asset_identifier_in": "nep141:ft1.near",
    "defuse_asset_identifier_out": "nep141:ft2.near",
    "exact_amount_in": "1000",
    "min_deadline_ms": 60000,
    "min_wait_ms": 1900,
    "max_wait_ms": 3000,
    "protocol_fee_included": false
  }
}
```

<Info>
  Only one of `exact_amount_in` or `exact_amount_out` will be specified in each request.
</Info>

Event fields:

* `quote_id` — Identifier for this auction; include it in your `quote_response`
* `defuse_asset_identifier_in` / `defuse_asset_identifier_out` — Token pair
* `exact_amount_in` or `exact_amount_out` — Fixed side of the trade
* `min_deadline_ms` — Minimum validity time your signed offer should support
* `min_wait_ms` / `max_wait_ms` — Auction collection window (see [Auction timing](#auction-timing-min_wait_ms--max_wait_ms))
* `protocol_fee_included` — When `true`, this is typically a router sub-leg auction with a shorter accept window

### Auction timing (`min_wait_ms` / `max_wait_ms`)

Each quote request opens a short auction on the relay. **You do not set these fields when responding** — they are chosen by the quote requester (or by the router for multi-hop sub-legs) and appear on the inbound event.

| Field         | Meaning                                                                                      |
| ------------- | -------------------------------------------------------------------------------------------- |
| `min_wait_ms` | Mandatory collection window. The relay will not close the auction early before this elapses. |
| `max_wait_ms` | Hard deadline. The auction always ends by then, even if no early close happened.             |

**Timeline**

1. The auction starts when the relay publishes the quote request.
2. From `0 → min_wait_ms`, the relay collects responses and does not settle early.
3. After `min_wait_ms`, if any responses have arrived, the relay may close after a short **grace period** (or immediately for certain whitelisted solvers).
4. At `max_wait_ms`, the auction always closes.
5. Once closed, the quote is removed. Late `quote_response` calls fail with `quote not found or already finished`.

**Effective windows solvers see**

| Request kind      | How to recognize it                                               | Effective collect window                                                                                                             |
| ----------------- | ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| Top-level auction | `protocol_fee_included` absent or `false`                         | `min_wait_ms` is floored to **at least 300ms**, then up to **\~300ms** grace if results exist                                        |
| Router sub-leg    | `protocol_fee_included: true` (often `partner_id: router-solver`) | **No 300ms floor.** Grace is **\~50ms**. FastQuote legs often use `min_wait_ms: 1`, so the accept window is roughly **\~51ms** total |

`max_wait_ms` is still the upper bound in both cases. When responses arrive quickly, the auction often ends around `min_wait_ms + grace`, not only at `max_wait_ms`.

<Warning>
  Router FastQuote legs (`protocol_fee_included: true`, `min_wait_ms: 1`) require end-to-end response latency well under \~50ms. A \~50ms WebSocket round trip is enough to miss the window even if your handler itself finishes in under 1ms.
</Warning>

### `quote_status` events

After subscribing to `"quote_status"`, you'll receive settlement notifications:

```json Event theme={null}
{
  "jsonrpc": "2.0",
  "method": "subscribe",
  "params": {
    "quote_hash": "00000000000000000000000000000000",
    "intent_hash": "00000000000000000000000000000000",
    "tx_hash": "8yFNEk7GmRcM3NMJihwCKXt8ZANLpL2koVFWWH1MEEj"
  }
}
```

<Info>
  `quote_status` events published while your connection is down are lost by default. To recover missed events after a disconnect, opt into [Guaranteed Delivery](/integration/market-makers/message-bus/guaranteed-delivery).
</Info>

***

## quote\_response

Respond to a quote request with a signed intent. A unique `id` is required in the JSON-RPC message.

<Info>
  `signed_data` is produced by signing your `token_diff` intent with the NEAR key you registered on the Verifier contract, it's not something you construct by hand. See the [Example Solver](/integration/market-makers/example#assembly-and-signing) for the real signing code, and [publish\_intent](/integration/market-makers/message-bus/rpc#publish_intent) for the exact `signed_data` shape.
</Info>

<ParamField body="quote_id" type="string" required>
  Quote request identifier from the event
</ParamField>

<ParamField body="quote_output" type="object" required>
  * `amount_out` - Proposed amount for `exact_amount_in` requests
  * `amount_in` - Proposed amount for `exact_amount_out` requests
</ParamField>

<ParamField body="signed_data" type="object" required>
  Signed intent data (same format as `publish_intent`)
</ParamField>

<ParamField body="other_quote_hashes" type="string[]">
  Optional hashes of other quotes needed to fulfill this intent
</ParamField>

<Accordion title="Example request">
  ```json theme={null}
  {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "quote_response",
    "params": [
      {
        "quote_id": "00000000-0000-0000-0000-000000000000",
        "quote_output": {
          "amount_out": "300"
        },
        "signed_data": {
          "standard": "nep413",
          "payload": {
            "recipient": "intents.near",
            "nonce": "Vij2xgAlKBKzAEiS6N1S/hcQLaHzX2s1fNKhBDblXT4=",
            "message": "{\"deadline\":\"2024-10-14T12:53:40.000Z\",\"intents\":[{\"intent\":\"token_diff\",\"diff\":{\"nep141:ft2.near\":\"-300\",\"nep141:ft1.near\":\"500\"}}],\"signer_id\":\"solver.near\"}"
          },
          "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
          "signature": "ed25519:WQXG37prMyT4f1vp6JSvamsjqDR5fSnDiinSPaCoq9sPcDgFGPRiMWX7csqqudDbzc8i6wrfpemgpVX2wQDmwww"
        }
      }
    ]
  }
  ```
</Accordion>

<Accordion title="Example response">
  ```json theme={null}
  {
    "jsonrpc": "2.0",
    "id": 1,
    "result": "OK"
  }
  ```

  Response fields:

  * `result` - `"OK"` when the quote response is accepted
</Accordion>

***

## unsubscribe

Unsubscribe from a subscription.

<ParamField body="params[0]" type="string" required>
  Subscription ID returned by `subscribe`
</ParamField>

<Accordion title="Example request">
  ```json theme={null}
  {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "unsubscribe",
    "params": ["00000000-0000-0000-0000-000000000000"]
  }
  ```
</Accordion>

<Accordion title="Example response">
  ```json theme={null}
  {
    "jsonrpc": "2.0",
    "id": 1,
    "result": "OK"
  }
  ```

  Response fields:

  * `result` - `"OK"` when unsubscribed successfully
</Accordion>

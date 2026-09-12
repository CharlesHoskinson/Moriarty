> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# API Reference

> Request quotes and publish intents via JSON-RPC

NEAR Intents exposes an RPC endpoint to request quotes, publish signed intents, and check intent status on the Message Bus.

**Endpoint:** `POST https://solver-relay-v2.chaindefuser.com/rpc`

## Authentication

This endpoint requires a JWT authentication token. Include the token in the `X-API-Key` header:

```bash theme={null}
X-API-Key: <your_jwt_token>
```

To obtain an API key, register through the [Partner Portal](https://partners.near-intents.org).

<Warning>
  Developers searching to integrate swap functionality should use the [1Click Swap API](../../distribution-channels/1click-api/about-1click-api) instead.
</Warning>

***

## quote

Request price quotes from connected solvers. The Message Bus forwards the request to all solvers, waits up to 3000ms, and returns all available options.

<Info>
  Only one of `exact_amount_in` or `exact_amount_out` should be provided, not both.
</Info>

<ParamField body="defuse_asset_identifier_in" type="string" required>
  Asset to trade from (e.g., `nep141:ft1.near`)
</ParamField>

<ParamField body="defuse_asset_identifier_out" type="string" required>
  Asset to trade to (e.g., `nep141:ft2.near`)
</ParamField>

<ParamField body="exact_amount_in" type="string">
  Amount of input token for exchange
</ParamField>

<ParamField body="exact_amount_out" type="string">
  Amount of output token for exchange
</ParamField>

<ParamField body="min_deadline_ms" type="number" default="60000">
  Minimum validity time for offers (in milliseconds). Shorter times may yield better prices.
</ParamField>

<Accordion title="Example request">
  ```json theme={null}
  {
    "id": 1,
    "jsonrpc": "2.0",
    "method": "quote",
    "params": [
      {
        "defuse_asset_identifier_in": "nep141:ft1.near",
        "defuse_asset_identifier_out": "nep141:ft2.near",
        "exact_amount_in": "1000",
        "min_deadline_ms": 60000
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
    "result": [
      {
        "quote_hash": "00000000000000000000000000000000",
        "defuse_asset_identifier_in": "nep141:ft1.near",
        "defuse_asset_identifier_out": "nep141:ft2.near",
        "amount_in": "1000",
        "amount_out": "2000",
        "expiration_time": "2024-10-01T12:10:27Z"
      }
    ]
  }
  ```

  Response fields:

  * `quote_hash` - Quote response hash
  * `defuse_asset_identifier_in` - Asset to trade from
  * `defuse_asset_identifier_out` - Asset to trade to
  * `amount_in` - Input amount (exact if specified, proposed otherwise)
  * `amount_out` - Output amount (exact if specified, proposed otherwise)
  * `expiration_time` - Expiration timestamp of the offer
</Accordion>

***

## publish\_intent

Submit a signed user intent for execution. Supported signature standards: `nep413`, `erc191`, `raw_ed25519`.

<Info>
  `public_key` and `signature` come from signing the intent message with the NEAR key you registered on the Verifier contract (`add_public_key`, see the [Quickstart](/integration/market-makers/quickstart)'s "Deposit liquidity" step), not from a value you construct by hand. The [Example Solver](/integration/market-makers/example#assembly-and-signing) walks through the real signing code (`near-api-js`'s `signMessage`, then bs58-encoding the result with an `ed25519:` prefix). For the full field-by-field spec of every supported standard (including `erc191` and `raw_ed25519` below), see [Signing Intents](/integration/verifier-contract/signing-intents).
</Info>

<Tabs>
  <Tab title="NEP-413">
    <ParamField body="quote_hashes" type="string[]" required>
      Quote response hashes from solvers
    </ParamField>

    <ParamField body="signed_data" type="object" required>
      * `standard` - `"nep413"`
      * `payload.message` - Stringified intent payload
      * `payload.nonce` - Unique nonce
      * `payload.recipient` - `"intents.near"`
      * `payload.callbackUrl` - Optional, for some wallets
      * `signature` - Signature of the payload
      * `public_key` - Signer's public key
    </ParamField>

    <Accordion title="Example request">
      ```json theme={null}
      {
        "id": 1,
        "jsonrpc": "2.0",
        "method": "publish_intent",
        "params": [
          {
            "quote_hashes": ["00000000000000000000000000000000"],
            "signed_data": {
              "standard": "nep413",
              "payload": {
                "recipient": "intents.near",
                "nonce": "Vij2xgAlKBKzAEiS6N1S/hfrNi8/We0ieTmcMBti1YE=",
                "message": "{\"deadline\":\"2024-10-14T12:53:40.000Z\",\"intents\":[{\"intent\":\"token_diff\",\"diff\":{\"nep141:ft1.near\":\"300\",\"nep141:ft2.near\":\"-500\"}}],\"signer_id\":\"user.near\"}"
              },
              "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
              "signature": "ed25519:5tk3UyFcAgnd6D4ZAuzEdZqMrneRSiTqe48ptjbjYHwiCy2vTw38uDB3KusW2cEsF3TGcqZXoQmRaeNs2erhPpqu"
            }
          }
        ]
      }
      ```
    </Accordion>
  </Tab>

  <Tab title="ERC-191">
    <ParamField body="quote_hashes" type="string[]" required>
      Quote response hashes from solvers
    </ParamField>

    <ParamField body="signed_data" type="object" required>
      * `standard` - `"erc191"`
      * `payload` - Stringified intent payload
      * `signature` - Signature of the payload
    </ParamField>

    <Accordion title="Example request">
      ```json theme={null}
      {
        "id": 1,
        "jsonrpc": "2.0",
        "method": "publish_intent",
        "params": [
          {
            "quote_hashes": ["00000000000000000000000000000000"],
            "signed_data": {
              "standard": "erc191",
              "payload": "{\"deadline\":\"2024-10-14T12:53:40.000Z\",\"intents\":[{\"intent\":\"token_diff\",\"diff\":{\"nep141:ft1.near\":\"300\",\"nep141:ft2.near\":\"-500\"}}],\"signer_id\":\"0x1dda60d784483fbb54304c68830d42a706327c6d\"}",
              "signature": "0x8a1b7e...c1b"
            }
          }
        ]
      }
      ```
    </Accordion>
  </Tab>

  <Tab title="Raw Ed25519">
    <ParamField body="quote_hashes" type="string[]" required>
      Quote response hashes from solvers
    </ParamField>

    <ParamField body="signed_data" type="object" required>
      * `standard` - `"raw_ed25519"`
      * `payload` - Stringified intent payload
      * `signature` - Signature of the payload
      * `public_key` - Signer's public key
    </ParamField>

    <Accordion title="Example request">
      ```json theme={null}
      {
        "id": 1,
        "jsonrpc": "2.0",
        "method": "publish_intent",
        "params": [
          {
            "quote_hashes": ["00000000000000000000000000000000"],
            "signed_data": {
              "standard": "raw_ed25519",
              "payload": "{\"deadline\":\"2024-10-14T12:53:40.000Z\",\"intents\":[{\"intent\":\"token_diff\",\"diff\":{\"nep141:ft1.near\":\"300\",\"nep141:ft2.near\":\"-500\"}}],\"signer_id\":\"user.near\"}",
              "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
              "signature": "ed25519:5tk3UyFcAgnd6D4ZAuzEdZqMrneRSiTqe48ptjbjYHwiCy2vTw38uDB3KusW2cEsF3TGcqZXoQmRaeNs2erhPpqu"
            }
          }
        ]
      }
      ```
    </Accordion>
  </Tab>
</Tabs>

<Accordion title="Example response">
  ```json theme={null}
  {
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
      "status": "OK",
      "intent_hash": "00000000000000000000000000000000"
    }
  }
  ```

  Response fields:

  * `status` - `"OK"` or `"FAILED"`
  * `reason` - Error reason (if failed)
  * `intent_hash` - Intent identifier
</Accordion>

***

## publish\_intents

Submit multiple signed intents in one call. If any fail, the relay automatically requotes and retries the failed ones rather than failing the whole batch.

<ParamField body="quote_hashes" type="string[]" required>
  Quote response hashes from solvers, one set per intent being published
</ParamField>

<ParamField body="signed_datas" type="object[]" required>
  Array of signed intents, same shape as `signed_data` in [publish\_intent](#publish_intent)
</ParamField>

<ParamField body="requote" type="boolean" default="false">
  When `true`, automatically requests a fresh quote and retries any intent that fails to publish, instead of just reporting the failure
</ParamField>

<Accordion title="Example request">
  ```json theme={null}
  {
    "id": 1,
    "jsonrpc": "2.0",
    "method": "publish_intents",
    "params": [
      {
        "quote_hashes": ["00000000000000000000000000000000"],
        "signed_datas": [
          {
            "standard": "nep413",
            "payload": {
              "recipient": "intents.near",
              "nonce": "Vij2xgAlKBKzAEiS6N1S/hfrNi8/We0ieTmcMBti1YE=",
              "message": "{\"deadline\":\"2024-10-14T12:53:40.000Z\",\"intents\":[{\"intent\":\"token_diff\",\"diff\":{\"nep141:ft1.near\":\"300\",\"nep141:ft2.near\":\"-500\"}}],\"signer_id\":\"user.near\"}"
            },
            "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
            "signature": "ed25519:5tk3UyFcAgnd6D4ZAuzEdZqMrneRSiTqe48ptjbjYHwiCy2vTw38uDB3KusW2cEsF3TGcqZXoQmRaeNs2erhPpqu"
          }
        ],
        "requote": true
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
    "result": {
      "status": "OK",
      "intent_hash": "00000000000000000000000000000000"
    }
  }
  ```

  Response fields are the same as [publish\_intent](#publish_intent).
</Accordion>

***

## get\_status

Check the status of an intent's execution.

<ParamField body="intent_hash" type="string" required>
  Intent identifier
</ParamField>

<Accordion title="Example request">
  ```json theme={null}
  {
    "id": 1,
    "jsonrpc": "2.0",
    "method": "get_status",
    "params": [
      {
        "intent_hash": "00000000000000000000000000000000"
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
    "result": {
      "intent_hash": "00000000000000000000000000000000",
      "status": "SETTLED",
      "status_details": "Settled in block 123456789",
      "data": {
        "hash": "8yFNEk7GmRcM3NMJihwCKXt8ZANLpL2koVFWWH1MEEj"
      },
      "filled_amounts": ["300"]
    }
  }
  ```

  Response fields:

  * `intent_hash` - Intent identifier
  * `status` - Execution status (see status values below)
  * `status_details` - Human-readable detail on the current status (e.g. failure reason for `NOT_FOUND_OR_NOT_VALID`)
  * `data.hash` - NEAR transaction hash (if available)
  * `filled_amounts` - Amounts actually filled per leg, in the order the intent's diffs were specified (useful when a `token_diff` only partially fills)

  | Status                   | Description                                      |
  | ------------------------ | ------------------------------------------------ |
  | `PENDING`                | Intent received, awaiting execution              |
  | `TX_BROADCASTED`         | Transaction sent to the Verifier contract        |
  | `SETTLED`                | Successfully settled on-chain                    |
  | `NOT_FOUND_OR_NOT_VALID` | Intent not received, expired, or execution error |
</Accordion>

> For the complete documentation index, see [llms.txt](https://hyperliquid.gitbook.io/hyperliquid-docs/llms.txt). Markdown versions of documentation pages are available by appending `.md` to page URLs; this page is available as [Markdown](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/websocket/post-requests.md).

# Post requests

This page describes posting requests using the WebSocket API.

### Request format

The WebSocket API supports posting requests that you can normally post through the HTTP API. These requests are either info requests or signed actions. For examples of info request payloads, please refer to the Info endpoint section. For examples of signed action payloads, please refer to the Exchange endpoint section.

To send such a payload for either type via the WebSocket API, you must wrap it as such:

```json
{
  "method": "post",
  "id": <number>,
  "request": {
    "type": "info" | "action",
    "payload": { ... }
  }
}
```

Note: The `method` and `id` fields are mandatory. It is recommended that you use a unique `id` for every post request you send in order to track outstanding requests through the channel.

Note: `explorer` requests are not supported via WebSocket.

### Response format

The server will respond to post requests with either a success or an error. For errors, a `String` is returned mirroring the HTTP status code and description that would have been returned if the request were sent through HTTP.

```json
{
  "channel": "post",
  "data": {
    "id": <number>,
    "response": {
      "type": "info" | "action" | "error",
      "payload": { ... }
    }
  }
}
```

### Examples

Here are a few examples of subscribing to different feeds using the subscription messages:

Sending an L2Book info request:

```json
{
  "method": "post",
  "id": 123,
  "request": {
    "type": "info",
    "payload": {
      "type": "l2Book",
      "coin": "ETH",
      "nSigFigs": 5,
      "mantissa": null
    }
  }
}
```

Sample response:

```json
{
  "channel": "post",
  "data": {
    "id": <number>,
    "response": {
      "type": "info",
      "payload": {
        "type": "l2Book",
        "data": {
          "coin": "ETH",
          "time": <number>,
          "levels": [
            [{"px":"3007.1","sz":"2.7954","n":1}],
            [{"px":"3040.1","sz":"3.9499","n":1}]
          ]
        }
      }
    }
  }
}
```

Sending an order signed action request:

```json
{
  "method": "post",
  "id": 256,
  "request": {
    "type": "action",
    "payload": {
      "action": {
        "type": "order",
        "orders": [{"a": 4, "b": true, "p": "1100", "s": "0.2", "r": false, "t": {"limit": {"tif": "Gtc"}}}],
        "grouping": "na"
      },
      "nonce": 1713825891591,
      "signature": {
        "r": "...",
        "s": "...",
        "v": "..."
      },
      "vaultAddress": "0x12...3"
    }
  }
}
```

Sample response:

```json
{
  "channel": "post",
  "data": {
    "id": 256,
    "response": {
      "type":"action",
      "payload": {
        "status": "ok",
        "response": {
          "type": "order",
          "data": {
            "statuses": [
              {
                "resting": {
                  "oid": 88383,
                }
              }
            ]
          }
        }
      }
    }
  }
}
```

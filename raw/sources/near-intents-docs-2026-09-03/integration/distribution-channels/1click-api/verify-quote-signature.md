> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Verify Quote Signatures

> Confirm quote and status payloads are signed by 1Click

1Click signs quote payloads so partners can verify the payload origin and detect tampering before using fields like `depositAddress` in 1click quote response payloads. This can help to

* Prevent man-in-the-middle tampering on transport or proxy layers.
* Ensure a quote/status payload was produced by the 1Click API.
* Detect unexpected field mutation before showing the quote or executing swaps.

## What is signed

The signed message is a Base58-encoded SHA-256 hash of a deterministic JSON object from the 1click quote payload response. The JSON is serialized with stable key ordering (`json-stable-stringify`), then hashed with SHA-256 and Base58-encoded.

The signed message input is:

```typescript theme={null}
stringify({ ...quoteRequest, ...quoteResponse, timestamp });
```

<Tip>
  Both dry and non-dry quotes can be verified to have come from 1Click. The difference is that non-dry quotes include extra execution fields (for example `depositAddress`) in the signed payload.
</Tip>

## Use the TypeScript SDK (recommended)

If you integrate with our TypeScript SDK, use `verifyQuoteSignature()` directly:

```typescript theme={null}
import { verifyQuoteSignature } from "@defuse-protocol/one-click-sdk-typescript";

const quoteResponse = await fetch("https://1click.chaindefuser.com/v0/quote", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify(quoteRequest),
}).then((r) => r.json());

const isValid = verifyQuoteSignature(quoteResponse);
if (!isValid) {
  throw new Error("Invalid quote signature");
}
```

<Warning>
  The `verifyQuoteSignature()` function requires `@defuse-protocol/one-click-sdk-typescript` version 0.1.24 or later.
</Warning>

Both include `signature` and `timestamp`, and both should be verified before consuming the quote payload.

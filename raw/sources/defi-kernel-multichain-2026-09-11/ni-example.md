> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Example Solver

> Understand how the AMM Solver example works

The [Quickstart](./quickstart) tutorial explains how to set up an [example AMM Solver](https://github.com/defuse-protocol/near-intents-amm-solver),
lets dive into understanding how it works: from connecting to the Message Bus through to settlement.

***

## Architecture

The project is organized into focused services, each handling one part of the workflow:

| Service              | File                                           | Responsibility                                                              |
| -------------------- | ---------------------------------------------- | --------------------------------------------------------------------------- |
| WebSocket connection | `src/services/websocket-connection.service.ts` | Connects to the Message Bus, subscribes to events, routes incoming messages |
| Quoter               | `src/services/quoter.service.ts`               | Evaluates quote requests, calculates pricing, builds and signs intents      |
| Cron                 | `src/services/cron.service.ts`                 | Refreshes token balances from the Verifier contract on a 15-second interval |

The WebSocket service receives a quote request, hands it to the quoter, and the quoter responds with a signed intent. Meanwhile, the cron service keeps balance data current so the quoter always knows what it can fill.

***

## Message Bus connection

The WebSocket connection service (`src/services/websocket-connection.service.ts`) manages the link to the Message Bus. On connect, it subscribes to `"quote"` and `"quote_status"`.

On `"quote"`, pass `tokens_in` / `tokens_out` for the configured pair. Filter shape: [WebSocket `subscribe`](/integration/market-makers/message-bus/websocket#subscribe).

```typescript theme={null}
import WebSocket from 'ws';

const tokens = [
  `nep141:${process.env.AMM_TOKEN1_ID}`,
  `nep141:${process.env.AMM_TOKEN2_ID}`,
];

const ws = new WebSocket("wss://solver-relay-v2.chaindefuser.com/ws", {
  headers: {
    Authorization: `Bearer ${process.env.PARTNER_JWT}`,
  },
});

ws.onopen = () => {
  // Subscribe to quote requests
  ws.send(JSON.stringify({
    jsonrpc: "2.0",
    id: 1,
    method: "subscribe",
    params: ["quote", { tokens_in: tokens, tokens_out: tokens }],
  }));

  // Subscribe to quote status updates
  ws.send(JSON.stringify({
    jsonrpc: "2.0",
    id: 2,
    method: "subscribe",
    params: ["quote_status"],
  }));
};
```

<Warning>
  The WebSocket endpoint requires a Partner JWT for authentication. See the [Quickstart](./quickstart) for how to obtain one.
</Warning>

***

## Processing requests

When a `quote` event arrives, the WebSocket service checks whether the requested token pair matches the solver's configured pair. If it does, the request is passed to the quoter service for evaluation.

Each quote request contains the following parameters:

```typescript theme={null}
interface IQuoteRequestData {
  quote_id: string;
  defuse_asset_identifier_in: string;
  defuse_asset_identifier_out: string;
  exact_amount_in?: string;
  exact_amount_out?: string;
  min_deadline_ms: number;
}
```

The quoter service (`src/services/quoter.service.ts`) handles the core decision-making. For each incoming request, it:

1. **Validates the deadline** — rejects requests with unreasonable timeframes
2. **Checks reserves** — looks up current balances for both tokens
3. **Calculates the price** — uses a constant-product AMM formula with the configured margin
4. **Signs the response** — creates a `token_diff` intent and signs it with the configured NEAR key

***

## Building and signing intents

Once the quoter has computed a price, it constructs a `token_diff` intent — a signed statement declaring which tokens the solver is willing to give and receive.

### Token diff structure

A `token_diff` intent expresses net token changes from the solver's perspective:

```typescript theme={null}
{
  intent: "token_diff",
  diff: {
    // Positive values = tokens you receive
    "nep141:usdc.near": "1000",
    // Negative values = tokens you give
    "nep141:usdt.near": "-1000",
  }
}
```

### Assembly and signing

The solver constructs the intent message directly and signs it using `near-api-js`. The message includes the solver's account ID, a deadline, and the `token_diff` intent:

```typescript theme={null}
import bs58 from 'bs58';

interface IMessage {
  signer_id: string;
  deadline: string;
  intents: { intent: 'token_diff'; diff: Record<string, string> }[];
}

const message: IMessage = {
  signer_id: nearService.getIntentsAccountId(),
  deadline: new Date(Date.now() + quoteDeadlineMs).toISOString(),
  intents: [
    {
      intent: 'token_diff',
      diff: {
        // Token you're receiving (positive = incoming)
        [params.defuse_asset_identifier_in]: params.exact_amount_in
          ? params.exact_amount_in
          : amount,
        // Token you're giving (negative = outgoing)
        [params.defuse_asset_identifier_out]: `-${
          params.exact_amount_out ? params.exact_amount_out : amount
        }`,
      },
    },
  ],
};
```

The message is then serialized and signed. The example solver uses the NEP-413 signing standard on NEAR:

```typescript theme={null}
const messageStr = JSON.stringify(message);
const nonce = currentState.nonce;
const recipient = intentsContract; // "intents.near"

// Serialize the intent using Borsh for signing
const quoteHash = serializeIntent(messageStr, recipient, nonce, 'nep413');

// Sign with the NEAR key
const signature = await nearService.signMessage(quoteHash);
```

<Accordion title="Generating a nonce">
  Every intent requires a [unique nonce](../verifier-contract/intent-types-and-execution#nonce-structure). The example solver uses a deterministic approach — it hashes the current token reserves with SHA-256, so the nonce only changes when the solver's balances change (i.e. after a trade settles):

  ```typescript theme={null}
  import { createHash } from 'crypto';

  const generateDeterministicNonce = (input: string): string => {
    const hash = createHash('sha256');
    hash.update(input);
    return hash.digest('base64');
  };

  // Called with the current reserves:
  const nonce = generateDeterministicNonce(
    `reserves:${reserves.join(':')}`
  );

  ```

  <Tip>
    In the example the nonce updates only after a trade settles, which results in a maximum throughput of roughly **1 swap per second**.

    Consider changing it for an incrementing counter, pending submission tracking, or multiple signing accounts to improve parallelism.
  </Tip>

  This avoids fetching the contract salt on every request. The nonce is recomputed whenever the cron service refreshes balances.

  <Info>
    Learn more about the nonce structure in the [Intent Types and Execution](/integration/verifier-contract/intent-types-and-execution#nonce-structure) docs.
  </Info>
</Accordion>

<Warning>
  Ensure your account has sufficient balance in the Verifier contract for the tokens you're offering. The intent will fail if you don't have enough tokens deposited.
</Warning>

***

## Quote response

After building and signing the intent, the solver sends a `quote_response` back through the WebSocket. The response includes the quote output (the calculated amount) and the full signed data:

```typescript theme={null}
const quoteResp: IQuoteResponseData = {
  quote_id: params.quote_id,
  quote_output: {
    amount_in: params.exact_amount_out ? amount : undefined,
    amount_out: params.exact_amount_in ? amount : undefined,
  },
  signed_data: {
    standard: 'nep413',
    payload: {
      message: messageStr,
      nonce,
      recipient,
    },
    signature: `ed25519:${bs58.encode(signature.signature)}`,
    public_key: `ed25519:${bs58.encode(signature.publicKey.data)}`,
  },
};
```

<Tip>
  Note that the response includes the `quote_id` from the incoming request — this is how the Message Bus links the quote to the original request.
</Tip>

The WebSocket service then sends this response to the relay:

```typescript theme={null}
await this.sendRequestToRelay(
  RelayMethod.QUOTE_RESPONSE,
  [quoteResp],
  logger
);
```

Under the hood, `sendRequestToRelay` wraps the response in a JSON-RPC message:

```typescript theme={null}
const request = {
  id: this.requestCounter++,
  jsonrpc: '2.0',
  method: 'quote_response',
  params: [quoteResp],
};
ws.send(JSON.stringify(request));
```

The `quote_output` field tells the Message Bus which side of the trade the solver is quoting. If the request specified `exact_amount_in`, the solver responds with `amount_out` (how much it will give). If the request specified `exact_amount_out`, it responds with `amount_in` (how much it wants to receive).

***

## Monitoring

### Settlements

The solver subscribes to `quote_status` events to learn when its quotes are selected and settled on-chain. The WebSocket service routes incoming messages based on the subscription type:

```typescript theme={null}
switch (subscription.eventKind) {
  case RelayEventKind.QUOTE:
    this.processQuote(req.params.data, req.params.metadata);
    break;
  case RelayEventKind.QUOTE_STATUS:
    this.processQuoteStatus(req.params.data);
    break;
}
```

When a `quote_status` event arrives, the solver checks whether the settled quote hash matches one of its own cached quotes. If it does, it triggers a balance refresh so that future quotes reflect the updated reserves:

```typescript theme={null}
private async processQuoteStatus(data: IPublishedQuoteData) {
  // data contains: { quote_hash, intent_hash, tx_hash }
  const quote = this.cacheService.get(data.quote_hash);

  if (!quote) {
    // Not one of our quotes, skip
    return;
  }

  // Our quote was settled — refresh balances
  await this.quoterService.updateCurrentState();
}
```

### Balances

A cron service (`src/services/cron.service.ts`) refreshes token balances from the Verifier contract every 15 seconds. It calls the `mt_batch_balance_of` method on the intents contract to get the solver's current reserves. After a successful trade, the settlement handler also triggers an immediate refresh. This ensures the quoter always has accurate reserve data when calculating prices.

***

## Customization

The example uses a constant-product AMM formula, but any pricing logic can be used. The quoter service is the place to start — replace the `getAmountOut` and `getAmountIn` functions with a custom strategy, whether that is pulling prices from external APIs, using order books, or applying custom spread models.

A few additional areas to customize:

* **Support more token pairs** — add additional token IDs in the configuration
* **Add position limits** — cap how much of a token the solver can allocate
* **Implement risk controls** — set minimum trade sizes, maximum exposure, or rate limits

<Tip>
  The `src/configs/` directory is a good starting point for customization. Each config file maps to a specific concern — tokens, margins, WebSocket URLs, and more.
</Tip>

For production deployments, consider running your solver in TEE (Trusted Execution Environment) mode, which provides additional security guarantees. See the [repository README](https://github.com/defuse-protocol/near-intents-amm-solver) for TEE setup instructions.

***

## Confidential Intents

The example solver also supports **Confidential Intents** — privacy-preserving swaps where balances and trade details are shielded from public view. If you're ready to solve for private liquidity, see the [Confidential Intents](./confidential-intents) guide to learn how to migrate.

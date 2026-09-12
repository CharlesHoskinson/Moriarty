> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Guaranteed Delivery

> Recover missed quote_status events after a disconnect

<Note>
  **Early access.** Guaranteed delivery is live in production, but no solver has exercised it yet. It should behave as described. If you see missing messages, duplicate floods, or subscribe errors, let us know in the shared support channels.
</Note>

Guaranteed delivery lets your solver recover `quote_status` events it would otherwise miss while disconnected.

The [`quote_status`](/integration/market-makers/message-bus/websocket#quote_status-events) stream notifies your solver when one of its quotes is executed, carrying `quote_hash`, `intent_hash`, and `tx_hash`. Without guaranteed delivery, any event published while your WebSocket is disconnected is lost.

With it, the relay keeps a server-side queue for each solver instance. Events wait while you are offline, then replay when you reconnect, for up to 7 days.

***

## Opt in

Guaranteed delivery requires an authenticated solver connection (JWT). Complete these three steps:

<Steps>
  <Step title="Add instance_id to the WebSocket URL">
    Connect with an `instance_id` query parameter:

    ```
    wss://solver-relay-v2.chaindefuser.com/ws?instance_id=<INSTANCE_ID>
    ```

    The relay finds your queue by this ID on every reconnect, and creates it the first time you subscribe with the guaranteed flag.

    Use a fixed value from your deploy config (`prod-1`, `prod-2`), not a random one generated at startup.

    IDs are scoped to your solver account, so two solvers using the same `instance_id` never collide. The relay gives each one a separate queue:

    ```
    QS_<partner_id>_<instance_id>
    ```

    * **Use the same value across restarts and reconnects.** A new value gives you a new, empty queue.
    * **One value per instance.** Two instances sharing an ID fight over one queue, and each message goes to only one of them, chosen arbitrarily.
    * **One live connection per ID.** Don't open two at once for redundancy.
  </Step>

  <Step title="Subscribe with the guaranteed flag">
    Open the WebSocket with your `instance_id`, then send a `subscribe` request for `quote_status`. Set the third positional parameter to `true`; the second parameter (filters) must be empty.

    ```typescript theme={null}
    import WebSocket from "ws";

    const ws = new WebSocket(
      "wss://solver-relay-v2.chaindefuser.com/ws?instance_id=<INSTANCE_ID>",
      { headers: { Authorization: `Bearer ${process.env.PARTNER_JWT}` } },
    );

    ws.on("open", () => {
      ws.send(
        JSON.stringify({
          jsonrpc: "2.0",
          id: 1,
          method: "subscribe",
          params: ["quote_status", null, true],
        }),
      );
    });
    ```

    The response returns your subscription ID:

    ```json theme={null}
    { "jsonrpc": "2.0", "id": 1, "result": "<subscription-id>" }
    ```

    The request is rejected without an authenticated Partner JWT and the `instance_id` from Step 1.

    <Info>
      If you already subscribe to `quote_status`, this replaces that call.
    </Info>
  </Step>

  <Step title="Handle and acknowledge every message">
    Messages arrive as `event` notifications carrying a `seq`:

    ```json theme={null}
    {
      "jsonrpc": "2.0",
      "method": "event",
      "params": {
        "subscription": "<subscription-id>",
        "data": { "quote_hash": "...", "intent_hash": "...", "tx_hash": "..." },
        "seq": 12345
      }
    }
    ```

    Handle each event by acknowledging it with your subscription ID and its `seq`, then running your settlement logic:

    ```typescript theme={null}
    // `seen` and `subscriptionId` live at module scope so they survive reconnects.
    // Bound `seen` in production (e.g. an LRU or a time window) so it does not grow forever.
    const seen = new Set<number>();
    let subscriptionId: string | undefined;

    function handleMessage(ws: WebSocket, raw: WebSocket.RawData) {
      const msg = JSON.parse(raw.toString());

      // The subscribe reply from Step 2 ({ "id": 1, "result": "<subscription-id>" }).
      // It echoes back our request id (1), so store its result as the subscription id.
      if (msg.result && msg.id === 1) {
        subscriptionId = msg.result;
        return;
      }

      // Event delivered for our subscription.
      if (msg.params?.subscription !== subscriptionId) return;
      const { seq, data } = msg.params;

      // 1. Acknowledge before doing any work.
      ws.send(
        JSON.stringify({
          jsonrpc: "2.0",
          id: Date.now(),
          method: "acknowledge",
          params: [subscriptionId, seq],
        }),
      );

      // 2. Skip duplicates (redeliveries can repeat a seq).
      if (seen.has(seq)) return;
      seen.add(seq);

      // 3. Now run your settlement logic.
      handleQuoteStatus(data);
    }

    // Wire it to the socket from Step 2:
    ws.on("message", (raw) => handleMessage(ws, raw));
    ```

    <Warning>
      **Acknowledge first, process second.** Redelivery fires after 5 seconds, which is too short for settlement logic. Persist or queue the message locally, acknowledge it, then do the work.
    </Warning>

    Deduplicate by `seq` or `quote_hash`. Redeliveries happen on reconnect, so you will sometimes receive the same message twice.
  </Step>
</Steps>

***

## After a reconnect

A subscription only lasts as long as the connection it was created on. When you reconnect, that old subscription is gone, so you have to send `subscribe` again. Doing so gives you a new subscription ID, and from that point on your acks must use the new ID, not the old one.

As soon as you re-subscribe, the relay replays your backlog in one burst: every message published while you were offline, plus any messages it had already sent you but that you hadn't acknowledged before the connection dropped.

Wrap the connection in a function so a drop reconnects and re-subscribes on its own. This reuses `handleMessage` from Step 3, and keeps `seen` across reconnects so the replayed backlog is still deduplicated:

```typescript theme={null}
function connect() {
  const ws = new WebSocket(
    "wss://solver-relay-v2.chaindefuser.com/ws?instance_id=<INSTANCE_ID>",
    { headers: { Authorization: `Bearer ${process.env.PARTNER_JWT}` } },
  );

  // Re-subscribe on every connect; a reconnect always needs a fresh subscribe (Step 2).
  ws.on("open", () => {
    ws.send(
      JSON.stringify({
        jsonrpc: "2.0",
        id: 1,
        method: "subscribe",
        params: ["quote_status", null, true],
      }),
    );
  });

  // Receive, acknowledge, and deduplicate each event (Step 3).
  ws.on("message", (raw) => handleMessage(ws, raw));

  // On drop, reconnect. The relay replays everything you missed once you re-subscribe.
  ws.on("close", () => setTimeout(connect, 1000));
}

connect();
```

***

## Limits

| Property                         | Value                                                               |
| -------------------------------- | ------------------------------------------------------------------- |
| Redelivery window (ack deadline) | 5 seconds                                                           |
| Max unacknowledged in flight     | 256 (delivery pauses past this until you catch up)                  |
| Retention / max offline          | Up to 7 days                                                        |
| Queue expiry                     | Dropped after 7 days idle                                           |
| First opt-in                     | Not retroactive; tracking starts at your first guaranteed subscribe |

***

## Errors

### Subscribe rejected

| Error                                                   | Cause                                             |
| ------------------------------------------------------- | ------------------------------------------------- |
| `guaranteed delivery needs instance_id to be specified` | No `instance_id` query parameter                  |
| `guaranteed delivery needs an authenticated partner`    | Connection isn't authenticated with a Partner JWT |
| `guaranteed delivery is only available on quote_status` | `guaranteed: true` set on another stream          |

### Acknowledge

| Error                                                          | Cause / action                                                                     |
| -------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| `no pending message with seq N`                                | Double-ack, or an ack from before a reconnect. Harmless; don't retry, don't alert. |
| `subscription '...' not found`                                 | Old subscription ID after reconnect. Re-subscribe and ack with the new one.        |
| `acknowledge is only supported for quote_status subscriptions` | The subscription ID belongs to a different stream                                  |

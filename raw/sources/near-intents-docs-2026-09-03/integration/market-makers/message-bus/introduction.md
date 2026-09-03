> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Overview

> Off-chain bus communicating market makers and users

The Message Bus is a system component that optimizes price discovery between users and market makers. It handles communication and submits settlement transactions to the Verifier contract.

<img className="block dark:hidden" src="https://mintcdn.com/defuselabsltd/OBaHx8_FZb8uK__Y/images/diagrams/msg-bus-light.png?fit=max&auto=format&n=OBaHx8_FZb8uK__Y&q=85&s=4a20b2baa19ee16e4fe9ac27cd91e86e" alt="Message Bus Architecture" width="1829" height="1632" data-path="images/diagrams/msg-bus-light.png" />

<img className="hidden dark:block" src="https://mintcdn.com/defuselabsltd/OBaHx8_FZb8uK__Y/images/diagrams/msg-bus-dark.png?fit=max&auto=format&n=OBaHx8_FZb8uK__Y&q=85&s=88770e8f07eec12de521fb5c043bcb36" alt="Message Bus Architecture" width="1829" height="1632" data-path="images/diagrams/msg-bus-dark.png" />

<Info>
  The protocol can operate without the Message Bus:

  * **Frontends** can use any quoting mechanism to compose and publish signed intents
  * **Market makers** can index the NEAR blockchain directly to find intents to fill
</Info>

***

## How it works

1. **User requests a quote** - A frontend sends a quote request to the Message Bus
2. **Market makers receive the request** - The Message Bus broadcasts the request to all connected solvers via WebSocket
3. **Market makers respond** - Solvers evaluate the trade and respond with signed intents
4. **User accepts a quote** - The frontend displays options to the user, who selects and signs their intent
5. **Settlement** - The Message Bus bundles the matching intents and submits them to the Verifier contract

***

## Next steps

<CardGroup cols={2}>
  <Card title="API Reference" icon="code" href="/integration/market-makers/message-bus/rpc">
    Read the API documentation for the Message Bus
  </Card>

  <Card title="WebSocket Reference" icon="code" href="/integration/market-makers/message-bus/websocket">
    Subscribe and respond to quote requests in real-time
  </Card>
</CardGroup>

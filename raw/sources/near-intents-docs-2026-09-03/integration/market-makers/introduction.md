> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Market Makers

> Fulfill cross-chain swap intents as a liquidity provider

Market Makers compete to fulfill user [Swap intents](../distribution-channels/introduction). They listen for swap requests on the [Message Bus](./message-bus/introduction), evaluate whether they can fill the request, and respond with signed quotes.

<img className="block dark:hidden" src="https://mintcdn.com/defuselabsltd/OBaHx8_FZb8uK__Y/images/diagrams/market-makers-intro-light.png?fit=max&auto=format&n=OBaHx8_FZb8uK__Y&q=85&s=2a907b1f71b967cc47d9accbcb049cf7" alt="NEAR Intents Market Makers" width="1516" height="1090" data-path="images/diagrams/market-makers-intro-light.png" />

<img className="hidden dark:block" src="https://mintcdn.com/defuselabsltd/OBaHx8_FZb8uK__Y/images/diagrams/market-makers-intro-dark.png?fit=max&auto=format&n=OBaHx8_FZb8uK__Y&q=85&s=b180131b3ec3c921a63edc30828c6ea3" alt="NEAR Intents Market Makers" width="1516" height="1090" data-path="images/diagrams/market-makers-intro-dark.png" />

***

## How it works

<Steps>
  <Step title="A Quote Request is sent">
    A user sends a [quote request](../distribution-channels/1click-api/quickstart/making-a-request#request-token) to the [Message Bus](/integration/market-makers/message-bus/introduction), a WebSocket relay that broadcasts the request to all connected solvers.
  </Step>

  <Step title="Solvers evaluate and quote">
    Each solver checks whether they can fulfill the swap. If they can, they compute pricing, and return a signed quote as response.
  </Step>

  <Step title="Quotes compete for selection">
    Multiple solvers can respond to the same request with different prices. The Message Bus collects responses and returns the top quotes to the user application.
  </Step>

  <Step title="Selected quote settles on-chain">
    After the user chooses a quote, the Message Bus matches the requested intent with the selected quote and submits it to the [Verifier contract](/integration/verifier-contract/introduction) where the swap settles on-chain.
  </Step>
</Steps>

<Info>
  The NEAR Intents protocol can operate without the Message Bus. Frontends can use other quoting mechanisms, and market makers can index the NEAR blockchain directly to find intents to fill.
</Info>

***

## Next steps

<CardGroup cols={2}>
  <Card title="Quickstart" icon="rocket" href="/integration/market-makers/quickstart">
    Set up and run a solver that automatically responds to quote requests
  </Card>

  <Card title="Usage Examples" icon="book-open" href="/integration/market-makers/example">
    Understand the code necessary to make a solver respond to quotes
  </Card>

  <Card title="Confidential Intents" icon="lock" href="/integration/market-makers/confidential-intents">
    Solve for private liquidity with shielded balances
  </Card>

  <Card title="Message Bus" icon="message" href="/integration/market-makers/message-bus/introduction">
    Learn about the Message Bus architecture and how it works
  </Card>
</CardGroup>

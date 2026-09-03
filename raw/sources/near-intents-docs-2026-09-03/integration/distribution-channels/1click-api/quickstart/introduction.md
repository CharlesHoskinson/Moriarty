> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Quickstart

> Execute your first intent-based swap using 1Click API

1Click API enables you to specify a cross-chain swap **intent** (**what** you want to swap), and handles the **execution** for you.

<Warning>
  There is no testnet version of NEAR Intents - use small amounts for test swaps.
</Warning>

***

## Prerequisites

* A small amount of tokens on ANY [supported chain](/resources/chain-support).
* A [JWT token](../authentication) to avoid the 0.2% fee.

***

## Choose your flow

<CardGroup cols={2}>
  <Card title="Making a Request" icon="paper-plane" href="./making-a-request">
    The standard flow: query tokens, request a quote, send your deposit, and track it to completion.
  </Card>

  <Card title="Confidential Swaps" icon="user-secret" href="./confidential-swaps">
    Add privacy to a swap, either foreign-to-foreign or from an embedded Confidential Intents balance.
  </Card>

  <Card title="Signed Intent Execution" icon="signature" href="./signed-intent-execution">
    Authorize a swap by signing an intent off-chain instead of sending an on-chain deposit.
  </Card>

  <Card title="Going Live" icon="flag-checkered" href="./going-live">
    Next steps and troubleshooting once your first swap works.
  </Card>
</CardGroup>

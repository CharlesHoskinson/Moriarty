> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# 1Click Swap API

> REST API for cross-chain swaps powered by NEAR Intents

1Click Swap simplifies NEAR Intents by temporarily transferring assets to a trusted swapping agent that coordinates with [Market Makers](/integration/market-makers/introduction) to execute your intent.

This REST API abstracts the complexity of intent creation, solver coordination, and transaction execution.

<Warning>
  By using the 1Click Swap API, you agree to the [1Click Terms of Service](/security-compliance/terms-of-service).
</Warning>

***

## Key Benefits

<CardGroup cols={1}>
  <Card title="Simple REST API" icon="code" horizontal>
    Create intents, submit deposits, and track status with a few endpoints.
  </Card>

  <Card title="Competitive Pricing" icon="chart-line" horizontal>
    Market makers compete on price through automatic solver discovery.
  </Card>

  <Card title="Built-in Transaction Handling" icon="arrows-rotate" horizontal>
    Includes status tracking, automatic retries, and refund handling.
  </Card>

  <Card title="Fee Collection" icon="wallet" horizontal>
    Configure fee collection with a single parameter in your quote requests.
  </Card>
</CardGroup>

***

## Ready to Dive In?

<CardGroup cols={2}>
  <Card title="Quickstart Guide" icon="rocket" href="./quickstart/introduction">
    Step-by-step tutorial to make your first swap in under 10 minutes
  </Card>

  <Card title="Keys" icon="key" href="./authentication">
    Learn how to generate API keys and authenticate your requests
  </Card>

  <Card title="SDK Libraries" icon="code" href="./sdk">
    Explore our SDK libraries for TypeScript, Go, and Rust
  </Card>

  <Card title="Fee Configuration" icon="wallet" href="./fee-config">
    Learn how to set up fee collection for your swaps
  </Card>

  <Card title="Earn" icon="chart-line" href="./earn">
    Offer multichain yield through the 1Click Swap API
  </Card>

  <Card title="Limit Orders" icon="clock" href="./orders">
    Offer a swap at the user's price through the 1Click Swap API
  </Card>

  <Card title="Explorer API" icon="magnifying-glass-chart" href="./explorer/introduction">
    Programmatic access to historical 1Click Swap transactions
  </Card>

  <Card title="Proactive Intents Security" icon="shield-halved" href="/security-compliance/proactive-intents-security">
    Learn how NEAR Intents proactively manages suspicious fund flows, transaction risk, and security posture across swaps
  </Card>
</CardGroup>

> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Going Live

> Next steps and troubleshooting for your first 1Click integration

## Next steps

<CardGroup cols={2}>
  <Card title="Clone the Example Repo" icon="github" href="https://github.com/near-examples/near-intents-examples">
    Clone and run a working TypeScript implementation to get started quickly.
  </Card>

  <Card title="Watch NEAR Intents 102" icon="youtube" href="https://www.youtube.com/watch?v=U8ngm2hR4a4">
    Follow along with this guided workshop explaining all the steps in detail.
  </Card>

  <Card title="Use the SDK" icon="window-restore" href="../sdk">
    Integrate with our official SDK libraries for TypeScript, Go, or Rust.
  </Card>

  <Card title="NEAR Intents Explorer" icon="magnifying-glass" href="https://explorer.near-intents.org">
    Track transactions and view swap history
  </Card>
</CardGroup>

## Troubleshooting

<AccordionGroup>
  <Accordion title="Swap taking a long time">
    * Check the blockchain explorer for your deposit transaction
    * Verify the deposit address matches the quote response
    * Allow up to 15 minutes for cross-chain processing
    * Use the status endpoint to check current state
  </Accordion>

  <Accordion title="Swap failed or was refunded">
    * Verify you sent the exact amount specified in the quote
    * Check your refund address for returned funds
    * Ensure your destination address format is correct for the target chain
    * Request a new quote and try again
  </Accordion>

  <Accordion title="Need help?">
    Join our [Telegram community](https://t.me/near_intents) for support.
  </Accordion>
</AccordionGroup>

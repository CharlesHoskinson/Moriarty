> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Making a Request

> Query tokens, request a quote, send your deposit, and track it to completion

This covers the standard `ORIGIN_CHAIN` swap flow end-to-end: find a token, request a quote, send your deposit, and poll status until it settles.

<Steps>
  <Step title="Query supported tokens">
    Fetch available tokens to find the `assetId` values you will need.

    <CodeGroup>
      ```bash cURL theme={null}
      curl https://1click.chaindefuser.com/v0/tokens
      ```

      ```typescript TypeScript theme={null}
      const response = await fetch('https://1click.chaindefuser.com/v0/tokens');
      const tokens = await response.json();
      ```
    </CodeGroup>

    The response includes tokens with their `assetId` in this format:

    * NEAR tokens: `nep141:wrap.near`
    * Bridged tokens: `nep141:eth-0xdac17f958d2ee523a2206206994597c13d831ec7.omft.near`

    <Accordion title="Example response">
      ```json theme={null}
      [
        {
          "assetId": "nep141:wrap.near",
          "decimals": 24,
          "blockchain": "near",
          "symbol": "wNEAR",
          "price": 1.1,
          "priceUpdatedAt": "2026-02-27T15:18:30.437Z",
          "contractAddress": "wrap.near"
        },
        {
          "assetId": "nep141:eth.omft.near",
          "decimals": 18,
          "blockchain": "eth",
          "symbol": "ETH",
          "price": 1947.28,
          "priceUpdatedAt": "2026-02-27T15:25:30.527Z",
          "contractAddress": null
        },
        {
          "assetId": "nep141:btc.omft.near",
          "decimals": 8,
          "blockchain": "btc",
          "symbol": "BTC",
          "price": 66093,
          "priceUpdatedAt": "2026-02-27T15:25:30.527Z",
          "contractAddress": null
        }
      ]
      ```
    </Accordion>
  </Step>

  <Step title="Request a quote">
    <span id="request-token" />

    Request a quote with your swap parameters. Include your [JWT token](../authentication) to avoid the 0.2% fee.

    <CodeGroup>
      ```bash cURL theme={null}
      curl -X POST https://1click.chaindefuser.com/v0/quote \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer YOUR_JWT_TOKEN" \
        -d '{
          "dry": false,
          "swapType": "EXACT_INPUT",
          "slippageTolerance": 100,
          "originAsset": "nep141:wrap.near",
          "depositType": "ORIGIN_CHAIN",
          "destinationAsset": "nep141:arb-0x912ce59144191c1204e64559fe8253a0e49e6548.omft.near",
          "amount": "100000000000000000000000",
          "recipient": "0xYourArbitrumAddress",
          "recipientType": "DESTINATION_CHAIN",
          "refundTo": "your-account.near",
          "refundType": "ORIGIN_CHAIN",
          "deadline": "2025-01-01T00:00:00.000Z"
        }'
      ```

      ```typescript TypeScript theme={null}
      const quote = await fetch('https://1click.chaindefuser.com/v0/quote', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': 'Bearer YOUR_JWT_TOKEN'
        },
        body: JSON.stringify({
          dry: false,
          swapType: 'EXACT_INPUT',
          slippageTolerance: 100,
          originAsset: 'nep141:wrap.near',
          depositType: 'ORIGIN_CHAIN',
          destinationAsset: 'nep141:arb-0x912ce59144191c1204e64559fe8253a0e49e6548.omft.near',
          amount: '100000000000000000000000',
          recipient: '0xYourArbitrumAddress',
          recipientType: 'DESTINATION_CHAIN',
          refundTo: 'your-account.near',
          refundType: 'ORIGIN_CHAIN',
          deadline: new Date(Date.now() + 3 * 60 * 1000).toISOString()
        })
      });
      const result = await quote.json();
      ```
    </CodeGroup>

    <Accordion title="Key parameters">
      | Parameter           | Description                                                                                                                                          |
      | ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
      | `dry`               | `true` to validate parameters and **get a quote without executing the swap**                                                                         |
      | `swapType`          | `EXACT_INPUT` (specify input amount) or `EXACT_OUTPUT` (specify output amount)                                                                       |
      | `slippageTolerance` | Maximum acceptable slippage in basis points (100 = 1%)                                                                                               |
      | `originAsset`       | Source token `assetId` from the tokens endpoint                                                                                                      |
      | `depositType`       | `ORIGIN_CHAIN` for origin-chain deposits, `INTENTS` for public Intents balances, or `CONFIDENTIAL_INTENTS`                                           |
      | `destinationAsset`  | Target token `assetId` from the tokens endpoint                                                                                                      |
      | `amount`            | Amount in smallest unit (wei, yoctoNEAR, etc.)                                                                                                       |
      | `recipient`         | Address to receive swapped tokens                                                                                                                    |
      | `recipientType`     | `DESTINATION_CHAIN`, `INTENTS`, or `CONFIDENTIAL_INTENTS`                                                                                            |
      | `refundTo`          | Address for refunds if swap fails                                                                                                                    |
      | `refundType`        | `ORIGIN_CHAIN`, `INTENTS`, or `CONFIDENTIAL_INTENTS`                                                                                                 |
      | `confidentiality`   | `public` (default), `basic`, or `advanced`. Enables a confidential swap on an otherwise normal quote, see [Confidential Swaps](./confidential-swaps) |
      | `deadline`          | Quote expiration timestamp in ISO format                                                                                                             |
    </Accordion>

    <Tip>
      This guide uses `EXACT_INPUT`, but 1Click also supports `EXACT_OUTPUT`, `FLEX_INPUT`, and `ANY_INPUT`. Check out [Swap Types](../swap-types) to see what each one does and when to use it.
    </Tip>
  </Step>

  <Step title="Send tokens">
    For `ORIGIN_CHAIN` quotes, transfer tokens to the `depositAddress` from the quote response. The swap begins automatically upon receipt.

    Save the deposit address and your transaction hash for tracking.

    If your quote uses `depositType: INTENTS` or `depositType: CONFIDENTIAL_INTENTS`, skip the on-chain transfer and use [Signed Intent Execution](./signed-intent-execution) instead.
  </Step>

  <Step title="Submit transaction hash (optional)">
    Speed up processing by notifying 1Click of your deposit.

    <CodeGroup>
      ```bash cURL theme={null}
      curl -X POST https://1click.chaindefuser.com/v0/deposit/submit \
        -H "Content-Type: application/json" \
        -d '{
          "depositAddress": "address-from-quote-response",
          "txHash": "0xYourTransactionHash"
        }'
      ```

      ```typescript TypeScript theme={null}
      await fetch('https://1click.chaindefuser.com/v0/deposit/submit', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          depositAddress: 'address-from-quote-response',
          txHash: '0xYourTransactionHash'
        })
      });
      ```
    </CodeGroup>
  </Step>

  <Step title="Monitor status">
    <span id="monitor-status" />

    Check swap progress using the deposit address.

    <CodeGroup>
      ```bash cURL theme={null}
      curl "https://1click.chaindefuser.com/v0/status?depositAddress=your-deposit-address"
      ```

      ```typescript TypeScript theme={null}
      const status = await fetch(
        'https://1click.chaindefuser.com/v0/status?depositAddress=your-deposit-address'
      );
      const result = await status.json();
      ```
    </CodeGroup>

    <Accordion title="Status response">
      | Status               | Description                                   |
      | -------------------- | --------------------------------------------- |
      | `PENDING_DEPOSIT`    | Awaiting your token deposit                   |
      | `KNOWN_DEPOSIT_TX`   | Deposit transaction detected                  |
      | `PROCESSING`         | Swap being executed                           |
      | `SUCCESS`            | Tokens delivered to destination address       |
      | `INCOMPLETE_DEPOSIT` | Deposit below required amount                 |
      | `REFUNDED`           | Swap failed, funds returned to refund address |
      | `FAILED`             | Swap encountered an error                     |
    </Accordion>

    <Callout color="#fb4d01">
      View detailed transaction info on the [NEAR Intents Explorer](https://explorer.near-intents.org) by searching for your deposit address.
    </Callout>
  </Step>

  <Step title="Celebrate! 🎉">
    You just completed your first intent-based cross-chain swap!
  </Step>
</Steps>

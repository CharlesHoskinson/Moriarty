> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Intents SDK

> Complete library to interact with NEAR Intents

The [NEAR Intents SDK](https://github.com/defuse-protocol/sdk-monorepo/tree/main/packages/intents-sdk) provides tools for intent execution, deposits, withdrawals, and interacting with various bridge implementations across multiple blockchains.

For a higher-level API focused on cross-chain swaps, check out the [1Click Swap API](/integration/distribution-channels/1click-api/about-1click-api) and its SDK libraries for TypeScript, Go, and Rust.

<Accordion title="Feature Status">
  | Feature          | Status | Description                                                            |
  | ---------------- | :----: | ---------------------------------------------------------------------- |
  | Intent Execution |    ✅   | Sign, submit, and track intent execution on NEAR Intents               |
  | Deposits         |    ❌   | Deposit funds to NEAR Intents (use bridge interfaces directly)         |
  | Withdrawals      |    ✅   | Complete withdrawal functionality from NEAR Intents to external chains |
</Accordion>

<Warning>
  The Intents SDK provides low-level functionality for interacting with NEAR Intents, check out the [1Click Swap API](/integration/distribution-channels/1click-api/about-1click-api) for a higher-level API focused on cross-chain swaps
</Warning>

***

## Using the SDK

<Steps>
  <Step title="Install the SDK">
    ```bash theme={null}
    npm install @defuse-protocol/intents-sdk --save-exact
    ```
  </Step>

  <Step title="Initialize the SDK">
    ```typescript theme={null}
    import { IntentsSDK, createIntentSignerNearKeyPair } from '@defuse-protocol/intents-sdk';
    import { KeyPair } from 'near-api-js';

    const sdk = new IntentsSDK({
      referral: 'your-referral-code',
      intentSigner: createIntentSignerNearKeyPair({
        signer: KeyPair.fromString('your-private-key'),
        accountId: 'your-account.near',
      }),
    });
    ```
  </Step>

  <Step title="Process a withdrawal">
    The most common use case — withdraw funds from NEAR Intents to an external chain:

    ```typescript theme={null}
    const result = await sdk.processWithdrawal({
      withdrawalParams: {
        assetId: 'nep141:usdt.tether-token.near',
        amount: 1000000n,                                       // 1 USDT (6 decimals)
        destinationAddress: '0x742d35Cc6634C0532925a3b8D84B2021F90a51A3',
        feeInclusive: false,
      },
    });

    console.log('Intent hash:', result.intentHash);
    console.log('Destination tx:', result.destinationTx);
    ```
  </Step>

  <Step title="Or execute a custom intent">
    For advanced use cases beyond withdrawals, use the lower-level `signAndSendIntent` method:

    ```typescript theme={null}
    const result = await sdk.signAndSendIntent({
      intents: [
        {
          intent: 'transfer',
          receiver_id: 'recipient.near',
          tokens: { 'usdt.tether-token.near': '1000000' },
        },
      ],
    });

    console.log('Intent hash:', result.intentHash);
    ```
  </Step>
</Steps>

<Tip>
  Use `processWithdrawal` for withdrawals and `signAndSendIntent` for custom intent logic. The withdrawal method handles fee estimation, validation, and completion tracking automatically.
</Tip>

***

## Next steps

Explore the [Github Repository](https://github.com/defuse-protocol/sdk-monorepo/tree/main/packages/intents-sdk) for the Intents SDK to learn about asset identifiers, intent signers, withdrawals, withdrawal routes, and intent management.

> For the complete documentation index, see [llms.txt](/llms.txt)

# Authentication

Midnight Network does not use traditional authentication methods. However, to submit transactions and interact with contracts, you must configure providers and sign transaction intents using your wallet address.

## Configure providers[​](#configure-providers "Direct link to Configure providers")

The [Midnight.js](/sdks/official/midnight-js.md) package enables you to deploy contracts and interact with them. To configure providers, you need to create a `MidnightProviders` object. This object defines the providers for submitting transactions and interacting with contracts.

```
const providers: MidnightProviders = {

  privateStateProvider: levelPrivateStateProvider({

    privateStoragePasswordProvider: () => password,

    accountId: walletAddress,

  }),

}
```

The `privateStateProvider` is responsible for storing and retrieving private state from the network. It requires the following properties:

* `privateStoragePasswordProvider`: A function that returns the password to the private state provider.
* `accountId`: The wallet address to sign transactions with.

note

The provider implementation above is for demonstration and intentionally omits some configuration. For the full provider configuration, see the [Midnight.js](/sdks/official/midnight-js.md) documentation.

## Submit transactions[​](#submit-transactions "Direct link to Submit transactions")

Any operation that modifies the state of the network requires a signature from your wallet address and a transaction. Each transaction cost gas which you must pay using DUST balance on your wallet.

DUST is the network resource that fuels transactions on the Midnight Network. Your wallet generates DUST from NIGHT tokens. You must delegate NIGHT tokens in your wallet to generate DUST.

If you don't have any DUST balance, then you cannot submit transactions or deploy contracts.

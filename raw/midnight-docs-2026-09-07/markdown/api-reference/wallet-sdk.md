> For the complete documentation index, see [llms.txt](/llms.txt)

# Wallet SDK API Reference

The Midnight Wallet SDK provides a comprehensive TypeScript library for managing wallets on the Midnight Network. It supports the three-token system that powers Midnight: unshielded tokens (NIGHT), shielded tokens with zero-knowledge proofs, and DUST for transaction fees.

For a detailed developer guide with setup instructions and walkthroughs, see the [Wallet SDK Developer Guide](/sdks/official/wallet-developer-guide.md).

## Architecture[​](#architecture "Direct link to Architecture")

The Wallet SDK uses a modular package architecture with a three-wallet model that maps to Midnight's token types.

```
wallet-sdk-facade          Unified entry point

├── wallet-sdk-unshielded-wallet  NIGHT and unshielded token operations

├── wallet-sdk-shielded           Shielded token operations with ZK proofs

├── wallet-sdk-dust-wallet        DUST management for transaction fees

├── wallet-sdk-hd                 HD key derivation (BIP 32 / BIP 44 / CIP 1852)

├── wallet-sdk-address-format     Bech32m address encoding and decoding

├── wallet-sdk-node-client        Node communication

├── wallet-sdk-indexer-client     Indexer queries

└── wallet-sdk-prover-client      Proof server interface
```

## Packages[​](#packages "Direct link to Packages")

| Package                                        | Purpose                                   |
| ---------------------------------------------- | ----------------------------------------- |
| `@midnight-ntwrk/wallet-sdk-facade`            | Unified API for all wallet operations     |
| `@midnight-ntwrk/wallet-sdk-unshielded-wallet` | Manages NIGHT and unshielded tokens       |
| `@midnight-ntwrk/wallet-sdk-shielded`          | Manages shielded tokens with ZK proofs    |
| `@midnight-ntwrk/wallet-sdk-dust-wallet`       | Manages DUST for transaction fees         |
| `@midnight-ntwrk/wallet-sdk-hd`                | Hierarchical deterministic key derivation |
| `@midnight-ntwrk/wallet-sdk-address-format`    | Bech32m address encoding and decoding     |
| `@midnight-ntwrk/wallet-sdk-node-client`       | Communicates with Midnight nodes          |
| `@midnight-ntwrk/wallet-sdk-indexer-client`    | Queries the Midnight indexer              |
| `@midnight-ntwrk/wallet-sdk-prover-client`     | Interfaces with the proving server        |

## Wallet Facade[​](#wallet-facade "Direct link to Wallet Facade")

The facade coordinates all three wallet types and provides a unified interface for balance queries, transfers, state management, and transaction execution.

### Initialize[​](#initialize "Direct link to Initialize")

```
import { type DefaultConfiguration } from '@midnight-ntwrk/wallet-sdk-facade';

import { InMemoryTransactionHistoryStorage } from '@midnight-ntwrk/wallet-sdk-unshielded-wallet';



const configuration: DefaultConfiguration = {

  networkId: 'preprod',

  costParameters: {

    feeBlocksMargin: 5,

  },

  relayURL: new URL('wss://rpc.preprod.midnight.network'),

  provingServerUrl: new URL('http://localhost:6300'),

  indexerClientConnection: {

    indexerHttpUrl: 'https://indexer.preprod.midnight.network/api/v4/graphql',

    indexerWsUrl: 'wss://indexer.preprod.midnight.network/api/v4/graphql/ws',

  },

  txHistoryStorage: new InMemoryTransactionHistoryStorage(),

};



const wallet = await WalletFacade.init({

  configuration,

  shielded: (config) => ShieldedWallet(config).startWithSecretKeys(shieldedKeys),

  unshielded: (config) => UnshieldedWallet(config).startWithPublicKey(publicKey),

  dust: (config) => DustWallet(config).startWithSecretKey(dustKey, dustParams),

});
```

### State[​](#state "Direct link to State")

Access the wallet state by waiting for initial sync or subscribing to updates.

```
// Wait for initial sync

const syncedState = await wallet.waitForSyncedState();

console.log('Shielded balance:', syncedState.shielded.balances);

console.log('Unshielded balance:', syncedState.unshielded.balances);

console.log('DUST balance:', syncedState.dust.totalCoins);



// Subscribe to state changes over time

wallet.state().subscribe((state) => {

  if (state.isSynced) {

    console.log('Shielded coins:', state.shielded.availableCoins.length);

    console.log('Unshielded UTxOs:', state.unshielded.availableCoins.length);

  }

});
```

### Transfers[​](#transfers "Direct link to Transfers")

Unshielded transfers use UTxO-based transactions with Schnorr signatures. Shielded transfers use zero-knowledge proofs and do not require signature operations.

```
import * as ledger from '@midnight-ntwrk/ledger-v8';



// Unshielded transfer

await wallet

  .transferTransaction(

    [

      {

        type: 'unshielded',

        outputs: [

          {

            amount: 1_000_000n,

            receiverAddress: await receiverWallet.unshielded.getAddress(),

            type: ledger.unshieldedToken().raw,

          },

        ],

      },

    ],

    { shieldedSecretKeys, dustSecretKey },

    { ttl: new Date(Date.now() + 30 * 60 * 1000) }

  )

  .then((recipe) => wallet.signRecipe(recipe, (payload) => keystore.signData(payload)))

  .then((recipe) => wallet.finalizeRecipe(recipe))

  .then((tx) => wallet.submitTransaction(tx));



// Shielded transfer (no signature required)

await wallet

  .transferTransaction(

    [

      {

        type: 'shielded',

        outputs: [

          {

            amount: 1_000_000n,

            receiverAddress: await receiverWallet.shielded.getAddress(),

            type: ledger.shieldedToken().raw,

          },

        ],

      },

    ],

    { shieldedSecretKeys, dustSecretKey },

    { ttl: new Date(Date.now() + 30 * 60 * 1000) }

  )

  .then((recipe) => wallet.finalizeRecipe(recipe))

  .then((tx) => wallet.submitTransaction(tx));
```

### Lifecycle[​](#lifecycle "Direct link to Lifecycle")

```
await wallet.start(shieldedSecretKeys, dustSecretKey);

await wallet.stop();
```

### Terms and Conditions[​](#terms-and-conditions "Direct link to Terms and Conditions")

```
const terms = await WalletFacade.fetchTermsAndConditions(networkId);

await wallet.acceptTermsAndConditions(terms);
```

## HD Wallet[​](#hd-wallet "Direct link to HD Wallet")

Derives all three key types from a single seed using BIP 32 / BIP 44 / CIP 1852 derivation.

```
import * as ledger from '@midnight-ntwrk/ledger-v8';

import { HDWallet, Roles } from '@midnight-ntwrk/wallet-sdk-hd';



function deriveRoleKey(accountKey, role, addressIndex = 0) {

  const result = accountKey.selectRole(role).deriveKeyAt(addressIndex);

  if (result.type === 'keyDerived') {

    return Buffer.from(result.key);

  }

  return deriveRoleKey(accountKey, role, addressIndex + 1);

}



const hdWallet = HDWallet.fromSeed(seed);

const account = hdWallet.hdWallet.selectAccount(0);

const shieldedSeed = deriveRoleKey(account, Roles.Zswap);

const dustSeed = deriveRoleKey(account, Roles.Dust);

const unshieldedKey = deriveRoleKey(account, Roles.NightExternal);

hdWallet.hdWallet.clear();
```

## Address Format[​](#address-format "Direct link to Address Format")

Midnight uses Bech32m format for addresses with network-specific prefixes.

```
import {

  MidnightBech32m,

  UnshieldedAddress,

  ShieldedAddress,

  DustAddress,

  ShieldedCoinPublicKey,

  ShieldedEncryptionPublicKey,

} from '@midnight-ntwrk/wallet-sdk-address-format';

import * as ledger from '@midnight-ntwrk/ledger-v8';



const networkId = 'preprod';



// Encode unshielded address

const verifyingKey = ledger.signatureVerifyingKey(unshieldedSecretKey.toString('hex'));

const unshieldedAddress = new UnshieldedAddress(

  Buffer.from(ledger.addressFromKey(verifyingKey), 'hex')

);

const unshieldedBech32m = MidnightBech32m.encode(networkId, unshieldedAddress).toString();



// Encode shielded address

const shieldedAddress = new ShieldedAddress(

  new ShieldedCoinPublicKey(Buffer.from(shieldedKeys.coinPublicKey, 'hex')),

  new ShieldedEncryptionPublicKey(Buffer.from(shieldedKeys.encryptionPublicKey, 'hex'))

);

const shieldedBech32m = MidnightBech32m.encode(networkId, shieldedAddress).toString();



// Encode DUST address

const dustAddress = new DustAddress(dustSecretKey.publicKey);

const dustBech32m = MidnightBech32m.encode(networkId, dustAddress).toString();



// Decode addresses

const parsed = MidnightBech32m.parse(unshieldedBech32m);

const decoded = parsed.decode(UnshieldedAddress, networkId);
```

## DUST Management[​](#dust-management "Direct link to DUST Management")

Register NIGHT coins to generate DUST for transaction fees.

```
const { unshielded } = await wallet.waitForSyncedState();



// Register NIGHT to start generating DUST

await wallet

  .registerNightUtxosForDustGeneration(

    unshielded.availableCoins,

    unshieldedKeystore.getPublicKey(),

    (payload) => unshieldedKeystore.signData(payload)

  )

  .then((recipe) => wallet.finalizeRecipe(recipe))

  .then((tx) => wallet.submitTransaction(tx));
```

## DUST Sponsorship[​](#dust-sponsorship "Direct link to DUST Sponsorship")

A sponsor can pay transaction fees on behalf of users.

```
// User prepares transaction without DUST balancing

const userRecipe = await userWallet.balanceUnboundTransaction(

  transaction,

  { shieldedSecretKeys, dustSecretKey },

  {

    ttl: new Date(Date.now() + 30 * 60 * 1000),

    tokenKindsToBalance: ['shielded', 'unshielded'],

  }

);



// Sponsor adds DUST and submits

const finalized = await userWallet.finalizeRecipe(

  await userWallet.signRecipe(userRecipe, (payload) => keystore.signData(payload))

);



await sponsorWallet

  .balanceFinalizedTransaction(

    finalized,

    { shieldedSecretKeys: sponsorKeys, dustSecretKey: sponsorDust },

    {

      ttl: new Date(Date.now() + 30 * 60 * 1000),

      tokenKindsToBalance: ['dust'],

    }

  )

  .then((recipe) => sponsorWallet.finalizeRecipe(recipe))

  .then((tx) => sponsorWallet.submitTransaction(tx));
```

## Atomic Swaps[​](#atomic-swaps "Direct link to Atomic Swaps")

Trustless token exchanges between parties using a single atomic transaction.

```
// Alice initiates swap offering token1 for token2

const aliceSwapTx = await aliceWallet

  .initSwap(

    { shielded: { [token1]: 1_000_000n } },

    [

      {

        type: 'shielded',

        outputs: [{

          type: token2,

          amount: 1_000_000n,

          receiverAddress: aliceShieldedAddress,

        }],

      },

    ],

    { shieldedSecretKeys: aliceKeys, dustSecretKey: aliceDust },

    { ttl: new Date(Date.now() + 30 * 60 * 1000) }

  )

  .then((recipe) => aliceWallet.finalizeRecipe(recipe));



// Bob completes the swap

await bobWallet

  .balanceFinalizedTransaction(

    aliceSwapTx,

    { shieldedSecretKeys: bobKeys, dustSecretKey: bobDust },

    { ttl: new Date(Date.now() + 30 * 60 * 1000) }

  )

  .then((recipe) => bobWallet.finalizeRecipe(recipe))

  .then((tx) => bobWallet.submitTransaction(tx));
```

## Alternative Proving[​](#alternative-proving "Direct link to Alternative Proving")

Use WASM-based proving when HTTP access to a proving server is not available.

```
import { makeWasmProvingService } from '@midnight-ntwrk/wallet-sdk-capabilities';



const wallet = await WalletFacade.init({

  configuration,

  shielded: (config) => ShieldedWallet(config).startWithSecretKeys(shieldedKeys),

  unshielded: (config) => UnshieldedWallet(config).startWithPublicKey(publicKey),

  dust: (config) => DustWallet(config).startWithSecretKey(dustKey, dustParams),

  provingService: () => makeWasmProvingService(),

});
```

## Release History[​](#release-history "Direct link to Release History")

See the [Wallet SDK release notes](/relnotes/wallet.md) for version history and changelog.

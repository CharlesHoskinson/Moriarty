# LevelPrivateStateProviderConfig

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-level-private-state-provider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider.md) / LevelPrivateStateProviderConfig

# Interface: LevelPrivateStateProviderConfig

Configuration properties for the LevelDB based private state provider.

## Properties[​](#properties "Direct link to Properties")

### accountId[​](#accountid "Direct link to accountId")

> `readonly` **accountId**: `string`

Account identifier used to scope storage. This ensures data isolation between different accounts/wallets using the same database.

The accountId is hashed (SHA-256, first 32 chars) before being used in storage paths, so any unique identifier can be used (e.g., wallet address).

#### Example[​](#example "Direct link to Example")

```
{

  accountId: walletAddress

}
```

***

### cryptoBackend?[​](#cryptobackend "Direct link to cryptoBackend?")

> `readonly` `optional` **cryptoBackend?**: [`CryptoBackendType`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/type-aliases/CryptoBackendType.md)

***

### levelFactory?[​](#levelfactory "Direct link to levelFactory?")

> `readonly` `optional` **levelFactory?**: [`LevelFactory`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/type-aliases/LevelFactory.md)

***

### midnightDbName[​](#midnightdbname "Direct link to midnightDbName")

> `readonly` **midnightDbName**: `string`

The name of the LevelDB database used to store all Midnight related data.

***

### privateStateStoreName[​](#privatestatestorename "Direct link to privateStateStoreName")

> `readonly` **privateStateStoreName**: `string`

The name of the object store containing private states.

***

### privateStoragePasswordProvider[​](#privatestoragepasswordprovider "Direct link to privateStoragePasswordProvider")

> `readonly` **privateStoragePasswordProvider**: [`PrivateStoragePasswordProvider`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/type-aliases/PrivateStoragePasswordProvider.md)

Provider function that returns the password used for encrypting private state. The password must be at least 16 characters long.

SECURITY: Use a strong, secret password. Never use public key material or other non-secret values as the password source.

#### Example[​](#example-1 "Direct link to Example")

```
{

  privateStoragePasswordProvider: async () => await getSecretPassword()

}
```

***

### signingKeyStoreName[​](#signingkeystorename "Direct link to signingKeyStoreName")

> `readonly` **signingKeyStoreName**: `string`

The name of the object store containing signing keys.

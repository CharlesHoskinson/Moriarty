# ContractConfiguration

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

Configuration interface for Midnight contracts.

## Properties[​](#properties "Direct link to Properties")

### privateStateStoreName[​](#privatestatestorename "Direct link to privateStateStoreName")

> `readonly` **privateStateStoreName**: `string`

Name of the store used for persisting private state data. This is used as a base name - a signing key store will also be created with "-signing-keys" appended.

***

### zkConfigPath[​](#zkconfigpath "Direct link to zkConfigPath")

> `readonly` **zkConfigPath**: `string`

File system path to the zero-knowledge proof configuration files. This should point to the directory containing the circuit verification keys and other ZK artifacts.

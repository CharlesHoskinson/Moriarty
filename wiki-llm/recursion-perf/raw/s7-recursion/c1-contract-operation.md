# ContractOperation | Midnight Docs

> Source: https://docs.midnight.network/develop/reference/midnight-api/ledger/classes/ContractOperation
> Collected: 2026-10-26
> Published: Unknown

SOURCE — Page last-updated label: 2026-10-23. Redirected to https://docs.midnight.network/api-reference/ledger/classes/ContractOperation.



[Skip to main content](#__docusaurus_skipToContent_fallback)

[![Midnight Logo](/img/midnight-header-logo-dark.svg)](/)

[API reference](/api-reference)[SDKs](/sdks)[Compact](/compact)[Dev Diaries](/blog)

AI search

Search

* [Overview](/api-reference)
* [Error reference](/api-reference/category/error-reference)
* [Compact runtime API](/api-reference/compact-runtime)
* [Midnight DApp connector API](/api-reference/dapp-connector)
* [Ledger API](/api-reference/ledger)

  + [classes](/api-reference/ledger/classes/Binding)

    - [Binding](/api-reference/ledger/classes/Binding)
    - [ChargedState](/api-reference/ledger/classes/ChargedState)
    - [ClaimRewardsTransaction](/api-reference/ledger/classes/ClaimRewardsTransaction)
    - [CoinSecretKey](/api-reference/ledger/classes/CoinSecretKey)
    - [ContractCall](/api-reference/ledger/classes/ContractCall)
    - [ContractCallPrototype](/api-reference/ledger/classes/ContractCallPrototype)
    - [ContractDeploy](/api-reference/ledger/classes/ContractDeploy)
    - [ContractMaintenanceAuthority](/api-reference/ledger/classes/ContractMaintenanceAuthority)
    - [ContractOperation](/api-reference/ledger/classes/ContractOperation)
    - [ContractOperationVersion](/api-reference/ledger/classes/ContractOperationVersion)
    - [ContractOperationVersionedVerifierKey](/api-reference/ledger/classes/ContractOperationVersionedVerifierKey)
    - [ContractState](/api-reference/ledger/classes/ContractState)
    - [CostModel](/api-reference/ledger/classes/CostModel)
    - [DustActions](/api-reference/ledger/classes/DustActions)
    - [DustGenerationState](/api-reference/ledger/classes/DustGenerationState)
    - [DustLocalState](/api-reference/ledger/classes/DustLocalState)
    - [DustLocalStateWithChanges](/api-reference/ledger/classes/DustLocalStateWithChanges)
    - [DustParameters](/api-reference/ledger/classes/DustParameters)
    - [DustRegistration](/api-reference/ledger/classes/DustRegistration)
    - [DustSecretKey](/api-reference/ledger/classes/DustSecretKey)
    - [DustSpend](/api-reference/ledger/classes/DustSpend)
    - [DustState](/api-reference/ledger/classes/DustState)
    - [DustStateChanges](/api-reference/ledger/classes/DustStateChanges)
    - [DustStateMerkleTreeCollapsedUpdate](/api-reference/ledger/classes/DustStateMerkleTreeCollapsedUpdate)
    - [DustUtxoState](/api-reference/ledger/classes/DustUtxoState)
    - [EncryptionSecretKey](/api-reference/ledger/classes/EncryptionSecretKey)
    - [Event](/api-reference/ledger/classes/Event)
    - [Intent](/api-reference/ledger/classes/Intent)
    - [LedgerParameters](/api-reference/ledger/classes/LedgerParameters)
    - [LedgerState](/api-reference/ledger/classes/LedgerState)
    - [MaintenanceUpdate](/api-reference/ledger/classes/MaintenanceUpdate)
    - [MerkleTreeCollapsedUpdate](/api-reference/ledger/classes/MerkleTreeCollapsedUpdate)
    - [NoBinding](/api-reference/ledger/classes/NoBinding)
    - [NoProof](/api-reference/ledger/classes/NoProof)
    - [PreBinding](/api-reference/ledger/classes/PreBinding)
    - [PrePartitionContractCall](/api-reference/ledger/classes/PrePartitionContractCall)
    - [PreProof](/api-reference/ledger/classes/PreProof)
    - [PreTranscript](/api-reference/ledger/classes/PreTranscript)
    - [Proof](/api-reference/ledger/classes/Proof)
    - [QueryContext](/api-reference/ledger/classes/QueryContext)
    - [QueryResults](/api-reference/ledger/classes/QueryResults)
    - [ReplaceAuthority](/api-reference/ledger/classes/ReplaceAuthority)
    - [SignatureEnabled](/api-reference/ledger/classes/SignatureEnabled)
    - [SignatureErased](/api-reference/ledger/classes/SignatureErased)
    - [StateBoundedMerkleTree](/api-reference/ledger/classes/StateBoundedMerkleTree)
    - [StateMap](/api-reference/ledger/classes/StateMap)
    - [StateValue](/api-reference/ledger/classes/StateValue)
    - [SystemTransaction](/api-reference/ledger/classes/SystemTransaction)
    - [Transaction](/api-reference/ledger/classes/Transaction)
    - [TransactionContext](/api-reference/ledger/classes/TransactionContext)
    - [TransactionCostModel](/api-reference/ledger/classes/TransactionCostModel)
    - [TransactionResult](/api-reference/ledger/classes/TransactionResult)
    - [UnshieldedOffer](/api-reference/ledger/classes/UnshieldedOffer)
    - [UtxoMeta](/api-reference/ledger/classes/UtxoMeta)
    - [UtxoState](/api-reference/ledger/classes/UtxoState)
    - [VerifiedTransaction](/api-reference/ledger/classes/VerifiedTransaction)
    - [VerifierKeyInsert](/api-reference/ledger/classes/VerifierKeyInsert)
    - [VerifierKeyRemove](/api-reference/ledger/classes/VerifierKeyRemove)
    - [VmResults](/api-reference/ledger/classes/VmResults)
    - [VmStack](/api-reference/ledger/classes/VmStack)
    - [WellFormedStrictness](/api-reference/ledger/classes/WellFormedStrictness)
    - [ZswapChainState](/api-reference/ledger/classes/ZswapChainState)
    - [ZswapInput](/api-reference/ledger/classes/ZswapInput)
    - [ZswapLocalState](/api-reference/ledger/classes/ZswapLocalState)
    - [ZswapLocalStateWithChanges](/api-reference/ledger/classes/ZswapLocalStateWithChanges)
    - [ZswapOffer](/api-reference/ledger/classes/ZswapOffer)
    - [ZswapOutput](/api-reference/ledger/classes/ZswapOutput)
    - [ZswapSecretKeys](/api-reference/ledger/classes/ZswapSecretKeys)
    - [ZswapStateChanges](/api-reference/ledger/classes/ZswapStateChanges)
    - [ZswapTransient](/api-reference/ledger/classes/ZswapTransient)
  + [functions](/api-reference/ledger/functions/addressFromKey)
  + [globals](/api-reference/ledger/globals)
  + [type-aliases](/api-reference/ledger/type-aliases/AlignedValue)
* [Midnight Indexer API v4](/api-reference/midnight-indexer)
* [Midnight JS API](/api-reference/midnight-js)
* [Onchain Runtime API](/api-reference/onchain-runtime)
* [Testkit JS API](/api-reference/testkit-js)
* [Wallet SDK](/api-reference/wallet-sdk)

* [Ledger API](/api-reference/ledger)
* classes
* ContractOperation

Explore with… ▾

Copy page as MarkdownCopied

On this page

ContractOperation
=================

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.1.3**](/api-reference/ledger)

---

[@midnight/ledger](/api-reference/ledger/globals) / ContractOperation

Class: ContractOperation
========================

An individual operation, or entry point of a contract, consisting primarily
of a ZK verifier keys, potentially for different versions of the proving
system.

Only the latest available version is exposed to this API.

Note that the serialized form of the key is checked on initialization

Constructors[​](#constructors "Direct link to Constructors")
------------------------------------------------------------

### Constructor[​](#constructor "Direct link to Constructor")

```
new ContractOperation(): ContractOperation;
```

#### Returns[​](#returns "Direct link to Returns")

`ContractOperation`

Properties[​](#properties "Direct link to Properties")
------------------------------------------------------

### verifierKey[​](#verifierkey "Direct link to verifierKey")

```
verifierKey: Uint8Array;
```

Methods[​](#methods "Direct link to Methods")
---------------------------------------------

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(): Uint8Array;
```

#### Returns[​](#returns-1 "Direct link to Returns")

`Uint8Array`

---

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-2 "Direct link to Returns")

`string`

---

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw): ContractOperation;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-3 "Direct link to Returns")

`ContractOperation`

[Edit this page](https://github.com/midnightntwrk/midnight-docs/edit/main/api-reference/ledger/classes/ContractOperation.md)

Last updated on **Oct 23, 2026**

[Previous

ContractMaintenanceAuthority](/api-reference/ledger/classes/ContractMaintenanceAuthority)[Next

ContractOperationVersion](/api-reference/ledger/classes/ContractOperationVersion)

* [Constructors](#constructors)
  + [Constructor](#constructor)
* [Properties](#properties)
  + [verifierKey](#verifierkey)
* [Methods](#methods)
  + [serialize()](#serialize)
  + [toString()](#tostring)
  + [deserialize()](#deserialize)

![Midnight Logo](/img/midnight-header-logo-dark.svg)![Midnight Logo](/img/midnight-header-logo-dark.svg)

© 2026 Input Output Global, Inc. All Rights Reserved.

Resources

* [Midnight Foundation](https://midnight.network/)
* [Dev Diaries](/blog)
* [Glacier Drop](https://www.midnight.gd/)
* [Careers](https://midnight.network/careers)

Legal

* [Cookie Policy](https://45047878.fs1.hubspotusercontent-na1.net/hubfs/45047878/Midnight%20Foundation%20cookie-policy.pdf)
* [Privacy Policy](https://45047878.fs1.hubspotusercontent-na1.net/hubfs/45047878/Midnight%20Foundation%20%20-%20Privacy%20Notice.pdf)
* [Terms and Conditions](https://45047878.fs1.hubspotusercontent-na1.net/hubfs/45047878/Midnight%20Foundation%20-%20Website%20Terms%20of%20Use.pdf)

Social

* [![YouTube](/img/youtube.svg)](https://www.youtube.com/channel/UCy3oZ64F3FOtjZ5sZGQNgkA)
* [![X/Twitter](/img/x.svg)](https://x.com/MidnightNtwrk)
* [![Discord](/img/discord.svg)](https://discord.com/invite/midnightnetwork)
* [![LinkedIn](/img/linkedin.svg)](https://www.linkedin.com/showcase/midnight-ntwrk/)
# CallContext

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / CallContext

# Type Alias: CallContext

```
type CallContext: {

  balance: Map<TokenType, bigint>;

  caller: PublicAddress;

  comIndices: Map<CoinCommitment, number>;

  lastBlockTime: bigint;

  ownAddress: ContractAddress;

  parentBlockHash: string;

  secondsSinceEpoch: bigint;

  secondsSinceEpochErr: number;

};
```

The context information of a call provided to the VM.

## Type declaration[​](#type-declaration "Direct link to Type declaration")

### balance[​](#balance "Direct link to balance")

```
balance: Map<TokenType, bigint>;
```

The balances held by the called contract at the time it was called.

### caller?[​](#caller "Direct link to caller?")

```
optional caller: PublicAddress;
```

A public address identifying an entity.

### comIndices[​](#comindices "Direct link to comIndices")

```
comIndices: Map<CoinCommitment, number>;
```

The commitment indices map accessible to the contract.

### lastBlockTime[​](#lastblocktime "Direct link to lastBlockTime")

```
lastBlockTime: bigint;
```

The [secondsSinceEpoch](/api-reference/onchain-runtime/type-aliases/CallContext.md#secondssinceepoch) of the previous block

### ownAddress[​](#ownaddress "Direct link to ownAddress")

```
ownAddress: ContractAddress;
```

### parentBlockHash[​](#parentblockhash "Direct link to parentBlockHash")

```
parentBlockHash: string;
```

The hash of the block prior to this transaction, as a hex-encoded string

### secondsSinceEpoch[​](#secondssinceepoch "Direct link to secondsSinceEpoch")

```
secondsSinceEpoch: bigint;
```

The seconds since the UNIX epoch that have elapsed

### secondsSinceEpochErr[​](#secondssinceepocherr "Direct link to secondsSinceEpochErr")

```
secondsSinceEpochErr: number;
```

The maximum error on [secondsSinceEpoch](/api-reference/onchain-runtime/type-aliases/CallContext.md#secondssinceepoch) that should occur, as a positive seconds value

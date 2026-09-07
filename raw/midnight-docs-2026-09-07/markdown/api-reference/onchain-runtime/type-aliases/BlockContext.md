# BlockContext

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / BlockContext

# Type Alias: BlockContext

```
type BlockContext: {

  lastBlockTime: bigint;

  parentBlockHash: string;

  secondsSinceEpoch: bigint;

  secondsSinceEpochErr: number;

};
```

Context information about the block forwarded to [CallContext](/api-reference/onchain-runtime/type-aliases/CallContext.md).

## Type declaration[​](#type-declaration "Direct link to Type declaration")

### lastBlockTime[​](#lastblocktime "Direct link to lastBlockTime")

```
lastBlockTime: bigint;
```

The [secondsSinceEpoch](/api-reference/onchain-runtime/type-aliases/BlockContext.md#secondssinceepoch) of the previous block

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

The maximum error on [secondsSinceEpoch](/api-reference/onchain-runtime/type-aliases/BlockContext.md#secondssinceepoch) that should occur, as a positive seconds value

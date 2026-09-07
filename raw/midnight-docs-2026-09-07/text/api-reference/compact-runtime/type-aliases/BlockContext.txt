# BlockContext

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / BlockContext

# Type Alias: BlockContext

```
type BlockContext = {

  lastBlockTime: bigint;

  parentBlockHash: string;

  secondsSinceEpoch: bigint;

  secondsSinceEpochErr: number;

};
```

Context information about the block forwarded to [CallContext](/api-reference/compact-runtime/type-aliases/CallContext.md).

## Properties[​](#properties "Direct link to Properties")

### lastBlockTime[​](#lastblocktime "Direct link to lastBlockTime")

```
lastBlockTime: bigint;
```

The [secondsSinceEpoch](#secondssinceepoch) of the previous block

***

### parentBlockHash[​](#parentblockhash "Direct link to parentBlockHash")

```
parentBlockHash: string;
```

The hash of the block prior to this transaction, as a hex-encoded string

***

### secondsSinceEpoch[​](#secondssinceepoch "Direct link to secondsSinceEpoch")

```
secondsSinceEpoch: bigint;
```

The seconds since the UNIX epoch that have elapsed

***

### secondsSinceEpochErr[​](#secondssinceepocherr "Direct link to secondsSinceEpochErr")

```
secondsSinceEpochErr: number;
```

The maximum error on [secondsSinceEpoch](#secondssinceepoch) that should occur, as a positive seconds value

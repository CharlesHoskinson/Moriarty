# SyntheticCost

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / SyntheticCost

# Type Alias: SyntheticCost

```
type SyntheticCost = {

  blockUsage: bigint;

  bytesChurned: bigint;

  bytesWritten: bigint;

  computeTime: bigint;

  readTime: bigint;

};
```

A modelled cost of a transaction or block.

## Properties[​](#properties "Direct link to Properties")

### blockUsage[​](#blockusage "Direct link to blockUsage")

```
blockUsage: bigint;
```

The number of bytes of blockspace used

***

### bytesChurned[​](#byteschurned "Direct link to bytesChurned")

```
bytesChurned: bigint;
```

The number of (modelled) bytes written temporarily or overwritten.

***

### bytesWritten[​](#byteswritten "Direct link to bytesWritten")

```
bytesWritten: bigint;
```

The net number of (modelled) bytes written, i.e. max(0, absolute written bytes less deleted bytes).

***

### computeTime[​](#computetime "Direct link to computeTime")

```
computeTime: bigint;
```

The amount of (modelled) time spent in single-threaded compute, measured in picoseconds.

***

### readTime[​](#readtime "Direct link to readTime")

```
readTime: bigint;
```

The amount of (modelled) time spent reading from disk, measured in picoseconds.

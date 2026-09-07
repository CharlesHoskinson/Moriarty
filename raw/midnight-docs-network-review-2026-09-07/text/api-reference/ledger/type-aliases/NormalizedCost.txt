# NormalizedCost

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / NormalizedCost

# Type Alias: NormalizedCost

```
type NormalizedCost = {

  blockUsage: number;

  bytesChurned: number;

  bytesWritten: number;

  computeTime: number;

  readTime: number;

};
```

A normalized form of [SyntheticCost](/api-reference/ledger/type-aliases/SyntheticCost.md).

## Properties[​](#properties "Direct link to Properties")

### blockUsage[​](#blockusage "Direct link to blockUsage")

```
blockUsage: number;
```

The number of bytes of blockspace used

***

### bytesChurned[​](#byteschurned "Direct link to bytesChurned")

```
bytesChurned: number;
```

The number of (modelled) bytes written temporarily or overwritten.

***

### bytesWritten[​](#byteswritten "Direct link to bytesWritten")

```
bytesWritten: number;
```

The net number of (modelled) bytes written, i.e. max(0, absolute written bytes less deleted bytes).

***

### computeTime[​](#computetime "Direct link to computeTime")

```
computeTime: number;
```

The amount of (modelled) time spent in single-threaded compute, measured in picoseconds.

***

### readTime[​](#readtime "Direct link to readTime")

```
readTime: number;
```

The amount of (modelled) time spent reading from disk, measured in picoseconds.

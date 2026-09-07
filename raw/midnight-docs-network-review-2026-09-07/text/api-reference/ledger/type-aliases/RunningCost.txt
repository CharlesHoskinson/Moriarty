# RunningCost

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / RunningCost

# Type Alias: RunningCost

```
type RunningCost = {

  bytesDeleted: bigint;

  bytesWritten: bigint;

  computeTime: bigint;

  readTime: bigint;

};
```

A running tally of synthetic resource costs.

## Properties[​](#properties "Direct link to Properties")

### bytesDeleted[​](#bytesdeleted "Direct link to bytesDeleted")

```
bytesDeleted: bigint;
```

The number of (modelled) bytes deleted.

***

### bytesWritten[​](#byteswritten "Direct link to bytesWritten")

```
bytesWritten: bigint;
```

The number of (modelled) bytes written.

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

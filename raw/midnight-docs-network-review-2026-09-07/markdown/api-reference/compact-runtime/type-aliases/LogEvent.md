# LogEvent

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / LogEvent

# Type Alias: LogEvent

```
type LogEvent = Extract<GatherResult, {

  tag: "log";

}>["content"] & {

  address: ContractAddress;

};
```

A `GatherResult` narrowed to log emissions, tagged with the address of the contract that emitted it; `content` is the encoded `VersionedLogItem` array.

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### address[​](#address "Direct link to address")

```
address: ContractAddress;
```

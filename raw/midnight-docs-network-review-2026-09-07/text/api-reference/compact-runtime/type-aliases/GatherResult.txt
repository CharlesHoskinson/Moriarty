# GatherResult

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / GatherResult

# Type Alias: GatherResult

```
type GatherResult = 

  | {

  content: AlignedValue;

  tag: "read";

}

  | {

  content: {

     data: EncodedStateValue;

     eventType: LogEventType;

     version: number;

  };

  tag: "log";

};
```

An individual result of observing the results of a non-verifying VM program execution

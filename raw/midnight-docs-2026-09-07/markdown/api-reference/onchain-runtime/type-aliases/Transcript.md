# Transcript

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / Transcript

# Type Alias: Transcript\<R>

```
type Transcript<R>: {

  effects: Effects;

  gas: RunningCost;

  program: Op<R>[];

};
```

A transcript of operations, to be recorded in a transaction

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

• **R**

## Type declaration[​](#type-declaration "Direct link to Type declaration")

### effects[​](#effects "Direct link to effects")

```
effects: Effects;
```

The effects of the transcript, which are checked before execution, and must match those constructed by [program](/api-reference/onchain-runtime/type-aliases/Transcript.md#program)

### gas[​](#gas "Direct link to gas")

```
gas: RunningCost;
```

The execution budget for this transcript, which [program](/api-reference/onchain-runtime/type-aliases/Transcript.md#program) must not exceed

### program[​](#program "Direct link to program")

```
program: Op<R>[];
```

The sequence of operations that this transcript captured

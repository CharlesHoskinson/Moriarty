# Transcript

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / Transcript

# Type Alias: Transcript\<R>

```
type Transcript<R> = {

  effects: Effects;

  gas: RunningCost;

  program: Op<R>[];

};
```

A transcript of operations, to be recorded in a transaction

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### R[​](#r "Direct link to R")

`R`

## Properties[​](#properties "Direct link to Properties")

### effects[​](#effects "Direct link to effects")

```
effects: Effects;
```

The effects of the transcript, which are checked before execution, and must match those constructed by [program](#program)

***

### gas[​](#gas "Direct link to gas")

```
gas: RunningCost;
```

The execution budget for this transcript, which [program](#program) must not exceed

***

### program[​](#program "Direct link to program")

```
program: Op<R>[];
```

The sequence of operations that this transcript captured

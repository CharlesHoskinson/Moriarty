# VmStack

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / VmStack

# Class: VmStack

Represents the state of the VM's stack at a specific point. The stack is an array of [StateValue](/api-reference/onchain-runtime/classes/StateValue.md)s, each of which is also annotated with whether it is "strong" or "weak"; that is, whether it is permitted to be stored on-chain or not.

## Constructors[​](#constructors "Direct link to Constructors")

### new VmStack()[​](#new-vmstack "Direct link to new VmStack()")

```
new VmStack(): VmStack
```

#### Returns[​](#returns "Direct link to Returns")

[`VmStack`](/api-reference/onchain-runtime/classes/VmStack.md)

## Methods[​](#methods "Direct link to Methods")

### get()[​](#get "Direct link to get()")

```
get(idx): undefined | StateValue
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### idx[​](#idx "Direct link to idx")

`number`

#### Returns[​](#returns-1 "Direct link to Returns")

`undefined` | [`StateValue`](/api-reference/onchain-runtime/classes/StateValue.md)

***

### isStrong()[​](#isstrong "Direct link to isStrong()")

```
isStrong(idx): undefined | boolean
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### idx[​](#idx-1 "Direct link to idx")

`number`

#### Returns[​](#returns-2 "Direct link to Returns")

`undefined` | `boolean`

***

### length()[​](#length "Direct link to length()")

```
length(): number
```

#### Returns[​](#returns-3 "Direct link to Returns")

`number`

***

### push()[​](#push "Direct link to push()")

```
push(value, is_strong): void
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### value[​](#value "Direct link to value")

[`StateValue`](/api-reference/onchain-runtime/classes/StateValue.md)

##### is\_strong[​](#is_strong "Direct link to is_strong")

`boolean`

#### Returns[​](#returns-4 "Direct link to Returns")

`void`

***

### removeLast()[​](#removelast "Direct link to removeLast()")

```
removeLast(): void
```

#### Returns[​](#returns-5 "Direct link to Returns")

`void`

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string
```

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-6 "Direct link to Returns")

`string`

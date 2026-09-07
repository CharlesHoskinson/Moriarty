# ProvingProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ProvingProvider

# Type Alias: ProvingProvider

```
type ProvingProvider = {

  check: Promise<(undefined | bigint)[]>;

  prove: Promise<Uint8Array<ArrayBufferLike>>;

};
```

## Methods[​](#methods "Direct link to Methods")

### check()[​](#check "Direct link to check()")

```
check(serializedPreimage, keyLocation): Promise<(undefined | bigint)[]>;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### serializedPreimage[​](#serializedpreimage "Direct link to serializedPreimage")

`Uint8Array`

##### keyLocation[​](#keylocation "Direct link to keyLocation")

`string`

#### Returns[​](#returns "Direct link to Returns")

`Promise`<(`undefined` | `bigint`)\[]>

***

### prove()[​](#prove "Direct link to prove()")

```
prove(

   serializedPreimage, 

   keyLocation, 

overwriteBindingInput?): Promise<Uint8Array<ArrayBufferLike>>;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### serializedPreimage[​](#serializedpreimage-1 "Direct link to serializedPreimage")

`Uint8Array`

##### keyLocation[​](#keylocation-1 "Direct link to keyLocation")

`string`

##### overwriteBindingInput?[​](#overwritebindinginput "Direct link to overwriteBindingInput?")

`bigint`

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<`Uint8Array`<`ArrayBufferLike`>>

# MaintenanceUpdate

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / MaintenanceUpdate

# Class: MaintenanceUpdate

A contract maintenance update, updating associated operations, or changing the maintenance authority.

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new MaintenanceUpdate(

   address, 

   updates, 

   counter): MaintenanceUpdate;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### address[​](#address "Direct link to address")

`string`

##### updates[​](#updates "Direct link to updates")

[`SingleUpdate`](/api-reference/ledger/type-aliases/SingleUpdate.md)\[]

##### counter[​](#counter "Direct link to counter")

`bigint`

#### Returns[​](#returns "Direct link to Returns")

`MaintenanceUpdate`

## Properties[​](#properties "Direct link to Properties")

### address[​](#address-1 "Direct link to address")

```
readonly address: string;
```

The address this deployment will attempt to create

***

### counter[​](#counter-1 "Direct link to counter")

```
readonly counter: bigint;
```

The counter this update is valid against

***

### dataToSign[​](#datatosign "Direct link to dataToSign")

```
readonly dataToSign: Uint8Array;
```

The raw data any valid signature must be over to approve this update.

***

### signatures[​](#signatures "Direct link to signatures")

```
readonly signatures: [bigint, string][];
```

The signatures on this update

***

### updates[​](#updates-1 "Direct link to updates")

```
readonly updates: SingleUpdate[];
```

The updates to carry out

## Methods[​](#methods "Direct link to Methods")

### addSignature()[​](#addsignature "Direct link to addSignature()")

```
addSignature(idx, signature): MaintenanceUpdate;
```

Adds a new signature to this update

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### idx[​](#idx "Direct link to idx")

`bigint`

##### signature[​](#signature "Direct link to signature")

`string`

#### Returns[​](#returns-1 "Direct link to Returns")

`MaintenanceUpdate`

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-2 "Direct link to Returns")

`string`

# GzipFile

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

A class for compressing and decompressing files using gzip.

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new GzipFile**(`inputFile`, `outputFile`): `GzipFile`

Creates a new GzipFile instance.

#### Parameters[​](#parameters "Direct link to Parameters")

##### inputFile[​](#inputfile "Direct link to inputFile")

`string`

The path to the input file to compress/decompress

##### outputFile[​](#outputfile "Direct link to outputFile")

`string`

The path where the compressed file will be saved

#### Returns[​](#returns "Direct link to Returns")

`GzipFile`

## Properties[​](#properties "Direct link to Properties")

### inputFile[​](#inputfile-1 "Direct link to inputFile")

> **inputFile**: `string`

The path to the input file

***

### outputFile[​](#outputfile-1 "Direct link to outputFile")

> **outputFile**: `string`

The path to the output file

## Methods[​](#methods "Direct link to Methods")

### compress()[​](#compress "Direct link to compress()")

> **compress**(): `Promise`<`void`>

Compresses the input file using gzip compression.

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<`void`>

A promise that resolves when compression is complete

#### Throws[​](#throws "Direct link to Throws")

If there is an error during compression

***

### decompress()[​](#decompress "Direct link to decompress()")

> **decompress**(): `Promise`<`string`>

Decompresses the input gzip file and returns its contents as a string.

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<`string`>

A promise that resolves with the decompressed file contents as a string

#### Throws[​](#throws-1 "Direct link to Throws")

If there is an error during decompression

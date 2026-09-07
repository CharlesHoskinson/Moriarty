# DParameterChange

> For the complete documentation index, see [llms.txt](/llms.txt)

D-parameter change record for history queries.

```
type DParameterChange {

  blockHeight: Int!

  blockHash: HexEncoded!

  timestamp: Int!

  numPermissionedCandidates: Int!

  numRegisteredCandidates: Int!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`DParameterChange.blockHeight`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#dparameterchangeblockheightint-- "Direct link to dparameterchangeblockheightint--")

The block height where this parameter became effective.

#### [`DParameterChange.blockHash`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#dparameterchangeblockhashhexencoded-- "Direct link to dparameterchangeblockhashhexencoded--")

The hex-encoded block hash where this parameter became effective.

#### [`DParameterChange.timestamp`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#dparameterchangetimestampint-- "Direct link to dparameterchangetimestampint--")

The UNIX timestamp when this parameter became effective.

#### [`DParameterChange.numPermissionedCandidates`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#dparameterchangenumpermissionedcandidatesint-- "Direct link to dparameterchangenumpermissionedcandidatesint--")

Number of permissioned candidates.

#### [`DParameterChange.numRegisteredCandidates`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#dparameterchangenumregisteredcandidatesint-- "Direct link to dparameterchangenumregisteredcandidatesint--")

Number of registered candidates.

### Returned By[​](#returned-by "Direct link to Returned By")

[`dParameterHistory`](/api-reference/midnight-indexer/operations/queries/d-parameter-history.md) query

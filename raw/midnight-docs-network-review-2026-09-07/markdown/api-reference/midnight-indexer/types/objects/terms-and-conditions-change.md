# TermsAndConditionsChange

> For the complete documentation index, see [llms.txt](/llms.txt)

Terms and Conditions change record for history queries.

```
type TermsAndConditionsChange {

  blockHeight: Int!

  blockHash: HexEncoded!

  timestamp: Int!

  hash: HexEncoded!

  url: String!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`TermsAndConditionsChange.blockHeight`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#termsandconditionschangeblockheightint-- "Direct link to termsandconditionschangeblockheightint--")

The block height where this T\&C version became effective.

#### [`TermsAndConditionsChange.blockHash`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#termsandconditionschangeblockhashhexencoded-- "Direct link to termsandconditionschangeblockhashhexencoded--")

The hex-encoded block hash where this T\&C version became effective.

#### [`TermsAndConditionsChange.timestamp`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#termsandconditionschangetimestampint-- "Direct link to termsandconditionschangetimestampint--")

The UNIX timestamp when this T\&C version became effective.

#### [`TermsAndConditionsChange.hash`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#termsandconditionschangehashhexencoded-- "Direct link to termsandconditionschangehashhexencoded--")

The hex-encoded hash of the Terms and Conditions document.

#### [`TermsAndConditionsChange.url`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#termsandconditionschangeurlstring-- "Direct link to termsandconditionschangeurlstring--")

The URL where the Terms and Conditions can be found.

### Returned By[​](#returned-by "Direct link to Returned By")

[`termsAndConditionsHistory`](/api-reference/midnight-indexer/operations/queries/terms-and-conditions-history.md) query

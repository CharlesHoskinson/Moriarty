# SystemParameters

> For the complete documentation index, see [llms.txt](/llms.txt)

System parameters at a specific block height.

```
type SystemParameters {

  dParameter: DParameter!

  termsAndConditions: TermsAndConditions

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`SystemParameters.dParameter`](#) ● [`DParameter!`](/api-reference/midnight-indexer/types/objects/dparameter.md) non-null object[​](#systemparametersdparameterdparameter-- "Direct link to systemparametersdparameterdparameter--")

The D-parameter controlling validator committee composition.

#### [`SystemParameters.termsAndConditions`](#) ● [`TermsAndConditions`](/api-reference/midnight-indexer/types/objects/terms-and-conditions.md) object[​](#systemparameterstermsandconditionstermsandconditions- "Direct link to systemparameterstermsandconditionstermsandconditions-")

The current Terms and Conditions, if any have been set.

### Member Of[​](#member-of "Direct link to Member Of")

[`Block`](/api-reference/midnight-indexer/types/objects/block.md) object

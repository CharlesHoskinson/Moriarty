[Skip to main content](#__docusaurus_skipToContent_fallback)

[![Actus Logo](/img/ActusLogoRGB.jpg)![Actus Logo](/img/ActusLogoRGB.jpg)](/)[Dictionary](https://www.actusfrf.org/dictionary)[Taxonomy](https://www.actusfrf.org/taxonomy)

[GitHub](https://github.com/actusfrf)

* [Welcome to ACTUS Documentation](/docs/intro)
* [Introduction to ACTUS](/docs/category/introduction-to-actus)
* [ACTUS Quick Start](/docs/quickstart)
* [Quickstart Extension for ACTUS Risk Factors](/docs/category/1-extention-for-actus-risk-factors)
* [Demos and Guides](/docs/actus-demo/demo-user-guide)
* [ACTUS Competition Pre-Announcement](/docs/competition)
* [Awesome Python Library](/docs/category/awesome-python-library)

  + [Awesome ACTUS Library](/docs/awesome-actus-lib/awesome-actus-library)
  + [ACTUS Standard](/docs/standard/overview)
  + [Getting Started](/docs/getting-started/installation)
  + [Examples](/docs/examples/basic-contract-types/example_PAM)

    - [Basic Contract Type Examples](/docs/examples/basic-contract-types/example_PAM)

      * [Basic PAM Example](/docs/examples/basic-contract-types/example_PAM)
      * [Basic ANN Example](/docs/examples/basic-contract-types/example_ANN)
      * [Basic OPTNS Example](/docs/examples/basic-contract-types/example_OPTNS)
      * [Basic LAM Example](/docs/examples/basic-contract-types/example_LAM)
      * [Basic BCS Example](/docs/examples/basic-contract-types/example_BCS)
      * [Basic CAPFL Example](/docs/examples/basic-contract-types/example_CAPFL)
      * [Basic CSH Example](/docs/examples/basic-contract-types/example_CSH)
      * [Basic CEC Example](/docs/examples/basic-contract-types/example_CEC)
      * [Basic COM Example](/docs/examples/basic-contract-types/example_COM)
      * [Basic LAX Example](/docs/examples/basic-contract-types/example_LAX)
      * [Basic FXOUT Example](/docs/examples/basic-contract-types/example_FXOUT)
      * [Basic FUTUR Example](/docs/examples/basic-contract-types/example_FUTUR)
      * [Basic CEG Example](/docs/examples/basic-contract-types/example_CEG)
      * [Basic NAM Example](/docs/examples/basic-contract-types/example_NAM)
      * [Basic SWPPV Example](/docs/examples/basic-contract-types/example_SWPPV)
      * [Basic STK Example](/docs/examples/basic-contract-types/example_STK)
      * [Basic UMP Example](/docs/examples/basic-contract-types/example_UMP)
      * [Basic SWAPS Example](/docs/examples/basic-contract-types/example_SWAPS)
      * [Basic CLM Example](/docs/examples/basic-contract-types/example_CLM)
    - [Advanced Examples](/docs/examples/advanced-examples/singleContract)
  + [Guides](/docs/guides/common-patterns)
* [[ACTUSFRF] CORE LICENSE 1.0](/docs/license)

* [Awesome Python Library](/docs/category/awesome-python-library)
* Examples
* [Basic Contract Type Examples](/docs/examples/basic-contract-types/example_PAM)
* Basic SWPPV Example

On this page

Basic SWPPV Example
===================

Contract Definition
-------------------

```
import pandas as pd  
from awesome_actus_lib import SWPPV, PublicActusService, ReferenceIndex  
  
contract = SWPPV(  
    contractID="swppv01",  
    statusDate="2014-12-30T00:00:00",  
    contractRole="PF",  
    currency="USD",  
    contractDealDate="2014-12-28T00:00:00",  
    initialExchangeDate="2015-01-01T00:00:00",  
    maturityDate="2015-04-01T00:00:00",  
    notionalPrincipal=1000,  
    cycleAnchorDateOfInterestPayment="2015-04-01T00:00:00",  
    cycleOfInterestPayment="P1ML1",  
    nominalInterestRate=0.05,  
    nominalInterestRate2=0.08,  
    dayCountConvention="A365",  
    cycleAnchorDateOfRateReset="2015-01-01T00:00:00",  
    rateSpread=0.0,  
    marketObjectCodeOfRateReset="USD_Treasury",  
    fixingPeriod="P0D",  
    deliverySettlement="D",  
    creatorID="Creator-01",  
    counterpartyID="Counterparty-01"  
)  
rf_data_USD_Treasury = pd.DataFrame({  
    "date": ["2015-01-01"],  
    "value": [0.01]  
})  
rf_USD_Treasury = ReferenceIndex(marketObjectCode="USD_Treasury", source=rf_data_USD_Treasury)  
service = PublicActusService()  
event_stream = service.generateEvents(portfolio=contract, riskFactors=[rf_USD_Treasury])  
print(event_stream.events_df)
```

Generated Events
----------------

| type | time | payoff | currency | nominalValue | nominalRate | nominalAccrued | contractId |
| --- | --- | --- | --- | --- | --- | --- | --- |
| IED | 2015-01-01T00:00 | 0 | USD | -1000 | 0.08 | 0 | swppv01 |
| RR | 2015-01-01T00:00 | 0 | USD | -1000 | 0.01 | 0 | swppv01 |
| IPFX | 2015-04-01T00:00 | -12.3288 | USD | -1000 | 0.01 | 0 | swppv01 |
| IPFL | 2015-04-01T00:00 | 2.46575 | USD | -1000 | 0.01 | 0 | swppv01 |
| MD | 2015-04-01T00:00 | 0 | USD | 0 | 0.01 | 0 | swppv01 |

[Previous

Basic NAM Example](/docs/examples/basic-contract-types/example_NAM)[Next

Basic STK Example](/docs/examples/basic-contract-types/example_STK)

* [Contract Definition](/docs/examples/basic-contract-types/example_SWPPV#contract-definition)
* [Generated Events](/docs/examples/basic-contract-types/example_SWPPV#generated-events)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
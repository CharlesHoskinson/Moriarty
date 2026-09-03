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
* Basic FUTUR Example

On this page

Basic FUTUR Example
===================

Contract Definition
-------------------

```
import pandas as pd  
from awesome_actus_lib import FUTUR, PublicActusService, ReferenceIndex  
  
contract = FUTUR(  
    contractID="future01",  
    contractRole="BUY",  
    contractStructure=[{'object': {'marketObjectCode': 'AAPL'}, 'referenceType': 'MOC', 'referenceRole': 'UDL'}],  
    currency="USD",  
    calendar="NC",  
    contractDealDate="2020-01-01T00:00:00",  
    statusDate="2020-01-01T00:00:00",  
    purchaseDate="2020-01-02T00:00:00",  
    priceAtPurchaseDate=10,  
    maturityDate="2020-03-30T00:00:00",  
    futuresPrice=80,  
    settlementPeriod="P0D",  
    deliverySettlement="S",  
    creatorID="Creator-01",  
    counterpartyID="Counterparty-01"  
)  
rf_data_AAPL = pd.DataFrame({  
    "date": ["2020-03-30", "2020-06-30", "2020-09-30"],  
    "value": [63.70, 91.20, 115.81]  
})  
rf_AAPL = ReferenceIndex(marketObjectCode="AAPL", source=rf_data_AAPL)  
rf_data_MSFT = pd.DataFrame({  
    "date": ["2020-03-30", "2020-06-30", "2020-09-30"],  
    "value": [160.23, 203.50, 210.33]  
})  
rf_MSFT = ReferenceIndex(marketObjectCode="MSFT", source=rf_data_MSFT)  
service = PublicActusService()  
event_stream = service.generateEvents(portfolio=contract, riskFactors=[rf_AAPL, rf_MSFT])  
print(event_stream.events_df)
```

Generated Events
----------------

| type | time | payoff | currency | nominalValue | nominalRate | nominalAccrued | contractId |
| --- | --- | --- | --- | --- | --- | --- | --- |
| PRD | 2020-01-02T00:00 | -10 | USD | 0 | 0 | 0 | future01 |
| MD | 2020-03-30T00:00 | 0 | USD | 0 | 0 | 0 | future01 |
| XD | 2020-03-30T00:00 | 0 | USD | 0 | 0 | 0 | future01 |
| STD | 2020-03-30T00:00 | -16.3 | USD | 0 | 0 | 0 | future01 |

[Previous

Basic FXOUT Example](/docs/examples/basic-contract-types/example_FXOUT)[Next

Basic CEG Example](/docs/examples/basic-contract-types/example_CEG)

* [Contract Definition](/docs/examples/basic-contract-types/example_FUTUR#contract-definition)
* [Generated Events](/docs/examples/basic-contract-types/example_FUTUR#generated-events)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
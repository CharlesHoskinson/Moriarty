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
* Basic CAPFL Example

On this page

Basic CAPFL Example
===================

Contract Definition
-------------------

```
import pandas as pd  
from awesome_actus_lib import CAPFL, PublicActusService, ReferenceIndex  
  
contract = CAPFL(  
    contractID="capfl01",  
    contractRole="BUY",  
    contractStructure=[{'object': {'contractType': 'PAM', 'contractID': 'capfl-underlying', 'contractDealDate': '2012-12-28T00:00:00', 'initialExchangeDate': '2013-01-01T00:00:00', 'currency': 'USD', 'statusDate': '2012-12-30T00:00:00', 'notionalPrincipal': '1000', 'dayCountConvention': 'A365', 'nominalInterestRate': '0.05', 'maturityDate': '2014-01-01T00:00:00', 'cycleAnchorDateOfInterestPayment': '2013-04-01T00:00:00', 'cycleOfInterestPayment': 'P3ML1', 'cycleAnchorDateOfRateReset': '2013-04-01T00:00:00', 'cycleOfRateReset': 'P3ML1', 'marketObjectCodeOfRateReset': 'LIBORUSD1M'}, 'referenceType': 'CNT', 'referenceRole': 'UDL'}],  
    currency="USD",  
    contractDealDate="2012-12-28T00:00:00",  
    statusDate="2012-12-30T00:00:00",  
    lifeCap=0.07,  
    creatorID="Creator-01",  
    counterpartyID="Counterparty-01"  
)  
rf_data_LIBORUSD1M = pd.DataFrame({  
    "date": ["2013-04-01", "2013-07-01", "2013-10-01"],  
    "value": [0.1, 0.01, 0.04]  
})  
rf_LIBORUSD1M = ReferenceIndex(marketObjectCode="LIBORUSD1M", source=rf_data_LIBORUSD1M)  
service = PublicActusService()  
event_stream = service.generateEvents(portfolio=contract, riskFactors=[rf_LIBORUSD1M])  
print(event_stream.events_df)
```

Generated Events
----------------

| type | time | payoff | currency | nominalValue | nominalRate | nominalAccrued | contractId |
| --- | --- | --- | --- | --- | --- | --- | --- |
| IP | 2013-04-01T00:00 | 0 | USD | 0 | 0 | 0 | capfl01 |
| IP | 2013-07-01T00:00 | 7.47945 | USD | 0 | 0 | 0 | capfl01 |
| IP | 2013-10-01T00:00 | 1.77636e-15 | USD | 0 | 0 | 0 | capfl01 |
| IP | 2014-01-01T00:00 | 0 | USD | 0 | 0 | 0 | capfl01 |

[Previous

Basic BCS Example](/docs/examples/basic-contract-types/example_BCS)[Next

Basic CSH Example](/docs/examples/basic-contract-types/example_CSH)

* [Contract Definition](/docs/examples/basic-contract-types/example_CAPFL#contract-definition)
* [Generated Events](/docs/examples/basic-contract-types/example_CAPFL#generated-events)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
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
* Basic CEC Example

On this page

Basic CEC Example
=================

Contract Definition
-------------------

```
import pandas as pd  
from awesome_actus_lib import CEC, PublicActusService, ReferenceIndex  
  
contract = CEC(  
    contractID="collateral01",  
    contractRole="BUY",  
    guaranteedExposure="NO",  
    contractStructure=[{'object': {'contractType': 'PAM', 'contractID': 'US91282XYZ01', 'statusDate': '2020-01-01T00:00:00', 'contractDealDate': '2020-01-01T00:00:00', 'currency': 'USD', 'notionalPrincipal': '1000000', 'initialExchangeDate': '2020-01-02T00:00:00', 'maturityDate': '2020-12-31T00:00:00', 'nominalInterestRate': '0.03', 'cycleAnchorDateOfInterestPayment': '2020-02-01T00:00:00', 'cycleOfInterestPayment': 'P1ML0', 'dayCountConvention': 'A365', 'endOfMonthConvention': 'SD', 'contractRole': 'RPA'}, 'referenceType': 'CNT', 'referenceRole': 'COVE'}, {'object': {'contractType': 'COM', 'contractID': 'GOLD01X12DF3VW', 'statusDate': '2020-01-01T00:00:00', 'contractDealDate': '2020-01-01T00:00:00', 'currency': 'USD', 'contractRole': 'RPA', 'creatorId': 'PartyXYZ', 'marketObjectCode': 'GOLD', 'quantity': '1', 'unit': 'ONC'}, 'referenceType': 'CNT', 'referenceRole': 'COVI'}],  
    currency="USD",  
    calendar="NC",  
    contractDealDate="2020-01-01T00:00:00",  
    statusDate="2020-01-01T00:00:00",  
    creditEventTypeCovered="DF",  
    settlementPeriod="P0D",  
    creatorID="Creator-01",  
    counterpartyID="Counterparty-01"  
)  
rf_data_GOLD = pd.DataFrame({  
    "date": ["2020-09-19"],  
    "value": [10000000]  
})  
rf_GOLD = ReferenceIndex(marketObjectCode="GOLD", source=rf_data_GOLD)  
service = PublicActusService()  
event_stream = service.generateEvents(portfolio=contract, riskFactors=[rf_GOLD])  
print(event_stream.events_df)
```

Generated Events
----------------

| type | time | payoff | currency | nominalValue | nominalRate | nominalAccrued | contractId |
| --- | --- | --- | --- | --- | --- | --- | --- |
| MD | 2020-12-31T00:00 | 0 | USD | 0 | 0 | 0 | collateral01 |

[Previous

Basic CSH Example](/docs/examples/basic-contract-types/example_CSH)[Next

Basic COM Example](/docs/examples/basic-contract-types/example_COM)

* [Contract Definition](/docs/examples/basic-contract-types/example_CEC#contract-definition)
* [Generated Events](/docs/examples/basic-contract-types/example_CEC#generated-events)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
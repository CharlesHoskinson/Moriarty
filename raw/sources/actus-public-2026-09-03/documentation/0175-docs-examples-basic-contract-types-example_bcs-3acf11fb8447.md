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
* Basic BCS Example

On this page

Basic BCS Example
=================

Contract Definition
-------------------

```
import pandas as pd  
from awesome_actus_lib import BCS, PublicActusService, ReferenceIndex  
  
contract = BCS(  
    contractID="brcsw01A",  
    contractRole="BUY",  
    boundaryValue=60,  
    boundaryDirection="DECR",  
    boundaryEffect="knockINFirstLeg",  
    boundaryLegInitiallyActive="NULL",  
    boundaryMonitoringAnchorDate="2020-01-01T00:00:00",  
    boundaryMonitoringCycle="P1ML1",  
    boundaryMonitoringEndDate="2020-03-30T00:00:00",  
    statusDate="2020-01-01T00:00:00",  
    boundaryCrossedFlag=False,  
    currency="USD",  
    calendar="NC",  
    contractDealDate="2020-01-01T00:00:00",  
    purchaseDate="2020-01-02T00:00:00",  
    maturityDate="2020-03-30T00:00:00",  
    priceAtPurchaseDate=10,  
    settlementPeriod="P1D",  
    deliverySettlement="S",  
    contractStructure=[{'object': {'marketObjectCode': 'AAPL'}, 'referenceType': 'MOC', 'referenceRole': 'externalReferenceIndex'}, {'object': {'contractType': 'OPTNS', 'contractID': 'brcsw01A-leg1', 'contractRole': 'BUY', 'currency': 'USD', 'calendar': 'NC', 'statusDate': '2020-01-01T00:00:00', 'maturityDate': '2020-03-30T00:00:00', 'optionExerciseType': 'E', 'optionType': 'P', 'settlementPeriod': 'P1D', 'optionStrike1': '65', 'quantity': '5', 'contractStructure': [{'object': {'marketObjectCode': 'AAPL'}, 'referenceType': 'MOC', 'referenceRole': 'UDL'}]}, 'referenceType': 'CNT', 'referenceRole': 'FIL'}],  
    creatorID="Creator-01",  
    counterpartyID="Counterparty-01"  
)  
rf_data_AAPL = pd.DataFrame({  
    "date": ["2020-01-01", "2020-02-01", "2020-03-01", "2020-03-30"],  
    "value": [66.0, 60.0, 59.0, 58.0]  
})  
rf_AAPL = ReferenceIndex(marketObjectCode="AAPL", source=rf_data_AAPL)  
service = PublicActusService()  
event_stream = service.generateEvents(portfolio=contract, riskFactors=[rf_AAPL])  
print(event_stream.events_df)
```

Generated Events
----------------

| type | time | payoff | currency | nominalValue | nominalRate | nominalAccrued | contractId |
| --- | --- | --- | --- | --- | --- | --- | --- |
| PRD | 2020-01-02T00:00 | -10 | USD | 0 | 0 | 0 | brcsw01A |
| PRD | 2020-02-01T00:00 | 0 | USD | 0 | 0 | 0 | brcsw01A |
| TD | 2020-03-30T00:00 | 0 | USD | 0 | 0 | 0 | brcsw01A |
| MD | 2020-03-30T00:00 | 0 | USD | 0 | 0 | 0 | brcsw01A |
| XD | 2020-03-30T00:00 | 0 | USD | 0 | 0 | 0 | brcsw01A |
| STD | 2020-03-31T00:00 | -7 | USD | 0 | 0 | 0 | brcsw01A |

[Previous

Basic LAM Example](/docs/examples/basic-contract-types/example_LAM)[Next

Basic CAPFL Example](/docs/examples/basic-contract-types/example_CAPFL)

* [Contract Definition](/docs/examples/basic-contract-types/example_BCS#contract-definition)
* [Generated Events](/docs/examples/basic-contract-types/example_BCS#generated-events)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
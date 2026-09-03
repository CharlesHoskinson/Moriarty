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
* Basic LAX Example

On this page

Basic LAX Example
=================

Contract Definition
-------------------

```
import pandas as pd  
from awesome_actus_lib import LAX, PublicActusService  
  
contract = LAX(  
    contractID="lax01",  
    creatorID="Creator-01",  
    counterpartyID="Counterparty-01",  
    statusDate="2020-01-01T00:00:00",  
    contractRole="RPA",  
    calendar="NC",  
    businessDayConvention="SCF",  
    endOfMonthConvention="SD",  
    currency="EUR",  
    contractDealDate="2020-01-01T00:00:00",  
    initialExchangeDate="2020-01-02T00:00:00",  
    maturityDate="2024-12-31T00:00:00",  
    notionalPrincipal=100,  
    arrayCycleAnchorDateOfInterestPayment="2021-01-01T00:00:00",  
    arrayCycleOfInterestPayment="P1YL1",  
    nominalInterestRate=0.05,  
    dayCountConvention='30E360',  
    arrayCycleAnchorDateOfPrincipalRedemption="2021-01-01T00:00:00",  
    arrayCycleOfPrincipalRedemption="P1YL1",  
    arrayNextPrincipalRedemptionPayment=20,  
    arrayIncreaseDecrease="DEC"  
)  
  
service = PublicActusService()  
event_stream = service.generateEvents(portfolio=contract, riskFactors=[])  
print(event_stream.events_df)
```

Generated Events
----------------

| type | time | payoff | currency | nominalValue | nominalRate | nominalAccrued | contractId |
| --- | --- | --- | --- | --- | --- | --- | --- |
| IED | 2020-01-02T00:00 | -100 | EUR | 100 | 0.05 | 0 | lax01 |
| PR | 2021-01-01T00:00 | 20 | EUR | 80 | 0.05 | 4.98611 | lax01 |
| IP | 2021-01-01T00:00 | 4.98611 | EUR | 80 | 0.05 | 0 | lax01 |
| PR | 2022-01-01T00:00 | 20 | EUR | 60 | 0.05 | 4 | lax01 |
| IP | 2022-01-01T00:00 | 4 | EUR | 60 | 0.05 | 0 | lax01 |
| PR | 2023-01-01T00:00 | 20 | EUR | 40 | 0.05 | 3 | lax01 |
| IP | 2023-01-01T00:00 | 3 | EUR | 40 | 0.05 | 0 | lax01 |
| PR | 2024-01-01T00:00 | 20 | EUR | 20 | 0.05 | 2 | lax01 |
| IP | 2024-01-01T00:00 | 2 | EUR | 20 | 0.05 | 0 | lax01 |
| PR | 2025-01-01T00:00 | 20 | EUR | 0 | 0.05 | 1 | lax01 |
| IP | 2025-01-01T00:00 | 1 | EUR | 0 | 0.05 | 0 | lax01 |
| PR | 2026-01-01T00:00 | 0 | EUR | 0 | 0.05 | 0 | lax01 |
| IP | 2026-01-01T00:00 | 0 | EUR | 0 | 0.05 | 0 | lax01 |
| PR | 2027-01-01T00:00 | 0 | EUR | 0 | 0.05 | 0 | lax01 |
| IP | 2027-01-01T00:00 | 0 | EUR | 0 | 0.05 | 0 | lax01 |
| PR | 2028-01-01T00:00 | 0 | EUR | 0 | 0.05 | 0 | lax01 |
| IP | 2028-01-01T00:00 | 0 | EUR | 0 | 0.05 | 0 | lax01 |
| PR | 2029-01-01T00:00 | 0 | EUR | 0 | 0.05 | 0 | lax01 |
| IP | 2029-01-01T00:00 | 0 | EUR | 0 | 0.05 | 0 | lax01 |
| PR | 2030-01-01T00:00 | 0 | EUR | 0 | 0.05 | 0 | lax01 |
| IP | 2030-01-01T00:00 | 0 | EUR | 0 | 0.05 | 0 | lax01 |
| IP | 2030-06-26T16:50:27.115721636 | 0 | EUR | 0 | 0.05 | 0 | lax01 |
| MD | 2030-06-26T16:50:27.115721636 | 0 | EUR | 0 | 0.05 | 0 | lax01 |

[Previous

Basic COM Example](/docs/examples/basic-contract-types/example_COM)[Next

Basic FXOUT Example](/docs/examples/basic-contract-types/example_FXOUT)

* [Contract Definition](/docs/examples/basic-contract-types/example_LAX#contract-definition)
* [Generated Events](/docs/examples/basic-contract-types/example_LAX#generated-events)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
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
* Basic LAM Example

On this page

Basic LAM Example
=================

Contract Definition
-------------------

```
import pandas as pd  
from awesome_actus_lib import LAM, PublicActusService, ReferenceIndex  
  
contract = LAM(  
    contractID="lam01",  
    contractRole="RPA",  
    contractDealDate="2012-12-28T00:00:00",  
    initialExchangeDate="2013-01-01T00:00:00",  
    statusDate="2012-12-30T00:00:00",  
    notionalPrincipal=5000,  
    cycleAnchorDateOfPrincipalRedemption="2013-02-01T00:00:00",  
    nextPrincipalRedemptionPayment=500,  
    dayCountConvention="A365",  
    nominalInterestRate=0.08,  
    currency="USD",  
    cycleOfPrincipalRedemption="P1ML0",  
    cycleAnchorDateOfRateReset="2013-04-01T00:00:00",  
    cycleOfRateReset="P3ML1",  
    rateMultiplier=1,  
    rateSpread=0.1,  
    fixingDays="P0D",  
    cycleAnchorDateOfInterestPayment="2013-02-01T00:00:00",  
    cycleOfInterestPayment="P1ML0",  
    endOfMonthConvention="SD",  
    interestCalculationBase="NT",  
    marketObjectCodeOfRateReset="USD.SWP",  
    interestCalculationBaseAmount=0,  
    creatorID="Creator-01",  
    counterpartyID="Counterparty-01"  
)  
rf_data_USD_SWP = pd.DataFrame({  
    "date": ["2013-04-01", "2013-07-01", "2013-10-01", "2014-01-01"],  
    "value": [0.01056790123456790, 0.01167901234567900, 0.01279012345679010, 0.01390123456790120]  
})  
rf_USD_SWP = ReferenceIndex(marketObjectCode="USD.SWP", source=rf_data_USD_SWP)  
service = PublicActusService()  
event_stream = service.generateEvents(portfolio=contract, riskFactors=[rf_USD_SWP])  
print(event_stream.events_df)
```

Generated Events
----------------

| type | time | payoff | currency | nominalValue | nominalRate | nominalAccrued | contractId |
| --- | --- | --- | --- | --- | --- | --- | --- |
| IED | 2013-01-01T00:00 | -5000 | USD | 5000 | 0.08 | 0 | lam01 |
| PR | 2013-02-01T00:00 | 500 | USD | 4500 | 0.08 | 33.9726 | lam01 |
| IP | 2013-02-01T00:00 | 33.9726 | USD | 4500 | 0.08 | 0 | lam01 |
| PR | 2013-03-01T00:00 | 500 | USD | 4000 | 0.08 | 27.6164 | lam01 |
| IP | 2013-03-01T00:00 | 27.6164 | USD | 4000 | 0.08 | 0 | lam01 |
| PR | 2013-04-01T00:00 | 500 | USD | 3500 | 0.08 | 27.1781 | lam01 |
| IP | 2013-04-01T00:00 | 27.1781 | USD | 3500 | 0.08 | 0 | lam01 |
| RR | 2013-04-01T00:00 | 0 | USD | 3500 | 0.110568 | 0 | lam01 |
| PR | 2013-05-01T00:00 | 500 | USD | 3000 | 0.110568 | 31.8072 | lam01 |
| IP | 2013-05-01T00:00 | 31.8072 | USD | 3000 | 0.110568 | 0 | lam01 |
| PR | 2013-06-01T00:00 | 500 | USD | 2500 | 0.110568 | 28.1721 | lam01 |
| IP | 2013-06-01T00:00 | 28.1721 | USD | 2500 | 0.110568 | 0 | lam01 |
| PR | 2013-07-01T00:00 | 500 | USD | 2000 | 0.110568 | 22.7194 | lam01 |
| IP | 2013-07-01T00:00 | 22.7194 | USD | 2000 | 0.110568 | 0 | lam01 |
| RR | 2013-07-01T00:00 | 0 | USD | 2000 | 0.111679 | 0 | lam01 |
| PR | 2013-08-01T00:00 | 500 | USD | 1500 | 0.111679 | 18.9701 | lam01 |
| IP | 2013-08-01T00:00 | 18.9701 | USD | 1500 | 0.111679 | 0 | lam01 |
| PR | 2013-09-01T00:00 | 500 | USD | 1000 | 0.111679 | 14.2276 | lam01 |
| IP | 2013-09-01T00:00 | 14.2276 | USD | 1000 | 0.111679 | 0 | lam01 |
| PR | 2013-10-01T00:00 | 500 | USD | 500 | 0.111679 | 9.1791 | lam01 |
| IP | 2013-10-01T00:00 | 9.1791 | USD | 500 | 0.111679 | 0 | lam01 |
| RR | 2013-10-01T00:00 | 0 | USD | 500 | 0.11279 | 0 | lam01 |
| IP | 2013-11-01T00:00 | 4.78972 | USD | 500 | 0.11279 | 0 | lam01 |
| MD | 2013-11-01T00:00 | 500 | USD | 0 | 0.11279 | 0 | lam01 |

[Previous

Basic OPTNS Example](/docs/examples/basic-contract-types/example_OPTNS)[Next

Basic BCS Example](/docs/examples/basic-contract-types/example_BCS)

* [Contract Definition](/docs/examples/basic-contract-types/example_LAM#contract-definition)
* [Generated Events](/docs/examples/basic-contract-types/example_LAM#generated-events)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
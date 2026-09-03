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
* Basic NAM Example

On this page

Basic NAM Example
=================

Contract Definition
-------------------

```
import pandas as pd  
from awesome_actus_lib import NAM, PublicActusService, ReferenceIndex  
  
contract = NAM(  
    contractID="nam01",  
    contractRole="RPA",  
    contractDealDate="2012-12-28T00:00:00",  
    initialExchangeDate="2013-01-01T00:00:00",  
    statusDate="2012-12-30T00:00:00",  
    notionalPrincipal=5000,  
    cycleAnchorDateOfPrincipalRedemption="2013-02-01T00:00:00",  
    cycleOfPrincipalRedemption="P1ML0",  
    nextPrincipalRedemptionPayment=500,  
    dayCountConvention="A365",  
    nominalInterestRate=0.08,  
    currency="USD",  
    maturityDate="2013-12-01T00:00:00",  
    cycleAnchorDateOfRateReset="2013-04-01T00:00:00",  
    cycleOfRateReset="P3ML1",  
    marketObjectCodeOfRateReset="LIBOR_USD",  
    rateSpread=0.1,  
    fixingDays="P0D",  
    cycleAnchorDateOfInterestPayment="2013-02-01T00:00:00",  
    cycleOfInterestPayment="P1ML0",  
    interestCalculationBase="NT",  
    interestCalculationBaseAmount=0,  
    creatorID="Creator-01",  
    counterpartyID="Counterparty-01"  
)  
rf_data_LIBOR_USD = pd.DataFrame({  
    "date": ["2013-04-01", "2013-07-01", "2013-10-01", "2014-01-01", "2014-04-01", "2014-07-01", "2014-10-01", "2015-01-01", "2015-04-01", "2015-07-01", "2015-10-01"],  
    "value": [0.010567901234567900, 0.011679012345679000, 0.012790123456790100, 0.013901234567901200, 0.015012345679012300, 0.016123456790123400, 0.017234567901234500, 0.018345679012345600, 0.019456790123456800, 0.020567901234567800, 0.021679012345679000]  
})  
rf_LIBOR_USD = ReferenceIndex(marketObjectCode="LIBOR_USD", source=rf_data_LIBOR_USD)  
service = PublicActusService()  
event_stream = service.generateEvents(portfolio=contract, riskFactors=[rf_LIBOR_USD])  
print(event_stream.events_df)
```

Generated Events
----------------

| type | time | payoff | currency | nominalValue | nominalRate | nominalAccrued | contractId |
| --- | --- | --- | --- | --- | --- | --- | --- |
| IED | 2013-01-01T00:00 | -5000 | USD | 5000 | 0.08 | 0 | nam01 |
| PR | 2013-02-01T00:00 | 466.027 | USD | 4533.97 | 0.08 | 33.9726 | nam01 |
| IP | 2013-02-01T00:00 | 33.9726 | USD | 4533.97 | 0.08 | 0 | nam01 |
| PR | 2013-03-01T00:00 | 472.175 | USD | 4061.8 | 0.08 | 27.8249 | nam01 |
| IP | 2013-03-01T00:00 | 27.8249 | USD | 4061.8 | 0.08 | 0 | nam01 |
| PR | 2013-04-01T00:00 | 472.402 | USD | 3589.4 | 0.08 | 27.598 | nam01 |
| IP | 2013-04-01T00:00 | 27.598 | USD | 3589.4 | 0.08 | 0 | nam01 |
| RR | 2013-04-01T00:00 | 0 | USD | 3589.4 | 0.110568 | 0 | nam01 |
| PR | 2013-05-01T00:00 | 467.38 | USD | 3122.02 | 0.110568 | 32.6196 | nam01 |
| IP | 2013-05-01T00:00 | 32.6196 | USD | 3122.02 | 0.110568 | 0 | nam01 |
| PR | 2013-06-01T00:00 | 470.682 | USD | 2651.33 | 0.110568 | 29.3179 | nam01 |
| IP | 2013-06-01T00:00 | 29.3179 | USD | 2651.33 | 0.110568 | 0 | nam01 |
| PR | 2013-07-01T00:00 | 475.905 | USD | 2175.43 | 0.110568 | 24.0947 | nam01 |
| IP | 2013-07-01T00:00 | 24.0947 | USD | 2175.43 | 0.110568 | 0 | nam01 |
| RR | 2013-07-01T00:00 | 0 | USD | 2175.43 | 0.111679 | 0 | nam01 |
| PR | 2013-08-01T00:00 | 479.366 | USD | 1696.06 | 0.111679 | 20.6341 | nam01 |
| IP | 2013-08-01T00:00 | 20.6341 | USD | 1696.06 | 0.111679 | 0 | nam01 |
| PR | 2013-09-01T00:00 | 483.913 | USD | 1212.15 | 0.111679 | 16.0873 | nam01 |
| IP | 2013-09-01T00:00 | 16.0873 | USD | 1212.15 | 0.111679 | 0 | nam01 |
| PR | 2013-10-01T00:00 | 488.874 | USD | 723.275 | 0.111679 | 11.1264 | nam01 |
| IP | 2013-10-01T00:00 | 11.1264 | USD | 723.275 | 0.111679 | 0 | nam01 |
| RR | 2013-10-01T00:00 | 0 | USD | 723.275 | 0.11279 | 0 | nam01 |
| PR | 2013-11-01T00:00 | 493.071 | USD | 230.204 | 0.11279 | 6.92857 | nam01 |
| IP | 2013-11-01T00:00 | 6.92857 | USD | 230.204 | 0.11279 | 0 | nam01 |
| IP | 2013-12-01T00:00 | 2.13409 | USD | 230.204 | 0.11279 | 0 | nam01 |
| MD | 2013-12-01T00:00 | 230.204 | USD | 0 | 0.11279 | 0 | nam01 |

[Previous

Basic CEG Example](/docs/examples/basic-contract-types/example_CEG)[Next

Basic SWPPV Example](/docs/examples/basic-contract-types/example_SWPPV)

* [Contract Definition](/docs/examples/basic-contract-types/example_NAM#contract-definition)
* [Generated Events](/docs/examples/basic-contract-types/example_NAM#generated-events)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
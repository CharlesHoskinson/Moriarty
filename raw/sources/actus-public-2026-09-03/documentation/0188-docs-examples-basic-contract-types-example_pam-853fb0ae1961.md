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
* Basic Contract Type Examples

On this page

Basic PAM Example
=================

Contract Definition
-------------------

```
import pandas as pd  
from awesome_actus_lib import PAM, PublicActusService  
  
contract = PAM(  
    contractID="pam01",  
    statusDate="2012-12-30T00:00:00",  
    contractDealDate="2012-12-28T00:00:00",  
    currency="USD",  
    notionalPrincipal=3000,  
    initialExchangeDate="2013-01-01T00:00:00",  
    maturityDate="2014-01-01T00:00:00",  
    nominalInterestRate=0.1,  
    cycleAnchorDateOfInterestPayment="2013-01-01T00:00:00",  
    cycleOfInterestPayment="P1ML0",  
    dayCountConvention="30E360",  
    endOfMonthConvention="SD",  
    premiumDiscountAtIED=0,  
    rateMultiplier=1.0,  
    contractRole="RPA",  
    creatorID="Creator-01",  
    counterpartyID="Counterparty-01"  
)  
  
service = PublicActusService()  
event_stream = service.generateEvents(portfolio=contract, riskFactors=[])  
print(event_stream.events_df)
```

Generated Events
----------------

| type | time | payoff | currency | nominalValue | nominalRate | nominalAccrued | contractId |
| --- | --- | --- | --- | --- | --- | --- | --- |
| IED | 2013-01-01T00:00 | -3000 | USD | 3000 | 0.1 | 0 | pam01 |
| IP | 2013-01-01T00:00 | 0 | USD | 3000 | 0.1 | 0 | pam01 |
| IP | 2013-02-01T00:00 | 25 | USD | 3000 | 0.1 | 0 | pam01 |
| IP | 2013-03-01T00:00 | 25 | USD | 3000 | 0.1 | 0 | pam01 |
| IP | 2013-04-01T00:00 | 25 | USD | 3000 | 0.1 | 0 | pam01 |
| IP | 2013-05-01T00:00 | 25 | USD | 3000 | 0.1 | 0 | pam01 |
| IP | 2013-06-01T00:00 | 25 | USD | 3000 | 0.1 | 0 | pam01 |
| IP | 2013-07-01T00:00 | 25 | USD | 3000 | 0.1 | 0 | pam01 |
| IP | 2013-08-01T00:00 | 25 | USD | 3000 | 0.1 | 0 | pam01 |
| IP | 2013-09-01T00:00 | 25 | USD | 3000 | 0.1 | 0 | pam01 |
| IP | 2013-10-01T00:00 | 25 | USD | 3000 | 0.1 | 0 | pam01 |
| IP | 2013-11-01T00:00 | 25 | USD | 3000 | 0.1 | 0 | pam01 |
| IP | 2013-12-01T00:00 | 25 | USD | 3000 | 0.1 | 0 | pam01 |
| IP | 2014-01-01T00:00 | 25 | USD | 3000 | 0.1 | 0 | pam01 |
| MD | 2014-01-01T00:00 | 3000 | USD | 0 | 0.1 | 0 | pam01 |

[Previous

Configuration](/docs/getting-started/configuration)[Next

Basic PAM Example](/docs/examples/basic-contract-types/example_PAM)

* [Contract Definition](/docs/examples/basic-contract-types/example_PAM#contract-definition)
* [Generated Events](/docs/examples/basic-contract-types/example_PAM#generated-events)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
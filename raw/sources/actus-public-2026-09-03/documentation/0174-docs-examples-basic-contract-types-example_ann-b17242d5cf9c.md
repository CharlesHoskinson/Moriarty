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
* Basic ANN Example

On this page

Basic ANN Example
=================

Contract Definition
-------------------

```
import pandas as pd  
from awesome_actus_lib import ANN, PublicActusService  
  
contract = ANN(  
    contractID="ann01",  
    contractRole="RPA",  
    contractDealDate="2012-12-28T00:00:00",  
    initialExchangeDate="2013-01-01T00:00:00",  
    statusDate="2012-12-30T00:00:00",  
    notionalPrincipal=5000,  
    cycleAnchorDateOfPrincipalRedemption="2013-02-01T00:00:00",  
    nextPrincipalRedemptionPayment=434.866594118346,  
    dayCountConvention="A365",  
    nominalInterestRate=0.08,  
    currency="USD",  
    cycleOfPrincipalRedemption="P1ML0",  
    maturityDate="2014-01-01T00:00:00",  
    rateMultiplier=1.0,  
    rateSpread=0.0,  
    fixingDays="P0D",  
    cycleAnchorDateOfInterestPayment="2013-02-01T00:00:00",  
    cycleOfInterestPayment="P1ML0",  
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
| IED | 2013-01-01T00:00 | -5000 | USD | 5000 | 0.08 | 0 | ann01 |
| PR | 2013-02-01T00:00 | 400.894 | USD | 4599.11 | 0.08 | 33.9726 | ann01 |
| IP | 2013-02-01T00:00 | 33.9726 | USD | 4599.11 | 0.08 | 0 | ann01 |
| PR | 2013-03-01T00:00 | 406.642 | USD | 4192.46 | 0.08 | 28.2247 | ann01 |
| IP | 2013-03-01T00:00 | 28.2247 | USD | 4192.46 | 0.08 | 0 | ann01 |
| PR | 2013-04-01T00:00 | 406.381 | USD | 3786.08 | 0.08 | 28.4858 | ann01 |
| IP | 2013-04-01T00:00 | 28.4858 | USD | 3786.08 | 0.08 | 0 | ann01 |
| PR | 2013-05-01T00:00 | 409.972 | USD | 3376.11 | 0.08 | 24.8948 | ann01 |
| IP | 2013-05-01T00:00 | 24.8948 | USD | 3376.11 | 0.08 | 0 | ann01 |
| PR | 2013-06-01T00:00 | 411.928 | USD | 2964.18 | 0.08 | 22.9391 | ann01 |
| IP | 2013-06-01T00:00 | 22.9391 | USD | 2964.18 | 0.08 | 0 | ann01 |
| PR | 2013-07-01T00:00 | 415.376 | USD | 2548.81 | 0.08 | 19.4905 | ann01 |
| IP | 2013-07-01T00:00 | 19.4905 | USD | 2548.81 | 0.08 | 0 | ann01 |
| PR | 2013-08-01T00:00 | 417.549 | USD | 2131.26 | 0.08 | 17.3179 | ann01 |
| IP | 2013-08-01T00:00 | 17.3179 | USD | 2131.26 | 0.08 | 0 | ann01 |
| PR | 2013-09-01T00:00 | 420.386 | USD | 1710.87 | 0.08 | 14.4809 | ann01 |
| IP | 2013-09-01T00:00 | 14.4809 | USD | 1710.87 | 0.08 | 0 | ann01 |
| PR | 2013-10-01T00:00 | 423.617 | USD | 1287.26 | 0.08 | 11.2496 | ann01 |
| IP | 2013-10-01T00:00 | 11.2496 | USD | 1287.26 | 0.08 | 0 | ann01 |
| PR | 2013-11-01T00:00 | 426.12 | USD | 861.136 | 0.08 | 8.74629 | ann01 |
| IP | 2013-11-01T00:00 | 8.74629 | USD | 861.136 | 0.08 | 0 | ann01 |
| PR | 2013-12-01T00:00 | 429.204 | USD | 431.932 | 0.08 | 5.66227 | ann01 |
| IP | 2013-12-01T00:00 | 5.66227 | USD | 431.932 | 0.08 | 0 | ann01 |
| IP | 2014-01-01T00:00 | 2.93477 | USD | 431.932 | 0.08 | 0 | ann01 |
| MD | 2014-01-01T00:00 | 431.932 | USD | 0 | 0.08 | 0 | ann01 |

[Previous

Basic PAM Example](/docs/examples/basic-contract-types/example_PAM)[Next

Basic OPTNS Example](/docs/examples/basic-contract-types/example_OPTNS)

* [Contract Definition](/docs/examples/basic-contract-types/example_ANN#contract-definition)
* [Generated Events](/docs/examples/basic-contract-types/example_ANN#generated-events)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
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

    - [Overview](/docs/standard/overview)
    - [Core Concepts](/docs/standard/core-concepts)
    - [Contract Types](/docs/contract-types/PAM)

      * [Principal at Maturity (PAM)](/docs/contract-types/PAM)
      * [Annuity (ANN)](/docs/contract-types/ANN)
      * [Option (OPTNS)](/docs/contract-types/OPTNS)
      * [Linear Amortizer (LAM)](/docs/contract-types/LAM)
      * [Boundary Controlled Switch (BCS)](/docs/contract-types/BCS)
      * [Cap Floors (CAPFL)](/docs/contract-types/CAPFL)
      * [Cash (CSH)](/docs/contract-types/CSH)
      * [Collateral (CEC)](/docs/contract-types/CEC)
      * [Commodity (COM)](/docs/contract-types/COM)
      * [Exotic Linear Amortizer (LAX)](/docs/contract-types/LAX)
      * [Foreign Ex-change Outright (FXOUT)](/docs/contract-types/FXOUT)
      * [Future (FUTUR)](/docs/contract-types/FUTUR)
      * [Guarantee (CEG)](/docs/contract-types/CEG)
      * [Negative Amortizer (NAM)](/docs/contract-types/NAM)
      * [Plain Vanilla Swap (SWPPV)](/docs/contract-types/SWPPV)
      * [Stock (STK)](/docs/contract-types/STK)
      * [Undefined Maturity Profile (UMP)](/docs/contract-types/UMP)
      * [Swap (SWAPS)](/docs/contract-types/SWAPS)
      * [Call Money (CLM)](/docs/contract-types/CLM)
    - [Contract Terms](/docs/contract-terms/contract-terms-overview)
  + [Getting Started](/docs/getting-started/installation)
  + [Examples](/docs/examples/basic-contract-types/example_PAM)
  + [Guides](/docs/guides/common-patterns)
* [[ACTUSFRF] CORE LICENSE 1.0](/docs/license)

* [Awesome Python Library](/docs/category/awesome-python-library)
* [ACTUS Standard](/docs/standard/overview)
* [Contract Types](/docs/contract-types/PAM)
* Call Money (CLM)

On this page

Call Money (CLM)
================

Description
-----------

Loans that are rolled over as long as they are not called. Once called it has to be paid back after the stipulated notice period.

Real-world Instrument Examples (but not limited to)
---------------------------------------------------

Interbank loans with call features.

Required Terms
--------------

* contractDealDate
* contractID
* contractRole
* contractType
* counterpartyID
* creatorID
* currency
* dayCountConvention
* initialExchangeDate
* nominalInterestRate
* notionalPrincipal
* statusDate
* xDayNotice

Conditional Groups
------------------

### Group 1

* **Drivers**: feeRate
* **Required if triggered**: feeBasis
* **Optional**: cycleAnchorDateOfFee, cycleOfFee, feeAccrued

### Group 9

* **Drivers**: cycleAnchorDateOfRateReset, cycleOfRateReset
* **Required if triggered**: rateSpread, marketObjectCodeOfRateReset
* **Optional**: fixingPeriod, nextResetRate, rateMultiplier

Standalone Optional Terms
-------------------------

* accruedInterest
* businessDayConvention
* calendar
* contractPerformance
* cycleAnchorDateOfInterestPayment
* cycleOfInterestPayment
* delinquencyPeriod
* delinquencyRate
* endOfMonthConvention
* gracePeriod
* maturityDate
* nonPerformingDate
* prepaymentPeriod
* seniority
* settlementCurrency

Notes
-----

* `contractType` is automatically set to "CLM" when using the class.

[Previous

Swap (SWAPS)](/docs/contract-types/SWAPS)[Next

Contract Terms Overview](/docs/contract-terms/contract-terms-overview)

* [Description](/docs/contract-types/CLM#description)
* [Real-world Instrument Examples (but not limited to)](/docs/contract-types/CLM#real-world-instrument-examples-but-not-limited-to)
* [Required Terms](/docs/contract-types/CLM#required-terms)
* [Conditional Groups](/docs/contract-types/CLM#conditional-groups)
  + [Group 1](/docs/contract-types/CLM#group-1)
  + [Group 9](/docs/contract-types/CLM#group-9)
* [Standalone Optional Terms](/docs/contract-types/CLM#standalone-optional-terms)
* [Notes](/docs/contract-types/CLM#notes)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
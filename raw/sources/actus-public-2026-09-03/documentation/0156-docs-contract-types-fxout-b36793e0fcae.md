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
* Foreign Ex-change Outright (FXOUT)

On this page

Foreign Ex-change Outright (FXOUT)
==================================

Description
-----------

Two parties agree to exchange two fixed cash flows in different currencies at a certain point in time in future.

Real-world Instrument Examples (but not limited to)
---------------------------------------------------

Any FX-outright transaction at a future date. This is also the underlying of FX-options and FX futures.

Required Terms
--------------

* contractDealDate
* contractID
* contractRole
* contractType
* counterpartyID
* creatorID
* currency
* currency2
* maturityDate
* notionalPrincipal
* notionalPrincipal2
* statusDate

Conditional Groups
------------------

### Group 5

* **Drivers**: purchaseDate
* **Required if triggered**: priceAtPurchaseDate
* **Optional**: None

### Group 6

* **Drivers**: terminationDate
* **Required if triggered**: priceAtTerminationDate
* **Optional**: None

### Group 7

* **Drivers**: exerciseDate
* **Required if triggered**: exerciseAmount
* **Optional**: None

Standalone Optional Terms
-------------------------

* businessDayConvention
* calendar
* contractPerformance
* delinquencyPeriod
* delinquencyRate
* deliverySettlement
* endOfMonthConvention
* gracePeriod
* marketObjectCode
* marketValueObserved
* nonPerformingDate
* seniority
* settlementCurrency
* settlementPeriod

Notes
-----

* `contractType` is automatically set to "FXOUT" when using the class.

[Previous

Exotic Linear Amortizer (LAX)](/docs/contract-types/LAX)[Next

Future (FUTUR)](/docs/contract-types/FUTUR)

* [Description](/docs/contract-types/FXOUT#description)
* [Real-world Instrument Examples (but not limited to)](/docs/contract-types/FXOUT#real-world-instrument-examples-but-not-limited-to)
* [Required Terms](/docs/contract-types/FXOUT#required-terms)
* [Conditional Groups](/docs/contract-types/FXOUT#conditional-groups)
  + [Group 5](/docs/contract-types/FXOUT#group-5)
  + [Group 6](/docs/contract-types/FXOUT#group-6)
  + [Group 7](/docs/contract-types/FXOUT#group-7)
* [Standalone Optional Terms](/docs/contract-types/FXOUT#standalone-optional-terms)
* [Notes](/docs/contract-types/FXOUT#notes)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
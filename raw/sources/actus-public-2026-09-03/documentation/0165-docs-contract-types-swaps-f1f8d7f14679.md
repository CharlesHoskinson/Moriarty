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
* Swap (SWAPS)

On this page

Swap (SWAPS)
============

Description
-----------

Exchange of two basic CTÂ´s (PAM, ANN etc.). Normally one is fixed, the other variable. However all variants possible including different currencies for cross currency swaps, basic swaps or even different principal exchange programs.

Real-world Instrument Examples (but not limited to)
---------------------------------------------------

All kind of swaps. The variety is defined by the underlying CTÂ´s which often are PAM and ANN in all its flavors. With each new basic CT the variety rises.

Required Terms
--------------

* contractDealDate
* contractID
* contractRole
* contractStructure
* contractType
* counterpartyID
* creatorID
* currency
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

Standalone Optional Terms
-------------------------

* contractPerformance
* delinquencyPeriod
* delinquencyRate
* deliverySettlement
* gracePeriod
* marketObjectCode
* marketValueObserved
* nonPerformingDate
* seniority
* settlementCurrency

Notes
-----

* `contractType` is automatically set to "SWAPS" when using the class.

[Previous

Undefined Maturity Profile (UMP)](/docs/contract-types/UMP)[Next

Call Money (CLM)](/docs/contract-types/CLM)

* [Description](/docs/contract-types/SWAPS#description)
* [Real-world Instrument Examples (but not limited to)](/docs/contract-types/SWAPS#real-world-instrument-examples-but-not-limited-to)
* [Required Terms](/docs/contract-types/SWAPS#required-terms)
* [Conditional Groups](/docs/contract-types/SWAPS#conditional-groups)
  + [Group 5](/docs/contract-types/SWAPS#group-5)
  + [Group 6](/docs/contract-types/SWAPS#group-6)
* [Standalone Optional Terms](/docs/contract-types/SWAPS#standalone-optional-terms)
* [Notes](/docs/contract-types/SWAPS#notes)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
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
* Guarantee (CEG)

On this page

Guarantee (CEG)
===============

Description
-----------

Guarantee creates a relationship between a guarantor, an obligee and a debtor, moving the exposure from the debtor to the guarantor.

Real-world Instrument Examples (but not limited to)
---------------------------------------------------

Personal guarantee. Government guarantee. Underlyings of CDOÂ´s.

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
* guaranteedExposure
* statusDate

Conditional Groups
------------------

### Group 1

* **Drivers**: feeRate
* **Required if triggered**: feeBasis
* **Optional**: cycleAnchorDateOfFee, cycleOfFee, feeAccrued

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
* coverageOfCreditEnhancement
* creditEventTypeCovered
* delinquencyPeriod
* delinquencyRate
* endOfMonthConvention
* gracePeriod
* maturityDate
* nonPerformingDate
* notionalPrincipal
* settlementCurrency
* settlementPeriod

Notes
-----

* `contractType` is automatically set to "CEG" when using the class.

[Previous

Future (FUTUR)](/docs/contract-types/FUTUR)[Next

Negative Amortizer (NAM)](/docs/contract-types/NAM)

* [Description](/docs/contract-types/CEG#description)
* [Real-world Instrument Examples (but not limited to)](/docs/contract-types/CEG#real-world-instrument-examples-but-not-limited-to)
* [Required Terms](/docs/contract-types/CEG#required-terms)
* [Conditional Groups](/docs/contract-types/CEG#conditional-groups)
  + [Group 1](/docs/contract-types/CEG#group-1)
  + [Group 5](/docs/contract-types/CEG#group-5)
  + [Group 6](/docs/contract-types/CEG#group-6)
  + [Group 7](/docs/contract-types/CEG#group-7)
* [Standalone Optional Terms](/docs/contract-types/CEG#standalone-optional-terms)
* [Notes](/docs/contract-types/CEG#notes)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
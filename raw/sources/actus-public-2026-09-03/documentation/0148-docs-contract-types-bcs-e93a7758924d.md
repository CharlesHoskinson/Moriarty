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
* Boundary Controlled Switch (BCS)

On this page

Boundary Controlled Switch (BCS)
================================

Description
-----------

A boundary controlled switch is a derivative contract with subcontract legs which can be activated (knocked in) or extinguished (knocked out) when the underlying asset price reaches a specified value. The underlying asset may be a stock, index, or exchange-traded fund. Boundary controlled switch contracts with a single boundary are currently defined.

Real-world Instrument Examples (but not limited to)
---------------------------------------------------

Knock-in and Knock-out barrier options with a single boundary. Bonus contracts with payout when underlying asset price remains above or below a specified level for specified period.

Required Terms
--------------

* boundaryDirection
* boundaryEffect
* boundaryLegInitiallyActive
* boundaryMonitoringAnchorDate
* boundaryMonitoringCycle
* boundaryMonitoringEndDate
* boundaryValue
* contractDealDate
* contractID
* contractRole
* contractStructure
* contractType
* maturityDate
* priceAtPurchaseDate
* purchaseDate
* statusDate

Conditional Groups
------------------

### Group 1

* **Drivers**: terminationDate
* **Required if triggered**: priceAtTerminationDate
* **Optional**: None

Standalone Optional Terms
-------------------------

* businessDayConvention
* calendar
* deliverySettlement
* endOfMonthConvention
* marketObjectCode
* marketValueObserved
* settlementPeriod

Notes
-----

* `contractType` is automatically set to "BCS" when using the class.

[Previous

Linear Amortizer (LAM)](/docs/contract-types/LAM)[Next

Cap Floors (CAPFL)](/docs/contract-types/CAPFL)

* [Description](/docs/contract-types/BCS#description)
* [Real-world Instrument Examples (but not limited to)](/docs/contract-types/BCS#real-world-instrument-examples-but-not-limited-to)
* [Required Terms](/docs/contract-types/BCS#required-terms)
* [Conditional Groups](/docs/contract-types/BCS#conditional-groups)
  + [Group 1](/docs/contract-types/BCS#group-1)
* [Standalone Optional Terms](/docs/contract-types/BCS#standalone-optional-terms)
* [Notes](/docs/contract-types/BCS#notes)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
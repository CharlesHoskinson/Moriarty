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
* Exotic Linear Amortizer (LAX)

On this page

Exotic Linear Amortizer (LAX)
=============================

Description
-----------

Exotic version of LAM. However step ups with respect to (i) Principal, (ii) Interest rates are possible. Highly flexible to match totally irregular principal payments. Principal can also be paid out in steps.

Real-world Instrument Examples (but not limited to)
---------------------------------------------------

A special version of this kind are teaser rate loans and mortgages.

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

Conditional Groups
------------------

### Group 1

* **Drivers**: feeRate
* **Required if triggered**: feeBasis
* **Optional**: cycleAnchorDateOfFee, cycleOfFee, feeAccrued

### Group 2

* **Drivers**: arrayCycleAnchorDateOfInterestPayment, arrayCycleOfInterestPayment
* **Required if triggered**: None
* **Optional**: cyclePointOfInterestPayment

### Group 3

* **Drivers**: interestCalculationBase
* **Required if triggered**: interestCalculationBaseAmount
* **Optional**: cycleAnchorDateOfInterestCalculationBase, cycleOfInterestCalculationBase

### Group 4

* **Drivers**: arrayCycleAnchorDateOfPrincipalRedemption, arrayCycleOfPrincipalRedemption, arrayNextPrincipalRedemptionPayment, arrayIncreaseDecrease
* **Required if triggered**: None
* **Optional**: None

### Group 5

* **Drivers**: purchaseDate
* **Required if triggered**: priceAtPurchaseDate
* **Optional**: None

### Group 6

* **Drivers**: terminationDate
* **Required if triggered**: priceAtTerminationDate
* **Optional**: None

### Group 7

* **Drivers**: scalingEffect
* **Required if triggered**: marketObjectCodeOfScalingIndex, scalingIndexAtContractDealDate, notionalScalingMultiplier, interestScalingMultiplier
* **Optional**: cycleAnchorDateOfScalingIndex, cycleOfScalingIndex

### Group 8

* **Drivers**: prepaymentEffect
* **Required if triggered**: None
* **Optional**: prepaymentPeriod, optionExerciseEndDate, cycleAnchorDateOfOptionality, cycleOfOptionality, penaltyType, penaltyRate

### Group 9

* **Drivers**: arrayCycleAnchorDateOfRateReset
* **Required if triggered**: arrayRate, arrayFixedVariable, marketObjectCodeOfRateReset
* **Optional**: arrayCycleOfRateReset, lifeCap, lifeFloor, periodCap, periodFloor, cyclePointOfRateReset, fixingPeriod, nextResetRate, rateMultiplier

Standalone Optional Terms
-------------------------

* accruedInterest
* businessDayConvention
* calendar
* capitalizationEndDate
* contractPerformance
* delinquencyPeriod
* delinquencyRate
* endOfMonthConvention
* gracePeriod
* marketObjectCode
* marketValueObserved
* maturityDate
* nonPerformingDate
* premiumDiscountAtIED
* seniority
* settlementCurrency

Notes
-----

* `contractType` is automatically set to "LAX" when using the class.

[Previous

Commodity (COM)](/docs/contract-types/COM)[Next

Foreign Ex-change Outright (FXOUT)](/docs/contract-types/FXOUT)

* [Description](/docs/contract-types/LAX#description)
* [Real-world Instrument Examples (but not limited to)](/docs/contract-types/LAX#real-world-instrument-examples-but-not-limited-to)
* [Required Terms](/docs/contract-types/LAX#required-terms)
* [Conditional Groups](/docs/contract-types/LAX#conditional-groups)
  + [Group 1](/docs/contract-types/LAX#group-1)
  + [Group 2](/docs/contract-types/LAX#group-2)
  + [Group 3](/docs/contract-types/LAX#group-3)
  + [Group 4](/docs/contract-types/LAX#group-4)
  + [Group 5](/docs/contract-types/LAX#group-5)
  + [Group 6](/docs/contract-types/LAX#group-6)
  + [Group 7](/docs/contract-types/LAX#group-7)
  + [Group 8](/docs/contract-types/LAX#group-8)
  + [Group 9](/docs/contract-types/LAX#group-9)
* [Standalone Optional Terms](/docs/contract-types/LAX#standalone-optional-terms)
* [Notes](/docs/contract-types/LAX#notes)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
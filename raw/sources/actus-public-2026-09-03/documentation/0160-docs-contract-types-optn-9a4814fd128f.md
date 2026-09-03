[Skip to main content](#__docusaurus_skipToContent_fallback)

[![Actus Logo](/img/ActusLogoRGB.jpg)![Actus Logo](/img/ActusLogoRGB.jpg)](/)[Dictionary](https://www.actusfrf.org/dictionary)[Taxonomy](https://www.actusfrf.org/taxonomy)

[GitHub](https://github.com/actusfrf)

On this page

Option (OPTN)
=============

Description
-----------

Calculates straight option pay-off for any basic CT as underlying (PAM, ANN etc.) but also SWAPS, FXOUT, STK and COM. Single, periodic and continuous strike is supported.

Real-world Instrument Examples (but not limited to)
---------------------------------------------------

European, American and Bermudan options with Interest rate, FX and stock futures as underlying instruments.

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
* maturityDate
* optionExerciseEndDate
* optionExerciseType
* optionStrike1
* optionType
* priceAtPurchaseDate
* purchaseDate
* statusDate

Conditional Groups
------------------

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
* cycleAnchorDateOfOptionality
* cycleOfOptionality
* delinquencyPeriod
* delinquencyRate
* deliverySettlement
* endOfMonthConvention
* gracePeriod
* marketObjectCode
* marketValueObserved
* nonPerformingDate
* optionStrike2
* seniority
* settlementCurrency
* settlementPeriod

Notes
-----

* `contractType` is automatically set to "OPTN" when using the class.

* [Description](/docs/contract-types/OPTN#description)
* [Real-world Instrument Examples (but not limited to)](/docs/contract-types/OPTN#real-world-instrument-examples-but-not-limited-to)
* [Required Terms](/docs/contract-types/OPTN#required-terms)
* [Conditional Groups](/docs/contract-types/OPTN#conditional-groups)
  + [Group 6](/docs/contract-types/OPTN#group-6)
  + [Group 7](/docs/contract-types/OPTN#group-7)
* [Standalone Optional Terms](/docs/contract-types/OPTN#standalone-optional-terms)
* [Notes](/docs/contract-types/OPTN#notes)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
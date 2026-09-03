[Skip to main content](#__docusaurus_skipToContent_fallback)

[![Actus Logo](/img/ActusLogoRGB.jpg)![Actus Logo](/img/ActusLogoRGB.jpg)](/)[Dictionary](https://www.actusfrf.org/dictionary)[Taxonomy](https://www.actusfrf.org/taxonomy)

[GitHub](https://github.com/actusfrf)

* [Welcome to ACTUS Documentation](/docs/intro)
* [Introduction to ACTUS](/docs/category/introduction-to-actus)
* [ACTUS Quick Start](/docs/quickstart)
* [Quickstart Extension for ACTUS Risk Factors](/docs/category/1-extention-for-actus-risk-factors)

  + [1. Extention for ACTUS Risk Factors](/docs/category/1-extention-for-actus-risk-factors)
  + [2. Create sample Risk Factors in Mongodb](/docs/category/2-create-sample-risk-factors-in-mongodb)
  + [3. Simulate ACTUS contract with Market Risk Scenario](/docs/category/3-simulate-actus-contract-with-market-risk-scenario)

    - [Contract Specifications](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/simple_ACTUS_contract)
    - [Contract Simulation](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/with_risk)
    - [Cash Flow Simulations](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/output_cashflow_event_series)
  + [4. Risk Modeling for Contract and Counterparty Behaviors](/docs/category/4-risk-modeling-for-contract-and-counterparty-behaviors)
  + [5. Future Enhancements](/docs/category/5-future-enhancements-)
* [Demos and Guides](/docs/actus-demo/demo-user-guide)
* [ACTUS Competition Pre-Announcement](/docs/competition)
* [Awesome Python Library](/docs/category/awesome-python-library)
* [[ACTUSFRF] CORE LICENSE 1.0](/docs/license)

* Quickstart Extension for ACTUS Risk Factors
* [3. Simulate ACTUS contract with Market Risk Scenario](/docs/category/3-simulate-actus-contract-with-market-risk-scenario)
* Contract Specifications

On this page

Contract Specifications
=======================

1. **PAMnoRF (Fixed Coupon Bullet Loan)**
-----------------------------------------

* **Contract Type**: PAM (Principal at Maturity)
* **Interest Rate**: Fixed at 2% per annum
* **Business Days**: Weekdays (`calendar: WEEKDAY`)
* **Start Date**: 2015-01-01 (`statusDate`)
* **Maturity Date**: 2020-01-02
* **Interest Payment**: Yearly (`cycleOfInterestPayment: P1YL0`), starting from 2016-01-02
* **Principal Repayment**: Full repayment at maturity
* **Day Count Convention**: 30E/360 (30-day months)
* **Currency**: USD

2. **PAMwRF (Variable Coupon Bullet Loan)**
-------------------------------------------

* **Contract Type**: PAM (Variable Rate)
* **Interest Rate**: Adjusted annually (`cycleOfRateReset: P1YL1`), based on:
  + **Base Rate**: UST5Y (`marketObjectCodeOfRateReset`)
  + **Rate Spread**: +1%
* **Start Date**: 2015-01-01
* **Maturity Date**: 2020-01-02
* **Interest Payment**: Semi-annual (`cycleOfInterestPayment: P6ML0`), starting from 2016-01-02
* **Principal Repayment**: Full repayment at maturity
* **Rate Reset**: First on 2015-07-02, then yearly

3. **ANNnoRF (Fixed Rate Annuity/Mortgage)**
--------------------------------------------

* **Contract Type**: ANN (Annuity)
* **Interest Rate**: Fixed
* **Start Date**: 2015-01-01
* **Maturity Date**: 2020-01-02
* **Payment Schedule**:
  + **Interest Payments**: Yearly (`cycleOfInterestPayment: P1YL0`)
  + **Principal Repayments**: Yearly (`cycleOfPrincipalRedemption: P1YL0`), starting from 2016-01-02
* **Total Payment**: Fixed sum covering both principal + interest

4. **ANNwRF (Variable Rate Annuity/Mortgage)**
----------------------------------------------

* **Contract Type**: ANN (Variable Rate)
* **Interest Rate**: Adjusted annually, same as PAMwRF
* **Start Date**: 2015-01-01
* **Maturity Date**: 2020-01-02
* **Payment Schedule**:
  + **Interest Payments**: Yearly
  + **Principal Repayments**: Yearly, starting from 2016-01-02
* **Total Payment**: Varies as interest rate changes

---

Each contract follows the ACTUS standard, allowing precise cash flow calculations based on predefined financial scenarios.

For more details, visit the [ACTUS Financial Research Foundation](https://www.actusfrf.org).

[Previous

3. Simulate ACTUS contract with Market Risk Scenario](/docs/category/3-simulate-actus-contract-with-market-risk-scenario)[Next

Contract Simulation](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/with_risk)

* [1. **PAMnoRF (Fixed Coupon Bullet Loan)**](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/simple_ACTUS_contract#1-pamnorf-fixed-coupon-bullet-loan)
* [2. **PAMwRF (Variable Coupon Bullet Loan)**](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/simple_ACTUS_contract#2-pamwrf-variable-coupon-bullet-loan)
* [3. **ANNnoRF (Fixed Rate Annuity/Mortgage)**](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/simple_ACTUS_contract#3-annnorf-fixed-rate-annuitymortgage)
* [4. **ANNwRF (Variable Rate Annuity/Mortgage)**](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/simple_ACTUS_contract#4-annwrf-variable-rate-annuitymortgage)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
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
  + [4. Risk Modeling for Contract and Counterparty Behaviors](/docs/category/4-risk-modeling-for-contract-and-counterparty-behaviors)

    - [Contract Behavior Risk Models](/docs/RiskFactors/Risk%20modelling/Example_of_contract)
    - [Setting Up a Sample](/docs/RiskFactors/Risk%20modelling/sample_2d_payment_model)
    - [Activating the Model for Simulation](/docs/RiskFactors/Risk%20modelling/activating_behaviour_model)
    - [Market Risk vs. Behavior Risk](/docs/RiskFactors/Risk%20modelling/Distinguishing)
    - [Sample Contract Simulation](/docs/RiskFactors/Risk%20modelling/Output)
  + [5. Future Enhancements](/docs/category/5-future-enhancements-)
* [Demos and Guides](/docs/actus-demo/demo-user-guide)
* [ACTUS Competition Pre-Announcement](/docs/competition)
* [Awesome Python Library](/docs/category/awesome-python-library)
* [[ACTUSFRF] CORE LICENSE 1.0](/docs/license)

* Quickstart Extension for ACTUS Risk Factors
* [4. Risk Modeling for Contract and Counterparty Behaviors](/docs/category/4-risk-modeling-for-contract-and-counterparty-behaviors)
* Contract Behavior Risk Models

On this page

Contract Behavior Risk Models
=============================

In earlier sections, we discussed **market risk**, where all contracts in a simulation experience the same projected market values, leading to consistent and aggregatable cash flows and events. However, real-world financial contracts are also influenced by **contract-specific and counterparty behaviors**, introducing additional uncertainty.

---

Market Risk vs. Behavior Risk
-----------------------------

| Market Risk | Behavior Risk |
| --- | --- |
| Affects all contracts equally in a simulation | Varies by contract and counterparty |
| Future market values are the same for all contracts | Future cashflows depend on contract-specific factors |
| Independent of a contract's internal state | Can be influenced by contract terms, counterparty credit rating, and market conditions |

---

Factors Affecting Contract Behavior Risk
----------------------------------------

* **Contract terms** – e.g., prepayment options, credit limits
* **Additional attributes** – borrower’s credit rating, contract’s business area
* **Internal contract state** – time to maturity, current interest rate
* **Market conditions** – future interest rates, liquidity changes

---

Examples of Contract Behavior Risks
-----------------------------------

### 1. Prepayment Risk

Some loan contracts permit early repayment, which reduces expected interest income for lenders.

### 2. Counterparty Default

A borrower defaults, terminating the contract early, often resulting in principal loss.

### 3. Deposit/Withdrawal Risk

Depositors withdraw funds or credit counterparties draw on available credit, affecting liquidity.

### 4. American Option Exercise Risk

An American Option holder can choose the time to exercise, impacting future cashflows.

---

ACTUS Risk Modeling
-------------------

ACTUS includes a basic **Prepayment Risk Model** for **ANN** and **PAM** contracts.  
However, real-world risk models are more complex, involving multiple parameters and various modeling approaches.

Unlike core ACTUS simulations, which produce **a single deterministic future cashflow**, behavior risk models evaluate different possible outcomes based on contract-specific factors.

---

Conclusion
----------

Understanding **contract behavior risks** is essential for accurate financial modeling. While market risk provides a broad view, behavior risks introduce real-world variability that must be considered in risk assessments.

[Previous

4. Risk Modeling for Contract and Counterparty Behaviors](/docs/category/4-risk-modeling-for-contract-and-counterparty-behaviors)[Next

Setting Up a Sample](/docs/RiskFactors/Risk%20modelling/sample_2d_payment_model)

* [Market Risk vs. Behavior Risk](/docs/RiskFactors/Risk%20modelling/Example_of_contract#market-risk-vs-behavior-risk)
* [Factors Affecting Contract Behavior Risk](/docs/RiskFactors/Risk%20modelling/Example_of_contract#factors-affecting-contract-behavior-risk)
* [Examples of Contract Behavior Risks](/docs/RiskFactors/Risk%20modelling/Example_of_contract#examples-of-contract-behavior-risks)
  + [1. Prepayment Risk](/docs/RiskFactors/Risk%20modelling/Example_of_contract#1-prepayment-risk)
  + [2. Counterparty Default](/docs/RiskFactors/Risk%20modelling/Example_of_contract#2-counterparty-default)
  + [3. Deposit/Withdrawal Risk](/docs/RiskFactors/Risk%20modelling/Example_of_contract#3-depositwithdrawal-risk)
  + [4. American Option Exercise Risk](/docs/RiskFactors/Risk%20modelling/Example_of_contract#4-american-option-exercise-risk)
* [ACTUS Risk Modeling](/docs/RiskFactors/Risk%20modelling/Example_of_contract#actus-risk-modeling)
* [Conclusion](/docs/RiskFactors/Risk%20modelling/Example_of_contract#conclusion)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
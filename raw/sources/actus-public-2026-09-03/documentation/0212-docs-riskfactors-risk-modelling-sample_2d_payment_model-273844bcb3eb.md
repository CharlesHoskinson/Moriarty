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
* Setting Up a Sample

On this page

Setting Up a Sample
===================

The **actus-riskservice-ce** component in ACTUS Quickstart includes a sample **two-dimensional prepayment behavior risk model** for analyzing prepayment risks in loan contracts.

---

1. Design Approach
------------------

This model uses two key parameters:

1. **Interest Rate Differential**:

   * The difference between the **current nominal interest rate** on the loan and the **current market interest rate** for refinancing.
   * If refinancing is favorable (i.e., market rate is lower than the loan rate), prepayment is more likely.
2. **Remaining Loan Maturity**:

   * If the loan is close to maturity, refinancing costs may discourage prepayment.
   * If maturity is far off, refinancing is more attractive.

### Model Execution

* The model is called at **specific simulation times** to determine potential prepayment amounts.
* A **lookup table** (2D surface) maps the probability or amount of prepayment based on interest rate differentials and time to maturity.

This model is a **basic example**; more advanced risk models can be integrated using the **ACTUS riskservice API**.

---

2. Creating a Prepayment Model Instance
---------------------------------------

To define and store a **sample prepayment model instance**, use:

```
 source putPpm01.txt
```

Loading Model Attributes from `putPpm01.txt` into `riskdata` Database
=====================================================================

This command loads model attributes from `putPpm01.txt` and saves them in the `riskdata` database.

Attributes of `ppm01` (Example Model)
-------------------------------------

* **Risk Factor ID**: `"ppm01"` (Unique identifier for the model)
* **Reference Rate ID**: `"ust5Y"` (5-year US Treasury rate as benchmark)
* **Prepayment Event Times**:
  + `"2015-03-01"`
  + `"2015-09-01"`
  + `"2016-03-01"`
* **Lookup Surface**: Defines prepayment probabilities for different interest rate differentials and maturities.

JSON Representation of `ppm01`
==============================

```
{  
  "riskFactorId": "ppm01",  
  "referenceRateId": "ust5Y",  
  "prepaymentEventTimes": ["2015-03-01", "2015-09-01", "2016-03-01"],  
  "surface": {  
    "interpolationMethod": "linear",  
    "extrapolationMethod": "constant",  
    "margins": [  
      { "dimension": 1, "values": [0.03, 0.025, 0.02, 0.015, 0.01, 0.0, -0.05] },  
      { "dimension": 2, "values": [0, 1, 2, 3, 5, 10] }  
    ],  
    "data": [  
      [0.01, 0.05, 0.1, 0.07, 0.02, 0],  
      [0.01, 0.04, 0.8, 0.05, 0.01, 0],  
      [0, 0.02, 0.5, 0.03, 0.005, 0],  
      [0, 0.01, 0.3, 0.01, 0, 0],  
      [0, 0.01, 0.2, 0, 0, 0],  
      [0, 0, 0.1, 0, 0, 0],  
      [0, 0, 0, 0, 0, 0]  
    ]  
  }  
}
```

Prepayment Model Details
========================

Lookup Surface
--------------

* **Margins**: Defines interest rate differentials (x-axis) and years until maturity (y-axis).
* **Data**: Specifies prepayment fractions for each combination of interest rate differential and maturity.

Adding the Prepayment Model to a Risk Scenario
----------------------------------------------

To make this model available in a risk scenario, create a scenario called `"scn02"` that includes `"ppm01"`.

```
 source putScn02.txt
```

Loading `scn02` into the Riskdata Database
==========================================

This command loads `putScn02.txt` and saves `"scn02"` in the `riskdata` database, linking it to `"ppm01"`.

Summary
-------

1. **Define a Prepayment Model Instance (`ppm01`)**

   * Uses interest rate differential and loan maturity to estimate prepayment.
   * Stored in `riskdata` using `putPpm01.txt`.
2. **Store the Model in a Risk Scenario (`scn02`)**

   * Links `ppm01` to `scn02` using `putScn02.txt`.
3. **Run Commands to Load the Model and Scenario**

   ```
   source putPpm01.txt  # Create prepayment model  
   source putScn02.txt  # Create scenario with model
   ```

[Previous

Contract Behavior Risk Models](/docs/RiskFactors/Risk%20modelling/Example_of_contract)[Next

Activating the Model for Simulation](/docs/RiskFactors/Risk%20modelling/activating_behaviour_model)

* [1. Design Approach](/docs/RiskFactors/Risk%20modelling/sample_2d_payment_model#1-design-approach)
  + [Model Execution](/docs/RiskFactors/Risk%20modelling/sample_2d_payment_model#model-execution)
* [2. Creating a Prepayment Model Instance](/docs/RiskFactors/Risk%20modelling/sample_2d_payment_model#2-creating-a-prepayment-model-instance)
* [Attributes of `ppm01` (Example Model)](/docs/RiskFactors/Risk%20modelling/sample_2d_payment_model#attributes-of-ppm01-example-model)
* [Lookup Surface](/docs/RiskFactors/Risk%20modelling/sample_2d_payment_model#lookup-surface)
* [Adding the Prepayment Model to a Risk Scenario](/docs/RiskFactors/Risk%20modelling/sample_2d_payment_model#adding-the-prepayment-model-to-a-risk-scenario)
* [Summary](/docs/RiskFactors/Risk%20modelling/sample_2d_payment_model#summary)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
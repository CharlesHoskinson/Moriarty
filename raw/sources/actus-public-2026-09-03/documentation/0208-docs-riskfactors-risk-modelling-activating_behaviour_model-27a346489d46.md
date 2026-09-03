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
* Activating the Model for Simulation

On this page

Activating the Model for Simulation
===================================

Once a **prepayment behavior model** is included in a risk scenario, individual contracts must specify which behavior models to activate during simulation.

---

1. Why is Activation Needed?
----------------------------

* **Risk scenarios** list all available behavior models, but not all contracts need every model.
* **Each contract must specify the relevant behavior models** for accurate risk modeling.
* Example:
  + A **loan contract** may need a **prepayment risk model**.
  + An **American Option contract** would require an **option exercise model**, not a prepayment model.

---

2. User-Defined Contract Terms
------------------------------

ACTUS contracts can include **custom user-defined terms**. These terms:

* Are not processed by the **actus-service** directly.
* Are passed to the **actus-riskservice** during simulation.

### Prepayment Model Activation

* The term `"prepaymentModels"` specifies **which prepayment model instances** should be used when simulating a contract.
* If the specified model is **not available** in the scenario, an **error occurs**.
* If available, it is **activated for the contract's simulation**.

---

3. Running a Contract Simulation with a Prepayment Model
--------------------------------------------------------

To run a contract simulation using a prepayment model, use:

```
 source ppm01PAMwRF_ss.txt
```

What This Command Does
======================

* Runs an **ACTUS simulation** for the contract `"PAMwRF"`.
* Uses the **risk scenario** `"scn02"`, which includes the prepayment model `"ppm01"`.
* Activates `"ppm01"` for prepayment modeling by including the user-defined term.

```
"prepaymentModels": ["ppm01"]
```

* Uses the simulation command:

```
/rf2/scenarioSimulation
```

instead of the standard /rf2/eventsBatch

[Previous

Setting Up a Sample](/docs/RiskFactors/Risk%20modelling/sample_2d_payment_model)[Next

Market Risk vs. Behavior Risk](/docs/RiskFactors/Risk%20modelling/Distinguishing)

* [1. Why is Activation Needed?](/docs/RiskFactors/Risk%20modelling/activating_behaviour_model#1-why-is-activation-needed)
* [2. User-Defined Contract Terms](/docs/RiskFactors/Risk%20modelling/activating_behaviour_model#2-user-defined-contract-terms)
  + [Prepayment Model Activation](/docs/RiskFactors/Risk%20modelling/activating_behaviour_model#prepayment-model-activation)
* [3. Running a Contract Simulation with a Prepayment Model](/docs/RiskFactors/Risk%20modelling/activating_behaviour_model#3-running-a-contract-simulation-with-a-prepayment-model)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
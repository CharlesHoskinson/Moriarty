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
* Contract Simulation

On this page

Contract Simulation
===================

Now that we have defined ACTUS contracts, let's simulate their cash flows under a market risk scenario using **ACTUS Quickstart**.

How Simulation Works
--------------------

To request a contract simulation, we need:

1. **A curl command** – Sends a REST request to the ACTUS service.
2. **JSON request data** – Includes:
   * Contract details
   * Market risk scenario to apply
3. **Service API call** – Specifies:
   * Target ACTUS service (`http://localhost:8083/`)
   * Command to execute (`rf2/eventsBatch` for batch simulation)

Running Simulations
-------------------

We use the `source` command to run a simulation.

Running Principal At Maturity (PAM) Contract Simulations
--------------------------------------------------------

```
source scn01PAMnoRF_eb.txt
```

This simulates the **PAMnoRF (Fixed Coupon Bullet Loan)** under **risk scenario "scn01"**. Since it has a **fixed** interest rate, the cash flows remain unchanged, even with different market conditions.

For the **PAMwRF (Variable Coupon Bullet Loan)**:

```
source scn01PAMwRF_eb.txt
```

* **Interest rate is variable**, based on **UST5Y (US Treasury 5-Year Rate)**.
* The scenario **scn01** provides a projected market rate.
* The interest payments change based on this projection.

Running Annuity/Mortgage Contract Simulations
---------------------------------------------

Similarly, we can simulate annuity/mortgage contracts:

```
source scn01ANNnoRF_eb.txt # Fixed-rate annuity (no effect from market rates)   
source scn01ANNwRF_eb.txt # Variable-rate annuity (market-sensitive)
```

* **ANNnoRF (Fixed-Rate Annuity)**: Cash flows remain the same.
* **ANNwRF (Variable-Rate Annuity)**: Cash flows depend on future **UST5Y** projections.

Simulating Multiple Contracts
-----------------------------

Instead of simulating one contract at a time, we can run **a portfolio of contracts** in a single request. The API will return a list of **cash flows for each contract**, identified by their **contractID**.

---

### Key Takeaways

* Fixed-rate contracts **don’t change** with market scenarios.
* Variable-rate contracts **adjust cash flows** based on projected interest rates.
* The API can handle **single or multiple** contract simulations.

**For more details, visit:** [ACTUS Financial Research Foundation](https://www.actusfrf.org)

[Previous

Contract Specifications](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/simple_ACTUS_contract)[Next

Cash Flow Simulations](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/output_cashflow_event_series)

* [How Simulation Works](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/with_risk#how-simulation-works)
* [Running Simulations](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/with_risk#running--simulations)
* [Running Principal At Maturity (PAM) Contract Simulations](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/with_risk#running-principal-at-maturity-pam-contract-simulations)
* [Running Annuity/Mortgage Contract Simulations](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/with_risk#running-annuitymortgage-contract-simulations)
* [Simulating Multiple Contracts](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/with_risk#simulating-multiple-contracts)
  + [Key Takeaways](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/with_risk#key-takeaways)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
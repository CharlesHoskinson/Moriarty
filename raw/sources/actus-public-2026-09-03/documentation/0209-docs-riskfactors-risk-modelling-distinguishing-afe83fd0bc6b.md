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
* Market Risk vs. Behavior Risk

On this page

Market Risk vs. Behavior Risk
=============================

ACTUS simulations can run under two different scenarios:

1. **Market Risk Only (`scn01`)**
2. **Market Risk + Behavior Modeling (`scn02`)**

These scenarios determine how market data is processed during simulation.

---

1. Market Risk Only Scenario (`scn01`)
--------------------------------------

* **Uses `/rf2/eventsBatch` simulation request.**
* **Optimization:**
  + All contracts in the batch **use the same market data projections**.
  + Market data is fetched **once** at the start and stored in memory.
  + **No need for repeated lookups** during the simulation.
  + Market data processing happens **entirely within the `actus-service`**.

### Why is this Efficient?

* Since all contracts share the same market data, fetching it once and reusing it **saves computation time**.
* The `actus-service` can **run the simulation independently** without repeatedly querying the `actus-riskservice`.

---

2. Market Risk + Behavior Modeling Scenario (`scn02`)
-----------------------------------------------------

* **Uses `/rf2/scenarioSimulation` request.**
* **Includes behavior risk modeling**, requiring **more interactions** between components.

### Key Differences from `scn01`

* Market data is needed in **two places**:
  1. **`actus-service`** – For contract-specific cashflow logic.
  2. **`actus-riskservice`** – For behavior modeling calculations.
* Market data is **fetched dynamically** rather than stored in memory.

### How Does it Work?

1. The `actus-service` requests market data **from the `actus-riskservice`** whenever needed.
2. The `actus-service` also **sends the current contract state** to the `actus-riskservice` at each scheduled risk callout.
3. The `actus-riskservice`:
   * Retrieves the required market data.
   * Computes behavior risk effects (e.g., prepayment adjustments).
   * Returns the behavior impact as an **event** (or null if no impact) to the `actus-service`.

---

3. Summary
----------

| Feature | Market Risk Only (`scn01`) | Market + Behavior Risk (`scn02`) |
| --- | --- | --- |
| Simulation Command | `/rf2/eventsBatch` | `/rf2/scenarioSimulation` |
| Market Data Handling | Fetched **once**, stored in memory | Looked up dynamically when needed |
| Component Interaction | `actus-service` handles all market data | `actus-service` & `actus-riskservice` interact dynamically |
| Behavior Modeling | Not included | Used for contract behavior changes |

**Choosing the Right Simulation Mode:**

* **Use `/rf2/eventsBatch` (`scn01`)** when simulating contracts **without behavior risk modeling** for **faster performance**.
* **Use `/rf2/scenarioSimulation` (`scn02`)** when behavior modeling is **needed** (e.g., prepayment, defaults).

This ensures simulations are both **efficient** and **accurate** based on the risk scenario being used.

[Previous

Activating the Model for Simulation](/docs/RiskFactors/Risk%20modelling/activating_behaviour_model)[Next

Sample Contract Simulation](/docs/RiskFactors/Risk%20modelling/Output)

* [1. Market Risk Only Scenario (`scn01`)](/docs/RiskFactors/Risk%20modelling/Distinguishing#1-market-risk-only-scenario-scn01)
  + [Why is this Efficient?](/docs/RiskFactors/Risk%20modelling/Distinguishing#why-is-this-efficient)
* [2. Market Risk + Behavior Modeling Scenario (`scn02`)](/docs/RiskFactors/Risk%20modelling/Distinguishing#2-market-risk--behavior-modeling-scenario-scn02)
  + [Key Differences from `scn01`](/docs/RiskFactors/Risk%20modelling/Distinguishing#key-differences-from-scn01)
  + [How Does it Work?](/docs/RiskFactors/Risk%20modelling/Distinguishing#how-does-it-work)
* [3. Summary](/docs/RiskFactors/Risk%20modelling/Distinguishing#3-summary)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
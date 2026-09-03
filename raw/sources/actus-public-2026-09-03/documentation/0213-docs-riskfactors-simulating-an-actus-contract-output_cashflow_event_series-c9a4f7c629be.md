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
* Cash Flow Simulations

On this page

Cash Flow Simulations
=====================

1. Understanding Cash Flow Events
---------------------------------

Each simulation generates a series of **cash flow events**, which are categorized as follows:

* **IED (Initial Exchange of Documents)** - Principal transfer at contract initiation.
* **IP (Interest Payment)** - Periodic interest payments.
* **MD (Maturity Date)** - Loan principal repayment at maturity.
* **RR (Rate Reset)** - Changes the interest rate in variable-rate contracts.
* **PRF (Principal Redemption Fix)** - Adjusts the principal payment based on rate changes.
* **PR (Principal Redemption)** - Scheduled loan repayment of principal.

---

2. Simulation Results
---------------------

### 2.1 Fixed Rate Bullet Loan (PAMnoRF)

* **Contract:** Fixed interest, annual payments, full repayment at maturity.
* **Events Summary:**
  + IED: Loan disbursed (-1000.0 USD).
  + IP: Annual interest payments of 20.0 USD.
  + MD: Final repayment of 1000.0 USD at maturity.

| Type | Date | Payoff (USD) | Nominal Value | Nominal Rate |
| --- | --- | --- | --- | --- |
| IED | 2015-01-02 | -1000.0 | 1000.0 | 0.02 |
| IP | 2016-01-02 | 20.0 | 1000.0 | 0.02 |
| IP | 2017-01-02 | 20.0 | 1000.0 | 0.02 |
| IP | 2018-01-02 | 20.0 | 1000.0 | 0.02 |
| IP | 2019-01-02 | 20.0 | 1000.0 | 0.02 |
| IP | 2020-01-02 | 20.0 | 1000.0 | 0.02 |
| MD | 2020-01-01 | 1000.0 | 0.0 | 0.02 |

---

### 2.2 Variable Rate Bullet Loan (PAMwRF)

```
source scn01PAMwRF_eb.txt
```

* **Contract:** Variable interest rate, semi-annual payments.
* **Events Summary:**
  + IED: Loan disbursed (-1000.0 USD).
  + RR: Rate resets periodically.
  + IP: Interest payments every 6 months.
  + MD: Final repayment at maturity.

| Type | Date | Payoff (USD) | Nominal Value | Nominal Rate |
| --- | --- | --- | --- | --- |
| IED | 2015-01-02 | -1000.0 | 1000.0 | 0.02 |
| RR | 2015-07-02 | 0.0 | 1000.0 | 0.049 |
| IP | 2016-01-02 | 34.05 | 1000.0 | 0.049 |
| IP | 2016-07-02 | 24.50 | 1000.0 | 0.049 |
| RR | 2016-07-02 | 0.0 | 1000.0 | 0.048 |
| IP | 2017-01-02 | 24.00 | 1000.0 | 0.048 |
| IP | 2017-07-02 | 24.00 | 1000.0 | 0.048 |
| MD | 2020-01-01 | 1000.0 | 0.0 | 0.045 |

---

### 2.3 Fixed Rate Annuity Loan (ANNnoRF)

```
source scn01ANNnoRF_eb.txt
```

* **Contract:** Fixed periodic payments, reducing principal.
* **Events Summary:**
  + IED: Loan disbursed (-1000.0 USD).
  + PRF: Initial principal redemption fix.
  + PR: Periodic principal repayments.
  + IP: Interest payments decrease over time.
  + MD: Final settlement at maturity.

| Type | Date | Payoff (USD) | Nominal Value | Nominal Rate |
| --- | --- | --- | --- | --- |
| IED | 2015-01-02 | -1000.0 | 1000.0 | 0.02 |
| PRF | 2016-01-01 | 0.0 | 1000.0 | 0.02 |
| PR | 2016-01-02 | 192.16 | 807.84 | 0.02 |
| IP | 2016-01-02 | 20.00 | 807.84 | 0.02 |
| PR | 2017-01-02 | 196.00 | 611.84 | 0.02 |
| IP | 2017-01-02 | 16.16 | 611.84 | 0.02 |
| PR | 2018-01-02 | 199.92 | 411.92 | 0.02 |
| MD | 2020-01-02 | 208.00 | 0.0 | 0.02 |

---

### 2.4 Variable Rate Annuity Loan (ANNwRF)

```
 source scn01ANNwRF_eb.txt
```

* **Contract:** Variable interest rate, fixed periodic payments adjusted at rate reset.
* **Events Summary:**
  + IED: Loan disbursed (-1000.0 USD).
  + RR: Rate resets periodically.
  + PRF: Principal redemption fixed after rate reset.
  + PR & IP: Payments adjusted after rate changes.
  + MD: Final settlement at maturity.

| Type | Date | Payoff (USD) | Nominal Value | Nominal Rate |
| --- | --- | --- | --- | --- |
| IED | 2015-01-02 | -1000.0 | 1000.0 | 0.02 |
| RR | 2015-07-02 | 0.0 | 1000.0 | 0.049 |
| PRF | 2015-07-02 | 0.0 | 1000.0 | 0.049 |
| PR | 2016-01-02 | 91.88 | 908.12 | 0.049 |
| IP | 2016-01-02 | 34.50 | 908.12 | 0.049 |
| PR | 2017-01-02 | 106.85 | 697.14 | 0.048 |
| IP | 2017-01-02 | 19.29 | 697.14 | 0.048 |
| PR | 2019-01-02 | 117.59 | 243.37 | 0.046 |
| MD | 2020-01-02 | 123.07 | 0.0 | 0.045 |

---

3. Key Takeaways
----------------

* **Fixed Rate Loans**: Payments remain constant, interest does not change.
* **Variable Rate Loans**: Payments vary based on rate reset (RR) events.
* **Bullet Loans (PAM)**: Interest-only payments, full principal repaid at maturity.
* **Annuity Loans (ANN)**: Fixed periodic payments including both principal and interest.

This document serves as a reference for understanding ACTUS-generated cash flow events in various contract simulations.

[Previous

Contract Simulation](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/with_risk)[Next

4. Risk Modeling for Contract and Counterparty Behaviors](/docs/category/4-risk-modeling-for-contract-and-counterparty-behaviors)

* [1. Understanding Cash Flow Events](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/output_cashflow_event_series#1-understanding-cash-flow-events)
* [2. Simulation Results](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/output_cashflow_event_series#2-simulation-results)
  + [2.1 Fixed Rate Bullet Loan (PAMnoRF)](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/output_cashflow_event_series#21-fixed-rate-bullet-loan-pamnorf)
  + [2.2 Variable Rate Bullet Loan (PAMwRF)](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/output_cashflow_event_series#22-variable-rate-bullet-loan-pamwrf)
  + [2.3 Fixed Rate Annuity Loan (ANNnoRF)](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/output_cashflow_event_series#23-fixed-rate-annuity-loan-annnorf)
  + [2.4 Variable Rate Annuity Loan (ANNwRF)](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/output_cashflow_event_series#24-variable-rate-annuity-loan-annwrf)
* [3. Key Takeaways](/docs/RiskFactors/Simulating%20an%20ACTUS%20contract/output_cashflow_event_series#3-key-takeaways)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
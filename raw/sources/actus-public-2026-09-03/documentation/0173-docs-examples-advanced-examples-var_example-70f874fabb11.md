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
  + [Getting Started](/docs/getting-started/installation)
  + [Examples](/docs/examples/basic-contract-types/example_PAM)

    - [Basic Contract Type Examples](/docs/examples/basic-contract-types/example_PAM)
    - [Advanced Examples](/docs/examples/advanced-examples/singleContract)

      * [Analysis Example: Variable Rate Annuity Contract](/docs/examples/advanced-examples/singleContract)
      * [Analysis Example: Multi-Contract Portfolio Analysis](/docs/examples/advanced-examples/portfolioExample)
      * [Simulating Value-at-Risk (VaR) for a Single PAM Contract](/docs/examples/advanced-examples/VaR_example)
  + [Guides](/docs/guides/common-patterns)
* [[ACTUSFRF] CORE LICENSE 1.0](/docs/license)

* [Awesome Python Library](/docs/category/awesome-python-library)
* Examples
* [Advanced Examples](/docs/examples/advanced-examples/singleContract)
* Simulating Value-at-Risk (VaR) for a Single PAM Contract

On this page

Simulating Value-at-Risk (VaR) for a Single PAM Contract
========================================================

This example illustrates how to simulate multiple interest rate scenarios and evaluate the Value-at-Risk (VaR) of a fixed-rate ACTUS PAM contract using the Awesome Actus Library (AAL).

> **Disclaimer:** This is a simplified, stylized example for educational purposes. It demonstrates simulation methodology and risk analysis but does not reflect actual market behavior or pricing standards.

---

Step 0: Setup and imports
-------------------------

```
import numpy as np  
import pandas as pd  
import matplotlib.pyplot as plt  
from awesome_actus_lib import PAM, PublicActusService, ValueAnalysis, ReferenceIndex, YieldCurve  
              
np.random.seed(42)
```

Step 1: Define the Contract
---------------------------

We define a single fixed-rate Principal-at-Maturity (PAM) contract with annual interest payments and maturity in 2029.

```
contract = PAM(  
    contractID="PAM-01",  
    statusDate="2025-12-30",  
    contractDealDate="2023-12-31",  
    currency="USD",  
    notionalPrincipal=10000,  
    initialExchangeDate="2024-01-01",  
    maturityDate="2029-12-31",  
    nominalInterestRate=0.05,  
    cycleAnchorDateOfInterestPayment="2024-01-01",  
    cycleOfInterestPayment="P1YL0",  
    dayCountConvention="30E360",  
    endOfMonthConvention="SD",  
    premiumDiscountAtIED=0,  
    contractRole="RPA",  
    creatorID="Bank-01",  
    counterpartyID="Counterparty-01",  
    marketObjectCodeOfRateReset="IR_SCENARIO",  
    rateSpread=0,  
    cycleOfRateReset="P1YL0",  
    cycleAnchorDateOfRateReset="2024-01-01",  
)
```

---

Step 2: Simulate Interest Rate Scenarios
----------------------------------------

We simulate 100 interest rate paths using a normal distribution centered at 3% with 1% volatility.

```
n_scenarios = 100  
scenarios = []  
date_range = pd.date_range(start="2023-09-01", periods=23, freq="Q")  
  
for _ in range(n_scenarios):  
    simulated_rates = np.random.normal(loc=0.03, scale=0.01, size=len(date_range))  
    df = pd.DataFrame({"date": date_range, "value": simulated_rates})  
    scenario = ReferenceIndex(marketObjectCode="IR_SCENARIO", source=df, base=1.0)  
    scenarios.append(scenario)  
  
# Plot interest rate scenarios  
plt.figure(figsize=(12, 6))  
for scenario in scenarios:  
    df = scenario._data.copy()  
    plt.plot(pd.to_datetime(df["date"]), df["value"], alpha=0.5)  
plt.title("Simulated Interest Rate Scenarios")  
plt.xlabel("Date")  
plt.ylabel("Interest Rate")  
plt.grid(True)  
plt.tight_layout()  
plt.show()
```

![Interest Rate Scenarios](/assets/images/ir_scenarios-6149b03259790a700a7e712c133c4ca5.png)

---

Step 3: Define the Yield Curve
------------------------------

The discounting is done using a upward-sloping yield curve.

```
yield_curve = YieldCurve(  
    marketObjectCode="DC_YC",  
    referenceDate="2024-01-01",  
    tenors=["0D", "1M", "3M", "6M", "1Y", "2Y", "3Y", "5Y", "7Y", "10Y"],  
    rates=[0.01, 0.012, 0.014, 0.016, 0.018, 0.021, 0.023, 0.026, 0.028, 0.03]  
)  
yield_curve.plot()
```

![Yield Curve](/assets/images/yield_curve-1eb84d323589082296963099ea813c92.png)

---

Step 4: Run Simulations and Compute NPVs
----------------------------------------

```
service = PublicActusService()  
npvs = []  
  
for scen in scenarios:  
    events = service.generateEvents(portfolio=contract, riskFactors=[scen, yield_curve])  
    val = ValueAnalysis(events, as_of_date="2026-01-01", discount_curve_code="DC_YC")  
    npvs.append(val.npv)
```

---

Step 5: Plot NPV Distribution and Calculate Risk Measures
---------------------------------------------------------

```
var_95 = np.quantile(npvs, 0.05)  
var_99 = np.quantile(npvs, 0.01)  
initial = contract.terms.get("notionalPrincipal").value  
breakeven_prob = sum(npv >= initial for npv in npvs) / len(npvs)  
  
plt.figure(figsize=(10, 6))  
plt.hist(npvs, bins=30, edgecolor="black")  
plt.axvline(var_95, color='red', linestyle='--', linewidth=2, label='VaR 95%')  
plt.axvline(var_99, color='purple', linestyle='--', linewidth=2, label='VaR 99%')  
plt.text(var_95, plt.ylim()[1]*0.9, f'9835.05', color='red', ha='right')  
plt.text(var_99, plt.ylim()[1]*0.8, f'9600.61', color='purple', ha='right')  
plt.title("NPV Distribution Across Interest Rate Scenarios")  
plt.xlabel("Net Present Value")  
plt.ylabel("Frequency")  
plt.legend()  
plt.grid(True)  
plt.tight_layout()  
plt.show()  
  
print(f"VaR 95%: {var_95}")  
print(f"VaR 99%: {var_95}")  
print(f"Break-even Probability (NPV ≥ 10000): {breakeven_prob}%")
```

![NPV Distribution](/assets/images/npv_distribution-2490b1a9d3bb7659013b7a24d8b3b9ce.png)

**Results:**

* VaR 95%: 9835.05
* VaR 99%: 9600.61
* Break-even Probability (NPV ≥ 10000): 70.00%

[Previous

Analysis Example: Multi-Contract Portfolio Analysis](/docs/examples/advanced-examples/portfolioExample)[Next

Common Patterns](/docs/guides/common-patterns)

* [Step 0: Setup and imports](/docs/examples/advanced-examples/VaR_example#step-0-setup-and-imports)
* [Step 1: Define the Contract](/docs/examples/advanced-examples/VaR_example#step-1-define-the-contract)
* [Step 2: Simulate Interest Rate Scenarios](/docs/examples/advanced-examples/VaR_example#step-2-simulate-interest-rate-scenarios)
* [Step 3: Define the Yield Curve](/docs/examples/advanced-examples/VaR_example#step-3-define-the-yield-curve)
* [Step 4: Run Simulations and Compute NPVs](/docs/examples/advanced-examples/VaR_example#step-4-run-simulations-and-compute-npvs)
* [Step 5: Plot NPV Distribution and Calculate Risk Measures](/docs/examples/advanced-examples/VaR_example#step-5-plot-npv-distribution-and-calculate-risk-measures)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
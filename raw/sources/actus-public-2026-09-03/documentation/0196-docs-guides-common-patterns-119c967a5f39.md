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
  + [Guides](/docs/guides/common-patterns)

    - [Common Patterns](/docs/guides/common-patterns)
    - [Troubleshooting](/docs/guides/troubleshooting)
* [[ACTUSFRF] CORE LICENSE 1.0](/docs/license)

* [Awesome Python Library](/docs/category/awesome-python-library)
* Guides

On this page

Common Patterns
===============

> Best practices and idiomatic usage in Awesome ACTUS Library.

This guide outlines the most common workflows, design patterns, and best practices for using the Awesome ACTUS Library effectively.

---

🧭 General Workflow Pattern
--------------------------

The most typical pattern in this library follows a three-step flow:

**Define → Simulate → Analyze**

```
# 1. Define contract(s)  
contract = PAM(...)  # or a full portfolio  
  
# 2. Simulate cash flows  
cf_stream = actus_service.generateEvents(portfolio=[contract], riskFactors=[my_curve])  
  
# 3. Analyze results  
income = IncomeAnalysis(cf_stream)  
value = ValueAnalysis(cf_stream, flat_rate=0.03)  
  
# Optional: visualize  
cf_stream.plot()  
income.plot()
```

This flow works consistently for both basic and advanced use cases.

---

🧱 Portfolio Modeling Pattern
----------------------------

You can simulate multiple contracts at once using the `Portfolio` class:

```
portfolio = Portfolio(contracts=[  
    PAM(contractID="pam01", ...),  
    ANN(contractID="ann01", ...)  
])  
  
cf_stream = actus_service.generateEvents(portfolio=portfolio, riskFactors=[my_curve])
```

* Contracts can be mixed by type
* `contractID` is used to track outputs
* The resulting `CashFlowStream` keeps the full portfolio context

---

📊 Risk Factor Injection
-----------------------

Risk factors like `YieldCurve` or `ReferenceIndex` can be provided at simulation time and will be embedded in the output stream for downstream use:

```
curve = YieldCurve(  
    marketObjectCode="USD_DISCOUNT",  
    referenceDate="2025-01-01",  
    tenors=["1M", "3M", "1Y"],  
    rates=[0.01, 0.015, 0.02]  
)  
  
cf_stream = actus_service.generateEvents(  
    portfolio=[contract],  
    riskFactors=[curve]  
)
```

The cash flow stream now includes this curve for later analysis (e.g. NPV).

---

📈 Analysis and Plotting
-----------------------

Once you have a `CashFlowStream`, multiple built-in analyses are available:

```
cf_stream.plot()  # Shows either contract or portfolio view  
  
income = IncomeAnalysis(cf_stream, freq="Q")  
income.plot()  
  
va = ValueAnalysis(cf_stream, flat_rate=0.03)  
print(va.summarize())
```

All analysis classes share a consistent structure and can be extended.

---

🧩 Implementing Custom Analysis
------------------------------

You can extend the analysis layer by implementing your own:

```
class CustomAnalysis(Analysis):  
    def __init__(self, cf_stream, threshold):  
        super().__init__(cf_stream)  
        self.threshold = threshold  
        self.results = self.analyze()  
  
    def analyze(self):  
        df = self.events_df  
        return df[df["payoff"] > self.threshold]  
  
    def plot(self):  
        self.results["payoff"].plot(kind="bar")
```

---

⚠️ Gotchas and Recommendations
------------------------------

* ✅ **No need to specify `contractType`** manually — the library sets it automatically per contract class.
* 🏷 **Always use named `marketObjectCode`s** like `"USD_DISCOUNT"` so that downstream analyses can find the correct curve.
* 🔍 Use `cf_stream.show()` before analysis to preview your data.
* 📉 `plot()` adapts automatically for single or multiple contracts.

---

🔗 Related Examples
------------------

* [Basic Examples](/docs/examples/basic-contract-types/example_ANN)
* [Advanced Examples](/docs/examples/advanced-examples/portfolioExample)

Each example includes full Python source and generated markdown for learning and reuse.

[Previous

Simulating Value-at-Risk (VaR) for a Single PAM Contract](/docs/examples/advanced-examples/VaR_example)[Next

Common Patterns](/docs/guides/common-patterns)

* [🧭 General Workflow Pattern](/docs/guides/common-patterns#-general-workflow-pattern)
* [🧱 Portfolio Modeling Pattern](/docs/guides/common-patterns#-portfolio-modeling-pattern)
* [📊 Risk Factor Injection](/docs/guides/common-patterns#-risk-factor-injection)
* [📈 Analysis and Plotting](/docs/guides/common-patterns#-analysis-and-plotting)
* [🧩 Implementing Custom Analysis](/docs/guides/common-patterns#-implementing-custom-analysis)
* [⚠️ Gotchas and Recommendations](/docs/guides/common-patterns#%EF%B8%8F-gotchas-and-recommendations)
* [🔗 Related Examples](/docs/guides/common-patterns#-related-examples)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
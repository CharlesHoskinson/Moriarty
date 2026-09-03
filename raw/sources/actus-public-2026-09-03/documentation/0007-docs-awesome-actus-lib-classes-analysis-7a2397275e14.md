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

    - [Overview](/docs/awesome-actus-lib/awesome-actus-library)
    - [Architecture](/docs/awesome-actus-lib/architecture)
    - [Core Classes](/docs/awesome-actus-lib/classes/contractModel)

      * [ContractModel](/docs/awesome-actus-lib/classes/contractModel)
      * [PAM](/docs/awesome-actus-lib/classes/PAM)
      * [Portfolio](/docs/awesome-actus-lib/classes/Portfolio)
      * [ActusService](/docs/awesome-actus-lib/classes/ActusService)
      * [RiskService](/docs/awesome-actus-lib/classes/RiskService)
      * [CashFlowStream](/docs/awesome-actus-lib/classes/cashflowstream)
      * [Analysis](/docs/awesome-actus-lib/classes/Analysis)
      * [Risk Factor](/docs/awesome-actus-lib/classes/riskfactor)
  + [ACTUS Standard](/docs/standard/overview)
  + [Getting Started](/docs/getting-started/installation)
  + [Examples](/docs/examples/basic-contract-types/example_PAM)
  + [Guides](/docs/guides/common-patterns)
* [[ACTUSFRF] CORE LICENSE 1.0](/docs/license)

* [Awesome Python Library](/docs/category/awesome-python-library)
* [Awesome ACTUS Library](/docs/awesome-actus-lib/awesome-actus-library)
* [Core Classes](/docs/awesome-actus-lib/classes/contractModel)
* Analysis

On this page

AnalysisLayer
=============

The Analysis Layer provides a modular system for interpreting ACTUS contract simulation results.  
It builds on a shared base class `Analysis` and offers specialized subclasses for common use cases:

* `IncomeAnalysis`: tracks earnings from interest and fees
* `LiquidityAnalysis`: tracks total net cash movement
* `ValueAnalysis`: computes nominal and discounted value (NPV)

Each analysis operates on a `CashFlowStream` and can be customized by time range or aggregation frequency.

---

🧱 Base Class: `Analysis`
------------------------

All analysis classes inherit from `Analysis`, which provides access to:

* The original portfolio (`.portfolio`)
* The cash flow events (`.events_df`)
* Risk factor data (`.risk_factors`)

```
class Analysis(ABC):  
    def __init__(self, cf_stream):  
        self.cf_stream = cf_stream  
        self.portfolio = cf_stream.portfolio  
        self.risk_factors = cf_stream.riskFactors  
        self.events_df = cf_stream.events_df.copy()
```

---

⏱ Frequency Options & Time Filtering
------------------------------------

Most analyses support aggregation and filtering:

| Code | Description |
| --- | --- |
| M | Monthly |
| Q | Quarterly |
| 2Q | Semi-Annually |
| Y | Yearly |

Optional parameters `start` and `end` define the analysis window.

```
IncomeAnalysis(cf_stream, freq="Q", start="2025-01-01", end="2027-12-31")
```

---

📈 IncomeAnalysis
----------------

Captures interest (`IP`) and fee payments (`FP`) over time.

```
ia = IncomeAnalysis(cf_stream, freq="M")  
print(ia.results)  
ia.plot()
```

* Output: net income per time bucket
* Supports `.plot()` and `return_fig=True`

---

💧 LiquidityAnalysis
-------------------

Summarizes **total net cash** (incoming and outgoing) across all events:

```
liq = LiquidityAnalysis(cf_stream, freq="Y")  
print(liq.results)  
liq.plot()
```

---

💰 ValueAnalysis
---------------

Computes:

* Undiscounted **nominal value** of all future payoffs
* **Net Present Value (NPV)** using a discount curve or flat rate

```
val = ValueAnalysis(cf_stream, as_of_date="2025-01-01", discount_curve_code="USD_DISCOUNT")  
print(val.summarize())
```

If multiple curves are provided, the user must specify one using `discount_curve_code`.

If none are available, a `flat_rate=0.03` can be passed.

---

🔧 Custom Analysis
-----------------

To implement your own logic:

```
class MyAnalysis(Analysis):  
    def __init__(self, cf_stream):  
        super().__init__(cf_stream)  
        self.results = self.analyze()  
  
    def analyze(self):  
        df = self.events_df  
        # your custom logic  
        return ...  
  
    def plot(self):  
        ...
```

You can then use `.results`, `.plot()` or `.to_dataframe()`.

---

🧠 Design Philosophy
-------------------

Each analysis is:

* **Independent**: uses only what's in `CashFlowStream`
* **Extensible**: override `.analyze()` and `.plot()` as needed
* **Composable**: results can be combined, exported, visualized

---

📚 See Also
----------

* [`CashFlowStream`](/docs/awesome-actus-lib/classes/cashflowstream)

[Previous

CashFlowStream](/docs/awesome-actus-lib/classes/cashflowstream)[Next

Risk Factor](/docs/awesome-actus-lib/classes/riskfactor)

* [🧱 Base Class: `Analysis`](/docs/awesome-actus-lib/classes/Analysis#-base-class-analysis)
* [⏱ Frequency Options & Time Filtering](/docs/awesome-actus-lib/classes/Analysis#-frequency-options--time-filtering)
* [📈 IncomeAnalysis](/docs/awesome-actus-lib/classes/Analysis#-incomeanalysis)
* [💧 LiquidityAnalysis](/docs/awesome-actus-lib/classes/Analysis#-liquidityanalysis)
* [💰 ValueAnalysis](/docs/awesome-actus-lib/classes/Analysis#-valueanalysis)
* [🔧 Custom Analysis](/docs/awesome-actus-lib/classes/Analysis#-custom-analysis)
* [🧠 Design Philosophy](/docs/awesome-actus-lib/classes/Analysis#-design-philosophy)
* [📚 See Also](/docs/awesome-actus-lib/classes/Analysis#-see-also)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
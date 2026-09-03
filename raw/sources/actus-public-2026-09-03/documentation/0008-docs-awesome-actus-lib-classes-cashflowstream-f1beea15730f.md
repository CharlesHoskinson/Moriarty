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
* CashFlowStream

On this page

CashFlowStream
==============

The `CashFlowStream` class encapsulates the **result of simulating ACTUS contracts** under a given risk factor scenario. It is the central object produced by the `generateEvents()` method of the ACTUS service.

It provides:

* A unified, flattened DataFrame of cash flow events across contracts
* A reference to the input `Portfolio`
* Optional `riskFactors` used during simulation (e.g., reference indices, yield curves)

This makes it ideal for **visualizing**, **analyzing**, and **debugging** simulation results.

---

📦 Attributes
------------

| Attribute | Type | Description |
| --- | --- | --- |
| `portfolio` | `Portfolio` | The portfolio of one or more contracts used for simulation |
| `riskFactors` | `list` or `None` | Risk factor objects (e.g., `ReferenceIndex`, `YieldCurve`) used in simulation |
| `events_df` | `pd.DataFrame` | Flattened event table across all portfolio contracts |

---

✅ Methods
---------

| Method | Description |
| --- | --- |
| `show(max_rows=10, full=False)` | Print a preview or the full `events_df` |
| `plot(title=None, y1_label="Notional/Principal", y2_label="Interest Payments", return_fig=False)` | Plot cash flows for either a single contract or the full portfolio |
| `__str__()` / `__repr__()` | Summary of the object (e.g., number of events, contracts) |

---

📊 Plotting Behavior
-------------------

The `.plot()` method adapts automatically:

* For a **single contract**, it invokes an **internal layered visualization engine** that produces a rich plot with dual Y-axes and semantic annotations (e.g., arrows for `IP`, `PR`, `IED`, dashed lines for accruals and nominal state).
* For a **multi-contract portfolio**, it automatically aggregates payoffs and generates a stacked bar chart grouped by event type and adaptive time intervals.

Both use `matplotlib` internally and support export via `return_fig=True`.

```
fig = cf_stream.plot(return_fig=True)  
fig.savefig("plots/my_cashflows.png")
```

---

### 🔍 What the Plot Includes (Single Contract)

For basic ACTUS contracts (e.g. PAM, ANN), the contract-level plot contains:

* Red arrows for **initial exchanges** and **maturities** (e.g., `IED`, `MD`)
* Green arrows for **interest payments** (`IP`)
* Red arrows (on Y2) for **principal redemptions** (`PR`)
* Dashed red lines for **outstanding nominal** over time
* Dashed green lines for **interest accruals** between `IP` and `RR`
* Sinusoidal wave markers between **rate reset** events (`RR`)

These layers are drawn using a modular internal system, not exposed as a public API.

---

### 📊 What the Plot Includes (Portfolio)

When plotting a multi-contract portfolio, the output shows:

* Aggregated cash flows grouped by **event type**
* **Stacked bar chart** binned over time intervals (daily, weekly, monthly, yearly – auto chosen)
* Helpful for identifying net inflows/outflows or cash flow timing mismatch

---

📄 Usage Example
---------------

```
from awesome_actus_lib import PublicActusService  
cf_stream = PublicActusService().generateEvents(portfolio=my_portfolio, riskFactors=[my_curve])  
  
cf_stream.show(max_rows=5)  
cf_stream.plot()
```

---

📁 Accessing Raw Events
----------------------

The full set of ACTUS-generated events is stored in the `events_df` attribute:

```
df = cf_stream.events_df
```

Each row represents one event (e.g., `IP`, `PR`, `MD`, `RR`) and includes:

* `contractId`
* `time`
* `type`
* `payoff`
* and additional ACTUS state information

---

🧠 Design Consideration
----------------------

Including both `portfolio` and `riskFactors` in the `CashFlowStream` enables:

* **Traceability**: Full visibility into where each event came from
* **Analysis support**: Analyses like `ValueAnalysis` and `IncomeAnalysis` require the original input terms and discount curves
* **Plot adaptation**: Plots depend on knowing the `contractType`

This design avoids the need to manage inputs and outputs separately.

---

🧪 Internal Behavior: `_parse_events()`
--------------------------------------

This helper flattens the nested contract simulation result:

* Walks parent and child contracts recursively
* Extracts all event lists
* Adds a `contractId` column
* Builds a unified event table for analysis

The result is assigned to `.events_df`.

---

⚠️ Known Limitations
--------------------

* `plot()` assumes standard ACTUS events and behavior
* For highly complex portfolios or non-standard contracts, visualizations may need custom layers
* Performance may degrade slightly with very large portfolios (hundreds of contracts)

---

[Previous

RiskService](/docs/awesome-actus-lib/classes/RiskService)[Next

Analysis](/docs/awesome-actus-lib/classes/Analysis)

* [📦 Attributes](/docs/awesome-actus-lib/classes/cashflowstream#-attributes)
* [✅ Methods](/docs/awesome-actus-lib/classes/cashflowstream#-methods)
* [📊 Plotting Behavior](/docs/awesome-actus-lib/classes/cashflowstream#-plotting-behavior)
  + [🔍 What the Plot Includes (Single Contract)](/docs/awesome-actus-lib/classes/cashflowstream#-what-the-plot-includes-single-contract)
  + [📊 What the Plot Includes (Portfolio)](/docs/awesome-actus-lib/classes/cashflowstream#-what-the-plot-includes-portfolio)
* [📄 Usage Example](/docs/awesome-actus-lib/classes/cashflowstream#-usage-example)
* [📁 Accessing Raw Events](/docs/awesome-actus-lib/classes/cashflowstream#-accessing-raw-events)
* [🧠 Design Consideration](/docs/awesome-actus-lib/classes/cashflowstream#-design-consideration)
* [🧪 Internal Behavior: `_parse_events()`](/docs/awesome-actus-lib/classes/cashflowstream#-internal-behavior-_parse_events)
* [⚠️ Known Limitations](/docs/awesome-actus-lib/classes/cashflowstream#%EF%B8%8F-known-limitations)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
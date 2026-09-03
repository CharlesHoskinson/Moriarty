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
* Risk Factor

On this page

📉 RiskFactor Classes
====================

The **`RiskFactor`** module in the Awesome ACTUS Library provides a structured and extensible interface for representing time-dependent market data used in ACTUS contract simulation and valuation.

It defines:

* An abstract base class `RiskFactor`
* Two concrete subclasses:
  + `ReferenceIndex`: for simple time series (e.g., LIBOR)
  + `YieldCurve`: for term structure of rates based on tenors

---

🔧 Design Philosophy
-------------------

The goal is to allow **pluggable and reusable** market objects (e.g., curves and indices) that can be referenced by one or many contracts and reused across different analysis steps.

Each subclass implements `.to_json()` to provide a canonical form used by the ACTUS simulation engine.

---

✅ `RiskFactor` Base Class
-------------------------

```
class RiskFactor(ABC):  
    def __init__(self, marketObjectCode: str, source):  
        ...  
  
    def loadData(self, source):  
        ...  
  
    def plot(self, title=None, figsize=(10, 5), return_fig=False):  
        ...  
  
    @abstractmethod  
    def to_json(self):  
        ...
```

### 🔍 Attributes

| Name | Type | Description |
| --- | --- | --- |
| `marketObjectCode` | `str` | Identifier for use in contract terms |
| `_data` | `pd.DataFrame` | Internal time series or curve data |

### 🔍 Methods

* `loadData(source)`: Load from CSV or `DataFrame`
* `plot(...)`: Plot the data
* `to_json()`: Must be implemented by subclasses
* `_validate_and_format_dates(...)`: Normalize and validate date column

---

📊 `ReferenceIndex`
------------------

Represents a simple time series of market values (e.g., interest rates, FX rates).

```
ReferenceIndex(marketObjectCode="USD_LIBOR", source="libor.csv", base=1)
```

**Expected Columns**: `["date", "value"]`

**Output Example**:

```
{  
  "marketObjectCode": "USD_LIBOR",  
  "base": 1,  
  "data": [  
    {"time": "2025-01-01T00:00:00", "value": 0.035},  
    ...  
  ]  
}
```

---

📈 `YieldCurve`
--------------

Represents a term structure defined by **tenors** and **rates**, anchored at a reference date.

```
YieldCurve(  
    marketObjectCode="USD_YC",  
    referenceDate="2025-01-01",  
    tenors=["1M", "3M", "6M", "1Y"],  
    rates=[0.01, 0.015, 0.0175, 0.02]  
)
```

**Tenors** support:

* Days (`D`)
* Weeks (`W`)
* Months (`M`)
* Years (`Y`)

> `"3M"` means 3 months from the reference date.

---

🎨 Visualization
---------------

You can quickly inspect any risk factor using:

```
my_curve.plot()
```

This will show a timeline plot of the rate or value over time.

Use `return_fig=True` to return a `matplotlib` figure object.

---

🧩 Adding Your Own Risk Factor
-----------------------------

To implement a custom risk factor, extend the base class and implement `to_json()`:

```
class MyCustomFactor(RiskFactor):  
    def to_json(self):  
        df = self._data.copy()  
        df["date"] = self._validate_and_format_dates("date")  
        return {  
            "marketObjectCode": self.marketObjectCode,  
            "data": [  
                {"time": f"{d}T00:00:00", "value": float(v)}  
                for d, v in zip(df["date"], df["value"])  
            ]  
        }
```

This allows full control over the data source, transformation, and output format.

---

📁 Related
---------

* [`contract_terms`](/docs/contract-terms/contract-terms-overview)
* [`CashFlowStream`](/docs/awesome-actus-lib/classes/cashflowstream)
* [`ValueAnalysis`](/docs/awesome-actus-lib/classes/Analysis)

[Previous

Analysis](/docs/awesome-actus-lib/classes/Analysis)[Next

Overview](/docs/standard/overview)

* [🔧 Design Philosophy](/docs/awesome-actus-lib/classes/riskfactor#-design-philosophy)
* [✅ `RiskFactor` Base Class](/docs/awesome-actus-lib/classes/riskfactor#-riskfactor-base-class)
  + [🔍 Attributes](/docs/awesome-actus-lib/classes/riskfactor#-attributes)
  + [🔍 Methods](/docs/awesome-actus-lib/classes/riskfactor#-methods)
* [📊 `ReferenceIndex`](/docs/awesome-actus-lib/classes/riskfactor#-referenceindex)
* [📈 `YieldCurve`](/docs/awesome-actus-lib/classes/riskfactor#-yieldcurve)
* [🎨 Visualization](/docs/awesome-actus-lib/classes/riskfactor#-visualization)
* [🧩 Adding Your Own Risk Factor](/docs/awesome-actus-lib/classes/riskfactor#-adding-your-own-risk-factor)
* [📁 Related](/docs/awesome-actus-lib/classes/riskfactor#-related)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
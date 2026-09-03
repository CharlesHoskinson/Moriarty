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
* Portfolio

On this page

Portfolio
=========

The `Portfolio` class represents a collection of ACTUS contracts, such as `PAM`, `ANN`, or other contract types. It allows you to batch contracts into a group for simulation or analysis.

> 📦 This is often the next step after creating individual `ContractModel` instances.

---

✅ Usage Example
---------------

### From Python Objects

```
from awesome_actus_lib import PAM, Portfolio  
  
pam1 = PAM(...parameters...)  
pam2 = PAM(...parameters...)  
  
portfolio = Portfolio([pam1, pam2])
```

### From CSV File

```
portfolio = Portfolio("path/to/your/portfolio/of/actus/contracts.csv")
```

The CSV file must contain one row per contract and include a `contractType` column (e.g., "PAM", "ANN", etc.).
TBD: Add sample Files

🧠 Features
----------

* Accepts input as either:
  + A list of `ContractModel` instances
  + A path to a `.csv` file with flat contract term definitions
* Converts each contract into a dictionary and aggregates them in a DataFrame (`contract_df`)
* Automatically instantiates contracts based on the `contractType` column in CSVs

---

📄 Attributes
------------

| Attribute | Type | Description |
| --- | --- | --- |
| `contracts` | `List[ContractModel]` | List of contract instances |
| `contract_df` | `pandas.DataFrame` | Tabular view of all contract terms |

---

🔄 Methods
---------

| Method | Description |
| --- | --- |
| `to_dict()` | Returns a list of dictionaries for all contracts |
| `write_to_file()` | Saves the current portfolio to a CSV file |
| `__len__()` | Returns the number of contracts |
| `__str__()` | Human-readable summary string |

---

⚠️ Error Handling
-----------------

* Will raise `FileNotFoundError` if CSV path is invalid
* Raises `TypeError` if input is not a valid list or string
* Handles instantiation errors per contract row in the CSV

---

🔗 See Also
----------

* [`ContractModel`](/docs/awesome-actus-lib/classes/contractModel)
* [`PAM`](/docs/awesome-actus-lib/classes/PAM)

[Previous

PAM](/docs/awesome-actus-lib/classes/PAM)[Next

ActusService](/docs/awesome-actus-lib/classes/ActusService)

* [✅ Usage Example](/docs/awesome-actus-lib/classes/Portfolio#-usage-example)
  + [From Python Objects](/docs/awesome-actus-lib/classes/Portfolio#from-python-objects)
  + [From CSV File](/docs/awesome-actus-lib/classes/Portfolio#from-csv-file)
* [🧠 Features](/docs/awesome-actus-lib/classes/Portfolio#-features)
* [📄 Attributes](/docs/awesome-actus-lib/classes/Portfolio#-attributes)
* [🔄 Methods](/docs/awesome-actus-lib/classes/Portfolio#-methods)
* [⚠️ Error Handling](/docs/awesome-actus-lib/classes/Portfolio#%EF%B8%8F-error-handling)
* [🔗 See Also](/docs/awesome-actus-lib/classes/Portfolio#-see-also)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
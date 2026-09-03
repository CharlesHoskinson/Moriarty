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
* RiskService

On this page

RiskService
===========

The `RiskService` class enables management of external market data scenarios and reference indexes. It is used when contracts rely on externalized risk factors instead of inline parameters.

---

✅ Usage Example
---------------

```
from awesome_actus_lib import RiskService, ReferenceIndex  
  
risk_service = RiskService(serverURL="http://localhost:8084")  
risk_service.upload_reference_index(reference_index, riskFactorID="ust5Y")
```

---

🔁 Functionality
---------------

| Category | Methods |
| --- | --- |
| 📈 Reference Index | `upload_reference_index()`, `delete_reference_index()`, `find_reference_index()` |
| 🎯 Scenarios | `create_scenario()`, `find_all_scenarios()` |

---

🧠 Concepts
----------

* Risk factors are uploaded using `to_json()` representations
* Scenarios are logical groupings of one or more risk factor IDs
* Required when calling `generateEventsWithExternalRisk()` on `ActusService`

---

🔗 See Also
----------

* [`ActusService`](/docs/awesome-actus-lib/classes/ActusService)

[Previous

ActusService](/docs/awesome-actus-lib/classes/ActusService)[Next

CashFlowStream](/docs/awesome-actus-lib/classes/cashflowstream)

* [✅ Usage Example](/docs/awesome-actus-lib/classes/RiskService#-usage-example)
* [🔁 Functionality](/docs/awesome-actus-lib/classes/RiskService#-functionality)
* [🧠 Concepts](/docs/awesome-actus-lib/classes/RiskService#-concepts)
* [🔗 See Also](/docs/awesome-actus-lib/classes/RiskService#-see-also)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
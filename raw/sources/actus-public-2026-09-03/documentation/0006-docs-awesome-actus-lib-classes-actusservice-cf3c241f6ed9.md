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
* ActusService

On this page

ActusService
============

The `ActusService` class is the core controller interface for generating ACTUS-compliant event schedules.

It connects your Python-based contracts to the ACTUS simulation backend via HTTP.

---

✅ Usage Example
---------------

```
from awesome_actus_lib import PAM, ActusService  
  
contract = PAM(...)  
  
service = ActusService(serverURL="http://localhost:8083")  
eventStream = service.generateEvents(portfolio=contract)
```

---

🚀 Features
----------

* Sends contracts to `/eventsBatch` endpoint of the ACTUS engine
* Converts contract terms and optional risk factors into a backend-compatible format
* Returns a `CashFlowStream` for downstream analysis

---

🔁 Methods
---------

| Method | Description |
| --- | --- |
| `generateEvents()` | Simulates contracts using inline risk factors (or none) |
| `generateEventsWithExternalRisk()` | Simulates contracts using pre-uploaded risk scenarios |
| `extract_required_risk_factors()` | Finds all referenced risk factors in a portfolio |
| `_validate_rate_reset_risk_factor_coverage()` | Ensures risk coverage dates are valid |

---

🔐 Configuration
---------------

| Parameter | Description |
| --- | --- |
| `serverURL` | Base URL of the ACTUS backend |
| `externalRiskService` | Toggle for using external RiskService |

---

🧪 Validation
------------

When `generateEvents()` is called, the controller:

* Checks contract formatting
* Validates risk factor compatibility
* Raises detailed exceptions for missing or malformed inputs

---

🔗 See Also
----------

* [`Portfolio`](/docs/awesome-actus-lib/classes/Portfolio)
* [`CashFlowStream`](/docs/awesome-actus-lib/classes/cashflowstream)
* [`RiskService`](/docs/awesome-actus-lib/classes/RiskService)

[Previous

Portfolio](/docs/awesome-actus-lib/classes/Portfolio)[Next

RiskService](/docs/awesome-actus-lib/classes/RiskService)

* [✅ Usage Example](/docs/awesome-actus-lib/classes/ActusService#-usage-example)
* [🚀 Features](/docs/awesome-actus-lib/classes/ActusService#-features)
* [🔁 Methods](/docs/awesome-actus-lib/classes/ActusService#-methods)
* [🔐 Configuration](/docs/awesome-actus-lib/classes/ActusService#-configuration)
* [🧪 Validation](/docs/awesome-actus-lib/classes/ActusService#-validation)
* [🔗 See Also](/docs/awesome-actus-lib/classes/ActusService#-see-also)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
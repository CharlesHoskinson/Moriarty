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
* PAM

On this page

PAM
===

**Principal At Maturity (PAM)** is a standard ACTUS contract type implemented in the Awesome Actus Library.

This class represents fixed-rate or variable-rate instruments such as:

* Bullet loans
* Term deposits
* Zero-coupon or interest-bearing bonds

> 📚 For full term semantics and rule logic, refer to the [ACTUS Standard - PAM](/docs/contract-types/PAM).

---

✅ Usage Examples
----------------

```
from awesome_actus_lib import PAM  
  
contract = PAM(  
    contractID="PAM-001",  
    contractRole="RPA",  
    contractDealDate="2025-01-01",  
    initialExchangeDate="2025-01-02",  
    maturityDate="2030-01-01",  
    nominalInterestRate=0.03,  
    notionalPrincipal=10000.0,  
    dayCountConvention="30E360",  
    statusDate="2025-01-01",  
    currency="USD",  
    counterpartyID="CP01",  
    creatorID="user01"  
)
```

---

🧠 Behavior
----------

The `PAM` class:

* Inherits from [`ContractModel`](/docs/awesome-actus-lib/classes/contractModel)
* Automatically sets `contractType` to `"PAM"`
* Enforces all ACTUS-specific business rules using conditional groups
* Supports optional extension via keyword arguments (`**other_terms`)

---

🛠️ Conditional Group Handling
-----------------------------

Certain fields become required when a "driver" term is provided. For example:

* `feeRate` → requires `feeBasis`
* `purchaseDate` → requires `priceAtPurchaseDate`
* `cycleAnchorDateOfRateReset` → requires `rateSpread` and `marketObjectCodeOfRateReset`

These are enforced automatically at instantiation via `validate_terms()`.
If a rule is violated, a useful message is printed out.

---

🔍 Notes
-------

* Accepts both `datetime` and ISO 8601 `str` inputs for all ACTUS Terms that refer to dates
* Supports both ACTUS-defined and user-defined terms
* Implements ACTUS time consistency checks

---

🔗 See Also
----------

* [ContractModel](/docs/awesome-actus-lib/classes/contractModel)
* [Contract Terms](/docs/contract-terms/contract-terms-overview)
* [PAM in ACTUS Standard](/docs/contract-types/PAM)

[Previous

ContractModel](/docs/awesome-actus-lib/classes/contractModel)[Next

Portfolio](/docs/awesome-actus-lib/classes/Portfolio)

* [✅ Usage Examples](/docs/awesome-actus-lib/classes/PAM#-usage-examples)
* [🧠 Behavior](/docs/awesome-actus-lib/classes/PAM#-behavior)
* [🛠️ Conditional Group Handling](/docs/awesome-actus-lib/classes/PAM#%EF%B8%8F-conditional-group-handling)
* [🔍 Notes](/docs/awesome-actus-lib/classes/PAM#-notes)
* [🔗 See Also](/docs/awesome-actus-lib/classes/PAM#-see-also)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
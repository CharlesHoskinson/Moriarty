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
* Core Classes

On this page

ContractModel
=============

The `ContractModel` class is the abstract base for all ACTUS contract types in the Awesome Actus Library. It defines the structure, validation, and lifecycle behavior of ACTUS-conformant financial contracts such as `PAM`, `ANN`, `LAM`, etc.

> All contract types like `PAM` inherit from `ContractModel`, which centralizes term management, validation, and internal representation.

---

🌍 Purpose
---------

* Standardizes how contract terms are structured, stored, and validated
* Handles the lifecycle of contract input validation
* Enforces ACTUS time consistency rules
* Enables extension with user-defined terms

---

✏️ Structure
------------

Each `ContractModel` holds contract terms in a dictionary:

```
self.terms = {  
  "contractID": ContractID("PAM-001"),  
  "notionalPrincipal": NotionalPrincipal(10000.0),  
  # ...  
}
```

* Keys are the term identifiers (e.g. `contractID`, `notionalPrincipal`)
* Values are instances of subclasses of `ContractTerm`

---

🛠️ TermBuilder Utility
----------------------

Inside the model, a helper class `TermBuilder` is used to dynamically construct the correct term class based on the name:

```
builder = self.TermBuilder(self.terms)  
builder.add("notionalPrincipal", 10000.0)
```

This maps a field like `notionalPrincipal` to the correct term class (e.g., `NotionalPrincipal`).

---

🔢 Term Access
-------------

### Get a Term

```
model.get_term("maturityDate")
```

### Serialize to Dictionary

```
model.to_dict()
```

Useful for exporting to JSON or saving contract data.

---

⚠️ Time Consistency Rules
-------------------------

The `ContractModel` enforces ACTUS time ordering constraints via `check_time_consistency_rules()`. This includes checks like:

* `contractDealDate <= initialExchangeDate <= maturityDate`
* `cycleAnchorDateOfInterestPayment < maturityDate`
* `optionExerciseEndDate <= maturityDate`

Warnings are printed if rules are violated.

---

🌐 User-Defined Terms
--------------------

If you add a custom term not in the ACTUS standard, the model can still accept it:

```
pam = PAM(..., myCustomField="customValue")
```

These are wrapped using a fallback `UserDefinedTerm` and stored normally in `self.terms`.

---

✅ Required Method
-----------------

### `validate_terms()` (abstract)

Each subclass (e.g. `PAM`, `ANN`) must implement `validate_terms()` to:

* Check ACTUS group dependencies
* Raise errors for missing fields

---

🔗 See Also
----------

* [`PAM`](/docs/awesome-actus-lib/classes/PAM) for a concrete example
* [Contract Terms](/docs/contract-terms/contract-terms-overview)
* [ACTUS Standard Overview](/docs/standard/overview)

[Previous

Architecture](/docs/awesome-actus-lib/architecture)[Next

ContractModel](/docs/awesome-actus-lib/classes/contractModel)

* [🌍 Purpose](/docs/awesome-actus-lib/classes/contractModel#-purpose)
* [✏️ Structure](/docs/awesome-actus-lib/classes/contractModel#%EF%B8%8F-structure)
* [🛠️ TermBuilder Utility](/docs/awesome-actus-lib/classes/contractModel#%EF%B8%8F-termbuilder-utility)
* [🔢 Term Access](/docs/awesome-actus-lib/classes/contractModel#-term-access)
  + [Get a Term](/docs/awesome-actus-lib/classes/contractModel#get-a-term)
  + [Serialize to Dictionary](/docs/awesome-actus-lib/classes/contractModel#serialize-to-dictionary)
* [⚠️ Time Consistency Rules](/docs/awesome-actus-lib/classes/contractModel#%EF%B8%8F-time-consistency-rules)
* [🌐 User-Defined Terms](/docs/awesome-actus-lib/classes/contractModel#-user-defined-terms)
* [✅ Required Method](/docs/awesome-actus-lib/classes/contractModel#-required-method)
  + [`validate_terms()` (abstract)](/docs/awesome-actus-lib/classes/contractModel#validate_terms-abstract)
* [🔗 See Also](/docs/awesome-actus-lib/classes/contractModel#-see-also)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
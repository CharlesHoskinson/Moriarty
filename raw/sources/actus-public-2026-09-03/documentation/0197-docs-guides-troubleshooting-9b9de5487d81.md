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
* [Guides](/docs/guides/common-patterns)
* Troubleshooting

On this page

🚧 Troubleshooting Guide
=======================

This page lists common issues users may encounter while using the Awesome ACTUS Library and how to resolve them.

---

❌ No Events Generated
---------------------

**Symptoms:**

* `CashFlowStream.events_df` is empty
* `.show()` says “No events available”

**Causes:**

* Missing mandatory contract terms
* Invalid date ranges (e.g. maturity before start)
* Flat `0` notional or interest rates

**Solutions:**

* Call `contract.validate_terms()` manually to identify missing terms
* Check contract definition for startDate, maturityDate, and notionalPrincipal
* Confirm dates are in ISO 8601 format

---

📉 ValueAnalysis Fails with Curve Lookup Error
---------------------------------------------

**Symptoms:**

* `ValueError: No curve found with marketObjectCode='...'`
* Or: `"Cannot compute NPV..."`

**Causes:**

* Curve not attached or not passed to `generateEvents()`
* Wrong marketObjectCode
* Missing `value` or `rate` column in curve data

**Solutions:**

* Pass curves as `riskFactors=[...]` to `generateEvents()`
* Check `marketObjectCode` matches exactly
* Inspect curve with `print(risk_factor._data.head())`

---

🧪 Analysis Returns Empty or Zero Results
----------------------------------------

**Symptoms:**

* `.results` is an empty DataFrame
* `.summarize()` returns zero values

**Causes:**

* Analysis period does not overlap with events
* Payoff column contains only zeros
* Filters (e.g., `type in [...]`) match nothing

**Solutions:**

* Use `cf_stream.show(full=True)` to inspect events
* Adjust `start` and `end` in analysis
* Verify expected event types (`IP`, `PR`, etc.)

---

🖼 Plotting Displays Empty or Incomplete Charts
----------------------------------------------

**Symptoms:**

* No arrows/labels in contract plot
* Bar charts are flat or empty

**Causes:**

* No events of plotted types (e.g., `IP`, `IED`, `PR`)
* Filtering removed important rows
* Missing nominal values or accrued interest

**Solutions:**

* Confirm your contract produces expected events
* Call `.show()` before plotting
* Check that events include `payoff`, `nominalValue`, etc.

---

🔍 Debugging Tips
----------------

* Use `contract.describe()` or `str(contract)` to print term status
* Inspect event DataFrame directly via `cf_stream.events_df`
* Use `.plot(return_fig=True)` to intercept and save plots
* Print curve JSON: `print(curve.to_json())`

---

✅ Additional Recommendations
----------------------------

* **Avoid setting `contractType` manually**: It's auto-injected by the contract class
* **Use named risk factors** (`marketObjectCode`) for clear curve/analysis routing
* **Use consistent date formats** (ISO8601, `YYYY-MM-DD`)
* **Bundle everything** (contracts, curves) into `generateEvents()` for reproducibility
* **Advanced:** Check Sourcecode for commented print statements labeled with [DEBUG] and uncomment them

[Previous

Common Patterns](/docs/guides/common-patterns)[Next

[ACTUSFRF] CORE LICENSE 1.0](/docs/license)

* [❌ No Events Generated](/docs/guides/troubleshooting#-no-events-generated)
* [📉 ValueAnalysis Fails with Curve Lookup Error](/docs/guides/troubleshooting#-valueanalysis-fails-with-curve-lookup-error)
* [🧪 Analysis Returns Empty or Zero Results](/docs/guides/troubleshooting#-analysis-returns-empty-or-zero-results)
* [🖼 Plotting Displays Empty or Incomplete Charts](/docs/guides/troubleshooting#-plotting-displays-empty-or-incomplete-charts)
* [🔍 Debugging Tips](/docs/guides/troubleshooting#-debugging-tips)
* [✅ Additional Recommendations](/docs/guides/troubleshooting#-additional-recommendations)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
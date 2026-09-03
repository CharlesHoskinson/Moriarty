[Skip to main content](#__docusaurus_skipToContent_fallback)

[![Actus Logo](/img/ActusLogoRGB.jpg)![Actus Logo](/img/ActusLogoRGB.jpg)](/)[Dictionary](https://www.actusfrf.org/dictionary)[Taxonomy](https://www.actusfrf.org/taxonomy)

[GitHub](https://github.com/actusfrf)

* [Welcome to ACTUS Documentation](/docs/intro)
* [Introduction to ACTUS](/docs/category/introduction-to-actus)
* [ACTUS Quick Start](/docs/quickstart)
* [Quickstart Extension for ACTUS Risk Factors](/docs/category/1-extention-for-actus-risk-factors)

  + [1. Extention for ACTUS Risk Factors](/docs/category/1-extention-for-actus-risk-factors)
  + [2. Create sample Risk Factors in Mongodb](/docs/category/2-create-sample-risk-factors-in-mongodb)

    - [Initialize Risk Factor Store in MongoDB](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Initialize_and_view_your_persistent_risk_factor_store_in_MongoDB)
    - [Insert Reference Index](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Inserting_Sample_Risk_Data_Reference_Indexes)
  + [3. Simulate ACTUS contract with Market Risk Scenario](/docs/category/3-simulate-actus-contract-with-market-risk-scenario)
  + [4. Risk Modeling for Contract and Counterparty Behaviors](/docs/category/4-risk-modeling-for-contract-and-counterparty-behaviors)
  + [5. Future Enhancements](/docs/category/5-future-enhancements-)
* [Demos and Guides](/docs/actus-demo/demo-user-guide)
* [ACTUS Competition Pre-Announcement](/docs/competition)
* [Awesome Python Library](/docs/category/awesome-python-library)
* [[ACTUSFRF] CORE LICENSE 1.0](/docs/license)

* Quickstart Extension for ACTUS Risk Factors
* [2. Create sample Risk Factors in Mongodb](/docs/category/2-create-sample-risk-factors-in-mongodb)
* Insert Reference Index

On this page

Insert Reference Index
======================

**Overview**

In the previous section, we **cleared the riskdata store** to prepare it for new sample data. Now, we will insert **sample reference indexes**, **scenarios**, and **contract behavior model samples** while explaining each step.

This section focuses on **creating and saving market reference index projections**.

Accessing Sample ACTUS Commands
-------------------------------

To follow along, you need access to the **sample ACTUS commands folder**. If you have installed **ACTUS Basic Quickstart**, the commands are available at:

```
ACTUS-QUICKSTART_HOME/actus-docker-networks/test
```

To run the commands, open a **terminal/command window** in this folder.

> **Note:** If using **Windows**, open a **Linux PowerShell**.

Creating a Market Reference Index
---------------------------------

Run the following command to **create and save** a new reference index:

```
source putMSFT_rising.txt
```

This command creates a new reference index with:

ID: `MSFT_rising`

Stored in: `referenceIndex` collection of the `riskdata` database.

What Is `MSFT_rising`?
----------------------

It is a projection of future Microsoft Stock prices in a risk scenario.
The objective is to estimate the likely path of 5-year U.S. Treasury interest rates over the coming five years, enabling more informed investment and policy decisions.
For proper risk analysis, multiple scenarios can be created:

* MSFT\_steady (stable market)
* MSFT\_falling (declining market)

A financial contract involving Microsoft Stock (Futures, Options, Purchases, or Sales) can be simulated under different market risk scenarios, generating distinct cashflow sequences.

Reference Index Attributes
--------------------------

Each reference index includes:

| Attribute | Description |
| --- | --- |
| `riskFactorID` | Unique identifier for the reference index |
| `marketObjectCode` | Market object being projected |
| `base` | Scaling type (absolute values or percentages) |
| `data` | Time series of `<date, value>` pairs |

Sample JSON for MSFT\_rising
----------------------------

The command in `Test_B/putMSFT_rising.txt` contains JSON data stored in MongoDB.
Formatted, it looks like:

```
{  
  "riskFactorID": "MSFT_rising",  
  "marketObjectCode": "MSFT",  
  "base": 1.0,  
  "data": [  
    { "time": "2023-06-01T00:00:00", "value": 105 },  
    { "time": "2023-09-01T00:00:00", "value": 112 },  
    { "time": "2024-01-01T00:00:00", "value": 129 }  
  ]  
}
```

Creating a Reference Index for US Treasury Bonds (5-Year)
---------------------------------------------------------

To create a reference index for 5-year U.S. Treasury bonds, run:

```
source putUst5Y_falling.txt
```

This command creates:

**ID**: `ust5Y_falling`

**Purpose**: A future projection of 5-year U.S. Treasury bond interest rates.

Why Use ust5Y\_falling?
-----------------------

* Variable-rate loans adjust interest rates at specific dates.
* The contract defines a market rate to calculate new rates.
* The new interest rate is calculated as:

```
new rate = contract spread + current reference index value
```

To analyze cashflow variability, multiple indexes can be used:

`ust5Y_falling`

`ust5Y_steady`

`ust5Y_rising`

Viewing the ust5Y\_falling Attributes
-------------------------------------

To see the data, format the file:

```
putUst5Y_falling.txt
```

[Previous

Initialize Risk Factor Store in MongoDB](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Initialize_and_view_your_persistent_risk_factor_store_in_MongoDB)[Next

3. Simulate ACTUS contract with Market Risk Scenario](/docs/category/3-simulate-actus-contract-with-market-risk-scenario)

* [Accessing Sample ACTUS Commands](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Inserting_Sample_Risk_Data_Reference_Indexes#accessing-sample-actus-commands)
* [Creating a Market Reference Index](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Inserting_Sample_Risk_Data_Reference_Indexes#creating-a-market-reference-index)
* [What Is `MSFT_rising`?](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Inserting_Sample_Risk_Data_Reference_Indexes#what-is-msft_rising)
* [Reference Index Attributes](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Inserting_Sample_Risk_Data_Reference_Indexes#reference-index-attributes)
* [Sample JSON for MSFT\_rising](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Inserting_Sample_Risk_Data_Reference_Indexes#sample-json-for-msft_rising)
* [Creating a Reference Index for US Treasury Bonds (5-Year)](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Inserting_Sample_Risk_Data_Reference_Indexes#creating-a-reference-index-for-us-treasury-bonds-5-year)
* [Why Use ust5Y\_falling?](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Inserting_Sample_Risk_Data_Reference_Indexes#why-use-ust5y_falling)
* [Viewing the ust5Y\_falling Attributes](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Inserting_Sample_Risk_Data_Reference_Indexes#viewing-the-ust5y_falling-attributes)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
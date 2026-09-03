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
* Initialize Risk Factor Store in MongoDB

On this page

Initialize Risk Factor Store in MongoDB
=======================================

Overview
--------

The **ACTUS Basic Quickstart** installs a **MongoDB NoSQL database** inside a **Docker container** running on the workstation. To view the contents of this database, you need to install the **MongoDB command-line client**.

Installing MongoDB CLI
----------------------

Follow the instructions at the official MongoDB documentation:  
🔗 [MongoDB CLI Installation Guide](https://www.mongodb.com/docs/mongocli/v1.28/install/)

Starting the MongoDB Shell (CLI)
--------------------------------

Run the following command in your terminal to start the MongoDB shell:

```
mongo
```

Once inside the MongoDB client shell, you can use these commands:

**List available databases:**

```
show dbs
```

**Select a database (replace DBNAME with your actual database name):**

```
use DBNAME
```

**Display the name of the currently active database:**

```
db
```

Working with the riskdata Database
----------------------------------

If you have previously run **ACTUS QuickStart validation tests**, a database named `riskdata` will have been created. MongoDB also creates default databases during installation.

The ACTUS QuickStart validations may have created the following collections inside `riskdata`:

`referenceIndex`

`scenario`

`twoDimensionalPrepaymentData`

Deleting Existing Collections
-----------------------------

You can remove these collections using:

```
db.referenceIndex.drop()  
db.scenario.drop()  
db.twoDimensionalPrepaymentData.drop()
```

After deleting, verify that the riskdata database is empty by running:

```
show collections
```

If this command returns an empty list, the database is now clean and ready for new tests.

Exiting the MongoDB Shell
-------------------------

Press `Ctrl + C` or use a shell escape command to exit the MongoDB client shell.

[Previous

2. Create sample Risk Factors in Mongodb](/docs/category/2-create-sample-risk-factors-in-mongodb)[Next

Insert Reference Index](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Inserting_Sample_Risk_Data_Reference_Indexes)

* [Overview](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Initialize_and_view_your_persistent_risk_factor_store_in_MongoDB#overview)
* [Installing MongoDB CLI](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Initialize_and_view_your_persistent_risk_factor_store_in_MongoDB#installing-mongodb-cli)
* [Starting the MongoDB Shell (CLI)](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Initialize_and_view_your_persistent_risk_factor_store_in_MongoDB#starting-the-mongodb-shell-cli)
* [Working with the riskdata Database](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Initialize_and_view_your_persistent_risk_factor_store_in_MongoDB#working-with-the-riskdata-database)
* [Deleting Existing Collections](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Initialize_and_view_your_persistent_risk_factor_store_in_MongoDB#deleting-existing-collections)
* [Exiting the MongoDB Shell](/docs/RiskFactors/Using%20mongodb%20ACTUS%20risk%20database/Initialize_and_view_your_persistent_risk_factor_store_in_MongoDB#exiting-the-mongodb-shell)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
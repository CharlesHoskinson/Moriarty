[Skip to main content](#__docusaurus_skipToContent_fallback)

[![Actus Logo](/img/ActusLogoRGB.jpg)![Actus Logo](/img/ActusLogoRGB.jpg)](/)[Dictionary](https://www.actusfrf.org/dictionary)[Taxonomy](https://www.actusfrf.org/taxonomy)

[GitHub](https://github.com/actusfrf)

* [Welcome to ACTUS Documentation](/docs/intro)
* [Introduction to ACTUS](/docs/category/introduction-to-actus)
* [ACTUS Quick Start](/docs/quickstart)
* [Quickstart Extension for ACTUS Risk Factors](/docs/category/1-extention-for-actus-risk-factors)
* [Demos and Guides](/docs/actus-demo/demo-user-guide)

  + [Companion User Guide](/docs/actus-demo/demo-user-guide)
  + [ACTUS Server cURL Samples](/docs/actus-demo/server-curl-samples)
  + [Data-Driven Financial Risk and Regulatory Reporting (DaDFiR3)](/docs/dadfir3-demo/About)
  + [ACTUS Rshiny Demo Guide](/docs/dadfir3-demo/ACTUS%20Rshiny%20Demo%20Guide)
  + [Guide to ACTUS Portfolio Interest Rate Risk Analysis](/docs/dadfir3-demo/Demo%20User%20Guide)
* [ACTUS Competition Pre-Announcement](/docs/competition)
* [Awesome Python Library](/docs/category/awesome-python-library)
* [[ACTUSFRF] CORE LICENSE 1.0](/docs/license)

* Demos and Guides
* ACTUS Server cURL Samples

On this page

ACTUS Server cURL Samples
=========================

This document presents the formatted cURL command samples to interact with the ACTUS server for cash flow simulations. These samples demonstrate how to structure the requests and include necessary headers and payloads.

Use this sample command and JSON payload structure to interact with the ACTUS server for performing cash flow simulations. Adjust the parameters as needed for your specific use case.

Sample cURL Command to Fetch Demo Meta Infos
--------------------------------------------

```
curl -v -X GET https://demo.actusfrf.org:8080/demos/meta/STK
```

Sample cURL Command to Fetch Demo Details
-----------------------------------------

```
curl -v -X GET https://demo.actusfrf.org:8080/demos/STK
```

Sample cURL Command to Fetch Specific Demo Details
--------------------------------------------------

```
curl -v -X GET https://demo.actusfrf.org:8080/demo/5db350f7d70b8e45852b3057
```

Sample cURL Command to Compute Events for a Demo Contract
---------------------------------------------------------

```
curl -v -X POST https://demo.actusfrf.org:8080/events \  
  -H 'Content-Type: application/json' \  
  --data '{  
    "contract": {  
      "contractType": "STK",  
      "statusDate": "2015-01-01T00:00:00",  
      "contractRole": "RPA",  
      "contractID": 101,  
      "currency": "USD",  
      "contractDealDate": "2015-01-01T00:00:00",  
      "purchaseDate": "2015-01-02T00:00:00",  
      "priceAtPurchaseDate": 90,  
      "marketValueObserved": 100  
    },  
    "riskFactors": []  
  }'
```

Sample cURL Command to Compute Batch Events
-------------------------------------------

```
curl -v -X POST https://demo.actusfrf.org:8080/eventsBatch \  
  -H 'Content-Type: application/json' \  
  --data '{  
    "contracts": [  
      {  
        "contractType": "STK",  
        "statusDate": "2015-01-01T00:00:00",  
        "contractRole": "RPA",  
        "contractID": 102,  
        "currency": "USD",  
        "contractDealDate": "2015-01-01T00:00:00",  
        "purchaseDate": "2015-01-02T00:00:00",  
        "priceAtPurchaseDate": 90,  
        "terminationDate": "2015-01-16T00:00:00",  
        "priceAtTerminationDate": 110,  
        "marketValueObserved": 100  
      },  
      {  
        "contractType": "STK",  
        "statusDate": "2015-01-01T00:00:00",  
        "contractRole": "RPL",  
        "contractID": 103,  
        "currency": "USD",  
        "contractDealDate": "2015-01-01T00:00:00",  
        "purchaseDate": "2015-01-02T00:00:00",  
        "priceAtPurchaseDate": 90,  
        "terminationDate": "2015-01-16T00:00:00",  
        "priceAtTerminationDate": 110,  
        "marketValueObserved": 100  
      },  
      {  
        "contractType": "STK",  
        "statusDate": "2015-01-01T00:00:00",  
        "contractRole": "RPA",  
        "contractID": 104,  
        "cycleOfDividend": "P1YL0",  
        "currency": "USD",  
        "contractDealDate": "2015-01-01T00:00:00",  
        "purchaseDate": "2015-01-02T00:00:00",  
        "priceAtPurchaseDate": 90,  
        "marketValueObserved": 100  
      },  
      {  
        "contractType": "STK",  
        "statusDate": "2015-01-01T00:00:00",  
        "contractRole": "RPA",  
        "contractID": 105,  
        "cycleOfDividend": "P1YL0",  
        "currency": "USD",  
        "contractDealDate": "2014-12-01T00:00:00",  
        "purchaseDate": "2014-12-02T00:00:00",  
        "priceAtPurchaseDate": 90,  
        "marketValueObserved": 100  
      }  
    ],  
    "riskFactors": []  
  }'
```

Explanation
-----------

* **`-v`**: Enables verbose mode to provide detailed information about the request and response.
* **`-X METHOD`**: Specifies the HTTP method to use (GET or POST).
* **`-H 'Content-Type: application/json'`**: Sets the content type of the request to JSON.
* **`--data '...'`**: Specifies the JSON payload for the request, which includes the contracts and risk factors data.
* **URL**: The endpoint for the ACTUS server where the request is sent (e.g., `https://demo.actusfrf.org:8080/eventsBatch`).

JSON Payload Structure
----------------------

#### For Single Contract Event Computation

* **`contract`**: The contract object containing details such as type, status date, role, ID, currency, and financial details.
* **`riskFactors`**: An array of risk factor objects (empty in the provided sample).

#### For Batch Event Computation

* **`contracts`**: An array of contract objects.
* **`riskFactors`**: An array of risk factor objects (empty in the provided sample).

#### Contract Object Attributes

* **`contractType`**: Specifies the type of contract (e.g., STK).
* **`statusDate`**: The status date of the contract.
* **`contractRole`**: Role of the contract party (e.g., RPA, RPL).
* **`contractID`**: Unique identifier for the contract.
* **`currency`**: Currency of the contract.
* **`contractDealDate`**: Date the contract deal was made.
* **`purchaseDate`**: Date of the purchase.
* **`priceAtPurchaseDate`**: Price at the purchase date.
* **`terminationDate`**: Termination date of the contract.
* **`priceAtTerminationDate`**: Price at the termination date.
* **`marketValueObserved`**: Observed market value.
* **`cycleOfDividend`**: Frequency of dividend cycles (if applicable).

[Previous

Companion User Guide](/docs/actus-demo/demo-user-guide)[Next

Data-Driven Financial Risk and Regulatory Reporting (DaDFiR3)](/docs/dadfir3-demo/About)

* [Sample cURL Command to Fetch Demo Meta Infos](/docs/actus-demo/server-curl-samples#sample-curl-command-to-fetch-demo-meta-infos)
* [Sample cURL Command to Fetch Demo Details](/docs/actus-demo/server-curl-samples#sample-curl-command-to-fetch-demo-details)
* [Sample cURL Command to Fetch Specific Demo Details](/docs/actus-demo/server-curl-samples#sample-curl-command-to-fetch-specific-demo-details)
* [Sample cURL Command to Compute Events for a Demo Contract](/docs/actus-demo/server-curl-samples#sample-curl-command-to-compute-events-for-a-demo-contract)
* [Sample cURL Command to Compute Batch Events](/docs/actus-demo/server-curl-samples#sample-curl-command-to-compute-batch-events)
* [Explanation](/docs/actus-demo/server-curl-samples#explanation)
* [JSON Payload Structure](/docs/actus-demo/server-curl-samples#json-payload-structure)

Community

* [X.com](https://x.com/ActusResearch)
* [GitHub](https://github.com/actusfrf)
* [LinkedIn](https://www.linkedin.com/company/actus-research)

Copyright © 2026 Financial Research Foundation. Built with Docusaurus.
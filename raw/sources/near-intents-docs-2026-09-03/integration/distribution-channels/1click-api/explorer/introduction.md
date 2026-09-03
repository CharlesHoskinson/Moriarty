> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Explorer API

> Programmatic access to historical 1Click Swap transactions

The Explorer API provides read-only access to historical 1Click Swap transactions and their statuses, mirroring the data available on the [NEAR Intents Explorer](https://explorer.near-intents.org/).

<Callout color="#ff7043">
  **Base URL:** `https://explorer.near-intents.org/api/v0`
</Callout>

***

## Overview

This API is designed for distribution channels and analytical services that need to:

* Retrieve historical swap transaction data
* Filter swaps by chains, tokens, timestamps, and status
* Build dashboards and analytics tools
* Monitor swap activity programmatically

<Info>
  The Explorer API is read-only and specifically for 1Click Swap transactions.
</Info>

## Authentication

Include the token in your requests:

```bash theme={null}
curl -X GET "https://explorer.near-intents.org/api/v0/transactions" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

<Warning>
  A JWT token is **required** for all requests. Get your token from the [Partner Dashboard](https://partners.near-intents.org/).
</Warning>

***

## Rate Limiting

All API endpoints are rate-limited per partner:

* **1 request every 5 seconds** per partner ID
* Rate limits are enforced based on your JWT token
* Exceeding the limit returns `429 Too Many Requests`

***

## Endpoints

<CardGroup cols={1}>
  <Card title="Get Transactions" icon="list" href="/integration/distribution-channels/1click-api/explorer/transactions">
    Cursor-based pagination for efficient traversal
  </Card>
</CardGroup>

***

## API Specification

<CardGroup cols={2}>
  <Card title="Swagger UI" icon="file-code" href="https://explorer.near-intents.org/api/docs">
    Interactive API documentation
  </Card>

  <Card title="OpenAPI Spec" icon="file" href="https://explorer.near-intents.org/api/v0/openapi.yaml">
    Download the YAML specification
  </Card>
</CardGroup>

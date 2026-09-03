> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Shield Incident API

> Partner-facing API for pulling active Shield incidents and submitting new ones

This guide covers the partner-facing flow for pulling active Shield incidents and submitting new incidents. It does not cover admin-only incident operations. For the concepts and security model behind incidents, see [Proactive Intents Security](/security-compliance/proactive-intents-security).

<Info>
  For non-programmatic viewing and submitting incidents, use [`https://partners.near-intents.org/shield/console`](https://partners.near-intents.org/shield/console).
</Info>

* Incidents endpoint: [`https://shield.chaindefuser.com/incident`](https://shield.chaindefuser.com/incident)
* Get a JWT token at [`https://partners.near-intents.org/api-keys`](https://partners.near-intents.org/api-keys) by requesting `key_type: SHIELD`.
* To submit incidents, contact Intents Support so support can grant the required incident permissions.

All requests use:

```http theme={null}
Authorization: Bearer <PARTNER_TOKEN>
Accept: application/json
```

## Pull active incidents

Use [`GET https://shield.chaindefuser.com/incident`](https://shield.chaindefuser.com/incident) to check whether Shield currently has active incidents.

```sh theme={null}
curl -sS "https://shield.chaindefuser.com/incident" \
  -H "Authorization: Bearer $PARTNER_TOKEN" \
  -H "Accept: application/json"
```

When there are no active incidents, Shield returns:

```json theme={null}
{
  "status": "operational"
}
```

When incidents are active, Shield returns:

```json theme={null}
{
  "status": "incidents",
  "incidents": [
    {
      "scopeType": "chain",
      "scopeValue": "eth",
      "direction": "withdraw",
      "publicDescription": "ETH withdrawals are delayed"
    }
  ]
}
```

Partners with the `status_read` grant receive the full internal incident record, including `id`, `status`, `description`, `createdBy`, `metadata`, and timestamps.

## Submit an incident

Use [`POST https://shield.chaindefuser.com/incident`](https://shield.chaindefuser.com/incident) to open a scoped incident.

```sh theme={null}
curl -sS -X POST "https://shield.chaindefuser.com/incident" \
  -H "Authorization: Bearer $PARTNER_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "scopeType": "chain",
    "scopeValue": "eth",
    "direction": "withdraw",
    "description": "ETH withdrawals are temporarily unavailable"
  }'
```

Successful response:

```json theme={null}
{
  "incident": {
    "id": "<id>",
    "status": "active",
    "scopeType": "chain",
    "scopeValue": "eth",
    "direction": "withdraw",
    "description": "ETH withdrawals are temporarily unavailable",
    "publicDescription": null,
    "createdBy": "partner-id",
    "resolvedBy": null,
    "metadata": null,
    "createdAt": "2026-06-29T08:00:00.000Z",
    "updatedAt": "2026-06-29T08:00:00.000Z",
    "resolvedAt": null
  }
}
```

## Incident scope

| Field         | Required | Values                                | Notes                                                      |
| ------------- | -------- | ------------------------------------- | ---------------------------------------------------------- |
| `scopeType`   | Yes      | `chain`, `bridge`, `token`, `address` | What kind of surface is affected                           |
| `scopeValue`  | Yes      | Scope-specific string                 | Examples: `eth`, `poa`, `nep141:<contract>`, or an address |
| `description` | No       | String                                | Incident context                                           |

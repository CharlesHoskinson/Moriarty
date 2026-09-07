# PresenceEvent

> For the complete documentation index, see [llms.txt](/llms.txt)

Presence event for an SPO in an epoch.

```
type PresenceEvent {

  epochNo: Int!

  idKey: String!

  source: String!

  status: String

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`PresenceEvent.epochNo`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#presenceeventepochnoint-- "Direct link to presenceeventepochnoint--")

#### [`PresenceEvent.idKey`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#presenceeventidkeystring-- "Direct link to presenceeventidkeystring--")

#### [`PresenceEvent.source`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#presenceeventsourcestring-- "Direct link to presenceeventsourcestring--")

#### [`PresenceEvent.status`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#presenceeventstatusstring- "Direct link to presenceeventstatusstring-")

### Returned By[​](#returned-by "Direct link to Returned By")

[`registeredPresence`](/api-reference/midnight-indexer/operations/queries/registered-presence.md) query

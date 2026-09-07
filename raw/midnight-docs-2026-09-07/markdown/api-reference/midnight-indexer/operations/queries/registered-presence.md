# registeredPresence

> For the complete documentation index, see [llms.txt](/llms.txt)

Get raw presence events for an epoch range.

```
registeredPresence(

  fromEpoch: Int!

  toEpoch: Int!

): [PresenceEvent!]!
```

### Arguments[​](#arguments "Direct link to Arguments")

#### [`registeredPresence.fromEpoch`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#registeredpresencefromepochint-- "Direct link to registeredpresencefromepochint--")

#### [`registeredPresence.toEpoch`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#registeredpresencetoepochint-- "Direct link to registeredpresencetoepochint--")

### Type[​](#type "Direct link to Type")

#### [`PresenceEvent`](/api-reference/midnight-indexer/types/objects/presence-event.md) object[​](#presenceevent- "Direct link to presenceevent-")

Presence event for an SPO in an epoch.

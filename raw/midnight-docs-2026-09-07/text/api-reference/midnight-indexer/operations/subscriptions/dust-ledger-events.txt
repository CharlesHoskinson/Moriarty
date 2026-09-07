# dustLedgerEvents

> For the complete documentation index, see [llms.txt](/llms.txt)

Subscribe to dust ledger events starting at the given ID or at the very start if omitted.

```
dustLedgerEvents(

  id: Int

): DustLedgerEvent!
```

### Arguments[​](#arguments "Direct link to Arguments")

#### [`dustLedgerEvents.id`](#) ● [`Int`](/api-reference/midnight-indexer/types/scalars/int.md) scalar[​](#dustledgereventsidint- "Direct link to dustledgereventsidint-")

### Type[​](#type "Direct link to Type")

#### [`DustLedgerEvent`](/api-reference/midnight-indexer/types/interfaces/dust-ledger-event.md) interface[​](#dustledgerevent- "Direct link to dustledgerevent-")

A dust related ledger event.

# committee

> For the complete documentation index, see [llms.txt](/llms.txt)

Get committee membership for an epoch.

```
committee(

  epoch: Int!

): [CommitteeMember!]!
```

### Arguments[​](#arguments "Direct link to Arguments")

#### [`committee.epoch`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#committeeepochint-- "Direct link to committeeepochint--")

### Type[​](#type "Direct link to Type")

#### [`CommitteeMember`](/api-reference/midnight-indexer/types/objects/committee-member.md) object[​](#committeemember- "Direct link to committeemember-")

Committee member for an epoch.

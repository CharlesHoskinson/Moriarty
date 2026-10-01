# Public signing API type repair

Repository observation: inferred declarations spread a generic record from the
native parser, so the packed TypeScript API omitted `statement` and `frame_hex`
from prepareOwnerIntent and typed signature details as unknown. A developer
could not use these actual returned values without assertions.

Root reproduced five errors in a strict TypeScript signing client, then defined
explicit structural SignedIntentStatement, NativeIntentReceipt, verification,
preparation and result types. The runtime parser still checks every response;
the cast is after those checks and carries no authority brand. The same client
now compiles. The packed-install declaration test includes these accesses so
source and distributed API declarations both remain useful.

Receipts: api-types-red-scoped.txt and api-types-green.txt in the external task
directory, to be retained at the reviewed evidence freeze. No wire bytes or
acceptance predicates changed.

# Full38 financial authority relation — proposed source contract

This new contract supplies the missing authority judgment for the frozen financial
schema. It does not implement that schema's financial transition interpreter,
signatures, ledger currentness or Midnight correspondence. These remain explicit
independent judgments. No original expression constructor or financial operation
is removed. The proposal needs fresh GPT6 and Grok design/source review.

The design uses finite typed grants plus a separate persistent authority ledger.
An operation-name whitelist alone cannot authorize a new liability, forgiveness,
assignment, seizure or migration. Those rights are derived independently from the
actual typed before/after objects and movements. A financial transition judgment
must bind the same entire transition document. It cannot authenticate grants.
Conversely, a signature over a grant cannot establish a financial transition.
The executable checker models both external relations as exact hash sets supplied
by a trusted context, never as document-local `allowed=true` or callbacks.

Each grant has one exact target scope: right, namespace, object identifier,
principal/counterparty, resource or denomination, before/after immutable identity
hashes and complete policy-path hash. Its finite operation/action/program lists,
network/deployment/profile, actor delegation and time window are also exact.
The target has no wildcard. A quantity is a checked UInt128 debit against the
grant's lifetime cap and its parent financial budget. Cash gross and cash fees have distinct rights; fees consume
both. Nominal creation, interest increase, capitalization, discharge and assignment
are independent budget domains. Refunds and migrations never reduce usage.
Assignment charges the assigned residual quantity; base-operation rights consume one use. Zero-valued monetary claim rights
consume zero monetary capacity while retaining their exact identity checks. Unchanged rights and backing cannot disappear in a transformation.

An OutcomeIntent grant allows repeated matching derived targets within these caps.
An ExactPlan grant additionally contains a finite list of one-use occurrences,
each committing the predecessor lineage head and entire transition hash. One
occurrence is consumed once per action using that grant, even if several of its
rights are used. Duplicate or externally consumed occurrences reject. The lineage
head hashes financial transitions, not grant hashes; the authenticated seed binds
the entire fixed grant set, financial state, usage and occurrence ledger together.
This separates commitments to avoid an exact-plan hash cycle. Moving to another
program/policy preserves that same authenticated seed lineage and grant set;
issuing replacement grants is a separate authenticated admission, not Migrate.

A seed and every externally current output must be unconsumed. Successful proposed
steps consume predecessor heads locally and derive successors. Only a successor
not in the external spent set can be returned. Replaying a seed or resetting its
usage requires a different authenticated seed judgment and is not authorized by
this checker. External acceptance must atomically consume the returned predecessor
and occurrence identifiers; the source checker performs no cryptography or writes.

Before reviewing grants, typecheck full financial states, operands and effects
against the copied exact schema, reconcile ordered object preimages with pre/post,
and require the independently established transition hash. Derive rights in
operation order, then base-operation rights, movement order, object-change order.
Match uses in that exact order. Validate grant identity/authentication/revocation,
time, target and occurrence before adding bounded lifetime debits. Reject the first
actual predicate failure. Malformed inputs return closed rejection codes, never
host exceptions or intended fixture labels. No partial usage is returned on failure.

The canonical signing display is the same canonical grant/occurrence/transition
bytes used by hashing, with domain-separated hashes and exact schema/profile
commitments. A richer UI may add labels, but must not replace or omit any canonical
field. Policy-path rows are immutable profile data for each operation/resource;
the source relation verifies all referenced policies and their validity. Domain
specific identity and legal beneficial ownership remain authentication premises.

Claim creation requires the obligor's explicit grant, bound to the complete fixed
claim identity. A positive increase requires an independent increase grant.
Payment discharge and authorized writeoff use distinct rights of the creditor;
a cash debit is never a discharge grant. Changes to beneficiary or obligor require
consent from both old and new affected roles. Resource, denomination, priority,
condition or backing transformations require scoped conversion/assignment rights
of both claim parties plus any changed collateral owner's rights. Full financial
claim-map rules still enforce paid receipts, exact residuals, unique backing and
successor retirement; this relation cannot waive them. Capitalization consumes a
separate right and lifetime nominal capacity, never a fresh allowance from payment.

This directory owns only the additional relation and proposed integration patch
instructions. Frozen financial and signing files remain untouched. Generic typed
validation uses every bounded record in the pinned schema; operation dispatch uses
the complete fixed38 roster, not example IDs. All48 financial/40 expression scopes,
asset-study cases and downstream native/crypto gates remain required.

# Proposed branch history decisions 04

Status: proposed new source contract; requires independent GPT6 and Grok design
votes. No approval or compatibility is inferred from draft03. Its exact33 pins
remain unchanged. This revision selects one bounded disjoint-parallel operator,
not shared-state interleaving or full38 financial ABI acceptance.

1. Partition whole profile accounts into disjoint ownership sets, including zero
   accounts needed for later receipts. Each account belongs to exactly one child.
   A branch may move value only between accounts it owns. This permits independent
   A and B flows without two children spending the same balance. Splitting one
   account's fungible amount into separately owned pieces is not implemented;
   it requires explicit resource identities in the full financial ABI. No claimed
   overlap can be legalized by splitting a balance field without that ABI.
2. A split copies cumulative gross/fee/receipt counters as immutable shared prefix,
   but partitions remaining gross/fee authority separately. Each child's quota is
   exactly the parent's remaining quota on accounts it owns and zero elsewhere.
   New charges subtract from those quotas. Join counts the common prefix once
   and adds each child's counter difference from that prefix. It sums owned
   balances and residual quotas, not duplicated original caps.
3. The one currently supported obligation has one debt-owner branch, even when its
   debt is zero. Only that branch may create/accrue/repay the obligation; others
   contain zero debt. Duties are assigned once, to the branch owning their backing
   debtor/asset account. The checker requires the full named backing balance and
   sufficient remaining payer authority there. Join restores all duties and debt
   without erasure. The full financial registry must generalize this per typed
   obligation/claim identity; no array of unknown financial records is admitted.
4. Split and join each cost the exact positive profile-owned ordinary price1.
   Split allocates post-charge ordinary/recovery work; explicit retirement is
   accumulated separately and cannot return through later actions or migration.
   Join sums only its unique live descendant siblings' residual work, subtracts
   its charge, and adds retirement. It never refunds previous charges.
5. Every graph node binds exact immutable origin and profile hashes plus consumed
   predecessor node IDs. Outputs are computed and committed; caller-supplied full
   post states or costs are checked by equality, never used as the oracle. The
   checker consumes each current predecessor once. Actions preserve branch lineage.
   Nested splits may rejoin only after their children have themselves rejoined;
   then the enclosing sibling lineage is restored. Out-of-order lists are allowed
   only where canonical sibling order is recovered from the internal lineage.
6. Authorization is an explicit assumed source context; node hash/currentness
   checks do not verify signatures or authenticate a ledger. ExactPlan roots can
   only use their signed allowed plan hashes; OutcomeIntent roots preserve the
   originating hash and enforce the same bounded permissions. Branch-specific
   quotas attenuate root authority; a child cannot replace the original intent.

These decisions resolve the executable disjoint-history gap with stated limits.
They do not select full multi-input policy composition, cross-origin merging,
shared-state concurrency, token-piece splitting, native proof obligations or
signature domains for production. The financial schema author retains full38 ABI
ownership. New domain MORIARTY-SIGNING-BRANCH-EXAMPLE/4 separates these graph hashes
from unchanged03 context hashes.

## Closed records and actual validation

branch-schema-04.json is a closed JSON Schema with tagged Split/Action/Join
nodes. The root is a complete03 seed document, independently checked first.
Every output contains the complete03 state, all account residual gross/fee
quotas, owned account list, one obligation-owner bit, and retired work totals.
No output is an opaque financial blob. All maps are the exact bound profile map.

Split's body is the finite list of account/work/debt-owner allocations plus
ordinary/recovery retirement. Action's body is the exact03 typed selected plan.
Join's body is the empty closed object: its result derives from its predecessor
siblings and stored split prefix, not a caller merge formula. Node hashes include
kind, origin, profile, predecessors, body, outputs, charges and observation time.
References name node digest/output index. The graph's current list must equal
all remaining live outputs exactly; omission cannot erase an unresolved branch.

The graph profile hash commits BOTH branch parameters and the complete04 schema.
It also binds the unchanged03 context profile/schema hash. An explicit new
compositionAuthorization body binds originating authority and graph profile and
allows Split/Join; the external source-test context must supply its exact digest.
This is deliberately separate from03 authority, which did not authorize branch
composition. A self-declared graph cannot manufacture that assumed authority.
This source context is not an implemented signature verifier or registry proof.

Validation order is finite shape; seed03 relations; profile/origin/composition
authority; each node's hash and immutable identities; unique live predecessors;
activation; derived ownership/financial/work transitions; exact charges and full
output equality; atomic local live-set replacement; exact terminal live list.
Shared-prefix counters are retained internally from the validated split record,
never taken from a later caller's selected prefix. Seed and graph intermediate
states, returned financial counters, remaining authority and backing are checked.
The external consumed-output set can make a locally valid predecessor unavailable.
A whole-graph result is computed without mutating the input or external context.

The positive retained graph starts after a fee-bearing prefix. It splits A/B
account ownership, advances10A against a10A debt on one branch and receives another
20B less2B fees on the other, then joins. Gross alice.B is40, not60; outstanding
debt10 survives; ordinary work5 and retired ordinary1 remain distinct. A separate
full cancellation→split→join case preserves the exact reserved2B duty. Both source
checks are unsigned and do not claim financial completion with residual debt.

Fourteen connected tests pass. Mutations rehash graph nodes and dependent refs,
then test exact first rejection for duplicate/consumed predecessors, cloned work
or caps, overlapping account ownership, immutable origin/profile, duplicated
prefix, dropped debt/duty, restored retired work, wrong charge and sibling spend.
The initial log preserves one fixture mistake: a mutated external context was not
passed to the checker. Fixing that test made the intended authority rejection
observable; it was not a checker acceptance repair. No old03 file was edited.

Open beyond this revision: full38 financial records, arbitrary fungible-piece
partition, general multi-obligation and incompatible-policy composition,
shared-state concurrency, native proofs and real signatures/currentness. These
remain required RP01/MC06 interfaces, not silently accepted approximations.

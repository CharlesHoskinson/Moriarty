# SP01 financial-operation contract — proposed correction04

This source proposal preserves the complete 38-operation roster and supplies
closed records, typed rule binding, deterministic record construction and the
financial consumer contract. It is **not yet accepted**, an implemented financial
interpreter, or a successor freeze. Draft03's independent GPT-6 request for changes
is retained; its Grok source approval did not remove those blocking findings.

Read the proposed contract in this order:

1. [Correction and review scope](CORRECTION-04.md).
2. [Closed financial types](financial-schema.md), [machine-readable schema](financial-schema.json)
   and [38 operand bindings](financial-operation-bindings.json).
3. [Record generation](record-generation-04.md), [complete field inventory](field-generation-04.json)
   and [claim/owner assignment](OWNER-ASSIGNMENT-04.md).
4. [Financial consumers](financial-consumers.md), [typed rule admission](rule-binding-and-admission.md),
   [actual policy events](policy-events-04.json) and [logical work](financial-work.md).
5. [Mixed arithmetic](MIXED-ARITHMETIC-PROPOSAL.md), [pure extensions](financial-pure-extensions.md),
   [signatures](financial-pure-signatures.json) and [remaining obligations](REMAINING-OBLIGATIONS.md).

The reference schema contains 206 records, 31 enums and eight finite variant
families: 41,113 canonical bytes and 3,467 JSON nodes. These fit the inherited
65,536-byte/4,096-node schema limits. Each populated state/effect and total work
must independently fit its own limits; a fitting schema proves neither of those.
The field inventory has 47 construction roles and 40 update roles, covering
1,163 field occurrences across all 38 operations. These are source definitions,
not a claim that a checker executes their prose equations.

Correction04 binds policy rules to actual computed movement legs, including net
outputs after fees, and gives each generated Duty, reservation, receipt, supply,
claim and owner record an explicit source. Share entitlements describe vault
ownership without inventing a fixed cash liability or legal promise. Immutable
external claim references are separate from local payable duties.

Vault mint/burn quote the state before their own asset flows and then perform
those flows internally. Position payouts release the precise reservation and
pay internally. Both changes avoid impossible or incorrectly priced prior-payment
compositions. Actual policy, gross/fee caps and complete effects still apply.
A 100-share request filled by 40 for 52 cash, then claimed by 20 and canceled by
60 shares, retains 32 cash owed and the original gross debit of 100.

The separate kernel resource envelope keeps final work accounting out of the
business Post consumed by Ensure, closing a computation-accounting cycle without
changing Boolean /1's actual selected-node costs. An immutable pre-action work
checkpoint is explicitly named. Final accounting remains mandatory acceptance
and correspondence data, not an optional report field.

The pure proposal preserves the original 40 constructors and adds eight explicit
constructors/eliminators. Those exact pure48 signature bytes and the 56-node
reference vault body remain unchanged from draft03; both draft03 reviewers gave
scoped pure-design approvals. Their approval does not adopt the new financial
consumer, assignment or work-envelope choices here. [Nine retained Boolean /1
source dependencies](boolean-dependency-04.json) fix the evaluation version used
by financial rules; the published 101-file expression candidate remains intact.

Checks actually run for this source draft:

- [Field/binding checks](field-generation-check-04.json): exact field coverage,
  typed references, all 38 rows, 22 policy-operation families, 12 malformed-source
  controls and 19 independent arithmetic/assignment discriminators.
- [Schema checks](schema-check-04.json): closed references, acyclicity, finite
  encoding size, 35 rule kinds, six rejected malformed schemas and 101 protected
  expression pins.
- [Inventory checks](draft-check-04.json) and [pure closure checks](closure-check-04.json):
  retained target/case counts, exact constructor inventory and reference-body math.
- [Independent request](independent-request-recheck-04.json) and
  [vault arithmetic](independent-vault-recheck-04.json) rechecks: the unchanged
  root-authored 13 and 20 challenges, including full-width multiplication cases.

These are source/structure/arithmetic checks. They do not establish a financial
transition interpreter, source/Core/K correspondence, authenticated observations,
complete authorization, proof-carrying transactions or Midnight execution.

For conflicts with earlier prose, this README, CORRECTION-04, record-generation-04
and OWNER-ASSIGNMENT-04 supply the explicit correction; current schema/bindings,
consumer/rule/work files supply the complete proposed layer. The early
STATE-AND-COMPOSITION, OPERATIONS/operations and rule-signatures files retain
source/case lineage and are not alternative active semantics. Original03 files
and audits remain available through [their preservation map](original-candidate-03-preservation.json).
All 137 original pins verify through that map. No private input, network,
compiler, prover, service, wallet or transaction operation was used here.

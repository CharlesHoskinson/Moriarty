# SP01 financial-operation contract — proposed closed-schema draft03

**Ready for semantic review; not a successor freeze or runtime implementation.**
This draft preserves all38 archived operation names and their exact source
crosswalk. It now supplies closed finite operand/effect records,35 typed rule
families, explicit generic consumers and a proposed logical helper-work schedule.
It leaves the published101-file expression candidate unchanged.

Read in this order:

1. [State and expression composition](STATE-AND-COMPOSITION.md) for the inherited
   financial obligations and proposed body → operations → Ensure order.
2. [Closed financial schema](financial-schema.md), [machine-readable Σ](financial-schema.json)
   and [38 operation bindings](financial-operation-bindings.json).
3. [Operation consumers](financial-consumers.md), [rule binding/admission](rule-binding-and-admission.md)
   and [logical helper work](financial-work.md).
4. [Mixed arithmetic](MIXED-ARITHMETIC-PROPOSAL.md), [pure extensions](financial-pure-extensions.md)
   and [constructor signatures](financial-pure-signatures.json).
5. [Target/asset crosswalk](crosswalk.json) and [remaining obligations](REMAINING-OBLIGATIONS.md).

The reference Σ has186 records,28 enums and7 finite variant families. Its canonical
encoding is35,139 bytes and2,968 JSON nodes, within the existing65,536/4,096 limits.
Each actual financial snapshot/effect is separately bounded. Schema size does not
prove that every possible populated profile fits those limits or lifetime work.
Variant payloads retain asset/vault/holder identity; nominal debt is not an asset.

The arithmetic/type proposal preserves all40 original constructors and explicitly
adds eight: ConstructAmount, ConstructShares, ConstructVariant, ProjectVariant,
ProjectSome, ConvertUInt, ScalarValue and Select. The48-constructor result needs
independent design review. Select supplies conditional values with exact arm types
and selected-arm evaluation; Boolean /1 remains separately owned and versioned.
UInt256 permits full UInt128 vault products; NetAmount retains the full signed
difference of unsigned128 incoming/fee counters. None of these pure values creates
cash, grants authority or settles a claim.

Concrete financial corrections include:

- Receipt class and purpose prevent a Fee payment from also settling principal
  or interest. Allocation consumes only previously funded, exact-role receipts.
- Request fill reduces input ownership and reservation together. The independent
  100-share commitment →40 filled for52 cash →20 claimed →60 shares refunded
  history preserves32 cash owed and all100 gross allowance consumed.
- Amount, nominal principal/interest, shares, fees and output entitlement use
  separate units and explicit role-bound conversion. Dynamic quantities have
  explicit constructors; no hidden UInt-to-Amount/Rate cast supplies rule results.
- Exact typed observation payloads, current-time domain and vault funding-account
  identities replace previously implicit inputs. Their authentication remains a
  mandatory separate judgment over the exact bytes.
- Each effect retains intermediate typed object preimages as well as hashes;
  a later change to the same object cannot erase the earlier transformation.
- Movement checks are ordered per leg: that leg's funding/reservation check, then
  its cap check. Earlier cap failure precedes later funding failure; all changes
  still roll back. Caller-provided work totals do not determine charged work.

[Nine additional claim-link/message cases](claim-link-cases-01.json) specify the
mint/debt linkage, message duty closure and fee-purpose failures; they are not
executed transition tests.

The [schema check](schema-checks-06.json) verifies all38 bindings,35 closed rule
pairs, reference type closure/acyclicity, six rejected malformed-schema controls
and101 protected expression pins. The [inventory/arithmetic check](draft-checks-05.json)
retains24 action targets,11 asset requirements,8 asset cases and17 calculations.
[Additional closure checks](closure-derivations-02.json) cover the48-constructor
catalog, exact56-node synthetic vault body and11 arithmetic/purpose discriminators.
The [independent request recheck](independent-request-recheck-03.json) and
[independent vault recheck](independent-vault-recheck-01.json) retain root's13 and20
specified cases, including four wide-product discriminators. These checks are
not a financial interpreter, complete source/AST typechecker, K reduction run,
cryptographic authorization proof or ledger result. The reference vault body is
an initialized, zero-fee example; it does not claim a complete product catalog.

Earlier proposed rows in OPERATIONS.md/operations.json and rule-signatures.json
retain their original source/case lineage. Where their early abstract type or
consumer wording differs, the draft03 files linked above supply the proposed
closure. Immutable snapshots preserve those prior bytes and their check results:
[draft01](draft-01-preservation.json), [draft02](draft-02-preservation.json), and
[pre-closure draft03](draft-03-preservation.json). Root's independent cases remain
unchanged. No private content, network, compiler/proof/service/wallet/native
operation or acceptance-flag change occurred in this task.

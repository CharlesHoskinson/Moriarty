# Closed financial schema and tagged variants — proposed draft03

This draft supersedes draft02's mathematical placeholder types. It does not edit
published expression candidate02. `financial-schema.json` contains a concrete
closed reference Σ:186 record types,28 enums,7 variant families, all38 operation
operand records, a common complete effect record and35 closed rule input/result
pairs. `financial-operation-bindings.json` binds each operation to those types.
These are source specification proposals; no interpreter, K or signature claim.

## Choice: one tagged payload, preserving its nominal type

A profile declares a new finite `variantTypes` map in Σ. Each named family maps
unique identifier tags to exact existing/extended value types. A Variant is not
an untyped JSON union. Type encoding is `["Variant",family]`. Value encoding is
exactly `{"tag":tag,"value":v(payloadType,payload)}`; no extra keys. The selected
tag must occur in the family and its payload must match that exact type. A value
has one variant node plus its payload nodes. Depth is one plus payload depth.
This extends the exact candidate02 canonical JSON rules without changing them.
The type reference and wrapper, when present, are included in W byte bounds.

The reference FinancialQuantity family contains Amount<AssetA>, Amount<AssetB>,
Amount<USD> and Shares<Vault01,H> for five explicitly declared holders. A share
payload does not become an Amount, and its holder index is retained. NetQuantity
and SignedQuantity similarly preserve asset indices. NominalQuantity contains
distinct Record<NominalUSDQuantity> and Record<NominalIndexQuantity>, each with a
UInt128 quanta field. A nominal denomination is not silently installed as a
ledger asset or a Price denominator. Explicit NominalConversion binds denomination,
asset, numerator, positive denominator and rounding independently.

A concrete JSON size comparison is in `schema-checks-04.json`. Unlike an option-
record sum, a tagged value stores only the selected payload. An option-record
alternative needs all fields, hits the64-field limit before128 variants, and does
not give a way to extract Some using the existing40 constructors. This choice
avoids those costs without erasing asset/holder identity.

**Option remains separate.** ProjectVariant below eliminates this new Variant;
it does not eliminate Option<T>. The explicit ProjectSome constructor in financial-pure-extensions.md separately
eliminates Some and rejects None. Administrative helpers also make their exact
Some/None tests; neither operation silently selects a default payload.

## New pure constructors and source projection

In addition to the separately proposed ConstructShares, add two explicit pure
constructors. Together with financial-pure-extensions.md, the proposal preserves
all40 original constructors and has47 total.

`ConstructVariant(family,tag,value)` has identifier metadata family/tag and one
evaluated value child. Resolve family/tag, typecheck that child and require exact
payload type. Its result is Variant<family>. Charge one node plus child nodes;
return the tagged value without state/effects. Unknown family/tag is TYPE_NAME;
wrong payload type is TYPE_MISMATCH; final aggregate overflow is VALUE_BOUND.

`ProjectVariant(tag,value)` has identifier metadata tag and one evaluated child
of type Variant<F>. Static typing resolves the tag in F and returns its declared
payload type. Charge one node, evaluate child once, then require selected tag
equals metadata tag; otherwise reject VARIANT_CASE at the projection's span.
No payload from a different branch is cast to the expected type. Child failure
retains the child code/span. Both constructors use the usual N/P shape with exact
named operands and the same lexical/static/error precedence. No hidden branch
interpreter, pattern matching or general callback is introduced.

Proposed explicit source forms are `variant<Family.Tag>(expr)` and
`project_variant<Tag>(expr)`. An Eq against a separately constructed Variant can
be a guard, but cannot inspect a tag without its payload. For guarded projection,
use a source action whose signed/declaration context fixes the variant, or an
explicit preceding condition already deriving that same context; a mismatching
projection still deterministically rejects. A future user-friendly match form
requires its own grammar/elaboration decision, not an assumed meaning here.

Financial source still uses Amount and Shares names. An operation builder whose
argument expects FinancialQuantity elaborates a statically known Amount<A> or
Shares<V,H> into the unique matching ConstructVariant tag. This is explicit
specified elaboration and adds its node charge; it does not erase the payload
index or coerce its units. More than one matching tag is invalid profile schema.
A dynamic Variant argument is already typed and is not reboxed. The source AST,
elaborated node and cost must all be retained in correspondence examples.

## Exact schema embedding and finite instantiation

All original Σ keys and their shapes remain as in candidate02: roster arrays,
record-name→field/type maps, enum-name→sorted-member arrays, field/argument/
observation maps and operation-name→operand-record-name map. The **only new Σ
key** is required `variantTypes`, mapping each family name to a tag/type map.
Original expression-only profiles do not accept it until this version is adopted.
All existing type tags keep their encoding. Add the five arithmetic tags from
MIXED-ARITHMETIC-PROPOSAL plus Variant and UInt256. Every record/type name in the reference JSON
resolves; there is no Any, opaque object, wildcard financial value or recursive ID
payload. ID references break history cycles rather than nesting old snapshots.

Profile instantiation is finite and deterministic:

1. Supply sorted unique asset, vault, party and nominal-denomination declarations
   with their complete identities; require at most16 assets,16 vaults,32 parties
   and16 nominal units. The total type/Σ bounds still apply simultaneously.
2. Assign one unique identifier tag to each declared admissible quantity type,
   including each supported vault/holder pair. Do not generate tags by an unsafe
   concatenation of arbitrary names. Require at most128 alternatives/family and
   reject duplicate payload types in FinancialQuantity/NetQuantity/SignedQuantity.
   A missing receiver share type rejects profile/operation binding; no runtime
   allocation of new types occurs.
3. Instantiate nominal quantity records with exact registered names and quanta
   field; instantiate indexed arithmetic/price types with declared identities and
   scales0..18. The reference selects scale4 and concrete A/B roles. Other rule
   instances carry explicit immutable A/B/S role bindings and closed generated
   input/result record names. Every such instantiated type is fully present in Σ;
   no type variable survives encoding or runtime. Binding collision rejects.
4. Install the fixed financial records and all38 descriptor records with those
   concrete variants; each `operations` entry names its complete Args record.
   Every operation emits the same closed OperationEffect envelope, whose allowed
   fields/deltas are fixed by its consumer. Empty lists remain present.
5. Check all names, shapes and references; include Variant payload edges in the
   existing schema acyclicity check, including edges through Option/Collection/
   Operation. Reject cycles before admitting values. All record fields≤64,
   collections≤128, family choices≤128; at most256 named record/enum/variant types.
   Source declaration limits and generated schema limits are separate checked
   quantities, never silently multiplied.
6. Canonical Σ must fit65,536 bytes and4,096 JSON-tree nodes, depth64. The reference
   fits those unchanged limits. Individual W values, Pre/Post/Args/Obs and complete
   financial effect arrays must each fit65,536 bytes/4,096 value nodes/depth64.
   Larger profile expansions reject, rather than widen bounds automatically.

Each named Id sort is a distinct Record<SortId> containing exactly `name:Text` satisfying
ASCII `[A-Za-z][A-Za-z0-9_]{0,63}`. A DebtId is not a ClaimId even with equal name.
ReceiptId is the explicit exception: its distinct record wraps a Hash as specified
in financial-consumers.md. Names resolve only in the corresponding finite registry. Hash is exactly
Record<Hash>{hex:Text},64 lowercase hex characters, semantic SHA256 only. It is
not a decoder for the Midnight SDK's differently sized transaction IDs. DomainId
binds an immutable network/deployment domain; equal display names do not prove
same external chain. Time fields are UInt64 seconds in the profile's fixed time
domain; Period requires end>start and means [start,end). No host current time,
calendar default, floating point, Unicode normalization or case folding is used.

AssetDefinition.nativeIdentifier is bounded Text retaining its domain's exact
identifier; a concrete ledger profile additionally validates that domain's native
encoding. That validator's result is a mandatory acceptance premise, not a
financial-operation Bool. Generic semantics does not invent a universal address
format. Similarly AuthenticationReference holds exact document/key/signature
commitments; actual witness authentication and currentness are separate mandatory
relations supplied by the signing/ledger specification, not booleans in state.

## Refinements before execution

After shape/Σ typing, validate complete financial state in sorted collection-key
order: unique IDs and balance/resource keys; all references resolve; every typed
quantity matches its fixed resource/holder role; every Amount/Nominal/Shares/cap
component lies in its exact range. Supply counters have burned≤issued when the
model represents issuance; external inventory is separately funded, not invented
as new issuance. Debt P/I have the same nominal tag; Settled requires both zero.
Request C=P+F+R, E=K+claimable, input reservation=P and output reservation=claimable
in the exact resources/escrows; claims and beneficiaries match. Message q=0 has
no reservation; q>0 Pending has its exact reservation. Terminal receive/refund has
no remaining message escrow reservation. Every exclusive reservation is counted
once and total reserved≤owner's matching balance. Duties original=discharged+
remaining with correctly typed denomination/resource and no duplicated backing.

Counters are checked UInt128, monotone across successful actions, with gross≤cap
and fees≤cap. Net is exact NetAmount difference, not SInt128. Instance ordinary/
recovery work uses the finite work domain and never increases except initial
signed Genesis funding. Administrative modes do not erase claims. A duplicate
priority within a collateral group rejects. Missing/revoked/stale authority or
observation evidence fails its independent context predicate before a financial
result can be accepted. The new schema is no substitute for that predicate.

No runtime caller may supply an alternative schema marking a financial field
ordinary or altering a rule input/output. The complete profile/schema/operation
consumer/version is committed by authorization and semantic statements.

## Rule quanta and fixed construction roles

The `closedRuleSignatures` and exact input/result record definitions supersede
abstract `rule-signatures.json` types where this closure makes a result numeric.
An output UInt128 is never an unlabelled authorization. Before evaluation, bind
an immutable RuleInstance to one operation role tuple. The consumer checks tuple
identity, reconstructs the specified Amount/Shares only in that role, and applies
the same quantity, actor, backing and cap checks as an explicit operand.

| Rule result | Fixed consuming role |
| --- | --- |
| FeeQuote.fee | FeeArgs.quantity asset, which must equal FeeArgs.basis asset |
| ExchangeQuote.inputFee | Exact ExchangeArgs.input asset |
| ExchangeQuote.output,outputFee | Exact ExchangeArgs.minimumOutput asset; distinct from input |
| SettlementQuote.requiredPayment,permittedExcess | Asset in the debt's bound NominalConversion; its denomination equals debt P/I |
| VaultConversion.assets,assetFee | VaultDefinition.underlying, asset-only for this conversion form |
| VaultConversion.shares | Exact selected vault and ShareMint/ShareBurn holder |
| ShareConversion.destinationShares | Exact ShareConvert.destination vault and same source/destination holder |
| RequestConversion.inputFee | Request.inputResource |
| RequestConversion.outputEntitlement,outputFee | Request.outputResource |
| EventPayout.entitlement | Existing EventClaim.remaining asset |
| RewardQuote.reward | Existing Reward.pending asset |
| SlashQuantity.slashedUnits | Existing Reward.vault and holder; exact shares |
| RebaseRate.newRateMantissa | Existing Reward.rate scale4; UInt128 must be<=2^127−1, then compare exact nonnegative operand Rate mantissa |
| AccrualQuote.interestMagnitude | Debt principal/interest nominal denomination, never a ledger asset |
| DebtAllocation/LossAllocation deltas | Same Debt P/I nominal denomination |

The reference monomorphization A=AssetA/B=AssetB is illustrative, not an operation
that may pay every resource as AssetA. A full profile fixes actual role substitutions
and typed rules before admission; wrong substitutions reject RULE_BINDING. Scalar
result construction is a fixed consumer step with one typed reconstruction charge,
not a pure UInt→Amount cast. The explicit source ConstructAmount remains required
when the source itself builds a dynamic Amount. Rebase uses a numeric result to
avoid an undeclared dynamic Rate constructor. It cannot reconstruct a negative or
out-of-range Rate. Future other-scale profiles instantiate their exact scale.

Observation records include a typed ObservationValue payload, its canonical hash,
purpose, schema hash, issuer/domain/time and authentication reference. Current time
is OperationContext.now in its explicit timeDomain. No host clock or absent payload
oracle supplies a rule input. Authentication of these exact bytes is still a
mandatory external judgment; storing a payload and hash does not prove truth.
The reference payload family is closed; a new evidence shape needs an explicit
profile declaration/version, not an Any field.


ChangedObject is the seventh finite variant family. Each ObjectChange carries the
exact typed before/after object payload and their hashes. Intermediate financial
changes therefore remain reconstructible even if an object changes several times
in one action. This additional retention consumes the same bounded effect bytes,
nodes and work; it is not excluded from the profile size limit.

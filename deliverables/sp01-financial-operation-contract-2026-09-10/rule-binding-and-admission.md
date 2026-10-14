# Closed rule bindings and admission — proposed correction04

This completes the binding mechanism of DECLARATIVE-RULE-INTERFACE.md. It defines
finite selection and encoding, not an implementation or proof that the entire
ACTUS/DeFi rule catalog has been populated. All35 rule kinds resolve through the
closed input/result records in financial-schema.json. The full38 ABI cannot run
with a missing body, placeholder return value or host callback.

## Exact profile table and selectors

The rule table is canonical JSON under the same key/string/number rules as the
expression representation. It is separate immutable profile metadata, not a
recursive value embedded in Σ. Exactly these root keys are required:
`{version,constants,definitions,instances,resourceAccounts}`. version is the identifier
`FinancialRules2`. constants is a name-keyed object of exact typed W values;
definitions is a sorted ID-keyed object of the RuleDefinition records below;
instances is a sorted RuleId-keyed family object. A family is an exact map from
FinancialOperation name to one Instance with that operation. Each root map has
at most128 entries, each family at most38, and total Instance records across all
families at most128. A stored RuleId resolves the family; execution selects
exactly (RuleId,actual outer operation), never an arbitrary first match. Combined
rule-table bytes<=65,536, total Core nodes<=4,096 and maximum depth64. Constant
values obey their declared types and value bounds. resourceAccounts is the exact sorted map of all source action names (at most64)
to Ordinary or Recovery. Resolve C.action once before body evaluation; missing
or extra action names reject admission. The selected class cannot change within
an action, and grants/mode eligibility remain independent mandatory judgments.
No duplicate JSON keys or alternative hash-only body is admitted.

Definition has exactly `{version,kind,inputType,resultType,body}`. version is a
Version name; kind is one of the35 exact names; inputType/resultType are the
concrete signature Record type arrays, and body is the exact proposed N encoding.
The body's ReadArg environment is the input record's lexical field map. It is a
pure expression with the restrictions in DR1, including all newly proposed pure
extensions and Boolean /1. No recursive function call or arbitrary work counter
is encoded. The full syntax node count is derived; earlier maxNodeEntries notation
means this derived value, not an accepted author total.

Instance has exactly `{definition,operation,roles,bindings,readSet,facts}`. definition
names a present definition; operation equals its family-map key and is one of its definition's allowed38 names.
Every instance in a family has the same kind and complete concrete role identity;
its typed selectors may differ where the outer Args records differ. A reference
used by multiple operations requires each corresponding family entry at admission.
roles is a lexical object of the definition's concrete asset/vault/holder/scale
substitutions: exact constants fixed by profile admission, not runtime outputs.
bindings maps every input field exactly once to a selector. readSet is a bounded
list (at most128) of state/context root selectors this instance is permitted to
read. Duplicate roots reject. facts is exactly `{observation:[]}` or `{observation:[selector]}`. Its selector
returns one ObservationId from Operand/Constant/State or an available generated
helper key; it cannot itself use ObservationPayload or another facts root.
Any use of ObservationPayload, MovementPolicyFacts or ReleaseValuation requires
this exact selector and a present unique observation. An operation's explicit
evidence operand must equal it when both refer to that same required fact.
No unspecified observation-name lookup is accepted. The referenced profile/definition/role tuple and
selector table are committed by the same authorization as the Core program.

Selector is exactly one of these closed JSON forms; every path is an ordered
list of at most8 projection steps:

- `{"root":"Operand","path":[steps]}` starts at this operation's Args record.
- `{"root":"Context","name":contextRoot,"path":[steps]}` starts at one of
  the closed roots Context, ObservationPayload, MovementFrozen, MovementPolicyFacts,
  CurrentMovement, CurrentRelease, OperationKind or CurrentResolutionMode. Context is OperationContext;
  the remaining types are respectively ObservationValue, Bool, PolicyFacts,
  CurrentMovement, CurrentRelease, FinancialOperation and ReceiveOrRefund and are derived by the fixed
  observation/restriction consumer below, never supplied as extra caller fields.
- `{"root":"Constant","name":identifier,"path":[steps]}` starts at that exact
  immutable constant's W value, whose type is retained.
- `{"root":"State","table":fieldName,"key":[keyParts],"path":[steps]}` starts
  at a uniquely selected row of that exact FinancialState table. keyParts are
  Operand/Constant selectors or the explicitly permitted current-helper keys; State nesting is forbidden. In addition to Operand/Constant keys, a Context
  CurrentMovement/CurrentRelease field selector may provide a key only during
  that helper. This supports exact computed actor/resource lookups and cannot
  access a different helper invocation. Other Context key roots reject. Their
  output types must equal that table's exact key types. Full-scan lookup is
  charged; no random host property access or inferred owner is permitted.

Each step is exactly `["Field",name]`, `["Some"]`, `["Variant",tag]`,
`["Quanta"]`, `["Mantissa"]`, `["Magnitude"]` or `["Negative"]`. Field requires
that exact Record field; Some requires present Option, otherwise RULE_BINDING;
Variant requires that declared tag and selects only the matching payload,
otherwise RULE_BINDING. The four scalar steps use exactly the ScalarValue typing
and value equations, with one typed-destructor charge; they perform no arithmetic,
unit conversion or ownership transfer. Each path has a unique statically known
result type. A collection is selected whole; no selector indexes, filters or
aggregates it. A rule may use explicit Core ProjectIndex and bounded literal
syntax if needed. Width conversion/arithmetic belongs in the Core body.

Table keys are exact typed tuples: asset/vault/party/nominal/encumbrance/debt/
position/request/message/event/reward/duty/instance/externalClaims use their id field; balances
use (owner,resource(quantity)); supplies use resource; counters use
(authority,party,resource); nominalCounters use (authority,denomination). Key
parts reconstruct those typed identities only. Context observations/policies/
restrictions are bounded collections; their selected facts are supplied by the
fixed observation/policy consumer described below, not a selector search loop.
An instance readSet may declare exact additional immutable observations/state
rows required by a product formula; it cannot mutate them. The compiler retains
this complete dependency footprint in the profile and signing/disclosure view.
A rule's input field may not bind a precomputed policy decision/receipt/authority
verification result. Facts such as a registry frozen field have a declared source
and cannot be rebound to an unrelated constant `false`.

Fixed observation selection first resolves the exact ObservationId named in the
operation/current record, or the exact facts.observation selector above.
The consumer checks its purpose, subject, domain, schema, payload hash, issuer,
expiry and mandatory authentication judgment. It then exposes the typed payload
as that RuleInstance's admitted context fact. For MovementPolicy, frozen is the
OR of the exact matching resource/holder restriction flags from the current
registry after a complete ordered scan; policyInputs is its selected authenticated
PolicyFacts payload. Missing required payload/rule/time rejects. A Price cannot
be substituted for Time merely because both serialize as decimal strings.
No `authenticated=true` field replaces the cryptographic/evidence premise.

These definitions replace the unused draft Binding record's broad text fields;
that record is not an alternative accepted selector codec. It can be removed
from a concrete instantiated profile if unreachable. Unknown selector tags,
extra keys, unresolved references, overlong paths or input/result mismatches
reject profile admission before reading private action inputs or evaluating Core.

## Ordered admission and deterministic runtime

1. Exact canonical syntax/Σ/profile/rule-table bounds and name/reference closure.
   Check duplicate keys before field coverage; reject recursive type graphs.
2. Monomorphized48-constructor typing and full operation/rule signature checking.
   Resolve all selectors/readSets/nominal result roles; reject undeclared access.
   Every operation retained in the full38 profile requires its applicable rules.
3. Admit exact input values, full financial state/context refinements and mandatory
   authenticated identity/current-predecessor relations. Time and observation
   values come from committed inputs; no host clock or lazy network fetch.
4. Evaluate action body into ordinary staged writes and ordered financial
   descriptors. Body Require failures occur here; no financial mutation occurs.
5. Reconstruct the conservative financial helper bound from exact input shape,
   descriptor list and rule syntax. Reject WORK_BOUND if it exceeds remaining
   ordinary work. No caller-supplied helper count participates.
6. Process each descriptor with financial-consumers.md. Select input fields in
   lexical order, evaluating each selector once and charging every field/path/
   lookup step. Materialize the exact input Record; evaluate rule Core once.
   Check exact result type/range and fixed role; construct effects only through
   that operation's consumer. Use fixed per-leg funding-before-cap precedence.
7. Check merged state reference/claim/frame invariants and all consumed release
   tickets. Evaluate Ensure in original source order against that stable business post. G10 resource fields are not in its type.
   Finally check business post/effect bounds and use the fixed resource-accounting
   epilogue in financial-work.md; subtract all actual work including that charge
   and construct FinancialPrepared with its separate kernel-owned envelope. No step publishes.
8. Ledger/proof acceptance separately authenticates and consumes the exact
   predecessor/authority/native relation. Preparation cannot satisfy that gate.

All failures discard tentative state/effects, preserve the actual earliest
code/span/rule/substep and retain the original predecessor and stored budgets.
A successful preparation has no unspecified choice of row, formula, work count,
partial resource credit or claim-deletion callback. Product admission still needs
actual rule bodies and source/signing/K derivations; this schema is not those
implementations or a claim that every finance model fits this particular profile.

## Correction04 helper binding and visibility rules

The mandatory fields and deterministic family dispatch in record-generation-04.md
G1 override any broader selector possibility. MovementPolicy cannot bind actual
quantity/source/recipient/resource from minimumOutput or another Operand/Constant.
ReleasePolicy similarly binds the four generated CurrentRelease values. Each
instance's input fields are still materialized once in lexical order. Current
helper roots are unavailable outside their helper call; stale previous-leg values
are never a fallback. Every family entry is fully statically typed against its
own operation Args, helper availability and closed input/result types.

The exact RuleDefinition codec is the five-field object above; DR1's earlier
maxNodeEntries/readSet-on-definition sketch is superseded. readSet belongs to
Instance and counts are derived from body. No alternative codec is accepted.

Kernel-only type admission follows G10. Traverse every accessible type through
Record/Variant/Option/Collection/Operation edges, including aliases and constants,
and reject a path to FinancialPrepared, ResourceEnvelope or InstanceResources.
Current immutable WorkCheckpoint is allowed and explicitly denotes before-action
resources. All selectors see stable business state or an explicitly named current
helper, never uncommitted final accounting.

CurrentResolutionMode exists only while MessageResolution runs. Its value is
Receive for MessageReceive and Refund for MessageRefund, from the actual outer
operation. MessageResolutionInput.resolutionMode must select this root without
substitution by Constant/Operand; the enum is not obtained by an undeclared cast
from FinancialOperation. Book-only MovementPolicy events and their exact actor/
quantity formulas are listed in policy-events-04.json. They do not debit cash a
second time. The nonzero policy-event ordinal is separate from receipt ordinal.

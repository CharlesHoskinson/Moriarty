# Closed schemas for `moriarty-bounded-atomic/1`

Status: S2 provisional corrected candidate, specified-only and not frozen or implemented.
Fresh independent Fable and GPT-6 audits of these exact bytes remain required.
Every record below is closed: all listed fields are required and no other field
is permitted. Arrays are ordered as stated; JSON `null`, JSON number tokens, and
omitted fields are never substitutes. The literal versions in this document are
part of the accepted bytes.

## Scalar and canonical JSON codec

`UInt128Text` is `0|[1-9][0-9]*` in the inclusive range `0..2^128-1`.
`SignedExponentText` is `0|-?[1-9][0-9]*` in `-16..16`; `+1` and `-0` reject.
`Digest` is exactly 64 lowercase hexadecimal characters. `Identifier` is 1..64
ASCII characters matching `[A-Za-z][A-Za-z0-9_]*`, is not a grammar keyword, and
is not `constructor`, `prototype`, or `__proto__`. `TextValue` is a Unicode scalar
sequence of at most 256 UTF-8 bytes. `OpaqueBytes` is lowercase, even-length hex.
`FieldLabel` is an Identifier or exactly `asset` or `amount`.

`moriarty-canonical-json/1` accepts only closed-schema objects, arrays, strings,
and booleans. It rejects numbers, null, duplicate/unknown keys, lone surrogates,
and a decoded value outside its applicable bounds. Object keys are emitted in
increasing unsigned ASCII-byte order. Arrays retain schema order. Encoding has no
whitespace. A string emits `\"`, `\\`, `\b`, `\t`, `\n`, `\f`, and `\r` for those
seven characters; any other U+0000..U+001F character emits lowercase `\u00xx`;
all other Unicode scalar values emit their shortest UTF-8 bytes without escaping.
No Unicode normalization occurs. A received canonical object is accepted only
when decoding and re-encoding produces byte-for-byte identical input.

The following shared records are exact:

```text
Span          = {endByte:UInt128Text, startByte:UInt128Text}
SourceRef     = {generatedTag:GeneratedTag, sourceHash:Digest, spans:[Span]}
GeneratedTag  = "Source" | "AmountUnitRetention" | "PolicyCheck" |
                "SettlementCheck" | "StatusDerivation"
UnitComponent = {exponent:SignedExponentText, unit:Identifier}
UnitVector    = [UnitComponent]
BoundsRef     = {boundsHash:Digest, registryId:"moriarty-bounds/1"}
ProgramRef    = {bounds:BoundsRef, coreVersion:"moriarty-core/1",
                 profile:"moriarty-bounded-atomic/1", programHash:Digest,
                 schemaVersion:"moriarty-program-ref/1", sourceHash:Digest}
```

`Span` is half-open, `startByte <= endByte <= sourceUtf8Bytes`. In
`moriarty-core/1`, every SourceRef has `generatedTag:"Source"` and exactly one
span. All other GeneratedTag spellings are reserved for MC02 lowering and reject
in this Core version; there are no generated-check Core nodes in this profile.
`UnitVector` omits zero exponents, sorts by the ASCII bytes of `unit`, contains no
duplicate unit, has at most eight members, and uses only declared units. Its
empty value is dimensionless for numeric UInt128. Bool and Text expressions,
including comparisons, references, literals, and Not, also carry exactly `[]` as
nonnumeric metadata. Their explicit `type` distinguishes them from UInt128;
unit-vector classification applies only to numeric expressions.

## Types and values

The only type records are:

```text
UIntType     = {tag:"UInt128"}
TextType     = {tag:"Text"}
BoolType     = {tag:"Bool"}
AmountType   = {tag:"Amount", unit:Identifier}
QuantityType = {tag:"Quantity", unitVector:UnitVector}
StoredType   = UIntType | TextType | AmountType
LocalType    = StoredType | BoolType | QuantityType
BareAmountEffectType = {tag:"AmountFromOperand"}
```

The only value records are:

```text
UIntValue     = {tag:"UInt128", value:UInt128Text}
TextWireValue = {tag:"Text", value:TextValue}
BoolValue     = {tag:"Bool", value:boolean}
AmountValue   = {tag:"Amount", unit:Identifier, value:UInt128Text}
QuantityValue = {tag:"Quantity", unitVector:UnitVector, value:UInt128Text}
StoredValue   = UIntValue | TextWireValue | AmountValue
LocalValue    = StoredValue | BoolValue | QuantityValue
NamedStoredValue = {name:Identifier, value:StoredValue}
NamedType        = {name:Identifier, type:StoredType}
```

Maps are never used for typed values. `NamedStoredValue` and `NamedType` arrays
are in declaration order with unique names. An `AmountValue` always carries its
unit, including in arguments, state, effects, obligations, receipts, and hash
preimages. `BoolValue` and `QuantityValue` may exist only as evaluated expressions
and `let` locals. They cannot appear in constants, state, arguments, observations,
set results, effect operands, obligations, or public result state.

## Source AST: `moriarty-ast/1`

The root is:

```text
SourceAST = {declarations:[SourceDeclaration], horizon:SpannedUIntToken,
             lifetime:SpannedUIntToken, name:Identifier,
             profile:"moriarty-bounded-atomic/1",
             schemaVersion:"moriarty-ast/1", sourceHash:Digest,
             sourceUtf8Bytes:UInt128Text, span:Span}
SpannedUIntToken = {span:Span, token:UInt128Text}
```

Source type syntax is represented by `SourceType = {tag:"UInt128",span:Span} |
{tag:"Text",span:Span} | {tag:"Amount",unit:Identifier,span:Span} |
{tag:"BareAmount",span:Span}`. `BareAmount` is legal only in a source effect
field. Source literals are exactly:

```text
UIntLiteral   = {span:Span, tag:"UIntLiteral", token:UInt128Text}
TextLiteral   = {decoded:TextValue, span:Span, tag:"TextLiteral", token:string}
AmountLiteral = {span:Span, tag:"AmountLiteral", token:UInt128Text,
                 unit:Identifier}
BoolLiteral   = {span:Span, tag:"BoolLiteral", value:boolean}
```

`TextLiteral.token` is the exact JSON string token including quotes; `decoded`
is its decoded scalar sequence. `SourceExpression` is exactly one of:

```text
{literal:SourceLiteral, span:Span, tag:"Literal"}
{span:Span, tag:"Remaining"}
{name:Identifier, span:Span, tag:"StateRef"|"ArgRef"|"ObservationRef"|"ConstRef"|"LocalRef"}
{denominator:SourceExpression, numerator:SourceExpression, span:Span, tag:"FloorDiv"}
{left:SourceExpression, right:SourceExpression, span:Span,
 tag:"Add"|"Sub"|"Mul"|"Eq"|"Lt"|"Lte"|"Gt"|"Gte"|"And"|"Or"}
{operand:SourceExpression, span:Span, tag:"Not"}
```

Declarations and statements are exactly:

```text
ReserveDecl     = {action:Identifier, closure:Identifier, span:Span, tag:"ReserveDecl"}
UnitDecl        = {name:Identifier, span:Span, tag:"UnitDecl"}
ConstDecl       = {name:Identifier, span:Span, tag:"ConstDecl", type:SourceType, value:SourceLiteral}
StateDecl       = {name:Identifier, span:Span, tag:"StateDecl", type:SourceType, value:SourceLiteral}
ObservationDecl = {name:Identifier, span:Span, tag:"ObservationDecl", type:SourceType}
SettlementDecl  = {asset:TextLiteral, name:Identifier, quantum:AmountLiteral, span:Span, tag:"SettlementDecl"}
PolicyWriteTarget  = {action:Identifier, field:Identifier, span:Span, tag:"Write"}
PolicyEffectTarget = {action:Identifier, field:FieldLabel, ordinal:UInt128Text, span:Span, tag:"Effect"}
PolicyRoundingNone  = {span:Span, tag:"None"}
PolicyRoundingFloor = {action:Identifier, local:Identifier, span:Span, tag:"Floor"}
FieldPolicyDecl = {comparison:TextLiteral, derivation:TextLiteral, name:Identifier,
                   proof:TextLiteral, remainder:TextLiteral,
                   rounding:PolicyRoundingNone|PolicyRoundingFloor, span:Span,
                   tag:"FieldPolicyDecl", targets:[PolicyWriteTarget|PolicyEffectTarget],
                   unit:Identifier}
EpisodeStatusDecl   = {field:Identifier, literal:SourceLiteral, span:Span, tag:"EpisodeStatusDecl"}
NotionalStatusDecl  = {field:Identifier, span:Span, tag:"NotionalStatusDecl"}
NoNotionalStatusDecl= {span:Span, tag:"NoNotionalStatusDecl"}
EffectFieldDecl = {label:FieldLabel, span:Span, type:SourceType}
EffectDecl      = {fields:[EffectFieldDecl], kind:EffectKind, span:Span, tag:"EffectDecl"}
Parameter       = {name:Identifier, span:Span, type:SourceType}
GuardStatement  = {condition:SourceExpression, message:TextLiteral, span:Span, tag:"Guard"}
LetStatement    = {expression:SourceExpression, name:Identifier, span:Span, tag:"Let"}
SetStatement    = {expression:SourceExpression, field:Identifier, span:Span, tag:"Set"}
EffectFieldExpr = {expression:SourceExpression, label:FieldLabel, span:Span}
EmitStatement   = {fields:[EffectFieldExpr], kind:EffectKind, span:Span, tag:"Emit"}
ActionDecl      = {name:Identifier, parameters:[Parameter], span:Span,
                   statements:[GuardStatement|LetStatement|SetStatement|EmitStatement],
                   tag:"ActionDecl"}
```

`SourceDeclaration` is the union of UnitDecl, ConstDecl, StateDecl, ObservationDecl,
SettlementDecl, FieldPolicyDecl, the three status variants, EffectDecl, ActionDecl,
and ReserveDecl. `EffectKind` is exactly `Transfer`, `Fee`,
`DueCreated`, or `DueSettled`. Field labels are the grammar's permitted labels.
Declarations and statements preserve source order.

## Exact source spans and structural lowering

Offsets count original UTF-8 bytes, with no newline or Unicode normalization.
Every nonempty syntax span begins at the first byte of its first token and ends
immediately after its last token. Interior whitespace is included; leading and
trailing whitespace is excluded. The SourceAST span runs from `agreement` through
the final `}`; sourceUtf8Bytes also counts whitespace outside that span.

- Declarations and statements include their terminating `;` or closing `}`.
  An action includes its keyword, parameters and complete block. A policy or
  effect declaration ends at its `}`. ReserveDecl covers `reserve` through `;`.
- SourceType spans its type keyword through the last type token, including `>`
  for Amount<Unit>. Parameter spans its name through its type. EffectFieldDecl
  spans its label through `;`. EffectFieldExpr spans its label through the final
  token of its expression, excluding a following comma or closing effect brace.
- SpannedUIntToken covers only its decimal token. UIntLiteral, AmountLiteral and
  TextLiteral produced by `uint(...)`, `amount(...)` and `text(...)` cover the
  entire wrapper through `)`. Their `token` fields retain only the inner decimal
  token or exact quoted JSON token. BoolLiteral covers `true` or `false`.
  A TextLiteral used for a bare JSON token (policy documentary fields or guard
  message) covers only that quoted token; no `text(...)` wrapper is invented.
- A Literal SourceExpression initially has the same span as its child literal.
  References cover every token (`state` through the identifier for StateRef);
  Remaining covers its token. Not spans `not` through its operand. Binary nodes
  span from the left child's start through the right child's end. FloorDiv spans
  `floor_div` through its closing `)`.
- Parentheses introduce no SourceExpression or Core node. Parsing `(expression)`
  replaces only the enclosed expression root's span with the interval from that
  `(` through its matching `)`. Repeat this rule outward for nested parentheses.
  Descendant spans are unchanged, including the literal child of a Literal node.
  Thus `((uint(1)))` has expression span `[0,11)` and UIntLiteral span `[2,9)`.
- Policy targets include their complete write/effect wrapper; rounding Floor
  includes `floor(...)` and rounding None includes only `none`.

Core lowering preserves tree structure and maps each expression sourceRef to
`{generatedTag:"Source",sourceHash:SourceAST.sourceHash,spans:[expression.span]}`.
Core instructions and actions similarly use their exact source statement/action
span. No extra span normalization, parenthesis node, or generated Core node is
permitted. Literal lowering changes syntax to the corresponding tagged value,
without retaining the separate literal-child span in Core. TypedAnnotation uses
the same expression SourceRef. These rules apply before hashing the manifest.

## Typed program: `moriarty-typed-program/1`

```text
TypedProgram = {annotations:[TypedAnnotation], profile:"moriarty-bounded-atomic/1",
                schemaVersion:"moriarty-typed-program/1", source:SourceAST}
TypedAnnotation = {nodeId:Identifier, resolution:Resolution, sourceRef:SourceRef,
                   type:LocalType, unitVector:UnitVector}
Resolution = {tag:"Literal"|"Remaining"} |
             {declaration:Identifier, tag:"State"|"Argument"|"Observation"|"Constant"} |
             {action:Identifier, local:Identifier, tag:"Local"} |
             {tag:"Operator"}
```

There is exactly one annotation for every expression node, in declaration,
statement, then expression preorder. `nodeId` is `a_<action-index>_s_<statement-index>_e_<preorder-index>`.
All indexes are zero-based canonical decimal without padding. Action indexes
count only ActionDecl nodes in source order. Statement indexes count every
statement in the action, and expression preorder restarts at zero per statement.
Visit each expression root before its children; binary children are left then
right, FloorDiv children numerator then denominator, and Not has one operand.
For Emit, concatenate the preorder walks of each field expression in source field
order into one sequence; do not number field wrappers or literal children.
Parentheses add no node or index. Resolution must name the unique declaration.

Worked example from loan.moriarty: `accrue` is action 0 and its
`let interest_calculated = floor_div(interest_numerator, interest_denominator);`
is statement 5. The instruction ID is `a_0_s_5`. Its exact expression IDs are:

| Expression | nodeId |
|---|---|
| FloorDiv root | `a_0_s_5_e_0` |
| LocalRef interest_numerator | `a_0_s_5_e_1` |
| LocalRef interest_denominator | `a_0_s_5_e_2` |

The policy rounding node for that let is `a_0_s_5_e_0`. Statement 6 restarts
at `a_0_s_6_e_0`; JSON key sorting never changes this traversal order.

## Core, policies, schemas, and semantic manifest

Core expressions are exactly these closed variants. `CoreExpression` denotes
their union; Core evaluation follows the recorded tree order, not a reassociated
form.

```text
{nodeId:Identifier, sourceRef:SourceRef, tag:"Literal", type:LocalType,
 unitVector:UnitVector, value:LocalValue}
{nodeId:Identifier, sourceRef:SourceRef, tag:"Remaining", type:UIntType,
 unitVector:UnitVector}
{declaration:Identifier, nodeId:Identifier, sourceRef:SourceRef,
 tag:"StateRef"|"ArgRef"|"ObservationRef"|"ConstRef", type:StoredType,
 unitVector:UnitVector}
{action:Identifier, local:Identifier, nodeId:Identifier, sourceRef:SourceRef,
 tag:"LocalRef", type:LocalType, unitVector:UnitVector}
{denominator:CoreExpression, nodeId:Identifier, numerator:CoreExpression,
 sourceRef:SourceRef, tag:"FloorDiv", type:LocalType, unitVector:UnitVector}
{left:CoreExpression, nodeId:Identifier, right:CoreExpression,
 sourceRef:SourceRef, tag:"Add"|"Sub"|"Mul"|"Eq"|"Lt"|"Lte"|"Gt"|"Gte"|"And"|"Or",
 type:LocalType, unitVector:UnitVector}
{nodeId:Identifier, operand:CoreExpression, sourceRef:SourceRef, tag:"Not",
 type:BoolType, unitVector:UnitVector}
```

```text
CoreGuard = {condition:CoreExpression, message:TextValue, sourceRef:SourceRef,
             statementId:Identifier, tag:"Guard"}
CoreLet   = {expression:CoreExpression, name:Identifier, sourceRef:SourceRef,
             statementId:Identifier, tag:"Let", type:LocalType, unitVector:UnitVector}
PolicyUse = {tag:"NonFinancial"} | {name:Identifier, tag:"Financial"}
CoreSet   = {expression:CoreExpression, field:Identifier, policy:PolicyUse,
             sourceRef:SourceRef, statementId:Identifier, tag:"Set"}
CoreEffectPolicy = {field:FieldLabel, policy:Identifier}
CoreEmit  = {fields:[{expression:CoreExpression,label:FieldLabel}], kind:EffectKind,
             ordinal:UInt128Text, policies:[CoreEffectPolicy], sourceRef:SourceRef,
             statementId:Identifier, tag:"Emit"}
CoreInstruction = CoreGuard | CoreLet | CoreSet | CoreEmit
CoreAction = {actorParameter:"actor", arguments:[NamedType], instructions:[CoreInstruction],
              name:Identifier, resourceCounts:ActionResourceCounts, sourceRef:SourceRef}
ActionResourceCounts = {effects:UInt128Text, expressionDepth:UInt128Text,
                        expressionNodes:UInt128Text, instructions:UInt128Text,
                        locals:UInt128Text}
CoreProgram = {actions:[CoreAction], coreVersion:"moriarty-core/1",
               schemaVersion:"moriarty-core-program/1"}
```

Statement IDs are `a_<action-index>_s_<statement-index>`. Emit ordinal is its
zero-based position among emits in that action. Each action must have exactly one
`actor: Text` parameter; it is the `actorParameter`. This makes the authenticated
principal-to-actor check mandatory rather than a convention inferred from guards.

Each declared effect kind occurs exactly once and must use its standard schema:

```text
Transfer   {asset:Text, from:Text, to:Text, amount:AmountFromOperand}
Fee        {asset:Text, from:Text, to:Text, amount:AmountFromOperand}
DueCreated {due_id:Text, debtor:Text, creditor:Text, denomination:Text, amount:AmountFromOperand}
DueSettled {due_id:Text, debtor:Text, creditor:Text, denomination:Text,
            amount:AmountFromOperand, asset:Text}
```

Field order above is semantic declaration order. Missing, reordered, duplicated,
or additional fields reject. Every emitted kind must be declared. An unused kind
may be omitted. The manifest uses the fixed schema for each declared kind.

For Amount writes, `PolicyUse` is Financial and names the unique checked policy.
For UInt128 or Text writes it is exactly NonFinancial. A financial target cannot
be assigned to a nonfinancial write. CoreEmit policy entries follow financial
field declaration order and contain exactly one entry per Amount field.

```text
SettlementBinding = {asset:TextValue, name:Identifier, quantum:AmountValue, unit:Identifier}
WritePolicyTarget  = {action:Identifier, field:Identifier, tag:"Write"}
EffectPolicyTarget = {action:Identifier, field:FieldLabel, ordinal:UInt128Text, tag:"Effect"}
RoundingNode       = {tag:"None"} |
                     {action:Identifier, coreNodeId:Identifier, local:Identifier, tag:"Floor"}
FieldPolicy = {comparisonPolicy:TextValue, derivation:TextValue, name:Identifier,
               proofStatement:TextValue, remainderDisposition:TextValue,
               roundingNode:RoundingNode, targets:[WritePolicyTarget|EffectPolicyTarget],
               unit:Identifier}
EpisodeRule = {field:Identifier, literal:StoredValue, tag:"ClosedWhenEqual"}
AgreementRule = {field:Identifier, tag:"RemainingNotional"} |
                {tag:"NoRemainingNotional"}
StatusRules = {agreement:AgreementRule, episode:EpisodeRule}
ReserveRule = {action:Identifier, closure:Identifier}
ClaimRequirement = {claimId:TextValue,
                    kind:"ContractProperty"|"IntentRefinement"|
                         "TransitionValidity"|"PredecessorHistory"}
```

Policy documentary strings are opaque text. Their exact bytes are program-hash
bound but are not parsed, compared with expressions, or treated as a proof or
safety predicate. `proofStatement` is an identifier for a mandatory future proof
claim, not evidence that the claim exists.

The manifest claim requirements are generated deterministically: include
ContractProperty `bounded_profile_safety_v1` and each distinct policy
proofStatement, IntentRefinement `atomic_intent_refinement_v1`,
TransitionValidity `bounded_atomic_transition_v1`, and PredecessorHistory
`bounded_history_compliance_v1`. No caller may add or omit requirements. The
built-ins denote the typed/bounded safety, authority refinement, exact transition,
and predecessor-history predicates defined by this profile. Policy-specific
predicates must be supplied by the profile-bound proof backend; an unknown
policy claim fails closed rather than becoming a vacuous assertion.

`requiredClaims` is sorted by `(kind ASCII, claimId UTF-8)`, has no duplicate
pair, and contains at least one requirement of each of the four kinds. Contract
property requirements include every policy `proofStatement`; the other three
kinds are the mandatory intent/refinement, transition, and predecessor-history
acceptance predicates. A claim list is specification of proof obligations, not
evidence that any proof system implements them.

The exact hashable semantic manifest is:

```text
SemanticManifest = {
  bounds:BoundsRef,
  constants:[NamedStoredValue],
  core:CoreProgram,
  effectSchemas:[{fields:[{label:FieldLabel,type:StoredType|BareAmountEffectType}],kind:EffectKind}],
  horizon:UInt128Text,
  initialState:[NamedStoredValue],
  lifetime:UInt128Text,
  name:Identifier,
  observationSchema:[NamedType],
  policies:[FieldPolicy],
  profile:"moriarty-bounded-atomic/1",
  schemaVersion:"moriarty-semantic-manifest/1",
  settlementBindings:[SettlementBinding],
  sourceHash:Digest,
  stateSchema:[NamedType],
  statusRules:StatusRules,
  requiredClaims:[ClaimRequirement],
  reserveRules:[ReserveRule],
  units:[Identifier]
}
BoundProgram = {manifest:SemanticManifest, programHash:Digest,
                schemaVersion:"moriarty-program/1"}
```

Arrays follow their source declaration order, including reserveRules and units.
Policy targets follow their source list; requiredClaims alone uses its specified
(kind,claimId) sort. Reserve metadata resolves over the complete action table. There is exactly one
bounds encoding: `BoundsRef`; the numeric `bounds.json` object is never embedded.
The full `BoundProgram`, including Core expressions, is checked against the
program-manifest limits in `bounds.json`. The effective joint bound is the actual
aggregate byte/depth/node/count result; the per-action maxima do not promise that
64 maximum actions fit. The compact `ProgramRef` is the only program object in a
signing statement and never contains `manifest`, `core`, or AST nodes.

## Hash preimages

The single complete domain registry is `bounds.json.domainRegistry`; the formulas
below apply those registry entries and do not introduce additional domains.
Concatenation below is byte concatenation and `00` is one zero byte:

```text
sourceHash  = SHA256(original source UTF-8 bytes)
boundsHash  = SHA256(UTF8("MORIARTY-BOUNDS-bounded-atomic/1") || 00 || exact bounds.json bytes)
programHash = SHA256(UTF8("MORIARTY-PROGRAM-bounded-atomic/1") || 00 || canonical(SemanticManifest))
claimRoot   = SHA256(UTF8("MORIARTY-CLAIMS-bounded-atomic/1") || 00 || canonical(SemanticManifest.requiredClaims))
genesisHash = SHA256(UTF8("MORIARTY-GENESIS-bounded-atomic/1") || 00 || canonical(GenesisBody))
stateHash   = SHA256(UTF8("MORIARTY-STATE-bounded-atomic/1") || 00 || canonical(StateBody))
actionHash  = SHA256(UTF8("MORIARTY-ACTION-bounded-atomic/1") || 00 || canonical(ActionCall))
observationsHash = SHA256(UTF8("MORIARTY-OBSERVATIONS-bounded-atomic/1") || 00 || canonical(ObservationSet))
statementDigest = SHA256(UTF8(authority.domain) || 00 || canonical(authority.statement))
authorityDigest = SHA256(UTF8("MORIARTY-AUTHORITY-bounded-atomic/1") || 00 || canonical(Authority))
proofContextHash = SHA256(UTF8("MORIARTY-PROOF-CONTEXT-bounded-atomic/1") || 00 || canonical(ProofContext))
traceHash   = SHA256(UTF8("MORIARTY-TRACE-bounded-atomic/1") || 00 || canonical(CompleteBody))
proofDigest = SHA256(resolved raw proof bytes)
```

None of the preimages contains the digest it defines. `BoundProgram`, genesis,
state, and Complete wrappers carry the separately computed digest. A verifier
recomputes it; omission-by-convention is not used.

## Genesis, state, observations, and action

```text
ExecutionDomain = {deployment:TextValue, network:TextValue}
PrincipalBinding   = {actor:TextValue, principal:TextValue}
ObservationBinding = {authenticationPolicy:TextValue, name:Identifier,
                      provider:TextValue}
GenesisBody = {bounds:BoundsRef, domain:ExecutionDomain, horizon:UInt128Text, initialState:[NamedStoredValue],
               instanceId:TextValue, lifetime:UInt128Text,
               observationBindings:[ObservationBinding], principalBindings:[PrincipalBinding],
               profile:"moriarty-bounded-atomic/1", program:ProgramRef,
               requiredClaimRoot:Digest, schemaVersion:"moriarty-genesis-body/1"}
Genesis = {body:GenesisBody, genesisHash:Digest, schemaVersion:"moriarty-genesis/1"}

ObligationRecord = {amount:AmountValue, creditor:TextValue, debtor:TextValue,
                    denomination:TextValue, dueId:TextValue,
                    status:"Outstanding"|"Settled"}
EpisodeStatus = "Open" | "Closed"
AgreementStatus = "Outstanding" | "NoOutstanding"
RemainingNotional = {amount:AmountValue, tag:"Amount"} | {tag:"NotApplicable"}
StateBody = {agreementStatus:AgreementStatus, episodeStatus:EpisodeStatus,
             genesisHash:Digest, instanceId:TextValue, obligations:[ObligationRecord],
             profile:"moriarty-bounded-atomic/1", programHash:Digest,
             remaining:UInt128Text, remainingNotional:RemainingNotional,
             revision:UInt128Text, schemaVersion:"moriarty-state-body/1",
             values:[NamedStoredValue]}
StateEnvelope = {body:StateBody, schemaVersion:"moriarty-state/1", stateHash:Digest}

ObservationValue = {evidenceDigest:Digest, name:Identifier, provider:TextValue,
                    value:StoredValue}
ObservationSet = {observations:[ObservationValue], schemaVersion:"moriarty-observations/1"}
ActionCall = {arguments:[NamedStoredValue], name:Identifier,
              schemaVersion:"moriarty-action/1"}
```

ObservationValue.evidenceDigest is an opaque external digest. The profile defines
no preimage or algorithm for it; the genesis authenticationPolicy and trusted
wrapper must validate it against the exact provider/name/value. Signature.algorithm
is likewise an opaque external algorithm identifier, restricted by the deployment's
trusted wrapper; an unsupported algorithm fails signature verification. Neither
field permits an unvalidated client assertion.

Principal bindings are unique by both principal and actor. Observation bindings
are unique by name and cover the manifest schema exactly; `now:UInt128` is
mandatory. The wrapper authenticates the principal, provider, evidence, claim
root, and genesis before evaluation. These schemas bind those external checks;
they do not implement cryptography, oracle truth, ledger ordering, or custody.

## Authority and compact signing objects

The new profile does not alter or reinterpret historical domains
`MORIARTY-SIGN-v1` and `MORIARTY-OUTCOME-SIGN-v1`. Historical statements are not
valid instances of these new schemas.

```text
Validity = {notBefore:UInt128Text, notAfterExclusive:UInt128Text}
Signature = {algorithm:TextValue, bytes:OpaqueBytes, keyId:TextValue}
ExactEffect = {effect:EffectRecord}
ExactWrite  = {field:Identifier, value:StoredValue}
DebitCap    = {actor:TextValue, asset:TextValue, maximumLedgerAmount:UInt128Text}
NetGoal     = {actor:TextValue, asset:TextValue, minimumLedgerAmount:UInt128Text}
CallPermission = {callee:TextValue, selector:TextValue}

ClaimEvidenceRef = {claimId:TextValue,
                    kind:"ContractProperty"|"IntentRefinement"|
                         "TransitionValidity"|"PredecessorHistory",
                    proofDigest:Digest, publicInputDigest:Digest}
ExactPlanStatement = {action:ActionCall, exactEffects:[ExactEffect], exactWrites:[ExactWrite],
                      beforeStateHash:Digest, domain:ExecutionDomain, genesisHash:Digest,
                      instanceId:TextValue, mode:"ExactPlan", nonce:TextValue, predecessors:[Digest],
                      principal:TextValue, program:ProgramRef, requiredClaimRoot:Digest, requiredClaims:[ClaimRequirement],
                      schemaVersion:"moriarty-exact-plan/1", validity:Validity}
OutcomeStatement = {allowedActions:[Identifier], grossDebitCaps:[DebitCap],
                    beforeStateHash:Digest, domain:ExecutionDomain, genesisHash:Digest,
                    instanceId:TextValue, minimumNetCredits:[NetGoal], mode:"IntentRefinement",
                    nonce:TextValue, permittedCalls:[CallPermission],
                    permittedRecipients:[TextValue], predecessors:[Digest],
                    principal:TextValue, program:ProgramRef, requiredClaimRoot:Digest, requiredClaims:[ClaimRequirement],
                    schemaVersion:"moriarty-outcome-intent/1", validity:Validity}
ExactPlanAuthority = {domain:"MORIARTY-SIGN-bounded-atomic/1",
                      schemaVersion:"moriarty-authority/1", signature:Signature,
                      statement:ExactPlanStatement, tag:"ExactPlan"}
IntentRefinementAuthority = {domain:"MORIARTY-OUTCOME-bounded-atomic/1",
                             schemaVersion:"moriarty-authority/1", signature:Signature,
                             statement:OutcomeStatement, tag:"IntentRefinement"}
Authority = ExactPlanAuthority | IntentRefinementAuthority
```

Signature bytes attest exactly
`UTF8(authority.domain) || 00 || canonical(authority.statement)`. The statement's
compact `ProgramRef`, principal, nonce, interval, predecessors, and claim requirements
are therefore signed. ExactPlan compares canonical `exactWrites` against the projection of produced
writes to `{field,value}`, in statement order, and compares each `exactEffects[i].effect` with the complete enriched
`effects[i]` in emit order, with identical array lengths. IntentRefinement requires the selected action be allowed,
all Transfer/Fee recipients and any calls be permitted, every outgoing principal Transfer/Fee have a matching cap and gross principal
debits not exceed it without netting refunds. Caps and goals may name only the
principal-bound actor. Due effects do not enter ledger debit/credit sums.
Each net goal requires checked credits >= checked(debits + minimumLedgerAmount),
with overflow rejecting, in ledger units after exact quantum conversion. This language has no call instruction, so
`permittedCalls` must be empty. Array members use manifest/effect order; caps and
goals sort by `(actor UTF-8, asset UTF-8)` and have no duplicate pair.

```text
ExternalChecks = {authenticatedPrincipal:TextValue, genesisValid:boolean,
                  nonceFresh:boolean, observationsAuthentic:boolean,
                  predecessorSetValid:boolean, signatureValid:boolean,
                  stateCurrentAndUnconsumed:boolean}
EvaluationInput = {action:ActionCall, authority:Authority, checks:ExternalChecks,
                   genesis:Genesis, observations:ObservationSet,
                   program:ProgramRef, schemaVersion:"moriarty-evaluation/1",
                   state:StateEnvelope}
```

`authenticatedPrincipal` must equal the signed principal; exactly one genesis
binding for it must exist; its actor must equal the action's mandatory `actor`
argument. Every boolean precheck must be true. The trusted wrapper constructs
these checks for the exact program, genesis, state, action, signed Authority and
ObservationSet supplied in this EvaluationInput. It authenticates these concrete
objects before derivation; there is no dependency on a future trace or proof.
The checks cannot be copied from an unauthenticated client or reused with changed
objects. EvaluationInput carries neither claim evidence nor proof verdicts.
Missing cryptographic/oracle/current-state precheck implementations fail closed.
Post-trace proof verification and final durable consumption are separate below.

## Settlement and result records

```text
SettlementResolution = {asset:TextValue, binding:Identifier,
                        ledgerAmount:UInt128Text, nominalAmount:AmountValue,
                        quantum:AmountValue, unit:Identifier}
TransferEffect = {amount:AmountValue, asset:TextValue, from:TextValue,
                  kind:"Transfer", ordinal:UInt128Text,
                  settlement:SettlementResolution, to:TextValue}
FeeEffect      = {amount:AmountValue, asset:TextValue, from:TextValue,
                  kind:"Fee", ordinal:UInt128Text,
                  settlement:SettlementResolution, to:TextValue}
DueCreatedEffect = {amount:AmountValue, creditor:TextValue, debtor:TextValue,
                    denomination:TextValue, dueId:TextValue,
                    kind:"DueCreated", ordinal:UInt128Text}
DueSettledEffect = {amount:AmountValue, asset:TextValue, creditor:TextValue,
                    debtor:TextValue, denomination:TextValue, dueId:TextValue,
                    kind:"DueSettled", ordinal:UInt128Text,
                    settlement:SettlementResolution}
EffectRecord = TransferEffect | FeeEffect | DueCreatedEffect | DueSettledEffect
WriteRecord = {field:Identifier, policy:PolicyUse, value:StoredValue}
ObligationDelta = {created:[ObligationRecord], settled:[ObligationRecord]}
AuthorityConsumption = {mode:"ExactPlan"|"IntentRefinement", nonce:TextValue,
                        principal:TextValue, statementDigest:Digest}
ResourceCounts = {canonicalDepth:UInt128Text, canonicalNodes:UInt128Text,
                  canonicalUtf8Bytes:UInt128Text, effects:UInt128Text,
                  executedInstructions:UInt128Text, expressionNodes:UInt128Text,
                  maximumExpressionDepth:UInt128Text, unitComponents:UInt128Text}
```

The `ordinal` on every result effect is its zero-based ordinal among all emits in
the selected action. A settlement resolution is required for Transfer, Fee, and
DueSettled and repeats the exact checked binding; it is not an optional side map.

```text
DiagnosticCode = one exact code from semantics.md "Closed diagnostics"
Diagnostic = {code:DiagnosticCode, message:TextValue, primarySpan:Span,
              relatedSpans:[Span], stage:UInt128Text}
Rejected = {diagnostics:[Diagnostic], outcome:"Rejected",
            profile:"moriarty-bounded-atomic/1", programHash:Digest,
            schemaVersion:"moriarty-result/1"}
CompleteBody = {actionHash:Digest, after:StateEnvelope, authorityConsumption:AuthorityConsumption,
                beforeStateHash:Digest, effects:[EffectRecord],
                obligationDelta:ObligationDelta, observationsHash:Digest,
                outcome:"Complete", predecessors:[Digest],
                profile:"moriarty-bounded-atomic/1", programHash:Digest,
                resourceCounts:ResourceCounts, schemaVersion:"moriarty-complete-body/1",
                writes:[WriteRecord]}
Complete = {body:CompleteBody, schemaVersion:"moriarty-result/1", traceHash:Digest}
Result = Rejected | Complete
```

Rejected has no next state, writes, effects, or trace. `Pending` is deliberately
not a Result variant; a request requiring it produces Rejected with
`UNSUPPORTED_PENDING`. Complete reports statuses and remaining notional inside
`after.body` and retains every bounded obligation record, including settled
identity tombstones. Canonical result limits apply to the whole wrapper and may
reject a transition even if its individual count limits fit.


## Binding and acyclic proof acceptance

The stage-9 binding equalities in semantics.md are mandatory before execution.
This single-input atomic profile requires predecessors exactly `[beforeStateHash]`,
including the checked genesis state at revision zero. Both signed requiredClaims
and requiredClaimRoot equal the manifest requirements and recomputed claimRoot.
The unsigned candidate CompleteBody.actionHash is recomputed from the exact
EvaluationInput.action for both authority modes; it is never supplied by the
caller as a substitute. This binds even argument changes that leave all effects
and resource counts unchanged.

```text
ProofContext = {actionHash:Digest, authorityDigest:Digest, beforeStateHash:Digest,
                domain:ExecutionDomain, genesisHash:Digest,
                predecessors:[Digest], program:ProgramRef,
                requiredClaimRoot:Digest, schemaVersion:"moriarty-proof-context/1",
                traceHash:Digest}
ResolvedProofRef = {byteLength:UInt128Text, claimId:TextValue,
                    kind:"ContractProperty"|"IntentRefinement"|
                         "TransitionValidity"|"PredecessorHistory",
                    reference:TextValue}
ProofAcceptanceInput = {claimEvidence:[ClaimEvidenceRef], proofContext:ProofContext,
                        proofs:[ResolvedProofRef],
                        schemaVersion:"moriarty-proof-acceptance-input/1"}
ProofAcceptanceVerdict = {historyProofValid:boolean, proofContextHash:Digest,
                          requiredClaimsValid:boolean,
                          schemaVersion:"moriarty-proof-acceptance-verdict/1"}
```

Each ProofContext field equals its locally derived value: actionHash equals both
recomputed input actionHash and CompleteBody.actionHash; authorityDigest hashes
the complete signed Authority; program equals the recomputed ProgramRef; traceHash
hashes the candidate CompleteBody; all remaining fields equal the input genesis,
authority, manifest and before-state bindings. ProofAcceptanceInput.proofContext
must equal that entire locally reconstructed context in canonical bytes.

External resolver contract: a trusted, profile-bound backend receives
ProofAcceptanceInput and a resolver operation `resolve(reference) -> raw bytes | unavailable`.
References are opaque, nonempty TextValue handles within that invocation, never
URLs to fetch implicitly. No proof bytes enter EvaluationInput or signed objects.
`proofs` and `claimEvidence` each contain exactly one entry per manifest requirement,
in requiredClaims order, matching each (kind,claimId), with no omissions or extras.
Each resolved length must equal byteLength. The limits in
bounds.json.externalProofResolution bound count, individual raw bytes and total
raw bytes before verification. The backend resolves each handle once and retains
those immutable bytes for hashing and verification, rejecting unavailable,
changed, oversized or length-mismatched data with PROOF_RESOLUTION.

Each ClaimEvidenceRef.publicInputDigest equals the recomputed proofContextHash;
proofDigest is SHA256 of the corresponding resolved raw bytes. The backend verifies
those bytes against the exact required claim kind/ID and complete ProofContext.
Unknown policy claims fail closed. The trusted verdict is accepted only from that
backend invocation, with its proofContextHash equal to the locally recomputed
hash and both booleans true. requiredClaimsValid covers all requirements including
history, and historyProofValid separately affirms every PredecessorHistory claim.
Caller booleans, a matching digest alone, and unavailable backend implementations
cannot stand in for proof verification.

The numbered protocol is normative:

1. Compile and bind BoundProgram/genesis, choose the exact ActionCall and before
   state, and obtain authenticated observations. An unsigned simulation may
   calculate proposed writes/effects to construct an ExactPlanStatement; an
   OutcomeStatement instead declares allowed outcomes. This simulation consumes
   nothing and produces no accepted Result.
2. Sign the statement bytes using its registered domain, obtaining Authority.
   The wrapper authenticates that Authority and the exact input tuple and builds
   EvaluationInput containing only the prechecks above.
3. `derive(boundProgram:BoundProgram, input:EvaluationInput)` validates stages
   9–12 and computes the provisional CompleteBody, stateHash, actionHash,
   traceHash and ProofContext at stage 13. It returns Rejected or an internal
   pair `(candidate:Complete, context:ProofContext)`. This pair is not an accepted
   Result and is not a separate public wire envelope.
4. The prover receives the exact ProofContext, requiredClaims and relation
   witnesses, constructs each mandatory proof, and supplies the bounded resolver
   handles and ClaimEvidenceRef values in ProofAcceptanceInput. Signature and
   trace preimages contain no proof bytes, proofDigest, or ClaimEvidenceRef.
5. `verifyProofs(input:ProofAcceptanceInput, resolver)` performs the resolution,
   digest, claim and context checks above and returns the trusted closed
   ProofAcceptanceVerdict, or a rejection diagnostic at stage 13.
6. `commit(boundProgram, input, candidate, proofAcceptanceInput, verdict)` checks
   equality to the locally derived candidate/context, a successful trusted
   verdict, and the current durable state/nonce again. Only then may the wrapper
   atomically consume state and nonce and expose Complete. A changed consumed
   state or stale nonce rejects; no candidate state or effect is committed.

The implementing acceptance API is
`evaluate(boundProgram:BoundProgram, input:EvaluationInput, proofAcceptanceBackend)
 -> Rejected | Complete`, where the mandatory backend callback receives the
locally derived ProofContext and requiredClaims, obtains ProofAcceptanceInput
through step 4, and performs step 5. It cannot bypass step 6. This callback
interface makes the two phases explicit without requiring evidence before trace
derivation. MC05 must implement the backend; this source profile supplies no
proof construction or actual verifier. Simulation output is explicitly labeled
simulation and outside accepted Result. The entire sequence remains acyclic.

## Counts, ordering, and malformed inputs

ResourceCounts.canonicalDepth, canonicalNodes and canonicalUtf8Bytes measure
canonical CompleteBody with the entire resourceCounts member omitted. This
preimage is called CountBody. Root depth is zero. Each object, array, scalar value
counts as one node; object key strings do not add nodes. Depth increases by one
for each contained value. UTF-8 bytes include all canonical punctuation and keys.
The actual complete wrapper is independently checked against resultEncoding;
CountBody counts never waive that bound. This avoids self-referential byte counts.
Other counts record actually evaluated expression nodes, instructions and effects;
maximumExpressionDepth uses root one; unitComponents is the largest evaluated
unit vector. Static ActionResourceCounts include all syntax, even short-circuited
branches. A TypedProgram uses typedProgramEncoding jointly with its embedded AST
bounds. SourceAST.declarations has an effective maximum of 128 total entries,
even when all per-category maxima fit. TypedProgram.annotations contains exactly
one annotation per source expression.

Named values, arguments, writes and emitted fields follow their declaration or
statement order as applicable. Obligations retain creation order including settled
tombstones. ObligationDelta.created and .settled follow emit order. Principal
bindings sort by principal UTF-8 bytes, observation bindings/values follow the
manifest observation order, and allowedActions follow manifest action order.
Recipient lists sort by UTF-8 bytes. Duplicate members reject. The signed interval
requires notBefore <= now < notAfterExclusive <= genesis.horizon.

Parsing/checking returns diagnostics directly before any valid program exists.
Evaluation Rejected uses the verified program hash if available, otherwise the
64-zero digest as an explicit unavailable-program sentinel. A Rejected object
is never a program identity or acceptance certificate. Unknown source spans use
`{startByte:"0",endByte:"0"}`. Diagnostics use stages 1..13 from semantics.md.

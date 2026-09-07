# Typed schema tables for `moriarty-bounded-atomic/1`

Status: S2, specified-only. Fields are closed unless marked as maps or arrays. `UInt128Text` is a canonical decimal string in `[0,2^128-1]`; JSON numeric tokens are forbidden. `Span` is `{startByte: UInt128Text, endByte: UInt128Text}` with `startByte <= endByte <= sourceUtf8Bytes`.

## Source AST envelope

| Field | Type | Required rule |
|---|---|---|
| `schemaVersion` | literal `moriarty-ast/1` | semantic-versioned |
| `profile` | literal `moriarty-bounded-atomic/1` | exact match |
| `sourceHash` | 32-byte digest as lowercase hex text | hash of original UTF-8 bytes |
| `sourceUtf8Bytes` | `UInt128Text` | at most 65,536 |
| `span` | `Span` | whole agreement |
| `name` | `Identifier` | 1..64 ASCII characters |
| `lifetime`, `horizon` | `UInt128Text` with spans | lifetime positive; horizon exclusive |
| `declarations` | ordered array of tagged declarations | source order, at most profile aggregate bounds |

Declaration tags are `UnitDecl`, `ConstDecl`, `StateDecl`, `ObservationDecl`, `SettlementDecl`, `FieldPolicyDecl`, `EffectDecl`, and `ActionDecl`. Every node has `tag` and `span`. Literal nodes are distinct `UIntLiteral{token,value}`, `TextLiteral{token,decoded}`, `AmountLiteral{token,value,unit}`, and `BoolLiteral{value}`; both token and value are retained, so numeric-looking text and lexical spelling are lossless. Expression tags are `Literal`, `Remaining`, `StateRef`, `ArgRef`, `ObservationRef`, `ConstRef`, `LocalRef`, `FloorDiv`, `Add`, `Sub`, `Mul`, `Eq`, `Lt`, `Lte`, `Gt`, `Gte`, `And`, `Or`, and `Not`. `FloorDiv` has explicit `numerator` and `denominator`. Statements are `Guard`, `Let`, `Set`, and `Emit`, each with its source span. Typed nodes add `type`, canonical unit vector, and resolution target without deleting source tokens/spans.

## Bound semantic manifest

| Field | Type | Binding |
|---|---|---|
| `schemaVersion` | `moriarty-manifest/1` | canonical schema |
| `profile`, `coreVersion` | text literals | semantic and lowering versions |
| `programHash`, `sourceHash` | lowercase digest text | canonical program and source bytes |
| `instanceGenesis` | text | agreement instance domain |
| `lifetime`, `horizon` | `UInt128Text` | frozen genesis values |
| `unitMap` | identifier -> canonical exponent vector | checked elaboration result |
| `constValues`, `initialState` | closed typed maps | frozen values and initial dynamic values |
| `stateSchema`, `observationSchema` | closed typed maps | state/observation types |
| `settlementBindings` | closed records `{unit,asset,quantum}` | asset mapping; no custody assertion |
| `effectSchemas` | effect kind -> closed field map | exactly declared fields |
| `fieldPolicies` | field -> `{unit,derivation,roundingNode,remainderDisposition,comparisonPolicy,proofStatement}` | mandatory elaboration and proof binding |
| `actions` | identifier -> `{argSchema,instructions,sourceMap,resourceCounts}` | deterministic Core plus origin spans |
| `bounds` | complete `moriarty-bounds/1` object or digest plus registry ID | simultaneous limits |
| `canonicalization` | `{domain,keyOrder,stringEncoding,integerEncoding}` | exact signing codec |

Canonical signing uses domain `MORIARTY-SIGN-bounded-atomic/1`, lexicographic ASCII keys, UTF-8, canonical JSON escaping, no whitespace, no duplicate or unknown keys, and decimal strings for integers. The actual public envelope limits are 65,536 UTF-8 bytes, depth 16 with root at zero, 8,192 nodes, 64 keys per record, arrays of at most 128, text at most 4,096 UTF-8 bytes and 4,096 JavaScript code units. It binds `programHash`; it does not embed a depth-40 AST.

## Evaluation input

| Field | Type | Rule |
|---|---|---|
| `schemaVersion`, `profile`, `programHash` | fixed/versioned text | exact manifest binding |
| `state` | `{instance,revision,remaining,values,obligations,episodeStatus,agreementStatus}` | closed schema; integers are `UInt128Text` |
| `action` | `{name,args}` | declared action and exact argument schema |
| `authority` | tagged `ExactPlan` or `IntentRefinement` | binds domain, principal, key, nonce, validity interval, predecessors, program/profile/instance, action or outcomes, permitted effects/calls, gross caps including fees, net goals, required claims, signature |
| `observations` | closed typed map including `now` | declared schema; externally authenticated |

The profile does not define counterparty authentication, oracle truth, nonce durability, signature implementation, or ledger consumption; wrappers must supply and bind those predicates. Missing required authority or observations fail closed.

## Result envelope

| Variant | Required fields | Meaning |
|---|---|---|
| `Rejected` | `{schemaVersion,profile,programHash,outcome:"Rejected",diagnostics:[Diagnostic]}` | no committed next state or effects |
| `Complete` | `{schemaVersion,profile,programHash,outcome:"Complete",beforeHash,after,effects,writes,obligations,episodeStatus,agreementStatus,authorityConsumption,observationsHash,predecessors,resourceCounts,traceHash}` | selected atomic plan completed; future obligations may remain |
| `Pending` | `{...,outcome:"Pending",progress,residualAuthority,residualObligations,residualLifetime,residualHorizon}` | reserved schema only; this profile always rejects before producing it |

`Diagnostic` is `{stage,code,message,primarySpan,relatedSpans}`. Spans are UTF-8 byte offsets. `effects` uses closed declared schemas. `obligations` retains separate created/settled/outstanding identities; it is not inferred from transfers. `resourceCounts` includes executed instructions, expression nodes/depth, emits, unit components, and canonical byte/node/depth counts. All result fields except `traceHash` are included in canonical trace hashing.

Hash preimages are explicit and noncircular. `programHash` is SHA-256 over UTF-8 `MORIARTY-PROGRAM-bounded-atomic/1`, one zero byte, and the canonical bound-manifest object with only its `programHash` member omitted. `sourceHash` is SHA-256 over the original source UTF-8 bytes. `traceHash` is SHA-256 over UTF-8 `MORIARTY-TRACE-bounded-atomic/1`, one zero byte, and the canonical result object with only its `traceHash` member omitted. Verifiers receive the complete preimage or an independently checked binding to it. No unhashed optional unit or policy map may substitute. Effect record labels may use keyword spellings such as `asset` and `amount`; declaration identifiers remain keyword-restricted.

Static inventory of frozen Core/4 at `81bed862c60b184e7f5fdb27577157c2bcf39530` against the retained expression K source. No live author files or evaluators were used. Requested reviewer: GPT-6 Astra medium; no separate provider-returned identity is exposed. Source pins and per-constructor static/dynamic line references are in the companion JSON.

Core/4 has 60 constructors. Retained K admits 40; 20 are missing. The frozen Task3 source requires 25 constructors by source/lowering inspection. These counts describe source rules, not runtime conformance.

| Constructor | Category | Task3 | Retained K |
|---|---|---|---|
| ConstructShares | construction/projection/conversion | no | missing |
| ConstructAmount | construction/projection/conversion | yes | missing |
| ConstructVariant | construction/projection/conversion | no | missing |
| ProjectVariant | construction/projection/conversion | no | missing |
| ProjectSome | construction/projection/conversion | no | missing |
| ConvertUInt | construction/projection/conversion | no | missing |
| ScalarValue | construction/projection/conversion | yes | missing |
| Select | boolean/comparison/control | no | missing |
| LitUInt | literal | yes | present; reconcile domains/staging |
| LitSInt | literal | no | present; reconcile domains/staging |
| LitBool | literal | yes | present; reconcile domains/staging |
| LitText | literal | yes | present; reconcile domains/staging |
| LitAmount | literal | no | present; reconcile domains/staging |
| LitQuantity | literal | yes | present; reconcile domains/staging |
| LitShares | literal | no | present; reconcile domains/staging |
| LitRate | literal | no | present; reconcile domains/staging |
| LitPrice | literal | no | present; reconcile domains/staging |
| ReadLocal | ordinary-read | yes | present; reconcile domains/staging |
| ReadPre | ordinary-read | yes | present; reconcile domains/staging |
| ReadArg | ordinary-read | yes | present; reconcile domains/staging |
| ReadObs | ordinary-read | no | present; reconcile domains/staging |
| ProjectField | construction/projection/conversion | no | present; reconcile domains/staging |
| AccessField | construction/projection/conversion | no | present; reconcile domains/staging |
| ProjectIndex | construction/projection/conversion | no | present; reconcile domains/staging |
| AccessIndex | construction/projection/conversion | no | present; reconcile domains/staging |
| ConstructRecord | construction/projection/conversion | yes | present; reconcile domains/staging |
| ConstructEnum | construction/projection/conversion | no | present; reconcile domains/staging |
| ConstructSome | construction/projection/conversion | no | present; reconcile domains/staging |
| ConstructNone | construction/projection/conversion | no | present; reconcile domains/staging |
| ConstructCollection | construction/projection/conversion | no | present; reconcile domains/staging |
| Add | arithmetic | yes | present; reconcile domains/staging |
| Sub | arithmetic | no | present; reconcile domains/staging |
| Mul | arithmetic | no | present; reconcile domains/staging |
| FloorDiv | arithmetic | no | present; reconcile domains/staging |
| CeilDiv | arithmetic | no | present; reconcile domains/staging |
| Eq | boolean/comparison/control | yes | present; reconcile domains/staging |
| Lt | boolean/comparison/control | no | present; reconcile domains/staging |
| Lte | boolean/comparison/control | no | present; reconcile domains/staging |
| Gt | boolean/comparison/control | yes | present; reconcile domains/staging |
| Gte | boolean/comparison/control | no | present; reconcile domains/staging |
| Not | boolean/comparison/control | no | present; reconcile domains/staging |
| And | boolean/comparison/control | no | present; reconcile domains/staging |
| Or | boolean/comparison/control | no | present; reconcile domains/staging |
| Require | statement | yes | present; reconcile domains/staging |
| Let | statement | yes | present; reconcile domains/staging |
| NextWrite | statement | yes | present; reconcile domains/staging |
| Ensure | statement | yes | present; reconcile domains/staging |
| Emit | statement | yes | present; reconcile domains/staging |
| ReadOutstanding | financial-pre-read | yes | missing |
| ReadPrincipal | financial-pre-read | no | missing |
| ReadAccrued | financial-pre-read | no | missing |
| ReadBalance | financial-pre-read | no | missing |
| ReadAllowanceRemaining | financial-pre-read | no | missing |
| ReadAllowanceSpent | financial-pre-read | no | missing |
| ReadPostOutstanding | financial-post-read | yes | missing |
| ReadPostPrincipal | financial-post-read | yes | missing |
| ReadPostAccrued | financial-post-read | yes | missing |
| ReadPostBalance | financial-post-read | yes | missing |
| ReadPostAllowanceRemaining | financial-post-read | yes | missing |
| ReadPostAllowanceSpent | financial-post-read | yes | missing |

The eight missing general constructors are ConstructShares, ConstructAmount, ConstructVariant, ProjectVariant, ProjectSome, ConvertUInt, ScalarValue and Select. All twelve PRE/POST financial reads are missing. Task3 itself uses PRE ReadOutstanding and all six POST reads; other PRE reads still require explicit coverage treatment. Amount syntax lowers to ConstructAmount, including literal-looking amount calls; it does not make LitAmount a Task3 requirement. ScalarValue requires both negative and magnitude components.

- closed keys: Core/4 requires variantTypes, including when empty; retained K closes ten keys and therefore rejects every ordinary Core/4 schema before evaluating a constructor.

- declaration cap: Core counts recordTypes+enumTypes+variantTypes only, <=256; K exDeclarations(exSchemaKeys()) counts all rosters/maps. Preserve old profile admission and add financial profile counting.

- variant declarations and cycles: Core admits 1..128 named distinct payload types, resolves variant references and detects R/V/O cycles. Retained K has no variant map/domain/cycle handling.

- bounds and identifiers: Preserve schema65536 bytes, nodes4096/depth64, records64 fields, enums128, sorted unique rosters. Review financial binder namespace now including variants. Type metadata in Core is not subject to schema/value-depth caps.

- arithmetic domains: Cross-Amount multiplication uses UInt256 AmountProduct; Price and Rate produce ScaledAmount/SignedScaledAmount; division requires actual LitUInt128 exact10^scale and produces Amount/SignedAmount. SignedAmount/SignedScaledAmount signed256; NetAmount symmetric UInt128 magnitude. Kernel monetary product overflow remains UInt128 before division, a distinct rule.

- funded boundary: Need full financial context admission, work equality, private ordered kernel, POST binding, suffix and complete FundedExpressionPrepared output. Retained expression rule only publishes post/descriptors/work.

Missing type families: AmountProduct, NetAmount, ScaledAmount, SignedAmount, SignedScaledAmount, UInt256, Variant. Existing generic constructors such as ConstructSome/ConstructCollection and comparisons can therefore have unsupported payload/domain cases even when their constructor rule exists.

The38 independent cases additionally mandate ConstructNone with Option metadata depth5947/Core65536 accepted and depth5948/Core65547 INPUT_BOUND. Dead-branch checking must have an explicit fixture; the prose does not uniquely select And, Or or Select, so this inventory does not invent a frozen constructor choice. Kernel overflow tests exercise protected emitted actions; they do not automatically demonstrate Core Mul or division coverage. Source-only invalid/unselected controls stay outside K invocation counts.

Missing context returns FINANCIAL_CONTEXT_REQUIRED with synthetic span, empty nodePath and workUsed0. WORK_MISMATCH is the bare status/code union. Root’s retained probes02 report13 actual checks; this reviewer inspected that evidence without rerunning it.

No test, K invocation, compile, provider call or resource/accounting reconstruction occurred. The companion JSON records every constructor, type family, required-case role and actual static/dynamic source references. Required lifecycle38cases and metadata5947 remain acceptance obligations; other admitted but unsupported Core cases remain explicit open coverage.

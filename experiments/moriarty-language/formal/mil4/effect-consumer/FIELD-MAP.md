# W-D2F observed field map and missing bindings

Read-only repository observations, 2026-09-30. This is a design sprint, with no
consumer implementation, signature verifier, authenticated registry, or ledger
acceptance. Every proposed check below is specified-only unless explicitly
identified as an existing module behavior. No normative W-D2 choice is made.

## Observed interfaces

The existing [wire schema](../wire/SPEC.md) signs fields 1–35. Its decoder checks
canonical byte shape, not ownership, selected program, state or effects.
[Source/6 AST](../../../src/successor/financial-agreement-source-v6-frontend.ts:119)
preserves additional presentation claims. Its
[lowerer](../../../src/successor/financial-agreement-source-v6-frontend.ts:324)
constructs the existing Core intent/state, and deliberately does not compute an
authorization digest. [Core/5](../../../src/successor/mil4-s0-core-v5.ts:131)
derives effects and post-cells internally before comparing supplied effects,
but accepts caller state without authentication. Its optional local stipulation
is a matching caller tuple, not evidence of an external premise.
The [Source wrapper](../../../src/successor/mil4-s0-source-v6.ts:13) names four
unverified bindings: agreement ID, selected program, scale and predecessor.
[W-D2E](../effect-wire/SPEC.md) adds a supplied effect-image encoder and field 26
equality check, but does not authenticate or semantically derive that image.

`AST.programId` is the identifier after `agreement`; `AST.selected.actionId`
is the selected action. `Core.intent.programId` equals the latter. They are
different nominal roles despite the overloaded property name. The existing
W-D2E fixture uses `agreementId=Agreement`, `actionId=Action`, and
`coreProgramId=TransferLiteralFee` or `RepayAccrualFirst`. It contains no
Source/6 document or source-to-wire binding. It must not be promoted to a
positive full-consumer fixture by treating `actionId` as ignorable.

There are two distinct layers. Literal `wire.agreementId == AST.programId`
is available as an inspection comparison; B01 authenticated agreement-instance
binding remains unavailable. Literal `wire.actionId == AST.selected.actionId`
is also available; B04's authenticated action/Core-program relationship remains
unselected. Comparing these strings does not require a registry or prove one.
Scale, predecessor, claimed sourceHash and claimed policyHash similarly permit
literal comparisons at the AST boundary while their meanings/authentication
remain unverified. Core's loss of a field does not erase the AST comparison.

## Complete signed field map

“Direct” means an existing scalar/constructor can be compared for literal
equality. It does not mean that scalar has been authenticated. “Absent” means
the downstream type has no slot. “Unselected” means the correspondence or hash
image has not been adopted and must not be supplied by a guessed constant.

| Tag | Signed field | Source/6 AST / lowering | Core/5 | W-D2E image | Missing binding / proposed behavior |
| --- | --- | --- | --- | --- | --- |
| header | schemaVersion=/3 | Source profile is /6; lowerer chooses /3 discriminator | intent.version=/3 | independent effects/1 header | V-S0-01 relates this fixed /3 header to Source/6, Core/5 and effects/1; no raw cross-vocabulary equality or /4 fallback |
| 1 | profile=s0-provisional/1 | /6 closed S0 grammar | Core/5 S0 selection | operationKind transfer/repayment | Selected local V-S0-01 tuple, checked at M/tag1; wire.profile != AST.profile is expected vocabulary difference |
| 2 | domain | ast.domain; state.domain; intent.domain | checks state/intent equality | root.domain and replay components | Direct equality; domain membership and ledger network mapping unverified |
| 3 | agreementId | ast.programId retained | absent | absent | Direct literal agreementId==AST.programId comparison available; B01 authenticated agreement-instance binding unavailable; never map to Core.intent.programId |
| 4 | stageId | absent | absent | absent | B02: authenticated stage identity and uniqueness unavailable |
| 5 | episodeId | absent | absent | absent | B03: authenticated episode/history identity unavailable |
| 6 | actionId | Direct equality with ast.selected.actionId available; distinct wire/coreProgramId relation unselected | programId contains Source selected action | absent | Direct literal action comparison available; B04 authenticated selected action/program relation unselected; W-D2E Action differs from Source's supported selected IDs |
| 7 | sourceVersion=6 | exact profile /6 | sourceProfile=/6 | absent | V-S0-01 explicitly relates integer6 to Source profile /6; no hash/lowering authenticity follows |
| 8 | sourceHash | ast.selected.sourceHash, opaque string; copied to intent | only validates opaque string | absent | Direct literal claim equality available; B05 canonical source hash image and source-to-selection verification undefined |
| 9 | coreVersion=5 | lowerer chooses Core/5 | state.core, intent.core | core=/5 | V-S0-01 relates integer5 to Core /5; B06 additionally required for exact selected code |
| 10 | coreProgramId | Literal coreProgramId==selected.actionId comparison available | programId accepts only TransferLiteralFee / RepayAccrualFirst for corresponding kind | operationKind only | Equal literal still leaves authenticated B04 action/program and B06 exact-code correspondence unavailable |
| 11 | coreHash | absent | absent | absent | B06: canonical Core program bytes/hash and selected program verifier undefined |
| 12 | policyHash | ast.selected.policyDigest, opaque; copied to intent.policyDigest | only opaque validation | absent | Direct literal claim equality available; B07 exact policy image and authenticated digest relationship undefined |
| 13 | signer | ast.intent.signer; Source owner/payer constraint | signer to debited owner/debtor | Debit account, allowance owner, replay signer | Direct equality; B08: nominal signer/key ownership and key rotation unavailable |
| 14 | keyScheme=schnorr_bip340 | no scheme field | no scheme field | absent | Exact wire literal is schnorr_bip340; B09 preprocessing/verifier remains unselected |
| 15 | signerKey | ast.intent.keyRef opaque string; copied to intent.keyRef | only opaque validation | absent | B08/B09: keyRef to exact x-only key mapping, curve validity, and authorized ownership unavailable |
| 16 | asset | ast.settlement.asset; state/intent asset | obligation asset equality; Debit/Credit asset | root.asset; obligation footprint asset; effect assets | Direct equality; B10 anchors here at asset/tag16, before scale/tag17; asset identity/unit registration and settlement binding unavailable |
| 17 | scale | ast.settlement.scale decimal string 0..18, retained only | absent | root.scale integer 0..38 | Direct integer equality available in 0..18 intersection; B10 meaning/authentication unverified; never rescale |
| 18 | preHead | ast.intent.preHead and ast.authenticated.head; lowerer state.head/intent.preHead | checks equality at history | root.preHead and AdvanceHead predecessor | Direct equality; B11: snapshot-to-this-head authentication unavailable |
| 19 | predecessor | ast.authenticated.predecessor retained; direct literal equality available | dropped; no predecessor slot | absent; AdvanceHead.predecessor is current preHead | Direct AST claim equality available; B12 actual authenticated prior-head relation unavailable; never equate field19 with AdvanceHead.predecessor |
| 20 | nonce | ast.intent.nonce opaque; lowerer composite replay tuple | replay key=(domain, signer, nonce) | typed replay tuples and histories | Direct equality on exact 64 lowercase hex text; B13: authenticated replay-status fact/unused predicate and durable uniqueness unavailable |
| 21 | validFrom | ast.intent.notBefore uint128 | intent.notBefore, compares state.round | round only | Direct exact integer equality in UInt64 intersection; B11: round authentication unavailable |
| 22 | validUntil | ast.intent.notAfter uint128 | intent.notAfter, closed round interval | round only | Same as validFrom; no clock-time conversion |
| 23 | grossCap | ast.intent.grossCap | intent.grossCap; gross Debit limit | amounts; pre/post allowance | Direct exact equality; B11/B14 authenticate grant/counters; never derive authority from net delta |
| 24 | feeCap | ast.intent.feeCap | intent.feeCap; repay requires zero | fee Credit when positive | Direct exact equality; zero-fee recipient still bound in action and footprint |
| 25 | netFloor | ast.intent.netFloor | transfer value floor; repay requires zero | effect amounts | Direct exact equality; no fee/net reinterpretation |
| 26 | effectCommitment | absent | absent; prepare returns internally derived vector/post | SHA256 of complete image | Rederive Core effects/post, derive complete footprint from same pre/post, then hash; supplied image is never the preparation oracle |
| 27 | failurePolicy=atomic-reject-terminal-success | ast.intent.failure=success_only | TerminalSuccess requested outcome in wrapper | fixed premise requirement; no failure variant | Selected local relation F-S0-01 maps these different literals for closed S0 only; not direct string equality or normative W-D2 adoption |
| 28 | supplyChanges=[] | no supply-change syntax | no mint/burn S0 operation | no supply-change field | Closed S0 supports empty only; existing wire decoder rejects nonempty |
| 29 | observations=[] | ast.intent.observations=empty | dropped; no observation slot | absent | Existing closed syntax rejects nonempty; no external observation claim |
| 30 | disclosures=[] | ast.intent.disclosures=empty | dropped | absent | Existing closed syntax rejects nonempty; no new disclosure mechanism |
| 31 | retainedEffects=[] | ast.intent.retainedEffects=empty | requestedOutcome.retainedEffects empty | absent | Closed terminal success; never fabricate failure effects |
| 32 | retainedDuties=[] | ast.intent.retainedDuties=empty | requestedOutcome.retainedDuties empty | absent | Closed terminal success; no retained duty execution |
| 33 | delegation=none | ast.intent.delegation=none | absent | absent | No delegated signer; fail closed on extension |
| 34 | recovery=none | ast.intent.recovery=none | absent | absent | No recovery rule; fail closed on extension |
| 35 | operation | signedAction; submitted.action must match | Transfer or Repay derived from signedAction | operationKind plus derived lines | All nested fields below must be bound before preparation |

## Complete nested operation map

| Signed operation field | Source / Core relationship | Required check |
| --- | --- | --- |
| transfer.kind | TransferLiteralFee / Transfer | Exact constructor; no generic effect record substitutes |
| transfer.owner | signedAction.from, submitted.from, intent.signer | All equal; Core does not retain from independently |
| transfer.recipient | signedAction.to, submitted.to, intent.recipient | Exact account equality including zero-value aliases prohibition |
| transfer.feeRecipient | signedAction.feeTo, submitted.feeTo, intent.feeRecipient | Exact equality even fee=0; required third balance remains |
| transfer.amount | signedAction.value, submitted.value, intent.amount | Exact nominal integer; >0 |
| transfer.fee | signedAction.fee, submitted.fee, intent.fee | Exact integer; >=0; Core checks fee cap and gross |
| repayment.kind | RepayAccrualFirst / Repay | Exact supported constructor |
| repayment.obligationId | signedAction.obligation, submitted.obligation, intent.obligationId, state obligation.id | All equal; no replacement obligation |
| repayment.payer | signedAction.payer, submitted.payer, intent.signer; debt.debtor relation is checked at next debtor subfield | Direct Source payer/signer equality here; retain funded Debit relation for debtor/Core |
| repayment.debtor | authenticated obligation.debtor; no separate signedAction debtor field | Exact payer/debtor/signer equality plus B11 snapshot authentication |
| repayment.creditor | authenticated obligation.creditor; Core derives bound creditor | Exact signed creditor to authenticated creditor equality; never overwrite snapshot creditor from wire |
| repayment.amount | signedAction.amount, submitted.amount, intent.amount | Exact nominal integer; >0 and <=outstanding |
| repayment.allocation=AccrualFirst | no explicit Source allocation; selected builtin semantics | B04/B06 must bind selected builtin; Core applies da=min(n, accrued), dp=n-da |
| repayment.conversion=identity | signedAction.conversion, submitted.conversion; no Core conversion slot | Exact selected Source literal and corresponding closed builtin; no invented rate or scale conversion |

## State, effect-image and footprint completeness

| Input / result | Image field | Required producer derivation and missing evidence |
| --- | --- | --- |
| state.core/domain/asset | root.core/domain/asset | Copy from bound Core input; compare signed compatible versions/domain/asset |
| state.round | root.round | Copy exact authenticated round, not validFrom; B11 |
| state.head, result.candidatePost.head | root.preHead/successor and AdvanceHead | Copy exact values; B11/B12/B15 |
| intent.kind | root.operationKind | Transfer->transfer or Repay->repayment, only existing closed constructor relation |
| asset metadata | root.scale | No Core slot; requires adopted B10 binding. Cannot infer from amounts or assume 0/2 |
| result.effects | effects ordered list | Use internally derived Core result, not caller image; preserve gross Debit and zero-fee omission |
| ordered state balances and post balances | footprint.balances | Exact three transfer rows or two repay rows; IDs and before/after values from matching positions; no missing receiver initialization |
| ordered allowances | footprint.allowances | Exactly signer allowance remaining/spent before and after; B11/B14 |
| ordered obligation(s) | footprint.obligations | Repay exactly one full id/debtor/creditor/asset/status/value row; transfer none; pre/post identity unchanged |
| workRemaining/workSpent pre/post | consumption work counters | Copy both authenticated pre-values and Core post-values; no spent reset; one unit debit is existing Core behavior |
| state.consumedReplay / candidatePost.consumedReplay | replayBefore/replayAfter | Preserve complete histories in order, append exact signed tuple; B11/B13/B16 |
| result.requiredPremises | requiredPremises | Fixed required list; requirements are not verified evidence |

Source/6 replay syntax only claims `unused` or `consumed` for the selected key.
Lowering fabricates the corresponding local representation [] or [selected
key] from that claim; it does not read complete ledger replay history. W-D2E
commits complete supplied arrays (cap16). A positive Source-based consumer with
nonempty unrelated history requires an adopted projection or a richer snapshot
transport. Do not discard authenticated history to make [] match. This is B16.

## Missing authenticated bindings

| ID | Required material / unavailable implementation |
| --- | --- |
| B01 | Agreement instance registry binds wire agreementId to Source agreement identity, selected code and authenticated cells |
| B02 | Stage identity and uniqueness, bound to same domain/agreement/episode/snapshot |
| B03 | Episode identity and continuing history, bound to same agreement |
| B04 | Adopted relation among signed actionId, signed coreProgramId, Source selected action and operation constructor |
| B05 | Selected Source code image, exact hash algorithm/domain separation, and authenticated sourceHash correspondence |
| B06 | Canonical selected Core code image, exact hash, source-to-Core lowering correspondence and builtin identity |
| B07 | Exact policy bytes/hash and policy-to-selected-source/intent relationship |
| B08 | Nominal signer/keyRef to x-only signerKey authorization, validity and key-rotation/replay policy |
| B09 | Exact signature-message preprocessing and verifier for selected scheme remain unselected; canonical-wire SHA256 content digest is not an adopted raw signature message |
| B10 | Asset identity, atomic-unit scale and settlement registration; approved scale relation |
| B11 | One authenticated snapshot binds all required balance/allowance/obligation/work/replay cells, head, domain, instance and round |
| B12 | Signed predecessor equals authenticated prior-head linkage of current preHead; field19 is not the current head |
| B13 | Authenticated replay-status evidence for full domain/signer/nonce tuple, require unused, and durable uniqueness policy |
| B14 | Grant/allowance/work authority whose remaining and spent cells bind to same signer, agreement and snapshot |
| B15 | Valid independently selected successor head and head-extension evidence; successor must not depend on this authorization digest |
| B16 | Source replay-claim to complete authenticated replay-history projection without omission |
| B17 | Atomic ledger compare-and-consume/apply binding the exact digest, snapshot, effects, allowances, replay and successor |

Core only names B09/B11/B15/B17 premises; its success does not discharge them.
The Source wrapper additionally names B01/B04/B10/B12 as unverified. The rest
are gaps exposed by complete field mapping. Neither an effect hash nor a caller
Boolean, matching tuple, callback promise, or opaque receipt authenticates them.

The current Source document embeds `source_hash` and `digest` claims. Defining
sourceHash as SHA256 of the entire document that contains sourceHash introduces
a self-reference. There is no selected canonical code-only image or exclusion
rule here. Defining a new masked-document hash, canonical AST serialization,
policy image, Core image, or registry would be a separate reviewed design.

## Compatibility boundaries and exact planned rejections

Wire IDs allow colons, slashes, hyphens and initial digits. Source identifiers
are ASCII initial letters followed by alphanumerics/underscore, with reserved
words excluded. Core IDs allow initial letters and alphanumerics/dot/hyphen/
underscore. No encoding, escaping, renaming or case folding has been adopted.
Source-valid IDs form the proposed local intersection. Wire/effect scale38
versus Source scale18 and UInt64 wire rounds versus UInt128 Source/Core rounds
must be checked before projection; no clamping or modulo arithmetic.

Reading B is the single repaired schedule: F-wire, F-source, D domain sweep,
M tag-ordered literal/binding checks, H complete-history bound, C Core, I image,
E field26, and optional separate L ledger consumption. B01–B16 are prerequisites
for local semantic comparison; B17 is explicitly pending after successful E.
It is never an earlier local-comparison rejection. These are adapter phases,
not an adoption of W-D3's canonical judgment precedence.

### Exact D domain sweep

Visit fields in tag order: domain, agreementId, stageId, episodeId, actionId,
sourceHash, coreProgramId, policyHash, signer, asset, scale, preHead,
predecessor, nonce, validFrom, validUntil; then nested operation fields in the
wire variant's order. Mapped Source values are checked before the corresponding
decoded wire value; Source's signed action before its submitted action.
For stageId/episodeId no Source slot exists, so check only the decoded identifier
against the proposed common identifier subtype. Do not synthesize a Source slot.

Identifier subtype is `[A-Za-z][A-Za-z0-9_]{0,63}` with Source reserved words
excluded. Hash32 claim subtype is exactly 64 lowercase hex characters for
Source selected.sourceHash, selected.policyDigest, intent.preHead,
authenticated.head, authenticated.predecessor and intent.nonce. At preHead,
check intent.preHead before authenticated.head. Source keyRef remains an opaque
reference; do not require it to equal raw key hex or invent a conversion.
Scale comparison uses canonical Source decimal value and wire byte in 0..18;
validity values must be <=2^64-1. The wire decoder already enforces its own
primitive formats. After all signed/operation domains, visit authenticated.round
(UInt64), submitted.postHead (hash32), and AdvanceHead line predecessor then
successor in submitted line order (hash32). For each line use its actual zero-based
parsed-array index i. The current parser produces a unique last AdvanceHead, so
i=submitted.effects.length-1; the diagnostic index is never inferred from signed
or submitted fee value. Optional third Credit presence is grammatical, and can
contradict fee semantics; D still runs before C. Thus symbolic h0/h1 Source claims
fail domain before any authentication check; valid but different heads proceed
to the literal/Core checks below. No Source offset is fabricated for adapter
errors: they carry field/sourcePath; SourceRejected alone preserves parser's
original UTF-8 offset.

D finishes completely before M starts. A malformed late hash or owner subtype
can precede missing B01. A domain-valid late value mismatch cannot precede B01.
This distinction is frozen in the expectation matrix's multi-defect controls.

### Exact M literal and binding order

At tag1 apply selected local V-S0-01 (the exact tuple in PLAN.md), not literal
wire.profile==AST.profile. Scan tags 1–35. At each tag, check direct Source literals first, authenticate
any provider facts anchored there, then compare declared prior-context facts
and this tag's signed value to the retained authenticated facts. Never compare
future signed fields early. An anchor authenticates provider integrity,
declared scope and availability now; agreement with already bound context is
a separate authenticated-fact comparison. It does not compare later signed
fields early.
Authentication is never deferred. Later signed equality is performed at that
field's tag, and every equality must pass before C can execute.
Binding anchors are tag3 B01; tag4 B02; tag5 B03; tag6 B04;
tag8 B05; tag11 B06; tag12 B07; tag13 B08; tag14 B09; tag16 B10; tag18 B11;
tag19 B12; tag20 B13; tag23 B14. After tag35 and its nested fields, perform
the unsigned Source snapshot tail, then B15 at successor, then B16 at replayBefore. Retained contexts are immutable verifier
outputs, not caller claims or booleans. Facts associated with already processed
fields must agree with that verified context at the anchor; equality with future
signed fields waits for those fields. A repeated literal does not waive any
provider authentication or field comparison.
Missing B10 reports field asset, not scale. B17 is not an M anchor.

| Anchor | Authenticate and retain at anchor | Signed comparisons at own tag / unsigned tail |
| --- | --- | --- |
| B01 agreementId/3 | Authentic agreement-instance record, scope to domain; compare current agreement identity | Retain declared stage/episode/action/code/asset associations; compare future fields at4/5/6/10/16 |
| B02 stageId/4 | Authentic stage record and link to already verified agreement; compare stageId | Retain episode association; compare at5 |
| B03 episodeId/5 | Authentic episode/history record and prior agreement/stage links; compare episodeId | Retain relevant later head/history facts; check at their own loci |
| B04 actionId/6 | Authentic selected action manifest; current actionId and prior instance links | Retain Core ID, source/Core/policy image identities and operation constructor; compare at10/8/11/12/35 |
| B05 sourceHash/8 | Authentic selected Source artifact/image and compute retained sourceHash | Compare current sourceHash after authentication; do not treat wire mismatch as invalid artifact evidence |
| B06 coreHash/11 | Authentic selected Core artifact/image and source-lowering correspondence; retain computed Core hash | Compare current coreHash after authentication; Core ID was compared at10 |
| B07 policyHash/12 | Authentic policy artifact and prior selected-context links; retain policy hash and selected policy predicates | Compare current policyHash; test later caps/floor under adopted predicates at23/24/25 |
| B08 signer/13 | Authentic signer ownership/keyRef resolution and prior instance/domain links; compare current signer | Retain authorized x-only key; compare wire.signerKey at15, never at13 |
| B09 keyScheme/14 | Real verifier succeeds under later adopted preprocessing, using retained B08 authorized key and exact canonical record binding | Compare current exact scheme schnorr_bip340; wire.signerKey equality remains tag15. Passing B09 alone cannot establish it |
| B10 asset/16 | Authentic registered asset metadata and prior instance association; compare current asset | Retain registered scale; compare wire.scale at17, never at16 |
| B11 preHead/18 | Authenticate integrity of declared same-instance snapshot proof; retain genuine facts | Compare core to V-S0-01, then prior domain, prior asset, current preHead; exact errors below. Signed predecessor19 and debtor/creditor35 later; unsigned Source round/cells/work at explicit tail |
| B12 predecessor/19 | Authenticate prior-head linkage fact and its already verified current-head scope | Compare current signed predecessor after verification |
| B13 nonce/20 | Authenticate replay-status fact, scoped to prior domain/signer/head; integrity accepts genuine unused or consumed status | Compare current nonce, then require unused; consumed -> W_D2F_REPLAY_CONSUMED at20, not EVIDENCE_INVALID; retain facts for B16 |
| B14 grossCap/23 | Authenticate grant/work authority fact and prior signer/instance/snapshot links | Test current grossCap and later feeCap/netFloor under adopted authority relation at23/24/25; Source allowance/work claims at unsigned tail; never equate cap with remaining counter |
| B15 successor tail | Authenticate head-extension fact and digest-independent selection under adopted protocol | Compare current Source proposed successor to retained authentic successor |
| B16 replay tail | Authenticate integrity/completeness of declared adopted history projection of retained B11 snapshot | Validate all tuple shapes/IDs/nonces, then distinctness, then B13/history membership consistency, then Source replay claim; exact errors in PLAN.md. Preserve complete history, never synthesize [] |

For B08/B10/B04, a valid provider fact with a different future signed key/scale/
Core ID is not an early evidence rejection. It produces FIELD_MISMATCH at15/17/10.
B09 must use the authenticated B08 key, because tag15 equality has not run yet.
A correctly authorized signature can bind a record containing inconsistent key
metadata; tag15 still rejects that metadata. No unselected message convention
is inferred by this scheduling rule. Invalid evidence that fails to authenticate
its declared provider fact remains EVIDENCE_INVALID at the anchor.

Direct literal mappings use the table above. Version/profile vocabularies use
V-S0-01; failure-policy vocabularies use F-S0-01, never raw cross-string equality. Source agreement and
selected action names are literal comparisons, independent of registry trust.
At coreProgramId, compare Source selected.actionId then lowered.intent.programId;
equal strings do not establish B04/B06. sourceHash/policyHash compare claims,
not a computed image; keyRef-to-key comparison requires B08's adopted reference
resolution and is never a guess based on matching opaque text. Missing Source
stage/episode slots require adopted B02/B03 wrapper evidence, not invented IDs.

Head precedence at tag18: wire.preHead versus AST.intent.preHead first, then
AST.authenticated.head; then authenticate B11's declared snapshot fact and compare
its retained facts in this exact order: core (V-S0-01), domain (prior bound
wire/AST domain), asset (prior wire/AST/B10 asset), then current signed preHead.
Genuine wrong core -> BindingRejected/W_D2F_PROFILE_UNSUPPORTED, field coreVersion,
relation V-S0-01; genuine wrong domain/asset -> BindingRejected/
W_D2F_FIELD_MISMATCH, field domain/asset. These are at tag18, binding B11,
sourcePath null, factPath B11.snapshot.core/domain/asset. Invalid proof ->
EvidenceRejected/W_D2F_EVIDENCE_INVALID, field preHead at18 before fact checks.
A valid proof of a different head yields FIELD_MISMATCH/preHead at18, binding B11,
factPath B11.snapshot.head and sourcePath null; the Source literal checks already
passed, so authenticated.head is not the mismatching locus. A proof that does not authenticate
its declared snapshot fact yields EVIDENCE_INVALID/B11 at18. Keep these separate.
At tag19 compare the AST predecessor then verified snapshot prior-head linkage
under B12. AdvanceHead.predecessor always denotes current preHead, never field19.
Submitted successor-line differences are Core effect mismatches at C. An image
root/line discrepancy that survived C is an image mismatch at I. For nested
operations, compare Source signed action then submitted action; repayment
debtor/creditor additionally compare the authenticated obligation. Preserve
owner/payer/recipient/feeRecipient/amount/fee field names exactly as the wire
schema, rather than using Source aliases in reported field names.

At tag35 repayment payer, compare Source signed payer, submitted payer and bound
signer. The relation to verified obligation.debtor is checked at the following
debtor subfield, not moved into payer. At debtor/creditor compare Source
`authenticated.obligation.debtor/creditor` first, then the retained
`B11.snapshot.obligations[0].debtor/creditor`. Source claim failure reports that
sourcePath and does not name a retained-fact binding. Only when wire and Source
agree may a genuine B11 mismatch report binding B11, sourcePath null and that
factPath. The Source document must remain formation-valid: signer/payer/debtor
agree and balance/allowance/creditor rows obey the existing parser. Do not
mutate Source into a formation failure to simulate a later B11 mismatch.

Likewise tag10 first compares AST.selected.actionId to wire.coreProgramId,
then lowerer intent.programId, then retained B04 Core ID. Tag17 first compares
AST.settlement.scale to wire.scale, then retained B10 registered scale. A
literal failure names selected.actionId or settlement.scale and does not claim
the later retained-fact predicate ran. Existing equal-claim B04/B10 mismatch
cases remain the independent authenticated-fact controls.

After tag35/subfields and before B15/B16, compare **unsigned Source snapshot
claims** against retained authenticated facts in this exact order: round,
balances in Source row order (account then amount), allowance (owner, remaining,
spent), obligation (id, debtor, creditor, asset, principal, accrued, outstanding,
status), workRemaining, workSpent. Counts and order are exact. Report field and
sourcePath as `authenticated.<path>` for these unsigned claims, not a fictional
wire tag. These checks cannot be skipped because signed head/commitment matches.
Source's selected replay claim is compared at B16; Core receives complete verified
history under the adopted projection, not Source's unauthenticated [] shortcut.
B16 authenticates complete-history proof integrity, then validates every canonical
three-string tuple/domain ID/signer ID/nonce hash, then distinctness, then
B13-versus-history selected membership, then the Source replay-status claim.
Shape/component -> BindingRejected/W_D2F_DOMAIN_UNSUPPORTED; first duplicate
-> BindingRejected/W_D2F_HISTORY_DUPLICATE. A genuine history containing selected
key while B13 verified unused -> BindingRejected/W_D2F_REPLAY_HISTORY_INCONSISTENT,
field consumption.replayBefore, binding B16, factPath B16.completeReplayHistory,
sourcePath null, exact selectedIndex. These precede the Source claim and H/C;
invalid proof remains EVIDENCE_INVALID before any fact check. Exact tuple and
failure loci are specified in PLAN.md. Thus no bad history ID falls into Core stage.
B16 then compares the selected replay-status claim:
Source consumed versus verified absent rejects FIELD_MISMATCH at the tail, field
and sourcePath authenticated.replay, factPath B16.selectedReplayStatus. A genuine
B13 consumed proof rejects W_D2F_REPLAY_CONSUMED at tag20 first; invalid B13 proof
rejects EVIDENCE_INVALID. Invalid B12/B13/B14 evidence needs only earlier tags
through18/19/22, respectively; the unsigned tail has not run at those anchors.
Invalid B16 evidence requires all35 fields, unsigned tail and B15 to pass.

F-S0-01 is explicitly selected for this local adapter: Source success_only
corresponds to wire atomic-reject-terminal-success only for closed S0 terminal
success with empty retained effects/duties and atomic unpublished rejection.
These literals differ and are never tested by string equality. Existing grammar/
codec rejects unsupported literals at formation; new failure branches require a
separate adopted relation. This local selection does not close normative W-D2.

Adapter D diagnostics use exact slots: intent.preHead -> field preHead,
sourcePath intent.preHead; authenticated.head -> field preHead, sourcePath
authenticated.head; authenticated.predecessor -> field predecessor, sourcePath
authenticated.predecessor; submitted.postHead -> field successor, sourcePath
submitted.postHead; submitted AdvanceHead line -> field effects[i].predecessor
or effects[i].successor and sourcePath submitted.effects[i].predecessor/successor.
The zero-based index i is the actual parsed AdvanceHead line position, currently
the unique final line i=submitted.effects.length-1. Concrete canonical vectors
have index5 for transfer with Debit plus two Credits or repayment, and
index4 for transfer without the optional fee Credit. Fee0 with an extra Credit
can still parse with index5; fee1 with omitted Credit can parse with index4.
D reports the actual index before any later C effect-vector rejection. There
is no universal index or fee-to-index function.
For a wire-only subtype failure, sourcePath is null and inputPath names wire.field.
This names current/prior/proposed heads distinctly without inventing offsets.

### H history and I image bounds

After B01–B16 verify, check complete authenticated replayBefore count, then
the full count after appending the unused selected tuple, before C. If either
exceeds16, return `EffectDomainRejected/W_D2F_REPLAY_HISTORY_BOUND` with exact
field (`consumption.replayBefore` or `consumption.replayAfter`), count, cap16,
null published effects/post. At I, an otherwise shaped supplied replay array
exceeding16 gets the same explicit result, before generic W-D2E encoding.
Do not truncate, deduplicate, discard unrelated keys, or classify these as
authorization WireRejected. Example: pre16 unused keys -> post17 rejects at
replayAfter; pre17 -> replayBefore wins. B17 remains pending in both cases and
does not alter this local phase ordering.

At I validate/encode the derived side first, then supplied root/replay array
shape and replayBefore/replayAfter counts, then supplied codec shape/bounds,
then complete equality. EffectImageRejected carries imageSide=derived or
supplied; shaped supplied history overflow carries imageSide=supplied, with
replayBefore winning if both counts overflow. H authoritative history overflow
carries imageSide=derived. These are effect image errors, never WireRejected.

The proposed adapter diagnostics are distinct from W-D3's six judgments:

- wire decode error -> `WireRejected`, preserve existing codec `code`, null
  published effects/post;
- Source parse error -> `SourceRejected`, preserve existing Source6Error code
  and offset, null published effects/post;
- out-of-intersection signed identifier, round or scale -> `BindingRejected`,
  `W_D2F_DOMAIN_UNSUPPORTED`, name exact field; no translation;
- missing adopted relation or authentication evidence -> `BindingRejected`,
  `W_D2F_BINDING_UNAVAILABLE`, exact field and binding ID;
- mismatch under an available adopted relation -> `BindingRejected`,
  `W_D2F_FIELD_MISMATCH`, exact field;
- invalid evidence under an implemented verifier -> `EvidenceRejected`,
  `W_D2F_EVIDENCE_INVALID`, exact binding ID; this is specified-only because
  those verifier implementations are unavailable;
- Core rejection -> `CoreRejected`, preserve existing judgment/code/
  diagnosticWork=1, null published effects/post;
- supplied image not exactly equal to complete internally derived image ->
  `EffectRejected/W_D2F_PREPARED_IMAGE_MISMATCH`, null published effects/post;
- matching derived image but field26 differs ->
  `EffectRejected/EFFECT_COMMITMENT_MISMATCH`, null published effects/post.

Image schema errors preserve W-D2E code under EffectImageRejected; replay-cap
overflow uses the explicit EffectDomainRejected result above. E equality yields
SemanticComparedUnqualified with verifiedBindings B01–B16 and pendingBindings
[B17], null published effects/post, and no ledger change. Only separately
requested L mode can report LedgerRejected/W_D2F_BINDING_UNAVAILABLE for B17.

After D passes and earlier direct literals match, the first unavailable
local-path mapping today is agreementId/B01. This is a planned reject, not an
observed consumer execution. At any downstream predicate tested in isolation,
earlier gates remain explicit preconditions; no synthetic evidence may make a
full-path positive. Even after future verified gates, a local comparison must
remain unqualified while atomic ledger acceptance B17 is unavailable.

### Exact direct Core construction boundary

The specified constructor in PLAN.md copies every S0State field from verified
B11.snapshot except consumedReplay, which comes from verified B16.completeReplayHistory
for that same snapshot. Bound lowerSource6(ast).intent supplies the exact S0Intent;
lowerSource6(ast).submittedEffects supplies ordered effects with transformed
composite UseReplay keys. Verified B15.successor supplies proposedPostHead.
TerminalSuccess has empty retained arrays; optional localStipulation is omitted.
Never pass lowerer.state, Source-fabricated []/[selectedKey], raw AST replay
lines, or prepareSource6S0Unqualified. The direct Core diagnostic remains intact.
The external interface names snapshotProofInput; only the adopted proof verifier
can create the internal verified B11 snapshot. No caller snapshot is trusted.

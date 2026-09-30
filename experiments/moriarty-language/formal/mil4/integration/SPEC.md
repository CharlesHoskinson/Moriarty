# W-D2F/G/H integration proposal

**Status: integration repair-02 / proposed / specified-only, 2026-09-30.**
This independent successor reconciles W-D2F `grok-repair-05`, W-D2G
`grok-repair-04`, and W-D2H `grok-repair-04`. Exact prior integration files are
preserved under [history/pre-integration-repair-02](history/pre-integration-repair-02/SPEC.md),
with their hashes in [RESULT](RESULT.md). No source proposal is changed.

F05, G04 and H04 have independently accepted **bounded specified-only design
dispositions**. Their acceptance adopts no image producer, authentication
implementation or normative W-D2. H04 frozen packet SHA256 is
`1e47bfc15567b12630da95814807caeffe739776a42cb9041fcfebf1b5fe1a3b`.
Both GPT-6.1 Sol high and requested Grok4.7 xhigh (returned grok-4.7-build)
report no high/medium defect; review limits/hashes are in [RESULT](RESULT.md).
This integration repair-02 remains unreviewed and closes no B01–B17 gate.

The immediately preceding integration repair-01 is preserved under
[history/pre-integration-repair-02](history/pre-integration-repair-02/SPEC.md);
the original reconciliation remains in pre-integration-repair-01. H04 source
bytes control this successor. It repairs H03's constructor/H-H14C diagnostic
issues while retaining exact16/17 and35 order. Other explicit integration
refinements remain proposals requiring exact-candidate audit.

The 87-test content-codec Proxy repair packet has GPT bounded approval and Grok
pending. Its separate 50-test Source adapter packet awaits both required reviews.
Neither implementation packet is accepted by this reconciliation. Test counts
are attributed packet/reviewer results; this agent executes no tests.

## Shared contract and authority

Repository observation: F's outer sequence is F-wire → F-source → complete
D sweep → tag-ordered M → H → C → I → E, with separately requested L ledger
mode ([F PLAN](../effect-consumer/PLAN.md), phases1–9). All three packets retain
formation errors before adapter checks, domain checks before any missing
binding, literal checks before provider authentication, and future equality at
that field's own tag. Successful E remains SemanticComparedUnqualified with
B01–B16 verified, B17 pending and null published post/effects. Today a compatible
full-path proposal with matching literals and absent providers would stop at
agreementId/B01; this is a specified outcome, never an observed run.

Recommendation: use G's mapping A only after adoption: agreementId equals the
Source agreement name; actionId and coreProgramId have distinct nominal types
but each equals the selected S0 builtin. The supported triples remain Transfer/
Transfer/transfer and Repay/Repay/repayment with their full builtin spellings.
Stage/episode come from explicit wrapper claims, never defaults or Core IDs.
Add `context.stageId/context.episodeId` as proposed inputs; their present domains
are checked before their respective wire values in D. Missing claims wait for
M/tag4 or5. This explicitly extends F's current wire-only D positions, rather
than claiming F already accepts the wrapper.

One authenticated immutable registry view scopes B01–B07 to domain/agreement/
selection. B01 associations are membership sets, not singleton equality or
unbounded implementation arrays. B02 authenticates a stage occurrence and
retains an association reference. B03 verifies exact prior-stage membership.
Neither supplies a second current head. B11 alone authenticates the declared
snapshot plus registry-view/stage-to-snapshot linkage, then compares core,
domain, asset and head in F's order. B12 owns prior predecessor, B15 owns the
digest-independent successor. AdvanceHead.predecessor is current preHead, not
signed field19 ([G SPEC](../identity-binding/SPEC.md), minimum registry/check
order; [F FIELD-MAP](../effect-consumer/FIELD-MAP.md), head precedence).

Authentication verifies a declared fact, its scoped provider and required
links. Invalid proof/wrong selected artifact link is EVIDENCE_INVALID under an
implemented verifier; absent definition/provider is BINDING_UNAVAILABLE. A
genuine fact whose value differs from a signed/current-context field is
FIELD_MISMATCH at its comparison locus. Equal hashes or caller flags establish
neither provenance nor scope. G's G38–G40 wrong B04-image links remain evidence
failures; H's H-H11/H-H19/H-H20 genuine policy-body differences remain fact
mismatches. These cases concern different links and must remain separate.

## Proposed merged M insertions

The following exact order incorporates H04 rules and identifies remaining
integration proposals in [ISSUE-MATRIX](ISSUE-MATRIX.md). All preceding gates are
explicit prerequisites; none of these downstream examples supplies providers.

Use `BindingRejected/W_D2F_FIELD_MISMATCH` for genuine unequal values and
`EvidenceRejected/W_D2F_EVIDENCE_INVALID` for an implemented verifier's failed
authentication/linkage. Missing definitions/providers use
`BindingRejected/W_D2F_BINDING_UNAVAILABLE`. Every rejection has null published
post/effects. Adapter errors have no fabricated parser offset.

For new retained-artifact comparison diagnostics, record `binding`,
`comparisonTag`, `sourcePath=null`, `factPath` and `inputPath` as specified below.
An owning comparison tag may exceed the signed field's original tag; it never
rewinds first-failure order. Existing F Source literal failures retain their
existing field/sourcePath and do not gain a retained-fact binding.

| Tag | Exact proposed sequence |
| --- | --- |
| 3 | Source agreement literal; B01 scoped authentication; current agreement fact. Retain association sets for later checks. |
| 4 | Wrapper stage literal; B02 scoped authentication; B01 stage membership. Retain B02 episode association/reference, with no head comparison. |
| 5 | Wrapper episode literal; B03 authentication including exact prior-stage membership; B01 episode membership; retained B02 episode equality; current B03 episode fact. |
| 6 | Source action literal; B04 declared manifest/prior-link authentication; B01 action membership; retained B04 current-action fact. Retain future Core/constructor/image facts. A changed current-action fact fails here. |
| 8 | Source sourceHash literal; B05 selected-artifact authentication including B04 reference/view linkage; validate/encode purpose1; compare prior definition facts in the order below; compare signed sourceHash to computed artifact hash. Retain later asset/scale/constructor. |
| 10 | Source selected.actionId literal; actual lowerer intent.programId literal; B01 Core-program membership; retained B04 expected CoreProgramId; retained B04 builtin selector. |
| 11 | B06 selected-package/provenance, actual labels/exports and loaded-artifact/correspondence authentication including B04 reference/view linkage; validate/encode purpose3; compare package labels/prior Core ID in the order below; compare signed coreHash to computed package hash. Retain constructor until35. |
| 12 | Source policyDigest literal; B07 declared-policy authentication including B04 reference/view linkage; decode/encode purpose4 including keyRef image-domain admission; compare H's nine prior body facts in the order below; compare signed policyHash to computed artifact hash. Retain later policy terms. |
| 16 | Direct Source settlement.asset equality; B10 authentication and already-bound scope checks; B01 asset membership; retained B10 current asset equality; retained B05 definition asset equality; retained B07 policy asset equality. |
| 17 | Direct Source settlement.scale equality; retained B10 registered scale equality; retained B05 definition scale equality; retained B07 policy scale equality. Do not reauthenticate B10 or compare scale at16. |
| 18 | Source intent.preHead then authenticated.head literals; B11 declared snapshot/linkage authentication; core then domain then asset then head facts, using F's exact diagnostics. No B02/B03 head veto. |
| 23 | Direct Source grossCap equality; B14 authentication and prior-scope comparisons; retained B07 grossCap equality; adopted B14 gross-cap authority predicate. |
| 24/25 | Direct Source feeCap/netFloor equality; retained B07 current term equality; adopted B14 current authority predicate. No B14 reauthentication or future-field comparison at23. |

At8 use H04's exact six-body-field order: wireProfile, domain,
agreementInstanceId, selectedActionId, sourceVersion, profile. Compare against
retained V-S0-01/tag2/B01/B04/tag7/formation-selected Source/6 respectively.
Image formation/domain validity precedes these equalities. All genuine unequal
body facts use FIELD_MISMATCH, binding B05, comparisonTag8,
field/inputPath `sourceDefinition.<name>`, factPath
`B05.sourceDefinition.<name>`, sourcePath null. Only then compare sourceHash
with the computed purpose1 digest. This replaces the prior integration's
five-check tuple grouping and PROFILE_UNSUPPORTED reinterpretation. Fixed
profile/version bytes cannot be mutated into a formation-valid hostile vector;
no nonexistent branch is claimed executed. Asset/scale/constructor remain later
facts, so an asset/scale-only change with compatible prior body facts and old
hash claims fails computed-hash equality at8. A permitted constructor switch
must change selectedActionId too: with unchanged prior selected action, the
fourth body comparison fails FIELD_MISMATCH, field/inputPath
sourceDefinition.selectedActionId, factPath B05.sourceDefinition.selectedActionId,
binding B05, comparisonTag8, sourcePath null, before hash. A kind changed
without its paired selector fails purpose1 image-domain validation earlier:
DOMAIN_UNSUPPORTED, field/inputPath sourceDefinition.operationKind, factPath
B05.sourceDefinition.operationKind, binding B05, comparisonTag8, sourcePath null.
Neither case is deferred to35 or confused with asset/scale stale-hash equality.

At11 compare package wireProfile, sourceVersion, coreVersion, coreProfile,
intentSchema, then coreProgramId. The first five must match the retained exact
V-S0-01 compatibility tuple; genuine incompatibility uses PROFILE_UNSUPPORTED,
binding B06, comparisonTag11, relation V-S0-01, field/inputPath
`corePackage.<name>`, factPath `B06.package.<name>`. Core-program difference uses
FIELD_MISMATCH with `corePackage.coreProgramId` and
`B06.package.coreProgramId`. Actual missing/false export or execution
correspondence rejects unavailable/invalid before these value comparisons;
fixed labels alone cannot authenticate execution. Package selector/kind self-
admission runs at11; the exact unpaired-kind diagnostic is H04 low residue.
Only comparison of an admitted package constructor with signed operation.kind
waits for35. Compatibility errors outside this fixed suite remain conditional;
no alternate image suite is tried to produce a passing digest.

At12 preserve H's exact nine-field order: wireProfile, domain,
agreementInstanceId, actionId, sourceVersion, sourceHash, coreVersion,
coreProgramId, coreHash. Compare with retained V-S0-01/tag2/B01/B04/tag7/B05/
tag9/tag10/B06 facts respectively. Use FIELD_MISMATCH, binding B07,
comparisonTag12, field/inputPath `policy.<name>`, factPath
`B07.policy.<name>`, sourcePath null. Only then compare policyHash itself.
This keeps H's existing policy mismatch family, including body metadata,
rather than introducing an implicit profile-error reinterpretation.

Purpose4 actionId/kind self-admission is distinct from signed operation.kind
equality at35; H04 low residue calls for explicit 12 admission controls.
Before any tag12 body equality, validate the authenticated purpose4 artifact's
keyRef as decoded Unicode scalars: shortest exact UTF-8,1–1024 bytes,
at most1024 UTF-16 code units, no lone surrogates, no U+0000..U+001F or U+007F.
H04 specifies that a genuine selected artifact with an inadmissible keyRef uses
BindingRejected/W_D2F_DOMAIN_UNSUPPORTED, field/inputPath `policy.keyRef`,
binding B07, comparisonTag12, factPath `B07.policy.keyRef`, sourcePath null.
No Source parser error or offset is fabricated. A missing image-domain
definition remains unavailable; a failed artifact authentication still rejects
invalid before decoded fact validation. This is now an H04 candidate rule;
its bounded design disposition is accepted. Malformed envelope/other parser
diagnostics still require an adopted image decoder contract and do not borrow
authorization WireRejected codes. Later keyRef-to-Source equality at13 is a
separate predicate, not a reason to postpone artifact admission.

At23 use H04's proposed grant-scope tuple
`(domain,agreementInstanceId,signer,asset,preHead)` after B14 authentication.
Compare in that order with retained tags2/3/13/16/18. A genuine difference uses
FIELD_MISMATCH, binding B14, comparisonTag23, sourcePath null and
inputPath/factPath `B14.grant.<component>`. Corresponding fields are
`domain/agreementId/signer/asset/preHead`, respectively. Current preHead is not
field19. This aligns with H04 and replaces the prior integration's snapshotHead
projection and grant.<name> field convention. Missing/invalid evidence remains
earlier; direct Source grossCap remains earlier still. Only after scope passes
compare policy grossCap, then the adopted grant authority predicate. The actual
financial/work predicate, its full transport and failure diagnostic remain open;
no cap is equated to allowance.remaining or spent/work claims moved before F's tail.

At16/17 a retained B05 or B07 mismatch reports signed field asset/scale,
comparisonTag16/17, binding B05/B07, factPath `B05.definition.asset/scale` or
`B07.policy.asset/scale`, inputPath `wire.asset/scale`, sourcePath null.
B10 current fact failures use B10.asset/B10.scale. Its genuine prior scope
checks at16 are domain then agreementInstanceId: field domain/agreementId,
inputPath wire.domain/agreementId, binding B10, factPath
B10.domain/agreementInstanceId, sourcePath null. Missing/invalid B10 uses field
asset, comparisonTag16, inputPath wire.asset, sourcePath/factPath null. B01
membership failure keeps binding B01 and factPath `B01.assetIds`.

Other later H terms must also be checked: B07 signer/keyRef at13 under B08,
interval at21/22, failure at27 through F-S0-01, exact empty/none constructors
at28–34, and operation terms at35. Their adopted owner/provider relations
remain missing. This proposal does not silently substitute policy equality for
B08, B09, B11 or B14 authority predicates.

## Proposed operation order and literal custody

At35 visit wire variant subfields exactly: transfer kind, owner, recipient,
feeRecipient, amount, fee; repayment kind, obligationId, payer, debtor,
creditor, amount, allocation, conversion. For each existing Source action slot
compare signed Source then submitted Source before retained facts.

For kind, compare direct Source constructors, then B05 definition constructor,
B04 manifest constructor, B06 package constructor, then B07 policy operationKind
and its variant discriminator. Genuine retained differences use FIELD_MISMATCH,
field operation.kind, comparisonTag35 and the owning binding/factPath. All
retained constructors participate; a package/body hash never waives this check.
This explicit B06 placement remains an integration refinement: H04 retains its
package constructor but does not enumerate that owning comparison in its35 list.

For ordinary policy operation terms, compare direct Source claims then the
retained B07 term. At repayment obligationId compare B07 before B11 obligation
ID. At payer compare signed Source payer, submitted payer, bound signer, then
B07 payer. **Do not compare B11 debtor at payer.**

At debtor compare wire.debtor to Source authenticated.obligation.debtor first.
If equal, compare retained B11.snapshot.obligations[0].debtor; require its
relation to the already bound payer/signer. At creditor compare wire.creditor
to Source authenticated.obligation.creditor first, then retained B11 creditor.
Source claim differences use field operation.debtor/creditor and sourcePath
`authenticated.obligation.debtor/creditor`, inputPath
`wire.operation.debtor/creditor`, comparisonTag35 and null binding/factPath.
Genuine
B11 differences require equal wire/formation-valid Source and use binding B11,
comparisonTag35, the same inputPath, sourcePath null and factPath
`B11.snapshot.obligations[0].debtor/creditor`. Neither policy variant has a
debtor/creditor slot. Source's parser equality restrictions still apply; no
malformed Source is used to simulate this late fact predicate.

The unsigned snapshot tail subsequently retains F's round, ordered balance
rows, allowance, obligation and both work-counter order. Repeating obligation
claims there does not erase their signed debtor/creditor checks at35.

## Replay, hash links and direct Core boundary

Preserve F B13 unused status at20 and B16 tail authentication/completeness;
validate every canonical three-string tuple and domain/signer/nonce component,
then distinctness, then B13 selected-membership consistency, then Source replay
claim. Historical tuples need not share current domain/signer. H checks pre
count then full appended post count against16 before C. Pre16 with unused
selected tuple gives post17 rejection; no truncation or deduplication.

C uses verified B11 cells/work with B16 complete history, actual lowered bound
intent and ordered composite-UseReplay effects, B15 independent successor and
TerminalSuccess. It calls direct prepareMil4S0; no Source wrapper, lowerer.state,
localStipulation or fabricated signedDigest supplies that boundary. I derives
the complete image from that same pre/post; validates derived side before
supplied shape/count/codec/equality; E hashes the derived image. H's purpose3
three-file package describes the local path and **does not commit the absent
complete-history adapter**; B06 needs separately bound implementation evidence
for that actual future adapter ([H SPEC](../hash-images/SPEC.md), package and
lowering obligations). Its format/authority is an open decision.

Recommend H purpose1 B, purpose3 exact package and purpose4 scoped policy only
after adoption. Producer dependencies are acyclic: Source definition/package
→ computed Source/Core hashes → policy → derived effects/image → field26 → wire
content digest. H purpose2 is a preimage alternative with no complete adopted
consumer path. Never hash a self-containing source_hash/policyHash, retry suites,
or turn canonical wire SHA256 into B09's unselected signature message. Manifest
links, policy body links and signed hash equality are three distinct predicates.

## First-failure examples for the proposed successor

| Multiple defect, with all unspecified earlier gates passing | First result |
| --- | --- |
| A genuine B04 current-action fact is Repay; Source/wire are Transfer; later Core ID also differs | actionId/tag6 FIELD_MISMATCH/B04. This is a separate isolating control; G04 G29 now keeps current action Transfer and correctly reaches10 if earlier gates pass. |
| B04 current-action remains Transfer but expected Core ID alone is Repay; B05 missing | sourceHash/tag8 BINDING_UNAVAILABLE/B05. With B05 and intervening gates passing, coreProgramId/tag10 mismatch follows. |
| B10 absent; B01 omits A and retained definition/policy assets differ | asset/tag16 BINDING_UNAVAILABLE/B10. With genuine compatible B10, B01 membership fails first. |
| B01 includes A and B10 asset is A; genuine definition says B and policy says C | asset/tag16 FIELD_MISMATCH/B05 before B07. |
| Source/wire scale agree at2; B10 registered scale3; definition4 and policy5 | scale/tag17 FIELD_MISMATCH/B10; Source literal difference would precede it. |
| Correctly selected authentic policy has wrong body domain and wrong signed policyHash | policy.domain/tag12 FIELD_MISMATCH/B07 before policyHash. Wrong B04-image linkage instead fails B07 authentication first. |
| Authentic selected policy has a forbidden control in keyRef and also a wrong body domain | policy.keyRef/tag12 DOMAIN_UNSUPPORTED before body comparisons; no invented Source error. |
| Source/wire grossCap match; genuine B14 grant has wrong signer; policy cap also differs | signer/tag23 FIELD_MISMATCH/B14, inputPath/factPath B14.grant.signer, before policy grossCap after authentication and earlier scope facts pass. |
| Policy payer differs; Source/wire payer agree; later B11 debtor differs | operation.payer/tag35 FIELD_MISMATCH/B07 before debtor. |
| Wire creditor differs from Source claim and B11 fact | operation.creditor/tag35 Source literal failure, no B11 fact label. With equal wire/Source and genuine other creditor, B11 fact mismatch follows. |
| B13 proves unused; genuine B16 complete history contains selected tuple and overflows16 | REPLAY_HISTORY_INCONSISTENT at B16 before H cap. |

## Current implementation evidence and bounded next work

The separately frozen content-codec Proxy repair-01 packet records 87 tests.
GPT's bounded local content review recomputed 18 manifest hashes, reran87
embedded tests and independently probed prior selector/asset/nested-value
attacks. It reports zero get calls and coherent owned snapshots, resolving the
high defect within its local reviewed model. **Grok is pending; this is not a
completed required implementation acceptance.** Preserve the earlier 72-test
packet and high finding as history rather than rewriting their evidence.

The Source-image adapter packet separately records 50 tests and text admission
through parseSource6 into the nine purpose1 definition fields. Both required
reviews remain pending. A packet claiming an AST projection cannot stand in for
accepted artifact provenance or execution correspondence. This reconciliation
runs neither implementation nor tests and makes no new success assertion.

The smallest current follow-on is to complete exact-result reviews for these
two frozen local content packets and repair any returned defects in their
existing scopes. Avoid starting another content producer while those results
remain pending. If accepted within their content scope, separately select the
first actual authenticated binding increment: B01 trusted-view/instance namespace
selection and a verifier with explicit unavailable/invalid/genuine-fact cases.
Its authority/proof format is still an open decision; caller flags/hashes must
not become authentication or a synthetic positive consumer fixture.

F/G low residue remains executable work: diagnostic key normalization, safe
obligation admission/indexing before 35 without moving unsigned-value checks,
signed/submitted paths, isolated current-action fact mismatch and exact abbreviated
Core-program diagnostics. An unreachable redundant tag10 check must not receive
an invented first-failure test.

H04 low residue must also be carried into any consumer implementation:
package unpaired-kind failure at11, purpose4 actionId/kind admission at12
separate from signed equality at35, and the keyRef image-domain step in every12
summary. These are specified safeguards requiring explicit executable controls,
not current implementation gate closures. Full consumer work still requires
B01–B16 real providers, actual adapter/runtime correspondence and independently
verified complete B16 transport. B17 remains separate atomic ledger acceptance.

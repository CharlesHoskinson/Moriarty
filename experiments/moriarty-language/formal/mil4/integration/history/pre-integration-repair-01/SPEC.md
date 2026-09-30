# W-D2F/G/H integration proposal

**Status: proposed / specified-only, 2026-09-30.** This independent reconciliation
compares W-D2F `grok-repair-05`, W-D2G `grok-repair-03`, and W-D2H
`grok-repair-01`. It adopts none of them. The merged rules below are a new
proposal requiring exact-candidate independent audit and the repository's
consequential decision process. They do not amend the source packets.
No consumer, encoder, verifier, implementation, test, proof or ledger outcome
is supplied. B01–B17 and normative W-D2/W-D3 remain open.

The inspected file hashes are recorded in [RESULT](RESULT.md). Relative links
below designate that inspected version; live source paths can change. The
parent reports G repair-04 and H repair-02 frozen with bounded GPT approval and
Grok review pending; H addresses four medium review findings. F repair-05's
Grok review is also pending. Those
successor bytes and complete fresh reviews are not evidence for this pinned
repair-03/repair-01 reconciliation. Required follow-up is exact-byte
reconciliation of both successors; this document does not pre-approve them.

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

The following exact order resolves gaps identified in [ISSUE-MATRIX](ISSUE-MATRIX.md).
It is a recommendation for a successor contract. All preceding gates are
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
| 12 | Source policyDigest literal; B07 declared-policy authentication including B04 reference/view linkage; decode/encode purpose4; compare H's nine prior body facts in the order below; compare signed policyHash to computed artifact hash. Retain later policy terms. |
| 16 | Direct Source settlement.asset equality; B10 authentication and already-bound scope checks; B01 asset membership; retained B10 current asset equality; retained B05 definition asset equality; retained B07 policy asset equality. |
| 17 | Direct Source settlement.scale equality; retained B10 registered scale equality; retained B05 definition scale equality; retained B07 policy scale equality. Do not reauthenticate B10 or compare scale at16. |
| 18 | Source intent.preHead then authenticated.head literals; B11 declared snapshot/linkage authentication; core then domain then asset then head facts, using F's exact diagnostics. No B02/B03 head veto. |
| 23 | Direct Source grossCap equality; B14 authentication and prior-scope comparisons; retained B07 grossCap equality; adopted B14 gross-cap authority predicate. |
| 24/25 | Direct Source feeCap/netFloor equality; retained B07 current term equality; adopted B14 current authority predicate. No B14 reauthentication or future-field comparison at23. |

At8 compare definition facts against retained context in this order:
wireProfile/V-S0-01, domain/tag2, agreementInstanceId/tag3, selectedActionId/tag6,
sourceVersion/tag7. Source profile is checked with sourceVersion as the exact
Source/6 compatibility tuple. A genuine incompatible tuple reports
PROFILE_UNSUPPORTED, binding B05, comparisonTag8, relation V-S0-01 and
field `sourceDefinition.profile`; other differences report FIELD_MISMATCH with
field/inputPath `sourceDefinition.domain`, `.agreementInstanceId` or
`.selectedActionId`, factPath `B05.definition.<name>`. Supported image construction
has fixed version/profile literals; this tuple error is conditional future
provider compatibility behavior, not a fabricated current encoder vector.

At11 compare package wireProfile, sourceVersion, coreVersion, coreProfile,
intentSchema, then coreProgramId. The first five must match the retained exact
V-S0-01 compatibility tuple; genuine incompatibility uses PROFILE_UNSUPPORTED,
binding B06, comparisonTag11, relation V-S0-01, field/inputPath
`corePackage.<name>`, factPath `B06.package.<name>`. Core-program difference uses
FIELD_MISMATCH with `corePackage.coreProgramId` and
`B06.package.coreProgramId`. Actual missing/false export or execution
correspondence rejects unavailable/invalid before these value comparisons;
fixed labels alone cannot authenticate execution. Package operationKind waits
for35. Compatibility errors outside this fixed suite remain conditional;
no alternate image suite is tried to produce a passing digest.

At12 preserve H's exact nine-field order: wireProfile, domain,
agreementInstanceId, actionId, sourceVersion, sourceHash, coreVersion,
coreProgramId, coreHash. Compare with retained V-S0-01/tag2/B01/B04/tag7/B05/
tag9/tag10/B06 facts respectively. Use FIELD_MISMATCH, binding B07,
comparisonTag12, field/inputPath `policy.<name>`, factPath
`B07.policy.<name>`, sourcePath null. Only then compare policyHash itself.
This keeps H's existing policy mismatch family, including body metadata,
rather than introducing an implicit profile-error reinterpretation.

Before any tag12 body equality, validate the authenticated purpose4 artifact's
keyRef as decoded Unicode scalars: shortest exact UTF-8,1–1024 bytes,
at most1024 UTF-16 code units, no lone surrogates, no U+0000..U+001F or U+007F.
For a genuine selected artifact with an inadmissible keyRef, propose
BindingRejected/W_D2F_DOMAIN_UNSUPPORTED, field/inputPath `policy.keyRef`,
binding B07, comparisonTag12, factPath `B07.policy.keyRef`, sourcePath null.
No Source parser error or offset is fabricated. A missing image-domain
definition remains unavailable; a failed artifact authentication still rejects
invalid before decoded fact validation. This new content diagnostic requires
audit; H repair01 did not fully specify it. Malformed envelope/other parser
diagnostics still require an adopted image decoder contract and do not borrow
authorization WireRejected codes. Later keyRef-to-Source equality at13 is a
separate predicate, not a reason to postpone artifact admission.

For tag23 B14 prior scope, propose a normalized genuine grant fact carrying
domain, agreementInstanceId, signer, asset and snapshotHead. Its selected-view/
record authenticity/linkage is verified first. Then compare those five values
in that exact prior wire-anchor order against tag2/B01/B08/B10/B11 respectively.
An authentic unequal value reports FIELD_MISMATCH, binding B14,
comparisonTag23, field/inputPath `grant.<name>`, factPath `B14.grant.<name>`,
sourcePath null; `snapshotHead` compares to the verified B11 head, not field19.
This is a new proposed logical projection, not an adopted grant encoding or
provider schema. False scoped provenance remains invalid; missing definition/
provider remains unavailable. Only after all five facts pass compare policy
grossCap, then the adopted grant authority predicate. Its actual financial/
work predicate and failure diagnostic remain an explicit open decision; these
scope comparisons neither equate grossCap to allowance.remaining nor move
unsigned spent/remaining/work claim checks ahead of F's tail.

At16/17 a retained B05 or B07 mismatch reports signed field asset/scale,
comparisonTag16/17, binding B05/B07, factPath `B05.definition.asset/scale` or
`B07.policy.asset/scale`, inputPath `wire.asset/scale`, sourcePath null.
B10 fact failures keep binding B10 and its corresponding fact path; B01
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

For ordinary policy operation terms, compare direct Source claims then the
retained B07 term. At repayment obligationId compare B07 before B11 obligation
ID. At payer compare signed Source payer, submitted payer, bound signer, then
B07 payer. **Do not compare B11 debtor at payer.**

At debtor compare wire.debtor to Source authenticated.obligation.debtor first.
If equal, compare retained B11.snapshot.obligations[0].debtor; require its
relation to the already bound payer/signer. At creditor compare wire.creditor
to Source authenticated.obligation.creditor first, then retained B11 creditor.
Source claim differences use field operation.debtor/creditor and sourcePath
`authenticated.obligation.debtor/creditor`, with no retained B11 label. Genuine
B11 differences require equal wire/formation-valid Source and use binding B11,
comparisonTag35, sourcePath null and factPath
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
| G29's genuine B04 current-action fact is Repay; Source/wire are Transfer; later Core ID also differs | actionId/tag6 FIELD_MISMATCH/B04. The repair-03 tag10 oracle is contradicted. |
| B04 current-action remains Transfer but expected Core ID alone is Repay; B05 missing | sourceHash/tag8 BINDING_UNAVAILABLE/B05. With B05 and intervening gates passing, coreProgramId/tag10 mismatch follows. |
| B10 absent; B01 omits A and retained definition/policy assets differ | asset/tag16 BINDING_UNAVAILABLE/B10. With genuine compatible B10, B01 membership fails first. |
| B01 includes A and B10 asset is A; genuine definition says B and policy says C | asset/tag16 FIELD_MISMATCH/B05 before B07. |
| Source/wire scale agree at2; B10 registered scale3; definition4 and policy5 | scale/tag17 FIELD_MISMATCH/B10; Source literal difference would precede it. |
| Correctly selected authentic policy has wrong body domain and wrong signed policyHash | policy.domain/tag12 FIELD_MISMATCH/B07 before policyHash. Wrong B04-image linkage instead fails B07 authentication first. |
| Authentic selected policy has a forbidden control in keyRef and also a wrong body domain | policy.keyRef/tag12 DOMAIN_UNSUPPORTED before body comparisons; no invented Source error. |
| Source/wire grossCap match; genuine B14 grant has wrong signer; policy cap also differs | grant.signer/tag23 FIELD_MISMATCH/B14 before policy grossCap, after B14 authentication and earlier scope facts pass. |
| Policy payer differs; Source/wire payer agree; later B11 debtor differs | operation.payer/tag35 FIELD_MISMATCH/B07 before debtor. |
| Wire creditor differs from Source claim and B11 fact | operation.creditor/tag35 Source literal failure, no B11 fact label. With equal wire/Source and genuine other creditor, B11 fact mismatch follows. |
| B13 proves unused; genuine B16 complete history contains selected tuple and overflows16 | REPLAY_HISTORY_INCONSISTENT at B16 before H cap. |

## Smallest executable next sprint, conditional recommendation

The first runnable deliverable should be a **standalone purpose1 Source-definition
encoder/comparator**, after the repaired identity mapping and exact purpose1
format receive independent adoption. This is smaller than a full consumer or
three-image authenticated path. It needs no invented registry or signature
provider. B01–B17 remain unresolved by its results.

1. Freeze two new complete formation-valid Source documents, strict Transfer and
   Repay selectors, plus independent purpose1 expected payload/envelope bytes and
   SHA256 digests. Preserve old W-D2E fixtures unchanged. Choose a literal
   shared-domain predicate for this standalone content entry point; do not claim
   F's wire-dependent full D sweep was executed without wire input.
2. Implement one callable Source-document → parsed nine-field tuple → purpose1
   bytes/digest path, and a comparator against an explicit hash32 claim. Reject
   unsupported profiles/domain/byte shape with documented content diagnostics.
   Return only bytes/digest/content equality, unresolved bindings and null
   published effects/post. No callback accepts `verified:true` or synthetic proofs.
3. Demonstrate the frozen bytes and claim mismatch, claim replacement invariance,
   whitespace/comment invariance, genuine declaration-change preimage differences,
   formation-first wrong-selector rejection and purpose/suite rejection. Describe
   expected differing hashes under collision resistance; unequal preimages alone
   are the deterministic oracle. Obtain fresh result audits on frozen exact bytes.

This is a proposed future sprint, not authorization or execution of it here.
Purpose3/4 and their independent vectors can follow under the same adopted
format. Full semantic consumer work must additionally adopt the merged M order,
registry/view/snapshot authority, B08–B16 providers and adapter correspondence.
B17 is a separate actual compare-and-consume/apply demonstration. Missing
providers never become positive fixtures or acceptance receipts.

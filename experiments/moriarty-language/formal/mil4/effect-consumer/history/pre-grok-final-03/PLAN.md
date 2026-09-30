# W-D2F local semantic-consumer design plan

**Status:** design only, frozen before implementation. No consumer module is
created by this sprint. Full positive consumption is specified-only because
named mappings and authentication providers are unavailable.

**Goal:** map each signed field, identify exact missing evidence, and select a
bounded next implementation that cannot silently discard agreement, selected
program, scale, hash, predecessor or history obligations.

**Inputs read-only:** wire/SPEC.md and codec.mjs; Source/6 frontend and local
wrapper; Core/5 S0 preparer; W-D2E SPEC.md, codec.mjs and frozen fixtures.
**Outputs this sprint:** FIELD-MAP.md, EXPECTATIONS.json, this PLAN.md,
freeze-receipt.json and RESULT.md; all inside effect-consumer/.
**Acceptance:** exhaustive header+35-field and nested-operation map; complete
state-to-image derivation map; independent intended expectations for all six
W-D2E positives and hostile gates; exact diagnostics with gate preconditions;
no invented mapping and no claim of an observed end-to-end consumer success.

## Proposed full-path interface, specified-only

`consumeSource6Authorization({canonicalAuthorizationBytes, sourceDocument,
authenticatedSnapshot, signatureEvidence, selectionEvidence,
headExtensionEvidence, submittedEffectImage})`.

Evidence parameters name future verifier outputs. Their constructors, trust
roots and verification are absent. A JSON object or a caller Boolean is not a
substitute. Implementation must not add an option that admits empty evidence.
This repair selects **Reading B**: B01–B16 must be adopted and verified for a
local semantic image/field26 comparison. B17 is recorded as pending after that
comparison; it is not a rejection prerequisite for local comparison. Actual
ledger consumption additionally requires B17. Every provider remains absent
today, and this sprint implements neither local comparison nor consumption.

One total sequence controls first failure; an isolated predicate observation
must not be presented as this sequence's result:

1. **F-wire:** existing canonical decoder, 4096-byte bound. On error preserve
   WireRejected/code; do not parse Source. A typed effect-image domain error
   later is never WireRejected.
2. **F-source:** existing parser, 65536 UTF-8-byte bound. Preserve SourceRejected,
   original Source6Error code and UTF-8 byte offset. Parser formation checks
   (including selected action, aliases and scale18) precede every adapter check.
3. **D:** complete compatibility-domain sweep before any binding check. Visit
   signed fields in tag order, each mapped Source counterpart before the wire
   value, then operation fields in wire variant order. Check identifier subtype,
   mapped Source hash32 claims, scale and validity widths; then Source snapshot
   round and proposed successor. Exact lists are in FIELD-MAP.md. First failure
   -> BindingRejected/W_D2F_DOMAIN_UNSUPPORTED with field and sourcePath. Thus a
   late out-of-domain operation.owner beats unavailable agreementId/B01; a
   well-formed late literal mismatch does not. Never translate, clamp or mask.
4. **M:** signed tags in order. At each tag, perform declared direct literal
   checks, authenticate the provider facts anchored there and retain immutable
   verified context, then compare this tag's signed value to authenticated facts.
   Anchors authenticate integrity/scope/availability immediately, without
   comparing future signed fields early. Future equality is checked at that
   field's own tag. Literal/fact difference -> W_D2F_FIELD_MISMATCH; unavailable
   definition/provider -> W_D2F_BINDING_UNAVAILABLE; invalid implemented evidence
   -> W_D2F_EVIDENCE_INVALID. Then operation subfields in wire variant order,
   followed by unsigned Source snapshot claims in exact FIELD-MAP order, then
   B15 successor and B16 replay-history projection. Each B01–B16
   binding is verified once at its anchor, not silently skipped because a
   repeated literal matches. B10 anchors at asset/tag16, before scale/tag17.
   B04/tag6 retains Core ID for comparison at10; B08/tag13 retains authorized
   key for comparison at15; B10/tag16 retains registered scale for comparison
   at17. Complete provider/field comparisons, including unsigned snapshot
   claims, are mandatory before C. B09/tag14 uses retained B08 authorized key;
   a passing signature never substitutes for the later signerKey comparison.
   B09 anchors at exact keyScheme=schnorr_bip340/tag14: its message preprocessing and verifier
   are unselected; raw SHA256 as the signature message is not adopted here.
   SHA256 of canonical authorization bytes is only the current wire content
   digest. Default local scope never resolves or demands B17.
   At tag27 use selected local F-S0-01 correspondence between Source success_only
   and wire atomic-reject-terminal-success for closed S0 success/atomic reject.
   Their different strings are not a direct literal equality. This local rule
   does not adopt normative W-D2. Provider authentication and current-tag fact
   checks are detailed for every anchor in FIELD-MAP.md.
5. **H:** complete verified replay history must fit W-D2E's cap16 before Core
   preparation. Check replayBefore count first, then count after appending the
   selected tuple (if absent). >16 -> EffectDomainRejected/
   W_D2F_REPLAY_HISTORY_BOUND with field consumption.replayBefore or replayAfter,
   exact count and cap16, and null published effects/post. Do not truncate,
   deduplicate, or call this an authorization decoder rejection. Selected-key
   reuse is a B13 evidence failure in this verified path; isolated Core reuse
   tests retain their own history rejection.
6. **C:** existing read-only Core prepare with the exact verified snapshot,
   bound signed intent, ordered submitted Source effects, independent successor
   and terminal-success outcome. Core derives internally before sameEffects.
   Preserve first existing judgment/code/diagnosticWork on rejection. Effect
   mismatches precede authority/history/failure; late Core predicate expectations
   explicitly require sameEffects to have passed. The outer B01–B16 verified
   path may reject stale-head or signature evidence earlier than an isolated
   Core predicate. Never turn those isolated tests into full-path assertions.
7. **I:** only after PreparedUnqualified, derive the entire W-D2E image from the
   same verified pre-state and Core post, including registered B10 scale, every
   required ordered cell, both work counters and complete replay histories.
   I sub-order is: validate/encode derived image; check supplied root/replay-array
   shape and replayBefore count then replayAfter count; validate/encode supplied
   image; compare complete images. Codec-shaped errors -> EffectImageRejected
   with original W-D2E code and imageSide=derived or supplied. Schema-valid
   unequal complete image -> EffectRejected/W_D2F_PREPARED_IMAGE_MISMATCH. Shaped
   supplied replay arrays over16 use explicit W_D2F_REPLAY_HISTORY_BOUND with
   imageSide=supplied before generic supplied codec checks; when both overflow,
   replayBefore wins. Validate both sides with the W-D2E encoder;
   any other effect codec bound (including record4096) preserves its original
   code under EffectImageRejected, with imageSide=derived or supplied. Neither
   side's encoder errors are authorization WireRejected. Do not hash the submitted
   image as the preparation oracle.
8. **E:** hash the derived complete image and compare decoded field26. Mismatch
   -> EffectRejected/EFFECT_COMMITMENT_MISMATCH. Equality ->
   `{status:'SemanticComparedUnqualified', commitment, verifiedBindings:[B01..B16],
   pendingBindings:[B17], publishedPost:null, publishedEffects:null}`. Candidate
   diagnostics may be returned separately; no effect is applied or published.
9. **L, separate requested ledger mode:** resolving B17 and performing actual
   atomic compare-and-consume is required for a ledger outcome. Missing B17
   fails only this later mode; it cannot retroactively block phases C/I/E or
   convert their unqualified result into ledger acceptance.

## Honest bounded runnable subset

An **inspection-only adapter** can be implemented without inventing binding
rules. Proposed API:

`inspectSource6Wire({canonicalAuthorizationBytes, sourceDocument,
submittedEffectImage}) -> {status:'InspectionOnly', directComparisons,
unresolvedBindings, localCoreObservation, imageObservation,
publishedPost:null, publishedEffects:null}`.

It may parse existing types, show direct scalar differences, retain every
unresolved signed field, run existing Source/Core local preparation, and show
W-D2E equality separately. It must not combine these observations into
SemanticAccepted, Authenticated, Accepted or Qualified. Mappings marked
unselected stay unresolved even when strings happen to match. Their equality
is not an implementation of the missing registry or hash relation.

The second honest subset is testing existing Core/effect-image predicates in
isolation against already frozen literal vectors, explicitly without a Source
to signed-wire identity claim. This is useful regression evidence but largely
overlaps W-D2E. A fail-closed gate can reject current full-path requests with
the named unavailable binding; that negative behavior is runnable. No full
positive semantic consumer is currently runnable honestly.

## Recommended decision order and first meaningful implementation

Do not build the inspection API now. Existing W-D2E already covers isolated
Core/equality observations. The executable gap is authenticated correspondence,
so the next work should resolve the definitions needed to test that gap.

1. Adopt an explicit nominal identity map for agreement instance, stage, episode,
   action and selected Core program (B01–B04). Decide which missing identities
   need a new typed wrapper, and reject ambiguous historical labels. Do not
   reuse Core.intent.programId for the agreement.
2. Select exact Source, Core and policy hash images and lowerer correspondence
   (B05–B07). Resolve Source's embedded source_hash/digest self-reference by a
   reviewed image definition, without silently masking fields. Pin exact bytes
   and independently expected hashes before coding.
3. Adopt signer/keyRef ownership and asset/scale mappings (B08/B10). Reconcile
   Source18 and wire38 scale bounds explicitly. Select common identifier and
   round domains without translation. Do not assume current literal key hashes
   or scale2 supply metadata authentication.
4. Specify and implement the authenticated snapshot transport, predecessor,
   replay policy, grant/work cells and complete replay-history projection
   (B11–B14/B16). Preserve same-head provenance and every required cell.
5. Adopt digest-independent head extension and exact signature message/verifier
   semantics (B15/B09). Use real selected implementations with negative evidence
   controls. These definitions must coexist with field26 without feedback into
   the successor or selected source image.
6. Freeze new independent full local positive/hostile vectors under those
   definitions. Existing W-D2E authorizations remain equality fixtures; they
   are not automatically upgraded to authenticated Source fixtures.
7. Implement the **bound semantic derivation adapter** as the first useful new
   consumer: decode exact authorization bytes, check every adopted mapping and
   available verified premise, prepare effects with read-only Core/5, derive all
   image rows from that same pre/post, compare complete submitted image, then
   recompute field26 from the derived image. Tests must reject a re-committed
   hostile vector even though W-D2E equality alone passes. Missing providers
   reject; no successful full authentication is simulated.
8. Preserve commands, exact outputs, scope hashes and dependency hashes in a
   new implementation packet. Obtain separately routed review. Only after this
   local result should B17 atomic ledger acceptance/consumption be implemented
   and empirically demonstrated; local success must expose B17 as still open.

## Implementation gate and missing decisions

Do not implement or assert a positive full consumer until B01 agreement-instance,
B02 stage, B03 episode, B04 action/program and B05–B07 source/Core/policy images
have adopted definitions. B08 keyRef, B10 asset/scale, B12 predecessor and B16
replay-history transport require adopted definitions too. B09/B11/B13–B15
require real verification implementations for local comparison; B17 requires
separate actual ledger consumption. Independent positive
fixtures must then be frozen from those definitions before full-path code.

This design does not expand Source/Core types, choose a masked source hash,
drop a signed field, create a synthetic authenticated registry, bind opaque
heads by a caller flag, or adopt new normative W-D2/W-D3 semantics.

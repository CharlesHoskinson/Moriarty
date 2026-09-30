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
Until adopted B01–B17 relationships exist, this interface must fail closed or
remain unimplemented. This sprint chooses unimplemented full consumption.

Proposed full sequence, with exact planned first failure:

1. Bound binary authorization by the existing decoder (4096 bytes) and source
   by the existing parser (65536 UTF-8 bytes). Wire rejection precedes Source
   parsing. Preserve existing codec or Source error code/offset; no candidate.
2. Parse Source/6 without changing existing lowering. Check common identifier
   alphabets, scale<=18 and Source rounds/validity inside UInt64 before any
   projection. Reject W_D2F_DOMAIN_UNSUPPORTED naming first signed field in tag
   order; no normalization or invented ID registry.
3. Resolve every relation in FIELD-MAP.md, signed tag order followed by nested
   operation order. Distinguish unavailable evidence from unequal values.
  Current first unresolved relation: agreementId/B01 ->
   W_D2F_BINDING_UNAVAILABLE. Literal agreementId==AST.programId can be inspected
   before this gate, but cannot authenticate an agreement instance. Likewise,
   literal actionId==AST.selected.actionId is available independently of B04's
   missing authenticated action/Core-program relation.
4. Verify signature over SHA256 of canonical authorization bytes, exact key
   ownership, selected Source/Core/policy identities and same-head snapshot.
   Bind agreement/stage/episode, asset/scale, predecessor, unused composite replay
   key, grant/work counters and digest-independent successor. Reject unavailable
   B08–B17 evidence; no matching-tuple acceptance.
5. Compare every directly carried Source signed term with decoded wire terms,
   under the adopted relations. Literal inspection can compare agreement/action
   claims even when their authentication is unavailable. Check all endpoints and fee even when
   fee=0. Repay creditor/asset/debtor come from authenticated obligation; compare
   wire fields without overwriting the authenticated cell.
6. Invoke existing read-only Core/5 prepare with bound state, bound intent,
   submitted ordered effects, preselected successor and terminal-success outcome.
   Core computes its vector internally. A supplied image cannot dictate the
   derived vector. Preserve existing Core rejection judgment and code exactly.
7. On PreparedUnqualified only, construct the W-D2E image from exact bound pre
   and candidate post fields using FIELD-MAP.md's row-by-row recipe. Scale comes
   from adopted B10 evidence, not a literal default. Preserve all required rows,
   unchanged zero-fee recipient, allowance spent, both work counters, and complete
   replay histories. Require B16's projection; do not replace a complete history
   by Source's []/selected-key local representation.
8. Compare the entire submitted image with the complete derived image using
   exact schema fields and ordered arrays. Mismatch ->
   W_D2F_PREPARED_IMAGE_MISMATCH. Hash only the derived image using W-D2E codec,
   compare decoded field26; mismatch -> EFFECT_COMMITMENT_MISMATCH.
9. No local comparison reports ledger acceptance. Atomic B17 verification and
   actual consumption is a separate later implementation. Successful local
   predicates, when eventually implemented, expose remaining obligations.

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
replay-history transport require adopted definitions too. B09/B11/B13–B15/B17
require real verification/consumption implementations. Independent positive
fixtures must then be frozen from those definitions before full-path code.

This design does not expand Source/Core types, choose a masked source hash,
drop a signed field, create a synthetic authenticated registry, bind opaque
heads by a caller flag, or adopt new normative W-D2/W-D3 semantics.

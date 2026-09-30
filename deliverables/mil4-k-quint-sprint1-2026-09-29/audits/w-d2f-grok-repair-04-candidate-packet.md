Independent read-only W-D2F grok-repair-04 design audit. Review ONLY exact embedded bytes; no tools, skills, delegation, external pages or workspace files. Requested GPT-6.1 Sol high and Grok 4.7 xhigh independently. Prior packet SHA256 6c31612244d3c3da0e382329b780f62a36c62c409bb217b07c1bcdfb65d1d7d1 had GPT bounded design approval and Grok four medium defects: generic preHead sourcePath mismatch, AdvanceHead index based on fee class instead of actual parsed vector, missing B11 snapshot core/domain/asset first-failure, and B13-unused versus B16-consumed plus invalid historical replay ID. Current candidate claims these repairs. Check exact F/D/M/H/C/I/E/L order, V-S0-01/F-S0-01, six specified-only positives/150 hostile/11 inspections, header+35 fields, 14 nested fields, B01-B17, C direct Core with verified history. Find remaining high/medium contradictions. Design only, zero consumer executions and W-D2/Sprint1 gates open.

## Manifest

```json
[
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-consumer/PLAN.md",
    "bytes": 21751,
    "sha256": "1c41a95eac4871223e77e15485a0534572d352aae728317fa2853ab58da3fbb2"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-consumer/FIELD-MAP.md",
    "bytes": 36726,
    "sha256": "b7257ef34a0cba9a25c4c660a76be16e9aae5d879062ad2d182f9931dca0a660"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-consumer/EXPECTATIONS.json",
    "bytes": 193955,
    "sha256": "131ee6a89bfbb32ecde2cc090738ad18b53c04d2c36d474e1839c3b72f69a0eb"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-consumer/RESULT.md",
    "bytes": 13459,
    "sha256": "226cf6c0fafef34aff79374b3373fccc3eaa3c6ebd8dd4f09ead3c935a4712d0"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-consumer/freeze-receipt.json",
    "bytes": 6386,
    "sha256": "0e94dcea9939278434467c2e9628708d9b27072054ba06f8ef6eee8aaf000e65"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/SPEC.md",
    "bytes": 9579,
    "sha256": "644ababf2c3c0a80a73a3d56a5f37e0780a5e6a197cc27a0eb2f356d731d0ae1"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/codec.mjs",
    "bytes": 9975,
    "sha256": "8cdc9e65e3005895cd69bebf240275ce5fedc17fe58283bba2757bf06e0743fe"
  },
  {
    "path": "experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts",
    "bytes": 20746,
    "sha256": "f3d2a05d07b051d2097fd7fb00ed1cd013eba91e9da7a2591b6f69707d4a02e7"
  },
  {
    "path": "experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts",
    "bytes": 2640,
    "sha256": "1b3bd3b01ed2694ea967f62fea6808843a198045d870270b1d44c121081f66d8"
  },
  {
    "path": "experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts",
    "bytes": 17917,
    "sha256": "855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda"
  },
  {
    "path": "experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md",
    "bytes": 11648,
    "sha256": "725662ae7572a93cbc8f0d1631ca364a56d1c3dff3b9b02cae92485cd5a899ec"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/SPEC.md",
    "bytes": 8671,
    "sha256": "53690f52df23dc20c7bf464e266f08fc233829f0d065f94b6404317bb5f945f8"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/codec.mjs",
    "bytes": 7988,
    "sha256": "1b47621bac1ff3ecb0c87eb730352739b155824df7e14f6b98333ddb044b55ad"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/fixtures.json",
    "bytes": 131232,
    "sha256": "23fc291984118217503349d3b033f1c56422ea161b7d0935758e55e18183df93"
  }
]
```

## experiments/moriarty-language/formal/mil4/effect-consumer/PLAN.md

sha256: `1c41a95eac4871223e77e15485a0534572d352aae728317fa2853ab58da3fbb2`

```text
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
snapshotProofInput, signatureEvidence, selectionEvidence,
headExtensionEvidence, submittedEffectImage})`.

Evidence parameters name untrusted proof objects or provider requests, never
trusted caller snapshots. Only successful adopted verifiers create internal
immutable verified context. Their constructors, trust roots and verification
are absent. A JSON object or a caller Boolean is not a
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
   round and proposed successor. Derive every AdvanceHead index from the actual
   parsed submitted.effects array; do not infer its location from fee value. Exact lists are in FIELD-MAP.md. First failure
   -> BindingRejected/W_D2F_DOMAIN_UNSUPPORTED with field and sourcePath. Thus a
   late out-of-domain operation.owner beats unavailable agreementId/B01; a
   well-formed late literal mismatch does not. Never translate, clamp or mask.
4. **M:** first apply local V-S0-01 at tag1 to the exact version/profile
   tuple below, then signed tags in order. At each tag, perform declared direct literal
   checks, authenticate the provider facts anchored there and retain immutable
   verified context, then compare declared prior-context facts and this tag's
   signed value to authenticated facts. Genuine prior-context differences use
   the named binding comparison; proof authenticity is a separate predicate.
   Anchors authenticate integrity/declared-scope/availability immediately, without
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
   reuse is a valid B13 status rejection (W_D2F_REPLAY_CONSUMED), not an
   invalid-proof error; isolated Core reuse
   tests retain their own history rejection.
6. **C:** call direct read-only prepareMil4S0 with the exact constructor below:
   verified B11 snapshot cells/work, complete B16 history, bound lowered intent,
   ordered lowerer-transformed submitted effects, verified B15 successor and
   terminal-success outcome. Core derives internally before sameEffects.
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

## Exact local profile relation and C handoff (specified-only)

V-S0-01 is a selected local compatibility relation over this single tuple:
wire header `moriarty-intent/3`, profile `s0-provisional/1`, sourceVersion `6`,
coreVersion `5`; AST.profile and lowered.intent.sourceProfile
`moriarty-financial-agreement-source/6`; lowered.intent.version
`moriarty-intent/3`; lowered.intent.core and verified snapshot.core
`moriarty-core/5`; derived image schema `moriarty-s0-effects/1` and core
`moriarty-core/5`. Different vocabularies are related, never compared as if
wire.profile must equal AST.profile. At M/tag1 check the formation-produced
wire/AST/lowered tuple; B11 later authenticates the declared snapshot proof and
checks core/domain/asset/head in the exact order below; I checks the
image tuple. A formation-valid unsupported tuple would reject
BindingRejected/W_D2F_PROFILE_UNSUPPORTED at field profile, relation V-S0-01.
The current closed decoder/parser/lowerer cannot produce that unsupported
M/tag1 branch; later genuine snapshot core mismatch has its separate B11
field coreVersion oracle. The tag1 branch is a specified future compatibility
branch, not fabricated wire
bytes or an executable hostile fixture. Current wrong-version inputs keep
their earlier F-wire/F-source formation errors. V-S0-01 establishes local
compatibility only, and neither source selection authenticity nor normative W-D2.

### B11 fact checks at tag18

After tag18 Source literal checks (intent.preHead, then authenticated.head),
authenticate integrity of the B11 proof and its declared same-instance snapshot
fact. A proof failing integrity/authenticity rejects EvidenceRejected/
W_D2F_EVIDENCE_INVALID, field preHead, binding B11, comparisonTag18, sourcePath
null; do not compare unauthenticated content. Proof validity does not imply
that its genuine values match the selected context. After integrity succeeds,
compare retained facts in this order: (1) snapshot.core against V-S0-01 Core/5,
(2) snapshot.domain against the already bound wire/AST domain, (3) snapshot.asset
against the already bound wire/AST/B10 asset, (4) snapshot.head against current
wire.preHead. A genuine wrong core rejects BindingRejected/
W_D2F_PROFILE_UNSUPPORTED, field coreVersion, relation V-S0-01; genuine wrong
domain/asset/head rejects BindingRejected/W_D2F_FIELD_MISMATCH, field domain/
asset/preHead respectively. All name binding B11, comparisonTag18, sourcePath
null and factPath B11.snapshot.core/domain/asset/head. Earlier signed fields
are compared here because this new provider first supplies their facts now;
no future-field equality is moved earlier. The unsigned snapshot tail still
follows all signed tags. No differing core/domain/asset can reach C and become
an accidental Core stage diagnostic.

### B16 complete-history checks before H/C

At the B16 tail, authenticate integrity/completeness and the declared history
projection of the already retained B11 snapshot. Invalid proof rejects
EvidenceRejected/W_D2F_EVIDENCE_INVALID, field replayBefore, binding B16 before
inspecting unverified values. A genuine fact is then checked, in this order:

1. Validate every tuple in supplied history order before semantic membership:
   canonical JSON string of exactly three string components, then domain ID,
   signer ID, nonce hash32. Domain/signer use the common identifier subtype from
   D (including reserved-word exclusion); nonce uses exactly64 lowercase hex.
   Shape/canonical failure -> BindingRejected/W_D2F_DOMAIN_UNSUPPORTED, field
   consumption.replayBefore[i]; component failure uses suffix .domain/.signer/
   .nonce. All use binding B16, sourcePath null, factPath
   B16.completeReplayHistory[i] (shape) or [i][0/1/2] (component). The first
   invalid tuple/component wins; this is M validation of newly authenticated
   facts, not the earlier D sweep or Core stage fallback.
2. Require distinct canonical tuple strings, scan order; first duplicate at i
   -> BindingRejected/W_D2F_HISTORY_DUPLICATE, field consumption.replayBefore[i],
   binding B16, sourcePath null, factPath B16.completeReplayHistory[i]. Do not
   deduplicate. No history length is truncated; H's pre/post cap checks follow.
3. Compare complete-history membership of the exact selected composite tuple
   to retained B13 status. Since B13 required unused at20, a genuine B16 history
   containing it rejects BindingRejected/W_D2F_REPLAY_HISTORY_INCONSISTENT,
   field consumption.replayBefore, binding B16, sourcePath null, factPath
   B16.completeReplayHistory, selectedIndex equal to its zero-based position.
   This is authenticated-fact inconsistency, not invalid proof or the distinct
   consumed-at-tag20 rejection. It precedes Source replay-claim equality even
   if Source claims consumed. Integrity of each fact does not waive consistency.
4. Compare Source authenticated.replay to the history-derived selected status;
   existing FIELD_MISMATCH/authenticated.replay oracle applies here.

Only then retain B16.completeReplayHistory for H and the exact C constructor.
Unrelated history domains/signers are permitted if their IDs are in the shared
subtype; they need not equal the current selected tuple's domain/signer.

At C all M predicates, including unsigned snapshot claims and B16, and H have
passed. The following is the exact proposed construction, not executable
consumer code. `verified` denotes immutable verifier outputs; none of its
fields is a caller assertion. All arrays retain order, counts and values;
copying permits neither sorting nor omission. B16 verifies that its complete
history is the adopted projection of this same B11 snapshot, with each tuple
serialized exactly as Core's canonical JSON string `[domain,signer,nonce]`.

```ts
const lowered = lowerSource6(ast); // read-only existing lowering, bound by B04/B06
const s = verified.B11.snapshot;
const coreState: S0State = {
  core: s.core, domain: s.domain, asset: s.asset, head: s.head, round: s.round,
  workRemaining: s.workRemaining, workSpent: s.workSpent,
  balances: s.balances.map(({ account, amount }) => ({ account, amount })),
  allowances: s.allowances.map(({ owner, remaining, spent }) => ({ owner, remaining, spent })),
  obligations: s.obligations.map(({ id, debtor, creditor, asset, principal, accrued, outstanding, status }) =>
    ({ id, debtor, creditor, asset, principal, accrued, outstanding, status })),
  consumedReplay: [...verified.B16.completeReplayHistory],
};
const coreIntent: S0Intent = { ...lowered.intent }; // every mapped field already bound in M
const coreSubmittedEffects: S0Effect[] = lowered.submittedEffects.map(line => ({ ...line }));
const outcome: S0RequestedOutcome = { phase: 'TerminalSuccess', retainedEffects: [], retainedDuties: [] };
const result = prepareMil4S0(coreState, coreIntent, coreSubmittedEffects,
  verified.B15.successor, outcome); // omit optional localStipulation
```

The lowerer transforms each submitted UseReplay nonce into
`JSON.stringify([ast.domain, ast.intent.signer, effect.key])`. Pass those
transformed effects, never raw AST UseReplay lines. Do not use lowered.state,
its Source-claim-derived `[]`/`[selectedKey]`, or Source wrapper preparation.
Do not manufacture signedDigest or a local stipulation. The Source wrapper's
unverified field republication and later overridden diagnostic are not the C
handoff. M has already checked signed/submitted action correspondence. Preserve
the direct Core result's judgment/code/diagnosticWork; build I from coreState
and its actual candidatePost/effects, not from republished Source state.

B13 first authenticates the replay-status proof, compares its tuple's current
nonce at tag20 (prior domain/signer/head must already agree), then requires
status unused. A genuine consumed fact rejects BindingRejected/
W_D2F_REPLAY_CONSUMED, field nonce, binding B13, factPath B13.replay.status,
comparisonTag20. An invalid proof remains EvidenceRejected/
W_D2F_EVIDENCE_INVALID. If B13 verifies unused and B16 verifies the selected key
absent but Source claims consumed, the B16 tail rejects FIELD_MISMATCH,
field/sourcePath authenticated.replay, factPath B16.selectedReplayStatus.
B13 rejection occurs before that tail even when Source also claims consumed.

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

```

## experiments/moriarty-language/formal/mil4/effect-consumer/FIELD-MAP.md

sha256: `b7257ef34a0cba9a25c4c660a76be16e9aae5d879062ad2d182f9931dca0a660`

```text
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
| repayment.payer | signedAction.payer, submitted.payer, intent.signer, debt.debtor | All equal; funded Debit |
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

```

## experiments/moriarty-language/formal/mil4/effect-consumer/EXPECTATIONS.json

sha256: `131ee6a89bfbb32ecde2cc090738ad18b53c04d2c36d474e1839c3b72f69a0eb`

```json
{
  "schema": "moriarty-wd2f-design-expectations/1",
  "status": "Repaired design expectations; no consumer implementation/execution; prior frozen design preserved",
  "implementationExists": false,
  "fullConsumerObservedSuccesses": 0,
  "sourceFixtureBinding": "No supplied Source6 document is bound to any W-D2E authorization; do not fabricate one",
  "existingEffectFixtureSha256": "23fc291984118217503349d3b033f1c56422ea161b7d0935758e55e18183df93",
  "positiveCandidates": [
    {
      "id": "transfer-fee",
      "sourceFixture": null,
      "coreEffectFixture": "../effect-wire/fixtures.json#transfer-fee",
      "effectBytes": 583,
      "effectSha256": "bc633665d8ae112b5b7a842dd9c02a010c584266dfd23e23afcf443f0fcb7c0e",
      "literalPostCondition": "Debit11 CreditR10 CreditF1; allowance11->0,spent0->11",
      "commonLiteralPostCondition": "workRemaining2->1,workSpent7->8; append exact composite nonce; preselected55*32->aa*32 head",
      "status": "specified-only-this-sprint",
      "localPredicateExpected": {
        "core": "PreparedUnqualified",
        "imageEquality": "CommitmentEqualUnqualified",
        "authentication": false
      },
      "fullConsumerExpectedNow": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "field": "agreementId",
        "binding": "B01",
        "publishedPost": null,
        "publishedEffects": null
      },
      "fullConsumerPositive": "specified-only-blocked",
      "blockers": [
        "B01",
        "B02",
        "B03",
        "B04",
        "B05",
        "B06",
        "B07",
        "B08",
        "B09",
        "B10",
        "B11",
        "B12",
        "B13",
        "B14",
        "B15",
        "B16"
      ],
      "fullPathPrecondition": "A well-formed compatible Source6 document and matching earlier direct fields are required before B01 resolution; no such authenticated Source fixture has been supplied or executed. Local V-S0-01 tuple relation must pass; C uses exact verified B11/B16 construction, never Source replay shortcuts.",
      "plannedLiteralInspection": {
        "agreementId": {
          "wire": "Agreement",
          "sourceAstProgramId": "Agreement",
          "relation": "equal",
          "authentication": false
        },
        "actionId": {
          "wire": "Action",
          "sourceSelectedActionId": "TransferLiteralFee",
          "relation": "different",
          "authentication": false
        },
        "consequence": "Original W-D2E authorization is not a full Source/wire positive; retain B01/B04 separately from literal comparisons."
      },
      "expectedPreState": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "100"
          },
          {
            "account": "R",
            "amount": "0"
          },
          {
            "account": "F",
            "amount": "0"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "11",
            "spent": "0"
          }
        ],
        "obligations": [],
        "consumedReplay": []
      },
      "expectedCompletePostState": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "89"
          },
          {
            "account": "R",
            "amount": "10"
          },
          {
            "account": "F",
            "amount": "1"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "0",
            "spent": "11"
          }
        ],
        "obligations": [],
        "consumedReplay": [
          "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
        ]
      },
      "expectedCompleteOrderedEffects": [
        {
          "kind": "Debit",
          "account": "O",
          "asset": "A",
          "amount": "11"
        },
        {
          "kind": "Credit",
          "account": "R",
          "asset": "A",
          "amount": "10"
        },
        {
          "kind": "Credit",
          "account": "F",
          "asset": "A",
          "amount": "1"
        },
        {
          "kind": "UseAllowance",
          "owner": "O",
          "amount": "11"
        },
        {
          "kind": "UseReplay",
          "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
        },
        {
          "kind": "AdvanceHead",
          "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
          "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        }
      ],
      "expectedCompleteEffectImage": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "oracleOrigin": "Unchanged preimplementation frozen W-D2E literal pre/post/effect/image expectations; no consumer result used",
      "pendingLedgerBinding": "B17",
      "futureLocalSuccess": {
        "status": "SemanticComparedUnqualified",
        "verifiedBindings": [
          "B01",
          "B02",
          "B03",
          "B04",
          "B05",
          "B06",
          "B07",
          "B08",
          "B09",
          "B10",
          "B11",
          "B12",
          "B13",
          "B14",
          "B15",
          "B16"
        ],
        "pendingBindings": [
          "B17"
        ],
        "publishedPost": null,
        "publishedEffects": null,
        "ledgerChange": false,
        "preconditions": "All F/D/M/H/C/I/E predicates pass under adopted B01-B16 definitions and real verified providers; no such execution exists."
      },
      "futureLocalSuccessPrecondition": "Requires new independently frozen Source/wire fixture after adopted bindings; original Action fixture cannot satisfy direct actionId equality."
    },
    {
      "id": "transfer-zero-fee",
      "sourceFixture": null,
      "coreEffectFixture": "../effect-wire/fixtures.json#transfer-zero-fee",
      "effectBytes": 560,
      "effectSha256": "ed5b4bff25bb0669421728744daf24bd15affe7d9c93ea94e6d19d4858a5aa6d",
      "literalPostCondition": "Debit10 CreditR10; F unchanged4; allowance20->10,spent3->13",
      "commonLiteralPostCondition": "workRemaining2->1,workSpent7->8; append exact composite nonce; preselected55*32->aa*32 head",
      "status": "specified-only-this-sprint",
      "localPredicateExpected": {
        "core": "PreparedUnqualified",
        "imageEquality": "CommitmentEqualUnqualified",
        "authentication": false
      },
      "fullConsumerExpectedNow": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "field": "agreementId",
        "binding": "B01",
        "publishedPost": null,
        "publishedEffects": null
      },
      "fullConsumerPositive": "specified-only-blocked",
      "blockers": [
        "B01",
        "B02",
        "B03",
        "B04",
        "B05",
        "B06",
        "B07",
        "B08",
        "B09",
        "B10",
        "B11",
        "B12",
        "B13",
        "B14",
        "B15",
        "B16"
      ],
      "fullPathPrecondition": "A well-formed compatible Source6 document and matching earlier direct fields are required before B01 resolution; no such authenticated Source fixture has been supplied or executed. Local V-S0-01 tuple relation must pass; C uses exact verified B11/B16 construction, never Source replay shortcuts.",
      "plannedLiteralInspection": {
        "agreementId": {
          "wire": "Agreement",
          "sourceAstProgramId": "Agreement",
          "relation": "equal",
          "authentication": false
        },
        "actionId": {
          "wire": "Action",
          "sourceSelectedActionId": "TransferLiteralFee",
          "relation": "different",
          "authentication": false
        },
        "consequence": "Original W-D2E authorization is not a full Source/wire positive; retain B01/B04 separately from literal comparisons."
      },
      "expectedPreState": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "100"
          },
          {
            "account": "R",
            "amount": "0"
          },
          {
            "account": "F",
            "amount": "4"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "20",
            "spent": "3"
          }
        ],
        "obligations": [],
        "consumedReplay": []
      },
      "expectedCompletePostState": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "90"
          },
          {
            "account": "R",
            "amount": "10"
          },
          {
            "account": "F",
            "amount": "4"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "10",
            "spent": "13"
          }
        ],
        "obligations": [],
        "consumedReplay": [
          "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
        ]
      },
      "expectedCompleteOrderedEffects": [
        {
          "kind": "Debit",
          "account": "O",
          "asset": "A",
          "amount": "10"
        },
        {
          "kind": "Credit",
          "account": "R",
          "asset": "A",
          "amount": "10"
        },
        {
          "kind": "UseAllowance",
          "owner": "O",
          "amount": "10"
        },
        {
          "kind": "UseReplay",
          "key": "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
        },
        {
          "kind": "AdvanceHead",
          "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
          "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        }
      ],
      "expectedCompleteEffectImage": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "10"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "90"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "4",
              "after": "4"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "20",
              "spentBefore": "3",
              "remainingAfter": "10",
              "spentAfter": "13"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "oracleOrigin": "Unchanged preimplementation frozen W-D2E literal pre/post/effect/image expectations; no consumer result used",
      "pendingLedgerBinding": "B17",
      "futureLocalSuccess": {
        "status": "SemanticComparedUnqualified",
        "verifiedBindings": [
          "B01",
          "B02",
          "B03",
          "B04",
          "B05",
          "B06",
          "B07",
          "B08",
          "B09",
          "B10",
          "B11",
          "B12",
          "B13",
          "B14",
          "B15",
          "B16"
        ],
        "pendingBindings": [
          "B17"
        ],
        "publishedPost": null,
        "publishedEffects": null,
        "ledgerChange": false,
        "preconditions": "All F/D/M/H/C/I/E predicates pass under adopted B01-B16 definitions and real verified providers; no such execution exists."
      },
      "futureLocalSuccessPrecondition": "Requires new independently frozen Source/wire fixture after adopted bindings; original Action fixture cannot satisfy direct actionId equality."
    },
    {
      "id": "repay-30",
      "sourceFixture": null,
      "coreEffectFixture": "../effect-wire/fixtures.json#repay-30",
      "effectBytes": 688,
      "effectSha256": "e8b64f934305b2b4cc3a374e28dfc7fc1184f9b79da1d8d716a0179b2ec04888",
      "literalPostCondition": "principal1000->980 accrued10->0 outstanding1010->980",
      "commonLiteralPostCondition": "workRemaining2->1,workSpent7->8; append exact composite nonce; preselected55*32->aa*32 head",
      "status": "specified-only-this-sprint",
      "localPredicateExpected": {
        "core": "PreparedUnqualified",
        "imageEquality": "CommitmentEqualUnqualified",
        "authentication": false
      },
      "fullConsumerExpectedNow": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "field": "agreementId",
        "binding": "B01",
        "publishedPost": null,
        "publishedEffects": null
      },
      "fullConsumerPositive": "specified-only-blocked",
      "blockers": [
        "B01",
        "B02",
        "B03",
        "B04",
        "B05",
        "B06",
        "B07",
        "B08",
        "B09",
        "B10",
        "B11",
        "B12",
        "B13",
        "B14",
        "B15",
        "B16"
      ],
      "fullPathPrecondition": "A well-formed compatible Source6 document and matching earlier direct fields are required before B01 resolution; no such authenticated Source fixture has been supplied or executed. Local V-S0-01 tuple relation must pass; C uses exact verified B11/B16 construction, never Source replay shortcuts.",
      "plannedLiteralInspection": {
        "agreementId": {
          "wire": "Agreement",
          "sourceAstProgramId": "Agreement",
          "relation": "equal",
          "authentication": false
        },
        "actionId": {
          "wire": "Action",
          "sourceSelectedActionId": "RepayAccrualFirst",
          "relation": "different",
          "authentication": false
        },
        "consequence": "Original W-D2E authorization is not a full Source/wire positive; retain B01/B04 separately from literal comparisons."
      },
      "expectedPreState": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "100"
          },
          {
            "account": "C",
            "amount": "0"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "100",
            "spent": "0"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "1000",
            "accrued": "10",
            "outstanding": "1010",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": []
      },
      "expectedCompletePostState": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "70"
          },
          {
            "account": "C",
            "amount": "30"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "70",
            "spent": "30"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "980",
            "accrued": "0",
            "outstanding": "980",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": [
          "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
        ]
      },
      "expectedCompleteOrderedEffects": [
        {
          "kind": "Debit",
          "account": "O",
          "asset": "A",
          "amount": "30"
        },
        {
          "kind": "Credit",
          "account": "C",
          "asset": "A",
          "amount": "30"
        },
        {
          "kind": "SetObligation",
          "id": "L",
          "principal": "980",
          "accrued": "0",
          "outstanding": "980",
          "status": "Outstanding"
        },
        {
          "kind": "UseAllowance",
          "owner": "O",
          "amount": "30"
        },
        {
          "kind": "UseReplay",
          "key": "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
        },
        {
          "kind": "AdvanceHead",
          "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
          "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        }
      ],
      "expectedCompleteEffectImage": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "980",
            "accrued": "0",
            "outstanding": "980",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "30"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "70"
            },
            {
              "account": "C",
              "before": "0",
              "after": "30"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "100",
              "spentBefore": "0",
              "remainingAfter": "70",
              "spentAfter": "30"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "980",
              "accruedAfter": "0",
              "outstandingAfter": "980",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "oracleOrigin": "Unchanged preimplementation frozen W-D2E literal pre/post/effect/image expectations; no consumer result used",
      "pendingLedgerBinding": "B17",
      "futureLocalSuccess": {
        "status": "SemanticComparedUnqualified",
        "verifiedBindings": [
          "B01",
          "B02",
          "B03",
          "B04",
          "B05",
          "B06",
          "B07",
          "B08",
          "B09",
          "B10",
          "B11",
          "B12",
          "B13",
          "B14",
          "B15",
          "B16"
        ],
        "pendingBindings": [
          "B17"
        ],
        "publishedPost": null,
        "publishedEffects": null,
        "ledgerChange": false,
        "preconditions": "All F/D/M/H/C/I/E predicates pass under adopted B01-B16 definitions and real verified providers; no such execution exists."
      },
      "futureLocalSuccessPrecondition": "Requires new independently frozen Source/wire fixture after adopted bindings; original Action fixture cannot satisfy direct actionId equality."
    },
    {
      "id": "repay-accrued-only",
      "sourceFixture": null,
      "coreEffectFixture": "../effect-wire/fixtures.json#repay-accrued-only",
      "effectBytes": 688,
      "effectSha256": "90b113d7f3d3d20c1ecb093c15db710b9da19d2fa60987ac0668f60d9a2898fa",
      "literalPostCondition": "principal1000->1000 accrued10->5 outstanding1010->1005",
      "commonLiteralPostCondition": "workRemaining2->1,workSpent7->8; append exact composite nonce; preselected55*32->aa*32 head",
      "status": "specified-only-this-sprint",
      "localPredicateExpected": {
        "core": "PreparedUnqualified",
        "imageEquality": "CommitmentEqualUnqualified",
        "authentication": false
      },
      "fullConsumerExpectedNow": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "field": "agreementId",
        "binding": "B01",
        "publishedPost": null,
        "publishedEffects": null
      },
      "fullConsumerPositive": "specified-only-blocked",
      "blockers": [
        "B01",
        "B02",
        "B03",
        "B04",
        "B05",
        "B06",
        "B07",
        "B08",
        "B09",
        "B10",
        "B11",
        "B12",
        "B13",
        "B14",
        "B15",
        "B16"
      ],
      "fullPathPrecondition": "A well-formed compatible Source6 document and matching earlier direct fields are required before B01 resolution; no such authenticated Source fixture has been supplied or executed. Local V-S0-01 tuple relation must pass; C uses exact verified B11/B16 construction, never Source replay shortcuts.",
      "plannedLiteralInspection": {
        "agreementId": {
          "wire": "Agreement",
          "sourceAstProgramId": "Agreement",
          "relation": "equal",
          "authentication": false
        },
        "actionId": {
          "wire": "Action",
          "sourceSelectedActionId": "RepayAccrualFirst",
          "relation": "different",
          "authentication": false
        },
        "consequence": "Original W-D2E authorization is not a full Source/wire positive; retain B01/B04 separately from literal comparisons."
      },
      "expectedPreState": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "100"
          },
          {
            "account": "C",
            "amount": "0"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "100",
            "spent": "0"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "1000",
            "accrued": "10",
            "outstanding": "1010",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": []
      },
      "expectedCompletePostState": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "95"
          },
          {
            "account": "C",
            "amount": "5"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "95",
            "spent": "5"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "1000",
            "accrued": "5",
            "outstanding": "1005",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": [
          "[\"D\",\"O\",\"7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a\"]"
        ]
      },
      "expectedCompleteOrderedEffects": [
        {
          "kind": "Debit",
          "account": "O",
          "asset": "A",
          "amount": "5"
        },
        {
          "kind": "Credit",
          "account": "C",
          "asset": "A",
          "amount": "5"
        },
        {
          "kind": "SetObligation",
          "id": "L",
          "principal": "1000",
          "accrued": "5",
          "outstanding": "1005",
          "status": "Outstanding"
        },
        {
          "kind": "UseAllowance",
          "owner": "O",
          "amount": "5"
        },
        {
          "kind": "UseReplay",
          "key": "[\"D\",\"O\",\"7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a\"]"
        },
        {
          "kind": "AdvanceHead",
          "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
          "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        }
      ],
      "expectedCompleteEffectImage": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "5"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "5"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "1000",
            "accrued": "5",
            "outstanding": "1005",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "5"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "95"
            },
            {
              "account": "C",
              "before": "0",
              "after": "5"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "100",
              "spentBefore": "0",
              "remainingAfter": "95",
              "spentAfter": "5"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "1000",
              "accruedAfter": "5",
              "outstandingAfter": "1005",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "oracleOrigin": "Unchanged preimplementation frozen W-D2E literal pre/post/effect/image expectations; no consumer result used",
      "pendingLedgerBinding": "B17",
      "futureLocalSuccess": {
        "status": "SemanticComparedUnqualified",
        "verifiedBindings": [
          "B01",
          "B02",
          "B03",
          "B04",
          "B05",
          "B06",
          "B07",
          "B08",
          "B09",
          "B10",
          "B11",
          "B12",
          "B13",
          "B14",
          "B15",
          "B16"
        ],
        "pendingBindings": [
          "B17"
        ],
        "publishedPost": null,
        "publishedEffects": null,
        "ledgerChange": false,
        "preconditions": "All F/D/M/H/C/I/E predicates pass under adopted B01-B16 definitions and real verified providers; no such execution exists."
      },
      "futureLocalSuccessPrecondition": "Requires new independently frozen Source/wire fixture after adopted bindings; original Action fixture cannot satisfy direct actionId equality."
    },
    {
      "id": "repay-settled",
      "sourceFixture": null,
      "coreEffectFixture": "../effect-wire/fixtures.json#repay-settled",
      "effectBytes": 688,
      "effectSha256": "006f01a123e3e5b835fadd384f00ee0e4a854f62cf0b1ed1296459d0e53baa90",
      "literalPostCondition": "principal1000->0 accrued10->0 outstanding1010->0 statusSettled",
      "commonLiteralPostCondition": "workRemaining2->1,workSpent7->8; append exact composite nonce; preselected55*32->aa*32 head",
      "status": "specified-only-this-sprint",
      "localPredicateExpected": {
        "core": "PreparedUnqualified",
        "imageEquality": "CommitmentEqualUnqualified",
        "authentication": false
      },
      "fullConsumerExpectedNow": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "field": "agreementId",
        "binding": "B01",
        "publishedPost": null,
        "publishedEffects": null
      },
      "fullConsumerPositive": "specified-only-blocked",
      "blockers": [
        "B01",
        "B02",
        "B03",
        "B04",
        "B05",
        "B06",
        "B07",
        "B08",
        "B09",
        "B10",
        "B11",
        "B12",
        "B13",
        "B14",
        "B15",
        "B16"
      ],
      "fullPathPrecondition": "A well-formed compatible Source6 document and matching earlier direct fields are required before B01 resolution; no such authenticated Source fixture has been supplied or executed. Local V-S0-01 tuple relation must pass; C uses exact verified B11/B16 construction, never Source replay shortcuts.",
      "plannedLiteralInspection": {
        "agreementId": {
          "wire": "Agreement",
          "sourceAstProgramId": "Agreement",
          "relation": "equal",
          "authentication": false
        },
        "actionId": {
          "wire": "Action",
          "sourceSelectedActionId": "RepayAccrualFirst",
          "relation": "different",
          "authentication": false
        },
        "consequence": "Original W-D2E authorization is not a full Source/wire positive; retain B01/B04 separately from literal comparisons."
      },
      "expectedPreState": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "1010"
          },
          {
            "account": "C",
            "amount": "0"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "1010",
            "spent": "0"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "1000",
            "accrued": "10",
            "outstanding": "1010",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": []
      },
      "expectedCompletePostState": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "0"
          },
          {
            "account": "C",
            "amount": "1010"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "0",
            "spent": "1010"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "0",
            "accrued": "0",
            "outstanding": "0",
            "status": "Settled"
          }
        ],
        "consumedReplay": [
          "[\"D\",\"O\",\"7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b\"]"
        ]
      },
      "expectedCompleteOrderedEffects": [
        {
          "kind": "Debit",
          "account": "O",
          "asset": "A",
          "amount": "1010"
        },
        {
          "kind": "Credit",
          "account": "C",
          "asset": "A",
          "amount": "1010"
        },
        {
          "kind": "SetObligation",
          "id": "L",
          "principal": "0",
          "accrued": "0",
          "outstanding": "0",
          "status": "Settled"
        },
        {
          "kind": "UseAllowance",
          "owner": "O",
          "amount": "1010"
        },
        {
          "kind": "UseReplay",
          "key": "[\"D\",\"O\",\"7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b\"]"
        },
        {
          "kind": "AdvanceHead",
          "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
          "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        }
      ],
      "expectedCompleteEffectImage": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "1010"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "1010"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "0",
            "accrued": "0",
            "outstanding": "0",
            "status": "Settled"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "1010"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "1010",
              "after": "0"
            },
            {
              "account": "C",
              "before": "0",
              "after": "1010"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "1010",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "1010"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "0",
              "accruedAfter": "0",
              "outstandingAfter": "0",
              "statusAfter": "Settled"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "oracleOrigin": "Unchanged preimplementation frozen W-D2E literal pre/post/effect/image expectations; no consumer result used",
      "pendingLedgerBinding": "B17",
      "futureLocalSuccess": {
        "status": "SemanticComparedUnqualified",
        "verifiedBindings": [
          "B01",
          "B02",
          "B03",
          "B04",
          "B05",
          "B06",
          "B07",
          "B08",
          "B09",
          "B10",
          "B11",
          "B12",
          "B13",
          "B14",
          "B15",
          "B16"
        ],
        "pendingBindings": [
          "B17"
        ],
        "publishedPost": null,
        "publishedEffects": null,
        "ledgerChange": false,
        "preconditions": "All F/D/M/H/C/I/E predicates pass under adopted B01-B16 definitions and real verified providers; no such execution exists."
      },
      "futureLocalSuccessPrecondition": "Requires new independently frozen Source/wire fixture after adopted bindings; original Action fixture cannot satisfy direct actionId equality."
    },
    {
      "id": "repay-near-bound",
      "sourceFixture": null,
      "coreEffectFixture": "../effect-wire/fixtures.json#repay-near-bound",
      "effectBytes": 688,
      "effectSha256": "f08895084d9ec865fafdbebbcc5dda66d8e40ef961813617a677ab0232226d43",
      "literalPostCondition": "principal2^127-2 unchanged,accrued1->0; creditor and allowanceSpent reach2^128-1",
      "commonLiteralPostCondition": "workRemaining2->1,workSpent7->8; append exact composite nonce; preselected55*32->aa*32 head",
      "status": "specified-only-this-sprint",
      "localPredicateExpected": {
        "core": "PreparedUnqualified",
        "imageEquality": "CommitmentEqualUnqualified",
        "authentication": false
      },
      "fullConsumerExpectedNow": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "field": "agreementId",
        "binding": "B01",
        "publishedPost": null,
        "publishedEffects": null
      },
      "fullConsumerPositive": "specified-only-blocked",
      "blockers": [
        "B01",
        "B02",
        "B03",
        "B04",
        "B05",
        "B06",
        "B07",
        "B08",
        "B09",
        "B10",
        "B11",
        "B12",
        "B13",
        "B14",
        "B15",
        "B16"
      ],
      "fullPathPrecondition": "A well-formed compatible Source6 document and matching earlier direct fields are required before B01 resolution; no such authenticated Source fixture has been supplied or executed. Local V-S0-01 tuple relation must pass; C uses exact verified B11/B16 construction, never Source replay shortcuts.",
      "plannedLiteralInspection": {
        "agreementId": {
          "wire": "Agreement",
          "sourceAstProgramId": "Agreement",
          "relation": "equal",
          "authentication": false
        },
        "actionId": {
          "wire": "Action",
          "sourceSelectedActionId": "RepayAccrualFirst",
          "relation": "different",
          "authentication": false
        },
        "consequence": "Original W-D2E authorization is not a full Source/wire positive; retain B01/B04 separately from literal comparisons."
      },
      "expectedPreState": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "1"
          },
          {
            "account": "C",
            "amount": "340282366920938463463374607431768211454"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "1",
            "spent": "340282366920938463463374607431768211454"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "170141183460469231731687303715884105726",
            "accrued": "1",
            "outstanding": "170141183460469231731687303715884105727",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": []
      },
      "expectedCompletePostState": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "0"
          },
          {
            "account": "C",
            "amount": "340282366920938463463374607431768211455"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "0",
            "spent": "340282366920938463463374607431768211455"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "170141183460469231731687303715884105726",
            "accrued": "0",
            "outstanding": "170141183460469231731687303715884105726",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": [
          "[\"D\",\"O\",\"7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c\"]"
        ]
      },
      "expectedCompleteOrderedEffects": [
        {
          "kind": "Debit",
          "account": "O",
          "asset": "A",
          "amount": "1"
        },
        {
          "kind": "Credit",
          "account": "C",
          "asset": "A",
          "amount": "1"
        },
        {
          "kind": "SetObligation",
          "id": "L",
          "principal": "170141183460469231731687303715884105726",
          "accrued": "0",
          "outstanding": "170141183460469231731687303715884105726",
          "status": "Outstanding"
        },
        {
          "kind": "UseAllowance",
          "owner": "O",
          "amount": "1"
        },
        {
          "kind": "UseReplay",
          "key": "[\"D\",\"O\",\"7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c\"]"
        },
        {
          "kind": "AdvanceHead",
          "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
          "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        }
      ],
      "expectedCompleteEffectImage": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "170141183460469231731687303715884105726",
            "accrued": "0",
            "outstanding": "170141183460469231731687303715884105726",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "1"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "1",
              "after": "0"
            },
            {
              "account": "C",
              "before": "340282366920938463463374607431768211454",
              "after": "340282366920938463463374607431768211455"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "1",
              "spentBefore": "340282366920938463463374607431768211454",
              "remainingAfter": "0",
              "spentAfter": "340282366920938463463374607431768211455"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "170141183460469231731687303715884105726",
              "accruedBefore": "1",
              "outstandingBefore": "170141183460469231731687303715884105727",
              "statusBefore": "Outstanding",
              "principalAfter": "170141183460469231731687303715884105726",
              "accruedAfter": "0",
              "outstandingAfter": "170141183460469231731687303715884105726",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "oracleOrigin": "Unchanged preimplementation frozen W-D2E literal pre/post/effect/image expectations; no consumer result used",
      "pendingLedgerBinding": "B17",
      "futureLocalSuccess": {
        "status": "SemanticComparedUnqualified",
        "verifiedBindings": [
          "B01",
          "B02",
          "B03",
          "B04",
          "B05",
          "B06",
          "B07",
          "B08",
          "B09",
          "B10",
          "B11",
          "B12",
          "B13",
          "B14",
          "B15",
          "B16"
        ],
        "pendingBindings": [
          "B17"
        ],
        "publishedPost": null,
        "publishedEffects": null,
        "ledgerChange": false,
        "preconditions": "All F/D/M/H/C/I/E predicates pass under adopted B01-B16 definitions and real verified providers; no such execution exists."
      },
      "futureLocalSuccessPrecondition": "Requires new independently frozen Source/wire fixture after adopted bindings; original Action fixture cannot satisfy direct actionId equality."
    }
  ],
  "hostileCases": [
    {
      "id": "wire-trailing",
      "phase": "wire",
      "mutation": "Append byte00 to valid canonical wire",
      "earlierGatePrecondition": "Existing decoder only; no Source input is parsed.",
      "expected": {
        "status": "WireRejected",
        "code": "TRAILING",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "wire-unknown-field-tag",
      "phase": "wire",
      "mutation": "Replace required tag26 withff",
      "earlierGatePrecondition": "Prior wire bytes canonical and bounded.",
      "expected": {
        "status": "WireRejected",
        "code": "FIELD_TAG",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "wire-unknown-operation",
      "phase": "wire",
      "mutation": "Replace operation kind byte with03",
      "earlierGatePrecondition": "Prior wire fields canonical.",
      "expected": {
        "status": "WireRejected",
        "code": "OPERATION_TAG",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "wire-wrong-version",
      "phase": "wire",
      "mutation": "Change header /3 to /4",
      "earlierGatePrecondition": "Bounded Uint8Array input.",
      "expected": {
        "status": "WireRejected",
        "code": "HEADER",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "source-wrong-profile",
      "phase": "source",
      "mutation": "Use profile /5",
      "earlierGatePrecondition": "Valid canonical /3 wire, then Source parser.",
      "expected": {
        "status": "SourceRejected",
        "code": "SOURCE6_VERSION",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null,
        "offset": 8
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions",
      "sourceByteFixture": "profile \"moriarty-financial-agreement-source/5\";",
      "offsetOracle": "Literal ASCII profile prefix is8 bytes; parser rejects profile token before agreement body is required."
    },
    {
      "id": "source-scale19",
      "phase": "source",
      "mutation": "Use settlement scale19",
      "earlierGatePrecondition": "Wire scale19 is codec valid; Source parser rejects before adapter.",
      "expected": {
        "status": "SourceRejected",
        "code": "SOURCE6_RANGE",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null,
        "offset": "exact original Source6Error.offset; no new source byte fixture exists, so numeric offset remains specified-only"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "source-mint",
      "phase": "source",
      "mutation": "Use signed_action mint",
      "earlierGatePrecondition": "Valid prior Source fields.",
      "expected": {
        "status": "SourceRejected",
        "code": "SOURCE6_UNKNOWN_TAG",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null,
        "offset": "exact original Source6Error.offset; no new source byte fixture exists, so numeric offset remains specified-only"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "source-alias",
      "phase": "source",
      "mutation": "Set owner=recipient in a complete Source document",
      "earlierGatePrecondition": "Otherwise valid Source shape; formation prevents Core alias test.",
      "expected": {
        "status": "SourceRejected",
        "code": "SOURCE6_CELL_SHAPE",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null,
        "offset": "exact original Source6Error.offset; no new source byte fixture exists, so numeric offset remains specified-only"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "wire-only-domain",
      "phase": "domain",
      "mutation": "Wire domain midnight:preview; no Source identifier renaming",
      "earlierGatePrecondition": "No positive provider or full consumer exists; test only named predicate in isolation.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "domain",
        "sourcePath": null,
        "inputPath": "wire.domain"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "wire-only-account",
      "phase": "domain",
      "mutation": "Wire owner contains slash or initial digit",
      "earlierGatePrecondition": "No positive provider or full consumer exists; test only named predicate in isolation.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "operation.owner",
        "sourcePath": null,
        "inputPath": "wire.operation.owner"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "opaque-head-label",
      "phase": "domain",
      "mutation": "Source uses h0/h1 instead of exact wire hash presentations",
      "earlierGatePrecondition": "No positive provider or full consumer exists; test only named predicate in isolation.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "preHead",
        "sourcePath": "intent.preHead"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "source-round-over-u64",
      "phase": "domain",
      "mutation": "Source round2^64 is valid Source/Core but outside wire/image UInt64 intersection",
      "earlierGatePrecondition": "No positive provider or full consumer exists; test only named predicate in isolation.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "round",
        "sourcePath": "authenticated.round"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "agreement-unbound",
      "phase": "missing-binding",
      "mutation": "Literal wire.agreementId==AST.programId can match; matching agreement name does not establish authenticated B01 instance binding",
      "earlierGatePrecondition": "F-wire/F-source and full D sweep pass; all earlier M tags/literal checks and anchored providers pass; exact target B01 is unavailable. B17 is not required.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "agreementId",
        "binding": "B01",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "stage-unbound",
      "phase": "missing-binding",
      "mutation": "No Source/Core stage identity slot",
      "earlierGatePrecondition": "F-wire/F-source and full D sweep pass; all earlier M tags/literal checks and anchored providers pass; exact target B02 is unavailable. B17 is not required.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "stageId",
        "binding": "B02",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "episode-unbound",
      "phase": "missing-binding",
      "mutation": "No Source/Core episode identity slot",
      "earlierGatePrecondition": "F-wire/F-source and full D sweep pass; all earlier M tags/literal checks and anchored providers pass; exact target B03 is unavailable. B17 is not required.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "episodeId",
        "binding": "B03",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "action-program-unbound",
      "phase": "missing-binding",
      "mutation": "Literal wire.actionId==AST.selected.actionId can be compared; separate authenticated action/coreProgram relation remains unselected even when literals match",
      "earlierGatePrecondition": "F-wire/F-source and full D sweep pass; all earlier M tags/literal checks and anchored providers pass; exact target B04 is unavailable. B17 is not required.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "actionId",
        "binding": "B04",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "source-image-unbound",
      "phase": "missing-binding",
      "mutation": "No canonical source image; embedded source_hash cannot be full-document hash without self-reference",
      "earlierGatePrecondition": "F-wire/F-source and full D sweep pass; all earlier M tags/literal checks and anchored providers pass; exact target B05 is unavailable. B17 is not required.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "sourceHash",
        "binding": "B05",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "core-image-unbound",
      "phase": "missing-binding",
      "mutation": "No canonical Core selected-code hash image",
      "earlierGatePrecondition": "F-wire/F-source and full D sweep pass; all earlier M tags/literal checks and anchored providers pass; exact target B06 is unavailable. B17 is not required.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "coreHash",
        "binding": "B06",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "policy-image-unbound",
      "phase": "missing-binding",
      "mutation": "Opaque selected digest has no selected policy image",
      "earlierGatePrecondition": "F-wire/F-source and full D sweep pass; all earlier M tags/literal checks and anchored providers pass; exact target B07 is unavailable. B17 is not required.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "policyHash",
        "binding": "B07",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "keyref-unbound",
      "phase": "missing-binding",
      "mutation": "Opaque keyRef cannot be assumed raw x-only key hex",
      "earlierGatePrecondition": "F-wire/F-source and full D sweep pass; all earlier M tags/literal checks and anchored providers pass; exact target B08 is unavailable. B17 is not required.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "signer",
        "binding": "B08",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "signature-unverified",
      "phase": "missing-binding",
      "mutation": "No verifier in current Source/Core/effect path",
      "earlierGatePrecondition": "F-wire/F-source and full D sweep pass; all earlier M tags/literal checks and anchored providers pass; exact target B09 is unavailable. B17 is not required.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "keyScheme",
        "binding": "B09",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "scale-unbound",
      "phase": "missing-binding",
      "mutation": "Matching numeral2 cannot establish registered atomic-unit scale",
      "earlierGatePrecondition": "F-wire/F-source and full D sweep pass; all earlier M tags/literal checks and anchored providers pass; exact target B10 is unavailable. B17 is not required.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "asset",
        "binding": "B10",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "snapshot-unverified",
      "phase": "missing-binding",
      "mutation": "Caller authenticated block is not snapshot authentication",
      "earlierGatePrecondition": "F-wire/F-source and full D sweep pass; all earlier M tags/literal checks and anchored providers pass; exact target B11 is unavailable. B17 is not required.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "preHead",
        "binding": "B11",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "predecessor-unbound",
      "phase": "missing-binding",
      "mutation": "Field19 prior linkage is absent from Core and image",
      "earlierGatePrecondition": "F-wire/F-source and full D sweep pass; all earlier M tags/literal checks and anchored providers pass; exact target B12 is unavailable. B17 is not required.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "predecessor",
        "binding": "B12",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "replay-unused-unverified",
      "phase": "missing-binding",
      "mutation": "Empty caller replay array is not unused-key evidence",
      "earlierGatePrecondition": "F-wire/F-source and full D sweep pass; all earlier M tags/literal checks and anchored providers pass; exact target B13 is unavailable. B17 is not required.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "nonce",
        "binding": "B13",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "authority-unverified",
      "phase": "missing-binding",
      "mutation": "Caller allowance/work counters are not authenticated grant",
      "earlierGatePrecondition": "F-wire/F-source and full D sweep pass; all earlier M tags/literal checks and anchored providers pass; exact target B14 is unavailable. B17 is not required.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "grossCap",
        "binding": "B14",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "headextension-unverified",
      "phase": "missing-binding",
      "mutation": "Different head string or matching stipulation does not establish extension",
      "earlierGatePrecondition": "F-wire/F-source and full D sweep pass; all earlier M tags/literal checks and anchored providers pass; exact target B15 is unavailable. B17 is not required.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "successor",
        "binding": "B15",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "replayhistory-unmapped",
      "phase": "missing-binding",
      "mutation": "Source unused/consumed selected-key claim does not transport complete history",
      "earlierGatePrecondition": "F-wire/F-source, complete D sweep, V-S0-01, all M signed tags/subfields, unsigned snapshot tail and B15 pass; B16 complete-history provider is unavailable. B17 is not required and C has not run.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "replayBefore",
        "binding": "B16",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "ledger-consumption-unavailable",
      "phase": "optional-ledger-mode",
      "mutation": "Local effects/hash equality does not atomically compare and consume",
      "earlierGatePrecondition": "F through E local comparison already succeeds unqualified with B17 pending; only separate requested L mode attempts actual consumption.",
      "expected": {
        "status": "LedgerRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "effectCommitment",
        "binding": "B17",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions",
      "localComparisonExpected": {
        "status": "SemanticComparedUnqualified",
        "verifiedBindings": [
          "B01",
          "B02",
          "B03",
          "B04",
          "B05",
          "B06",
          "B07",
          "B08",
          "B09",
          "B10",
          "B11",
          "B12",
          "B13",
          "B14",
          "B15",
          "B16"
        ],
        "pendingBindings": [
          "B17"
        ],
        "publishedPost": null,
        "publishedEffects": null,
        "ledgerChange": false,
        "preconditions": "All F/D/M/H/C/I/E predicates pass under adopted B01-B16 definitions and real verified providers; no such execution exists."
      }
    },
    {
      "id": "domain-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire D vs Source X",
      "earlierGatePrecondition": "F/D pass; M tags before target and their providers pass; this target direct literal differs before its binding check. B17 pending does not prevent this check.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "domain",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "signer-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire O vs Source X",
      "earlierGatePrecondition": "F/D pass; M tags before target and their providers pass; this target direct literal differs before its binding check. B17 pending does not prevent this check.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "signer",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "nonce-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire77*32 vs Source78*32",
      "earlierGatePrecondition": "F/D pass; M tags before target and their providers pass; this target direct literal differs before its binding check. B17 pending does not prevent this check.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "nonce",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "prehead-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire preHead55*32 versus Source intent.preHead bb*32; authenticated.head may also differ, but intent comparison is first",
      "earlierGatePrecondition": "F/D, V-S0-01 and M tags1-17 pass; tag18 first Source literal intent.preHead differs before authenticated.head comparison or B11 provider lookup.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "preHead",
        "sourcePath": "intent.preHead",
        "comparisonTag": 18
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "validfrom-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire100 vs Source101",
      "earlierGatePrecondition": "F/D pass; M tags before target and their providers pass; this target direct literal differs before its binding check. B17 pending does not prevent this check.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "validFrom",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "validuntil-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire200 vs Source201",
      "earlierGatePrecondition": "F/D pass; M tags before target and their providers pass; this target direct literal differs before its binding check. B17 pending does not prevent this check.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "validUntil",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "grosscap-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire11 vs Source12",
      "earlierGatePrecondition": "F/D pass; M tags before target and their providers pass; this target direct literal differs before its binding check. B17 pending does not prevent this check.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "grossCap",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "feecap-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire1 vs Source2",
      "earlierGatePrecondition": "F/D pass; M tags before target and their providers pass; this target direct literal differs before its binding check. B17 pending does not prevent this check.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "feeCap",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "netfloor-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire10 vs Source9",
      "earlierGatePrecondition": "F/D pass; M tags before target and their providers pass; this target direct literal differs before its binding check. B17 pending does not prevent this check.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "netFloor",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "asset-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire A vs Source B",
      "earlierGatePrecondition": "F/D pass; M tags before target and their providers pass; this target direct literal differs before its binding check. B17 pending does not prevent this check.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "asset",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "owner-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Signed wire owner X vs Source signed owner O",
      "earlierGatePrecondition": "F/D pass; M tags before target and their providers pass; this target direct literal differs before its binding check. B17 pending does not prevent this check.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "operation.owner",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "recipient-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Signed wire R vs Source signed X",
      "earlierGatePrecondition": "F/D pass; M tags before target and their providers pass; this target direct literal differs before its binding check. B17 pending does not prevent this check.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "operation.recipient",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "zero-fee-recipient-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Fee0, signed wire F vs Source X must still differ",
      "earlierGatePrecondition": "F/D pass; M tags before target and their providers pass; this target direct literal differs before its binding check. B17 pending does not prevent this check.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "operation.feeRecipient",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "amount-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Signed amount10 vs Source11",
      "earlierGatePrecondition": "F/D pass; M tags before target and their providers pass; this target direct literal differs before its binding check. B17 pending does not prevent this check.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "operation.amount",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "fee-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Signed fee1 vs Source0",
      "earlierGatePrecondition": "F/D pass; M tags before target and their providers pass; this target direct literal differs before its binding check. B17 pending does not prevent this check.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "operation.fee",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "repay-id-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Signed L vs snapshot/source M",
      "earlierGatePrecondition": "F/D pass; M tags before target and their providers pass; this target direct literal differs before its binding check. B17 pending does not prevent this check.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "operation.obligationId",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "repay-payer-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire payer X vs signed Source payer O",
      "earlierGatePrecondition": "F/D pass; M tags before target and their providers pass; this target direct literal differs before its binding check. B17 pending does not prevent this check.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "operation.payer",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "repay-debtor-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire debtor X vs bound snapshot debtor O",
      "earlierGatePrecondition": "F/D and M prior tags pass; B11/tag18 authenticates complete obligation without comparing future wire debtor/creditor fields. Current tag35 compares signed subfield to Source claim then retained authentic obligation fact.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "operation.debtor",
        "sourcePath": null,
        "comparisonTag": 35,
        "retainedFactBinding": "B11"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "repay-creditor-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire creditor X vs bound snapshot creditor C; never replace C",
      "earlierGatePrecondition": "F/D and M prior tags pass; B11/tag18 authenticates complete obligation without comparing future wire debtor/creditor fields. Current tag35 compares signed subfield to Source claim then retained authentic obligation fact.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "operation.creditor",
        "sourcePath": null,
        "comparisonTag": 35,
        "retainedFactBinding": "B11"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "predecessor-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire66*32 vs verified snapshot predecessorbb*32",
      "earlierGatePrecondition": "F/D and M through18 pass; B11 retains authentic prior predecessorbb without comparing future tag19. Source prior-head claim66 matches wire66; B12 linkage fact authenticatesbb, then current tag19 fact equality fails.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "predecessor",
        "sourcePath": null,
        "comparisonTag": 19,
        "retainedFactBinding": "B12"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "core-changed-debit",
      "phase": "existing-Core-predicate",
      "mutation": "Supply Debit12 while signed transfer10 fee1 remains",
      "earlierGatePrecondition": "Existing direct Core predicate in isolation only. In a future outer consumer, unavailable/invalid B01-B16 or H bounds can reject earlier; do not claim this Core code as full-path first failure.",
      "expected": {
        "status": "CoreRejected",
        "code": "S0_EFFECT_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "judgment": "effect",
        "diagnosticWork": 1,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "isolated-existing-module",
      "sameEffectsMustPass": false,
      "submittedEffectsPrecondition": "Named existing direct Core predicate only; no outer verified binding sequence is asserted."
    },
    {
      "id": "core-line-order",
      "phase": "existing-Core-predicate",
      "mutation": "Reverse complete ordered effects",
      "earlierGatePrecondition": "Existing direct Core predicate in isolation only. In a future outer consumer, unavailable/invalid B01-B16 or H bounds can reject earlier; do not claim this Core code as full-path first failure.",
      "expected": {
        "status": "CoreRejected",
        "code": "S0_EFFECT_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "judgment": "effect",
        "diagnosticWork": 1,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "isolated-existing-module",
      "sameEffectsMustPass": false,
      "submittedEffectsPrecondition": "Named existing direct Core predicate only; no outer verified binding sequence is asserted."
    },
    {
      "id": "core-extra-zero-fee",
      "phase": "existing-Core-predicate",
      "mutation": "Supply extra fee Credit0 in transfer fee0",
      "earlierGatePrecondition": "Existing direct Core predicate in isolation only. In a future outer consumer, unavailable/invalid B01-B16 or H bounds can reject earlier; do not claim this Core code as full-path first failure.",
      "expected": {
        "status": "CoreRejected",
        "code": "S0_EFFECT_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "judgment": "effect",
        "diagnosticWork": 1,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "isolated-existing-module",
      "sameEffectsMustPass": false,
      "submittedEffectsPrecondition": "Named existing direct Core predicate only; no outer verified binding sequence is asserted."
    },
    {
      "id": "core-rehashed-hostile",
      "phase": "existing-Core-predicate",
      "mutation": "Supply Debit12 and matching hostile field26 SHA; signed operation remains10+1",
      "earlierGatePrecondition": "Existing direct Core predicate in isolation only. In a future outer consumer, unavailable/invalid B01-B16 or H bounds can reject earlier; do not claim this Core code as full-path first failure.",
      "expected": {
        "status": "CoreRejected",
        "code": "S0_EFFECT_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "judgment": "effect",
        "diagnosticWork": 1,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "isolated-existing-module",
      "sameEffectsMustPass": false,
      "submittedEffectsPrecondition": "Named existing direct Core predicate only; no outer verified binding sequence is asserted."
    },
    {
      "id": "core-repay-allocation",
      "phase": "existing-Core-predicate",
      "mutation": "Repay30 against1000+10 but supply principal990 accrued0",
      "earlierGatePrecondition": "Existing direct Core predicate in isolation only. In a future outer consumer, unavailable/invalid B01-B16 or H bounds can reject earlier; do not claim this Core code as full-path first failure.",
      "expected": {
        "status": "CoreRejected",
        "code": "S0_EFFECT_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "judgment": "effect",
        "diagnosticWork": 1,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "isolated-existing-module",
      "sameEffectsMustPass": false,
      "submittedEffectsPrecondition": "Named existing direct Core predicate only; no outer verified binding sequence is asserted."
    },
    {
      "id": "core-missing-credit",
      "phase": "existing-Core-predicate",
      "mutation": "Remove funded creditor Credit from repayment",
      "earlierGatePrecondition": "Existing direct Core predicate in isolation only. In a future outer consumer, unavailable/invalid B01-B16 or H bounds can reject earlier; do not claim this Core code as full-path first failure.",
      "expected": {
        "status": "CoreRejected",
        "code": "S0_EFFECT_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "judgment": "effect",
        "diagnosticWork": 1,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "isolated-existing-module",
      "sameEffectsMustPass": false,
      "submittedEffectsPrecondition": "Named existing direct Core predicate only; no outer verified binding sequence is asserted."
    },
    {
      "id": "core-insufficient-balance",
      "phase": "existing-Core-predicate",
      "mutation": "Transfer11 with owner balance10",
      "earlierGatePrecondition": "Existing direct Core predicate in isolation only. In a future outer consumer, unavailable/invalid B01-B16 or H bounds can reject earlier; do not claim this Core code as full-path first failure.",
      "expected": {
        "status": "CoreRejected",
        "code": "S0_EFFECT_RANGE",
        "publishedPost": null,
        "publishedEffects": null,
        "judgment": "effect",
        "diagnosticWork": 1,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "isolated-existing-module",
      "sameEffectsMustPass": false,
      "submittedEffectsPrecondition": "Named existing direct Core predicate only; no outer verified binding sequence is asserted."
    },
    {
      "id": "core-insufficient-allowance",
      "phase": "existing-Core-predicate",
      "mutation": "Transfer11 with allowance remaining10",
      "earlierGatePrecondition": "Existing direct Core predicate in isolation only. In a future outer consumer, unavailable/invalid B01-B16 or H bounds can reject earlier; do not claim this Core code as full-path first failure.",
      "expected": {
        "status": "CoreRejected",
        "code": "S0_AUTH_SCOPE",
        "publishedPost": null,
        "publishedEffects": null,
        "judgment": "authority",
        "diagnosticWork": 1,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "isolated-existing-module",
      "sameEffectsMustPass": true,
      "submittedEffectsPrecondition": "Exact internally expected effect lines for the mutated state/intent/successor must be supplied; all earlier Core judgments pass."
    },
    {
      "id": "core-work-zero",
      "phase": "existing-Core-predicate",
      "mutation": "workRemaining0",
      "earlierGatePrecondition": "Existing direct Core predicate in isolation only. In a future outer consumer, unavailable/invalid B01-B16 or H bounds can reject earlier; do not claim this Core code as full-path first failure.",
      "expected": {
        "status": "CoreRejected",
        "code": "S0_AUTH_SCOPE",
        "publishedPost": null,
        "publishedEffects": null,
        "judgment": "authority",
        "diagnosticWork": 1,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "isolated-existing-module",
      "sameEffectsMustPass": true,
      "submittedEffectsPrecondition": "Exact internally expected effect lines for the mutated state/intent/successor must be supplied; all earlier Core judgments pass."
    },
    {
      "id": "core-stale-head",
      "phase": "existing-Core-predicate",
      "mutation": "state.headbb*32, intent.preHead55*32, matching submitted55 predecessor",
      "earlierGatePrecondition": "Existing direct Core predicate in isolation only. In a future outer consumer, unavailable/invalid B01-B16 or H bounds can reject earlier; do not claim this Core code as full-path first failure.",
      "expected": {
        "status": "CoreRejected",
        "code": "S0_HISTORY_STALE",
        "publishedPost": null,
        "publishedEffects": null,
        "judgment": "history",
        "diagnosticWork": 1,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "isolated-existing-module",
      "sameEffectsMustPass": true,
      "submittedEffectsPrecondition": "Exact internally expected effect lines for the mutated state/intent/successor must be supplied; all earlier Core judgments pass."
    },
    {
      "id": "core-used-replay",
      "phase": "existing-Core-predicate",
      "mutation": "state.consumedReplay already includes exact selected tuple",
      "earlierGatePrecondition": "Existing direct Core predicate in isolation only. In a future outer consumer, unavailable/invalid B01-B16 or H bounds can reject earlier; do not claim this Core code as full-path first failure.",
      "expected": {
        "status": "CoreRejected",
        "code": "S0_HISTORY_REPLAY",
        "publishedPost": null,
        "publishedEffects": null,
        "judgment": "history",
        "diagnosticWork": 1,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "isolated-existing-module",
      "sameEffectsMustPass": true,
      "submittedEffectsPrecondition": "Exact internally expected effect lines for the mutated state/intent/successor must be supplied; all earlier Core judgments pass."
    },
    {
      "id": "core-same-successor",
      "phase": "existing-Core-predicate",
      "mutation": "proposed successor=preHead and submittedAdvanceHead agrees",
      "earlierGatePrecondition": "Existing direct Core predicate in isolation only. In a future outer consumer, unavailable/invalid B01-B16 or H bounds can reject earlier; do not claim this Core code as full-path first failure.",
      "expected": {
        "status": "CoreRejected",
        "code": "S0_HISTORY_SUCCESSOR",
        "publishedPost": null,
        "publishedEffects": null,
        "judgment": "history",
        "diagnosticWork": 1,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "isolated-existing-module",
      "sameEffectsMustPass": true,
      "submittedEffectsPrecondition": "Exact internally expected effect lines for the mutated state/intent/successor must be supplied; all earlier Core judgments pass."
    },
    {
      "id": "core-failure",
      "phase": "existing-Core-predicate",
      "mutation": "requestedOutcome RequestedFailure with otherwise valid direct Core input",
      "earlierGatePrecondition": "Existing direct Core predicate in isolation only. In a future outer consumer, unavailable/invalid B01-B16 or H bounds can reject earlier; do not claim this Core code as full-path first failure.",
      "expected": {
        "status": "CoreRejected",
        "code": "S0_FAILURE_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "judgment": "failure",
        "diagnosticWork": 1,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "isolated-existing-module",
      "sameEffectsMustPass": true,
      "submittedEffectsPrecondition": "Exact internally expected effect lines for the mutated state/intent/successor must be supplied; all earlier Core judgments pass."
    },
    {
      "id": "core-wrong-debtor",
      "phase": "existing-Core-predicate",
      "mutation": "Repay state obligation debtor X while intent signer O",
      "earlierGatePrecondition": "Existing direct Core predicate in isolation only. In a future outer consumer, unavailable/invalid B01-B16 or H bounds can reject earlier; do not claim this Core code as full-path first failure.",
      "expected": {
        "status": "CoreRejected",
        "code": "S0_STAGE_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "judgment": "stage",
        "diagnosticWork": 1,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "isolated-existing-module",
      "sameEffectsMustPass": false,
      "submittedEffectsPrecondition": "Named existing direct Core predicate only; no outer verified binding sequence is asserted."
    },
    {
      "id": "image-omitted-fee-cell",
      "phase": "conditional-derived-image-predicate",
      "mutation": "Drop unchanged zero-fee recipient balance footprint",
      "earlierGatePrecondition": "F/D/M with B01-B16 verified and H pass; C returns PreparedUnqualified with sameEffects passed; schema/earlier I checks pass as required. B17 remains pending and is not a rejection prerequisite.",
      "expected": {
        "status": "EffectRejected",
        "code": "W_D2F_PREPARED_IMAGE_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "image-changed-before",
      "phase": "conditional-derived-image-predicate",
      "mutation": "Change owner balance before100 to101",
      "earlierGatePrecondition": "F/D/M with B01-B16 verified and H pass; C returns PreparedUnqualified with sameEffects passed; schema/earlier I checks pass as required. B17 remains pending and is not a rejection prerequisite.",
      "expected": {
        "status": "EffectRejected",
        "code": "W_D2F_PREPARED_IMAGE_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "image-changed-after",
      "phase": "conditional-derived-image-predicate",
      "mutation": "Change owner balance after89 to88",
      "earlierGatePrecondition": "F/D/M with B01-B16 verified and H pass; C returns PreparedUnqualified with sameEffects passed; schema/earlier I checks pass as required. B17 remains pending and is not a rejection prerequisite.",
      "expected": {
        "status": "EffectRejected",
        "code": "W_D2F_PREPARED_IMAGE_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "image-allowance-reset",
      "phase": "conditional-derived-image-predicate",
      "mutation": "Set spentAfter0 instead of11",
      "earlierGatePrecondition": "F/D/M with B01-B16 verified and H pass; C returns PreparedUnqualified with sameEffects passed; schema/earlier I checks pass as required. B17 remains pending and is not a rejection prerequisite.",
      "expected": {
        "status": "EffectRejected",
        "code": "W_D2F_PREPARED_IMAGE_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "image-work-reset",
      "phase": "conditional-derived-image-predicate",
      "mutation": "Set workSpentAfter0 instead of8",
      "earlierGatePrecondition": "F/D/M with B01-B16 verified and H pass; C returns PreparedUnqualified with sameEffects passed; schema/earlier I checks pass as required. B17 remains pending and is not a rejection prerequisite.",
      "expected": {
        "status": "EffectRejected",
        "code": "W_D2F_PREPARED_IMAGE_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "image-obligation-debtor",
      "phase": "conditional-derived-image-predicate",
      "mutation": "Change obligation footprint debtor",
      "earlierGatePrecondition": "F/D/M with B01-B16 verified and H pass; C returns PreparedUnqualified with sameEffects passed; schema/earlier I checks pass as required. B17 remains pending and is not a rejection prerequisite.",
      "expected": {
        "status": "EffectRejected",
        "code": "W_D2F_PREPARED_IMAGE_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "image-root-head",
      "phase": "conditional-derived-image-predicate",
      "mutation": "Change root preHead while derived Core head remains",
      "earlierGatePrecondition": "F/D/M with B01-B16 verified and H pass; C returns PreparedUnqualified with sameEffects passed; schema/earlier I checks pass as required. B17 remains pending and is not a rejection prerequisite.",
      "expected": {
        "status": "EffectRejected",
        "code": "W_D2F_PREPARED_IMAGE_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "image-replay-omission",
      "phase": "conditional-derived-image-predicate",
      "mutation": "Omit unrelated authenticated replay history",
      "earlierGatePrecondition": "F/D/M with B01-B16 verified and H pass; C returns PreparedUnqualified with sameEffects passed; schema/earlier I checks pass as required. B17 remains pending and is not a rejection prerequisite.",
      "expected": {
        "status": "EffectRejected",
        "code": "W_D2F_PREPARED_IMAGE_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "image-footprint-order",
      "phase": "conditional-derived-image-predicate",
      "mutation": "Reorder complete balance rows",
      "earlierGatePrecondition": "F/D/M with B01-B16 verified and H pass; C returns PreparedUnqualified with sameEffects passed; schema/earlier I checks pass as required. B17 remains pending and is not a rejection prerequisite.",
      "expected": {
        "status": "EffectRejected",
        "code": "W_D2F_PREPARED_IMAGE_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "image-rehashed-omission",
      "phase": "conditional-derived-image-predicate",
      "mutation": "Omit required cell and rehash field26; image completeness mismatch still rejects",
      "earlierGatePrecondition": "F/D/M with B01-B16 verified and H pass; C returns PreparedUnqualified with sameEffects passed; schema/earlier I checks pass as required. B17 remains pending and is not a rejection prerequisite.",
      "expected": {
        "status": "EffectRejected",
        "code": "W_D2F_PREPARED_IMAGE_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "field26-only-mismatch",
      "phase": "conditional-commitment-predicate",
      "mutation": "Keep exact complete derived image, change field26 toff*32",
      "earlierGatePrecondition": "F/D/M with B01-B16 verified and H pass; C returns PreparedUnqualified with sameEffects passed; schema/earlier I checks pass as required. B17 remains pending and is not a rejection prerequisite.",
      "expected": {
        "status": "EffectRejected",
        "code": "EFFECT_COMMITMENT_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "invalid-signature",
      "phase": "conditional-verifier-predicate",
      "mutation": "A real selected verifier rejects the signature under later adopted B09 preprocessing; no raw digest or exact-message byte convention is assumed.",
      "earlierGatePrecondition": "F/D and M tags1-13 pass with authenticated B08 authorized key retained; adopted B09 preprocessing/verifier is available and reports failure for its declared verification fact. No B09 implementation exists today.",
      "expected": {
        "status": "EvidenceRejected",
        "code": "W_D2F_EVIDENCE_INVALID",
        "publishedPost": null,
        "publishedEffects": null,
        "binding": "B09",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "wrong-snapshot",
      "phase": "conditional-verifier-predicate",
      "mutation": "Snapshot proof does not authenticate the declared snapshot fact (invalid proof/evidence), rather than a genuine proof of a different head.",
      "earlierGatePrecondition": "F/D and prior M tags pass; both Source head literals match wire preHead; B11 fact authentication fails before its current-tag equality check.",
      "expected": {
        "status": "EvidenceRejected",
        "code": "W_D2F_EVIDENCE_INVALID",
        "publishedPost": null,
        "publishedEffects": null,
        "binding": "B11",
        "sourcePath": null,
        "field": "preHead",
        "comparisonTag": 18
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "digest-derived-successor",
      "phase": "conditional-verifier-predicate",
      "mutation": "Select successor from authorization digest then claim independence",
      "earlierGatePrecondition": "F and full D pass; every M predicate before this exact anchor passes; a real verifier with adopted preprocessing/relation is present. No provider exists today; specified-only.",
      "expected": {
        "status": "EvidenceRejected",
        "code": "W_D2F_EVIDENCE_INVALID",
        "publishedPost": null,
        "publishedEffects": null,
        "binding": "B15",
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "replay-bound-pre17",
      "phase": "H",
      "mutation": "Complete authenticated pre17 distinct unused histories",
      "earlierGatePrecondition": "F/D/M with B01-B16 verified pass; authoritative complete history; H before C. Never truncate/deduplicate and never WireRejected.",
      "expected": {
        "status": "EffectDomainRejected",
        "code": "W_D2F_REPLAY_HISTORY_BOUND",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "consumption.replayBefore",
        "count": 17,
        "cap": 16,
        "imageSide": "derived"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "replay-bound-post17",
      "phase": "H",
      "mutation": "Complete authenticated pre16 distinct keys excluding selected nonce; append selected tuple -> post17",
      "earlierGatePrecondition": "F/D/M with B01-B16 verified pass; authoritative complete history; H before C. Never truncate/deduplicate and never WireRejected.",
      "expected": {
        "status": "EffectDomainRejected",
        "code": "W_D2F_REPLAY_HISTORY_BOUND",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "consumption.replayAfter",
        "count": 17,
        "cap": 16,
        "imageSide": "derived"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "supplied-replay-bound17",
      "phase": "I",
      "mutation": "Shaped supplied image consumption.replayBefore has17 entries",
      "earlierGatePrecondition": "F/D/M/H/C pass; supplied image object/replay array shape valid; replay cap checked before generic effect codec at I.",
      "expected": {
        "status": "EffectDomainRejected",
        "code": "W_D2F_REPLAY_HISTORY_BOUND",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "consumption.replayBefore",
        "count": 17,
        "cap": 16,
        "imageSide": "supplied"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "late-domain-beats-b01",
      "phase": "D",
      "mutation": "Wire operation.owner=O/path and B01 unavailable",
      "earlierGatePrecondition": "Wire and Source formation pass; domain subtype failure in late signed operation, full D sweep precedes any M provider lookup.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "operation.owner",
        "sourcePath": null,
        "inputPath": "wire.operation.owner"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "late-mismatch-does-not-beat-b01",
      "phase": "M",
      "mutation": "Domain-valid operation.recipient=X while Source R; agreement literals match but B01 unavailable",
      "earlierGatePrecondition": "All F/D checks pass; M reaches tag3 missing B01 before tag35 recipient difference.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "agreementId",
        "binding": "B01"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "agreement-literal-before-provider",
      "phase": "M",
      "mutation": "Wire agreementId=Other vs Source Agreement, and B01 unavailable",
      "earlierGatePrecondition": "F/D and M tag2 pass; direct literal at tag3 precedes B01 lookup at same tag.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "agreementId"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "b10-asset-anchor-before-scale",
      "phase": "M",
      "mutation": "Registered-asset provider B10 unavailable while scale literals equal2",
      "earlierGatePrecondition": "F/D and M tags1-15 pass; B10 anchors at tag16 asset, not scale17.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_BINDING_UNAVAILABLE",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "asset",
        "binding": "B10"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "prehead-intent-before-snapshot-claim",
      "phase": "M",
      "mutation": "Wire preHead55; Source intent preHeadbb; Source authenticated headcc; all hash32",
      "earlierGatePrecondition": "F/D and M tags1-17 pass; intent.preHead literal comparison precedes authenticated.head claim at tag18.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "preHead",
        "sourcePath": "intent.preHead"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "prehead-snapshot-claim-before-evidence",
      "phase": "M",
      "mutation": "Wire preHead55 equals Source intent55; Source authenticated headbb; B11 unavailable",
      "earlierGatePrecondition": "F/D and M tags1-17 pass; intent preHead equality passes, authenticated.head claim differs before B11 availability.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "preHead",
        "sourcePath": "authenticated.head"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "prior-predecessor-is-not-current-head",
      "phase": "M",
      "mutation": "Field19 prior predecessor66 vs Source predecessorbb, while all current preHead/AdvanceHead predecessor values55",
      "earlierGatePrecondition": "F/D and M tags1-18 including B11 pass; compare field19 only with prior-head claim/B12 relation, never AdvanceHead current predecessor.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "predecessor",
        "sourcePath": "authenticated.predecessor"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "agreement-field-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire Agreement vs Source Other",
      "earlierGatePrecondition": "F/D and all earlier M tags/providers pass; target literal or adopted resolution differs. KeyRef comparison requires adopted B08 resolution, never raw opaque-string coercion.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "agreementId"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "action-field-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire Action vs Source selected TransferLiteralFee",
      "earlierGatePrecondition": "F/D and all earlier M tags/providers pass; target literal or adopted resolution differs. KeyRef comparison requires adopted B08 resolution, never raw opaque-string coercion.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "actionId"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "coreprogram-field-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire RepayAccrualFirst vs Source selected TransferLiteralFee",
      "earlierGatePrecondition": "F/D and M tags1-9 pass; B04/tag6 authenticated selected manifest retains expected Core ID TransferLiteralFee without comparing future wire tag10. At10 Source selected and retained Core ID are compared to wire RepayAccrualFirst.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "coreProgramId",
        "comparisonTag": 10,
        "retainedFactBinding": "B04"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "sourcehash-claim-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire22*32 vs Source23*32",
      "earlierGatePrecondition": "F/D and all earlier M tags/providers pass; target literal or adopted resolution differs. KeyRef comparison requires adopted B08 resolution, never raw opaque-string coercion.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "sourceHash"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "corehash-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire33*32 vs selected authenticated Core image hash34*32",
      "earlierGatePrecondition": "F/D and prior M tags pass; B06 authenticates its selected artifact/image fact and computed hash34; then current tag11 wire hash33 differs. A different signed hash does not invalidate authentic artifact evidence.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "coreHash",
        "comparisonTag": 11,
        "retainedFactBinding": "B06"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "policyhash-claim-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire44*32 vs Source45*32",
      "earlierGatePrecondition": "F/D and all earlier M tags/providers pass; target literal or adopted resolution differs. KeyRef comparison requires adopted B08 resolution, never raw opaque-string coercion.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "policyHash"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "scale-field-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Wire scale2 vs Source scale3 within common domain",
      "earlierGatePrecondition": "F/D and M tags1-16 pass; B10/tag16 authenticates asset A metadata and retains scale3, without comparing future wire scale. At17 wire2 differs from Source3/retained scale3.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "scale",
        "comparisonTag": 17,
        "retainedFactBinding": "B10"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "keyref-resolution-mismatch",
      "phase": "conditional-field-predicate",
      "mutation": "Adopted B08 resolution yields different key than wire signerKey",
      "earlierGatePrecondition": "F/D and M tags1-14 pass; B08/tag13 retains real authorized key from adopted keyRef resolution; B09/tag14 verifies under adopted preprocessing using that retained authorized key over the canonical record containing different signerKey metadata. At15 wire signerKey differs; no future key equality occurred at13/14.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "signerKey",
        "comparisonTag": 15,
        "retainedFactBinding": "B08"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "sourcehash-malformed",
      "phase": "D",
      "mutation": "Source selected.sourceHash has63 hex chars",
      "earlierGatePrecondition": "F-wire/F-source pass; malformed Source claim fails complete D subtype sweep before any M binding.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "sourceHash",
        "sourcePath": "selected.sourceHash"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "policyhash-uppercase",
      "phase": "D",
      "mutation": "Source selected.policyDigest uppercase64hex",
      "earlierGatePrecondition": "F-wire/F-source pass; malformed Source claim fails complete D subtype sweep before any M binding.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "policyHash",
        "sourcePath": "selected.policyDigest"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "nonce-malformed",
      "phase": "D",
      "mutation": "Source nonce is0x-prefixed or63hex",
      "earlierGatePrecondition": "F-wire/F-source pass; malformed Source claim fails complete D subtype sweep before any M binding.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "nonce",
        "sourcePath": "intent.nonce"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "predecessor-malformed",
      "phase": "D",
      "mutation": "Source predecessor symbolic genesis",
      "earlierGatePrecondition": "F-wire/F-source pass; malformed Source claim fails complete D subtype sweep before any M binding.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "predecessor",
        "sourcePath": "authenticated.predecessor"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "successor-malformed",
      "phase": "D",
      "mutation": "Source submitted.postHead symbolic h1",
      "earlierGatePrecondition": "F-wire/F-source pass; malformed Source claim fails complete D subtype sweep before any M binding.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "successor",
        "sourcePath": "submitted.postHead"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "source-invalid-string",
      "phase": "F-source",
      "mutation": "Source JSON string token invalid escape",
      "earlierGatePrecondition": "Valid wire; actual Source parser first error returned unchanged with exact UTF8 byte offset.",
      "expected": {
        "status": "SourceRejected",
        "code": "INVALID_STRING",
        "publishedPost": null,
        "publishedEffects": null,
        "offset": 8
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions",
      "sourceByteFixture": "profile \"\\q\";",
      "offsetOracle": "Literal profile prefix is8 bytes; lexical JSON string parsing rejects invalid escape at token start."
    },
    {
      "id": "source-unknown-selected",
      "phase": "F-source",
      "mutation": "Source selected Action outside supported builtin names",
      "earlierGatePrecondition": "Valid wire; actual Source parser first error returned unchanged with exact UTF8 byte offset.",
      "expected": {
        "status": "SourceRejected",
        "code": "SOURCE6_PROFILE_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "offset": "exact original Source6Error.offset; numeric offset requires a separately frozen byte fixture"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "image-extra-signature",
      "phase": "I",
      "mutation": "Supplied image has extra signature property",
      "earlierGatePrecondition": "F/D/M/H/C pass; I validates supplied schema before complete image equality; preserve effect codec code without calling it WireRejected.",
      "expected": {
        "status": "EffectImageRejected",
        "code": "SHAPE",
        "publishedPost": null,
        "publishedEffects": null,
        "imageSide": "supplied"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "image-integer-leading-zero",
      "phase": "I",
      "mutation": "Supplied effect amount011",
      "earlierGatePrecondition": "F/D/M/H/C pass; I validates supplied schema before complete image equality; preserve effect codec code without calling it WireRejected.",
      "expected": {
        "status": "EffectImageRejected",
        "code": "INTEGER",
        "publishedPost": null,
        "publishedEffects": null,
        "imageSide": "supplied"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "image-unknown-line",
      "phase": "I",
      "mutation": "Supplied line kindMint",
      "earlierGatePrecondition": "F/D/M/H/C pass; I validates supplied schema before complete image equality; preserve effect codec code without calling it WireRejected.",
      "expected": {
        "status": "EffectImageRejected",
        "code": "VARIANT",
        "publishedPost": null,
        "publishedEffects": null,
        "imageSide": "supplied"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "genuine-different-snapshot-head",
      "phase": "M",
      "mutation": "Wire/Source intent/authenticated-head claims55 agree; a genuine B11 proof authenticates same-instance snapshot headbb",
      "earlierGatePrecondition": "F/D, V-S0-01 and M tags1-17 and both tag18 Source head literals pass; B11 proof integrity succeeds, then retained core/domain/asset checks pass before genuine headbb differs from signed55 at18.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "preHead",
        "binding": "B11",
        "comparisonTag": 18,
        "sourcePath": null,
        "factPath": "B11.snapshot.head"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "unsigned-round-mismatch",
      "phase": "M-unsigned-tail",
      "mutation": "Source claims100, authentic retained snapshot round101 within signed interval",
      "earlierGatePrecondition": "F/D and all signed M tags/subfields pass; B11 already authenticated full snapshot. This unsigned Source claim is compared at exact tail before B15/B16/C; all earlier tail claims pass.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "authenticated.round",
        "binding": "B11",
        "sourcePath": "authenticated.round",
        "factPath": "B11.snapshot.round"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "unsigned-balance-mismatch",
      "phase": "M-unsigned-tail",
      "mutation": "Source owner balance100, authentic retained owner balance99",
      "earlierGatePrecondition": "F/D and all signed M tags/subfields pass; B11 already authenticated full snapshot. This unsigned Source claim is compared at exact tail before B15/B16/C; all earlier tail claims pass.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "authenticated.balances[0].amount",
        "binding": "B11",
        "sourcePath": "authenticated.balances[0].amount",
        "factPath": "B11.snapshot.balances[0].amount"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "unsigned-allowance-mismatch",
      "phase": "M-unsigned-tail",
      "mutation": "Source spent0, authentic retained spent1",
      "earlierGatePrecondition": "F/D and all signed M tags/subfields pass; B11 already authenticated full snapshot. This unsigned Source claim is compared at exact tail before B15/B16/C; all earlier tail claims pass.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "authenticated.allowance.spent",
        "binding": "B11",
        "sourcePath": "authenticated.allowance.spent",
        "factPath": "B11.snapshot.allowances[0].spent"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "unsigned-work-mismatch",
      "phase": "M-unsigned-tail",
      "mutation": "Source workSpent0, authentic retained workSpent7",
      "earlierGatePrecondition": "F/D and all signed M tags/subfields pass; B11 already authenticated full snapshot. This unsigned Source claim is compared at exact tail before B15/B16/C; all earlier tail claims pass.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "authenticated.workSpent",
        "binding": "B11",
        "sourcePath": "authenticated.workSpent",
        "factPath": "B11.snapshot.workSpent"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "unsigned-obligation-mismatch",
      "phase": "M-unsigned-tail",
      "mutation": "Source principal1000, authentic retained principal1001 with internally valid accrued/outstanding",
      "earlierGatePrecondition": "F/D and all signed M tags/subfields pass; B11 already authenticated full snapshot. This unsigned Source claim is compared at exact tail before B15/B16/C; all earlier tail claims pass.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "authenticated.obligation.principal",
        "binding": "B11",
        "sourcePath": "authenticated.obligation.principal",
        "factPath": "B11.snapshot.obligations[0].principal"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "auth-scale-mismatch-with-equal-claims",
      "phase": "M",
      "mutation": "Wire and Source scale2 match; B10 retained authentic registered scale3",
      "earlierGatePrecondition": "F/D and all earlier tags pass; anchored provider integrity is genuine and retains expected facts without early future equality. Direct current Source literal equality passes, authenticated current-tag equality fails.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "scale",
        "binding": "B10",
        "comparisonTag": 17
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "auth-core-id-mismatch-with-equal-claims",
      "phase": "M",
      "mutation": "Wire and Source selected Core ID TransferLiteralFee match; authentic retained manifest names a different approved Core ID",
      "earlierGatePrecondition": "F/D and all earlier tags pass; anchored provider integrity is genuine and retains expected facts without early future equality. Direct current Source literal equality passes, authenticated current-tag equality fails.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "coreProgramId",
        "binding": "B04",
        "comparisonTag": 10
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "auth-sourcehash-mismatch-with-equal-claims",
      "phase": "M",
      "mutation": "Wire and Source claim22 match; authentic selected source artifact hash23",
      "earlierGatePrecondition": "F/D and all earlier tags pass; anchored provider integrity is genuine and retains expected facts without early future equality. Direct current Source literal equality passes, authenticated current-tag equality fails.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "sourceHash",
        "binding": "B05",
        "comparisonTag": 8
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "supplied-replayafter-bound17",
      "phase": "I",
      "mutation": "Supplied replayBefore fits and replayAfter has17 otherwise-shaped entries",
      "earlierGatePrecondition": "F/D/M/H/C pass; derived image codec validation passes; supplied root/replay arrays shaped; countBefore checked before countAfter; no truncation.",
      "expected": {
        "status": "EffectDomainRejected",
        "code": "W_D2F_REPLAY_HISTORY_BOUND",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "consumption.replayAfter",
        "count": 17,
        "cap": 16,
        "imageSide": "supplied"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "supplied-both-replay-arrays-bound17",
      "phase": "I",
      "mutation": "Supplied both replayBefore/replayAfter have17 otherwise-shaped entries",
      "earlierGatePrecondition": "F/D/M/H/C pass; derived image codec validation passes; supplied root/replay arrays shaped; countBefore checked before countAfter; no truncation.",
      "expected": {
        "status": "EffectDomainRejected",
        "code": "W_D2F_REPLAY_HISTORY_BOUND",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "consumption.replayBefore",
        "count": 17,
        "cap": 16,
        "imageSide": "supplied"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "derived-record-byte-bound",
      "phase": "I",
      "mutation": "All verified common IDs have64 characters; complete replayBefore15/replayAfter16 entries each164 bytes, so consumption alone exceeds4096 while H counts pass",
      "earlierGatePrecondition": "F/D/M/H/C pass with complete verified histories and compatible IDs. I validates derived side first; actual record byte bound fails, not authorization wire shape.",
      "expected": {
        "status": "EffectImageRejected",
        "code": "LENGTH",
        "publishedPost": null,
        "publishedEffects": null,
        "imageSide": "derived"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "invalid-evidence-b12",
      "phase": "M",
      "mutation": "Real adopted verifier rejects: Invalid proof fails to authenticate declared prior-link fact",
      "earlierGatePrecondition": "F/D, V-S0-01 and M tags1-18 pass, including current-tag direct literals before B12 verification; the unsigned snapshot tail has not run. Target provider exists but proof of its declared fact is invalid. Genuine different-value facts are not invalid evidence.",
      "expected": {
        "status": "EvidenceRejected",
        "code": "W_D2F_EVIDENCE_INVALID",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "predecessor",
        "binding": "B12",
        "comparisonTag": 19
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "invalid-evidence-b13",
      "phase": "M",
      "mutation": "Adopted verifier fails to authenticate the declared replay-status fact; no genuine status has been established",
      "earlierGatePrecondition": "F/D, V-S0-01 and M tags1-19 pass, including current-tag direct literals before B13 verification; the unsigned snapshot tail has not run. Target provider exists but proof of its declared fact is invalid. Genuine different-value facts are not invalid evidence.",
      "expected": {
        "status": "EvidenceRejected",
        "code": "W_D2F_EVIDENCE_INVALID",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "nonce",
        "binding": "B13",
        "comparisonTag": 20
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "invalid-evidence-b14",
      "phase": "M",
      "mutation": "Real adopted verifier rejects: Invalid grant/work evidence fails to authenticate declared authority fact",
      "earlierGatePrecondition": "F/D, V-S0-01 and M tags1-22 pass, including current-tag direct literals before B14 verification; the unsigned snapshot tail has not run. Target provider exists but proof of its declared fact is invalid. Genuine different-value facts are not invalid evidence.",
      "expected": {
        "status": "EvidenceRejected",
        "code": "W_D2F_EVIDENCE_INVALID",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "grossCap",
        "binding": "B14",
        "comparisonTag": 23
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "invalid-evidence-b16",
      "phase": "M",
      "mutation": "Real adopted verifier rejects: Invalid projection/completeness evidence fails to authenticate declared complete-history fact",
      "earlierGatePrecondition": "F/D, V-S0-01, all M tags1-35/subfields, unsigned snapshot tail and B15 pass. B16 provider exists but complete-history proof is invalid; no C execution has occurred.",
      "expected": {
        "status": "EvidenceRejected",
        "code": "W_D2F_EVIDENCE_INVALID",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "replayBefore",
        "binding": "B16"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "wire-scale19-source18-domain",
      "phase": "D",
      "mutation": "Wire scale19 is existing-codec legal; well-formed Source scale18 remains unchanged",
      "earlierGatePrecondition": "F-wire/F-source pass; complete D sweep checks Source18 then decoded wire19..38 outside selected intersection0..18.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "scale",
        "sourcePath": null,
        "inputPath": "wire.scale"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "wire-scale38-source18-domain",
      "phase": "D",
      "mutation": "Wire scale38 is existing-codec legal; well-formed Source scale18 remains unchanged",
      "earlierGatePrecondition": "F-wire/F-source pass; complete D sweep checks Source18 then decoded wire19..38 outside selected intersection0..18.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "scale",
        "sourcePath": null,
        "inputPath": "wire.scale"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "source-validFrom-over-u64",
      "phase": "D",
      "mutation": "Source notBefore2^64,notAfter2^64+1 is Source UInt128 legal while wire validity stays100..200",
      "earlierGatePrecondition": "F-wire/F-source pass with canonical decimals and ordered Source interval; mapped Source validity must fit UInt64 during D before M.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "validFrom",
        "sourcePath": "intent.notBefore"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "source-validUntil-over-u64",
      "phase": "D",
      "mutation": "Source notBefore100,notAfter2^64 is Source UInt128 legal while wire validity stays100..200",
      "earlierGatePrecondition": "F-wire/F-source pass with canonical decimals and ordered Source interval; mapped Source validity must fit UInt64 during D before M.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "validUntil",
        "sourcePath": "intent.notAfter"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "source-authenticated-head-domain",
      "phase": "D",
      "mutation": "Source intent hash55 is valid but authenticated.head symbolic h0",
      "earlierGatePrecondition": "F-wire/F-source pass; every earlier D slot passes; exact head claim subtype fails with distinct field/sourcePath.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "preHead",
        "sourcePath": "authenticated.head"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "source-advance-current-head-domain",
      "phase": "D",
      "mutation": "For fee-positive TransferLiteralFee or RepayAccrualFirst (six effects, AdvanceHead index5): Root current/predecessor/proposed heads are hash32 but submitted AdvanceHead current predecessor h0",
      "earlierGatePrecondition": "F-wire/F-source pass; every earlier D slot passes; exact head claim subtype fails with distinct field/sourcePath.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "effects[5].predecessor",
        "sourcePath": "submitted.effects[5].predecessor"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions",
      "constructor": "This explicit parsed submitted vector has AdvanceHead at index5; infer index from actual list, never fee.",
      "indexDerivation": "Actual parsed submitted.effects list has 6 lines with unique last AdvanceHead at index5; fee class is not an index rule."
    },
    {
      "id": "source-advance-successor-domain",
      "phase": "D",
      "mutation": "For fee-positive TransferLiteralFee or RepayAccrualFirst (six effects, AdvanceHead index5): Root heads hash32 and line predecessor hash32, but submitted AdvanceHead successor h1",
      "earlierGatePrecondition": "F-wire/F-source pass; every earlier D slot passes; exact head claim subtype fails with distinct field/sourcePath.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "effects[5].successor",
        "sourcePath": "submitted.effects[5].successor"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions",
      "constructor": "This explicit parsed submitted vector has AdvanceHead at index5; infer index from actual list, never fee.",
      "indexDerivation": "Actual parsed submitted.effects list has 6 lines with unique last AdvanceHead at index5; fee class is not an index rule."
    },
    {
      "id": "wire-keyscheme-literal",
      "phase": "F-wire",
      "mutation": "Canonical record keyScheme encoded literal byte changed from01 to00",
      "earlierGatePrecondition": "Earlier wire bytes canonical; decoder only admits exact schnorr_bip340 literal, not display label BIP340.",
      "expected": {
        "status": "WireRejected",
        "code": "LITERAL",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "keyScheme"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "core-intent-cap-scope",
      "phase": "existing-Core-predicate",
      "mutation": "Direct Core grossCap10 vs amount10+fee1; other fields/effects unchanged",
      "earlierGatePrecondition": "Existing direct Core predicate only: Stage passes; scope predicates before target pass as specified. Source parser alias formation and outer M may reject earlier; no consumer execution claimed.",
      "expected": {
        "status": "CoreRejected",
        "code": "S0_INTENT_SCOPE",
        "publishedPost": null,
        "publishedEffects": null,
        "judgment": "intent",
        "diagnosticWork": 1
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "isolated-existing-module",
      "sameEffectsMustPass": false
    },
    {
      "id": "core-intent-round-scope",
      "phase": "existing-Core-predicate",
      "mutation": "Direct Core round201 outside signed100..200; other fields/effects unchanged",
      "earlierGatePrecondition": "Existing direct Core predicate only: Stage passes; scope predicates before target pass as specified. Source parser alias formation and outer M may reject earlier; no consumer execution claimed.",
      "expected": {
        "status": "CoreRejected",
        "code": "S0_INTENT_SCOPE",
        "publishedPost": null,
        "publishedEffects": null,
        "judgment": "intent",
        "diagnosticWork": 1
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "isolated-existing-module",
      "sameEffectsMustPass": false
    },
    {
      "id": "core-intent-alias",
      "phase": "existing-Core-predicate",
      "mutation": "Direct Core recipient=signerO with complete distinct pre-state balance rows O/R/F and otherwise valid scope",
      "earlierGatePrecondition": "Existing direct Core predicate only: Stage passes; scope predicates before target pass as specified. Source parser alias formation and outer M may reject earlier; no consumer execution claimed.",
      "expected": {
        "status": "CoreRejected",
        "code": "S0_INTENT_ALIAS",
        "publishedPost": null,
        "publishedEffects": null,
        "judgment": "intent",
        "diagnosticWork": 1
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "isolated-existing-module",
      "sameEffectsMustPass": false
    },
    {
      "id": "source-zero-fee-advance-current-head-domain",
      "phase": "D",
      "mutation": "For zero-fee TransferLiteralFee (five effects, AdvanceHead index4): Root current/predecessor/proposed heads are hash32 but submitted AdvanceHead current predecessor h0",
      "earlierGatePrecondition": "F-wire/F-source pass; every earlier D slot passes; exact head claim subtype fails with distinct field/sourcePath.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "effects[4].predecessor",
        "sourcePath": "submitted.effects[4].predecessor"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions",
      "constructor": "This explicit parsed submitted vector has AdvanceHead at index4; infer index from actual list, never fee.",
      "indexDerivation": "Actual parsed submitted.effects list has 5 lines with unique last AdvanceHead at index4; fee class is not an index rule."
    },
    {
      "id": "source-zero-fee-advance-successor-domain",
      "phase": "D",
      "mutation": "For zero-fee TransferLiteralFee (five effects, AdvanceHead index4): Root heads hash32 and line predecessor hash32, but submitted AdvanceHead successor h1",
      "earlierGatePrecondition": "F-wire/F-source pass; every earlier D slot passes; exact head claim subtype fails with distinct field/sourcePath.",
      "expected": {
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "publishedPost": null,
        "publishedEffects": null,
        "field": "effects[4].successor",
        "sourcePath": "submitted.effects[4].successor"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions",
      "constructor": "This explicit parsed submitted vector has AdvanceHead at index4; infer index from actual list, never fee.",
      "indexDerivation": "Actual parsed submitted.effects list has 5 lines with unique last AdvanceHead at index4; fee class is not an index rule."
    },
    {
      "id": "genuine-consumed-replay-source-unused",
      "phase": "M",
      "mutation": "A genuine B13 proof authenticates selected domain/signer/nonce already consumed; Source selected replay claim is unused",
      "earlierGatePrecondition": "F/D, V-S0-01 and M tags1-19 pass; tag20 direct nonce and authenticated tuple nonce agree. B13 proof integrity/scope succeeds, then its genuine consumed status fails unused predicate. No unsigned tail or B16 runs.",
      "expected": {
        "publishedPost": null,
        "publishedEffects": null,
        "status": "BindingRejected",
        "code": "W_D2F_REPLAY_CONSUMED",
        "field": "nonce",
        "binding": "B13",
        "comparisonTag": 20,
        "sourcePath": null,
        "factPath": "B13.replay.status"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "genuine-consumed-replay-source-consumed",
      "phase": "M",
      "mutation": "A genuine B13 proof authenticates selected domain/signer/nonce already consumed; Source selected replay claim is consumed",
      "earlierGatePrecondition": "F/D, V-S0-01 and M tags1-19 pass; tag20 direct nonce and authenticated tuple nonce agree. B13 proof integrity/scope succeeds, then its genuine consumed status fails unused predicate. No unsigned tail or B16 runs.",
      "expected": {
        "publishedPost": null,
        "publishedEffects": null,
        "status": "BindingRejected",
        "code": "W_D2F_REPLAY_CONSUMED",
        "field": "nonce",
        "binding": "B13",
        "comparisonTag": 20,
        "sourcePath": null,
        "factPath": "B13.replay.status"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "source-replay-consumed-verified-history-absent",
      "phase": "M",
      "mutation": "Source authenticated.replay claims consumed; genuine B13 status is unused and valid B16 complete history excludes selected tuple",
      "earlierGatePrecondition": "F/D, V-S0-01, all M tags1-35/subfields, unsigned snapshot tail and B15 pass; B13 authentically unused passes at20; B16 integrity/completeness/projection succeeds with selected status unused. Source replay equality now fails at B16 tail before H/C.",
      "expected": {
        "publishedPost": null,
        "publishedEffects": null,
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "field": "authenticated.replay",
        "binding": "B16",
        "sourcePath": "authenticated.replay",
        "factPath": "B16.selectedReplayStatus",
        "comparisonLocus": "B16 tail after unsigned snapshot claims and B15"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "source-fee0-noncanonical-vector-head-domain",
      "phase": "D",
      "mutation": "Signed/submitted transfer fee0 agree; parsed submitted effects include optional third effect Credit despite zero fee; actual last AdvanceHead at5 has predecessor h0, successor hash32",
      "earlierGatePrecondition": "F-wire/F-source pass; all earlier D slots and submitted.postHead pass. Parser permits optional Credit independent of fee. D fails actual AdvanceHead predecessor before M or C can reject the vector.",
      "expected": {
        "publishedPost": null,
        "publishedEffects": null,
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "field": "effects[5].predecessor",
        "sourcePath": "submitted.effects[5].predecessor"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "source-fee1-noncanonical-vector-head-domain",
      "phase": "D",
      "mutation": "Signed/submitted transfer fee1 agree; parsed submitted effects omit optional third effect Credit despite positive fee; actual last AdvanceHead at4 has predecessor h0, successor hash32",
      "earlierGatePrecondition": "F-wire/F-source pass; all earlier D slots and submitted.postHead pass. Parser permits optional Credit independent of fee. D fails actual AdvanceHead predecessor before M or C can reject the vector.",
      "expected": {
        "publishedPost": null,
        "publishedEffects": null,
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "field": "effects[4].predecessor",
        "sourcePath": "submitted.effects[4].predecessor"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "genuine-different-snapshot-core",
      "phase": "M",
      "mutation": "Genuine B11 proof authenticates same-instance declared snapshot with core=moriarty-core/6; corresponding wire/Source/B10/version context remains unchanged",
      "earlierGatePrecondition": "F/D, V-S0-01, M tags1-17 and both tag18 Source head literals pass. B11 proof integrity succeeds; all B11 fact comparisons before core pass. Genuine core differs at its explicit B11/tag18 check before C.",
      "expected": {
        "publishedPost": null,
        "publishedEffects": null,
        "status": "BindingRejected",
        "code": "W_D2F_PROFILE_UNSUPPORTED",
        "field": "coreVersion",
        "binding": "B11",
        "comparisonTag": 18,
        "sourcePath": null,
        "factPath": "B11.snapshot.core",
        "relation": "V-S0-01"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "genuine-different-snapshot-domain",
      "phase": "M",
      "mutation": "Genuine B11 proof authenticates same-instance declared snapshot with domain=OtherD; corresponding wire/Source/B10/version context remains unchanged",
      "earlierGatePrecondition": "F/D, V-S0-01, M tags1-17 and both tag18 Source head literals pass. B11 proof integrity succeeds; all B11 fact comparisons before domain pass. Genuine domain differs at its explicit B11/tag18 check before C.",
      "expected": {
        "publishedPost": null,
        "publishedEffects": null,
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "field": "domain",
        "binding": "B11",
        "comparisonTag": 18,
        "sourcePath": null,
        "factPath": "B11.snapshot.domain"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "genuine-different-snapshot-asset",
      "phase": "M",
      "mutation": "Genuine B11 proof authenticates same-instance declared snapshot with asset=OtherA; corresponding wire/Source/B10/version context remains unchanged",
      "earlierGatePrecondition": "F/D, V-S0-01, M tags1-17 and both tag18 Source head literals pass. B11 proof integrity succeeds; all B11 fact comparisons before asset pass. Genuine asset differs at its explicit B11/tag18 check before C.",
      "expected": {
        "publishedPost": null,
        "publishedEffects": null,
        "status": "BindingRejected",
        "code": "W_D2F_FIELD_MISMATCH",
        "field": "asset",
        "binding": "B11",
        "comparisonTag": 18,
        "sourcePath": null,
        "factPath": "B11.snapshot.asset"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "invalid-snapshot-proof-before-core-domain-asset",
      "phase": "M",
      "mutation": "Invalid B11 proof declares wrong core/domain/asset/head; no declared content is authenticated",
      "earlierGatePrecondition": "F/D, V-S0-01, M1-17 and both tag18 Source literal head checks pass. B11 proof integrity fails before core/domain/asset/head comparisons.",
      "expected": {
        "publishedPost": null,
        "publishedEffects": null,
        "status": "EvidenceRejected",
        "code": "W_D2F_EVIDENCE_INVALID",
        "field": "preHead",
        "binding": "B11",
        "comparisonTag": 18,
        "sourcePath": null
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "b13-unused-b16-selected-present-source-unused",
      "phase": "M",
      "mutation": "Genuine B13 unused status passed at20; genuine complete B16 history contains selected composite at index0; Source claims unused",
      "earlierGatePrecondition": "F/D and all M tags/subfields, unsigned snapshot tail and B15 pass. B16 proof integrity/completeness succeeds; all tuple domains/canonicality and distinctness pass. History/B13 relation fails before Source replay claim equality, H or C.",
      "expected": {
        "publishedPost": null,
        "publishedEffects": null,
        "status": "BindingRejected",
        "code": "W_D2F_REPLAY_HISTORY_INCONSISTENT",
        "field": "consumption.replayBefore",
        "binding": "B16",
        "sourcePath": null,
        "factPath": "B16.completeReplayHistory",
        "selectedIndex": 0
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "b13-unused-b16-selected-present-source-consumed",
      "phase": "M",
      "mutation": "Genuine B13 unused status passed at20; genuine complete B16 history contains selected composite at index0; Source claims consumed",
      "earlierGatePrecondition": "F/D and all M tags/subfields, unsigned snapshot tail and B15 pass. B16 proof integrity/completeness succeeds; all tuple domains/canonicality and distinctness pass. History/B13 relation fails before Source replay claim equality, H or C.",
      "expected": {
        "publishedPost": null,
        "publishedEffects": null,
        "status": "BindingRejected",
        "code": "W_D2F_REPLAY_HISTORY_INCONSISTENT",
        "field": "consumption.replayBefore",
        "binding": "B16",
        "sourcePath": null,
        "factPath": "B16.completeReplayHistory",
        "selectedIndex": 0
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "verified-history-domain-domain",
      "phase": "M",
      "mutation": "Genuine B16 complete-history fact has unrelated first tuple with domain=9D; Source only claims unused selected key",
      "earlierGatePrecondition": "F/D and all M signed tags/subfields, unsigned tail and B15 pass; B13 unused passed. B16 proof integrity succeeds. Tuple0 canonical three-string shape and all earlier components pass; new authenticated component violates common domain before membership/H/C.",
      "expected": {
        "publishedPost": null,
        "publishedEffects": null,
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "field": "consumption.replayBefore[0].domain",
        "binding": "B16",
        "sourcePath": null,
        "factPath": "B16.completeReplayHistory[0][0]"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "verified-history-signer-domain",
      "phase": "M",
      "mutation": "Genuine B16 complete-history fact has unrelated first tuple with signer=9O; Source only claims unused selected key",
      "earlierGatePrecondition": "F/D and all M signed tags/subfields, unsigned tail and B15 pass; B13 unused passed. B16 proof integrity succeeds. Tuple0 canonical three-string shape and all earlier components pass; new authenticated component violates common domain before membership/H/C.",
      "expected": {
        "publishedPost": null,
        "publishedEffects": null,
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "field": "consumption.replayBefore[0].signer",
        "binding": "B16",
        "sourcePath": null,
        "factPath": "B16.completeReplayHistory[0][1]"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "verified-history-nonce-domain",
      "phase": "M",
      "mutation": "Genuine B16 complete-history fact has unrelated first tuple with nonce=h0; Source only claims unused selected key",
      "earlierGatePrecondition": "F/D and all M signed tags/subfields, unsigned tail and B15 pass; B13 unused passed. B16 proof integrity succeeds. Tuple0 canonical three-string shape and all earlier components pass; new authenticated component violates common domain before membership/H/C.",
      "expected": {
        "publishedPost": null,
        "publishedEffects": null,
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "field": "consumption.replayBefore[0].nonce",
        "binding": "B16",
        "sourcePath": null,
        "factPath": "B16.completeReplayHistory[0][2]"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "verified-history-noncanonical-tuple",
      "phase": "M",
      "mutation": "Genuine B16 history first tuple is a JSON three-string array with added whitespace; components otherwise valid",
      "earlierGatePrecondition": "F/D and all M signed tags/subfields, unsigned tail and B15 pass; B13 unused passed. B16 proof integrity succeeds. Tuple0 canonical-presentation test fails before component/membership/H/C checks.",
      "expected": {
        "publishedPost": null,
        "publishedEffects": null,
        "status": "BindingRejected",
        "code": "W_D2F_DOMAIN_UNSUPPORTED",
        "field": "consumption.replayBefore[0]",
        "binding": "B16",
        "sourcePath": null,
        "factPath": "B16.completeReplayHistory[0]"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    },
    {
      "id": "verified-history-duplicate-tuple",
      "phase": "M",
      "mutation": "Genuine B16 history has exact duplicate unrelated canonical tuple at indices0/1; selected key absent",
      "earlierGatePrecondition": "F/D and all M signed tags/subfields, unsigned tail and B15 pass; B13 unused passed. B16 proof integrity and every tuple-domain check succeed. Distinctness scan first duplicate at1 fails before selected membership/H/C.",
      "expected": {
        "publishedPost": null,
        "publishedEffects": null,
        "status": "BindingRejected",
        "code": "W_D2F_HISTORY_DUPLICATE",
        "field": "consumption.replayBefore[1]",
        "binding": "B16",
        "sourcePath": null,
        "factPath": "B16.completeReplayHistory[1]"
      },
      "status": "specified-only-not-executed",
      "outerScheduleScope": "planned-adapter-predicate-with-earlier-phase-preconditions"
    }
  ],
  "expectedInspectionResult": {
    "status": "InspectionOnly",
    "authentication": false,
    "unresolvedBindings": [
      "B01",
      "B02",
      "B03",
      "B04",
      "B05",
      "B06",
      "B07",
      "B08",
      "B09",
      "B10",
      "B11",
      "B12",
      "B13",
      "B14",
      "B15",
      "B16",
      "B17"
    ],
    "publishedPost": null,
    "publishedEffects": null
  },
  "runnableSubset": "Formation/mapping inspector and isolated existing Core/effect predicates; full semantic consumer positives remain blocked specified-only",
  "literalInspectionCases": [
    {
      "id": "agreement-literal-equal",
      "field": "agreementId",
      "wireValue": "Agreement",
      "sourceAstValue": "Agreement",
      "expectedRelation": "equal",
      "authentication": false,
      "stillUnresolvedBinding": "B01",
      "status": "specified-only-not-executed",
      "comparisonRule": "Exact literal string equality"
    },
    {
      "id": "agreement-literal-different",
      "field": "agreementId",
      "wireValue": "Agreement",
      "sourceAstValue": "Agreement2",
      "expectedRelation": "different",
      "authentication": false,
      "stillUnresolvedBinding": "B01",
      "status": "specified-only-not-executed",
      "comparisonRule": "Exact literal string equality"
    },
    {
      "id": "action-literal-equal",
      "field": "actionId",
      "wireValue": "TransferLiteralFee",
      "sourceAstValue": "TransferLiteralFee",
      "expectedRelation": "equal",
      "authentication": false,
      "stillUnresolvedBinding": "B04",
      "status": "specified-only-not-executed",
      "comparisonRule": "Exact literal string equality"
    },
    {
      "id": "action-existing-fixture-different",
      "field": "actionId",
      "wireValue": "Action",
      "sourceAstValue": "TransferLiteralFee",
      "expectedRelation": "different",
      "authentication": false,
      "stillUnresolvedBinding": "B04",
      "status": "specified-only-not-executed",
      "comparisonRule": "Exact literal string equality"
    },
    {
      "id": "sourcehash-claim-equal",
      "field": "sourceHash",
      "wireValue": "2222222222222222222222222222222222222222222222222222222222222222",
      "sourceAstValue": "2222222222222222222222222222222222222222222222222222222222222222",
      "expectedRelation": "equal",
      "authentication": false,
      "stillUnresolvedBinding": "B05",
      "status": "specified-only-not-executed",
      "comparisonRule": "Exact literal string equality"
    },
    {
      "id": "policyhash-claim-equal",
      "field": "policyHash",
      "wireValue": "4444444444444444444444444444444444444444444444444444444444444444",
      "sourceAstValue": "4444444444444444444444444444444444444444444444444444444444444444",
      "expectedRelation": "equal",
      "authentication": false,
      "stillUnresolvedBinding": "B07",
      "status": "specified-only-not-executed",
      "comparisonRule": "Exact literal string equality"
    },
    {
      "id": "scale-literal-equal",
      "field": "scale",
      "wireValue": 2,
      "sourceAstValue": "2",
      "expectedRelation": "equal",
      "authentication": false,
      "stillUnresolvedBinding": "B10",
      "status": "specified-only-not-executed",
      "comparisonRule": "Exact integer equality in shared scale domain"
    },
    {
      "id": "predecessor-claim-equal",
      "field": "predecessor",
      "wireValue": "6666666666666666666666666666666666666666666666666666666666666666",
      "sourceAstValue": "6666666666666666666666666666666666666666666666666666666666666666",
      "expectedRelation": "equal",
      "authentication": false,
      "stillUnresolvedBinding": "B12",
      "status": "specified-only-not-executed",
      "comparisonRule": "Exact literal string equality"
    },
    {
      "id": "coreprogram-selected-literal-equal",
      "field": "coreProgramId",
      "wireValue": "TransferLiteralFee",
      "sourceAstValue": "TransferLiteralFee",
      "expectedRelation": "equal",
      "authentication": false,
      "stillUnresolvedBindings": [
        "B04",
        "B06"
      ],
      "comparisonRule": "Exact literal string equality with AST.selected.actionId and lowerer intent.programId",
      "status": "specified-only-not-executed"
    },
    {
      "id": "equal-looking-keyref-is-not-key-binding",
      "field": "signerKey",
      "wireValue": "1111111111111111111111111111111111111111111111111111111111111111",
      "sourceAstValue": "1111111111111111111111111111111111111111111111111111111111111111",
      "expectedRelation": "equal-text-only",
      "authentication": false,
      "stillUnresolvedBindings": [
        "B08",
        "B09"
      ],
      "comparisonRule": "Display-only textual equality between raw-key presentation and opaque keyRef; no keyRef resolution, curve validity or ownership is inferred",
      "status": "specified-only-not-executed"
    },
    {
      "id": "local-profile-vocabulary-compatible",
      "field": "profile",
      "wireValue": "s0-provisional/1",
      "sourceAstValue": "moriarty-financial-agreement-source/6",
      "expectedRelation": "compatible-under-V-S0-01",
      "comparisonRule": "Exact selected V-S0-01 tuple relation, not string equality",
      "authentication": false,
      "stillUnresolvedBindings": [
        "B04",
        "B06"
      ],
      "status": "specified-only-not-executed"
    }
  ],
  "revision": "grok-repair-04",
  "selectedReading": "B",
  "phaseOrder": [
    "F-wire",
    "F-source",
    "D",
    "M",
    "H",
    "C",
    "I",
    "E",
    "L-optional-ledger"
  ],
  "domainSweepBeforeBinding": true,
  "domainFieldOrder": [
    "domain",
    "agreementId",
    "stageId",
    "episodeId",
    "actionId",
    "sourceHash",
    "coreProgramId",
    "policyHash",
    "signer",
    "asset",
    "scale",
    "preHead",
    "predecessor",
    "nonce",
    "validFrom",
    "validUntil",
    "operation fields in wire variant order",
    "authenticated.round",
    "submitted.postHead",
    "submitted.AdvanceHead predecessor then successor in line order"
  ],
  "bindingAnchors": {
    "agreementId": [
      "B01"
    ],
    "stageId": [
      "B02"
    ],
    "episodeId": [
      "B03"
    ],
    "actionId": [
      "B04"
    ],
    "sourceHash": [
      "B05"
    ],
    "coreHash": [
      "B06"
    ],
    "policyHash": [
      "B07"
    ],
    "signer": [
      "B08"
    ],
    "keyScheme": [
      "B09"
    ],
    "asset": [
      "B10"
    ],
    "preHead": [
      "B11"
    ],
    "predecessor": [
      "B12"
    ],
    "nonce": [
      "B13"
    ],
    "grossCap": [
      "B14"
    ],
    "successor": [
      "B15"
    ],
    "replayBefore": [
      "B16"
    ]
  },
  "signaturePreprocessing": "Unselected. SHA256(canonical authorization bytes) is existing wire content digest, not an adopted raw signature message.",
  "localSuccessSpecifiedOnly": {
    "status": "SemanticComparedUnqualified",
    "verifiedBindings": [
      "B01",
      "B02",
      "B03",
      "B04",
      "B05",
      "B06",
      "B07",
      "B08",
      "B09",
      "B10",
      "B11",
      "B12",
      "B13",
      "B14",
      "B15",
      "B16"
    ],
    "pendingBindings": [
      "B17"
    ],
    "publishedPost": null,
    "publishedEffects": null,
    "ledgerChange": false,
    "preconditions": "All F/D/M/H/C/I/E predicates pass under adopted B01-B16 definitions and real verified providers; no such execution exists."
  },
  "repairs": [
    "Reading B; B17 is pending after successful local image/equality comparison",
    "D complete domain sweep before M tag/literal/binding order",
    "Replay overflow explicit EffectDomainRejected without truncation or WireRejected",
    "B09 exact signature preprocessing unselected",
    "Equal coreProgramId literal still B04/B06 unauthenticated",
    "Head precedence and B10 asset anchor explicit",
    "Full post/effect/image oracles and late sameEffects preconditions",
    "Full source mismatch/hash subtype/image schema cases and preserved source offsets",
    "GPT P2: authenticate provider facts at anchor; retain context; per-tag signed comparison with complete unsigned snapshot tail",
    "Grok H1: invalid signature means verifier failure under later adopted preprocessing, never raw authorization-digest convention",
    "Grok medium: exact scheme literal and selected F-S0-01 correspondence, supplied-after/both replay cap oracles, imageSide, head sourcePath, equal-looking keyRef, invalid-evidence and domain/Core intent controls",
    "Grok final six-medium repair: snapshot factPath; early invalid-proof preconditions; constructor-specific head indices; exact proof-derived direct-Core handoff; genuine consumed replay versus invalid proof and Source/history disagreement; selected V-S0-01 cross-vocabulary compatibility; untrusted snapshotProofInput name",
    "Grok repair04: tag18 direct intent sourcePath; actual parsed AdvanceHead indices before C; B11 proof-versus-fact core/domain/asset/head exact order; B16 tuple domains/distinctness and B13/history consistency reject before H/C"
  ],
  "consumerTestsExecuted": false,
  "observedLocalSemanticComparisons": 0,
  "providerAuthenticationTiming": "At anchor authenticate provider integrity/declared-scope/availability and retain immutable verified facts. Agreement with already bound context is a separate fact comparison at this anchor. Future signed equality runs only at its own tag; no required comparison/authentication can be skipped before C.",
  "perTagSubOrder": [
    "direct Source literal checks or selected local correspondence",
    "authenticate anchored provider fact and retain verified context",
    "compare declared prior-context facts in specified anchor order, then current signed field; future-field equality remains at its tag"
  ],
  "unsignedSnapshotTailOrder": [
    "authenticated.round",
    "authenticated.balances[index].account",
    "authenticated.balances[index].amount",
    "authenticated.allowance.owner",
    "authenticated.allowance.remaining",
    "authenticated.allowance.spent",
    "authenticated.obligation.id",
    "authenticated.obligation.debtor",
    "authenticated.obligation.creditor",
    "authenticated.obligation.asset",
    "authenticated.obligation.principal",
    "authenticated.obligation.accrued",
    "authenticated.obligation.outstanding",
    "authenticated.obligation.status",
    "authenticated.workRemaining",
    "authenticated.workSpent",
    "successor/B15",
    "selected replay claim and complete history projection/B16"
  ],
  "keySchemeLiteral": "schnorr_bip340",
  "selectedLocalCorrespondences": [
    {
      "id": "F-S0-01",
      "tag": 27,
      "wireLiteral": "atomic-reject-terminal-success",
      "sourceLiteral": "success_only",
      "relation": "Closed S0 terminal success with empty retained effects/duties and atomic unpublished rejection",
      "directLiteralEquality": false,
      "normativeWD2Adopted": false
    },
    {
      "id": "V-S0-01",
      "tag": 1,
      "wireTuple": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "sourceVersion": 6,
        "coreVersion": 5
      },
      "sourceProfile": "moriarty-financial-agreement-source/6",
      "loweredIntentTuple": {
        "version": "moriarty-intent/3",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "core": "moriarty-core/5"
      },
      "snapshotCore": "moriarty-core/5",
      "effectTuple": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5"
      },
      "directCrossVocabularyLiteralEquality": false,
      "normativeWD2Adopted": false,
      "laterChecks": {
        "snapshotCore": "B11 tag18 proof integrity, then core/domain/asset/head fact checks; core mismatch PROFILE_UNSUPPORTED, other mismatches FIELD_MISMATCH",
        "effectTuple": "I derived/supplied validation"
      },
      "unsupportedTupleOracle": {
        "status": "BindingRejected",
        "code": "W_D2F_PROFILE_UNSUPPORTED",
        "field": "profile",
        "relation": "V-S0-01",
        "publishedPost": null,
        "publishedEffects": null
      },
      "unsupportedTupleReachability": "M/tag1 formation-produced unsupported tuple is uninhabited under current closed decoder/parser/lowerer. Genuine B11 snapshot wrong core is separately specified at18 with field coreVersion; it must reject before C."
    }
  ],
  "anchorRetainedFacts": {
    "B01": {
      "anchor": "agreementId/3",
      "retain": "Authenticated instance record and future stage/episode/action/code/asset associations",
      "compareLater": [
        "stageId/4",
        "episodeId/5",
        "actionId/6",
        "coreProgramId/10",
        "asset/16"
      ]
    },
    "B02": {
      "anchor": "stageId/4",
      "retain": "Authenticated stage record with prior agreement linkage and future episode identity",
      "compareLater": [
        "episodeId/5"
      ]
    },
    "B03": {
      "anchor": "episodeId/5",
      "retain": "Authenticated continuing-history record",
      "compareLater": [
        "preHead/18",
        "predecessor/19"
      ]
    },
    "B04": {
      "anchor": "actionId/6",
      "retain": "Authenticated selected manifest expected Core/source/policy identities and operation constructor",
      "compareLater": [
        "sourceHash/8",
        "coreProgramId/10",
        "coreHash/11",
        "policyHash/12",
        "operation/35"
      ]
    },
    "B05": {
      "anchor": "sourceHash/8",
      "retain": "Authenticated source artifact and computed source hash",
      "compareCurrentAfterAuthentication": "sourceHash/8"
    },
    "B06": {
      "anchor": "coreHash/11",
      "retain": "Authenticated selected Core artifact/lowering correspondence and computed hash",
      "compareCurrentAfterAuthentication": "coreHash/11"
    },
    "B07": {
      "anchor": "policyHash/12",
      "retain": "Authenticated policy artifact hash and selected predicates",
      "compareLater": [
        "grossCap/23",
        "feeCap/24",
        "netFloor/25"
      ]
    },
    "B08": {
      "anchor": "signer/13",
      "retain": "Authenticated signer/keyRef ownership and authorized x-only key",
      "compareLater": [
        "signerKey/15"
      ],
      "noWireKeyEqualityAtAnchor": true
    },
    "B09": {
      "anchor": "keyScheme/14",
      "retain": "Selected real-verifier success under later adopted preprocessing using B08 authorized key",
      "wireKeyEqualityAlreadyChecked": false
    },
    "B10": {
      "anchor": "asset/16",
      "retain": "Authenticated registered asset metadata and scale",
      "compareLater": [
        "scale/17"
      ],
      "noWireScaleEqualityAtAnchor": true
    },
    "B11": {
      "anchor": "preHead/18",
      "retain": "Authenticated head, predecessor, round, complete cells/work/history",
      "compareCurrentAfterAuthentication": "preHead/18",
      "compareLater": [
        "predecessor/19",
        "operation.debtor/35",
        "operation.creditor/35",
        "unsigned snapshot tail"
      ],
      "anchorFactComparisonOrder": [
        "snapshot.core against V-S0-01",
        "snapshot.domain against prior bound domain",
        "snapshot.asset against prior bound asset/B10",
        "snapshot.head against current preHead"
      ],
      "comparePriorContextAfterAuthentication": [
        "coreVersion/V-S0-01",
        "domain/2",
        "asset/16"
      ]
    },
    "B12": {
      "anchor": "predecessor/19",
      "retain": "Authenticated prior linkage scoped to established current head",
      "compareCurrentAfterAuthentication": "predecessor/19"
    },
    "B13": {
      "anchor": "nonce/20",
      "retain": "Authenticated replay tuple and genuine unused/consumed status scoped to prior domain/signer/head",
      "compareCurrentAfterAuthentication": "nonce/20",
      "compareLater": [
        "replay projection/B16"
      ],
      "currentTagSubOrder": [
        "Authenticate proof integrity/scope",
        "Compare current nonce to authenticated tuple",
        "Require unused; genuine consumed -> W_D2F_REPLAY_CONSUMED"
      ]
    },
    "B14": {
      "anchor": "grossCap/23",
      "retain": "Authenticated grant/work authority and adopted policy predicates",
      "compareLater": [
        "feeCap/24",
        "netFloor/25",
        "unsigned allowance/work tail"
      ],
      "counterEqualityIsNotCapEquality": true
    },
    "B15": {
      "anchor": "successor tail",
      "retain": "Authenticated digest-independent extension fact",
      "compareCurrentAfterAuthentication": "Source proposed successor"
    },
    "B16": {
      "anchor": "replay tail",
      "retain": "Authenticated adopted projection preserving complete verified history",
      "compareCurrentAfterAuthentication": "After all tuple-domain/distinctness/B13-history consistency checks, compare Source selected replay claim; then preserve full history transport",
      "postAuthenticationCheckOrder": [
        "All tuple canonical shape/domain ID/signer ID/nonce hash checks in history order",
        "Distinctness, first duplicate scan position",
        "Selected membership versus B13 unused fact; genuine present -> REPLAY_HISTORY_INCONSISTENT",
        "Source selected replay claim equality",
        "Retain complete history for H/C"
      ]
    }
  },
  "directCoreHandoff": {
    "inputBoundary": "snapshotProofInput is untrusted proof/provider input, not trusted caller state",
    "stateFieldSource": {
      "core": "B11.snapshot.core",
      "domain": "B11.snapshot.domain",
      "asset": "B11.snapshot.asset",
      "head": "B11.snapshot.head",
      "round": "B11.snapshot.round",
      "workRemaining": "B11.snapshot.workRemaining",
      "workSpent": "B11.snapshot.workSpent",
      "balances": "B11.snapshot.balances exact ordered copies",
      "allowances": "B11.snapshot.allowances exact ordered copies",
      "obligations": "B11.snapshot.obligations exact ordered copies",
      "consumedReplay": "B16.completeReplayHistory exact canonical composite strings, authenticated projection of same B11 snapshot"
    },
    "intent": "Exact bound lowerSource6(ast).intent; no signedDigest added",
    "submittedEffects": "Exact ordered lowerSource6(ast).submittedEffects, including JSON.stringify([ast.domain,ast.intent.signer,effect.key]) UseReplay transformation",
    "proposedPostHead": "B15.successor",
    "requestedOutcome": {
      "phase": "TerminalSuccess",
      "retainedEffects": [],
      "retainedDuties": []
    },
    "localStipulation": "omitted",
    "entryPoint": "prepareMil4S0 direct; never Source wrapper",
    "forbiddenStateSources": [
      "lowerSource6(ast).state",
      "Source replay shortcut []/[selectedKey]"
    ],
    "rejectionOracle": "Preserve direct Core judgment/code/diagnosticWork; never wrapper override",
    "imageSources": "This same coreState and actual Core candidatePost/effects"
  },
  "advanceHeadIndexRule": {
    "rule": "Use actual parsed submitted.effects index of each AdvanceHead; current grammar unique final line gives submitted.effects.length-1",
    "feeDoesNotDetermineIndex": true,
    "canonicalVectorExamples": {
      "transferWithOptionalCredit": 5,
      "transferWithoutOptionalCredit": 4,
      "repayment": 5
    }
  },
  "b16HistoryValidation": {
    "tuple": "Canonical JSON exactly three strings",
    "componentOrder": [
      "domain common ID",
      "signer common ID",
      "nonce hash32"
    ],
    "commonId": "Same D subtype with reserved-word exclusion; do not force historical tuples to current domain/signer",
    "typedFailure": "BindingRejected/W_D2F_DOMAIN_UNSUPPORTED at B16 before H/C",
    "duplicateFailure": "BindingRejected/W_D2F_HISTORY_DUPLICATE at first duplicate index",
    "selectedPresentAfterB13Unused": "BindingRejected/W_D2F_REPLAY_HISTORY_INCONSISTENT before Source replay equality",
    "invalidProof": "EvidenceRejected/W_D2F_EVIDENCE_INVALID before any unverified value check"
  }
}

```

## experiments/moriarty-language/formal/mil4/effect-consumer/RESULT.md

sha256: `226cf6c0fafef34aff79374b3373fccc3eaa3c6ebd8dd4f09ead3c935a4712d0`

```text
# W-D2F design sprint result

**Design only. No consumer implementation or full-consumer positive execution.**
The read-only map covers the authorization header and all 35 signed fields,
all 14 nested operation fields, and the complete Core pre/post-to-W-D2E image
recipe. The independent expectation matrix specifies six positive candidates,
150 hostile cases, and eleven inspection cases before any implementation.
No expected case was executed in this sprint; these are specified-only values,
not new test successes or a fresh approval of the repaired packet. The prior
design received Grok 4.7 xhigh feedback naming high ambiguity; this successor
repairs that feedback and still awaits fresh reviews.

## Repair disposition

This is `grok-repair-04`, superseding reviewed `grok-final-repair-03`. Its exact
five files are preserved under `history/pre-grok-repair-04/`; `review-repair-02`
remains under `history/pre-grok-final-03/`; earlier reviewed
`grok-repair-01` remains under `history/grok-repair-01/`; the initial five-file
design and original receipt remain under `history/pre-grok-repair-01/`.
The parent-reported original reviewed packet prefix is
`ff47de9e…`; its external frozen packet and raw audit are untouched. This design
revision neither rewrites that audit nor claims a new approval.

The fourth repair addresses four medium findings in the exited Grok review:

- Generic prehead-mismatch now reports intent.preHead at tag18, matching first
  direct literal precedence. Genuine snapshot mismatch still names a factPath.
- AdvanceHead diagnostics use the actual parsed effects index. Fee0 with an
  extra Credit has index5; fee1 with omitted Credit has index4. Both domain
  controls precede Core vector rejection. Canonical indices are examples only.
- After B11 proof integrity, check core then domain then asset then head at18.
  Genuine core mismatch is PROFILE_UNSUPPORTED; genuine domain/asset/head mismatch
  is FIELD_MISMATCH. Invalid proof is EVIDENCE_INVALID before content checks.
- After B16 proof integrity, validate every history tuple shape/domain/signer/
  nonce, then distinctness, then selected membership versus B13, then Source
  claim. Genuine selected-present after B13 unused rejects the named
  REPLAY_HISTORY_INCONSISTENT before Source equality/H/C. Invalid proof and
  consumed-at-tag20 remain separate. Malformed history never falls into Core stage.

All 13 new cases remain specified-only. Prior Reading B, GPT P2 and Grok H1
repairs remain in force. Previous third-repair dispositions follow:

- Genuine other-head B11 comparison names factPath B11.snapshot.head and null
  sourcePath. The equal Source authenticated.head claim is not its failure locus.
- Invalid B12/B13/B14 proofs require earlier tags only (through18/19/22), not a
  future unsigned tail. Invalid B16 requires all tags, unsigned tail and B15.
- Canonical-vector AdvanceHead indices are explicit: optional-Credit transfer/
  repayment5 and no-optional-Credit transfer4. The fourth repair makes the actual
  parsed list authoritative for D, even when its fee/vector semantics disagree.
- The exact C constructor copies verified B11 snapshot cells/work and B16
  complete history, bound lowered intent, lowerer-transformed composite UseReplay
  effects and B15 successor into direct prepareMil4S0. No Source wrapper, replay
  shortcut, fabricated signedDigest or local stipulation supplies the handoff.
- Genuine B13 consumed-status proofs reject W_D2F_REPLAY_CONSUMED at20; invalid
  proofs still reject EVIDENCE_INVALID. Source consumed versus verified absent
  rejects FIELD_MISMATCH at B16 tail. Each has an explicit first-failure oracle.
- Selected local V-S0-01 relates /3 wire, s0-provisional/1, /6 Source, /5 Core
  and effects/1 vocabularies. Raw cross-string equality is forbidden. An unsupported
  M/tag1 tuple branch is specified but uninhabited in current closed formation
  modules; the separate genuine B11 wrong-core fact has its explicit oracle.
  snapshotProofInput names untrusted proof/provider input; only verifiers produce
  internal verified snapshot facts.

Prior repair dispositions:


- Provider integrity/scope/availability is authenticated at the anchor and
  immutable facts retained. Comparisons with later signed fields run at their
  own tags: B04 Core ID10, B08 authorized key15, B10 registered scale17. Every
  authenticated-fact comparison and unsigned Source round/cell/work tail must
  pass before Core. Neither authentication nor a required equality is bypassed.
- B09 uses the retained authorized key under later adopted preprocessing. The
  invalid-signature oracle now specifies that selected verifier's failure,
  without assuming raw digest bytes as its signature message.
- An invalid snapshot proof remains EVIDENCE_INVALID; a genuine proof of a
  different head yields current tag18 FIELD_MISMATCH. Separate unsigned-tail
  mismatches cover round, balance, allowance, obligation and work claims.
- Exact keyScheme is schnorr_bip340. Local F-S0-01 explicitly relates Source
  success_only to wire atomic-reject-terminal-success for closed S0; these are
  different strings, and this correspondence does not adopt normative W-D2.
- Added supplied replayAfter and both-array overflow cases (preBefore wins),
  effect error imageSide, disambiguated head sourcePath, equal-looking opaque
  keyRef inspection, invalid B12/B13/B14/B16 evidence, codec-legal scale19/38 and
  Source validity UInt64-overflow controls, and isolated Core intent/alias cases.

All remain specified-only. Numeric Source error offsets beyond the two small
literal oracles still require separately frozen source byte fixtures; this
repair preserves the original parser-offset contract and fabricates none.

- **Single Reading B schedule:** F-wire, F-source, complete D domain sweep,
  tag-ordered M literal/binding checks, H history bound, C Core preparation,
  I complete image and E field26 comparison. B01–B16 must be verified. Successful
  local E is specified to return SemanticComparedUnqualified with B17 pending;
  missing B17 blocks only separate requested L ledger mode.
- **Replay bound:** complete pre or post history >16 rejects
  EffectDomainRejected/W_D2F_REPLAY_HISTORY_BOUND, with field/count/cap16 and
  null publication. Pre16 plus unused selected key produces post17 rejection.
  No truncation/deduplication and no authorization WireRejected label.
- **Signature:** B09 exact message preprocessing/verifier remains unselected.
  SHA256 of canonical wire is the current content digest, not an adopted raw
  signature message.
- **Equality layer:** coreProgramId==Source selected.actionId can literally
  match while B04/B06 remain unauthenticated; an explicit ninth case records it.
- **Medium clarifications:** head-slot first-failure order, exact wire field
  names, B10's asset/tag16 anchor, missing field/hash-domain/image-schema cases,
  original Source UTF8 error offsets, sameEffects preconditions for late Core
  predicates, and full literal repay pre/post/effect/image oracles are recorded.

No consumer module or test execution was added. Semantic comparisons observed
in this sprint: zero. Structural expectation/hash checks are not consumer runs.

The full positive consumer cannot currently be implemented honestly from the
available interfaces. Existing wire, Source/6, Core/5 and W-D2E modules supply
canonical bytes, local preparation and equality predicates, while authenticated
field correspondence and consumption remain missing. No module, existing
packet, normative W-D2 decision, proof claim or ledger claim was edited.

## Direct comparison and authentication are distinct

Literal `wire.agreementId == Source AST.programId` is available for inspection.
`AST.programId` is the Source agreement name, while `Core.intent.programId`
contains the selected action. Even matching agreement literals leave B01's
authenticated agreement-instance/registry binding unavailable.

Literal `wire.actionId == AST.selected.actionId` is also available. The separate
relationship to signed `coreProgramId`, exact selected code and operation
semantics remains B04/B06. Existing W-D2E fixtures carry actionId `Action`;
Source/6 accepts only selected `TransferLiteralFee` or `RepayAccrualFirst` for
the corresponding S0 action. The planned literal check would find a difference.
Do not silently ignore actionId or rewrite frozen fixtures to promote them to
full consumer positives.

Domain, asset, signer, pre-head, nonce, validity, caps, floor and every operation
endpoint/amount have direct equality predicates. Source claim values for scale,
predecessor, sourceHash and policyHash can be compared literally too, while
their authenticated interpretation remains unavailable. Stage and episode
identities have no Source/Core slot. These layers are explicit in FIELD-MAP.md
and EXPECTATIONS.json; absence of Core slots does not imply AST literal
comparisons are impossible.

## Exact blockers

| Binding | Open obligation |
| --- | --- |
| B01 | Authenticated agreement-instance registry and same-instance cells |
| B02 | Authenticated stage identity/uniqueness |
| B03 | Authenticated episode identity/history |
| B04 | Adopted action/source selection/Core-program relationship |
| B05 | Exact Source hash image and selected-source authentication |
| B06 | Exact Core hash image and source-to-selected-Core correspondence |
| B07 | Exact policy image and policy binding |
| B08 | Signer/keyRef/x-only-key ownership, validity and key rotation |
| B09 | Unselected exact signature-message preprocessing and verifier in this path |
| B10 | Asset identity, atomic-unit scale and settlement registration |
| B11 | One authenticated head/round/snapshot for every required cell |
| B12 | Signed prior predecessor linkage, distinct from current preHead |
| B13 | Authenticated composite replay-status proof, unused predicate and uniqueness policy |
| B14 | Authenticated allowance/grant and work spent/remaining authority |
| B15 | Valid successor and digest-independent head-extension evidence |
| B16 | Complete replay-history transport from Source's selected-key claim |
| B17 | Atomic ledger compare-and-consume/apply for the same digest and effects |

Current proposed first unavailable mapping is B01 at field agreementId, after
well-formed compatible wire/Source inputs, the complete D domain sweep and earlier
M direct predicates. A late out-of-domain value can beat B01; a late valid-value
mismatch cannot. At the same tag, the literal check precedes provider lookup.
The planned result is `BindingRejected/W_D2F_BINDING_UNAVAILABLE`, with field,
binding ID and null published effects/post. This is a design expectation, not
an observed consumer result. Missing input Source documents and earlier
formation failures are not bypassed to reach this predicate.

The Source document embeds opaque source_hash and digest claims. A whole-source
hash that includes its own source_hash would self-reference. No canonical
code-only Source image, masked-document rule, Core image or policy image has
been adopted. Source scale cap18 differs from wire/effect38. Source/Core rounds
are UInt128 while wire/effect rounds use UInt64. Identifier alphabets differ.
Source replay lowering yields [] or the selected key from a caller claim,
without complete unrelated replay history. Each requires an explicit decision
or transport, not a guessed mapping or successful caller stipulation.

## Recommended order and next useful implementation

1. Adopt agreement/stage/episode/action/Core identity definitions (B01–B04).
2. Define noncircular exact Source/Core/policy images and correspondence (B05–B07).
3. Define signer/keyRef and registered asset/scale bindings (B08/B10), including
   the exact shared identifier, scale and round domain.
4. Implement snapshot, predecessor, replay/grant/work and complete-history
   evidence (B11–B14/B16), then head-extension and exact signature verification
   (B15/B09).
5. Freeze new independent Source/wire/full-state positive and hostile fixtures
   under those adopted definitions.
6. Implement the bound semantic derivation adapter: check every signed field,
   derive Core effects/post, reconstruct complete image from that same pre/post,
   compare the entire supplied image, then recompute field26 from the derived
   image. A hostile image with a matching rehash must still reject before
   equality. Preserve B01–B16 absent-provider rejection and unqualified local
   scope; B17 remains pending after this local comparison, not an earlier gate.
7. Implement and demonstrate B17 separately before any ledger acceptance claim.

An inspection-only mismatch/rejection subset is runnable without these missing
definitions; isolated existing Core/equality checks are also runnable. Their
scope cannot establish a positive authenticated semantic consumer. Building an
inspection API now would largely repeat W-D2E. This sprint recommends the
decision sequence above and creates no inspection or consumer module.

PLAN.md supplies the concrete sequence, exact proposed rejection schedule and
implementation gate. EXPECTATIONS.json names earlier-gate preconditions for
conditional cases so a missing provider cannot be replaced with synthetic
evidence. freeze-receipt.json pins the design and all read-only dependencies.
W-D2, wallet/signature interoperability, native proof and ledger acceptance
remain open; this packet is separate from frozen S1B/W-D2 packets and W-D2E.

```

## experiments/moriarty-language/formal/mil4/effect-consumer/freeze-receipt.json

sha256: `0e94dcea9939278434467c2e9628708d9b27072054ba06f8ef6eee8aaf000e65`

```json
{
  "schema": "moriarty-wd2f-design-freeze/5",
  "revision": "grok-repair-04",
  "selectedReading": "B",
  "frozenAt": "2026-09-30T10:37:52.794405+00:00",
  "implementationFiles": [],
  "consumerTestsExecuted": false,
  "observedLocalSemanticComparisons": 0,
  "observedFullConsumerSuccesses": 0,
  "counts": {
    "signedFields": 35,
    "header": 1,
    "nestedOperationFields": 14,
    "positiveCandidates": 6,
    "hostileCases": 150,
    "literalInspectionCases": 11,
    "missingBindings": 17,
    "unsignedSnapshotTailSpecified": true,
    "localCompatibilityRelations": 2
  },
  "designInputs": {
    "PLAN.md": "1c41a95eac4871223e77e15485a0534572d352aae728317fa2853ab58da3fbb2",
    "FIELD-MAP.md": "b7257ef34a0cba9a25c4c660a76be16e9aae5d879062ad2d182f9931dca0a660",
    "EXPECTATIONS.json": "131ee6a89bfbb32ecde2cc090738ad18b53c04d2c36d474e1839c3b72f69a0eb",
    "RESULT.md": "226cf6c0fafef34aff79374b3373fccc3eaa3c6ebd8dd4f09ead3c935a4712d0"
  },
  "readOnlyDependencies": {
    "experiments/moriarty-language/formal/mil4/wire/SPEC.md": "644ababf2c3c0a80a73a3d56a5f37e0780a5e6a197cc27a0eb2f356d731d0ae1",
    "experiments/moriarty-language/formal/mil4/wire/codec.mjs": "8cdc9e65e3005895cd69bebf240275ce5fedc17fe58283bba2757bf06e0743fe",
    "experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts": "f3d2a05d07b051d2097fd7fb00ed1cd013eba91e9da7a2591b6f69707d4a02e7",
    "experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts": "1b3bd3b01ed2694ea967f62fea6808843a198045d870270b1d44c121081f66d8",
    "experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts": "855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda",
    "experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md": "725662ae7572a93cbc8f0d1631ca364a56d1c3dff3b9b02cae92485cd5a899ec",
    "experiments/moriarty-language/formal/mil4/effect-wire/SPEC.md": "53690f52df23dc20c7bf464e266f08fc233829f0d065f94b6404317bb5f945f8",
    "experiments/moriarty-language/formal/mil4/effect-wire/codec.mjs": "1b47621bac1ff3ecb0c87eb730352739b155824df7e14f6b98333ddb044b55ad",
    "experiments/moriarty-language/formal/mil4/effect-wire/fixtures.json": "23fc291984118217503349d3b033f1c56422ea161b7d0935758e55e18183df93"
  },
  "preservedReviewedPacket": {
    "path": "history/pre-grok-repair-04",
    "revision": "grok-final-repair-03",
    "files": {
      "PLAN.md": "b7cabbe427cb9ea9d05010b498baec30275c0a939070b4379bb42a34a99981e0",
      "FIELD-MAP.md": "e51cd539699bd0afd21f0c81aecd0af39bf12e24e8c0a50fba88f8929e9c1129",
      "EXPECTATIONS.json": "bdd3567edd17baef7ab99a74285a23810ca35f7598502f9bfb5c19737868b492",
      "RESULT.md": "39880a1008013147f3e97c6b4ee66d75e8aeabe891d3fd31510e11f8ab51e444",
      "freeze-receipt.json": "344c413bc3493baf519b0fe92f13606c36e1a47e7e686a090c6565d006f48cc4"
    },
    "rawReviews": "Parent-held exited Grok xhigh review and prior raw reviews unchanged; repair04 awaits fresh independent review"
  },
  "earlierReviewedPackets": [
    {
      "path": "history/pre-grok-final-03",
      "revision": "review-repair-02",
      "files": {
        "PLAN.md": "bc9320f5f35feed677739d5c9d2ffd3c4853d39dd801990e37d9aff009e5a6f0",
        "FIELD-MAP.md": "bbd5a2378303b455650fad0aef4c819b8e974ef7eb5dfe75c03c926406504e58",
        "EXPECTATIONS.json": "0730e35e3284fdcb4086e84f6433f1532415cac492007223580b239cbf9f2f14",
        "RESULT.md": "0be6c73636510c57e430b9e1024ac8a9fa0b6c7837b993c3c0f5be840e3294f5",
        "freeze-receipt.json": "371f404d335e07297ff392afa05ff624054b837348e553aae4423dd6fda52243"
      },
      "rawReviews": "Parent-held exited final Grok review and prior raw reviews unchanged; this successor awaits fresh independent review"
    },
    {
      "path": "history/grok-repair-01",
      "freezeReceiptSha256": "3c8a345beb6de65dda242d960c38e81aea1f5ccef8d624ea0ed4050facef4564",
      "files": {
        "PLAN.md": "0fac6664d6f351819bf9992b8403bbb736e017062f6500e1fd291c1cc72de7c0",
        "FIELD-MAP.md": "9c478f85887511ee4eca56cb5dd1a1a29f72281d4a8789ea3452e1bbdc0c24dd",
        "EXPECTATIONS.json": "da4cccdc43315dd390bd0d7166cce01359fdcdabe21a7503cc4e79299830eb4b",
        "RESULT.md": "4e645c0bc27ec6fc95bc08d7cb55814db01504674253be04b51db3f6526bbd30",
        "freeze-receipt.json": "3c8a345beb6de65dda242d960c38e81aea1f5ccef8d624ea0ed4050facef4564"
      },
      "rawReviews": "Parent-held exited GPT/Grok raw reviews are unchanged; repaired successor has no new review approval"
    }
  ],
  "preservedOriginal": {
    "path": "history/pre-grok-repair-01",
    "freezeReceiptSha256": "0b130071e1630e9071fba1e1e7174a5414cf50f4058f0623c050f18720181578",
    "files": {
      "PLAN.md": "a8cc5ba281ea01333d01abf6e79d72f4826d69106afe9604105074cc349a7daf",
      "FIELD-MAP.md": "7c7f8e876b3b9e55807cfeececb8c86c4a3d96a4db0812e190ceeabd6900caa9",
      "EXPECTATIONS.json": "17d1b3d67dab35b96da2a1b65e747589d0b99366fbae7dc590892c9343e9bb53",
      "RESULT.md": "e5619bce9537342e2c880dfd25a9337ee3bfc28f883de42f278ce42df4a6674c",
      "freeze-receipt.json": "0b130071e1630e9071fba1e1e7174a5414cf50f4058f0623c050f18720181578"
    },
    "parentReportedReviewedPacketPrefix": "ff47de9e",
    "originalRawAudit": "External parent-held raw audit unchanged; not rewritten or represented as a fresh approval"
  },
  "repairDisposition": [
    "M1:generic prehead direct mismatch names intent.preHead at18",
    "M2:D derives AdvanceHead index from actual parsed list,never fee;malformed semantic vector controls at4/5",
    "M3:B11 proof integrity precedes explicit core/domain/asset/head fact checks at18;wrong core PROFILE_UNSUPPORTED,other mismatches FIELD_MISMATCH",
    "M4:B16 proof integrity precedes full tuple domains,distinctness,B13/history consistency,Source replay claim;genuine selected-present REPLAY_HISTORY_INCONSISTENT",
    "Prior Reading B,P2 provider versus per-tag equality,H1 unselected signature preprocessing and exact direct-Core construction preserved"
  ],
  "verification": "150 unique specified-only hostile cases,13 new cases,6 unchanged positive oracles and11 unchanged inspections;actual parsed-index controls,B11 genuine-fact versus invalid-proof oracles,B16 history domain/distinctness/consistency order and null publication checked;historical five-file freeze and9 read-only dependency hashes unchanged;no Source parser/Core/consumer executed"
}

```

## experiments/moriarty-language/formal/mil4/wire/SPEC.md

sha256: `644ababf2c3c0a80a73a3d56a5f37e0780a5e6a197cc27a0eb2f356d731d0ae1`

```text
# PROVISIONAL `/3` S0 signed authorization codec

**Candidate only.** This isolated wire experiment does not freeze W-D1/W-D2, migrate `/3` to `/4`, or implement a source/Core/ledger consumer. The caps below are experimental admission choices, not measured wallet or network limits.

## Bytes and primitive types

Header is 18 bytes: the 17 ASCII bytes `moriarty-intent/3` followed by one NUL. Every subsequent field starts with its one-byte tag, in the exact order below. There are no optional fields. No generic extension tag, padding or trailing byte is admitted. Binary input must be a Uint8Array of at most 4096 bytes; tag order is part of canonicality. Decoder checks the intrinsic view brand/type, byte length, backing buffer and offset; own metadata properties and accessors cannot change the bound or copied bytes. It copies only the checked view into a fresh bounded buffer. Proxied or forged views, detached storage and incompatible view types return `SHAPE`. Oversized actual views return `LENGTH` before allocation, even when their own byteLength reports zero.

`id` is a UInt16 big-endian byte count followed by 1–64 ASCII bytes matching `[A-Za-z0-9][A-Za-z0-9._:/-]*`. There is no Unicode normalization, NUL, case folding or whitespace trimming. IDs are nominal and distinct fields preserve distinct sorts; the codec does not authenticate identifier ownership. Encoder string type and the 64-code-unit length limit precede alphabet validation and byte allocation; any longer string returns `LENGTH`, including a malformed string. Accepted ASCII strings have equal code-unit and byte lengths.

`hash32` and `key32` are exactly 32 raw bytes. Their JSON presentation is exactly 64 lowercase hex characters, without `0x`. A key's length is checked; membership on secp256k1 is an external cryptographic check.

`u64` and `nominal` are respectively 8 and 16 big-endian bytes. Their JSON presentation is a canonical nonnegative decimal string: `0` or a nonzero digit followed by digits. u64 cap is `2^64−1`; nominal cap is `2^127−1`. Negative numbers, numeric JS values, leading zeroes, decimal points, exponent notation and cap+1 reject. Encoder string type and the 20/39-code-unit limits precede decimal validation and BigInt conversion; any longer string returns `RANGE`, including a malformed string. Fixed width prohibits redundant integer encodings. An operation amount must be positive. This codec does not check fee/gross/net arithmetic or balances.

`scale` is one unsigned byte, cap 38. `empty` is an explicit UInt16 count equal to zero and presents as `[]`; any element/count rejects. Fixed literal fields present as the exact strings/numbers in the table. Unknown object keys, missing fields, inherited required fields and unknown operation keys reject; property insertion order does not affect encoded bytes. Each required root and operation property is read once through its own enumerable data descriptor and retained in a local snapshot. Encoding and validity/positive-amount checks use that same snapshot. Property get traps are not used for these fields; operation kind also uses its single captured descriptor. For a Proxy, the validated descriptor snapshot is the accepted input value. Ordinary parsed JSON has the same bytes as before. This is not a general sandbox for arbitrary reflection traps or concurrent backing-storage mutation.

## Ordered common fields

| Tag | JSON field | Type or exact encoded value |
| --- | --- | --- |
| 1 | profile | `s0-provisional/1`, byte `01` |
| 2 | domain | id |
| 3 | agreementId | id |
| 4 | stageId | id |
| 5 | episodeId | id |
| 6 | actionId | id |
| 7 | sourceVersion | number 6, byte `06` |
| 8 | sourceHash | hash32 |
| 9 | coreVersion | number 5, byte `05` |
| 10 | coreProgramId | id |
| 11 | coreHash | hash32 |
| 12 | policyHash | hash32 |
| 13 | signer | id |
| 14 | keyScheme | `schnorr_bip340`, byte `01` |
| 15 | signerKey | key32 |
| 16 | asset | id |
| 17 | scale | scale |
| 18 | preHead | hash32 |
| 19 | predecessor | hash32 |
| 20 | nonce | hash32 |
| 21 | validFrom | u64 |
| 22 | validUntil | u64; must be ≥validFrom |
| 23 | grossCap | nominal |
| 24 | feeCap | nominal |
| 25 | netFloor | nominal |
| 26 | effectCommitment | hash32, supplied commitment |
| 27 | failurePolicy | `atomic-reject-terminal-success`, byte `01` |
| 28 | supplyChanges | empty |
| 29 | observations | empty |
| 30 | disclosures | empty |
| 31 | retainedEffects | empty |
| 32 | retainedDuties | empty |
| 33 | delegation | `none`, byte `00` |
| 34 | recovery | `none`, byte `00` |
| 35 | operation | variant below |

The JSON field `schemaVersion` is mandatory and exactly `moriarty-intent/3`; the header encodes it. Domain is explicitly signed. Exactly one signer key, one domain and one settlement asset exist in this record. Validity is a closed interval of rounds in that domain; conversion to clock time is not implemented. Provisional replay identity is the tuple `(domain, signer, nonce)`. The nonce is a separately supplied replay field, selected before digest construction; field 20 is included in the signed record, so changing it changes the intent digest. The two fixtures use distinct nonces and replay identities. The tuple uses nominal signer identity; authenticating its relationship to signerKey, predecessor/head, and consuming replay requires a consumer outside this codec. This tuple does not settle the normative key-rotation/replay policy.

## Operation variants

After tag 35, operation kind byte `01` means `{kind:'transfer', owner:id, recipient:id, feeRecipient:id, amount:nominal, fee:nominal}` in that order. All endpoints, fee and principal are signed, including feeRecipient when fee=0. Alias rejection and zero-fee effect policy belong to the semantic consumer.

Kind byte `02` means `{kind:'repayment', obligationId:id, payer:id, debtor:id, creditor:id, amount:nominal, allocation:'AccrualFirst', conversion:'identity'}` in that order. The last two literals encode as bytes `01`, `01`. This admits a reference to an existing obligation only: obligation opening, status, debtor/creditor/asset correspondence and funded reduction require an authenticated consumer. Identity means mantissa=1, scale=0, rounding=none; the asset's signed atomic-unit scale remains the common scale field.

## Digest and effect commitment

The candidate content digest is SHA-256 of the complete canonical record bytes, including the `/3` header. `authorizationDigest` returns its lowercase 64-character hex presentation. No wallet prefix, prehash wrapper, signature, signature-valid flag, proof or transaction is part of this codec.

`effectCommitment` is an opaque 32-byte commitment supplied before encoding. The intended producer commits the complete prepared effect vector and required consumption/footprint information under a separately selected effect encoding. That vector must not include this authorization digest, this commitment, a signature, or a post-head defined from this authorization digest. Thus the dependency is `authenticated pre-state + operation → prepared effects → supplied effectCommitment → authorization bytes → digest`; no codec operation feeds the digest back into effects. The fixture commitments are illustrative byte strings, not authenticated effects. Selecting the effect encoding and proving commitment equality are explicitly unimplemented.

The consumer must prove sourceHash/Core identity/policy correspondence, derive effects from these signed terms at preHead, bind effectCommitment to those effects and bind the same digest across proof and acceptance. No such binding follows from a round trip.

## Stable prototype rejection codes

`SHAPE` rejects missing/extra/inherited fields or wrong object/byte-input types; `LITERAL` fixed-version/profile/scheme/failure/allocation/conversion mismatch; `ID` noncanonical alphabet/empty ID; `LENGTH` ID or record cap; `HEX` noncanonical key/hash presentation; `INTEGER` noncanonical decimal presentation; `RANGE` numeric/scale cap or zero operation amount; `VALIDITY` reversed interval; `EMPTY` nonempty deferred field; `HEADER` wrong header; `FIELD_TAG` missing/out-of-order/unknown field tag; `OPERATION_TAG` unknown variant; `TRUNCATED` incomplete payload; `TRAILING` excess bytes. Object shape precedes primitive validation; primitive fields follow the fixed order. Validity is checked after validUntil; positive amount is checked after the entire operation. Decoder record type/size/header checks precede field decoding. These are codec errors, not W-D3 six-judgment codes.

## Evidence and unverified loci

The [current connector specification](https://github.com/midnightntwrk/midnight-dapp-connector-api/blob/612db2b62dbd78c079da62e57e7b8585a204b858/docs/api/_media/SPECIFICATION.md#L371) requires a signed-message prefix and advertises Schnorr/ECDSA. The [ledger host verifier](https://github.com/midnightntwrk/midnight-ledger/blob/9f9842ebed66cdff0f54d3fb09efc6a7cd077ed8/onchain-runtime-wasm/src/primitives.rs#L93) uses Schnorr, whose [k256 preprocessing](https://github.com/RustCrypto/elliptic-curves/blob/5ac8f5d77f11399ff48d87b0554935f6eddda342/k256/src/schnorr/verifying.rs#L111) needs an explicit interoperability discriminator. This prototype does not implement those paths. Its raw key bytes are not the ledger's tagged serialization.

Canonical Source/6 lowering, Core/5 round trips, semantic execution, SHA-256 circuit cost, native certificate, wallet/signature interoperability, on-chain verification, effect correspondence, cap feasibility and final W-D2 votes all remain open.

```

## experiments/moriarty-language/formal/mil4/wire/codec.mjs

sha256: `8cdc9e65e3005895cd69bebf240275ce5fedc17fe58283bba2757bf06e0743fe`

```javascript
// PROVISIONAL wire experiment. No source/Core, wallet, proof or ledger consumer.
import { createHash } from 'node:crypto';

export const CAPS = Object.freeze({ recordBytes: 4096, identifierBytes: 64, scale: 38, u64: 2n ** 64n - 1n, nominal: 2n ** 127n - 1n });
export const HEADER = 'moriarty-intent/3\0';
const fields = [
  ['profile', 'literal', 's0-provisional/1', 1],
  ['domain', 'id'], ['agreementId', 'id'], ['stageId', 'id'], ['episodeId', 'id'], ['actionId', 'id'],
  ['sourceVersion', 'literal', 6, 6], ['sourceHash', 'hex'], ['coreVersion', 'literal', 5, 5],
  ['coreProgramId', 'id'], ['coreHash', 'hex'], ['policyHash', 'hex'], ['signer', 'id'],
  ['keyScheme', 'literal', 'schnorr_bip340', 1], ['signerKey', 'hex'], ['asset', 'id'], ['scale', 'scale'],
  ['preHead', 'hex'], ['predecessor', 'hex'], ['nonce', 'hex'], ['validFrom', 'u64'], ['validUntil', 'u64'],
  ['grossCap', 'nominal'], ['feeCap', 'nominal'], ['netFloor', 'nominal'], ['effectCommitment', 'hex'],
  ['failurePolicy', 'literal', 'atomic-reject-terminal-success', 1],
  ['supplyChanges', 'empty'], ['observations', 'empty'], ['disclosures', 'empty'], ['retainedEffects', 'empty'], ['retainedDuties', 'empty'],
  ['delegation', 'literal', 'none', 0], ['recovery', 'literal', 'none', 0], ['operation', 'operation'],
];
const transfer = [['owner', 'id'], ['recipient', 'id'], ['feeRecipient', 'id'], ['amount', 'nominal'], ['fee', 'nominal']];
const repayment = [['obligationId', 'id'], ['payer', 'id'], ['debtor', 'id'], ['creditor', 'id'], ['amount', 'nominal'], ['allocation', 'literal', 'AccrualFirst', 1], ['conversion', 'literal', 'identity', 1]];
const typedArrayPrototype = Object.getPrototypeOf(Uint8Array.prototype);
const viewType = Object.getOwnPropertyDescriptor(typedArrayPrototype, Symbol.toStringTag).get;
const viewByteLength = Object.getOwnPropertyDescriptor(typedArrayPrototype, 'byteLength').get;
const viewBuffer = Object.getOwnPropertyDescriptor(typedArrayPrototype, 'buffer').get;
const viewByteOffset = Object.getOwnPropertyDescriptor(typedArrayPrototype, 'byteOffset').get;
function fail(code, field) { const error = new Error(`${code}: ${field}`); error.code = code; throw error; }
function shape(value, names, field, kindDescriptor) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) fail('SHAPE', field);
  const keys = Reflect.ownKeys(value);
  if (keys.length !== names.length || keys.some(key => typeof key !== 'string' || !names.includes(key))) fail('SHAPE', field);
  const snapshot = Object.create(null);
  for (const name of names) {
    const descriptor = name === 'kind' && kindDescriptor ? kindDescriptor : Object.getOwnPropertyDescriptor(value, name);
    if (!descriptor || !('value' in descriptor) || !descriptor.enumerable) fail('SHAPE', `${field}.${name}`);
    snapshot[name] = descriptor.value;
  }
  return snapshot;
}
function uintBytes(value, width) {
  const result = Buffer.alloc(width);
  for (let i = width - 1; i >= 0; i--) { result[i] = Number(value & 255n); value >>= 8n; }
  return result;
}
function readUInt(bytes) { let value = 0n; for (const byte of bytes) value = (value << 8n) | BigInt(byte); return value; }
function encodePrimitive(value, type, name, literal, byte) {
  switch (type) {
    case 'literal': if (value !== literal) fail('LITERAL', name); return Buffer.from([byte]);
    case 'id': {
      if (typeof value !== 'string') fail('ID', name);
      // Accepted IDs are ASCII, so a code-unit limit bounds validation and allocation.
      if (value.length > CAPS.identifierBytes) fail('LENGTH', name);
      if (!/^[A-Za-z0-9][A-Za-z0-9._:/-]*$/.test(value)) fail('ID', name);
      const data = Buffer.from(value, 'ascii');
      return Buffer.concat([uintBytes(BigInt(data.length), 2), data]);
    }
    case 'hex': if (typeof value !== 'string' || !/^[0-9a-f]{64}$/.test(value)) fail('HEX', name); return Buffer.from(value, 'hex');
    case 'u64': case 'nominal': {
      if (typeof value !== 'string') fail('INTEGER', name);
      // Bound text before regex scanning and BigInt conversion, even if malformed.
      if (value.length > (type === 'u64' ? 20 : 39)) fail('RANGE', name);
      if (!/^(0|[1-9][0-9]*)$/.test(value)) fail('INTEGER', name);
      const number = BigInt(value);
      if (number > CAPS[type]) fail('RANGE', name);
      return uintBytes(number, type === 'u64' ? 8 : 16);
    }
    case 'scale': if (!Number.isInteger(value) || Object.is(value, -0) || value < 0 || value > CAPS.scale) fail('RANGE', name); return Buffer.from([value]);
    case 'empty': if (!Array.isArray(value) || Reflect.ownKeys(value).length !== 1 || value.length !== 0) fail('EMPTY', name); return Buffer.alloc(2);
    default: fail('SHAPE', name);
  }
}
function operationFields(value) {
  const descriptor = value && Object.getOwnPropertyDescriptor(value, 'kind');
  if (!descriptor || !('value' in descriptor)) fail('SHAPE', 'operation.kind');
  if (descriptor.value === 'transfer') return [1, transfer, descriptor];
  if (descriptor.value === 'repayment') return [2, repayment, descriptor];
  fail('OPERATION_TAG', 'operation.kind');
}
function encodeOperation(value) {
  const [tag, schema, kindDescriptor] = operationFields(value);
  value = shape(value, ['kind', ...schema.map(([name]) => name)], 'operation', kindDescriptor);
  const parts = [Buffer.from([tag])];
  for (const [name, type, literal, byte] of schema) parts.push(encodePrimitive(value[name], type, `operation.${name}`, literal, byte));
  if (value.amount === '0') fail('RANGE', 'operation.amount');
  return Buffer.concat(parts);
}
export function encodeAuthorization(value) {
  value = shape(value, ['schemaVersion', ...fields.map(([name]) => name)], 'authorization');
  if (value.schemaVersion !== 'moriarty-intent/3') fail('LITERAL', 'schemaVersion');
  const parts = [Buffer.from(HEADER, 'ascii')];
  for (let i = 0; i < fields.length; i++) {
    const [name, type, literal, byte] = fields[i];
    parts.push(Buffer.from([i + 1]));
    parts.push(type === 'operation' ? encodeOperation(value[name]) : encodePrimitive(value[name], type, name, literal, byte));
    if (name === 'validUntil' && BigInt(value.validFrom) > BigInt(value.validUntil)) fail('VALIDITY', name);
  }
  const result = Buffer.concat(parts);
  if (result.length > CAPS.recordBytes) fail('LENGTH', 'authorization');
  return result;
}
class Reader {
  constructor(bytes) { this.bytes = bytes; this.offset = 0; }
  take(size, field) {
    if (this.offset + size > this.bytes.length) fail('TRUNCATED', field);
    const result = this.bytes.subarray(this.offset, this.offset + size); this.offset += size; return result;
  }
  byte(field) { return this.take(1, field)[0]; }
  primitive(type, name, literal, byte) {
    switch (type) {
      case 'literal': if (this.byte(name) !== byte) fail('LITERAL', name); return literal;
      case 'id': {
        const length = Number(readUInt(this.take(2, name)));
        if (length === 0) fail('ID', name);
        if (length > CAPS.identifierBytes) fail('LENGTH', name);
        const data = this.take(length, name);
        if (data.some(value => value > 127)) fail('ID', name);
        const value = data.toString('ascii');
        if (!/^[A-Za-z0-9][A-Za-z0-9._:/-]*$/.test(value)) fail('ID', name);
        return value;
      }
      case 'hex': return this.take(32, name).toString('hex');
      case 'u64': case 'nominal': {
        const number = readUInt(this.take(type === 'u64' ? 8 : 16, name));
        if (number > CAPS[type]) fail('RANGE', name);
        return number.toString();
      }
      case 'scale': { const value = this.byte(name); if (value > CAPS.scale) fail('RANGE', name); return value; }
      case 'empty': if (readUInt(this.take(2, name)) !== 0n) fail('EMPTY', name); return [];
      default: fail('SHAPE', name);
    }
  }
  operation() {
    const tag = this.byte('operation.kind');
    const schema = tag === 1 ? transfer : tag === 2 ? repayment : null;
    if (!schema) fail('OPERATION_TAG', 'operation.kind');
    const result = { kind: tag === 1 ? 'transfer' : 'repayment' };
    for (const [name, type, literal, byte] of schema) result[name] = this.primitive(type, `operation.${name}`, literal, byte);
    if (result.amount === '0') fail('RANGE', 'operation.amount');
    return result;
  }
}
export function decodeAuthorization(bytes) {
  // Brand check rejects proxies/forged prototypes before any shadowable property read.
  if (!ArrayBuffer.isView(bytes)) fail('SHAPE', 'bytes');
  let length, buffer, offset;
  try {
    if (viewType.call(bytes) !== 'Uint8Array') fail('SHAPE', 'bytes');
    length = viewByteLength.call(bytes);
    buffer = viewBuffer.call(bytes);
    offset = viewByteOffset.call(bytes);
  } catch { fail('SHAPE', 'bytes'); }
  if (length > CAPS.recordBytes) fail('LENGTH', 'bytes');
  let copy;
  try {
    // Construct a trusted view from intrinsic metadata; copy at most the checked cap.
    // Construction also rejects detached or incompatible backing storage with SHAPE.
    const view = new Uint8Array(buffer, offset, length);
    copy = Buffer.alloc(length);
    Uint8Array.prototype.set.call(copy, view);
  } catch { fail('SHAPE', 'bytes'); }
  const reader = new Reader(copy);
  if (!reader.take(Buffer.byteLength(HEADER), 'header').equals(Buffer.from(HEADER, 'ascii'))) fail('HEADER', 'header');
  const value = { schemaVersion: 'moriarty-intent/3' };
  for (let i = 0; i < fields.length; i++) {
    const [name, type, literal, byte] = fields[i];
    if (reader.byte(name) !== i + 1) fail('FIELD_TAG', name);
    value[name] = type === 'operation' ? reader.operation() : reader.primitive(type, name, literal, byte);
    if (name === 'validUntil' && BigInt(value.validFrom) > BigInt(value.validUntil)) fail('VALIDITY', name);
  }
  if (reader.offset !== reader.bytes.length) fail('TRAILING', 'bytes');
  return value;
}
export function authorizationDigest(value) { return createHash('sha256').update(encodeAuthorization(value)).digest('hex'); }

```

## experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts

sha256: `f3d2a05d07b051d2097fd7fb00ed1cd013eba91e9da7a2591b6f69707d4a02e7`

```typescript
/** Provisional, closed Source/6 S0 presentation parser. No authentication occurs here. */
import {
  MIL4_S0_CORE, MIL4_S0_INTENT, MIL4_S0_SOURCE,
  type S0Effect, type S0Intent, type S0State,
} from './mil4-s0-core-v5.ts';

const U128 = (1n << 128n) - 1n;
const S128 = (1n << 127n) - 1n;
const encoder = new TextEncoder();
const RESERVED = new Set((
  'profile agreement unit party asset const state action requires let next emit ensures true false not and or domain settlement scale selected source_hash digest intent signer key nonce pre_head post_head valid gross_cap fee_cap net_floor failure success_only signed_action observations empty disclosures retained_effects retained_duties delegation none recovery authenticated head predecessor round balance allowance remaining spent obligation debtor creditor principal accrued outstanding settled status replay unused consumed work_remaining work_spent submit transfer from to fee_to value fee repay payer amount conversion identity effects debit credit set_obligation use_allowance use_replay advance_head'
).split(' '));

type TokenKind = 'word' | 'integer' | 'string' | 'punctuation' | 'eof';
interface Token { kind: TokenKind; text: string; value: string; start: number; end: number }
export class Source6Error extends Error {
  readonly code: string;
  readonly offset: number;
  constructor(code: string, offset: number, message: string) {
    super(message);
    this.name = 'Source6Error';
    this.code = code;
    this.offset = offset;
  }
}
function fail(code: string, offset: number, message: string): never {
  throw new Source6Error(code, offset, message);
}
function isSurrogate(value: number): boolean { return value >= 0xd800 && value <= 0xdfff; }
function scalarString(value: string, offset: number): void {
  for (let i = 0; i < value.length; i++) {
    const c = value.charCodeAt(i);
    if (c >= 0xd800 && c <= 0xdbff && i + 1 < value.length) {
      const low = value.charCodeAt(i + 1);
      if (low >= 0xdc00 && low <= 0xdfff) { i++; continue; }
    }
    if (isSurrogate(c)) fail('INVALID_SURROGATE', offset + encoder.encode(value.slice(0, i)).length, 'Lone UTF-16 surrogate');
  }
}
function lexical(source: string): Token[] {
  scalarString(source, 0);
  if (encoder.encode(source).length > 65536) fail('SOURCE_BOUND', 0, 'Source exceeds 65536 UTF-8 bytes');
  const tokens: Token[] = [];
  let i = 0;
  let byte = 0;
  const advance = (end: number): void => { byte += encoder.encode(source.slice(i, end)).length; i = end; };
  const emit = (kind: TokenKind, end: number, value = source.slice(i, end)): void => {
    if (tokens.length >= 8191) fail('TOKEN_BOUND', byte, 'Too many tokens');
    const start = byte;
    const raw = source.slice(i, end);
    advance(end);
    tokens.push({ kind, text: raw, value, start, end: byte });
  };
  while (i < source.length) {
    const c = source[i];
    if (/[ \t\r\n]/.test(c)) { advance(i + 1); continue; }
    if (source.startsWith('//', i)) {
      const next = source.indexOf('\n', i + 2);
      advance(next < 0 ? source.length : next); continue;
    }
    if (source.startsWith('/*', i)) {
      const next = source.indexOf('*/', i + 2);
      if (next < 0) fail('UNTERMINATED_COMMENT', byte, 'Unclosed block comment');
      advance(next + 2); continue;
    }
    if (c === '"') {
      let end = i + 1;
      let escaped = false;
      for (; end < source.length; end++) {
        const x = source[end];
        if (x === '"' && !escaped) { end++; break; }
        if (x === '\\' && !escaped) escaped = true;
        else escaped = false;
      }
      const raw = source.slice(i, end);
      if (!raw.endsWith('"') || raw.length < 2) fail('INVALID_STRING', byte, 'Unclosed string');
      let value: string;
      try { value = JSON.parse(raw) as string; }
      catch { fail('INVALID_STRING', byte, 'Invalid JSON string'); }
      scalarString(value, byte);
      if (encoder.encode(value).length > 1024) fail('STRING_BOUND', byte, 'Decoded string exceeds 1024 UTF-8 bytes');
      emit('string', end, value); continue;
    }
    if (/[0-9]/.test(c)) {
      let end = i + 1;
      while (end < source.length && /[0-9]/.test(source[end])) end++;
      const raw = source.slice(i, end);
      if (raw.length > 78) fail('INTEGER_BOUND', byte, 'Integer exceeds 78 digits');
      if (!/^(0|[1-9][0-9]*)$/.test(raw)) fail('INVALID_INTEGER', byte, 'Noncanonical integer');
      emit('integer', end); continue;
    }
    if (/[A-Za-z]/.test(c)) {
      let end = i + 1;
      while (end < source.length && /[A-Za-z0-9_]/.test(source[end])) end++;
      const next = source.codePointAt(end);
      if (next !== undefined && /[\p{L}\p{N}\p{Pc}\p{Mn}\p{Mc}]/u.test(String.fromCodePoint(next)))
        fail('NON_ASCII_IDENTIFIER', byte, 'Non-ASCII identifier');
      if (end - i > 64) fail('IDENTIFIER_BOUND', byte, 'Identifier exceeds 64 characters');
      emit('word', end); continue;
    }
    const point = source.codePointAt(i)!;
    if (/[\p{L}\p{N}\p{Pc}\p{Mn}\p{Mc}]/u.test(String.fromCodePoint(point)))
      fail('NON_ASCII_IDENTIFIER', byte, 'Non-ASCII identifier');
    if (source.startsWith('..', i)) { emit('punctuation', i + 2); continue; }
    if ('{};'.includes(c)) { emit('punctuation', i + 1); continue; }
    fail('UNEXPECTED_CHAR', byte, 'Unexpected source character');
  }
  tokens.push({ kind: 'eof', text: '', value: '', start: byte, end: byte });
  return tokens;
}

export type Source6Action =
  | { kind: 'Transfer'; from: string; to: string; feeTo: string; value: string; fee: string }
  | { kind: 'Repay'; obligation: string; payer: string; amount: string; conversion: 'identity' };
export interface Source6Obligation {
  id: string; debtor: string; creditor: string; asset: string;
  principal: string; accrued: string; outstanding: string; status: 'Outstanding';
}
export interface Source6Ast {
  profile: typeof MIL4_S0_SOURCE; programId: string; domain: string;
  settlement: { asset: string; scale: string };
  selected: { actionId: string; sourceHash: string; policyDigest: string };
  intent: {
    signer: string; keyRef: string; nonce: string; preHead: string;
    notBefore: string; notAfter: string; grossCap: string; feeCap: string; netFloor: string;
    signedAction: Source6Action; failure: 'success_only';
    observations: 'empty'; disclosures: 'empty'; retainedEffects: 'empty'; retainedDuties: 'empty';
    delegation: 'none'; recovery: 'none';
  };
  authenticated: {
    head: string; predecessor: string; round: string;
    balances: { account: string; amount: string }[];
    allowance: { owner: string; remaining: string; spent: string };
    obligation?: Source6Obligation; replay: 'unused' | 'consumed';
    workRemaining: string; workSpent: string;
  };
  submitted: { action: Source6Action; effects: S0Effect[]; postHead: string };
}

class Parser {
  private index = 0;
  private nodes = 0;
  private depth = 0;
  private readonly tokens: Token[];
  constructor(tokens: Token[]) { this.tokens = tokens; }
  private get here(): Token { return this.tokens[this.index]; }
  private node(): void {
    if (++this.nodes > 8192) fail('AST_BOUND', this.here.start, 'Too many AST nodes');
  }
  private enter(): void { if (++this.depth > 64) fail('DEPTH_BOUND', this.here.start, 'Nesting exceeds 64'); }
  private leave(): void { this.depth--; }
  private take(word: string, code = 'SOURCE6_SHAPE'): void {
    if (this.here.text !== word) fail(code, this.here.start, `Expected ${word}`);
    this.index++;
  }
  private effectTag(word: string): void {
    if (this.here.text !== word && this.here.kind === 'word'
        && !new Set(['debit', 'credit', 'set_obligation', 'use_allowance', 'use_replay', 'advance_head']).has(this.here.text))
      fail('SOURCE6_UNKNOWN_TAG', this.here.start, 'Unknown effect tag');
    this.take(word);
  }
  private id(): string {
    const token = this.here;
    if (token.kind !== 'word' || RESERVED.has(token.text)) fail('SOURCE6_SHAPE', token.start, 'Expected identifier');
    this.index++; return token.value;
  }
  private string(): string {
    const token = this.here;
    if (token.kind !== 'string') fail('SOURCE6_SHAPE', token.start, 'Expected string');
    if (token.value.length === 0) fail('SOURCE6_SHAPE', token.start, 'Empty opaque string');
    this.index++; return token.value;
  }
  private uint(max: bigint = U128): string {
    const token = this.here;
    if (token.kind !== 'integer') fail('SOURCE6_SHAPE', token.start, 'Expected integer');
    this.index++;
    if (BigInt(token.value) > max) fail('SOURCE6_RANGE', token.start, 'Integer exceeds nominal bound');
    return token.value;
  }
  private action(): Source6Action {
    this.node();
    if (this.here.text === 'transfer') {
      this.take('transfer'); this.take('from'); const from = this.id();
      this.take('to'); const to = this.id(); this.take('fee_to'); const feeTo = this.id();
      this.take('value'); const value = this.uint(S128); this.take('fee'); const fee = this.uint(S128);
      this.take(';'); return { kind: 'Transfer', from, to, feeTo, value, fee };
    }
    if (this.here.text === 'repay') {
      this.take('repay'); this.take('obligation'); const obligation = this.id();
      this.take('payer'); const payer = this.id(); this.take('amount'); const amount = this.uint(S128);
      this.take('conversion'); this.take('identity'); this.take(';');
      return { kind: 'Repay', obligation, payer, amount, conversion: 'identity' };
    }
    fail('SOURCE6_UNKNOWN_TAG', this.here.start, 'Unknown action');
  }
  private effects(action: Source6Action, asset: string): S0Effect[] {
    this.node(); this.enter(); this.take('effects'); this.take('{');
    const debit = (): S0Effect => { this.effectTag('debit'); const account = this.id(); const amount = this.uint(); this.take(';'); return { kind: 'Debit', account, asset, amount }; };
    const credit = (): S0Effect => { this.effectTag('credit'); const account = this.id(); const amount = this.uint(); this.take(';'); return { kind: 'Credit', account, asset, amount }; };
    const result: S0Effect[] = [debit(), credit()];
    if (action.kind === 'Transfer' && this.here.text === 'credit') result.push(credit());
    if (action.kind === 'Repay') {
      this.effectTag('set_obligation'); const id = this.id(); this.take('principal'); const principal = this.uint(S128);
      this.take('accrued'); const accrued = this.uint(S128); this.take('outstanding'); const outstanding = this.uint(S128);
      this.take('status');
      if (this.here.text !== 'outstanding' && this.here.text !== 'settled')
        fail('SOURCE6_SHAPE', this.here.start, 'Expected obligation status');
      const status = this.here.text === 'settled' ? 'Settled' : 'Outstanding'; this.index++; this.take(';');
      result.push({ kind: 'SetObligation', id, principal, accrued, outstanding, status });
    }
    this.effectTag('use_allowance'); const owner = this.id(); const amount = this.uint(); this.take(';');
    result.push({ kind: 'UseAllowance', owner, amount });
    this.effectTag('use_replay'); const key = this.string(); this.take(';'); result.push({ kind: 'UseReplay', key });
    this.effectTag('advance_head'); const predecessor = this.string(); const successor = this.string(); this.take(';');
    result.push({ kind: 'AdvanceHead', predecessor, successor }); this.take('}'); this.leave();
    return result;
  }
  parse(): Source6Ast {
    this.node(); this.take('profile', 'SOURCE6_VERSION');
    const profileToken = this.here;
    if (profileToken.kind !== 'string' || profileToken.text !== '"moriarty-financial-agreement-source/6"')
      fail('SOURCE6_VERSION', profileToken.start, 'Unsupported source profile');
    this.index++;
    this.take(';'); this.take('agreement'); const programId = this.id(); this.take('{'); this.enter();
    this.take('domain'); const domain = this.id(); this.take(';');
    this.take('settlement'); const asset = this.id(); this.take('scale'); const scale = this.uint(18n); this.take(';');
    this.take('selected'); const actionId = this.id(); this.take('source_hash'); const sourceHash = this.string();
    this.take('digest'); const policyDigest = this.string(); this.take(';');
    this.take('intent'); this.take('{'); this.enter(); this.node();
    this.take('signer'); const signer = this.id(); this.take('key'); const keyRef = this.string(); this.take(';');
    this.take('nonce'); const nonce = this.string(); this.take(';');
    this.take('pre_head'); const preHead = this.string(); this.take(';');
    this.take('valid'); const notBefore = this.uint(); this.take('..'); const notAfter = this.uint(); this.take(';');
    if (BigInt(notBefore) > BigInt(notAfter))
      fail('SOURCE6_RANGE', this.here.start, 'Validity lower bound exceeds upper bound');
    this.take('gross_cap'); const grossCap = this.uint(S128); this.take(';');
    this.take('fee_cap'); const feeCap = this.uint(S128); this.take(';');
    this.take('net_floor'); const netFloor = this.uint(S128); this.take(';');
    this.take('failure'); this.take('success_only'); this.take(';'); this.take('signed_action');
    const signedAction = this.action();
    for (const field of ['observations', 'disclosures', 'retained_effects', 'retained_duties']) {
      this.take(field); this.take('empty'); this.take(';');
    }
    this.take('delegation'); this.take('none'); this.take(';');
    this.take('recovery'); this.take('none'); this.take(';'); this.take('}'); this.leave();
    this.take('authenticated'); this.take('{'); this.enter(); this.node();
    this.take('head'); const head = this.string(); this.take(';');
    this.take('predecessor'); const predecessor = this.string(); this.take(';');
    this.take('round'); const round = this.uint(); this.take(';');
    const balances: { account: string; amount: string }[] = [];
    const balanceCount = signedAction.kind === 'Transfer' ? 3 : 2;
    for (let i = 0; i < balanceCount; i++) {
      this.take('balance', 'SOURCE6_CELL_SHAPE');
      balances.push({ account: this.id(), amount: this.uint() }); this.take(';');
    }
    if (this.here.text === 'balance')
      fail('SOURCE6_CELL_SHAPE', this.here.start, 'Too many balance rows');
    this.take('allowance'); const owner = this.id(); this.take('remaining'); const remaining = this.uint();
    this.take('spent'); const spent = this.uint(); this.take(';');
    let obligation: Source6Obligation | undefined;
    if (signedAction.kind === 'Repay') {
      this.take('obligation', 'SOURCE6_CELL_SHAPE'); const id = this.id(); this.take('{'); this.enter(); this.node();
      this.take('debtor'); const debtor = this.id(); this.take(';');
      this.take('creditor'); const creditor = this.id(); this.take(';');
      this.take('asset'); const obligationAsset = this.id(); this.take(';');
      this.take('principal'); const principal = this.uint(S128); this.take(';');
      this.take('accrued'); const accrued = this.uint(S128); this.take(';');
      this.take('outstanding'); const outstanding = this.uint(S128); this.take(';');
      this.take('status'); this.take('outstanding'); this.take(';'); this.take('}'); this.leave();
      obligation = { id, debtor, creditor, asset: obligationAsset, principal, accrued, outstanding, status: 'Outstanding' };
    }
    if (this.here.text === 'obligation')
      fail('SOURCE6_CELL_SHAPE', this.here.start, 'Unexpected obligation row');
    this.take('replay');
    if (this.here.text !== 'unused' && this.here.text !== 'consumed')
      fail('SOURCE6_SHAPE', this.here.start, 'Expected replay status');
    const replay = this.here.text as 'unused' | 'consumed'; this.index++; this.take(';');
    this.take('work_remaining'); const workRemaining = this.uint(); this.take(';');
    this.take('work_spent'); const workSpent = this.uint(); this.take(';'); this.take('}'); this.leave();
    this.take('submit'); const action = this.action(); const effects = this.effects(action, asset);
    this.take('post_head'); const postHead = this.string(); this.take(';'); this.take('}'); this.leave();
    if (this.tokens[this.index].kind !== 'eof') fail('SOURCE6_SHAPE', this.here.start, 'Trailing source');
    const expectedActionId = signedAction.kind === 'Transfer' ? 'TransferLiteralFee' : 'RepayAccrualFirst';
    if (actionId !== expectedActionId)
      fail('SOURCE6_PROFILE_UNSUPPORTED', this.here.start, 'Selected action is outside the S0 profile');
    const accounts = balances.map((row) => row.account);
    const expectedAccounts = signedAction.kind === 'Transfer'
      ? [signedAction.from, signedAction.to, signedAction.feeTo]
      : [signedAction.payer, obligation?.creditor];
    if (accounts.some((id, i) => id !== expectedAccounts[i]) || new Set(accounts).size !== accounts.length
        || owner !== signer || (signedAction.kind === 'Transfer' &&
          (signedAction.from !== signer || new Set([signedAction.from, signedAction.to, signedAction.feeTo]).size !== 3))
        || (signedAction.kind === 'Repay' && (!obligation || obligation.id !== signedAction.obligation
          || signedAction.payer !== signer || obligation.debtor !== signer || obligation.asset !== asset
          || obligation.creditor === signer)))
      fail('SOURCE6_CELL_SHAPE', this.here.start, 'Authenticated cells do not match action');
    if (BigInt(workRemaining) + BigInt(workSpent) > U128
        || BigInt(remaining) + BigInt(spent) > U128)
      fail('SOURCE6_RANGE', this.here.start, 'Counter total exceeds UInt128');
    if (obligation && BigInt(obligation.principal) + BigInt(obligation.accrued) !== BigInt(obligation.outstanding))
      fail('SOURCE6_RANGE', this.here.start, 'Obligation outstanding must equal principal plus accrued');
    return {
      profile: MIL4_S0_SOURCE, programId, domain, settlement: { asset, scale },
      selected: { actionId, sourceHash, policyDigest },
      intent: { signer, keyRef, nonce, preHead, notBefore, notAfter, grossCap, feeCap, netFloor,
        signedAction, failure: 'success_only', observations: 'empty', disclosures: 'empty',
        retainedEffects: 'empty', retainedDuties: 'empty', delegation: 'none', recovery: 'none' },
      authenticated: { head, predecessor, round, balances, allowance: { owner, remaining, spent },
        obligation, replay, workRemaining, workSpent },
      submitted: { action, effects, postHead },
    };
  }
}

/** Parse one exact Source/6 S0 document. Throws Source6Error on formation failure. */
export function parseSource6(source: string): Source6Ast { return new Parser(lexical(source)).parse(); }

export interface Source6Lowered {
  ast: Source6Ast; state: S0State; intent: S0Intent;
  submittedEffects: S0Effect[]; proposedPostHead: string;
}

/** Lower claims for local Core/5 preparation; no signed digest or external premise is manufactured. */
export function lowerSource6(ast: Source6Ast): Source6Lowered {
  const { authenticated: auth, intent: signed, submitted, settlement, selected } = ast;
  const replayKey = JSON.stringify([ast.domain, signed.signer, signed.nonce]);
  const state: S0State = {
    core: MIL4_S0_CORE, domain: ast.domain, asset: settlement.asset, head: auth.head,
    round: auth.round, workRemaining: auth.workRemaining, workSpent: auth.workSpent,
    balances: auth.balances.map((row) => ({ ...row })), allowances: [{ ...auth.allowance }],
    obligations: auth.obligation ? [{ ...auth.obligation }] : [],
    consumedReplay: auth.replay === 'consumed' ? [replayKey] : [],
  };
  const base = {
    version: MIL4_S0_INTENT, core: MIL4_S0_CORE, sourceProfile: MIL4_S0_SOURCE,
    programId: selected.actionId, sourceHash: selected.sourceHash, policyDigest: selected.policyDigest,
    keyRef: signed.keyRef, domain: ast.domain, asset: settlement.asset,
    signer: signed.signer, nonce: signed.nonce, preHead: signed.preHead,
    notBefore: signed.notBefore, notAfter: signed.notAfter,
    grossCap: signed.grossCap, feeCap: signed.feeCap, netFloor: signed.netFloor,
  };
  const action = signed.signedAction;
  const intent: S0Intent = action.kind === 'Transfer'
    ? { ...base, kind: 'Transfer', recipient: action.to, feeRecipient: action.feeTo,
        amount: action.value, fee: action.fee }
    : { ...base, kind: 'Repay', obligationId: action.obligation, amount: action.amount };
  // Source `use_replay` names a nonce. Core/5 compares its domain/signer/nonce tuple.
  const submittedEffects = submitted.effects.map((effect): S0Effect =>
    effect.kind === 'UseReplay'
      ? { kind: 'UseReplay', key: JSON.stringify([ast.domain, signed.signer, effect.key]) }
      : { ...effect });
  return { ast, state, intent, submittedEffects, proposedPostHead: submitted.postHead };
}

export function parseAndLowerSource6(source: string): Source6Lowered {
  return lowerSource6(parseSource6(source));
}

```

## experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts

sha256: `1b3bd3b01ed2694ea967f62fea6808843a198045d870270b1d44c121081f66d8`

```typescript
/** Provisional Source/6 to Core/5 local preparation. This module cannot admit a ledger stage. */
import { prepareMil4S0, type S0PreparedUnqualified, type S0Rejected } from './mil4-s0-core-v5.ts';
import {
  parseAndLowerSource6, Source6Error,
  type Source6Ast, type Source6Lowered,
} from './financial-agreement-source-v6-frontend.ts';

export type Source6S0Outcome =
  | { status: 'SourceRejected'; code: string; offset: number; publishedPost: null; publishedEffects: null }
  | { status: 'CoreRejected'; ast: Source6Ast; rejection: S0Rejected }
  | {
      status: 'PreparedUnqualified'; ast: Source6Ast; candidate: S0PreparedUnqualified;
      unverifiedBindings: readonly ['agreement-id', 'selected-program', 'asset-scale', 'authenticated-predecessor'];
    };

function sameAction(a: Source6Ast['intent']['signedAction'], b: Source6Ast['submitted']['action']): boolean {
  if (a.kind !== b.kind) return false;
  if (a.kind === 'Transfer' && b.kind === 'Transfer') {
    return a.from === b.from && a.to === b.to && a.feeTo === b.feeTo
      && a.value === b.value && a.fee === b.fee;
  }
  return a.kind === 'Repay' && b.kind === 'Repay'
    && a.obligation === b.obligation && a.payer === b.payer && a.amount === b.amount
    && a.conversion === b.conversion;
}

/** Parse and prepare one S0 stage without asserting source authentication or ledger acceptance. */
export function prepareSource6S0Unqualified(source: string): Source6S0Outcome {
  let lowered: Source6Lowered;
  try {
    lowered = parseAndLowerSource6(source);
  } catch (error) {
    if (!(error instanceof Source6Error)) throw error;
    return {
      status: 'SourceRejected', code: error.code, offset: error.offset,
      publishedPost: null, publishedEffects: null,
    };
  }
  const { ast, state, intent, submittedEffects, proposedPostHead } = lowered;
  const result = prepareMil4S0(state, intent, submittedEffects, proposedPostHead);
  if (result.status === 'Rejected' && result.judgment === 'stage') {
    return { status: 'CoreRejected', ast, rejection: result };
  }
  if (!sameAction(ast.intent.signedAction, ast.submitted.action)) {
    return {
      status: 'CoreRejected', ast,
      rejection: {
        status: 'Rejected', judgment: 'intent', code: 'S0_INTENT_SCOPE',
        diagnosticWork: 1, publishedPost: null, publishedEffects: null,
      },
    };
  }
  return result.status === 'Rejected'
    ? { status: 'CoreRejected', ast, rejection: result }
    : {
        status: 'PreparedUnqualified', ast, candidate: result,
        unverifiedBindings: ['agreement-id', 'selected-program', 'asset-scale', 'authenticated-predecessor'],
      };
}

```

## experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts

sha256: `855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda`

```typescript
/** Provisional MIL/4 S0 preparation. This module never returns ledger admission. */

export const MIL4_S0_CORE = 'moriarty-core/5' as const;
export const MIL4_S0_INTENT = 'moriarty-intent/3' as const;
export const MIL4_S0_SOURCE = 'moriarty-financial-agreement-source/6' as const;
const U128 = (1n << 128n) - 1n;
const S128 = (1n << 127n) - 1n;
const IDENTIFIER = /^[A-Za-z][A-Za-z0-9._-]{0,63}$/;

export interface S0Balance { account: string; amount: string }
export interface S0Allowance { owner: string; remaining: string; spent: string }
export interface S0Obligation {
  id: string; debtor: string; creditor: string; asset: string;
  principal: string; accrued: string; outstanding: string;
  status: 'Outstanding' | 'Settled';
}
export interface S0State {
  core: typeof MIL4_S0_CORE; domain: string; asset: string;
  head: string; round: string; workRemaining: string; workSpent: string;
  balances: S0Balance[]; allowances: S0Allowance[];
  obligations: S0Obligation[]; consumedReplay: string[];
}
interface S0IntentBase {
  version: typeof MIL4_S0_INTENT; core: typeof MIL4_S0_CORE;
  sourceProfile: typeof MIL4_S0_SOURCE; programId: string;
  sourceHash: string; policyDigest: string; signedDigest?: string; keyRef: string;
  domain: string; asset: string; signer: string; nonce: string;
  preHead: string; notBefore: string; notAfter: string;
  grossCap: string; feeCap: string; netFloor: string;
}
export interface S0TransferIntent extends S0IntentBase {
  kind: 'Transfer'; recipient: string; feeRecipient: string;
  amount: string; fee: string;
}
export interface S0RepayIntent extends S0IntentBase {
  kind: 'Repay'; obligationId: string; amount: string;
}
export type S0Intent = S0TransferIntent | S0RepayIntent;

export type S0Effect =
  | { kind: 'Debit'; account: string; asset: string; amount: string }
  | { kind: 'Credit'; account: string; asset: string; amount: string }
  | { kind: 'SetObligation'; id: string; principal: string; accrued: string; outstanding: string; status: 'Outstanding' | 'Settled' }
  | { kind: 'UseAllowance'; owner: string; amount: string }
  | { kind: 'UseReplay'; key: string }
  | { kind: 'AdvanceHead'; predecessor: string; successor: string };

export type S0Judgment = 'stage' | 'intent' | 'effect' | 'authority' | 'history' | 'failure';
export interface S0Rejected {
  status: 'Rejected'; judgment: S0Judgment; code: string;
  diagnosticWork: 1; publishedPost: null; publishedEffects: null;
}
export interface S0PreparedUnqualified {
  status: 'PreparedUnqualified'; core: typeof MIL4_S0_CORE;
  preHead: string; effects: S0Effect[]; candidatePost: S0State;
  requiredPremises: readonly ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume'];
}
export type S0Result = S0Rejected | S0PreparedUnqualified;
export interface S0RequestedOutcome {
  phase: 'TerminalSuccess' | 'RequestedFailure';
  retainedEffects: unknown[];
  retainedDuties: unknown[];
}
/** An experiment assumption. Possession of this tuple does not authenticate it. */
export interface S0LocalStipulation {
  intent: S0Intent; state: S0State; round: string; expectedSuccessor: string;
  requestedOutcome: S0RequestedOutcome;
}
const TERMINAL_SUCCESS: S0RequestedOutcome = {
  phase: 'TerminalSuccess', retainedEffects: [], retainedDuties: [],
};

function reject(judgment: S0Judgment, code: string): S0Rejected {
  return { status: 'Rejected', judgment, code, diagnosticWork: 1, publishedPost: null, publishedEffects: null };
}
function id(value: unknown): value is string {
  return typeof value === 'string' && IDENTIFIER.test(value);
}
function opaque(value: unknown): value is string {
  return typeof value === 'string' && value.length > 0 && value.length <= 1024
    && !/[\u0000-\u001f\u007f]/.test(value);
}
function uint(value: unknown, max: bigint = U128): bigint | null {
  if (typeof value !== 'string' || !/^(0|[1-9][0-9]*)$/.test(value)) return null;
  const parsed = BigInt(value);
  return parsed <= max ? parsed : null;
}
function distinct<T>(values: T[]): boolean { return new Set(values).size === values.length; }
function stableValue(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(stableValue);
  if (value !== null && typeof value === 'object') {
    const record = value as Record<string, unknown>;
    return Object.fromEntries(Object.keys(record).sort().map((key) => [key, stableValue(record[key])]));
  }
  return value;
}
function sameValue(left: unknown, right: unknown): boolean {
  return JSON.stringify(stableValue(left)) === JSON.stringify(stableValue(right));
}
function replayId(value: unknown): value is string {
  if (typeof value !== 'string') return false;
  try {
    const parts: unknown = JSON.parse(value);
    return Array.isArray(parts) && parts.length === 3
      && id(parts[0]) && id(parts[1]) && opaque(parts[2])
      && JSON.stringify(parts) === value;
  } catch { return false; }
}
function boundedAdd(a: bigint, b: bigint): bigint | null {
  const sum = a + b;
  return sum <= U128 ? sum : null;
}
function sameEffects(expected: S0Effect[], supplied: unknown): boolean {
  if (!Array.isArray(supplied) || supplied.length !== expected.length) return false;
  return expected.every((line, index) => {
    const got = supplied[index];
    if (got === null || typeof got !== 'object' || Array.isArray(got)) return false;
    const a = line as unknown as Record<string, unknown>;
    const b = got as Record<string, unknown>;
    return Object.keys(a).length === Object.keys(b).length
      && Object.keys(a).every((key) => JSON.stringify(a[key]) === JSON.stringify(b[key]));
  });
}

/**
 * Prepare a complete local candidate against supplied state.
 * The state, signature and ledger head are not authenticated by this function.
 * A localStipulation is a finite-comparison assumption supplied by the caller;
 * matching it does not authenticate any external fact or qualify the result.
 */
export function prepareMil4S0(
  state: S0State,
  intent: S0Intent,
  submittedEffects: unknown,
  proposedPostHead: string,
  requestedOutcome: S0RequestedOutcome = TERMINAL_SUCCESS,
  localStipulation?: S0LocalStipulation | null,
): S0Result {
  if (state?.core !== MIL4_S0_CORE || intent?.core !== MIL4_S0_CORE
      || intent?.version !== MIL4_S0_INTENT || intent?.sourceProfile !== MIL4_S0_SOURCE
      || (intent?.kind !== 'Transfer' && intent?.kind !== 'Repay')
      || (intent?.kind === 'Transfer' && intent?.programId !== 'TransferLiteralFee')
      || (intent?.kind === 'Repay' && intent?.programId !== 'RepayAccrualFirst')
      || !id(state.domain) || !id(state.asset)
      || !opaque(state.head) || !id(intent.domain) || !id(intent.asset)
      || !Array.isArray(state.balances) || !Array.isArray(state.allowances)
      || !Array.isArray(state.obligations) || !Array.isArray(state.consumedReplay)
      || uint(state.round) === null || uint(state.workRemaining) === null
      || uint(state.workSpent) === null
      || !requestedOutcome || typeof requestedOutcome !== 'object'
      || !['TerminalSuccess', 'RequestedFailure'].includes(requestedOutcome.phase)
      || !Array.isArray(requestedOutcome.retainedEffects)
      || !Array.isArray(requestedOutcome.retainedDuties)
      || boundedAdd(BigInt(state.workRemaining), BigInt(state.workSpent)) === null) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (state.balances.some((v) => !v || typeof v !== 'object')
      || state.allowances.some((v) => !v || typeof v !== 'object')
      || state.obligations.some((v) => !v || typeof v !== 'object')
      || !distinct(state.balances.map((v) => v.account))
      || !distinct(state.allowances.map((v) => v.owner))
      || !distinct(state.obligations.map((v) => v.id))
      || !distinct(state.consumedReplay)
      || state.balances.some((v) => !id(v.account) || uint(v.amount) === null)
      || state.allowances.some((v) => !id(v.owner) || uint(v.remaining) === null || uint(v.spent) === null
        || boundedAdd(BigInt(v.remaining), BigInt(v.spent)) === null)
      || state.obligations.some((v) => !id(v.id) || !id(v.debtor) || !id(v.creditor) || !id(v.asset)
        || uint(v.principal, S128) === null || uint(v.accrued, S128) === null
        || uint(v.outstanding, S128) === null
        || !['Outstanding', 'Settled'].includes(v.status)
        || BigInt(v.principal) + BigInt(v.accrued) !== BigInt(v.outstanding)
        || (v.status === 'Settled') !== (v.outstanding === '0'))
      || state.consumedReplay.some((v) => !replayId(v))) return reject('stage', 'S0_STAGE_UNSUPPORTED');

  if (intent.kind === 'Transfer' && id(intent.signer) && id(intent.recipient) && id(intent.feeRecipient)
      && (!state.balances.some((v) => v.account === intent.signer)
        || !state.balances.some((v) => v.account === intent.recipient)
        || !state.balances.some((v) => v.account === intent.feeRecipient)
        || !state.allowances.some((v) => v.owner === intent.signer)
        || state.obligations.length !== 0
        || state.allowances.length !== 1)) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (intent.kind === 'Transfer' && id(intent.signer) && id(intent.recipient)
      && id(intent.feeRecipient)
      && distinct([intent.signer, intent.recipient, intent.feeRecipient])
      && (state.balances.length !== 3 || state.obligations.length !== 0
        || state.balances[0].account !== intent.signer
        || state.balances[1].account !== intent.recipient
        || state.balances[2].account !== intent.feeRecipient
        || state.allowances.length !== 1
        || state.allowances[0].owner !== intent.signer)) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (intent.kind === 'Repay' && id(intent.signer) && id(intent.obligationId)) {
    const debt = state.obligations.find((v) => v.id === intent.obligationId);
    if (!debt || debt.asset !== state.asset || debt.debtor !== intent.signer
        || debt.creditor === intent.signer
        || debt.status !== 'Outstanding'
        || !state.balances.some((v) => v.account === intent.signer)
        || !state.balances.some((v) => v.account === debt.creditor)
        || !state.allowances.some((v) => v.owner === intent.signer)
        || state.balances.length !== 2 || state.obligations.length !== 1
        || state.allowances.length !== 1
        || state.balances[0].account !== intent.signer
        || state.balances[1].account !== debt.creditor
        || state.allowances[0].owner !== intent.signer) {
      return reject('stage', 'S0_STAGE_UNSUPPORTED');
    }
  }
  if (localStipulation !== undefined && (localStipulation === null
      || !sameValue(localStipulation.intent, intent)
      || !sameValue(localStipulation.state, state)
      || localStipulation.round !== state.round
      || !opaque(localStipulation.expectedSuccessor)
      || !sameValue(localStipulation.requestedOutcome, requestedOutcome))) {
    return reject('stage', 'S0_STAGE_PREMISE');
  }

  const notBefore = uint(intent.notBefore);
  const notAfter = uint(intent.notAfter);
  const grossCap = uint(intent.grossCap, S128);
  const feeCap = uint(intent.feeCap, S128);
  const netFloor = uint(intent.netFloor, S128);
  if (!id(intent.programId) || !opaque(intent.sourceHash) || !opaque(intent.policyDigest)
      || (intent.signedDigest !== undefined && !opaque(intent.signedDigest)) || !opaque(intent.keyRef)
      || !id(intent.signer) || !opaque(intent.nonce) || !opaque(intent.preHead)
      || intent.domain !== state.domain || intent.asset !== state.asset
      || notBefore === null || notAfter === null || notBefore > notAfter
      || grossCap === null || feeCap === null || netFloor === null
      || BigInt(state.round) < notBefore || BigInt(state.round) > notAfter) {
    return reject('intent', 'S0_INTENT_SCOPE');
  }

  const replayKey = JSON.stringify([state.domain, intent.signer, intent.nonce]);
  const balances = state.balances.map((v) => ({ ...v }));
  const allowances = state.allowances.map((v) => ({ ...v }));
  const obligations = state.obligations.map((v) => ({ ...v }));
  const ownerBalance = balances.find((v) => v.account === intent.signer);
  const ownerAllowance = allowances.find((v) => v.owner === intent.signer);
  let gross: bigint;
  let effects: S0Effect[];

  if (intent.kind === 'Transfer') {
    const v = uint(intent.amount, S128);
    const fee = uint(intent.fee, S128);
    if (!id(intent.recipient) || !id(intent.feeRecipient) || v === null || fee === null || v === 0n) {
      return reject('intent', 'S0_INTENT_SCOPE');
    }
    gross = v + fee;
    if (fee > feeCap || gross > grossCap || v < netFloor) return reject('intent', 'S0_INTENT_SCOPE');
    if (!distinct([intent.signer, intent.recipient, intent.feeRecipient])) {
      return reject('intent', 'S0_INTENT_ALIAS');
    }
    const recipient = balances.find((row) => row.account === intent.recipient);
    const feeRecipient = balances.find((row) => row.account === intent.feeRecipient);
    if (!ownerBalance || !recipient || !feeRecipient) return reject('stage', 'S0_STAGE_UNSUPPORTED');
    if (BigInt(ownerBalance.amount) < gross
        || boundedAdd(BigInt(recipient.amount), v) === null
        || (fee > 0n && boundedAdd(BigInt(feeRecipient!.amount), fee) === null)) {
      return reject('effect', 'S0_EFFECT_RANGE');
    }
    ownerBalance.amount = (BigInt(ownerBalance.amount) - gross).toString();
    recipient.amount = (BigInt(recipient.amount) + v).toString();
    if (fee > 0n) feeRecipient!.amount = (BigInt(feeRecipient!.amount) + fee).toString();
    effects = [
      { kind: 'Debit', account: intent.signer, asset: state.asset, amount: gross.toString() },
      { kind: 'Credit', account: intent.recipient, asset: state.asset, amount: v.toString() },
      ...(fee > 0n ? [{ kind: 'Credit' as const, account: intent.feeRecipient, asset: state.asset, amount: fee.toString() }] : []),
    ];
  } else if (intent.kind === 'Repay') {
    const n = uint(intent.amount, S128);
    if (!id(intent.obligationId) || n === null || n === 0n) return reject('intent', 'S0_INTENT_SCOPE');
    const obligation = obligations.find((row) => row.id === intent.obligationId);
    if (!obligation || obligation.asset !== state.asset || obligation.debtor !== intent.signer
        || obligation.status !== 'Outstanding') return reject('stage', 'S0_STAGE_UNSUPPORTED');
    const p = BigInt(obligation.principal);
    const a = BigInt(obligation.accrued);
    gross = n;
    if (gross > grossCap || feeCap !== 0n || netFloor !== 0n) return reject('intent', 'S0_INTENT_SCOPE');
    if (n > BigInt(obligation.outstanding)) return reject('effect', 'S0_EFFECT_RANGE');
    const creditor = balances.find((row) => row.account === obligation.creditor);
    if (!ownerBalance || !creditor) return reject('stage', 'S0_STAGE_UNSUPPORTED');
    if (BigInt(ownerBalance.amount) < n || boundedAdd(BigInt(creditor.amount), n) === null) {
      return reject('effect', 'S0_EFFECT_RANGE');
    }
    const da = n < a ? n : a;
    const dp = n - da;
    const afterP = p - dp;
    const afterA = a - da;
    const afterOutstanding = afterP + afterA;
    ownerBalance.amount = (BigInt(ownerBalance.amount) - n).toString();
    creditor.amount = (BigInt(creditor.amount) + n).toString();
    obligation.principal = afterP.toString();
    obligation.accrued = afterA.toString();
    obligation.outstanding = afterOutstanding.toString();
    obligation.status = afterOutstanding === 0n ? 'Settled' : 'Outstanding';
    effects = [
      { kind: 'Debit', account: intent.signer, asset: state.asset, amount: n.toString() },
      { kind: 'Credit', account: obligation.creditor, asset: state.asset, amount: n.toString() },
      { kind: 'SetObligation', id: obligation.id, principal: obligation.principal,
        accrued: obligation.accrued, outstanding: obligation.outstanding, status: obligation.status },
    ];
  } else return reject('stage', 'S0_STAGE_UNSUPPORTED');

  effects.push(
    { kind: 'UseAllowance', owner: intent.signer, amount: gross.toString() },
    { kind: 'UseReplay', key: replayKey },
    { kind: 'AdvanceHead', predecessor: intent.preHead, successor: proposedPostHead },
  );
  if (!sameEffects(effects, submittedEffects)) {
    return reject('effect', 'S0_EFFECT_MISMATCH');
  }
  if (!ownerAllowance || BigInt(ownerAllowance.remaining) < gross
      || boundedAdd(BigInt(ownerAllowance.spent), gross) === null
      || BigInt(state.workRemaining) < 1n || boundedAdd(BigInt(state.workSpent), 1n) === null) {
    return reject('authority', 'S0_AUTH_SCOPE');
  }
  ownerAllowance.remaining = (BigInt(ownerAllowance.remaining) - gross).toString();
  ownerAllowance.spent = (BigInt(ownerAllowance.spent) + gross).toString();
  if (intent.preHead !== state.head) return reject('history', 'S0_HISTORY_STALE');
  if (state.consumedReplay.includes(replayKey)) return reject('history', 'S0_HISTORY_REPLAY');
  if (!opaque(proposedPostHead) || proposedPostHead === state.head) {
    return reject('history', 'S0_HISTORY_SUCCESSOR');
  }
  if (localStipulation !== undefined
      && proposedPostHead !== localStipulation!.expectedSuccessor) {
    return reject('history', 'S0_HISTORY_SUCCESSOR');
  }
  if (requestedOutcome.phase !== 'TerminalSuccess'
      || requestedOutcome.retainedEffects.length !== 0
      || requestedOutcome.retainedDuties.length !== 0) {
    return reject('failure', 'S0_FAILURE_UNSUPPORTED');
  }

  return {
    status: 'PreparedUnqualified', core: MIL4_S0_CORE, preHead: state.head, effects,
    candidatePost: {
      ...state, balances, allowances, obligations,
      consumedReplay: [...state.consumedReplay, replayKey],
      head: proposedPostHead,
      workRemaining: (BigInt(state.workRemaining) - 1n).toString(),
      workSpent: (BigInt(state.workSpent) + 1n).toString(),
    },
    requiredPremises: ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume'],
  };
}

```

## experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md

sha256: `725662ae7572a93cbc8f0d1631ca364a56d1c3dff3b9b02cae92485cd5a899ec`

```text
# Financial agreement Source/6 and Core/5 S0 contract

**Status:** provisional Sprint 0 specification, 2026-09-29. The grammar is a closed presentation syntax for one S0 stage proposal, and an isolated Source/6 parser and local Core/5 preparer implement this first slice. It is not a signed `/3` encoding or an adopted MIL/4 profile. W-D0–W-D4 remain open. See the [MIL/4 semantic contract](../../formal/mil4/semantics-contract.md) and [S0 implementation contract](../../formal/mil4/s0-implementation-contract.md).

## Formation and version gate

The [EBNF](financial-agreement-source-v6-grammar.ebnf) parses exactly one `profile`, one `agreement`, and one stage proposal. Its header must contain the raw token `"moriarty-financial-agreement-source/6"` before parsing the agreement body; a differently escaped spelling is outside this provisional presentation. The fixed declaration and field order is presentation syntax, not a claim about signed bytes. Unknown, repeated, omitted or out-of-order fields reject at Source/6 formation with `SOURCE6_SHAPE`; an unknown action or effect tag rejects with `SOURCE6_UNKNOWN_TAG`. A different header rejects with `SOURCE6_VERSION`. The closed source grammar also rejects a nonempty failure, observation, disclosure, retained-effect or duty form at formation with `SOURCE6_SHAPE`; `S0_FAILURE_UNSUPPORTED` applies only when a typed Core/5 stage reaches the failure judgment. These source rejections have no Core/5 term or K transition. A local Source/5 parser test confirms that a `/6` header rejects with `PROFILE_MISMATCH`; Core/4-to-Core/5 compatibility has not been implemented or verified here.

The grammar borrows only the token definitions from `lexical.md`; all literal words in the Source/6 EBNF are profile-local reserved words and cannot be identifiers. This does not alter Source/5 keywords. Limits are simultaneous: source UTF-8 bytes ≤65536, tokens ≤8192, AST nodes ≤8192, nesting depth ≤64, identifiers ≤64 ASCII characters, decoded strings ≤1024 UTF-8 bytes, and one stage/effect vector per document. Opaque string fields in this S0 presentation must be nonempty. `scale` is 0..18; each nominal amount (`value`, `fee`, `amount`, caps and floor) is 0..`2^127−1`; each balance, allowance counter and work count is 0..`2^128−1`. S0 uses checked UInt128 intermediates, including `value+fee`, `principal+accrued`, credits and spent counters. The current source/kernel nominal bound does not by itself impose the same bound on every lifecycle state field. This S0 proposal additionally caps principal, accrued and outstanding at `2^127−1` and requires `outstanding=principal+accrued`. W-D4 must review that additional narrowing. Invalid bounds, including an inverted validity interval, reject before K admission with `SOURCE6_RANGE`.

Source/6 formation applies an action-dependent shape check after the closed EBNF parses: transfer requires three balance rows in owner, recipient, fee-recipient order and no obligation; repay requires two rows in payer, bound-creditor order and exactly one obligation. All endpoint identifiers are pairwise distinct for transfer; payer and creditor differ for repay. Duplicate or missing authenticated cells reject `SOURCE6_CELL_SHAPE`; an absent receiver balance is not silently initialized. The allowance owner equals signer. The replay row states whether the selected signed key is unused or consumed; a consumed row rejects at history. The authenticated block is a *claim* until a snapshot-to-head premise establishes each cell and the current head. The strings used for key, nonce, digest and heads are opaque typed identifiers in this syntax, not hash byte definitions.

## Source fields to Core/5

One elaboration proposes `Core5Stage(pre, action, signedScope, submittedEffects, premises)`. The current direct parser preserves agreement ID, asset scale and authenticated predecessor on its AST, but the local Core preparer does not bind those three fields or an exact `/3` digest. It reports the candidate as `PreparedUnqualified`. The Core/5 tags below are proposed typed constructors. No caller Boolean can establish authentication or signature validity.

The local intent object carries the proposed version label `moriarty-intent/3`. That label is a type discriminator only. Source/6 lowering does not encode canonical `/3` bytes or calculate their digest, and no verifier consumes this object as a signed authorization.

| Source/6 field or form | Core/5 field or constructor | Rule |
| --- | --- | --- |
| `profile`, `agreement`, `domain`, `settlement` | Proposed `Version(Source6,Core5)`, `AgreementId`, `DomainId`, `AssetId(scale)` | The parser retains all four. The current local Core preparer does not bind `AgreementId` or scale; these remain named unverified bindings. |
| `selected … source_hash … digest` | Proposed `SelectedProgram(actionId,sourceHash,policyDigest)` | The current Core field named `programId` contains the selected **action ID**, not the agreement ID. Binding all three to a signed statement remains unverified. S0 uses exact action IDs `TransferLiteralFee` and `RepayAccrualFirst`, matching the signed and submitted constructor. Any other action ID rejects `SOURCE6_PROFILE_UNSUPPORTED`. |
| `signer … key`, `nonce`, `pre_head`, `valid` | `SignedScope(signer,keyRef,replayKey,preHead,roundLo,roundHi)` | `replayKey=(domain,signer,nonce)`; the exact digest and verifier are external typed premises. |
| `signed_action` | `SignedAction(TransferLiteralFee | RepayAccrualFirst)` | Fix owner/payer, recipients, fee recipient, obligation ID and quantities under the signature. The submitted action must match this signed action exactly. |
| `gross_cap`, `fee_cap`, `net_floor` | `Bounds(grossCap,feeCap,netFloor)` | Bind all three to the signed scope; authority uses gross debit. |
| `failure success_only` and six explicit empty/none fields | `FailurePolicy(SuccessOnly)`, empty observation, disclosure, retained effect and duty, no delegation or recovery | The current parser rejects a nonempty source variant at formation with `SOURCE6_SHAPE`. A separately constructed typed Core/5 requested outcome reaches `S0_FAILURE_UNSUPPORTED` at Failure. The Source/6 parser cannot express that typed hostile input. No accepted fee-bearing failure exists. |
| `authenticated head`, `predecessor`, `round` | `PreHead`, `Predecessor`, `CurrentRound` | Must be authenticated against the same snapshot; current head comparison is atomic with replay consumption. |
| `balance`, `allowance`, `obligation`, `replay`, `work_remaining`, `work_spent` | `BalanceCell`, `AllowanceCell`, `ObligationCell`, `ReplayCell`, `WorkCell` | These are read cells. Obligation binds debtor, creditor, asset, principal, accrued, outstanding and status. Both work counters come from the authenticated snapshot; lowering never resets spent work. |
| `transfer` | `TransferLiteralFee(owner,recipient,feeRecipient,value,fee)` | Owner=signer. Prepare debit gross, recipient credit value, and fee credit only when fee>0. |
| `repay` | `RepayAccrualFirst(obligationId,payer,amount,IdentityConversion)` | Payer=debtor=signer. Read bound creditor and asset from obligation; do not accept caller supplied substitutes. |
| `effects` | `PreparedEffects` comparison candidate | Order and values must equal the internally derived vector below. The supplied vector does not define the effect. In this provisional text profile, `use_replay` supplies the signed nonce; Core derives the domain/signer/nonce replay key. |
| `post_head` | `PostHead` candidate | Must be a valid authenticated successor under the external head-extension premise. |

For transfer, prepare `Debit(owner,v+f)`, `Credit(recipient,v)`, optional `Credit(feeRecipient,f)` when `f>0`, `UseAllowance(owner,v+f)`, `UseReplay(key)`, `AdvanceHead(pre,post)` in exactly that order. The optional line is present iff `f>0`; a zero-valued fee line rejects. Require `v>0`, `f≤feeCap`, `v+f≤grossCap`, `v≥netFloor`, sufficient owner balance and remaining allowance, and no overflow in any receiver or spent counter. Debit and credits conserve the same nominal asset. Preserve gross effects even though balance changes could be netted. The local wrapper checks Stage first, then requires the transfer in `submit` to equal `signed_action`; it withholds any candidate prepared during that local check when they differ.

For repay, require one Outstanding obligation, `0<n≤outstanding`, matching asset, identity conversion `(mantissa=1,scale=0,rounding=none)`, `feeCap=0`, `netFloor=0`, sufficient payer balance and allowance, and no overflow in the creditor or spent counter. Let `da=min(n,accrued)` and `dp=n−da`. Prepare `Debit(payer,n)`, `Credit(boundCreditor,n)`, `SetObligation(p−dp,a−da,p+a−n,status')`, `UseAllowance(payer,n)`, `UseReplay(key)`, `AdvanceHead(pre,post)`. `status'` is Settled iff outstanding becomes zero. Every unlisted authenticated cell is unchanged. A debt reduction without the creditor credit rejects. The local wrapper checks Stage first, then requires the repayment in `submit` to equal `signed_action`; it withholds any candidate prepared during that local check when they differ.

## Admission and observation

Core/5 applies `stage → intent → effect → authority → history → failure`. `stage` checks version, selected program, typed cells and authentication premises. `intent` checks exact signed scope, validity, endpoints, alias policy and all signed nominal caps and floors. `effect` computes and compares the complete ordered vector and post cells. `authority` checks signer, allowance and work budget. `history` compares the current head and unused replay key and verifies successor binding. `failure` accepts only terminal success with empty retained effects and duties. The first failing judgment returns `(judgment,code,diagnosticWork)` without a published post-state or effects. Within-judgment code spelling and precedence remain W-D3 choices; provisional codes are in `s0-implementation-contract.md`.

The result shape is `Core5Observation(pre,action,signedScope,preparedEffects,post,remainingDuty,remainingWork,replay,preHead,postHead,phase,judgment,code)`. Accepted S0 success contains complete effects, consumption, writes, one post-head and empty duty. Atomic rejection contains the first judgment/code and diagnostic work, with `post`, published effects and post-head absent. Snapshot authentication, exact signature verification, head extension and ledger compare-and-consume are typed external premises; failure or absence rejects. Source/6 grammar acceptance alone does not imply admission.

## Boundary with earlier and later forms

Source/5 `profile`, declarations, `action`, expression and `emit` forms have no automatic injection into this S0 stage. A future migration must map every legacy field, selected Core/4 program, authenticated state, signed digest and complete effect obligation, then prove the old and new observations equivalent on the stated domain. `Repay` in Core/4 consumes a separate Transfer; Core/5's `RepayAccrualFirst` is one funded stage. All old Source/5 and Core/4 behavior remains historical until such a mapping is demonstrated.

The eight MIL/4 first families and all later profiles reject from this S0 grammar with `SOURCE6_PROFILE_UNSUPPORTED` or `SOURCE6_UNKNOWN_TAG` at formation. A later full Source/6 grammar must add typed productions and Core/5 constructors per family. General Φ₁, uncertified Ω, division, rounding, mint, reserve, foreign evidence, accepted failures and recovery are outside S0. This document does not turn those forms into generic records or strings.

```

## experiments/moriarty-language/formal/mil4/effect-wire/SPEC.md

sha256: `53690f52df23dc20c7bf464e266f08fc233829f0d065f94b6404317bb5f945f8`

```text
# W-D2E finite effect commitment candidate

Experiment specification; separate from the frozen S1B and W-D2 audit packets.
This proposal does not select normative W-D2 bytes or close wallet, proof,
source/Core correspondence, snapshot authentication, or ledger consumption.

## Construction and dependency

Select a successor head independently of the authorization digest. Local S0
preparation supplies complete ordered effects and required state cells. Encode
the effects and complete pre/post footprint under the scheme below, compute
SHA-256 of these bytes, and insert the lowercase hexadecimal hash into signed
authorization field 26. Only then encode and digest the authorization.

`state + operation + independently selected successor -> effects + footprint ->
effect bytes -> effect commitment -> authorization bytes -> authorization digest
-> external signature`.

The effect schema admits no authorization digest, effect commitment, signature,
proof, signature-valid flag, or head derived from this authorization digest.
Absence of a field cannot prove an external caller selected an independent head;
that dependency remains a producer/protocol premise. The consumer decodes the
existing canonical authorization bytes and compares field 26 with a recomputed
hash. A successful comparison returns `CommitmentEqualUnqualified`. It neither
verifies a signature nor prepares, authenticates, or consumes a state transition.
An attacker who can replace both bytes and commitment can obtain equality.

## Canonical encoding

The header is ASCII `moriarty-s0-effects/1` followed by NUL (22 bytes).
All integers and counts use big endian. No field tags, optional fields, trailing
metadata, or generic extensions exist. Property insertion order has no effect;
array order is preserved and committed. The implementation is an encoder only.

Primitive `id`: UInt16 byte length, then 1–64 ASCII bytes matching
`[A-Za-z0-9][A-Za-z0-9._:/-]*`. `hash`: 32 raw bytes from exactly 64 lowercase
hex digits. `u64`: eight bytes, cap 2^64-1. `u128`: sixteen bytes, cap 2^128-1.
`nominal`: sixteen bytes, cap 2^127-1. JSON integers are canonical unsigned
decimal strings; JSON numbers, signs, leading zeros, or exponents reject.
`status`: byte 01 Outstanding, 02 Settled. `kind`: byte 01 transfer, 02 repayment.
Scale is one integer byte, cap 38. Core is fixed `moriarty-core/5` byte 05.
Arrays have UInt16 counts. Maximum record 4096 bytes, effects 6, balances 3,
allowances 1, obligations 1, replay history 16 entries per array. These are
experimental finite bounds, with no claim about network or wallet feasibility.

Root object keys and their exact byte order:

1. `schemaVersion`: exact `moriarty-s0-effects/1`, represented by the header.
2. `core`: fixed Core/5 byte; `domain`: id; `asset`: id; `scale`: byte;
   `operationKind`: kind; `round`: u64; `preHead`: hash; `successor`: hash.
3. `effects`: count then each line in supplied order, using variants below.
4. `footprint`: object with `balances`, `allowances`, `obligations`, in that order.
5. `consumption`: object with work counters followed by replay arrays below.
6. `requiredPremises`: exact ordered four-string array below, encoded byte 0f.

Effect variants (one kind byte followed by the fields in this order):

| Kind byte | Kind | Fields |
| --- | --- | --- |
| 01 | Debit | account:id, asset:id, amount:nominal |
| 02 | Credit | account:id, asset:id, amount:nominal |
| 03 | SetObligation | id:id, principal:nominal, accrued:nominal, outstanding:nominal, status:status |
| 04 | UseAllowance | owner:id, amount:nominal |
| 05 | UseReplay | key:replay |
| 06 | AdvanceHead | predecessor:hash, successor:hash |

`replay` JSON presentation is exactly `JSON.stringify([domain, signer, nonce])`
with domain and signer satisfying id and nonce satisfying hash. Encode three
typed components (id,id,hash), without the JSON quotes or punctuation. This is
the Core/5 composite replay key presentation for these finite fixtures. A
whitespace-modified or otherwise noncanonical JSON spelling rejects.

Each footprint array has a count then rows:

| Array | Fields in each row |
| --- | --- |
| balances | account:id, before:u128, after:u128 |
| allowances | owner:id, remainingBefore:u128, spentBefore:u128, remainingAfter:u128, spentAfter:u128 |
| obligations | id:id, debtor:id, creditor:id, asset:id, principalBefore:nominal, accruedBefore:nominal, outstandingBefore:nominal, statusBefore:status, principalAfter:nominal, accruedAfter:nominal, outstandingAfter:nominal, statusAfter:status |

Consumption fields in exact order are `workRemainingBefore`, `workSpentBefore`,
`workRemainingAfter`, `workSpentAfter` (all u128), then `replayBefore` and
`replayAfter` (each count then replay entries). Head consumption appears both
in the root pre/successor fields and the ordered AdvanceHead line. Allowance
consumption appears both in UseAllowance and its pre/post footprint. These
redundant views deliberately commit disagreements instead of silently erasing
them. Semantic consistency and exact required row selection are producer duties.
The footprint includes all cells admitted by these S0 Core/5 fixtures, including
the unchanged zero-fee recipient balance. No netting, sorting, or deduplication
occurs. Work debit is explicit through both pre/post counters.

`requiredPremises` is exactly `canonical-intent-signature`, `snapshot-to-head`,
`head-extension`, `atomic-ledger-compare-and-consume`, in that order. Byte 0f
records requirements, not claims that they were established. Failure, retained
effects and retained duties remain the existing authorization's terminal-success
empty fields; this experiment does not introduce a failure path.

Only exact own enumerable data properties are admitted; inherited, accessor,
missing and extra keys reject. Array indexes must be dense own enumerable data
properties. Reflection on arbitrary proxies is outside the experiment's host
sandbox claims. Primitive bounds precede allocation or integer parsing.

## Independent expected cases, frozen before JS implementation

`reference-vectors.py` supplies literal pre-state, signed terms, ordered effects,
and literal post-state; arithmetic is not delegated to Core/5. Its separate
Python struct/hashlib construction freezes exact bytes and SHA-256 in
`fixtures.json`. JS must not generate or update these expected values.

Positive cases: transfer 10 fee 1 (gross 11); transfer 10 fee 0 (omit fee Credit,
retain fee recipient footprint); AccrualFirst repayment 30 against 1000+10
(980+0); accrued-only repayment 5 (1000+5); full repayment 1010 (0+0 Settled);
near-bound repayment 1 with UInt128 credit/allowance-spent endpoint and signed
nominal principal endpoint. Every case consumes one work unit, appends its exact
replay key, and advances a preselected head.

Hostile cases retain the original authorization commitment: changed Debit
amount, dropped line, appended line, reordered lines, changed recipient,
changed allowance line, changed work counter, omitted balance footprint,
changed balance value, changed allowance pre-state, changed obligation debtor,
changed principal allocation, replay line nonce, replay history, replay domain,
root head, successor line, footprint order, and required premise order.
All encodable mutations must produce different bytes and commitment, and return
`Rejected/EFFECT_COMMITMENT_MISMATCH`; an invalid schema mutation returns a codec
error. Exact per-case expected codes and hashes are frozen in fixtures.

Additional schema controls: extra digest/commitment/signature fields, inherited
fields, accessor fields, numeric values, cap+1, noncanonical decimals, uppercase
hash, malformed replay key, excessive row count, and unknown line kind reject.
Core/5 comparison reads the existing preparation module and compares its result
to every independent pre/post/effect vector; status must remain
`PreparedUnqualified`. This does not prove cross-layer correspondence.

## Stable local diagnostics and limits

Encoder codes: `SHAPE`, `LITERAL`, `ID`, `LENGTH`, `HEX`, `INTEGER`, `RANGE`,
`REPLAY`, `VARIANT`. Equality mismatch is `EFFECT_COMMITMENT_MISMATCH`.
These are experiment codes; W-D3 diagnostics remain open.

Equality demonstrates binding only for the supplied canonical data and frozen
finite cases, subject to SHA-256 assumptions. It does not establish semantic
preparation for arbitrary input, collision resistance empirically, a circuit,
proof soundness, source identity, state/head authentication, replay prevention,
signature or wallet interoperability, native verification, atomic acceptance,
or financial ledger settlement. W-D2 and all these acceptance gates remain open.

```

## experiments/moriarty-language/formal/mil4/effect-wire/codec.mjs

sha256: `1b47621bac1ff3ecb0c87eb730352739b155824df7e14f6b98333ddb044b55ad`

```javascript
// W-D2E finite candidate. Equality is not authentication or ledger admission.
import { createHash } from 'node:crypto';
import { decodeAuthorization } from '../wire/codec.mjs';

export const HEADER = 'moriarty-s0-effects/1\0';
export const CAPS = Object.freeze({ recordBytes: 4096, id: 64, effects: 6,
  balances: 3, allowances: 1, obligations: 1, replay: 16,
  u64: (1n<<64n)-1n, u128: (1n<<128n)-1n, nominal: (1n<<127n)-1n });
const PREMISES = ['canonical-intent-signature', 'snapshot-to-head', 'head-extension',
  'atomic-ledger-compare-and-consume'];
const status = ['Outstanding', 'Settled'];
const effects = {
  Debit: [1, [['account','id'],['asset','id'],['amount','nominal']]],
  Credit: [2, [['account','id'],['asset','id'],['amount','nominal']]],
  SetObligation: [3, [['id','id'],['principal','nominal'],['accrued','nominal'],
    ['outstanding','nominal'],['status','status']]],
  UseAllowance: [4, [['owner','id'],['amount','nominal']]],
  UseReplay: [5, [['key','replay']]],
  AdvanceHead: [6, [['predecessor','hash'],['successor','hash']]],
};
const balance = [['account','id'],['before','u128'],['after','u128']];
const allowance = [['owner','id'],['remainingBefore','u128'],['spentBefore','u128'],
  ['remainingAfter','u128'],['spentAfter','u128']];
const obligation = [['id','id'],['debtor','id'],['creditor','id'],['asset','id'],
  ['principalBefore','nominal'],['accruedBefore','nominal'],['outstandingBefore','nominal'],
  ['statusBefore','status'],['principalAfter','nominal'],['accruedAfter','nominal'],
  ['outstandingAfter','nominal'],['statusAfter','status']];
const footprint = [['balances','balances'],['allowances','allowances'],['obligations','obligations']];
const consumption = [['workRemainingBefore','u128'],['workSpentBefore','u128'],
  ['workRemainingAfter','u128'],['workSpentAfter','u128'],
  ['replayBefore','replays'],['replayAfter','replays']];
const root = [['core','literal','moriarty-core/5',5],['domain','id'],['asset','id'],
  ['scale','scale'],['operationKind','operationKind'],['round','u64'],['preHead','hash'],
  ['successor','hash'],['effects','effects'],['footprint','footprint'],
  ['consumption','consumption'],['requiredPremises','premises']];

function fail(code, field) { const e = new Error(`${code}: ${field}`); e.code=code; throw e; }
function own(value, key, name) {
  const d=Object.getOwnPropertyDescriptor(value,key);
  if (!d || !('value' in d) || !d.enumerable) fail('SHAPE',name);
  return d.value;
}
function snapshot(value, names, name, captured = {}) {
  if (!value || typeof value !== 'object' || Array.isArray(value)) fail('SHAPE',name);
  const keys=Reflect.ownKeys(value);
  if (keys.length!==names.length || keys.some(k=>typeof k!=='string'||!names.includes(k))) fail('SHAPE',name);
  const result=Object.create(null);
  for (const key of names) result[key]=Object.hasOwn(captured,key) ? captured[key] : own(value,key,`${name}.${key}`);
  return result;
}
function arraySnapshot(value, max, name) {
  if (!Array.isArray(value)) fail('SHAPE',name);
  const d=Object.getOwnPropertyDescriptor(value,'length');
  if (!d || !('value' in d) || !Number.isInteger(d.value) || d.value<0) fail('SHAPE',name);
  const length=d.value;
  if (length>max) fail('LENGTH',name);
  const names=Array.from({length},(_,i)=>String(i));
  const keys=Reflect.ownKeys(value);
  if (keys.length!==length+1 || keys.some(k=>k!=='length'&&!names.includes(k))) fail('SHAPE',name);
  return names.map(key=>own(value,key,`${name}.${key}`));
}
function uint(value, width) {
  const out=Buffer.alloc(width);
  for(let i=width-1;i>=0;i--){out[i]=Number(value&255n);value>>=8n;}
  return out;
}
function concat(parts,name) {
  const size=parts.reduce((n,b)=>n+b.length,0);
  if(size>CAPS.recordBytes) fail('LENGTH',name);
  return Buffer.concat(parts,size);
}
function record(value,schema,name,captured) {
  const snap=snapshot(value,schema.map(f=>f[0]),name,captured);
  return concat(schema.map(([key,type,literal,byte])=>primitive(snap[key],type,`${name}.${key}`,literal,byte)),name);
}
function line(value,name) {
  if(!value || typeof value!=='object' || Array.isArray(value)) fail('SHAPE',name);
  const kind=own(value,'kind',`${name}.kind`);
  if(typeof kind!=='string' || !Object.hasOwn(effects,kind)) fail('VARIANT',`${name}.kind`);
  const [tag,schema]=effects[kind];
  return record(value,[['kind','literal',kind,tag],...schema],name,{kind});
}
function rows(value,max,name,encode) {
  const values=arraySnapshot(value,max,name);
  return concat([uint(BigInt(values.length),2),...values.map((x,i)=>encode(x,`${name}.${i}`))],name);
}
function primitive(value,type,name,literal,byte) {
  switch(type) {
    case 'literal': if(value!==literal) fail('LITERAL',name); return Buffer.from([byte]);
    case 'id': {
      if(typeof value!=='string') fail('ID',name);
      if(value.length>CAPS.id) fail('LENGTH',name);
      if(!/^[A-Za-z0-9][A-Za-z0-9._:/-]*$/.test(value)) fail('ID',name);
      const bytes=Buffer.from(value,'ascii');
      return Buffer.concat([uint(BigInt(bytes.length),2),bytes]);
    }
    case 'hash':
      if(typeof value!=='string'||value.length!==64||!/^[0-9a-f]{64}$/.test(value)) fail('HEX',name);
      return Buffer.from(value,'hex');
    case 'u64': case 'u128': case 'nominal': {
      if(typeof value!=='string') fail('INTEGER',name);
      if(value.length>(type==='u64'?20:39)) fail('RANGE',name);
      if(!/^(0|[1-9][0-9]*)$/.test(value)) fail('INTEGER',name);
      const n=BigInt(value);
      if(n>CAPS[type]) fail('RANGE',name);
      return uint(n,type==='u64'?8:16);
    }
    case 'scale':
      if(!Number.isInteger(value)||Object.is(value,-0)||value<0||value>38) fail('RANGE',name);
      return Buffer.from([value]);
    case 'status': case 'operationKind': {
      const variants=type==='status'?status:['transfer','repayment'];
      const i=variants.indexOf(value);
      if(i<0) fail('VARIANT',name);
      return Buffer.from([i+1]);
    }
    case 'replay': {
      if(typeof value!=='string'||value.length>202) fail('REPLAY',name);
      let parts;
      try { parts=JSON.parse(value); } catch { fail('REPLAY',name); }
      if(!Array.isArray(parts)||parts.length!==3||JSON.stringify(parts)!==value) fail('REPLAY',name);
      try { return Buffer.concat([primitive(parts[0],'id',name),primitive(parts[1],'id',name),primitive(parts[2],'hash',name)]); }
      catch { fail('REPLAY',name); }
    }
    case 'effects': return rows(value,CAPS.effects,name,line);
    case 'balances': return rows(value,CAPS.balances,name,(v,n)=>record(v,balance,n));
    case 'allowances': return rows(value,CAPS.allowances,name,(v,n)=>record(v,allowance,n));
    case 'obligations': return rows(value,CAPS.obligations,name,(v,n)=>record(v,obligation,n));
    case 'replays': return rows(value,CAPS.replay,name,(v,n)=>primitive(v,'replay',n));
    case 'footprint': return record(value,footprint,name);
    case 'consumption': return record(value,consumption,name);
    case 'premises': {
      const values=arraySnapshot(value,4,name);
      if(values.length!==4||values.some((v,i)=>v!==PREMISES[i])) fail('LITERAL',name);
      return Buffer.from([15]);
    }
    default: fail('SHAPE',name);
  }
}
export function encodeEffects(value) {
  const snap=snapshot(value,['schemaVersion',...root.map(f=>f[0])],'effects');
  if(snap.schemaVersion!=='moriarty-s0-effects/1') fail('LITERAL','schemaVersion');
  return concat([Buffer.from(HEADER,'ascii'),...root.map(([k,t,l,b])=>primitive(snap[k],t,k,l,b))],'effects');
}
export function effectCommitment(prepared) {
  return createHash('sha256').update(encodeEffects(prepared)).digest('hex');
}
export function compareEffectCommitment(canonicalAuthorizationBytes,prepared) {
  const authorization=decodeAuthorization(canonicalAuthorizationBytes);
  const commitment=effectCommitment(prepared);
  return authorization.effectCommitment===commitment
    ? {status:'CommitmentEqualUnqualified',commitment}
    : {status:'Rejected',code:'EFFECT_COMMITMENT_MISMATCH'};
}

```

## experiments/moriarty-language/formal/mil4/effect-wire/fixtures.json

sha256: `23fc291984118217503349d3b033f1c56422ea161b7d0935758e55e18183df93`

```json
{
  "status": "W-D2E independent expectations frozen before JS codec; no acceptance",
  "positive": [
    {
      "id": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "TransferLiteralFee",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7777777777777777777777777777777777777777777777777777777777777777",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "11",
        "feeCap": "1",
        "netFloor": "10",
        "effectCommitment": "bc633665d8ae112b5b7a842dd9c02a010c584266dfd23e23afcf443f0fcb7c0e",
        "failurePolicy": "atomic-reject-terminal-success",
        "supplyChanges": [],
        "observations": [],
        "disclosures": [],
        "retainedEffects": [],
        "retainedDuties": [],
        "delegation": "none",
        "recovery": "none",
        "operation": {
          "kind": "transfer",
          "owner": "O",
          "recipient": "R",
          "feeRecipient": "F",
          "amount": "10",
          "fee": "1"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "100"
          },
          {
            "account": "R",
            "amount": "0"
          },
          {
            "account": "F",
            "amount": "0"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "11",
            "spent": "0"
          }
        ],
        "obligations": [],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "TransferLiteralFee",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7777777777777777777777777777777777777777777777777777777777777777",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "11",
        "feeCap": "1",
        "netFloor": "10",
        "amount": "10",
        "kind": "Transfer",
        "recipient": "R",
        "feeRecipient": "F",
        "fee": "1"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "89"
          },
          {
            "account": "R",
            "amount": "10"
          },
          {
            "account": "F",
            "amount": "1"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "0",
            "spent": "11"
          }
        ],
        "obligations": [],
        "consumedReplay": [
          "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
        ]
      },
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "bc633665d8ae112b5b7a842dd9c02a010c584266dfd23e23afcf443f0fcb7c0e"
      }
    },
    {
      "id": "transfer-zero-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "10"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "90"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "4",
              "after": "4"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "20",
              "spentBefore": "3",
              "remainingAfter": "10",
              "spentAfter": "13"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "TransferLiteralFee",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7878787878787878787878787878787878787878787878787878787878787878",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "10",
        "feeCap": "0",
        "netFloor": "10",
        "effectCommitment": "ed5b4bff25bb0669421728744daf24bd15affe7d9c93ea94e6d19d4858a5aa6d",
        "failurePolicy": "atomic-reject-terminal-success",
        "supplyChanges": [],
        "observations": [],
        "disclosures": [],
        "retainedEffects": [],
        "retainedDuties": [],
        "delegation": "none",
        "recovery": "none",
        "operation": {
          "kind": "transfer",
          "owner": "O",
          "recipient": "R",
          "feeRecipient": "F",
          "amount": "10",
          "fee": "0"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "100"
          },
          {
            "account": "R",
            "amount": "0"
          },
          {
            "account": "F",
            "amount": "4"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "20",
            "spent": "3"
          }
        ],
        "obligations": [],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "TransferLiteralFee",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7878787878787878787878787878787878787878787878787878787878787878",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "10",
        "feeCap": "0",
        "netFloor": "10",
        "amount": "10",
        "kind": "Transfer",
        "recipient": "R",
        "feeRecipient": "F",
        "fee": "0"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "90"
          },
          {
            "account": "R",
            "amount": "10"
          },
          {
            "account": "F",
            "amount": "4"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "10",
            "spent": "13"
          }
        ],
        "obligations": [],
        "consumedReplay": [
          "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
        ]
      },
      "expected": {
        "length": 560,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00050100014f0001410000000000000000000000000000000a020001520001410000000000000000000000000000000a0400014f0000000000000000000000000000000a0500014400014f7878787878787878787878787878787878787878787878787878787878787878065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f000000000000000000000000000000640000000000000000000000000000005a000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000400000000000000000000000000000004000100014f00000000000000000000000000000014000000000000000000000000000000030000000000000000000000000000000a0000000000000000000000000000000d0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f78787878787878787878787878787878787878787878787878787878787878780f",
        "commitment": "ed5b4bff25bb0669421728744daf24bd15affe7d9c93ea94e6d19d4858a5aa6d"
      }
    },
    {
      "id": "repay-30",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "980",
            "accrued": "0",
            "outstanding": "980",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "30"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "70"
            },
            {
              "account": "C",
              "before": "0",
              "after": "30"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "100",
              "spentBefore": "0",
              "remainingAfter": "70",
              "spentAfter": "30"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "980",
              "accruedAfter": "0",
              "outstandingAfter": "980",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "RepayAccrualFirst",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7979797979797979797979797979797979797979797979797979797979797979",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "30",
        "feeCap": "0",
        "netFloor": "0",
        "effectCommitment": "e8b64f934305b2b4cc3a374e28dfc7fc1184f9b79da1d8d716a0179b2ec04888",
        "failurePolicy": "atomic-reject-terminal-success",
        "supplyChanges": [],
        "observations": [],
        "disclosures": [],
        "retainedEffects": [],
        "retainedDuties": [],
        "delegation": "none",
        "recovery": "none",
        "operation": {
          "kind": "repayment",
          "obligationId": "L",
          "payer": "O",
          "debtor": "O",
          "creditor": "C",
          "amount": "30",
          "allocation": "AccrualFirst",
          "conversion": "identity"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "100"
          },
          {
            "account": "C",
            "amount": "0"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "100",
            "spent": "0"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "1000",
            "accrued": "10",
            "outstanding": "1010",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "RepayAccrualFirst",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7979797979797979797979797979797979797979797979797979797979797979",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "30",
        "feeCap": "0",
        "netFloor": "0",
        "amount": "30",
        "kind": "Repay",
        "obligationId": "L"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "70"
          },
          {
            "account": "C",
            "amount": "30"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "70",
            "spent": "30"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "980",
            "accrued": "0",
            "outstanding": "980",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": [
          "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
        ]
      },
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000001e020001430001410000000000000000000000000000001e0300014c000000000000000000000000000003d400000000000000000000000000000000000000000000000000000000000003d4010400014f0000000000000000000000000000001e0500014400014f7979797979797979797979797979797979797979797979797979797979797979065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f0000000000000000000000000000006400000000000000000000000000000046000143000000000000000000000000000000000000000000000000000000000000001e000100014f0000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000000460000000000000000000000000000001e000100014c00014f000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f201000000000000000000000000000003d400000000000000000000000000000000000000000000000000000000000003d401000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f79797979797979797979797979797979797979797979797979797979797979790f",
        "commitment": "e8b64f934305b2b4cc3a374e28dfc7fc1184f9b79da1d8d716a0179b2ec04888"
      }
    },
    {
      "id": "repay-accrued-only",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "5"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "5"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "1000",
            "accrued": "5",
            "outstanding": "1005",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "5"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "95"
            },
            {
              "account": "C",
              "before": "0",
              "after": "5"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "100",
              "spentBefore": "0",
              "remainingAfter": "95",
              "spentAfter": "5"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "1000",
              "accruedAfter": "5",
              "outstandingAfter": "1005",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "RepayAccrualFirst",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "5",
        "feeCap": "0",
        "netFloor": "0",
        "effectCommitment": "90b113d7f3d3d20c1ecb093c15db710b9da19d2fa60987ac0668f60d9a2898fa",
        "failurePolicy": "atomic-reject-terminal-success",
        "supplyChanges": [],
        "observations": [],
        "disclosures": [],
        "retainedEffects": [],
        "retainedDuties": [],
        "delegation": "none",
        "recovery": "none",
        "operation": {
          "kind": "repayment",
          "obligationId": "L",
          "payer": "O",
          "debtor": "O",
          "creditor": "C",
          "amount": "5",
          "allocation": "AccrualFirst",
          "conversion": "identity"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "100"
          },
          {
            "account": "C",
            "amount": "0"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "100",
            "spent": "0"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "1000",
            "accrued": "10",
            "outstanding": "1010",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "RepayAccrualFirst",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "5",
        "feeCap": "0",
        "netFloor": "0",
        "amount": "5",
        "kind": "Repay",
        "obligationId": "L"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "95"
          },
          {
            "account": "C",
            "amount": "5"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "95",
            "spent": "5"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "1000",
            "accrued": "5",
            "outstanding": "1005",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": [
          "[\"D\",\"O\",\"7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a\"]"
        ]
      },
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000502000143000141000000000000000000000000000000050300014c000000000000000000000000000003e800000000000000000000000000000005000000000000000000000000000003ed010400014f000000000000000000000000000000050500014400014f7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f000000000000000000000000000000640000000000000000000000000000005f0001430000000000000000000000000000000000000000000000000000000000000005000100014f00000000000000000000000000000064000000000000000000000000000000000000000000000000000000000000005f00000000000000000000000000000005000100014c00014f000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f201000000000000000000000000000003e800000000000000000000000000000005000000000000000000000000000003ed01000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a0f",
        "commitment": "90b113d7f3d3d20c1ecb093c15db710b9da19d2fa60987ac0668f60d9a2898fa"
      }
    },
    {
      "id": "repay-settled",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "1010"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "1010"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "0",
            "accrued": "0",
            "outstanding": "0",
            "status": "Settled"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "1010"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "1010",
              "after": "0"
            },
            {
              "account": "C",
              "before": "0",
              "after": "1010"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "1010",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "1010"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "0",
              "accruedAfter": "0",
              "outstandingAfter": "0",
              "statusAfter": "Settled"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "RepayAccrualFirst",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "1010",
        "feeCap": "0",
        "netFloor": "0",
        "effectCommitment": "006f01a123e3e5b835fadd384f00ee0e4a854f62cf0b1ed1296459d0e53baa90",
        "failurePolicy": "atomic-reject-terminal-success",
        "supplyChanges": [],
        "observations": [],
        "disclosures": [],
        "retainedEffects": [],
        "retainedDuties": [],
        "delegation": "none",
        "recovery": "none",
        "operation": {
          "kind": "repayment",
          "obligationId": "L",
          "payer": "O",
          "debtor": "O",
          "creditor": "C",
          "amount": "1010",
          "allocation": "AccrualFirst",
          "conversion": "identity"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "1010"
          },
          {
            "account": "C",
            "amount": "0"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "1010",
            "spent": "0"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "1000",
            "accrued": "10",
            "outstanding": "1010",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "RepayAccrualFirst",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "1010",
        "feeCap": "0",
        "netFloor": "0",
        "amount": "1010",
        "kind": "Repay",
        "obligationId": "L"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "0"
          },
          {
            "account": "C",
            "amount": "1010"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "0",
            "spent": "1010"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "0",
            "accrued": "0",
            "outstanding": "0",
            "status": "Settled"
          }
        ],
        "consumedReplay": [
          "[\"D\",\"O\",\"7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b\"]"
        ]
      },
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f000141000000000000000000000000000003f202000143000141000000000000000000000000000003f20300014c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020400014f000000000000000000000000000003f20500014400014f7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f000000000000000000000000000003f20000000000000000000000000000000000014300000000000000000000000000000000000000000000000000000000000003f2000100014f000000000000000000000000000003f20000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003f2000100014c00014f000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f20100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b0f",
        "commitment": "006f01a123e3e5b835fadd384f00ee0e4a854f62cf0b1ed1296459d0e53baa90"
      }
    },
    {
      "id": "repay-near-bound",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "170141183460469231731687303715884105726",
            "accrued": "0",
            "outstanding": "170141183460469231731687303715884105726",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "1"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "1",
              "after": "0"
            },
            {
              "account": "C",
              "before": "340282366920938463463374607431768211454",
              "after": "340282366920938463463374607431768211455"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "1",
              "spentBefore": "340282366920938463463374607431768211454",
              "remainingAfter": "0",
              "spentAfter": "340282366920938463463374607431768211455"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "170141183460469231731687303715884105726",
              "accruedBefore": "1",
              "outstandingBefore": "170141183460469231731687303715884105727",
              "statusBefore": "Outstanding",
              "principalAfter": "170141183460469231731687303715884105726",
              "accruedAfter": "0",
              "outstandingAfter": "170141183460469231731687303715884105726",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "RepayAccrualFirst",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "1",
        "feeCap": "0",
        "netFloor": "0",
        "effectCommitment": "f08895084d9ec865fafdbebbcc5dda66d8e40ef961813617a677ab0232226d43",
        "failurePolicy": "atomic-reject-terminal-success",
        "supplyChanges": [],
        "observations": [],
        "disclosures": [],
        "retainedEffects": [],
        "retainedDuties": [],
        "delegation": "none",
        "recovery": "none",
        "operation": {
          "kind": "repayment",
          "obligationId": "L",
          "payer": "O",
          "debtor": "O",
          "creditor": "C",
          "amount": "1",
          "allocation": "AccrualFirst",
          "conversion": "identity"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "1"
          },
          {
            "account": "C",
            "amount": "340282366920938463463374607431768211454"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "1",
            "spent": "340282366920938463463374607431768211454"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "170141183460469231731687303715884105726",
            "accrued": "1",
            "outstanding": "170141183460469231731687303715884105727",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "RepayAccrualFirst",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "1",
        "feeCap": "0",
        "netFloor": "0",
        "amount": "1",
        "kind": "Repay",
        "obligationId": "L"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "0"
          },
          {
            "account": "C",
            "amount": "340282366920938463463374607431768211455"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "0",
            "spent": "340282366920938463463374607431768211455"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "170141183460469231731687303715884105726",
            "accrued": "0",
            "outstanding": "170141183460469231731687303715884105726",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": [
          "[\"D\",\"O\",\"7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c\"]"
        ]
      },
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000102000143000141000000000000000000000000000000010300014c7ffffffffffffffffffffffffffffffe000000000000000000000000000000007ffffffffffffffffffffffffffffffe010400014f000000000000000000000000000000010500014400014f7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f0000000000000000000000000000000100000000000000000000000000000000000143fffffffffffffffffffffffffffffffeffffffffffffffffffffffffffffffff000100014f00000000000000000000000000000001fffffffffffffffffffffffffffffffe00000000000000000000000000000000ffffffffffffffffffffffffffffffff000100014c00014f0001430001417ffffffffffffffffffffffffffffffe000000000000000000000000000000017fffffffffffffffffffffffffffffff017ffffffffffffffffffffffffffffffe000000000000000000000000000000007ffffffffffffffffffffffffffffffe01000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c0f",
        "commitment": "f08895084d9ec865fafdbebbcc5dda66d8e40ef961813617a677ab0232226d43"
      }
    }
  ],
  "hostile": [
    {
      "id": "changed-debit",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "12"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000c020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "a413899541c60a5118f898f971f782acf3ea55dbd826ab5489d66753d32ba173"
      }
    },
    {
      "id": "dropped-line",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 560,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00050100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a0400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "053cd21be0ff7e2b84fcec4d3722c887d333bfe53f8b6e9d5a2cf6458f2bcd86"
      }
    },
    {
      "id": "appended-line",
      "base": "transfer-zero-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "10"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "90"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "4",
              "after": "4"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "20",
              "spentBefore": "3",
              "remainingAfter": "10",
              "spentAfter": "13"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000a020001520001410000000000000000000000000000000a0400014f0000000000000000000000000000000a0500014400014f7878787878787878787878787878787878787878787878787878787878787878065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa0200014600014100000000000000000000000000000001000300014f000000000000000000000000000000640000000000000000000000000000005a000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000400000000000000000000000000000004000100014f00000000000000000000000000000014000000000000000000000000000000030000000000000000000000000000000a0000000000000000000000000000000d0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f78787878787878787878787878787878787878787878787878787878787878780f",
        "commitment": "b497a80a6ee91da5db6828700fc8f68f47a478855678fa3e0dd6b569a76a52d9"
      }
    },
    {
      "id": "line-order",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa0006065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa0500014400014f77777777777777777777777777777777777777777777777777777777777777770400014f0000000000000000000000000000000b0200014600014100000000000000000000000000000001020001520001410000000000000000000000000000000a0100014f0001410000000000000000000000000000000b000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "8b7bd1c1aafd1aca2952f85e4ddbd8bbb5276d0f798ad29e7f97e3b2ba24fba8"
      }
    },
    {
      "id": "recipient",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "X",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001580001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "e5ef93825658e880d7502fac2467d6a05766cfbf439c9733a40c28700f6f8b72"
      }
    },
    {
      "id": "allowance-line",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "10"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000a0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "af174da5ed1d16d89f089c62b7e89837ddff931300052780a3b9db1349689a59"
      }
    },
    {
      "id": "work-counter",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "9",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000090000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "fb76a96c769d0581d2d1c4956a0b2b9beec45e1c6994eb95d70ee6ce16c31809"
      }
    },
    {
      "id": "omitted-footprint",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 548,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "a04cf590396a7e6124787589f58cba97971900b730715fe57fe9e04f9395dbcb"
      }
    },
    {
      "id": "balance-footprint",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "88"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000058000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "f7880298148d0bd4a5a866577c0bcf1804f7f4bacb91a45f05e2d8ef0f7c8090"
      }
    },
    {
      "id": "allowance-prestate",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "1",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "afd61c0a33c4253f0445f35107ad928073de793635845383bd773434dab14b4f"
      }
    },
    {
      "id": "obligation-debtor",
      "base": "repay-30",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "980",
            "accrued": "0",
            "outstanding": "980",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "30"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "70"
            },
            {
              "account": "C",
              "before": "0",
              "after": "30"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "100",
              "spentBefore": "0",
              "remainingAfter": "70",
              "spentAfter": "30"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "X",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "980",
              "accruedAfter": "0",
              "outstandingAfter": "980",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000001e020001430001410000000000000000000000000000001e0300014c000000000000000000000000000003d400000000000000000000000000000000000000000000000000000000000003d4010400014f0000000000000000000000000000001e0500014400014f7979797979797979797979797979797979797979797979797979797979797979065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f0000000000000000000000000000006400000000000000000000000000000046000143000000000000000000000000000000000000000000000000000000000000001e000100014f0000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000000460000000000000000000000000000001e000100014c000158000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f201000000000000000000000000000003d400000000000000000000000000000000000000000000000000000000000003d401000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f79797979797979797979797979797979797979797979797979797979797979790f",
        "commitment": "e0597fa8d561e1b7895475c3078885309024eda2bce0285e3377e4bb6c2e23b4"
      }
    },
    {
      "id": "principal-allocation",
      "base": "repay-30",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "990",
            "accrued": "0",
            "outstanding": "990",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "30"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "70"
            },
            {
              "account": "C",
              "before": "0",
              "after": "30"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "100",
              "spentBefore": "0",
              "remainingAfter": "70",
              "spentAfter": "30"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "980",
              "accruedAfter": "0",
              "outstandingAfter": "980",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000001e020001430001410000000000000000000000000000001e0300014c000000000000000000000000000003de00000000000000000000000000000000000000000000000000000000000003de010400014f0000000000000000000000000000001e0500014400014f7979797979797979797979797979797979797979797979797979797979797979065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f0000000000000000000000000000006400000000000000000000000000000046000143000000000000000000000000000000000000000000000000000000000000001e000100014f0000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000000460000000000000000000000000000001e000100014c00014f000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f201000000000000000000000000000003d400000000000000000000000000000000000000000000000000000000000003d401000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f79797979797979797979797979797979797979797979797979797979797979790f",
        "commitment": "c90dc47483729707d96a5cd666fa6248ceb9c45ed35c7846b33d301b78aecb6b"
      }
    },
    {
      "id": "replay-line",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014feeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "77fd5da6b511b9b3b97f8c130ac51a5606c733cf7d09c6157abe241e18bce8fb"
      }
    },
    {
      "id": "replay-history",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [
            "[\"D\",\"O\",\"eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee\"]"
          ],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 621,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b000000000000000000000000000000000002000000000000000000000000000000070000000000000000000000000000000100000000000000000000000000000008000100014400014feeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "8c5e38faa8bae29fdaae2d64cbb6380d2e6f258e30508f288c22ef20eebda227"
      }
    },
    {
      "id": "replay-domain",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"X\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500015800014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "60a2d27b93beb5eed67299ccb97a3121cc90420569e55a693f889da88df14a46"
      }
    },
    {
      "id": "root-head",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f31000500014400014102010000000000000064bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "1d8175071b704f8af1efecb1c355c07ab04939aa7a114e98070c587294e50fa3"
      }
    },
    {
      "id": "successor-line",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "e782c133039f5d75b7dc2f6eea415234bab5f02d3e2293dab95987552dc423b2"
      }
    },
    {
      "id": "footprint-order",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "F",
              "before": "0",
              "after": "1"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "O",
              "before": "100",
              "after": "89"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00030001460000000000000000000000000000000000000000000000000000000000000001000152000000000000000000000000000000000000000000000000000000000000000a00014f0000000000000000000000000000006400000000000000000000000000000059000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "66de3c01954550edf4d0f78af534a9a6ca358bd803afc4a7c130f8b01d14812b"
      }
    },
    {
      "id": "premise-order",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "atomic-ledger-compare-and-consume",
          "head-extension",
          "snapshot-to-head",
          "canonical-intent-signature"
        ]
      },
      "expectedCode": "LITERAL"
    }
  ]
}

```


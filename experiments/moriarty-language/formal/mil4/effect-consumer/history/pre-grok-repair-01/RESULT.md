# W-D2F design sprint result

**Design only. No consumer implementation or full-consumer positive execution.**
The read-only map covers the authorization header and all 35 signed fields,
all 14 nested operation fields, and the complete Core pre/post-to-W-D2E image
recipe. The independent expectation matrix specifies six positive candidates,
77 hostile cases, and eight literal inspection cases before any implementation.
No expected case was executed in this sprint; these are specified-only values,
not new test successes or an independent reviewer approval.

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
| B09 | Exact authorization digest/message signature verifier in this path |
| B10 | Asset identity, atomic-unit scale and settlement registration |
| B11 | One authenticated head/round/snapshot for every required cell |
| B12 | Signed prior predecessor linkage, distinct from current preHead |
| B13 | Authenticated unused composite replay key and uniqueness policy |
| B14 | Authenticated allowance/grant and work spent/remaining authority |
| B15 | Valid successor and digest-independent head-extension evidence |
| B16 | Complete replay-history transport from Source's selected-key claim |
| B17 | Atomic ledger compare-and-consume/apply for the same digest and effects |

Current proposed first unavailable mapping is B01 at field agreementId, after
well-formed compatible wire/Source inputs and earlier direct predicates.
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
   equality. Preserve absent-provider rejection and unqualified local scope.
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

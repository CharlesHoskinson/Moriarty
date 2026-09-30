# W-D2H decision matrix and design oracles

**Status: proposed / specified-only, 2026-09-30.** No vote, implementation,
test run, hash vector, authentication or acceptance result is recorded here.
The [SPEC](SPEC.md) supplies exact recommended payloads and ordering.

## Source strategy comparison

| Criterion | A: parsed-location document projection | B: canonical selected-definition projection |
| --- | --- | --- |
| Hash purpose | 02 | 01 |
| Included bytes | Exact UTF-8 document except two replaced selected claim tokens; all remaining comments, whitespace, runtime state and proposal terms | Binary nine-field tuple: Source version/profile, wire profile, agreement, domain, settlement asset/scale, selected builtin and constructor |
| Excluded claims | Only selected sourceHash/policyDigest string tokens are replaced with exact distinct fixed markers | All digest claims; policy/runtime/submission fields are outside this definition image |
| Self-reference | None: replacing either claim's whole token fixes the input independently of that claim's value/length | None: both claim leaves absent; separate policy includes computed Source/Core hashes and excludes itself |
| Whitespace/comments | Any remaining byte change changes source image; offsets must be grammar-derived | Source presentation is ignored after formation; oversized/malformed source still rejects before projection |
| Runtime change | Snapshot, nonce, effects and successor change sourceHash | These do not change definition hash; their own signed/authenticated checks remain mandatory |
| Policy change | Policy terms change sourceHash as well as policyHash | Policy terms change policyHash; definition hash stays stable unless scoped definition fields change |
| Builtin identity | Selector bytes included, exact code still needs separate Core commitment | Selector/kind pair included; actual code/lowerer/wrapper committed by Core package |
| Stable reusable meaning | Exact stage presentation with two exclusions; no reusable code identity | Instance-scoped builtin selection/settlement definition; no user-defined action body or template reuse claim |
| Failure risk | Naive mask can miss escaped tokens, mask comments or create unexpected omission; new token-span extractor needed | Overbroad reuse can omit future action-body semantics; reject future profiles until new body image is adopted |
| Consequence of adoption | Every snapshot/presentation update needs fresh Source commitment/signature; digest-independent successor remains required | Explicitly separate definition, exact implementation and bounded policy; later fields checked at their own tags |
| Recommendation | Keep as the fully specified alternative, not a fallback hash mode | Recommend for this closed S0 proposal; still requires independent adoption |

Hashing the entire unmodified Source document is rejected because it contains
its own `source_hash`. Hashing a builtin name alone is rejected as B06 because
that name cannot identify changed code. Hashing only an intent or effect vector
cannot identify the parser/lowerer/wrapper that produced it. No fixed-point
search, caller-selected mask, arbitrary JSON serialization or legacy opaque
claim becomes an accepted strategy by matching a value.

## Core and policy identity choices

| Choice | Benefit | Cost / required evidence | Recommendation |
| --- | --- | --- | --- |
| Closed raw three-module Core package | Exact implementation-source bytes and selected builtin are unambiguous; includes shared lowering/wrapper behavior | Comments/unused branch changes invalidate hash; actual loaded execution and correspondence still need independent evidence | Proposed purpose3 for current host prototype only |
| New canonical abstract Core program image | Could survive implementation presentation changes and identify semantics directly | Core/5 has no canonical program serializer or proved compiler relation; defining an IR is a separate consequential design | Defer; do not invent a name-only stand-in |
| Exact scoped policy with Source/Core links and fixed signed terms | A policy cannot migrate to another definition/package by preserving a digest claim; endpoints/caps/interval/zero-fee recipient are explicit | New amount/interval/key/cap needs a new policy; nonce/head/history remain separate; real policy authority is unavailable | Proposed purpose4, without adopting a general policy language |

## Exact design-oracle conventions

These are independently specified predicates for later implementation and
review, not observed results. Let `S(x)`, `C(p)` and `P(q,S,C)` denote the
complete recommended preimage bytes from SPEC, and `H` SHA256. Construct two
new Source positives with strict W-D2G action IDs, one fee transfer and one
accrual-first repayment. Parseable documents must supply compatible ordered
cells, canonical hash-shaped claims, exact submitted actions/effects and an
independent successor. Existing W-D2E fixtures are not these positives.

Standalone image predicates below need no synthetic authentication. Any
full-path expectation additionally requires independent real passing evidence
for every preceding F/D/M gate, including B01–B04. For an anchor8/11/12 oracle,
all earlier anchors pass. Later Core/image/ledger outcomes are never inferred.
A positive image comparison would establish bytes/content only, with null
published effects/post; it would not establish a full consumer or B17.
Expected digest equality follows identical bytes. Unequal preimages must be
asserted directly; expected unequal SHA256 values additionally rely on the
hash's collision-resistance assumption and are not a proof of injectivity.

| ID | Exact positive or hostile construction | Proposed image / path oracle |
| --- | --- | --- |
| H-P01 | Strict transfer Source selectedActionId/coreProgramId=TransferLiteralFee, constructorTransfer; compute three images from independent selected artifacts, then put their hashes into AST/wire claims | Replacing claims leaves S/P unchanged; artifact digest comparisons at8/11/12 pass as isolated predicates. Full path still requires all other bindings. |
| H-P02 | Strict repayment selectedActionId/coreProgramId=RepayAccrualFirst, constructorRepay, allocationAccrualFirst, conversionidentity, one matching obligation | Image selection/constructor and policy fields pass; repayment C differs from transfer C even for identical package files because ID/kind bytes differ. This is not repayment execution evidence. |
| H-P03 | Add ASCII whitespace/CRLF and comments between tokens, below all Source bounds; preserve parsed values and the exact required raw profile token | Recommended S/P and their digests are byte-identical; A's projected document bytes differ. No broad semantic-equivalence claim follows. |
| H-H01 | Change only Source selected.sourceHash to another canonical 64-hex value and change wire.sourceHash to the same value, leaving authenticated artifact unchanged | S/P producer bytes do not change; tag8 fails sourceHash/computed-artifact equality, W_D2F_FIELD_MISMATCH. Claim equality alone cannot pass B05. |
| H-H02 | Change only Source selected.policyDigest and wire.policyHash together, leaving the selected policy artifact unchanged | No feedback into S/C/P; tag12 policyHash mismatch. An authorizationDigest presented as `digest` is the same hostile case unless it independently equals the correct policy commitment; equal content still cannot supply policy authority. |
| H-H03 | Try to hash the complete original Source bytes with source_hash included, or include policyHash in its own policy payload | Rejected profile/definition; only purpose1 or reviewed purpose2 exclusions are defined. A caller's fixed-point/equal-claim assertion supplies no accepted image. Missing adopted image/provider uses B05/B07 unavailable; invalid evidence under an actual selected verifier uses invalid at the owning anchor. No encoder for such a mode is implemented. |
| H-H04 | Change Transfer's selected.actionId to RepayAccrualFirst without changing the Transfer constructor | Existing Source formation rejects SOURCE6_PROFILE_UNSUPPORTED before image or adapter checks. No tag8/10 oracle is reachable. |
| H-H05 | Keep strict transfer Source and valid B04 provider, change only signed coreProgramId to RepayAccrualFirst | Tag10 coreProgramId mismatch after tag8 passes; never diagnose it as invalid B04 at6. Source definition stays fixed; wrong package is not substituted. |
| H-H06 | Provide authentic alternative package bytes with one Core byte changed, retain the original signed coreHash; otherwise valid selected-program/correspondence evidence for the alternative package | Purpose3 preimage differs; tag11 coreHash mismatch. A real verifier's inability to establish the changed package correspondence instead produces B06 invalid/unavailable before that equality; these are separate prerequisites. No success from unchanged builtin name. |
| H-H07 | Keep the original signed coreHash and exact package files, but run a different lowerer, an uncommitted imported helper or another runtime/compiled artifact | B06 correspondence/loaded-artifact obligation fails (invalid with real verifier, unavailable without it). Matching source package hash alone cannot reach positive execution correspondence. New module closure requires a new package profile. |
| H-H08 | Change only a comment or LF/CRLF in one committed package file, with old coreHash | Core preimage differs and needs a fresh hash/correspondence decision; tag11 coreHash mismatch given independent evidence for new bytes. A comment change in a Source proposal instead follows H-P03. |
| H-H09 | Change only Source/wire grossCap from100 to101 while retaining a valid selected policy with grossCap100, identical nonce/head/definition/package/hash claims | Source definition and selected artifact policy preimages unchanged; tag12 hash checks pass, then tag23 grossCap field mismatch. Do not recompute submitted later terms at12 and reorder the diagnosis. An independent candidate policy producer would encode different P bytes. |
| H-H10 | Change policy artifact's feeRecipient with fee=0, recompute its policy hash, but retain original Source/wire feeRecipient; independently authenticate the new policy for the same required context | New P bytes differ despite no positive fee Credit; hash equality at12 can pass after updating claims. Exact operation.feeRecipient mismatch at35. Earlier signer/interval/caps/empty terms must match. No netted effect or fee=0 exemption. |
| H-H11 | Policy artifact links to another computed Source definition or Core package under a different instance/selector, with a freshly computed policy digest and internally equal claim strings | Required B07 selected-context relation fails at12; equal digest claims are insufficient. With no actual verifier this remains unavailable, not an observed evidence rejection. |
| H-H12 | Same selected policy, vary only fresh nonce/preHead and matching snapshot/successor/effects for a legitimate next proposal | S/C/P preimages remain identical; canonical authorization bytes change. Snapshot/history/replay/head/signature gates must be freshly verified; old signature/replay consumption does not carry over. This is an image invariance oracle, not a full-path positive. |
| H-H13 | Change snapshot balance/work spent, predecessor, unrelated replay history or submitted effect while retaining S/C/P | Their images remain identical. B11/B12/B16 claim checks or Core/effect image comparison must detect the hostile content at its existing anchor. Matching hashes cannot authorize it; no [] history shortcut. |
| H-H14 | Change Source agreement/domain/asset/scale/selected definition, preserve old Source claim and authenticate the changed definition independently | S bytes differ. Earlier domain/agreement/action literal/provider differences win if present; otherwise old computed-source claim rejects at8. A valid provider with later asset/scale differences retains them for16/17, without early tag8 comparison to those submitted fields. |
| H-H15 | Use identical payload bytes x with purpose01 and purpose03, or purpose03 and04; try to relabel a digest/parse payload from the other role | Complete preimages differ at the purpose byte. Typed encoder/decoder rejects a payload of the wrong purpose/shape; raw digest equality cannot authenticate the missing scoped relation. Even a deliberately assumed SHA256 collision must not enable cross-sort coercion. Do not assert a mathematical proof that SHA256 cannot collide. |
| H-H16 | Change only raw JSON escape spelling of keyRef while preserving its decoded scalar value; keep legal Source profile's raw header unchanged | P text bytes/digest unchanged; A document bytes change. Different decoded keyRef changes P and is checked under the policy/signer relation; neither spelling converts keyRef to signerKey. |
| H-H17 | Change purpose/image version, add an unknown slot, use a noncanonical ID/round/scale, omit a package role, reorder roles or append a byte | Proposed image format rejects; no alternate-suite retry. Source formation/common-domain failures retain their earlier diagnostics. No new image-code execution is claimed. |
| H-H18 | Use unchanged image/claims but omit every authentication provider | Current planned full path stops at agreementId/B01 after F/D/direct checks; standalone content predicates remain possible. Do not fabricate providers to report that8/11/12 passed end-to-end. |

No binary/vector files or tests are created. A later implementation sprint must
freeze independently generated exact bytes, claimed-versus-computed hashes and
expected outcomes before its encoder/consumer is written, then exercise these
cases through the actual callable path. Real signature/snapshot/native/ledger
controls are additional evidence, not implied by this table.

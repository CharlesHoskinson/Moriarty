# Fresh Sol design vote — signed native v2

**Verdict: APPROVE.** This vote approves the exact v2 design contract for the bounded local, single-owner integrated signed financial transition. It is a consequential DESIGN vote only. It does not approve an implementation, imported verifier source, circuit soundness, resource allocation, native execution, publication, merge, ledger acceptance or closure of M7/M8/MC05/U0–U7.

Reviewer assignment: fresh independent `gpt-6.1-sol`, high effort; agent `/root/sol_signed_native_v2_design_vote`. Requested model/effort are recorded separately from provider attestation: this Markdown content is not a provider-returned identity receipt. No other current or original auditor findings or verdicts were read. Root's reported reproduction success was received after my own independent reproduction; it was not used to decide the arithmetic result or this vote.

## Exact scope and commitments

- Checkout: `/home/charl/Moriarty/.worktrees/native-intents-20261011`.
- Basis HEAD: `3748fe8278287a3729c17c5ddc245f669e21e847`.
- Basis tree: `85ad41f4d0a2ff2c3f9576bba9a70a95c7f42b9c`; source status was clean at startup and final inspection.
- Normative packet `next-signed-native-design-v2.md`: SHA256 `a4a012366c8d202b3766fab00b5933f97b30fea522c07cae28c465678a1fb142`.
- Normative laboratory vector `signed-native-golden-vector-v2.json`: SHA256 `9c035c8e42e2b378e8ed2b3388767d3a8d60ac7691aafd3d45dc7066eeb285f0`.
- Authorship manifest `next-signed-native-design-v2-manifest.json`: SHA256 `818dd4a3d348b851b59c2c562bb5bdb1fd193288e93ea4b4fdf933608a3e57e5`.
- Read-only interface intake `next-jubjub-interface-intake.md`: SHA256 `bd1fc1709ec808e96c82f375ecbf945be7fb78a9a2b8734ba7aa5695f645d906`.

All 24 entries in the v2 manifest, including cached library hashes, were independently rehashed without mismatches. That establishes the identified inputs, not endorsement of author execution or imported library binaries. The author tooling/network failure and partial toolchain installation remain disclosed; this vote does not excuse or authorize them.

Startup loaded the installed Moriarty develop skill, checkout `AGENTS.md`, October 10 routing assignment, and guarded status. Status still reports stale older campaign inputs/accounting/resource state and no financial transaction evidence. Those records do not grant native admission.

## Production inspection and decision rationale

I inspected current Moriarty `mori-core/src/intent.rs`, `json.rs`; backend `lib.rs`, `liabilities.rs`, `proving.rs`; and production CLI `intent`, `verify_intent`, `prove`, `verify`, `proof_io` consumers. The baseline has UInt/Bool financial public assignments with unconditional Impact, complete registered duty presence/amount frames, no private signature inputs, a host claim binding, and consumer-derived source circuit/VK. The proposed typed backend-only authorization ABI and explicit input-bearing preparation changes are therefore necessary real consumer changes, rather than a claim that today's backend already consumes signatures.

I also inspected the actual pinned ledger hash, point encoding/assignment, scalar conversion and Schnorr routes; resolved `midnight-circuits 7.2.4` subgroup assignment and Poseidon code/constants; resolved `midnight-curves 0.3.1` generator; and the observed Fenrir `sig/spec.rs`, strict reference and emitter sources. The FENS challenge order matches those selected sources. Generic pinned Schnorr lacks FENS and cannot substitute. Point assignment supplies constrained subgroup points; extracting their actual coordinates and equating PK to the public expected coordinates is essential. Native-to-scalar reduction alone does not make the response canonical; the prescribed roundtrip equality does.

The packet supplies an implementable, acyclic contract: context is an 18-element source-fixed array excluding all constant values and authorization/runtime values; context constants are inserted after layout staging; unsigned payload hashing excludes signatures, message and current time; MOI1 hashing includes all original constrained financial values, exact tags/counts/assets/caps and metadata, while excluding auth-added values. Two SHA limbs are independently reconstructed and ranged rather than truncated or reduced. Version3 and explicit authorization mode prevent unsigned/signed or historical revision reinterpretation.

The independent verifier caller owns Target, all expected authority fields and current logical time. Candidate data cannot define key/account/domain/caps, source layout, VK or current time. The design requires reconstruction and comparison of every expected field, complete signed argument groups and refusal before SRS on malformed/downgrade cases. Label identities are explicitly local labels, with ledger ownership/history qualification still open.

Financial constraints use the actual ordered effect wires and every registered closing owner duty. Gross includes transfers and owner fees without refund offsets; fee totals are bounded; checked net subtracts beneficiary transfer debits and owner fees, with beneficiary fees additionally counted only when beneficiary differs from owner. This is the existing `check_effects` arithmetic, including the explicitly specified selftransfer gross10/net0 case. Overflow, underflow, missing authority and source-wide second payer/debtor refusal preserve the intended financial boundary. Limits are source-derived full AssetIds, not abbreviated names, and all caps enter the same signed message wires that constrain accounting.

The native statement is correctly existential in private R/s. Host acceptance independently validates the exact attachment. Because Claim excludes attachment bytes, a second valid attachment for the same payload/key/message may pass the same proof, while an invalid attachment must fail the strict host check. This distinction is explicit and avoids claiming a native attachment commitment that the ABI does not provide.

Strict canonical schema, decoded duplicate-key refusal, canonical UInt/native/scalar encodings, whole-wrapper size admission and preallocation asset/duty ceilings are specified. Prepared report and laboratory vector are expressly distinct schemas. No materially required message, trust-root, signature, cap or consumer binding is left to an alternative invented normative format.

## Independently reproduced fixed-vector observations

A fresh in-memory Python calculation used the normative data and independently interpreted pinned source constants; it did not read or execute the author builder or helper and wrote no files. It reconstructed canonical JSON bytes/SHA256 identities and limbs, all 28 financial values from the small settlement example, context descriptor projection, 18-element context, MOI1 framing and authorization positions. It implemented the unoptimized Poseidon permutation directly from the resolved field constants: width3/rate2, 8 full plus60 partial rounds, fixed input-length capacity, with the partial S-box on the last state coordinate. It independently implemented affine Edwards addition/multiplication using d=−10240/10241 and the resolved generator multiplied by cofactor8.

Observed results:

| Predicate | Independent observation |
|---|---|
| Raw source and seven canonical asset/Program/Scenario/Target/predecessor/payload/context hashes | Match exact vector bytes/digests and available limbs |
| Financial values | N=28; payment7/fee2; closing payment3 and untouched reserve present-zero; alice91/bob17/treasury5; settled7 |
| Public ABI count/positions | 28 financial +15 authorization =43; continuous positions0..42 |
| MOI1 preimage | Exactly75 =13+2×28+6×1 fields, matching every field; context limbs occur in both specified places |
| Native message | `19363418475454014545745970686362478004989189609513260076233824868044537194154` |
| FENS challenge hash | `14646508590963721718195940607939419691497145173904549316643053166178564177939` |
| Challenge reduced modulo r | `1537539797182174098334005480892928232085302642159914753912334841394197669541` |
| Points and signature | Standard subgroup generator; PK=7B; R=11B; PK/R nonidentity and rP=identity; canonical s; sB=R+ePK |
| Encodings | Message/challenge/scalar32LE and key/signature affine32LE bytes match |

These are off-circuit observations only. No signed compiler, integrated native circuit, real proof or deployment was executed. Independent source/compiler correspondence and hostile witness behavior remain future acceptance work.

## Independent required outcomes — specified only

The following are my acceptance expectations for the later real production implementation and controls. They were not executed against a signed circuit in this review.

| Case | Expected result and evidentiary distinction |
|---|---|
| Golden amount7/fee2, caps9/2/5/3, at15 | Host strict signature and financial acceptance; eventual genuine integrated native positive uses the exact production private/public formats |
| Payment70/fee2, closing duty30 +untouched5 +present-zero | Gross72, fees2, net68, total closing duties35; exact caps72/2/68/35 accept |
| Independently re-signed caps71/2/68/35,72/1/68/35,72/2/69/35,72/2/68/34 | Respectively reject gross, fee, net, duty constraint; signature validity cannot replace financial satisfaction |
| Present-zero versus absent-zero, partial/full discharge, untouched positive duty | Preserve presence semantics, canonical absent amount0, and include the entire source duty frame; discharge creates no transfer credit |
| Selftransfer10, owner=beneficiary, nofee | Gross10/net0; positive net floor rejects. Ownerfees debit net once; outgoing payment/fees producing underflow reject |
| Aggregate credit/debit/fees and duties near UInt128 bound | Checked sum overflow or net underflow rejects; aggregation order must not manufacture an intermediate refusal for a valid final host net |
| State-only/observe/zero-finance signed action | Signature remains mandatory; require zero-compatible caps/net floors for its actual financial result |
| Zero/16 assets and duties; asset17/duty17 | Admitted shapes retain complete frames;17 refuses before proportional layout/native allocation |
| Second payer/debtor in unexecuted source action; different-debtor discharge | Source-wide signed admission refuses even when the selected Scenario would not execute the statement |
| Duplicate/aliased source identities; missing/extra/duplicate/reordered limits | Refuse; distinct full AssetIds and numeric scale determine order, not source short names |
| Duplicate decoded JSON keys, unknown/null/missing fields, trailing data, noncanonical decimal/hex/native/scalar, invalid labels | Refuse before SRS; raw and canonical size bounds apply to the complete signed wrapper and exported report |
| Field exactly p; response r/r+1 or honest s+r | Refuse canonical admission/roundtrip, without field or scalar reduction of supplied encodings |
| Response0/1/r−1 | Range-valid; full acceptance still requires the actual equation |
| Identity PK/R, offcurve, pure torsion, mixed-order points | Strict host refusal; later forced witness evidence identifies the actual subgroup/public-cell/equation constraint rather than arbitrary panic/error |
| Changed source/Program/Scenario/Target/action/domain/owner/beneficiary/key/asset/scale/caps/nonce/predecessor/interval | Independently reconstructed expected payload differs; old signature cannot authorize the changed message; valid fresh signatures require admitted matching authority |
| Different expected-authority field, caller time outside interval, inverted interval | Reject. at10/20 inclusive accept with compatible transition;9/21 reject. Changing at inside interval preserves signed message but changes public statement/Claim |
| Disconnected arbitrary msg, unrelated key, disconnected cap wires, wrong MOI1/FENS tag or challenge ordering | Reject through actual reconstruction/equality/signature/financial constraints; a standalone helper success is insufficient |
| Unchanged real proof with changed financial/msg/PK/cap/at public positions | Actual native verification must reject using source-derived VK, with unchanged proof bytes |
| Unchanged real proof plus invalid replacement attachment | Strict host attachment refusal; do not falsely attribute rejection to a changed native statement |
| Unchanged real proof plus second valid same-payload/key/message attachment | May pass the same proof after independent host strict validation; attachment bytes are not committed by native Claim |
| Skipped Impact, wrong field position/count/type, candidate descriptor/VK/artifact substitution | Refuse; every public cell is Impact-bound in reconstructed order and consumer source derives the relation/VK |
| Signed Target with partial/unsigned/legacy dispatch arguments, or stale backend2 artifact/proof | Refuse with version/group diagnostics before SRS; no silent unsigned fallback |

Use identical raw witness format for honest and adverse controls. A negative passes only through an identified expected constraint refusal or verification failure of an actual produced proof; arbitrary panic, allocation error or prover failure is insufficient. Quint must model finite source admission, complete duties, checked arithmetic and atomic refusal, with cryptography an explicit premise; sampling is not M8 closure.

## Remaining authority and acceptance gates

Only this exact design receives this affirmative vote. Root must obtain the other selected fresh substantive agreeing design vote before implementation adoption. Actual adapted source, production caller behavior, maximum admitted integrated IR shape/costs, independent controls and later exact source/result audits remain owed. No proof/SRS/keygen/native/MockProver/ignored-test/resource/network/dependency action was performed or authorized by this review; only this runtime vote file was written.

New native work still requires both separate exact resource votes and an eligible bounded one-proof allocation; the consumed liability allocation cannot be reused. k≤19 is necessary but cannot certify fraud cost or64KB transactions. Ledger identity/key association, authentic external frames/history, clock/nonce consumption/revocation/creditor consent/cumulative caps, recursion/private handoff, Preview financial settlement and wider roadmap gates remain open. Public-label matching and fixed-vector agreement do not close them.

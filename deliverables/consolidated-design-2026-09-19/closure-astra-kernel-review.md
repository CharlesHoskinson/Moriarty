# Expanded-scope final review: kernel and Mina integration

Verdict: **approved — final consolidated design only**.

Reviewed the entire 897-line `review-candidate-v2.txt`, all 11 files in `candidate-v2-manifest.json`, and the appended Mina result, recursion-semantics and developer-integration reports. All 11 embedded files reconstructed from the frozen review packet match the v2 manifest SHA-256 digests and byte counts. The live candidate has the separately reviewed deterministic Pel correction described below; its changed bytes are not mislabeled as v2. The 35 original developer-evidence excerpt hashes still match their pinned source files. No other closure-review outputs were read. No candidate edits, builds, proof runs or deployments were performed.

The expanded candidate addresses the user's Mina request through eight useful mandatory refinements of ZR01–16. It preserves one roadmap, permissionless public programming, the optional federation boundary, native Midnight/ZKIRv3, no Lean dependency, and full native recursive/private composition under the dated March 2027 planning assumption. No blocking defect or necessary correction remains for design adoption.

## Mina fidelity

**MNR03 correctly separates three judgments:** compatible proof shape, key data-to-identity integrity, and authorization of that identity by owner/program policy. This accurately reflects D03–D05. It does not infer consent from a valid DynamicProof or from a host verifier using the supplied key data. Unauthorized key-root mutation remains a distinct negative control. The candidate does not turn program-specific key policy into a public Moriarty maintainer allowlist.

**MNR04 correctly handles D09 auxiliary output.** It distinguishes ordinary verified public statements from auxiliary/witness-only values and requires relevant effect/history dependencies to be constrained. An auxiliary field may still be used with a properly constrained commitment/opening; the candidate does not categorically claim that every auxiliary value can never be authenticated. Its altered-fee/recipient test is an appropriate consumer-boundary test. It also preserves D01/D10's distinction between reading predecessor fields and requesting cryptographic predecessor verification. The inspected example omission is consistently labeled source-only, not a demonstrated deployed exploit.

**MNR04 correctly handles current-state settlement.** A verified permitted successor still needs authoritative current-state preconditions. D06's combination of proof verification, stored pre-state equality and action-state equality supports this lesson. The candidate does not claim that an offchain-state proof alone establishes finality, unique spendability or arbitrary financial correctness.

**MNR05 faithfully captures proofs-disabled behavior.** The developer report identifies the program-specific verification path and distinguishes it from standalone host verification. The requirement demands real proving/complete-verification configuration for production evidence, while leaving simulation/dummy modes available with distinct assurance labels. It does not allege an unproven production vulnerability.

**MNR01/02/06/07 are coherent with the supplied semantic study:** transcript/challenge ordering, complete deferred-obligation lifecycle, active masks/base cases, parameter identity and batching soundness remain explicit. Safe order-insensitive batching is still permitted under ZR08, and MNR07 expressly requires a soundness argument rather than treating finite negative tests as a probabilistic security proof. MNR06's cache and parameter requirements do not elevate an inspected TODO to an exploit claim.

**MNR08 appropriately limits source/audit transfer.** Historical audit coverage is component- and version-specific; the acquired repositories are not represented as one tested release tuple. Mina's Pickles/IPA/Pasta implementation supplies comparative failure cases, not interchangeable proofs, a Midnight backend port or automatic theorem transfer. This closure review does not independently reproduce the historical audit or establish cryptographic soundness.

## Consolidation and boundary closure

Both findings from my v1 review are corrected: requirements.md includes UNI-017, and ZR08 rejects reordering only where it causes semantic mismatch while preserving sound batching. The expanded crosswalk makes every ZR and MNR row normative and assigns its U owner.

The strengthened U4 requirement demonstrates private handoff and retained composition operators without the federated kernel. U5 separately qualifies optional joint proving/services, explicit federation policy and epoch migration. This prevents a mandatory hosted dependency from entering through privacy or recursion. ZK/MPC/TEE assumptions remain separate; bare-threshold compromise and external observation/finality assumptions remain visible.

The design preserves authorized retained-phase duties, prohibits unconsented obligations from request recording, and separates initiation/reconciliation/recovery authority. Recovery grants have signed termination/revocation semantics; they are neither automatically perpetual nor allowed to erase debt. Per-stage fan-in and growing finite history are distinguished without resetting signed lifetime budgets. SP07/SP08 semantic inputs and U6 extension requalification prevent premature full MC05/SP09 closure.

No implementation, empirical proof or release completion is asserted by this approval. U0–U7, ZR01–16 and MNR01–08 remain specified-only until their exact evidence is produced. Those disclosed future obligations are not defects in this design consolidation.

## Narrow post-freeze Pel correction

Root reported that static validation rejected the new symbolic artifact name and requested review of a deterministic correction. I read both live corrected files in full or their unchanged context plus corrected section: `implementation.pel` now uses the installed snapshot's `artifact:approved-spec` input slot, and `workflow.md` explicitly requires binding that slot to this consolidation's concrete execution packet before use. The template remains symbolic, bounded, internal and not ready for dispatch. This change is **approved** within the same design scope and introduces no public program admission requirement.

Reviewed corrected hashes: implementation.pel `fc8a5562e088524e8113961ad06f6851edabdc388cffdea13a531cb137c388f9`; workflow.md `34a5552c66061ad21a77fd3cc0bfe1edbb2f46e71626860ae411bee30080c79f`. Root reports Pel check/plan passed; I did not rerun those checks. The frozen v2 packet and its identity remain unchanged and separately verified.

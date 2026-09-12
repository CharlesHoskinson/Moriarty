# DeFi kernel SDK interface: reconciliation record

This record accompanies `2026-09-11-defi-kernel-sdk-interface-design.md`. It holds what the specification deliberately does not: the conflicts resolved during drafting, the positions that lost, and the earlier conclusions this revision overturns. The specification itself carries no trace of its making; this file is where that material lives.

## Conflicts resolved

Three authors drafted disjoint section sets against one brief. Six cross-section conflicts required a decision rather than a merge.

| # | Conflict | Resolution | Why |
|---|---|---|---|
| 1 | Binding taxonomy. Two independent drafts each split the two admitted binding forms into three, but named and divided them differently: one by who refuses a mismatch, one closer to mechanism | The who-refuses axis governs: `SignedNetworkDiscriminant`, `SignedVerifierIdentity`, `SignedChainState`. The mechanism-named classes were re-mapped onto it | Separating the signed value from the party that checks it is what makes the test decidable. The mechanism split cannot express a chain identifier that is present but unchecked, which is the exact defect both drafts found |
| 2 | `registerPrimitive` was deleted from the call surface by one author and cited as an emitting call by another | Deleted. The refusal code for an unadmitted effect is emitted by `loadProfile` and `prepareTransition` | Runtime registration of a foreign primitive is an unrestricted new call shape, which section 13 forbids. Admission is a governed deployment change |
| 3 | Section 6 returned an untyped `MissingNetworkBinding` while section 10 defined `E_NETWORK_BINDING_MISSING` | Section 6 uses the typed code | An untyped refusal cannot be tested against the refusal contract |
| 4 | Section 3 re-based the language-gap labels on the kernel design; section 11's tier lines still cited the superseded labels | Section 11 re-mapped to the kernel design's labels, and the fan-in line replaced | The two documents had used the same labels for different changes while section 11 cited them, so the tier requirements pointed at the wrong gaps |
| 5 | Two failures had no refusal code: an import that would exhaust the reserved transition allowance, and an attempt increment without proven non-execution | Added `E_TRANSITION_ALLOWANCE_EXHAUSTED` and `E_NONEXECUTION_UNPROVEN` | Both are reachable failures. An uncoded failure either degrades into a partial attempt or surfaces as the wrong refusal |
| 6 | Section 14 received overlapping unknowns from all three section sets | Merged: quorum size, threshold and bond into one row; every meter value into one; the bridge's mutable finality parameters into one; the venue failure-code vocabulary into one | A duplicated unknown reads as two obligations and is closed twice or not at all |

## Positions this revision overturns

Each of these was stated in the superseded specification or its vault decision page, and is reversed here on cited evidence.

**Hyperliquid was not the sole Tier 2 candidate.** The earlier position held that its signed domain binding was decodable and therefore qualified it. The signed typed-data chain identifier is a constant across mainnet and testnet, and a second constant is reused across environments. The only per-network value is a message field inside the signed structure, checked by venue software, committed to no ledger, and no verification artifact exists in any pinned repository. It is now a discriminant enforced by a trusted operator: Tier 1, and Tier 2 only for per-item venue effects or an imported bridge finalization. Two authors reached this independently.

**TRON was not correctly blocked.** The earlier position blocked it for want of a chain identifier in the raw transaction. Reference-block validation authenticates the referenced block to the network, inside the signed preimage, at 64 bits of block-hash content within a 65,536-block window under a 24-hour expiration ceiling. The typed-data domain supplies a genuine 32-bit network identity checked inside the virtual machine. TRON reaches Tier 1 bare and Tier 2 behind a pinned custody contract. The blocking condition was answered rather than restated.

**The chain-bound NEAR envelope is not the remedy for the delegate action.** An earlier reading treated it as the mechanism that would unblock NEAR value movement. Its own standard makes resolution a view-function convention, warns against on-chain callers, and forbids gating state changes on it. It is authority for off-ledger requests only. The delegate action is therefore refused outright for value rather than gated: its record carries no network identity, its application path never re-checks the outer block hash, and a receiving contract cannot repair the signature it receives.

**The import path did not work as specified.** The superseded text claimed continuations bind the pending identity instead of the head, with no language change required. Every accepted transition rejects unless the signed predecessor is exactly the current before-state hash, and that head is unknown when the principal signs. The unattended import sequence did not exist in the profile. A pending-bound continuation authority is now named as a required language change, and until it is admitted each import needs a fresh principal signature.

**Fan-in above one was never the dependency.** It was carried as a required language change. Each import advances the single pending record and the settling transition reads that record, so no join over several predecessors is needed and correctness holds at fan-in one. The real constraint is elsewhere: the instance transition allowance is frozen at genesis and consumed one unit per accepted action, so legs, retries, settlement and recovery can exhaust it and strand a funded pending record.

**The relation does not enforce the whole import checklist.** The superseded text asserted it did. Observation authenticity is a trusted external check against a provider bound at genesis, and the evidence digest is opaque to the profile. Section 7 is split accordingly, because claiming otherwise presents an off-ledger fact as anchored.

**The five planes were not exclusive.** They were asserted to be separately accounted, but no rule assigned a charge to a plane, so a venue-collected ordering payment booked to both foreign execution and priority. An ordered assignment test now decides every charge. Two value movements were also unaccounted: locked capital and surplus, neither of which is a charge on any plane.

## Parameters removed from normative text

The tariff table, the meter fallback rates, the replenishment window, the protocol share and its split, the author royalty rate, the observation quorum size, the aggregate exposure cap and the per-operator bond constants were all stated as proposed launch values. None was fixed by a source or by a measurement. All are now named unknowns in section 14 with an owner and a settling method. The fee rate unit and its denominator are retained, because they match a deployed venue's encoding.

The application fee percentages were misattributed. They are one venue's builder-fee ceiling, not a Moriarty parameter. The cap is now the minimum of capability rate, profile cap and the venue's enforced maximum, with per-venue figures in the adapter manifest.

## Roles removed

The watcher role was removed: an availability fee pays an idle watcher in full, so its most profitable deviation is to do nothing, and its misbehaviour clause forfeited a bounty that did not exist. Scheduled observation is already a priced attestor job. The primitive author was removed from the operator table: it carries no liveness obligation, no custody and no runtime work to price. The royalty survives as a revenue-routing rule.

## What remains unverified

No adapter in the specification has been tested against deployed behaviour. No tariff has been benchmarked. No candidate quorum's operators have been audited for independence. The pending and import relations have not been checked against admitted resource bounds. The live-network evidence underpinning sections 6 and 11 is read-only observation; nothing was broadcast to any network. Section 14 records each of these with the role that owns it.

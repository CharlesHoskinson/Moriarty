---
title: "APSS permission: how-to"
diataxis: how-to
status: research-draft
created: 2026-09-19
updated: 2026-09-19
type: research
tags: [moriarty, apss, research]
---

# How to review a proposed Permission interface

Use this procedure when you already have a candidate grant schema or wallet/signing adapter and want to check its authority boundary. It is a design/audit procedure, not a requirement to obtain anyone's approval before deploying a program. It produces a reviewable record and test scenarios; it cannot by itself prove the implementation correct.

## 1. Identify the owners and the actual effects

List whose assets can decrease, whose liabilities can increase, who receives value, who pays which fees and who learns private information. Record the authorized principal for each effect. Include successful, pending and failed paths. Exclude reviewer credentials and research campaign identifiers from this product schema. Use the distinctions in [reference.md](reference.md), particularly APSS-PERM-001/002/009.

Output: an effect-to-owner table. A missing owner or an unexplained right is an unresolved design issue, not permission to add an administrative gate. P01 supplies the architectural division; P07 supplies examples of separating structured authorization dimensions. [P01](https://frontier.tech/the-cake-framework), [P07 §2](https://www.rfc-editor.org/rfc/rfc9396.html#section-2).

## 2. Trace one exact signed object to one exact enforced interpretation

Write down the canonical payload, profile/domain, hash, signing algorithm, signature verifier, readable display and effect predicate. Identify every transformation between them. Change one field at a time: recipient, asset denomination, fee ceiling, chain/domain, liability permission and expiry. Each unauthorized change must alter the binding or fail the predicate.

Do not use a DPoP token or a generic wallet login as evidence that a body containing financial instructions was signed. Record domain separation and replay enforcement as separate checks. [P02](https://eips.ethereum.org/EIPS/eip-712), [P08 §11.7](https://www.rfc-editor.org/rfc/rfc9449.html#section-11.7).

Output: a binding map and mutation matrix. If the display has a field absent from the binding, or a financially meaningful field is absent from the display, identify it explicitly.

## 3. Check delegation and the negotiated response

Compare the actual granted permissions with the requested permissions; do not assume equality. For every child grant, compare the set of allowed effects, not only field magnitudes. Check nested calls, new recipients, liability creation, policy changes and expiry. Keep owner-approved re-granting separate from delegation. Specify a supported maximum chain length and verification work.

Output: parent/child examples that preserve authority and one adversarial example for each dimension. Use P06's response caveat and P09's contextual restriction model as comparison sources, without importing HMAC bearer credentials as ledger proofs. [P06](https://eips.ethereum.org/EIPS/eip-7715), [P09](https://research.google.com/pubs/archive/41892.pdf).

## 4. Locate the durable consumption and revocation state

Identify where an allowance, nonce/nullifier, epoch or cancellation is authoritative. Define whether pending actions reserve allowance and when a reservation can safely be released. Test two concurrent fills, replay after restart, fill-versus-revoke in both orders, and migration with an outstanding reservation. Check that revocation does not delete debt already created.

Output: state-transition cases with explicit predecessor/successor identities. A cached signature verdict is inadequate when validity depends on contract state. [P03](https://eips.ethereum.org/EIPS/eip-1271). Do not mistake an RPC success response for finalized revocation.

## 5. Separate custodial/signing assumptions from policy enforcement

For a threshold or external signer, record share-generation assumptions, corruption threshold, coordinator/network availability, payload validation and derivation/domain rules. Determine what an attacker controlling the requesting application can ask the signer to sign. State which component verifies financial policy. For a sponsored transaction, identify who can change the payload and who can only pay gas.

Output: a trust-boundary table and failure behavior. A valid threshold signature is not an application-policy proof or evidence of settlement. [P11](https://docs.near.org/chain-abstraction/chain-signatures), [P12 §7](https://www.rfc-editor.org/rfc/rfc9591.html#section-7), [P04](https://eips.ethereum.org/EIPS/eip-4337).

## 6. Audit disclosure and recovery independently

For every secret or proof witness, name readers and the authority they gain. Test whether a viewing key, delegated session key, witness or recovery key can be used to spend, change policy or re-delegate. Document information that cannot be withdrawn after disclosure. Inspect policy upgrade and recovery paths for allowance resets or forgotten liabilities.

Output: an observer/right matrix and migration invariants. Zcash is an example of separated key components; it does not supply the Midnight correspondence argument. [P10 pp.14–15](https://zips.z.cash/protocol/protocol.pdf), [P05 security considerations](https://eips.ethereum.org/EIPS/eip-7702#security-considerations).

## 7. Label the result by evidence class

Record separately: proposed schema, reviewed relation, mechanized theorem and assumptions, runtime enforcement, finite adversarial tests, and finalized ledger observations. Keep unresolved semantics open. A passing finite suite does not imply universal non-amplification; a proof about a stale state does not establish current authorization.

Output: a candidate-bound finding list with APSS-PERM requirement IDs and specific missing evidence. This is evidence for implementers and users to assess; it is not an authorization service for independent developers.


Evidence archive: [captured source manifest and reading records](evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.

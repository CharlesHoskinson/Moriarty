---
title: "APSS settlement: how-to"
diataxis: how-to
status: research-draft
created: 2026-09-19
updated: 2026-09-19
type: research
tags: [moriarty, apss, research]
---

# How to audit a Moriarty settlement contract

Use this guide when reviewing a source-to-ledger action or an optional asynchronous application. It produces an evidence map and concrete failure cases. It does not authorize deployment campaigns. Public developers need no Moriarty administrator's approval to deploy supported programs; signed spending authority and protocol checks still apply.

The most important check is the [Midnight phase contract](https://docs.midnight.network/concepts/how-midnight-works/semantics#transaction-fallibility): a failed fallible phase can leave guaranteed effects and fees. Treat that as an explicit outcome to model, not a generic exception handled by “rollback.”

## 1. Freeze the claim and version boundary

Record source hash, profile, compiler, generated artifacts, proof/verification keys, ledger version and network. State the exact user-visible outcome. Record which evidence is source documentation, a proved theorem, finite tests, local execution or actual finalized ledger observations.

Expected artifact: one claim table linking each promised financial result to its predicate and evidence. Mark an untested backend mapping open. Do not convert historical source reviews into approval of changed bytes.

## 2. Inventory signed authority and asset semantics

List concrete input/output asset identifiers or the signed bounded substitution predicate, units, rounding, recipients, gross caps, all fees, minimum net success outcome, new liabilities, expiry, replay identity and cumulative partial-fill allowance. Check the rendered description against canonical signed bytes.

Expected failure cases: same ticker/different issuer; wrapped asset substituted without authority; fee moved outside the cap; recipient or domain changed after signing; two individually valid fills exceeding the cumulative allowance.

## 3. Map every operation into ledger phases

Use the pinned compiler output and acceptance implementation, not only the language evaluator. List effects in well-formedness validation, guaranteed execution and fallible execution. For each result, derive the surviving state, transfers, fees, commitments, authority and obligations.

Minimum table:

| Outcome | Required observation |
|---|---|
| Construction or well-formedness rejection | No accepted ledger transaction; separately account for off-chain/service costs if any |
| Guaranteed-phase failure | No ledger inclusion under the documented phase rule; retain rejection evidence |
| Fallible-phase failure | Partial success; read back guaranteed effects and fees, rejected fallible effects and surviving duties |
| Full success | Full intended state/effects, net goal and all mandatory claims |
| Missing acknowledgement | Unknown until reconciled; do not treat absence as proof of non-execution |

Expected artifact: phase/effect mapping plus a precise argument for each atomicity statement. If the action uses only an atomic subset, demonstrate that subset and its fee treatment rather than assuming the whole ledger shares the evaluator's rollback semantics.

## 4. Inspect the proof predicate and its dependencies

Trace ContractInvariant, IntentRefinement, TransitionValidity and HistoryCompliance through the actual proof and ledger acceptance path. Include genesis and administrative transitions in scope. Require exact predecessor consumption, complete effects and durable authority accounting.

List observations consumed by the predicate. For each, distinguish a witness supplied by a caller, a verified signature, a threshold attestation, a light-client proof, and locally verified ledger state. Record source, freshness, policy and trust assumptions. [Compact's witness warning](https://docs.midnight.network/compact/reference/compact-reference) means a trusted local callback implementation is not sufficient enforcement.

Expected mutations: remove a claim; switch verifier/profile; forge a witness output; replay stale or wrong-domain observation; verify only an outer successful transaction while ignoring a failed financial sub-action; replace a final proof check with a host boolean.

## 5. Audit asynchronous progress and recovery

Build the state machine for pending, partially filled, claimable, fulfilled, cancelled, expired and unresolved outcomes that the application actually supports. Define who can resume, which witness is needed, and what funds/work remain reserved. A receipt timeout requires appropriate evidence: [IBC's rule](https://github.com/cosmos/ibc/blob/6eb8792e987220d7afcc8c926426f3af5695cb7b/spec/core/ics-004-channel-and-packet-semantics/README.md#timeouts) illustrates why a sender-side timer is insufficient.

Expected failures: destination halt, stale finality, delayed receipt, duplicate delivery, fill/cancel race, expired authority with a live obligation, missing witness, restart after broadcast, and recovery reserve exhaustion. No continuation may reset a promised global work bound. Specify compensation as its own conditional transfer.

## 6. Verify the real result and retain limitations

Run the required local fault matrix before separately authorized network work. Compare complete finalized state and native effects against independent expectations. Record transaction IDs, canonical finality evidence, raw process status and any cleanup separately. Reconcile unknown outcomes before retrying.

Expected artifact: a result that distinguishes financial success, partial ledger success, rejected transition and unresolved observation. Preserve all open PCD/correspondence/Preview gates; a passing corpus is finite conformance. The proposed requirements and scenarios are in contract-amendment.md (`.raw/captured/apss-2026-09-19/settlement/contract-amendment.md`).


Evidence archive: [captured source manifest and reading records](evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.

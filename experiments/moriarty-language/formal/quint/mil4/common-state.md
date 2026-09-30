# Quint common state design

**Status:** Sprint 0 design only. No `.qnt` file, typecheck, simulation or invariant result exists.

Actors are the signer/owner, signed recipient, fee beneficiary, bound debtor and creditor, and the ledger that compares heads. Later profiles add pool, vault, issuer, observer, governor, and bridge source/destination roles. Actors may submit competing proposals. One accepted stage and its head/replay consumption are atomic.

Store an immutable signed-intent table keyed by domain, signer and nonce/digest. Each record binds the version, selected program, validity window, owner/payer, recipient, fee recipient, obligation party and asset, caps, evidence choice and failure branch. A proposal carries a claimed pre-head and supplied fill. Signing and submission are separate actions so a stale or substituted proposal remains expressible.

Committed state includes typed balance maps, allowance remaining and spent, obligations with principal/accrued/outstanding/status and bound parties, current head, consumed replay IDs, remaining work, and a complete ordered effect and consumption record. S0 uses one nominal asset and one domain. Keep gross debit lines separate from net cell deltas. A local rejection is an observation with first semantic judgment and code, no post-state and no published effects. Accepted signed failure, pending and unknown become separate committed phases in later profiles.

Time is an abstract finite round. It checks signed validity and selected observation rounds. It never creates foreign nonreceipt. Imported signature truth, snapshot authentication, native proof and finality are named premises. Quint does not model cryptography or wire encoding.

The first model will add `sign`, `submitTransfer` and `submitRepay` incrementally. Each committed action needs a reachable witness and invariants for gross cap, allowance, conservation, creditor-bound debt discharge, no replay and one head chain. The initial-state and step relations must match the K projection before any cross-semantics claim. The first source for these requirements is [the MIL/4 candidate contract](../../mil4/semantics-contract.md) and its [projection](../../mil4/projection.md).

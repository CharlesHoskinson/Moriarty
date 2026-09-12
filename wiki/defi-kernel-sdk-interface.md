---
id: moriarty.defi.sdk-interface
type: decision
title: DeFi kernel interface, fee model and operator compensation
status: active
updated_at: 2026-09-12T21:40:00Z
sources:
  - src-90ceed5a7fb927166df6
  - src-3152ff250a736e6d3dd3
  - src-fe99317c9548df775eaa
  - src-11f0b62e330d9b25537d
  - src-967545d364543f0ad63b
  - src-d824334224065724073e
  - src-99be4827894a97420da8
  - src-4de1eb1125926f13f2c2
created: 2026-09-12
updated: 2026-09-12
tags:
  - moriarty
  - defi
  - multichain
  - sdk
---

# DeFi kernel interface

The interface for the Moriarty DeFi kernel. The normative text is the
[interface specification](../docs/superpowers/specs/2026-09-11-defi-kernel-sdk-interface-design.md),
committed as `194f633` on `main` and `DeFiInterface` and pushed. Its companion
[reconciliation record](../docs/superpowers/specs/2026-09-12-defi-kernel-sdk-interface-reconciliation.md)
holds the conflicts and the losing positions. Evidence sits in
[[wiki/defi-kernel-protocol-graph|the protocol graph]] and in the four category
studies under `deliverables/defi-interface-research-2026-09-12/`.

This page supersedes an earlier version of itself. What it got wrong is recorded
below rather than deleted.

## The shape of the document

Four layers, each with an audience that never needs the one below it. **Part I**
is the whole of what an account holder and an application see, and it names no
chain, venue or signing scheme — chains and assets are routing details. **Part
II** is the kernel. **Part III** is the adapter contract, where every
chain-specific fact lives. **Part IV** is the record.

The premise is that you state an outcome and never a route. Part I therefore has
five surfaces: identity, spending, receiving, orchestration, outcomes. The
application interface is grouped by what a caller is doing, not by execution
stage, and the per-leg dispatch and import calls sit below the line: an
application that could reach them could exceed the authority its user granted.

Exactly one property of the mechanism reaches a caller, because it changes what
an outcome is worth rather than how it was obtained. Every outcome and receipt
carries a strength mark, proven or attested, and a finality mark, final or
contingent. Nothing else about the mechanism surfaces.

## The five guarantees

Bounded loss on both sides, since a cap on what leaves permits any price.
Honest outcomes, where waiting is never reported as failure and an outcome
describes an intent rather than a holding. Authority is bounded and ends, and an
outcome reachable only through authority that cannot be bounded is refused
rather than served. One identity that outlives whatever authorizes it, so
retiring a means of authorization never changes an account. And nothing arriving
unbidden spends your money.

**Four earlier guarantees were found false or unsupported**, each by an
independent study of one category. Bounded loss bound only outflow. "No standing
authority" was false twice over: capabilities carried no expiry and could not be
enumerated, and unbounded allowances are structurally unavoidable on real venues,
so the honest form refuses the outcome rather than denying the reality. "One
identity" failed after rotation, because the rotation index sat inside the
address derivation and rotating a compromised key silently changed every
address. Honest outcomes failed because six terminal states could not express a
waiting, expired or replaced position.

## Capability classes, not tiers

Routability is classified by what a chain can evidence, and the classes are
inputs to the router. No account holder or application sees them; what reaches a
caller is a grade or a refusal. Class A is settlement only, Class B adds
observation, Class C moves value. No path that moves value is currently routable.

| Adapter | Binding | Class |
|---|---|---|
| Anchored settlement | Head read-then-write under ledger induction | A, pending Preview acceptance |
| Hyperliquid exchange action | An environment discriminant inside the signed structure; the typed domain's chain identifier is a constant and binds nothing | B. C only for per-item venue effects or an imported bridge finalization |
| Tron contract call, bare | A signed reference to recent chain state: 64 bits of block-hash content, 65,536-block window, 24-hour ceiling | B. Refused for value release |
| Tron via custody contract | A typed domain whose chain identifier is the trailing four bytes of the genesis block identifier, checked inside the virtual machine | C, conditional on a pinned custody contract |
| NEAR outer function call | A block hash checked against the local store with an ancestry test | B. C conditional on custody bounding arguments and deposits |
| NEAR delegate action | None. No network field; the outer block hash is never re-checked on the delegate path | **Refused outright for value** |
| NEAR offchain envelope | Chain identifier bound in a signed envelope, resolved by a view convention | Off-ledger authority only |

Custody contracts are required, not optional, for value release on Tron and
NEAR. Recovery runs only once the dispatched authorization has expired by venue
rule, which converts the deadline paradox into a per-venue falsifiable gate.

## Fee model and operator compensation

Five separately accounted planes, with an ordered test assigning every charge to
exactly one: the earlier table asserted separation without giving a rule, so a
venue-collected ordering payment booked to two planes at once. Two further value
movements are accounted as vectors that are not planes: commitments, being
locked capital priced only when a third party finances it, and surplus, assigned
in the signed terms because unassigned surplus is kept by whichever operator
touches it last.

Orchestration is metered in payload bytes and registered work credits, both of
declared work, neither claiming to measure elapsed compute. Capacity is
purchased rather than staked. Budget exhaustion stops the admission of new work
while a per-leg completion reserve, set aside before dispatch, keeps the stop
from stranding a dispatched leg.

Seven operator roles are paid for declared work: solver, attestor, sponsor,
relayer, prover, foreign signer and coordinator. A bond is posted only where one
deviation can exceed the expected revenue from continued selection; otherwise
deselection enforces. Penalties compensate verified losses first and are not
burned.

The application fee is proportional with an absolute ceiling, and the cap is the
minimum of capability rate, profile cap and the venue's own enforced maximum.

## What this page previously got wrong

- **Hyperliquid was not the sole Class C candidate.** Its signed typed-data
  chain identifier is constant across environments, so the network is a message
  field its own software is trusted to check. Two independent reviews found this.
- **Tron was not correctly blocked.** Reference-block validation authenticates
  the block to the network inside the signed preimage, and the typed-data domain
  supplies a real network identity. Tron outranks Hyperliquid, reversing the
  ordering this page asserted.
- **The chain-bound NEAR envelope is not the remedy for the delegate action.**
  Its own standard makes resolution a view convention and forbids gating state
  changes on it.
- **The import path did not work as specified.** Continuations cannot bind the
  pending identity without a language change, because every accepted transition
  binds the exact current head and that head is unknown when the principal signs.
- **Fan-in above one was never the dependency.** The real constraint is the
  instance transition allowance, frozen at genesis and consumed per action.
- **Nine operator roles were two too many.** An availability fee pays an idle
  watcher in full, and authorship carries no custody or runtime work to bond.

## Honest limits

Forty-two parameters are named as unfixed, each with a settling method and an
owner. No adapter has been tested against deployed behaviour, no meter value is
benchmarked, and no candidate attestation committee has been audited for
independence. The live-network evidence behind the binding classes is read-only
observation; nothing was broadcast. Six language changes remain unadmitted,
including the pending-bound continuation authority the import path needs.

## Where the work lives

- [Interface specification](../docs/superpowers/specs/2026-09-11-defi-kernel-sdk-interface-design.md) — the normative text
- [Reconciliation record](../docs/superpowers/specs/2026-09-12-defi-kernel-sdk-interface-reconciliation.md) — conflicts and overturned positions
- [Kernel design](../docs/superpowers/specs/2026-09-11-defi-kernel-multichain-design.md) — the constraints it extends
- [[wiki/defi-kernel-protocol-graph|Protocol graph]] — the evidence base

---
id: moriarty.defi.sdk-interface
type: decision
title: DeFi kernel SDK interface, fee model and operator compensation
status: active
updated_at: 2026-09-12T02:40:00Z
sources:
  - src-90ceed5a7fb927166df6
  - src-3152ff250a736e6d3dd3
  - src-fe99317c9548df775eaa
  - src-11f0b62e330d9b25537d
created: 2026-09-12
updated: 2026-09-12
tags:
  - moriarty
  - defi
  - multichain
  - sdk
---

# DeFi kernel SDK interface

The interface decision for the Moriarty DeFi kernel: what the SDK exposes, how
orchestration is priced, and how operators are paid. It rests on the
[[wiki/defi-kernel-protocol-graph|protocol graph]] and the earlier kernel study.
The normative text is the [SDK interface design](../docs/superpowers/specs/2026-09-11-defi-kernel-sdk-interface-design.md),
committed on branch `DeFiInterface`.

## What was decided

The SDK is orchestration around a bounded language. It adds no fifth effect and
extends nothing. One Midnight coordinator instance anchors a session; foreign
legs never advance it through `Step` but advance a bounded pending record
through `ImportFrom`, which keeps predecessor fan-in at 1.

Three statements bound the whole surface. Cross-chain atomicity is not offered,
only a bounded sequence with explicit custody, deadlines, imports and recovery.
A signature is not a result, so a signature certificate is never accepted where
an import certificate is required. Submission is not success, so an outer
transaction that succeeds while the financial operation inside it fails is a
failure.

## The three positions

The design was settled by reconciling three independent reviews, each given the
same graph and design brief and each required to cite file and line.

**Surface.** Proposed the namespace `moriarty-kernel-sdk/1`, the record and
method surface, typed per-venue argument records rather than a generic action,
and the commitment order in which terms bind plan, plan binds quote, and neither
quote signatures nor proof bytes enter their own preimages. It corrected the
starting position on net-zero deltas, salt rotation, derivation paths, recovery
guarantees and foreign network costs.

**Economics.** Argued the four-plane cost model was incomplete and that foreign
execution is a plane of its own. Defined two meters, payload bytes and
registered work credits, with a tariff; rejected refundable stake as a way to
fund recurring capacity; defined nine operator roles with payment and
misbehaviour terms; and set the application fee formula and caps.

**Adversarial.** Voted no-go for a value-moving multichain SDK under the current
profile. Its findings are specific and were accepted rather than argued down:
salt rotation preserves the previous salt and so is not a kill switch; zero-sum
deltas prove local conservation, not cross-chain atomicity; deadline-based
recovery cannot be simultaneously safe and non-stranding; a resource model whose
capacity is proportional to a share of total stake is not deterministic reserved
capacity; a single fee-limit field is not a universal cost cap; and network
binding is venue-specific rather than a chain label.

## Where they conflicted

Four conflicts required a decision rather than a merge.

**Cost planes.** Four planes against five. Five governs: foreign execution
consumes resources independently of both settlement and orchestration, and
folding it into orchestration hides a cost the user pays in a different asset.

**Budget exhaustion.** The economic review kept an earlier rule that fallback
charges rather than hard-stopping. The other two rejected it. They govern: once
capacity and the signed budget are exhausted, work stops. A priced fallback is
available only where the payer signed for that fallback, a sponsor shortfall
never becomes user debt, and an operator fault never becomes an unsigned user
charge.

**The no-go.** Treated as a gate, not a veto. Admission is staged in three
tiers, so local orchestration can ship while value-moving foreign legs stay
blocked. This is also where the surface review independently landed: return
`Unavailable` and name the exact missing adapter, policy or benchmark.

**Adjudication.** A quorum cannot adjudicate its own false claim without
equivocating, so adjudication uses a separate deployment-bound quorum whose
verdict is disclosed as another trusted import, not as objective truth.

## The admission ladder

- **Tier 0, local orchestration.** Metered services, application fees, capacity
  reservation, sponsorship, quoting and settlement on Midnight alone. No foreign
  leg, no import, no foreign signing.
- **Tier 1, foreign observation.** Read-only imports with no Midnight value
  released against them. Needs a pinned observation policy and a published
  threat model.
- **Tier 2, value-moving legs.** Foreign signing and value-releasing imports.
  Gated on adapter conformance evidence, atomic nonce and fund reservation,
  receipt consumption across intents and recovery branches, funded compensation
  before irreversible authority is issued, numeric exposure caps, and a
  verification-enabled Preview acceptance on the exact candidate.

Of the inspected adapters, only the Hyperliquid exchange action is a Tier 2
candidate, because its network binding is decodable from the signed domain. The
Tron contract call and the NEAR delegate action are both blocked: neither record
carries a network identity that the adapter can bind without further analysis.

## Fee model and operator compensation

Five separately accounted planes: Midnight settlement in DUST, foreign execution
in native atoms, orchestration services, application fee, and an opt-in priority
bid. Every total is a vector by domain-qualified asset, never one scalar adding
DUST, a native gas token and a stablecoin.

Orchestration is metered in payload bytes and registered work credits, both of
declared work. Neither claims to measure elapsed compute, because off-ledger
compute cannot be proven. Capacity is purchased rather than staked: refundable
stake has no revenue source for recurring costs.

The application fee is proportional with an absolute ceiling, carried by a
revocable capability and realized as an ordinary `Fee` effect inside the signed
gross-debit caps. Revocation applies to future admissions only, never
retroactively to a dispatched obligation.

Nine roles are paid for declared work: solver, attestor, sponsor, relayer,
primitive author, prover, watcher, foreign signer and coordinator. Authorship
and authority are bonded differently, because supplying an immutable artifact
carries reputational risk while holding a signing key or operating a mutable
oracle carries custody risk. Penalties compensate verified losses and
replacement costs first; a residual is disposed of only afterwards, rather than
being burned.

## Honest limits

The remaining uncertainty is empirical, not a missing interface field. No
inspected source proves that any proposed adapter matches deployed behaviour,
that a candidate quorum's operators are independent, or that the pending and
import relations fit admitted resource bounds. Every tariff, bond, quorum size
and exposure cap is a proposed launch parameter, not a measured one. No adapter
test, finality experiment or benchmark was run.

The language changes the interface depends on are not admitted. Fan-in stays at
1, and the import sequence is designed to work at that value.

## Where the work lives

- [SDK interface design](../docs/superpowers/specs/2026-09-11-defi-kernel-sdk-interface-design.md) — the normative text
- [Kernel design](../docs/superpowers/specs/2026-09-11-defi-kernel-multichain-design.md) — the constraints it extends
- [Kernel study](../deliverables/defi-kernel-multichain-2026-09-11/REPORT.md) — the prior report
- [[wiki/defi-kernel-protocol-graph|Protocol graph]] — the evidence base

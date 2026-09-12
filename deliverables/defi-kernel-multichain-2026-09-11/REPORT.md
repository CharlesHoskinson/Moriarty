# A DeFi kernel for Moriarty: multichain execution, intents, and a fee model separate from settlement

**Status.** Design study, 2026-09-11. Nothing here is implemented, deployed or committed. Every acceptance gate of the existing Moriarty program stands unchanged, including verification-enabled mandatory acceptance on Midnight Preview.

**Primary sources.** 115 documents retrieved on 2026-09-11 from NEAR, Hyperliquid and Tron protocol documentation and from the cross-chain intents standard, each with a URL, timestamp and SHA-256 in `inbox/defi-kernel-multichain-collection-2026-09-11.json` and the capture directory `raw/sources/defi-kernel-multichain-2026-09-11/`. Inventory rows SRC-0114 through SRC-0117. Repository claims cite paths and line numbers at the current working tree.

**Reading note.** Citations in square brackets name a captured document; citations in backticks name a repository file.

---

## 1. What the kernel has to solve

Moriarty is a bounded financial language whose transactions settle on Midnight. The settlement model was fixed earlier this year: a ledger-anchored certified state machine, one fused proof per entry point, history by ledger induction, recursion reserved for bounded certificates. That model answers how an agreement advances on one ledger. It does not answer how an agreement reaches a venue that is not that ledger.

A DeFi kernel that quotes, routes and executes across chains needs four things Moriarty does not have:

1. **A way to authorize a foreign payload.** Nothing in the repository constructs, signs or verifies a transaction for a non-Midnight chain. The only signature objects sign Moriarty statement bytes (`experiments/moriarty-language/spec/typed-schemas.md:490`).
2. **A way for a foreign fact to enter history.** Midnight's ledger cannot see a fill on another venue. The settlement model already names this class: imports across domains that cannot share a transaction (`deliverables/pcd-midnight-native-2026-09-11/REPORT.md:424`).
3. **An intent that outlives a single step.** A multichain intent is signed before its legs execute, while the adopted digest binds the exact head, so any accepted step invalidates every outstanding signature over it (`deliverables/pcd-midnight-native-2026-09-11/REPORT.md:801`).
4. **A cost plane for work done off the ledger.** Midnight's DUST pays for settlement and is invisible to the contract circuit; Moriarty's own counters bound work inside one transition and explicitly exclude host and network fees (`experiments/moriarty-language/spec/successor/repayment-kernel.md:30`).

The fourth is the one the user asked about directly, and it is the one with the clearest external evidence. Tron shows what a metered execution economy looks like when capacity is bought with stake rather than paid per transaction. Hyperliquid shows what an application fee looks like when it is a field of a signed order rather than gas. NEAR shows what it costs to sign for another chain and to route an intent through third-party solvers.

None of the three is adopted wholesale. Each answers one question.

---

## 2. Where Moriarty actually stands

The kernel builds on a narrower base than the roadmap implies.

**Two language lineages that do not meet.** The atomic profile has a working parser, checker, evaluator and a restricted Compact backend, and carries the financial machinery: obligations with identity and tombstones, settlement bindings with an exact quantum, field policies, status rules and signed authority (`experiments/moriarty-language/spec/grammar.ebnf:12`). The successor lineage has a 48-constructor expression core, real source programs and a funded repayment kernel, but declares no obligations, settlement, policy, status, observation or effect schemas (`experiments/moriarty-language/spec/successor/syntax-profile.json:53`). A kernel needs both halves, and the document that would join them is an explicit proposal that no runtime implements (`experiments/moriarty-language/spec/successor/composition-proposal.md:3`).

**A closed effect set.** Four kinds, fixed schemas: transfer, fee, obligation created, obligation settled (`experiments/moriarty-language/spec/grammar.ebnf:68`). There is no generic effect and no external-call effect.

**Signed authority in two modes, one of which is already shaped for solvers.** An exact-plan signature attests the precise writes and effects. An outcome signature attests gross debit caps, net credit goals and permitted recipients, and leaves the route open (`experiments/moriarty-language/spec/typed-schemas.md:506`). The refinement rules are enforced: every outgoing debit needs a cap, refunds never reduce gross debit, and the net goal is checked against credits (`experiments/moriarty-language/spec/semantics.md:342`).

**A call-permission hook that is deliberately closed.** The outcome statement carries `permittedCalls`, and a non-empty list is rejected because the language has no call instruction (`experiments/moriarty-language/spec/semantics.md:363`). The record that would describe a call exists and is dead (`experiments/moriarty-language/spec/typed-schemas.md:495`). This is precisely the field a foreign leg needs.

**Work counters that are not gas.** One lifecycle allowance per accepted action, one kernel work unit per action, one expression work unit per entered node, and the two budgets never net against each other (`experiments/moriarty-language/spec/bounds.json:113`, `experiments/moriarty-language/spec/successor/repayment-kernel.md:147`, `experiments/moriarty-language/spec/successor/semantic-contract.md:37`).

**Four structural limits that a multichain kernel meets immediately.** Predecessor fan-in is one and split or join is an unsupported profile (`experiments/moriarty-language/spec/bounds.json:94`, `:120`). A request needing residual progress is rejected outright (`experiments/moriarty-language/spec/bounds.json:119`). Partial settlement of an obligation is rejected (`experiments/moriarty-language/spec/semantics.md:204`). A settlement binding names an asset as text with no chain identity (`experiments/moriarty-language/spec/semantics.md:143`).

---

## 3. Three cost planes and a fourth axis

The user's requirement is that the kernel's fee model be separate from the DUST that pays for Midnight settlement. That separation is not a convention; it follows from what each layer can see.

| Plane | Pays for | Who charges it | Visible to the Moriarty proof |
|---|---|---|---|
| Settlement | Midnight block space and proof verification | The ledger | No. Fees are not bound into the contract-call statement, and sufficiency is checked outside the circuit |
| Application | An economic term of the agreement | The agreement, as a fee effect | Yes. Already bound by the signed intent's caps |
| Orchestration | Kernel work off the ledger: quoting, solving, foreign signing, watching, retrying | The kernel | Only as a bound, not as a payment |

The first two already exist. Under-funding settlement is a liveness failure, not a validity failure (`deliverables/pcd-midnight-native-2026-09-11/REPORT.md:1229`), and an application fee is an ordinary constrained effect that the signed caps already count (`experiments/moriarty-language/spec/semantics.md:172`).

The third is new, and it is the reason the kernel cannot simply reuse DUST. Quoting, solving and foreign signing happen before any Midnight transaction exists. There is nothing for DUST to pay for yet, and the circuit that would check payment has not run.

**A fourth axis: ordering.** Hyperliquid prices latency separately from both trading fees and gas, and burns the proceeds: write priority is a field on the order charged as a fraction of notional from staking balance, and read priority is a three-minute Dutch auction paid from spot balance [hl-priority-fees]. The published effect is about 45 ms per basis point of write priority and about 25 ms per auction slot. NEAR takes the opposite position: gas costs are deterministic and priority cannot be bought [near-gas-src].

The kernel should follow Hyperliquid here and keep ordering explicit. If priority is not its own axis, applications buy it implicitly by inflating the fee rate, and the fee stops meaning what it says.

---

## 4. The orchestration meter

Tron is the reference because it solved the same problem in the opposite direction: rather than charging per transaction, it sells capacity against staked capital and keeps a priced fallback for users who have none.

**What Tron does.** Two meters, not one: bandwidth for transaction bytes at one unit per byte, energy for virtual-machine instructions [tron-bandwidth-energy]. Capacity is a pro-rata share of a fixed daily pool, `own stake / network stake × pool`, recovering linearly over 24 hours [tron-protocol-resource]. When capacity runs out the protocol burns the token at a governed price: 1,000 sun per byte and 100 sun per unit of energy [tron-paying-resources]. A contract deployer may subsidise callers by setting the share of compute it pays, from zero to one hundred percent, changeable afterwards by the deployer alone. A caller sets a budget cap, and the cap bounds the caller's whole resource spend, not only the burned part. Congestion is priced per contract, not globally: a factor steps up 0.2 each six-hour period above a usage threshold, capped at 3.4.

**What the kernel should adopt.**

1. **Two meters.** One for payload the kernel carries, meaning intent envelopes, quotes, signed foreign payloads and attestations. One for kernel compute, meaning solving, simulation, proving and signature requests. Keeping them separate is what lets a cheap, chatty workload and an expensive, quiet one be priced honestly.
2. **Staked capacity with a priced fallback.** Without a fallback, a user whose capacity is exhausted is stopped rather than charged, which is a denial of service dressed as a budget.
3. **A published payer waterfall per operation class.** Tron's is explicit and ordered; a kernel with sponsors, solvers and users needs the same.
4. **Sponsorship as a first-class primitive.** A DeFi primitive can pay its users' orchestration cost without a relayer. Tron's failure mode here is instructive: when the sponsor's capacity falls short, the shortfall falls silently to the caller and counts against the caller's cap [tron-paying-resources]. The kernel should pre-check and fail closed instead of silently reassigning cost.
5. **A budget cap that defaults to the quote, not to zero.** Tron's cap defaults to zero, which means a caller who omits it can only execute if fully sponsored [tron-fees].
6. **Error semantics that separate abort from failure.** Tron charges only executed instructions on an explicit revert, and the whole allowance on crash or timeout. A kernel with an identified operator can go further: a user-signalled cancel is cheap, and an operator's own crash is charged to the operator.
7. **A separate latency bound.** Tron caps wall-clock execution at 80 ms independently of the resource budget [tron-fees]. Metered compute does not bound latency.
8. **Per-primitive congestion pricing**, so one hot market is surcharged without repricing the whole kernel.

**What the kernel should reject.**

- **Pro-rata capacity against a global pool.** Tron's own documentation shows an account's capacity falling when other accounts stake [tron-protocol-resource]. A kernel that quotes costs to users cannot have capacity that moves underneath them; capacity must be deterministic over a window, or quoted and held.
- **Charging the user for the operator's crash.** Tron does this because a validator cannot attribute the fault. A kernel with a named solver can attribute it.

**Where the revenue goes.** NEAR routes 30 percent of the gas burned executing a contract to that contract's account [near-gas-src]. That is the cleanest available model for paying the author of a DeFi primitive out of metered work rather than out of a separate fee, and it composes with sponsorship: a primitive that subsidises its callers is earning part of what it spends.

**Where the meter state lives.** Per-user meter state on Midnight is expensive under head discipline, because every write must be preceded by a read of the same head in one transcript section (`deliverables/pcd-midnight-native-2026-09-11/REPORT.md:828`). NEAR prices persistent state at 1e19 yoctoNEAR per byte and warns explicitly about the pattern where many cheap additions cost the owner dearly [near-storage-staking-src]. The kernel should keep meters off-ledger with periodic anchored settlement, and treat on-ledger meter state as a deliberate exception.

---

## 5. The application fee

Hyperliquid answers the shape question exactly. Its builder code is one optional object inside the signed order, naming a recipient address and a fee in tenths of a basis point, so a value of ten charges one basis point of notional [hl-api-exchange]. It rides the order's own signature. There is no second transaction and no second signature.

Four properties are worth importing unchanged.

- **A revocable ceiling, approved in advance.** The user approves a maximum rate per application, revocable at any time [hl-api-exchange].
- **The ceiling is approved by the principal, never by a delegated key.** Hyperliquid requires the main wallet rather than an API wallet [hl-builder-codes]. Moriarty already distinguishes a principal from the key that signs a given action, and should carry the same restriction.
- **Caps live in the protocol, not the client.** 0.1 percent on perpetuals, 1 percent on spot, at most ten active approvals per user [hl-builder-codes].
- **One integer unit.** Hyperliquid's own inconsistency makes the point: the order field and the cross-domain path use integer tenths of a basis point, while the approval action takes a percent string [hl-api-exchange]. A bounded language should fix one exact-rational unit and reject the other spelling.

Two operational properties matter as much as the shape. The fee is reported inside the fill as a component of one total rather than as a second charge discovered later [hl-api-info]. And it accrues off the hot path, claimed explicitly, so a trade never blocks on a fee payout [hl-builder-codes].

NEAR layers fees differently and confirms the same separation. Its settlement contract takes a protocol fee of one pip, 0.0001 percent, on every transfer or swap, while distribution channels add their own fee in basis points capped at 500, with a default fifty-fifty revenue share between the channel and the routing service [ni-fees], [ni-fee-config]. The protocol fee is small, on-ledger and universal; the channel fee is large, configurable and off to the side. That is the same two-layer split the kernel needs, with one difference: Moriarty's application fee is already an effect of the agreement, bound by the signed caps, so it does not need a parallel accounting system.

**What Moriarty has to add.** A fee capability type: a revocable authorization from a principal to an application, carrying a maximum rate in one integer unit, checked when the fee effect is emitted. The fee itself is already expressible.

---

## 6. Signing for another chain

NEAR's chain signatures are the only production model in the sources for authorizing a foreign payload from an account on a different chain.

**How it works.** A contract, `v1.signer`, is served by a network of eight nodes, none of which can sign alone [near-chain-sig-impl], [near-chain-signatures]. A call names the payload to sign, a derivation path, and the signature scheme, where zero selects Secp256k1 and one selects Ed25519 [near-chain-signatures]. The foreign address is derived by additive key derivation from the account, the path and the service's master public key, and the derivation is exactly reproducible: the same account and path always yield the same foreign address, and no other account controls it [near-chain-signatures], [near-chain-sig-impl]. The client flow is five steps: derive the address, build the foreign transaction, request the signature, format it into the transaction, broadcast it.

**The rule the documentation states in the strongest terms.** A signature binds to a network only if the signed payload encodes that network. Transactions carrying a chain id are safe; legacy transactions without one can be replayed on any compatible chain, and several Bitcoin-derived chains accept the same payload absent chain-specific replay protection. The stated remedy is a distinct derivation path per chain [near-chain-signatures].

For a kernel this is not advice, it is an invariant. Path separation per chain must be enforced by the kernel, because the failure it prevents is silent and total: a user who signs one payment authorizes the same payment everywhere the payload is valid.

**What the kernel adopts.**

- **The signing service sits outside the settlement contract.** NEAR separates who may authorize from where value moves. Keeping the same separation means DUST pays only for Midnight settlement, and foreign signing is priced by the orchestration meter as what it is: kernel compute.
- **One derivation path per chain, enforced.**
- **Chain of origin is not authenticated data.** In NEAR's own account abstraction the curve determines the account format, and the documentation states that the chain a key came from is not recoverable, with ambiguity remaining even when the signing standard differs [ni-account-abstraction]. A kernel must never treat a claimed source chain as authenticated.
- **The payload must carry its own binding or be refused.** The kernel should reject any foreign payload template that cannot encode a network identifier.

**What the sources do not settle.** No latency figure, no gas cost for requesting a signature, and no threshold value for the signing network appear anywhere. The contract interface itself is documented two incompatible ways, with three parameters in one place and four with a differently named and typed scheme selector in another. A kernel adapter must pin the interface against the deployed contract, not against the documentation.

**One capability worth noting for section 8.** The same node software also performs foreign-chain transaction verification: nodes independently query configured providers, run deterministic extractors, and produce a threshold signature over the observed values, which the documentation frames as letting contracts react to external events without a trusted relayer [near-mpc-repo]. That is an attestation primitive, and it is the shape the kernel needs for importing facts.

---

## 7. Intents and the solver boundary

NEAR's intents system and the cross-chain intents standard disagree productively, and the disagreement tells the kernel what to standardize.

**NEAR's model.** A single settlement contract holds an internal ledger; users deposit, then swap or transfer by signed intent, then withdraw [near-intents-verifier]. A signed payload names the signer, the verifying contract, a deadline, a nonce and a list of intents. Three details are worth copying:

- **The verifying contract is in the signature.** It exists to stop the same signed intent being replayed against another deployment of the same code [ni-intent-types-and-execution]. Moriarty's intent must bind its target instance the same way, and the adopted digest already does.
- **The nonce is salted and versioned.** It is a four-byte rotating salt plus 28 bytes of uniqueness, and an intent is valid only if its salt matches the contract's current salt [ni-intent-types-and-execution]. Rotating the salt invalidates every outstanding signed intent at once. For a kernel holding long-lived multichain intents, that is a cheap and legible kill switch.
- **A swap is a balance-delta declaration that must net to zero.** Deltas are signed, negative for given and positive for received, and a batch fails unless the deltas sum to zero per token [ni-intent-types-and-execution]. This is a conservation law checkable before execution, and it is exactly the kind of construct a bounded language can enforce. A kernel intent spanning legs on several chains should carry the same per-asset delta vector and the same rule.

**What the standard learned.** The cross-chain intents standard was rewritten to be resolver-centric: protocols expose an opaque payload plus a resolver that a solver reads off-chain to obtain an executable description. The earlier draft, which standardized order structures and minimum-received bounds, was abandoned because protocol-specific subtypes made it standard only in appearance [erc-7683]. It mandates no common settlement contract, and it does not standardize user authorization at all.

**The kernel's position.** Standardize the resolved, executable description of an order, not a universal order struct. Moriarty is well placed for this because it already has the thing most systems lack: a signed statement of what the user authorized, with caps, recipients and goals, and a rule that the route may vary within it. The invariant is already written into the unified semantics design: a solver may search for an acceptable plan outside the language, but cannot enlarge the authorized relation (`docs/superpowers/specs/2026-09-06-moriarty-unified-semantics-design.md:32`).

**Which signing mode.** Outcome mode, not exact-plan mode. A multichain intent is signed before its legs execute, and the exact-head binding dies on any accepted step. Outcome mode already carries caps, goals and permitted recipients, and the adopted acceptance model gives it a per-instance consumed-nonce set. Exact-plan mode remains correct for a single anchored step.

**What running a solver network actually costs.** NEAR's quoting path is a WebSocket relay with millisecond-scale auctions: a top-level auction floors at about 300 ms of collection with roughly 300 ms of grace, while router sub-legs have no floor and an effective window around 51 ms, where a 50 ms round trip alone loses the fill [ni-websocket]. Access requires an API key behind identity checks [ni-quickstart]. The contractual regime is strict: a quote becomes firm on selection, and inventory, market movement, stale pricing and latency are explicitly not valid excuses for failing to honour it [ni-solver-terms-of-use]. Delivery guarantees are modest and finite: a five-second acknowledgement window, at most 256 unacknowledged messages, seven-day retention.

Two consequences for the kernel. First, quote transport is a latency-critical path that must be separated from settlement, and it is not where Moriarty's proving belongs. Second, a solver's obligation is economic and contractual before it is cryptographic, so the kernel needs a bonding or reputation answer, and the sources give none: the economics of running a solver are absent from the documentation.

---

## 8. How a foreign leg enters history

This is the safety core of the design.

The adopted Midnight model discharges on-ledger history by induction: with immutable keys, constrained genesis and head discipline, a successor needs no evidence of its predecessor beyond the head commitment it reads. Anything the anchor ledger cannot see is a different class, and the model already names it: imports across instances or domains that cannot share a transaction.

| Class | Example | Mechanism | What carries the evidence |
|---|---|---|---|
| Anchored | A step on a Moriarty agreement | Fused step circuit, head read-then-write | Ledger induction; no predecessor proof |
| Imported | A fill on a foreign venue, a foreign transfer | An import entry point that consumes an attestation or certificate | An explicit, bounded object named in the signed intent |

**The rule.** An imported fact may never be presented as an anchored one. A kernel that blurs this is claiming the Midnight ledger verified something it never saw.

**First cut: attestation.** The threshold-signature verification described in section 6 is the available primitive: independent nodes observe a foreign chain, apply deterministic extraction and jointly sign the observed value. The kernel binds the attesting quorum's key set at deploy time, checks the signature in the step relation exactly as it already checks oracle observations, and binds the observation policy into the intent digest. The upgrade path is a certificate verified in-circuit once recursion ships, which the Midnight roadmap already schedules as bounded certificate work.

**The independence rule.** The quorum attesting to a fact must not be the party that profits from it. The sources do not settle who that quorum is; the kernel must.

**Three gaps to close in the language.**

1. **Call permission.** Open the dead `permittedCalls` field with a domain-qualified callee and a signed cap per call. The record shape already exists.
2. **A pending lifecycle.** An asynchronous foreign leg is exactly the shape the language rejects today. The model to copy is already in the Midnight design: a head leaves the live state only into a state that a bounded recovery can return, as the release and reclaim pair does for cross-contract calls. A pending head needs a deadline and a resolution path that cannot strand value.
3. **Domain-qualified asset identity.** A settlement binding naming an asset as text cannot distinguish the same symbol on two chains.

Two further limits bound the first version: predecessor fan-in is one, so a join over legs from several chains has no admitted arity, and the designed cap is two pending a benchmark; and there is no light client, so the first version's trust rests on the attesting quorum rather than on verification.

---

## 9. The SDK

The developer interface already exists on paper, with record types for program bundles, observations, intents, plan certificates, signed intents, proof requests, transaction bundles and submission records, and a firm ordering rule: plan verification precedes signing, and signing precedes proving (`docs/superpowers/specs/2026-09-06-moriarty-developer-interface-design.md:217`). None of it is implemented, and results are a tagged union rather than a boolean.

The kernel extends that list rather than replacing it:

| Record | What it carries |
|---|---|
| `Quote` | A priced, time-bounded plan from a solver, including the orchestration cost it will charge |
| `Leg` | One chain-local action: domain, asset identity, venue, and the evidence class it will produce |
| `ForeignPayload` | The exact bytes to be signed for a foreign chain, with the derivation path and the network binding |
| `SignatureRequest` and `SignatureCertificate` | The request to the signing service and its verifiable result |
| `SponsorGrant` | Who pays the orchestration meter for this intent, and up to what cap |
| `MeterReceipt` | What was charged, against which capacity, with the recovery state |
| `ImportCertificate` | Evidence that a foreign leg happened, in the class the intent named |

**Shape to copy from the routing API.** A quote carries a deadline that is the point at which refunds begin and must exceed the time to confirm a deposit, an estimate in seconds, a separate time after which the address goes cold, slippage in basis points, an optional dry mode, and an application-fee array in basis points [ni-submit-deposit-transaction-hash], [ni-fee-config]. Terminal states are explicit: success, refunded, failed, with under-payment distinguished from failure [ni-making-a-request].

**Parity to copy from the settlement contract.** Simulation takes the same payload as execution and returns the events, the earliest deadline and the current fee, so a client cannot drift between the two paths [ni-simulating-intents]. Moriarty should expose one type on both paths for the same reason.

**Reality check for adapters.** A venue adapter cannot always be written from documentation. Hyperliquid's own documentation never specifies how its non-typed actions are serialised and hashed, deferring twice to its SDK; the construction is msgpack encoding, a keccak action hash, and a typed wrapper, which is discoverable only from the client source [hl-sdk-signing]. Two canonical-encoding rules there are load-bearing: two boolean fields must be omitted rather than sent false, or the action is rejected [hl-api-exchange]. Moriarty already canonicalises signed bytes and should state the same rule for kernel envelopes.

---

## 10. Decisions

Defaults below are supported by the evidence cited above. They are proposals, not adopted positions.

| Area | Decision | Default |
|---|---|---|
| Profile | Which lineage the kernel extends | A profile composing the successor expression core with the atomic financial objects; the registered composition rule is the first deliverable |
| Language | Open the call-permission field | Yes, domain-qualified callee with a signed per-call cap |
| Language | Asset identity | Domain-qualified, chain id included |
| Language | Pending legs | Add a pending head state with a deadline and bounded recovery |
| Language | Fan-in for multichain joins | Raise only against a retained benchmark |
| Evidence | How a foreign fact enters | Threshold attestation first, certificate upgrade path, never silent trust |
| Evidence | Attesting quorum | Independent of the party that profits |
| Meter | Number of meters | Two: payload and compute |
| Meter | Capacity | Deterministic or quoted and held, never pro-rata against a moving pool |
| Meter | Fallback | Priced, never a hard stop |
| Meter | Sponsorship | Explicit signed grant with a cap; shortfall fails closed |
| Meter | Budget cap default | The quoted cost |
| Meter | Failure charging | Abort charges executed work; operator crash charges the operator |
| Meter | State location | Off-ledger with periodic anchored settlement |
| Fee | Application fee shape | A signed field of the intent under a revocable ceiling approved by the principal |
| Fee | Priority | A separate axis, never folded into the fee rate |
| Fee | Operator classes | Keep fee-on-another's-market separate from fee-on-your-own-market |
| Authorization | Signing mode for multichain intents | Outcome mode with a consumed-nonce set |
| Authorization | Multi-party shape | Weighted keys with a threshold and an operation mask |
| Authorization | Session keys | Retirement must be monotone; nonce state may never be pruned into replayability |

**Open, and properly the user's.** The unit of account for stake and fallback pricing. Whether the kernel hosts its own solver network or publishes a resolver and lets others route. The operator risk class for a party that defines a primitive, which on Hyperliquid ranges from a 100 USDC threshold for attaching a fee to another's market up to a 500,000 token bond that is slashable to zero for operating one's own [hl-builder-codes], [hl-hip3-md]. And whether the orchestration meter settles on Midnight at all, or stays entirely off-ledger with periodic reconciliation.

---

## 11. Risks

**Signature griefing gets worse, not better.** Binding a signature to an exact head means any accepted step invalidates it. A multichain intent is long-lived by construction. Outcome mode mitigates this; per-party sub-heads are the structural answer; neither is implemented.

**The first version's trust is the quorum, not verification.** Without a light client, an imported fact is only as good as the attesting set. This must be stated in the product, not buried.

**Nothing is ledger-accepted yet.** The Midnight side has no implemented acceptance, no ledger-accepted Moriarty proof, and certificate entry points that measured above the current campaign ceiling. A kernel design does not change that, and the kernel cannot claim settlement guarantees the base layer has not demonstrated.

**Operational surfaces are contractual.** Access to the one live intent network in the sources requires identity checks and carries firm-quote obligations with no service-level guarantee, and its settlement contract has no test network [ni-solver-terms-of-use], [near-intents-verifier]. Any pilot inherits those terms.

**Documentation is not an interface.** Two of the three venues have load-bearing behaviour that is absent from their documentation: the signing construction in one case, and in the other, contradictory statements about what a budget cap actually caps [tron-paying-resources], [tron-protocol-resource]. Adapters must be pinned against deployed behaviour and re-checked.

---

## What would have to be true

The kernel is buildable on this evidence, in this order: register the composition rule that joins the two language lineages; open call permission with signed caps; add the pending lifecycle and domain-qualified assets; define the orchestration meter and its sponsorship grant; define the attestation class and its quorum; then adapters, one venue at a time, each pinned against deployed behaviour rather than documentation.

The fee model the user asked for is the part with the least invention in it. Midnight's DUST keeps paying for settlement and stays invisible to the proof. The agreement's own fees stay where they already are, as effects bound by signed caps. What is new is a meter for work that happens before any Midnight transaction exists, and the evidence for how to build one is unusually concrete.

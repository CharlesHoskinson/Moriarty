# Moriarty multichain DeFi kernel: design

- Date: 2026-09-11.
- Status: design, specified-only. No implementation, no admission, no acceptance. Every existing Moriarty gate stands, including verification-enabled mandatory acceptance on Midnight Preview.
- Study: `deliverables/defi-kernel-multichain-2026-09-11/REPORT.md`.
- Evidence: `inbox/defi-kernel-multichain-collection-2026-09-11.json`, inventory rows SRC-0114 to SRC-0117.

## 1. Goal

A kernel that lets a Moriarty agreement quote, authorize and execute financial actions on chains other than Midnight, with an SDK that orchestrates those actions, and a fee model that meters kernel work separately from the DUST that pays for Midnight settlement.

## 2. Inherited constraints

These are not design choices. They come from the adopted Midnight acceptance model and the current language.

1. DUST is not bound into the contract-call statement, and a contract circuit cannot see fee sufficiency. Under-funding settlement is a liveness failure, not a validity failure.
2. On-ledger history is discharged by induction under immutable keys, constrained genesis and head read-then-write. A successor needs no predecessor proof.
3. Anything the Midnight ledger cannot see is an import, not a step.
4. A contract-call proof cannot be an inner proof, so a foreign leg cannot be verified by recursion over a Midnight proof.
5. The signed intent digest binds the exact head in its head-bound mode, so any accepted step invalidates outstanding signatures over that head.
6. Effects are a closed four-kind set; there is no external-call effect. Predecessor fan-in is one. Pending is rejected. Partial settlement is rejected. Asset identity is a bare text with no chain.
7. Moriarty's work counters are allowance, kernel work and expression work, and they never net against each other. Host and network fees are out of scope of the kernel counters.

## 3. Architecture

### 3.1 Cost planes

Four distinct things are priced, and the kernel must never merge them.

| Plane | Unit | Charged by | Bound where |
|---|---|---|---|
| Settlement | DUST | Midnight ledger | Outside the circuit |
| Application fee | A fee effect in a settlement asset | The agreement | Signed intent caps, in-circuit |
| Orchestration | Kernel meters, section 3.2 | The kernel | Program digest bounds and the kernel ledger |
| Priority | An explicit ordering price | The kernel or the venue | The intent, as its own field |

### 3.2 Orchestration meters

Two meters, never one:

- **Payload**: bytes the kernel carries on a user's behalf, being intent envelopes, quotes, foreign payloads and attestations.
- **Compute**: kernel work, being solving, simulation, proving and signature requests.

Rules:

1. Capacity is acquired by stake and is deterministic over a stated window, or quoted and held for the life of a quote. Capacity that varies with other users' stake is rejected.
2. Every meter has a priced fallback. Exhausted capacity charges; it does not hard-stop.
3. Each operation class publishes a payer waterfall. The waterfall is part of the profile, not of an implementation.
4. Sponsorship is an explicit signed grant naming the sponsor, the covered classes and a cap. A shortfall fails closed; it never reassigns cost silently to the user.
5. The per-intent budget cap defaults to the quoted cost. A zero default is rejected.
6. Charging on failure distinguishes a user-signalled abort, which charges executed work, from an operator fault, which charges the operator.
7. Congestion pricing is per primitive, not global.
8. Latency is bounded separately from capacity.
9. Meter state lives off-ledger with periodic anchored settlement. On-ledger meter state is an exception that must be justified against head-discipline cost.
10. A share of metered revenue may be routed to the author of the primitive that consumed it.

### 3.3 Application fee

A fee capability is a revocable authorization from a principal to an application: a maximum rate, in one integer unit of an exact rational, optionally scoped to asset classes.

1. The capability is granted by the principal and never by a session key.
2. The rate is carried as a field of the signed intent and charged as an ordinary fee effect, which the existing gross-debit caps already count.
3. Protocol-level caps bound the rate per market class, and a user holds a bounded number of active capabilities.
4. The settlement record reports one total with the application component itemised inside it.
5. Fees accrue and are claimed explicitly. Execution never blocks on a fee payout.

### 3.4 Anchored and imported facts

| Class | Mechanism | Evidence |
|---|---|---|
| Anchored | Fused step circuit with head read-then-write | Ledger induction |
| Imported | An import entry point consuming an attestation, later a certificate | The object named in the signed intent |

1. An imported fact is never presented as anchored. The distinction is machine-checked, not documentary.
2. The first version's attestation is a threshold signature over deterministically extracted foreign state, with the quorum's key set bound at deploy time and its policy bound in the intent digest.
3. The attesting quorum is independent of the party that profits from the fact.
4. The upgrade path is an in-circuit certificate once bounded recursion is available, under the existing certificate rules.

### 3.5 Foreign signing

1. A distinct derivation path per chain is a kernel invariant, not developer discretion.
2. A foreign payload must encode its own network binding, or the kernel refuses to request a signature for it.
3. A claimed source chain is never authenticated data.
4. The signing service is outside the settlement contract, and its work is charged to the compute meter.
5. An adapter pins the signing interface against deployed behaviour, because documentation for at least two target venues omits or contradicts it.

### 3.6 Intents and solvers

1. Multichain intents use outcome mode with a per-instance consumed-nonce set. Head-bound mode remains for single anchored steps.
2. An intent carries a per-asset delta vector that must net to zero across its legs, checked before execution.
3. The intent binds the target instance, a deadline, the observation policy, the fee capability in force and the evidence class required for each leg.
4. A salted nonce scheme gives bulk invalidation of outstanding intents.
5. The kernel publishes a resolved, executable description of an order for solvers. It does not publish a universal order struct.
6. A solver may search for a plan outside the language but may not enlarge the authorized relation.
7. Simulation and execution accept the same payload type.

### 3.7 SDK

Extends the existing record set, keeping the ordering rule that plan verification precedes signing and signing precedes proving. New records: `Quote`, `Leg`, `ForeignPayload`, `SignatureRequest`, `SignatureCertificate`, `SponsorGrant`, `MeterReceipt`, `ImportCertificate`. Every result is a tagged union, never a bare boolean.

## 4. Language changes required

| # | Change | Why |
|---|---|---|
| L1 | Register the composition rule joining the expression core to the financial objects | The two lineages do not meet, and the kernel needs both |
| L2 | Open the call-permission field with a domain-qualified callee and a signed per-call cap | The hook exists and is closed |
| L3 | Domain-qualified asset identity | A bare asset text cannot distinguish chains |
| L4 | A pending head state with a deadline and bounded recovery | Asynchronous legs are rejected today |
| L5 | A signed liability cap | Caps do not bound new nominal debt |
| L6 | Fan-in above one, only against a retained benchmark | A join over several chains has no admitted arity |
| L7 | A fee capability type | The fee effect exists; the authorization to charge it does not |
| L8 | Meter counters as a fourth family that never nets against the existing three | Existing counters bound one transition |

## 5. Failure handling

- A failed attestation leaves the head pending until its deadline, then recovers by a bounded path that cannot strand value.
- A sponsor shortfall fails the intent before execution.
- An operator fault charges the operator and refunds the user's unexecuted work.
- A quote that expires is void, and the kernel never executes against a stale quote.
- A foreign payload without a network binding is refused before signing.

## 6. Out of scope

Implementation, venue adapters, the solver network's economics and bonding, a light client for foreign state, confidential routing, and any change to Midnight settlement. The certificate route depends on bounded recursion that remains blocked on release and on an unapproved resource amendment.

## 7. Decisions for the user

1. The unit of account for stake and fallback pricing.
2. Whether the kernel hosts a solver network or publishes a resolver and lets others route.
3. The operator risk class for a party defining a primitive: cheap and revocable, or bonded and slashable.
4. Whether meter state settles on Midnight at all, and at what cadence.

## 8. Validation before any build

- Every bound in the meter profile has a retained benchmark before it is frozen.
- Each adapter is pinned against deployed behaviour, with a recorded divergence list against its documentation.
- An attestation is never accepted unless its quorum, policy and freshness are bound in the intent digest.
- The anchored and imported distinction is checked by tooling, not by review.

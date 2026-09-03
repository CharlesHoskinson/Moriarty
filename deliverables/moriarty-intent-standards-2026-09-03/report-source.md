# Moriarty intent-semantics standards research

Research date: 2026-09-03
Scope: CAKE, NEAR Intents, current Ethereum intent and account standards, and
the Open Intents Framework
Semantic baseline: Moriarty `0.0.0-e00.2`, unchanged

## Decision

Moriarty should use two related calculi: the finite financial-agreement Core
and a typed intent calculus whose plans must refine signed outcome and effect
predicates. It should not copy a deployed intent service, an opaque resolver,
or an account-abstraction envelope into Core.

The standards evidence makes one boundary decisive. An objective, quote,
authorization, order payload, resolved plan, wallet transaction, fill,
fulfillment proof, claim, cancellation, refund, and final settlement are
different objects. A safe SDK must preserve their identities, hashes,
authorities, states, and evidence. “Intent” cannot be the type of all of them.

## Corpus and method

Scrapling 0.4.15 acquired the complete public one-hop corpus from the
[CAKE Working Group](https://frontier.tech/cake-working-group): 14 unique links,
13 successful receipts, and one recorded TLS failure for xerc20.com. It also
acquired the [CAKE framework](https://frontier.tech/the-cake-framework) and the
working-group notes. The notes are incomplete workshop material and are not a
ratified specification.

For NEAR Intents, Scrapling acquired the official
[documentation index](https://docs.near-intents.org/llms.txt),
[full corpus](https://docs.near-intents.org/llms-full.txt), and every one of the
68 canonical sitemap pages. It also acquired the published
[1Click OpenAPI](https://1click.chaindefuser.com/docs/v0/openapi.yaml) and
[Explorer OpenAPI](https://explorer.near-intents.org/api/v0/openapi.yaml).
The byte-level acquisition records are in
`evidence/near-intents-docs-acquisition-2026-09-03.json` and
`evidence/cake-working-group-acquisition-2026-09-03.json`.

The Ethereum lane inspected current primary standards and version history.
Consequential claims use the current normative page rather than a blog or an
older interface description. Repository and deployed-system claims still
require separate commit and deployment pins during the assignment.

## CAKE: useful architecture, incomplete protocol

CAKE supplies four valuable responsibility layers:

1. Application defines the desired outcome and application trust policy.
2. Permission discovers accounts and assets, checks authority, and obtains a
   signature.
3. Solver validates the request and proposes price, fee, path, and timing.
4. Settlement moves information and value and establishes the requested
   postconditions.

This is an architecture taxonomy, not an executable intent standard. It has no
normative wire schema, signature domain, cancellation state machine, residual
intent rule, partial-fill algebra, refund protocol, solver-selection rule, or
portable settlement receipt. Its cross-chain security language is also an
objective: the working notes acknowledge that real bridges carry trust
assumptions.

Moriarty should adopt the four-layer boundary as a review map. It should
strengthen every edge with a typed contract, version, authority, idempotency
rule, privacy class, failure state, and verification receipt.

## NEAR Intents: deployed evidence and its exact boundary

### Contract-level semantics

The deployed Verifier at `intents.near` maintains an internal multi-token
ledger. A signed payload names `signer_id`, `verifying_contract`, `deadline`, a
256-bit nonce, and an ordered list of intents. The nonce embeds a rotating
four-byte contract salt and 28 unique bytes. The recipient or verifying
contract binds the signature domain.

The most relevant primitive for Moriarty is `token_diff`. A signer states a
map of token changes using base-unit integers: negative quantities are given
and positive quantities are received. Across a submitted batch, the diffs must
sum to zero for every token or the batch fails. This is a strong executable
conservation pattern for solver-matched transfers.

It is not a complete cross-chain settlement theorem. The documentation says
that Verifier actions start in order, but asynchronous cross-contract calls
need not complete in that order. Simulation excludes those external effects.
Withdrawals then use bridges with distinct trust models.

### Authorization and identity

The Verifier supports NEP-413, ERC-191, raw Ed25519, WebAuthn, TonConnect,
SEP-53, and TIP-191 wrappers. Partner API credentials, an end-user session, and
the wallet signature are separate authorities. An API partner cannot be
treated as the user.

The account model has a material ambiguity. An implicit account is derived
from the public key and curve; the documentation says that it cannot always
identify the source chain when the same seed or key is used in different
wallets. Moriarty must therefore bind an explicit network namespace and address
profile. It must not infer chain identity from a key or displayed address.

### Quote, execution, and recovery lifecycle

The operational path is more detailed than the overview:

`quote request → provider-signed quote → deposit or generated payload → user
signature → relay → Verifier batch → bridge withdrawal → destination evidence`

The service also has deposit, swap, order-fill, payout, cancellation, and
refund status machines. Limit-order cancellation is asynchronous; a final fill
can win while cancellation is pending. Fill status and payout status are
separate. A successful internal batch or `SETTLED` relay event must not imply
destination finality.

Guaranteed relay delivery is at-least-once, not exactly-once. The relay retains
events for up to seven days, redelivers after a five-second acknowledgement
deadline, caps unacknowledged messages at 256, and requires client-side durable
deduplication. The feature is documented as live but not yet exercised by a
solver. Moriarty's SDK must model `acknowledgeDelivery`, durable persistence,
deduplication, subscription resumption, and reconciliation with authoritative
chain evidence.

### Trust and privacy qualifications

The overview describes non-custodial execution and automatic refunds. Lower
layers qualify both claims. The Verifier holds contract-ledger balances;
1Click says it temporarily transfers assets to a trusted swapping agent; some
withdrawal and storage operations do not refund; and failed indexed deposits
can require manual handling.

Confidential Intents adds a private NEAR fork called FAR, a small permissioned
validator set, a treasury-backed token representation, a private relay, and a
PoA bridge. The public documentation does not normatively define the
information-flow guarantees of `basic` and `advanced` confidentiality.
Moriarty may support such an adapter profile, but must name these trust and
disclosure assumptions. A valid proof on Midnight cannot erase them.

### NEAR documentation contradictions to preserve

- The overview's general atomic-refund language conflicts with documented
  asynchronous calls and nonrefundable or manually recovered paths.
- The human intent page shows fewer variants than the OpenAPI and source enum.
- Narrative payload requirements and the OpenAPI required-field list disagree
  about `intents`; empty authentication payloads intentionally consume a nonce.
- Message Bus signature support is narrower than the Verifier's seven wrappers.
- Raw Ed25519 deadline encoding differs from the general ISO-8601 description.
- Narrative quote access permits an unauthenticated surcharge, while the
  OpenAPI declares authentication.
- The quickstart lists `KNOWN_DEPOSIT_TX`, which the published status enum does
  not include; the order enum also contains states omitted from prose.
- Supported SDKs and examples lag the current OpenAPI in endpoints and fees.
- SHIELD controls are partly deployed and partly rolling out by route.

These contradictions are requirements for conformance tests and fail-closed
unknown handling, not reasons to copy one representation silently.

## Ethereum and OIF: a layered stack, not one standard

Ethereum supplies complementary pieces:

- EIP-712, ERC-1271, ERC-6492, ERC-7739, ERC-2612, ERC-3009, and Permit2 address
  typed signing, contract signatures, counterfactual accounts, replay domains,
  permits, and nonce strategies.
- ERC-4337, EIP-5792, ERC-7710, ERC-7715, ERC-7579, and ERC-7821 address account
  execution, wallet calls, delegation, permissions, and batching.
- ERC-7930 and CAIP-2, CAIP-10, and CAIP-350 address cross-chain identifiers,
  but explicitly do not provide universal textual canonicalization.
- ERC-7521 is a draft programmable intent envelope with solver-arranged
  segments. It has no mandatory envelope nonce or deadline.
- ERC-7683 is now a resolver interface. It is not the previous order-and-fill
  interface that many integrations still call ERC-7683.

The post-2026-05-13 ERC-7683 design uses a protocol-specific opaque payload and
an on-chain resolver. Off-chain resolution returns an acyclic graph of steps,
variables, payments, queries, witnesses, and assumptions. Authorization, order
creation, price discovery, escrow, fill, settlement, and cancellation remain
protocol-specific.

This creates a time-of-check/time-of-use obligation. A Moriarty adapter must
bind the resolver network and address, implementation and code hash, upgrade
state, resolution block, payload hash, resolved-plan hash, all query results,
witnesses, and assumptions. It must revalidate mutable dependencies before
signing and before execution.

The Open Intents Framework still uses `StandardOrder` and `MandateOutput`
vocabulary associated with the earlier ERC-7683 design. Its API, contracts, and
solver also have different versions and lifecycle meanings. Moriarty must treat
OIF as a separate compatibility profile rather than blending it into current
ERC-7683.

No reviewed standard provides general end-to-end cross-chain atomicity. The
research must separate divisible economic fill, partial output completion,
same-chain transaction atomicity, contiguous wallet execution, cross-domain
all-or-refund, and end-to-end atomicity.

## Required Moriarty intent model

The research prompt now requires the following typed artifacts:

- `IntentObjective`, `QuoteRequest`, and provider-authenticated `Quote`;
- `UserIntent`, `IntentDomain`, and `IntentAuthorization`;
- protocol `OrderPayload`, `ResolverIdentity`, `ResolutionSnapshot`, and
  `AssumptionManifest`;
- `ResolvedPlan`, `SolverPlan`, `PlanEffectSummary`, and `StateEvidence`;
- `DisclosurePolicy`, `FeeLedger`, and `FinalityPolicy`;
- `DeliveryReceipt`, `FillReceipt`, `FulfillmentProof`, `ClaimReceipt`,
  `CancellationReceipt`, `RefundReceipt`, and `ResidualIntent`;
- the compiler, proof, signing, submission, rollback, and composed correctness
  artifacts already required by version 1.0.

The lifecycle must state, for every transition, the actor, authority, signed
fields, domain, network, verifying contract, nonce, validity, mutable state,
idempotency, observations, disclosure, proof, failure, retry, cancellation,
rollback, and finality rule.

The standard SDK must add quote verification, solver discovery, resolution
verification, intent cancellation, fill and fulfillment verification, claims,
refunds, delivery acknowledgements, durable deduplication, subscription resume,
and status reconciliation. These operations remain specified-only until their
component and data-contract inventories are amended by an OpenSpec change.

## Semantic-scope disposition

This research changes the assignment, not Moriarty semantics. It adds no Core
constructor and accepts no semantic motion. The active scope remains
`0.0.0-e00.2` with SHA-256
`9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`.

The next falsification test is a cross-domain trace where the Verifier batch
succeeds and the bridge withdrawal fails or is rolled back. Any candidate
`fulfilled` predicate that accepts the internal batch must be rejected.

## Confidence and remaining acquisition work

Confidence is high in documentation coverage and the high-level object and
status distinctions. Confidence is medium in deployed correspondence until the
Verifier, SDK, OIF, and adapter repositories are pinned and checked against
deployment identities. Confidence is low for universal privacy, finality,
refund, censorship-resistance, and decentralization claims because the public
sources either qualify them or do not specify them.

The next research sprint must acquire repository commits, deployed contract
code and upgrade authority, audit reports, live status traces, and official
conformance vectors. The current report does not treat prose, an OpenAPI file,
or a successful simulation as proof of deployed behavior.

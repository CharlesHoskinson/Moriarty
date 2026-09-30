# Hyperliquid as a second boundary reference for Moriarty

**Research date:** 2026-09-29. **Status:** source-grounded design comparison. This is not a Hyperliquid security audit, a Moriarty implementation result, or a decision to adopt its execution model. The [NEAR-based kernel proposal](../near-kernel-scope-2026-09-29/PROPOSAL.md) is the comparison target.

## Finding

Hyperliquid makes a stronger **native financial-kernel** choice than NEAR Intents: HyperCore includes the order book, matching and margin state under HyperBFT, while HyperEVM is a general application environment on the same chain. [Core overview](https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/overview). This is a useful counterexample to assuming that every Moriarty financial action needs off-chain solver discovery. A Moriarty kernel adapter can call a native venue directly when the signed policy permits it. The language still has to state the user's exact financial constraints and verify the resulting effects under the selected settlement domain.

| Boundary | Hyperliquid source fact | Consequence for Moriarty |
| --- | --- | --- |
| Financial execution | HyperCore holds order book, matching and margin state, with a common consensus order; orders and liquidations are platform actions. [Core overview](https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/overview), [order book](https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/order-book) | `VenueCall` should admit a native matching/clearing endpoint. The language binds limit, quantity, recipient, fees, position and residual duty. The adapter handles the venue's concrete action bytes and status. |
| App-to-core interface | HyperEVM reads selected Core state through precompiles. CoreWriter emits a versioned action log; some order and vault actions are delayed before Core execution and appear first as enqueued, then executed. [Interacting with HyperCore](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm/interacting-with-hypercore), [interaction timings](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm/interaction-timings) | A same-chain system call can still have asynchronous phases. `submitted`/`enqueued` and `executed` must be separate receipts. Bind action version and exact encoding in an adapter. |
| Builder fees | User main wallet approves a per-builder maximum; later orders may carry a builder fee within that cap. The fee is processed on-chain and the approval is revocable. [Builder codes](https://hyperliquid.gitbook.io/hyperliquid-docs/trading/builder-codes) | Separate signed fee authority from an operator's quoted fee. The actual fee must enter gross effects and net outcome checks. |
| Delegated signing | API/agent wallets can sign for a master account; nonce state is per signer. The docs warn that pruning a deregistered agent's nonce state can make old actions replayable if its address is reused. [Nonces and API wallets](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/nonces-and-api-wallets) | Moriarty's consumed authority cannot depend only on an external signer's ephemeral nonce store. It needs durable logical-request consumption and explicit key-epoch retirement. |
| Typed external action | The Exchange API distinguishes order, cancel, modify, TWAP, withdrawal and other actions, each with exact fields; order responses distinguish resting from filled. [Exchange endpoint](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/exchange-endpoint) | A universal `success` receipt loses meaningful state. Adapter receipts need action-specific observed effects and open-order/cancel/partial-fill state. |
| Market governance | HIP-3 deployers define and operate perp markets, including oracle and settlement inputs, while inheriting HyperCore matching/margin facilities. Its docs distinguish protocol-valid conduct from subjective expectations and give validator slashing a separate role. [HIP-3](https://hyperliquid.gitbook.io/hyperliquid-docs/hyperliquid-improvement-proposals-hips/hip-3-builder-deployed-perpetuals) | Application policy, operator duties, objective settlement checks and governance remedies must be separate. A bond or slash is not proof of a correct price or payoff. |
| External bridge | Hyperliquid's native USDC bridge is described as connecting Arbitrum and Hyperliquid. The exchange withdrawal request precedes validators signing and sending the bridge transaction. [Trading guide](https://hyperliquid.gitbook.io/hyperliquid-docs/onboarding/how-to-start-trading), [Exchange endpoint](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/exchange-endpoint) | Even an integrated venue has an external bridge workflow. Request acceptance, validator signing, external submission and destination delivery are distinct facts. |
| Oracle | Validators publish oracle inputs used in funding, margin and liquidations. [Oracle](https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/oracle) | An oracle observation needs an issuer/policy and selected round in the language. The kernel can collect it, but cannot turn it into an objective fact by labeling it `verified`. |

## Refined architecture

Use two meanings of “kernel” carefully:

1. **Venue financial kernel:** the settlement domain's native matching, margin, transfer, oracle and liquidation machinery. HyperCore is an example. Moriarty does not own this machinery when it calls it.
2. **Moriarty DeFi Kernel:** the optional coordination service proposed for Moriarty. It constructs authorized calls to such venues, bridge and signer services, gathers evidence, reconciles effects and continues or recovers a workflow.

The intent language is a third layer. It defines allowed results, exact authority, resource limits, evidence and failure transitions. This three-way separation avoids giving the optional service credit for guarantees that only a venue's ledger or verifier can establish. It also avoids compiling an AMM, perp engine or bridge implementation into every Moriarty program. The distinction is an architectural **inference** from the source facts above, not Hyperliquid terminology.

### Adapter classes to expose

| Class | Example | Evidence required before Moriarty accepts a dependent transition |
| --- | --- | --- |
| `native-financial-call` | HyperCore order, cancel, vault transfer | Authenticated venue effect and state/receipt semantics; a resting order is an open duty, not a completed trade. |
| `same-domain-queued-call` | HyperEVM CoreWriter to HyperCore | Queue event plus later execution result; no completion from enqueue alone. |
| `foreign-chain-call` | Bridge withdrawal to Arbitrum | Source acceptance, signer/relayer steps and destination evidence according to the signed trust policy. |
| `signer-call` | Agent wallet or threshold service | Exact payload authority, current grant/key epoch and replay consumption; signing is not execution. |
| `observation-call` | Oracle or external chain result | Selected source, round, finality and verifier premise. |

The exact action schema is a proposed MIL/4 design task. A venue adapter may expose a richer order/position state than a bridge adapter; both must share stable IDs, exact payload commitments, fee accounting and typed outcomes. Direct Midnight execution remains possible without the optional coordinator.

## What to change in the NEAR-based proposal

- Add **native venue and same-domain queued calls** to the kernel capability matrix. The earlier text emphasizes foreign-chain signing and bridges; Hyperliquid shows that domain-local calls also need precise phase and effect boundaries.
- Require an explicit **settlement locus** on each leg: Moriarty/Midnight, foreign venue ledger, same-chain system contract, or bridge destination. This determines which verifier can establish which fact.
- Add an **open-order/partial-fill continuation** to operation receipts. Hyperliquid order responses can report a resting order; its eventual fill is a later fact.
- Split **application fee authority** from service and venue fees. Hyperliquid's builder code supplies an example of an approved maximum that later order-specific fees must respect.
- Include **key and nonce retirement** in the signer profile. External signer nonce pruning cannot erase Moriarty's consumed-authority history.

## Discriminating checks for the Aeon work

Aeon can probe whether a refinement-typed proposal rejects a plan with a larger fee, wrong recipient or changed signer domain, and whether a state transition requires evidence before moving from queued/submitted to executed/finalized. It cannot establish that HyperCore, NEAR, a bridge, a threshold signer or Midnight actually enforces the corresponding runtime predicate. The three parallel Aeon experiments will be reported separately and compared against this table after their command logs are available.

## Limits and provenance

Ten official Hyperliquid documentation pages were retrieved through Scrapling 0.4.15 on 2026-09-29. Captures and SHA-256 digests are recorded in [SOURCES.md](SOURCES.md). These are platform-authored descriptions. The HyperCore source implementation and a live chain transaction were not inspected. Documentation on HyperEVM rollout uses broad and feature-specific wording; this report relies on the concrete CoreWriter and timing pages for queue semantics and does not assert universal mainnet availability of every listed action. Source facts, architecture inferences and recommendations are separated above.

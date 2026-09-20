# Chain Signatures, MPC/TEE and Omni Bridge: what Moriarty can prove and what it cannot promise

Scope note. This is one bounded pass over the supplied excerpts only. I did not read omitted pages, did not open the pinned repositories beyond the excerpted files, and did not run code or query any live deployment. Every proposition below is tagged in the claims array as verified-from-excerpt, inferred, or unverified. Most excerpts come from two publishers (near/near-one and docs.near-intents.org); agreement between them is not independent assurance.

## Explanation

### Signing authority is not fact verification

The central architectural fact for Moriarty is that Chain Signatures produce a signature, not a settlement. The `v1.signer` contract exposes a `sign` method that takes a payload, a derivation path and a domain id, yields while the MPC network signs, and returns a signature that the caller must still relay to the destination chain (https://docs.near.org/chain-abstraction/chain-signatures lines 137-176; implementation page lines 231-295). The docs page itself states that Chain Signatures is a one-way tool for outbound transactions and points at Omni Bridge for reading external state (lines 36-37). A signature therefore proves, at most, that a threshold of MPC participants authorized a specific byte string under a key derived from the requesting NEAR account and path. It does not prove broadcast, inclusion, finality, or that the destination chain interpreted the bytes as intended.

The MPC README adds a second capability the older docs do not describe: foreign chain transaction verification, in which nodes independently query configured RPC providers, run deterministic extractors, and produce a threshold signature over the observed values (/home/charl/research/near-teardown-2026-09-19/repos/near/mpc/README.md line 38). This is an attestation of RPC responses, not of chain consensus. The foreign-chain design confirms the safety statement is relative: fewer than `signing_threshold` participants cannot force a false attestation, and each node compares every configured provider and errors on disagreement (/home/charl/research/near-teardown-2026-09-19/repos/near/mpc/docs/design/calculating-supported-foreign-chains.md lines 60-72, 123-126). The same file says the terminal-on-disagreement rule is an implementation requirement, not current behavior, and that the voted RPC quorum is stored but not yet consumed (lines 17-19, 65-67). For Moriarty, an MPC-signed foreign observation is exactly what the product contract calls a named assumption (/home/charl/Moriarty-aeon-study/docs/MORIARTY-PRODUCT-CONTRACT.md line 39): it can be bound into a proof statement as an observation, never substituted for the transition proof.

### Governance, operators and upgrades are one trust domain

The domain-separation design records that today the governance voting threshold and the cryptographic reconstruction threshold are the same value (/home/charl/research/near-teardown-2026-09-19/repos/near/mpc/docs/design/domain-separation.md lines 5-9). The split into `GovernanceThreshold` and per-key `ReconstructionThreshold`, with a 60 percent governance floor, is proposed; PRs 1 and 2 are marked done, PRs 3 through 7 are not (lines 339-361, 383-600). The TEE design states that participants also act as operators (/home/charl/research/near-teardown-2026-09-19/repos/near/mpc/docs/design/securing-mpc-with-tee/securing-mpc-with-tee.md line 136), and that the node account key is granted access to all contract methods, widening the in-contract blast radius as an accepted tradeoff (lines 414-423). New signature domains and participant sets are added by participant vote (`vote_add_domains`, `vote_new_parameters`; docs.near.org chain-signatures lines 179-181, 193-197). Image upgrades follow a launcher pattern where a threshold of participants votes a new image hash and stale nodes are evicted after a default seven-day deadline (/home/charl/research/near-teardown-2026-09-19/repos/near/mpc/docs/design/tee-lifecycle.md lines 262-298).

The consequence for Moriarty: the signer key, the rules for who signs, the code they run, and the foreign-chain RPC policy are all controlled by the same participant set. A Moriarty proof that binds a signer public key binds an artifact whose controlling semantics can change by vote without the user's consent. The proof statement must therefore pin the signer contract account, the domain id and the public key at signing time as execution-domain facts, and the signed intent must state whether a later participant or key change invalidates the plan.

### TEE narrows the threat model; it does not remove operators

The threat model states a conservative TEE assumption: integrity is relied on, confidentiality is best-effort (/home/charl/research/near-teardown-2026-09-19/repos/near/mpc/docs/design/threat-model-diagram.md line 385). Disk rollback is listed as an open risk (T6, line 281), and T13 records that attestation proves what code runs, not who owns the machine (lines 288, 298). The attestation-verifier design adds that the MPC contract cannot verify another account's code hash or access-key list, so auditing a candidate verifier is a voter responsibility (/home/charl/research/near-teardown-2026-09-19/repos/near/mpc/docs/design/attestation-verifier-contract.md lines 233-240), and that rotating a broken verifier does not purge attestations it already accepted; they age out over up to seven days, described as containment of future trust, not remediation of past damage (lines 212-218). Moriarty cannot inherit TEE claims as proof; at most a Moriarty deployment can record the attestation policy hash as an observation.

### NEAR execution patterns Moriarty must model

Three NEAR facts in the excerpts shape contingent settlement. First, a failed promise result carries no payload; a rejection and an unreachable callee are indistinguishable unless the outcome is returned as a value on a successful receipt (attestation-verifier-contract.md lines 284-292). Second, yield-resume callbacks fire exactly once, either on resume or after roughly 200 blocks of silence, and an out-of-gas in the callback drops cleanup with no retry (lines 114-126). Third, the target of a cross-contract call is fixed when scheduled, not re-read at execution, so governance changes can race in-flight calls (lines 357-358). These are the concrete shapes of the product contract's requirement that each failure outcome be an explicit transition retaining residual duties (MORIARTY-PRODUCT-CONTRACT.md line 39). A Moriarty NEAR pattern library should model a sign request as a state machine: requested, signature returned, timeout without signature, signature returned but destination outcome unknown, destination confirmed by observation, destination reverted by observation.

### Omni Bridge: per-direction verification

Inbound to NEAR, Omni Bridge verifies a source-chain proof: light clients for Ethereum, Bitcoin and Zcash; Wormhole for Solana, BNB and EVM L2s (/home/charl/research/near-teardown-2026-09-19/repos/Near-One/omni-bridge/README.md lines 14-18; https://docs.near.org/chain-abstraction/omnibridge/overview lines 24-48). Outbound, MPC operators sign a destination-chain payment and relayers only carry it (https://docs.near-intents.org/learn/omni-bridge/overview lines 10-17, 26-29). Near One runs the bridge (line 8). Anyone can submit an inbound proof (line 25). Inbound trust therefore differs by chain: a light client inherits the source chain's consensus and the light client's own correctness; Wormhole adds a guardian set as a distinct trust root; neither is described in the excerpts at the level of a verifiable specification. The README lists versioned prover helper contracts (eth-prover-0_4_1, vaa-prover-0_4_3; lines 36-38) but the upgrade authority over `omni.bridge.near` and those provers is not in the excerpts. Finality waits are published (Ethereum 960 s, Arbitrum 1066 s, Base 1026 s, Solana 14 s; lines 88-93); those are operator policies, not proofs of irreversibility.

The Intents bridging page documents a manual BTC refund path for deposits that never finalized on NEAR (https://docs.near-intents.org/integration/bridging/overview lines 52-53). That is direct evidence that partial and unknown states exist in production and that recovery is operational, not protocol-guaranteed.

## Reference

### Trust boundary table (from excerpts)

| Boundary | Who controls | Excerpt basis | Moriarty treatment |
|---|---|---|---|
| Signer key and domains | MPC participants by vote | chain-signatures lines 179-197; domain-separation lines 5-9 | Pin contract, domain id, public key as execution-domain facts |
| Foreign-chain observation | Participants plus their configured RPC providers | mpc README line 38; calculating-supported-foreign-chains lines 60-72 | Named observation assumption; never a transition proof |
| TEE attestation policy | Participants vote image and verifier hashes | tee-lifecycle lines 238-247; attestation-verifier lines 208-218 | Observation of policy hash only |
| Inbound bridge proof | Light client or Wormhole guardians per chain | omni-bridge README lines 16-18 | Per-chain named assumption, distinct per route |
| Outbound relay and destination inclusion | Relayers and destination chain | intents omni-bridge overview lines 10-17, 26-29 | Cannot be promised; explicit pending/unknown states |
| Bridge contract upgrades | Not in excerpts | omni-bridge README lines 31-39 lists versioned provers | Unverified; treat as assumption |

### Failure outcomes a Moriarty NEAR pattern must enumerate

1. Sign request yields, MPC never responds, callback fires with failed promise after roughly 200 blocks. Fees and any guaranteed-phase effects are retained.
2. Signature returned, caller never broadcasts. Authority was exercised; no destination effect.
3. Signature returned and broadcast; destination outcome unknown at time of NEAR observation.
4. Destination chain includes the transaction, later reorganizes. Excerpts give operator wait times only.
5. Payload lacks chain binding (pre-EIP-155 or shared UTXO format); the same signature may be valid on another chain (chain-signatures lines 113-135).

## How-to

### How to bind a Chain Signatures request into a Moriarty proof statement

1. Encode the destination transaction from typed fields inside the Moriarty program; hash it to the exact payload passed to `sign`. The proof shows payload equals hash of a transaction whose effects refine the signed intent under a named destination-chain model.
2. Require the destination encoding to include chain identity, and use a per-chain derivation path, following the replay guidance (chain-signatures lines 129-135).
3. Record the signer contract account, domain id and derived public key in the execution domain.
4. Model the sign yield as a fallible phase with an explicit timeout transition.
5. Treat any later foreign-chain verification result as an observation with its provider policy hash, not as settlement.

### How to represent an inbound Omni Bridge deposit

1. Name the route (light client or Wormhole) per chain as a distinct assumption.
2. Treat bridge acceptance on NEAR as the settled event for NEAR-side duties; treat source-chain finality as the bridge operator's policy, not a proof.
3. Provide a recovery transition for deposits that never finalize on NEAR, mirroring the documented manual BTC refund path, without promising a refund.

## Tutorial

A developer authoring a BTC-settled loan in Moriarty would: declare a liability opened on NEAR when the borrower signs intent; request an MPC signature over a Bitcoin transaction paying the lender, with the payload derived in-program; mark the liability as pending-settlement once the signature is returned; consume an MPC foreign-chain observation of the Bitcoin transaction as a named assumption to discharge the liability; and provide explicit unknown and recovered transitions that keep the liability open if the observation never arrives. At no point does the program claim the coins moved; it claims that if the observation assumption holds, the liability discharge is valid. This matches the product contract's rule that debt cannot disappear through a local success result (MORIARTY-PRODUCT-CONTRACT.md line 37).

## What remains unverified

The excerpts do not show: the actual `v1.signer` contract source at the pinned mpc commit; the omni.bridge.near admin or upgrade keys; the Wormhole guardian set and its verification on NEAR; the light client contracts' correctness or their own upgrade paths; whether foreign-chain verification is enabled on mainnet; whether the domain-separation PRs 3 through 7 have landed; whether the terminal-on-disagreement rule has been implemented; and any of the referenced audit reports. The near/intents, amm-solver and sdk-monorepo commits are pinned but no excerpt from them was supplied, so nothing about the Verifier contract or solver behavior is verified here.
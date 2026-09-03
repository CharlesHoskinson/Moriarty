# Blinded review brief: Moriarty intent research prompt version 1.1

Review type: independent non-author architecture and specification review

Frozen base: `79b02272b37b4f77065930610edd62695d839128`

Frozen head: `0069cec3ad190a06975bc45cded2a9b63fea16c2`

Non-raw diff SHA-256:
`1ed62e791402a2e9fc14742f95737088d929e65fe6466419fc62bd1309042776`

Candidate prompt SHA-256:
`68d083631288556df9e120e1a517eedeb8a273eb22f2ccd47bbeefc93cd1fd5e`

Research source SHA-256:
`70b05ef3007c02c714d34c044580b2daf7009a358db6280090ab02d107a76e82`

Semantic scope: Moriarty `0.0.0-e00.2`, unchanged, digest
`9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`.

## Review object

The candidate is a deep-research assignment for a finite financial-agreement
Core plus a typed intent calculus that compiles through Compact to ZKIR. It has
12 workstreams, 13 experiments, 18 deliverables, 12 evidence-gated sprints, 18
release gates, and 33 required intent data contracts.

The intent-safety theorem requires a well-typed intent, valid authorization,
fresh state, verified artifacts, a locally verified plan, and an execution
relation. The conclusion permits only the signed intent's effects under named
assumptions. Separate theorems cover no extra spend, diverted change, mint or
burn, fees, signers, disclosures, capabilities, replay, cancellation,
partial-fill residuals, and composition.

The prompt distinguishes:

1. user objective;
2. provider quote;
3. signed authorization;
4. protocol order payload;
5. resolver snapshot and resolved plan;
6. wallet transaction or UserOperation;
7. fill;
8. fulfillment proof;
9. claim, cancellation, or refund;
10. final spendable settlement.

The candidate requires each lifecycle transition to record actor, authority,
signed fields, domain, network, verifying contract, nonce, validity, mutable
state, idempotency, observations, disclosures, proof, failure, retry,
cancellation, rollback, and finality.

The prompt separates six meanings that are often called partial fill or
atomicity: divisible economic fill, partial output completion, same-chain
transaction atomicity, contiguous wallet execution, cross-domain
all-or-refund, and end-to-end atomicity. It prohibits a general claim of
cross-domain atomicity.

## Evidence incorporated

CAKE contributes Application, Permission, Solver, and Settlement boundaries.
It does not supply a normative wire schema, complete cancellation or refund
semantics, residual-intent algebra, finality rule, or settlement receipt.

The complete official NEAR Intents documentation corpus was acquired: 68 of 68
sitemap pages, `llms-full.txt`, 1Click OpenAPI 0.1.10, and Explorer OpenAPI
0.0.0. The deployed comparison shows:

- `token_diff` batches conserve each token on the Verifier's internal ledger;
- signed payloads bind signer, verifier, deadline, nonce, and action list;
- external calls are asynchronous and excluded from Verifier simulation;
- quote, Verifier settlement, bridge withdrawal, payout, refund, and
  destination finality are different states;
- limit-order cancellation is asynchronous and may lose to a final fill;
- relay delivery is bounded at-least-once and needs durable deduplication;
- 1Click names a trusted swapping agent;
- confidential execution adds a permissioned private chain, treasury, private
  relay, and PoA bridge;
- documentation and OpenAPI disagree on some required fields, statuses,
  authentication, signature profiles, and examples.

The Ethereum comparison shows:

- the post-2026-05-13 ERC-7683 draft is resolver-based;
- its prior order/open/fill interfaces are no longer current;
- current OIF implementations still use `StandardOrder` and `MandateOutput`;
- resolution has code-upgrade, mutable-query, witness, assumption, and
  time-of-check/time-of-use risks;
- EIP-712 does not provide replay protection;
- contract signature validity can be state-dependent;
- CAIP-10 and ERC-7930 do not create universal text canonicalization;
- no reviewed standard supplies general end-to-end cross-chain atomicity.

## Candidate SDK additions

The standard interface adds `createObjective`, quote request and verification,
solver discovery, resolution verification, intent cancellation, fill and
fulfillment verification, claims, refunds, finality checks, delivery
acknowledgement, durable deduplication, subscription resume, and status
reconciliation. It also requires a disclosure matrix for every actor.

These are research requirements only. The 65-component SDK baseline remains
unchanged until an OpenSpec change reconciles component ownership and schemas.

## Review questions

1. Is the object taxonomy complete, minimal, and non-overlapping?
2. Does the intent-refinement theorem still hide authorization, state, solver,
   settlement, finality, proof, or privacy assumptions?
3. Are NEAR, current ERC-7683, prior ERC-7683, OIF, and CAKE assigned the right
   evidence status and compatibility boundary?
4. Are cancellation, partial fill, recovery, at-least-once delivery, rollback,
   and confidential execution specified strongly enough to prevent false
   safety claims?
5. Is the complete SDK interface sufficient for a client to reject an
   unauthorized plan before signing and to verify fulfillment after execution?
6. Which exact prompt changes are release-blocking, and which are optional?

## Required response schema

Return one JSON object only:

```json
{
  "reviewer_role": "string",
  "verdict": "approve | changes_requested | reject | abstain",
  "blocking_findings": [
    {
      "id": "B01",
      "finding": "string",
      "evidence": "string",
      "exact_change": "string"
    }
  ],
  "nonblocking_findings": [
    {
      "id": "N01",
      "finding": "string",
      "exact_change": "string"
    }
  ],
  "strongest_design_feature": "string",
  "largest_unsoundness_risk": "string",
  "confidence": "low | medium | high"
}
```

Do not identify or speculate about other reviewers. Do not use external tools
or web search. Base the review only on this frozen brief.

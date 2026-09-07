# Midnight network execution evidence — 2026-09-07

This package records an official Midnight local-dev settlement test. It does not
establish any Moriarty semantic, proof-carrying-transaction, recursion, or native
financial-proof claim.

The local stack remains running on loopback ports `19944` (node), `18088`
(indexer), and `16300` (proof server). Each namespaced container has a 5 GiB
memory, 4 CPU, and 1024 PID ceiling. Exact image digests and transaction receipts
are in `local-receipts.json`; the unredacted funding log contains public addresses
and transaction identifiers but no wallet seed.

The first local test settled four distinct transactions: NIGHT transfer, DUST
registration, Compact contract deployment, and the `storeMessage` contract call.
The call's state read back as `Moriarty settlement test`. The indexer reported
`SUCCESS` for each transaction, and the node later reported finalized height 222,
above receipt heights 7, 12, 86, and 201.

The generated official hello-world scaffold initially installed two physical
copies of `@midnight-ntwrk/onchain-runtime-v3` (3.1.1 and 3.0.0). This made the
call fail because a `StateValue` constructed by one WASM module was not an
instance of the other module's class. Pinning `3.0.0` in npm overrides and
running `npm dedupe` left one physical runtime copy; the same call then settled.

Public Preprod remains incomplete. The dedicated address and exact blocker are
recorded in `public-preprod-status.json`. The official faucet requires a
Cloudflare Turnstile CAPTCHA, so no automated request was attempted.


A second deployment/call pair used the dedicated funded local wallet through
`MIDNIGHT_WALLET_SEED_FILE`. Those receipts are at blocks 293 and 305, and the
second contract read back `Dedicated wallet settlement`. There are six recorded
transactions in total. The original height-222 finality observation covers only
the first four; [parent verification](parent-six-receipt-verification.json)
checks all six against the indexer and node, with finalized height 358.
Independent [first contract readback](parent-contract-state-verification.json)
and [dedicated contract readback](parent-dedicated-state-verification.json)
confirm both messages without submitting another transaction.

The [reproduction instructions](../../experiments/moriarty-midnight-network/README.md)
preserve the official source pins, local patch, scaffold lockfile, immutable
Docker image digests and external wallet-state paths. Source modifications are
experimental. The upstream scaffold's build command masks TypeScript failures
with `|| true`; it is not used as evidence of typecheck success here. The proof
of local execution is the actual contract transaction and state receipts.

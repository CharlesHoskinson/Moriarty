# Proposed experiments for a Moriarty eUTXO backend

Every experiment below is **specified-only**. None was executed in this research task. They do not replace Midnight Preview gates or authorize new financial/proof campaigns. Select and review a bounded target profile before implementation.

## P0: minimal authenticated state machine

Use one fixed loan or escrow agreement, one state output and one script, with public state initially. Bind semantic profile, Core/program digest, asset identifiers, participant keys, instance identity, genesis, lifecycle, revision, remaining work and obligations. Use a one-shot mint policy tied to a consumed seed out-reference to identify genesis; enforce its minting, propagation and terminal burn/disposition. Document all permitted scripts and transitions.

| Input/scenario | Expected observable result |
|---|---|
| Correct signed genesis and one bounded valid step | Exactly one authenticated live successor, expected state and exact authorized native outputs |
| Two distinct valid transactions spending same head | At most one appears on the canonical chain; other conflicts, with retry/finality recorded separately |
| Duplicate genesis, counterfeit state token, extra token mint | Reject, with no counterfeit live instance |
| Missing/duplicate successor or successor at attacker script | Reject; state identity cannot disappear or fork except a declared split/termination |
| Fake datum at same script address without genuine instance token | Reject |
| Reference-read of state without consuming it | Cannot count as a state transition or consume authority |
| Correct hash but incorrect contract property certificate/build | Fails registration/deploy audit; hash alone never promotes invariant claim |
| Stale revision, wrong instance/program/network, wrong actor | Reject |

Capture complete transaction CBOR, validator and mint-policy hashes, blueprint, target protocol/cost model, genesis identity, evaluator input/output, ledger result and independently decoded state. Test both direct local evaluation and target-ledger validation; their scopes differ.

## P1: complete loan settlement and economic projection

Replay the semantics of one retained bounded loan period in a new target-specific profile. Include interest rounding, principal allocation and residual debt. Define which wallet/input owner is the debtor, which input supplies fees/minimum ADA, and which movements are contract effects versus authorized funding mechanics.

| Input/scenario | Expected observable result |
|---|---|
| Correct principal plus interest | Creditor receives exact target units; residual principal and accrued amounts match Core |
| Wrong asset policy/name, wrong creditor/staking credential, wrong amount | Reject |
| Input 100,000 A, business transfer 10,000 A, change 90,000 A, gross cap 10,000 A | Reject under retained whole-input gross policy |
| Same values with gross cap 100,000 A and valid net goals | May accept if all other conditions hold; change does not replenish allowance |
| Refund/inflow disguises excess gross spent authority | Reject |
| A token bundle carries unrelated assets | Preserve them to the authorized change owner or reject unauthorized movement |
| Fee or required funding pushes net goal below bound | Reject or rebuild before signing; payer sponsorship must be explicitly modeled |
| Valid ledger transaction reduces debt without matching payment | Reject |
| Overpayment, partial payment, floor/ceil edge and overflow | Match independently computed semantic disposition and residual obligations |

Read all actual inputs, outputs, mint/burn, fees, withdrawals/deposits and collateral branches relevant to the target era. Do not project only script-local outputs or replace the whole-input policy with net balance difference. UTXO value conservation is necessary, but cannot prove discharge of nominal liabilities.

Cardano PlutusV3 TxInfo does not expose collateral inputs/return/total collateral. Enforce collateral exposure in a separately specified final-transaction signing/wallet policy and record actual failed-transaction effects; do not invent an on-chain validator check against unavailable fields. A validator that rejects cannot itself prevent collateral being charged for that rejection. Fee and normal-input/output checks use the actual fields available to the pinned script context.

## P2: double satisfaction and compositional effects

Construct one transaction with two loan or order states. Supply a payment sufficient for only one state. Both validators must not reuse it to accept both settlements. Bind each payment/effect allocation to an instance and input out-reference or enforce a globally checked injective allocation. Reject repeated heads, mixed policies, missing effects, recreated allowance, dropped obligations and undeclared recipients. Test aggregation only under a specified allocation rule.

For declared Split/Join, retain child identity derivation, nonce history and conserved allowances/work/obligations. Spend each join input once and preserve provenance. A single state-token representation cannot advertise arbitrary split/join before the identity protocol is specified.

## P3: outcome authority across heads

Sign outcome constraints independently of transaction witnesses. Freeze a canonical target encoding and digest vectors. Bind signer, network, program allowlist, instance scope, permitted actions/recipients, gross caps, minimum net goals, liability limits, observation policy, certificate requirements and expiry. Consume outcome nonce in state with a bounded storage/retirement policy or a reviewed one-shot authority-token scheme.

Define whether once-only scope is per-instance or global across route candidates. Per-instance nonce sets cannot supply a global guarantee. Test COSE signature-structure verification separately from transaction-body witnesses and require exact credential/address mapping. Required-signer hashes are not the set of all witness keys.

Reject signature reuse on a later head, reuse after cancellation, reuse across two concurrently split children, replay after migration and altered fee sponsorship. A signed transaction body protects that body; it is not a route-independent outcome intent signature. Exact-head authority replay should already fail after its identified out-reference is consumed. Record how rollback of an unconfirmed transaction restores availability without incorrectly marking finalized authority reusable.

## P4: time, oracle and failed transaction semantics

Require a bounded validity interval wholly within the agreement/observation admissible window. Authenticate feed data and freshness over that interval. Test very wide intervals, old reference inputs, signatures from unapproved feeds, boundary slots, timezone/calendar conversion and transactions validated close to expiry. Current block time is not directly interchangeable with a Cardano validity interval.

Exercise phase-one rejection, phase-two-invalid inclusion with collateral charge, expiry, mempool conflict and chain rollback. Expected records distinguish semantic state unchanged from wallet collateral lost, transaction pending from confirmed, and chain inclusion from sufficient finality. Define gross/fee/collateral risk authorization for sponsor and principal separately.

## P5: target-compatible private step proof

Only after public semantic/state correspondence passes, choose a proof system and verifier supported by the pinned target Plutus version/cost model. Produce a retained proof for a representative complete relation; independently verify it and evaluate the actual on-chain verifier. Bind all public inputs: program/verification key, predecessor identity/commitment, authorization digest, state result, asset movements, liabilities, nonce and observation policy.

Controls: incorrect key, altered input/state/effect, wrong asset/recipient/network, stale head, reused nonce, wrong guard, missing proof, malformed encoding, invalid curve point and substituted verifier. Record proof bytes, setup/trust assumptions, proving time/peak memory, verifier CPU/memory, script bytes, transaction bytes and fees at the captured protocol parameters. A generic Groth16 or Halo2 demo is not Moriarty relation feasibility. BN254/KZG or Poseidon proof formats cannot be treated as BLS12-381 support merely because they are succinct proofs.

## P6: private witness availability and handoff

Give the successor participant only its encrypted opening and allowed public state, using distinct OS users or containers. Verify it can generate the next proof without predecessor private fields. Tampered/wrong-recipient openings fail. Commitments do not solve opening availability, lost keys or forced closure; define these outcomes and disclose which state/action/value metadata remains public.

## Decision criteria

| Gate | Minimum evidence before claim |
|---|---|
| Public backend semantics | Exact accepted/rejected outputs match frozen Core cases and complete native effect projection |
| Ledger state lineage | Unique genesis, continuation, replay and conflict controls pass on actual target validation |
| Financial authority | Gross inputs, net goals, liabilities, fees and collateral branches match signed policy |
| Composition | Double-satisfaction and split/join conservation controls pass |
| Private parity | Representative ZK verifier fits measured target limits and binds complete relation; handoff test passes |
| Production/backend acceptance | Independent exact-candidate review, target-ledger evidence, published compatibility scope and required project gates |

If private proof verification fails, public simulation or a public backend can remain a separately labeled capability. It cannot be advertised as preserving Moriarty's complete Midnight privacy/acceptance contract. No generic production claim follows from the experiments' existence.

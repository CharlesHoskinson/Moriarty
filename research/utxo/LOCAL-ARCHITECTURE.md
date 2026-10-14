# Moriarty architecture inspected for UTXO research

Evidence classification: repository observations and compatibility inferences, not a new implementation or network experiment. Exact file digests are in [MORIARTY-EVIDENCE.json](MORIARTY-EVIDENCE.json). Baseline commit: `84f64b30df53ff0b8ccaf475f57324d3d7c6e594`.

## What exists

Moriarty has a language front end, typed bounded Core, local evaluation, restricted Compact generation, specialized funded Compact custody wrappers, and retained bounded Preview financial examples. These are separate layers, with different acceptance scope. `Prepared`, a computed effect descriptor, a generated transition kernel, a valid cryptographic proof, and a finalized financial settlement do not mean the same thing.

The atomic `/1` manifest binds source and program hashes, Core, resource bounds, policies, nominal units, settlement asset/quantum bindings, state schema and required claims. Core instructions are guards, immutable locals, state writes and emitted financial effects. Effects include Transfer, Fee, DueCreated and DueSettled. This is a useful reusable semantic boundary; it is not itself an eUTXO transaction format.

Later successor profiles have their own schemas and factories. Source `/6` exists locally and delegates to financial expression `/5`. The loan lifecycle `/2` has Transfer, Repay, Originate and Accrue, preserves residual obligations and explicit rounding, and models work and allowances. Research must select a specific profile before proposing compiler correspondence. It cannot claim the atomic backend supports all successor constructs.

`lower-compact.ts` publishes explicit restrictions: typed operands and state transition arithmetic only; no obligation-ledger/authority/proof/settlement acceptance relation. It rejects unsupported persistent/observed text and Boolean short-circuit forms. Arithmetic hints are constrained by the generated circuit. A new UTXO backend needs its own lowering and rejection contract; an off-chain evaluator returning `true` is insufficient.

The custody wrappers pin the program, network, parties, assets and revision, constrain ledger time, and perform native movements. Their hard-coded examples are narrower than a generic backend. Historical loan evidence pays 533,972,602 test-token units, with 4,500,000,000 residual principal. The later corrected swap evidence records command exit zero, UTXO linkage and native effects: spend a 100,000-A wallet output, trade 10,000 A for 19,743 B, recreate 90,000 A change, and later close pool reserves. Independent audits establish those scopes under trusted node/indexer observations; they do not establish authenticated state proofs or general mandatory acceptance.

## The current proof/history direction

The September 11 amendment takes precedence over older prose for the intended mechanism, while remaining specified-only with outstanding review/experiments. It selects a ledger-anchored certified state machine:

1. Constrained, signed genesis.
2. Immutable operation keys audited against a reproducible build.
3. A fused per-step relation checking authorization, transition, effects, intent and invariant.
4. A head read followed by a write of that same head, checked in the transcript.
5. HistoryCompliance by induction over ledger-accepted head extensions.

The head binds instance, head identity, revision, state commitment, lifecycle, program digest and network tag. Exact-head authority binds this state; a consumed head/revision prevents its replay. Outcome authority remains valid across eligible heads, so it also requires durable per-instance nonce consumption. A head token alone does not prevent an old outcome signature from being used on a later head.

The universal contract property certificate is checked by the deploy audit and bound into the program digest. A certificate hash alone does not discharge the property. Recursive proofs serve bounded off-ledger segments and imports, conditional on a compatible native route and accepted resources. They are not a prerequisite for every ordinary on-ledger history step in this revised design. Ordinary Midnight contract proofs use a different transcript from inner Poseidon proofs; proof bytes cannot be interchanged because both are called Halo2.

## Candidate correspondence to eUTXO

The following are design inferences to validate against the research sources and actual target tooling.

| Moriarty concept | Candidate eUTXO realization | Obligation that remains |
|---|---|---|
| Live head | Authenticated state output with datum/commitment and unique thread asset | Enforce unique genesis/minting and successor count, not merely a matching datum |
| Revision-bound step | Spend the exact state out-reference, validate action, create exact successor | Audit every accepted path, including terminal and migration paths |
| Immutable operation key | Content-addressed validator script and pinned language/toolchain/cost model | Validate expected script/address and protocol upgrades; same script serves many instances |
| Complete state/effects | Redeemer-driven transition plus transaction-wide input/output/mint/fee projection | Prevent duplicate satisfaction, unlisted movements and wrong recipients |
| Exact-head authority | Signature binding network, instance, consumed head, program, action and limits | Separate off-chain intent signature from a transaction-body witness |
| Outcome authority | Signed constraints plus consumed-nonce state/token discipline | Preserve nonce use through split/join/migration and retries |
| Observation policy | Signed oracle data and bounded validity interval | Freshness must hold throughout the accepted interval; a reference input only supplies data |
| Budgets and duties | Successor datum with conserved work/authority/obligations | Native value conservation says nothing about nominal debt, caps or liability creation |
| Atomic composition | One transaction spending distinct participating states and producing validated successors | Every validator must bind its own obligations and shared output allocation |
| Private fields | Commitments plus target-compatible ZK verifier and encrypted recipient opening | Public datums/redeemers do not provide Midnight privacy automatically |
| Property certificate | Audited registration/genesis policy bound to program digest | Deployment verification remains a separate trust/assurance claim |

A read-only reference input must never count as consumed state or consumed authority. An output reference is not a persistent instance identity. A validator address is not a unique contract instance. An NFT or state token is only an identity mechanism if its mint/burn and continuation paths enforce the intended lifecycle.

## Financial subtleties the adapter must preserve

The existing swap result distinguishes gross authorized wallet input from the economic trade. Under that scoped policy the whole 100,000-A input counts against authority even though 90,000 A returns as change. The local outcome checker counts explicit Transfer/Fee records; a target adapter must supply the complete movement projection and enforce the intended gross-input rule. It cannot reuse that checker on the 10,000-A business transfer alone and declare native authority preserved. This is a cross-layer obligation, not an inference that all current profiles implement identical accounting.

Minimum ADA funding and collateral introduce target-specific additional authority. Unrelated native assets in an input must be preserved or explicitly authorized. A phase-two-invalid Cardano transaction can charge collateral while leaving the attempted contract transition unapplied; semantic rejection and financial wallet outcome need separate records. Which payer sponsors fees/collateral must be explicit in the signature and effect schema.

ACTUS calendars, decimal scale, rounding, due dates and residual liabilities stay semantic responsibilities. Ledger token conservation does not establish them. Likewise, a decreasing work bound does not establish that a counterparty or executor will submit a due transaction, that a price feed remains available, or that there is sufficient future funding to close the contract.

## Scope boundary

This study does not change Moriarty's sole Preview execution gate, start a financial retry, expand proof-campaign resources, modify the language, or claim backend acceptance. It produces a reviewable target assessment and specified experiments. The startup CLI's stale binding/accounting gaps govern its dependent campaign dispatch; they do not block this independent read-only research.

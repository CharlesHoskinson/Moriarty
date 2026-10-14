# Open-source apps to learn UTXO design patterns

Start with the validator and its accepted transaction shape, then trace the builder. All references were researched on2026-10-12; three app snapshots were inspected directly. The catalogue distinguishes complete protocol components from SDKs and tutorials. No app was run, audited anew or checked against fresh deployed bytes. Exact acquisition metadata is in [researcher6's report](06-reference-apps/REPORT.md) and [sources.json](06-reference-apps/sources.json).

## Suggested reading order

| Order | Reference | What to learn | Concrete reading path |
|---|---|---|---|
| 1 | [Aiken vesting tutorial](https://aiken-lang.org/example--vesting/mesh) | Datum/redeemer, signatures, deadlines, lock/unlock builder | Captured `06-reference-apps/vesting-page.extract.txt`; tutorial `validators/vesting.ak`. Educational example, not production assurance. |
| 2 | [Marlowe](https://github.com/marlowe-lang/marlowe-cardano) | Financial DSL → evaluator → validator → transaction builder; authenticated contract continuation and role payouts | `03-marlowe/marlowe-cardano/marlowe-plutus/src/Marlowe/Plutus/{Semantics,Scripts}.hs`, then `marlowe-transactions/src/Language/Marlowe/Runtime/Transaction/BuildConstraints.hs` and `Safety.hs`. |
| 3 | [Minswap V2](https://github.com/minswap/minswap-dex-v2) | One-shot authenticated genesis, state tokens, factory/pool lifecycle, batch settlement and builder correspondence | `06-reference-apps/minswap-dex-v2/validators/authen_minting_policy.ak`, `factory_validator.ak`, `pool_validator.ak`, `order_validator.ak`; `lib/amm_dex_v2/order_validation.ak`; `src/batcher-sample.ts`. |
| 4 | [Sundae contracts](https://github.com/SundaeSwap-finance/sundae-contracts) | Shared withdrawal validation, input-index uniqueness, sequential pool effects, exact-once payout allocation | `06-reference-apps/sundae-contracts/lib/calculation/InputSorting.md`; `validators/{order,stake,pool}.ak`; `lib/calculation/process.ak`. Read all delegated validator paths together. |
| 5 | [Cardano-Loans](https://github.com/fallen-icarus/cardano-loans) | Active loan state, partial payment, residual rational debt, tagged creditor payments, default/recovery and recipient compatibility | `06-reference-apps/cardano-loans/aiken/lib/cardano_loans/data/datums.ak`, `payment_validation.ak`, `accept_validation.ak`, `burn_and_claim_validation.ak`; then `scripts/local-node/`. V2 explicitly unaudited. |
| 6 | [Cardano-Swaps](https://github.com/fallen-icarus/cardano-swaps) | Distributed order states, previous-input linkage, expiration and asset purity | Captured README in `06-reference-apps/swaps-page.*`; downloaded source `reference-app-snapshots/cardano-swaps/{aiken,src,scripts}`. Implementation bodies were not substantively reviewed in the initial discovery pass. |
| 7 | [Cardano-Aftermarket](https://github.com/fallen-icarus/cardano-aftermarket), [Cardano-Options](https://github.com/fallen-icarus/cardano-options) | Composable financial rights: lender-address updates, premiums, exercise, expiry and writer recovery | Captured READMEs `06-reference-apps/{aftermarket,options}-page.*`; study lifecycle requirements before copying token ownership patterns. |
| 8 | [Spectrum contracts](https://github.com/spectrum-finance/cardano-dex-contracts) + [backend](https://github.com/spectrum-finance/cardano-dex-backend) | Pairing validator conditions with order-executor, indexer, wallet and retries | Captured READMEs `06-reference-apps/spectrum-{contracts,backend}-page.*`; `cardano-dex-contracts-{onchain,offchain}`, `amm-executor`, `wallet-helper`. Backend licensing/maintenance unverified. |
| 9 | [MuesliSwap hybrid DEX](https://github.com/MuesliSwapTeam/muesliswap-liquidity-efficient-dex) | Pool/order hybrid and concentrated liquidity checks | Captured `06-reference-apps/muesli-page.*`; `poolScript/` and implementation README. Preprod examples are not current mainnet evidence. |
| 10 | [Hydra](https://github.com/cardano-scaling/hydra) | L2 state machine, confirmed snapshots, deposits/decommits, contestation and L1 fanout | `05-concurrency/hydra/docs/docs/protocol-overview.md`, `known-issues.md`, then `hydra-node/src/Hydra/HeadLogic.hs`. Check exit feasibility, especially head-minted assets. |

Optional governance reference: [Agora](https://github.com/Liqwid-Labs/agora) is a validator library, not Liqwid lending's full app. Minswap/Sundae SDKs demonstrate client integration, but a SDK does not replace inspection of on-chain conditions. [Aiken gift card](04-toolchains/aiken-repo/examples/gift_card/validators/oneshot.ak) is a small local example of one-shot minting/spending.

All12 accessible catalogue repositories are downloaded: the initial3 inspected apps in `06-reference-apps/`, plus9 ancillary snapshots in `reference-app-snapshots/` (both SDKs, Swaps, Aftermarket, Options, Spectrum contracts/backend, Muesli and Agora). Extra downloads extend source availability, not the scope of the initial code review. Each additional snapshot has its own remote/branch/full-commit receipt.

## Patterns to extract for Moriarty

| Pattern | Best references | Question the reader should answer |
|---|---|---|
| Authentic state and constrained genesis | Minswap, current Marlowe | Can anyone create a same-address output or mint a counterfeit state token? |
| Controlled continuation | Marlowe, loan state | Must every live step produce exactly the right successor with residual duties? |
| Input/output attribution | Sundae, Cardano-Loans, Swaps | Can a repeated input index or one payment satisfy two distinct obligations? |
| Shared validation/batching | Minswap, Sundae | Which validator checks the complete relation, and how is every order forced through it? |
| Private authority versus bearer assets | Marlowe roles, loan lender tokens | Does transferring a role/token intentionally transfer authority or financial rights? |
| Off-chain/on-chain correspondence | Minswap builder, Marlowe runtime, Spectrum backend | What does completion add, and can those inputs/fees violate signed limits? |
| Liveness and failure | Cancellation/default paths, Hydra | Who can execute recovery, how is it funded, and can exit outputs settle on L1? |

All these apps use public ledger data unless a separately inspected proof system says otherwise. They do not demonstrate preserved Midnight private financial/history semantics. Existing Moriarty gross-input authority, fee-inclusive net goals and liability bounds must be added explicitly; changing state or conserving tokens is insufficient.

## Pins and source-use qualifications

The three app snapshots are Minswap `c3293ad435792ba502f5a2b1691c6736e5f167e6`, Sundae `be33466b7dbe0f8e6c0e0f46ff23737897f45835`, and Cardano-Loans `66c556bdff4cdf4e8ffb765a22cab6ecd6be8e77`. Their dates and inspected paths are recorded in the report. Old immutable contract heads and current development need different maintenance interpretations; public/nonarchived does not mean actively maintained or deployed.

Minswap root license says GPL-3.0 while its Aiken package metadata says Apache-2.0; resolve before copying. Sundae declares Apache-2.0 in package metadata without a root license file in the snapshot. Cardano-Loans has a root Apache-2.0 license. Other catalogue licenses are page/README declarations and are not complete per-file audits. Audit evidence is version-specific: a V1 audit cannot certify V2. Indigo's discovered old repository returned404 and is intentionally excluded from currently available references.

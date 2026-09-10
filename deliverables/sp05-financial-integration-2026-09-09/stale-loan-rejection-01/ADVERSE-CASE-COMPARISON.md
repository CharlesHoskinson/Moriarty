# Stale loan accrue versus stale swap close

Recommendation only, 2026-09-10. Read public source and retained public receipts/state/native bytes only. Source14 and /tmp/moriarty-adverse-source-15 were not edited. No private/wallet/services/proofs/compiler/submission access.

**Prefer stale loan accrue for the smallest actual ledger state-conflict rejection test. Do not describe it as an included fallible rollback test.** Actual native decoding gives a stronger distinction than source-level guesses.

| Candidate | Retained actual native partition | Financial dependencies | Principal uncertainty |
|---|---|---|---|
| Loan accrue from original initialized state, expectedRevision0, against settled revision2 | Actual accrued tx473b363b745c4be4864e93a04d475a6f343ff7e56a8f179d86060716ba9a5480, physical segment43718: guaranteedTranscript present; fallibleTranscript absent | No unshielded inputs, outputs, mints or claimed spends; fresh DUST only | Whether a fresh stale candidate passes ordinary proving/balancing and is rejected at which node boundary; cannot promise inclusion |
| Swap close from pre-close revision1 against closed revision2 | Actual retained close transaction, physical segment25787: guaranteedTranscript absent; fallibleTranscript present | Attempts to pay out old contract reserves1,010,000A and1,980,257B while current closed reserves are empty | Balance/UTXO validation may reject before fallible execution; actual historical partition does not prove an included rollback is obtainable |

## Actual public observations

The above partition facts were obtained by read-only Transaction.deserialize('signature','proof','binding', actualBytes) through the installed midnight-js-protocol/ledger adapter, then inspecting each ContractCall.guaranteedTranscript and fallibleTranscript. The API is documented in ledger-v8.d.ts:1885–1910. No transaction was constructed or replayed. Both transcript object types expose gas/effects/program/version. This reads actual previously submitted native bytes; it does not infer the partition from financial output locations.

Retained loan stage-accrue.json records block20377, txId001ac8252beed46b047b6032710893182db342ce85e195403517738bf47e7248e7, revision1, remaining1, principal4,500,000,000, principal due500,000,000 and interest due33,972,602. Its nativeEffects maps/lists are all empty and contractBalances is{}. Stage-settle.json records block20381, revision2, **remaining0**, principal4,500,000,000, dues0 and actual paid533,972,602; contractBalances remains{}. Residual principal is not permission for another current-state action: finite execution fuel is exhausted. These retained observations are not a new live current-state read.

The original initialized loan full state from local-recovery-03/indexed-initialize-state.bin remains the proper candidate input for stale accrue: revision0, remaining2, principal5,000,000,000. Reusing current settled state would hit local assertion/fuel/fixed financial checks and yield only pre-submit rejection. Do not alter current state or pretend the old state is fresh.

## Why accrue is the narrower candidate

Loan Compact lines71–79 check program/network/revision, initialized flag, borrower capability and actor, then execute transition0. Lines80–90 pin exact due identities/amounts and residual principal; lines91–94 update kernelState, remaining, revision and lastAccrue. No receiveUnshielded/sendUnshielded operation occurs in accrue. Those transfers occur only in settle lines128–129. The historical actual empty effect record corroborates that source property.

Swap close lines115–137 reads entry balances, matches the kernel reserves and requires exact contract surplus payouts before transfers. Using a pre-close snapshot against a now-empty contract therefore adds asset/balance failure paths unrelated to the desired optimistic-state conflict.

The loan needs no funded counterparty, new token wallet, rebuild or original-token UTXO. It still needs existing authorized borrower capability/private namespace and fresh current DUST funding through the unchanged production provider. This investigation accessed neither. Do not submit old raw accrue bytes: they contain already consumed DUST and are blocked by issued-ticket/replay protections anyway.

## Smallest preparation and evidence path

1. Reconcile exact original loan deploy/initialize/accrue/settle history and a fresh canonical finalized current loan snapshot. Bind revision2/remaining0/full state/authority/residual principal; preserve exact existing private namespace before any operational read. This is a future gate, not performed here.
2. Build one new unproven accrue via pinned createUnprovenCallTxFromInitialStates (SDK index.mjs:1330–1368), using the exact retained initialized state, private{}, existing borrower identity, original compiled build/program/network, expectedRevision0 and fresh public time. Reuse original fixed arithmetic hints for principal5e9, rate8,31/365 day calculation. The SDK must produce the new transcript normally; no tampering with signatures/proofs/claims or state hashes.
3. Inspect the actual new candidate for one exact accrue call, no financial inputs/outputs/mints, expected stale reads/writes, and actual guaranteed/fallible partition. The prior transaction is strong evidence but not a substitute for inspecting this candidate. Initial ledger parameters and time are not authority to assume valid current parameters.
4. Keep normal prove→balance→issued-ticket→submit protections (SDK25–29; providers.mjs370–399). If the ordinary proof/wallet guard rejects before submission, retain that scoped result and stop; no bypass to force node traffic.
5. Only if a bounded one-submit allocation is separately reviewed, retain exact public candidate bytes, submission outcome, actual failure classification and unchanged full contract state at canonical finalized before/after heads. Retain any DUST/fee/reservation charges. If included, classify successfulSegments and guaranteed/fallible effects; if not included, retain exact node/pool rejection instead of claiming finalized failure.

## Limits and recommendation

The native guaranteed-only accrue provides a cleaner prospective **ledger stale-state rejection / financial nonmutation** case. It does not currently provide a route to **included fallible section rollback**. The name LedgerStateConflict, its error encoding, and its location in pool validation versus application must be observed; no error code is promised here. ledger-v8.d.ts:2291–2297 documents guaranteed fee execution followed by atomic rollback of failed fallible section, while TransactionResult at2110–2133 distinguishes failure/partialSuccess/success. Those semantics do not imply every stale call reaches fallible execution.

A successful negative result would require unchanged loan serialized contract state, debt/residual/fuel/revision/authority and financial asset balances, plus separately accounted fees. It must not erase remaining principal4.5e9 or historical charges. Any SDK local assertion is a different result from node rejection. A timeout is unknown, not rejection. Neither local case closes Preview or mandatory-PCD obligations.

Do not choose stale close merely because its original call was fallible. If an included rollback predicate is mandatory, retain it as a separate open obligation until a candidate can be shown to pass guaranteed validation without adding funding/rebuild/global guard changes. No operational adverse design or resource allocation is created by this recommendation.

## Exact source/public artifact digests

- `/home/charl/Moriarty/.worktrees/sp05-deadline-review/experiments/moriarty-midnight-financial/custody/loan.compact` SHA256 `c1485f2915cedf173b858dfdbfdb9cf135915e109dea18bf00c8d237f302759e`
- `/home/charl/Moriarty/.worktrees/sp05-deadline-review/experiments/moriarty-midnight-financial/custody/swap.compact` SHA256 `29b4be0dafcb6013577f67d0446622be7cf9b8ef0bb3fc1fb81a32801a879a56`
- `/home/charl/Moriarty/.worktrees/sp05-deadline-review/deliverables/sp05-financial-integration-2026-09-09/local-continuation-02/run-public/stage-accrue.json` SHA256 `a208a4face56c59cde5ad486a972cfd206d86da6d7955349abcf3b07e11f25d8`
- `/home/charl/Moriarty/.worktrees/sp05-deadline-review/deliverables/sp05-financial-integration-2026-09-09/local-continuation-02/run-public/stage-settle.json` SHA256 `d70deb9c992c9f643e14da96d00efa2635e95f6b5e99cef9cb2ee19691cce895`
- `/home/charl/Moriarty/.worktrees/sp05-deadline-review/deliverables/sp05-financial-integration-2026-09-09/local-continuation-02/run-public/public-transactions/473b363b745c4be4864e93a04d475a6f343ff7e56a8f179d86060716ba9a5480.bin` SHA256 `473b363b745c4be4864e93a04d475a6f343ff7e56a8f179d86060716ba9a5480`
- `/home/charl/Moriarty/.worktrees/sp05-deadline-review/deliverables/sp05-financial-integration-2026-09-09/local-recovery-03/indexed-initialize-state.bin` SHA256 `1f0724d2afe55e1c2aa22580f1bf9f0eed1a30e74b7a04ff78fe622a1e6fb30f`
- `/home/charl/Moriarty/.worktrees/sp05-deadline-review/deliverables/sp05-financial-integration-2026-09-09/swap-close-state-01/close-transaction.bin` SHA256 `7a1c8e6ef185b78f3634766a1f435d39d7958fdce260af3270fbdfa56c24f0f3`
- `/home/charl/Moriarty/.worktrees/sp05-deadline-review/deliverables/sp05-financial-integration-2026-09-09/swap-close-state-01/indexed-swap-state.bin` SHA256 `269ec012749cb12101288bdf29cc17d366727b51c8e202a15ac239b1f6c22dee`
- `/home/charl/Moriarty/.worktrees/sp05-deadline-review/deliverables/sp05-financial-integration-2026-09-09/swap-close-state-01/indexed-close-state.bin` SHA256 `fcd0b1621edab64096932b9a602ba3b55c55fb6299f6ff7e5d4e434c59331cfd`
- `/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/midnight-js-contracts/dist/index.mjs` SHA256 `9c8079430513e7b459d52fc6d22ab5dede22f86073a4b50dcbdf35de99bb4072`
- `/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/ledger-v8/ledger-v8.d.ts` SHA256 `4a6eaccd531f0bb711dd8a8926d14beedd2ab8345deff378f86068608d67dc36`

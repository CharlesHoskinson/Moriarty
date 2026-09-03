---
id: marlowe.v1.baseline
type: semantics
title: Pinned Marlowe V1 baseline
status: active
updated_at: 2026-09-02T23:18:37Z
sources:
  - SRC-0002
  - SRC-0008
  - SRC-0009
  - SRC-0010
---

# Pinned Marlowe V1 baseline

The implementation baseline is `marlowe-lang/marlowe-cardano` default branch
commit `99f432d8ef9dbd1b52b7fa089254de15913b490f`. The exact V1 types and evaluator
are in `marlowe/src/Language/Marlowe/Core/V1/Semantics/Types.hs` and
`Semantics.hs`. This is an implementation observation, not a declaration that
this repository is the normative specification.

## Core algebra

`Contract` is `Close | Pay | If | When | Let | Assert`. `Action` is
`Deposit | Choice | Notify`. A `Case` is either an inline continuation or a
`MerkleizedCase` containing a continuation hash. Values cover available money,
constants, integer arithmetic, choices, interval bounds, bound values, and
conditionals. Observations cover Boolean connectives, choice existence, and
integer comparisons. Parties are Cardano addresses or role names; an account
identifier is a party. Money is Cardano multi-asset value.

The transition pipeline is:

```text
fix interval -> reduce to quiescence -> apply first matching input
             -> repeat until inputs are consumed -> transaction result
```

Case order is semantic. An earlier matching case shadows a later case. `Close`
refunds all internal accounts in the abstract semantics, but ledger transaction
limits can prevent a large abstract refund set from fitting in one transaction.

Warnings are non-positive deposit, non-positive payment, partial payment,
shadowed bound value, and failed assertion. Errors are ambiguous interval,
interval in the past, invalid interval, no matching input, useless transaction,
and continuation-hash mismatch. They must not be collapsed into one generic
failure class in Moriarty.

## Assurance boundary

The Haskell source comments record two specification drifts: refund order can
differ from Isabelle, and the Isabelle semantics do not model Merkleized
continuations. The Isabelle `BlockchainTypes.thy` also contains an unresolved
interval-endpoint TODO while Haskell treats bounds as inclusive. These are
release blockers for any claimed equivalence, not cosmetic documentation bugs.

The local `marlowe-spec-test` run passed all 59 tests on 2026-09-02. One
QuickCheck distribution warning showed only 2% non-`Close` contracts and 13%
cases with at least three inputs. The test pass is valid; it does not establish
adequate state-space coverage.

## Operational evidence

The current `caseAsDataV1Scripts` registry identifies semantics script
`377325ad84a55ba0282d844dff2d5f0f18c33fd4a28a0a9d73c6f60d` and payout
script `fcb8885eb5e4f9a5cfca3c75e8c7280e482af32dcdf2d13e47d05d27`.
Scrapling-acquired Koios evidence found 137 transactions associated with the
current semantics credential, from 2024-07-25 through 2025-03-29, and 87
unspent outputs holding 125,027,660 lovelace at observation time. An unspent
script output is not proof of a valid or active contract. The absence of a more
recent transaction in this bounded public query is weak adoption evidence, not
proof that nobody uses Marlowe.

## Moving Cardano constraints

At the 2026-09-02 observation, mainnet protocol version was 11.0, maximum
transaction size 16,384 bytes, transaction execution memory 16.5 million,
transaction steps 10 billion, fee coefficients 44 lovelace per byte plus
155,381, and `coinsPerUTxOByte` 4,310. These are moving governance-controlled
parameters. Moriarty uses them only for the Cardano migration baseline; they do
not become language constants.

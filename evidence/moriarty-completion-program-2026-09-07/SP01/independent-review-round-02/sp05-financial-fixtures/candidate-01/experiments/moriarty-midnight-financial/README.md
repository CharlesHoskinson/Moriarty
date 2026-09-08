# moriarty-midnight-financial

This package holds independent expected financial records for the accepted FIXED loan and swap sources. It also holds a full normalized financial comparison utility.

The utility compares two closed JSON financial records. It does not settle a ledger. It does not decode a chain receipt. It does not authenticate a UTXO owner.

## Scope

Use this package for SP05.1 only.

- Accepted loan source: `experiments/moriarty-language/spec/examples/loan.mori`
- Accepted swap source: `experiments/moriarty-language/spec/examples/swap.mori`
- Fixture values come from independent integer arithmetic on those sources.
- Generated Compact kernels, evaluators, and host success flags do not supply expected amounts.

Logical role IDs and asset IDs are symbolic. They are not deployed Midnight colors or addresses.

## What this package does not do

- It does not submit a transaction.
- It does not create a wallet.
- It does not run a proof.
- It does not decode a raw receipt or prove canonical finality.
- It does not bind symbolic IDs to live token colors, units, or parties.
- It does not treat a metadata success boolean as a financial match.

A later exact binding supplies actual token colors, units, and parties. Do not label an unbound synthetic comparison as network verified.

## Schema

Schema version: `moriarty-financial-record/1`.

The schema is closed. Unknown fields fail. Required dimensions must be present.

Unsigned money and counters are canonical decimal strings in `0 .. 2^128-1`. The canonical form is `0` or a digit sequence with no leading zero, sign, space, newline, or hex prefix.

Do not use IEEE Number arithmetic on money. Use integer strings and BigInt.

Indexed objects carry a unique `id`. Comparison matches complete identity. Duplicate identities fail. Sort order does not hide duplicates.

Each stage record includes:

- role, asset, denomination, quantum, and color bindings
- pre and post balances
- ordered transfers with `from`, `to`, `color`, `unit`, `amount`
- per-actor per-asset `grossDebit`, `refund`, `netCredit`, and `fee`
- nominal liabilities, debt IDs, and principal/accrual/due allocation
- stage, revision, remaining, work, and status
- residual duties

Refund does not erase gross debit. Fee is not dropped from a net goal. Current due is not remaining agreement debt.

Every synthetic observation used by tests is labeled `synthetic-local`. These records do not contain fabricated transaction IDs or ledger results.

## compareFinancialEffects(expected, observed)

Export from `src/differential.mjs`.

Return value:

```
{
  ok: boolean,
  errors: [{ code, path, message }, ...],
  networkAcceptance: false,
  networkEvidence: "incompleteNetworkEvidence"
}
```

`ok` is true only when both records are well-formed, internally consistent, and financially equal.

The function recomputes transfer/balance/gross/net consistency on both inputs. Two equal self-contradictory objects still fail.

Inputs are not mutated on success or failure.

Error order is deterministic: `path` ascending, then `code` ascending.

Stable error codes include `MALFORMED_STRUCTURE`, `UNKNOWN_FIELD`, `MISSING_FIELD`, `NONCANONICAL_INTEGER`, `INTEGER_OVERFLOW`, `DUPLICATE_IDENTITY`, `BALANCE_INCONSISTENT`, `GROSS_DEBIT_MISMATCH`, `REFUND_MISMATCH`, `NET_CREDIT_MISMATCH`, `FEE_MISMATCH`, `TRANSFER_MISMATCH`, `EXTRA_TRANSFER`, `OMITTED_TRANSFER`, `COLOR_MISMATCH`, `QUANTUM_MISMATCH`, `DENOMINATION_MISMATCH`, `LIABILITY_MISMATCH`, `DUE_MISMATCH`, `RESIDUAL_DUTY_MISMATCH`, `REVISION_MISMATCH`, `WORK_MISMATCH`, `REMAINING_MISMATCH`, `STATUS_MISMATCH`, `BINDING_MISMATCH`, `VALUE_MISMATCH`, `NETWORK_CLAIM_FORBIDDEN`.

## Network acceptance

Network transaction and proof fees are separate from the swap economic fee of 30 ASSET_A.

These fixtures do not invent a Midnight fee asset or unit. They do not assume those fees are zero. Network fee accounting is unresolved.

The utility may prove fixed economic comparison only. It always returns `networkAcceptance: false` and `networkEvidence: "incompleteNetworkEvidence"`, even when `ok` is true.

No caller field can make this utility claim on-chain authentication. A record that sets `networkAcceptance` to true is rejected.

## Loan fixture

Nominal initial principal is 5000000000 USD_micro. Rate is 8/100. Period is 31/365.

Independent interest is `floor(5000000000 * 8 * 31 / (100 * 365)) = 33972602`. The exact remainder 54/73 micro-USD is retained and discarded for this sample only.

Principal installment is 500000000. Total cash is 533972602.

Before settle, borrower cash is 20000000000 and lender cash is 0. After settle, borrower cash is 19466027398 and lender cash is 533972602.

Asset is symbolic `USD_TEST_ASSET`. Denomination is `USD_micro`. Quantum is 1.

Accrue creates distinct dues `lam01:period1:PR` and `lam01:period1:IP`. Settle clears those dues. Paid principal is 500000000. Paid interest is 33972602. Notional 4500000000 remains. Cursor is 2. Episode closed is 1. Lifetime is 2. Revision is 2. Remaining is 0.

Remaining agreement debt is not zero. Remaining agreement debt is not current due.

Initial borrower cash is test-setup mint funding. It is not evidence of loan origination.

## Swap fixture

Post-setup baseline: pool 1000000 A / 2000000 B, trader 100000 A / 0 B, provider 0 / 0.

Input is 10000 A. Economic fee multiplier is 997/1000. 30 A stays in the pool as the swap economic fee.

Independent output is `floor((10000 * 997) * 2000000 / (1000000 * 1000 + 10000 * 997)) = 19743` B.

After trade: pool 1010000 A / 1980257 B, trader 90000 A / 19743 B.

Close transfers those remaining reserves to the provider once. Pool becomes 0 / 0. Provider becomes 1010000 / 1980257. Epoch closed is 1.

Lifetime is 8. Revision and remaining update after each action. The close reservation is counted once.

Pool setup funding 1000000 A / 2000000 B is an explicit required setup record. It is not user funding proof.

## Run tests

From this directory:

```
npm test
```

That script runs `node --test tests/*.test.mjs`. There is no build step and no dependency.

## Deployment binding

Status is unresolved. A later task supplies actual colors, units, and parties. Raw receipt, finality, and payer validation remain a later task.

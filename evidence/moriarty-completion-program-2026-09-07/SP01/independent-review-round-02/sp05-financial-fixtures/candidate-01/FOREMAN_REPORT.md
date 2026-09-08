# SP05.1 financial fixtures and comparison utility

Worker: Grok 4.6 high in worktree `sp05-financial-fixtures-grok`.

This task created independent loan and swap financial records and a closed-schema comparison utility. It did not settle a ledger, submit a transaction, create a wallet, or run a proof.

## Result

Tests were written first. `npm test` with `src/differential.mjs` absent exited 1. Node reported `ERR_MODULE_NOT_FOUND`. Counts were tests 1, pass 0, fail 1.

After implementation, `npm test` exited 0. Counts were tests 29, pass 29, fail 0.

Independent BigInt arithmetic in the test file matches the fixtures:

- loan interest `floor(5000000000 * 8 * 31 / (100 * 365)) = 33972602`
- remainder 54/73 micro-USD
- settle cash 533972602
- borrower after 19466027398
- remaining notional 4500000000
- swap output `floor((10000 * 997) * 2000000 / (1000000 * 1000 + 10000 * 997)) = 19743`
- economic fee 30 ASSET_A
- no-fee output 19801 is larger, so the fee effect is real

`compareFinancialEffects` returns `networkAcceptance: false` and `networkEvidence: "incompleteNetworkEvidence"` even when `ok` is true. A caller field cannot authenticate a chain result.

## Source pins, untouched

These input hashes match the binding and were not written by this task:

| Path | SHA-256 |
| --- | --- |
| experiments/moriarty-language/spec/examples/loan.mori | 1e1e61158ef80d44aa326399731440971fe50de7147ae5fb04e3fb36c48fef49 |
| experiments/moriarty-language/spec/examples/swap.mori | 0c2217365f2e518ec70835cf05a09150253d2df334e7dcf0504fd8c8d887051e |
| experiments/moriarty-language/compact/generated/loan/metadata.json | 9e13a2677797e6328ad6b7b8d1a34a447d34a9ed9828e1c80867618040648ab5 |
| experiments/moriarty-language/compact/generated/swap/metadata.json | e0b96181aad293cac32668cfaa3152c5c7cf38dca55e5f567616d17511fe7a9e |

Metadata was inspected. It was not used to derive expected amounts.

## Owned file hashes

| Path | SHA-256 |
| --- | --- |
| experiments/moriarty-midnight-financial/package.json | fbb2a657c2b7c8ef9c5242ffc430257f85d639a5bf30f82234dc5e23b14ad367 |
| experiments/moriarty-midnight-financial/README.md | 429987a2f936e0b2ce20b68556a19ad79df93a36c86874b3f3f647f52414ef39 |
| experiments/moriarty-midnight-financial/fixtures/loan.json | cfb274a1062c75a1b558b04daad70c0c45adc7245810b3375025b420c28abbd2 |
| experiments/moriarty-midnight-financial/fixtures/swap.json | d7d96e6ffa3bac967d7f8bdd4823c9ac894c5cb93fc9168d2cdd0e15bc117374 |
| experiments/moriarty-midnight-financial/src/differential.mjs | fcc77f1368528ec71fb763cef9634a86ef330ad5daa8f1b545e44725e25676a4 |
| experiments/moriarty-midnight-financial/tests/differential.test.mjs | 24e5fa1d3232c66fce11832bd02499e211ddd57e86b8db0ab5c5fff2516686cf |

`test-evidence.json` and these reports are additional owned files. Hash them after write.

## Commands

RED: `cd experiments/moriarty-midnight-financial && npm test`

RED output SHA-256: `b21afcfc74d208ac925bdd4f123f2d93a00b26b279d61717e9e414a2c46d5902`

GREEN: `cd experiments/moriarty-midnight-financial && npm test`

GREEN output SHA-256: `f8d82d1794450dadcb83df675801579cb6db5e4b6d6c636b35fe29d3e35236e1`

Exact counts and output text are in `experiments/moriarty-midnight-financial/test-evidence.json`. Tests were run. This report does not claim unrun tests.

## Field map

| Field | Check |
| --- | --- |
| schemaVersion, observationKind | closed schema, synthetic-local only |
| networkAcceptance, networkEvidence | must be false / incompleteNetworkEvidence. True is NETWORK_CLAIM_FORBIDDEN |
| deploymentBinding | unresolved status, symbolic roles, quantum, denomination, color |
| networkFeeAccounting | unresolved, fee asset/unit null, not assumed zero |
| agreement lifetime/horizon | canonical u128, loan 2, swap 8 |
| stage revision/remaining/work/status | exact match, work refresh fails |
| bindings | role set, asset identity, quantum, denomination, color |
| balances pre/post | unique actor\|asset\|unit identity, amount |
| transfers | unique id, ordinal, from, to, color, unit, amount, extra/omitted |
| actorEffects | grossDebit equals outgoing, netCredit equals incoming, refund does not erase gross, fee <= gross |
| economicFee | 0 on loan, 30 ASSET_A on swap trade, separate from network fees |
| liabilities | remaining notional, principal paid/due, accrual, due IDs PR/IP |
| residualDuties | remaining 4500000000 after settle, close reserve counted once |
| integers | canonical decimal strings 0..2^128-1, reject overflow, newline, leading zeros |
| conservation | recomputed on expected and observed from transfers and balances |

## Controls that failed comparison

Wrong payee, color, quantum, denomination, omitted transfer, extra output, duplicate identity, missing remaining 4500000000 debt, wrong due allocation, interest off-by-one, floor remainder/dust, swapped asset reserve, omitted swap fee, swap fee credited twice, close reserve shortfall, wrong revision, work refresh, gross/refund/net substitution, net-after-fee shortfall, unknown field, overflow, newline numeric, malformed structure, mutated input, non-deterministic error order, and self-equal inconsistent records.

## Limitations

Logical IDs `USD_TEST_ASSET`, `ASSET_A`, `ASSET_B`, `borrower`, `lender`, `trader`, `pool`, and `provider` are symbolic. They are not deployed colors or addresses.

Initial loan cash is synthetic-local mint funding. It is not origination evidence.

Pool setup funding is an explicit required setup record. It is not user funding proof.

Network transaction and proof fees are unresolved. This utility does not invent a Midnight fee asset. It does not assume those fees are zero.

Raw receipt decoding, canonical finality, and UTXO owner authentication remain a later task.

No commit was made. Main and other worktrees were not modified.

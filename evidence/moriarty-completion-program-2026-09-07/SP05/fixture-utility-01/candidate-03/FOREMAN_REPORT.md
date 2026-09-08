# SP05.1 financial fixture utility correction

Worker: Grok 4.6 high in worktree `sp05-financial-fixtures-grok`.

This correction repairs the frozen candidate-01 comparison utility. GPT-6 Astra blocked that candidate. This task did not settle a ledger, submit a transaction, create a wallet, or run a proof.

Independent GPT-6 Astra review of this correction is still required. This packet is not full SP05 I2 acceptance.

## Result

New regression tests were written first against the uncorrected implementation.

RED command:

```
node --test --test-reporter=tap tests/differential.test.mjs
```

Working directory: `experiments/moriarty-midnight-financial`. Exit 1. Counts: tests 45, pass 29, fail 16. The original 29 controls passed. The 16 new controls failed.

RED stdout SHA-256: `88d72f59836773cbefa50197158ecc174db88760790e7c2c88861fd2fa6127d7`

RED stderr SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`

After the utility correction, the same command exited 0. Counts: tests 45, pass 45, fail 0.

GREEN stdout SHA-256: `7ecadc5351e5137c803cebc80420a1a5095b4bb9047249fe9cf73e738a848881`

GREEN stderr SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`

Exact argv, UTF-8 stdout/stderr strings, exits, and counts are in `experiments/moriarty-midnight-financial/test-evidence.json`. Those SHA-256 values were derived from the captured bytes. They were checked again after the file write.

`compareFinancialEffects` still returns `networkAcceptance: false` and `networkEvidence: "incompleteNetworkEvidence"` when `ok` is true.

## Findings addressed

R1. A changed `deploymentBinding.roles.lender.logicalId` no longer compares as equal. The utility compares every closed role-map field, including `logicalId` and `status`. It binds source pins to the admitted case.

R2. Nested missing `economicFee`, a null stage, and a null role map return structured errors. The utility does not throw. It does not mutate inputs. Unknown-field errors remain. Diagnostics stay independent and ordered.

R3. Equal contradictory records fail internal case validation. The utility recomputes loan interest 33972602, cash 533972602, remaining 4500000000, swap fee 30, output 19743, and the close provider reserves. It checks due IDs, residual duties, fee reconciliation, work, revision, remaining, and stage continuity.

R4. Missing balances or actor effects cannot bypass replay. Duplicate transfer ordinals, duplicate role names, and quantum 0 fail. Setup-mint remains limited to declared setup transfers.

R5. Candidate-01 stored output strings do not hash to the hashes they declared. Those strings are preserved as an unverified corrected-record interpretation. This correction does not invent another unseen original capture.

## Source pins, untouched

These input hashes match the binding and were not written by this task:

| Path | SHA-256 |
| --- | --- |
| experiments/moriarty-language/spec/examples/loan.mori | 1e1e61158ef80d44aa326399731440971fe50de7147ae5fb04e3fb36c48fef49 |
| experiments/moriarty-language/spec/examples/swap.mori | 0c2217365f2e518ec70835cf05a09150253d2df334e7dcf0504fd8c8d887051e |
| experiments/moriarty-language/compact/generated/loan/metadata.json | 9e13a2677797e6328ad6b7b8d1a34a447d34a9ed9828e1c80867618040648ab5 |
| experiments/moriarty-language/compact/generated/swap/metadata.json | e0b96181aad293cac32668cfaa3152c5c7cf38dca55e5f567616d17511fe7a9e |

Fixture loan and swap JSON hashes are unchanged from candidate-01.

Metadata was inspected. It was not used to derive expected amounts.

## Owned file hashes

Hash the reports after this write. Implementation hashes at report draft time:

| Path | SHA-256 |
| --- | --- |
| experiments/moriarty-midnight-financial/package.json | 0ff5360e6efef04719bba8dbd8c19412f9aab1cc4dc3fde0ffa0f4de48edfd00 |
| experiments/moriarty-midnight-financial/README.md | 3661aed603d48587289d2efdafeb15b892589e9b121120feae630b5ab9c0b757 |
| experiments/moriarty-midnight-financial/fixtures/loan.json | cfb274a1062c75a1b558b04daad70c0c45adc7245810b3375025b420c28abbd2 |
| experiments/moriarty-midnight-financial/fixtures/swap.json | d7d96e6ffa3bac967d7f8bdd4823c9ac894c5cb93fc9168d2cdd0e15bc117374 |
| experiments/moriarty-midnight-financial/src/differential.mjs | aa6351df2a2644fcf463a2897261c36fb386d9ebcb718efae0871716da19c0a4 |
| experiments/moriarty-midnight-financial/tests/differential.test.mjs | e851aeb58126d22d114b3e27206ad381bc7c4a9cf3fdb5782c5b53585a72619d |

## Limitations

Logical IDs `USD_TEST_ASSET`, `ASSET_A`, `ASSET_B`, `borrower`, `lender`, `trader`, `pool`, and `provider` are symbolic. They are not deployed colors or addresses.

Initial loan cash is synthetic-local mint funding. It is not origination evidence.

Pool setup funding is an explicit required setup record. It is not user funding proof.

Network transaction and proof fees are unresolved. This utility does not invent a Midnight fee asset. It does not assume those fees are zero.

Raw receipt decoding, canonical finality, and UTXO owner authentication remain a later task.

No commit was made. Main and other worktrees were not modified. Frozen candidate-01 was not modified.

`okFinancialComparisonUtility` remains false until independent GPT-6 Astra review.

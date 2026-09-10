# Pure financial expression implementation candidate 01

This candidate implements the separately versioned `moriarty-financial-expression-contract/1` API. `createFinancialExpressionContractV1(schemaCanonicalJSON).evaluate(requestCanonicalJSON)` executes the original forty constructors and the eight approved additions. It preserves the original result records and canonical request format except for the explicit contract identifier. Original forty-profile source and tests remain byte-identical. No old profile is widened.

The added constructors are ConstructShares, ConstructVariant, ProjectVariant, ProjectSome, ConvertUInt, ScalarValue, ConstructAmount, and Select. The new profile admits UInt256, AmountProduct, ScaledAmount, SignedScaledAmount, SignedAmount and NetAmount. Arithmetic checks each intermediate range, preserves dimensional identities, and requires a literal power-of-ten divisor for dimensional scale removal. NetAmount uses the exact symmetric UInt128 difference range, not SInt128. Select type-checks both arms and evaluates only the selected arm; inherited And/Or behavior remains unchanged.

The old runtime exposes no extension seam. Two new versioned modules therefore contain an explicit checked-machine/type-system adaptation and reuse the unchanged canonical wire module. There are no host callbacks or caller-provided semantics. This is not a source-language frontend; syntax for the added constructors remains separate work.

## Exact schema-bound interpretation

Copied `inputs/financial-schema.md:111–120` explicitly limits named record/enum/variant types to 256 while retaining simultaneous 65,536-byte, 4,096-JSON-node and depth-64 whole-schema bounds. This differs materially from the old profile's 256 total declarations. The new profile follows the financial clause; the old profile stays unchanged. Every roster and declaration map—units, assets, vaults, parties, records, enums, variants, fields, arguments, observations and operations—is validated and counted by the global bounds. `verification-01.json` records the exact full reference counts: 221 named types. All reference operations and records remain present when evaluating the vault rule; this admits their types, not their financial transition semantics.

Variant tags and payload types must both be distinct within a family. Distinct payload types follow the retained financial source checker; the prose alone emphasizes distinct tags. This interpretation is explicit for independent review. The first root probe incorrectly expected dynamic rejection for duplicate payload types; its original failed fixture/report remain retained. The corrected twenty root boundary probes pass and their original scripts retain absolute author-checkout imports.

## Evidence and limits

`green-final-01.tap` records 324 passing tests: 205 unchanged old-profile tests and 119 new-profile tests, with no skips. The new profile repeats all forty positive/rejection pairs, executes the unchanged 56-node VaultConversion body against twenty independent arithmetic cases, and checks widths, bounds, units, variants, options, static failures, work and source spans. `red-extensions-01.tap` preserves fail-first evidence. TypeScript strict checking passes (`typecheck-01.txt`).

The initialized vault subrule requires positive supply and valuation. Two explicit Require expressions precede the unchanged body; their guards plus the enclosing Let consume nine additional entered nodes. The unguarded body does not independently enforce that domain, and a test preserves that distinction. Empty financial collections supplied to the pure rule are typed source fixtures, not validated financial states. No full38 transition, signing, cryptography, ledger, Midnight, financial, or deployment acceptance follows.

Both retained design receipts approve the eight pure choices; this does not approve the defective full financial03 draft. Fresh independent implementation reviews must assess these new bytes, including the bound interpretation and reference Sigma admission. No commit or operation has been performed.

Run from the worktree root:

```sh
node --test experiments/moriarty-language/tests/expression-v1*.test.mjs experiments/moriarty-language/tests/financial-expression-v1*.test.mjs
tsc --noEmit --strict --target ES2022 --module NodeNext --moduleResolution NodeNext --allowImportingTsExtensions --lib ES2022,DOM experiments/moriarty-language/src/successor/financial-expression-v1.ts experiments/moriarty-language/src/successor/financial-expression-types-v1.ts
```

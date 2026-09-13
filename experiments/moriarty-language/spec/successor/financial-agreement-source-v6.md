# Financial agreement source /6

Repository observation: `/6` adds a protected `Fee` operation and requires lifecycle state `/2` for evaluation. Older source, Core, state factories, and examples retain their exact frozen bytes.

`createFinancialAgreementSourceV6()` exposes `check`, `elaborate`, and `evaluate`. The frontend provides named parse and format exports. Evaluation accepts source text, an action name, canonical ordinary snapshots, and financial state JSON. It derives Core from source each time.

The grammar remains `/5` with the exact literal header `profile "moriarty-financial-agreement-source/6";`. The `/6` frontend rewrites only that token in its private parser input. All source byte spans remain unchanged. Escaped profile spellings are not admitted by this entry.

The five protected operations are Transfer, Repay, Originate, Accrue, and Fee. Fee uses the exact Transfer field schema: id, from, to, settlementAsset, and transferAmount. Core `/5` represents it as `Emit` with operation `Fee`. No additional expression constructor is needed. The version changes because funded operation semantics change.

Fee debits balance, gross allowance, and one kernel work unit. It produces a distinct Fee effect. Its ID enters the existing source transfer-ID namespace for uniqueness. It never creates transfer credit for Repay or Originate. Zero and self Fees are permitted. They still require a gross cap. A self Fee preserves balance while consuming gross allowance. Repay and Originate still require a Transfer from the same batch.

Lifecycle state `/2` adds required `authority` alongside the nine existing fields. It contains networkTag, programDigest, debtor, lender, and outcomeIntent. Each role contains party and capability. Commitments are 32-byte lowercase hexadecimal strings. Roles must have distinct parties and commitments. Retained obligations and new originations must match those roles. Secrets never enter source state.

`outcomeIntent` contains sorted, unique `grossCaps` and `minimumNetCredits` arrays. Rows use the frozen actor/asset and maximumLedgerAmount/minimumLedgerAmount fields. Amounts are UInt128 decimal ledger quanta. This version limits authorized actors to the debtor and lender. This is a source binding, not proof of consent.

A net goal passes exactly when `checked(credits) >= checked(debits + minimumLedgerAmount)`. Transfer and Fee contribute per asset. Self effects contribute to both sides. Nonmonetary effects contribute to neither. Missing sums are zero. Every outgoing monetary effect requires a cap, including zero. Refunds never reduce gross debits. Overflow rejects before publication. The public evaluator invokes this check inside funded preparation, before publishing any ordinary or financial state.

Amounts already denote exact ledger quanta. The SDK binding supports the selected Cash quantum 1 only. Its exact conversion helper rejects nondivisible conversions and bounded intermediate overflow. `fees.netGoal.assetMixing` stays open. In-asset economic fees enter goals. Native DUST/SPECK has no Cash conversion and no invented cap.

Repository observation: the private prefix/kernel/suffix staging is retained in new versioned runtime modules. The legacy runtime files are frozen by file digest. This requires code duplication across the version boundary. Future bug fixes need review in both versions.

`custody/generate-lifecycle.mjs` emits the separate Compact artifact. The retained full-build receipt also hashes `custody/generate.mjs`, so that generator remains unchanged. The new artifact binds only the selected source's authority and fee disbursement. It is not a complete financial lifecycle compiler. It does not store obligations, allowances, or used identifiers. The SDK caller compares supplied accepted-read bindings and prepares `createUnprovenCallTx` arguments. Authenticity of that read, compiled-contract provenance, native payer attribution, full state correspondence, and live acceptance remain separate obligations. No compiled or network result is claimed.

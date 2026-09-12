# Financial state reads and current README semantics

Implemented typed reads of outstanding debt, principal, accrued debt, balances and allowances from a fully validated local financial projection. Agreement source `/3` and expression Core `/2` preserve the existing profile contracts. The `repay_remaining` action computes its payment from financial state.

The executable example pays 30, then 20, then the remaining 50: debt 100 → 70 → 50 → 0. It preserves unrelated financial rows and all transfer/allocation identifiers, finishes with balances 0/100, allowance remaining/spent 0/100, ordinary paid 100, and work remaining/spent/reserve 115/141/16. Expression costs are 43, 45 and 47, plus two kernel operations per payment. A subsequent payment fails its settled-debt guard atomically.

Root and language READMEs now document current source-defined schemas, multiple named actions, funded execution and typed financial reads. Root README includes the canonical current grammar, result types, typing and reduction rules, immutable financial pre-state, exact work composition and tested CLI/demo commands.

Grok 4.6 high authored the implementation and repair; returned model was `grok-4.6-build`. Fresh GPT-6 Astra medium approved the full exact candidate without substantive findings. All 178 manifest hashes matched before and after audit. See `audit-01/review.json`.

Validation: 794 package tests and TypeScript typecheck pass. Root independent checks passed 52 API/CLI probes, 37 Core probes, 174 prior-profile comparisons and four actual README commands. Astra independently passed 127 probes, 112 prior-profile comparisons and a three-payment CLI continuation with failure/retry checks. After integration, the main workspace passed all 52 API/CLI probes and all four README commands, and its grammar still matches the canonical file.

Root findings were repaired before final audit: Core state-admission failures now conform to their declared rejection envelope; grammar and parser agree on generic argument parsing with static arity checks; README explicitly states financial-read types and rules. Original failed experiments and corrected test oracles remain in the evidence history.

The reviewed source and README were applied to the main workspace after checking all 165 baseline files. All 178 candidate files match, and 52 unrelated modified tracked files retain their original hashes. See `integration.json`. The feature branch is based on reviewed multiple-actions commit `96860344978e176236a6f152cd694433263c8d5f`; publication is a draft PR stacked on `feat/multiple-named-actions`.

This result establishes local language execution against a validated projection. Ledger authentication, K correspondence, native proofs and Preview settlement remain separate acceptance work.

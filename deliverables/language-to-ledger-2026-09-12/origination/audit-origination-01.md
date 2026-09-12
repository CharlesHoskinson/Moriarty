# Independent exact-candidate Task2 audit

**APPROVED** for the bounded local origination/accrual implementation. No blocking findings remain on these frozen bytes.

Requested reviewer: GPT-6 Astra, medium effort. Actual visible identity: fresh host sub-agent `/root/origination_final_audit`, requested as GPT-6 Astra medium. No independently returned endpoint model telemetry is available; this receipt does not convert the host request into a provider-returned identity claim. This reviewer neither authored nor previously reviewed this candidate. The author's recorded returned identity is `grok-4.6-build`; that is separate author evidence.

## Exact scope and integrity

Worktree: `/home/charl/Moriarty/.worktrees/loan-origination-accrual`.
Planning HEAD: `a7beda220ea727dcd8b296f8fd72c48460e1500d`.
Task1 baseline: `53f9b78981031f8c12759b422416eef78adc51fd`.
Manifest: `candidate-01.json`, **215 files**, digest **`2a3a69e6170e000f3b9c85991868bcfc1ed080266f96434569163ca8ddf47a61`**.
Canonical contract: `openspec/changes/language-to-ledger-lifecycle/loan-origination-accrual-contract.md`, SHA256 `74299d10cca2874828c4c847f6817677b033090bb85d38bb898dd462fee98383`.

Every manifest file matched before and after review. Independently recomputed the manifest digest from the sorted compact JSON path/hash map. All 13 output hashes in `root-final-checks-01.json` also match their files. No candidate source was changed. Audit artifacts are separate files in the main evidence directory.

Loaded the installed Moriarty develop skill, read checkout instructions and routing, and refreshed checkout CLI status. Status has no pending transaction notifications; dependent dispatch remains blocked by existing binding/accounting/resource evidence. These blocks do not prevent this read-only review. Latest task routing supersedes older recovered Opus assignments.

The review covers the integrated source/5 parser and formatter, source compiler and structural protected-operation binding, Core/4 machine and private staging, lifecycle input/state admission and all four action applications, output work/size/ownership, CLI consumer, docs/examples, public TypeScript contracts and regression coverage. This is a full candidate review through its actual production paths, not a diff-only approval. Byte verification of all 215 files does not mean every unchanged legacy fixture received a line-by-line review. Unrelated plugin/AGENTS edits outside this manifest are excluded.

## Substantive conclusions

The closed lifecycle schemas enforce canonical signed/unsigned domains, one-to-one origination history, unique funding provenance, retained settled obligations, the checked next-boundary equation and cursor/history count consistency. Origination uses only an executed transfer from the current candidate, checks roles and asset, rejects any prior allocation, requires exact whole-transfer conversion, and consumes that witness once. Repayment preserves initial principal, incurred liability, cap, terms and cursor; paying principal does not restore lifetime cap capacity.

Accrual uses current principal and checked UInt128 multiplication before division, including floor/ceil and zero interest. Period sequence, time eligibility, arithmetic/boundary overflow, nominal range, lifetime cap and capacity follow the frozen priority. Cash and gross allowances do not change during accrual. Fresh IDs cannot bypass the cursor; zero interest still consumes work and history. Full-history arithmetic overflow precedes capacity on an independently admitted fixture.

The source compiler structurally binds all four operations while allowing each invocation to emit a nonempty subset. Nominal Quantity values are kept distinct from cash Amount and UInt64 periods. Originate units and Repay/Accrue target denomination are checked. Financial PRE remains fixed; a same-call new debt is visible to POST only after kernel success. Prefix/kernel/suffix execution charges the same work pool and keeps reserve separate. Failed suffixes publish no financial or ordinary candidate. Compact financial state is checked after kernel debit and again after suffix debit, including the exact 65536/65537-byte boundary.

Public entry points accept primitive text and publish owned results. No public continuation can inject a financial POST. Independent mutation checks confirmed that nested Origination effect terms/conversion do not alias financialPost or a later evaluation. Old source/Core version gates and repayment/0 remain closed; repayment.ts has no baseline diff.

## Reproduced checks

Commands, exit codes and output digests are in `audit-origination-01-checks.json`.

- Full package tests: **874 passed**, zero failures/skips; package typecheck passed.
- Independent preimplementation oracle probes: **37 kernel cases** and **13 source cases** passed. These compare complete states/effects, rollback, arithmetic/error priorities, exact work, and result-size bounds.
- Six new reviewer probes passed: same-call Originate+Accrue costs62; fixed PRE lookup failure; zero-interest source event costs21 and rejects replay; negative nominal cap rejects; nested effect/post ownership; overflow before full history; primitive-object coercion is not invoked. Some tests contain multiple assertions/cases.
- Complete TypeScript equality checks passed for ten legacy exported types/factories, including entire factory signatures, not just evaluate results. Source/5 keeps its separate effect set. Evidence: `audit-origination-01-full-api.mts` and its compiler output.
- README canonical /5 EBNF matched; all four actual documented check/format/simulate/demo commands succeeded.
- Supplemental root receipts retain old source17/Core25/legacy261, continuation and rejection ownership, unchanged exhaustive client, and diff checks. Their output hashes were verified; these are distinguished from this reviewer's reruns.

An initial inspection command mistakenly requested the nonexistent `examples/financial-lifecycle.mjs`; the actual package command targets `examples/financial-lifecycle-payment.mjs`, which was inspected and executed successfully. This was an inspection-path typo, not a candidate failure.

## Retained findings and limits

**R1 resolved:** the initial widening of legacy funded result effects is repaired with separate lifecycle types and precise overloads. The unchanged exhaustive client passes; the same incomplete effect switch correctly fails for source/5. Full factory/type equivalence independently passed.

**R2 resolved:** root README now contains real /5 commands and the canonical grammar. Their actual executions passed. Earlier author/root findings and receipts remain untouched.

This approval establishes local Task2 behavior only. The current demo intentionally stops after partial repayment; a fully chained ordinary/financial settlement is Task3. Supplied cap, state and observedTime remain untrusted local projection data. No K correspondence, proof, Docker, Preview, authenticated consent/time/head or aggregate SDK liability acceptance is established. No provider calls, subagents, campaigns or network transactions were performed by this reviewer. Any candidate byte change invalidates this audit.

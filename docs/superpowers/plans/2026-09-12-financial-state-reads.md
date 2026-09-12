# Kernel-backed financial reads implementation plan

> Execute this approved capability through executing-plans. Grok4.6 high authors; fresh GPT-6 Astra medium independently audits the complete exact final candidate. The user approved beginning and updating README syntax and semantics; no routine approval pause is needed.

**Goal:** Let source actions compute payments from fully validated financial pre-state and document the implemented syntax and semantics accurately.
**Architecture:** New agreement-source/3 and financial Core contract/2 entries share the current compiler/evaluator behind strict profile gates. Dedicated financial read constructors consume an owned validated read-only pre-state; ordinary Pre/Obs cannot supply financial values. Existing repayment preparation remains the sole Transfer/Repay execution path.
**Tech stack:** Existing TypeScript/Node.js, node:test, EBNF and Markdown. No new dependencies.

## Authority and baseline

User approved typed reads of obligations, balances and allowances, repay_remaining, and the same Grok/Astra workflow. The existing native goal owns autonomous continuation.
Base commit:96860344978e176236a6f152cd694433263c8d5f.
Worktree:/home/charl/Moriarty/.worktrees/financial-state-reads.
Branch:feat/financial-state-reads.
Evidence:/home/charl/Moriarty/deliverables/financial-state-reads-2026-09-12.
Main README and all164 language files match this baseline; baseline-files.json records165 hashes.
Copied AGENTS/ROADMAP/plugin/routing files are current instructions only and must not be committed.
Scope excludes new financial write operations, state initialization, action calls, batch invocations, financial-post ensures, native/K/proofs/Preview work, infrastructure, plugins and unrelated roadmap edits.
Read-only state in this local API is a validated supplied projection, not authenticated ledger state. Preserve that distinction.

## Design and alternatives

Root and the independent expectations reviewer agree on dynamic Text identities, generic declared units/assets, dedicated read constructors and separate immutable financial context. This avoids extra declaration grammar while retaining typed reads.
Do not project kernel values into ordinary Pre/Obs: those are caller inputs and would blur provenance. Do not precompute reads into literals or rewrite source: that would break runtime short-circuiting, spans and work accounting. Do not duplicate the complete interpreter.
The source profile is `moriarty-financial-agreement-source/3`. Its Core contract is `moriarty-financial-expression-contract/2` because the previous exact constructor set must remain closed. Share implementation behind explicit gates. Previous source profiles and Core /1 retain API arity, envelopes, reserved names, rejection priorities and constructor rejection.

## Source and public API contract

Create `src/successor/financial-agreement-source-v3.ts` with frozen `createFinancialAgreementSourceV3()`:
```
elaborate(source: string)
check(source: string)
evaluate(source: string, actionName: string, snapshotsCanonicalJSON: string, repaymentStateJSON: string)
```
Return the /2 source shapes with sourceProfile/3 and contract/2 as appropriate, retaining declaration-order actions and independent schemas/cores/bounds. No aggregate runtime bound. Evaluate returns unchanged FundedExpressionResult. No caller Core, parsed state object, financial callback or mutable returned artifact is accepted by the source boundary.
Expose a matching frontend module with FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE, parseFinancialAgreementSourceV3 and formatFinancialAgreementSourceV3.
All existing /2 declarations/actions and ordinary expressions work in /3, with only the new profile's reserved names added. New intrinsics are unavailable through older profile entries, and do not retroactively reserve old legal declaration/local/parameter names.

Six new generic intrinsics each accept one Text expression, evaluated exactly once:

| Source | Result | Lookup |
| --- | --- | --- |
| outstanding<Cash>(obligationId) | Quantity<Units<Cash,1>,0> | obligation.outstanding; Cash is a declared unit |
| principal<Cash>(obligationId) | Quantity<Units<Cash,1>,0> | obligation.principal; declared unit |
| accrued<Cash>(obligationId) | Quantity<Units<Cash,1>,0> | obligation.accrued; declared unit |
| balance<Cash>(partyId) | Amount<Cash> | balance amount for exact (party,Cash); declared asset |
| allowance_remaining<Cash>(partyId) | Amount<Cash> | allowance remaining for exact (party,Cash); declared asset |
| allowance_spent<Cash>(partyId) | Amount<Cash> | allowance spent for exact (party,Cash); declared asset |

Require exactly one simple generic symbol and one argument. Wrong argument type rejects TYPE_MISMATCH statically, unknown unit/asset TYPE_NAME. Use original UTF-8 call/argument spans. Choose dedicated explicit read Core constructors (one common constructor with closed kind/metric operands or a small closed constructor family); document their exact shape and type/reduction rule. General callbacks and arbitrary property paths are not a constructor contract.
At reduction, identity must match the kernel ASCII identifier rule (primitive Text already checked): invalid -> INVALID_IDENTIFIER; missing -> MISSING_OBLIGATION/MISSING_BALANCE/MISSING_ALLOWANCE. Never default a missing entry to zero. Obligation denomination different from the generic unit -> NOMINAL_UNIT. Quantity read above signed128 maximum -> ARITH_RANGE, without truncation; full UInt128 Amount values remain representable. Settled obligations are valid and have zero outstanding.
No reads of financial post-state. All reads in an action, including ensures, see the same financial PRE projection; emitted descriptors are staged and do not mutate it. Ordinary post semantics stay unchanged and must be documented clearly.

## Validation, ownership and work

Evaluation order:
1. Primitive bounded source, parse, shared declarations/protected binding, all actions statically checked (existing /2 order).
2. Mandatory primitive exact action selector (same /2 errors and synthetic external spans).
3. Bounded primitive repayment-state JSON, full kernel state admission using the existing parseState rules, including unrelated entries, closed keys, uniqueness, arithmetic relationships, conversion/status and work invariants.
4. Work equality and selected ordinary snapshot admission, then selected expression evaluation with immutable owned financial pre-state.
5. Debit expression work, map descriptors, run the existing repayment kernel once and publish combined ordinary/financial result only on success.

`parseOwnedState` currently checks only object/work.remaining shape. It alone is NOT financial validation. Factor/expose the kernel state parser or an equivalent shared primitive-string admission entry; do not duplicate its rules or fabricate a dummy Transfer/empty-batch execution. Preserve original prepareRepayment behavior/order.
The implementation may choose private factory plumbing, but must keep source public inputs primitive strings, validate before guards/lookups, and keep context immutable/not caller-owned. A separately reachable Core /2 evaluation entry without financial context rejects FINANCIAL_CONTEXT_REQUIRED; static checking does not require live state. Old Core /1 never accepts new reads, even in dead branches.
External state admission failures return closed rejection without effects/post and workUsed0 where applicable; retain kernel validation codes rather than inventing lossy success. Pin exact envelope and priority in tests and spec. Runtime read errors use read node source span/path and actual charged work, not synthetic transport errors.
Each read constructor costs one expression reduction plus its identity child's actual reductions. Short-circuit And/Or/Select executes only the selected runtime branch, but static checking covers all branches/actions. Unselected actions and lookup-table order do not affect debit. State admission/index building is bounded validation work, outside runtime E; each collection remains capped128. Document this instead of claiming unbounded lookup is free.
Retain all existing source/token/node/depth/schema/value/effect bounds and global work accounting. Reads cannot modify financial state, reset spent/remaining work or consume closure reserve. Failed guard/read/ensure/funding/work/identifier reuse exposes no tentative output.

## CLI and demonstration

Same positional CLI forms as /2, with profile/3:
```
node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-agreement-source/3 SOURCE
node experiments/moriarty-language/src/cli.ts format --profile moriarty-financial-agreement-source/3 SOURCE
node experiments/moriarty-language/src/cli.ts simulate --profile moriarty-financial-agreement-source/3 --action NAME --snapshots SNAPSHOTS --repayment-state STATE SOURCE
```
No --schema; check/format reject --action; simulate requires one. Semantic rejection exit1, stderr JSON, empty stdout; usage/I/O retain exit2. Existing profile commands retain exact forms.
Create `spec/successor/examples/financial-state-payment.mori`, matching canonical snapshots and state, and `examples/financial-state-payment.mjs` plus package script `financial-state-demo`. Demo must load actual .mori/fixture files through public source API, not hand-build Core. Include actual CLI tests separately.
Fixture has shared Cash/Transfer/Repay declarations, ordinary due/paid, and repay, repay_installment, repay_remaining. Use the current two bodies with guards based on outstanding<Cash>("Due100"), then repay_remaining sets `let nominal = outstanding<Cash>("Due100"); let payment = magnitude(nominal);`, requires positive payment and adequate typed balance/allowance, stages paid update and emits the existing matching Transfer+Repay. Keep identity arguments explicit and distinct per invocation (T1/Alloc1,T2/Alloc2,T3/Alloc3). No nominal argument for repay_remaining; extra Args reject.
Start financial remaining work256, spent0 and reserve16; snapshots workInitial256. Preserve full previous post/financialPost/work on each invocation. Pay30 then20 then computed50; final principal/accrued/outstanding0, statusSettled, payer/lender0/100, allowance remaining/spent0/100, six IDs retained, ordinary paid100. Determine exact E independently from final bodies; don't reuse prior43/45 counts after adding read guards. Fourth repay_remaining fails positive-payment guard with no effects. Include interest-first control and extra unrelated projection entries preserved.

## README and specification work (explicit user request)

Update root README.md and language README.md as part of this candidate. Remove stale global claims that source-defined schemas, multiple actions or descriptor-to-kernel funded execution are absent. Preserve accurate historical profile descriptions, Preview/native acceptance limits and unrelated root prose.
Root `Language specification and formal semantics` must distinguish syntax/0, expression-source/1, pure financial-expression/1 and agreement-source/1,/2,/3. Present the current /3 grammar with uninitialized state, record/operation declarations, multiple actions and six generic reads; make historical full /1 grammar clearly historical or replace that subsection with current canonical /3 EBNF. Do not mislabel a partial excerpt complete. If full EBNF is reproduced, check exact canonical text equality; links and concise explanations may accompany it.
Update stale expression composition paragraph: funded adapter now maps descriptors, charges E+N and publishes atomically; ordinary ensures see ordinary post and new financial reads see financial pre. Add concrete typed-read rule and reduction/work rule with exact source syntax, full-state admission, runtime lookup/range failures and short-circuit behavior. Distinguish TypeScript executable semantics from existing bounded K kernel; no claim of K coverage of new constructors.
Add tested repository-root check/format/simulate and financial-state-demo commands, plus the three-step expected result. Keep older atomic workflow examples scoped correctly. Update successor README and CLI/profile docs; add `financial-agreement-source-v3.md`, matching complete grammar and read/Core contract documentation. No broad roadmap rewrite.

## Implementation and verification steps

- [ ] Add focused failing /3 source/API/CLI/state validation tests and retain red run before implementation.
- [ ] Share full kernel state admission; preserve old kernel behavior and all baseline tests.
- [ ] Add strictly gated read Core shape/type/runtime support and /3 source lowering/parser/compiler/factory/CLI.
- [ ] Implement fixtures/demo and README/specification changes in the same deliverable.
- [ ] Exercise all six reads, dynamic identifiers, max/range boundaries, zero vs missing, denomination mismatch, duplicate/unrelated invalid state, spoofed Pre/Obs, same-call immutable reads after emit, unselected static errors, skipped runtime errors, exact selected Args, source bounds and spans, stale work and no partial outputs.
- [ ] Compare older source and Core public contracts against reviewed baseline, including new intrinsic names that were previously legal and compound-invalid validation order.
- [ ] Run package tests/typecheck, demo and actual CLI commands. Independently derive full financial and work expectations, including exact work boundary and interest-first behavior.
- [ ] Root runs independent probes; return concrete findings to same Grok author for repair. After two same-class failures reproduce and change approach.
- [ ] Freeze all language files plus rootREADME and plan; fresh Astra medium reviews complete exact affected path and own probes. Changed bytes require current audit.
- [ ] Integrate only scoped files against baseline hashes, preserve unrelated work, commit and publish stacked on feat/multiple-named-actions. Verify actual main CLI and remote head; only then close goal.

Commands: `npm --prefix experiments/moriarty-language test`, `npm --prefix experiments/moriarty-language run typecheck`, `npm --prefix experiments/moriarty-language run financial-state-demo`, and CLI forms above using committed fixtures.

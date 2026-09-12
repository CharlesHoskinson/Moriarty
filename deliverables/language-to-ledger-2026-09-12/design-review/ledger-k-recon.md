# Ledger and K implementation feasibility

Repository observation, 2026-09-12 UTC, HEAD `229545fada12c151ed5693f4e6cebdab956bd222`. Read-only reconnaissance; no test, proof, compilation, wallet query, service start or network submission performed. Installed Moriarty develop skill and execution-focus reference applied. Latest task routing supplied by parent is Grok 4.6 high author with fresh Astra medium audits; historical identities remain historical.

## Current callable boundaries

| Area | Existing reusable implementation | Missing behavior / restriction |
| --- | --- | --- |
| Main K | `experiments/moriarty-language/formal/k/{codec.py,moriarty.k,run.py}`, numeric fixtures | Fixed two balances, allowance, obligation, empty history, Transfer optionally Repay. Arbitrary lifecycle, origination, accrual, Pending/Complete and authenticated state are unsupported interfaces, not dashboard gaps. |
| Expression K | `.worktrees/sp03-expression-k/experiments/moriarty-language/formal/k/`: `expression_codec.py`, `expression-{wire,schema,json,values,infer,v1,types,shape}.k`, expression toolchain lock and fixtures | Unmerged worktree route for expression preparation, not full financial transitions. Runner supports `--suite expression-v1`; 113 invocation/2512 second original allocation. The original worktree has no normal build directory; frozen compiled artifacts are present in sibling `sp03-expression-k-parse04` and `sp03-expression-k-macro05` worktrees (details below). Bind exact selected artifact and amendment before reproducing. |
| Financial Compact | `experiments/moriarty-midnight-financial/custody/{generate.mjs,loan.compact,bindings.json}` and language `src/lower-compact.ts`, `compact/generated/loan/` | Concrete older bounded-atomic loan wrapper, not successor lifecycle lowering. `initialize` hardcodes principal/state and mints test tokens; `accrue` and `settle` constrain fixed amounts and effects. |
| Authentication | `loan.compact::capabilityHash`, constructor, initialize/accrue/settle; ledger `run-local.mjs::checkBinding` | Program/network/borrower-capability/revision/actor and block-time checks exist. They do not authenticate arbitrary successor state, transaction fee-inclusive postconditions, replay history or imported evidence. Network endpoints in driver are configured bindings, explicitly not fresh chain-identity verification. |
| Real driver | `ledger/{run-local,integrate-local,launch-local,providers,receipt,financial-comparison,finalized-financial-state,contract-balances}.mjs` | SDK deploy/submit calls and native transaction/state comparisons exist. Loan path calls deploy → initialize → accrue → settle with fixed values. Reuse transport and decoding while replacing semantic binding explicitly; current example does not demonstrate new source origination. |
| Exit retention | `deliverables/sp05-loan-exit-retention-grok-2026-09-10/candidate/{cli,loan_exit_operator,terminal_predicate,launch_contract,exit_retention}.py` | Reviewed collector is reusable source; live `collect-once` deliberately refused. Actual installed executor/prover-lifetime helper absent in main and inspected AFK worktree. This is missing implementation, not just stale bookkeeping. |

The full executor contract already exists at `openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md`. Reuse it rather than designing another orchestrator. It requires the real parser/main→collector→durable writer→exit selector→identity-safe cleanup path and a small static `plugins/moriarty-dev/scripts/moriarty_dev/prover_lifetime.c`. Both are specified, not found implemented. It permits a separately reviewed never-started invocation container with the static lifetime wrapper as PID 1; the historical proof container must not simply be started under an assumed host lifetime bound.

## Commands that exist (not run here)

Offline existing integration controls:

```sh
cd /home/charl/Moriarty/experiments/moriarty-midnight-financial
npm run test:ledger
npm run test:compiled
npm run test:finalized-state
npm run test:preview
```

These check different boundaries; preserve unavailable compiled prerequisites and test failures. They are not current acceptance merely because commands exist.

Actual launcher CLI syntax, only after exact reviewed plan, binding, allocation and guarded dispatch:

```sh
node experiments/moriarty-midnight-financial/ledger/launch-local.mjs --run --plan <private-plan-path> --sha256 <exact-plan-hash>
node experiments/moriarty-midnight-financial/ledger/preview-bootstrap.mjs --run --plan <private-plan-path> --sha256 <exact-plan-hash>
```

Neither placeholder command is an admitted action. Existing guarded entry point is `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . run --action <existing-admitted-action>`; resolve current action from `next --json`, do not bypass policy by invoking launcher directly.

Expression K has the parser route `python3 run.py --suite expression-v1 evaluate <input.json>` from its K directory, but that route requires a matching compiled binding/artifact and remaining debits. Historical resource argv is retained at `.worktrees/sp03-expression-k/deliverables/sp03-expression-k-2026-09-10/resource-proposal-02.json`. It is a proposal, not fresh admission. Preserve source/counter checks. First resolve the retained 5947 metadata case and artifact, then admit one reproduction with diagnostics; do not replay 105 passing cases or introduce a metadata ceiling. Financial lifecycle parity requires implementing additional K state/actions beyond this expression route and then full final conformance.

## Fresh observed blockers and historical evidence

Fresh plugin `status --json` reports `binding-input-stale` and `candidate-input-stale` for `openspec/sprints/sp01-financial-contract-and-execution-admission.md`, absent `.moriarty-dev/runtime/current-accounting.json`, and unavailable live resource state for `sp01-loan-swap-grok-01`. Pending transactions were empty. These are bookkeeping/admission blockers; they do not imply missing financial transport.

Fresh Docker status: `moriarty-midnight-node` Exited (0), indexer Exited (143), proof-server Exited (0), all about 30 hours old. No Midnight service runs. Existing identities can be preserved, but this is not an armed Docker execution environment and no wallet/funding/finality condition was measured.

`deliverables/sp05-loan-executor-grok-2026-09-11/root-prerequisite-inspection01.json` is historical: last known reservation total 27 submissions and 8100000000000027 DUST SPECK, not paid fees or global remaining balance. It identified changed local driver modules against earlier closure. Do not turn that historical observation into current admission. Existing local loan/swap and stale-loan reviewed-result records and accepted Preview swap scope are reusable evidence only after exact scope/lineage checks. Historical loan raw exit was unavailable; success output cannot repair it.

Bounded bookkeeping amendments are feasible within existing delegated authority: bind actual current source closure, reconcile historical charged attempts with immutable records, obtain exact resource observations, and review a bounded resource amendment with current substantive votes. Missing accounting must be reconstructed from evidence, never initialized to zero; fresh source votes do not create runtime or resource approval. Unsupported lifecycle/Compact/executor interfaces require actual source implementation first. No new campaign framework is necessary.

## Minimal implementation route for the requested capability

1. Freeze bounded source lifecycle semantics and fee-inclusive financial predicates in existing language/OpenSpecs. Define independently expected complete effects, residual obligations, stale revision/replay/auth rejection, rollback, gross debit preserved under refunds and net goals after fees.
2. Extend the actual successor-to-Compact consumer, custody state/authentication and ledger comparator together. Represent persistent obligation/lifecycle state and bind it to source/program/profile and predecessor revision; avoid treating host-generated state flags as authenticated evidence. Keep existing four-effect boundary and explicitly scoped profile.
3. Extend/reuse expression K loader with real corresponding lifecycle state/actions. First resolve the single known backend crash under bounded diagnosis. Fresh byte-matched full conformance is required after semantic changes, not merely historical prefix reuse.
4. Implement the already specified executor around existing launch/receipt modules; cover complete fault matrix offline. Reuse installed collector adaptation and static prover lifetime contract. Freeze exact source; obtain current independent full-source audits.
5. Reconcile current allocation/admission once; prepare preserved-identity Docker services under real containment; run the exact source-produced lifecycle through authenticated Compact state and retain native bytes, finality, fee/effect comparisons, raw exit before stop and independent containment. Audit actual result.
6. Admit necessary Preview predicate separately after successful Docker evidence and exact wallet/resource observations. Reuse applicable historical swap evidence instead of spending solely to refresh dates. Audit actual Preview result; keep I2 uncertified until MC05 and SP05 dependent on SP01.

## September 11 SDK update

Inspected `docs/superpowers/specs/2026-09-11-defi-kernel-sdk-interface-design.md` from `be2bf44`: explicitly specified-only. It anchors a session to one Midnight coordinator; foreign legs advance bounded pending records by ImportFrom, not Step. Final Step checks actual gross debits, net credits, fees and liabilities. The SDK explicitly cannot simulate absent language lifecycle interfaces through local bookkeeping, and returns E_LIFECYCLE_UNSUPPORTED / E_PROFILE_NOT_ADMITTED for unadmitted tiers. Preserve this compatibility when designing lifecycle/auth state and fee fields; do not add multichain execution or claim the SDK is implemented in this interval. Existing four effects and current Preview-only public target remain intact.

## Located frozen K crash artifacts

Follow-up read-only filesystem inspection located both complete artifact directories under `.worktrees/sp03-expression-k-{parse04,macro05}/experiments/moriarty-language/formal/k/.build-expression-v1/`. Each binding records 106 invocations; each observations file contains 105 matching observations ending `metadata-depth-1500`. `trace-106.command.json` records parse04 return code 139, no timeout, 2.461 seconds; macro05 return code 113, no timeout, 3.927 seconds. These are inspected historical commands, not a new reproduction. The macro05 command adds `--no-expand-macros`; this already changed approach without curing the failure.

Reuse `trace-106.input`, `trace-106.command.json`, `binding.json`, `expression-v1-kompiled/{interpreter,definition.kore}` and the corresponding `expression-parser.py` in the chosen sibling worktree. Exact recorded krun executable is `/nix/store/y63xkr8pk2bqd5lh4889rlwldw26v9f4-k-7.1.337-4a46d1231473b599c699160132fd6e76a5c46406/bin/krun`; recorded argv has input, `--definition` directory, `--output json`, `--parser` path and (macro05 only) `--no-expand-macros`. A bounded amendment can authorize this one retained case with stack/exit diagnostics and unchanged artifact; source/artifact/tool pins must be verified before the run. Do not silently use the original worktree runner or its budget as if it were the selected artifact campaign.

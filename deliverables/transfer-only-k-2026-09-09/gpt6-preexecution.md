# Independent GPT-6 Astra preexecution review

Decision: **APPROVE** the exact source candidate and **APPROVE** the exact bounded resource proposal for one provisional local SP03 Transfer-only experiment. No blocking finding. This is a preexecution vote, not a result acceptance or a full-stage admission.

Reviewer: fresh independent `gpt-6-astra` delegated reviewer, 2026-09-09. Checkout: `/home/charl/Moriarty/.worktrees/sp03-transfer-only`. The reviewer did not author the implementation or fixtures. Review writes are confined to this receipt; no K compilation, krun, proof, native run or network submission was performed.

Exact commitments:

- Candidate `candidate.json`: `2b802c07feb801585cb2521f69ccecb7df8879d597fa856a63c5f6852e30b5a0`.
- Resource `resource-proposal.json`: `176542f1ed5a429369dcf8eabf4768477589546e60dfc2e88bf822c3375634d5`.
- Independently recomputed SHA-256 for every one of the candidate's **35 files**; all match. Base commit is `3d6ea75097fb7c9f8cefb7d35ec848e54f3631cc`.

## Source findings

Repository observation: the separate `transferPacket` constructor reuses the balance-index and movement functions without changing the repayment rules. Its 14 ordered guards check duplicate balance keys, allowance/debt/work invariants, one unit of ordinary work, and the eight Transfer checks. This matches the admitted subset of the financial contract: admission precedes work, reserve cannot fund work, and cash movement does not require the sender to be the debtor or recipient to be the creditor. A settled unrelated obligation stays legal. The fixed collection sizes and empty input histories make replay and capacity failures unreachable in this projection; they have not been claimed as general K coverage.

Repository observation: `preparedTransfer` contains the seven changed scalar amounts/counters, receiver index and input digest. The changed values come from K arithmetic. Sender debit, receiver credit or append, gross allowance consumption and one work unit are explicit. The codec copies the complete unchanged obligation, closure reserve and allocation history, appends only the Transfer ID and returns only the complete Transfer effect. Consequently there is no implicit debt discharge or repayment effect. Rejections clear the continuation and decode to only status/code/index.

Repository observation: admission accepts precisely `[Transfer]` or `[Transfer, Repay]`, retaining the two-balance/one-allowance/one-obligation, identity-conversion and empty-history restrictions. Encoding preserves the old two-action constructor and chooses the new constructor only for one action. Decoding requires the correct result constructor and arity, UInt128 scalar strings, exact input digest, receiver index tied to the input, and zero unused append field. A one-action result cannot report action index 1. Copying unchanged fields is an explicit codec projection, not a claim that full records are calculated inside K.

Repository observation: the repayment inspect/finish rules and arithmetic helpers are unchanged in the diff. The shared rejection check's new action-count bound retains the old allowed indices for two-action inputs. The six retained repayment cases preserve both success behavior and atomic late rejection.

## Independent cases and checks

The fixture provenance records independent contract-derived expectations, without consultation of K, codec, evaluator or observed runtime outputs. I checked the contract and the ten new cases: the four successes cover exact one-work arithmetic, missing receiver append/order, swapped third-party/non-creditor movement and settled debt preservation. The six failures exercise reserve exclusion, zero amount, receiver overflow, invariant-before-work precedence, missing exact allowance and insufficient cash. Their expected complete states/effects or rejection-only shapes agree with the contract. This is finite case coverage; it does not exercise every guard independently.

I independently verified 16 unique records and compared all six retained repayment records against their original fixtures as **exact raw JSON object substrings**, including input and expected bytes. These six remain inherited expectations, not newly independent derivations.

The source checker constructs actual source text, calls `prepareSuccessor`, and compares the entire returned object with each fixed expected result. It neither generates expectations nor calls K. The distinct nominal denomination and settlement asset survive this path.

Independent reviewer executions:

- `python3 -m unittest test_codec test_runner test_transfer` in `formal/k`: **12 tests passed**.
- `node deliverables/transfer-only-k-2026-09-09/check-source-cases.mjs`: **16 complete source observations matched**.
- Candidate hash verification and raw fixture-substring comparison: passed.

The committed evidence reports **236 language tests passed**, with no failures or skips; I inspected that retained result and its bound hash rather than rerunning the unchanged language suite. None of these checks establishes actual K execution. The `.build` directory was absent when inspected.

## Presentation and limits

The Felleisen–Hieb-style README presentation distinguishes control steps from internal K rewrites and financial work: 14 guards plus START, EXPAND and PREPARE give 17 successful Transfer-only control steps; 21 guards give 24 for repayment. Context-erasing failure and the distinct scalar result constructors are represented. The explanatory abstraction does not assert a correspondence theorem.

The scoped deliverable explicitly says K execution and result acceptance are pending. The root README's earlier 16-case execution sentence links to the historical bounded-repayment receipt; it must continue to be read as that historical result, not as evidence for this pending candidate. Final publication should report the actual new run status and link its exact receipt. This is a reporting follow-through, not a request to widen this candidate or its infrastructure.

## Resource vote and execution conditions

**APPROVE** exactly one compile attempt at 180 seconds and at most 16 sequential krun invocations at 20 seconds each, within the 512-second aggregate ceiling. The exact proposal uses a unique systemd user unit, this worktree, nonblocking shared flock, `MemoryMax=4G`, `MemorySwapMax=0`, `RuntimeMaxSec=512`, `KillMode=control-group` and `KillSignal=SIGKILL`. The kill signal is immediate; `TimeoutStopSec=5` does not introduce a TERM grace into this specified SIGKILL stop. Whole-tree containment belongs to systemd; the runner additionally kills command process groups and uses a cumulative monotonic deadline.

The existing runner records its exclusive compile-attempt marker before compilation, debits each krun before dispatch, verifies source/toolchain/build bindings, stops on the first command or observation failure, and has no automatic retry or backend fallback. Only the suite registration changed. Its existing 16-call ceiling is appropriate for these 16 fixed records. The historical receipts confirm **four prior compile attempts and 33 prior krun invocations**, preserved separately from this allocation.

Before dispatch, root must retain the proposal's existing checks: exact current commitments, fresh build, no conflicting heavy run, successful named-unit/flock containment, and the required agreeing independent votes. This vote does not waive those checks or authorize deleting a consumed attempt. Any failure ends this run and remains evidence.

Startup status was inspected after loading AGENTS.md and the tracked development skill. It reports unresolved old SP01 operational history and no pending transactions. This review leaves that stop intact. The approved scope is the separately reviewed, root-supervised provisional SP03 experiment described in SP03.md; it does not dispatch or unblock the old loan action, admit the full successor-semantics stage, close SP03, establish proofs/correspondence, or establish ledger settlement. Exact runtime artifacts require a separate independent result audit.

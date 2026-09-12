# Exact canonical Task 2 design review 01

Verdict: **APPROVED** for the exact Task 2 design at SHA-256 `74299d10cca2874828c4c847f6817677b033090bb85d38bb898dd462fee98383`. No residual blocking design contradiction was found. This approval covers the closed local lifecycle contract and its required source/Core integration; it does not approve an implementation or any ledger execution.

Reviewed checkout: `/home/charl/Moriarty/.worktrees/loan-origination-accrual`, HEAD `53f9b78981031f8c12759b422416eef78adc51fd`. Main and worktree canonical contract bytes match. The original proposal, JSON oracle and root decisions retain their prior reviewed hashes. Requested reviewer remains gpt-6-astra, medium, fork none; actual reviewer is the same fresh-review Codex GPT-6 agent `/root/origination_schema_review`. Concrete deployment identity is not independently exposed inside this child; preserve parent host model metadata. This is a follow-up design audit by the original design reviewer, not a fresh implementation auditor.

The Moriarty development skill remained loaded and required status was refreshed for the new checkout. Its dependent dispatch remains blocked by recorded binding/accounting/resource gaps; no pending public transaction was returned. No author, service, campaign, external model or installation was launched. Only this review and its JSON companion were written.

## Disposition of prior findings

R1 is resolved. The canonical contract requires full lifecycle admission and compact UTF-8 state size <=65536 after kernel actions/work debit and again after source suffix debit. Both checks occur before publication. Oversize rejects `{status:"Rejected",code:"RESULT_BOUND",actionIndex:null}` without tentative fields. Effects remain outside the state byte count. The independent 64401/65516/65544-byte construction and neighboring success/re-admission controls are required. This closes the identified continuation-size omission without changing old versions.

R2 is resolved. Originate and Accrue now have explicit ordered procedures. Originate checks identity/funding and prior consumption before conversion, then exact amount, UInt64 initial boundary, cap and append capacity. Accrue checks target/status/identity/period/time, then all UInt arithmetic including next boundary, then signed range, cap and capacity. Full admission and action-count work preflight precede application. Combined-failure cases distinguish each relevant branch and avoid the unreachable full-origin/fresh-transfer fixture. Existing Transfer/Repay order remains inherited.

## Contract consistency

The four explicit versions and public factories separate closed legacy behavior. The inherited Core/3 and source/4 envelope rules are resolved by the unchanged canonical design and inspected implementation: malformed primitive financial-state transport retains bare INPUT_SCHEMA/INPUT_BOUND, admitted kernel-state faults retain code/actionIndex:null, and expression/static errors retain their synthetic or original source envelopes. The new RESULT_BOUND is a financial-result rejection at both publication boundaries, with the explicitly specified null actionIndex; it should not be normalized into a source diagnostic. The state-only admission helper inherits the existing owned-tree transport conventions; the complete kernel input inherits compact JSON admission. Neither implies caller objects or trusted callbacks.

Protected nested record binding is structural against exact declared fields and types, not hard-coded preferred record names. All four protected operation declarations are required, while individual invocations may emit any nonempty supported subset. Source U=A names do not merge Quantity and Amount types. Accrue target denomination must be checked despite lacking a nominal amount, and post-target checks before suffix allow explicit same-call Originate/Accrue against prepared obligations. Unrelated foreign rows remain unchanged.

Complete Origination and Accrual effect key sets match the retained oracle. Existing Transfer and Repayment effects remain complete; ordered effects correspond one-for-one to actual kernel actions. Full original funding equality plus unallocated-remainder equality excludes partial or repeated origination funding. Repay retains its prior partial-allocation semantics and cannot reuse a remainder consumed by Originate. Cash moves once through Transfer.

The immutable lifetime cap/incurred policy, checked simple-interest formula, signed nominal bounds, zero-interest work/ID use, exact period progression and retained settled history remain feasible. The supplied main trace remains 0→100→110→80→0 with lender110/borrower0, preserved unrelated rows and kernel work249/24/reserve16. Source work still requires actual parsed source and independent dynamic reduction counts; the kernel oracle must not be copied as source work.

The exact contract is a scoped refinement of the Task 2 boundary left open in the broader design/spec/plan. The plan's earlier Originate field sketch is not the exact closed schema: the reviewed canonical contract supplies nominalLiabilityCap, full lifecycle state and effects. No earlier Task 1 semantics are amended.

## Evidence and remaining gates

Checks performed: full canonical contract review; comparison with prior proposal/oracle/review; main/worktree byte equality; unchanged original evidence digests; inspection of current source/Core state-admission and publication paths; refreshed plugin status. The companion JSON pins every reviewed input. No Task 2 code or proposed lifecycle execution was tested because implementation has not started.

Required implementation checks remain those in the contract: complete closed-schema and malformed-state admission, structural binding, all-action error priority, exact consumed funding, checked arithmetic/cap/period failures, result-bound rollback and successful re-admission, actual CLI/source behavior, exact work and full legacy regressions. A fresh exact-candidate implementation audit remains mandatory.

Local shape admission is unauthenticated. Consistent substitutions of cap, consent, roles, terms, predecessor or time require Task 5 authentication and aggregate-liability controls. K correspondence, Docker/Preview execution and proofs remain open. Approval grants no acceptance credit to those later predicates.

# MC01 execution decision

Status: proposed exception, not authorized or executed.

## Problem

The approved charter requires Endstop to enforce cumulative subprocess budgets and protected package reservations.
The installed runtime provides durable action counts and an admission deadline.
It does not supply the required cumulative budget, historical debit, or process-lifetime enforcement.
Nested queues do not close this gap. The parent exits after submitting its child.
See `endstop-compatibility.json` and `endstop-compatibility.md` in this directory.

The runtime installation and Grok worker readiness checks passed.
No actionful worker, native campaign, or public submission has started.
The Codex runtime goal is active. That does not establish worker readiness under the charter.

## Recommended bounded exception

Permit a directly supervised MC01 run through the existing process launcher instead of the contract-bound queue.
This exception applies only to MC01. It does not change acceptance or authorize MC02–MC08 dispatch.
Keep the current 50-minute MC01 reservation and four-worker maximum.
Keep the existing master ceiling, planning debit, and protected allocations.
Do not add a new budget or repair Foreman.

Before each process, persist its full worst-case charge and the remaining review reservations in external state.
Charge the full bound even if the process exits early. Interrupted work does not restore its reservation.
Execute one supervised process group at a time. Stop before its bound expires, including termination grace.
Use the installed strong process containment with the existing 8-GiB and two-CPU limits.
Do not claim that a shell timeout alone enforces aggregate memory or CPU constraints.
If the existing launcher cannot enforce a required process limit, stop before dispatch and report that exact gap.

First authorized worker bundle, subject to the exception:

1. Complete the source-bound grammar, typed numeric/bounds schemas, lifecycle judgments, and complete 32/72 target crosswalk.
2. Write only the MC01 specification files and its evidence directory in the isolated worktree.
3. Keep unsupported behavior explicit and every unperformed implementation predicate unchecked.

Reserve at most eight minutes for that worker and two minutes for independent structural checks.
Protect ten minutes for Fable and ten minutes for a fresh GPT-6 result audit before starting the worker.
Use remaining MC01 funds only for justified corrections or a separately frozen frontend bundle with its own required audit reserves.
Do not assume that all MC01 implementation fits after the profile reviews.
Preserve the draft and stop if those reservations cannot fit the remaining package allocation.

Require both exact reviewers before freezing the profile or accepting implementation.
No native proving, public transactions, financial key access, runtime repair, or new provider login is allowed in this exception.
Before retaining generated evidence, check its byte total against the remaining program allowance.
Treat worker scratch as unaccepted until that check passes.

This is direct supervision with conservative external accounting.
It is not equivalent to the charter's original autonomous Endstop enforcement.
User authorization must explicitly amend that one execution requirement before use.

## Alternative

Retain the current first-worker stop until a separately authorized enforcing component is available.
Source-only design can continue, but cannot be reported as implemented language behavior.

## Already prepared

- Active goal receipt: `../goal-armed-20260907.json`.
- Checked intake and source manifests: `receipt.json`.
- Native-to-ledger interface findings: `verifier-interface-intake.json` and `.md`.
- Draft worktree: `/home/charl/Moriarty-wt-moriarty-mc01-20260907-plan-profile`.
- Draft profile: `experiments/moriarty-language/spec/profile-proposal.md`.
- Proposed EBNF, numeric profile, and bounds in that same specification directory.

The draft has no Fable or GPT-6 result approval. No parser, compiler, or native proof was executed.

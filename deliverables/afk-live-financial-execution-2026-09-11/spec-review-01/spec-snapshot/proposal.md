# AFK live financial execution

Status: specified-only; independent specification approval pending. This package grants no admission and records no completed implementation or accepted result.

## Why

The eight-hour block must repair execution readiness and deliver a real, independently audited loan invocation. The reviewed collector exists, but its CLI refuses live collection. Current guarded status reports stale SP01 binding/candidate inputs and missing current accounting. Existing SP01 design and SP05 local-runtime action mappings do not authorize a public loan.

The user requires OpenSpec requirements for every item before execution and delegates audit/approval while AFK. This package supplies that specification step within the existing workflow.

## What changes

Each item follows requirements → independent specification approval → Grok implementation or authorized execution → actual checks → both applicable fresh audits → scoped result. Fresh Claude Opus (`claude-opus-5`) and GPT-6 Astra medium review the full exact specification/candidate independently. Grok 4.6 high implements. Returned identities, effort, verdict scope and historical dissent remain recorded.

The items are: (1) current I2 binding/action/accounting repair and bounded allocation; (2) complete callable loan executor; (3) final exact-candidate live admission and one bounded invocation; (4) financial/result review and original MC02 reconciliation; (5) publication and containment/handoff; (6) optional read-only F0 diagnosis. Item 6 cannot displace financial execution. Item 1 may receive approval and begin while later item specifications are refined; each later item must receive its own applicable current approval before work begins.

## Scope and clock

The controlling plan is `docs/superpowers/plans/2026-09-11-eight-hour-afk-development.md`. T0 is `2026-09-11T06:46:18Z`; the deadline is `2026-09-11T14:46:18Z`, including waits and reviews. Reserve the last 45 minutes for publication, cleanup and handoff. From T0 + 6.5 hours, launch no new live or proof dispatch. Earlier launches must leave enough time for their admitted maximum duration, cleanup and both result audits. No clock reset on continuation.

The block creates no funds, attempts or proof authority. The historical loan allocation's two attempts remain consumed. Any new allocation requires an explicit bounded amendment reviewed under standing authority and all applicable admission gates. Missing services or evidence leave the execution objective incomplete.

Use existing campaign, accounting, operational-history, result and session records. Do not create a scheduler, evidence framework or alternate acceptance register. Preserve wallet identities, keys, contract state, immutable receipts and unrelated work. Full I2 acceptance requires every original MC02 predicate; I2 remains uncertified until MC05. Full SP05 additionally requires SP01.

## Observed implementation boundary

Inspected `plugins/moriarty-dev/scripts/moriarty_dev/records.py`:

- `_verify_binding` verifies `moriarty.sp01-execution-binding/1`, its exact binding hash and input hashes. The current SP01 binding commits an older `openspec/sprints/sp01-financial-contract-and-execution-admission.md` hash.
- `_verify_resource` reads the selected campaign's `currentAccounting`. `_remaining_from_current` accepts the closed `moriarty.supervised-accounting/1` family, with `charges`, `reserved`, `externalPackageCharges`, planning charges, package/master limits and dispatch counters. It explicitly refuses the historical `resourceRuntimeSnapshot` path as current credit.
- The current accounting shape does not contain wallet identity or observation timestamps. Do not silently add unknown fields or claim those checks already exist. Bind genuine public wallet/resource observations through the existing allocation/admission evidence and review their actual consumer before dispatch; change a consumer only for an independently specified, reproduced gap.
- `runner.py::load_runner` requires one existing debit matching action, candidate, runner digest and charged seconds. The runner does not produce accounting or create credit. A missing producer/invocation remains an implementation obligation; tests that manufacture accounting fixtures are not that producer.
- `.moriarty-dev/actions.json` currently declares syntax/local-runtime actions. `.moriarty-dev/commands.json` does not supply the proposed live executor. Refresh these facts before implementation; valid design verification is not live admission.

Inspected `deliverables/sp05-loan-exit-retention-grok-2026-09-10/candidate/cli.py` and `loan_exit_operator.py`: `collect-once` refuses; `retain_loan_main_exit` delegates to the collector after a separately admitted launch. The caller, durable writer, exit decision and cleanup path must be integrated and tested together. Never import historical actionful scripts in offline tests.

`deliverables/sp05-loan-executor-grok-2026-09-11/root-prerequisite-inspection01.json` records historical reservation totals, not paid fees or comprehensive global totals; it also records three changed local modules. Reinspect current closure and costs rather than reusing those observations as current authority.

The September 11 accounting inspection located the retained master at `~/.local/state/moriarty/mc01-supervised-20260907/budget.json`: master limit 210890 seconds, planning 2323.0473305040214, overhead 165, charges 108960, reservations 300 and external charges 99135 leave approximately 6.95267 seconds. Package dispatches are 52/52 and aggregate dispatches are 83/83. These observations cannot fund the new action. The retained producer `~/.local/state/moriarty/moriarty-dev-plugin-20260908/successor-binding-03/run.py` uses `budget.lock`, transfers reservations into pre-launch charges/dispatches and replaces accounting atomically. Inspect it as text; do not replay it. Restore its relevant existing accounting operation with the reviewed current amendment and caller.

Consumer limitations must be handled explicitly: `_verify_resource` adds amendment, successor and binding allocations, so repeating one envelope across those fields can double-count apparent credit. `_remaining_from_current` does not itself enforce the aggregate master dispatch ceiling or authenticate amendment votes. The real producer/admission path must enforce those obligations; shape-valid reader success is insufficient. Preserve the original authority identity and reconcile any reviewed ceiling/counter extension cumulatively. Keep mutable accounting outside candidate/binding input hashes; the runner's exact debit identity binds the action/candidate/runner without creating a circular hash commitment.

## Acceptance

The normative requirements and positive/rejection scenarios are in `specs/afk-live-financial-execution/spec.md`. `tasks.md` gives the per-item specification gate and completion evidence. No checkbox is satisfied by this package's existence. Preserve original MC02 acceptance and later model-routing overrides together.

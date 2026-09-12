# Fresh candidate audit

Review the complete frozen source-defined repayment candidate as GPT-6 Astra at
medium effort. This is an independent audit, not implementation. Do not change
candidate source or the plan. Write only this task's audit output directory.

Checkout: `/home/charl/Moriarty/.worktrees/source-defined-repayment`.
Evidence: `/home/charl/Moriarty/deliverables/source-defined-repayment-2026-09-11`.
Read checkout AGENTS.md, load the installed Moriarty develop skill and run the
guarded CLI status from this checkout before review. Current user routing is
Grok 4.6 high author plus fresh Astra medium audit, overriding older routing.
Existing live admission/accounting stops remain; no live dispatch is requested.

The approved plan is `docs/superpowers/plans/2026-09-11-source-defined-repayment.md`.
Verify every hash in the supplied candidate manifest before and after review.
Review complete affected implementation and its parser, formatter, source
compiler, type checker, funded adapter and repayment kernel dependencies, not
only the diff. Hash verification alone is not semantic review. Identify actual
review scope and limitations in the receipt.

Acceptance:

- Units, assets, records, ordinary uninitialized state, protected operations and
  one action are authored in one .mori file under the separate profile.
- Declaration elaboration yields the exact existing checked schema. Public
  evaluate accepts only source/snapshot/state strings, compiles internally, and
  preserves source UTF-8 offsets. No second evaluator, source stripping or
  caller-supplied Core execution bypass.
- Normal check/format/funded simulate CLI needs no --schema. Existing profiles,
  forms, bounds and runtime financial semantics remain unchanged.
- Duplicates in each namespace, record fields and parameters reject. Unit and
  asset same-name exception is intentional; all other top-level collisions
  reject. Forward references work; unknown types and cycles reject.
- Exactly Transfer/Repay bindings; changed fields/types, extra/missing
  operations, mixed units and financial reclassification reject during check.
- Principal30 against100, interest-first7 against P100/I10, continuation,
  complete residual state, identifiers, allowances, exact work E+N, unchanged
  separate closure reserve, and atomic rejection match the old funded path.
- Guards/ensures/work/funding/unit failures expose no tentative state/effects.
  Ordinary fields cannot replace kernel financial projection. Ensures sees
  ordinary post only. Snapshot/state transport stays bounded and owned.
- Source diagnostics point at the offending original declaration/type,
  including parameters, nested indexed types and malformed Repay bindings.
- Format is idempotent, schema-stable and simulation-equivalent. Source and
  token/node/depth/declaration/arity limits remain enforced.

Run independent focused tests and actual CLI/API probes, including fault cases
you choose after inspecting the implementation. Root probe/test evidence is
available but does not replace your independent checks. Preserve actual command
outcomes and reproduction evidence. Test failure logs from earlier author/root
runs remain historical; do not confuse them with final candidate results.

Return a review.json with requested/actual reviewer identity and effort, exact
candidate manifest hash, scope, commands, findings with severity/reproduction,
limitations, and verdict. No unresolved finding may be silently omitted.
Do not commit, publish, call external reviewers, install tools, edit plugins or
run native/K/proving/Preview campaigns. This review is local language acceptance,
not formal proof or financial ledger settlement acceptance.

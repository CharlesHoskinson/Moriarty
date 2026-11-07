# VOTE fable-3 on DRAFT v1: OBJECT (workflow runnability; not a soundness or security objection)

Reviewed 2026-11-07 21:18Z. File identities (sha256, `openspec/changes/sherlock-vm-consensus-20261107/`): workflow.md `415a479b…`, tasks.md `c3b1e8ce…`, requirements.md `4e37777a…`, traceability.md `e3f0e872…`, assurance.md `0c71851e…`, design.md `67265296…`, proposal.md `f26594d8…`, dissent.md `7d897e0b…`.

The draft adopts the factory-only workflow, operations, committed receipts, pins checks and the extended E10. The architecture and scope decisions are supported. I object because the brief requires that `workflow.md` "must be runnable with the existing `scripts/doctorwatson` commands, or must name the exact additions needed" (BRIEF.md:111), and four places in v1 do not meet that at `8fd34ed79`. Each has a text fix; none needs a new engine. I will vote ACCEPT on a v2 that carries fixes 1-4; fix 5 is non-blocking.

## 1. SWF-005 / workflow §3: "same commit as `state-commit`" is impossible with the existing command

Evidence: `models/doctorwatson/factory/evalloop.py:14` `STATE_FILES = ("qa/sherlock/status.json", "qa/sherlock/runs.jsonl")`; `:39-46` `git commit … --only -- *STATE_FILES`. A receipt under `qa/sherlock/receipt-traces/` cannot be in that commit. (astra-3 found this first; verified.)

Fix: SWF-005 and §3 read: "the receipt is committed first in an ordinary commit (the pre-commit hook accepts it: `receipt-traces/` is outside the docs-only paths `.githooks/pre-commit:40-56`), the receipt-first reader records the receipt's SHA-256 in the check output and therefore in the `runs.jsonl` line, then `scripts/doctorwatson state-commit` records the flip. A pass whose `runs.jsonl` line names a receipt hash that is not in the tree at that commit does not count." Acceptance test: uncommitted receipt → reader exit 1; committed receipt + later state-commit → eligible.

## 2. E7 / gate rule: head-only staleness makes recording evidence invalidate it

Evidence: `state-commit` advances HEAD (above); E7 marks "run `head` != HEAD" stale; §4 gate requires passes "at the current head"; §9 says a stale entry does not count. Under v1, every T3 pass becomes stale the moment it is recorded. (astra-3 point 2; verified by reading the three texts together.)

Fix: E7 interim rule: an entry is `stale` when (a) its run head differs from HEAD **and** `git diff --quiet <run_head> HEAD -- . ':!qa/sherlock/status.json' ':!qa/sherlock/runs.jsonl' ':!qa/sherlock/receipt-traces' ':!wiki-llm' ':!docs'` reports a change, or (b) the `target-pins.json` row or key-manifest hash recorded in the run differs from the current files. State-only, receipt-only and docs-only commits preserve eligibility. Gate rule: "no failing and no stale non-retired predicate among the milestone's rows". SAS-002's full dependency closure remains the target; this is the minimum. Acceptance test: a state-only commit keeps a pass fresh; a change under `crates/` or `Cargo.toml` or the pins file marks it stale.

## 3. W2: evaluator-owned Preview holdout additions are missing from §8 and tasks

Evidence: `qa/holdout/run.sh:40-41` accepts only `rejections-local|rejections-stagenet`; `qa/holdout/lib/stagenet_check.py:19` `INDEXER = "https://indexer.stagenet.shielded.tools/api/v4/graphql"`; `qa/holdout/lib/stagenet/produce-a9.sh` is stagenet-only (`stagenet_check.py:3`); `qa/holdout/contracts/U023.json` pins `network:'stagenet'`. All under `qa/holdout/` = evaluator-owned (`CONTRACT.md:27,112-113`; pre-commit rejects non-evaluator edits; F6 fails on an unsealed edit). tasks 6.3 uses `rejections-preview` and 6.5 the U023 scenario-2 record as if they existed.

Fix: add E12 to §8: "evaluator (`DW_ROLE=evaluator`) adds `rejections-preview`, parametrises `stagenet_check.py`/`produce-a9.sh` by network, updates `contracts/U023.json` pins per target (CONTRACT CHANGE, `contract-check U023`), reseals `MANIFEST.sha256`"; add task 6.0 "evaluator lands E12; F6 passes" as a precondition of 6.3 and 6.5; name the evaluator identity (`qa/sherlock/roles.json` `dw-evaluator`).

## 4. W3: M6 operation commands name scripts and flags that do not exist

Evidence: `tools/sherlock-sdk/preview-session.mjs` does not exist; `deploy-run.mjs:16` usage is `SDK_ROOT BUILD VERIFIER PROG_D INIT_D OUTPUT [--submit]` and `:42` calls `setNetworkId('stagenet')`; `call-run.mjs:29,65,128` likewise, no `--reject-suite`; `stagenet-session.mjs:24-26,31,41` hard-code endpoints, `assert.equal(getNetworkId(), 'stagenet')` and `~/.midnight/stagenet-wallet`; `readback-run.mjs:30,106` hard-code the indexer and `LedgerState.blank('stagenet')`. `setNetworkId` is removed in Midnight.js 5 (`wiki/midnight-ledger-9-hardfork.md`). workflow.md §0 states every command exists unless in §8; for M6 this is false.

Fix: add E13 to §8: "network-parametrised session module (Midnight.js 5 `providers.config{networkId,ttlSeconds}`), `--network <name>` on `deploy-run.mjs`, `call-run.mjs`, `readback-run.mjs`, a reject-suite driver, per-network wallet directory and receipt schema `sherlock-run-<network>-deployment/1` (today `call-run.mjs:44` asserts `sherlock-run-stagenet-deployment/1`)"; make task 6.1 deliver E13 and mark the M6 operation commands "after 6.1".

## 5. W1 residual (non-blocking): retire B5 and B9 with the stagenet rows

Evidence: `qa/sherlock/predicates.yaml:141` B5 `depends: [B4]`; `:177` B9 `depends: [B5, B6, B7, B8]`. tasks 1.1 retires A8, A9, A10, B4, B8 only; under the new E10/SWF-011 `validate` rule (active row depending on a retired row is refused), `validate` fails after 1.1. Fix: retire B5 and B9 too (B9p already exists in the list; add B5p if the owner keeps B5), and land 1.4 (E10) before 1.1 so `next` never selects a retired row in between.

## Not objected

ISSUE 1-10 resolutions as recorded in `dissent.md`; SWF-001..004, 006..011; SMN-*; the operations table shape; T4 as tasks; the retirement extension in E10/SWF-011 (W1 core, absorbed); E1-E3 and F14/F15/F16. On the soundness items raised by astra-2/astra-3 (SRC-004 scenario wording, SRC-005 argument, SAS-013 scenario) I have no independent evidence and defer to the recursion and security lenses; they do not change my vote.

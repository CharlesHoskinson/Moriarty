# Proposal: the optimal Sherlock design for performance (`sherlock-optimal-design-20261115`)

Status: S2 (written requirements). This change builds, keys, proves and measures nothing. The source of truth is the debate result `~/.cache/design-debate/FINAL-DESIGN.md` (below `FD`), after the tie audit `~/.cache/design-debate/TIE-AUDIT.md` (below `AUDIT`). Inputs of the debate: `~/.cache/design-debate/BRIEF.md`, `R3/LEDGER.md` and the six R3 vote files.

## Why

Owner, 2026-11-15 night: "we can change keys until we have a final design. We are looking for the optimal design. Have 3 Fable and 3 Astra agents talk to each other, debate, and converge to a final design that improves performance." The debate converged on one design (FD sections 1 to 6). Today a warm call takes 301.24 s (MEASURED, h185). The design names the key-neutral work that removes most of it, the one final rekey, and the experiments that must pass before the final keys are cut. This change turns each normative `FD-` statement into an EARS requirement with an ID and scenarios, and each experiment into a task with its pass bar and box.

## What Changes

- One spec per module the design touches. Each requirement is one EARS sentence with one SHALL, an ID, at least one scenario, and a label (MEASURED, DERIVED or ESTIMATE) on every number it carries.
- A requirement that an experiment must admit is written with a `Where <experiment> has passed` clause. A candidate that no experiment has admitted is a task, not a requirement.
- Owner questions OQ-1 to OQ-6 are `OPEN (owner)` notes in `design.md`. None is a requirement.
- `tasks.md` lists the 22 experiments of FD section 5 with their pass bars and boxes, split into key-neutral work now, prototype rekeys, and the one final rekey.
- The tie audit reversed one chair decision in part (C-18: tail-only rebind is a design statement for S2 only; for S3 it is a candidate). This change follows the audited text.

## Scope

| Spec folder | Module | ID prefix | FD statements |
|---|---|---|---|
| `sherlock-opt-recursion-shape` | layer stack, fields, arity, path C, MERGE-WORK, CORE split | RSH | FD-01 to FD-08 |
| `sherlock-opt-lfr-terminal` | PCS parameter sets, `L_Fr` reshape | LFT | FD-10 to FD-14 |
| `sherlock-opt-wrap` | wrap row and VK budget | WRB | FD-15 |
| `sherlock-opt-key-cache` | universal key set and its key-derived data | KCH | FD-20, FD-21, FD-22, FD-41 |
| `sherlock-opt-collector-settlement` | collector run, `claims_root`, contract surface, checkpoints | CSL | FD-30 to FD-36, FD-38 |
| `sherlock-opt-prover-runtime` | resident service, blinder lifetimes, pipelines, Case A | PRT | FD-40, FD-42, FD-45, FD-48 |
| `sherlock-opt-gpu-tree` | GPU order of ports, verifier check, card layout | GTR | FD-43, FD-44, FD-46 |
| `sherlock-opt-settlement-client` | wallet, template, client timing, retained root proof | STC | FD-37, FD-47 |
| `sherlock-opt-qualification` | gates before the final keys | FQL | FD-70 to FD-74 |

The prefixes do not collide with any prefix under `openspec/changes/` on this branch (WRP and SCL are taken, so the wrap and client specs use WRB and STC).

## Hard rules

This change keeps every BRIEF hard rule: never k = 22, one BLS12-381 wrap, 100-bit security in unique-decoding form, ZK end to end, one universal key set, ledger 9.1.0.1 verifier unchanged with wrap VK at most 50,000 B, batched Midnight writes, Case A, GP-1 to GP-4 never narrowed, no GPU setting or power-limit change. A collection holds run, `DEP` and `FOREIGN` members (CSL-010); a run-only collection does not ship. AUDIT section "Hard-rule check" has the row-by-row check.

## Canon rule

Only the owner's words are canon. Every `FD-` decision is a debate result (majority or chair decision, as FD section 3 records); none is an owner ruling. Debate decisions stay open until the final design and the owner's T15 confirmation (owner 2026-11-15: keys and parameters may change until the end-to-end prototype).

## Non-goals

- No change to a hard rule. OQ-1 and OQ-2 touch hard rules and stay owner questions.
- No edit of other changes. Where this change and `sherlock-processor-final-20261111` or `sherlock-completion-20261115` differ, the owner decides at T15; this change does not correct them in place.
- No production key set before FQL-005 holds. Prototype rekeys are allowed.
- No latency target below 41 s. The ladder of FD section 2 is ESTIMATE below that figure and is not a requirement.

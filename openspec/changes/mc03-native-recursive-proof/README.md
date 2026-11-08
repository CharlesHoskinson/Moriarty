# MC03: Reviewed native encoding and recursive proof

Status: **superseded** (2026-11-08) by [`sherlock-vm-consensus-20261107`](../sherlock-vm-consensus-20261107/proposal.md) for its implementation route. ~~Status: full package acceptance remains open.~~

The financial IVC on `midnight-aggregation` traits (every task unchecked, R3 blocked at k17) is replaced by the Sherlock linear IVC node (`crates/mori-recursion/src/ivc.rs`). The obligations below are retained by reference, not closed: the Sherlock change closes none of them by itself (its `proposal.md` and `traceability.md`). The original tasks, checked history and frozen reviewer contracts below stay unchanged as history.

## Obligations crosswalk (2026-11-08)

| MC03 requirement | Carried by (Sherlock change, `requirements.md`) |
|---|---|
| Reviewed restart: changed hypothesis, checked ranges and commitment preimages, no unjustified retry | SRC-004 (range checks on `prev_h`, `steps`, `h`), SRC-009 (pin change regenerates and hash-compares relations), SWF-004 (no same-command same-head retry), task 2.14 council review |
| Real recursion and independent verification: separate-process verification, constrained genesis, discharged final accumulator, every invalid-context mutant rejected for its reason | SRC-005 (genesis), SRC-006 (run decider), SRC-010 (verifier process that is not the prover), SAS-003, SAS-004, SAS-006 |
| Hard resource stop: time, memory, k and attempt limits; receipt with peak usage, digests and status; no automatic escalation | SRC-012, SRC-016 (cgroup memory limit), SWF-004, SAS-001 |
| Internal maintainer audit and provenance, outside public validity | SAS-001, SAS-012, SRC-008 (key manifests), SWF-005 |
| Failed predicate stops promotion | SAS-003, SAS-012, SWF-002, SWF-006 |
| Complete backend decision before native dispatch: fixed-instance IVC distinguished from general PCD; defined final verifier and handoff | SRC-001 (one versioned leaf statement), SRC-006, SRC-017 (general-purpose recursion), design D4 (linear IVC) |
| Financial obligations (two-step accrual and due-settlement history) | Retained by reference. They run as Sherlock programs and close only under the financial packages, not under this crosswalk. |

- [Proposal](proposal.md)
- [Design](design.md)
- [Requirements](specs/mc03-native-recursive-proof/spec.md)
- [Tasks](tasks.md)
- [Program charter](../../MORIARTY-COMPLETION-PROGRAM.md)

## Current capability reconciliation — 2026-10-14

Historical scoped local native results remain evidence for their exact predicates. Full two-step native recursive financial history remains open. Current native authority is zero; no command below authorizes a retry.

The [PR50 reconciliation](../consolidated-language-kernel/reconciliation-2026-10-14.md) controls current profile, task-status and workflow interpretation. Original numbered tasks, checked history and frozen reviewer contracts remain unchanged. New nonsecurity work follows Sol high / fresh non-author Muse high; all security work belongs to Opus.

## Report-informed acceptance refinement

The [September 9 refinement](../../ROADMAP-REFINEMENT-2026-09-09.md) applies through [SP01](../../sprints/sp01-financial-contract-and-execution-admission.md#report-informed-acceptance-refinement), [SP04](../../sprints/sp04-complete-native-verifier-component-feasibility.md#report-informed-acceptance-refinement), [SP06](../../sprints/sp06-real-recursive-financial-history.md#report-informed-acceptance-refinement). Retain every original numbered task and requirement; the [task crosswalk](../../sprints/package-task-map.json) names its closing sprint task. Research-derived changes require the scoped positive/negative evidence and requalification owned there. Planning status does not promote this package or replace its existing admission gates.

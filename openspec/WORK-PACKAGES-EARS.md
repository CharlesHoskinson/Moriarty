# EARS and OpenSpec work-package index

Status: A0 local intake and A1 plans accepted; A2–A7 completion contracts open, recorded 2026-09-05.
Existing implementation evidence remains separate from these specifications.

## Notation and preservation

EARS states a condition and a required system response.
OpenSpec packages contain proposals, designs, tasks, requirements, and scenarios.
The forms follow [the EARS author's guide](https://alistairmavin.com/ears/) and
[OpenSpec documentation](https://github.com/Fission-AI/OpenSpec/blob/main/docs/getting-started.md).
These format references do not establish Moriarty behavior.

Existing adopted S01 and S02 packages remain unchanged.
The new program package maps all fifteen sprints and all twenty-four release gates.
The Candidate A packages refine S02 without replacing its acceptance floor.

## Candidate A packages

| ID | OpenSpec change | Dependencies |
| --- | --- | --- |
| A0 | [Recover and accept the completed boundary unit](changes/s02-candidate-a-a0/README.md) | Pinned existing boundary |
| A1 | [Correct and adopt concrete lifecycle plans](changes/s02-candidate-a-a1/README.md) | A0 |
| A2 | [Complete actual installment authority lifecycle](changes/s02-candidate-a-a2/README.md) | A1 |
| A3 | [Complete actual swap authority lifecycle](changes/s02-candidate-a-a3/README.md) | A1 |
| A4 | [Export actual integrated records and independently compare](changes/s02-candidate-a-a4/README.md) | A2 A3 |
| A5 | [Obtain honest bounded model-checking evidence](changes/s02-candidate-a-a5/README.md) | A2 A3 |
| A6 | [Independent acceptance, Council and dossier](changes/s02-candidate-a-a6/README.md) | A0 A1 A2 A3 A4 A5 |
| A7 | [Commit, integrate and hand off without scope inflation](changes/s02-candidate-a-a7/README.md) | A6 |

## Global requirements

### GLOBAL-U01: ubiquitous

The Moriarty workstream SHALL preserve the pinned Core, source XML, and existing acceptance contracts.

### GLOBAL-E01: event-driven

WHEN evidence is submitted, the package validator SHALL recompute its source hashes and acceptance predicates.

### GLOBAL-W01: unwanted-behavior

IF required evidence is missing or inconclusive, THEN the package validator SHALL retain an open acceptance status.

### GLOBAL-S01: state-driven

WHILE a required review remains incomplete, the integration gate SHALL refuse package integration.

### GLOBAL-W02: unwanted-behavior

IF a semantic conflict appears, THEN the workstream SHALL record a disposition before changing the adopted contract.

## Program sprint packages

The [program OpenSpec change](changes/moriarty-roadmap-completion/README.md) maps every sprint.
Each sprint has a separate capability specification and task inventory.

| Sprint | Capability specification |
| --- | --- |
| S01: Terminology and intent theorem freeze | [program-s01](changes/moriarty-roadmap-completion/specs/program-s01/spec.md) |
| S02: Core and intent semantic alternatives | [program-s02](changes/moriarty-roadmap-completion/specs/program-s02/spec.md) |
| S03: Semantic motions | [program-s03](changes/moriarty-roadmap-completion/specs/program-s03/spec.md) |
| S04: Mechanized reference semantics | [program-s04](changes/moriarty-roadmap-completion/specs/program-s04/spec.md) |
| S05: Surface and certifying elaboration | [program-s05](changes/moriarty-roadmap-completion/specs/program-s05/spec.md) |
| S06: Compact compiler and translation validation | [program-s06](changes/moriarty-roadmap-completion/specs/program-s06/spec.md) |
| S07: Intent verifier and CAKE boundaries | [program-s07](changes/moriarty-roadmap-completion/specs/program-s07/spec.md) |
| S08: ERC compatibility profiles | [program-s08](changes/moriarty-roadmap-completion/specs/program-s08/spec.md) |
| S09: Proof-of-intent vertical slice | [program-s09](changes/moriarty-roadmap-completion/specs/program-s09/spec.md) |
| S10: Complete developer interface | [program-s10](changes/moriarty-roadmap-completion/specs/program-s10/spec.md) |
| S11: ACTUS source lock and conformance harness | [program-s11](changes/moriarty-roadmap-completion/specs/program-s11/spec.md) |
| S12: ACTUS typed packages and independent semantics | [program-s12](changes/moriarty-roadmap-completion/specs/program-s12/spec.md) |
| S13: ACTUS shared compilation and backend correspondence | [program-s13](changes/moriarty-roadmap-completion/specs/program-s13/spec.md) |
| S14: Adversarial conformance and audit | [program-s14](changes/moriarty-roadmap-completion/specs/program-s14/spec.md) |
| S15: Terminal decision | [program-s15](changes/moriarty-roadmap-completion/specs/program-s15/spec.md) |

## Release gates

[All G01–G24 EARS gate contracts](changes/moriarty-roadmap-completion/specs/release-gates/spec.md)
preserve the full XML criteria.

## Validation boundary

Run OpenSpec structural validation for each new change.
A successful structural check establishes document validity only.
Future package validators must execute the acceptance checks.
No implementation task becomes complete from document generation.

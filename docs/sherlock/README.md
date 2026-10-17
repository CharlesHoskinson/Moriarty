# Sherlock documentation

Sherlock is Moriarty’s universal interpreter and its handwritten Midnight realization. This corpus follows [Diátaxis](https://diataxis.fr/): tutorials teach through an exercise, how-to guides solve a task, reference describes interfaces, and explanation develops the design.

## Tutorial

- [First local observations](tutorials/first-local-observations.md)

## How-to guides

- [Compare complete observations](how-to/compare-complete-observations.md)
- [Find the first difference](how-to/find-first-difference.md)
- [Add a normal case](how-to/add-normal-case.md)
- [Check the stream realization](how-to/check-stream-realization.md)

## Reference

- [Components](reference/components.md)
- [Profiles and results](reference/profiles-results.md)
- [Host machine](reference/host-machine.md)
- [K-A host event grouping](reference/host-grouping.md)
- [Image layout](reference/image-layout.md)
- [Bridges and ledger](reference/bridges-ledger.md)
- [DoctorWatson](reference/doctorwatson.md)

## Explanation

- [Universal interpreter](explanation/universal-interpreter.md)
- [Observation boundaries](explanation/observation-boundaries.md)
- [Relations, witnesses and ledger](explanation/relations-witnesses-ledger.md)
- [Evidence and acceptance](explanation/evidence-and-acceptance.md)
- [Chunks and financial boundaries](explanation/chunks-and-financial-boundaries.md)
- [Decisions and lessons](explanation/decisions-and-lessons.md)

## Maintain the corpus during implementation

Every implementation batch identifies the affected tutorial, guide, reference and explanation pages. Update interfaces, examples and observed results alongside their owning code; if a category is unaffected, record that disposition in the batch result. Bind examples and claims to the source/profile/consumer actually checked. Validate relative links and named API anchors before publication. Preserve earlier results with their dates and qualifications.

The [OpenSpec](../../openspec/changes/sherlock-universal-interpreter-design/requirements.md), [correctness loop](../SHERLOCK-CORRECTNESS-LOOP.md) and [DoctorWatson local commands](../DOCTORWATSON-CI.md) retain their separate acceptance predicates. A proposed API, source implementation, ordinary host observation, formal result and native target observation each have their own status.

## Design and implementation

[Revision03](design.md), [OpenSpec/EARS adoption](../../openspec/changes/sherlock-completion-convergence/ADOPTION.md), [implementation plan](../SHERLOCK-IMPLEMENTATION-PLAN.md), [loop contract](implementation-loop.md) and [independent reviews](reviews/README.md) define the current work. [Actual stream results](../results/sherlock-stream-20261017/README.md) and [composed333 results](../results/sherlock-composed333-20261017/README.md) retain their exact observed scope.

At the October17 integration cut, [finite Quint validation](../results/sherlock-quint-design-20261017/README.md) records44 passing tests, five passing directed/sampled runs and38 reached witnesses. The independent result review closes that ordinary entry condition. Root subsequently armed the new full implementation goal (creation epoch1792260805); this dated observation does not attest that a worker is running when this page is read. Compiler implementation and native/target acceptance remain separate work.

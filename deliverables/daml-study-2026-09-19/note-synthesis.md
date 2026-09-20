---
title: "Daml synthesis and Moriarty requirements"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: explanation
tags: [moriarty, daml, research]
---

# Daml synthesis and Moriarty requirements

The bounded comparative study is complete: 1,427 current Canton sitemap pages and two archived comparator pages are captured, alongside pinned Finance documentation and selected Splice evidence. The three studies read selected evidence ranges, not the entire captured corpus. All 142 cited evidence references passed byte-hash verification. No Daml examples, proof builds or production transactions were executed.

Daml separates consistency, code conformance and authorization. Moriarty needs all three tied to the signed intention and actual ZKIRv3 execution. Immediate-parent authority must not become ambient authority; valid current fields must not manufacture an approved history. Knowing a contract or holding a document does not confer spending authority.

Conditional settlement has distinct request, acceptance, reservation, evidence and delivery states. A Pending result, completed iteration, off-ledger acknowledgement or preferred settlement date must retain its precise meaning. Cancellation must preserve state for late allocations. Netting must preserve gross consent and liabilities. Reassignment preparation is separate from an atomic transaction on one synchronizer.

Structural upgrade checks and interface matching do not imply semantic equivalence. Private lookup failure does not establish global absence. Canton privacy projections are not a Midnight ZK theorem; hosting and network observations need explicit treatment. Daml-LF's unrestricted recursion is not the total language profile proposed for Moriarty.

The security direction therefore requires legitimate origins, recursive preservation, complete effect accounting, statement binding and assumption-aware recovery. The kernel composes ZK, MPC and TEE evidence for the same authorized effect. Where an external chain accepts a bare threshold signature, threshold compromise can bypass an off-chain proof policy; the model must retain that trust assumption or require destination enforcement.

- [MPLR-024: Explicit settlement domains](../mplr/MPLR-024.md)
- [MPLR-025: Netting preserves gross economics](../mplr/MPLR-025.md)
- [MPLR-026: Behavioral contracts for settlement implementations](../mplr/MPLR-026.md)
- [MPLR-027: Authenticated origins and recursive compliance](../mplr/MPLR-027.md)
- [MPLR-028: Consent-preserving semantic evolution](../mplr/MPLR-028.md)
- [MPLR-029: Authenticated completeness of private state](../mplr/MPLR-029.md)
- [MPLR-030: Common evidence statement and federation trust](../mplr/MPLR-030.md)

See [security and PCD](security-provability.md), [kernel boundaries](kernel-boundaries.md), [reference](reference.md), and the [Simplicity comparison](../simplicity/index.md). These are research requirements. Concrete language syntax, proof systems, compiler correspondence and actual target acceptance remain future implementation obligations.


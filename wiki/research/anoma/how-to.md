---
title: "How to evaluate an Anoma feature for Moriarty"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: how-to
tags: [moriarty, anoma, pl-theory]
---

# How to evaluate an Anoma feature for Moriarty

## User clarification — native Midnight proofs

On 2026-09-19 the user confirmed that Moriarty must use Midnight's existing proof stack and does not need Lean. Lean is not a compiler, developer-tool, proving or deployment dependency. References to Juvix's Lean machinery describe comparative evidence only; they create no Moriarty Lean workstream.

Moriarty must encode signed intention, authority, financial conditions and residual duties into the constraints checked by Midnight's native stack through pinned ZKIRv3. The native cryptographic proof proves the encoded relation. Language and compiler correctness must ensure that this relation expresses the intended behavior. That responsibility remains without prescribing a separate theorem-prover backend. See the [product contract](../../../docs/MORIARTY-PRODUCT-CONTRACT.md) and existing [native architecture evidence](../../decisions/pcd-midnight-native-architecture.md).

Start from a signed financial behavior and explicit observables. Select one compatible version tuple: language profile, dependency closure, resource semantics, compiler, guest image, public statement, verifier and settlement adapter. Record each source and hash. Independently pulled HEADs are not a supported stack.

Locate the exact acceptance condition. Decide whether it establishes structural validity, authority, policy satisfaction, conservation, local atomicity or external settlement evidence. State all remaining assumptions and duties. Distinguish generated proof syntax, discharged proof obligations, execution receipts and recursive history guarantees.

Implement a useful positive case and single-property hostile mutations before importing the mechanism. Preserve permissionless developer and solver participation. Measure bounded costs and discharge the exact Midnight ZKIRv3 correspondence obligations. Record failures and abstentions. Reuse code only after component license and compatibility review; concept reuse does not imply executable portability.

[Research plan](../../../deliverables/anoma-study-2026-09-19/RESEARCH-PLAN.md) · [Tutorial](tutorial.md) · [Index](index.md).


---
title: "Assess a Moriarty certified primitive"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: how-to
tags: [moriarty, simplicity, research]
---

# Check a proposed jet-like primitive

1. Pin its typed reference definition, semantic version, public-input relation and allowed failure behavior.
2. State preconditions and prove them at every call site; include representation, width, range and aliasing assumptions.
3. Compare reference execution, host implementation and actual ZKIRv3 lowering. Prove correspondence or validate artifacts with a sound checker.
4. Bind primitive identity and the proof relation into the program/intent commitment. Different commitments are not interchangeable merely because sample outputs agree.
5. Check cost under a pinned model. Distinguish logical work, native execution cost, circuit size and cumulative workflow reserves.
6. Test hostile witnesses, boundary arithmetic, invalid signatures, ignored conditions, stale versions and unsupported substitutions. Tests complement rather than discharge the general proof obligation.
7. Compose local guarantees with predecessor PCD and actual ledger resource consumption. Record external assumptions and privacy leakage separately.

[Source and coverage reference](reference.md) records the acquired corpus. No target compiler or proof implementation was changed in this study.

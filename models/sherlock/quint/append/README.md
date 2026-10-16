# Finite ValueSlots append model

The model represents EXT/COPY append behavior with two holders, four abstract references and capacities 7/8/9. The eight normal tests use eleven literal arenas. The production computation never consumes the independent materialized sequence oracle. Capacity refusal preserves the arena and remains separate from a machine outcome.

The exact published model/test bytes were used in the [captured checks](../../machine-circuit/append-model-review-20261016/README.md): model-root typecheck, all eight named tests, and a TypeScript seeded simulation configured for 32 samples of eight steps. All reported trace lengths were nine states and no violation was reported. The sample count is inferred from pinned consumer control flow and printed timing/rate, rather than printed directly.

The original source comment retains its authoring-time UNRUN label. The later immutable captures and independent result review govern the observed result. Checks ran at the recorded runtime paths; no check ran in this publication checkout.

The invariant reuses selector/required-length helpers and the first-unavailable formula. Literal expectations supply separate checks for those named cases. Some appended Constant(0) values equal padding. This is finite QA, without exhaustive or full-machine correspondence. No A177/A181, StepInv, circuit, financial, proof, ledger or Midnight stagenet acceptance follows.

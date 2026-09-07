# Moriarty overnight checklist — 2026-09-07

Current result: an experimental bounded financial DSL runs loan and swap simulations from actual authoring source and generates restricted Compact kernels. The implementation remains isolated and awaits final independent result review. This is not a completed proof-carrying financial language.

## Accomplished

- [x] Freeze the first authoring grammar, nominal types, checked UInt128 arithmetic, rounding, finite bounds, and transition semantics; obtain scoped independent source reviews.
- [x] Implement source parser, type checker, canonical encodings, source maps, deterministic lowering and local evaluator.
- [x] Run source-authored loan accrue/settle and swap/close demos, comparing complete local state/effects with retained reference traces.
- [x] Generate and compile restricted Compact kernels; compare their numeric state and effect operands with all four local transitions. This does not transfer assets or verify PCD.
- [x] Correct exact bounds registration, policy declaration ordering, source diagnostics and inert runtime admission; add authority, obligation capacity and result-byte boundary rejection controls.
- [x] Recheck current implementation this morning: strict build, 78 language tests and developer demos pass. Root receipts are in the review-corrections worktree; final independent result review remains open.
- [x] Design a smaller checked native recursive-state encoding. Preserve original row-exhaustion failure. Both historical named reviewers approved the scoped successor source experiment; no proof was run or produced.
- [x] Inspect actual Preview financial ledger APIs and draft the six-submission loan/swap test with complete effect/finality comparison requirements.
- [x] Identify native-to-ledger verifier incompatibilities requiring a checked wrapper. Keep those gaps explicit.
- [x] Retain complete 32 ACTUS / 72 DeFi target traceability; inventory is not conformance.

Existing network foundation, established earlier: Preview test wallet funded and hello-world deployment/call finalized. Docker node/indexer/proof server passed health checks. Those results are not financial settlement or Moriarty proof acceptance.

## Still required

- [ ] Final MC01 implementation approval and integration; resolve remaining review/admission observations.
- [ ] Actual Preview loan/swap financial settlement and full finalized-effect comparison.
- [ ] Actual native recursive proof, retained verification, and remaining proof controls.
- [ ] Compiler-to-ledger correspondence and durable authority/replay protection.
- [ ] Actual acceptance of mandatory contract, intent, transition and predecessor-history proofs.
- [ ] Private witness handoff and proved split/join composition.
- [ ] Complete ACTUS/DeFi and held-out behavioral conformance, then final developer/release evidence.

## Current reviewer change

Future external reviews use exact claude-opus-5 and fresh gpt-6-astra under raw/assignments/moriarty-opus-review-2026-09-07.md. Both access probes succeeded this morning. Historical Fable reviews are preserved with their original scope. Native admission fields still require an explicit successor migration before execution. Provider usage reset was not performed or verified by the assistant.

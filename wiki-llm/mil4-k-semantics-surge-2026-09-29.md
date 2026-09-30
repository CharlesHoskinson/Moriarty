# MIL/4 K semantics surge: recommendation

**Status:** specified-only research implementation, begun 2026-09-29. The [K composition root](../deliverables/mil4-k-surge-2026-09-29/mil4-proposal.k), [coverage and recommendation](../deliverables/mil4-k-surge-2026-09-29/RESULT.md), [nine-seat audit reconciliation](../deliverables/mil4-k-surge-2026-09-29/AUDIT-RECONCILIATION.md), raw model receipts and compiler receipt are the source record. This note does not adopt MIL/4 or establish a native or ledger result.

## Recommendation

Keep the six-judgment K escrow relation as a bounded research slice, and keep the eight DeFi family arithmetic rules as projections. The separate `m4AdmitFamily` rule must reject every family until an authenticated state/effect adapter binds its quantities, canonical lines, signed authority, predecessor, proof public inputs and observed effects. This is a concrete boundary decision: the kernel can route and observe, but a kernel receipt cannot fill the language's financial-effect judgment.

Freeze M4-C1–C5 in order: canonical `/4` bytes and nominal types; per-primitive fee/rounding and whole-unit reserve effects; certified Ω widths and exact quotient checks; complete authenticated footprints; then a signature/evidence/native public-input contract. The [result matrix](../deliverables/mil4-k-surge-2026-09-29/RESULT.md#recommended-decisions-and-implementation-order) names the discriminators. Start with S0 funded repayment and local escrow, then AMM after its complete pool-state adapter. Bridge settlement remains specified-only until partial claim conservation and qualified terminal nonreceipt exist.

## Evidence and dissent

Three GPT-6 Sol seats authored separate modules. Three Opus 5.5 reviews and three Grok 4.7 audits returned final artifacts; exact requested/returned identities and failed first attempts are in the [audit reconciliation](../deliverables/mil4-k-surge-2026-09-29/AUDIT-RECONCILIATION.md). The external reviews found concrete overacceptance cases: a caller-selected fee draining partial escrow, a false typed outcome, arbitrary successor heads, malformed rounding, caller-supplied family effects, and foreign receipt claims with no verified conservation. The K modules were revised for local defects and fail closed at the family admission boundary. Wider episode, byte, authority, native and bridge findings remain open rather than being treated as disproved by a clean compile.

The final [compiler receipt](../deliverables/mil4-k-surge-2026-09-29/compile.receipt.json) records `kompile` v7.1.337 exit 0, 30 unused-variable warnings and no errors. No K execution, behavioral test, source/Core differential check, native proof or Preview transaction was run. The guarded SP01.6 lane remained blocked; this surge did not dispatch it.

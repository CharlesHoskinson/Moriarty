# About chunks, projection and financial boundaries

Draft, 2026-10-17. Final03 specifies the planned persistent boundary contracts below; production persistent storage/import remains missing.

A chunk groups a contiguous portion of machine execution. It changes where evidence is divided, not which language transition occurs. A cut before an inner call returns must preserve the caller environment, pending operation and remaining work. A cut before terminal publication must preserve that publication as pending. Restarting either case from a convenient source expression would describe a different computation.

Within one Step representation, the next record consumes the exact previous output. At a persistent boundary, the proposed projection may drop unreachable physical rows or renumber references while preserving decoded semantic meaning. That is why a correct resume needs both the original Step image and an explicitly successful canonical projection, with program/profile/layout/bounds, complete frames and helper cursors, pending result, supported origins and work/allocation counters. Physical occupancy and logical allocation are different quantities. Projection refusal is a tooling failure, not application exhaustion.

The existing public A codec supports trusted diagnostic encoding/decoding/materialization. It is not an arbitrary Machine-state importer or canonical native wire ABI. Test-only image/checkpoint/slice helpers and repeated `visit_steps` calls on one in-memory Machine do not establish persistent export/import. The later actual stream attempt includes eight passing in-memory observation tests, including a recursive cut preserving caller environments and a cut before zero-fuel publication. Its 216-run comparison adds host observation evidence while leaving persistent boundary storage/import unimplemented. Final03 retains the existing Opus-owned StepInv and canonical-boundary requirements without claiming that those obligations have been discharged.

A financial commit is a different boundary. It records the language/ledger transition that makes effects and duties persistent. Await, recovery and later-segment failure have their own phase meanings. A technical proof cut creates no payment, discharge or fee policy. A pure return also supplies no evidence of financial settlement.

The existing Core/2 load/start/resume behavior is the starting point for the financial owner's mapping; universal Commit/Yield/Await opcodes have not been selected by this explanation. The final03/R18 map distinguishes committed snapshot from tentative segment, ordered effects including supply changes, gross debit/fees/net receipts, reserves/dust, liability identity/value, pending captures/observations and residual duties. Earlier committed prefixes must retain their specified meaning when a later local segment fails before commit. Guaranteed/fallible ledger phases remain separate from local semantic outcomes.

These requirements matter even when the final balance happens to match. A refund can conceal gross debit, fees can change net receipts, and an omitted pending duty can change the next segment. Complete prefix observations explain how the outcome arose. The independent proposed financial oracle currently omits the full PassT result; a comparison must remain B/F-only until that result has its own expectation.

See [image/layout boundaries](../reference/image-layout.md), [bridge/ledger consumers](../reference/bridges-ledger.md) and [complete-observation comparison](../how-to/compare-complete-observations.md). [Observation boundaries](observation-boundaries.md) explains the narrower in-memory grouping API.

## The current Core2 observation boundary

The public [financial observation adapter](../reference/financial-observation-adapter.md)
now retains the actual Core/2 program, captured invocation and complete returned
stage result, with both frames, ordered steps/effects, full state, duties and
logical work. Its account aggregates describe one published segment. A false
await has no segment; a successful segment with no effects remains present.
Neither aggregate equality nor a technical chunk creates a financial commit.

The [captured positive result](../../results/sherlock-financial-observation-20261018/README.md)
observes the first paid prefix, an unchanged false-await continuation and a true
resume that completes. This is scoped Core/2 producer/adapter evidence. At that historical cut the
financial producer/adapter relationship remained unobserved and UIREQ-017 stayed
open; the owning path and final review below add the new source/result evidence.
UIREQ-018 lowering and UIREQ-019 later local failure remain open. These observations do not fill
the separate PassT or native/ledger evidence gaps above.

`StageRejection` returns error and work counters without a discarded tentative
snapshot/effect carrier or replacement continuation. The adapter preserves that
absence. A caller's retained pre-call continuation remains input; it cannot be
presented as a returned rollback result. The [how-to](../how-to/observe-stage-financial-result.md)
shows capture at the existing stage call sites.


## Same-call ownership is a narrower connection than settlement

The [owning financial producer candidate](../../results/sherlock-owning-financial-producer-20261018/README.md)
connects one existing Core/2 stage invocation to a complete observation: private
record fields keep the actual program, inputs/flags and returned Result together.
Its shared publisher serializes that record without another evaluator call.
The exact positive start/false/true sequence passed full in-memory comparisons;
no persisted actual financial report or enclosing CLI execution is claimed.
This advances the producer/adapter relationship beyond arbitrary diagnostic
pairing while retaining local-reference-stage, unauthenticated and untracked
consumption labels. It creates no payment policy, authenticated history, universal
await lowering or ledger transition. The final actual-result/interface review recommends whole017 closure within this
declared domain; Root owns semantic adoption. Universal lowering and later local
failure remain separate requirements.

Root has now [adopted whole UIREQ-017](../../results/sherlock-owning-financial-producer-20261018/ROOT-FINANCIAL-PRODUCER-ADOPTION.json). The [current requirement report](../../results/sherlock-owning-financial-producer-20261018/ROOT-CURRENTNESS-READBACK.json) confirms fourteen current original closures. The source-only enclosing CLI and separate lowering, failure, authentication, proof and settlement qualifications above remain.

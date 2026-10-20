# Additive public typed action inputs

Adopted follow-on design, 2026-10-20; implementation candidate only, all new
ordinary checks UNRUN. This extends B04/SCON-003/004/019/020/021 and leaves
SCON-025 and FIRST-COMPILER-BATCH.md unchanged. B04 remains unchecked.

Predicate: public_typed_action_input_binding holds for a claimed supported case
only when the real public selected handler binds exact typed values in source
declaration order, retains original parameter/body structure and the complete
HostOnly artifact, and an explicit observation has actual admission plus Halted
with a complete typed value matching a separately prefrozen expectation.

| Clause | Requirement |
| --- | --- |
| INP1 | When explicit supported Core action inputs are supplied, bind every exact named typed value through the existing prepare/run consumer. |
| INP2 | When the flag is absent, preserve legacy selection, empty-map narrowing and artifact/error bytes. Explicit {} remains a distinct mode. |
| INP3 | On invalid profile/action/entry/input selection, preserve original selection priority, then refuse non-Core input scope before observe/config/IO. |
| INP4 | On input read/strict JSON/Value failure, retain the original inputs-stage payload and do not publish success. |
| INP5 | On existing input-policy refusal, retain original guard order, extents and complete payload. |
| INP6 | On compile-only success, retain source parameters/Vars/lets/operators/entry Call and full admitted projection without owner run. |
| INP7 | On observe, run the one existing prepared/admitted pair and preserve Halted, rejection and exhaustion distinctly. |
| INP8 | Emit artifact/1 with optional binding values then raw-file digest, omitted exactly for legacy None; preserve all existing projections. |
| INP9 | On post-run output refusal, retain actual evaluation, completion and forwarded-event metadata. |
| INP10 | Preserve unsupported source/effect/financial/native/proof boundaries and permissionless public use. |
| INP11 | Freeze separate source-denotational typed expectations before producer implementation; author assertions are not independent. |
| INP12 | Retain both complete actual artifacts/commands/streams/pins durably before independent expected-body comparison. |

The CLI option is --execution-inputs FILE, requires execution-profile and action,
and conflicts with entry. The map type is BTreeMap<String, mori_core::Value>.
Reuse existing codecs, source reader, guards and consumer unchanged. The exact
scope message and IO/decode order are in the
[reference](../../../docs/sherlock/reference/typed-action-inputs.md).

Two contrasting cases share one new source. Their exact input/config/target
specimens were frozen before implementation and delivered independently without
producer code or author predictions. The independent expectation is conditional
on eventual admission and Halted and covers only complete typed terminal values.
No independent full-state/work/ID/trace or native/financial oracle is claimed.

Tests are authored, not executed: real Args/selected handler/publication, Core
API composition, source structure and declaration order, input digests and
visible values, legacy omissions, codec and guard payloads, selection/file/
decode priority, refusal metadata and actual rejection/exhaustion controls.
A new full normal Core qualification, focused tests and two retained captures
need separate reviewed Root selection. No old rlib qualifies changed Core.

General B04 coverage and H1/formal/native/proof/target acceptance remain open.

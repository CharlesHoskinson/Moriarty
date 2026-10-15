# Optimization 04: compact frames with faithful environment restoration

Status: nonsecurity, specified-only. **P1:** persistent environment roots and compact frame payloads; **P2:** vector-slot lowering; **P3:** liveness/tail-call compression after complete-state projection is explicit. All host, allocation, circuit-row, native, proof-size and preprocessing costs remain unmeasured. No execution, benchmark, theorem or security judgment is supplied.

Baseline: Candidate A freeze SHA256 `57d830f46fee57c56ed8b854d21f5dd7a695d2977d061b1ef6905eaecd7de30e`; K `model/k/main.k` SHA256 `b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41`. UIREQ-024/026 apply. This proposal changes representation only; it adds no closures, nominal datatype extensions or semantic rules per input program.

**Source facts.** K lines 48–55 already defunctionalize nine frame variants; seven carry environments. Lines 135–143 charge each push and enforce logical depth against stackLimit. Lines 145–152 replace Let with Scope and Left with Right without changing depth. Argument, Return and Scope rules restore saved maps at 177–198. Function entry installs only parameter bindings at 204–209. Official K LAMBDA++ explicitly saves/restores environment with computation [K1]; its closure/recursion constructs are source context, not features proposed for Sherlock.

**Recommendation/inference.** Use immutable environment versions, initially persistent roots rather than copying whole maps into each frame. A root names the exact binding map, including shadowed bindings recoverable by saved scopes. Proposed representation interfaces:

```text
EnvRoot; read(root,sourceId); extend(root,sourceId,valueRef)
decodeEnv(root) -> source binding map
FrameDesc = {tag, payloadRef}
decodeFrame(desc,program,envVersions,payloads) -> A frame
```

Root sharing is an implementation option; the source Map may already share structure, so a new speedup is not assumed. Capture records retain only fields present in their source variant:

| Source variant | Compact payload |
|---|---|
| Let / Scope | binder, bodyCodeRef, root / root |
| Left / Right | operator, rightCodeRef, root / operator, valueRef |
| Unary / If | operator / thenCodeRef, elseCodeRef, root |
| Match / Args | casesRef, root / functionId, collectedArgsRef, remainingArgsRef, root |
| Return | resultType, callerRoot |

Code references address immutable input-program syntax. They do not replace the function table with bespoke evaluator rules. A descriptor arena must represent every stack admitted by the existing frame cap. Smaller per-variant payload pools cannot introduce an earlier capacity outcome; absent a capacity argument, retain full-profile storage and claim no footprint reduction.

For vector slots, prepare `slotLayout(functionId,binderOccurrence)` and `resolve(pc,sourceId)` as program data. Binder occurrences distinguish shadowed source IDs. Preserve source-order arguments and defer call binding/return checks to their original phases. Slot snapshots must be immutable/versioned or retain enough restoration data; overwriting caller slots in place loses the saved environment semantics. Dense slots require declared per-function/local capacities and preparation work, so new bounds are explicit profile restrictions, not silently universal A preservation.

**Liveness limit.** Saving only variables read by the continuation can preserve final values yet discard a complete saved map. A-preserving compression therefore retains a reconstructible environment root even when execution carries a smaller live-slot vector. Dropping dead binding data requires a weaker observation projection and cannot be advertised as full-state preservation under this proposal. Freeze continuation-specific liveness over code/frames separately; no liveness result is asserted.

**Tail-call limit.** Ordinary tail-call elimination removes Return frames and changes A's frame-cap/exhaustion and work outcomes. Only physical activation reuse is proposed: retain logical push/depth, result-type check, caller-root restoration and one return-pop charge per original frame. A compressed return-run still exposes each logical obligation on decoding and unwinds one at a time. Its depth includes virtual frames when checking every push. If the required roots/types differ, retain their sequence; a counter alone is insufficient. Otherwise defer this optimization.

**Preservation predicate, not a proof.** At every corresponding logical boundary, decoding yields A's control, ordered frames, complete environment, values/result, rejection code, depth, cap, consumed/remaining fuel and evaluation order. Every old charged transition remains one charge; administrative representation steps add none. Push attempts and terminal publication retain A's exact rules. Pure A has no financial effects/duties/suspension boundaries; future extensions must add these observations explicitly.

**Normal witness plans.** Retain all eight frozen fixtures, especially pending multiplication in factorial3=6, pending sibling traversal in tree45=9, and caller-local restoration=12. Add independently specified nested shadowing `(let x=10 in (let x=1 in x)+x)=11`, sibling calls producing 2+3=5, and a tail-recursive countdown returning0. Before any future execution, hand-freeze their complete frame/root projections and logical work/cap observations; current fixture fuel totals remain unfrozen.

UIREQ-026 requires the same mapping in Lean semantic/finite-encoding definitions and the bounded Clean step interface. Actual integration belongs in existing private Fenrir; this public proposal uses only public baseline sources. Counterargument: root indirection, reversible snapshots and virtual return obligations may save little, while vector preparation adds complexity. Measure each cost plane later; retain the simpler A representation where no benefit appears.

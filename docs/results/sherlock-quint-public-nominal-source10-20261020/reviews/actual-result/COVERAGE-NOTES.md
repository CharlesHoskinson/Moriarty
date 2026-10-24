> Public-only derivative. Entire private Clean detailed coverage subsection1.3 omitted. The original combined review remains privately retained, SHA256 `1d14615cfc3c15852af9323b6593751408625ff63a7ede3ad01c8bc10bd101e5`. K/Quint sections and permitted identity/result/scope metadata are retained. This is a publication omission, not a new review or verdict.

# Tested Coverage & Remaining Correspondence Gaps (`review03`)

## 1. Exact Tested Coverage Per Counterpart Lane

### 1.1 K `source09` (`SHERLOCK-PUBLIC-NOMINAL-SOURCE09-LAWS`, `actual-laws4-01`)
- **`npLawHeldOutNotFalse` (`true`)**:
  - Executes `npRunReference(npHeldOutNotProgram(), npNoTypeRows(), npNoCtorRows(), 512)` on `ngNot(3000, ngLiteral(3001, ngBool(false)))` and verifies the projected value is `npBool(true)`.
- **`npLawMapQueue` (`true`)**:
  - Executes `npRunReference(npMapQueueProgram(), npMapQueueTypes(), npMapQueueCtors(), 512)` on the K translation of `map-queue.mori` (`entry pack(Push(3, Push(8, Empty())))`) and verifies the complete projected tree `Envelope<Queue<UInt>, Bool>::Wrap(z_items: Queue<UInt>::Push(z_item: 5, a_tail: Queue<UInt>::Push(z_item: 10, a_tail: Queue<UInt>::Empty())), a_marker: true)`.
- **`npLawMirrorArbor` (`true`)**:
  - Executes `npRunReference(npMirrorArborProgram(), npMirrorArborTypes(), npMirrorArborCtors(), 512)` on the K translation of `mirror-arbor.mori` (`entry stamp(Fork(Tip(true), Fork(Tip(false), Tip(true))))`) and verifies the complete projected tree `Stamp<Arbor<Bool>, UInt>::Seal(z_tree: Arbor<Bool>::Fork(z_left: Arbor<Bool>::Fork(z_left: Arbor<Bool>::Tip(z_value: false), a_right: Arbor<Bool>::Tip(z_value: true)), a_right: Arbor<Bool>::Tip(z_value: false)), a_code: 41)`.
- **`npLawNotCompleteSavedState` (`true`)**:
  - Verifies one exact `ngStep` of `ngReturn(ngBool(false))` into `ngNotArgument(17, 1017, (99 |-> ngUInt(8)))` while live `Rho = (99 |-> ngBool(true))`, checking the entire resulting `NGState` (control `ngReturn(ngBool(true))`, restored `Saved = (99 |-> ngUInt(8))`, popped frame stack `ngFrame(ngArmScope(3, 1003, 0, .Map), ngNoFrames())`, and event log with `ngNotReturned(17, 1017, ngBool(false), ngBool(true), (99 |-> ngUInt(8)))`).

### 1.2 Quint `source10` (`nominalPublicSimulation`, `nominalPublicSourceTest`, `nominalPublicSourceRuntime`)
- **Frontend Typecheck (`01-newNominalSourceRuntimeFrontend`)**:
  - Typechecks `nominalPublicSourceRuntime.qnt` and its imported module closure (`nominalPublicSimulation.qnt`, `nominalPublicSourceInputs.qnt`) with 0 warnings/errors.
- **Four Exact Unit Tests (`02-fourExactNominalSourceCaseTests`)**:
  - `q10MapQueueTypedReturnAndOrderTest`: verifies `Done(q10MapExpected)`, `q10CompleteAccounting` across all states, `selectedArms == [1, 1, 0]`, `4` call entries, `7` constructor builds, `10` field slots, `2` `PrimitiveReturn` events, and non-lexical `z_item, a_tail` / `z_items, a_marker` field order.
  - `q10MirrorArborTypedShapeAndOrderTest`: verifies `Done(q10MirrorExpected)`, `q10CompleteAccounting` across all states, `selectedArms == [1, 1, 0, 0, 0]`, `6` call entries, `11` constructor builds, `16` field slots, `3` `PrimitiveReturn` events, and non-lexical `z_left, a_right` / `z_tree, a_code` field order.
  - `q10SameTypeFieldSwapIsDistinguishedTest`: proves that swapping the two same-type (`[30, 30]`) children of the outer reflected `Fork` satisfies `validValue` yet produces an unequal preorder token sequence (`swappedArg != treeArg`).
  - `q10ExistingCallBuildPathAndPreservationTest`: verifies `program` invariance across every state of both traces, final empty `environment` and `frames`, final `TerminalPublication`, `3` `UnaryEvent` steps in `MirrorArbor`, and `4`/`6` `CallEnter` steps.
- **Complete 257-Snapshot `MirrorArbor` ITF Trace (`03-oneMirrorArborOperationalWitness`)**:
  - Verifies `allInv` (`accountingInv and valueInv`) across all 257 states and witnesses `mirroredNominalPublished`.
  - Reaches `Done(q10MirrorExpected)` at state index `108` (`108` logical charged transitions, `maxDepth = 12`, `helperWork = 0`), followed by `148` terminal stutter steps (`states[109..256]`).


> [Detailed private Clean section omitted from this public derivative; see PUBLIC-ONLY-DERIVATION-AND-EXCLUSIONS.json.]

---

## 2. Remaining Open Correspondence Gaps

1. **Compiler Canonical Identity & Monomorphization Gap**:
   - All three lanes use local declared numeric IDs (`4, 5` in K; `10, 20, 30, 40` in Quint; `100, 101, 200, 201` in Clean) rather than compiler-computed `BLAKE2b-256` `TypeIdC` / `VariantIdC` / `FieldIdC` hashes or a verified generic-instantiation lowering pass from `.mori` syntax.
2. **Clean Step-Machine `Bool-not` & Operational Evaluation Bridge Gap**:
   - `Clean.Sherlock.NominalSemantics.Expr` does not yet include a machine-level Boolean-not expression constructor, and `QueueBump` / `ArborReflect` are inductive source-clause relations over `NominalValue.Value` rather than step-machine `NominalSemantics.Evaluation` theorems.
3. **Quint Profile Parameters vs. Core Clocks & Standalone `MapQueue` Trace**:
   - Quint `source10` uses `MAX_SCALAR = 255` (not `2^128 - 1`), `helperWork = 0`, `TERMINAL_PUBLICATION_CHARGE = 1`, and `108` transitions + `148` stutters under a `256`-step bound. `MapQueue` is verified in `quint test` (`02`), with no standalone `MapQueue` ITF file exported in `03`.
4. **No Cross-Model Refinement, Rust Producer Comparison, Native Proof, or EARS Closure**:
   - All expectations are same-author source derivations (`Sourceauthorsnotvotes`); no comparison against actual Rust compiler/interpreter execution outputs, cross-layer `StepInv` refinement, ZKIRv3/halo2 native proving, stagenet settlement, or EARS requirement closure is established by these results.

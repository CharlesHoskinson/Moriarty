# Fresh Independent Non-Author Formal Math/Source/API Review — Quint Lane (`source10`)

## 1. Reviewer Identity, Routing & Execution Attestation
- **Role**: Fresh independent non-author formal math/source/API reviewer (under the standing October 16 Opus quota fallback to Gemini 4 Argon at high effort via AGY).
- **Requested Identity / Effort**: Gemini 4 Argon (High) via AGY (`IdentityrequestedHIGH/backendunattested`: model selection requested at High effort in AGY session; underlying provider backend runtime remains unattested by host cryptographic receipt).
- **Execution Discipline**: Read-only inspection only. Zero Node, Quint, `rustc`, Cargo, K, Lean, SDK, native, proof, provider, network, or Git commands were executed. No shared workspace files were written.
- **Independence Qualification**: `SOURCE-FIRST-MANUAL-EXPECTATIONS.json` and `PUBLIC-TYPE-FIELD-AND-OBSERVATION-CONTRACT.json` were frozen by the Quint lane author from `map-queue.mori` and `mirror-arbor.mori` before extending the Quint model, without reading new Rust test assertions or actual compiler outputs. Same-author pre-code expectations are independent of the Rust producer outputs, not a fresh blinded oracle vote (`Eachauthornotavote`).

---

## 2. Inspected Candidate & Pin Inventory
- **Handoff**: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-simulate-source10/HANDOFF.json` (`7151` bytes, SHA256 `d70d4a5b2ddcb07a6155166333352662d4ee25b049f34964f21629417e2d9118`)
- **Full Review Prompt**: `FULL-SOURCE-API-REVIEW-PROMPT.txt` (`117215` bytes, SHA256 `fcf2f5a6d7a1aca97ff1bf146b378deb4f1c1b49fbf44963ea2d3d9899838259`) — read in full.
- **Base Preimage**:
  - `preimages/nominalConstruction.qnt` (`18761` bytes, SHA256 `77a487c5b7ce84569540faa0a780991d67cf8217d3a1d68a81916426d8943008`)
- **Candidate Five-Module Afterimage (UNTYPECHECKED / UNRUN)**:
  1. `afterimages/nominalPublicSimulation.qnt` (`19394` bytes, SHA256 `40459df5715b3b13417aaf32b0a77789abbc34cdf6ff8e14e97deadc67e162c5`)
  2. `afterimages/nominalPublicSourceExpected.qnt` (`1080` bytes, SHA256 `368d29bde4c1515a4fd06a4706d736d6d805537ae914af4527a57a6fda57ffa6`)
  3. `afterimages/nominalPublicSourceInputs.qnt` (`4137` bytes, SHA256 `b4acc537d225a0ced552f5490a52cfe539741684f16bf53c2ec21698a3bebc93`)
  4. `afterimages/nominalPublicSourceRuntime.qnt` (`434` bytes, SHA256 `109d92bb5b24c4547ba9a8c26bb712476f8809fc5b59636457e6b51ffe28fb0c`)
  5. `afterimages/nominalPublicSourceTest.qnt` (`4412` bytes, SHA256 `d67008ae881bb94c2c146462e917ba2c452a51e166035aa3e2a7a647d30cdc40`)
- **Allowed Source Fixtures**:
  - `fixtures/map-queue.mori` (`638` bytes, SHA256 `b8b30eb7b66280065092d4e8590c8c798b640e64b3c748153a997107f8c8010d`)
  - `fixtures/mirror-arbor.mori` (`735` bytes, SHA256 `504fd3f85c354b778f4f5c4d49cd75155c21a62d087c24530f9a2fa51e93bfa9`)

---

## 3. Independent Math, Source & API Verification

### 3.1 Operational `BooleanNot` Extension (`nominalPublicSimulation.qnt`)
- **Exact Diff against `nominalConstruction.qnt`**:
  - Adds `| BooleanNot(int)` to `Expression`, `| NotFrame` to `Frame`, and `| UnaryEvent` to `Event`.
  - In `evaluate(s, id)`: `| BooleanNot(operand) => push(marked, NotFrame, operand, s.environment, UnaryEvent)`.
  - Adds `pure def booleanNot(s: Snapshot, value: Value): Snapshot`:
    - Maps `List(BoolToken(true))` to `emit(s, PrimitiveReturn, Return(List(BoolToken(false))))`.
    - Maps `List(BoolToken(false))` to `emit(s, PrimitiveReturn, Return(List(BoolToken(true))))`.
    - Maps any non-Bool `Value` to `stopDomain(s)` (`outcome: OutsideDomain, control: Stopped, event: DomainStop`) as finite model partiality (not a production guard/refusal claim).
  - In `returnValue(s, value)`: `| NotFrame => booleanNot(popped, value)`.
- **Environment & Frame Preservation**:
  - Like `RightFrame` in the base model, `NotFrame` relies on the invariant that every subexpression evaluation in `nominalPublicSimulation` (`Let`, `Binary`, `If`, `Constructor`, `Call`, `Match`) restores its saved `Environment` upon returning (`ScopeFrame`, `ReturnFrame`, `ConstructorFields`). Thus `popped.environment` at `NotFrame` is guaranteed to equal the caller's environment when `BooleanNot` was pushed.
- **Exhaustiveness & Purity**:
  - All `match` expressions over `Expression`, `Frame`, `Control`, `Token`, and `Pattern` are exhaustive and pure. All 17 `Snapshot` fields, `FRAME_LIMIT = 32`, `MAX_SCALAR = 255`, `helperWork = 0`, and `accountingInv` / `valueInv` are preserved unchanged.

### 3.2 Independent Verification of Source Projections, Returns, Field Order & Discriminators
1. **`q10MapProgram` (`map-queue.mori`)**:
   - Finite monomorphic TypeId bindings: `0 = UInt`, `1 = Bool`, `10 = Queue<UInt>` (args `["UInt"]`), `20 = Envelope<Queue<UInt>, Bool>` (args `["Queue<UInt>", "Bool"]`).
   - Layouts: `(10, 0) -> []` (`Empty`), `(10, 1) -> [0, 10]` (`Push(z_item, a_tail)`), `(20, 0) -> [10, 1]` (`Wrap(z_items, a_marker)`).
   - Execution trace through `initial(q10MapProgram, 256)` and `advance`:
     - Builds input `Empty` (build 0, 0 fields), `Push(8, Empty)` (build 1, 2 fields), `Push(3, Push(8, Empty))` (build 2, 2 fields).
     - Enters `pack` (call 1), then `bump(Push(3, Push(8, Empty)))` (call 2, selects arm `1`), computes `3 + 2 = 5` (primitive return 1), enters `bump(Push(8, Empty))` (call 3, selects arm `1`), computes `8 + 2 = 10` (primitive return 2), enters `bump(Empty)` (call 4, selects arm `0`).
     - Builds result `Empty` (build 3, 0 fields), `Push(10, Empty)` (build 4, 2 fields, head `uint(10)`), `Push(5, Push(10, Empty))` (build 5, 2 fields, head `uint(5)`), and `Wrap(Push(5, Push(10, Empty)), true)` (build 6, 2 fields).
     - Totals: `selectedArms = List(1, 1, 0)`, `enteredArguments.length() = 4`, `constructorNodes = 7`, `constructorFieldSlots = 10`, `PrimitiveReturn` events = `2`, completing in ~77 steps (within `256` fuel and `FRAME_LIMIT = 32`).
     - Final pretrained token list `q10MapExpected`:
       `[ConstructorToken(20,0,2), ConstructorToken(10,1,2), UIntToken(5), ConstructorToken(10,1,2), UIntToken(10), ConstructorToken(10,0,0), BoolToken(true)]`.

2. **`q10MirrorProgram` (`mirror-arbor.mori`)**:
   - Finite monomorphic TypeId bindings: `30 = Arbor<Bool>` (args `["Bool"]`), `40 = Stamp<Arbor<Bool>, UInt>` (args `["Arbor<Bool>", "UInt"]`).
   - Layouts: `(30, 0) -> [1]` (`Tip(z_value)`), `(30, 1) -> [30, 30]` (`Fork(z_left, a_right)`), `(40, 0) -> [30, 0]` (`Seal(z_tree, a_code)`).
   - Execution trace through `initial(q10MirrorProgram, 256)` and `advance`:
     - Builds input `Tip(true)` (build 0), `Tip(false)` (build 1), `Tip(true)` (build 2), inner `Fork(Tip(false), Tip(true))` (build 3), outer `Fork(Tip(true), innerFork)` (build 4).
     - Enters `stamp` (call 1), then `reflect(outerFork)` (call 2, arm `1`), which evaluates `reflect(right)` (expr `19` on var `303`) before `reflect(left)` (expr `20` on var `302`):
       - Call 3 (`reflect(innerFork)`, arm `1`) evaluates `reflect(Tip(true))` (call 4, arm `0`, `BooleanNot(true) -> false`, build 5 `Tip(false)`), then `reflect(Tip(false))` (call 5, arm `0`, `BooleanNot(false) -> true`, build 6 `Tip(true)`), and builds inner reflected `Fork(Tip(false), Tip(true))` (build 7).
       - Call 6 (`reflect(outer left Tip(true))`, arm `0`, `BooleanNot(true) -> false`, build 8 `Tip(false)`).
       - Builds outer reflected `Fork(Fork(Tip(false), Tip(true)), Tip(false))` (build 9), then `Seal(outerReflectedFork, 41)` (build 10).
     - Totals: `selectedArms = List(1, 1, 0, 0, 0)`, `enteredArguments.length() = 6`, `constructorNodes = 11`, `constructorFieldSlots = 16`, `UnaryEvent` count = `3`, `PrimitiveReturn` count = `3`, completing in ~117 steps (within `256` fuel and `FRAME_LIMIT = 32`).
     - Final token list `q10MirrorExpected`:
       `[ConstructorToken(40,0,2), ConstructorToken(30,1,2), ConstructorToken(30,1,2), ConstructorToken(30,0,1), BoolToken(false), ConstructorToken(30,0,1), BoolToken(true), ConstructorToken(30,0,1), BoolToken(false), UIntToken(41)]`.

3. **Falsifiability & Discriminators Across All Four Tests (`nominalPublicSourceTest.qnt`)**:
   - `q10MapQueueTypedReturnAndOrderTest` and `q10MirrorArborTypedShapeAndOrderTest` check full token-sequence return equality, `q10CompleteAccounting` across every trace snapshot, arm order, call count, build count, field-slot count, primitive event count, type-argument/field-name bindings, and intermediate `builds` entries.
   - `q10SameTypeFieldSwapIsDistinguishedTest` specifically addresses the fact that `Arbor::Fork` has two children of identical type `30` (`[30, 30]`): it constructs `swappedArg` by swapping `childrenArg.nth(0)` (`Fork(Tip(false), Tip(true))`) and `childrenArg.nth(1)` (`Tip(false)`), proves `validValue(q10MirrorProgram, swappedArg)` holds (so type/layout checking alone cannot distinguish them), and proves `swappedArg != treeArg`.
   - `q10ExistingCallBuildPathAndPreservationTest` verifies `program` invariance across all trace steps, empty terminal `environment` and `frames`, `TerminalPublication` terminal event, and exact `UnaryEvent` (3) and `CallEnter` (4 and 6) transition counts.

---

## 4. Prospective Operations & Feasibility Assessment
- **Proposed Operations (`PROSPECTIVE-VALIDATION.json`)**:
  1. **First Feasible Check (`01-newNominalSourceRuntimeFrontend`)**:
     - `node --max-old-space-size=512 .../quint/dist/src/cli.js typecheck .../afterimages/nominalPublicSourceRuntime.qnt` (which transitively imports `nominalPublicSimulation` and `nominalPublicSourceInputs`).
     - Resource caps: `wallSeconds = 60`, `keptBytesPerStream = 4194304` (4 MiB), `stdin = DEVNULL`.
  2. **Second Check (`02-fourExactNominalSourceCaseTests`, gated on `01` PASS)**:
     - `node --max-old-space-size=512 .../quint/dist/src/cli.js test .../afterimages/nominalPublicSourceTest.qnt --main nominalPublicSourceTest --match ^(q10MapQueueTypedReturnAndOrderTest|q10MirrorArborTypedShapeAndOrderTest|q10SameTypeFieldSwapIsDistinguishedTest|q10ExistingCallBuildPathAndPreservationTest)$ --backend typescript --max-samples 1 --seed 1` under the same `60s / 512MiB / 4MiB` caps.
  3. **Optional Third Check (`03-oneMirrorArborOperationalWitness`, gated on `02` PASS)**:
     - Single trace run with `--max-steps 256 --verbosity 1 --out-itf ...` under the existing 64 MiB ITF cap (noting that `max-steps 256` includes inherited `terminalStutter` after `Done`).
- **Wrapper Note**: Root must bind the current 5-file source/package/tool pin manifest to the existing `capture(op, phase)` subprocess mechanism (without invoking `capture-helper.FULL.py`'s Cargo-specific `main()` entrypoint). No old source09 cohorts are replayed.

---

## 5. Verdict, Mandatory Fixes & Open Boundaries
- **Source/API Verdict**: **APPROVE** (adequacy of scoped Quint `source10` model extension, input projections, and 4 tests; eligibility of `01-newNominalSourceRuntimeFrontend` -> `02-fourExactNominalSourceCaseTests` -> optional `03-oneMirrorArborOperationalWitness`).
- **Mandatory Source Fixes**: **None**.
- **Explicit Open Premises & Non-Claims Preserved**:
  - All 5 Quint modules and 4 tests are currently **UNTYPECHECKED and UNRUN**.
  - Monomorphic TypeIds (`10, 20, 30, 40`), constructor IDs, expression IDs, and binder IDs (`100..400`) are a finite manual projection, **not** canonical compiler `TypeIdC` allocations or a generic monomorphization proof.
  - Finite scalar bounds (`MAX_SCALAR = 255`), `FRAME_LIMIT = 32`, `helperWork = 0`, and `TERMINAL_PUBLICATION_CHARGE = 1` are finite model parameters, not owner-B runtime work clocks, C6 host arena layouts, native Halo2 proofs, or EARS requirement closure.

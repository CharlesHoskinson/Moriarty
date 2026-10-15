# Optimization 05: compare three realizations of one Sherlock machine

Status: nonsecurity performance/interface proposal, specified-only. **Priority P1:** freeze a common step/component interface and comparison matrix; **P2:** measure realization alternatives after separately authorized implementation. No circuit, proof, benchmark or native result is supplied. All preparation, rows, capacity/degree, memory, proving time, proof-size and DUST costs are unknown.

Baseline: Candidate A content freeze SHA256 `57d830f46fee57c56ed8b854d21f5dd7a695d2977d061b1ef6905eaecd7de30e`; reference K SHA256 `b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41`. UIREQ-024/026 remain required. Bounds, logical work schedule and reference machine M stay identical across comparisons; changing any is a different profile, not this optimization.

**Public source facts.** The frozen Midnight source at commit `fac0b16844f9b23922e47cd1dfc418ddd1e92968` provides `IvcContext` configuration at line55; `IvcState` separates native State and constant-size AssignedState at94–109; `IvcIO` supplies assignment/public-input formatting at136–177; `IvcTransition` declares native `transition`, architecture and `circuit_transition` at186–207 [M1]. These are candidate adapter surfaces, not evidence of Sherlock compatibility. RISC Zero's captured official overview distinguishes execution segments from recursive aggregation [R1]; this supports keeping realization and composition costs separate, without importing its CPU, proof formats or backend.

**Recommendation/inference: common interfaces first.** Proposed interfaces are:

```text
RealizationConfig = {encoding, unchangedBounds, logicalWorkSchedule}
prepare(program,config) -> ProgramView + PreparationReport
StepInput = {encodedState, fetchedCodeRows, operands, auxiliaryData}
StepOutput = {encodedNextState, logicalDelta, terminalOutcome}
emitStep(kind,config,ProgramView,StepInput) -> component description
```

`decode` maps each encoded state/row back to M. ProgramView is immutable program-derived data; preparation never specializes to one observed execution trace or unrolls recursive source calls. Report program preparation, recurring step work, wrapper/composition and endpoint costs separately.

| Realization | Proposed interface use | Cost hypothesis and gap |
|---|---|---|
| Per-program step | `emitStep(Specialized(P),...)`; constant code/table layout for P | May remove unreachable opcode branches/code selection. Adds per-program preparation/artifacts; all runtime branches of P and recursive calls still follow M. |
| Single universal step | `emitStep(Universal,...)`; same shape, program supplied as bounded data | Amortizes shape reuse across programs. May pay dispatch, code-fetch and shared state-layout overhead on every step. |
| Fixed family | `emitStep(Family(class),...)`; common state layout and a fixed set of components | May reduce unused branch machinery. Adds family dispatch/interface crossings; native composition support for the family remains uninspected. |

Family classification must follow **semantic microsteps**, not just source opcode: EvalSchedule, FrameReturn, Scalar, CallBind, DataMatch and Terminal are candidate classes. A return is selected by control plus top-frame tag. Helper granularity stays the frozen schedule; separately metered helpers would require a new schedule. Each family boundary maps to exactly one original transition, including original zero-charge publication. Administrative padding cannot consume application fuel or turn Halted into another result.

**Code-fetch/preparation options.** Compare linear input-table scan, program-local first-occurrence index, and constant per-program code selection. Keep the same function-ID/branch meaning, argument order and missing-function/type outcome phase. Source IDs remain integer identifiers. Preprocessing is explicit cost, not a free transition or a reason to change source fuel. A table index may improve recurring access while increasing preparation and state/scratch footprint; a circuit backend may implement a selector differently from a host map. Neither “one active branch” nor “unused source branch” establishes actual constraint savings.

**Preservation predicate, not a theorem.** For the same program/input/profile, decoding each realization's logical-boundary states recovers M's control, environments, ordered pending frames, values, result/rejection, depth/caps and exact consumed/remaining work. The ordering and points of Exhausted/cap outcomes remain identical. Future financial effects, duties and suspension boundaries need their own full projection; they are absent from current pure M. No comparison claims them preserved from final-value agreement.

**Normal comparison membership.** Retain all eight frozen K fixtures: factorial3=6, distinct sum3=6, list123=6, tree45=9, bool/let=7, caller-local=12, mutual-even4=true and constructor-list=[1,2]. Add normal branch-heavy versus arithmetic-heavy programs and repeated runs of the same input program, with independently frozen state/work expectations before execution. Compare the same logical traces/profile across realizations, including pending non-tail returns and terminal publication. A measurable predicate is lower total recurring component cost after separately reporting preparation for a stated reuse count, with identical decoded observations. No winner, break-even count or numeric cost is invented.

UIREQ-026 supplies the required Lean representation/Clean bounded-step stage. Reuse public `ProvableStruct`, `witnessIR` and `InductiveTable.step` interfaces; actual development belongs in existing private Fenrir. A proposed native `MoriartyTransition` adapter maps common state/step I/O to M1's traits. Clean-to-native lowering, family support and endpoint mapping remain unimplemented qualifications. Existing ledger9 custom-decider integration is a session premise, not reproduced here; no foreign proof-system port is proposed.

**Counterargument.** Specialization may dominate for a single program, while a universal shape may amortize across many programs; family boundaries can erase branch savings. Choose from the eventual measured workload and component inventory, keeping the same M and observation contract throughout.

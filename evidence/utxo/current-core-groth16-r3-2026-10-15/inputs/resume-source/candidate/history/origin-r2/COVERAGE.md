# Coverage and refusal map: mori-r1cs first slice (UNADOPTED, source only)

The data source is `source/mori-r1cs/src/coverage.rs`. Its `expr_support`, `binary_support` and `statement_support` functions match the actual `mori_core` enums with no wildcard arm, so a new Core variant stops compilation until its row exists. Nothing here has been compiled, tested or run. "Lowered" means the success relation constrains the construct for admitted successful transitions. It does not mean the lowering is verified.

Profile: `moriarty/1` source, `moriarty-core/1` semantics, `moriarty-reference-logical-cost/1`, at Core `6c7b1fdaffec98d4fa912c9b16a3c4879f572a47` (parent `9c63048a`). `moriarty/2` is admitted by the actual `stage::load` and then refused with a staged lowering plan. This is not full `/2` support and does not count as `/2` evidence.

## Expressions (`ir.rs:69-90`)

| Construct | Status | Lowering | Core source |
|---|---|---|---|
| `LiabilityPresent` | Lowered | current duty cell `present` wire; present-zero reads true | runtime.rs:430-441 |
| `LiabilityAmount` | Lowered | current duty cell `amount`; an absent cell is canonical `(false, 0)` | runtime.rs:430-441 |
| `Literal` (UInt, Bool) | Lowered | constants; folding uses checked u128, and a failing constant is a static refusal | runtime.rs:442 |
| `Name` | Lowered | environment wire: pre states + inputs, then `Let`, then `Next` rebinding | runtime.rs:443-446, 716-746 |
| `Not` | Lowered | `1 - b` | runtime.rs:447-450 |
| `Binary` | Lowered | per operator below; `lhs` then `rhs` always lowered first | runtime.rs:451-479 |

Evaluation model: each statement lowers every node reachable from its root once, using a fresh per-statement memo, so shared DAG nodes are re-lowered in later statements against the then-current environment (test D1). There is no short-circuit and no gate: the actual evaluator has no branch (runtime.rs:451-453), so no path is inactive.

## Binary operators (`ir.rs:54-66`)

| Op | Status | Constraint | Core source |
|---|---|---|---|
| Add | Lowered | `a + b = c`, c 128-bit range-proven | runtime.rs:455-459 |
| Sub | Lowered | `b + c = a`, c range-proven | runtime.rs:460-464 |
| Mul | Lowered | limb products; `a1·b1 = 0`; `c = a0b0 + 2^64(a0b1 + a1b0)`, c range-proven (exact 256-bit intermediate) | runtime.rs:465-469 |
| Eq | Lowered | UInt: `d·inv = 1 - f`, `d·f = 0`; Bool: `1 - a - b + 2ab` | runtime.rs:470 |
| Ne | Lowered | `not(Eq)` | runtime.rs:471 |
| Lt / Gt | Lowered | `f(2(a-b)+1) = (a-b) - t`, f Boolean, t range-proven (Gt swaps operands) | runtime.rs:472, 474 |
| Le / Ge | Lowered | `not(b<a)` / `not(a<b)` | runtime.rs:473, 475 |
| And / Or | Lowered | both operands lowered first; `ab` / `a + b - ab` | runtime.rs:476-477 |

## Statements (`ir.rs:122-169`)

| Statement | Status | Lowering | Core source |
|---|---|---|---|
| `Await` | Planned, not lowered | `/1` admission refuses it; `/2` refuses with `StagedLoweringPlan` | stage/profile.rs:10-23 |
| `Let` | Lowered | binds the root wire; no effect | runtime.rs:716-720 |
| `Requires` | Lowered | root = 1 at its position; a constant false refuses at compile | runtime.rs:721-734 |
| `Ensures` | Lowered | root = 1 **at its position** (positional, not trailing) | runtime.rs:721-734 |
| `Next` | Lowered | State effect (before, after); state and name both rebound | runtime.rs:735-746 |
| `Emit` | Lowered | Emit value slot, UInt or Bool. The initial ledger target refuses whole-program Emit as a target policy; the compiler does not drop it | runtime.rs:747-754 |
| `Transfer` | Lowered | checked debit, then checked credit; a self-transfer keeps the debit check and changes nothing | runtime.rs:668-700, 755-786 |
| `Fee` | Lowered | Transfer's balance semantics; distinct effect kind | runtime.rs:755-786 |
| `Liability` | Lowered | checked accrual into the registry cell; sets present (present-zero allowed); identity is a registry constant | runtime.rs:787-836 |
| `Discharge` | Lowered | `present = 1`, checked subtraction, present-zero kept | runtime.rs:837-860 |

## IR, scenario and execution fields

| Field | Treatment |
|---|---|
| `Program.language`, `semantics` | Admission requires `moriarty/1` / `moriarty-core/1`; both are in the profile digest (header) |
| `Program.name` | Inside the canonical Program hash (header). Never compared by the compiler |
| `Program.states` | Slot types and name order. Initial values are Core's scenario default; the pre-state is claimed |
| `Program.assets` | Cell identities `(network, issuer, reference, scale)` in the shape digest. Any Core-valid scale is admitted; a target may refuse nonzero scale (`layout.rs:25-29`) |
| `Program.expressions` | Lowered per statement as above |
| `Program.actions` | One relation per action; action index in the header |
| `Scenario.state`, `inputs` | Pre-state and input slots, typed |
| `Scenario.balances` | Represented cells: touched cells plus optional `FrameCells::Include`. Other rows are a frame-completeness target obligation |
| `Scenario.liabilities` | One `(present, amount)` cell per source-registry duty, sorted by name |
| `Execution.pre`, `post`, `effects` | Public body and effect slots |
| `Execution.steps` | Every step value is a constrained internal wire. Step snapshots are not published, because they are determined by pre-state and inputs |
| `CostReport` | `W` is a header constant; with `CapBinding::PublicCap` the circuit enforces `W ≤ cap` |
| Owner intent limits (`check_effects`) | Outside the relation. `AccountingBinding::GrossDebitAndFees` exposes exact gross debit and fees per debit cell |
| DUST, service and external cost planes; source sponsorship | Outside `logical-cost/1`; unmeasured |

## Statement layout (DirectFieldAdapterV0, test-only)

Header: relation version (1 field), program hash (2: hi/lo halves), profile digest (2), action index (1), shape digest (2), compiler build digest (2), W (2: lo64/hi64), then cap (2) if bound. Body: inputs, pre states, pre cells, pre duties `[present, lo, hi]`, then post states, cells and duties, then effects in statement order, then accounting `[lo64, mid64, top16]` sums. Each UInt limb is forced below 2^64 by a bit-sum binding, each Bool to {0,1}, and each wide sum below 2^144, so no public vector has a modular alias. Frozen counts with a public cap: open 70, partial 62, finish 56, allocate 89, rebate 56, pay 68, settle 49.

## First-slice refusals (each with a reason; nothing is silently dropped)

| Refusal | Error | Reason |
|---|---|---|
| Any `moriarty/2` source | `StagedNotLowered(plan)` | Staged segment relations are not written yet; the plan from the real `StagePlan` is returned |
| Duty touched or read with no source `Liability` anywhere (external duty) | `Unsupported{"external duty"}` | Its identity would be prover data; the first slice fixes identities at compile time |
| Action that can never succeed (constant false predicate, constant overflow or underflow, conflicting source identities) | `StaticFailure` | No success relation exists |
| Represented snapshot above Core bounds (4,096 balances, 1,024 liabilities, 65,536 trace cells) | `Bound` | Conservative bound: every registry duty is counted as present |
| Unknown action, unknown frame asset, bad frame account | `UnknownAction` / `Option` | — |
| Scalar field under 198 bits; a circuit over another field | `Field` | The exact-integer arguments need `r > 2^195` |
| All-zero compiler build or Core closure digest | `Identity` | The build identity is mandatory |
| `ClaimantBoundAbiV1Pending` adapter | `Adapter(Unadopted)` | That ABI is frozen but unreviewed; no mapping is defined |
| Initial ledger target with any Emit in the program | `initial_ledger_target_admits` → `TargetRefusal` | Two adopted votes (target policy). Full Emit stays a required future track |
| Rejection proofs (proving that an input is refused) | not produced | Refusal semantics are declared below; only success relations exist |

## Refusal semantics on the success path (declared, not proven)

Every Core refusal reachable for an admitted `/1` action has no satisfying success assignment. The cases: overflow (Add, Mul, credit, accrual), underflow (Sub), insufficient balance (self-transfer included), false `requires`/`ensures`, unknown residual liability, discharge exceeding the residual, and meter exhaustion with a bound cap. Admission and scenario refusals are compile-time facts or slot typing. The full list is `coverage::REFUSALS`. A separate rejection relation (code and step) is future work.

## Target obligations outside the transition relation

`adapter::TARGET_OBLIGATIONS` lists them: pre-state authenticity (ScriptCustodied datum binding versus KeyHeldExternal unauthenticated premises), frame completeness, the physical asset delta, debit authority and owner intent limits, replay and consumption, ledger phase and failure outcomes, and setup provenance. A pure transition proof neither authenticates a ledger nor moves an asset.

## Remaining required work (none waived; no whole-language percentage)

Full staged `/2` lowering: continuation codec correspondence, waits, captured values, the complete StagePlan, residuals, present-zero across segments, logical-cost/2, Idle/Pending and emitted effects (`staged::STAGED_REMAINING`). Also: all 20 patterns and the general obligations, complete physical effects and value, liability execution and asset integration, private witnesses and private handoff, multi-parent composition, recursion and PCD, and every U0–U7, M1–M17 and MC/SP/G exit. The product contract's terminal target is Midnight ZKIRv3. This arkworks/Groth16 relation is separately scoped Cardano eUTxO research, not a ZKIRv3 result.

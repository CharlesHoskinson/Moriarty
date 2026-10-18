# Financial specimens: current compiler source coverage

Both merged financial programs have matching Core/1 parser, validation and `mori-backend` lowering branches for every construct they use. No missing statement/expression variant was identified on that inspected path. This is a source observation, **not an exact-program compiler pass**. No compiler was invoked, no artifact was produced, and no target/authorization selection is supplied by their host Scenario fixtures.

The immediate ordinary gap is a compiler-facing independent expectation and actual-consumer test for these exact sources. Replacing them with the older Counter/Payment pair would omit their liability reads/evolution, multiasset frame, installment linkage and after-fee reserve specimen. No new production lowering implementation is justified by this inspection alone.

Source shorthand below: **F** = `crates/fixtures/financial-pair/`; **T** = `crates/mori-core/tests/financial_pair.rs`; **B** = `crates/mori-backend/src/lib.rs`; **R** = `crates/mori-core/src/runtime.rs`. `SOURCE-INVENTORY.json` supplies complete paths, exact file hashes and cited line ranges. Line references describe inspected source, not executed behavior.

## Consumers and profile boundaries

| Actual consumer | Current source path | What can be stated here |
| --- | --- | --- |
| Financial host tests | T:35–39,157–238 calls public parse/validate/simulate/simulate_metered | Core/1 host consumer, three literal scenarios and 39 financial prefixes. Invoice first actual post becomes the final Scenario and must equal its frozen literal. Per-call work expectations are 42/42/45. No compiler/backend import exists in T. This diagnosis does not rerun or adjudicate earlier host results. |
| Native-feature `mori compile` | `commands/compile.rs`:20–70 → `source_projection.rs`:35–41 → B:367–425 → lower_action | Core/1 dispatch selects unsigned/signed backend entry from the supplied Target, validates it and lowers every action. Both fixtures declare `/1`, so the CLI's separate `/2` branch is not their route. Exact current compile results are unobserved. |
| Build without native support | `commands/compile.rs`:267–274; `commands/mod.rs`:70–77 | Explicit `mori::command::native-unavailable` stub, independent of whether the Core source is parseable. Cargo.toml:10–25 declares the feature/default; this inspection does not identify an executed binary or feature selection. |
| Backend profile/target admission | `stage/profile.rs`:10–22; B:88–117,367–408 | Core/1 requires exact language/semantics and no Await. Target requires its exact pinned profile/revision/version and matching asset network. The two source networks are `local`; their fixtures select no Target, anchor or authorization mode. These are interface prerequisites, not observed admission. |
| Separate native-call compiler | `commands/compile_call.rs`:52–68 → `mori-ledger/relation/mod.rs`:86–104 | This route consumes Deployment/StaticBindings in addition to source, not a `mori_backend::Artifact`. Its own compiler has all used expression/statement branches, but layout admission requires exact role/asset/duty mappings (`layout.rs`:157–264). Those bindings are absent from this fixture packet. No route equivalence or native-call admission follows. |
| Core/2 or waiting behavior | Core `stage/profile.rs`:10–30; backend `stage/mod.rs`:22–35 | Legacy Core/1 compiler consumers explicitly refuse Await/wrong profile. A separate staged compiler exists. Neither financial source contains Await, continuation, join or resume; reserve approval is an ordinary Requires guard. They cannot stand in for persistent conditional settlement. |

The explicit external SDK boundary in ordinary backend lowering is B:1232–1237: SDK IR serialization, `IrSource::load` and comparison. SDK field/type operations are also used while constructing instructions. External package implementations and build effects were not inspected. This is the exact unadmitted dependency boundary to retain, without classifying its security or physical resource behavior.

## Construct matrix for the exact frozen sources

“Branch present” means the named source code handles the construct; it promises neither complete execution nor constraint/correspondence correctness.

| Construct | Installment invoice | Conditional reserve | Actual source support |
| --- | --- | --- | --- |
| Profile/actions | F/installment-invoice.mori:1,8–12: register then collect | F/conditional-reserve.mori:1,8–13: register then allocate | Parser:5–6; R:120–125; B:424–425 compiles **both** actions per Program, including register, although host scenarios never invoke register. |
| State and initial literals | Lines4–7: paid0, dueDay30, installments0, marker77, all UInt | Lines5–7: allocated0 UInt, closedfalse Bool, marker77 UInt | Core IR:33–43,93–103 and parser:74–81; B:805–810/1106–1116 includes every declared pre/post state. Compilation consumes no Scenario and does not execute those initial values. |
| Exact assets | CASH `(local,financial-pair-invoice,CASH,scale2)` | CASH scale2 and BOND scale0, issuer financial-pair-reserve | Parser:82–116 retains all four identity components; B:393–401 network gate and835–839 asset map. Scale is identity metadata here; source amounts remain integral UInt expressions. |
| Inputs | collect amount/atDay UInt | allocate approved Bool, amount/bondAmount UInt | B:812–818 creates action-input entries by declared type. These are compiler inputs, not the fixture's concrete47/80/50/7 values. |
| Literal and Name expressions | Amounts, days, state, captured opening | UInt and Bool literals, approved/closed, amount/bondAmount, residual | Parser:277–295; B:704–717. All used expression leaves have branches. |
| Arithmetic/comparisons | Add/Sub, Gt/Ge/Eq | Sub, Gt/Le/Ge/Eq | Parser:307–317; B:612–624,725–741. Mul/Or/Ne/Lt branches elsewhere do not mean these specimens exercise them. |
| Boolean expressions | `liability_present("invoice") and liability_amount("invoice") >= amount` at line15 | `approved and not closed` at line14 | B:718–720 and742. `and` and `not` have branches; no branch statement or alternate program is synthesized by these guards. |
| Liability reads | Lines15–16,23: present/amount(invoice), capture before discharge and read afterward | Line27: amount(allocation) after discharge | Parser:285–292; B:694–702 reads the current liability map. Let stores the earlier Wire at936–939; the map is replaced at1096. This identifies the source route for capture/current-state distinctions without certifying it. |
| Source-wide duty identities | Register invoice/CASH/payer/seller and warranty/CASH/payer/auditor | Register allocation/CASH/reserve/supplier, bond/BOND/reserve/auditor, emergency/CASH/reserve/emergency; allocation repeats with same identity in allocate | `liabilities.rs`:42–109 builds a registry over **all actions**, checks named reads/discharges and repeated identity consistency. The unused register action is part of the compiled source; removing it would change the relevant source and registry. |
| Liability/create-or-accrue statements | Register lines9–10 | Register lines9–11 and allocate line19 | R:286–306; B:1038–1096 has the shared Liability/Discharge branch and records identity/operation metadata. |
| Requires | collect lines13–15 | allocate lines14–16,18 | R:249–252; B:940–945 has the predicate branch. This is accepted construct coverage, not observed acceptance of concrete inputs. |
| Let capture | line16 opening | line17 residual `(57-amount)-2` | R:240–248; B:936–939. The reserve expression uses literal 57, not a balance-query expression. It describes the chosen opening 57 specimen rather than a general balance-derived reserve rule. |
| Transfer | line17 CASH payer→seller | lines20/23 CASH reserve→supplier, BOND reserve→auditor | R:268–285; B:978–1037 handles exact asset/from/to and records ordered effect metadata. |
| Fee | line18 CASH payer→treasury3 | line21 CASH reserve→treasury2 | Same B branch distinguishes fee at1019–1023; no host netting or alternative compiler is used. |
| Discharge | line19 invoice | lines22/24 allocation and bond | R:308–315; B:1038–1096 has the explicit Discharge path and preserves its separate operation label. |
| Next | lines20–22 update paid/dueDay/installments | lines25–26 update allocated/closed | R:254–260; B:946–963 updates the environment for subsequent expressions and emits State metadata. Marker remains a declared pre/post state member. |
| Ensures | line23 current invoice vs captured opening minus amount | line27 allocation amount equals0 | Same predicate branch B:940–945; evaluation occurs at that source position. |
| Emit | line24 collected(paid) | line28 allocated(allocated) | R:262–267; B:964–977 emits metadata without another state assignment. |

Invoice collect's effect-bearing source ordinals are4,5,6,7,8,9,11; reserve allocate's are5,6,7,8,9,10,11,12,14. B:1099–1100 appends only nonempty effect descriptors. Backend descriptor spelling/layout differs from Core Effect JSON, so the host oracle is not already an independent backend ABI golden.

## Complete host observations versus compiler artifact frames

B:820–861 gathers balance keys only from Transfer/Fee endpoints. Invoice collect therefore includes CASH payer/seller/treasury, while its host oracle also preserves observer 23 and reserve 19. Reserve allocate includes BOND auditor/reserve and CASH reserve/supplier/treasury, while its host oracle also preserves observer BOND 7 and CASH 11. Those untouched rows are not additional balance-frame entries on this backend path. This is the source-defined frame boundary, not a finding that the rows were lost in an actual execution.

All declared state and all source-registry duties receive pre/post descriptor positions (B:805–810,863–895,1106–1153). Their presence/amount metadata must remain distinct from a balance frame. Compiler output does not itself execute either Scenario, link invoice calls, compare all 39 host prefixes or report reference work 42/45. T checks full financial snapshots, ordered effects, Step indices/discriminants and aggregate cost, but excludes Step.value, raw statement ExprIds and actual per-prefix cost. F/financial-and-work.json:162–169 retains those unresolved fields; newer normal-report design adoption does not rewrite this older freeze.

## Contrast with the existing Counter/Payment compiler check

`mori-backend/tests/stages.rs`:374–398 parses historical IndependentCounter and IndependentPayment source files and calls the genuine production compiler. Counter checks action names plus public role/name sequences; Payment checks effect-frame debug text against existing snapshots. Production lowering reaches real SDK IR construction/serialization/loading on that path. It is useful compiler-structure coverage, not a DTO mock.

It does not consume either financial-pair source, any financial Scenario or their independent full outcomes. Payment has one asset and no liability statement/read/discharge, reserve residual or installment linkage. Counter adds scalar/Bool state and two actions, not those missing financial observations. The assertions do not independently cover complete instructions, types/constants, target/context identities, every frame or full Artifact bytes. The existing `compiler-local-check-proposal` retains that exact narrow scope and UNRUN selection. Neither pair nor this diagnosis closes full U2 or qualifies native behavior.

## Next ordinary slice and retained requirements

Prepare a new independent **compiler-facing structural expectation packet** for the unchanged invoice/reserve sources, before any compiler run. Its immediate prerequisite is an explicit chosen consumer and test Target/profile; the existing host packet supplies neither Target nor native-call bindings. Do not invent an anchor/authorization deployment identity or use a raw source hash as a compiled identity.

For the selected backend path, independently specify action ordering including register, complete state/input descriptor types, exact duty tuples/presence slots, touched balance endpoints, and ordered effect-descriptor step/kind/identity mappings. Keep the complete host snapshots/prefixes and omitted-frame mapping beside those expectations. Unknown emitted IR/context/artifact bytes remain unknown; do not generate goldens by compiling. Then prepare a narrow integration-test source that parses the exact frozen `.mori` files and calls existing `mori_backend::compile`, comparing independently frozen metadata through the real consumer. No new lowerer, fixture dispatcher or production edit is indicated now. Execution and external dependency classification remain separately unadmitted.

The UNI citations below identify existing EARS requirements in the consolidated-language-kernel specification; they do not add or close requirements.

| Existing requirement | Exact retained gap |
| --- | --- |
| ROADMAP U2:288 | These authored contrasting programs remain candidates for the complete claimed pipeline. Source branch coverage, host results or the historical Counter/Payment check do not establish pinned compiled output and complete downstream observations. |
| UNI-018:190–199 | Identify Core/1, actual build/consumer and admitted subset. Native-unavailable builds, wrong profiles and separate native-call bindings cannot inherit host admission. |
| UNI-019:209–218 | Declare the mapping between complete Core observations and touched-frame artifact representations before comparing lowerings. No whole-consumer equivalence follows here. |
| UNI-020:220–233 | Preserve the independent host outcomes/work and freeze new compiler-facing expectations without deriving them from the compiler under review. |
| UNI-021:235–246 | Keep implementation, static inspection, actual observations and acceptance separate for exact source/profile/consumer; no completed parent denominator follows. |
| UNI-005:47–56; UNI-007:69–78 | Full financial-state and conditional-settlement requirements remain; a guarded atomic allocation is not an observed durable waiting lifecycle. Their broader acceptance work is outside this diagnosis. |

All original U0–U7, M1–M17, MC/SP/G and full ACTUS/DeFi/finance/composition exits retain their original predicates. This diagnosis supplies no security, constraint-correctness, proof, resource-limit or native/ledger judgment.

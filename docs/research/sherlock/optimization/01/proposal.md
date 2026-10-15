# Optimization 01: bounded helper microsteps and program-local indexes

Status: nonsecurity proposal, source-inspected/specified-only; no model, test, theorem, build, benchmark or native command. Priority **P0: make helper granularity/capacities explicit before circuit costing; P1: reversed argument collection and program-local lookup index; P2: other caches after the preservation interface is fixed**. No timing, row or native-capacity benefit is measured.

## Exact baseline

Sherlock Candidate A content freeze SHA256 `57d830f46fee57c56ed8b854d21f5dd7a695d2977d061b1ef6905eaecd7de30e`, repository baseline `9c63048a18315d7eb3cf114b983bd68c0d842dc6`. K source `model/k/main.k` SHA256 `b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41`. Requirement anchors: UIREQ-024, complete declared observation/cost preservation; UIREQ-026, required Lean semantics/representation and bounded Clean step stage. The freeze is unchanged by this proposal. Exact source membership/hashes are in sources.json.

**Source facts.** K lines 200–205 wrap `uiFind`/`uiBind` in charged machine rules, while their recursion at 280–298 is uncharged. `uiAppend` at 285–287 traverses every accumulated argument prefix. `uiValid` at 229–235 traverses literal constructors. `uiBinaryResult(uiEq(),A,B)` at 258 invokes structural `==K`. Matching itself examines one concrete constructor, but its environment-map operations and `uiBindOne` membership/update have no separately stated physical work cost. The frozen design/model coverage acknowledges these gaps and leaves exact fixture fuel totals unfrozen. Official K tutorial snapshots demonstrate explicit semantic computation items and evaluation control, not this proposal's work accounting [K1–K3].

## Two distinct changes, with concrete interfaces

**A-preserving lowering candidate.** Retain every A logical rule and its fuel charge. Represent evaluated argument prefixes internally as a reversed accumulator; each argument-return stage prepends once, then a bounded explicit cursor restores source order before binding. Replace repeated scans with an index derived from the current *input program*, not generated semantic rules per program. Interface: `PreparedProgram = {originalCode, firstOccurrenceIndex, declaredBounds}` and `lookupPrepared(program,id)`. Its functional condition is `lookupPrepared(P,id) = uiFind(id,P.originalCode)`, including first-occurrence behavior. The index is invalidated/rebuilt when the input code changes. Parameter slots and environment layouts are similarly program/configuration data. Source identifiers remain arbitrary integer IDs; physical slots do not redefine their identity. Preparation must not surface missing-function or argument-type outcomes before A finishes evaluating arguments; it may cache metadata, but publishes those outcomes only at the original call phase.

Physical helper states may use this schematic interface (not compiled K source):

```text
findScan(id, codeCursor, callArgs, resume)
reverseArgs(remainingReverse, forward, resume)
bindScan(remainingParams, remainingArgs, partialEnv, resume)
validate(todoValuePaths, resume)
equal(todoValuePathPairs, resume)
```

Each cursor examines/moves one bounded slot or one constructor occurrence. Worklists carry references/paths, not implicit whole-tree copies; their capacities are explicit profile parameters. `uiAppend` becomes constant-size prepend plus one linear reversal; for n arguments its old helper-rule visits total n(n+1)/2, versus a single n-item reversal and n prepends. This is a source-level traversal count, not measured runtime. A lookup-index build costs one traversal per program rather than one traversal per call; source fuel remains unchanged under this lowering option.

The lowering keeps helper transitions administratively invisible to A logical fuel, but records a separate physical-helper counter. A profile must give bounded input/code/arity/constructor/worklist sizes and a physical budget adequate for every admitted A step; physical budget refusal cannot be relabeled A logical FuelExhausted. If that total bound/representation is absent, this is only an implementation sketch and does not eliminate A's metering gap. No claim of physically constant map access or zero-cost preprocessing is made. A currently lacks these enforced input-size caps: introducing them narrows its admitted domain and must be declared as a profile change, not represented as preservation over every A input.

**Separately versioned logical schedule Aμ.** If every constructor/code/binding visit must consume semantic work, publish a new schedule identifier and explicit helper control variants. Retain the charge on the old enclosing machine-rule entry; add one charge per helper cursor operation. This changes exhaustion points and consumed work, so it is **a logical-cost version change**, not A-preserving lowering. Suggested mapping:

| A helper site | Aμ replacement after unchanged entry charge | Additional work |
|---|---|---|
| `uiInvoke → uiEnter(uiFind(...))` | `findScan`, one record comparison at a time | k for hit at 1-based position k; n+1 for no match in n records |
| append at each argument return | prepend into reverse accumulator; finish `reverseArgs` | n moves + one finish per n-argument call |
| `uiEnter → uiBound(uiBind(...))` | source-order `bindScan`, one typed parameter pair at a time | n pairs + one finish on a valid n-argument call |
| `uiEval(uiLiteral(V))` validity helper | `validate` visits constructor/UInt/Bool occurrences, then finish | N(V)+1 on a valid literal |
| binary structural equality | `equal`, pair stack, head/tag/scalar comparison, ordered child enqueue | compared pairs + one empty-worklist finish for equality; stop on first unequal pair |

Environment membership/update, frame/code/data reads, worklist push/pop and bounded UInt limb operations must have explicit bounded-slot interfaces too. A fixed capacity makes their cost configuration-bounded; it does not justify hiding an unbounded Map or giant integer traversal inside a microstep. Exact per-slot cost is a further profile decision, not assigned an invented native price here. Pattern matching remains source ordered and only the selected body runs. Validation/equality count structural **occurrences** even if a physical arena shares a node; no pointer or digest equality replaces structural equality.

## Old/new logical work and preservation predicate

Let W_A count frozen machine-section rules. Under the lowering option, W_new = W_A, with unchanged remaining logical fuel and a separate physical counter/preparation record. Under the proposed Aμ schedule, for normal completed runs:

`W_μ = W_A + Σ lookupVisits + Σ reverseVisits + Σ bindVisits + Σ validationVisits + Σ equalityVisits + any separately frozen slot-access charges`.

Do not copy A fuel literals into Aμ. Simple hand-counted examples, specified independently from the interface above and **excluding any additional slot-access charges**: a direct literal `[1,2,3]` has seven value occurrences, so A costs 1 and Aμ costs 1+7+1=9; direct `uiU(6)` costs 1 versus 3. Equality of two literal `[1,2,3]` lists costs A's seven machine rules; Aμ adds two eight-visit validation traversals plus eight equality visits, giving 31. Both return the same respective literal/Bool values with sufficient work; equal numerical fuel budgets need not produce the same completion under different schedules. These are proposed schedule literals, not executed observations or assertions about frozen fixture totals.

**Functional preservation predicate, not a theorem claim.** On the same declared source/profile domain, decoding optimized completed states must recover A's value/result, rejection code, control/frame/environment meaning, selected branch and argument order. A-preserving lowering additionally preserves logical consumed/remaining work and exhaustion at each A boundary; helper stutter maps to the unchanged enclosing transition. Aμ instead declares a cumulative cost map at helper-completion boundaries: terminal results agree where both budgets suffice, while intermediate exhausted helper states have a separately specified projection. No unchanged-work or identical-exhaustion claim applies to Aμ. This pure fragment has no financial effects, duties or durable boundaries; extensions must add their observations explicitly rather than infer preservation from this predicate.

## Normal witness membership and required Clean work

Retain all eight frozen K fixtures, with exact terms/hashes listed in sources.json: factorial3=6, sum3=6, list123=6, tree45=9, bool/let=7, caller-local=12, mutual-even4=true, constructor-list=[1,2]. They cover only unary calls and do not exercise long argument-prefix append. Additional specified normal witness contracts are (a) three-argument named sum(1,2,3)=6, preserving source-order parameter association; (b) call the second of two distinct constant-return functions and obtain its second value under unchanged interpreter rules; (c) direct list literal and equal-list comparison above, with occurrence-count schedules; (d) the same recursive list with shared versus expanded physical representation and equal logical occurrence accounting. No witness was executed, and no security fixtures or judgments are included.

UIREQ-026 stays mandatory: specify these controls in recursive Lean semantic/fuel-run definitions, then encode bounded helper cursors/worklists/slot operations in the Clean fixed-width step interface. Public upstream `ProvableType`, `ProvableStruct`, `witnessIR` and `InductiveTable.step` are reusable interfaces from the frozen public API receipt, not an existing Sherlock implementation. Actual Clean development belongs in the existing private Fenrir integration; no private path/content/pin appears here. Its native exporter/lowerer and helper-to-native cost correspondence remain unimplemented/unmeasured dependencies.

**Counterargument.** Charging every helper visit can dwarf source-level work, lengthen traces and expose representation choices in a public cost schedule; cached indexes add preparation/state complexity and can change the cost of identical code queries if cache state is observable. Prefer deterministic program-local preparation and a stable logical cost contract. If configuration-bounded helpers are adequate for the target, keep A's schedule and report the independent physical bound; adopt Aμ only as an explicit new profile after the concrete accounting choice is reviewed. Neither choice has a measured circuit or native advantage.

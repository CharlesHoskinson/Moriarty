# C6P1 host codec interface

`mori_core::sherlock::helper_machine::c6_phys` provides the source-declared C6P1root/frame physical section and strict recorders around the existing fixed owner step. Its tags, raw stack order, caps and first-error rules are defined by the complete module source. `c6_phys_program` provides the complete owned ProgramSnapshot codec section. Neither codec admits a program or acts as another evaluator. `PushOwnerCharge::from_event` validates caller-supplied event/snapshot data and carries no owner authenticity; the private observed carrier is created by the concrete owner recorder.

The first bounded qualification compiled the real full Core test harness and ran18new exact host tests. See [the qualified receipt archive](../../results/sherlock-c6-physical-host18-20261019/README.md) for source hashes, exact assertions, complete streams, compile warnings, source-basis differences and remaining scope. The archive is evidence for its recorded snapshot; source applicability to the current public tree is reviewed separately. The mandatory SDK-linked backend test and the later full-state sections are separate obligations. No native proof or global encoder-conformance claim follows from these host checks.


## Declared recursive list: owner rows

The [2026-10-20 public body-typed consumer](../how-to/observe-declared-recursive-list.md) produced a user-declared `ListUInt128`, not the dedicated builtin List carrier. Its [actual final snapshot](../../results/sherlock-original007-recursive-nominal-list-20261020/actual/outputs/final.json) records nominal type 4, Nil constructor 4 and Cons constructor 5. These numbers identify this admitted fixture only; resolve them through its tables rather than assuming a global assignment.

| Actual location | Copied row/value | Ordered references |
|---|---|---|
| Program Const 0 | UInt 1 | none |
| Program Const 1 | UInt 2 | none |
| Program Const 2 | nominal Constructor Nil | empty fields |
| Data 0 | nominal Constructor Cons, fields(start 0,len 2) | Const 1, Const 2 |
| Data 1 | nominal Constructor Cons, fields(start 2,len 2) | Const0, Data 0 |
| Final control and outcome root | Data 1 | resolves the whole outer Cons |

The inner Cons was allocated before the outer Cons; source field order remains head then tail in both slices. The five reachable semantic occurrences are not a whole-heap allocation count. Nullary Nil is retained in the program constant table, so it need not appear as a data-arena row.

The public [`c6_phys_data`](../../../crates/mori-core/src/sherlock/helper_machine/c6_phys_data.rs) row codec declares `Constructor [6,type,ctor,start,len]`, with references encoded separately as Const `[0,index]` or Data `[1,index]`. Thus these Cons data rows match the declared Constructor representation, not the dedicated `Cons` tag 3. The layout has a seven-lane header followed by data rows, data-field references, environment rows and value-slot references. This source mapping describes an interface: the UIREQ007 consumer does not call the DATA encoder or emitter, and its outputs are complete owner JSON, not captured DATA-lane vectors.

The codec constructs no admitted image, advances no owner step and checks no cross-table membership. The separate handwritten `c6_data_canonical` source/reviews, SDK execution and in-circuit/native proof remain distinct work. This list observation supplies no new DATA-gadget pass, C6 ABI equivalence, StepInv theorem or target evidence.

## Handwritten Push and PROG/CTX source boundary

The backend now exposes the bounded [`c6_push_step`](../../../crates/mori-backend/src/sherlock_machine/c6_push_step.rs), [`c6_data_canonical`](../../../crates/mori-backend/src/sherlock_machine/c6_data_canonical.rs), [`c6_meta_canonical`](../../../crates/mori-backend/src/sherlock_machine/c6_meta_canonical.rs) and [`c6_program_context_canonical`](../../../crates/mori-backend/src/sherlock_machine/c6_program_context_canonical.rs) source modules. This is source publication, not a whole-backend build or SDK execution result. [The source/review record](../../results/sherlock-c6-progctx-push-source-20261020/README.md) preserves the exact cut, prior findings and remaining limits.

`emit_c6_program_context_canonical_block` takes a gate, the complete pre-PROG operands, a fixed 22-lane pre-CTX array and caller-supplied `C6StateCaps`. Its source parser follows the 22-table Program codec and CTX structure. The stateful emitter retains temporary numbering; the single-shot wrapper has a documented one-use-per-scope contract. The parent calls this pre-only parser once after DATA and META. It retains unconditional pre/post META, PROG and CTX lane ranges, exact equality and length checks, including disabled steps. The source-level domain/security review and integrated normal-production API review are separate records; neither is a runtime observation.

The exported `emit_c6_program_context_ctx_binding_block` helper is not called by this parent. Publication therefore establishes no retained-config-to-CTX/caps binding, authenticated context key, admission/digest authentication, native key issuer or verifier configuration binding. META04 is the called unbound parser; the proposed full-config-bound META leaf and its dissent are separate and are not installed by this change. The HELPER codec is a Core dependency; the separate HELPER canonicality gadget is not newly integrated here.

The shipped DATA test source has static reviews but all SDK preprocessing/synthesis tests remain **UNRUN**. The Push `cfg(test)` fixtures and PROG/CTX test-design document are not approved test results and are not counted. In particular, no compilation, MockProver, measured circuit parameter, proof, native acceptance, all-39 dispatch, `StepInv`, financial/refinement correspondence, EARS closure or stagenet result follows from these exports. Developers need no maintainer review permission to use public interfaces; these evidence qualifications describe the repository's observed implementation state.

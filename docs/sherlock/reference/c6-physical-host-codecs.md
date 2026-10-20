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

## Handwritten Push and retained-family source boundary

The backend exposes bounded [`c6_push_step`](../../../crates/mori-backend/src/sherlock_machine/c6_push_step.rs), [`c6_data_canonical`](../../../crates/mori-backend/src/sherlock_machine/c6_data_canonical.rs), [`c6_meta_canonical`](../../../crates/mori-backend/src/sherlock_machine/c6_meta_canonical.rs) and [`c6_program_context_canonical`](../../../crates/mori-backend/src/sherlock_machine/c6_program_context_canonical.rs) sources. [The PC5 record](../../results/sherlock-c6-progctx-push-source-20261020/README.md) preserves the earlier pre-only/unbound composition. [The retained-family integration record](../../results/sherlock-c6-bound-meta-retained-family-source-20261020/README.md) contains the two complete independent source opinions and exact new source pins. Neither record is a backend build or SDK execution result.

`C6PushFamilyParams::new` retains caller-selected original `C6HostConfig`, separate admission `BoundsC` and limits. It derives the META binding, effective caps, checked padded geometry and immediate expected CTX from those inputs. Fields are private and accessors read-only. The public builder now takes that family rather than loose geometry/META limits; low-level geometry is validated against the family before emission as a host derivation refusal. Admission bounds and layout bounds remain separate.

Push replaces its one unbound META call with one `emit_bound` call. One retained `C6ProgramContextCanonicalEmitter` parses the pre-only 22-table PROG/fixed22-lane CTX and then calls `emit_ctx_binding` on the same temporary serial, using the family's immediate expected CTX. Absent ext and present-zero ext remain distinct. Parent pre/post META/PROG/CTX ranges, exact equality and length premises remain unconditional, including disabled steps. DATA is still called once. The separate HELPER canonicality gadget is not integrated.

This is a caller-selected family relation, not authenticated provenance or a P1 verifier expectation. A family copied from a witness does not establish the verifier's family. Gate0 preserves unconditional ranges/equalities while canonical/binding assertions are inactive; it does not establish family membership. Independent verifier family/invocation selection, source/IR reconstruction, real issuer keys, complete statement/output/communications comparison and native proof verification remain required future work. No native authority type, source checker or digest authentication is introduced.

Two fresh independent Muse HIGH opinions approve bounded design, source relation, normal-production API and test source with no mandatory fixes. The11Push tests and13META leaf tests—24 source tests total—are all **UNRUN**; preprocessing negatives, host derivation refusals and host serialized-IR comparisons remain distinct categories. No synthesis/MockProver/constraint-category/native result follows. Existing DATA tests remain UNRUN separately; PROG/CTX test-design notes are not counted. The combined backend has not been compiled. Full39 dispatch, `StepInv`, admission/refinement, proof/native/ledger/target/EARS and financial/persistence obligations stay open. Public interface use remains permissionless.

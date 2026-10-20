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

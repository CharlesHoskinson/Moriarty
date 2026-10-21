# Nominal source runtime handoff

A fresh full normal Core build, external consumer link, exact three-test listing
and all three positive runtime tests passed. Each test ran individually: one
passed and two filtered. The checked source bytes remained unchanged.

The body-typed nominal compiler prepares, admits and observes recursive generic
ADTs. `nominal_body_command::run` and `run_with_nominal_inputs` return a
`NominalHostRun`: the complete unstepped preparation and the same actual
post-run `CompletedRun`, with every full `StepEventC` forwarded to a caller sink.
The sink receives owner events in order; this adapter retains no event vector.
Caller retention/allocation and sink panics follow the existing dedicated-source
and scalar-action consumer contracts.

`preparation()` and `completed()` borrow the actual products. `observe()` uses
the existing bound-pair observation. `into_parts()` is an owned transfer of the
snapshot and completed pair; these three tests did not exercise that method.
`into_observation()` consumes the handoff into the unchanged
`sherlock-nominal-host-observation/1` envelope without another owner step. Its
`event_capture` is `not-retained`: the adapter has not retained the caller's
sink data.

The existing parser budgets, input binder, all-body checking, admission/genesis,
owner engine, errors and full23-field ProgramView projection are reused. The
Rust entry points are public through `sherlock_compile_artifact::nominal_body_command`.
Existing compile and observe bodies retain their bytes. The CLI dispatch still
uses those existing functions; this change adds no CLI flags or new Core/1
admission for Leaf, Cons or Branch. Source grammar, owner guards and native ABI
are unchanged.

The three passing tests use the recursive generic Ribbon/Parcel program with a
closed entry and contrasting empty/nonempty external inputs. They checked whole
typed results, concrete generic types, field/child order, complete preparation
and observation equality, and full ordered `StepEventC` vector equality with a
fresh run of the same admitted program through the existing owner. Those vectors
and value trees were checked in memory, not separately serialized as retained
runtime artifacts. Complete raw test receipts and streams were retained.

The expected values are source-author predictions. Direct-owner event and
old-observer comparisons share producers; these passes are not an independent
semantic oracle, whole-state trace proof or formal correspondence result. The
Core build emitted31 warnings and no errors; the link/list/test stderr was empty.

A successful API call carries the owner's actual summary, including Rejected or
Exhausted, and runtime HostViolation mapping retains the partial forwarded count.
Those dispositions, observation refusals, sink panics and `into_parts()` ownership
transfer remain unexercised by these three positive tests. This HostOnly handoff
does not implement witness or constraint plans, native export, target qualification,
checkpoint import or H1 whole-process guarantees. Full B04, EARS, native/proof/
target acceptance and refinement remain open.

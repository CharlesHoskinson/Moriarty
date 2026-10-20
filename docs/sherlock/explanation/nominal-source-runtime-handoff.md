# Nominal source runtime handoff

Source candidate; builds and all three positive tests are **UNRUN**.

The body-typed nominal compiler already prepares, admits and observes recursive
generic ADTs. Its existing command observer returns a complete JSON envelope.
The additive `nominal_body_command::run` and `run_with_nominal_inputs` return a
`NominalHostRun`: the complete unstepped preparation and the same actual
post-run `CompletedRun`, with every full `StepEventC` forwarded to a caller sink.
The sink receives owner events in order; this adapter retains no event vector.
Caller retention/allocation and sink panics follow the existing dedicated-source
and scalar-action consumer contracts.

`preparation()` and `completed()` borrow the actual products. `observe()` uses
the existing bound-pair observation. `into_parts()` transfers the snapshot and
real completed pair to another typed consumer. `into_observation()` consumes the
handoff into the unchanged `sherlock-nominal-host-observation/1` envelope without
another owner step. Its `event_capture` is `not-retained`: the adapter has not
retained the caller's sink data.

The existing parser budgets, input binder, all-body checking, admission/genesis,
owner engine, errors and full23-field ProgramView projection are reused. There
are no new CLI flags, numerical defaults, source grammar, guard or ABI changes.
The two new Rust functions are additive public API, requiring Root integration.
The existing compile and observe functions retain their exact previous bodies.

A successful call carries the owner's actual summary, including Rejected or
Exhausted; it does not assert Halted. Runtime HostViolation preserves the actual
partial forwarded count. This HostOnly handoff does not implement witness or
constraint plans, native export, target qualification, checkpoint import or H1
whole-process resource guarantees. Full B04, formal correspondence and EARS
acceptance remain separate.

The new positive tests use a fresh Ribbon/Parcel recursive generic program and
contrasting explicit inputs. Their complete typed-value expectations are source
author predictions. Full-event direct-owner and complete old-observer checks
are shared-producer equivalence baselines, not an independent semantic oracle.
No red/green run or successful build is claimed.

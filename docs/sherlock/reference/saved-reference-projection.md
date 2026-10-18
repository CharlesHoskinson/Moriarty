# Saved owner records as reference prefixes

Public module: `mori_core::sherlock::helper_machine::reference_projection`.
These diagnostic APIs project saved B host observations into the public A39
reference representation. They do not step either machine, import states or
create an authenticated/canonical checkpoint.

| API | Contract |
| --- | --- |
| `reference_value(&DecodedValue)` | Resolve the existing flat decoded UInt/Bool/List/Tree representation into `Result<Value, ProjectionGap>`. |
| `ReferenceSource::new(Arc<Program>, &HostSnapshot)` | Associate the retained exact reference source with observed program, expression, function, parameter and arm rows. |
| `project_reference_boundary(&ReferenceSource, &HostSnapshot, &DecodedSnapshot, initial_fuel, remaining_fuel)` | Return `ReferenceBoundary::Complete { reference, raw, decoded }` or `PendingHelper { raw, decoded }`. Fuel arguments are explicit `u128` reference values. |
| `ReferencePrefixProjection::new(Arc<Program>, &HostSnapshot, reference_fuel)` | Begin accumulation at a complete initial boundary. |
| `push(&mut self, &CompleteStepRecord)` | Return `ProjectionProgress::Pending { raw }` or `Complete(ReferenceSpan)` for the next saved record. |
| `finish(&self)` | Return the current reference snapshot if no incomplete helper interval remains. A complete boundary may still contain a pending A call. |

`initial()`, `records()`, `returned()` and `pending_records()` borrow the
accumulator's retained state/records. `ReferenceSpan` carries `b_start`,
`b_end_exclusive`, `source_event`, every raw micro-record in the interval and the
complete translated reference `StepRecord`.

The source association retains the original AST. Reference environments select
the first visible name from the B newest-first chain; all shadowed cells remain
in the raw/decoded boundary. Source expressions, values, frames, controls and
outcomes are translated rather than evaluated. Pending helper/result/task
payloads remain explicit and are not coerced into a completed A boundary.

A fixed public-reference rule/charge table supplies logical A charges. Reference
fuel is a separate caller input: B remaining/consumed/helper work is never renamed
A fuel. The accumulator checks saved-record continuity and waits for a mapped
source event at a helper/result/task-free boundary. It retains raw intervals and
reference records, so this API does not claim streaming memory savings.

`ProjectionGap` retains witness failures, source association/field/payload gaps,
unrepresentable values or nominal types, reference-fuel mismatch, changed
admission, discontinuity, unmapped/unapplied rules, incomplete intervals and B
resource outcomes. Preserve the supplied raw observations when inspecting a gap;
no rollback, canonical serialization or transition authentication is implied.

The [ordinary evidence](../../results/sherlock-witness-reference-prefix-20261018/README.md)
contains two successful compiles and three exact normal tests comparing complete
independent A39 prefixes/work and factorial pending-caller snapshots against
actual owner-produced observations. Absolute B-clock expectations, additional
rule/countercases, physical costs, canonical ProjectRel/Link, native/proof/target
coverage and whole UIREQ-035/SCON-013/024 remain open. The original accepted test
used absolute fixture includes; its separate portable-path proposal and compile
receipts must be identified explicitly. See the [saved-prefix guide](../how-to/project-saved-owner-prefix.md).

## Portable fixture publication evidence — 2026-10-18

The [dated evidence addendum](../../results/sherlock-witness-reference-prefix-20261018/addenda/20261018-portable-currentness/README.md) records the fresh four-path source review and one successful portable consumer compile (0.571720103 seconds, verified five dep-info inputs). The three functional runtime passes remain evidence for the original absolute-include consumer. No runtime case was replayed on the portable binary. The four fixture copies are byte-identical; only the include paths changed.

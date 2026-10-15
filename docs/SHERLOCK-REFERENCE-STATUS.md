# Sherlock host reference component — 2026-10-15

Status: implemented typed A-profile host component with finite normal comparison
evidence. Candidate B design adoption, implementation and product acceptance
remain separate. This delivery does not close a full roadmap exit.

## Implemented behavior

[`mori_core::sherlock`](../crates/mori-core/src/sherlock/mod.rs) supplies a typed
program API, ordered function tables, named self/mutual calls, strict argument
evaluation, checked UInt128 arithmetic, dedicated lists/trees and shallow ordered
matching. The machine retains explicit controls, nine frame forms, full saved
environments and each intermediate call/binding/primitive boundary. `step()`
returns complete before/after snapshots, source-rule line and charge; `run()`
consumes that same relation. The [public example](../crates/mori-core/examples/sherlock_reference.rs)
constructs mutually recursive functions using the library API. That example has
not been separately compiled or run in this delivery.

The translation contains 36 charged rule branches reachable through these typed
inputs. The [113-case source catalog](../models/sherlock/reference/rules.json)
also retains K's startup, invalid-input and pure-helper cases. Those counts are
source membership, not observed rule coverage or a refinement theorem.

The API starts after K's valid startup rule. `u128` literals and nonnegative host
budgets exclude K's invalid-literal and negative-startup inputs.
Host fuel is bounded above by `u128::MAX`; stack limits by `usize::MAX`.
These upper magnitude bounds also restrict the K common domain. Canonical
signed-decimal names define the claimed K comparison domain; opaque host names
outside it have no K correspondence claim. Dedicated lists/trees are not yet
general declaration-driven ADTs. Helpers, copying, structural equality and host
allocation remain unmetered under `sherlock-k-a/1`.

## Complete typed observation comparison

The new [`sherlock::compare`](../crates/mori-core/src/sherlock/compare.rs) API
returns the first identity/state/record/prefix difference between two supplied
A traces. It covers every Snapshot field, typed structural program/frame/value
meaning, rule and charge, with deterministic ordering and raw values retained in
the borrowed inputs. It neither validates a trace nor supplies missing model or
circuit adapters. The [RED/GREEN evidence](audits/2026-10-15/sherlock-trace-comparison/README.md)
records the actual29-test host run including16 new normal comparison functions;
all original expectations and92 fuel-prefix checks are unchanged. The existing
H1 untrusted-input restriction remains in force. Independent source audits and
full cross-layer/native/stagenet acceptance remain separate.

## Added complete normal and flat-image observations

The [additional normal evidence](audits/2026-10-15/sherlock-extra-normal/README.md)
extends the public host comparison to eight programs and 216 literal prefixes.
The [flat-image preparation](audits/2026-10-15/sherlock-program-image-normal/README.md)
adds a private test-only builder and genuine expression-ID interpreter. Its
captured 46-test closure checks both interpreters against the same frozen
complete-state expectations. Public API exports are unchanged. Arbitrary image
admission, bounded native representation and all B/circuit/target acceptance
remain open. The older observations below retain their original scopes.

## Indexed calls and retained checkpoints

Subsequent scoped deliveries add full snapshot rows, causal expression/frame
origins, finite call-helper contexts and owned in-memory checkpoints. The
[indexed interpreter](../crates/mori-core/src/sherlock/image_reference.rs) retains
occurrence IDs while the [call adapter](../crates/mori-core/src/sherlock/helper_call_adapter.rs)
retains the actual callee, arguments, caller and pending return state.
Checkpoint restoration consumes the saved machine or helper context directly.
It does not reconstruct a state from a display snapshot or replay earlier steps.

The [checkpoint evidence](audits/2026-10-15/sherlock-host-checkpoint-normal/README.md)
records **86 passing host checks** for its captured source, eight programs and
216 frozen fuel prefixes. Its new observations cover 11 image cuts, 37 helper
positions, an exhausted helper prefix and a retained arithmetic error. The
[origin evidence](audits/2026-10-15/sherlock-source-origin-normal/README.md)
separately checks distinct same-name call sites and repeated recursive bodies.
These are private test-module consumers; no public CLI source execution is added.

[Reference slices](../crates/mori-core/src/sherlock/reference_slice.rs) group the
same A events at widths 1, 2, 4 and 8. All internal records, charges and earliest
stops remain observable. These finite comparisons do not freeze or implement
the complete B helper schedule, canonical checkpoint codec, fixed-capacity
image, financial suspension, constrained step or target relation.

## Core source projection

The [source projection evidence](audits/2026-10-15/sherlock-source-projection/README.md)
records actual Core library compilation and six passing tests against that
library. Two stateless `/1` specimens check handwritten Sherlock ASTs and
complete source occurrence paths. The preparation API does not execute their
generated functions; source-to-machine values and prefixes remain unobserved.
The source-reviewed backend/CLI wrapper adds separate projection metadata while
preserving artifact selection. Its build, SDK-backed tests and publication
execution are not established by the six Core tests. Recursive source grammar,
financial effects, bounded B representation and target acceptance remain open.

## Observed comparison

The [dedicated consumer](../models/sherlock/reference/check-reference.py) compiled
the actual production modules with the pinned Rust 1.99.0 compiler, without
Cargo, SDK dependencies, model execution or native operations. Eight test
functions passed. Six independently authored programs cover 86 complete trace
boundaries and 92 literal fuel prefixes: literal, addition, conditional,
shadowed lexical restoration, named self-recursion and ordered list/tree
matching. Checks include input terms, controls, complete saved frames and
environments, code, depth, initial/remaining fuel, outcome and exact source-rule
lines. Free publication at zero remaining fuel is included.

The expected JSON predates implementation. The consumer verifies the committed
Rust constants against a literal transcription; it does not generate answers or work
from the machine. The separate initial RED test compiled, then failed at the
missing transition implementation before the implementation was written. Exact
results and source membership are retained in the delivery audit directory.
The new comparison passed on its first execution; no expected literal changed.

The follow-up wiring declares the seven oracle tests in the production test
module graph and commits their literal constants. A fresh direct-rustc run of
that graph passed all eight tests and 92 prefixes, with the constants checked
byte for byte against the frozen JSON. Whole-crate Cargo execution remains
unobserved; module discovery is not a claim that it was run.

The [diagnostic follow-up](audits/2026-10-15/sherlock-resource-diagnostics/README.md)
implements Argon's resource classifier, additive checked counter and host API
documentation. Its five authored tests first failed against stubs, then all
thirteen tests passed, including the original 92 prefixes. That capture used an
isolated checkout; integration membership records the sole doc-header difference
from this combined tree. The error/cap tests are finite observations; full host
resource admission remains open and untrusted consumers remain excluded.

These are finite host observations. The full `mori-core` Cargo build and public
CLI were not exercised. The separate `/1` preparation API above projects a
scalar subset; general `/1` and `/2` compiler lowering and execution through
this reference API remain open. This is not observed K execution, K/Lean
correspondence, B helper metering, bounded circuit behavior or native acceptance.

## Prepared design inputs and remaining work

The rule/profile catalogs retain actual K/Quint differences. The independent
financial-prefix JSON records proposed complete arithmetic observations; this
host component does not execute financial opcodes. The B schema vocabulary and
Opus-authored Phase 0 proposal remain unfinished, unadopted source preparation.
No layout, capacity, backend or cost winner is selected here.

Next work retains all of B-T01–B-T14: complete the independent demonstrator pack,
settle/freeze the exact B rule/representation contracts, prepare ProgramImage and
source maps, implement bounded helpers/ADTs/frames/arithmetic, reconcile the
existing Lean foundation, build Clean constraints and correspondence, then the
actual Midnight exporter and qualified recursive proof integration. Financial
continuation mappings and full grammar/compiler consumers remain required.

U0–U7, M1–M17, MC/SP/G/ZR/MNR/OSR, ACTUS, DeFi and composition exits remain open.
Native authority and historical counters remain unchanged.

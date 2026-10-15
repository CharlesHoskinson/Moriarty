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
CLI were not exercised, and the source `/1` and `/2` parsers/compilers do not yet
lower to this reference API. This is not observed K execution, K/Lean
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

# Sherlock source rule and profile catalogs

Status: source preparation ready for integration. These files do not freeze Phase 0,
implement an interpreter or report runtime, correspondence or security acceptance.
The integrator owns the shared schemas. This catalog specifies their source inputs.

[rules.json](rules.json) contains every actual rule in
[main.k](../k/main.k): **113 entries**, comprising 5 administrative, 37 charged
machine and 71 unmetered pure helper cases. The latter include 11 `uiTerminal`
cases. Each entry has an external catalog ID, exact rule text, inclusive source
line span, whole-source SHA256, input/guard, output cells/helper result, charge
and literal/propagated errors. These IDs are labels assigned by this catalog;
the K source has no corresponding rule-label declarations. Error codes **1–11**
are listed from their actual producers. Saved environments and the nine actual
frame forms are separate source facts.

[profiles.json](profiles.json) retains the distinct K, actual repaired Quint and
proposed B schedules. Its 59 exact Quint top-level declarations/definitions and
22 event rows describe the finite algorithm abstraction. The source function is
`push`. It checks `frames.length()+1 > MAX_FRAMES`; its caller's tick has already
charged. The exact source also writes argument/cursor/control fields after some
`push` calls, including its refusal branch. K's explicit `uiPush` is a separate
charged transition. Quint terminal return costs one tick; K final/error
publication and startup cost zero. A terminal K control publishes at fuel zero;
nonterminal controls exhaust there. These differences require adapters and do
not establish cross-profile same-fuel correspondence.

Source locators use one-based inclusive lines and SHA256 of the complete file's
bytes. Counts concern source cases, not execution frequency. A rule's exact
`source_text` and cell patterns govern over prose descriptions. Omitted K cells
remain unchanged. Pure functions and imported Map/equality operations contribute
zero K semantic fuel; this is not a bounded host-work or circuit-cost claim.
The source maps are not cryptographic commitments or membership proofs.

The `proposed_b` section is deliberately separate from `k_rules`. Nine helper
families cover fixed scalar/tag/primitive work, environment lookup/update,
function lookup, binding, append, validation, structural equality and matching.
Their 66 proposed events explicitly include entry, exit, empty/missing scans,
source-result failure exits, task cursors and bounded resume fields. Four extra
task/resume bookkeeping events prevent hidden recursive initialization/cleanup.
Each helper event inspects at most one declared occurrence/record; variable
children, copy/unwind, duplicate scans and release work take further events.
Every machine rule has its own instance of the separately charged
`b.legacy.complete` event. Entry/exit are proposed as separate events here;
combining them requires an explicit catalog revision before exact B fuel oracles.

An in-progress helper interval erases to its saved legacy pre-state; only its
legacy completion erases to the post-state and advances the reference cursor.
Saved state uses handles/selectors, never an implicit whole-state copy. The
conditional equation `W_B = W_A + H_B` remains an unproved correspondence target.
Binding records K's tail-result dependency: source pairs are encountered from
left to right; `uiBindOne` inserts on unwind from right to left. B ordered visits
are proposals where K equations do not supply an event trace.

Dedicated source lists/trees and shallow patterns do not implement declaration-
driven ADTs. Separate coverage rows retain `TypeDecl`, `CtorLayout`, strict
ordered constructor fields, nested `MatchPlan`/`workMap`, nominal/mutual groups,
and later extensions. No generic implementation is claimed.

The named `S-B-*` rows remain exclusively Opus 5.5-owned: exact per-rule failure
charge/allocation, cap/precedence, resource/admission policy, representation and
security evidence. They must be resolved and reviewed with the shared I schemas
before any Phase 0 freeze. This catalog reproduces existing proposed schedule
facts without supplying a new policy. No numeric capacity, limb layout, k,
circuit family or cost winner is selected. Whole-program closed-body and
unique-function admission remain missing; first matching source function row
continues to govern the existing K source.

## Executable host component and draft B schemas

The separate [`mori_core::sherlock` component](../../../crates/mori-core/src/sherlock/mod.rs)
implements the typed host common domain of `sherlock-k-a/1`. It uses the actual
ordered function table, explicit control and nine frame forms. Source recursion
uses named calls; it is not unrolled. The public library API is consumed by the
[mutual-recursion example](../../../crates/mori-core/examples/sherlock_reference.rs)
and the independent normal comparison consumer below. This is not yet a source
parser/compiler adapter or the new B profile.

The API starts at K's valid post-startup boundary. Its `u128` values and
nonnegative budgets exclude K's negative startup and out-of-range literals;
those K rules are not tested or implemented by this typed input boundary.
Canonical signed-decimal names define the K comparison domain. Host strings
outside that domain have no asserted K correspondence. Snapshot frames are top
first, following K, rather than B's proposed bottom-first frame array. A-profile
helper work, cloning, equality, formatting and host allocation remain unmetered.

[`check-reference.py`](check-reference.py) transcribes the independent JSON
literals into temporary Rust constants and compiles the actual production
modules with a supplied pinned `rustc`, without Cargo or SDK dependencies:

```sh
python3 -B models/sherlock/reference/check-reference.py \
  --rustc /absolute/path/to/pinned/rustc --timeout-seconds 30
```

It compares all complete charged boundaries, free publication and 92 fuel
prefixes of six normal programs. It checks source-term identity before comparing
execution. The expectations predate this implementation; the script does not
evaluate a program or compute expected work. Actual results and their exact
source membership belong in the separately linked audit record, not in these
source catalogs. The original oracle author's temporary literal serializer was
not retained; these literal JSON files and their derivation notes are retained.
This new consumer is a reproducible transcription of those frozen files, not
the lost original serializer.

[`schema-types.json`](schema-types.json) is an unfinished typed vocabulary for
the separate B representation proposal. It is not the Rust A API, a frozen
layout, a validator, a circuit encoding or an adoption decision. Widths,
dictionaries, capacity formulas and integration with Opus dispositions remain
open. [`security-phase0.md`](security-phase0.md) is copied verbatim from its
Opus 5.5 author report and remains proposed and unexecuted. Its B tests are not
implemented or passed by this host component.

Catalog checks are ordinary bounded JSON/source membership and hash inspection.
The separate host comparison has its own source-inspected execution scope;
no K/Quint/Lean or SDK/native execution is involved. Existing model,
fixture, expected-result and observation bytes are preserved. Fresh integrated
Muse nonsecurity and applicable Opus reviews remain separate work. The full
U0–U7, M1–M17, MC/SP/G/ZR/MNR/OSR, ACTUS, DeFi and composition exits stay open.

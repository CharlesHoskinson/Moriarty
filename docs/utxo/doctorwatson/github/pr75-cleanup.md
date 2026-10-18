# PR75 source and documentation cleanup

PR75 remains OPEN/DRAFT at `2c87d1a8c45677acf2568aeff1511d0c7d72d044`. GitHub reports MERGEABLE/CLEAN.
The staged-test feature is absent from main `1721457439dbd6d988d00c6c86d1cf648caf2a0c`.
The merge base is `9a7c042d3546331f6529e72f768af996bfdad6f3`; the actual diff contains 167 paths:
166 additions and one modification (`crates/mori/src/commands/test.rs`).
No path changed on both branches since that base. `test.rs` on main is unchanged
from the base. This is a read-only conflict assessment; no merge was attempted.

The unapplied [patch](pr75-provenance.patch) changes only the guide and current
provenance document. It links to the header-correction R3 binding, labels the
older binding historical, removes stale “Root must supply” wording, and states
that the changed documentation still requires fresh applicable reviews. Frozen
JSON, old expectations, historical reviews and product source are untouched.
Exact before/after byte sizes and SHA-256 values are in [the record](pr75-cleanup.json).

The historical-fixture question is resolved at the source level. `test.rs:9–11`
requires a directory; `test.rs:40–45` opens only its `mori-tests.json` and selects
`moriarty-tests/2` by the profile key. `stage_test.rs:69–75,91–138` uses only the
cases and paths in that manifest. The current root manifest names two cases,
`two-deliveries` and `quoted-fee`, and six expected reports. It contains no history
path. `stage_test_public.rs:24–31` copies root regular files only; subdirectories
are skipped. Its `run` method (43–48) selects that temporary root explicitly.
There is no recursive discovery on these consumer paths. Explicitly selecting
`models/core-stage-tests/history/blocked-r2` would select its preserved manifest;
that does not restore its invalidated positive expectations. No run was made.
The inspected search scope is captured in `pr75-source-snapshot/selector-search.txt`.
External wrappers are outside this finding. Copied assembly scripts remain
unportable historical provenance and were not run.

Applicable EARS mapping is explicit in the JSON: UNI-018 (profile/consumer),
UNI-020 (independent frozen expectations), UNI-021 (scoped evidence and open
parents), UNI-024 (public consumer and current routing), MPLR-012 (assurance
status), and DWU-03/04/08/09 (currency, source/result review, no inferred product
success, reporting-only default). PR75 is a Rust local Core/2 precursor;
registration under doctorwatson<UTXO> grants no Cardano/Groth16/target acceptance.
M10 maps to U2/U7/MC08 in traceability; U3 and partial/conditional obligations
remain open. No catalogue or scenario membership changed.

Readiness remains NOT_APPROVED for ready-for-review and merge. The existing
Muse and Opus dispositions cover their exact prior source/draft candidate only.
The patch needs fresh applicable non-author source review under current AGENTS
routing; security judgments remain assigned to authorized Opus. Separate
execution admission and independent actual-result review are still missing.
Any later check must inspect the exact consumer/build graph first: the staged
harness itself calls local Core APIs, but the mori binary and package link
backend/ledger dependencies and the package contains native integration tests.
A command named test is not blanket ordinary execution authority.

All builds, Cargo, CLI, tests, evaluator and runtime/printed-format comparisons
remain UNRUN. Comparison admission is NOT_GRANTED; native authority is zero.
P-TRACE-PROJ/draft is unadopted; all fourteen gaps, C1–C6, historical N1–N6,
G1–G6/K1–K4 and every U/M/MC/SP/G/partial/conditional parent stop remain.
Metadata/source reads, SHA-256, JSON parsing and patch reconstruction are the
only checks recorded here. No product/security/financial/proof/ledger result,
publication, ready transition, merge, issue closure or new execution is claimed.

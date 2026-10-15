# Sherlock K-A diagnostic implementation and observed tests

The separate Gemini 4 Argon security-author proposal in
[`ARGON-HOST-FOLLOWUP-AUTHOR.json`](../sherlock-reference-wiring/ARGON-HOST-FOLLOWUP-AUTHOR.json)
supplies the implementation, API documentation and five new tests. The
integrator copied its exact code and expectations; authoring is not approval.

Implemented host diagnostics:

- `Outcome::is_resource_limit()` follows the existing K-A A1 partition:
  `Exhausted` and `Rejected(4)` return true. It is not a native predicate.
- `Snapshot::checked_charged_steps()` returns `None` for invalid hand-built
  counters. The existing getter retains its type and has a documented invariant.
- The machine API documents its unmetered host behavior and excluded consumers.
  Executable machine transitions are unchanged; `reference.rs` adds rustdoc only.

`ARGON-COMMAND*` binds the security owner's classification of the exact RED and
GREEN source sets as ordinary host checks. The source packet is retained in
`COMMAND-SOURCE-REQUEST.txt.gz`. The command compiled the actual module graph
directly with pinned Rust 1.99/std only; no Cargo, SDK or native operation ran.

`RED-RESULT.json` records compilation success followed by five intended runtime
assertion failures against the author's stub methods; eight old tests passed.
`GREEN-RESULT.json` records all thirteen tests passing after the exact authored
implementation was applied. The new tests cover the resource partition, error
codes 3–11, frame-cap boundaries 0/1/2, zero-fuel publication precedence and the
checked diagnostic counter. The six old normal programs and 92 literal fuel
prefixes remain unchanged. This is finite host evidence, not refinement.

Execution occurred in the isolated diagnostic checkout while the parent
candidate remained fixed for review. `INTEGRATION-SOURCE-MEMBERSHIP.json` records
that the integrated source differs from that tested GREEN source only in the
normal-test module's doc header. Executable lines and expectations match. No
new execution of the combined current tree is claimed by copying those receipts.
Raw receipts contain source/runtime paths; keep them as evidence, never input
to Clean or semantic schemas. Tool timing fields are not performance metrics.

Full H1 host admission bounds remain open: these helpers and rustdoc do not
admit untrusted CLI/LSP/MCP/network or source `/1`/`/2` consumers. Whole-crate
Cargo, public example execution, B helpers/representations, Clean constraints,
correspondence, native export and every existing parent exit remain open.
Native authority and historical counters are unchanged. Fresh non-author
Muse and security-owner reviews are required for source integration.

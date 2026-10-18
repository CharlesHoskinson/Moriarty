# Original requirements and exact current source

All links below point into the pinned `source-basis/` snapshot at commit74a909d69639df8823a527a4d608a7516ccd69a2, treef18c2346ff435f9ac95a7f6f56457c2edde3f71c. SOURCE-PINS.json records bytes, SHA256 and line counts for every full file. Line references identify the inspected rule or API; they do not narrow the retained full requirement.

| Basis | Exact location | Relation to this proposal |
| --- | --- | --- |
| MPLR001 committed steps/outstanding state | partial-conditional spec lines3–12 | Keep separate complete actual imported prefixes and ordered subsequent invocation histories; no fictional cross-parent chronology |
| MPLR002 typed persistent continuations | same spec lines14–23 | Typed opaque in-process subset only; persistence, authority and exactly-once obligations remain open |
| MPLR003 conditional evidence combination | same spec lines25–34 | Two explicit unauthenticated local Bool observations only; no authenticated signature/document/proof claim |
| MPLR007 joins and late results | same spec lines69–78 | Separate all_observed availability and all_complete success; preserve every duty; quorum/any-success/late branches not supplied |
| MPLR009 cumulative costs | same spec lines91–100 | Exact complete normal Core receipt sums without local committed resets; orchestration unmeasured; signed envelope/resource obligations open |
| MPLR011 predecessor/concurrency | same spec lines113–122 | Explicit disjoint input domains and local handoff links only; no global concurrency or stale-state claim |
| MPLR012 assurance reporting | same spec lines124 onward | Visible local-reference-stage/authentication:none/consumption:untracked and all omitted judgments |
| MPLR017 typed financial language | same spec lines179–188 | Full existing typed snapshots/assets/duties/effects; no invented settlement or omitted residual rows |
| C3 join/recovery delivery | implementation-plan.md line13 | Partial local all-of design only; complete C3 equations/proofs, branch obligations and recovery remain open |
| Original join research | wiki/research/mplr/MPLR-007.md complete47lines | Concrete syntax/policy/proof/target remained open; proposed syntax is a new choice, not already adopted design |
| Current normal stage API/DTOs | stage/mod.rs lines1–193 | Existing start/resume, whole Program/Plan identity, full Continuation, PendingFrame, StageOutcome and qualification labels reused |
| Current real stage producer | stage/admit.rs complete392lines | Inspected actual start/resume path, continuation cost prefix and boundary formation; not an independent oracle |
| Current complete child report | stage/report.rs lines1–38 | Keep unchanged nested whole Waiting/Complete envelope projection |
| Existing disjoint product | stage/disjoint.rs lines139–170,242–391 | Current Pair starts two members and preserves disjoint full boundaries/history; lacks declared successor handoff and one typed join control object |
| Current single-Await conjunction test | tests/core2_multi_condition_complete.rs complete213lines | One frame/whole tuple, false events not accumulated; this proposal joins independently pending frames instead |

The spec path is `openspec/changes/partial-and-conditional-transactions/specs/partial-conditional-transactions/spec.md`; stage paths are under `crates/mori-core/src/`; the conjunction test is under `crates/mori/`. Full originals are present under source-basis, including their broader and security clauses. This author neither supplies those judgments nor substitutes ordinary tests for them.

The current public CLI simulate writer is inspected and pinned, but this proposal adds no CLI command or stdout envelope. The concrete integration proposal is a future SDK-free mori-core library test target only. Existing CLI reporting and byte contracts remain unchanged. ROADMAP.md, product contract, Rust guidelines and source AGENTS are pinned as governing scope; no closure/count changes are proposed.

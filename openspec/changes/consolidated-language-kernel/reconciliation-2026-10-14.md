# OpenSpec reconciliation — 2026-10-14

This is current interpretation of the existing package, not another delivery queue.
Baseline: merged PR50, `fc6353bf19b87a76410319c378efe529d52d23d9`.
The [roadmap](../../../ROADMAP.md), [mid-roadmap review](../../../docs/audits/2026-10-14/MID-ROADMAP-REVIEW.md)
and [audit plan](../../../docs/audits/2026-10-14/AUDIT-PLAN.md) control this reconciliation.
The review's PR49 source observations retain their recorded trees and methods.
This document ran no product, SDK, model, native or ledger operation.

## Authority and evidence interpretation

U0–U7 remains the single sequence. The user's direct M1–M17 request controls its
language-capability obligations; it needs no further adoption vote. MC01–MC08,
SP01–SP12, G01–G24, L1–L8, ZR01–ZR16, MNR01–MNR08, MPLR-001–035,
UNI-001–017, MOR-001–012 and AEO-001–004 remain required at their existing scope.
The [traceability register](traceability.md) retains each family and its owners.
P/C/K and sprint schedules are supporting decomposition, not competing queues.

Use three distinct status axes: implementation (`implemented`, `partial`, `stub`,
`specified`, `unknown`), evidence (`source-inspected`, `historical-local`,
`current-local`, `native-local`, `target-observed`) and acceptance (`open`,
`accepted`, `refuted`). Each record names its predicate, source tree, profile,
consumer, target, artifact and limitation. Unknown cells stay unknown. A blocked
stage supplies no downstream pass. All full U/M/MC/SP/G exits remain open.

GPT-6.1 Sol at high effort implements current nonsecurity language work.
Fresh non-author Muse Wonder 1.0 at high effort audits the exact candidate and
applicable actual results. Changed bytes require fresh review. Record requested
and returned identities separately. Missing reviewers cannot approve. Existing
terminal task contracts and historical reviewer receipts keep their identities.
The [October 14 routing assignment](../../../raw/assignments/moriarty-sol-muse-routing-2026-10-14.md)
supersedes older active Fable/Astra/Grok selection in inherited prose.

All security work belongs to Opus under [security routing](../../../docs/audits/2026-10-14/SECURITY-ROUTING.md).
Existing security clauses are preserved; this reconciliation supplies no security
assessment or redesign. Current native authority is zero. Planned commands in
inherited packages grant no authority to execute SDK checks/queries, model/degree
queries, VM execution, key generation, proving, native verification or ledger work.
Existing counters, failures, resource controls and private-material restrictions persist.

The separate [Opus security addendum](security-reconciliation-2026-10-14.md) retains
its OSR-1014 IDs, authorship and stated evidence limits. It supplements the
canonical 160-heading `spec.md` index and is separately indexed in the current
crosswalk; this nonsecurity reconciliation does not restate or assess its clauses.

## Profiles and consumers

| Profile / path | Observed consumer | What the observation supports | What remains open |
|---|---|---|---|
| Rust `moriarty/1` / `moriarty-core/1` | `crates/mori-core`; Rust `mori` Core branch | Bounded parsing, deterministic evaluation, local intents, manifest tests and reference replay | Full profile/grammar/numeric matrix and U0/U2 correspondence |
| Rust beta authoring | Rust `mori check`, alternate parser branch | Local syntax/static checking in its declared profile | Compilation acceptance in Rust Core; check success is insufficient |
| Node beta / Source/6 | `packages/moriarty-beta/src/bridge.ts` | Source/6 expansion and local `PreparedUnqualified` through experimental TypeScript Core/5 | Rust compilation and native/target/release qualification |
| Rust transition lowering | `mori compile`; `crates/mori-backend` | Bounded ledger-9 ZKIR lowering with a supplied target descriptor | Admitted-grammar coverage, qualified deployed tuple and U1/U2 exits |
| Rust native-call lowering | `mori compile-call`; `crates/mori-ledger` | Captured all-action source-static export and exact reopen, including mapped-role static mode | Complete installed/native/target chain; mapped-role mode remains unproven, uninstalled and unauthenticated |
| Rust server entry points | `mori lsp`, `mori mcp` | Explicit `todo!` bodies | Working Rust language-server/MCP implementation and release qualification |

Both Rust and Node packages expose an executable named `mori`. A developer record
names the resolved executable and package. Rust Core is not experimental Core/5.
The [review](../../../docs/audits/2026-10-14/MID-ROADMAP-REVIEW.md#integration-findings-and-audit-priorities)
and inspected `crates/mori/src/commands/check.rs`, `compile.rs`, `lsp.rs`,
`mcp.rs`, and the Node bridge establish these source observations.

The two lowerings encode different statements and admit different subsets.
U0 records an admission lattice and explicit per-consumer refusals. Compare
complete observations on the documented common subset. Do not require global
equivalence or identical bytes. Exact static reopen checks deterministic identity;
replay through the same evaluator does not supply an independent semantic oracle.
Core positional `ensures`, successor trailing postconditions, eager Boolean
evaluation, present-zero duties and finite-width arithmetic need separate contracts.
The successor numeric decision does not silently change other profiles.

## Existing task and child-capability reconciliation

An unchecked parent task can already contain implemented child behavior. Preserve
the checkbox and original task text until its complete evidence is reconciled.
The rows below select actual gaps under existing IDs; they add no parallel task list.

| Existing owner / task scope | Delivered child or retained evidence | Disposition / remaining predicate |
|---|---|---|
| U0; MC01; SP01/SP02 | Rust Core and separate beta/Source/6 implementations exist | Reconcile versioned grammar, semantics, numeric scope and consumer matrix; old TypeScript paths remain provenance, not names for Rust Core |
| U0/U2; MC01/MC04; SP03 | September 17 finite K lifecycle results and retained Quint models | Preserve exact profile/trace adapters; full MIL/4 claims remain open; large timed-out Quint sample remains unknown |
| U1/U2; MC02/MC04; SP04/SP05 | Historical scoped local native and Preview loan/swap records | Preserve each target/tree and original failure; no transfer to Fenrir or general language acceptance |
| U2; MC04; SP05/SP09 | PR47/49 all-action static export/reopen | Record source-static child implementation; retained native and complete-effect obligations remain open |
| U3; MC05/MC06; SP08/SP10 | Local sponsorship/recovery holds and policy/meter consumers from PR46/48 | Qualify actual funding/finality, complete lifecycle and applicable cost planes; supplied observations remain labeled |
| U4; MC03/MC06; SP06/SP10 | Retained research, semantic/model inputs and historical scoped proofs | Full native recursive financial history and private composition remain open; no proof permission is created |
| U5; MC05/MC07; SP08 | Local policy and metering precursors | Exact adapters, complete services and M16/M17 remain open; identities and external target facts remain unknown |
| U6; MC07; SP07/SP08/SP11 | PR49 finite FeePool: 23 actions and 162 captured financial cases | Captured local/reference/static evidence only; complete M11 includes Capacity Exchange and target effects; full conformance remains open |
| U7; MC08; SP02/SP12 | Real Rust local commands; separate beta authoring/editor/MCP package | Resolve executable/profile journey, Rust server stubs and aggregate 1 MiB report overflow; independent release remains open |
| Language-to-ledger tasks 1–3 | PR5–PR7 local postconditions/origination/lifecycle are already checked with retained receipts | Keep their historical scope; task 4 finite K evidence needs item-level reconciliation; task 5 authenticated execution remains open |
| MOR/AEO/DEV and P/C/K tasks | Detailed retained requirements and symbolic Pel templates | Map through U owners; an unchecked implementation item does not assert all child APIs are absent; template checks are not product results |

The FeePool counts come from the published PR49 dossier linked by the mid-roadmap
review. They are captured measurements, not new runs here. Library specimens do
not define the language boundary or reduce conformance denominators.

## Required next demonstrations under the existing U sequence

U0 first reconciles profiles, admitted grammar, numeric contracts and both
lowering interfaces. Independent expectations precede execution. Expected results
must not import the evaluator/compiler under review. Preserve complete ordered
effects, affected and unchanged state, gross debit, fees, net receipts, dust,
opening/accrued/discharged/closing liabilities and absent versus present-zero duties.
Count logical work under a versioned schedule separately from target cost,
DUST, service fees and recovery reservations.

U2 still needs two positive, newly authored contrasting programs through the same
claimed supported pipeline. The audit plan proposes an installment invoice and
conditional reserve allocation; these are unexecuted specimens. Mixed-payer static
export or an intended native refusal is a separate track, not the second positive.
Two examples also do not replace rule-level grammar and field coverage.

U3 retains spend-at-most-11-A, fees-at-most-1-A, receive-at-least-20-B, two-asset
escrow, partial progress, continuing duties and recovery requirements. U4 retains
sequential, disjoint parallel, shared-state interleaving, atomic synchronization
and asynchronous messaging. Applicable accepted SP07/SP08 semantic inputs and
extension requalification still govern full SP09 and U4 scope.

U6 preserves all 277 ACTUS fixtures and expected fields, 18 executable types,
32 taxonomy dispositions and DS01–07; all 72 DeFi rows, DA01–24, adopted
supplements, three held-outs, eight intent cases and eight regression classes.
Check actual membership against frozen manifests before scoring. Keep semantic,
local, native, Preview and Fenrir evidence separate. No completion percentage is
derived from the discovery package's 69 top-level IDs or from test totals.

Preview evidence does not qualify Fenrir. Fenrir network ID, Anchor, committee,
registry identities and the matching target tuple remain unestablished. The user
reports no Fenrir versioning system yet; no invented version is required to
continue independent source work. M1–M17 remains controlling despite those gaps.

## Document and machine-register interpretation

Dated studies, original audits, frozen resource contracts, original package-task
text and historical crosswalks keep their recorded claims and source digests.
The older S2 label describes the full package exit, not every present child API.
Current entry points link here to interpret those records against PR50.

[current-requirement-crosswalk.json](../../current-requirement-crosswalk.json)
indexes every current OpenSpec requirement heading and its U owners. It retains
the nine renamed historical heading aliases without changing original coverage.
It is a requirement index, not an atomic predicate denominator or acceptance ledger.
The existing `openspec/sprints/verify.py` checks this current index alongside the
original task, family, report and release-gate retention checks.

The historical report crosswalk pins an older `docs/FOOTGUNS.md`. Its exact bytes
are retained under `openspec/source-snapshots/74027e6d0e574f03700acc01405288d5d8e77f503b766a46ac18bb7e0efc4a27/FOOTGUNS.md`,
recovered from commit `214141c1b0a547f7d8b8684ecb1669e4ba96e161` and matched to
the original SHA-256. The validator resolves this explicit historical snapshot;
it does not relabel today's maintained guidance as the old report source.

`parallel-language-workflow.json` retains the October 13 task contracts and roster
as historical scoped preparation. Its current-guidance reference selects current
routing; no old roster, proposed command or JSON flag grants native authority.
The same distinction applies to the completion program and sprint registers.

Ordinary documentation checks are `python3 openspec/sprints/verify.py`,
`python3 openspec/sprints/test_verify.py`, and
`openspec validate --all --strict --no-interactive`. These check specification
structure and retention only. This reconciliation does not invoke product tests.
